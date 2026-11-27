//! Strict p-code lowering for proof terms. Every operand must be modeled.

use std::rc::Rc;

use kuna_num::opcodes::{get_opname, OpCode};

use crate::concolic::{self, Emitter, Expr};

pub(super) type Term = Rc<Expr>;

pub(super) fn lower(opcode: OpCode, out: u32, operands: &[Term]) -> Result<Term, String> {
    use OpCode::*;
    let unsupported = || {
        format!(
            "unsupported symbolic {} with {out}-byte result",
            get_opname(opcode)
        )
    };
    if !(1..=16).contains(&out) || operands.iter().any(|arg| !valid(arg)) {
        return Err(unsupported());
    }
    let unary = match opcode {
        CPUI_COPY | CPUI_CAST | CPUI_INT_ZEXT | CPUI_INT_SEXT | CPUI_INT_NEGATE
        | CPUI_INT_2COMP | CPUI_BOOL_NEGATE | CPUI_POPCOUNT => true,
        CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_MULT | CPUI_INT_DIV | CPUI_INT_SDIV
        | CPUI_INT_REM | CPUI_INT_SREM | CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR
        | CPUI_BOOL_AND | CPUI_BOOL_OR | CPUI_BOOL_XOR | CPUI_INT_LEFT | CPUI_INT_RIGHT
        | CPUI_INT_SRIGHT | CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_INT_LESS | CPUI_INT_SLESS
        | CPUI_INT_LESSEQUAL | CPUI_INT_SLESSEQUAL | CPUI_INT_CARRY | CPUI_INT_SCARRY
        | CPUI_INT_SBORROW | CPUI_PIECE | CPUI_SUBPIECE => false,
        _ => return Err(unsupported()),
    };
    if operands.len() != if unary { 1 } else { 2 } {
        return Err(unsupported());
    }
    let bits = out * 8;
    if opcode == CPUI_SUBPIECE {
        let Expr::Const(offset, _) = *operands[1] else {
            return Err("symbolic SUBPIECE requires a constant byte offset".to_owned());
        };
        if offset > 16 || offset as u32 * 8 + bits > operands[0].width() {
            return Err("symbolic SUBPIECE falls outside its input".to_owned());
        }
    }
    if opcode == CPUI_CAST {
        return Ok(resize(Rc::clone(&operands[0]), bits));
    }
    if opcode == CPUI_POPCOUNT {
        let input = &operands[0];
        return Ok(
            (0..input.width()).fold(Rc::new(Expr::Const(0, bits)), |sum, n| {
                let bit = Rc::new(Expr::Extract(Rc::clone(input), n, n));
                Rc::new(Expr::Op("bvadd", vec![sum, resize(bit, bits)], bits))
            }),
        );
    }
    let inputs: Vec<_> = operands
        .iter()
        .map(|arg| {
            let constant = match **arg {
                Expr::Const(value, _) => value,
                _ => 0,
            };
            (Some(Rc::clone(arg)), constant, arg.width() / 8)
        })
        .collect();
    concolic::op(opcode, out, &inputs).ok_or_else(unsupported)
}

/// The p-code interpreter's division domain for the supported x86 model.
pub(super) fn division_guard(opcode: OpCode, operands: &[Term]) -> Option<Term> {
    use OpCode::*;
    if !matches!(
        opcode,
        CPUI_INT_DIV | CPUI_INT_SDIV | CPUI_INT_REM | CPUI_INT_SREM
    ) {
        return None;
    }
    let a = &operands[0];
    let b = &operands[1];
    let bits = b.width();
    let not = |left: &Term, constant| {
        Rc::new(Expr::Test(
            "distinct",
            Rc::clone(left),
            Rc::new(Expr::Const(constant, bits)),
        ))
    };
    let nonzero = not(b, 0);
    if bits != 64 || !matches!(opcode, CPUI_INT_SDIV | CPUI_INT_SREM) {
        return Some(nonzero);
    }
    let safe = Rc::new(Expr::Op(
        "bvor",
        vec![not(a, 1 << 63), not(b, u64::MAX.into())],
        8,
    ));
    Some(Rc::new(Expr::Op("bvand", vec![nonzero, safe], 8)))
}

