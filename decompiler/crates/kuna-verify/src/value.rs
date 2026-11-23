//! Concrete p-code value semantics, shared by both sides of a check.
//!
//! Every op on values of at most eight bytes is evaluated through kuna's own
//! `OpBehavior` table, the one `EmulatePcodeCache` uses, so the two sides cannot
//! disagree on what an `INT_SRIGHT` means. Integer ops on wider values, up to
//! sixteen bytes (a 64-bit multiply-high widens to 128 bits), and the
//! decompiler-only ops (`CAST`, `PTRADD`, `PTRSUB`) are defined here. Floating
//! point runs on IEEE single and double, through the same table; other float
//! encodings and anything wider are refused, never approximated.

use std::rc::Rc;

use kuna_num::float::FloatFormat;
use kuna_num::opbehavior::{register_instructions, FloatFormatProvider, OpBehavior};
use kuna_num::opcodes::{get_opname, OpCode};

/// Why a run stopped before it could be compared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Halt {
    /// An op or value the interpreter does not model: `UNKNOWN(unsupported-op)`.
    Unsupported(String),
    /// The step budget ran out: `UNKNOWN(budget)`.
    Budget,
    /// The sides differ after an in-image callee the model could not run got
    /// the stub, with a frame object it may only have read.
    Guessed(String),
    /// The op itself has no defined result (division by zero).
    Trap(String),
    /// The harness could not set the run up: `UNKNOWN(harness)`.
    Harness(String),
    /// A switch met a selector no row of its table carries, and the table has
    /// no default.
    NoRow(String),
    /// The sides differ after a call whose argument a recovered callee
    /// prototype may have wrong: the prototype, not the body, may be at fault.
    Doubtful(String),
    /// The sides differ after the decompiled function read a local nothing
    /// wrote: C does not define the value.
    Undefined(String),
}

impl Halt {
    pub fn render(&self) -> String {
        match self {
            Halt::Unsupported(s) => format!("UNKNOWN(unsupported-op: {s})"),
            Halt::Budget => "UNKNOWN(budget)".to_string(),
            Halt::Trap(s) => format!("UNKNOWN(trap: {s})"),
            Halt::Harness(s) => format!("UNKNOWN(harness: {s})"),
            Halt::NoRow(s) => format!("UNKNOWN(no switch row: {s})"),
            Halt::Doubtful(s) => format!("UNKNOWN(prototype-suspect: {s})"),
            Halt::Undefined(s) => format!("UNKNOWN(uninitialized-local: {s})"),
            Halt::Guessed(s) => format!("UNKNOWN(guessed-callee: {s})"),
        }
    }
}

/// The widest value either side models.
pub const MAX_SIZE: u32 = 16;

pub fn mask(size: u32) -> u128 {
    if size >= 16 {
        u128::MAX
    } else {
        (1u128 << (size * 8)) - 1
    }
}

/// IEEE single and double: the float formats both sides evaluate with.
struct Ieee {
    single: FloatFormat,
    double: FloatFormat,
}

impl FloatFormatProvider for Ieee {
    fn get_float_format(&self, size: i32) -> Option<&FloatFormat> {
        match size {
            4 => Some(&self.single),
            8 => Some(&self.double),
            _ => None,
        }
    }
}

/// The IEEE operations that only move bits: a sign change and a format
/// conversion of a NaN keep its payload, which a trip through the host's
/// floats would not.
fn ieee_bits(opc: OpCode, out: u32, ins: &[(u128, u32)]) -> Option<u128> {
    let &(a, size) = ins.first()?;
    let sign = |n: u32| 1u128 << (n * 8 - 1);
    match (opc, size) {
        (OpCode::CPUI_FLOAT_NEG, 4 | 8) if out == size => Some((a ^ sign(size)) & mask(size)),
        (OpCode::CPUI_FLOAT_ABS, 4 | 8) if out == size => Some(a & !sign(size) & mask(size)),
        (OpCode::CPUI_FLOAT_FLOAT2FLOAT, 4) if out == 8 && is_nan(a, 4) => {
            let (s, frac) = ((a >> 31) & 1, a & 0x7f_ffff);
            Some((s << 63) | (0x7ff << 52) | (frac << 29) | (1 << 51))
        }
        (OpCode::CPUI_FLOAT_FLOAT2FLOAT, 8) if out == 4 && is_nan(a, 8) => {
            let (s, frac) = ((a >> 63) & 1, a & 0xf_ffff_ffff_ffff);
            Some((s << 31) | (0xff << 23) | (frac >> 29) | (1 << 22))
        }
        _ => None,
    }
}

fn is_nan(v: u128, size: u32) -> bool {
    match size {
        4 => (v >> 23) & 0xff == 0xff && v & 0x7f_ffff != 0,
        8 => (v >> 52) & 0x7ff == 0x7ff && v & 0xf_ffff_ffff_ffff != 0,
        _ => false,
    }
}

