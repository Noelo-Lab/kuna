//! Fix `captured_var_clash_name`: a captured variable takes the clashing name
//! of the creating method's variable it holds.

use crate::class_wrapper::ClassWrapper;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;

/// Gives every captured variable of `wrapper`'s methods the clashing name of
/// the variable it holds.
pub fn inherit_clashing_names(wrapper: ClassWrapper) {
    let methods = wrapper.get_methods().borrow().to_vec();
    for method in methods {
        let varproc = method.with(|d| d.varproc);
        for (pair, (source, original)) in varproc.captured_sources() {
            if let Some(name) = clashing_name(source, original) {
                varproc.set_clashing_name(pair, name);
            }
        }
    }
}

/// The clashing name `varproc`'s variable `pair` is printed with: its own, or
/// for a captured variable the one of the variable it holds.
pub fn clashing_name(varproc: VarProcessor, pair: VarVersionPair) -> Option<String> {
    let mut current = (varproc, pair);
    for _ in 0..64 {
        if let Some(name) = current.0.get_clashing_name(current.1) {
            return Some(name);
        }
        current = current.0.get_captured_source(current.1)?;
    }
    None
}
