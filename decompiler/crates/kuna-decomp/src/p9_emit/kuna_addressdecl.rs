//! Declare an address-only local from the referenced object, not the PTRSUB offset.

use std::rc::Rc;

use crate::context::HighVariableId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// A reference high has no storage of its own. Its constants encode an address
/// offset whose width says nothing about the size of the referenced scalar.
pub(crate) fn object_type(fd: &Funcdata, high: HighVariableId) -> Option<Rc<Datatype>> {
    if crate::kuna_paramrefdecl::references_parameter_symbol(fd, high) {
        return None;
    }
    let h = fd.high_bank().get(high)?;
    let object = h.kuna_symbol_type()?;
    if h.kuna_symbol_offset() < 0
        || h.num_instances() == 0
        || matches!(object.get_metatype(), type_metatype::TYPE_ARRAY | type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION)
        || !(0..h.num_instances()).all(|i| {
            fd.vbank().get(h.get_instance(i)).is_some_and(|v| v.is_constant())
        })
    {
        return None;
    }
    Some(object.clone())
}
