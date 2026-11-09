//! kuna fix `kt_reference_anonymous_super_call`: the superclass
//! constructor call of a function-reference class that is an anonymous class.
//!
//! `KotlinWriter.stringifyReference` reads what a reference names (owner,
//! name, signature) from the constants of the class's `super(..)` call, which
//! it expects as the only exprent of the constructor's first statement. When
//! the class is an anonymous class (its `InnerClasses` entry has no name, and
//! `verify-anonymous-classes` is off, which is the default),
//! `NestedClassProcessor.insertLocalVars` has already taken that call out of
//! the constructor and kept it as the node's `superInvocation` (Java writes it
//! back into `new Super(..) { }`), so the constructor reads empty, the
//! reference is not recognised, and the `new` falls through to the
//! anonymous-object path, which throws for a class without Class metadata (a
//! `FieldExprent` of the class's `INSTANCE` prints `<unrepresentable>.INSTANCE`).
//! With the fix an empty first statement of an anonymous class's constructor
//! stands for the super call the node keeps.

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::exprent::Exprent;
use crate::rt::JList;

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
