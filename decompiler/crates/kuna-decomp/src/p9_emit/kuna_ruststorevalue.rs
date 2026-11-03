//! Convert Rust store values to their destination type without changing their bits.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::printc::PrintC;

impl PrintC {
    pub(crate) fn push_rust_store_value(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        pointer: VarnodeId,
        value: VarnodeId,
        op: OpId,
        mods: u32,
    ) -> bool {
        let Some(target) = self.rust_integer_value_type(fd, arch, pointer, op).get_ptr_to() else {
            return false;
        };
        let saved = self.context.mods();
        self.context.set_mods(mods);
        let converted = self.push_rust_value_as(fd, arch, value, op, &target);
        self.context.set_mods(saved);
        converted
    }
}
