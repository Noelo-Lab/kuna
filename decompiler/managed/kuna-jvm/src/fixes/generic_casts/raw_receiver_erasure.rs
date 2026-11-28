//! Fix `raw_receiver_erasure`: a call or field read on a receiver of raw static
//! type has its erased type (a field read passed for a parameter of a
//! type-variable type is cast to it), and a for-each over a raw iterable
//! casts the iterable to its class parameterized by the loop variable's type.

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::field_exprent::FieldExprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::fixes::generic_casts;
use crate::fixes::lambdas::raw_target_cast;
use crate::generic_type::GenericType;
use crate::invocation_exprent::{InvocationExprent, Type as InvocationType};
use crate::kuna_static_types::{field_declared, formals, is_bare_var, is_raw_type, param_type, param_type_at, supertype_args};
use crate::rt::JHashMap;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;
use crate::var_type::{get_class, instance_of, VarType};

/// Whether `param`, a parameter type of a method (not a constructor, whose
/// class's variables a diamond infers) of `class`, names a type
/// variable of `class` that `bound` says the call does not bind and that is
/// not in scope at the call (neither the current class nor an enclosing one
/// is `class`): the receiver's type leaves it unknown, so the argument is
/// written against the parameter's erasure, not a variable of another
/// declaration.
pub fn unbound_class_variable(class: &str, param: &VarType, bound: &dyn Fn(&VarType) -> bool) -> bool {
    let mut scope = DecompilerContext::get_current_class_node();
    while let Some(node) = scope {
        let (name, parent) = node.with(|d| (d.class_struct.qualified_name(), d.parent));
        if name == class {
            return false;
        }
        scope = parent;
    }
    formals(class).is_some_and(|fs| {
        fs.iter()
            .any(|f| !bound(f) && generic_casts::mentions(param, f.value_str()))
    })
}

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

/// Whether an argument of `inv` has a raw type, as the decompiled program
/// declares it, where the parameter's type, with the receiver's declared
/// type arguments for its class's type variables, has type arguments other
/// than unbounded wildcards: the method is then applicable only by
/// unchecked conversion, and the call's result type is the erasure of the
/// declared one (JLS 15.12.2.6), as javac types it. A cast argument is
/// written with its parameter's type, so it is not taken as raw.
pub fn has_unchecked_argument(inv: InvocationExprent) -> bool {
    if inv.0.is_kotlin() || inv.get_functype() == InvocationType::Init {
        return false;
    }
    let Some(desc) = desc_of(inv) else {
        return false;
    };
    let Some(sig) = desc.get_signature() else {
        return false;
    };
    let params = sig.parameter_types();
    let args = inv.get_lst_parameters();
    if params.len() != args.len() {
        return false;
    }
    let mut subst: Option<JHashMap<VarType, Option<VarType>>> = None;
    for (p, a) in params.iter().zip(args.iter()) {
        let var = p.type_ == CodeType::Genvar && p.array_dim == 0;
        let cast = a.try_as_function().is_some_and(|f| f.get_func_type() == FunctionType::Cast);
        if cast || !(var || parameterized(p)) || !is_raw(a) {
            continue;
        }
        if !var {
            return true;
        }
        let map = subst.get_or_insert_with(|| receiver_substitution(inv, desc).unwrap_or_default());
        if map.contains_key(p) && p.remap(map).is_some_and(|t| parameterized(&t)) {
            return true;
        }
    }
    false
}

/// Whether `t` is a class type with a type argument other than an unbounded
/// wildcard.
fn parameterized(t: &VarType) -> bool {
    t.type_ == CodeType::Object
        && t.array_dim == 0
        && t.try_as_generic().is_some_and(|g| {
            g.get_arguments().iter().any(|a| {
                a.as_ref().is_some_and(|a| a.try_as_generic().is_none_or(|ag| ag.get_wildcard() != GenericType::WILDCARD_UNBOUND))
            })
        })
}

/// The type variables of the class declaring `desc` mapped to the type
/// arguments the declared type of `inv`'s receiver gives that class.
fn receiver_substitution(inv: InvocationExprent, desc: StructMethod) -> Option<JHashMap<VarType, Option<VarType>>> {
    if desc.has_modifier(CodeConstants::ACC_STATIC) {
        return None;
    }
    let receiver = declared_type(inv.get_instance()?)?;
    let owner = desc.get_class_qualified_name();
    let class_formals = formals(&owner)?;
    let args = supertype_args(&receiver, &owner)?;
    if args.len() != class_formals.len() {
        return None;
    }
    let mut map = JHashMap::new();
    for (f, a) in class_formals.into_iter().zip(args) {
        map.put(f, Some(a?));
    }
    Some(map)
}

/// Whether `field` is an instance field read through a receiver whose static
/// type is a raw type, so that its type is the erasure of the declared one
/// (JLS 4.8), not the declared type with its class's type variables: passed
/// for a method's parameter whose type is a type variable (or an array of
/// one), which the erasure does not convert to as it converts to a
/// parameterized type, it is written with the cast to the parameter's type.
pub fn has_raw_field_receiver(field: FieldExprent) -> bool {
    !field.0.is_kotlin() && field.get_instance().is_some_and(is_raw)
}