pub(super) fn observed_terms(
    emitter: &mut Emitter,
    left: super::execute::Execution,
    right: super::execute::Execution,
) -> Result<String, String> {
    let normalize = |side: &super::execute::Execution| {
        let zero = Rc::new(Expr::Const(0, side.value.width()));
        select(Rc::clone(&side.defined), Rc::clone(&side.value), zero)
    };
    let left_value = normalize(&left)?;
    let right_value = normalize(&right)?;
    let (la, lv) = (emitter.name(&left.defined), emitter.name(&left_value));
    let (ra, rv) = (emitter.name(&right.defined), emitter.name(&right_value));
    Ok(format!("(or (distinct {la} {ra}) (distinct {lv} {rv}))"))
}

fn valid(term: &Term) -> bool {
    let bits = term.width();
    (8..=128).contains(&bits) && bits % 8 == 0
}

fn resize(term: Term, bits: u32) -> Term {
    let in_bits = term.width();
    match bits.cmp(&in_bits) {
        std::cmp::Ordering::Equal => term,
        std::cmp::Ordering::Greater => Rc::new(Expr::ZeroExt(term, bits - in_bits)),
        std::cmp::Ordering::Less => Rc::new(Expr::Extract(term, bits - 1, 0)),
    }
}

/// Lower an SSA edge selection using a nonzero guard and equal-width arms.
pub(super) fn select(guard: Term, yes: Term, no: Term) -> Result<Term, String> {
    let bits = yes.width();
    if !valid(&guard) || !valid(&yes) || no.width() != bits {
        return Err("invalid symbolic conditional widths".to_owned());
    }
    let zero = Rc::new(Expr::Const(0, guard.width()));
    let truth = Rc::new(Expr::Test("distinct", guard, zero));
    let mask = Rc::new(Expr::Op("bvneg", vec![resize(truth, bits)], bits));
    let complement = Rc::new(Expr::Op("bvnot", vec![Rc::clone(&mask)], bits));
    let chosen = Rc::new(Expr::Op("bvand", vec![mask, yes], bits));
    let other = Rc::new(Expr::Op("bvand", vec![complement, no], bits));
    Ok(Rc::new(Expr::Op("bvor", vec![chosen, other], bits)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_lowering_rejects_unsupported_and_out_of_range_terms() {
        let word = Rc::new(Expr::Var(Rc::from("b0")));
        assert!(lower(OpCode::CPUI_LOAD, 1, &[Rc::clone(&word), Rc::clone(&word)]).is_err());
        assert!(lower(
            OpCode::CPUI_SUBPIECE,
            1,
            &[Rc::clone(&word), Rc::clone(&word)]
        )
        .is_err());
        let bad_offset = Rc::new(Expr::Const(1, 32));
        assert!(lower(OpCode::CPUI_SUBPIECE, 1, &[Rc::clone(&word), bad_offset]).is_err());
        assert!(lower(OpCode::CPUI_INT_CARRY, 2, &[Rc::clone(&word), word]).is_err());
    }

    #[test]
    fn selection_retains_the_arms_width() {
        let guard = Rc::new(Expr::Var(Rc::from("b0")));
        let high = Rc::new(Expr::Const(1u128 << 127, 128));
        let low = Rc::new(Expr::Const(0, 128));
        assert_eq!(select(guard, high, low).unwrap().width(), 128);
    }

    #[test]
    fn signed_division_distinguishes_an_overflow_trap_from_a_result() {
        use crate::symbolic::obligation::{Obligation, Verdict};
        use std::process::Command;
        use std::time::{Duration, Instant};

        let mut z3 = Command::new("z3");
        if z3.arg("-version").output().is_err() {
            return;
        }
        let inputs = [
            Rc::new(Expr::Const(1 << 63, 64)),
            Rc::new(Expr::Const(u64::MAX.into(), 64)),
        ];
        let signed = OpCode::CPUI_INT_SDIV;
        let unsigned = OpCode::CPUI_INT_DIV;
        let mut emitter = Emitter::default();
        let compare = |op| super::super::execute::Execution {
            value: lower(op, 8, &inputs).unwrap(),
            defined: division_guard(op, &inputs).unwrap(),
        };
        let mismatch = observed_terms(&mut emitter, compare(signed), compare(unsigned)).unwrap();
        let proof = Obligation {
            domain: format!("(set-logic QF_BV)\n{}", emitter.definitions()),
            mismatch,
            input_bytes: Vec::new(),
        };
        let mut z3 = Command::new("z3");
        z3.args(["-in", "-smt2"]);
        assert!(matches!(
            proof.solve(&mut z3, Instant::now() + Duration::from_secs(5)),
            Verdict::Counterexample(_),
        ));
    }
}
