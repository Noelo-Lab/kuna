//! A float stored into bytes of unknown type is stored as a float.
//!
//! `TypeOpStore::getInputCast` casts the stored value to the pointee type, and
//! `CastStrategyC::castStandard` needs no cast into `TYPE_UNKNOWN`: upstream
//! prints the store as `*puVar1 = fVar2`, a copy of bits into an `undefined4`.
//! kuna's C declares unknown bytes as an integer (`unsigned int *v1`), where
//! the same assignment is C's numeric float-to-integer conversion: the 1.5f a
//! `vstr s0,[r6]` writes as 0x3fc00000 reads as a store of 1. [`pointer`] gives
//! the store a pointer to the value's own type instead, so it prints as
//! `*(float *)v1 = v2`. With `realtypes` off the bytes print as `xunknown4`,
//! upstream's bit copy, and the store is left as upstream prints it; a float
//! narrower than C's `float` (a half float) has no C type to point at.

use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;

/// The type of value `value` that a store through a pointer to `pointed_to`
/// must be cast to point at: `value` itself, when it is a float C can name and
/// the pointee is unknown bytes of the same size that print as an integer.
pub(crate) fn pointer(data: &Funcdata, pointed_to: &Datatype, value: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    let mut bytes = pointed_to;
    while let Some(base) = bytes.qualified_base().or_else(|| bytes.get_typedef()) {
        bytes = base.as_ref();
    }
    let arch = data.get_arch();
    (arch.unknown_is_integer
        && bytes.get_metatype() == type_metatype::TYPE_UNKNOWN
        && value.get_metatype() == type_metatype::TYPE_FLOAT
        && value.get_size() == bytes.get_size()
        && value.get_size() >= arch.types_rc()?.get_size_of_float())
    .then(|| Rc::clone(value))
}
