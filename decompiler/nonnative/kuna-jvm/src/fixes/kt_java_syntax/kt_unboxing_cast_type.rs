//! kuna fix `kt_unboxing_cast_type`: in Kotlin output, the cast whose value
//! an unboxing call unboxes is written to the Kotlin type of the unboxed
//! value, `x as Boolean`, not to the box class, `x as java.lang.Boolean`.
//!
//! Vineflower does not write Java's unboxing calls (`booleanValue()`): Java
//! unboxes `(Boolean)x` where a `boolean` is needed, so it writes the
//! instance alone, here the cast `x as java.lang.Boolean`. Kotlin unboxes a
//! value of its own `Boolean`, `Int`, `Long` ... type but not one of the Java
//! box class, which kotlinc rejects where a `Boolean` is needed ("condition
//! type mismatch: inferred type is 'java.lang.Boolean' but 'Boolean' was
//! expected"). The Kotlin plugin writes `Integer` as `Int` and so unboxes
//! `x as Int`, but writes the other box classes with their Java names. With
//! the fix a cast to the box class whose unboxing call is not written is
//! written to the Kotlin type of the call's result; kotlinc compiles
//! `x as Boolean` there to the `checkcast java/lang/Boolean` and
//! `booleanValue()` of the bytecode.

use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::invocation_exprent::InvocationExprent;
use crate::k_types::KTypes;
use crate::text_buffer::TextBuffer;

/// `instance`, the instance of the unboxing call `call` that is not written,
/// written `x as Boolean`, if it is a cast to the call's box class.
pub fn write(call: InvocationExprent, instance: Exprent, indent: i32) -> Option<TextBuffer> {
    let cast = instance.try_as_k_function()?;
    if cast.get_func_type() != FunctionType::Cast || !cast.does_cast() {
        return None;
    }
    let operands = cast.get_lst_operands();
    let type_ = operands.get(1).try_as_const()?.get_const_type();
    if type_.array_dim != 0 || type_.value_str() != call.get_classname() {
        return None;
    }
    let mut buf = TextBuffer::new();
    buf.add_bytecode_mapping(cast.get_bytecode().as_ref());
    buf.append_buffer(&cast.0.wrap_operand_string(operands.get(0), true, indent))
        .append(" as ")
        .append(&KTypes::get_kotlin_type(Some(&call.get_descriptor().ret)));
    Some(buf)
}
