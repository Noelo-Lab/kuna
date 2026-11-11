//! The kuna fix `compound_assignment_shape` (`kuna_fixes`), called from
//! `SecondaryFunctionsHelper::identify_secondary_functions_exprent`.
//!
//! Vineflower writes `left = left op e` as `left op= e` by a heuristic: a
//! field target only when `e` has no side effects, an array element only with
//! a variable or constant array and index, never through a narrowing cast that
//! is not `e`'s own type, and `left = e op left` as `left op= e` for a
//! commutative `op`, which evaluates `left` before `e`. The bytecode tells the
//! forms apart where it differs: javac evaluates the receiver of a compound
//! assignment to a field once and `dup`s it, and an element's array and index
//! once with `dup2`, while `a.f = a.f op e` loads them twice; and the compound
//! form always reads `left` first. With the fix an instance field or array
//! element target is written compound exactly when its receiver (array and
//! index) and the read's are one load, with any `e` and through the cast to
//! the target's own type that the compound form implies; and `left = e op
//! left` is never written compound. Locals and static fields, which compile
//! the same either way, keep Vineflower's choice, and so do classes compiled
//! from Kotlin (with `kotlin.Metadata`), whose compiler does not `dup` a
//! property's receiver. The same holds for `x.f = x.f + 1`, which
//! `PPandMMHelper`, `isPPIorMMI` and `isIPPorIMM2` write `x.f++`
//! ([`loads_twice`]).

use crate::assignment_exprent::AssignmentExprent;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::FunctionType;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::kuna_fixes::{self, KunaFix};

/// The fix's decision for `asexpr` (the read of `left` may be widened, as in
/// `i += someLong`): `Some(true)` when it was made compound,
/// `Some(false)` when it stays as it is, `None` to leave it to Vineflower.
pub fn compound(asexpr: AssignmentExprent) -> Option<bool> {
    use FunctionType::*;

    if DecompilerContext::get_current_class()
        .is_some_and(|cl| KotlinMetadataAnnotation::find(cl).is_some())
    {
        return None;
    }
    let left = asexpr.get_left();
    let mut func = asexpr.get_right().try_as_function()?;
    let mut midlayer = None;
    if func.get_func_type().cast_type().is_some() {
        midlayer = func.get_simple_cast_type();
        func = func.get_lst_operands().get(0).try_as_function()?;
    }
    let operands = func.get_lst_operands();
    let func_type = func.get_func_type();
    if operands.len() != 2
        || !matches!(
            func_type,
            Add | Sub | Mul | Div | Rem | And | Or | Xor | Shl | Shr | Ushr
        )
    {
        return None;
    }
    let mut read = operands.get(0);
    if midlayer.is_some() {
        if let Some(widened) = read
            .try_as_function()
            .filter(|f| matches!(f.get_func_type(), I2l | I2f | I2d | L2f | L2d | F2d))
            .map(|f| f.get_lst_operands().get(0))
            .filter(|w| left.equals(*w))
        {
            read = widened;
        }
    }
    if !left.equals(read) {
        if matches!(func_type, Add | And | Or | Xor) && left.equals(operands.get(1)) {
            return Some(false);
        }
        return None;
    }
    let one_load = match left.get_type() {
        ExprentType::Field => {
            let (target, source) = (left.as_field(), read.as_field());
            if target.is_static() {
                return None;
            }
            match (target.get_instance(), source.get_instance()) {
                (Some(a), Some(b)) => same_load(a, b),
                _ => false,
            }
        }
        ExprentType::Array => {
            let (target, source) = (left.as_array(), read.as_array());
            same_load(target.get_array(), source.get_array())
                && same_load(target.get_index(), source.get_index())
        }
        _ => return None,
    };
    if !one_load || midlayer.is_some_and(|m| !m.equals(&left.get_expr_type())) {
        return Some(false);
    }
    asexpr.set_right(operands.get(1));
    asexpr.set_cond_type(Some(func_type));
    Some(true)
}

/// Whether `left = read ± 1`, an instance field or array element written with
/// its own read, loads the receiver (array and index) twice, as
/// `x.f = x.f + 1` does and `x.f++` does not: it then stays an assignment
/// instead of becoming `++x.f` (`PPandMMHelper`, `isPPIorMMI`, and
/// `isIPPorIMM2` for a read kept in a stack variable).
pub fn loads_twice(left: Exprent, read: Exprent) -> bool {
    if !kuna_fixes::enabled(KunaFix::CompoundAssignmentShape)
        || DecompilerContext::get_current_class()
            .is_some_and(|cl| KotlinMetadataAnnotation::find(cl).is_some())
        || left.get_type() != read.get_type()
    {
        return false;
    }
    match left.get_type() {
        ExprentType::Field => {
            let (target, source) = (left.as_field(), read.as_field());
            match (target.get_instance(), source.get_instance()) {
                (Some(a), Some(b)) if !target.is_static() => two_loads(a, b),
                _ => false,
            }
        }
        ExprentType::Array => {
            let (target, source) = (left.as_array(), read.as_array());
            two_loads(target.get_array(), source.get_array())
                || two_loads(target.get_index(), source.get_index())
        }
        _ => false,
    }
}

/// Whether `a` and `b` are two loads of one value: not the same stack
/// variable, with instruction offsets of their own that differ.
fn two_loads(a: Exprent, b: Exprent) -> bool {
    if same_load(a, b) {
        return false;
    }
    if [a, b].iter().any(|e| e.try_as_var().is_some_and(|v| v.is_stack())) {
        return true;
    }
    matches!((a.get_bytecode(), b.get_bytecode()), (Some(x), Some(y)) if !x.is_empty() && !y.is_empty())
}

/// Whether `a` and `b` are one value on the operand stack: the same stack
/// variable, or copies of one expression (same instruction offsets).
fn same_load(a: Exprent, b: Exprent) -> bool {
    if let (Some(va), Some(vb)) = (a.try_as_var(), b.try_as_var()) {
        if va.is_stack() || vb.is_stack() {
            return va.is_stack()
                && vb.is_stack()
                && va.get_var_version_pair() == vb.get_var_version_pair();
        }
    }
    match (a.get_bytecode(), b.get_bytecode()) {
        (Some(x), Some(y)) => !x.is_empty() && x == y,
        _ => false,
    }
}
