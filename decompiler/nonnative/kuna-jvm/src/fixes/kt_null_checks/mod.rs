//! Fix family `kt_null_checks` (Kotlin output): kotlinc's null checks, nullable
//! locals and nullable casts. Here: the expression-value checks of values stored
//! into fields, and Kotlin 1.3's parameter checks, dropped from a function's
//! first block and from before a constructor's delegation call, as the plugin
//! drops Kotlin 1.4's.

pub(crate) mod kt_not_null_assertion;
pub(crate) mod kt_nullable_local;

use std::collections::BTreeMap;

use crate::code_constants::ACC_STATIC;
use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::fixes::kt_util::{CHECK_DESCRIPTOR, EXPRESSION_CHECKS, INTRINSICS};
use crate::k_element::KElement;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::pass_context::PassContext;
use crate::root_statement::RootStatement;
use crate::statement::{Statement, StatementType};
use crate::statements::Statements;
use crate::struct_method::StructMethod;
use crate::type_family::TypeFamily;
use crate::var_version_pair::VarVersionPair;

const LEGACY_CHECK: &str = "checkParameterIsNotNull";
const NULLABLE: &str = "org/jetbrains/annotations/Nullable";

/// Deletes the expression-value checks of values stored into fields under `root`.
pub fn remove_stored_value_checks(root: RootStatement) -> bool {
    let mut res = false;
    remove_under(root.0, &mut res);
    res
}

