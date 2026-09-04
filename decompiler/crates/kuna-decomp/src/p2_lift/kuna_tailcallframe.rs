//! (kuna) `kuna_tailcallframe` — recover an `-O2` tail jump whose callee was
//! never discovered, from the stack-frame teardown that precedes it (S2 flow
//! classification).  Sibling of [`kuna_tailcalljump`](crate::kuna_tailcalljump),
//! which handles the case where the branch target *is* a known function entry.
//!
//! ## The structural gap
//!
//! [`kuna_tailcalljump`](crate::kuna_tailcalljump) recognises `jmp X` as a tail
//! call only when `query_call(X)` resolves — X must already be a function in the
//! symbol table.  Every discovery oracle kuna has reaches a function either from
//! a symbol, from an unwind record, or from a direct `call` the recursive-descent
//! walk arrived at.  A callee that is reached **only** through a code pointer in
//! initialized data satisfies none of them: the walk never enters the callback,
//! so it never sees the callback's `call`, so neither the callback nor anything
//! it calls becomes a function.
//!
//! Decompiling such a callback by address then walks its tail `jmp` as ordinary
//! intraprocedural flow and decodes the whole callee into it.  On the round-2
//! RE-friction witness (a stripped Wayland/xkb PIE, keyboard callback at
//! `0x6500`) the tail `jmp 0x4610` at `0x66e2` annexed the renderer: 1,555 lines
//! of C for a 500-byte function, with the renderer's UI strings and its callees
//! emitted inside the keyboard handler.  The same function's *other* reference to
//! `0x4610` is a real `call` a few blocks later, so the callee is emitted twice —
//! once as a named call, once inlined.
//!
//! ## The signal this module owns
//!
//! A tail jump is the last instruction of the caller: the compiler tears the
//! frame down first, so `jmp` executes with the stack pointer back at its
//! entry value and the return address on top of the stack, exactly as `ret`
//! would find it.  That teardown is visible in the raw p-code before any
//! dataflow analysis has run, and it is what tells a tail jump apart from an
//! ordinary intraprocedural jump — which never restores the frame, because the
//! code it jumps to still needs it.
//!
//! [`kuna_is_frame_teardown_tail_call`] measures two constant stack-pointer
//! deltas over the already-decoded raw p-code and requires that they cancel:
//!
//!   * the **prologue** delta, accumulated forward from the function's entry
//!     address over the straight-line run that establishes the frame
//!     (`push rbx` / `sub rsp,0x10` → `-0x18`);
//!   * the **epilogue** delta, accumulated backward from the branch over the
//!     straight-line run that ends at it (`add rsp,0x10` / `pop rbx` → `+0x18`).
//!
//! Both scans stop at the first control-flow op, so neither can leave the block
//! it starts in, and both stop at a stack-pointer write that is not
//! `SP = SP +/- <const>` (a `leave`-style `SP = FP` restore is not modelled and
//! declines).  The rule fires only when the epilogue is a strictly positive
//! teardown that exactly undoes the prologue, which is why a frameless leaf
//! (`prologue == 0`) can never match: with no frame to tear down there is no
//! evidence, and an unconditional intraprocedural jump would be
//! indistinguishable from a tail call.
//!
//! ## What this rule cannot distinguish
//!
//! The evidence is the frame, not the function bound, and kuna's
//! `FunctionSymbol` is an entry address with **no extent** (see
//! [`kuna_funcboundflow`](crate::kuna_funcboundflow)), so nothing here can ask
//! whether `dest` is still inside the caller.  A jump that tears the frame down
//! *completely* and then branches to a shared return sequence **inside the same
//! function** is therefore indistinguishable from a tail call, and is recovered
//! as one.  Measured, not assumed: the shape does not occur in optimizer output,
//! because a shared return sequence has to be shared *including* its teardown --
//! the jump is emitted part-way through the epilogue, so the epilogue delta is a
//! partial teardown and the exact-cancellation test rejects it (this is what
//! `partial_teardown_declines` pins).  gcc and clang at `-O1/-O2/-O3/-Os` all
//! emit `add rsp,0x68; jmp <shared tail>` against a `-0x70` prologue, and a
//! 26,458-function LLVM `-O2` corpus has 103 sites of the raw shape and **zero**
//! that fire.  Once the frame is fully gone the only thing left to jump to is
//! `ret` itself, which a compiler emits inline rather than branching to.  The
//! sound fix is to defer the decision until the flow work-stack drains and ask
//! whether the function decoded `dest` by another path; that is a change to the
//! flow walk's ordering, not to this predicate, and is deliberately not made
//! here.
//!
//! ## What this module owns
//!
//! The *decision* only.  The rewrite (`BRANCH` → `CPUI_CALL` + an artificial
//! `RETURN`) is the one `flow.rs` already drives for
//! [`kuna_tailcalljump`](crate::kuna_tailcalljump) at the `CPUI_BRANCH` arm of
//! `xref_control_flow`; this predicate is consulted second, so a target that is
//! a known function keeps the existing path and the existing warning text.

