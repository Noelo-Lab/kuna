//! Port of `org/vineflower/kotlin/util/KEnumProcessor.java`.

#![allow(dead_code, unused_variables)]

use crate::class_wrapper::ClassWrapper;
use crate::unported;

pub struct KEnumProcessor;

impl KEnumProcessor {
    const ENUM_ENTRIES: &'static str = "Lkotlin/enums/EnumEntries;";

    pub fn clear_enum(wrapper: ClassWrapper) {
        unported!("KEnumProcessor.clearEnum")
    }
}
