//! kuna fix `raw_receiver_erasure` (`kuna_fixes`): in Java the members of a
//! raw type are erased (JLS 4.8): `map.keySet()` on a raw `Map` is a raw
//! `Set`, and `set.toArray(new String[0])` on it returns `Object[]`.
//! Vineflower infers a call's generic result from its arguments whatever its
//! receiver, finds `String[]`, and drops the `checkcast` that makes the
//! program compile; a for-each over a raw iterable it writes with a typed
//! loop variable that javac sees as `Object`. With the fix a call on a
//! receiver whose static type is raw has its erased result type, so the cast
//! the bytecode holds is written, and a for-each over a raw iterable casts
//! the iterable to its class parameterized by the loop variable's type.
//! Kotlin output is left alone.

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::generic_type::GenericType;
use crate::invocation_exprent::{InvocationExprent, Type as InvocationType};
use crate::kuna_static_types::{field_declared, formals, is_bare_var, is_raw_type, param_type};
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::var_type::{get_class, instance_of, VarType};

/// Whether `inv` is an instance method call whose receiver's static type is
/// a raw type, so that its result type is the erasure of the declared one.
pub fn has_raw_receiver(inv: InvocationExprent) -> bool {
    if inv.0.is_kotlin()
        || inv.get_functype() == InvocationType::Init
        || desc_of(inv).is_none_or(|d| d.has_modifier(CodeConstants::ACC_STATIC))
    {
        return false;
    }
    inv.get_instance().is_some_and(is_raw)
}

/// Whether the static type of `e`, as the decompiled program declares it, is
/// a raw type.
pub fn is_raw(e: Exprent) -> bool {
    declared_type(e).is_some_and(|t| is_raw_type(&t))
}

/// The static type the decompiled program gives `e`, when it is declared and
/// kuna knows the declaration: a variable other than `this` and a lambda
/// parameter, a field of a known class other than a synthetic outer
/// instance, a written cast, and a call of a known method.
fn declared_type(e: Exprent) -> Option<VarType> {
    if let Some(var) = e.try_as_var() {
        return declared_var_type(var);
    }
    if let Some(field) = e.try_as_field() {
        if field.get_name().starts_with("this$") || !field_declared(field) {
            return None;
        }
        return Some(field.get_inferred_expr_type(None));
    }
    if let Some(func) = e.try_as_function() {
        if func.get_func_type() != FunctionType::Cast {
            return None;
        }
        return match is_written_cast(func)? {
            true => Some(func.get_expr_type()),
            false => declared_type(func.get_lst_operands().get(0)),
        };
    }
    if let Some(inv) = e.try_as_invocation() {
        let desc = desc_of(inv)?;
        return match desc.get_signature() {
            Some(sig) if !has_raw_receiver(inv) => sig.return_type(),
            _ => Some(inv.get_expr_type()),
        };
    }
    None
}

/// The type `var` is declared with: a parameter's from its method's
/// signature or descriptor, a local's from the local variable table, or the
/// variable's own type in a method compiled without one. A variable of an
/// enclosing method, which a lambda or a nested class captures, is not
/// known.
fn declared_var_type(var: VarExprent) -> Option<VarType> {
    let processor = var.get_processor()?;
    let pair = var.get_var_version_pair();
    if processor.get_this_vars().borrow().contains_key(&pair)
        || processor.get_external_vars().borrow().contains(&pair)
    {
        return None;
    }
    let name = var.get_name();
    if Exprent::with_inferred_lambda_types(|m| m.contains_key(&name)) {
        return None;
    }
    let method = processor.get_method();
    if let Some(param) = param_type(var) {
        return param.filter(|_| !method.get_name().starts_with("lambda$"));
    }
    if var.get_lvt().is_some() {
        return Some(var.get_inferred_expr_type(None));
    }
    if method.get_local_variable_attr().is_none() {
        return Some(var.get_var_type());
    }
    None
}

