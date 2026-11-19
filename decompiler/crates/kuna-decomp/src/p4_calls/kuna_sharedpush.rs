//! P4 a stack argument pushed once for two calls on mutually exclusive arms.
//!
//! MSVC hoists an argument push that both arms of a branch need above the
//! branch: `push 0x14; jz L; call [malloc]; jmp M; L: call _emalloc; M: pop ecx`.
//! Both calls are entered with the same stack pointer and read the same slot,
//! so at run time both receive `0x14`. The upstream double-use test
//! (`checkCallDoubleUse`, funcdata_varnode.cc:1802) refuses an input trial whose
//! value another call also reads unless that call's own trial is still open;
//! a call with a declared parameter list has no trial, so the declared call
//! always keeps the push and the undeclared one prints `_emalloc()`.
//!
//! The refusal exists for the stale slot: a cdecl argument the caller has not
//! popped yet is still on the stack when a later call runs, and that call must
//! not take it. That is a matter of order, so the rule asks for its absence.
//! It accepts the declared call's read when all of these hold:
//!
//! * both ops are calls and the trial is a stack trial read as the slot's own
//!   Varnode, which the declared call reads directly or through the copy it
//!   makes for its parameter, so no write of the slot comes between;
//! * the declared call has a parameter at the trial's call-relative address and
//!   size, and both calls sit at the same stack offset: the push is an argument
//!   the declared callee takes, at the position the other callee is entered
//!   with it, and never deeper. When the declared list places every parameter
//!   on the stack but the callee's own entry reads `r` of EAX, ECX and EDX
//!   before writing them (x86-32 Windows), the list is not its convention:
//!   every x86-32 convention passes the leading parameters in registers, so
//!   the callee's stack arguments are at most the parameters after the first
//!   `r`, and a slot above them is no argument of it. GCC passes a static
//!   function's arguments in EAX, EDX and ECX while its debug information lists
//!   them as ordinary parameters; a MinGW `thiscall` member takes `this` in ECX
//!   and the rest on the stack;
//! * the slot is written by the push (not a merge or a call's side effect), the
//!   push and the two calls sit in three distinct blocks, and neither call's
//!   block reaches the other's without passing through the push's block;
//! * every other call reading the slot, a copy of it, or its continuation
//!   through a merge or a call, is apart from the scored call in the same sense.
//!
//! The third condition is the soundness of the rule. A path from one call to
//! the other that avoids the push's block would run the second call on the slot
//! the first one already consumed, the stale case. A path through the push's
//! block pushes the value again, so arms inside a loop body still qualify. The
//! fourth keeps the scored call the only reader on its arm: with two (a
//! control-flow guard check, which takes its target in ECX, then the guarded
//! call) the stack cannot show which one the push was for.
//!
//! Two undeclared calls keep upstream's first-scored-wins treatment: neither
//! side states its arity, so the push may be a later call's deeper argument.
//! The undeclared callee's own arity is not read either: a compiler hoists a
//! push only when both arms begin with it, the inference a lone push rests on.

use std::collections::HashSet;

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::fspec::{FuncCallSpecs, ParamTrial, ProtoParameter, OFFSET_UNKNOWN};
use crate::funcdata::Funcdata;
use crate::p4_calls::kuna_exclusivearguse::MAX_VISIT;

/// Cap on Varnodes the reader walk visits.
const MAX_VALUES: usize = 256;

/// Whether `op`'s read of `vn` as a declared parameter leaves the stack trial
/// matched at `opmatch` a parameter too.
pub(crate) fn declared_arm_shares(
    data: &Funcdata,
    opmatch: OpId,
    op: OpId,
    vn: VarnodeId,
    trial: &ParamTrial,
) -> bool {
    let (Some(m), Some(o)) = (data.obank().get(opmatch), data.obank().get(op)) else {
        return false;
    };
    let is_call = |c| matches!(c, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND);
    if !is_call(m.code()) || !is_call(o.code()) {
        return false;
    }
    let j = o.get_slot(vn);
    let addr = trial.get_address();
    let (Some(space), Some(slot)) = (addr.get_space(), m.get_in(trial.get_slot())) else {
        return false;
    };
    let Some(sv) = data.vbank().get(slot) else {
        return false;
    };
    if j <= 0 || space.get_type() != spacetype::IPTR_SPACEBASE || sv.get_space().get_index() != space.get_index() {
        return false;
    }
    let Some(push) = sv
        .get_def()
        .and_then(|d| data.obank().get(d))
        .filter(|d| !matches!(d.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT))
    else {
        return false;
    };
    let copies_slot = data
        .vbank()
        .get(vn)
        .and_then(|v| v.get_def())
        .and_then(|d| data.obank().get(d))
        .is_some_and(|d| d.code() == OpCode::CPUI_COPY && d.get_in(0) == Some(slot) && d.get_parent() == o.get_parent());
    if slot != vn && !copies_slot {
        return false;
    }
    let (Some(fc), Some(matchfc)) = (
        data.get_call_specs_index(op).map(|i| data.get_call_specs(i)),
        data.get_call_specs_index(opmatch).map(|i| data.get_call_specs(i)),
    ) else {
        return false;
    };
    if fc.is_input_active() || !fc.is_input_locked() {
        return false;
    }
    let base = fc.get_spacebase_offset();
    if base == OFFSET_UNKNOWN
        || matchfc.get_spacebase_offset() != base
        || space.wrap_offset(sv.get_offset().wrapping_sub(base)) != addr.get_offset()
    {
        return false;
    }
    if !fc.proto().get_param(j - 1).is_some_and(|p| p.get_address() == *addr && p.get_size() == trial.get_size()) {
        return false;
    }
    if !within_stack_arguments(fc, addr.get_offset()) {
        return false;
    }
    let (Some(mb), Some(ob), Some(wall)) = (m.get_parent(), o.get_parent(), push.get_parent()) else {
        return false;
    };
    mb != wall && apart(data, mb, ob, wall) && only_reader(data, slot, opmatch, op, mb, wall)
}

