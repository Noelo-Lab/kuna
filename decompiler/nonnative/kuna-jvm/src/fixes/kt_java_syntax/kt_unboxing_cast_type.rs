//! Fix `kt_unboxing_cast_type`: the cast whose value an unwritten unboxing call
//! unboxes is written to the Kotlin type of the call's result (`x as Boolean`).

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
