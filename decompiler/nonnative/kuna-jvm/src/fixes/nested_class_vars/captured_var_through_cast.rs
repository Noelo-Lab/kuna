//! Fix `captured_var_through_cast`: a constructor argument that casts a
//! variable to a supertype of its class captures that variable.

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;

/// The variable `arg` passes when it is a reference cast of a variable whose
/// class is a subtype of the cast's class.
pub fn cast_var(arg: Exprent) -> Option<VarExprent> {
    let cast = arg
        .try_as_function()
        .filter(|f| f.get_func_type() == FunctionType::Cast)?;
    let operands = cast.get_lst_operands();
    if operands.len() != 2 {
        return None;
    }
    let var = operands.get(0).try_as_var()?;
    let target = operands.get(1).get_expr_type();
    let source = var.get_var_type();
    let (Some(source), Some(target)) = (class_name(&source), class_name(&target)) else {
        return None;
    };
    DecompilerContext::get_struct_context()
        .instance_of(source, target)
        .then_some(var)
}

/// The class a non-array reference type names.
fn class_name(t: &VarType) -> Option<&str> {
    (t.type_ == CodeType::Object && t.array_dim == 0)
        .then_some(t.value.as_deref())
        .flatten()
}