/// Whether the callee-relative `offset` can hold a stack argument of the
/// declared call `fc`, given the argument registers its callee reads: with a
/// declared list that places every parameter on the stack, those registers
/// carry the leading parameters, and only the rest can be on the stack above
/// the return address.
fn within_stack_arguments(fc: &FuncCallSpecs, offset: u64) -> bool {
    let registers = usize::from(fc.callee_arg_registers());
    let proto = fc.proto();
    let params: Vec<_> = (0..proto.num_params()).filter_map(|i| proto.get_param(i)).collect();
    let in_register =
        |p: &&dyn ProtoParameter| p.get_address().get_space().is_some_and(|s| s.get_type() != spacetype::IPTR_SPACEBASE);
    if registers == 0 || params.iter().any(in_register) {
        return true;
    }
    let stack: u64 = params.iter().skip(registers).map(|p| (p.get_size().max(0) as u64).div_ceil(4) * 4).sum();
    offset < 4 + stack
}

/// Distinct blocks, neither reaching the other without entering `wall`.
fn apart(data: &Funcdata, a: BlockId, b: BlockId, wall: BlockId) -> bool {
    a != b && b != wall && !reaches_avoiding(data, a, b, wall) && !reaches_avoiding(data, b, a, wall)
}

/// Whether every call other than `opmatch` and `op` that reads `slot`, or a
/// copy, merge or call-crossing of it, as an argument is apart from `opmatch`'s
/// block `mb`. Declines once [`MAX_VALUES`] Varnodes are seen.
fn only_reader(data: &Funcdata, slot: VarnodeId, opmatch: OpId, op: OpId, mb: BlockId, wall: BlockId) -> bool {
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut stack = vec![slot];
    while let Some(v) = stack.pop() {
        if !seen.insert(v) {
            continue;
        }
        if seen.len() > MAX_VALUES {
            return false;
        }
        let Some(var) = data.vbank().get(v) else {
            return false;
        };
        for d in var.descend_iter() {
            let Some(o) = data.obank().get(d) else {
                continue;
            };
            match o.code() {
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    if d != opmatch && d != op && o.get_slot(v) > 0 && !o.get_parent().is_some_and(|b| apart(data, mb, b, wall)) {
                        return false;
                    }
                }
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => stack.extend(o.get_out()),
                OpCode::CPUI_INDIRECT if o.get_in(0) == Some(v) => stack.extend(o.get_out()),
                _ => {}
            }
        }
    }
    true
}

/// Is `target` reachable from `from` along CFG out-edges without entering
/// `wall`? Declines (true) once [`MAX_VISIT`] blocks are seen.
fn reaches_avoiding(data: &Funcdata, from: BlockId, target: BlockId, wall: BlockId) -> bool {
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
            if next != wall {
                stack.push(next);
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};

    use super::*;
    use crate::context::ArchContext;

    fn build_fd() -> Funcdata {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            4,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        let glb = Rc::new(ArchContext::new(m));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        Funcdata::new("f", "f", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
    }

    #[test]
    fn arms_are_apart_only_when_every_path_between_them_repushes() {
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let entry = fd.bblocks_mut().new_block_basic(root);
        let push = fd.bblocks_mut().new_block_basic(root);
        let left = fd.bblocks_mut().new_block_basic(root);
        let right = fd.bblocks_mut().new_block_basic(root);
        let join = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(entry, push);
        fd.bblocks_mut().add_edge(push, left);
        fd.bblocks_mut().add_edge(push, right);
        fd.bblocks_mut().add_edge(left, join);
        fd.bblocks_mut().add_edge(right, join);
        assert!(apart(&fd, left, right, push));
        assert!(apart(&fd, right, left, push));
        assert!(!apart(&fd, left, join, push));
        assert!(!apart(&fd, left, left, push));
        assert!(!apart(&fd, left, push, push));
        fd.bblocks_mut().add_edge(join, push);
        assert!(apart(&fd, left, right, push), "a loop back through the push re-pushes");
        assert!(!apart(&fd, left, right, entry), "the back edge reaches the other arm");
        fd.bblocks_mut().add_edge(left, right);
        assert!(!apart(&fd, left, right, push));
        assert!(!apart(&fd, right, left, push));
    }
}
