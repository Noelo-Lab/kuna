//! Port of `org/jetbrains/java/decompiler/util/collections/fixed/ShortFastFixedSetFactory.java`.

use std::cell::Cell;
use std::rc::Rc;

use crate::fast_fixed_set_factory::FastFixedSetFactory;
use crate::rt::{JLinkedHashMap, JavaEq, JavaHash};
use crate::validation_helper::ValidationHelper;

struct ShortFactoryData<E> {
    masks: JLinkedHashMap<E, i64>,
    full: i64,
    identity: i32,
}

pub struct ShortFastFixedSetFactory<E>(Rc<ShortFactoryData<E>>);

impl<E> Clone for ShortFastFixedSetFactory<E> {
    fn clone(&self) -> Self {
        ShortFastFixedSetFactory(self.0.clone())
    }
}

thread_local! {
    static NEXT_IDENTITY: Cell<i32> = const { Cell::new(1) };
}

/// A stand-in for `Object.hashCode()` of a factory.
pub(crate) fn next_factory_identity() -> i32 {
    NEXT_IDENTITY.with(|n| {
        let id = n.get();
        n.set(id.wrapping_add(1));
        id
    })
}

impl<E> ShortFastFixedSetFactory<E> {
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn hash_code(&self) -> i32 {
        self.0.identity
    }
}

impl<E: JavaHash + JavaEq + Clone> ShortFastFixedSetFactory<E> {
    pub fn new(set: Vec<E>) -> Self {
        ValidationHelper::assert_true(
            set.len() <= 64,
            "Short fast fixed set can't contain more than 64 elements",
        );
        let mut masks = JLinkedHashMap::new();
        let mut mask: i64 = 1;
        for element in set {
            masks.put(element, mask);
            mask = mask.wrapping_shl(1);
        }
        ShortFastFixedSetFactory(Rc::new(ShortFactoryData {
            masks,
            full: mask.wrapping_sub(1),
            identity: next_factory_identity(),
        }))
    }

    pub fn spawn_empty_set(&self) -> ShortFastFixedSet<E> {
        ShortFastFixedSet {
            factory: self.clone(),
            data: 0,
        }
    }

    pub fn get_entries(&self) -> Vec<E> {
        self.0.masks.key_vec()
    }

    pub fn entries_len(&self) -> usize {
        self.0.masks.len()
    }

    pub fn entries_contain(&self, item: &E) -> bool {
        self.0.masks.contains_key(item)
    }

    fn mask(&self, element: &E) -> i64 {
        *self
            .0
            .masks
            .get(element)
            .expect("ShortFastFixedSet: element outside the factory")
    }
}

pub struct ShortFastFixedSet<E> {
    factory: ShortFastFixedSetFactory<E>,
    data: i64,
}

/// `clone()`: same factory, copied bits.
impl<E> Clone for ShortFastFixedSet<E> {
    fn clone(&self) -> Self {
        ShortFastFixedSet {
            factory: self.factory.clone(),
            data: self.data,
        }
    }
}

impl<E: JavaHash + JavaEq + Clone> ShortFastFixedSet<E> {
    pub fn get_factory(&self) -> FastFixedSetFactory<E> {
        FastFixedSetFactory::Short(self.factory.clone())
    }

    pub fn set_all_elements(&mut self) {
        self.data = self.factory.0.full;
    }

    pub fn add(&mut self, element: E) -> bool {
        let mask = self.factory.mask(&element);
        let old = self.data;
        self.data |= mask;
        old != self.data
    }

    pub fn clear(&mut self) {
        self.data = 0;
    }

    pub fn remove(&mut self, element: &E) -> bool {
        let mask = self.factory.mask(element);
        let old = self.data;
        self.data &= !mask;
        old != self.data
    }

    pub fn contains(&self, element: &E) -> bool {
        let mask = self.factory.mask(element);
        (self.data & mask) != 0
    }

    pub fn contains_all(&self, set: &ShortFastFixedSet<E>) -> bool {
        (set.data & !self.data) == 0
    }

    pub fn add_all(&mut self, set: &ShortFastFixedSet<E>) -> bool {
        let old = self.data;
        self.data |= set.data;
        old != self.data
    }

    pub fn retain_all(&mut self, set: &ShortFastFixedSet<E>) -> bool {
        let old = self.data;
        self.data &= set.data;
        old != self.data
    }

    pub fn remove_all(&mut self, set: &ShortFastFixedSet<E>) -> bool {
        let old = self.data;
        self.data &= !set.data;
        old != self.data
    }

    pub fn equals(&self, that: &ShortFastFixedSet<E>) -> bool {
        self.factory.ptr_eq(&that.factory) && self.data == that.data
    }

    pub fn hash_code(&self) -> i32 {
        31i32
            .wrapping_mul(self.factory.hash_code())
            .wrapping_add(self.data.java_hash())
    }

    pub fn get_real_size(&self) -> i32 {
        self.data.count_ones() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.data == 0
    }

    /// Members in factory order.
    pub fn iter(&self) -> impl Iterator<Item = E> + '_ {
        self.factory
            .0
            .masks
            .iter()
            .filter(move |(_, &mask)| (self.data & mask) != 0)
            .map(|(item, _)| item.clone())
    }

    /// An iterator-`remove()` loop: drops the members `keep` rejects.
    pub fn retain(&mut self, mut keep: impl FnMut(&E) -> bool) {
        let factory = self.factory.clone();
        for (item, &mask) in factory.0.masks.iter() {
            if (self.data & mask) != 0 && !keep(item) {
                self.data &= !mask;
            }
        }
    }
}

impl<E: JavaHash + JavaEq + Clone + std::fmt::Display> std::fmt::Display for ShortFastFixedSet<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items: Vec<String> = self.iter().map(|item| item.to_string()).collect();
        write!(f, "{{{}}}", items.join(","))
    }
}
