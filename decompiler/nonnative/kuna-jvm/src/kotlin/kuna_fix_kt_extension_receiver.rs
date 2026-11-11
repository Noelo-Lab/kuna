//! kuna fix `kt_extension_receiver`: in an extension function's body the
//! receiver is written `this`.
//!
//! kotlinc passes an extension function's receiver as its first parameter
//! and names it `$this$<function>` in the local variable table. The plugin
//! writes the function's header in Kotlin (`fun Buffer.commonWrite(..)`) but
//! its body uses the parameter's debug name, `` `$this$commonWrite` ``,
//! which names nothing in Kotlin. With the fix the receiver parameter is
//! written `this` while the function's body is written (not in the lambdas
//! and objects written inside it, which have their own variables). In an
//! extension declared inside a class, plain `this` is the extension
//! receiver, so the class's own instance is written `this@Class`; such a
//! function in a class without a name keeps Vineflower's text, and so does
//! one whose code stores to a receiver's slot (a suspend function's state
//! machine reloads its receivers there; `this` cannot be assigned).

use std::cell::RefCell;

use crate::code_constants::{OPC_ASTORE, OPC_ASTORE_3, OPC_IINC, OPC_ISTORE, OPC_ISTORE_0};
use crate::method_wrapper::MethodWrapper;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;

#[derive(Clone)]
struct Receiver {
    varproc: VarProcessor,
    slot: i32,
    dispatch: Option<String>,
}

thread_local! {
    static RECEIVERS: RefCell<Vec<Receiver>> = const { RefCell::new(Vec::new()) };
}

/// While alive, the extension function whose body is being written.
pub struct ReceiverScope;

impl Drop for ReceiverScope {
    fn drop(&mut self) {
        RECEIVERS.with(|r| {
            r.borrow_mut().pop();
        });
    }
}

/// Enters the body of the extension function `method` of `cl`; `None` when
/// its class's own instance could not be named, or when the code stores to a
/// receiver's slot (a suspend function's state machine reloads it there).
pub fn enter(cl: StructClass, method: MethodWrapper, is_static: bool) -> Option<ReceiverScope> {
    let dispatch = if is_static {
        None
    } else {
        Some(format!("this@{}", class_label(cl)?))
    };
    let slot = if is_static { 0 } else { 1 };
    if stores_to(cl, method.with(|d| d.method_struct), slot) {
        return None;
    }
    let varproc = method.with(|d| d.varproc);
    RECEIVERS.with(|r| {
        r.borrow_mut().push(Receiver {
            varproc,
            slot,
            dispatch,
        })
    });
    Some(ReceiverScope)
}

/// The name `var` is written with in the extension function being written.
pub fn name(var: VarExprent) -> Option<String> {
    let receiver = RECEIVERS.with(|r| r.borrow().last().cloned())?;
    let processor = var.get_processor()?;
    if processor != receiver.varproc {
        return None;
    }
    let index = var.get_index();
    let slot = processor.get_var_original_index(index).unwrap_or(index);
    if slot == receiver.slot {
        Some("this".to_string())
    } else if slot == 0 {
        receiver.dispatch
    } else {
        None
    }
}

/// The label of `cl`'s `this`: its simple name, when it has one.
pub(crate) fn class_label(cl: StructClass) -> Option<String> {
    let name = cl.qualified_name();
    let simple = name.rsplit('/').next()?.rsplit('$').next()?.to_string();
    let first = simple.chars().next()?;
    (first.is_alphabetic() || first == '_')
        .then(|| crate::kotlin_writer::KotlinWriter::to_valid_kotlin_identifier(&simple))
}

/// Whether `mt`'s code stores to a local slot up to `last` (true when it cannot be read).
fn stores_to(cl: StructClass, mt: StructMethod, last: i32) -> bool {
    if !mt.contains_code() {
        return false;
    }
    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return true;
    }
    let found = mt.get_instruction_sequence().is_none_or(|seq| {
        (0..seq.length()).any(|i| {
            let instr = seq.get_instr(i);
            let op = instr.opcode();
            let slot = if (OPC_ISTORE..=OPC_ASTORE).contains(&op) || op == OPC_IINC {
                instr.operand(0)
            } else if (OPC_ISTORE_0..=OPC_ASTORE_3).contains(&op) {
                (op - OPC_ISTORE_0) % 4
            } else {
                return false;
            };
            slot <= last
        })
    });
    if !expanded {
        mt.release_resources();
    }
    found
}
