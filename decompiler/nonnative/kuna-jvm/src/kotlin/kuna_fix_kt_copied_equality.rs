//! kuna fix `kt_copied_equality`: in Kotlin output, a copy of an `==` that
//! tests equality stays `==`.
//!
//! `ResugarKotlinMethodsPass` turns `Intrinsics.areEqual(a, b)`, Kotlin's
//! `a == b` (an `equals` call), into a `KFunctionExprent` `==`, which the
//! Kotlin plugin writes `==`. Every other `KFunctionExprent` `==` is made
//! from Java's `==`, and its constructor makes that `===` (identity) unless
//! an operand is `null`. `copy` builds the copy through the same
//! constructor, so a copy of a resugared `==` became `===`: an identity test
//! where the bytecode calls `equals`. With the fix a copy keeps the function
//! type of what it copies.

use crate::function_exprent::FunctionType;
use crate::k_function_exprent::{KFunctionExprent, KFunctionType};

/// Gives `copy` the function type of `original` where the constructor
/// changed it: an `==` it made `===`.
pub fn keep_function_type(original: KFunctionExprent, copy: KFunctionExprent) {
    if original.get_func_type() == FunctionType::Eq
        && original.kd(|d| d.k_type) == KFunctionType::None
        && copy.get_func_type() != FunctionType::Eq
    {
        copy.set_func_type(FunctionType::Eq);
    }
}
