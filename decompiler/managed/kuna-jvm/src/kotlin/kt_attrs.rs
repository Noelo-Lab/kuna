//! The Kotlin plugin's member attributes. Vineflower's plugin puts its own
//! objects into a member's attribute map (`KElement.KEY`, `PropertyMethod.KEY`,
//! `KConstructor.PRIMARY_CONSTRUCTOR`); the Rust attribute map only holds class
//! file attributes, so they live in this side table instead, keyed by member
//! and key name. No core code reads them. The table is per thread and has
//! session scope: the Kotlin plugin clears it when it closes.
//!
//! Writing a class marks members it reads again (`parse_with_companion`,
//! `mark_object_instance`). A single run processes every class before it
//! writes any, so processing never sees those marks; an engine the parallel
//! driver reuses would process its next batch after writing this one.
//! [`KtAttrs::marking`] records the marks and [`KtAttrs::undo_marks`] takes
//! them back at the end of a batch.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::jthrow;
use crate::key::Key;
use crate::struct_member::StructMember;

type Entry = (StructMember, String);
/// An attribute changed while marking, with the value it had.
type Mark = (Entry, Option<Rc<dyn Any>>);

thread_local! {
    static ATTRS: RefCell<BTreeMap<Entry, Rc<dyn Any>>> =
        const { RefCell::new(BTreeMap::new()) };
    static RECORDING: Cell<bool> = const { Cell::new(false) };
    static MARKS: RefCell<Vec<Mark>> = const { RefCell::new(Vec::new()) };
}

fn record(entry: Entry, previous: Option<Rc<dyn Any>>) {
    if RECORDING.with(Cell::get) {
        MARKS.with(|m| m.borrow_mut().push((entry, previous)));
    }
}

/// Ends a [`KtAttrs::marking`] however it is left.
struct Recording(bool);

impl Drop for Recording {
    fn drop(&mut self) {
        RECORDING.with(|r| r.set(self.0));
    }
}

pub struct KtAttrs;

impl KtAttrs {
    /// `member.getAttributes().put(key, value)`.
    pub fn put<T: Clone + 'static>(member: impl Into<StructMember>, key: &Key<T>, value: T) {
        let entry = (member.into(), key.name.clone());
        let previous = ATTRS.with(|a| a.borrow_mut().insert(entry.clone(), Rc::new(value)));
        record(entry, previous);
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

    /// Runs `f`, a class's writing marking members, recording every change
    /// for [`KtAttrs::undo_marks`].
    pub fn marking<R>(f: impl FnOnce() -> R) -> R {
        let _recording = Recording(RECORDING.with(|r| r.replace(true)));
        f()
    }

    /// Takes back every change recorded by [`KtAttrs::marking`], latest first.
    pub fn undo_marks() {
        let marks = MARKS.with(|m| std::mem::take(&mut *m.borrow_mut()));
        ATTRS.with(|a| {
            let mut attrs = a.borrow_mut();
            for (entry, previous) in marks.into_iter().rev() {
                match previous {
                    Some(value) => attrs.insert(entry, value),
                    None => attrs.remove(&entry),
                };
            }
        });
    }

    /// Drops every attribute of this thread.
    pub fn clear() {
        let dropped = ATTRS.with(|a| std::mem::take(&mut *a.borrow_mut()));
        drop(dropped);
        MARKS.with(|m| m.borrow_mut().clear());
    }
}
