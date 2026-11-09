//! kuna fix `kt_property_reference`: a property reference (`x::p`,
//! `C::p`, `R::p`, `::p`) is written as one.
//!
//! kotlinc compiles a property reference it cannot leave to
//! `invokedynamic` to a synthetic class with synthetic-class metadata and no
//! data, extending `kotlin.jvm.internal.PropertyReference{0,1,2}Impl` (or
//! the `MutablePropertyReference` ones), whose constructor calls
//! `super([receiver,] Owner.class, "name", "getterSignature", flags)`.
//! `KotlinChooser` recognises only `FunctionReferenceImpl` subclasses as
//! references, so a property reference reaches `KotlinWriter` as an
//! anonymous object without Class metadata, which throws for a bound one
//! (`new C$f$1(receiver)`) and prints `<unrepresentable>.INSTANCE` for an
//! unbound one. With the fix such a class is written as the reference its
//! super call describes: `receiver::name` when bound; unbound, the number in
//! the superclass name counts the receivers `get` takes, and bit 0 of
//! `flags` says the property is top-level (kotlin's `CallableReference`
//! `isTopLevel`), so `::name` (top-level, none), `Receiver::name` (top-level
//! extension, the getter's first parameter type), or `Owner::name` (a member,
//! one). A member extension (two receivers) has no reference syntax, and any
//! other constructor shape is left as before.

use crate::classes_processor::ClassNode;
use crate::code_constants::INIT_NAME;
use crate::exprent::Exprent;
use crate::k_types::KTypes;
use crate::kuna_fix_kt_reference_anonymous_super_call;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::BitSet;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

const PROPERTY_REFERENCE_PREFIXES: [&str; 2] = [
    "kotlin/jvm/internal/PropertyReference",
    "kotlin/jvm/internal/MutablePropertyReference",
];

/// The number of receivers `get` takes when `superclass` is one of the
/// property-reference base classes (`PropertyReference1Impl` -> 1).
fn receiver_count(superclass: &str) -> Option<usize> {
    PROPERTY_REFERENCE_PREFIXES.iter().find_map(|prefix| {
        let rest = superclass.strip_prefix(prefix)?.strip_suffix("Impl")?;
        match rest {
            "0" => Some(0),
            "1" => Some(1),
            "2" => Some(2),
            _ => None,
        }
    })
}

/// The reference the property-reference class `node` stands for, created
/// with `receiver` (bound) or read from its `INSTANCE` (unbound); `None` when
/// `node` is not such a class or its constructor has another shape.
pub fn stringify(
    indent: i32,
    node: ClassNode,
    bytecode: Option<&BitSet>,
    receiver: Option<Exprent>,
) -> Option<TextBuffer> {
    let cl = node.with(|d| d.class_struct);
    let receivers = receiver_count(&cl.super_class()?.get_string())?;
    let init = node.get_wrapper()?.get_method_wrapper(
        INIT_NAME,
        if receiver.is_some() {
            "(Ljava/lang/Object;)V"
        } else {
            "()V"
        },
    )?;
    let exprents = init.with(|d| d.root)?.get_first()?.get_exprents();
    let super_call = kuna_fix_kt_reference_anonymous_super_call::super_call(node, exprents)?
        .try_as_invocation()?;
    if super_call.get_name() != INIT_NAME {
        return None;
    }
    let parameters = super_call.get_lst_parameters().to_vec();
    let consts = &parameters[usize::from(receiver.is_some())..];
    let [owner, name, signature, flags] = consts else {
        return None;
    };
    let string = |e: &Exprent, class: bool| -> Option<String> {
        let c = e.try_as_const()?;
        let is_class = c.get_const_type().value.as_deref() == Some("java/lang/Class");
        if is_class != class {
            return None;
        }
        c.get_value().map(|v| v.java_to_string())
    };
    let (owner, name, signature) = (
        string(owner, true)?,
        string(name, false)?,
        string(signature, false)?,
    );
    let top_level = flags.try_as_const()?.get_value()?.as_int() & 1 == 1;
    let paren = signature.find('(')?;
    let (getter, descriptor) = signature.split_at(paren);

    let mut buf = TextBuffer::new();
    buf.add_bytecode_mapping(bytecode);
    match (receiver, receivers, top_level) {
        (Some(receiver), 0, _) => {
            buf.append_buffer(&receiver.to_java(indent));
        }
        (None, 0, true) => {}
        (None, 1, true) => {
            let first = MethodDescriptor::parse_descriptor(descriptor)
                .params
                .first()?
                .clone();
            buf.append_type_name_str(&KTypes::get_kotlin_type(Some(&first)), &first);
        }
        (None, 1, false) => {
            let owner_type = VarType::new_signature_cl(&owner, true);
            buf.append_type_name_str(&KTypes::get_kotlin_type(Some(&owner_type)), &owner_type);
        }
        _ => return None,
    }
    buf.append("::");
    buf.append_method(&name, false, &owner, getter, descriptor);
    Some(buf)
}
