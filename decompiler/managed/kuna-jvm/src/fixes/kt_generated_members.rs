//! Fix family `kt_generated_members` (Kotlin output): the members kotlinc
//! generates, which the writer leaves out (default-arguments constructors,
//! `@JvmOverloads` overloads, a file class's constructor, `DefaultImpls`
//! delegates) or recognises (a companion member's static copy, an interface
//! companion's `$$INSTANCE`).

use std::collections::BTreeSet;

use crate::code_constants::{
    ACC_STATIC, OPC_ALOAD, OPC_ARETURN, OPC_CHECKCAST, OPC_DLOAD, OPC_DRETURN, OPC_FLOAD,
    OPC_FRETURN, OPC_ILOAD, OPC_INVOKESPECIAL, OPC_INVOKESTATIC, OPC_IRETURN, OPC_LDC, OPC_LDC_W,
    OPC_LLOAD, OPC_LRETURN, OPC_RETURN,
};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::fixes::kt_util::{INTRINSICS, PARAMETER_CHECKS};
use crate::k_element::KElement;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;

const MARKER_TAIL: &str = "ILkotlin/jvm/internal/DefaultConstructorMarker;)V";
const JVM_OVERLOADS: &str = "kotlin/jvm/JvmOverloads";
const DEFAULT_IMPLS: &str = "$DefaultImpls";

/// Whether `mt`, a member of the Kotlin class `cl`, is a constructor kotlinc
/// generated for default arguments: it only calls the default-arguments constructor.
pub fn is_generated_constructor(cl: StructClass, mt: StructMethod) -> bool {
    if mt.get_name() != "<init>"
        || KtAttrs::get(mt, &KElement::KEY()).is_some()
        || !matches!(KtAttrs::get(cl, &KElement::KEY()), Some(KElement::Class(_)))
    {
        return false;
    }
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    let pool = cl.get_pool();
    let name = cl.qualified_name();
    let mut calls = 0;
    let mut returned = false;
    for i in 0..seq.length() {
        let instr = seq.get_instr(i);
        let opcode = instr.opcode();
        if returned {
            return false;
        }
        if (1..=45).contains(&opcode) {
            continue;
        }
        if opcode == OPC_RETURN {
            returned = true;
            continue;
        }
        if opcode != OPC_INVOKESPECIAL || calls > 0 {
            return false;
        }
        let Some(link) = pool.get_link_constant(instr.operand(0)) else {
            return false;
        };
        if link.classname.as_deref() != Some(name.as_str())
            || link.elementname != "<init>"
            || !link.descriptor.ends_with(MARKER_TAIL)
        {
            return false;
        }
        calls += 1;
    }
    calls == 1 && returned
}

/// Whether `mt`, a member of `cl` without Kotlin metadata, is an overload
/// kotlinc generated for an `@JvmOverloads` function: it only checks its
/// parameters and calls the function's `$default` bridge.
pub fn is_generated_overload(cl: StructClass, mt: StructMethod) -> bool {
    if mt.get_name().starts_with('<') || KtAttrs::get(mt, &KElement::KEY()).is_some() {
        return false;
    }
    let annotated = KotlinWriter::ANNOTATION_ATTRIBUTES.iter().any(|key| {
        mt.get_attribute(*key)
            .is_some_and(|a| a.get_annotations().iter().any(|an| an.get_class_name() == JVM_OVERLOADS))
    });
    if !annotated {
        return false;
    }
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    let pool = cl.get_pool();
    let bridge = format!("{}$default", mt.get_name());
    let mut calls = 0;
    let mut returned = false;
    for i in 0..seq.length() {
        let instr = seq.get_instr(i);
        let opcode = instr.opcode();
        if returned {
            return false;
        }
        if (1..=45).contains(&opcode) {
            continue;
        }
        if (OPC_IRETURN..=OPC_ARETURN).contains(&opcode) || opcode == OPC_RETURN {
            returned = true;
            continue;
        }
        if opcode != OPC_INVOKESTATIC || calls > 0 {
            return false;
        }
        let Some(link) = pool.get_link_constant(instr.operand(0)) else {
            return false;
        };
        if link.classname.as_deref() == Some(INTRINSICS) && PARAMETER_CHECKS.contains(&link.elementname.as_str()) {
            continue;
        }
        if link.elementname != bridge {
            return false;
        }
        calls += 1;
    }
    calls == 1 && returned
}

