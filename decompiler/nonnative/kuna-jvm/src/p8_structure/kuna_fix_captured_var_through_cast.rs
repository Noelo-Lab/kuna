//! kuna fix `captured_var_through_cast`: a `new` of an anonymous or local
//! class that passes a captured variable through a reference cast that cannot
//! fail still captures that variable.
//!
//! `NestedClassProcessor.computeLocalVarsAndDefinitions` takes a constructor
//! argument for a captured variable only when it is a variable. kotlinc copies
//! an inline function's lambda parameter into a local with a `checkcast` to
//! the parameter's type (`val cont = cancellable as CancellableContinuation`)
//! and passes that local to the objects the lambda creates; the Kotlin plugin
//! inlines every single-use variable, so the `new` passes
//! `(CancellableContinuation) cancellable`. Vineflower then leaves the
//! parameter uncaptured: the object keeps a field and a constructor parameter,
//! which the Kotlin writer cannot express as an `object` expression and fails
//! (kotlinx-coroutines' `awaitClose`).
//!
//! With the fix a cast argument whose operand is a variable is the capture of
//! that variable when the variable's type is a class that is a subtype of the
//! cast's class: such a cast never throws and yields the same reference, and
//! every use the object makes of the value as the cast type is valid on the
//! variable's type.

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
