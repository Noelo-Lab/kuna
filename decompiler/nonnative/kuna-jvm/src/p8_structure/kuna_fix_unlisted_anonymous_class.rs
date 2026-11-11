//! kuna fix `unlisted_anonymous_class`: an anonymous class that its
//! enclosing class does not list among its inner classes is nested in it.
//!
//! `ClassesProcessor` nests a class only from the `InnerClasses` attribute of
//! the class that encloses it, starting from the classes no other class lists.
//! kotlinc copies the anonymous objects and lambdas of an inline function into
//! a new class (`Foo$bar$$inlined$map$1`) at every call site that inlines it;
//! the copy lists itself as anonymous and names the method it was inlined
//! into (`EnclosingMethod`), but that method's class does not list it, and
//! the copy's own nested classes (its continuation) list it back, which also
//! keeps it from being a starting point. Vineflower then writes the copy, and
//! its nested classes, as top-level classes: its captured values stay fields
//! that its code reads (`this$0`, `$name`, `$block$inlined`) and the method
//! that inlined it calls its constructor by its `$` name.
//!
//! With the fix, after that nesting, a class that is still a top-level class
//! although it lists itself as anonymous (no simple name, no outer class) is
//! nested in the class its `EnclosingMethod` names, as an anonymous class, the
//! way the enclosing class's entry would have nested it; so are its own
//! anonymous classes that listed it. `NestedClassProcessor` then maps its
//! captured values at the `new` like any anonymous class's, and the writers
//! write it where it is created.

use crate::classes_processor::{sort_nodes, ClassNode, ClassNodeType, ClassesProcessor};
use crate::code_constants::ACC_STATIC;
use crate::rt::{JHashMap, Shared};
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_type::VarType;

/// Nests every class of `classes` (all class nodes, by name) that is still a
/// top-level class but lists itself as anonymous in the class its
/// `EnclosingMethod` names.
pub fn nest_in_enclosing_class(
    classes: &Shared<JHashMap<String, ClassNode>>,
    verify_anonymous_classes: bool,
) {
    let mut nodes: Vec<(String, ClassNode)> = classes
        .borrow()
        .iter()
        .map(|(name, node)| (name.clone(), *node))
        .collect();
    nodes.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, node) in nodes {
        let (type_, parent, cl) = node.with(|d| (d.type_, d.parent, d.class_struct));
        if type_ != ClassNodeType::Root || parent.is_some() {
            continue;
        }
        let Some(entry) = cl
            .get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES)
            .and_then(|inner| {
                inner
                    .get_entries()
                    .into_iter()
                    .find(|e| e.inner_name == name)
            })
        else {
            continue;
        };
        if entry.simple_name_idx != 0 || entry.outer_name_idx != 0 {
            continue;
        }
        let Some(enclosing) = cl
            .get_attribute(StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD)
            .and_then(|attr| attr.get_class_name())
        else {
            continue;
        };
        if enclosing == name {
            continue;
        }
        let Some(enclosing_node) = classes.borrow().get_cloned(&enclosing) else {
            continue;
        };
        if is_within(enclosing_node, node) {
            continue;
        }
        let enclosing_cl = enclosing_node.with(|d| d.class_struct);
        if verify_anonymous_classes && !ClassesProcessor::is_anonymous(cl, enclosing_cl) {
            continue;
        }
        let interfaces = cl.get_interfaces();
        let Some(anonymous_class_type) = (if !interfaces.is_empty() {
            Some(VarType::new_signature_cl(&cl.get_interface(0), true))
        } else {
            cl.super_class()
                .map(|s| VarType::new_signature_cl(&s.get_string(), true))
        }) else {
            continue;
        };
        node.with_mut(|d| {
            d.simple_name = None;
            d.type_ = ClassNodeType::Anonymous;
            d.enclosing_method = None;
            d.access = entry.access_flags & !ACC_STATIC;
            d.anonymous_class_type = Some(anonymous_class_type);
            d.parent = Some(enclosing_node);
        });
        node.with(|d| d.enclosing_classes.clone())
            .borrow_mut()
            .add(enclosing.clone());
        let nested = enclosing_node.with(|d| d.nested.clone());
        nested.add(node);
        sort_nodes(&nested);
    }
}

/// Whether `node` is `ancestor` or nested in it.
fn is_within(node: ClassNode, ancestor: ClassNode) -> bool {
    let mut current = Some(node);
    while let Some(n) = current {
        if n == ancestor {
            return true;
        }
        current = n.with(|d| d.parent);
    }
    false
}
