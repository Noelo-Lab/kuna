//! Port of `org/jetbrains/java/decompiler/util/InterpreterUtil.java`.
//!
//! The file and stream helpers (`copyFile`, `copyStream`, `getBytes`,
//! `readBytes`, `discardBytes`) served only the replaced console and
//! class-path front ends and are not ported.

use crate::rt::{JHashSet, JavaEq, JavaHash};

pub struct InterpreterUtil;

impl InterpreterUtil {
    pub const EMPTY_INT_ARRAY: [i32; 0] = [];

    pub fn equal_sets<T: JavaHash + JavaEq + Clone>(c1: Option<&[T]>, c2: Option<&[T]>) -> bool {
        let (c1, c2) = match (c1, c2) {
            (None, c2) => return c2.is_none(),
            (Some(_), None) => return false,
            (Some(c1), Some(c2)) => (c1, c2),
        };
        if c1.len() != c2.len() {
            return false;
        }
        let mut set = JHashSet::from_collection(c1.iter().cloned());
        for item in c2 {
            set.remove(item);
        }
        set.is_empty()
    }

    pub fn equal_objects<T: JavaEq + ?Sized>(first: Option<&T>, second: Option<&T>) -> bool {
        match first {
            None => second.is_none(),
            Some(first) => second.is_some_and(|second| first.java_equals(second)),
        }
    }

    pub fn equal_lists<T: JavaEq>(first: Option<&[T]>, second: Option<&[T]>) -> bool {
        let (first, second) = match (first, second) {
            (None, second) => return second.is_none(),
            (Some(_), None) => return false,
            (Some(first), Some(second)) => (first, second),
        };
        if first.len() == second.len() {
            for i in 0..first.len() {
                if !InterpreterUtil::equal_objects(Some(&first[i]), Some(&second[i])) {
                    return false;
                }
            }
            return true;
        }
        false
    }

    /// Java: `makeUniqueKey(String, String)`.
    pub fn make_unique_key(name: &str, descriptor: &str) -> String {
        format!("{name} {descriptor}")
    }

    /// Java: `makeUniqueKey(String, String, String)`.
    pub fn make_unique_key_3(name: &str, descriptor1: &str, descriptor2: &str) -> String {
        format!("{name} {descriptor1} {descriptor2}")
    }
}
