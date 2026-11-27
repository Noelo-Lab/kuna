//! Fixes `kt_object_super_call` and `kt_synthetic_object_super_call`: an
//! object expression's superclass is written with the arguments of its
//! constructor's superclass call, each constructor parameter replaced by what
//! the `new` passes for it, computed where the `new` stands before the class is
//! written; the second fix covers anonymous classes without Class metadata.

use std::cell::RefCell;
use std::thread::LocalKey;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::INIT_NAME;
use crate::exprent::Exprent;
use crate::fixes::kt_anonymous_objects::kt_synthetic_anonymous_object;
use crate::fixes::kt_delegation_calls::kt_default_args_super_call;
use crate::fixes::util::all_exprents;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_type::KType;
use crate::kt_attrs::KtAttrs;
use crate::method_descriptor::MethodDescriptor;
use crate::statements::Statements;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

const DEFAULT_CONSTRUCTOR_MARKER: &str = "Lkotlin/jvm/internal/DefaultConstructorMarker;";

type Arguments = Option<(ClassNode, TextBuffer)>;

thread_local! {
    static ARGUMENTS: RefCell<Arguments> = const { RefCell::new(None) };
    static SYNTHETIC_ARGUMENTS: RefCell<Arguments> = const { RefCell::new(None) };
}

