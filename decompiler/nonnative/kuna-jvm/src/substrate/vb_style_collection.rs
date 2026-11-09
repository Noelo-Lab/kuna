//! Port of `org/jetbrains/java/decompiler/util/collections/VBStyleCollection.java`.
//!
//! An `ArrayList<E>` whose elements may carry a key (`null` for plain adds).
//! Read access to the list goes through `Deref<Target = [E]>`; every mutation
//! goes through the methods below so the key index stays in step.

use std::ops::Deref;

use crate::jthrow;
use crate::rt::{JHashMap, JavaEq, JavaHash, JavaListExt};

pub struct VBStyleCollection<E, K> {
    list: Vec<E>,
    map: JHashMap<K, i32>,
    lst_keys: Vec<Option<K>>,
}

impl<E, K> Deref for VBStyleCollection<E, K> {
    type Target = [E];
    fn deref(&self) -> &[E] {
        &self.list
    }
}

impl<E: std::fmt::Debug, K: JavaHash + JavaEq + std::fmt::Debug> std::fmt::Debug
    for VBStyleCollection<E, K>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.lst_keys.iter().zip(&self.list))
            .finish()
    }
}

impl<E, K: JavaHash + JavaEq> Default for VBStyleCollection<E, K> {
    fn default() -> Self {
        VBStyleCollection::new()
    }
}

/// `clone()`: a new list of the same elements, a copy of the key map and of the key list.
impl<E: Clone, K: JavaHash + JavaEq + Clone> Clone for VBStyleCollection<E, K> {
    fn clone(&self) -> Self {
        let mut c = VBStyleCollection::new();
        c.add_all(self.list.clone());
        c.map = self.map.clone();
        c.lst_keys = self.lst_keys.clone();
        c
    }
}

impl<E, K: JavaHash + JavaEq> VBStyleCollection<E, K> {
    pub fn new() -> Self {
        VBStyleCollection {
            list: Vec::new(),
            map: JHashMap::new(),
            lst_keys: Vec::new(),
        }
    }

    /// Java: `VBStyleCollection(int initialCapacity)`.
    pub fn with_capacity(initial_capacity: i32) -> Self {
        VBStyleCollection {
            list: Vec::with_capacity(initial_capacity as usize),
            map: JHashMap::with_capacity(initial_capacity as usize),
            lst_keys: Vec::with_capacity(initial_capacity as usize),
        }
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn as_slice(&self) -> &[E] {
        &self.list
    }

    pub fn add(&mut self, element: E) -> bool {
        self.lst_keys.push(None);
        self.list.push(element);
        true
    }

    /// Java: `remove(Object)`, which Vineflower refuses.
    pub fn remove_item(&mut self, _element: &E) -> bool {
        jthrow!("not implemented!")
    }

    /// Java: `addAll(Collection)`; the new elements carry no key.
    pub fn add_all(&mut self, c: impl IntoIterator<Item = E>) -> bool {
        let items: Vec<E> = c.into_iter().collect();
        for _ in 0..items.len() {
            self.lst_keys.push(None);
        }
        let changed = !items.is_empty();
        self.list.extend(items);
        changed
    }

    pub fn add_with_key(&mut self, element: E, key: K)
    where
        K: Clone,
    {
        self.map.put(key.clone(), self.list.len() as i32);
        self.list.push(element);
        self.lst_keys.push(Some(key));
    }

    pub fn add_all_with_key(&mut self, elements: Vec<E>, keys: Vec<K>)
    where
        K: Clone,
    {
        let mut index = self.list.len() as i32;
        for key in &keys {
            self.map.put(key.clone(), index);
            index += 1;
        }
        self.list.extend(elements);
        self.lst_keys.extend(keys.into_iter().map(Some));
    }

    /// Replaces the element under `key`, or appends it; returns the replaced element.
    pub fn put_with_key(&mut self, element: E, key: K) -> Option<E>
    where
        K: Clone,
    {
        match self.map.get(&key).copied() {
            None => {
                self.add_with_key(element, key);
                None
            }
            Some(index) => Some(std::mem::replace(&mut self.list[index as usize], element)),
        }
    }

    /// Java: `add(int index, E element)`.
    pub fn add_at(&mut self, index: usize, element: E)
    where
        K: Clone,
    {
        self.add_to_list_index(index, 1);
        self.lst_keys.insert(index, None);
        self.list.insert(index, element);
    }

    pub fn add_with_key_and_index(&mut self, index: usize, element: E, key: K)
    where
        K: Clone,
    {
        self.add_to_list_index(index, 1);
        self.map.put(key.clone(), index as i32);
        self.list.insert(index, element);
        self.lst_keys.insert(index, Some(key));
    }

    pub fn remove_with_key(&mut self, key: &K)
    where
        K: Clone,
    {
        let Some(index) = self.map.get(key).copied() else {
            return;
        };
        let index = index as usize;
        self.add_to_list_index(index + 1, -1);
        self.list.remove(index);
        self.lst_keys.remove(index);
        self.map.remove(key);
    }

    /// Java: `remove(int index)`.
    pub fn remove_at(&mut self, index: usize) -> E
    where
        K: Clone,
    {
        self.add_to_list_index(index + 1, -1);
        if let Some(obj) = &self.lst_keys[index] {
            self.map.remove(obj);
        }
        self.lst_keys.remove(index);
        self.list.remove(index)
    }

    /// Java: `getIndexByKey`; an absent key is Java's unboxing `NullPointerException`.
    pub fn get_index_by_key(&self, key: &K) -> usize {
        *self
            .map
            .get(key)
            .expect("VBStyleCollection.getIndexByKey: no such key") as usize
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.lst_keys.clear();
        self.list.clear();
    }

    /// `set(index, element)`, inherited: the key of the slot is kept.
    pub fn set(&mut self, index: usize, element: E) -> E {
        std::mem::replace(&mut self.list[index], element)
    }

    pub fn get_lst_keys(&self) -> &Vec<Option<K>> {
        &self.lst_keys
    }

    fn add_to_list_index(&mut self, index: usize, diff: i32)
    where
        K: Clone,
    {
        for i in (index..self.lst_keys.len()).rev() {
            if let Some(obj) = self.lst_keys[i].clone() {
                self.map.put(obj, i as i32 + diff);
            }
        }
    }
}

impl<E: Clone, K: JavaHash + JavaEq> VBStyleCollection<E, K> {
    pub fn get(&self, index: usize) -> E {
        self.list[index].clone()
    }

