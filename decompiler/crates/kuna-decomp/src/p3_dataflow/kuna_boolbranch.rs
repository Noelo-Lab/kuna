//! (kuna `boolbranch`) Keep a branch instruction that selects the constants 1 and 0.
//!
//! `RuleConditionalMove` folds a `MULTIEQUAL` of `1` and `0` under a `CBRANCH`
//! into the branch's condition (`v = c`). The fold is right for conditional
//! execution: an ARM `ite ne; movne r0,#1; moveq r0,#0`, an x86 `cmovne` or an
//! AArch64 `csel` lifts to a `CBRANCH` that skips the rest of its own instruction,
//! so the diamond exists only in the p-code and the author wrote `v = c`. It is
//! wrong for a branch instruction whose two arms are other instructions: an
//! unoptimized `if (c) return 1; return 0;` is a `jne` and two `mov` blocks, and
//! folding it prints `return c;`, dropping an `if` the author wrote.
//!
//! The two are told apart by instruction address. A conditional-execution guard
//! falls through into the rest of its own instruction, so one arm block starts at
//! the guard's address (or still holds an op of it); a branch instruction ends
//! its instruction, so both arms start at other instructions. The block's start
//! is the evidence that survives `ActionConditionalExe`, which merges an ARM `ite`
//! pair into one diamond and may re-create the arms' COPYs at the second
//! instruction's address.
//!
//! Two kinds of branch instruction still fold, because the compiler wrote them for
//! a source with no `if`. A jump on the unordered flag of a float comparison: GCC
//! materializes a float `!=` as `mov $1,%edi; ucomisd; jp 1f; cmove %eax,%edi`.
//! And a flag the compiler sets with a jump and only tests: GCC at `-O0` computes
//! `__builtin_add_overflow` as `mov $0,%ecx; add; jno 1f; mov $1,%ecx; 1: test
//! %cl,%cl; je`, which kept would print `v = 0; if (SCARRY8(a,b)) v = 1; if (v)`.
//! Such a select has arms that assign the constant and nothing else, and its
//! value reaches nothing but branches and boolean operations. A flag the author
//! wrote usually does something else in its arm (`set = 1; noeval++;`) or is used
//! as a value, and keeps its branch.

use kuna_base::error::{KunaError, KunaResult};
use kuna_num::opcodes::OpCode;

use crate::block::block_get_start;
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Most varnodes the use walk visits from one select.
const USE_WALK_LIMIT: usize = 32;

/// Should the 0/1 select `op` under `cbranch`, ending `root`, keep its branch?
/// `arms` are the select's two input blocks; an arm equal to `root` holds no
/// guarded op.
pub(super) fn keeps_branch(data: &Funcdata, op: OpId, cbranch: OpId, root: BlockId, arms: [BlockId; 2]) -> bool {
    if !is_branch_instruction(data, cbranch, root, arms) || branches_on_nan(data, cbranch) {
        return false;
    }
    let out = data.obank().get(op).and_then(|o| o.get_out());
    !(arms_only_assign(data, root, arms) && out.is_some_and(|vn| only_tested(data, vn)))
}

/// Is `cbranch` a branch instruction rather than the guard of a conditionally
/// executed instruction?
pub(super) fn is_branch_instruction(data: &Funcdata, cbranch: OpId, root: BlockId, arms: [BlockId; 2]) -> bool {
    let Some(addr) = data.obank().get(cbranch).map(|o| o.get_addr().clone()) else {
        return false;
    };
    !arms.into_iter().filter(|&arm| arm != root).any(|arm| {
        block_get_start(&data.bblocks_ref().arena, arm) == addr
            || data.bb_ops(arm).into_iter().any(|op| data.obank().get(op).is_some_and(|o| *o.get_addr() == addr))
    })
}

