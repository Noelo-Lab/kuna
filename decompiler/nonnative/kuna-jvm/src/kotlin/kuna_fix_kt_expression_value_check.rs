//! kuna fix `kt_expression_value_check` (spec 10.16): kotlinc's check of a
//! platform-typed value stored into a property goes before the last
//! stack-variable pass.
//!
//! Where a Java method's result (a platform type) is stored into a non-null
//! Kotlin property, kotlinc keeps the value and checks it:
//! `e; dup; Intrinsics.checkNotNullExpressionValue(e, "source text"); putfield`
//! (`checkExpressionValueIsNotNull` before Kotlin 1.4). The source has no code
//! for it, and kotlinc inserts it again for the property's non-null type.
//! Vineflower's Kotlin plugin deletes the 1.4 call after the stack variables
//! are simplified, so the stack variable that held the value stays
//! (`val var10000: Logger = ...; logger = var10000`), and keeps the 1.3 call;
//! in a static initializer either stops `InitializerProcessor`, and the
//! property loses its initializer to a leftover `<clinit>`. Here the check of
//! a stack variable that the next statement stores into a field is deleted
//! just before the last stack-variable pass, which then writes the value into
//! the store. Checks of values that go elsewhere are left as they were, since
//! there the check is all that keeps the value's non-null contract.

use crate::exprent::Exprent;
use crate::kotlin::kuna_fix_kt_not_null_assertion;
use crate::root_statement::RootStatement;
use crate::statement::Statement;

const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
const CHECKS: [&str; 2] = ["checkNotNullExpressionValue", "checkExpressionValueIsNotNull"];

/// Deletes the expression-value checks of values stored into fields under `root`.
pub fn remove(root: RootStatement) -> bool {
    let mut res = false;
    visit(root.0, &mut res);
    res
}

fn visit(stat: Statement, res: &mut bool) {
    let stats = stat.get_stats().borrow().to_vec();
    for st in stats {
        visit(st, res);
    }
    let Some(exprents) = stat.get_exprents() else {
        return;
    };
    let mut i = exprents.len();
    while i > 1 {
        i -= 1;
        if i + 1 < exprents.len()
            && is_stored_check(exprents.get(i - 1), exprents.get(i), exprents.get(i + 1))
        {
            exprents.remove_at(i);
            *res = true;
        }
    }
}

/// Whether `check` is `Intrinsics.checkNotNullExpressionValue(v, "...")` (or
/// its Kotlin 1.3 name) of the stack variable `v` that `prev` assigns (or of
/// the value read `prev` assigns it) and `next` stores `v` into a field.
fn is_stored_check(prev: Exprent, check: Exprent, next: Exprent) -> bool {
    let Some(inv) = check.try_as_invocation() else {
        return false;
    };
    let is_check = inv.is_static()
        && inv.get_classname() == INTRINSICS
        && CHECKS.contains(&inv.get_name().as_str())
        && inv.get_string_descriptor() == "(Ljava/lang/Object;Ljava/lang/String;)V";
    if !is_check {
        return false;
    }
    let value = inv.get_lst_parameters().get(0);
    let Some(stack) = prev.try_as_assignment().filter(|a| a.get_cond_type().is_none()).and_then(|a| {
        let left = a.get_left().try_as_var().filter(|v| v.is_stack())?;
        let same = match value.try_as_var() {
            Some(var) => var.get_index() == left.get_index() && var.get_version() == left.get_version(),
            None => kuna_fix_kt_not_null_assertion::is_read(value) && a.get_right().equals(value),
        };
        same.then_some(left)
    }) else {
        return false;
    };
    next.try_as_assignment().is_some_and(|store| {
        store.get_cond_type().is_none()
            && store.get_left().is_field()
            && store.get_right().try_as_var().is_some_and(|v| {
                v.get_index() == stack.get_index() && v.get_version() == stack.get_version()
            })
    })
}