use kuna_base::address::{Address, SeqNum};
use kuna_base::marshal::ElementId;
use kuna_base::space::VarnodeStorage;
use kuna_base::types::uintm;
use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::funcdata::Funcdata;

/// Marshaling element `<tailcallframe>` (kuna).  ElementIds live in the 4000+
/// range; `scripts.repipe.counters --check` derives the next free id and fails
/// on a duplicate — do not read the high-water mark by hand.
pub const ELEM_TAILCALLFRAME: ElementId = ElementId::new("tailcallframe", 4139);

/// Largest byte distance between two consecutive decoded instruction addresses
/// that the scans will treat as fall-through adjacency.  x86-64's maximum
/// instruction length is 15 bytes and every other supported architecture is
/// shorter, so a wider gap means the neighbouring address in the op bank is not
/// the neighbouring *instruction* and the run has to end.
const MAX_INSTR_BYTES: u64 = 16;

/// Upper bound on instructions either scan will look at.  A prologue or an
/// epilogue is a handful of instructions; the bound keeps the predicate O(1) per
/// branch on a function with thousands of ops.
const MAX_SCAN_INSTRS: u32 = 24;

/// What one instruction does to the stack pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpEffect {
    /// No write to the stack pointer.
    None,
    /// `SP = SP + delta` with a literal `delta` (a `push`/`pop`/`add`/`sub`).
    Const(i64),
    /// The stack pointer is written, but not as a constant displacement of
    /// itself (`leave`, `mov rsp,rbx`, an alloca).  The scan cannot account for
    /// it and stops.
    Opaque,
}

/// Is `vn` exactly the stack-pointer register location?
fn is_stack_pointer(data: &Funcdata, vn: Option<crate::context::VarnodeId>, sp: &VarnodeStorage) -> bool {
    let Some(id) = vn else { return false };
    let Some(v) = data.vbank().get(id) else { return false };
    let Some(spspace) = sp.space.as_ref() else { return false };
    std::rc::Rc::ptr_eq(v.get_space(), spspace)
        && v.get_offset() == sp.offset
        && v.get_size() as u32 == sp.size
}

/// The constant behind `vn`, or `None` when it is not a literal.
fn const_of(data: &Funcdata, vn: Option<crate::context::VarnodeId>) -> Option<u64> {
    let v = data.vbank().get(vn?)?;
    v.is_constant().then(|| v.get_offset())
}

