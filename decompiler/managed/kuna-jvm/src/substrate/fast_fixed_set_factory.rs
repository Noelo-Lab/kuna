//! Port of `org/jetbrains/java/decompiler/util/collections/fixed/FastFixedSetFactory.java`.
//!
//! The abstract factory is a closed enum over its two implementations;
//! factories are shared references (clones alias).

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
