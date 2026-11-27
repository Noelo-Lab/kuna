//! Fix `raw_target_cast`: a lambda or method reference passed for a
//! parameter whose type is a raw functional interface as kuna writes the
//! call (a call on a receiver of raw type or a raw `new`, whose member types
//! are erased (JLS 4.8), or a parameter declared raw) has the raw type's
//! function type, whose parameters are the erasures of the interface's: an
//! implicitly typed lambda types them `Object`, an explicitly typed one must
//! declare them so, and `s -> s.length()` does not compile. When the
//! `invokedynamic` instantiates a parameter more specifically, the lambda is
//! cast to the parameterization it instantiates, which the raw parameter
//! accepts by widening: `list.removeIf((Pred<String>) (s -> s.isEmpty()))`.
//! The cast is written only where the call's class has no other method of
//! its name and arity, so that the argument it makes pertinent to
//! applicability (JLS 15.12.2.2) cannot select another method, and the
//! call's result type does not depend on it (a raw member's is erased, and a
//! raw parameter takes part in no inference). A lambda written against such
//! a type declares its own parameters with their erased types, which
//! [`declared_parameter`] gives `raw_receiver_erasure` while its body is
//! written.

use std::cell::RefCell;

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::fixes::generic_casts::raw_receiver_erasure;
use crate::fixes::kt_util::writing_kotlin;
use crate::invocation_exprent::{InvocationExprent, Type as InvocationKind};
use crate::kuna_static_types::{formals, is_raw_type};
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

type Declared = (String, String, Vec<(i32, VarType)>);

thread_local! {
    static DECLARED: RefCell<Vec<Declared>> = const { RefCell::new(Vec::new()) };
}

/// The cast for the argument at `index` of `inv` the module header
/// describes; marks a lambda written for a raw parameter whose parameters
/// keep their erased types.
pub fn argument_cast(inv: InvocationExprent, index: usize) -> Option<VarType> {
    if writing_kotlin() || inv.0.is_kotlin() {
        return None;
    }
    let arg = inv.get_lst_parameters().to_vec().get(index).copied()?;
    let lambda = arg.try_as_new().filter(|n| n.is_lambda())?;
    let (iface, instantiated, sam) = super::functional_method(lambda)?;
    let param = inv.get_descriptor().params.get(index).cloned()?;
    if param.array_dim != 0 || param.value != iface.value || !raw_parameter(inv, index) {
        return None;
    }
    let instantiated = MethodDescriptor::parse_descriptor(&instantiated);
    let raw = MethodDescriptor::parse_descriptor(&sam.get_descriptor());
    if instantiated.params.len() != raw.params.len() {
        return None;
    }
    if instantiated.params.iter().zip(&raw.params).all(|(a, b)| a.equals(b)) {
        lambda.set_kuna_erased_params();
        return None;
    }
    if !sole_method(inv) {
        return None;
    }
    let cast = instantiated_cast(lambda)?;
    lambda.set_kuna_erased_params();
    Some(cast)
}

/// Writes `arg` under the cast `cast`.
pub fn write(buf: &mut TextBuffer, arg: Exprent, cast: &VarType, indent: i32) {
    buf.append("(")
        .append_cast_type_name(cast)
        .append(")(")
        .append_buffer(&arg.to_java(indent))
        .append(")");
}

/// The parameterization `lambda`'s `invokedynamic` instantiates
/// (`lambdas::instantiated_parameterization`) when it can be written as a
/// cast: each type argument within its type parameter's bounds, as far as
/// bounds that are classes decide, and each a class the current class can
/// name.
pub fn instantiated_cast(lambda: NewExprent) -> Option<VarType> {
    let t = super::instantiated_parameterization(lambda)?;
    let cls = DecompilerContext::get_struct_context().get_class(t.value_str())?;
    let bounds = cls.get_signature()?.fbounds().clone();
    let args = t.try_as_generic()?.get_arguments().to_vec();
    if bounds.len() != args.len() {
        return None;
    }
    for (arg, bounds) in args.iter().zip(&bounds) {
        let arg = arg.as_ref()?;
        if arg.type_ != CodeType::Object && arg.array_dim == 0 {
            return None;
        }
        for b in bounds {
            if b.type_ != CodeType::Object || b.array_dim != 0 || b.try_as_generic().is_some_and(|g| !g.get_arguments().is_empty()) {
                return None;
            }
            let fits = if arg.array_dim > 0 {
                b.value_str() == "java/lang/Object"
            } else {
                DecompilerContext::get_struct_context().instance_of(arg.value_str(), b.value_str())
            };
            if !fits {
                return None;
            }
        }
        if arg.type_ == CodeType::Object && !nameable(arg.value_str()) {
            return None;
        }
    }
    Some(t)
}

