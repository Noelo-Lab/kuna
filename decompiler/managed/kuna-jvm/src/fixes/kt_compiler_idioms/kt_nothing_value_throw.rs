//! Fix `kt_nothing_value_throw`: the `throw KotlinNothingValueException()`
//! kotlinc puts after a call of a function the metadata declares to return
//! `Nothing` is left out.

use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::k_element::KElement;
use crate::fixes::kt_calls::kt_synthetic_accessor_call;
use crate::statement::Statement;

const NOTHING_VALUE_EXCEPTION: &str = "kotlin/KotlinNothingValueException";

/// Leaves out, under `stat`, each `throw KotlinNothingValueException()` that
/// follows a call of a function returning `Nothing`; whether there was one.
pub fn remove(stat: Statement) -> bool {
    
    let mut res = false;
    match stat.get_exprents() {
        Some(exprents) => {
            let mut i = 1;
            while i < exprents.len() {
                if is_nothing_value_throw(exprents.get(i)) && returns_nothing(exprents.get(i - 1)) {
                    exprents.remove_at(i);
                    res = true;
                } else {
                    i += 1;
                }
            }
        }
        None => {
            for st in stat.get_stats().borrow().to_vec() {
                res |= remove(st);
            }
        }
    }
    res
}

fn is_nothing_value_throw(e: Exprent) -> bool {
    let Some(exit) = e.try_as_exit() else {
        return false;
    };
    if exit.get_exit_type() != ExitType::Throw {
        return false;
    }
    let Some(new) = exit.get_value().and_then(|v| v.try_as_new()) else {
        return false;
    };
    new.get_new_type().value.as_deref() == Some(NOTHING_VALUE_EXCEPTION)
        && new.get_lst_dims().is_empty()
        && new
            .get_constructor()
            .is_some_and(|ctor| ctor.get_lst_parameters().is_empty() && ctor.get_string_descriptor() == "()V")
}

/// Whether `e` is a call whose callee the metadata declares to return the
/// non-null `Nothing`.
fn returns_nothing(e: Exprent) -> bool {
    let Some(inv) = e.try_as_k_invocation() else {
        return false;
    };
    let accessed =
        kt_synthetic_accessor_call::resugar(inv.0).map(|access| access.calling_data.clone());
    match accessed.or_else(|| inv.kd(|d| d.calling_data.clone())) {
        Some(KElement::Function(function)) => {
            let k = function.return_type.k();
            k.kotlin_type == "kotlin/Nothing" && !k.is_nullable
        }
        _ => false,
    }
}
