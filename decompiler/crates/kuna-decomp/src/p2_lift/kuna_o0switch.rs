//! (kuna `loweredswitch auto`) Whether an unoptimized function's compare chain
//! was written as a `switch` or as an `if`/`else if` chain.
//!
//! An unoptimizing gcc lowers a small `switch` to a run of compares laid out
//! one after another: each test jumps to its case on a match and falls
//! through to the next test, and the case bodies follow the whole run. An
//! `if (x == K)` falls through into its own body on a match and jumps away,
//! to the next `else if`, on a mismatch:
//!
//! ```text
//! switch (x)          cmp x,1 ; je case1 ; cmp x,2 ; je case2 ; jmp default
//! if (x == 1) ...     cmp x,1 ; jne next ; <body> ; next: cmp x,2 ; jne ...
//! ```
//!
//! So every equality test of a switch jumps on its match, except at most the
//! last one of the run, whose case body can be the block laid out right after
//! it, and an if/else-if chain has tests that fall through on a match before
//! its last. A switch over a value that needs a temporary (a `char` promoted
//! to `int`, a field, a global, a computed value) also compares one loaded
//! value in every test, where an if-chain loads the variable again for each;
//! a switch over a plain `int` variable compares its frame slot every time,
//! as an if-chain does. clang tests a switch the same way (`je case; jmp
//! next`).
//!
//! [`record`] reads each conditional branch at the first heritage pass of a
//! function [`crate::p3_dataflow::kuna_unoptimized`] calls unoptimized, before
//! simplification normalizes branch senses and propagates the loads away;
//! [`verdict`] reads the tests of a cascade the lowered-switch recogniser
//! found.

use std::collections::BTreeMap;

use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::PcodeOp;

/// Longest chain of ops followed back from a branch condition to the compare.
const SLICE_DEPTH: u32 = 16;

/// How one conditional branch was compiled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Test {
    /// For an equality test against a constant: whether the branch is taken
    /// when the values are equal.
    pub jumps_on_match: Option<bool>,
    /// The instruction that produced the compared value: a load, or the
    /// computation a register carries from it.
    pub root: Option<u64>,
}

/// The conditional branches of one function, by instruction address.
#[derive(Debug, Clone, Default)]
pub struct Record {
    tests: BTreeMap<u64, Test>,
}

/// The source form a cascade's tests were compiled from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    Switch,
    IfChain,
}

/// Read every conditional branch; once, at the first heritage pass, in a
/// function the detector calls unoptimized.
pub(crate) fn record(fd: &mut Funcdata) {
    if fd.get_heritage_pass() != 1 || fd.kuna_o0switch.is_some() {
        return;
    }
    let arch = fd.get_arch();
    if !arch.recover_lowered_switch || !arch.lowered_switch_compiled {
        return;
    }
    if !crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd) {
        return;
    }
    let mut rec = Record::default();
    for i in 0..fd.bblocks_get_size() {
        let b = fd.bblocks_get_block(i);
        let Some(op) = fd.bb_op_tail(b).and_then(|o| fd.obank().get(o)) else { continue };
        if op.code() != OpCode::CPUI_CBRANCH {
            continue;
        }
        let Some(cond) = op.get_in(1) else { continue };
        let test = Test { jumps_on_match: jumps_on_match(fd, cond), root: root(fd, cond, b) };
        rec.tests.insert(op.get_addr().get_offset(), test);
    }
    fd.kuna_o0switch = Some(Box::new(rec));
}

/// The form of a cascade whose tests branch at `addrs`: a switch when every
/// equality test but the last (by address) jumps on its match and the last
/// does too, or there are three or more, or one load feeds every test; an
/// if-chain when a test before the last falls through on its match; `None`
/// outside a recorded function, and for two tests that only the switch's
/// last-test layout explains.
pub fn verdict(fd: &Funcdata, addrs: impl IntoIterator<Item = u64>) -> Option<Form> {
    let rec = fd.kuna_o0switch.as_deref()?;
    form(addrs.into_iter().map(|a| rec.tests.get(&a).map(|t| (a, *t))).collect::<Option<Vec<_>>>()?)
}

fn form(tests: Vec<(u64, Test)>) -> Option<Form> {
    let mut eqs: Vec<(u64, bool)> = tests.iter().filter_map(|(a, t)| t.jumps_on_match.map(|j| (*a, j))).collect();
    eqs.sort_unstable();
    let n = eqs.len();
    if n < 2 {
        return None;
    }
    if eqs[..n - 1].iter().any(|&(_, j)| !j) {
        return Some(Form::IfChain);
    }
    let one_load = tests[0].1.root.is_some() && tests.iter().all(|(_, t)| t.root == tests[0].1.root);
    (eqs[n - 1].1 || n >= 3 || one_load).then_some(Form::Switch)
}

