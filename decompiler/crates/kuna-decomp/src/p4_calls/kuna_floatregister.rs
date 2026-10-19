//! Allocate scalar floating register payloads independently of object padding.

use crate::dtype::{type_class, type_metatype, Datatype};
use crate::fspec::ParamEntry;
use kuna_base::space::AddrSpaceManager;

pub fn storage_size(entry: &ParamEntry, datatype: &Datatype, manager: &AddrSpaceManager) -> i32 {
    if datatype.get_metatype() == type_metatype::TYPE_FLOAT
        && entry.get_type() == type_class::TYPECLASS_FLOAT
        && entry.is_exclusion()
        && manager
            .get_space_by_name("register")
            .is_some_and(|space| space.get_index() == entry.get_space().get_index())
    {
        datatype.get_size()
    } else {
        datatype.get_align_size()
    }
}
