//! Fix `kt_function_parameter_name`: while a function's or constructor's body is
//! written, a parameter without a local variable table entry is named as the
//! header names it.

use std::cell::RefCell;

use crate::code_constants::ACC_STATIC;
use crate::k_parameter::KParameter;
use crate::kotlin_writer::KotlinWriter;
use crate::method_wrapper::MethodWrapper;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;

/// A body's variable processor and its parameters' slots and header names.
type Names = (VarProcessor, Vec<(i32, String)>);

thread_local! {
    static NAMES: RefCell<Vec<Names>> = const { RefCell::new(Vec::new()) };
}

/// While alive, the body whose parameters are named by its header.
pub struct BodyScope;

impl Drop for BodyScope {
    fn drop(&mut self) {
        NAMES.with(|n| {
            n.borrow_mut().pop();
        });
    }
}

/// Enters the body of `method_wrapper`, whose header writes `parameters`
/// after `leading` other JVM parameters (an extension receiver, an inner
/// class's outer instance) and before `trailing` ones (a suspend function's
/// continuation).
pub fn enter(
    method_wrapper: MethodWrapper,
    parameters: &[KParameter],
    leading: usize,
    trailing: usize,
) -> BodyScope {
    let (varproc, method) = method_wrapper.with(|d| (d.varproc, d.method_struct));
    let jvm_params = method.method_descriptor().params.clone();
    let mut names = Vec::new();
    if jvm_params.len() == leading + parameters.len() + trailing {
        let used = varproc.get_var_names();
        let mut slot: i32 = if method.has_modifier(ACC_STATIC) { 0 } else { 1 };
        for param in &jvm_params[..leading] {
            slot += param.stack_size;
        }
        for (parameter, param) in parameters.iter().zip(&jvm_params[leading..]) {
            let pair = VarVersionPair::new(slot, 0);
            let name = &parameter.name;
            let current = varproc.get_var_name(pair);
            if current.as_deref() != Some(name.as_str())
                && current.is_some()
                && varproc.get_var_lvt(pair).is_none()
                && !name.is_empty()
                && !name.starts_with('<')
                && !used.iter().any(|u| u == name)
            {
                names.push((slot, KotlinWriter::to_valid_kotlin_identifier(name)));
            }
            slot += param.stack_size;
        }
    }
    NAMES.with(|n| n.borrow_mut().push((varproc, names)));
    BodyScope
}

/// The name `var` is written with in the body being written.
pub fn name(var: VarExprent) -> Option<String> {
    NAMES.with(|n| {
        let n = n.borrow();
        let (varproc, names) = n.last()?;
        if var.get_processor()? != *varproc || var.get_version() != 0 {
            return None;
        }
        let index = var.get_index();
        names
            .iter()
            .find(|(slot, _)| *slot == index)
            .map(|(_, name)| name.clone())
    })
}
