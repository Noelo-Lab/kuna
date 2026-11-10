//! kuna fix `captured_var_clash_name`: a captured variable is printed with
//! the name the creating method's variable is printed with, also after the
//! clash renaming has renamed that variable.
//!
//! `NestedClassProcessor` gives the variable that stands for a captured value
//! in an anonymous or local class's methods (`insertLocalVars`), and in a
//! lambda body (`setLambdaVars`), the name and local-variable-table entry of
//! the creating method's variable it holds, copied once. Later
//! `VarDefinitionHelper.remapClashingNames` renames a variable of the
//! creating method whose name clashes with another one in scope (`x`
//! becomes `xx`), by a clashing name that takes precedence over the variable's
//! own; it runs again when the class is written (`ClassWriter.invokeProcessors`).
//! The captured copy keeps the old name, so the nested body reads the other
//! variable of that name (kotlinx-coroutines' `catchImpl`: the object stores
//! into the `Throwable fromDownstream` instead of the `ObjectRef` it captured,
//! renamed `fromDownstreamx`), or a name that is not in scope.
//!
//! With the fix `NestedClassProcessor` records which variable each captured
//! variable holds, and when a class is about to be written (after the clash
//! renaming of the methods it is written from) each of its captured variables
//! takes the clashing name its source variable is printed with, if it has one.

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
