//! Fix family `kt_suspend` (Kotlin output): suspend functions and lambdas,
//! rebuilt from their state machines (`kt_suspend_state_machine`,
//! `kt_suspend_lambda`), their calls and bodies. Here: a member without metadata
//! that overrides a suspend function written as one, the `$$forInline` copy
//! left out, a suspend function type written `suspend (A) -> R`, and kotlinc's
//! coroutine boxing written as implicit boxing.

pub(crate) mod kt_suspend_call_continuation;
pub(crate) mod kt_suspend_impl;
pub(crate) mod kt_suspend_lambda;
pub(crate) mod kt_suspend_state_machine;

use std::rc::Rc;

use crate::code_constants::{ACC_BRIDGE, ACC_STATIC, ACC_SYNTHETIC};
use crate::decompiler_context::DecompilerContext;
use crate::fixes::kt_util::SUSPEND_DESCRIPTOR_END;
use crate::fixes::util::all_exprents;
use crate::generic_type::GenericType;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::k_type::{KType, KTypeData};
use crate::k_types::KTypes;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;

pub(crate) const CONTINUATION: &str = "kotlin/coroutines/Continuation";
const FOR_INLINE: &str = "$$forInline";
const BOXING: &str = "kotlin/coroutines/jvm/internal/Boxing";

/// The suspend function `mt`, a method without Kotlin metadata, overrides:
/// its declaration in a supertype's metadata. `None` when `mt` is not one.
pub fn overridden(mt: StructMethod) -> Option<Rc<KFunction>> {
    let descriptor = mt.get_descriptor();
    if !descriptor.ends_with(SUSPEND_DESCRIPTOR_END)
        || mt.get_name().starts_with('<')
        || mt.has_modifier(ACC_STATIC)
        || mt.has_modifier(ACC_SYNTHETIC)
        || KtAttrs::get(mt, &KElement::KEY()).is_some()
    {
        return None;
    }
    let cl = DecompilerContext::get_struct_context().get_class(&mt.get_class_qualified_name())?;
    let name = mt.get_name();
    if let Some(function) = in_supertypes(cl, &name, &descriptor, &mut Vec::new()) {
        return Some(function);
    }
    let arity = mt.method_descriptor().params.len();
    let methods = cl.get_methods().to_vec();
    let same_shape = |m: &StructMethod| m.name_eq(&name) && m.method_descriptor().params.len() == arity;
    let specialised = methods.iter().filter(|m| same_shape(m) && !m.has_modifier(ACC_BRIDGE)).count() == 1;
    if !specialised {
        return None;
    }
    methods
        .iter()
        .filter(|b| same_shape(b) && b.has_modifier(ACC_BRIDGE) && b.has_modifier(ACC_SYNTHETIC))
        .filter(|b| b.get_descriptor().ends_with(SUSPEND_DESCRIPTOR_END))
        .find_map(|b| in_supertypes(cl, &name, &b.get_descriptor(), &mut Vec::new()))
}

/// Whether `mt` is the method of a bridge that overrides a suspend function,
/// not the overriding method itself.
pub fn through_bridge(mt: StructMethod) -> bool {
    let Some(cl) = DecompilerContext::get_struct_context().get_class(&mt.get_class_qualified_name()) else {
        return false;
    };
    overridden(mt).is_some()
        && in_supertypes(cl, &mt.get_name(), &mt.get_descriptor(), &mut Vec::new()).is_none()
}

/// The suspend function declared for `name` and `descriptor` by the
/// metadata of a supertype of `cl`, searched as `KotlinWriter.searchForMethod`
/// searches them.
fn in_supertypes(cl: StructClass, name: &str, descriptor: &str, seen: &mut Vec<String>) -> Option<Rc<KFunction>> {
    let context = DecompilerContext::get_struct_context();
    let supers = cl
        .super_class()
        .map(|s| s.get_string().to_string())
        .into_iter()
        .chain(cl.get_interface_names());
    for super_name in supers {
        if seen.contains(&super_name) {
            continue;
        }
        seen.push(super_name.clone());
        let Some(super_class) = context.get_class(&super_name) else {
            continue;
        };
        if let Some(method) = super_class.get_method(name, descriptor).filter(|m| !m.has_modifier(ACC_STATIC)) {
            KotlinChooser::parse_metadata_for(super_class);
            if let Some(KElement::Function(function)) = KtAttrs::get(method, &KElement::KEY()) {
                if Flags::IS_SUSPEND.get(function.flags) {
                    return Some(function);
                }
            }
        }
        if let Some(function) = in_supertypes(super_class, name, descriptor, seen) {
            return Some(function);
        }
    }
    None
}

