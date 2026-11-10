//! kuna fix `kt_synthetic_object_super_call`: an anonymous class with no
//! Class metadata whose superclass constructor takes arguments is written as
//! an `object` expression that passes them.
//!
//! A suspend function that suspends compiles to a state machine whose
//! continuation is an anonymous subclass of `ContinuationImpl`, created on
//! the first call by `new f$1(this, $completion)`: its constructor stores the
//! captured `this` and passes `$completion` to the superclass constructor.
//! The class has only synthetic-class metadata (`k = 3`), so
//! `KotlinWriter.writeClassDefinition` throws `Anonymous class does not have
//! Class Kotlin metadata` and the whole function fails; the fix
//! `kt_synthetic_anonymous_object` writes only objects whose superclass is
//! `Object`. With this fix the class is written where the `new` stands, as
//! `object : ContinuationImpl($completion) { .. }`: the supertypes from the
//! class file as that fix writes them, the superclass arguments as the fix
//! `kt_object_super_call` computes them for an object with Class metadata
//! (the superclass call `NestedClassProcessor` keeps for the node, each
//! constructor parameter replaced by what the `new` passes for it), and the
//! fields and methods as that fix writes its objects'. That is exactly what
//! the bytecode does only when the class has one constructor, which does
//! nothing but store its captured variables and call the superclass
//! constructor with its parameters and constants, each captured variable is
//! passed the variable of the name the object's body gives it, and every
//! other argument is a variable or a constant (evaluating it where the
//! superclass call stands, once, more often or not at all, means the same);
//! any other such class still fails. The function around the `new` is
//! printed as compiled, as Vineflower prints every suspend function: the
//! state machine, with the continuation's type, which Kotlin cannot name,
//! written `<unrepresentable>`.

use std::cell::RefCell;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::INIT_NAME;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::kt_attrs::KtAttrs;
use crate::kuna_fix_kt_synthetic_anonymous_object;
use crate::method_descriptor::MethodDescriptor;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;

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
/// with `constructor`) with the superclass arguments of this `new` at hand
/// when the object can be written here ([`is_writable`]).
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

/// Whether the `new` being written creates `node` as an object expression
/// whose superclass takes the arguments [`write_at_site`] found.
pub fn is_writable(node: ClassNode) -> bool {
    ARGUMENTS.with(|a| a.borrow().as_ref().is_some_and(|(n, _)| *n == node))
}

/// Appends `object : Super(arguments)` and the class's interfaces.
pub fn write_object_header(node: ClassNode, buffer: &mut TextBuffer) {
    let arguments = ARGUMENTS.with(|a| {
        a.borrow()
            .as_ref()
            .filter(|(n, _)| *n == node)
            .map(|(_, text)| text.clone())
    });
    kuna_fix_kt_synthetic_anonymous_object::write_object_header_with(
        node,
        arguments.as_ref(),
        buffer,
    );
}

/// The text of `node`'s superclass arguments at a `new` calling
/// `constructor`, or `None` when the object cannot be written there.
fn super_arguments(
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

    let mut slots = Vec::with_capacity(params.len());
    let mut slot = 1;
    for param in params.iter() {
        slots.push(slot);
        slot += param.stack_size;
    }
    let readable = mask.iter().zip(&args).all(|(pair, arg)| match pair {
        Some(pair) => {
            kuna_fix_kt_synthetic_anonymous_object::capture_reads_at_site(node, *pair, *arg)
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
    let mut rest = Vec::new();
    collect_exprents(root.0, &mut rest);
    if !rest.is_empty() {
        return None;
    }

    let call = call.0.copy().as_invocation();
    for param in call.get_lst_parameters().to_vec() {
        let replaced = substitute(param, &slots, &args)?;
        if replaced != param {
            call.0.replace_exprent(param, replaced);
        }
    }
    Some(call.append_param_list(indent + 1))
}

/// Every exprent of `stat`'s tree, nested ones included.
fn collect_exprents(stat: Statement, out: &mut Vec<Exprent>) {
    let top: Vec<Exprent> = match stat.get_exprents() {
        Some(exprents) => exprents.iter().collect(),
        None => {
            for st in stat.get_stats().borrow().iter() {
                collect_exprents(*st, out);
            }
            stat.get_stat_exprents()
        }
    };
    for e in top {
        out.extend(e.get_all_exprents_rec(true));
        out.push(e);
    }
}

/// `exprent` with every read of a constructor parameter replaced by a copy
/// of what the `new` passes for it; `None` when it reads another variable.
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
