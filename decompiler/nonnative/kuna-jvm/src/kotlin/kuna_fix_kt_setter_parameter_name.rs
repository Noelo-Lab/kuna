//! kuna fix `kt_setter_parameter_name`: a setter's body names its parameter
//! as the setter's header does.
//!
//! `KProperty` writes a setter's header with the parameter name the metadata
//! records (`set(value)`), but its body with the local variable table's
//! name, which for a setter kotlinc generates (`@JvmName(..) set`,
//! `@Synchronized set`) is `<set-?>`, no identifier, so the body says
//! `var1`: `set(value) { field = var1 }` assigns an unknown name. With the
//! fix the parameter (the setter's last, after an extension property's
//! receiver) is written in the body by the header's name, and a metadata
//! name that is no Kotlin identifier (`<set-?>`) is written `value` in both.

use std::cell::RefCell;

use crate::code_constants::ACC_STATIC;
use crate::kotlin_writer::KotlinWriter;
use crate::method_wrapper::MethodWrapper;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;

thread_local! {
    static SETTERS: RefCell<Vec<(VarProcessor, i32, String)>> = const { RefCell::new(Vec::new()) };
}

/// While alive, the setter whose body is being written.
pub struct SetterScope;

impl Drop for SetterScope {
    fn drop(&mut self) {
        SETTERS.with(|s| {
            s.borrow_mut().pop();
        });
    }
}

/// The name the setter's parameter is written with, from the metadata's `name`.
pub fn parameter_name(name: Option<&str>) -> String {
    match name {
        Some(name)
            if !name.is_empty()
                && !name.starts_with('<')
                && KotlinWriter::to_valid_kotlin_identifier(name) == name =>
        {
            name.to_string()
        }
        _ => "value".to_string(),
    }
}

/// Enters the body of the setter `setter`, whose parameter (the last one,
/// after an extension property's receiver) is written `name`.
pub fn enter(setter: MethodWrapper, name: String) -> SetterScope {
    let (varproc, method) = setter.with(|d| (d.varproc, d.method_struct));
    let params = method.method_descriptor().params.clone();
    let before: i32 = params
        .iter()
        .take(params.len().saturating_sub(1))
        .map(|p| p.stack_size)
        .sum();
    let slot = before
        + if method.has_modifier(ACC_STATIC) {
            0
        } else {
            1
        };
    SETTERS.with(|s| s.borrow_mut().push((varproc, slot, name)));
    SetterScope
}

/// The name `var` is written with in the setter being written.
pub fn name(var: VarExprent) -> Option<String> {
    let (varproc, slot, name) = SETTERS.with(|s| s.borrow().last().cloned())?;
    let processor = var.get_processor()?;
    if processor != varproc {
        return None;
    }
    let index = var.get_index();
    (processor.get_var_original_index(index).unwrap_or(index) == slot).then_some(name)
}
