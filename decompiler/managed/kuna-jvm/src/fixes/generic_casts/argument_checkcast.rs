//! Fix `argument_checkcast`: javac casts an argument to the erasure of its
//! parameter's type in the call's instantiated method type, and inserts no
//! other cast there. On a receiver of raw type that type is the erasure of
//! the declared one (JLS 4.8), so a `checkcast` of an argument whose erased
//! type is not of the cast's class, passed where that erasure is not of the
//! class either (`ArrayList list; list.add((Spatial) page.getKey(i))`,
//! where `add` takes `Object`), is written even when the argument's
//! inferred type already is of the class: without it the recompiled call
//! has no `checkcast`.

use std::cell::Cell;

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::var_type::{instance_of, VarType};

thread_local! {
    static ERASED: Cell<bool> = const { Cell::new(false) };
}

/// Marks the next argument `ExprProcessor.getCastedArgument` writes as
/// passed for a parameter of a call on a raw receiver, until the returned
/// guard drops.
#[must_use]
pub fn mark_erased_parameter() -> ErasedParameter {
    ERASED.with(|c| c.set(true));
    ErasedParameter
}

/// Clears the mark [`mark_erased_parameter`] set when dropped.
pub struct ErasedParameter;

impl Drop for ErasedParameter {
    fn drop(&mut self) {
        ERASED.with(|c| c.set(false));
    }
}

/// Whether the argument being written was marked; clears the mark, so that
/// the values nested in it are not.
pub fn take_erased_parameter() -> bool {
    ERASED.with(|c| c.replace(false))
}

/// Writes the cast `arg` is, if it is one, for the erased parameter type
/// `param` of a call on a raw receiver, where the module header says javac
/// would not insert it; `arg`'s type was inferred against `param`.
pub fn keep(arg: Exprent, param: &VarType) {
    let Some(cast) = arg.try_as_function().filter(|f| f.get_func_type() == FunctionType::Cast && !f.does_cast()) else {
        return;
    };
    let ops = cast.get_lst_operands();
    if ops.len() != 2 {
        return;
    }
    let target = ops.get(1).get_expr_type();
    let erased = ops.get(0).get_expr_type();
    if target.type_ != CodeType::Object || target.array_dim != 0 || erased.type_ != CodeType::Object || erased.array_dim != 0 {
        return;
    }
    if instance_of(&erased.value, &target.value) {
        return;
    }
    let inserted = match (param.type_, param.array_dim) {
        (CodeType::Object, 0) => instance_of(&param.value, &target.value),
        (CodeType::Genvar, 0) => false,
        _ => return,
    };
    if !inserted {
        cast.set_needs_cast(true);
    }
}
