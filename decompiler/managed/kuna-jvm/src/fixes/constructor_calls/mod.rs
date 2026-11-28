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
use crate::method_descriptor::MethodDescriptor;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_version_pair::VarVersionPair;

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

/// The class that `class`, a class outside the input, is an inner
/// (non-static) member of, as the InnerClasses attribute of the class being
/// written lists it (JVMS 4.7.6: a class lists every nested class its
/// constant pool names, with its outer class, simple name and flags).
pub(crate) fn listed_inner_member(class: &str) -> Option<String> {
    if class_node(class).is_some() {
        return None;
    }
    let current = DecompilerContext::get_current_class_node()?.with(|d| d.class_struct);
    let attribute = current.get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES)?;
    let entry = attribute.get_entries().into_iter().find(|e| e.inner_name == class)?;
    if entry.simple_name.is_none() || (entry.access_flags & ACC_STATIC) != 0 {
        return None;
    }
    entry.enclosing_name
}

/// The synthetic-parameter mask of a creation of `class` with constructor
/// `descriptor` and `count` arguments when `class` is a listed inner member
/// of a class outside the input: its constructor's first parameter is the
/// outer instance.
pub fn listed_outer_instance_mask(
    class: &str,
    descriptor: &str,
    count: usize,
) -> Option<Vec<Option<VarVersionPair>>> {
    let outer = listed_inner_member(class)?;
    let md = MethodDescriptor::parse_descriptor(descriptor);
    let first = md.params.first()?;
    if count == 0 || first.array_dim != 0 || first.value.as_deref() != Some(outer.as_str()) {
        return None;
    }
    let mut mask = vec![None; count];
    mask[0] = Some(VarVersionPair::new(-1, 0));
    Some(mask)
}
