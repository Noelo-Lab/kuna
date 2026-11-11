//! Port of `org/jetbrains/java/decompiler/util/collections/fixed/LongFastFixedSetFactory.java`.

use std::rc::Rc;

use crate::fast_fixed_set_factory::FastFixedSetFactory;
use crate::rt::{JLinkedHashMap, JavaEq, JavaHash};
use crate::short_fast_fixed_set_factory::next_factory_identity;

struct LongFactoryData<E> {
    indexes: JLinkedHashMap<E, i32>,
    /// The entries by index, when the indexes are exactly their insertion
    /// order 0, 1, 2, ... (no duplicate input): members can then be listed by
    /// scanning set bits, in the same order as the factory walk.
    by_index: Option<Vec<E>>,
    data_length: i32,
    identity: i32,
}

pub struct LongFastFixedSetFactory<E>(Rc<LongFactoryData<E>>);

impl<E> Clone for LongFastFixedSetFactory<E> {
    fn clone(&self) -> Self {
        LongFastFixedSetFactory(self.0.clone())
    }
}

impl<E> LongFastFixedSetFactory<E> {
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn hash_code(&self) -> i32 {
        self.0.identity
    }
}

impl<E: JavaHash + JavaEq + Clone> LongFastFixedSetFactory<E> {
    pub fn new(set: Vec<E>) -> Self {
        let data_length = (set.len() as i32 + 63) >> 6;
        let mut indexes = JLinkedHashMap::new();
        for (index, element) in set.into_iter().enumerate() {
            indexes.put(element, index as i32);
        }
        let by_index = indexes
            .iter()
            .enumerate()
            .all(|(n, (_, &i))| i == n as i32)
            .then(|| indexes.key_vec());
        LongFastFixedSetFactory(Rc::new(LongFactoryData {
            indexes,
            by_index,
            data_length,
            identity: next_factory_identity(),
        }))
    }

    pub fn spawn_empty_set(&self) -> LongFastFixedSet<E> {
        LongFastFixedSet {
            factory: self.clone(),
            data: vec![0; self.0.data_length as usize],
        }
    }

    pub fn get_entries(&self) -> Vec<E> {
        self.0.indexes.key_vec()
    }

    pub fn entries_len(&self) -> usize {
        self.0.indexes.len()
    }

    pub fn entries_contain(&self, item: &E) -> bool {
        self.0.indexes.contains_key(item)
    }

    fn index(&self, element: &E) -> i32 {
        *self
            .0
            .indexes
            .get(element)
            .expect("LongFastFixedSet: element outside the factory")
    }
}

pub struct LongFastFixedSet<E> {
    factory: LongFastFixedSetFactory<E>,
    data: Vec<i64>,
}

/// `clone()`: same factory, copied bits.
impl<E> Clone for LongFastFixedSet<E> {
    fn clone(&self) -> Self {
        LongFastFixedSet {
            factory: self.factory.clone(),
            data: self.data.clone(),
        }
    }
}

fn bit(index: i32) -> i64 {
    1i64.wrapping_shl((index & 63) as u32)
}

impl<E: JavaHash + JavaEq + Clone> LongFastFixedSet<E> {
    pub fn get_factory(&self) -> FastFixedSetFactory<E> {
        FastFixedSetFactory::Long(self.factory.clone())
    }

    pub fn set_all_elements(&mut self) {
        let last = self.data.len() - 1;
        let n = self.factory.0.indexes.len() as i32;
        self.data[last] = 2i64.wrapping_shl(((n & 63) - 1) as u32).wrapping_sub(1);
        for d in &mut self.data[..last] {
            *d = -1;
        }
    }

