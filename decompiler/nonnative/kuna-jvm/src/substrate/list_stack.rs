//! Port of `org/jetbrains/java/decompiler/util/collections/ListStack.java`.
//!
//! The `ArrayList` it extends is reachable through `Deref<Target = Vec<T>>`;
//! the stack methods below shadow `Vec`'s like-named ones with Java's semantics.

use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListStack<T> {
    items: Vec<T>,
}

impl<T> Default for ListStack<T> {
    fn default() -> Self {
        ListStack::new()
    }
}

impl<T> Deref for ListStack<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.items
    }
}

impl<T> DerefMut for ListStack<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }
}

impl<T> From<Vec<T>> for ListStack<T> {
    fn from(items: Vec<T>) -> Self {
        ListStack { items }
    }
}

impl<T> ListStack<T> {
    pub fn new() -> Self {
        ListStack { items: Vec::new() }
    }

    /// Java: `ListStack(Collection)`.
    pub fn from_collection(list: impl IntoIterator<Item = T>) -> Self {
        ListStack {
            items: list.into_iter().collect(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> T {
        let last = self.items.len() - 1;
        self.items.remove(last)
    }

    /// Java: `pop(int count)`; `None` when `count <= 0`.
    pub fn pop_n(&mut self, count: i32) -> Option<T> {
        let mut o = None;
        let mut i = count;
        while i > 0 {
            o = Some(self.pop());
            i -= 1;
        }
        o
    }

    pub fn remove_multiple(&mut self, count: i32) {
        self.pop_n(count);
    }

    pub fn insert_by_offset(&mut self, offset: i32, item: T) {
        let at = (self.items.len() as i32 + offset) as usize;
        self.items.insert(at, item);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

impl<T: Clone> ListStack<T> {
    pub fn peek(&self) -> T {
        self.items[self.items.len() - 1].clone()
    }

    pub fn get(&self, index: usize) -> T {
        self.items[index].clone()
    }

    pub fn get_by_offset(&self, offset: i32) -> T {
        self.items[(self.items.len() as i32 + offset) as usize].clone()
    }
}