/// Whether the argument `arg` is such a field read ([`has_raw_field_receiver`]),
/// under the casts of its erasure javac added.
pub fn raw_field_argument(arg: Exprent) -> bool {
    let mut arg = arg;
    while let Some(cast) = arg
        .try_as_function()
        .filter(|f| f.get_func_type() == FunctionType::Cast)
    {
        arg = cast.get_lst_operands().get(0);
    }
    arg.try_as_field().is_some_and(has_raw_field_receiver)
}

/// Whether the static type of `e`, as the decompiled program declares it, is
/// a raw type.
pub fn is_raw(e: Exprent) -> bool {
    declared_type(e).is_some_and(|t| is_raw_type(&t))
}

/// The static type the decompiled program gives `e`, when it is declared and
/// kuna knows the declaration: a variable other than `this` and a lambda
/// parameter, a field of a known class other than a synthetic outer
/// instance, a written cast, an element of an array of a declared type, and
/// a call of a known method.
fn declared_type(e: Exprent) -> Option<VarType> {
    if let Some(var) = e.try_as_var() {
        return declared_var_type(var);
    }
    if let Some(field) = e.try_as_field() {
        if field.get_name().starts_with("this$") || !field_declared(field) {
            return None;
        }
        if has_raw_field_receiver(field) {
            return Some(field.get_expr_type());
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
    if let Some(element) = e.try_as_array() {
        return declared_type(element.get_array()).filter(|t| t.array_dim > 0).map(|t| t.decrease_array_dim());
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
/// variable's own type in a method compiled without one; a lambda's
/// parameter, own or captured, where `raw_target_cast` declares it; a
/// variable of an enclosing method that a nested class captures, as
/// [`captured_type`] gives it.
pub(crate) fn declared_var_type(var: VarExprent) -> Option<VarType> {
    let processor = var.get_processor()?;
    let pair = var.get_var_version_pair();
    if processor.get_this_vars().borrow().contains_key(&pair) {
        return None;
    }
    if processor.get_external_vars().borrow().contains(&pair) {
        return captured_type(processor, pair);
    }
    let name = var.get_name();
    if Exprent::with_inferred_lambda_types(|m| m.contains_key(&name)) {
        return None;
    }
    let method = processor.get_method();
    if let Some(param) = param_type(var) {
        if method.get_name().starts_with("lambda$") {
            let slot = processor.get_var_original_index(var.get_index()).unwrap_or(var.get_index());
            return raw_target_cast::declared_parameter(method, slot);
        }
        return param;
    }
    if var.get_lvt().is_some() {
        return Some(var.get_inferred_expr_type(None));
    }
    if method.get_local_variable_attr().is_none() {
        return Some(var.kuna_typed().unwrap_or_else(|| var.get_var_type()));
    }
    None
}

/// The type the variable of an enclosing method that the variable `pair` of
/// a nested class's method `processor` captures is declared with, as
/// [`declared_var_type`] gives it, when that method has no local variable
/// table or the variable is a parameter.
fn captured_type(processor: VarProcessor, pair: VarVersionPair) -> Option<VarType> {
    let (source, original) = processor.get_captured_source(pair)?;
    if source.get_this_vars().borrow().contains_key(&original) {
        return None;
    }
    if source.get_external_vars().borrow().contains(&original) {
        return captured_type(source, original);
    }
    let method = source.get_method();
    if let Some(param) = param_type_at(source, original.var) {
        if method.get_name().starts_with("lambda$") {
            let slot = source.get_var_original_index(original.var).unwrap_or(original.var);
            return raw_target_cast::declared_parameter(method, slot);
        }
        return param;
    }
    if method.get_local_variable_attr().is_some() {
        return None;
    }
    source
        .kuna_typed(original.var)
        .filter(|t| t.java || crate::fixes::kt_types::kt_raw_generic_type::visible(&t.type_))
        .map(|t| t.type_)
        .or_else(|| source.get_var_type(original))
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

/// The method `inv` calls; past a bridge (javac's public forwarder, in a
/// public class, to a method of a superclass that is not public: same name
/// and descriptor, no signature) to the superclass's method it forwards to.
fn desc_of(inv: InvocationExprent) -> Option<StructMethod> {
    let context = DecompilerContext::get_struct_context();
    let (name, descriptor) = (inv.get_name(), inv.get_string_descriptor());
    let desc = inv.find_desc()?;
    if !desc.has_modifier(CodeConstants::ACC_BRIDGE) {
        return Some(desc);
    }
    let mut cls = context.get_class(&inv.get_classname());
    while let Some(c) = cls {
        if let Some(m) = c.get_method(&name, &descriptor) {
            if !m.has_modifier(CodeConstants::ACC_BRIDGE) {
                return Some(m);
            }
        }
        cls = c.super_class().and_then(|s| s.get_string_opt()).and_then(|s| context.get_class(&s));
    }
    Some(desc)
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
