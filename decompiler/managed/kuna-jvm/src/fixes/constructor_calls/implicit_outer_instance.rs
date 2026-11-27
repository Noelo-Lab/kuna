//! Fix `implicit_outer_instance`: a creation of an inner class whose outer
//! instance is the one an unqualified `new Inner()` would pass, and whose null
//! check was not removed, is written unqualified.

use std::collections::VecDeque;

use super::{has_member, inner_member_declaring, this_class};
use crate::classes_processor::ClassNodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::new_exprent::NewExprent;
use crate::rt::JList;

/// After javac's outer-instance null check of `target` was removed before
/// `next`: marks every creation in `next` whose first argument is `target`,
/// or the stack variable holding it that `same` accepts (the value javac's
/// check left on the stack, not yet inlined there).
pub fn mark_checked(target: Exprent, next: Exprent, same: &dyn Fn(&JList<Exprent>) -> bool) {
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
        if !parameters.is_empty()
            && (parameters.get(0).equals(target)
                || (parameters.get(0).try_as_var().is_some_and(|v| v.is_stack())
                    && same(&parameters)))
        {
            new_expr.set_outer_checked();
        }
    }
}

/// Whether `new` of inner class `classname` with constructor arguments
/// `params` passes the outer instance an unqualified creation would.
pub fn is_implicit(new: NewExprent, classname: &str, params: &[Exprent]) -> bool {
    if new.is_outer_checked() || params.is_empty() {
        return false;
    }
    let Some(declaring) = inner_member_declaring(classname) else {
        return false;
    };
    let Some(outer) = this_class(params[0]) else {
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