    pub fn get_with_key(&self, key: &K) -> Option<E> {
        let index = *self.map.get(key)?;
        Some(self.list[index as usize].clone())
    }

    pub fn get_last(&self) -> E {
        self.list[self.list.len() - 1].clone()
    }

    pub fn to_vec(&self) -> Vec<E> {
        self.list.clone()
    }

    /// `subList(from, to)` materialized as a vector.
    pub fn sub_vec(&self, from: usize, to: usize) -> Vec<E> {
        self.list[from..to].to_vec()
    }
}

impl<E, K: Clone> VBStyleCollection<E, K> {
    pub fn get_key(&self, index: usize) -> Option<K> {
        self.lst_keys[index].clone()
    }
}

impl<E: JavaEq, K> VBStyleCollection<E, K> {
    /// `indexOf(Object)` by Java `equals`; -1 when absent.
    pub fn index_of(&self, element: &E) -> i32 {
        self.list.j_index_of(element)
    }

    pub fn contains(&self, element: &E) -> bool {
        self.list.j_contains(element)
    }
}

impl<E: std::fmt::Display, K: std::fmt::Display> VBStyleCollection<E, K> {
    pub fn to_string_vb(&self) -> String {
        let mut sb = String::from("[");
        for i in 0..self.list.len() {
            if i > 0 {
                sb.push_str(", ");
            }
            match &self.lst_keys[i] {
                Some(key) => sb.push_str(&key.to_string()),
                None => sb.push_str("null"),
            }
            sb.push('=');
            sb.push_str(&self.list[i].to_string());
        }
        sb.push(']');
        sb
    }
}

/// `ArrayList.equals`: element-wise, keys ignored.
impl<E: JavaEq, K> JavaEq for VBStyleCollection<E, K> {
    fn java_equals(&self, other: &Self) -> bool {
        self.list.len() == other.list.len()
            && self
                .list
                .iter()
                .zip(&other.list)
                .all(|(a, b)| a.java_equals(b))
    }
}

impl<E: JavaHash, K> JavaHash for VBStyleCollection<E, K> {
    fn java_hash(&self) -> i32 {
        self.list
            .iter()
            .fold(1i32, |h, e| h.wrapping_mul(31).wrapping_add(e.java_hash()))
    }
}

impl<'a, E, K> IntoIterator for &'a VBStyleCollection<E, K> {
    type Item = &'a E;
    type IntoIter = std::slice::Iter<'a, E>;
    fn into_iter(self) -> Self::IntoIter {
        self.list.iter()
    }
}