fn is_float(opc: OpCode) -> bool {
    (OpCode::CPUI_FLOAT_EQUAL as u32..=OpCode::CPUI_FLOAT_ROUND as u32).contains(&(opc as u32))
}

thread_local! {
    static BEHAVIORS: Vec<Option<Rc<dyn OpBehavior>>> = {
        let mut inst = Vec::new();
        let provider: Rc<dyn FloatFormatProvider> = Rc::new(Ieee { single: FloatFormat::new(4), double: FloatFormat::new(8) });
        register_instructions(&mut inst, &provider);
        inst
    };
}

fn signed(v: u128, size: u32) -> i128 {
    let bits = size * 8;
    if bits >= 128 {
        v as i128
    } else {
        ((v << (128 - bits)) as i128) >> (128 - bits)
    }
}

/// Integer semantics on values of up to sixteen bytes.
fn eval_wide(opc: OpCode, out: u32, ins: &[(u128, u32)]) -> Result<u128, Halt> {
    let m = mask(out);
    let (a, asz) = ins[0];
    let (b, bsz) = ins.get(1).copied().unwrap_or((0, 0));
    let abits = u128::from(asz * 8);
    let bool_of = |x: bool| u128::from(x);
    let r = match opc {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ZEXT => a,
        OpCode::CPUI_INT_SEXT => signed(a, asz) as u128,
        OpCode::CPUI_INT_ADD => a.wrapping_add(b),
        OpCode::CPUI_INT_SUB => a.wrapping_sub(b),
        OpCode::CPUI_INT_MULT => a.wrapping_mul(b),
        OpCode::CPUI_INT_AND => a & b,
        OpCode::CPUI_INT_OR => a | b,
        OpCode::CPUI_INT_XOR => a ^ b,
        OpCode::CPUI_INT_NEGATE => !a,
        OpCode::CPUI_INT_2COMP => a.wrapping_neg(),
        OpCode::CPUI_INT_LEFT => if b >= abits { 0 } else { a << b },
        OpCode::CPUI_INT_RIGHT => if b >= abits { 0 } else { (a & mask(asz)) >> b },
        OpCode::CPUI_INT_SRIGHT => {
            let s = signed(a, asz);
            (if b >= abits { s >> 127 } else { s >> b }) as u128
        }
        OpCode::CPUI_SUBPIECE => if b >= 16 { 0 } else { a >> (b * 8) },
        OpCode::CPUI_PIECE => if bsz >= 16 { b } else { (a << (bsz * 8)) | b },
        OpCode::CPUI_INT_EQUAL => bool_of(a == b),
        OpCode::CPUI_INT_NOTEQUAL => bool_of(a != b),
        OpCode::CPUI_INT_LESS => bool_of(a < b),
        OpCode::CPUI_INT_LESSEQUAL => bool_of(a <= b),
        OpCode::CPUI_INT_SLESS => bool_of(signed(a, asz) < signed(b, asz)),
        OpCode::CPUI_INT_SLESSEQUAL => bool_of(signed(a, asz) <= signed(b, asz)),
        OpCode::CPUI_INT_CARRY => bool_of(a.wrapping_add(b) & mask(asz) < a),
        OpCode::CPUI_INT_SCARRY => {
            let r = signed(a.wrapping_add(b) & mask(asz), asz);
            let (x, y) = (signed(a, asz), signed(b, asz));
            bool_of((x < 0) == (y < 0) && (r < 0) != (x < 0))
        }
        OpCode::CPUI_INT_SBORROW => {
            let r = signed(a.wrapping_sub(b) & mask(asz), asz);
            let (x, y) = (signed(a, asz), signed(b, asz));
            bool_of((x < 0) != (y < 0) && (r < 0) != (x < 0))
        }
        OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_REM | OpCode::CPUI_INT_SDIV | OpCode::CPUI_INT_SREM if b == 0 => {
            return Err(Halt::Trap(format!("{}: divide by 0", get_opname(opc))));
        }
        OpCode::CPUI_INT_DIV => a / b,
        OpCode::CPUI_INT_REM => a % b,
        OpCode::CPUI_INT_SDIV => signed(a, asz).wrapping_div(signed(b, asz)) as u128,
        OpCode::CPUI_INT_SREM => signed(a, asz).wrapping_rem(signed(b, asz)) as u128,
        OpCode::CPUI_POPCOUNT => u128::from(a.count_ones()),
        OpCode::CPUI_LZCOUNT => u128::from(a.leading_zeros()) - (128 - abits),
        OpCode::CPUI_BOOL_NEGATE => bool_of(a == 0),
        OpCode::CPUI_BOOL_AND => bool_of(a != 0 && b != 0),
        OpCode::CPUI_BOOL_OR => bool_of(a != 0 || b != 0),
        OpCode::CPUI_BOOL_XOR => bool_of((a != 0) != (b != 0)),
        _ => return Err(Halt::Unsupported(format!("{} wider than 8 bytes", get_opname(opc)))),
    };
    Ok(r & m)
}

