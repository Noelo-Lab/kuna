//! Fix family `anonymous_classes`: finding, nesting and constructing anonymous
//! and local classes. In this file, `anonymous_static_qualifier` writes a static
//! call of an anonymous class's method unqualified when the owner encloses the
//! class being written and no class in between has a method of that name;
//! `constructor_signature_alignment` trims a constructor signature that omits
//! only the leading synthetic parameters (kotlinc's pre-IR convention) to the
//! real parameters; `local_class_anonymous_subclass` counts `new` of an
//! anonymous class extending a local class as a use of the local class;
//! `initializer_local_class` finds the initializers that declare a local
//! class whose `EnclosingMethod` attribute names no method.

pub(crate) mod anonymous_class_created_elsewhere;
pub(crate) mod unlisted_anonymous_class;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::generic_method_descriptor::GenericMethodDescriptor;
use crate::struct_class::StructClass;
use crate::var_type::VarType;

/// Whether the static call of `owner.name` whose qualifier would be
/// `short_name` is written without a qualifier.
pub fn drop_qualifier(owner: &str, name: &str, short_name: &str) -> bool {
    if short_name != ExprProcessor::UNREPRESENTABLE_TYPE_STRING {
        return false;
    }
    enclosed_without_shadow(DecompilerContext::get_current_class_node(), owner, name)
}

/// Walks from `node` out through its enclosing classes: true when it reaches
/// `owner` before any class (other than a lambda's, which is its enclosing
/// class) that has a method `name` of its own or from a supertype.
fn enclosed_without_shadow(mut node: Option<ClassNode>, owner: &str, name: &str) -> bool {
    while let Some(n) = node {
        let (type_, cl, parent) = n.with(|d| (d.type_, d.class_struct, d.parent));
        if cl.qualified_name() == owner {
            return true;
        }
        if type_ != ClassNodeType::Lambda && has_method_named(cl, name) {
            return false;
        }
        node = parent;
    }
    false
}

/// Whether `cl` or a supertype in the struct context declares a method `name`
/// (every such method counts, so the test errs towards keeping the qualifier).
fn has_method_named(cl: StructClass, name: &str) -> bool {
    let context = DecompilerContext::get_struct_context();
    let mut queue = vec![cl];
    let mut visited: Vec<String> = Vec::new();
    while let Some(cls) = queue.pop() {
        let qualified = cls.qualified_name();
        if visited.contains(&qualified) {
            continue;
        }
        visited.push(qualified);
        if cls
            .get_methods()
            .to_vec()
            .iter()
            .any(|m| m.get_name() == name)
        {
            return true;
        }
        let supers = cls
            .super_class()
            .map(|s| s.get_string().to_string())
            .into_iter()
            .chain(cls.get_interface_names());
        queue.extend(supers.filter_map(|s| context.get_class(&s)));
    }
    false
}

/// The signature restricted to the real parameters, when `sig` describes the
/// descriptor parameters `params` after a leading run of synthetic ones
/// (`real[i]` is false for a synthetic parameter).
pub fn align(
    sig: &GenericMethodDescriptor,
    params: &[VarType],
    real: &[bool],
) -> Option<GenericMethodDescriptor> {
    let sig_params = sig.parameter_types().clone();
    if real.len() != params.len() || sig_params.len() >= params.len() {
        return None;
    }
    let skipped = params.len() - sig_params.len();
    if real[..skipped].iter().any(|&r| r) {
        return None;
    }
    let described = &params[skipped..];
    if !described
        .iter()
        .zip(sig_params.iter())
        .all(|(d, s)| erases_to(s, d))
    {
        return None;
    }
    let kept = sig_params
        .iter()
        .zip(&real[skipped..])
        .filter(|(_, &r)| r)
        .map(|(s, _)| s.clone())
        .collect();
    let d = sig.borrow();
    Some(GenericMethodDescriptor::new(
        d.type_parameters.clone(),
        d.type_parameter_bounds.clone(),
        kept,
        d.return_type.clone(),
        d.exception_types.clone(),
    ))
}

