//! Member addresses must use a pointer compatible with the emitted variable.

use std::rc::Rc;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

pub(crate) fn needs_cast(
    fd: &Funcdata,
    decl_high_type: bool,
    base: VarnodeId,
    pointer: &Rc<Datatype>,
) -> bool {
    if !fd.get_arch().stack_views
        || !fd.get_func_proto().has_store()
        || pointer.is_formal_pointer_rel()
    {
        return false;
    }
    let Some(declared) = crate::printc::declared_variable_type(fd, decl_high_type, base) else {
        return false;
    };
    declared.get_size() == pointer.get_size()
        && matches!(
            declared.get_metatype(),
            type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT
                | type_metatype::TYPE_UNKNOWN
                | type_metatype::TYPE_PTR
        )
        && !Rc::ptr_eq(&declared, pointer)
        && !matches!(declared.compare(pointer, 10), Ok(0))
}
