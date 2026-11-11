//! kuna fix `kt_reused_inline_object`: an object an inline function creates,
//! which kotlinc creates again in a caller without copying its class, is
//! written as the call of the inline function.
//!
//! kotlinc inlines `compareBy(selector)` into its caller by copying the
//! function's body. The anonymous class the body creates
//! (`ComparisonsKt__ComparisonsKt$compareBy$2`) is copied too when the copy
//! must differ (a lambda argument inlined into it); when the caller passes
//! on a value it already holds (`sortedWith(compareBy(selector))` in the
//! inline `Sequence.sortedBy`, whose `selector` is its own parameter),
//! kotlinc creates the function's own class at the call site:
//! `new ComparisonsKt__ComparisonsKt$compareBy$2(selector)`. Vineflower
//! writes a `new` of an anonymous class as the class's body, which was
//! decompiled against the variables of the method that declares it and
//! cannot be written elsewhere; the Kotlin writer throws on it ("Anonymous
//! class does not have Class Kotlin metadata") and the caller fails.
//!
//! With the fix such a `new` in another top-level class than the one that
//! declares the class is written as the call of the declaring method when
//! the call means the same: the method is a public top-level inline
//! function (not an extension, no context receivers, not suspend) whose
//! code is, after checks of its parameters and kotlinc's `$i$f$` marker,
//! `new C(p1, .., pn)` of its parameters in order with the constructor the
//! call site calls, returned: `compareBy(selector)`. Inlined, the call
//! creates the class from the same arguments.

use std::rc::Rc;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::ACC_STATIC;
use crate::code_constants::{
    OPC_ALOAD, OPC_ARETURN, OPC_BIPUSH, OPC_CHECKCAST, OPC_DUP, OPC_ILOAD, OPC_INVOKESPECIAL,
    OPC_INVOKESTATIC, OPC_ISTORE, OPC_LDC, OPC_LDC_W, OPC_NEW,
};
use crate::decompiler_context::DecompilerContext;
use crate::full_instruction_sequence::FullInstructionSequence;
use crate::invocation_exprent::{InvocationExprent, InvocationType, Type};
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::kotlin::kuna_fix_kt_facade_call as facade;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::Visibility;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;

const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
const PARAMETER_CHECKS: [&str; 2] = ["checkNotNullParameter", "checkParameterIsNotNull"];

/// The call of the inline function whose object the anonymous class `node`
/// is, written for its `new` calling `constructor` outside that function;
/// `None` to leave the `new` to the writer.
pub fn write(node: ClassNode, constructor: InvocationExprent, indent: i32) -> Option<TextBuffer> {
    let site = top_level(DecompilerContext::get_current_class_node()?);
    if site == top_level(node) {
        return None;
    }
    let (function, owner, mt) = creator(node, &constructor.get_string_descriptor())?;
    let call = constructor.copy().as_invocation();
    call.set_classname(owner.qualified_name());
    call.set_name(mt.get_name());
    call.set_string_descriptor(mt.get_descriptor());
    call.set_descriptor(mt.method_descriptor());
    call.set_static(true);
    call.set_instance(None);
    call.set_functype(Type::General);
    call.d_mut(|d| d.invocation_type = InvocationType::Static);
    facade::write_function(call, &function, indent)
}

/// The top-level class `node` is nested in (itself when it is one).
fn top_level(node: ClassNode) -> ClassNode {
    let mut node = node;
    while let Some(parent) = node.with(|d| d.parent) {
        node = parent;
    }
    node
}

