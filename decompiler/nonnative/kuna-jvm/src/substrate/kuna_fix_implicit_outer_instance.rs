//! The kuna fix `implicit_outer_instance` (`kuna_fixes`), called from
//! `NewExprent::to_java` and `SimplifyExprentsHelper::is_qualified_new_get_class`.
//!
//! Vineflower writes the creation of an inner class unqualified only when its
//! outer instance is the current class's `this`; any enclosing class's
//! instance (`Outer.this`, from `this.this$0` or a constructor's outer
//! parameter) is written `Outer.this.new Inner()`. javac compiles that
//! qualified form with a null check of the outer instance
//! (`Objects.requireNonNull`, `getClass()` before Java 9), which
//! `is_qualified_new_get_class` removes, and an unqualified `new Inner()`
//! without one. With the fix, a creation whose null check was not removed
//! ([`mark_checked`]) and whose outer instance is the instance of the
//! innermost lexically enclosing class that has `Inner` as a member, which is
//! the instance an unqualified `new Inner()` passes, is written unqualified.

use std::collections::VecDeque;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::new_exprent::NewExprent;

/// After javac's outer-instance null check of `target` was removed before
/// `next`: marks every creation in `next` whose first argument is `target`.
pub fn mark_checked(target: Exprent, next: Exprent) {
    let mut queue: VecDeque<Exprent> = VecDeque::new();
    queue.push_back(next);
    while let Some(expr) = queue.pop_front() {
        queue.extend(expr.get_all_exprents());
        let Some(new_expr) = expr.try_as_new() else {
            continue;
        };
        let Some(constructor) = new_expr.get_constructor() else {
            continue;
        };
        let parameters = constructor.get_lst_parameters();
        if !parameters.is_empty() && parameters.get(0).equals(target) {
            new_expr.set_outer_checked();
        }
    }
}

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

/// Whether `new` of inner class `classname` with constructor arguments
/// `params` passes the outer instance an unqualified creation would.
pub fn is_implicit(new: NewExprent, classname: &str, params: &[Exprent]) -> bool {
    if new.is_outer_checked() || params.is_empty() {
        return false;
    }
    let Some(node) = class_node(classname) else {
        return false;
    };
    let (type_, access, parent) = node.with(|d| (d.type_, d.access, d.parent));
    if type_ != ClassNodeType::Member || (access & ACC_STATIC) != 0 {
        return false;
    }
    let Some(declaring) = parent.map(|p| p.with(|d| d.class_struct.qualified_name())) else {
        return false;
    };
    let Some(var) = params[0].try_as_var() else {
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
    let mut scope = DecompilerContext::get_current_class_node();
    while let Some(cn) = scope {
        let (cn_type, cn_class, cn_parent) =
            cn.with(|d| (d.type_, d.class_struct.qualified_name(), d.parent));
        if cn_class == outer {
            return has_member(&outer, &declaring);
        }
        if cn_type != ClassNodeType::Lambda && has_member(&cn_class, &declaring) {
            return false;
        }
        scope = cn_parent;
    }
    false
}
