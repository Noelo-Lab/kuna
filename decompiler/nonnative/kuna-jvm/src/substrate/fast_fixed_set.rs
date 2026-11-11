//! Port of `org/jetbrains/java/decompiler/util/collections/fixed/FastFixedSet.java`.
//!
//! A set over the fixed universe of its factory: the abstract class is a closed
//! enum over the two factories' set types. Rust `clone()` is Java `clone()`.
//! The `Collection` overloads of `addAll`/`retainAll`/`removeAll` are the
//! `*_collection` methods; Java's `containsAll(Collection)` for a non-fixed set
//! recurses into itself forever and has no counterpart.

use crate::fast_fixed_set_factory::FastFixedSetFactory;
use crate::long_fast_fixed_set_factory::LongFastFixedSet;
use crate::rt::{exception, JHashSet, JavaEq, JavaHash};
use crate::short_fast_fixed_set_factory::ShortFastFixedSet;
use crate::validation_helper::ValidationHelper;

pub enum FastFixedSet<E> {
    Short(ShortFastFixedSet<E>),
    Long(LongFastFixedSet<E>),
}

impl<E> Clone for FastFixedSet<E> {
    fn clone(&self) -> Self {
        match self {
            FastFixedSet::Short(s) => FastFixedSet::Short(s.clone()),
            FastFixedSet::Long(s) => FastFixedSet::Long(s.clone()),
        }
    }
}

#[track_caller]
fn incompatible() -> ! {
    exception::throw("ClassCastException", "FastFixedSet of another factory kind")
}

impl<E: JavaHash + JavaEq + Clone> FastFixedSet<E> {
    pub fn get_factory(&self) -> FastFixedSetFactory<E> {
        match self {
            FastFixedSet::Short(s) => s.get_factory(),
            FastFixedSet::Long(s) => s.get_factory(),
        }
    }

    /// Deprecated in Java: reports a validation failure, then answers the factory's size.
    pub fn size(&self) -> i32 {
        ValidationHelper::validate_true(
            false,
            "The behaviour of FastFixedSet.size() is not correct",
        );
        self.get_factory().entries_len() as i32
    }

    pub fn get_real_size(&self) -> i32 {
        match self {
            FastFixedSet::Short(s) => s.get_real_size(),
            FastFixedSet::Long(s) => s.get_real_size(),
        }
    }

    pub fn set_all_elements(&mut self) {
        match self {
            FastFixedSet::Short(s) => s.set_all_elements(),
            FastFixedSet::Long(s) => s.set_all_elements(),
        }
    }

    pub fn add(&mut self, element: E) -> bool {
        match self {
            FastFixedSet::Short(s) => s.add(element),
            FastFixedSet::Long(s) => s.add(element),
        }
    }

    pub fn remove(&mut self, element: &E) -> bool {
        match self {
            FastFixedSet::Short(s) => s.remove(element),
            FastFixedSet::Long(s) => s.remove(element),
        }
    }

    pub fn clear(&mut self) {
        match self {
            FastFixedSet::Short(s) => s.clear(),
            FastFixedSet::Long(s) => s.clear(),
        }
    }

    pub fn contains(&self, element: &E) -> bool {
        match self {
            FastFixedSet::Short(s) => s.contains(element),
            FastFixedSet::Long(s) => s.contains(element),
        }
    }

    /// Java: `containsAll(FastFixedSet)`.
    pub fn contains_all(&self, set: &FastFixedSet<E>) -> bool {
        match (self, set) {
            (FastFixedSet::Short(a), FastFixedSet::Short(b)) => a.contains_all(b),
            (FastFixedSet::Long(a), FastFixedSet::Long(b)) => a.contains_all(b),
            _ => incompatible(),
        }
    }

    /// Java: `addAll(FastFixedSet)`.
    pub fn add_all(&mut self, set: &FastFixedSet<E>) -> bool {
        match (self, set) {
            (FastFixedSet::Short(a), FastFixedSet::Short(b)) => a.add_all(b),
            (FastFixedSet::Long(a), FastFixedSet::Long(b)) => a.add_all(b),
            _ => incompatible(),
        }
    }

    /// Java: `addAll(Collection)` for a collection that is not a set of this factory.
    pub fn add_all_collection(&mut self, c: impl IntoIterator<Item = E>) -> bool {
        let mut modified = false;
        for e in c {
            if self.add(e) {
                modified = true;
            }
        }
        modified
    }

