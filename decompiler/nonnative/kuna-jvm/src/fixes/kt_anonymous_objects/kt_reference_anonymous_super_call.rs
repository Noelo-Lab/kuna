//! Fix `kt_reference_anonymous_super_call`: a function-reference class that is
//! an anonymous class is read from the super call its node keeps (an empty first
//! constructor statement stands for it), and its signature constant is split as
//! `jvmName(descriptor)`.

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::exprent::Exprent;
use crate::rt::{JList, JavaStr};

/// The `super(..)` call of the reference class `node`'s constructor: the only
/// exprent of its first statement, or, when `NestedClassProcessor` has moved
/// it out of an anonymous class's constructor, the call it kept.
pub fn super_call(node: ClassNode, exprents: Option<JList<Exprent>>) -> Option<Exprent> {
    match exprents {
        Some(exprents) if exprents.len() == 1 => Some(exprents.get(0)),
        Some(exprents) if exprents.is_empty() => {
            let (type_, kept) = node.with(|d| (d.type_, d.super_invocation));
            if type_ == ClassNodeType::Anonymous {
                kept.map(|call| call.0)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// The JVM name and descriptor of the method a reference named `name` with
/// the signature constant `signature` refers to.
pub fn reference_name_and_descriptor(name: &str, signature: &str) -> (String, String) {
    let split = split_signature(signature);
    split.unwrap_or_else(|| (name.to_string(), signature.jsubstring_from(name.jlen())))
}

/// The JVM name and descriptor of a reference's signature constant
/// `jvmName(descriptor)ret`, whose name is the Kotlin name or, for a function
/// taking an inline-class value, that name mangled as `name-<hash>`.
pub fn split_signature(signature: &str) -> Option<(String, String)> {
    let at = signature.find('(')?;
    Some((signature[..at].to_string(), signature[at..].to_string()))
}
