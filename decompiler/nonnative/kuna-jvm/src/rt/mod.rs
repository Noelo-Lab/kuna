//! The Java-runtime kernel the ported engine stands on.
//!
//! Vineflower is written against Java's object model: objects with identity,
//! shared mutable collections, hash collections whose iteration order leaks
//! into the output, UTF-16 strings and unchecked exceptions. This module gives
//! each of those one faithful Rust shape so the port can follow the original
//! line for line.

pub mod arena;
pub mod bitset;
pub mod exception;
pub mod jchar;
pub mod jhash;
pub mod jlist;
pub mod jnum;
pub mod jstring;
pub mod stack;

pub use arena::{reset_class_scope, reset_session_scope};
pub use bitset::BitSet;
pub use exception::{rethrow, throw, try_catch, Caught, JavaException};
pub use jhash::{
    objects_equals, objects_hash, JHashMap, JHashSet, JLinkedHashMap, JLinkedHashSet, JavaEq,
    JavaHash,
};
pub use jlist::{JList, JavaListExt, JavaVecExt, Shared};
pub use jstring::JavaStr;
