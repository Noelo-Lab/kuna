//! Preserve integer bits when Go stores require a destination type conversion.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::printc::PrintC;

fn integer(ty: &Datatype) -> bool {
    matches!(ty.get_size(), 1 | 2 | 4 | 8)
        && matches!(ty.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
}

impl PrintC {
    pub(crate) fn push_go_store_value(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        pointer: VarnodeId,
        value: VarnodeId,
        op: OpId,
        mods: u32,
    ) -> bool {
        let Some(target) = self.go_value_type(fd, arch, pointer, op).get_ptr_to() else {
            return false;
        };
        if !integer(&target)
            || !fd.vbank().get(value).is_some_and(|value| value.get_size() == target.get_size())
        {
            return false;
        }
        let source = self.go_value_type(fd, arch, value, op);
        if !integer(&source) || target.get_size() != source.get_size()
            || target.get_metatype() == source.get_metatype()
        {
            return false;
        }
        let saved = self.context.mods();
        self.context.set_mods(mods);
        self.push_go_value_as(fd, arch, value, op, &target);
        self.context.set_mods(saved);
        true
    }
}