    /// Java: `retainAll(FastFixedSet)`.
    pub fn retain_all(&mut self, set: &FastFixedSet<E>) -> bool {
        match (self, set) {
            (FastFixedSet::Short(a), FastFixedSet::Short(b)) => a.retain_all(b),
            (FastFixedSet::Long(a), FastFixedSet::Long(b)) => a.retain_all(b),
            _ => incompatible(),
        }
    }

    /// Java: `retainAll(Collection)` for a collection that is not a set of this factory.
    pub fn retain_all_collection(&mut self, c: &[E]) -> bool {
        let mut stuffs = JHashSet::from_collection(self.get_factory().get_entries());
        for item in c {
            stuffs.remove(item);
        }
        self.remove_all_collection(stuffs.iter())
    }

    /// Java: `removeAll(FastFixedSet)`.
    pub fn remove_all(&mut self, set: &FastFixedSet<E>) -> bool {
        match (self, set) {
            (FastFixedSet::Short(a), FastFixedSet::Short(b)) => a.remove_all(b),
            (FastFixedSet::Long(a), FastFixedSet::Long(b)) => a.remove_all(b),
            _ => incompatible(),
        }
    }

    /// Java: `removeAll(Collection)` for a collection that is not a set of this factory.
    pub fn remove_all_collection<'a>(&mut self, c: impl IntoIterator<Item = &'a E>) -> bool
    where
        E: 'a,
    {
        let mut mutated = false;
        for o in c {
            mutated |= self.remove(o);
        }
        mutated
    }

    pub fn equals(&self, o: &FastFixedSet<E>) -> bool {
        match (self, o) {
            (FastFixedSet::Short(a), FastFixedSet::Short(b)) => a.equals(b),
            (FastFixedSet::Long(a), FastFixedSet::Long(b)) => a.equals(b),
            _ => false,
        }
    }

    pub fn hash_code(&self) -> i32 {
        match self {
            FastFixedSet::Short(s) => s.hash_code(),
            FastFixedSet::Long(s) => s.hash_code(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            FastFixedSet::Short(s) => s.is_empty(),
            FastFixedSet::Long(s) => s.is_empty(),
        }
    }

    /// Members in the factory's entry order.
    pub fn iter(&self) -> Box<dyn Iterator<Item = E> + '_> {
        match self {
            FastFixedSet::Short(s) => Box::new(s.iter()),
            FastFixedSet::Long(s) => Box::new(s.iter()),
        }
    }

    pub fn to_vec(&self) -> Vec<E> {
        self.iter().collect()
    }

    /// An iterator-`remove()` loop: drops the members `keep` rejects.
    pub fn retain(&mut self, keep: impl FnMut(&E) -> bool) {
        match self {
            FastFixedSet::Short(s) => s.retain(keep),
            FastFixedSet::Long(s) => s.retain(keep),
        }
    }

    pub fn to_plain_set(&self) -> JHashSet<E> {
        let mut set = JHashSet::with_capacity(self.get_real_size() as usize);
        for item in self.iter() {
            set.add(item);
        }
        set
    }

    /// Deprecated in Java: membership in the factory's universe.
    pub fn contains_key(&self, id: &E) -> bool {
        self.get_factory().entries_contain(id)
    }

    /// Deprecated in Java: `contains(FastFixedSet)` is `containsAll`.
    pub fn contains_set(&self, set: &FastFixedSet<E>) -> bool {
        self.contains_all(set)
    }

    /// Deprecated in Java: `removeAll`.
    pub fn complement(&mut self, set: &FastFixedSet<E>) {
        self.remove_all(set);
    }

    /// Deprecated in Java: `clone()`.
    pub fn get_copy(&self) -> FastFixedSet<E> {
        self.clone()
    }

    /// Deprecated in Java: `addAll`.
    pub fn union(&mut self, set: &FastFixedSet<E>) {
        self.add_all(set);
    }

    /// Deprecated in Java: `retainAll`.
    pub fn intersection(&mut self, set: &FastFixedSet<E>) {
        self.retain_all(set);
    }
}

impl<E: JavaHash + JavaEq + Clone> JavaEq for FastFixedSet<E> {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl<E: JavaHash + JavaEq + Clone> JavaHash for FastFixedSet<E> {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl<E: JavaHash + JavaEq + Clone + std::fmt::Display> std::fmt::Display for FastFixedSet<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FastFixedSet::Short(s) => s.fmt(f),
            FastFixedSet::Long(s) => s.fmt(f),
        }
    }
}