/// Whether the parameter at `index` of `inv` has, as kuna writes the call, a
/// raw type: the call is on a receiver of raw type, or is a `new` of a
/// generic class written without type arguments, whose member types are
/// erased; or the method declares the parameter with a raw type.
fn raw_parameter(inv: InvocationExprent, index: usize) -> bool {
    match inv.get_functype() {
        InvocationKind::General => {
            if raw_receiver_erasure::has_raw_receiver(inv) {
                return true;
            }
        }
        InvocationKind::Init => {
            if inv.get_instance().is_none()
                && inv.printed_generic_args().is_empty()
                && formals(&inv.get_classname()).is_some_and(|f| !f.is_empty())
            {
                return true;
            }
        }
        _ => return false,
    }
    let Some(m) = inv.find_desc() else {
        return false;
    };
    let descriptor = inv.get_descriptor();
    match m.get_signature() {
        None => true,
        Some(sig) => {
            let declared = sig.parameter_types();
            declared.len() == descriptor.params.len() && is_raw_type(&declared[index])
        }
    }
}

/// Whether the class `inv` names, and every class it extends or implements,
/// is known and declares no method of `inv`'s name and arity with other
/// parameter types than `inv`'s descriptor. `Object`, when the class context
/// lacks it, declares only the methods JLS 4.3.2 names.
pub(crate) fn sole_method(inv: InvocationExprent) -> bool {
    const OBJECT_METHODS: [&str; 9] =
        ["getClass", "hashCode", "equals", "clone", "toString", "notify", "notifyAll", "wait", "finalize"];
    let ctx = DecompilerContext::get_struct_context();
    let name = inv.get_name();
    let own = inv.get_descriptor().params.clone();
    let mut queue = vec![inv.get_classname()];
    let mut seen: Vec<String> = Vec::new();
    while let Some(c) = queue.pop() {
        if seen.contains(&c) {
            continue;
        }
        let Some(cls) = ctx.get_class(&c) else {
            if c == "java/lang/Object" && !OBJECT_METHODS.contains(&name.as_str()) {
                continue;
            }
            return false;
        };
        seen.push(c);
        for mt in cls.get_methods().to_vec() {
            if !mt.name_eq(&name) || mt.has_modifier(CodeConstants::ACC_BRIDGE) {
                continue;
            }
            let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
            if md.params.len() == own.len() && !md.params.iter().zip(&own).all(|(a, b)| a.equals(b)) {
                return false;
            }
        }
        if name == "<init>" {
            break;
        }
        queue.extend(cls.super_class().and_then(|s| s.get_string_opt()).map(|s| s.to_string()));
        queue.extend(cls.get_interface_names());
    }
    true
}

/// Whether the class `class` can be named from the class being written: not
/// anonymous, and it and each class enclosing it accessible by the access
/// its `InnerClasses` entry, or the class file of a top-level class, gives
/// it. A top-level class outside the class context is taken as accessible in
/// the current package and in `java/` and `javax/`.
fn nameable(class: &str) -> bool {
    let ctx = DecompilerContext::get_struct_context();
    let Some(current) = DecompilerContext::get_current_class_node().map(|n| n.with(|d| d.class_struct).qualified_name()) else {
        return false;
    };
    let same_package = package(class) == package(&current);
    match inner_entry(class) {
        Some((simple, outer, flags)) => {
            let Some(outer) = outer.filter(|_| simple.is_some()) else {
                return simple.is_some();
            };
            let public = flags & CodeConstants::ACC_PUBLIC != 0;
            let private = flags & CodeConstants::ACC_PRIVATE != 0;
            let accessible = public || if private { outermost(class) == outermost(&current) } else { same_package };
            accessible && nameable(&outer)
        }
        None => match ctx.get_class(class) {
            Some(cls) => same_package || cls.has_modifier(CodeConstants::ACC_PUBLIC),
            None => same_package || class.starts_with("java/") || class.starts_with("javax/"),
        },
    }
}

