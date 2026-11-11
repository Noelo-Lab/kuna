//! kuna fix `lambda_pertinence` (`kuna_fixes`): javac infers a generic
//! method's type variables from a lambda or method reference argument only
//! as far as the argument can be typed before them (JLS 18.5.2): the types
//! of an implicitly typed lambda's parameters, and of an inexact method
//! reference's (its name overloaded, its method generic or variable-arity,
//! its class generic and written raw, as `Constructor::getParameterCount`),
//! are the function type's parameter types, so the variables in those are
//! resolved first, without the argument; and a lambda or method reference
//! whose target is a bare type variable, as a lambda returned by a lambda
//! whose function type returns one, needs that variable resolved first. A
//! call that is the qualifier of another has no target type either (JLS
//! 15.12), so such a variable becomes its bound there.
//! `InvocationExprent.getInferredExprType` counts every argument's type as
//! fixing the variables it maps, finds nothing missing and writes no type
//! arguments: `names.map(n -> () -> n.length()).iterator()`,
//! `Comparator.comparingInt(Constructor::getParameterCount).reversed()`,
//! which javac rejects. With the fix those variables do not count, so a
//! qualifier call they leave unfixed gets its explicit type arguments
//! (`names.<Task<Integer>>map(...)`), unless one of them cannot be written
//! where the call is. A call of a generic method that infers its result
//! from its target only (`Predicates.always()`) does not fix a bare type
//! variable either, and a varargs array counts as its elements
//! (`Stream.<Predicate<ShardRouting>>of(lambda, Predicates.always())`).

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::new_exprent::NewExprent;
use crate::rt::{JHashMap, JHashSet};
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

const OBJECT_METHODS: &[(&str, &str)] = &[
    ("equals", "(Ljava/lang/Object;)Z"),
    ("hashCode", "()I"),
    ("toString", "()Ljava/lang/String;"),
];

/// The type variables of the formal `param_type` that the argument
/// `parameter` cannot fix for javac; `fparams` are the called method's own
/// type parameters.
pub fn excluded_vars(parameter: Exprent, param_type: &VarType, fparams: &[String]) -> Vec<VarType> {
    if is_type_parameter(param_type, fparams) && free_generic_call(parameter) {
        return vec![param_type.resize_array_dim(0)];
    }
    let Some(new) = parameter.try_as_new().filter(|n| n.is_lambda()) else {
        return Vec::new();
    };
    if is_type_parameter(param_type, fparams) {
        return vec![param_type.resize_array_dim(0)];
    }
    if !param_type.is_generic() || new.is_method_reference() && is_exact_reference(new) {
        return Vec::new();
    }
    let Some((params, ret)) = function_type(param_type) else {
        return Vec::new();
    };
    let mut out: Vec<VarType> = Vec::new();
    for p in &params {
        collect_vars(p, &mut out);
    }
    if !new.is_method_reference() && is_type_parameter(&ret, fparams) && returns_lambda(new) {
        out.push(ret.resize_array_dim(0));
    }
    out
}

/// For the array a varargs call passes for its last formal `param_type`
/// (`T[]`), written as its elements (`of(a, b)`): the type variables no
/// element fixes for javac (each is a lambda, a method reference or a
/// [`free_generic_call`] for them), and the array type to map them from, the
/// first element's parameterized type where the array's is raw.
pub fn vararg_elements(
    parameter: Exprent,
    param_type: &VarType,
    fparams: &[String],
) -> Option<(Vec<VarType>, Option<VarType>)> {
    let new = parameter.try_as_new()?;
    let array = new.get_new_type();
    if array.array_dim == 0 || param_type.array_dim == 0 {
        return None;
    }
    let elements = new.get_lst_array_elements();
    if elements.is_empty() {
        return None;
    }
    let component = param_type.resize_array_dim(param_type.array_dim - 1);
    let mut vars: Option<Vec<VarType>> = None;
    for element in elements.iter() {
        let excluded = excluded_vars(element, &component, fparams);
        vars = Some(match vars {
            None => excluded,
            Some(v) => v
                .into_iter()
                .filter(|x| excluded.iter().any(|e| e.equals(x)))
                .collect(),
        });
    }
    let vars = vars.unwrap_or_default();
    if vars.is_empty() {
        return None;
    }
    let element_type = elements
        .iter()
        .map(|e| e.get_inferred_expr_type(None))
        .find(|t| t.is_generic() && t.array_dim == 0 && t.value == array.value)
        .map(|t| t.resize_array_dim(array.array_dim));
    Some((vars, element_type))
}

/// Whether `parameter` calls a generic method whose result mentions a type
/// parameter of its own that none of its parameter types does
/// (`Predicates.always()`, `List.of()`): javac infers that one from the
/// call's target only, which for an argument of another generic call is that
/// call's inference variable, so the argument fixes the variable to the
/// result with the parameter's bound (`Predicate<Object>`). A method of a
/// class outside the input returning a generic class may be one.
fn free_generic_call(parameter: Exprent) -> bool {
    let Some(inv) = parameter.try_as_invocation() else {
        return false;
    };
    let context = DecompilerContext::get_struct_context();
    if context.get_class(&inv.get_classname()).is_none() {
        let ret = inv.get_descriptor().ret.clone();
        return ret.type_ == CodeType::Object
            && ret.array_dim == 0
            && ret
                .value
                .as_deref()
                .and_then(|c| context.get_class(c))
                .and_then(|c| c.get_signature())
                .is_some_and(|s| !s.fparameters().is_empty());
    }
    let Some(sig) = inv.get_desc().and_then(|desc| desc.get_signature()) else {
        return false;
    };
    let own = sig.type_parameters().clone();
    let Some(ret) = sig.return_type() else {
        return false;
    };
    let mut ret_vars: Vec<VarType> = Vec::new();
    collect_vars(&ret, &mut ret_vars);
    let mut param_vars: Vec<VarType> = Vec::new();
    for p in sig.parameter_types().iter() {
        collect_vars(p, &mut param_vars);
    }
    ret_vars.iter().any(|v| {
        v.value
            .as_deref()
            .is_some_and(|name| own.iter().any(|o| o == name))
            && !param_vars.iter().any(|p| p.equals(v))
    })
}

