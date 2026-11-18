//! kuna fix `kt_inner_class_constructor`: in Kotlin output, an `inner` class
//! constructed on the current instance is called by its simple name.
//!
//! An `inner` class's constructor takes its outer instance first.
//! `NewExprent.getQualifiedNewInstance` writes that instance as a qualifier
//! (`outer.Inner(..)`) unless it is the current class's `this`, where Java
//! writes `new Outer.Inner(..)` and the instance is implicit. `KNewExprent`
//! then writes `Outer.Inner(..)`, which Kotlin reads as the constructor of a
//! nested class and rejects for an `inner` one ("constructor of the inner
//! class can only be called with a receiver of the containing class"). With
//! the fix the call is `Inner(..)` when `Inner` is a member of the current
//! class: inside the class the simple name is that class, and the current
//! instance is the implicit receiver Kotlin passes, as Java does. An inner
//! class of a superclass keeps Vineflower's text.

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;

/// The class's simple name when the constructor call of class `classname`
/// with `params` builds an `inner` member class of the current class on its
/// `this`.
pub fn implicit_receiver_name(classname: &str, params: &[Exprent]) -> Option<String> {
    let node: ClassNode = DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&classname.to_string())
        .copied()?;
    let (type_, access) = node.with(|d| (d.type_, d.access));
    if type_ != ClassNodeType::Member || (access & ACC_STATIC) != 0 {
        return None;
    }
    let var = params.first()?.try_as_var()?;
    let current = DecompilerContext::get_current_class_node()?;
    if node.with(|d| d.parent) != Some(current) {
        return None;
    }
    let current_class = current.with(|d| d.class_struct);
    let this_classname = var
        .get_processor()?
        .get_this_vars()
        .borrow()
        .get_cloned(&var.get_var_version_pair());
    if this_classname.as_deref() != Some(current_class.qualified_name().as_str()) {
        return None;
    }
    node.with(|d| d.simple_name.clone())
}
