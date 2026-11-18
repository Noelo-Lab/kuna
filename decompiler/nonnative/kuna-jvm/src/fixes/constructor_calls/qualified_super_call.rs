//! The kuna fix `qualified_super_call` (`kuna_fixes`), called from
//! `InvocationExprent::to_java`, `NewExprent::to_java` and
//! `SimplifyExprentsHelper::is_qualified_new_get_class`.
//!
//! A constructor of a class whose superclass `S` is an inner member class
//! passes `S`'s constructor an outer instance first, which
//! `ExprUtil.getSyntheticParametersMask` hides: Vineflower writes every such
//! call `super(..)`, so `o.super(3)` becomes `super(3)`, which passes the
//! enclosing instance (another object) or does not compile. javac
//! null-checks an explicit outer instance (`dup; requireNonNull; pop`) unless
//! it is a `this` or a `new`, and Vineflower keeps that check as a statement
//! before the call. With the fix, the check is removed and marks the call
//! ([`mark_checked`]), and a marked call, or one whose outer instance is not
//! the instance of the innermost enclosing class having `S` as a member, is
//! written `o.super(..)`. An anonymous class created by `o.new S(..) {..}`
//! takes `o` as a constructor parameter of its own that its constructor passes
//! to `S`'s: that creation is written qualified, without the parameter. A
//! superclass outside the input that the class's InnerClasses attribute names
//! an inner member class gets the same treatment, and its outer instance,
//! which Vineflower writes as the first argument (`super(Outer.this, ctx)`),
//! is left out of the list.

use std::collections::VecDeque;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::expr_util::ExprUtil;
use crate::exprent::Exprent;
use crate::invocation_exprent::{InvocationExprent, Type as InvocationKind};
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::text_buffer::TextBuffer;

fn class_node(name: &str) -> Option<ClassNode> {
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()
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

fn has_member(class: &str, declaring: &str) -> bool {
    class == declaring || DecompilerContext::get_struct_context().instance_of(class, declaring)
}

/// Whether `arg` is the instance of the innermost class enclosing the current
/// one that has the member class declared in `declaring`.
fn is_implicit(arg: Exprent, declaring: &str) -> bool {
    let Some(var) = arg.try_as_var() else {
        return false;
    };
    let Some(processor) = var.get_processor() else {
        return false;
    };
    let Some(outer) = processor
        .get_this_vars()
        .borrow()
        .get_cloned(&var.get_var_version_pair())
    else {
        return false;
    };
    let mut scope = DecompilerContext::get_current_class_node().and_then(|n| n.with(|d| d.parent));
    while let Some(cn) = scope {
        let (cn_type, cn_class, cn_parent) =
            cn.with(|d| (d.type_, d.class_struct.qualified_name(), d.parent));
        if cn_type != ClassNodeType::Lambda && has_member(&cn_class, declaring) {
            return cn_class == outer;
        }
        scope = cn_parent;
    }
    false
}

/// The class declaring `class` when the current class's InnerClasses
/// attribute lists it as an inner member class and it is not in the input.
fn external_inner_member_declaring(class: &str) -> Option<String> {
    if class_node(class).is_some() {
        return None;
    }
    let current = DecompilerContext::get_current_class_node()?.with(|d| d.class_struct);
    let attr = current.get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES)?;
    attr.get_entries()
        .iter()
        .find(|e| {
            e.inner_name == class && e.simple_name.is_some() && (e.access_flags & ACC_STATIC) == 0
        })
        .and_then(|e| e.enclosing_name.clone())
}

/// How a superclass constructor call `call` (written `super(..)`) passes
/// its inner member superclass's outer instance: the instance when it is
/// written as the qualifier (not the one an unqualified call passes), and
/// whether the first argument is to be left out of the list, which Vineflower
/// does only for a superclass of the input.
pub fn super_outer(call: InvocationExprent) -> Option<(Option<Exprent>, bool)> {
    let classname = call.get_classname();
    let params = call.get_lst_parameters();
    if params.is_empty() {
        return None;
    }
    let (declaring, hide) = match inner_member_declaring(&classname) {
        Some(declaring) => {
            let mask = ExprUtil::get_synthetic_parameters_mask(
                class_node(&classname)?,
                &call.get_string_descriptor(),
                params.len() as i32,
            )?;
            if mask.first().is_none_or(|m| m.is_none()) {
                return None;
            }
            (declaring, false)
        }
        None => (external_inner_member_declaring(&classname)?, true),
    };
    let arg = params.get(0);
    let qualifier = call.is_kuna_checked_outer() || !is_implicit(arg, &declaring);
    Some((qualifier.then_some(arg), hide))
}

/// `arg` written as the qualifier of `.super(..)` or `.new`.
pub fn qualifier_text(arg: Exprent, indent: i32) -> TextBuffer {
    let mut text = arg.to_java(indent);
    let plain_new = arg
        .try_as_new()
        .is_some_and(|n| !n.is_anonymous() && !n.is_lambda() && !n.is_method_reference());
    if arg.get_precedence() > 0 && !plain_new {
        text.enclose_with_parens();
    }
    text
}

