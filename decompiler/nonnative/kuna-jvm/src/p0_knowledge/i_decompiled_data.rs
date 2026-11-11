//! Port of `org/jetbrains/java/decompiler/struct/IDecompiledData.java`.

use crate::struct_class::StructClass;

pub trait IDecompiledData {
    /// `None` when the class is not written out (Java returns `null`).
    fn get_class_entry_name(&self, cl: StructClass, entryname: &str) -> Option<String>;

    fn process_class(&self, cl: StructClass);

    /// `None` when nothing is written for the class (Java returns `null`).
    fn get_class_content(&self, cl: StructClass) -> Option<String>;
}
