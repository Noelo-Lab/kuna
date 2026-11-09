//! Shared mutable collections: the Rust shape of a Java collection reference
//! that more than one holder reads and mutates.

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::jhash::JavaEq;

/// A Java object reference to any mutable value: clones alias the same value.
pub struct Shared<T>(Rc<RefCell<T>>);

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Shared(self.0.clone())
    }
}

impl<T: Default> Default for Shared<T> {
    fn default() -> Self {
        Shared::new(T::default())
    }
}

impl<T> Shared<T> {
    pub fn new(value: T) -> Self {
        Shared(Rc::new(RefCell::new(value)))
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }

    /// Java reference `==`.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Shared<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}

/// A Java `List<T>` reference: clones alias the same list.
///
/// Every method takes a short borrow and returns owned values, so calls never
/// overlap; `iter` walks a snapshot, so the list may be mutated inside the loop
/// (a Java index loop that observes its own mutations must stay an index loop).
pub struct JList<T>(Rc<RefCell<Vec<T>>>);

impl<T> Clone for JList<T> {
    fn clone(&self) -> Self {
        JList(self.0.clone())
    }
}

impl<T> Default for JList<T> {
    fn default() -> Self {
        JList::new()
    }
}

impl<T> From<Vec<T>> for JList<T> {
    fn from(v: Vec<T>) -> Self {
        JList(Rc::new(RefCell::new(v)))
    }
}

impl<T> JList<T> {
    pub fn new() -> Self {
        JList(Rc::new(RefCell::new(Vec::new())))
    }

    pub fn borrow(&self) -> Ref<'_, Vec<T>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Vec<T>> {
        self.0.borrow_mut()
    }

    /// Java reference `==`.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    pub fn add(&self, item: T) {
        self.0.borrow_mut().push(item)
    }

    pub fn add_at(&self, index: usize, item: T) {
        self.0.borrow_mut().insert(index, item)
    }

    pub fn add_all(&self, items: impl IntoIterator<Item = T>) {
        let items: Vec<T> = items.into_iter().collect();
        self.0.borrow_mut().extend(items)
    }

    pub fn add_all_at(&self, index: usize, items: impl IntoIterator<Item = T>) {
        let items: Vec<T> = items.into_iter().collect();
        self.0.borrow_mut().splice(index..index, items);
    }

    /// `set(index, item)`: returns the element it replaced.
    pub fn set(&self, index: usize, item: T) -> T {
        std::mem::replace(&mut self.0.borrow_mut()[index], item)
    }

    /// `remove(int index)`.
    pub fn remove_at(&self, index: usize) -> T {
        self.0.borrow_mut().remove(index)
    }

    pub fn clear(&self) {
        self.0.borrow_mut().clear()
    }

    /// Replace the whole content (`clear(); addAll(items)`).
    pub fn set_all(&self, items: Vec<T>) {
        *self.0.borrow_mut() = items;
    }
}

impl<T: Clone> JList<T> {
    pub fn get(&self, index: usize) -> T {
        self.0.borrow()[index].clone()
    }

    pub fn first(&self) -> Option<T> {
        self.0.borrow().first().cloned()
    }

    pub fn last(&self) -> Option<T> {
        self.0.borrow().last().cloned()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.0.borrow().clone()
    }

    /// Iterate a snapshot of the current content.
    pub fn iter(&self) -> std::vec::IntoIter<T> {
        self.to_vec().into_iter()
    }

    /// `new ArrayList<>(this)`: a distinct list with the same elements.
    pub fn copy(&self) -> JList<T> {
        JList::from(self.to_vec())
    }

    /// `subList(from, to)` materialized as a vector.
    pub fn sub_vec(&self, from: usize, to: usize) -> Vec<T> {
        self.0.borrow()[from..to].to_vec()
    }
}

impl<T: JavaEq> JList<T> {
    /// `indexOf(item)` by Java `equals`; -1 when absent.
    pub fn index_of(&self, item: &T) -> i32 {
        self.0.borrow().j_index_of(item)
    }

    pub fn contains(&self, item: &T) -> bool {
        self.index_of(item) >= 0
    }

    /// `remove(Object)`: drops the first element equal to `item`.
    pub fn remove_item(&self, item: &T) -> bool {
        self.0.borrow_mut().j_remove(item)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for JList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}

/// `List` operations that compare with Java `equals`, for plain vectors.
pub trait JavaListExt<T> {
    fn j_index_of(&self, item: &T) -> i32;
    fn j_last_index_of(&self, item: &T) -> i32;
    fn j_contains(&self, item: &T) -> bool;
}

impl<T: JavaEq> JavaListExt<T> for [T] {
    fn j_index_of(&self, item: &T) -> i32 {
        self.iter()
            .position(|x| x.java_equals(item))
            .map_or(-1, |i| i as i32)
    }

    fn j_last_index_of(&self, item: &T) -> i32 {
        self.iter()
            .rposition(|x| x.java_equals(item))
            .map_or(-1, |i| i as i32)
    }

    fn j_contains(&self, item: &T) -> bool {
        self.j_index_of(item) >= 0
    }
}

/// `List.remove(Object)` for plain vectors.
pub trait JavaVecExt<T> {
    fn j_remove(&mut self, item: &T) -> bool;
}

impl<T: JavaEq> JavaVecExt<T> for Vec<T> {
    fn j_remove(&mut self, item: &T) -> bool {
        match self.iter().position(|x| x.java_equals(item)) {
            Some(i) => {
                self.remove(i);
                true
            }
            None => false,
        }
    }
}
