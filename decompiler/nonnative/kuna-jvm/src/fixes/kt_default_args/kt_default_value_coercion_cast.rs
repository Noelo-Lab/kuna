//! kuna fix `kt_default_value_coercion_cast` (`kuna_fixes`), called from
//! `DefaultArgsMap::from`: a default value is written without the cast to
//! its parameter's type that kotlinc compiles into it.
//!
//! kotlinc coerces a default value to its parameter's JVM type with a
//! `checkcast` (`aconst_null; checkcast MediaType` for `contentType:
//! MediaType? = null`, `checkcast java/util/List` after `ArrayList()`,
//! `checkcast Function0` on a lambda's instance), the same instruction for
//! the declaration with or without a cast. Vineflower prints it as a cast
//! (`= null as MediaType?`, `= ArrayList() as java.util.List`, `= {} as
//! Function0`), which is no part of the source and which Kotlin rejects
//! where the type is written as a raw Java type. With the fix a default
//! that is a cast of a reference to exactly its parameter's JVM type is
//! written as the cast's operand: the parameter's type is the default's
//! expected type, so Kotlin compiles the same `checkcast`, and an operand
//! that is not of that type is rejected, not converted. A cast of an
//! operand that writes a class body (a lambda, an object) is kept: where
//! that body could not be decompiled, its failure comment in an otherwise
//! empty lambda would without the cast be a lambda Kotlin accepts.

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::k_element::KElement;
use crate::kt_attrs::KtAttrs;
use crate::method_wrapper::MethodWrapper;
use crate::var_type::VarType;

/// `default` (a copy, whose operands are the Kotlin plugin's) without its
/// cast to the type of the parameter the branch statements `exprents`
/// (ending with the parameter's assignment) assign, a parameter of the
/// static default-arguments method `bridge`.
pub fn uncast(default: Exprent, exprents: &[Exprent], bridge: MethodWrapper) -> Exprent {
    let operand = || {
        let cast = default.try_as_function()?;
        if cast.get_func_type() != FunctionType::Cast {
            return None;
        }
        let operands = cast.get_lst_operands();
        if writes_class_body(operands.get(0)) {
            return None;
        }
        let target = operands.get(1).try_as_const()?.get_const_type();
        if target.type_ != CodeType::Object && target.array_dim == 0 {
            return None;
        }
        let param = exprents
            .last()?
            .try_as_assignment()?
            .get_left()
            .try_as_var()?;
        let processor = param.get_processor()?;
        let index = param.get_index();
        let slot = processor.get_var_original_index(index).unwrap_or(index);
        param_type(bridge, slot)?
            .equals(&target)
            .then(|| operands.get(0))
    };
    operand().unwrap_or(default)
}

/// The type of the parameter at `slot` of the static method `method`.
fn param_type(method: MethodWrapper, slot: i32) -> Option<VarType> {
    let mut at = 0;
    for param in method.desc().params.iter() {
        if at == slot {
            return Some(param.clone());
        }
        at += param.stack_size;
    }
    None
}

/// Whether `expr` creates an instance of an anonymous class or a lambda,
/// or reads a singleton lambda.
fn writes_class_body(expr: Exprent) -> bool {
    expr.try_as_new()
        .is_some_and(|new| new.is_anonymous() || new.is_lambda())
        || expr.try_as_field().is_some_and(|field| {
            field.is_static()
                && DecompilerContext::get_struct_context()
                    .get_class(&field.get_classname())
                    .and_then(|cl| KtAttrs::get(cl, &KElement::KEY()))
                    .is_some_and(|e| matches!(e, KElement::Function(_)))
        })
        || expr.get_all_exprents().into_iter().any(writes_class_body)
}