fn remove_under(stat: Statement, res: &mut bool) {
    let stats = stat.get_stats().borrow().to_vec();
    for st in stats {
        remove_under(st, res);
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
        && EXPRESSION_CHECKS.contains(&inv.get_name().as_str())
        && inv.get_string_descriptor() == CHECK_DESCRIPTOR;
    if !is_check {
        return false;
    }
    let value = inv.get_lst_parameters().get(0);
    let Some(stack) = prev.try_as_assignment().filter(|a| a.get_cond_type().is_none()).and_then(|a| {
        let left = a.get_left().try_as_var().filter(|v| v.is_stack())?;
        let same = match value.try_as_var() {
            Some(var) => var.get_index() == left.get_index() && var.get_version() == left.get_version(),
            None => kt_not_null_assertion::is_read(value) && a.get_right().equals(value),
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

/// Drops the legacy checks that open the body of the function `ctx`
/// decompiles; whether it dropped any.
pub fn remove_legacy_checks(ctx: &PassContext) -> bool {
    let method = ctx.get_method();
    if method.get_name().starts_with('<') {
        return false;
    }
    let Some(first) = ctx.get_root().and_then(|root| first_block(root.0)) else {
        return false;
    };
    let Some(exprents) = first.get_exprents() else {
        return false;
    };
    let non_null = non_null_parameters(method);
    let mut count = 0;
    while count < exprents.len()
        && checked_parameter(exprents.get(count))
            .is_some_and(|slot| non_null.get(&slot) == Some(&true))
    {
        count += 1;
    }
    for _ in 0..count {
        exprents.remove_at(0);
    }
    count > 0
}

/// The block `stat` runs first: as `Statements::find_first_data`, also
/// under a statement something breaks out of.
fn first_block(stat: Statement) -> Option<Statement> {
    if stat.get_exprents().is_some() {
        return Some(stat);
    }
    match stat.type_() {
        StatementType::Sequence
        | StatementType::If
        | StatementType::Root
        | StatementType::Switch
        | StatementType::Synchronized => first_block(stat.get_first()?),
        _ => None,
    }
}

/// The slot of `p` in `Intrinsics.checkParameterIsNotNull(p, "..")`.
fn checked_parameter(exprent: Exprent) -> Option<i32> {
    let inv = exprent.try_as_invocation()?;
    if !inv.is_static()
        || inv.get_classname() != INTRINSICS
        || inv.get_name() != LEGACY_CHECK
        || inv.get_string_descriptor() != CHECK_DESCRIPTOR
    {
        return None;
    }
    let params = inv.get_lst_parameters().to_vec();
    if params.len() != 2
        || !params[1]
            .try_as_const()
            .is_some_and(|c| matches!(c.get_value(), Some(ConstValue::String(_))))
    {
        return None;
    }
    params[0].try_as_var().map(|v| v.get_index())
}

/// Whether each reference parameter of `method` is written with a non-null
/// type, by slot.
fn non_null_parameters(method: StructMethod) -> BTreeMap<i32, bool> {
    let md = method.method_descriptor();
    let mut res = BTreeMap::new();
    let mut slot = if method.has_modifier(ACC_STATIC) { 0 } else { 1 };
    let written: Vec<Option<bool>> = match KtAttrs::get(method, &KElement::KEY()) {
        Some(KElement::Function(f)) => {
            let kotlin: Vec<Option<bool>> = f
                .receiver_type
                .iter()
                .chain(f.parameters.iter().map(|p| &p.type_))
                .map(|t| Some(!t.k().is_nullable))
                .collect();
            if kotlin.len() > md.params.len() {
                return res;
            }
            kotlin
        }
        _ => (0..md.params.len()).map(|i| Some(!annotated_nullable(method, i))).collect(),
    };
    for (i, param) in md.params.iter().enumerate() {
        if let Some(Some(non_null)) = written.get(i) {
            res.insert(slot, *non_null && param.type_family == TypeFamily::Object);
        }
        slot += param.stack_size;
    }
    res
}

/// Whether the parameter `param` of `method` is annotated `@Nullable`.
fn annotated_nullable(method: StructMethod, param: usize) -> bool {
    KotlinWriter::PARAMETER_ANNOTATION_ATTRIBUTES.iter().any(|key| {
        method.get_attribute(*key).is_some_and(|attribute| {
            attribute
                .get_param_annotations()
                .get(param)
                .is_some_and(|annotations| annotations.iter().any(|a| a.get_class_name() == NULLABLE))
        })
    })
}

/// Drops the legacy checks before the delegation call of the constructor
/// `ctx` decompiles; whether it dropped any.
pub fn remove_legacy_constructor_checks(ctx: &PassContext) -> bool {
    let method = ctx.get_method();
    if method.get_name() != "<init>" {
        return false;
    }
    let Some(first) = ctx.get_root().and_then(|root| Statements::find_first_data(root.0)) else {
        return false;
    };
    let Some(exprents) = first.get_exprents() else {
        return false;
    };
    let parameter_slots: i32 = ctx
        .get_method_descriptor()
        .params
        .iter()
        .map(|p| p.stack_size)
        .sum();
    let this_vars = ctx.get_var_proc().get_this_vars();
    let is_this = |e: Option<Exprent>| {
        e.and_then(|e| e.try_as_var())
            .is_some_and(|v| this_vars.borrow().contains_key(&VarVersionPair::from_var(v)))
    };

    let mut checks = Vec::new();
    for (i, exprent) in exprents.to_vec().into_iter().enumerate() {
        if checked_parameter(exprent).is_some_and(|slot| (1..=parameter_slots).contains(&slot)) {
            checks.push(i);
        } else if let Some(inv) = exprent.try_as_invocation() {
            if inv.get_name() != "<init>" || !is_this(inv.get_instance()) {
                return false;
            }
            for &i in checks.iter().rev() {
                exprents.remove_at(i);
            }
            return !checks.is_empty();
        } else if !exprent.try_as_assignment().is_some_and(|a| {
            a.get_right().try_as_var().is_some()
                && a.get_left().try_as_field().is_some_and(|f| is_this(f.get_instance()))
        }) {
            if checks.is_empty() || !delegates_later(ctx.get_root().map(|r| r.0), &is_this) {
                return false;
            }
            for &i in checks.iter().rev() {
                exprents.remove_at(i);
            }
            return true;
        }
    }
    false
}

/// Whether a statement under `stat` makes this constructor's delegation call.
fn delegates_later(stat: Option<Statement>, is_this: &dyn Fn(Option<Exprent>) -> bool) -> bool {
    let Some(stat) = stat else {
        return false;
    };
    let delegation = |e: Exprent| {
        e.get_all_exprents_rec_self(true, true).into_iter().any(|e| {
            e.try_as_invocation()
                .is_some_and(|inv| inv.get_name() == "<init>" && is_this(inv.get_instance()))
        })
    };
    if stat.get_exprents().is_some_and(|list| list.to_vec().into_iter().any(delegation)) {
        return true;
    }
    let mut i = 0;
    while i < stat.get_stats().borrow().len() {
        let st = stat.get_stats().borrow().get(i);
        if delegates_later(Some(st), is_this) {
            return true;
        }
        i += 1;
    }
    false
}
