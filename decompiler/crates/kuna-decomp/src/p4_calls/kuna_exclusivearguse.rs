//! P4 mutually-exclusive-path tolerance in input-trial scoring — the
//! `exclusivearguse` decision point.
//!
//! # The gap
//!
//! `Funcdata::only_op_use` (C++ `Funcdata::onlyOpUse`, `funcdata_varnode.cc:1851`)
//! walks every descendant of the Varnode reaching a CALL in a candidate argument
//! register and asks whether the value is used *only* as that call's argument. A
//! `CPUI_LOAD` or `CPUI_STORE` among those descendants rejects the trial outright,
//! and `FuncCallSpecs::check_input_trial_use` turns that into `mark_inactive` — an
//! inactive trial is also CHECKED, so it is never re-scored and the argument is
//! gone, taking every argument positioned behind it with it.
//!
//! The rejection asks the right question of the wrong set of ops. A container
//! `append` compiled with the one-element fast path inlined and the grow path left
//! as a call sets its arguments up *before* the capacity test, because both arms
//! need them:
//!
//! ```text
//!   mov   rdx,[rbx+0x8]        ; the write cursor
//!   lea   r8,[rsi+rdi]         ; the source byte
//!   cmp   rdx,[rbx+0x10]
//!   jz    slow
//!   movzx eax,[r8]             ; fast path: LOAD through r8
//!   mov   [rdx],al             ;            STORE through rdx
//!   inc   qword [rbx+0x8]
//!   jmp   done
//! slow:
//!   mov   rcx,rbx
//!   call  append_slow          ; <- the trials being scored
//! ```
//!
//! `rdx` and `r8` reach the call unmodified, so the Varnodes scored there are the
//! same ones the fast path dereferences. The walk finds the `STORE` and the `LOAD`
//! and both trials go inactive: `append_slow(container, n, p)` renders
//! `append_slow(container)`. kuna's own recovery of the callee disagrees with
//! itself here — decompiled on its own it comes out
//! `char *append_slow(long long *,long long,char *)`, and its prologue reads both
//! dropped registers.
//!
//! The competing use cannot compete. The fast path and the call are the two arms of
//! one branch: no execution path contains both, so on every execution that reaches
//! the call the value's only use *is* the call.
//!
//! This is not the shape the `calleearity` family already covers. All three of
//! [`calleearity`](crate::p4_calls::kuna_calleearity),
//! [`calleearityfwd`](crate::p4_calls::kuna_calleearityfwd) and
//! [`calleearitybody`](crate::p4_calls::kuna_calleearitybody) were built for a
//! `CBRANCH` use in a block that *dominates* the call (`test rcx,rcx; jz; call`),
//! and their documentation is explicit that relaxing `only_op_use` for that shape
//! would fabricate an argument at every guarded call there is. A dominating block
//! always co-executes with the call, so it is never mutually exclusive with it and
//! this rule declines it — the two mechanisms do not overlap in either direction.
//!
//! # The rule
//!
//! Skip a `CPUI_LOAD`/`CPUI_STORE` descendant when all four hold:
//!
//! * the matched op is a `CALL`/`CALLIND`, so this is a caller-side input trial
//!   and not the function's own output trial;
//! * the Varnode is the *address* operand of the access, not the stored datum. A
//!   dereference says the value is a pointer, which is a plausible argument; a
//!   frame store of the value is the caller-save spill shape and belongs to
//!   [`spillargtrial`](crate::p4_calls::kuna_spillargtrial), which owns the
//!   `STORE` value slot;
//! * the Varnode is **defined in a block that both the access's block and the
//!   call's block are immediate successors of** — the value was produced for the
//!   branch that separates them; and
//! * the access and the call sit in distinct basic blocks and neither is reachable
//!   from the other in the CFG.
//!
//! The last condition is the soundness of the rule. An execution path is one walk
//! from the entry block, so a path holding both blocks would make the later one
//! reachable from the earlier; mutual unreachability therefore means no path holds
//! both. It is also conservative in the presence of loops: two arms of an `if`
//! inside a loop body *do* reach each other through the back edge, and the rule
//! declines them.
//!
//! The third condition is what keeps it from inventing arguments, and it was
//! written against a measured false positive rather than out of caution. Without
//! it, a register the compiler picked as a long-lived scratch copy qualifies:
//! phantomgate.exe's `random_device` constructor opens `mov r8,rcx` and then
//! dereferences `r8` on the arms that succeed, so every `throw` call on an arm
//! that fails gained a third argument the disassembly does not pass. The witness
//! shape is the opposite of that — the pointers are computed in the two
//! instructions before the test — and requiring the definition to sit in the very
//! block that branches says exactly that: the value was set up *for* this branch,
//! not merely still live when it was taken.
//!
//! Every other descendant keeps the upstream treatment, `BRANCH`/`CBRANCH`/
//! `BRANCHIND` included.