/// Whether the signature type `sig` can be the generic form of the
/// descriptor type `desc`.
fn erases_to(sig: &VarType, desc: &VarType) -> bool {
    if sig.array_dim != desc.array_dim {
        return false;
    }
    match sig.type_ {
        CodeType::Genvar => desc.type_ == CodeType::Object,
        CodeType::Object => desc.type_ == CodeType::Object && sig.value == desc.value,
        t => desc.type_ == t,
    }
}

/// Whether the class `new_type` names extends or implements `classname`.
pub fn extends(new_type: &VarType, classname: &str) -> bool {
    let Some(value) = new_type.value.as_deref() else {
        return false;
    };
    let Some(class) = DecompilerContext::get_struct_context().get_class(value) else {
        return false;
    };
    class.super_class().is_some_and(|s| &*s.get_string() == classname)
        || class.get_interface_names().iter().any(|i| i == classname)
}

/// Whether `node` is `ancestor` or nested in it.
fn is_within(node: ClassNode, ancestor: ClassNode) -> bool {
    let mut current = Some(node);
    while let Some(n) = current {
        if n == ancestor {
            return true;
        }
        current = n.with(|d| d.parent);
    }
    false
}

/// Whether `inner`, listed without an outer class by the `InnerClasses`
/// attribute of `holder`, is a local class declared in an initializer of
/// `holder`: its `EnclosingMethod` attribute names `holder` and no method
/// (JVMS 4.7.7). Vineflower takes such a class for a member, finds its name
/// is not `Outer$Name`, and writes it as a top-level class `Outer$1Name`
/// of its own.
pub fn initializer_local(holder: StructClass, inner: StructClass) -> bool {
    if crate::fixes::stack_var_inlining::is_kotlin_class(holder) || holder.has_modifier(crate::code_constants::ACC_INTERFACE) {
        return false;
    }
    inner
        .get_attribute(crate::struct_general_attribute::StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD)
        .is_some_and(|a| a.get_method_name().is_none() && a.get_class_name().as_deref() == Some(holder.qualified_name().as_str()))
}

/// Whether `e` names the class `class`: creates it, has its type, or reads
/// or calls a member of it.
pub fn mentions_class(e: crate::exprent::Exprent, class: &str) -> bool {
    let named = |t: VarType| t.type_ == CodeType::Object && t.value.as_deref() == Some(class);
    e.try_as_new().is_some_and(|n| !n.is_lambda() && named(n.get_new_type()))
        || e.try_as_var().is_some_and(|v| named(v.get_var_type()))
        || e.try_as_invocation().is_some_and(|i| i.get_classname() == class)
        || e.try_as_field().is_some_and(|f| f.get_classname() == class)
}

/// The methods of `wrapper` that declare the local class `class`, whose
/// `EnclosingMethod` attribute names no method ([`initializer_local`]): the
/// class is declared where it is mentioned, in `<clinit>`, or else in each
/// constructor, which holds a copy of the instance initializer, or else in
/// the other methods that mention it (the body of a lambda in an
/// initializer); a class nothing mentions is declared in `<clinit>` or the
/// first constructor.
pub fn initializer_methods(wrapper: crate::class_wrapper::ClassWrapper, class: &str) -> Vec<crate::method_wrapper::MethodWrapper> {
    use crate::code_constants::{CLINIT_NAME, INIT_NAME};
    let methods: Vec<crate::method_wrapper::MethodWrapper> =
        wrapper.get_methods().borrow().to_vec().into_iter().filter(|m| m.with(|d| d.root.is_some())).collect();
    let mentions = |method: &crate::method_wrapper::MethodWrapper| {
        let root = method.with(|d| d.root).expect("root");
        crate::fixes::util::all_exprents(root.0).iter().any(|e| mentions_class(*e, class))
    };
    let name = |m: &crate::method_wrapper::MethodWrapper| m.with(|d| d.method_struct).get_name();
    let mentioning: Vec<_> = methods.iter().copied().filter(mentions).collect();
    for wanted in [CLINIT_NAME, INIT_NAME] {
        let found: Vec<_> = mentioning.iter().copied().filter(|m| name(m) == wanted).collect();
        if !found.is_empty() {
            return found;
        }
    }
    if !mentioning.is_empty() {
        return mentioning;
    }
    [CLINIT_NAME, INIT_NAME].iter().find_map(|wanted| methods.iter().copied().find(|m| name(m) == *wanted)).into_iter().collect()
}
