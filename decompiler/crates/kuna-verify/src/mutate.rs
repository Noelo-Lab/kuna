//! Mutation testing of the check itself: plant one semantic fault in kuna's
//! side of a function that agrees, and see whether the check reports it.
//!
//! A mutant changes one op of the compiled decompiled side (`program`) or one
//! call site's target, which is what the printed C names a direct callee by.
//! The machine side is untouched. A mutant whose op no run reaches cannot be
//! seen by any check, so recall counts only the mutants a run reached.

use std::time::Instant;

use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

use crate::calls::Calls;
use crate::check::Options;
use crate::program::{Kind, Operand, Program};
use crate::report::Verdict;
use crate::state::Env;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operator {
    /// An arithmetic, logic, comparison or extension op becomes a near one.
    Opcode,
    /// A constant input's lowest bit is flipped.
    Constant,
    /// The inputs of an op that does not commute trade places.
    Swap,
    /// A conditional branch goes the other way.
    Branch,
    /// A store does nothing.
    DropStore,
    /// A direct call goes to another function.
    CallTarget,
    /// A call loses its last argument: a recovered prototype loses it too, as
    /// when kuna recovers one parameter too few; a declared one keeps it.
    DropArg,
    /// A call that returns is said not to. This is kuna's claim about the call,
    /// which both sides read; the machine side has to find it false itself.
    NoReturn,
}

pub const OPERATORS: [Operator; 8] = [
    Operator::Opcode,
    Operator::Constant,
    Operator::Swap,
    Operator::Branch,
    Operator::DropStore,
    Operator::CallTarget,
    Operator::DropArg,
    Operator::NoReturn,
];

impl Operator {
    pub fn name(self) -> &'static str {
        match self {
            Operator::Opcode => "opcode",
            Operator::Constant => "constant",
            Operator::Swap => "swap",
            Operator::Branch => "branch",
            Operator::DropStore => "drop-store",
            Operator::CallTarget => "call-target",
            Operator::DropArg => "drop-arg",
            Operator::NoReturn => "noreturn",
        }
    }
}

/// One mutant: the op at `op` in block `block`, changed by `operator`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mutation {
    pub block: usize,
    pub op: usize,
    pub operator: Operator,
}

fn near(opc: OpCode) -> Option<OpCode> {
    use OpCode::*;
    Some(match opc {
        CPUI_INT_ADD => CPUI_INT_SUB,
        CPUI_INT_SUB => CPUI_INT_ADD,
        CPUI_INT_AND => CPUI_INT_OR,
        CPUI_INT_OR => CPUI_INT_AND,
        CPUI_INT_XOR => CPUI_INT_OR,
        CPUI_INT_MULT => CPUI_INT_ADD,
        CPUI_INT_LESS => CPUI_INT_SLESS,
        CPUI_INT_SLESS => CPUI_INT_LESS,
        CPUI_INT_LESSEQUAL => CPUI_INT_LESS,
        CPUI_INT_SLESSEQUAL => CPUI_INT_SLESS,
        CPUI_INT_EQUAL => CPUI_INT_NOTEQUAL,
        CPUI_INT_NOTEQUAL => CPUI_INT_EQUAL,
        CPUI_INT_RIGHT => CPUI_INT_SRIGHT,
        CPUI_INT_SRIGHT => CPUI_INT_RIGHT,
        CPUI_INT_LEFT => CPUI_INT_RIGHT,
        CPUI_INT_ZEXT => CPUI_INT_SEXT,
        CPUI_INT_SEXT => CPUI_INT_ZEXT,
        CPUI_INT_DIV => CPUI_INT_SDIV,
        CPUI_INT_SDIV => CPUI_INT_DIV,
        CPUI_INT_REM => CPUI_INT_SREM,
        CPUI_INT_SREM => CPUI_INT_REM,
        _ => return None,
    })
}

fn ordered(opc: OpCode) -> bool {
    use OpCode::*;
    matches!(
        opc,
        CPUI_INT_SUB
            | CPUI_INT_LESS
            | CPUI_INT_SLESS
            | CPUI_INT_LESSEQUAL
            | CPUI_INT_SLESSEQUAL
            | CPUI_INT_DIV
            | CPUI_INT_SDIV
            | CPUI_INT_REM
            | CPUI_INT_SREM
    )
}

/// Every mutant `p` and `calls` allow, in program order.
pub fn mutants(p: &Program, calls: &Calls) -> Vec<Mutation> {
    let mut out = Vec::new();
    for (b, block) in p.blocks.iter().enumerate() {
        for (k, op) in block.ops.iter().enumerate() {
            let mut add = |operator| out.push(Mutation { block: b, op: k, operator });
            match &op.kind {
                Kind::Eval(opc) => {
                    if near(*opc).is_some() {
                        add(Operator::Opcode);
                    }
                    if op.ins.iter().any(|a| matches!(a.op, Operand::Const(_))) {
                        add(Operator::Constant);
                    }
                    if ordered(*opc) && op.ins.len() == 2 && op.ins[0].size == op.ins[1].size {
                        add(Operator::Swap);
                    }
                }
                Kind::CBranch { .. } => add(Operator::Branch),
                Kind::Store(_) => add(Operator::DropStore),
                Kind::Call { direct: Some(_) } => {
                    if calls.site(op.at).target.is_some() {
                        add(Operator::CallTarget);
                    }
                    if !calls.site(op.at).noreturn {
                        add(Operator::NoReturn);
                    }
                    if op.ins.len() > 1 {
                        add(Operator::DropArg);
                    }
                }
                Kind::Call { direct: None } if op.ins.len() > 1 => add(Operator::DropArg),
                _ => {}
            }
        }
    }
    out
}

