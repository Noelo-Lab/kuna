//! kuna fix `kt_suspend_without_metadata` (spec 10.16): in Kotlin output, a
//! suspend function that has no Kotlin metadata of its own, but overrides
//! one that the metadata of its supertype declares `suspend`, is written as
//! a suspend function.
//!
//! kotlinc writes no function metadata for the members of the classes it
//! generates or regenerates without it: the anonymous objects of inline
//! functions copied into `$$inlined$` classes (kotlinx-coroutines'
//! `FlowCollector`s inside `filter`, `map`, `collect { .. }`), and the
//! classes of SAM-converted suspend lambdas. Vineflower writes such a
//! member from its JVM signature, `override fun emit(value: T, $completion:
//! Continuation<in Unit>): Any`, and its body as kotlinc's state machine
//! (fix `kt_suspend_state_machine` rebuilds only functions the metadata
//! declares `suspend`). A method in kotlinc's suspend calling convention (an
//! instance method whose last parameter is a `Continuation` and that returns
//! `Object`) that overrides a method some supertype's metadata declares
//! `suspend`, or whose bridge does (kotlinc's for a SAM class's specialised
//! `emit(int, Continuation)`: a synthetic bridge of the same name and arity,
//! the class's one other method of that shape), is that suspend function: Kotlin allows
//! nothing else to override it. With the fix it is written `override
//! suspend fun emit(value: T)`, without the continuation parameter, with
//! the result type the overridden function declares (or, when that names a
//! type parameter, the continuation's type argument in the method's
//! signature), and it counts as a suspend function for the rebuild of its
//! state machine and its calls (fixes `kt_suspend_state_machine` and
//! `kt_suspend_call_continuation`).

use std::rc::Rc;

use crate::code_constants::{ACC_BRIDGE, ACC_STATIC, ACC_SYNTHETIC};
use crate::decompiler_context::DecompilerContext;
use crate::generic_type::GenericType;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::k_types::KTypes;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;

const SUSPEND_DESCRIPTOR_END: &str = "Lkotlin/coroutines/Continuation;)Ljava/lang/Object;";

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
