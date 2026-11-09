//! Port of `org/jetbrains/java/decompiler/main/extern/IIdentifierRenamer.java`.
//!
//! Renamers are shared as `Rc<dyn IIdentifierRenamer>`; stateful ones keep
//! their counters in cells.

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Type {
    ElementClass,
    ElementField,
    ElementMethod,
}

pub trait IIdentifierRenamer {
    fn to_be_renamed(
        &self,
        element_type: Type,
        class_name: Option<&str>,
        element: Option<&str>,
        descriptor: Option<&str>,
    ) -> bool;

    fn get_next_class_name(&self, full_name: &str, short_name: Option<&str>) -> String;

    fn get_next_field_name(&self, class_name: &str, field: &str, descriptor: &str) -> String;

    fn get_next_method_name(&self, class_name: &str, method: &str, descriptor: &str) -> String;
}