/// Apply `m` to `p` and `calls`; a description of what changed, or `None`
/// when the mutant does not apply.
pub fn apply(p: &mut Program, calls: &mut Calls, env: &Env, m: Mutation) -> Option<String> {
    let op = p.blocks.get_mut(m.block)?.ops.get_mut(m.op)?;
    let at = op.at;
    let what = match (m.operator, &mut op.kind) {
        (Operator::Opcode, Kind::Eval(opc)) => {
            let to = near(*opc)?;
            let s = format!("{opc:?} -> {to:?}");
            *opc = to;
            s
        }
        (Operator::Constant, Kind::Eval(_)) => {
            let a = op.ins.iter_mut().find(|a| matches!(a.op, Operand::Const(_)))?;
            let Operand::Const(c) = &mut a.op else { return None };
            let was = *c;
            *c ^= 1;
            format!("constant {was:#x} -> {:#x}", *c)
        }
        (Operator::Swap, Kind::Eval(opc)) => {
            let s = format!("{opc:?} inputs swapped");
            op.ins.swap(0, 1);
            s
        }
        (Operator::Branch, Kind::CBranch { flip }) => {
            *flip = !*flip;
            "branch inverted".to_string()
        }
        (Operator::DropStore, Kind::Store(_)) => {
            op.kind = Kind::Nop;
            "store dropped".to_string()
        }
        (Operator::CallTarget, Kind::Call { .. }) => {
            let site = calls.by_addr.get_mut(&at)?;
            let was = site.target?;
            let to = env.another_function(was)?;
            site.target = Some(to);
            let place = |t: u64| if env.is_code(t) { "code" } else if env.code_mapped(t) { "mapped" } else { "unmapped" };
            let resolved = if site.resolved { ", resolved site" } else { "" };
            format!("call target {was:#x} -> {to:#x} [{} -> {}{resolved}]", place(was), place(to))
        }
        (Operator::DropArg, Kind::Call { .. }) => {
            let k = op.ins.len() - 1;
            let site = calls.by_addr.get_mut(&at)?;
            let class = match site.args.get(k - 1) {
                Some(Some(_)) => "register",
                Some(None) => "not in a register",
                None => "beyond the prototype",
            };
            op.ins.pop();
            site.args.truncate(k - 1);
            site.input_storage.truncate(k - 1);
            let proto = if site.locked {
                "declared prototype kept"
            } else {
                site.params = site.params.min(k - 1);
                "recovered prototype shrunk"
            };
            format!("last call argument dropped [{class}, {proto}]")
        }
        (Operator::NoReturn, Kind::Call { .. }) => {
            let site = calls.by_addr.get_mut(&at)?;
            site.noreturn = true;
            let place = site.target.map_or("indirect", |t| if env.code_mapped(t) { "callee in the image" } else { "callee outside the image" });
            format!("call said not to return [{place}]")
        }
        _ => return None,
    };
    p.watch = Some((m.block, m.op));
    Some(format!("{what} at {at:#x}"))
}

/// How one mutant fared.
#[derive(Clone, Debug)]
pub struct Outcome {
    pub function: String,
    pub operator: Operator,
    pub what: String,
    pub verdict: Verdict,
    /// A run reached the mutated op.
    pub reached: bool,
    pub ms: f64,
}

impl Outcome {
    pub fn killed(&self) -> bool {
        matches!(self.verdict, Verdict::Differs { .. })
    }
}

/// Up to `per_function` mutants of `fd`, taking each operator in turn so every
/// kind is tried; none when the unmutated function does not agree.
pub fn mutate_function(prog: &ConsoleProgram, fd: &Funcdata, opts: &Options, per_function: usize) -> Vec<Outcome> {
    let base = crate::check::check_function(prog, fd, None, opts);
    if !matches!(base.verdict, Verdict::Agrees(_)) {
        return Vec::new();
    }
    let Ok(env) = Env::new(prog, opts.seed) else { return Vec::new() };
    let calls = crate::calls::sites(fd, &env);
    let program = crate::program::compile(prog, fd, &calls);
    let all = mutants(&program, &calls);
    let mut picked = Vec::new();
    let mut by_kind: Vec<Vec<Mutation>> = OPERATORS.iter().map(|o| all.iter().filter(|m| m.operator == *o).copied().collect()).collect();
    for v in &mut by_kind {
        let n = v.len();
        if n > 1 {
            let start = (opts.seed as usize ^ fd.get_address().get_offset() as usize) % n;
            v.rotate_left(start);
        }
    }
    let mut round = 0;
    while picked.len() < per_function && by_kind.iter().any(|v| round < v.len()) {
        for v in &by_kind {
            if let Some(&m) = v.get(round) {
                if picked.len() < per_function {
                    picked.push(m);
                }
            }
        }
        round += 1;
    }
    picked
        .into_iter()
        .filter_map(|m| {
            let started = Instant::now();
            let (report, what, reached) = crate::check::check_mutant(prog, fd, opts, m)?;
            Some(Outcome {
                function: fd.get_name().to_string(),
                operator: m.operator,
                what,
                verdict: report.verdict,
                reached,
                ms: started.elapsed().as_secs_f64() * 1e3,
            })
        })
        .collect()
}