/// For the creation of anonymous class `child` by `constructor`: the position
/// of the argument its constructor passes to its inner member superclass's
/// constructor as the outer instance, when that is a parameter of its own and
/// not its outer instance (the creation is `o.new S(..) {..}`).
pub fn anonymous_qualifier(child: ClassNode, constructor: InvocationExprent) -> Option<usize> {
    let super_call = child.with(|d| d.super_invocation)?;
    let super_type = child.with(|d| d.anonymous_class_type.clone())?;
    let super_class = super_type.value.as_deref()?.to_string();
    if super_call.get_classname() != super_class || inner_member_declaring(&super_class).is_none() {
        return None;
    }
    if child
        .get_wrapper()?
        .get_class_struct()
        .get_signature()
        .is_some()
    {
        return None;
    }
    let super_params = super_call.get_lst_parameters();
    if super_params.is_empty() {
        return None;
    }
    let slot = super_params.get(0).try_as_var()?.get_index();
    let descriptor = constructor.get_descriptor();
    let args = constructor.get_lst_parameters();
    if args.len() != descriptor.params.len() {
        return None;
    }
    let mut next = 1;
    let mut position = None;
    for (i, param) in descriptor.params.iter().enumerate() {
        if next == slot {
            position = Some(i);
            break;
        }
        next += param.stack_size;
    }
    let position = position?;
    let mask = ExprUtil::get_synthetic_parameters_mask(
        child,
        &constructor.get_string_descriptor(),
        args.len() as i32,
    );
    if mask.is_some_and(|m| m.get(position).is_none_or(|p| p.is_some())) {
        return None;
    }
    Some(position)
}

/// After javac's null check of `target` before `next`: whether `next` is the
/// superclass constructor call or anonymous class creation whose outer
/// instance it checks; marks a superclass constructor call.
pub fn mark_checked(target: Exprent, next: Exprent) -> bool {
    if let Some(call) = next.try_as_invocation() {
        if is_super_call(call) && is_inner_member(&call.get_classname()) {
            let params = call.get_lst_parameters();
            if !params.is_empty() && params.get(0).equals(target) {
                call.set_kuna_checked_outer();
                return true;
            }
        }
    }
    let mut queue: VecDeque<Exprent> = VecDeque::new();
    queue.push_back(next);
    while let Some(expr) = queue.pop_front() {
        queue.extend(expr.get_all_exprents());
        let Some(new_expr) = expr.try_as_new() else {
            continue;
        };
        if !new_expr.is_anonymous() || new_expr.is_lambda() {
            continue;
        }
        let Some(class) = new_expr.get_new_type().value.clone() else {
            continue;
        };
        let Some(super_class) = class_node(&class).and_then(|n| {
            n.with(|d| d.anonymous_class_type.clone())
                .and_then(|t| t.value.clone())
        }) else {
            continue;
        };
        if inner_member_declaring(&super_class).is_none() {
            continue;
        }
        if let Some(constructor) = new_expr.get_constructor() {
            if constructor
                .get_lst_parameters()
                .iter()
                .any(|p| p.equals(target))
            {
                return true;
            }
        }
    }
    false
}

/// After javac's null check of `target` before `next`, when no construct of
/// `next` takes `target` itself: whether `next` is a superclass constructor
/// call whose outer instance (another copy of the checked value) has
/// `target`'s type, the test `isQualifiedNewGetClass` makes for a `new`;
/// marks the call.
pub fn mark_checked_copy(target: Exprent, next: Exprent, is_static: bool) -> bool {
    let Some(call) = next.try_as_invocation() else {
        return false;
    };
    if !is_super_call(call) || !is_inner_member(&call.get_classname()) {
        return false;
    }
    let params = call.get_lst_parameters();
    if params.is_empty()
        || !params
            .get(0)
            .get_expr_type()
            .equals(&target.get_expr_type())
    {
        return false;
    }
    if is_static
        && (1..params.len()).any(|i| {
            params
                .get(i)
                .get_expr_type()
                .equals(&target.get_expr_type())
        })
    {
        return false;
    }
    call.set_kuna_checked_outer();
    true
}

fn is_inner_member(class: &str) -> bool {
    inner_member_declaring(class).is_some() || external_inner_member_declaring(class).is_some()
}

fn is_super_call(call: InvocationExprent) -> bool {
    if call.get_functype() != InvocationKind::Init || call.is_static() {
        return false;
    }
    if !call
        .get_instance()
        .and_then(|i| i.try_as_var())
        .is_some_and(|v| v.get_index() == 0)
    {
        return false;
    }
    DecompilerContext::get_current_class()
        .and_then(|c| c.super_class())
        .and_then(|s| s.get_string_opt().map(|s| s.to_string()))
        .is_some_and(|s| s == call.get_classname())
}