/// The stack-pointer effect of one raw p-code op.
fn sp_effect(data: &Funcdata, op: OpId, sp: &VarnodeStorage) -> SpEffect {
    let Some(o) = data.obank().get(op) else { return SpEffect::None };
    if !is_stack_pointer(data, o.get_out(), sp) {
        return SpEffect::None;
    }
    let opc = o.code();
    if opc != OpCode::CPUI_INT_ADD && opc != OpCode::CPUI_INT_SUB {
        return SpEffect::Opaque;
    }
    if !is_stack_pointer(data, o.get_in(0), sp) {
        return SpEffect::Opaque;
    }
    let Some(k) = const_of(data, o.get_in(1)) else { return SpEffect::Opaque };
    // Sign-extend the literal from the operand width before applying it, so a
    // `sub rsp,-8` style encoding accounts as a growth rather than as 2^64-8.
    let width = data
        .vbank()
        .get(o.get_in(1).expect("sp_effect: in1 present"))
        .map(|v| v.get_size())
        .unwrap_or(8);
    let signed = sign_extend(k, width);
    match opc {
        OpCode::CPUI_INT_ADD => SpEffect::Const(signed),
        _ => SpEffect::Const(signed.wrapping_neg()),
    }
}

/// Sign-extend the low `size` bytes of `val` into an `i64`.
fn sign_extend(val: u64, size: i32) -> i64 {
    if size <= 0 || size >= 8 {
        return val as i64;
    }
    let bits = (size as u32) * 8;
    let m = 1u64 << (bits - 1);
    (((val & ((1u64 << bits) - 1)) ^ m).wrapping_sub(m)) as i64
}

/// Are two instruction addresses in the same address space?
fn same_space(a: &Address, b: &Address) -> bool {
    match (a.get_space(), b.get_space()) {
        (Some(x), Some(y)) => std::rc::Rc::ptr_eq(x, y),
        _ => false,
    }
}

/// Does this op end the straight-line run (a control-flow transfer)?
fn is_flow_op(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    matches!(
        o.code(),
        OpCode::CPUI_BRANCH
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_RETURN
    )
}

/// What one whole machine instruction (every raw p-code op at one address) does
/// to the stack pointer and to the run.
struct GroupEffect {
    /// Net constant stack-pointer displacement across the instruction.
    delta: i64,
    /// The instruction transfers control, so it ends the straight-line run.
    /// Read BEFORE `delta` is used: an x86 `call` lowers to `SP = SP - 8` and a
    /// store of the return address *before* its `CPUI_CALL`, so counting the ops
    /// of a control-flow instruction would charge the run a phantom push.
    flow: bool,
    /// The instruction writes the stack pointer in a form the scan cannot
    /// account for (`leave`, an alloca), so no run through it is measurable.
    opaque: bool,
}

/// Summarize every raw p-code op at instruction address `at`.
fn group_effect(data: &Funcdata, at: &Address, sp: &VarnodeStorage) -> GroupEffect {
    let mut eff = GroupEffect { delta: 0, flow: false, opaque: false };
    for (_, id) in data.obank().iter_at(at) {
        if is_flow_op(data, id) {
            eff.flow = true;
        }
        match sp_effect(data, id, sp) {
            SpEffect::None => {}
            SpEffect::Const(d) => eff.delta = eff.delta.wrapping_add(d),
            SpEffect::Opaque => eff.opaque = true,
        }
    }
    eff
}

/// Is `next` the fall-through neighbour of `cur` in the direction `forward`?
fn adjacent(cur: &Address, next: &Address, forward: bool) -> bool {
    if !same_space(cur, next) {
        return false;
    }
    let (lo, hi) = if forward { (cur, next) } else { (next, cur) };
    hi.get_offset() > lo.get_offset() && hi.get_offset() - lo.get_offset() <= MAX_INSTR_BYTES
}

