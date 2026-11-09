//! Port of `org/jetbrains/java/decompiler/util/collections/fixed/FastFixedSetFactory.java`.
//!
//! The abstract factory is a closed enum over its two implementations;
//! factories are shared references (clones alias, `ptr_eq` is Java `==`).

use crate::fast_fixed_set::FastFixedSet;
use crate::long_fast_fixed_set_factory::LongFastFixedSetFactory;
use crate::rt::{JavaEq, JavaHash};
use crate::short_fast_fixed_set_factory::ShortFastFixedSetFactory;

pub enum FastFixedSetFactory<E> {
    Short(ShortFastFixedSetFactory<E>),
    Long(LongFastFixedSetFactory<E>),
}

impl<E> Clone for FastFixedSetFactory<E> {
    fn clone(&self) -> Self {
        match self {
            FastFixedSetFactory::Short(f) => FastFixedSetFactory::Short(f.clone()),
            FastFixedSetFactory::Long(f) => FastFixedSetFactory::Long(f.clone()),
        }
    }
}

impl<E> FastFixedSetFactory<E> {
    /// Java reference `==`.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FastFixedSetFactory::Short(a), FastFixedSetFactory::Short(b)) => a.ptr_eq(b),
            (FastFixedSetFactory::Long(a), FastFixedSetFactory::Long(b)) => a.ptr_eq(b),
            _ => false,
        }
    }

    /// Identity `hashCode()`.
    pub fn hash_code(&self) -> i32 {
        match self {
            FastFixedSetFactory::Short(f) => f.hash_code(),
            FastFixedSetFactory::Long(f) => f.hash_code(),
        }
    }
}

impl<E: JavaHash + JavaEq + Clone> FastFixedSetFactory<E> {
    /// Up to 64 entries use one `long` per set, more use a `long[]`.
    pub fn create(entries: impl IntoIterator<Item = E>) -> FastFixedSetFactory<E> {
        let entries: Vec<E> = entries.into_iter().collect();
        if entries.len() <= 64 {
            FastFixedSetFactory::Short(ShortFastFixedSetFactory::new(entries))
        } else {
            FastFixedSetFactory::Long(LongFastFixedSetFactory::new(entries))
        }
    }

    pub fn spawn_empty_set(&self) -> FastFixedSet<E> {
        match self {
            FastFixedSetFactory::Short(f) => FastFixedSet::Short(f.spawn_empty_set()),
            FastFixedSetFactory::Long(f) => FastFixedSet::Long(f.spawn_empty_set()),
        }
    }

    /// The entries in their original order.
    pub fn get_entries(&self) -> Vec<E> {
        match self {
            FastFixedSetFactory::Short(f) => f.get_entries(),
            FastFixedSetFactory::Long(f) => f.get_entries(),
        }
    }

    pub fn entries_len(&self) -> usize {
        match self {
            FastFixedSetFactory::Short(f) => f.entries_len(),
            FastFixedSetFactory::Long(f) => f.entries_len(),
        }
    }

    pub fn entries_contain(&self, item: &E) -> bool {
        match self {
            FastFixedSetFactory::Short(f) => f.entries_contain(item),
            FastFixedSetFactory::Long(f) => f.entries_contain(item),
        }
    }

    pub fn create_empty_set(&self) -> FastFixedSet<E> {
        self.spawn_empty_set()
    }

    pub fn create_copied_set(&self) -> FastFixedSet<E> {
        let mut set = self.spawn_empty_set();
        set.set_all_elements();
        set
    }
}
