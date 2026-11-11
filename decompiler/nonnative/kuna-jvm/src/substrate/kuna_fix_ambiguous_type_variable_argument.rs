//! kuna fix `ambiguous_type_variable_argument` (`kuna_fixes`):
//! `InvocationExprent.appendParamList` casts an argument that
//! `getAmbiguousParameters` finds ambiguous to the type it writes for the
//! parameter, which for a generic method is the signature's type with the
//! type variables replaced by what inference found. When the parameter is
//! declared as one of the method's own type variables, that replacement is
//! the argument's own type, so the cast meant to pick the bytecode's method
//! can make an overload taking that type applicable and more specific, and
//! javac prefers it: Guava's `appendTo((Appendable) builder, parts)` of
//! `<A extends Appendable> A appendTo(A, Iterator<?>)` came out
//! `appendTo((StringBuilder) builder, parts)`, a call of
//! `appendTo(StringBuilder, Iterator<?>)`, the method itself. With the fix,
//! when the arguments as written would resolve (JLS 15.12.2, phases 1 and 2,
//! as fix `vararg_spread_resolution` models them) to another method, such an
//! argument is cast to the variable's erasure, the descriptor's parameter
//! type: no other overload declares that type at that position (the two
//! would erase alike), so the cast rules out the overloads it was meant to.
//! `getInferredExprType` binds the variable to that erasure as javac will, so
//! a cast of the call's result the bytecode makes is written. Where no other
//! method is chosen the inferred type stays: it keeps the result's type.

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::BitSet;
use crate::substrate::kuna_fix_unseen_class_overload;
use crate::substrate::kuna_fix_vararg_spread_resolution::{
    candidates, more_specific, phase, same_parameters, written_type,
};
use crate::var_type::VarType;

/// Whether `declared`, a parameter type in the signature of a generic method
/// whose type parameters are `method_type_parameters`, is one of them or an
/// array of one, for an argument a cast to its erasure can stand for
/// (not a lambda or method reference, which need a functional interface).
pub fn declared_by_method(
    declared: &VarType,
    method_type_parameters: &[String],
    argument: Exprent,
) -> bool {
    declared.type_ == CodeType::Genvar
        && declared
            .value
            .as_deref()
            .is_some_and(|name| method_type_parameters.iter().any(|p| p == name))
        && !argument.try_as_new().is_some_and(|n| n.is_lambda())
}

/// The arguments of `call` that `ambiguous` (`getAmbiguousParameters`)
/// marks, whose parameters `declared` (the signature's, one per descriptor
/// parameter) declares as the method's own type variables, when the call as
/// written would resolve to another method: those get the erasure.
pub fn erased_arguments(
    call: InvocationExprent,
    declared: &[VarType],
    method_type_parameters: &[String],
    ambiguous: &BitSet,
) -> Vec<usize> {
    let params = call.get_lst_parameters();
    let own = call.get_descriptor().params.clone();
    if params.len() != own.len() || declared.len() != own.len() {
        return Vec::new();
    }
    let erased: Vec<usize> = (0..own.len())
        .filter(|&x| {
            ambiguous.get(x)
                && declared_by_method(&declared[x], method_type_parameters, params.get(x))
        })
        .collect();
    if erased.is_empty() {
        return erased;
    }
    let args: Vec<VarType> = (0..own.len())
        .map(|j| {
            if ambiguous.get(j) && !erased.contains(&j) {
                own[j].clone()
            } else {
                written_type(params.get(j), &own[j])
            }
        })
        .collect();
    if selects_other(call, &own, &args) {
        erased
    } else {
        Vec::new()
    }
}

/// Whether `call`, with arguments of types `args`, resolves to a method other
/// than its own (parameters `own`): one applicable in an earlier phase, or in
/// the same phase and strictly more specific.
fn selects_other(call: InvocationExprent, own: &[VarType], args: &[VarType]) -> bool {
    let Some(own_phase) = phase(args, own) else {
        return false;
    };
    methods(call).iter().any(|md| {
        !same_parameters(&md.params, own)
            && phase(args, &md.params).is_some_and(|p| {
                p < own_phase
                    || (p == own_phase
                        && more_specific(&md.params, own)
                        && !more_specific(own, &md.params))
            })
    })
}

/// The methods javac may choose for `call` by its name: those of the class it
/// names and its known supertypes, less private methods of a supertype (not
/// members of the named class); for a class outside the input, the
/// descriptors the input calls it by (fix `unseen_class_overload`).
fn methods(call: InvocationExprent) -> Vec<MethodDescriptor> {
    let classname = call.get_classname();
    let name = call.get_name();
    match DecompilerContext::get_struct_context().get_class(&classname) {
        Some(cl) => candidates(cl, &name)
            .into_iter()
            .filter(|mt| {
                !mt.has_modifier(CodeConstants::ACC_PRIVATE)
                    || mt.get_class_qualified_name() == classname
            })
            .map(|mt| mt.method_descriptor())
            .collect(),
        None => kuna_fix_unseen_class_overload::referenced(&classname, &name)
            .iter()
            .map(|d| MethodDescriptor::parse_descriptor(d))
            .collect(),
    }
}