/// Whether `mt`, a member of `cl`, is the constructor of a Kotlin file class
/// that only calls its superclass's constructor.
pub fn is_file_class_constructor(cl: StructClass, mt: StructMethod) -> bool {
    if mt.get_name() != "<init>"
        || mt.get_descriptor() != "()V"
        || KtAttrs::get(mt, &KElement::KEY()).is_some()
        || !matches!(KtAttrs::get(cl, &KElement::KEY()), Some(KElement::File(_)))
    {
        return false;
    }
    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return false;
    }
    let only_super_call = calls_only_super(cl, mt);
    if !expanded {
        mt.release_resources();
    }
    only_super_call
}

fn calls_only_super(cl: StructClass, mt: StructMethod) -> bool {
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    let Some(super_class) = cl.super_class() else {
        return false;
    };
    if seq.length() != 3
        || seq.get_instr(0).opcode() != OPC_ALOAD
        || seq.get_instr(0).operand(0) != 0
        || seq.get_instr(1).opcode() != OPC_INVOKESPECIAL
        || seq.get_instr(2).opcode() != OPC_RETURN
    {
        return false;
    }
    cl.get_pool()
        .get_link_constant(seq.get_instr(1).operand(0))
        .is_some_and(|link| {
            link.classname.as_deref() == Some(&*super_class.get_string())
                && link.elementname == "<init>"
                && link.descriptor == "()V"
        })
}

/// Whether `mt`, a method of `cl`, is a call of an interface member's
/// default body kotlinc generated: it checks its parameters and passes them
/// on to the `DefaultImpls` method, or the interface's default method.
pub fn is_delegate(cl: StructClass, mt: StructMethod) -> bool {
    if mt.get_name().starts_with('<') || KtAttrs::get(mt, &KElement::KEY()).is_some() {
        return false;
    }
    let name = cl.qualified_name();
    let is_static = mt.has_modifier(ACC_STATIC);
    let owner = match name.strip_suffix(DEFAULT_IMPLS) {
        Some(interface) if is_static => interface.to_string(),
        None if !is_static => name.to_string(),
        _ => return false,
    };
    if !kotlin_class(&owner) {
        return false;
    }
    let desc = mt.get_descriptor();
    let Some(close) = desc.find(')') else {
        return false;
    };
    let (params, ret) = (&desc[1..close], &desc[close + 1..]);
    let rest = if is_static {
        match params
            .strip_prefix('L')
            .and_then(|p| p.find(';').map(|i| &p[i + 1..]))
        {
            Some(rest) => rest,
            None => return false,
        }
    } else {
        params
    };
    let md = MethodDescriptor::parse_descriptor(&desc);
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    let pool = cl.get_pool();
    let mut i = 0;
    while i + 2 < seq.length()
        && seq.get_instr(i).opcode() == OPC_ALOAD
        && matches!(seq.get_instr(i + 1).opcode(), OPC_LDC | OPC_LDC_W)
        && seq.get_instr(i + 2).opcode() == OPC_INVOKESTATIC
        && pool
            .get_link_constant(seq.get_instr(i + 2).operand(0))
            .is_some_and(|l| {
                l.classname.as_deref() == Some(INTRINSICS)
                    && PARAMETER_CHECKS.contains(&l.elementname.as_str())
            })
    {
        i += 3;
    }
    let mut slot = 0;
    for param in
        std::iter::once(None).chain(md.params.iter().skip(usize::from(is_static)).map(Some))
    {
        let load = param.map_or(OPC_ALOAD, load_opcode);
        if i >= seq.length()
            || seq.get_instr(i).opcode() != load
            || seq.get_instr(i).operand(0) != slot
        {
            return false;
        }
        i += 1;
        if i < seq.length() && seq.get_instr(i).opcode() == OPC_CHECKCAST {
            i += 1;
        }
        slot += param.map_or(1, |p| p.stack_size);
    }
    if i + 2 != seq.length() || seq.get_instr(i).opcode() != OPC_INVOKESTATIC {
        return false;
    }
    if seq.get_instr(i + 1).opcode() != return_opcode(&md.ret) {
        return false;
    }
    let Some(link) = pool.get_link_constant(seq.get_instr(i).operand(0)) else {
        return false;
    };
    let Some(interface) = link
        .classname
        .as_deref()
        .and_then(|c| c.strip_suffix(DEFAULT_IMPLS))
    else {
        return false;
    };
    link.elementname == mt.get_name()
        && link.descriptor == format!("(L{interface};{rest}){ret}")
        && supertypes(&owner).contains(interface)
}