fn is_type_parameter(t: &VarType, fparams: &[String]) -> bool {
    t.type_ == CodeType::Genvar
        && t.value
            .as_deref()
            .is_some_and(|v| fparams.iter().any(|p| p == v))
}

fn collect_vars(t: &VarType, out: &mut Vec<VarType>) {
    if t.type_ == CodeType::Genvar {
        out.push(t.resize_array_dim(0));
    } else if t.is_generic() {
        out.extend(
            t.as_generic()
                .get_all_generic_vars()
                .into_iter()
                .map(|g| g.0),
        );
    }
}

/// The parameter and return types of the function type of the functional
/// interface `target`, in terms of `target`'s type arguments.
pub fn function_type(target: &VarType) -> Option<(Vec<VarType>, VarType)> {
    let base: StructClass =
        DecompilerContext::get_struct_context().get_class(target.value.as_deref()?)?;
    let sam: StructMethod = base.get_methods().to_vec().into_iter().find(|m| {
        !m.has_modifier(CodeConstants::ACC_STATIC)
            && m.get_instruction_sequence().is_none()
            && !OBJECT_METHODS.contains(&(m.get_name().as_str(), m.get_descriptor().as_str()))
    })?;
    let (base_sig, sam_sig) = (base.get_signature()?, sam.get_signature()?);
    let mut map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
    base_sig
        .generic_type()
        .map_gen_vars_to(&target.as_generic(), &mut map);
    let params = sam_sig
        .parameter_types()
        .iter()
        .map(|p| p.remap(&map).unwrap_or_else(|| p.clone()))
        .collect();
    let ret = sam_sig.return_type()?;
    let ret = ret.remap(&map).unwrap_or(ret);
    Some((params, ret))
}

/// Whether a value the lambda `new` returns is itself a lambda or method
/// reference.
fn returns_lambda(new: NewExprent) -> bool {
    let Some(node) = node_of(new) else {
        return false;
    };
    let (Some(info), Some(wrapper)) = (
        node.with(|d| d.lambda_information.clone()),
        node.get_wrapper(),
    ) else {
        return false;
    };
    let Some(mt) =
        wrapper.get_method_wrapper(&info.content_method_name, &info.content_method_descriptor)
    else {
        return false;
    };
    let Some(root) = mt.with(|d| d.root) else {
        return false;
    };
    root.get_dummy_exit()
        .get_all_predecessor_edges()
        .into_iter()
        .any(|edge| {
            edge.get_source().get_exprents().is_some_and(|lst| {
                !lst.is_empty()
                    && lst
                        .get(lst.len() - 1)
                        .try_as_exit()
                        .filter(|ex| ex.get_exit_type() == ExitType::Return)
                        .and_then(|ex| ex.get_value())
                        .and_then(|v| v.try_as_new())
                        .is_some_and(|v| v.is_lambda())
            })
        })
}

fn node_of(new: NewExprent) -> Option<crate::classes_processor::ClassNode> {
    let name = new.get_new_type().value.clone()?;
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()
}

/// A method reference javac types without a target (JLS 15.13.1): its
/// method is the only one of that name in its class, is neither generic
/// nor of variable arity, and a `Type::name` form does not name a generic
/// class raw. A class outside the input is taken for exact.
fn is_exact_reference(new: NewExprent) -> bool {
    let Some(info) = node_of(new).and_then(|n| n.with(|d| d.lambda_information.clone())) else {
        return true;
    };
    let Some(content) = DecompilerContext::get_struct_context().get_class(&info.content_class_name)
    else {
        return true;
    };
    let bound = !info.is_content_method_static
        && new
            .get_constructor()
            .and_then(|c| c.get_instance())
            .is_some();
    if !bound
        && content
            .get_signature()
            .is_some_and(|s| !s.fparameters().is_empty())
    {
        return false;
    }
    let same_name: Vec<StructMethod> = content
        .get_methods()
        .to_vec()
        .into_iter()
        .filter(|m| m.get_name() == info.content_method_name)
        .collect();
    let [method] = same_name[..] else {
        return false;
    };
    !method.has_modifier(CodeConstants::ACC_VARARGS)
        && method
            .get_signature()
            .is_none_or(|s| s.type_parameters().is_empty())
}

/// Whether the explicit type arguments can be written where the call is
/// and mean what the bytecode does: no type variable out of scope, and no
/// raw type, which would erase the members javac types the call with.
pub fn writable(args: &[VarType], named: &JHashSet<VarType>) -> bool {
    args.iter().all(|a| {
        if a.type_ == CodeType::Genvar {
            return named.contains(&a.resize_array_dim(0));
        }
        !(a.is_generic() && a.as_generic().has_unknown_generic_type(named))
            && !crate::kuna_static_types::is_raw_type(&a.resize_array_dim(0))
    })
}

/// The qualifier explicit type arguments need on a call written without
/// one: its class for a static method, else `this`.
pub fn append_qualifier(buf: &mut TextBuffer, is_static: bool, classname: &str) {
    if is_static {
        let short_name = DecompilerContext::get_import_collector()
            .get_short_name_in_class_context(&ExprProcessor::build_java_class_name(classname));
        buf.append_all_classes(&short_name, Some(classname));
    } else {
        buf.append("this");
    }
    buf.append(".");
}
