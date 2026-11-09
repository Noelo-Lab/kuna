//! kuna fix `kt_object_super_call`: an `object` expression's superclass is
//! written with its constructor call's arguments.
//!
//! `KotlinWriter.writeClassDefinition` writes an anonymous class's header from
//! its Class metadata's supertype list, which names types only: `object :
//! Base(k + 1)` prints as `object : Base`, which drops the arguments (and
//! `object : Plain()` as `object : Plain`, which does not compile). kotlinc
//! evaluates an object's superclass arguments where the object is created and
//! passes them to the object's constructor, whose first statement passes them
//! (with any constants it adds) to the superclass constructor; captured
//! variables arrive the same way. So the arguments are the object
//! constructor's superclass call (which `NestedClassProcessor` takes out of
//! the constructor and keeps as the node's `superInvocation`) with each of
//! its parameters replaced by what the `new` passes for it, written where the
//! `new` stands. `KNewExprent`
//! computes that text before it writes the class ([`write_at_site`]) and the
//! header appends it to the superclass ([`append_arguments`]). When the call
//! is not there, `InitializerProcessor.hideEmptySuper` removed it because it
//! passes nothing, and the arguments are `()`. A call through the
//! superclass's default-arguments constructor is written as the call it
//! stands for when `kt_default_args_super_call` can resugar it. A call whose
//! arguments read anything but the constructor's parameters and constants,
//! or that cannot be resugared, is left out: the object is then written as in
//! Vineflower.

use std::cell::RefCell;

use crate::classes_processor::ClassNode;
use crate::code_constants::INIT_NAME;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_type::KType;
use crate::kotlin::kuna_fix_kt_default_args_super_call;
use crate::kt_attrs::KtAttrs;
use crate::kuna_fixes::{self, KunaFix};
use crate::method_descriptor::MethodDescriptor;
use crate::statements::Statements;
use crate::text_buffer::TextBuffer;

const DEFAULT_CONSTRUCTOR_MARKER: &str = "Lkotlin/jvm/internal/DefaultConstructorMarker;";

thread_local! {
    static ARGUMENTS: RefCell<Option<(ClassNode, TextBuffer)>> = const { RefCell::new(None) };
}

/// Puts the enclosing `new`'s arguments back when dropped.
struct Restore(Option<(ClassNode, TextBuffer)>);

impl Drop for Restore {
    fn drop(&mut self) {
        let outer = self.0.take();
        ARGUMENTS.with(|a| *a.borrow_mut() = outer);
    }
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
    let _restore = Restore(ARGUMENTS.with(|a| a.replace(arguments)));
    write()
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
    let mut slots = Vec::with_capacity(params.len());
    let mut slot = 1;
    for param in params.iter() {
        slots.push(slot);
        slot += param.stack_size;
    }

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
        let hidden = node.with(|d| d.super_invocation.is_none())
            && DecompilerContext::get_option(IFernflowerPreferences::HIDE_EMPTY_SUPER);
        return hidden.then(TextBuffer::new);
    };

    let call = call.0.copy().as_invocation();
    for param in call.get_lst_parameters().to_vec() {
        let replaced = substitute(param, &slots, &args)?;
        if replaced != param {
            call.0.replace_exprent(param, replaced);
        }
    }
    if call
        .get_string_descriptor()
        .contains(DEFAULT_CONSTRUCTOR_MARKER)
        && !(kuna_fixes::enabled(KunaFix::KtDefaultArgsSuperCall)
            && kuna_fix_kt_default_args_super_call::resugar(call))
    {
        return None;
    }
    Some(call.append_param_list(indent + 1))
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
