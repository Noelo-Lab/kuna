//! Port of `org/jetbrains/java/decompiler/util/collections/FastSparseSetFactory.java`.
//!
//! Sets made by one factory share its element numbering, and adding an element
//! unknown to the factory registers it there, so the factory is a shared
//! reference (clones alias) while each `FastSparseSet` is an owned value.

use std::cell::RefCell;
use std::rc::Rc;

use crate::packed_map::PackedMap;
use crate::rt::{JHashSet, JavaEq, JavaHash};

struct FactoryState<E> {
    col_values_internal: PackedMap<E>,
    last_block: i32,
    last_mask: i32,
}

pub struct FastSparseSetFactory<E>(Rc<RefCell<FactoryState<E>>>);

impl<E> Clone for FastSparseSetFactory<E> {
    fn clone(&self) -> Self {
        FastSparseSetFactory(self.0.clone())
    }
}

impl<E> FastSparseSetFactory<E> {
    /// Java reference `==`.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<E: JavaHash + JavaEq + Clone> FastSparseSetFactory<E> {
    pub fn new(set: impl IntoIterator<Item = E>) -> Self {
        let mut col_values_internal = PackedMap::new();
        let mut block = -1;
        let mut mask = -1i32;
        for (index, element) in set.into_iter().enumerate() {
            let index = index as i32;
            block = index >> 5;
            if (index & 31) == 0 {
                mask = 1;
            } else {
                mask = mask.wrapping_shl(1);
            }
            col_values_internal.put_with_key_packed(mask, block, element);
        }
        FastSparseSetFactory(Rc::new(RefCell::new(FactoryState {
            col_values_internal,
            last_block: block,
            last_mask: mask,
        })))
    }

    fn add_element(&self, element: E) -> i64 {
        let mut st = self.0.borrow_mut();
        if st.last_mask < 0 {
            st.last_mask = 1;
            st.last_block += 1;
        } else {
            st.last_mask = st.last_mask.wrapping_shl(1);
        }
        let (mask, block) = (st.last_mask, st.last_block);
        st.col_values_internal
            .put_with_key_packed(mask, block, element)
    }

    pub fn create_empty_set(&self) -> FastSparseSet<E> {
        FastSparseSet::new(self.clone())
    }

    fn get_last_block(&self) -> i32 {
        self.0.borrow().last_block
    }

    /// The packed `(mask, block)` of `element`, registering it when unknown.
    fn index_of(&self, element: &E) -> i64 {
        let known = {
            let st = self.0.borrow();
            st.col_values_internal
                .contains_key(element)
                .then(|| st.col_values_internal.get_with_key(element))
        };
        match known {
            Some(index) => index,
            None => self.add_element(element.clone()),
        }
    }

    fn size(&self) -> i32 {
        self.0.borrow().col_values_internal.size()
    }

    fn get(&self, index: i32) -> i64 {
        self.0.borrow().col_values_internal.get(index)
    }

    fn get_key(&self, index: i32) -> Option<E> {
        self.0.borrow().col_values_internal.get_key(index)
    }
}

pub struct FastSparseSet<E> {
    factory: FastSparseSetFactory<E>,
    data: Vec<i32>,
    next: Option<Vec<i32>>,
}

/// `getCopy()`.
impl<E> Clone for FastSparseSet<E> {
    fn clone(&self) -> Self {
        FastSparseSet {
            factory: self.factory.clone(),
            data: self.data.clone(),
            next: self.next.clone(),
        }
    }
}

impl<E: JavaHash + JavaEq + Clone> FastSparseSet<E> {
    fn new(factory: FastSparseSetFactory<E>) -> Self {
        let length = factory.get_last_block().max(1);
        FastSparseSet {
            factory,
            data: vec![0; length as usize],
            next: None,
        }
    }

    pub fn get_copy(&self) -> FastSparseSet<E> {
        self.clone()
    }

    fn ensure_capacity(&mut self, index: i32) {
        let mut newlength = self.data.len() as i32;
        if newlength == 0 {
            newlength = 1;
        }
        while newlength <= index {
            newlength *= 2;
        }
        self.data.resize(newlength as usize, 0);
        if let Some(next) = &mut self.next {
            next.resize(newlength as usize, 0);
        }
    }

