//! kuna fix `ternary_numeric_arms` (`kuna_fixes`): a reference conditional
//! whose arms print as numeric operands of different types keeps one arm cast
//! to the conditional's type.
//!
//! In bytecode `c ? Integer.valueOf(1) : Long.valueOf(2L)` passes each boxed
//! value through. `InvocationExprent.toJava` prints a boxing call unboxed, so
//! `FunctionExprent.toJava` writes `c ? 1 : 2L`, and javac types that as a
//! numeric conditional (JLS 15.25): both operands are promoted to `long`, the
//! `Integer` becomes a `Long`, and a boxed operand is unboxed (a `null` throws).
//! The same holds for `int` against `Integer` or `Integer` against `Long`.
//! With the fix an arm is written `(T)arm`, `T` the conditional's reference
//! type: the first arm when `T` is no boxed type, so it is no numeric operand;
//! every arm not already of type `T` when `T` is a boxed type, so both arms
//! have the same type. Either way the conditional stays a reference
//! conditional as in the bytecode. Also the test the fix
//! `switch_expression_poly_cast` uses for a switch expression's results.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::var_type::VarType;

/// The exprent `expr` is written as: a boxing call printed without its
/// `valueOf` is written as its argument.
pub fn written_operand(expr: Exprent) -> Exprent {
    if let Some(inv) = expr.try_as_invocation() {
        if prints_unboxed(inv) {
            return inv.get_lst_parameters().get(0);
        }
    }
    expr
}

fn prints_unboxed(inv: crate::invocation_exprent::InvocationExprent) -> bool {
    inv.is_boxing_call() && inv.d(|d| d.can_ignore_boxing) && !inv.should_force_boxing()
}

/// The type `expr` is written with: a boxing call printed without its
/// `valueOf` has its parameter's type.
pub fn printed_type(expr: Exprent) -> Option<VarType> {
    if let Some(inv) = expr.try_as_invocation() {
        if prints_unboxed(inv) {
            return Some(inv.get_descriptor().params[0].clone());
        }
    }
    expr.get_expr_type_opt()
}

/// Whether javac would unbox or promote operands of these types, typed
/// together as a standalone conditional or switch (JLS 15.25, 15.28.1): all
/// are convertible to numeric types, or all to `boolean`, and not all alike.
pub fn promotes(types: &[Option<VarType>]) -> bool {
    let Some(types) = types.iter().cloned().collect::<Option<Vec<VarType>>>() else {
        return false;
    };
    let Some(first) = types.first() else {
        return false;
    };
    if types.iter().all(|t| t.equals(first)) {
        return false;
    }
    types.iter().all(is_numeric) || types.iter().all(is_boolean)
}

fn is_numeric(t: &VarType) -> bool {
    if t.array_dim != 0 {
        return false;
    }
    match t.type_ {
        CodeType::Byte
        | CodeType::Char
        | CodeType::Short
        | CodeType::Int
        | CodeType::Long
        | CodeType::Float
        | CodeType::Double
        | CodeType::Bytechar
        | CodeType::Shortchar => true,
        CodeType::Object => matches!(
            t.value.as_deref(),
            Some(
                "java/lang/Byte"
                    | "java/lang/Character"
                    | "java/lang/Short"
                    | "java/lang/Integer"
                    | "java/lang/Long"
                    | "java/lang/Float"
                    | "java/lang/Double"
            )
        ),
        _ => false,
    }
}

fn is_boolean(t: &VarType) -> bool {
    t.array_dim == 0
        && (t.type_ == CodeType::Boolean
            || (t.type_ == CodeType::Object && t.value.as_deref() == Some("java/lang/Boolean")))
}

/// The type to cast each arm of a conditional to (`None`: written as it is),
/// when javac would unbox or promote its arms and the conditional's type `T`
/// (`FunctionExprent.getExprType`, the join of the arms) is a reference type:
/// with a boxed `T`, every arm not written with it, so both arms have that
/// type; with any other `T`, the first arm, which is then of no numeric type.
pub fn arm_casts(second: Exprent, third: Exprent) -> [Option<VarType>; 2] {
    let printed = [printed_type(second), printed_type(third)];
    if !promotes(&printed) {
        return [None, None];
    }
    let (Some(a), Some(b)) = (second.get_expr_type_opt(), third.get_expr_type_opt()) else {
        return [None, None];
    };
    let Some(t) = VarType::join(&a, &b) else {
        return [None, None];
    };
    if t.type_ != CodeType::Object {
        return [None, None];
    }
    if is_numeric(&t) || is_boolean(&t) {
        printed.map(|p| (!p.is_some_and(|p| p.equals(&t))).then(|| t.clone()))
    } else {
        [Some(t), None]
    }
}