/// Whether the cast `func` is written: Vineflower leaves out a cast whose
/// operand already has a type of the cast's class. `None` when the
/// operand's type comes from inference against its context (a call whose
/// result is one of the method's own type variables).
fn is_written_cast(func: FunctionExprent) -> Option<bool> {
    let cast = func.get_expr_type();
    let operand = func.get_lst_operands().get(0);
    if let Some(inv) = operand.try_as_invocation() {
        let sig = desc_of(inv).and_then(|d| d.get_signature());
        if let Some(sig) = sig {
            let own = sig.type_parameters().clone();
            let ret = sig.return_type();
            if ret.is_some_and(|r| {
                r.try_as_generic().is_some_and(|g| {
                    let mut vars = g.get_all_generic_vars();
                    if r.type_ == CodeType::Genvar {
                        vars.push(g.clone());
                    }
                    vars.iter()
                        .any(|v| own.iter().any(|p| p.as_str() == v.value_str()))
                })
            }) {
                return None;
            }
        }
    }
    let t = operand.get_inferred_expr_type(None);
    if t.array_dim != cast.array_dim {
        return Some(true);
    }
    Some(match t.type_ {
        CodeType::Object => !instance_of(&t.value, &cast.value),
        CodeType::Genvar => !func
            .0
            .get_named_generics()
            .get(&t.resize_array_dim(0))
            .is_some_and(|bounds| bounds.iter().any(|b| instance_of(&b.value, &cast.value))),
        _ => true,
    })
}

fn desc_of(inv: InvocationExprent) -> Option<StructMethod> {
    inv.get_desc().or_else(|| {
        DecompilerContext::get_struct_context()
            .get_class(&inv.get_classname())?
            .get_method_recursive(&inv.get_name(), &inv.get_string_descriptor())
    })
}

/// The cast a for-each over the raw `iterable` needs for a loop variable of
/// type `elem`: the iterable's class parameterized by `elem`, when the class
/// has one type parameter and it is the one it gives `Iterable`.
pub fn foreach_cast_type(iterable: Exprent, elem: &VarType) -> Option<VarType> {
    if elem.equals(&VarType::VARTYPE_OBJECT()) || !is_raw(iterable) {
        return None;
    }
    let t = declared_type(iterable)?;
    let own = formals(t.value_str())?;
    if own.len() != 1 || !iterates_own_parameter(&t, &own[0]) {
        return None;
    }
    let elem_arg = if elem.type_ == CodeType::Object || elem.type_ == CodeType::Genvar {
        elem.clone()
    } else {
        VarType::new_value(CodeType::Object, 0, boxed(elem.type_)?)
    };
    Some(
        GenericType::new(
            CodeType::Object,
            0,
            t.value_str(),
            None,
            vec![Some(elem_arg)],
            GenericType::WILDCARD_NO,
        )
        .0,
    )
}

/// Whether the class of `t` is `Iterable<param>`, or gives `Iterable` its
/// type parameter `param` as it is.
fn iterates_own_parameter(t: &VarType, param: &VarType) -> bool {
    const ITERABLE: &str = "java/lang/Iterable";
    if t.value_str() == ITERABLE {
        return true;
    }
    let Some(cls) = get_class(&t.value) else {
        return false;
    };
    let hierarchy = cls.get_all_generics();
    if let Some(map) = hierarchy.get(&ITERABLE.to_string()) {
        let Some(iter_t) = formals(ITERABLE).and_then(|f| f.into_iter().next()) else {
            return false;
        };
        return map
            .get(&iter_t)
            .is_some_and(|e| is_bare_var(e) && e.equals(param));
    }
    let Some(sig) = cls.get_signature() else {
        return false;
    };
    let supers = sig.superinterfaces();
    supers.iter().any(|s| {
        s.value_str() == ITERABLE
            && s.try_as_generic().is_some_and(
                |g| matches!(g.get_arguments(), [Some(a)] if is_bare_var(a) && a.equals(param)),
            )
    })
}

fn boxed(t: CodeType) -> Option<&'static str> {
    Some(match t {
        CodeType::Boolean => "java/lang/Boolean",
        CodeType::Byte => "java/lang/Byte",
        CodeType::Char => "java/lang/Character",
        CodeType::Short => "java/lang/Short",
        CodeType::Int => "java/lang/Integer",
        CodeType::Long => "java/lang/Long",
        CodeType::Float => "java/lang/Float",
        CodeType::Double => "java/lang/Double",
        _ => return None,
    })
}

/// Writes `iterable` cast to `cast`; a written cast of the iterable has its
/// type replaced.
pub fn append_cast(buf: &mut TextBuffer, iterable: Exprent, cast: &VarType, indent: i32) {
    let operand = match iterable.try_as_function() {
        Some(f) if f.get_func_type() == FunctionType::Cast && is_written_cast(f) == Some(true) => {
            f.get_lst_operands().get(0)
        }
        _ => iterable,
    };
    buf.append("(").append_cast_type_name(cast).append(")");
    let quote = operand.get_precedence() >= FunctionType::Cast.precedence();
    if quote {
        buf.append("(");
    }
    buf.append_buffer(&operand.to_java(indent));
    if quote {
        buf.append(")");
    }
}
