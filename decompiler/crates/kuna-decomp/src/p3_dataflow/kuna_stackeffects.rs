//! Memory-effect barriers for unresolved writes through escaped frame pointers.

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_base::space::spacetype;

pub fn enabled_for(fd: &Funcdata, addr: &Address) -> bool {
    fd.get_arch().stack_views
        && addr
            .get_space()
            .is_some_and(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
}

/// A literal global address cannot designate this frame. A pointer reloaded
/// from memory can: its store must start a new memory version of escaped slots.
pub fn unresolved_store(fd: &Funcdata, op: OpId) -> bool {
    fd.obank()
        .get(op)
        .and_then(|o| o.get_in(1))
        .and_then(|v| fd.vbank().get(v))
        .is_some_and(|v| !v.is_constant())
}
