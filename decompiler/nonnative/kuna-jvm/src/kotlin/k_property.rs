//! Port of `org/vineflower/kotlin/struct/KProperty.java`. Only the type and
//! `PropertyMethod` are declared so far; its record components and methods
//! come with the struct port.

use crate::key::Key;

pub struct KProperty {}

/// `KProperty.PropertyMethod`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum PropertyMethod {
    Getter,
    Setter,
}

impl PropertyMethod {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            PropertyMethod::Getter => "GETTER",
            PropertyMethod::Setter => "SETTER",
        }
    }

    /// `PropertyMethod.KEY` (`Key.of(PropertyMethod.class.getName())`).
    #[allow(non_snake_case)]
    pub fn KEY() -> Key<PropertyMethod> {
        Key::of("org.vineflower.kotlin.struct.KProperty$PropertyMethod")
    }
}