/// Does `cbranch` jump on the unordered result of a float comparison (`NAN(x)`,
/// the x86 `jp`)? A compiler emits that jump to materialize a float `!=` or
/// NaN test as 0 or 1 (`mov $1,%edi; ucomisd; jp 1f; cmove %eax,%edi`).
pub(super) fn branches_on_nan(data: &Funcdata, cbranch: OpId) -> bool {
    let mut vn = data.obank().get(cbranch).and_then(|o| o.get_in(1));
    for _ in 0..2 {
        let Some(def) = vn.and_then(|v| data.vbank().get(v)).and_then(|v| v.get_def()) else { return false };
        let Some(o) = data.obank().get(def) else { return false };
        match o.get_opcode().map(|t| t.get_opcode()) {
            Some(OpCode::CPUI_FLOAT_NAN) => return true,
            Some(OpCode::CPUI_BOOL_NEGATE) => vn = o.get_in(0),
            _ => return false,
        }
    }
    false
}

/// Do the arms other than `root` hold nothing but constant COPYs and branches?
pub(super) fn arms_only_assign(data: &Funcdata, root: BlockId, arms: [BlockId; 2]) -> bool {
    arms.into_iter().filter(|&arm| arm != root).all(|arm| {
        data.bb_ops(arm).into_iter().all(|op| {
            let Some(o) = data.obank().get(op) else { return true };
            match o.get_opcode().map(|t| t.get_opcode()) {
                Some(OpCode::CPUI_BRANCH) => true,
                Some(OpCode::CPUI_COPY) => o.get_in(0).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_constant()),
                _ => false,
            }
        })
    })
}

/// Does `out` reach, through copies, extensions, truncations and joins, at least
/// one branch or boolean operation and nothing else?
pub(super) fn only_tested(data: &Funcdata, out: VarnodeId) -> bool {
    let mut work = vec![out];
    let mut seen = vec![out];
    let mut tested = false;
    while let Some(vn) = work.pop() {
        for op in data.descend_snapshot(vn) {
            let Some(o) = data.obank().get(op) else { continue };
            match o.get_opcode().map(|t| t.get_opcode()) {
                Some(
                    OpCode::CPUI_CBRANCH
                    | OpCode::CPUI_BOOL_NEGATE
                    | OpCode::CPUI_BOOL_AND
                    | OpCode::CPUI_BOOL_OR
                    | OpCode::CPUI_BOOL_XOR
                    | OpCode::CPUI_INT_EQUAL
                    | OpCode::CPUI_INT_NOTEQUAL,
                ) => tested = true,
                Some(
                    OpCode::CPUI_COPY
                    | OpCode::CPUI_INT_ZEXT
                    | OpCode::CPUI_INT_SEXT
                    | OpCode::CPUI_SUBPIECE
                    | OpCode::CPUI_CAST
                    | OpCode::CPUI_MULTIEQUAL,
                ) => {
                    let Some(next) = o.get_out() else { continue };
                    if seen.contains(&next) {
                        continue;
                    }
                    if seen.len() >= USE_WALK_LIMIT {
                        return false;
                    }
                    seen.push(next);
                    work.push(next);
                }
                _ => return false,
            }
        }
    }
    tested
}

/// Parse `fold|keep|return` into the `keep` flag, the `return` flag
/// ([`crate::p8_structure::kuna_boolret`]) and a confirmation message.
pub fn parse_bool_branch(p1: &str) -> KunaResult<(bool, bool, String)> {
    match p1 {
        "return" => Ok((
            true,
            true,
            "0/1 selects under a branch instruction keep the branch; a function that only tests a short-circuit condition returns it"
                .to_string(),
        )),
        "keep" => Ok((true, false, "0/1 selects under a branch instruction keep the branch".to_string())),
        "fold" => Ok((false, false, "0/1 selects fold into the branch condition".to_string())),
        _ => Err(KunaError::parse(format!("Unknown boolbranch value: {p1} (expected fold, keep or return)"))),
    }
}

#[cfg(test)]
#[path = "kuna_boolbranch/tests.rs"]
mod tests;
