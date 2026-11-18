//! kuna fix `kt_reference_inequality`: in Kotlin output, Java's `!=` of two
//! references is written `!==`.
//!
//! In bytecode `==` and `!=` of two references (`if_acmpeq`, `if_acmpne`)
//! test identity; Kotlin's `==` and `!=` call `equals` (kotlinc compiles them
//! to `Intrinsics.areEqual`), its `===` and `!==` test identity. When the
//! Kotlin plugin turns Java's expressions into its own, `KFunctionExprent`
//! makes a reference `==` whose operands are not `null` its `===`, but leaves
//! `!=` as it is, so `a !== b` comes out `a != b`, an `equals` call, and a
//! condition the decompiler negates after that turns `a === b` back into
//! `!=` too. With the fix such a comparison is marked as an identity test
//! when it is made, the mark survives negation, and it is written `!==` (or
//! `===`) when both operands are references; a copy keeps the mark of what
//! it copies. `==` resugared from `Intrinsics.areEqual`, and its negation,
//! keep `==` and `!=`.

use crate::function_exprent::FunctionType;
use crate::k_function_exprent::{AnyFunctionType, KFunctionExprent, KFunctionType};
use crate::type_family::TypeFamily;
use crate::var_type::VarType;

/// Marks `comparison`, made from a Java comparison, as an identity test when
/// it compares two operands neither of which is the `null` literal.
pub fn mark(comparison: KFunctionExprent) {
    if !matches!(
        comparison.get_func_type(),
        FunctionType::Eq | FunctionType::Ne
    ) && comparison.get_any_function_type() != AnyFunctionType::Kotlin(KFunctionType::Equals3)
    {
        return;
    }
    let operands = comparison.get_lst_operands();
    let not_null = |i: usize| {
        operands
            .get(i)
            .get_expr_type_opt()
            .is_some_and(|t| !t.ptr_eq(&VarType::VARTYPE_NULL()))
    };
    if not_null(0) && not_null(1) {
        comparison.kd_mut(|d| d.identity = true);
    }
}

/// Whether `comparison` (`==` or `!=`) is an identity test of two references.
pub fn identity(comparison: KFunctionExprent) -> bool {
    let operands = comparison.get_lst_operands();
    let reference = |i: usize| {
        operands
            .get(i)
            .get_expr_type_opt()
            .is_some_and(|t| t.type_family == TypeFamily::Object)
    };
    comparison.kd(|d| d.identity) && reference(0) && reference(1)
}
