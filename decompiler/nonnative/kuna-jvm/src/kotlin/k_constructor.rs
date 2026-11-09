//! Port of `org/vineflower/kotlin/struct/KConstructor.java`. Only the type and
//! its attribute key are declared so far; its record components and methods
//! come with the struct port.

use crate::key::Key;

pub struct KConstructor {}

impl KConstructor {
    /// `KConstructor.PRIMARY_CONSTRUCTOR`: the class attribute holding its primary constructor.
    #[allow(non_snake_case)]
    pub fn PRIMARY_CONSTRUCTOR() -> Key<KConstructor> {
        Key::of("primary-constructor")
    }
}
