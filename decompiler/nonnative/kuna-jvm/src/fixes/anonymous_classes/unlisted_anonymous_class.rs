//! Fix `unlisted_anonymous_class`: a top-level class that lists itself as
//! anonymous is nested, after `ClassesProcessor`'s nesting, in the class its
//! `EnclosingMethod` names; the locals its constructor takes stay locals, and
//! its captured variables take the creating method's names.

use super::is_within;
use crate::classes_processor::{sort_nodes, ClassNode, ClassNodeType, ClassesProcessor};
use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::invocation_exprent::Type as InvocationKind;
use crate::kotlin_options::KotlinOptions;
use crate::method_wrapper::MethodWrapper;
use crate::new_exprent::NewExprent;
use crate::rt::{JHashMap, JList, Shared};
use crate::struct_class::StructClass;
use crate::struct_enclosing_method_attribute::StructEnclosingMethodAttribute;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

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
        let Some(attr) = cl.get_attribute(StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD)
        else {
            continue;
        };
        let Some(enclosing) = attr.get_class_name() else {
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
        if written_without_arguments(cl) || created_in_hidden_method(enclosing_cl, attr) {
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

/// Whether the local `var`, defined by an assignment before `new` (`index` is
/// the position of `new`'s statement in `list`), is what an argument of
/// `new`'s constructor call reads, directly or through a stack variable that
/// copies it: `new` creates a class this fix nested. `reads_def` tells
/// whether a variable version reads `var`'s definition.
pub fn takes_local(
    node: ClassNode,
    new: NewExprent,
    list: &JList<Exprent>,
    index: i32,
    var: VarExprent,
    reads_def: &dyn Fn(VarVersionPair) -> bool,
) -> bool {
    if var.is_stack() || !is_unlisted_nested(node) {
        return false;
    }
    let name = node.with(|d| d.class_struct).qualified_name();
    let exprents = list.to_vec();
    let call = new.get_constructor().or_else(|| {
        exprents.iter().skip(index.max(0) as usize).find_map(|e| {
            e.get_all_exprents_rec_self(true, true)
                .into_iter()
                .filter_map(|e| e.try_as_invocation())
                .find(|inv| {
                    inv.get_functype() == InvocationKind::Init && inv.get_classname() == name
                })
        })
    });
    let copies: Vec<VarVersionPair> = exprents
        .iter()
        .filter_map(|e| e.try_as_assignment())
        .filter(|a| {
            a.get_right()
                .try_as_var()
                .is_some_and(|r| !r.is_stack() && reads_def(r.get_var_version_pair()))
        })
        .filter_map(|a| a.get_left().try_as_var())
        .filter(|l| l.is_stack())
        .map(|l| l.get_var_version_pair())
        .collect();
    call.is_some_and(|c| {
        c.get_lst_parameters()
            .to_vec()
            .into_iter()
            .filter_map(|arg| arg.try_as_var())
            .any(|arg| {
                let pair = arg.get_var_version_pair();
                if arg.is_stack() {
                    copies.contains(&pair)
                } else {
                    reads_def(pair)
                }
            })
    })
}

/// The debug entry of a captured variable of `node`, which holds the
/// variable `pair` of the method `enclosing` that creates it, named `name`
/// there: when `node` is a class this fix nested and the variable's first
/// assignment in `enclosing` has no entry of its own, `enclosing` declares
/// it by `name`, so the entry (from the variable's later reads) is renamed to
/// it; otherwise `lvt` as it is.
pub fn captured_lvt(
    node: ClassNode,
    enclosing: MethodWrapper,
    pair: VarVersionPair,
    name: Option<&str>,
    lvt: Option<LocalVariable>,
) -> Option<LocalVariable> {
    let (Some(name), Some(entry)) = (name, &lvt) else {
        return lvt;
    };
    if entry.get_name() == name || !is_unlisted_nested(node) {
        return lvt;
    }
    let Some(graph) = enclosing.get_or_build_graph() else {
        return lvt;
    };
    let mut declared_unnamed = false;
    graph.iterate_exprents_deep(|e: Exprent| {
        let Some(left) = e
            .try_as_assignment()
            .and_then(|a| a.get_left().try_as_var())
        else {
            return 0;
        };
        if left.get_var_version_pair() != pair {
            return 0;
        }
        declared_unnamed = left.get_lvt().is_none();
        1
    });
    if declared_unnamed {
        Some(entry.rename(name.to_string()))
    } else {
        lvt
    }
}

/// Whether `node` is an anonymous class nested in a class whose
/// `InnerClasses` attribute does not list it (this fix's, or one
/// `anonymous_class_created_elsewhere` moved to its creator).
fn is_unlisted_nested(node: ClassNode) -> bool {
    let (type_, parent, cl) = node.with(|d| (d.type_, d.parent, d.class_struct));
    let name = cl.qualified_name();
    type_ == ClassNodeType::Anonymous
        && parent.is_some_and(|p| {
            !p.with(|d| d.class_struct)
                .get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES)
                .is_some_and(|inner| inner.get_entries().iter().any(|e| e.inner_name == name))
        })
}

/// Whether the writer would write `cl` at its `new` without the values it is
/// given: the Kotlin writer cannot write a suspend lambda and writes its
/// failure comment instead, so such a copy stays a class of its own.
fn written_without_arguments(cl: StructClass) -> bool {
    DecompilerContext::get_option(KotlinOptions::DECOMPILE_KOTLIN)
        && cl.super_class().is_some_and(|s| {
            matches!(
                &*s.get_string(),
                "kotlin/coroutines/jvm/internal/SuspendLambda"
                    | "kotlin/coroutines/jvm/internal/RestrictedSuspendLambda"
            )
        })
}

/// Whether the method `attr` names, where the class is created, is not
/// written: a synthetic method (kotlinc's reified `inline fun`s) under
/// `remove-synthetic`; nested there, the class would not be written at all.
fn created_in_hidden_method(
    enclosing_cl: StructClass,
    attr: StructEnclosingMethodAttribute,
) -> bool {
    let (Some(name), Some(descriptor)) = (attr.get_method_name(), attr.get_method_descriptor())
    else {
        return false;
    };
    DecompilerContext::get_option(IFernflowerPreferences::REMOVE_SYNTHETIC)
        && enclosing_cl
            .get_method(&name, &descriptor)
            .is_some_and(|mt| mt.is_synthetic())
}