    pub fn add(&mut self, element: E) -> bool {
        let index = self.factory.index(&element);
        let word = &mut self.data[(index >> 6) as usize];
        let old = *word;
        *word |= bit(index);
        old != *word
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    pub fn remove(&mut self, element: &E) -> bool {
        let index = self.factory.index(element);
        let word = &mut self.data[(index >> 6) as usize];
        let old = *word;
        *word &= !bit(index);
        old != *word
    }

    pub fn contains(&self, element: &E) -> bool {
        let index = self.factory.index(element);
        (self.data[(index >> 6) as usize] & bit(index)) != 0
    }

    pub fn contains_all(&self, set: &LongFastFixedSet<E>) -> bool {
        let extdata = self.get_data(set);
        for i in (0..self.data.len()).rev() {
            if (extdata[i] & !self.data[i]) != 0 {
                return false;
            }
        }
        true
    }

    fn get_data<'a>(&self, set: &'a LongFastFixedSet<E>) -> &'a [i64] {
        crate::validation_helper::ValidationHelper::validate_true(
            set.factory.ptr_eq(&self.factory),
            "Incompatible set factories",
        );
        &set.data
    }

    pub fn add_all(&mut self, set: &LongFastFixedSet<E>) -> bool {
        let extdata = self.get_data(set);
        let mut mutated = false;
        for i in (0..self.data.len()).rev() {
            let old = self.data[i];
            self.data[i] |= extdata[i];
            mutated |= old != self.data[i];
        }
        mutated
    }

    pub fn retain_all(&mut self, set: &LongFastFixedSet<E>) -> bool {
        let extdata = self.get_data(set);
        let mut mutated = false;
        for i in (0..self.data.len()).rev() {
            let old = self.data[i];
            self.data[i] &= extdata[i];
            mutated |= old != self.data[i];
        }
        mutated
    }

    pub fn remove_all(&mut self, set: &LongFastFixedSet<E>) -> bool {
        let extdata = self.get_data(set);
        let mut mutated = false;
        for i in (0..self.data.len()).rev() {
            let old = self.data[i];
            self.data[i] &= !extdata[i];
            mutated |= old != self.data[i];
        }
        mutated
    }

    pub fn equals(&self, that: &LongFastFixedSet<E>) -> bool {
        self.factory.ptr_eq(&that.factory) && self.data == that.data
    }

    /// `31 * factory.hashCode() + Arrays.hashCode(data)`.
    pub fn hash_code(&self) -> i32 {
        let data_hash = self
            .data
            .iter()
            .fold(1i32, |h, d| h.wrapping_mul(31).wrapping_add(d.java_hash()));
        31i32
            .wrapping_mul(self.factory.hash_code())
            .wrapping_add(data_hash)
    }

    pub fn get_real_size(&self) -> i32 {
        self.data.iter().map(|d| d.count_ones() as i32).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|&d| d == 0)
    }

    /// Members in factory order.
    pub fn iter(&self) -> impl Iterator<Item = E> + '_ {
        let scan = || {
            self.factory
                .0
                .indexes
                .iter()
                .filter(move |(_, &i)| (self.data[(i >> 6) as usize] & bit(i)) != 0)
                .map(|(item, _)| item.clone())
        };
        match &self.factory.0.by_index {
            Some(by_index) => Members::Bits {
                by_index,
                data: &self.data,
                word: 0,
                rest: self.data.first().map_or(0, |&w| w as u64),
            },
            None => Members::Scan(scan()),
        }
    }

    /// An iterator-`remove()` loop: drops the members `keep` rejects.
    pub fn retain(&mut self, mut keep: impl FnMut(&E) -> bool) {
        let factory = self.factory.clone();
        if let Some(by_index) = &factory.0.by_index {
            for word in 0..self.data.len() {
                let mut rest = self.data[word] as u64;
                while rest != 0 {
                    let i = (word * 64) as i32 + rest.trailing_zeros() as i32;
                    rest &= rest - 1;
                    if (i as usize) < by_index.len() && !keep(&by_index[i as usize]) {
                        self.data[word] &= !bit(i);
                    }
                }
            }
            return;
        }
        for (item, &i) in factory.0.indexes.iter() {
            let word = (i >> 6) as usize;
            if (self.data[word] & bit(i)) != 0 && !keep(item) {
                self.data[word] &= !bit(i);
            }
        }
    }
}

enum Members<'a, E, I> {
    Bits {
        by_index: &'a [E],
        data: &'a [i64],
        word: usize,
        rest: u64,
    },
    Scan(I),
}

impl<E: Clone, I: Iterator<Item = E>> Iterator for Members<'_, E, I> {
    type Item = E;

    fn next(&mut self) -> Option<E> {
        match self {
            Members::Scan(scan) => scan.next(),
            Members::Bits {
                by_index,
                data,
                word,
                rest,
            } => loop {
                if *rest != 0 {
                    let i = *word * 64 + rest.trailing_zeros() as usize;
                    *rest &= *rest - 1;
                    if i < by_index.len() {
                        return Some(by_index[i].clone());
                    }
                    continue;
                }
                *word += 1;
                if *word >= data.len() {
                    return None;
                }
                *rest = data[*word] as u64;
            },
        }
    }
}

impl<E: JavaHash + JavaEq + Clone + std::fmt::Display> std::fmt::Display for LongFastFixedSet<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items: Vec<String> = self.iter().map(|item| item.to_string()).collect();
        write!(f, "{{{}}}", items.join(","))
    }
}