    pub fn add(&mut self, element: E) {
        let index = self.factory.index_of(&element);
        let block = PackedMap::<E>::unpack_low(index);
        if block as usize >= self.data.len() {
            self.ensure_capacity(block);
        }
        self.data[block as usize] |= PackedMap::<E>::unpack_high(index);
        let old = self.get_next_idx(block);
        self.change_next(block, old, block);
    }

    fn get_next_idx(&self, block: i32) -> i32 {
        match &self.next {
            None => 0,
            Some(next) => next[block as usize],
        }
    }

    fn alloc_next(&mut self) -> &mut Vec<i32> {
        let len = self.data.len();
        self.next.get_or_insert_with(|| vec![0; len])
    }

    pub fn remove(&mut self, element: &E) {
        let index = self.factory.index_of(element);
        let block = PackedMap::<E>::unpack_low(index);
        if (block as usize) < self.data.len() {
            self.data[block as usize] &= !PackedMap::<E>::unpack_high(index);
            if self.data[block as usize] == 0 {
                let next = self.get_next_idx(block);
                self.change_next(block, block, next);
            }
        }
    }

    pub fn contains(&self, element: &E) -> bool {
        let index = self.factory.index_of(element);
        let block = PackedMap::<E>::unpack_low(index);
        (block as usize) < self.data.len()
            && (self.data[block as usize] & PackedMap::<E>::unpack_high(index)) != 0
    }

    fn set_next(&mut self) {
        let mut link = 0;
        for i in (0..self.data.len()).rev() {
            if link != 0 && self.next.is_none() {
                self.alloc_next()[i] = link;
            }
            if self.data[i] != 0 {
                link = i as i32;
            }
        }
    }

    fn change_next(&mut self, key: i32, oldnext: i32, newnext: i32) {
        let mut i = key - 1;
        while i >= 0 {
            if self.get_next_idx(i) == oldnext {
                self.alloc_next()[i as usize] = newnext;
            } else {
                break;
            }
            i -= 1;
        }
    }

    pub fn union(&mut self, set: &FastSparseSet<E>) {
        let intlength = self.data.len() as i32;
        let mut pointer = 0;
        loop {
            if pointer >= intlength {
                self.ensure_capacity(set.data.len() as i32 - 1);
            }
            let nextrec = self.data[pointer as usize] == 0;
            self.data[pointer as usize] |= set.data[pointer as usize];
            if nextrec {
                let old = self.get_next_idx(pointer);
                self.change_next(pointer, old, pointer);
            }
            pointer = set.get_next_idx(pointer);
            if pointer == 0 {
                break;
            }
        }
    }

    pub fn intersection(&mut self, set: &FastSparseSet<E>) {
        let minlength = set.data.len().min(self.data.len());
        for i in (0..minlength).rev() {
            self.data[i] &= set.data[i];
        }
        for i in (minlength..self.data.len()).rev() {
            self.data[i] = 0;
        }
        self.set_next();
    }

    pub fn complement(&mut self, set: &FastSparseSet<E>) {
        let extlength = set.data.len() as i32;
        let mut pointer = 0;
        loop {
            if pointer >= extlength {
                break;
            }
            self.data[pointer as usize] &= !set.data[pointer as usize];
            if self.data[pointer as usize] == 0 {
                let next = self.get_next_idx(pointer);
                self.change_next(pointer, pointer, next);
            }
            pointer = self.get_next_idx(pointer);
            if pointer == 0 {
                break;
            }
        }
    }

    pub fn equals(&self, o: &FastSparseSet<E>) -> bool {
        let (shortdata, longdata) = if self.data.len() > o.data.len() {
            (&o.data, &self.data)
        } else {
            (&self.data, &o.data)
        };
        for i in (0..shortdata.len()).rev() {
            if shortdata[i] != longdata[i] {
                return false;
            }
        }
        for i in (shortdata.len()..longdata.len()).rev() {
            if longdata[i] != 0 {
                return false;
            }
        }
        true
    }

    /// 0, 1, or 2 for "two or more".
    pub fn get_cardinality(&self) -> i32 {
        let mut found = false;
        for i in (0..self.data.len()).rev() {
            let block = self.data[i];
            if block != 0 {
                if found {
                    return 2;
                } else if (block & block.wrapping_sub(1)) == 0 {
                    found = true;
                } else {
                    return 2;
                }
            }
        }
        if found {
            1
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty() || (self.get_next_idx(0) == 0 && self.data[0] == 0)
    }

    pub fn iter(&self) -> FastSparseSetIterator<'_, E> {
        FastSparseSetIterator::new(self)
    }