/// The inline function of `node`'s enclosing method, its class and method,
/// when the method's code is the creation of `node` with the constructor
/// `descriptor` from its parameters in order.
fn creator(
    node: ClassNode,
    descriptor: &str,
) -> Option<(Rc<KFunction>, StructClass, StructMethod)> {
    let (type_, cl) = node.with(|d| (d.type_, d.class_struct));
    if type_ != ClassNodeType::Anonymous {
        return None;
    }
    let attr = cl.get_attribute(StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD)?;
    let owner_name = attr.get_class_name()?;
    let owner = DecompilerContext::get_struct_context().get_class(&owner_name)?;
    let mt = owner.get_method(&attr.get_method_name()?, &attr.get_method_descriptor()?)?;
    facade::top_level_package(&owner_name)?;
    KotlinChooser::parse_metadata_for(owner);
    let Some(KElement::Function(function)) = KtAttrs::get(mt, &KElement::KEY()) else {
        return None;
    };
    if !mt.has_modifier(ACC_STATIC)
        || !Flags::IS_INLINE.get(function.flags)
        || Flags::IS_SUSPEND.get(function.flags)
        || Flags::VISIBILITY.get(function.flags) != Some(Visibility::Public)
        || function.receiver_type.is_some()
        || !function.context_receiver_types.is_empty()
        || mt.method_descriptor().params.len() != function.parameters.len()
        || mt.get_descriptor().split(')').next() != descriptor.split(')').next()
    {
        return None;
    }
    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(owner)).is_err() {
        return None;
    }
    let creates = mt.get_instruction_sequence().is_some_and(|seq| {
        creates_from_parameters(owner, mt, seq, &cl.qualified_name(), descriptor)
    });
    if !expanded {
        mt.release_resources();
    }
    creates.then_some((function, owner, mt))
}

/// Whether `mt`'s code checks some parameters, sets an unread local (the
/// `$i$f$` marker), creates `class` with the constructor `descriptor` from
/// its parameters in order and returns it.
fn creates_from_parameters(
    owner: StructClass,
    mt: StructMethod,
    seq: FullInstructionSequence,
    class: &str,
    descriptor: &str,
) -> bool {
    let pool = owner.get_pool();
    let n = seq.length();
    let at = |i: i32| (i < n).then(|| seq.get_instr(i));
    let invokes = |i: i32, opcode: i32, class: &str, names: &[&str], desc: &str| {
        at(i).is_some_and(|ins| {
            ins.opcode() == opcode
                && pool.get_link_constant(ins.operand(0)).is_some_and(|link| {
                    link.classname.as_deref() == Some(class)
                        && names.contains(&link.elementname.as_str())
                        && link.descriptor == desc
                })
        })
    };
    let mut slots = Vec::new();
    let mut slot = 0;
    for param in &mt.method_descriptor().params {
        slots.push(slot);
        slot += param.stack_size;
    }
    let loads = |i: i32, slot: i32| {
        at(i).is_some_and(|ins| {
            (OPC_ILOAD..=OPC_ALOAD).contains(&ins.opcode()) && ins.operand(0) == slot
        })
    };
    let mut i = 0;
    while at(i).is_some_and(|ins| ins.opcode() == OPC_ALOAD)
        && at(i + 1).is_some_and(|ins| matches!(ins.opcode(), OPC_LDC | OPC_LDC_W))
        && invokes(
            i + 2,
            OPC_INVOKESTATIC,
            INTRINSICS,
            &PARAMETER_CHECKS,
            "(Ljava/lang/Object;Ljava/lang/String;)V",
        )
    {
        i += 3;
    }
    if at(i).is_some_and(|ins| ins.opcode() == OPC_BIPUSH && ins.operand(0) == 0)
        && at(i + 1).is_some_and(|ins| ins.opcode() == OPC_ISTORE && ins.operand(0) >= slot)
    {
        i += 2;
    }
    let creates = at(i).is_some_and(|ins| {
        ins.opcode() == OPC_NEW
            && pool
                .get_primitive_constant(ins.operand(0))
                .is_some_and(|c| &*c.get_string() == class)
    });
    if !creates || at(i + 1).is_none_or(|ins| ins.opcode() != OPC_DUP) {
        return false;
    }
    i += 2;
    for &slot in &slots {
        if !loads(i, slot) {
            return false;
        }
        i += 1;
    }
    if !invokes(i, OPC_INVOKESPECIAL, class, &["<init>"], descriptor) {
        return false;
    }
    i += 1;
    if at(i).is_some_and(|ins| ins.opcode() == OPC_CHECKCAST) {
        i += 1;
    }
    at(i).is_some_and(|ins| ins.opcode() == OPC_ARETURN) && i + 1 == n
}
