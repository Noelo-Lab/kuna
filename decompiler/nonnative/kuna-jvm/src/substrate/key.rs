//! Port of `org/jetbrains/java/decompiler/util/Key.java`.

use std::marker::PhantomData;

use crate::rt::{JavaEq, JavaHash};

/// A typed name: keys compare by `name` alone, whatever their value type.
pub struct Key<T> {
    pub name: String,
    marker: PhantomData<fn() -> T>,
}

impl<T> Key<T> {
    /// Java: `Key(String)`.
    pub fn new(name: String) -> Key<T> {
        Key {
            name,
            marker: PhantomData,
        }
    }

    pub fn of(name: &str) -> Key<T> {
        Key::new(name.to_string())
    }

    /// The unchecked cast `(Key<U>) key`.
    pub fn cast<U>(&self) -> Key<U> {
        Key::new(self.name.clone())
    }

    pub fn equals<U>(&self, other: &Key<U>) -> bool {
        self.name == other.name
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Key::new(self.name.clone())
    }
}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl<T> Eq for Key<T> {}

impl<T> std::fmt::Debug for Key<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key({})", self.name)
    }
}

impl<T> JavaEq for Key<T> {
    fn java_equals(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T> JavaHash for Key<T> {
    fn java_hash(&self) -> i32 {
        self.name.java_hash()
    }
}