/// The simple name, outer class and access flags of `class`'s
/// `InnerClasses` entry, from its own class file or the current class's
/// (each lists every nested class its constant pool names, JVMS 4.7.6).
fn inner_entry(class: &str) -> Option<(Option<String>, Option<String>, i32)> {
    let ctx = DecompilerContext::get_struct_context();
    let current = DecompilerContext::get_current_class_node().map(|n| n.with(|d| d.class_struct));
    for holder in [ctx.get_class(class), current].into_iter().flatten() {
        if let Some(attribute) = holder.get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES) {
            if let Some(e) = attribute.get_entries().into_iter().find(|e| e.inner_name == class) {
                return Some((e.simple_name, e.enclosing_name, e.access_flags));
            }
        }
    }
    None
}

fn package(class: &str) -> &str {
    class.rfind('/').map_or("", |i| &class[..i])
}

fn outermost(class: &str) -> &str {
    class.split('$').next().unwrap_or(class)
}

/// Declares, until the returned guard drops, the own parameters (those
/// after the captured values, `own` of them) of the lambda whose
/// implementation is `content`, while its body is written, with their
/// erased types: its implementation's.
pub fn declare_parameters(content: StructMethod, own: usize) -> DeclaredParameters {
    let md = MethodDescriptor::parse_descriptor(&content.get_descriptor());
    let mut slot = if content.has_modifier(CodeConstants::ACC_STATIC) { 0 } else { 1 };
    let mut params = Vec::new();
    let first_own = md.params.len().saturating_sub(own);
    for (i, p) in md.params.iter().enumerate() {
        if i >= first_own {
            params.push((slot, p.clone()));
        }
        slot += p.stack_size;
    }
    let key = format!("{}{}", content.get_name(), content.get_descriptor());
    DECLARED.with(|d| d.borrow_mut().push((content.get_class_qualified_name(), key, params)));
    DeclaredParameters
}

/// Ends a [`declare_parameters`] when dropped.
pub struct DeclaredParameters;

impl Drop for DeclaredParameters {
    fn drop(&mut self) {
        DECLARED.with(|d| d.borrow_mut().pop());
    }
}

/// The type the parameter in `slot` of `method`, a lambda's implementation
/// being written, is declared with, when [`declare_parameters`] gave
/// it.
pub fn declared_parameter(method: StructMethod, slot: i32) -> Option<VarType> {
    let class = method.get_class_qualified_name();
    let key = format!("{}{}", method.get_name(), method.get_descriptor());
    DECLARED.with(|d| {
        d.borrow()
            .iter()
            .rev()
            .find(|(c, k, _)| *c == class && *k == key)
            .and_then(|(_, _, params)| params.iter().find(|(s, _)| *s == slot).map(|(_, t)| t.clone()))
    })
}

/// Whether `t` is the type a lambda written for it gives its own
/// parameters with their erased types: `lambda`'s raw interface, when its
/// instantiated parameter types are those of the raw function type, or the
/// parameterization its `invokedynamic` instantiates.
pub fn erases_parameters(t: &VarType, lambda: NewExprent) -> bool {
    let Some((iface, instantiated, sam)) = super::functional_method(lambda) else {
        return false;
    };
    if t.array_dim != 0 || t.value != iface.value {
        return false;
    }
    if is_raw_type(t) {
        let instantiated = MethodDescriptor::parse_descriptor(&instantiated);
        let raw = MethodDescriptor::parse_descriptor(&sam.get_descriptor());
        return instantiated.params.len() == raw.params.len()
            && instantiated.params.iter().zip(&raw.params).all(|(a, b)| a.equals(b));
    }
    t.is_generic()
        && super::instantiated_parameterization(lambda)
            .is_some_and(|p| p.try_as_generic().is_some_and(|g| g.equals_exact(t)))
}
