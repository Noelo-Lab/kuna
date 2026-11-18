//! kuna fix `kt_throwable_cast`: a thrown value is written without the cast
//! to `java.lang.Throwable` that kotlinc puts before every `athrow`.
//!
//! kotlinc compiles `throw e` to `checkcast java/lang/Throwable; athrow`,
//! as its `kotlin.Throwable` is the JVM's `java.lang.Throwable`. The cast is
//! a no-op: Kotlin only throws an expression typed `Throwable`. Vineflower
//! keeps it and writes `throw IllegalStateException(..) as
//! java.lang.Throwable`, which Kotlin rejects (`java.lang.Throwable` is not
//! `kotlin.Throwable` when named). With the fix a thrown cast to
//! `java/lang/Throwable` of an operand of a class type is written as the
//! operand alone; an operand typed `Object` (a value of an erased type
//! variable) keeps the cast.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;

/// The operand of `value` when `value` is a no-op cast to `java/lang/Throwable`.
pub fn uncast(value: Exprent) -> Option<Exprent> {
    let cast = value.try_as_function()?;
    if cast.get_func_type() != FunctionType::Cast {
        return None;
    }
    let operands = cast.get_lst_operands();
    let target = operands.get(1).try_as_const()?.get_const_type();
    if target.type_ != CodeType::Object
        || target.array_dim != 0
        || target.value.as_deref() != Some("java/lang/Throwable")
    {
        return None;
    }
    let operand = operands.get(0);
    let operand_type = operand.get_expr_type();
    (operand_type.type_ == CodeType::Object
        && operand_type.array_dim == 0
        && operand_type
            .value
            .as_deref()
            .is_some_and(|v| v != "java/lang/Object"))
    .then_some(operand)
}
