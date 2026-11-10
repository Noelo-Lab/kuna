//! kuna fix `kt_synthetic_accessor_call`: a call of a synthetic accessor
//! kotlinc generates for a private member is written as the access it
//! makes.
//!
//! A private member used from another JVM class of its file (a lambda or
//! object class, a class beside a top-level declaration, an inlined body) is
//! reached through a static synthetic method of its class: `access$getX$p`
//! and `access$setX$p` read and write a property's field, `access$foo`
//! calls a function. Vineflower replaces these calls only where the caller
//! is nested in the accessor's class; elsewhere it writes the accessor,
//! `` Http2Connection.`access$getPushObserver$p`(c) ``, which Kotlin has no
//! name for. With the fix, when the accessor's code only passes its
//! parameters, in order, to one field access or call of a member of its
//! class that has Kotlin metadata, and returns what that gives, the call is
//! the access: a property read `c.pushObserver` or assignment, a call
//! `log(task, queue, message)` of the function, written as the call of that
//! function is (receiver, name, import: the fixes `kt_extension_call`,
//! `kt_facade_call`, `kt_jvm_name_call`). A static field is resugared only
//! for a top-level property; an `invokespecial` only of the accessor's own
//! class (a super call keeps its accessor).

use crate::code_constants::{
    ACC_INTERFACE, ACC_STATIC, ACC_SYNTHETIC, OPC_ALOAD, OPC_ALOAD_3, OPC_GETFIELD, OPC_GETSTATIC, OPC_ILOAD,
    OPC_ILOAD_0, OPC_INVOKEINTERFACE, OPC_INVOKESPECIAL, OPC_INVOKESTATIC, OPC_INVOKEVIRTUAL,
    OPC_IRETURN, OPC_PUTFIELD, OPC_PUTSTATIC, OPC_RETURN,
};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::{InvocationExprent, InvocationType, Type};
use crate::k_element::KElement;
use crate::k_property::PropertyMethod;
use crate::kotlin::kuna_fix_kt_facade_call;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::link_constant::LinkConstant;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

/// What the accessor's code does after loading its parameters.
struct Access {
    opcode: i32,
    link: LinkConstant,
}

/// The accessor call's own shape, which [`Resugared`] puts back.
struct Shape {
    name: String,
    descriptor: MethodDescriptor,
    string_descriptor: String,
    is_static: bool,
    instance: Option<Exprent>,
    params: Vec<Exprent>,
    invocation_type: InvocationType,
}

/// An accessor call rewritten into the access while it is written; dropping
/// it restores the call.
pub struct Resugared {
    invocation: InvocationExprent,
    shape: Shape,
    pub calling_data: KElement,
    pub property_method: Option<PropertyMethod>,
}

impl Drop for Resugared {
    fn drop(&mut self) {
        let invocation = self.invocation;
        if !invocation.is_live() {
            return;
        }
        let shape = &self.shape;
        invocation.set_name(shape.name.clone());
        invocation.set_descriptor(shape.descriptor.clone());
        invocation.set_string_descriptor(shape.string_descriptor.clone());
        invocation.set_static(shape.is_static);
        invocation.set_instance(shape.instance);
        invocation.set_lst_parameters(shape.params.clone());
        let invocation_type = shape.invocation_type;
        invocation.d_mut(|d| d.invocation_type = invocation_type);
    }
}

