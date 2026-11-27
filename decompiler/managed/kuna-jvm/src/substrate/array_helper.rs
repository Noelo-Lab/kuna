//! Port of `org/jetbrains/java/decompiler/util/ArrayHelper.java`.

pub struct ArrayHelper;

impl ArrayHelper {
    /// Reference containment: meant for arena handles, whose `==` is Java's `==`.
    pub fn contains_by_ref<T: PartialEq>(array: &[T], value: &T) -> bool {
        array.iter().any(|t| t == value)
    }
}
