//! kuna fix `lambda_return_type_argument` (`kuna_fixes`), called from
//! `InvocationExprent::get_inferred_expr_type`.
//!
//! `getInferredExprType` writes explicit type arguments on a generic call
//! (a qualifier of another) when it maps a type variable from no argument,
//! and takes the value it has for it, which may come from where the chain's
//! result goes rather than from the call: for `ext.map(Ext::names).orElse(
//! Collections.emptyList())`, whose method reference's class is outside the
//! input, `U` gets `orElse`'s argument's `List<String>`, and `ext.<List<String>>
//! map(Ext::names)` does not compile, `Ext.names` returning a `Collection`.
//! javac infers a variable the function type of a lambda or method reference
//! argument returns from what its implementation returns. With the fix,
//! type arguments contradicting that (the implementation's erased return
//! type is no subclass of the argument's class) are not written, and javac
//! infers them as the source did (Elasticsearch's `TransportVersion`).

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::invocation_exprent::InvocationExprent;
use crate::kuna_fix_lambda_pertinence::function_type;
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
