//! Port of `org/vineflower/kotlin/struct/KHiddenElement.java`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum KHiddenElement {
    Generic,
    CompanionInstance,
    CompanionItem,
    ObjectInstance,
    DefaultImpl,
}

impl KHiddenElement {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            KHiddenElement::Generic => "GENERIC",
            KHiddenElement::CompanionInstance => "COMPANION_INSTANCE",
            KHiddenElement::CompanionItem => "COMPANION_ITEM",
            KHiddenElement::ObjectInstance => "OBJECT_INSTANCE",
            KHiddenElement::DefaultImpl => "DEFAULT_IMPL",
        }
    }
}
