//! Heritage's own reads of the storage it guards, filled in by renaming
//! instead of through placeholder Varnodes.
//!
//! Upstream gives every input slot of a new MULTIEQUAL, the input of every
//! call and store guard INDIRECT and the input of every persist RETURN COPY a
//! free placeholder Varnode at the op's own output storage. Renaming replaces
//! each one with the Varnode on top of that storage's stack and deletes it. A
//! huge function's pass builds millions of them (one per join slot, one per
//! guarded range per call), all alive between placement and renaming, so they
//! set the Varnode arena's high-water mark for the rest of the function. Here
//! the slot stays empty and the op is marked; renaming reads the op's output
//! storage where upstream reads the placeholder's, which is the same storage,
//! so the slot is linked to the same Varnode. A slot renaming never reaches
//! (an edge from a block outside the dominator tree, or a pass stopped by its
//! deadline) gets upstream's placeholder when the pass finishes.

use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::op::pcodeop_addlflags::kuna_rename_read;
use kuna_base::address::Address;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

/// Is `op` waiting for renaming to fill its read of its own storage?
#[inline]
pub(crate) fn pending(fd: &Funcdata, op: OpId) -> bool {
    fd.obank().get(op).is_some_and(|o| o.get_addlflags() & kuna_rename_read != 0)
}

/// Mark `op`, whose `n` empty input slots read its output storage, for
/// renaming. The slots' placeholders keep their creation indices, so every
/// later Varnode is numbered as upstream numbers it.
pub(crate) fn mark(fd: &mut Funcdata, op: OpId, n: uint4, pending: &mut Vec<(OpId, uint4)>) {
    let first = fd.vbank_mut().reserve_create_index(n);
    push(fd, op, first, pending);
}

fn push(fd: &mut Funcdata, op: OpId, first: uint4, pending: &mut Vec<(OpId, uint4)>) {
    if let Some(o) = fd.obank_mut().get_mut(op) {
        o.set_additional_flag(kuna_rename_read);
    }
    pending.push((op, first));
}

/// The storage a pending slot of `op` reads: its output's.
pub(crate) fn storage(fd: &Funcdata, op: OpId) -> Option<(Address, int4)> {
    let out = fd.obank().get(op)?.get_out()?;
    let vn = fd.vbank().get(out)?;
    Some((vn.get_addr().clone(), vn.get_size()))
}

/// Build upstream's guard `INDIRECT` (`newIndirectOp`) on `addr:sz` at
/// `indeffect`, its input left for renaming.
pub(crate) fn new_indirect(
    fd: &mut Funcdata,
    indeffect: OpId,
    addr: &Address,
    sz: int4,
    extra_flags: uint4,
    pending: &mut Vec<(OpId, uint4)>,
) -> OpId {
    let first = fd.vbank_mut().reserve_create_index(1);
    let indaddr = fd.obank().get(indeffect).expect("renamereads: stale effect op").get_addr().clone();
    let newop = fd.new_op(2, indaddr);
    fd.obank_mut().get_mut(newop).expect("renamereads: new INDIRECT").set_flag(extra_flags);
    let _ = fd.new_varnode_out(sz, addr, newop);
    fd.op_set_opcode_code(newop, OpCode::CPUI_INDIRECT);
    let iop = fd.new_varnode_iop(indeffect);
    let _ = fd.op_set_input(newop, iop, 1);
    fd.op_insert_before(newop, indeffect);
    push(fd, newop, first, pending);
    newop
}

/// Clear the marks of the pass's `pending` ops, giving every slot renaming
/// left empty upstream's placeholder: a free Varnode at the op's storage,
/// active for heritage except on a MULTIEQUAL.
pub(crate) fn finish(fd: &mut Funcdata, pending: &mut Vec<(OpId, uint4)>) {
    for (op, first) in std::mem::take(pending) {
        let Some(o) = fd.obank_mut().get_mut(op) else { continue };
        if o.get_addlflags() & kuna_rename_read == 0 {
            continue;
        }
        o.clear_additional_flag(kuna_rename_read);
        let phi = o.code() == OpCode::CPUI_MULTIEQUAL;
        let slots = if phi { o.num_input() } else { 1 };
        let Some((addr, size)) = storage(fd, op) else { continue };
        for slot in 0..slots {
            if fd.obank().get(op).and_then(|o| o.get_in(slot)).is_some() {
                continue;
            }
            let vn = fd.new_varnode_reserved(size, &addr, first.wrapping_add(slot as uint4));
            if !phi {
                fd.vbank_mut().get_mut(vn).expect("renamereads: placeholder").set_active_heritage();
            }
            let _ = fd.op_set_input(op, vn, slot);
        }
    }
}
