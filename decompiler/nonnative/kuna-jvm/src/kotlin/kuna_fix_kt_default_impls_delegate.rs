//! kuna fix `kt_default_impls_delegate` (spec 10.16): the methods kotlinc
//! generates to call an interface member's default body are not written.
//!
//! Without JVM default methods (`-Xjvm-default` off: kotlinx-coroutines,
//! okhttp), kotlinc compiles the body of an interface's non-abstract member
//! into a static method of the interface's `DefaultImpls` class, and gives
//! each class that implements the interface without overriding the member a
//! method of the member's signature that calls it with `this`, `return
//! Job.DefaultImpls.plus(this, other)`; the `DefaultImpls` of an interface
//! that inherits the member gets one that calls the super-interface's. The
//! source has neither: the class inherits the member, and the class's
//! metadata does not declare it. Vineflower writes each as a function, an
//! `override` in a class, whose call names a class Kotlin does not see
//! (`DefaultImpls`) and whose signature, written from the JVM one, may
//! override nothing (`fold(initial: R, operation: (R?, Element?) -> R)`).
//! With the fix a method the metadata does not declare is not written when
//! its code loads `this` (in a `DefaultImpls` class, its first parameter)
//! and then each of its parameters in order (each perhaps cast), calls the
//! method of the same name, parameters and result of the `DefaultImpls`
//! class of an interface its class implements (or, in a `DefaultImpls`, its
//! interface extends) with them, and returns the result (after Kotlin 1.3's
//! checks of its parameters); a `DefaultImpls` class left with nothing else
//! is not written either.

use std::collections::BTreeSet;

use crate::code_constants::{
    ACC_STATIC, OPC_ALOAD, OPC_ARETURN, OPC_CHECKCAST, OPC_DLOAD, OPC_DRETURN, OPC_FLOAD,
    OPC_FRETURN, OPC_ILOAD, OPC_INVOKESTATIC, OPC_IRETURN, OPC_LDC, OPC_LDC_W, OPC_LLOAD,
    OPC_LRETURN, OPC_RETURN,
};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::k_element::KElement;
use crate::kt_attrs::KtAttrs;
use crate::kuna_fixes::{self, KunaFix};
use crate::method_descriptor::MethodDescriptor;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;

const DEFAULT_IMPLS: &str = "$DefaultImpls";
const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
const PARAMETER_CHECKS: [&str; 2] = ["checkNotNullParameter", "checkParameterIsNotNull"];

/// Whether `mt`, a method of `cl`, is a call of an interface member's
/// default body kotlinc generated (see the module header).
pub fn is_delegate(cl: StructClass, mt: StructMethod) -> bool {
    if !kuna_fixes::enabled(KunaFix::KtDefaultImplsDelegate)
        || mt.get_name().starts_with('<')
        || KtAttrs::get(mt, &KElement::KEY()).is_some()
    {
        return false;
    }
    let name = cl.qualified_name();
    let is_static = mt.has_modifier(ACC_STATIC);
    let owner = match name.strip_suffix(DEFAULT_IMPLS) {
        Some(interface) if is_static => interface.to_string(),
        None if !is_static => name.to_string(),
        _ => return false,
    };
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
    kuna_fixes::enabled(KunaFix::KtDefaultImplsDelegate)
        && cl.qualified_name().ends_with(DEFAULT_IMPLS)
        && cl.get_fields().is_empty()
        && {
            let methods = cl.get_methods();
            !methods.is_empty() && methods.iter().all(|m| is_delegate(cl, *m))
        }
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