/// Ops whose output and inputs are one size. Anything else is ill-formed IR,
/// and truncating it would invent a value.
fn same_size(opc: OpCode) -> bool {
    matches!(
        opc,
        OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_REM
            | OpCode::CPUI_INT_SREM
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_2COMP
    )
}

/// Evaluate a value-producing op. `ins` holds each input's value and size.
pub fn eval(opc: OpCode, out_size: u32, ins: &[(u128, u32)]) -> Result<u128, Halt> {
    if out_size > MAX_SIZE || ins.iter().any(|&(_, s)| s > MAX_SIZE) {
        return Err(Halt::Unsupported(format!("{} wider than {MAX_SIZE} bytes", get_opname(opc))));
    }
    if is_float(opc) && (out_size == 10 || ins.iter().any(|&(_, s)| s == 10)) {
        return crate::x87::eval(opc, out_size, ins).ok_or_else(|| Halt::Unsupported(format!("{} on x87's 80-bit format", get_opname(opc))));
    }
    if let (OpCode::CPUI_FLOAT_INT2FLOAT, 4 | 8, Some(&(v, size))) = (opc, out_size, ins.first()) {
        if size > 8 && size <= MAX_SIZE {
            let n = signed(v, size);
            return Ok(if out_size == 4 { u128::from((n as f32).to_bits()) } else { u128::from((n as f64).to_bits()) });
        }
    }
    if is_float(opc) && (out_size > 8 || ins.iter().any(|&(_, s)| s > 8)) {
        return Err(Halt::Unsupported(get_opname(opc).to_string()));
    }
    if let Some(v) = ieee_bits(opc, out_size, ins) {
        return Ok(v);
    }
    let arg = |i: usize| -> Result<(u128, u32), Halt> {
        ins.get(i)
            .copied()
            .ok_or_else(|| Halt::Harness(format!("{} lacks input {i}", get_opname(opc))))
    };
    let m = mask(out_size);
    match opc {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => return Ok(arg(0)?.0 & m),
        OpCode::CPUI_PTRADD => {
            let (base, _) = arg(0)?;
            let (index, _) = arg(1)?;
            let (scale, _) = arg(2)?;
            return Ok(base.wrapping_add(index.wrapping_mul(scale)) & m);
        }
        OpCode::CPUI_PTRSUB => return Ok(arg(0)?.0.wrapping_add(arg(1)?.0) & m),
        _ => {}
    }
    arg(0)?;
    if same_size(opc) && (ins.iter().any(|&(_, s)| s != out_size)) {
        return Err(Halt::Unsupported(format!("{} with operands of mixed size", get_opname(opc))));
    }
    if out_size > 8 || ins.iter().any(|&(_, s)| s > 8) {
        return eval_wide(opc, out_size, ins);
    }
    BEHAVIORS.with(|table| {
        let behave = table
            .get(opc as usize)
            .and_then(Option::as_ref)
            .filter(|b| !b.is_special())
            .ok_or_else(|| Halt::Unsupported(get_opname(opc).to_string()))?;
        let (a, asize) = arg(0)?;
        let result = if behave.is_unary() {
            behave.evaluate_unary(out_size as i32, asize as i32, a as u64)
        } else {
            let (b, _) = arg(1)?;
            behave.evaluate_binary(out_size as i32, asize as i32, a as u64, b as u64)
        };
        result.map(|v| u128::from(v) & m).map_err(|e| {
            let why = format!("{}: {}", get_opname(opc), e.explain());
            if is_float(opc) {
                Halt::Unsupported(why)
            } else {
                Halt::Trap(why)
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unsigned_word_converts_through_a_wider_integer() {
        let big = u128::from(u64::MAX);
        assert_eq!(eval(OpCode::CPUI_FLOAT_INT2FLOAT, 4, &[(big, 9)]).unwrap(), u128::from((u64::MAX as f32).to_bits()));
        assert_eq!(eval(OpCode::CPUI_FLOAT_INT2FLOAT, 8, &[(big, 9)]).unwrap(), u128::from((u64::MAX as f64).to_bits()));
    }

    #[test]
    fn decompiler_only_ops_are_defined_here() {
        assert_eq!(eval(OpCode::CPUI_PTRADD, 4, &[(0x100, 4), (3, 4), (4, 4)]), Ok(0x10c));
        assert_eq!(eval(OpCode::CPUI_PTRSUB, 4, &[(0xfffffff0, 4), (0x20, 4)]), Ok(0x10));
        assert_eq!(eval(OpCode::CPUI_CAST, 2, &[(0x12345, 4)]), Ok(0x2345));
    }

    #[test]
    fn sleigh_ops_use_the_behavior_table() {
        assert_eq!(eval(OpCode::CPUI_INT_SEXT, 8, &[(0x80000000, 4)]), Ok(0xffffffff80000000));
        assert_eq!(eval(OpCode::CPUI_INT_SRIGHT, 4, &[(0x80000000, 4), (31, 4)]), Ok(0xffffffff));
        assert_eq!(eval(OpCode::CPUI_PIECE, 8, &[(1, 4), (2, 4)]), Ok(0x100000002));
    }

    #[test]
    fn multiply_high_widens_to_sixteen_bytes() {
        let x = eval(OpCode::CPUI_INT_SEXT, 16, &[(u128::from(u64::MAX), 8)]).unwrap();
        assert_eq!(x, u128::MAX);
        let p = eval(OpCode::CPUI_INT_MULT, 16, &[(0x5555_5555_5555_5556, 16), (x, 16)]).unwrap();
        assert_eq!(eval(OpCode::CPUI_SUBPIECE, 8, &[(p, 16), (8, 4)]), Ok(u128::from(u64::MAX)));
    }

    /// The wide evaluator agrees with the behavior table wherever both apply.
    #[test]
    fn wide_semantics_match_the_behavior_table_at_eight_bytes() {
        let ops = [
            OpCode::CPUI_INT_ADD,
            OpCode::CPUI_INT_SUB,
            OpCode::CPUI_INT_MULT,
            OpCode::CPUI_INT_LEFT,
            OpCode::CPUI_INT_RIGHT,
            OpCode::CPUI_INT_SRIGHT,
            OpCode::CPUI_INT_LESS,
            OpCode::CPUI_INT_SLESS,
            OpCode::CPUI_INT_SLESSEQUAL,
            OpCode::CPUI_INT_CARRY,
            OpCode::CPUI_INT_SCARRY,
            OpCode::CPUI_INT_SBORROW,
            OpCode::CPUI_INT_DIV,
            OpCode::CPUI_INT_SDIV,
            OpCode::CPUI_INT_REM,
            OpCode::CPUI_INT_SREM,
        ];
        let vals = [0u128, 1, 3, 7, 31, 32, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff, 0x1234_5678];
        for size in [1u32, 2, 4] {
            for &op in &ops {
                for &a in &vals {
                    for &b in &vals {
                        let (a, b) = (a & mask(size), b & mask(size));
                        let out = if matches!(
                            op,
                            OpCode::CPUI_INT_LESS
                                | OpCode::CPUI_INT_SLESS
                                | OpCode::CPUI_INT_SLESSEQUAL
                                | OpCode::CPUI_INT_CARRY
                                | OpCode::CPUI_INT_SCARRY
                                | OpCode::CPUI_INT_SBORROW
                        ) {
                            1
                        } else {
                            size
                        };
                        let table = eval(op, out, &[(a, size), (b, size)]);
                        let wide = eval_wide(op, out, &[(a, size), (b, size)]);
                        match (&table, &wide) {
                            (Err(Halt::Trap(_)), Err(Halt::Trap(_))) => {}
                            _ => assert_eq!(table, wide, "{} size {size} a={a:#x} b={b:#x}", get_opname(op)),
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn ieee_floats_run_and_division_by_zero_stops() {
        let (one, two, three) = (1.0f32.to_bits().into(), 2.0f32.to_bits().into(), 3.0f32.to_bits().into());
        assert_eq!(eval(OpCode::CPUI_FLOAT_ADD, 4, &[(one, 4), (two, 4)]), Ok(three));
        let half = 1.5f64.to_bits().into();
        assert_eq!(eval(OpCode::CPUI_FLOAT_MULT, 8, &[(half, 8), (2.0f64.to_bits().into(), 8)]), Ok(3.0f64.to_bits().into()));
        assert!(matches!(eval(OpCode::CPUI_FLOAT_ADD, 2, &[(0, 2), (0, 2)]), Err(Halt::Unsupported(_))));
        assert!(matches!(eval(OpCode::CPUI_FLOAT_ADD, 16, &[(0, 16), (0, 16)]), Err(Halt::Unsupported(_))));
        assert!(matches!(eval(OpCode::CPUI_INT_DIV, 4, &[(1, 4), (0, 4)]), Err(Halt::Trap(_))));
        assert!(matches!(eval(OpCode::CPUI_INT_DIV, 16, &[(1, 16), (0, 16)]), Err(Halt::Trap(_))));
    }
}