    pub fn to_plain_set(&self) -> JHashSet<E> {
        let mut set = JHashSet::new();
        let mut size = self.data.len() as i32 * 32;
        let total = self.factory.size();
        if size > total {
            size = total;
        }
        for i in (0..size).rev() {
            let index = self.factory.get(i);
            if (self.data[PackedMap::<E>::unpack_low(index) as usize]
                & PackedMap::<E>::unpack_high(index))
                != 0
            {
                set.add(self.factory.get_key(i).expect("registered element"));
            }
        }
        set
    }

    fn get_factory(&self) -> &FastSparseSetFactory<E> {
        &self.factory
    }
}

impl<E: JavaHash + JavaEq + Clone> JavaEq for FastSparseSet<E> {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

/// `toPlainSet().hashCode()`: the sum of the element hashes.
impl<E: JavaHash + JavaEq + Clone> JavaHash for FastSparseSet<E> {
    fn java_hash(&self) -> i32 {
        self.to_plain_set()
            .iter()
            .fold(0i32, |h, e| h.wrapping_add(e.java_hash()))
    }
}

/// `toString()`: the plain set's `[a, b]` in `HashSet` order.
impl<E: JavaHash + JavaEq + Clone + std::fmt::Display> std::fmt::Display for FastSparseSet<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&java_collection_to_string(self.to_plain_set().iter()))
    }
}

/// `AbstractCollection.toString()`: `[a, b, c]`.
pub fn java_collection_to_string<T: std::fmt::Display>(items: impl Iterator<Item = T>) -> String {
    let mut s = String::from("[");
    for (n, item) in items.enumerate() {
        if n > 0 {
            s.push_str(", ");
        }
        s.push_str(&item.to_string());
    }
    s.push(']');
    s
}

impl<'a, E: JavaHash + JavaEq + Clone> IntoIterator for &'a FastSparseSet<E> {
    type Item = E;
    type IntoIter = FastSparseSetIterator<'a, E>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Ascending element-number order. Java's `remove()` is not provided: no
/// Vineflower code removes through this iterator.
pub struct FastSparseSetIterator<'a, E> {
    col_values_internal: FastSparseSetFactory<E>,
    data: &'a [i32],
    next: Option<&'a [i32]>,
    size: i32,
    pointer: i32,
    next_pointer: i32,
}

impl<'a, E: JavaHash + JavaEq + Clone> FastSparseSetIterator<'a, E> {
    fn new(set: &'a FastSparseSet<E>) -> Self {
        let col_values_internal = set.get_factory().clone();
        let size = col_values_internal.size();
        FastSparseSetIterator {
            col_values_internal,
            data: &set.data,
            next: set.next.as_deref(),
            size,
            pointer: -1,
            next_pointer: -1,
        }
    }

    fn get_next_index(&self, index: i32) -> i32 {
        let index = index + 1;
        let mut bindex = (index as u32 >> 5) as i32;
        let mut dindex = index & 0x1F;
        while (bindex as usize) < self.data.len() {
            let mut block = self.data[bindex as usize];
            if block != 0 {
                block = (block as u32 >> dindex) as i32;
                while dindex < 32 {
                    if (block & 1) != 0 {
                        return (bindex << 5) + dindex;
                    }
                    block = (block as u32 >> 1) as i32;
                    dindex += 1;
                }
            }
            dindex = 0;
            bindex = match self.next {
                None => 0,
                Some(next) => next[bindex as usize],
            };
            if bindex == 0 {
                break;
            }
        }
        -1
    }

    pub fn has_next(&mut self) -> bool {
        self.next_pointer = self.get_next_index(self.pointer);
        self.next_pointer >= 0
    }

    /// Java's `next()`; `None` where Java returns `null`.
    pub fn next_element(&mut self) -> Option<E> {
        if self.next_pointer >= 0 {
            self.pointer = self.next_pointer;
        } else {
            self.pointer = self.get_next_index(self.pointer);
            if self.pointer == -1 {
                self.pointer = self.size;
            }
        }
        self.next_pointer = -1;
        if self.pointer < self.size {
            self.col_values_internal.get_key(self.pointer)
        } else {
            None
        }
    }
}

impl<E: JavaHash + JavaEq + Clone> Iterator for FastSparseSetIterator<'_, E> {
    type Item = E;
    fn next(&mut self) -> Option<E> {
        if self.has_next() {
            self.next_element()
        } else {
            None
        }
    }
}
