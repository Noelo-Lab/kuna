//! The Kotlin plugin's member attributes. Vineflower's plugin puts its own
//! objects into a member's attribute map (`KElement.KEY`, `PropertyMethod.KEY`,
//! `KConstructor.PRIMARY_CONSTRUCTOR`); the Rust attribute map only holds class
//! file attributes, so they live in this side table instead, keyed by member
//! and key name. No core code reads them. The table is per thread and has
//! session scope: the Kotlin plugin clears it when it closes.

use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::jthrow;
use crate::key::Key;
use crate::struct_member::StructMember;

thread_local! {
    static ATTRS: RefCell<BTreeMap<(StructMember, String), Rc<dyn Any>>> =
        const { RefCell::new(BTreeMap::new()) };
}

pub struct KtAttrs;

impl KtAttrs {
    /// `member.getAttributes().put(key, value)`.
    pub fn put<T: Clone + 'static>(member: impl Into<StructMember>, key: &Key<T>, value: T) {
        let entry = (member.into(), key.name.clone());
        ATTRS.with(|a| a.borrow_mut().insert(entry, Rc::new(value)));
    }

    /// `member.getAttribute(key)`; a value of another type is a `ClassCastException`.
    pub fn get<T: Clone + 'static>(member: impl Into<StructMember>, key: &Key<T>) -> Option<T> {
        let entry = (member.into(), key.name.clone());
        let value = ATTRS.with(|a| a.borrow().get(&entry).cloned())?;
        match value.downcast_ref::<T>() {
            Some(v) => Some(v.clone()),
            None => jthrow!(ClassCastException; "attribute {} has another type", key.name),
        }
    }

    /// `member.hasAttribute(key)`.
    pub fn has<T>(member: impl Into<StructMember>, key: &Key<T>) -> bool {
        let entry = (member.into(), key.name.clone());
        ATTRS.with(|a| a.borrow().contains_key(&entry))
    }

    /// `member.getAttributes().remove(key)`; whether there was a value.
    pub fn remove<T>(member: impl Into<StructMember>, key: &Key<T>) -> bool {
        let entry = (member.into(), key.name.clone());
        ATTRS.with(|a| a.borrow_mut().remove(&entry).is_some())
    }

    /// Drops every attribute of this thread.
    pub fn clear() {
        let dropped = ATTRS.with(|a| std::mem::take(&mut *a.borrow_mut()));
        drop(dropped);
    }

    /// How many attributes are set on this thread.
    pub fn len() -> usize {
        ATTRS.with(|a| a.borrow().len())
    }
}
