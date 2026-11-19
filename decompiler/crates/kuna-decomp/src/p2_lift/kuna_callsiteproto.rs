//! (kuna) A loader-stated prototype for one call instruction.
//!
//! An analysis pass that reads a call's arguments out of the image can know
//! more about that one call than its callee's prototype says: the `uefi` pass
//! knows which interface a `LocateProtocol` call asks for. It parks
//! `call instruction -> prototype` on the [`Architecture`], and
//! `FlowInfo::setup_call_specs` asks [`parked`] for the instruction of every
//! call it builds, installing the answer as that call's prototype override.
//! An override the function already carries for the instruction wins.

use kuna_base::address::Address;

use crate::architecture::Architecture;
use crate::fspec::PrototypePieces;

/// The prototype parked for the call instruction at `site`.
pub fn parked(arch: &Architecture, site: &Address) -> Option<PrototypePieces> {
    if arch.kuna_callsite_protos.is_empty() {
        return None;
    }
    let code = arch.manage().get_default_code_space()?;
    if !site.get_space().is_some_and(|space| std::rc::Rc::ptr_eq(space, code)) {
        return None;
    }
    arch.kuna_callsite_protos.get(&site.get_offset()).cloned()
}