/// The result type to write for `mt`, the suspend function overriding
/// `function`: `None` for `Unit`.
pub fn result_type(mt: StructMethod, function: &KFunction, indent: i32) -> Option<TextBuffer> {
    let declared = &function.return_type;
    if declared.k().type_parameter_name.is_none() && !mentions_type_parameter(declared) {
        if declared.k().kotlin_type == KTypes::UNIT && !declared.k().is_nullable {
            return None;
        }
        return Some(declared.stringify(indent));
    }
    let mut buf = TextBuffer::new();
    let argument = mt
        .get_signature()
        .and_then(|s| s.parameter_types().last().cloned())
        .and_then(|continuation| continuation.try_as_generic())
        .and_then(|g| g.get_arguments().first().cloned().flatten());
    match argument {
        Some(argument) => {
            let plain = GenericType::with_wildcard(&argument, GenericType::WILDCARD_NO);
            if plain.value.as_deref() == Some(KTypes::UNIT) && plain.array_dim == 0 {
                return None;
            }
            buf.append(&KTypes::get_kotlin_type(Some(&plain)));
            if declared.k().is_nullable {
                buf.append("?");
            }
        }
        None => {
            buf.append("Any?");
        }
    }
    Some(buf)
}

fn mentions_type_parameter(t: &crate::k_type::KType) -> bool {
    t.k().type_parameter_name.is_some()
        || t
            .k()
            .type_arguments
            .as_ref()
            .is_some_and(|args| args.iter().any(|a| mentions_type_parameter(&a.type_)))
}

/// Whether `mt`, a method of `cl`, is the `$$forInline` copy of another of
/// its methods.
pub fn is_for_inline_copy(cl: StructClass, mt: StructMethod) -> bool {
    let name = mt.get_name();
    let Some(original) = name.strip_suffix(FOR_INLINE).filter(|n| !n.is_empty()) else {
        return false;
    };
    cl.get_method(original, &mt.get_descriptor()).is_some()
}

/// Whether the metadata type `type_` is a suspend function type.
pub fn is_suspend_function_type(type_: &kt_proto::Type) -> bool {
    let flags = type_.message().scalar(1).map_or(0, |v| v as i32);
    Flags::SUSPEND_TYPE.get(flags)
}

/// `k`, a function type marked suspend, written as one: `None` when its last
/// parameter is not a continuation.
pub fn write_suspend_function_type(k: &KTypeData, indent: i32) -> Option<TextBuffer> {
    let arguments = k.type_arguments.as_ref()?;
    if arguments.len() < 2 {
        return None;
    }
    let continuation = &arguments[arguments.len() - 2].type_;
    let continuation = continuation.0.kotlin.as_deref()?;
    if continuation.kotlin_type != CONTINUATION {
        return None;
    }
    let result: KType = continuation.type_arguments.as_ref()?.first()?.type_.clone();
    let mut buf = TextBuffer::new();
    buf.append(if k.is_nullable { "(suspend (" } else { "suspend (" });
    buf.push_newline_group(indent, 1);
    buf.append_possible_newline();
    for (i, argument) in arguments[..arguments.len() - 2].iter().enumerate() {
        if i != 0 {
            buf.append(",").append_possible_newline_alt(" ");
        }
        buf.append_buffer(&argument.stringify(indent + 1));
    }
    buf.append_possible_newline_dedent("", true);
    buf.pop_newline_group();
    buf.append(") -> ");
    buf.append_buffer(&result.stringify(indent + 1));
    if k.is_nullable {
        buf.append(")?");
    }
    Some(buf)
}

/// The `Boxing` functions and the wrapper class whose `valueOf` boxes the same
/// primitive.
const BOXES: [(&str, &str, &str); 8] = [
    ("boxBoolean", "Z", "java/lang/Boolean"),
    ("boxByte", "B", "java/lang/Byte"),
    ("boxShort", "S", "java/lang/Short"),
    ("boxInt", "I", "java/lang/Integer"),
    ("boxLong", "J", "java/lang/Long"),
    ("boxFloat", "F", "java/lang/Float"),
    ("boxDouble", "D", "java/lang/Double"),
    ("boxChar", "C", "java/lang/Character"),
];

/// Turns each coroutine boxing call under `root`, the body of `mt`, into its
/// `valueOf`; whether there was one. kotlinc boxes with `Boxing` only in
/// coroutine code: methods in the suspend calling convention and suspend
/// lambdas' `invokeSuspend`.
pub fn rewrite_boxing(root: RootStatement, mt: StructMethod) -> bool {
    let coroutine = mt.get_descriptor().ends_with(SUSPEND_DESCRIPTOR_END)
        || mt.name_eq("invokeSuspend");
    if !coroutine {
        return false;
    }
    let exprents = all_exprents(root.0);
    let mut res = false;
    for e in exprents {
        let Some(inv) = e.try_as_invocation() else {
            continue;
        };
        if !inv.is_static() || inv.get_classname() != BOXING || inv.get_lst_parameters().len() != 1 {
            continue;
        }
        let name = inv.get_name();
        let descriptor = inv.get_string_descriptor();
        let Some((_, primitive, wrapper)) = BOXES
            .iter()
            .find(|(n, p, w)| *n == name && descriptor == format!("({p})L{w};"))
        else {
            continue;
        };
        let value_of = format!("({primitive})L{wrapper};");
        inv.set_classname(wrapper.to_string());
        inv.set_name("valueOf".to_string());
        inv.set_descriptor(MethodDescriptor::parse_descriptor(&value_of));
        inv.set_string_descriptor(value_of);
        if let Some(k) = e.try_as_k_invocation() {
            k.kd_mut(|d| {
                d.calling_data = None;
                d.property_method = None;
            });
        }
        res = true;
    }
    res
}
