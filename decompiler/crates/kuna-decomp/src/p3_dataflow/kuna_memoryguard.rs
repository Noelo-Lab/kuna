//! (kuna) Heritage guards for user operations that may write memory: the
//! system calls of `syscallregs` and `x64syscall`, and the Octeon atomics of
//! `octeoneffects`. Each owner picks its operations and [`memory_ops`] lists
//! them all, so heritage guards, `ActionMarkImplied` cover checks and
//! `persistcalls` sparse guarding see the same set. One INDIRECT per operation
//! guards a writable RAM or stack range, as a call guard does. Read-only image
//! ranges are never guarded, and only an address-tied range is forced, so a
//! local whose address never escapes still collapses.

use kuna_base::{address::Address, space::spacetype, types::{int4, uint4}};

use crate::{
    context::{OpId, VarnodeId},
    funcdata::Funcdata,
    varnode::varnode_flags,
};

/// Is `addr` in the default data space or a stack (space-base) space?
pub fn memory_space(data: &Funcdata, addr: &Address) -> bool {
    addr.get_space().is_some_and(|space| {
        space.get_type() == spacetype::IPTR_SPACEBASE
            || data
                .get_arch()
                .manage()
                .get_default_data_space()
                .is_some_and(|ram| ram.get_index() == space.get_index())
    })
}

/// Can a range at `addr` with heritage properties `flags` take a guard: writable
/// RAM or stack storage?
pub fn guarded(data: &Funcdata, flags: uint4, addr: &Address) -> bool {
    flags & varnode_flags::readonly == 0 && memory_space(data, addr)
}

/// Every user operation with a memory effect, in owner order: the enabled
/// system calls, then the Octeon operations the mode selects.
pub fn memory_ops(data: &Funcdata) -> Vec<OpId> {
    let mut ops = crate::kuna_syscallregs::memory_calls(data);
    ops.extend(crate::kuna_x64syscall::memory_calls(data));
    ops.extend(crate::kuna_octeoneffects::memory_ops(data));
    ops
}

/// Guard a writable range with heritage properties `flags` across every
/// [`memory_ops`] operation. `ops` caches those for the heritage pass: guarding
/// adds no CALLOTHER.
pub fn guard_memory(
    data: &mut Funcdata,
    ops: &mut Option<Vec<OpId>>,
    flags: uint4,
    addr: &Address,
    size: int4,
    write: &mut Vec<VarnodeId>,
) {
    if !guarded(data, flags, addr) {
        return;
    }
    let ops = ops.get_or_insert_with(|| memory_ops(data));
    guard(data, ops, flags, addr, size, write);
}

/// Guard the range `[addr, addr+size)` with heritage properties `flags` across
/// each of `ops`, pushing every guard output onto `write`.
fn guard(data: &mut Funcdata, ops: &[OpId], flags: uint4, addr: &Address, size: int4, write: &mut Vec<VarnodeId>) {
    for &op in ops {
        let guard = data.new_indirect_op(op, addr, size, 0);
        if let Some(input) = data.obank().get(guard).and_then(|o| o.get_in(0)) {
            data.vbank_mut().get_mut(input).expect("memory guard input").set_active_heritage();
        }
        if let Some(output) = data.obank().get(guard).and_then(|o| o.get_out()) {
            let v = data.vbank_mut().get_mut(output).expect("memory guard output");
            v.set_active_heritage();
            if flags & varnode_flags::addrtied != 0 {
                v.set_addr_force();
            }
            write.push(output);
        }
    }
}
