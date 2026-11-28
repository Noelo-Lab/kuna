//! Fix `synthetic_receiver_capture`: Vineflower inlines the value assigned
//! to a local without a LocalVariableTable entry into the lambda created
//! next that captures only it, when the lambda's body is one call on the
//! capture: javac's lowering of a method reference `expr::m` to `rec$ =
//! expr; x -> rec$.m(x)`. A lambda captures variables, and its body is
//! written with the name of the variable its captured argument is, so after
//! the inlining of any other value the body named a variable that does not
//! exist (`Object o = c.toString(); add(() -> o.hashCode())` compiled
//! without debug information printed `add(() -> var0.hashCode())`, the call
//! `c.toString()` gone, as Vineflower prints it) or, where a lambda of the
//! same body named the capture, a variable of that lambda's method. The
//! value is inlined only where the lambda can capture it as written:
//! `this`, or a parameter the method never assigns, or a local it assigns
//! once.

use crate::code_constants::ACC_STATIC;
use crate::exprent::Exprent;
use crate::fixes::util::all_exprents;
use crate::function_exprent::FunctionType;
use crate::statement::Statement;

/// Whether a lambda created in `stat`'s method can capture `value` as it
/// is written.
pub fn capturable(value: Exprent, stat: Statement) -> bool {
    let Some(var) = value.try_as_var().filter(|v| !v.is_stack()) else {
        return false;
    };
    let index = var.get_index();
    let root = stat.get_top_parent();
    let mt = root.d(|d| d.mt);
    let mut parameters = if mt.has_modifier(ACC_STATIC) { 0 } else { 1 };
    parameters += mt.method_descriptor().params.iter().map(|p| p.stack_size).sum::<i32>();
    let writes = |e: &Exprent| {
        let target = match e.try_as_assignment() {
            Some(assign) => Some(assign.get_left()),
            None => e
                .try_as_function()
                .filter(|f| {
                    matches!(
                        f.get_func_type(),
                        FunctionType::Imm | FunctionType::Mmi | FunctionType::Ipp | FunctionType::Ppi
                    )
                })
                .map(|f| f.get_lst_operands().get(0)),
        };
        target.and_then(|t| t.try_as_var()).is_some_and(|v| v.get_index() == index)
    };
    let assigned = all_exprents(root.0).iter().filter(|e| writes(e)).count();
    assigned == 0 || (assigned == 1 && index >= parameters)
}
