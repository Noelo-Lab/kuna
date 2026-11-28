//! Fix `lambda_return_type_argument`: explicit type arguments that contradict
//! the return type of a lambda or method reference argument's implementation
//! are not written.

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::invocation_exprent::InvocationExprent;
use crate::fixes::lambdas::lambda_pertinence::function_type;
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;
use crate::var_type::VarType;

/// The erased type a lambda or method reference `new`'s implementation
/// returns, as its instantiated method type says.
fn implementation_return(new: NewExprent) -> Option<VarType> {
    let name = new.get_new_type().value.clone()?;
    let node = DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()?;
    let info = node.with(|d| d.lambda_information.clone())?;
    Some(
        MethodDescriptor::parse_descriptor(&info.method_descriptor)
            .ret
            .clone(),
    )
}

/// Whether `generic_args`, the explicit type arguments for `fparams` of
/// `call` whose formal parameter types are `param_types`, contradict a lambda
/// or method reference argument whose function type returns one of them.
pub fn contradicts(
    call: InvocationExprent,
    param_types: &[VarType],
    fparams: &[String],
    generic_args: &[VarType],
) -> bool {
    let args = call.get_lst_parameters();
    if args.len() != param_types.len() || generic_args.len() != fparams.len() {
        return false;
    }
    let context = DecompilerContext::get_struct_context();
    args.iter().zip(param_types).any(|(arg, formal)| {
        let Some(new) = arg.try_as_new().filter(|n| n.is_lambda()) else {
            return false;
        };
        let Some(ret) = implementation_return(new) else {
            return false;
        };
        let Some((_, returned)) = function_type(formal) else {
            return false;
        };
        if returned.type_ != CodeType::Genvar || returned.array_dim != 0 {
            return false;
        }
        let Some(index) = fparams
            .iter()
            .position(|f| returned.value.as_deref() == Some(f.as_str()))
        else {
            return false;
        };
        let written = &generic_args[index];
        match (
            ret.type_,
            written.type_,
            ret.value.as_deref(),
            written.value.as_deref(),
        ) {
            (CodeType::Object, CodeType::Object, Some(r), Some(w)) => {
                ret.array_dim == 0
                    && written.array_dim == 0
                    && r != w
                    && w != "java/lang/Object"
                    && !context.instance_of(r, w)
            }
            _ => false,
        }
    })
}
