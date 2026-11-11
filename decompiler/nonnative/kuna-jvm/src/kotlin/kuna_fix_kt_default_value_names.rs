//! kuna fix `kt_default_value_names` (`kuna_fixes`): the variables of a
//! function's default value are written with the names its declaration
//! gives them.
//!
//! A default is an expression of the function's default-arguments method,
//! which reads the function's parameters at the function's own slots (a
//! member's instance first). Vineflower names them from the function's
//! local variable table (`DefaultArgsMap.updateExprent`): an extension
//! receiver by its debug name `` `$this$f` ``, a member's instance `this`
//! also in an extension, where Kotlin's `this` is the extension receiver
//! (`this.length` then reads the receiver's member), and a parameter of a
//! function without code (an interface's abstract function) not at all
//! (`var1`). With the fix, while a function's defaults are written, a value
//! parameter is written by its name in the metadata, the extension receiver
//! `this`, and a member's instance `this`, or `this@Class` in an extension;
//! an extension in a class without a name keeps Vineflower's names, as its
//! body does (`kt_extension_receiver`).

use std::cell::RefCell;

use crate::code_constants::ACC_STATIC;
use crate::k_parameter::KParameter;
use crate::kotlin::kuna_fix_kt_extension_receiver;
use crate::kotlin_writer::KotlinWriter;
use crate::method_wrapper::MethodWrapper;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;

struct Names {
    varproc: VarProcessor,
    slots: Vec<(i32, String)>,
}

thread_local! {
    static NAMES: RefCell<Vec<Names>> = const { RefCell::new(Vec::new()) };
}

/// While alive, the defaults of a function are being written.
pub struct NamesScope;

impl Drop for NamesScope {
    fn drop(&mut self) {
        NAMES.with(|n| {
            n.borrow_mut().pop();
        });
    }
}

/// Enters the defaults of `method` of `cl` (its value parameters
/// `parameters`, an extension when `extension`), read from `bridge`.
pub fn enter(
    bridge: MethodWrapper,
    cl: StructClass,
    method: StructMethod,
    parameters: &[KParameter],
    extension: bool,
    suspend: bool,
) -> Option<NamesScope> {
    let is_static = method.has_modifier(ACC_STATIC);
    let descriptor = method.method_descriptor();
    let jvm = &descriptor.params;
    let value_end = jvm.len().checked_sub(usize::from(suspend))?;
    let value_start = value_end.checked_sub(parameters.len())?;
    let receiver = if extension {
        Some(value_start.checked_sub(1)?)
    } else {
        None
    };

    let mut slots: Vec<(i32, String)> = Vec::new();
    if !is_static {
        let instance = if extension {
            format!("this@{}", kuna_fix_kt_extension_receiver::class_label(cl)?)
        } else {
            "this".to_string()
        };
        slots.push((0, instance));
    }
    let mut slot = i32::from(!is_static);
    for (i, param) in jvm.iter().enumerate() {
        if Some(i) == receiver {
            slots.push((slot, "this".to_string()));
        } else if (value_start..value_end).contains(&i) {
            let name = &parameters[i - value_start].name;
            slots.push((slot, KotlinWriter::to_valid_kotlin_identifier(name)));
        }
        slot += param.stack_size;
    }

    let varproc = bridge.with(|d| d.varproc);
    NAMES.with(|n| n.borrow_mut().push(Names { varproc, slots }));
    Some(NamesScope)
}

/// The name `var` is written with in the defaults being written.
pub fn name(var: VarExprent) -> Option<String> {
    let processor = var.get_processor()?;
    NAMES.with(|n| {
        let n = n.borrow();
        let names = n.last()?;
        if names.varproc != processor {
            return None;
        }
        let index = var.get_index();
        let slot = processor.get_var_original_index(index).unwrap_or(index);
        names
            .slots
            .iter()
            .find(|(s, _)| *s == slot)
            .map(|(_, name)| name.clone())
    })
}