fn def_op(fd: &Funcdata, vn: VarnodeId) -> Option<&PcodeOp> {
    fd.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| fd.obank().get(d))
}

fn constant(fd: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let v = fd.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset())
}

fn is_compare(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_BOOL_XOR
    )
}

/// Is `vn` a flag: a one-byte value a comparison computed, or a copy of one?
fn is_flag(fd: &Funcdata, vn: VarnodeId) -> bool {
    let mut vn = vn;
    for _ in 0..4 {
        if fd.vbank().get(vn).is_none_or(|v| v.get_size() != 1) {
            return false;
        }
        let Some(op) = def_op(fd, vn) else { return false };
        if op.code() != OpCode::CPUI_COPY {
            return is_compare(op.code());
        }
        let Some(x) = op.get_in(0) else { return false };
        vn = x;
    }
    false
}

/// Whether a branch on `cond` is taken when an equality against a constant
/// holds (`x == K`, or the flag form `x - K == 0`, possibly negated or read
/// back through a flag test); `None` for any other condition.
fn jumps_on_match(fd: &Funcdata, cond: VarnodeId) -> Option<bool> {
    let mut cond = cond;
    let mut negated = false;
    for _ in 0..6 {
        let op = def_op(fd, cond)?;
        match op.code() {
            OpCode::CPUI_BOOL_NEGATE => {
                negated = !negated;
                cond = op.get_in(0)?;
            }
            OpCode::CPUI_COPY => cond = op.get_in(0)?,
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                let equal = (op.code() == OpCode::CPUI_INT_EQUAL) != negated;
                let (a, b) = (op.get_in(0)?, op.get_in(1)?);
                let x = match (constant(fd, a), constant(fd, b)) {
                    (None, Some(0)) => a,
                    (Some(0), None) => b,
                    (None, Some(_)) | (Some(_), None) => return Some(equal),
                    _ => return None,
                };
                if is_flag(fd, x) {
                    negated = equal;
                    cond = x;
                    continue;
                }
                return Some(equal);
            }
            _ => return None,
        }
    }
    None
}

fn is_move(code: OpCode) -> bool {
    matches!(code, OpCode::CPUI_COPY | OpCode::CPUI_SUBPIECE | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT)
}

/// The value a test in `block` compares: the non-constant side of an ordering
/// or equality compare, or of the subtraction a flag compare tests against
/// zero, found among the block's own ops.
fn compared(fd: &Funcdata, cond: VarnodeId, block: BlockId) -> Option<VarnodeId> {
    let mut stack = vec![(cond, 0u32)];
    let mut seen: Vec<VarnodeId> = Vec::new();
    while let Some((vn, depth)) = stack.pop() {
        if depth > SLICE_DEPTH || seen.contains(&vn) {
            continue;
        }
        seen.push(vn);
        let Some(op) = def_op(fd, vn) else { continue };
        if op.get_parent() != Some(block) {
            continue;
        }
        match op.code() {
            OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_SBORROW => {
                let (a, b) = (op.get_in(0)?, op.get_in(1)?);
                let flag_test = matches!(op.code(), OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL);
                match (constant(fd, a), constant(fd, b)) {
                    (None, Some(0)) if flag_test => stack.push((a, depth + 1)),
                    (None, Some(_)) => return Some(a),
                    (Some(_), None) => return Some(b),
                    _ => {}
                }
            }
            OpCode::CPUI_LOAD | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_MULTIEQUAL => {}
            _ => {
                for i in 0..op.num_input() {
                    if let Some(x) = op.get_in(i) {
                        stack.push((x, depth + 1));
                    }
                }
            }
        }
    }
    None
}

/// The instruction that produced the value a test compares, through register
/// moves: the load (or memory copy) it read, or the op that computed it.
fn root(fd: &Funcdata, cond: VarnodeId, block: BlockId) -> Option<u64> {
    let mut vn = compared(fd, cond, block)?;
    for _ in 0..8 {
        let op = def_op(fd, vn)?;
        let next = match op.code() {
            c if is_move(c) => op.get_in(0).filter(|&x| def_op(fd, x).is_some()),
            OpCode::CPUI_PIECE => op.get_in(1),
            _ => None,
        };
        match next {
            Some(x) => vn = x,
            None => return Some(op.get_addr().get_offset()),
        }
    }
    None
}

#[cfg(test)]
#[path = "kuna_o0switch/tests.rs"]
mod tests;