use std::collections::HashSet;

use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// The address operand of a `CPUI_LOAD`/`CPUI_STORE`. Slot 0 is the space id.
const ADDRESS_SLOT: i32 = 1;

/// Cap on blocks visited by one reachability walk; exceeding it declines (keeps
/// the upstream rejection) rather than paying for a big CFG at every trial.
const MAX_VISIT: usize = 4096;

/// Whether the `CPUI_LOAD`/`CPUI_STORE` at `op` may be skipped while scoring the
/// input trial matched at `opmatch`. Off restores the upstream rejection.
pub fn access_cannot_reach_call(
    data: &Funcdata,
    opmatch: OpId,
    op: OpId,
    vn: VarnodeId,
) -> bool {
    if !data.get_arch().exclusive_arg_use {
        return false;
    }
    if !matches!(
        data.obank().get(opmatch).map(|o| o.code()),
        Some(OpCode::CPUI_CALL) | Some(OpCode::CPUI_CALLIND)
    ) {
        return false;
    }
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    if !matches!(o.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE) {
        return false;
    }
    if o.get_in(ADDRESS_SLOT) != Some(vn) {
        return false;
    }
    let (Some(useblk), Some(matchblk)) = (o.get_parent(), data.obank().get(opmatch).and_then(|m| m.get_parent()))
    else {
        return false;
    };
    if useblk == matchblk {
        return false;
    }
    let Some(defblk) = def_block(data, vn) else {
        return false;
    };
    if !is_predecessor(data, defblk, useblk) || !is_predecessor(data, defblk, matchblk) {
        return false;
    }
    !reaches(data, useblk, matchblk) && !reaches(data, matchblk, useblk)
}

/// The block holding `vn`'s defining op, if it has one. A function input, a
/// constant and a free Varnode all answer `None` and decline the rule.
fn def_block(data: &Funcdata, vn: VarnodeId) -> Option<BlockId> {
    let def = data.vbank().get(vn)?.get_def()?;
    data.obank().get(def)?.get_parent()
}

/// Is `pred` an immediate predecessor of `bl`?
fn is_predecessor(data: &Funcdata, pred: BlockId, bl: BlockId) -> bool {
    let b = data.bblocks_ref().block(bl);
    (0..b.size_in()).any(|i| b.get_in(i) == pred)
}

/// Is `target` reachable from `from` by following CFG out-edges? Declines (true,
/// the conservative answer for the caller) once [`MAX_VISIT`] blocks are seen.
fn reaches(data: &Funcdata, from: BlockId, target: BlockId) -> bool {
    let graph = data.bblocks_ref();
    let mut seen: HashSet<BlockId> = HashSet::new();
    let mut stack: Vec<BlockId> = vec![from];
    while let Some(bl) = stack.pop() {
        if !seen.insert(bl) {
            continue;
        }
        if seen.len() > MAX_VISIT {
            return true;
        }
        let b = graph.block(bl);
        for i in 0..b.size_out() {
            let next = b.get_out(i);
            if next == target {
                return true;
            }
            stack.push(next);
        }
    }
    false
}

#[cfg(test)]
#[path = "kuna_exclusivearguse/tests.rs"]
mod tests;