/// Net constant stack-pointer delta over the straight-line run that *starts* at
/// `entry` (the prologue), or `None` when the run cannot be accounted for.
fn prologue_delta(data: &Funcdata, entry: &Address, sp: &VarnodeStorage) -> Option<i64> {
    // Nothing decoded at the entry address: the caller asked about a Funcdata
    // whose entry is not this instruction stream.
    let first = data.obank().first_op_at_or_after(entry)?;
    if data.obank().get(first)?.get_addr() != entry {
        return None;
    }
    let mut delta: i64 = 0;
    let mut at = entry.clone();
    for _ in 0..MAX_SCAN_INSTRS {
        let eff = group_effect(data, &at, sp);
        if eff.flow {
            break;
        }
        if eff.opaque {
            return None;
        }
        delta = delta.wrapping_add(eff.delta);
        let Some((sq, _)) = data.obank().first_after_seq(&SeqNum::new(at.clone(), uintm::MAX))
        else {
            break;
        };
        let next = sq.get_addr().clone();
        if !adjacent(&at, &next, true) {
            break;
        }
        at = next;
    }
    Some(delta)
}

/// Net constant stack-pointer delta over the straight-line run that *ends* at
/// the instruction holding `branch` (the epilogue), or `None` when the run
/// cannot be accounted for.  The branch's own instruction is excluded: it is a
/// control-flow instruction by construction.
fn epilogue_delta(data: &Funcdata, branch: OpId, sp: &VarnodeStorage) -> Option<i64> {
    let site = data.obank().get(branch)?.get_addr().clone();
    let first_at_site = data.obank().iter_at(&site).next().map(|(_, id)| id)?;
    let mut cur = data.obank().op_before(first_at_site);
    let mut delta: i64 = 0;
    let mut prev = site;
    for _ in 0..MAX_SCAN_INSTRS {
        let Some(id) = cur else { break };
        let at = data.obank().get(id)?.get_addr().clone();
        if !adjacent(&prev, &at, false) {
            break;
        }
        let eff = group_effect(data, &at, sp);
        if eff.flow {
            break;
        }
        if eff.opaque {
            return None;
        }
        delta = delta.wrapping_add(eff.delta);
        let first_at = data.obank().iter_at(&at).next().map(|(_, i)| i)?;
        cur = data.obank().op_before(first_at);
        prev = at;
    }
    Some(delta)
}

/// (kuna) Is `op` a direct `jmp` that tears the frame down first, and so is a
/// tail call to a callee no discovery oracle found?
///
/// Fires iff:
///   - `gate` is on (`glb->tail_call_frame`);
///   - `op` is a direct, non-indirect `CPUI_BRANCH`;
///   - `dest` is neither the current function's own entry (self tail recursion
///     stays an ordinary back-edge, as in
///     [`kuna_tailcalljump`](crate::kuna_tailcalljump)) nor an address this
///     function has already decoded (an address already in the op bank is live
///     intraprocedural flow, whatever the stack looks like);
///   - the function establishes a frame (`prologue < 0`) and the run of
///     instructions ending at the branch tears down exactly that frame
///     (`epilogue == -prologue > 0`).
///
/// `sp` is the stack-pointer register location (`getStackSpace()
/// ->getSpacebaseFull(0)`), resolved by the caller.  With no stack space in the
/// compiler spec there is no frame to reason about and the rule declines.
pub fn kuna_is_frame_teardown_tail_call(
    data: &Funcdata,
    op: OpId,
    gate: bool,
    entry: &Address,
    dest: &Address,
    sp: Option<&VarnodeStorage>,
) -> bool {
    if !gate {
        return false;
    }
    let Some(sp) = sp else { return false };
    let Some(o) = data.obank().get(op) else { return false };
    if o.code() != OpCode::CPUI_BRANCH {
        return false;
    }
    if dest == entry || dest == o.get_addr() {
        return false;
    }
    // Already-decoded target: this branch is flow inside the body being built,
    // and turning it into a call would strand the blocks that are already there.
    if data.obank().iter_at(dest).next().is_some() {
        return false;
    }
    let Some(prologue) = prologue_delta(data, entry, sp) else { return false };
    if prologue >= 0 {
        return false;
    }
    let Some(epilogue) = epilogue_delta(data, op, sp) else { return false };
    epilogue > 0 && epilogue == -prologue
}

#[cfg(test)]
#[path = "kuna_tailcallframe/tests.rs"]
mod tests;