/// Rewrites `invocation` into the access the accessor it calls makes, until
/// the returned guard is dropped; with the callee's metadata (and accessor
/// kind).
pub fn resugar(invocation: InvocationExprent) -> Option<Resugared> {
    let name = invocation.get_name();
    if !invocation.is_static()
        || invocation.get_functype() != Type::General
        || !name.starts_with("access$")
    {
        return None;
    }
    let context = DecompilerContext::get_struct_context();
    let classname = invocation.get_classname();
    let cl = context.get_class(&classname)?;
    let mt = cl.get_method(&name, &invocation.get_string_descriptor())?;
    if !mt.has_modifier(ACC_STATIC) || !mt.has_modifier(ACC_SYNTHETIC) {
        return None;
    }
    let access = read(cl, mt)?;
    let owner_name = access.link.classname.clone()?;
    if owner_name != classname {
        return None;
    }
    KotlinChooser::parse_metadata_for(cl);
    let args = invocation.get_lst_parameters().to_vec();
    let element = access.link.elementname.clone();
    let desc = access.link.descriptor.clone();
    let field = |static_field: bool| {
        let field = cl.get_field(&element, &desc)?;
        if field.has_modifier(ACC_STATIC) != static_field {
            return None;
        }
        match KtAttrs::get(field, &KElement::KEY()) {
            Some(KElement::Property(p)) => Some(KElement::Property(p)),
            _ => None,
        }
    };
    let (calling, method, instance, params, descriptor) = match access.opcode {
        OPC_GETFIELD if args.len() == 1 => (
            field(false)?,
            Some(PropertyMethod::Getter),
            Some(args[0]),
            Vec::new(),
            format!("(){desc}"),
        ),
        OPC_PUTFIELD if args.len() == 2 => (
            field(false)?,
            Some(PropertyMethod::Setter),
            Some(args[0]),
            vec![args[1]],
            format!("({desc})V"),
        ),
        OPC_GETSTATIC if args.is_empty() => {
            kuna_fix_kt_facade_call::top_level_package(&classname)?;
            (
                field(true)?,
                Some(PropertyMethod::Getter),
                None,
                Vec::new(),
                format!("(){desc}"),
            )
        }
        OPC_PUTSTATIC if args.len() == 1 => {
            kuna_fix_kt_facade_call::top_level_package(&classname)?;
            (
                field(true)?,
                Some(PropertyMethod::Setter),
                None,
                vec![args[0]],
                format!("({desc})V"),
            )
        }
        OPC_INVOKEVIRTUAL | OPC_INVOKESPECIAL | OPC_INVOKEINTERFACE | OPC_INVOKESTATIC => {
            let target = cl.get_method(&element, &desc)?;
            let is_static = access.opcode == OPC_INVOKESTATIC;
            let taken = target.method_descriptor().params.len() + usize::from(!is_static);
            if target.has_modifier(ACC_STATIC) != is_static || args.len() != taken {
                return None;
            }
            let calling = KtAttrs::get(target, &KElement::KEY())?;
            if !matches!(calling, KElement::Function(_) | KElement::Property(_)) {
                return None;
            }
            let method = KtAttrs::get(target, &PropertyMethod::KEY());
            if is_static {
                (calling, method, None, args, desc.clone())
            } else {
                (calling, method, Some(args[0]), args[1..].to_vec(), desc.clone())
            }
        }
        _ => return None,
    };
    let shape = Shape {
        name: name.clone(),
        descriptor: invocation.get_descriptor(),
        string_descriptor: invocation.get_string_descriptor(),
        is_static: true,
        instance: invocation.get_instance(),
        params: invocation.get_lst_parameters().to_vec(),
        invocation_type: invocation.get_invocation_type(),
    };
    if instance.is_some() {
        invocation.set_static(false);
        invocation.set_instance(instance);
        let invocation_type = if cl.has_modifier(ACC_INTERFACE) {
            InvocationType::Interface
        } else {
            InvocationType::Virtual
        };
        invocation.d_mut(|d| d.invocation_type = invocation_type);
    }
    invocation.set_name(element);
    invocation.set_descriptor(MethodDescriptor::parse_descriptor(&descriptor));
    invocation.set_string_descriptor(descriptor);
    invocation.set_lst_parameters(params);
    Some(Resugared {
        invocation,
        shape,
        calling_data: calling,
        property_method: method,
    })
}

/// The one access `mt`'s code makes after loading every parameter in
/// order, when it then returns.
fn read(cl: StructClass, mt: StructMethod) -> Option<Access> {
    if !mt.contains_code() {
        return None;
    }
    let expanded = mt.get_instruction_sequence().is_some_and(|seq| seq.is_live());
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return None;
    }
    let access = scan(cl, mt);
    if !expanded {
        mt.release_resources();
    }
    access
}

fn scan(cl: StructClass, mt: StructMethod) -> Option<Access> {
    let seq = mt.get_instruction_sequence()?;
    let params = mt.method_descriptor().params.clone();
    if seq.length() != params.len() as i32 + 2 {
        return None;
    }
    let mut slot = 0;
    for (i, param) in params.iter().enumerate() {
        let instr = seq.get_instr(i as i32);
        let op = instr.opcode();
        let loaded = if (OPC_ILOAD..=OPC_ALOAD).contains(&op) {
            instr.operand(0)
        } else if (OPC_ILOAD_0..=OPC_ALOAD_3).contains(&op) {
            (op - OPC_ILOAD_0) % 4
        } else {
            return None;
        };
        if loaded != slot {
            return None;
        }
        slot += param.stack_size;
    }
    let access = seq.get_instr(params.len() as i32);
    let ret = seq.get_instr(params.len() as i32 + 1).opcode();
    if !(OPC_IRETURN..=OPC_RETURN).contains(&ret) {
        return None;
    }
    let opcode = access.opcode();
    if !matches!(
        opcode,
        OPC_GETFIELD
            | OPC_PUTFIELD
            | OPC_GETSTATIC
            | OPC_PUTSTATIC
            | OPC_INVOKEVIRTUAL
            | OPC_INVOKESPECIAL
            | OPC_INVOKESTATIC
            | OPC_INVOKEINTERFACE
    ) {
        return None;
    }
    let link = cl.get_pool().get_link_constant(access.operand(0))?;
    Some(Access { opcode, link })
}