/// Puts the enclosing `new`'s arguments back into its cell when dropped.
struct Restore(&'static LocalKey<RefCell<Arguments>>, Arguments);

impl Drop for Restore {
    fn drop(&mut self) {
        let outer = self.1.take();
        self.0.with(|a| *a.borrow_mut() = outer);
    }
}

fn at_site<R>(
    cell: &'static LocalKey<RefCell<Arguments>>,
    arguments: Arguments,
    write: impl FnOnce() -> R,
) -> R {
    let _restore = Restore(cell, cell.with(|a| a.replace(arguments)));
    write()
}

/// Runs `write` (`KNewExprent` writing the anonymous class `node` it creates
/// with `constructor`) with the superclass arguments of this `new` at hand.
pub fn write_at_site<R>(
    node: ClassNode,
    constructor: Option<InvocationExprent>,
    indent: i32,
    write: impl FnOnce() -> R,
) -> R {
    let arguments = constructor
        .and_then(|c| super_arguments(node, c, indent))
        .map(|text| (node, text));
    at_site(&ARGUMENTS, arguments, write)
}

/// [`write_at_site`] for an anonymous class without Class metadata, which
/// is then written here when [`is_writable`].
pub fn write_synthetic_at_site<R>(
    node: ClassNode,
    constructor: Option<InvocationExprent>,
    indent: i32,
    write: impl FnOnce() -> R,
) -> R {
    let arguments = constructor
        .and_then(|c| synthetic_super_arguments(node, c, indent))
        .map(|text| (node, text));
    at_site(&SYNTHETIC_ARGUMENTS, arguments, write)
}

/// Appends `(arguments)` after `supertype` in `node`'s header when it is the
/// class's superclass and [`write_at_site`] found its arguments.
pub fn append_arguments(node: ClassNode, supertype: &KType, buffer: &mut TextBuffer) {
    let superclass = node.with(|d| d.class_struct).super_class();
    if superclass.is_none_or(|s| supertype.value.as_deref() != Some(&*s.get_string())) {
        return;
    }
    ARGUMENTS.with(|a| {
        if let Some((_, text)) = a.borrow().as_ref().filter(|(n, _)| *n == node) {
            buffer.append_char('(').append_buffer(text).append_char(')');
        }
    });
}

/// Whether the `new` being written creates `node` as an object expression
/// whose superclass takes the arguments [`write_synthetic_at_site`] found.
pub fn is_writable(node: ClassNode) -> bool {
    SYNTHETIC_ARGUMENTS.with(|a| a.borrow().as_ref().is_some_and(|(n, _)| *n == node))
}

/// Appends `object : Super(arguments)` and the class's interfaces.
pub fn write_object_header(node: ClassNode, buffer: &mut TextBuffer) {
    let arguments = SYNTHETIC_ARGUMENTS.with(|a| {
        a.borrow()
            .as_ref()
            .filter(|(n, _)| *n == node)
            .map(|(_, text)| text.clone())
    });
    kt_synthetic_anonymous_object::write_object_header_with(
        node,
        arguments.as_ref(),
        buffer,
    );
}

/// The text of the superclass arguments of `node` created with
/// `constructor`, written in the current (the `new`'s) context; only for a
/// class whose header is written from Class metadata (writing the arguments
/// registers their imports).
fn super_arguments(
    node: ClassNode,
    constructor: InvocationExprent,
    indent: i32,
) -> Option<TextBuffer> {
    let cl = node.with(|d| d.class_struct);
    if !matches!(KtAttrs::get(cl, &KElement::KEY()), Some(KElement::Class(_))) {
        return None;
    }
    let superclass = cl.super_class()?.get_string().to_string();
    if superclass == "java/lang/Object" {
        return None;
    }
    let wrapper = node.get_wrapper()?;
    let init = wrapper.get_method_wrapper(INIT_NAME, &constructor.get_string_descriptor())?;
    let root = init.with(|d| d.root.filter(|_| d.decompile_error.is_none()))?;
    let args = constructor.get_lst_parameters().to_vec();
    let params = MethodDescriptor::parse_descriptor(&constructor.get_string_descriptor())
        .params
        .clone();
    if params.len() != args.len() {
        return None;
    }
    let slots = parameter_slots(&params);

    let is_super_call = |inv: &InvocationExprent| {
        inv.get_classname() == superclass
            && inv
                .get_instance()
                .and_then(|i| i.try_as_var())
                .is_some_and(|v| v.get_index() == 0)
    };
    let call = match node.with(|d| d.super_invocation) {
        Some(call) => Some(call).filter(is_super_call),
        None => root
            .get_basichead()
            .get_exprents()?
            .to_vec()
            .iter()
            .find_map(|e| {
                e.try_as_invocation().filter(|inv| {
                    Statements::is_invocation_init_constructor(*inv, init, wrapper, false)
                        && is_super_call(inv)
                })
            }),
    };
    let Some(call) = call else {
        let hidden = node.with(|d| d.super_invocation.is_none());
        return hidden.then(TextBuffer::new);
    };

    let call = substituted(call, &slots, &args)?;
    if call
        .get_string_descriptor()
        .contains(DEFAULT_CONSTRUCTOR_MARKER)
        && !kt_default_args_super_call::resugar(call)
    {
        return None;
    }
    Some(call.append_param_list(indent + 1))
}

/// The text of the superclass arguments of `node`, an anonymous class without
/// Class metadata, at a `new` calling `constructor`, or `None` when the object
/// cannot be written there.
fn synthetic_super_arguments(
    node: ClassNode,
    constructor: InvocationExprent,
    indent: i32,
) -> Option<TextBuffer> {
    let (type_, cl, super_call) = node.with(|d| (d.type_, d.class_struct, d.super_invocation));
    if type_ != ClassNodeType::Anonymous
        || matches!(KtAttrs::get(cl, &KElement::KEY()), Some(KElement::Class(_)))
    {
        return None;
    }
    let superclass = cl.super_class()?.get_string().to_string();
    if superclass == "java/lang/Object" {
        return None;
    }
    let descriptor = constructor.get_string_descriptor();
    let constructors = cl
        .get_methods()
        .as_slice()
        .iter()
        .filter(|m| m.get_name() == INIT_NAME)
        .count();
    let wrapper = node.get_wrapper()?;
    let init = wrapper.get_method_wrapper(INIT_NAME, &descriptor)?;
    let (root, mask) = init.with(|d| {
        (
            d.root.filter(|_| d.decompile_error.is_none()),
            d.synth_parameters.clone(),
        )
    });
    let root = root?;
    let args = constructor.get_lst_parameters().to_vec();
    let params = MethodDescriptor::parse_descriptor(&descriptor)
        .params
        .clone();
    let mask = mask.unwrap_or_else(|| vec![None; params.len()]);
    if constructors != 1 || params.len() != args.len() || mask.len() != args.len() {
        return None;
    }

    let slots = parameter_slots(&params);
    let readable = mask.iter().zip(&args).all(|(pair, arg)| match pair {
        Some(pair) => {
            kt_synthetic_anonymous_object::capture_reads_at_site(node, *pair, *arg)
        }
        None => arg.is_var() || arg.is_const(),
    });
    if !readable {
        return None;
    }

    let call = super_call.filter(|call| {
        call.get_classname() == superclass
            && call
                .get_instance()
                .and_then(|i| i.try_as_var())
                .is_some_and(|v| v.get_index() == 0)
    })?;
    if !all_exprents(root.0).is_empty() {
        return None;
    }

    let call = substituted(call, &slots, &args)?;
    Some(call.append_param_list(indent + 1))
}

/// The slot of each constructor parameter of `params`, after `this`.
fn parameter_slots(params: &[VarType]) -> Vec<i32> {
    let mut slots = Vec::with_capacity(params.len());
    let mut slot = 1;
    for param in params.iter() {
        slots.push(slot);
        slot += param.stack_size;
    }
    slots
}

/// A copy of the superclass call `call` whose arguments read what the `new`
/// passes (`args`) for the constructor parameters at `slots`.
fn substituted(call: InvocationExprent, slots: &[i32], args: &[Exprent]) -> Option<InvocationExprent> {
    let call = call.0.copy().as_invocation();
    for param in call.get_lst_parameters().to_vec() {
        let replaced = substitute(param, slots, args)?;
        if replaced != param {
            call.0.replace_exprent(param, replaced);
        }
    }
    Some(call)
}

/// `exprent` with every read of a constructor parameter replaced by a copy
/// of what the `new` passes for it; `None` when it reads anything else.
fn substitute(exprent: Exprent, slots: &[i32], args: &[Exprent]) -> Option<Exprent> {
    if let Some(var) = exprent.try_as_var() {
        let i = slots.iter().position(|&s| s == var.get_index())?;
        return Some(args[i].copy());
    }
    for child in exprent.get_all_exprents() {
        let replaced = substitute(child, slots, args)?;
        if replaced != child {
            exprent.replace_exprent(child, replaced);
        }
    }
    Some(exprent)
}
