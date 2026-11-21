//! Fix family `constructor_calls`: outer instances, qualified `super` calls,
//! split constructor calls and javac's null checks around them.

pub(crate) mod implicit_outer_instance;
pub(crate) mod qualified_super_call;
pub(crate) mod split_constructor_call;
pub(crate) mod synthetic_null_check_shape;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;

fn class_node(name: &str) -> Option<ClassNode> {
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()
}

fn has_member(class: &str, declaring: &str) -> bool {
    class == declaring || DecompilerContext::get_struct_context().instance_of(class, declaring)
}

/// The class declaring `class` when it is an inner (non-static) member class
/// of the input.
fn inner_member_declaring(class: &str) -> Option<String> {
    let node = class_node(class)?;
    let (type_, access, parent) = node.with(|d| (d.type_, d.access, d.parent));
    if type_ != ClassNodeType::Member || (access & ACC_STATIC) != 0 {
        return None;
    }
    parent.map(|p| p.with(|d| d.class_struct.qualified_name()))
}

/// The class whose `this` the variable `arg` holds, when it holds one.
fn this_class(arg: Exprent) -> Option<String> {
    let var = arg.try_as_var()?;
    var.get_processor()?
        .get_this_vars()
        .borrow()
        .get_cloned(&var.get_var_version_pair())
}
