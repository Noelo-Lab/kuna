//! (kuna `msvcsig`) Resolve a call through an import slot that carries a locked
//! MSVC prototype while the flow is followed, instead of after a restart.
//!
//! `msvcsig` locks each MSVC import's declared prototype onto the import's
//! FunctionSymbol, under the convention the name states. A 32-bit `call dword
//! ptr [slot]` is a CALLIND until `ActionDeindirect` resolves the slot, and the
//! locked `__thiscall`/`__cdecl` prototype it then merges is incompatible with the
//! default-model one the call was given, so the function restarts with the call
//! direct from the flow on. That is upstream's behavior, and on an MSVC C++ image
//! nearly every function pays it: a restart re-runs the whole pipeline.
//!
//! The restart's only lasting effect on such a call is the indirect override it
//! records, so the call is made direct here from the start: when the CALLIND's
//! target is the value its own instruction loads from a constant address, and
//! that address is an import slot (`peimportcall` paints it external) whose
//! prototype `msvcsig` locked. Any other indirect call is left to
//! `ActionDeindirect`, as before.

use std::collections::BTreeSet;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::funcdata::Funcdata;

/// The import slot `op` calls through, when `locked` holds it (as
/// `(space index, offset)`) and it is an external import slot.
pub fn locked_import_slot(
    data: &Funcdata,
    op: OpId,
    locked: &BTreeSet<(int4, uintb)>,
) -> Option<Address> {
    let call = data.obank().get(op)?;
    if call.code() != OpCode::CPUI_CALLIND {
        return None;
    }
    let target = data.vbank().get(call.get_in(0)?)?;
    let size = target.get_size();
    let slot = match target.get_space().get_type() {
        spacetype::IPTR_PROCESSOR => target.get_addr().clone(),
        spacetype::IPTR_INTERNAL => loaded_slot(data, op, target.get_addr().clone(), size)?,
        _ => return None,
    };
    let space = slot.get_space()?;
    if !locked.contains(&(space.get_index(), slot.get_offset())) {
        return None;
    }
    let props = data.get_arch().query_global_properties(&slot, size, &slot);
    if props & crate::varnode::varnode_flags::externref == 0 {
        return None;
    }
    let (_, entry, proto) = data.get_arch().query_function(&slot)?;
    (entry == slot && proto.is_input_locked()).then_some(slot)
}

/// The memory word the temporary `(addr, size)` that `op` reads was filled from,
/// within `op`'s own instruction: a `COPY` of a memory varnode (the form a
/// constant-address load takes once SLEIGH folds it) or a `LOAD` through a
/// constant pointer, through any chain of temporary `COPY`s.
fn loaded_slot(data: &Funcdata, op: OpId, mut addr: Address, mut size: int4) -> Option<Address> {
    let insn = data.obank().get(op)?.get_addr().clone();
    let mut cur = op;
    loop {
        if data.obank().get(cur)?.is_instruction_start() {
            return None;
        }
        cur = data.obank().dead_prev(cur)?;
        let prev = data.obank().get(cur)?;
        if prev.get_addr() != &insn {
            return None;
        }
        let writes = prev
            .get_out()
            .and_then(|v| data.vbank().get(v))
            .is_some_and(|v| v.get_addr() == &addr && v.get_size() == size);
        if !writes {
            continue;
        }
        match prev.code() {
            OpCode::CPUI_COPY => {
                let src = data.vbank().get(prev.get_in(0)?)?;
                match src.get_space().get_type() {
                    spacetype::IPTR_PROCESSOR => return Some(src.get_addr().clone()),
                    spacetype::IPTR_INTERNAL => (addr, size) = (src.get_addr().clone(), src.get_size()),
                    _ => return None,
                }
            }
            OpCode::CPUI_LOAD => {
                let space_id = data.vbank().get(prev.get_in(0)?)?;
                let pointer = data.vbank().get(prev.get_in(1)?)?;
                if !space_id.is_constant() || !pointer.is_constant() {
                    return None;
                }
                let manager = data.get_arch().manage();
                let index = space_id.get_offset();
                if index >= manager.num_spaces() as u64 {
                    return None;
                }
                let space = std::rc::Rc::clone(manager.get_space(index as int4)?);
                let offset = space.wrap_offset(pointer.get_offset());
                return Some(Address::new(space, offset));
            }
            _ => return None,
        }
    }
}