/// Whether `cl` is a `DefaultImpls` class that holds nothing but such
/// calls, which the fix leaves it without.
pub fn is_emptied_default_impls(cl: StructClass) -> bool {
    cl.qualified_name().ends_with(DEFAULT_IMPLS) && cl.get_fields().is_empty() && {
        let methods = cl.get_methods();
        !methods.is_empty() && methods.iter().all(|m| is_delegate(cl, *m))
    }
}

/// Whether `class` has Kotlin class metadata, which declares the members its
/// source does.
fn kotlin_class(class: &str) -> bool {
    let Some(cl) = DecompilerContext::get_struct_context().get_class(class) else {
        return false;
    };
    KotlinChooser::parse_metadata_for(cl);
    matches!(KtAttrs::get(cl, &KElement::KEY()), Some(KElement::Class(_)))
}

/// The interfaces and classes `class` extends or implements, transitively.
fn supertypes(class: &str) -> BTreeSet<String> {
    let context = DecompilerContext::get_struct_context();
    let mut seen = BTreeSet::new();
    let mut pending = vec![class.to_string()];
    while let Some(name) = pending.pop() {
        if seen.len() > 256 {
            break;
        }
        let Some(cl) = context.get_class(&name) else {
            continue;
        };
        for parent in cl
            .get_interface_names()
            .into_iter()
            .chain(cl.super_class().map(|s| s.get_string().to_string()))
        {
            if seen.insert(parent.clone()) {
                pending.push(parent);
            }
        }
    }
    seen
}

fn load_opcode(t: &VarType) -> i32 {
    if t.array_dim > 0 {
        return OPC_ALOAD;
    }
    match t.type_ {
        CodeType::Long => OPC_LLOAD,
        CodeType::Float => OPC_FLOAD,
        CodeType::Double => OPC_DLOAD,
        CodeType::Object => OPC_ALOAD,
        _ => OPC_ILOAD,
    }
}

fn return_opcode(t: &VarType) -> i32 {
    if t.array_dim > 0 {
        return OPC_ARETURN;
    }
    match t.type_ {
        CodeType::Void => OPC_RETURN,
        CodeType::Long => OPC_LRETURN,
        CodeType::Float => OPC_FRETURN,
        CodeType::Double => OPC_DRETURN,
        CodeType::Object => OPC_ARETURN,
        _ => OPC_IRETURN,
    }
}

/// Whether `in_parent`, a method of a companion's class with a companion
/// member's JVM signature, is that member's static copy.
pub fn is_copy(in_parent: StructMethod) -> bool {
    in_parent.has_modifier(ACC_STATIC)
}

/// Whether `initializer` reads the static `$$INSTANCE` field of `companion`, an
/// interface companion's instance.
pub fn is_instance_read(initializer: &Exprent, companion: StructClass) -> bool {
    let Some(field) = initializer.try_as_field() else {
        return false;
    };
    let descriptor = format!("L{};", companion.qualified_name());
    field.is_static()
        && field.get_instance().is_none()
        && field.get_classname() == companion.qualified_name()
        && field.get_name() == "$$INSTANCE"
        && field.get_descriptor().descriptor_string == descriptor
        && companion.has_field("$$INSTANCE", &descriptor)
}
