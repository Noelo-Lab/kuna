//! Port of `org/jetbrains/java/decompiler/api/plugin/LanguageChooser.java`.

use crate::struct_class::StructClass;

pub trait LanguageChooser {
    fn is_language(&self, cl: StructClass) -> bool;
}

impl<F: Fn(StructClass) -> bool> LanguageChooser for F {
    fn is_language(&self, cl: StructClass) -> bool {
        self(cl)
    }
}
