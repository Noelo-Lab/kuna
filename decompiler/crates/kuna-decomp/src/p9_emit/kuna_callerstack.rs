//! Render an unnamed AMD64 incoming-stack address through the compiler's CFA.

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::architecture::Architecture;
use crate::funcdata::Funcdata;

/// The AMD64 CFA is entry RSP + 8. Locals and the saved return address are not
/// incoming caller storage, and retain their existing symbol handling.
pub(crate) fn address(
    arch: &Architecture,
    fd: &Funcdata,
    loc: &Address,
    ptr_size: int4,
) -> Option<String> {
    let proto = fd.get_func_proto();
    if arch.archid != "x86:LE:64:default:gcc"
        || ptr_size != 8
        || !proto.has_model()
        || proto.model().get_name() != "__stdcall"
        || proto.model().get_extra_pop() != 8
    {
        return None;
    }
    let stack = arch.manage().get_stack_space()?;
    let space = loc.get_space()?;
    if !std::rc::Rc::ptr_eq(stack, space)
        || space.get_addr_size() != 8
        || space.get_word_size() != 1
        || !space.stack_grows_negative()
        || space.num_spacebase() != 1
    {
        return None;
    }
    let base = space.get_spacebase(0).ok()?;
    if base.size != 8
        || arch
            .translate()
            .get_register_name(base.space.as_ref()?, base.offset, 8)
            != "RSP"
    {
        return None;
    }
    let offset = i64::try_from(loc.get_offset()).ok()?.checked_sub(8)?;
    if offset < 0 {
        return None;
    }
    Some(if offset == 0 {
        "((char *)__builtin_dwarf_cfa())".to_owned()
    } else {
        format!("((char *)__builtin_dwarf_cfa() + {offset:#x})")
    })
}
