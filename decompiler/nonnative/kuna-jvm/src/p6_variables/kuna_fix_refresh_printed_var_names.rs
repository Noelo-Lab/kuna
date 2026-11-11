//! kuna fix `refresh_printed_var_names`: renaming a method's variables apart
//! from new names allocates the names the variables print.
//!
//! `VarProcessor.refreshVarNames` gives every variable of a method a name the
//! collector has not handed out yet, in turn, starting from each variable's
//! own name. `NestedClassProcessor.insertLocalVars` calls it on every method
//! of a nested class to keep its variables apart from the captured values it
//! names. By then the method's debug names are set and its clash pass has
//! given a variable that shares its name with another live one a clashing
//! name (`value` and `valuex`), which is what that variable prints. Taking
//! the own names, the refresh renames the first variable to the name the
//! second already prints: kotlinc names an inlined lambda's parameter like
//! the function's parameter it is passed (`put(value)` calling `let { value
//! -> .. }`), and a suspend function's body then reads `valuex = valuex`
//! where the bytecode copies the parameter. (The Java writer prints a
//! variable's debug name before this one, so only variables without one.)
//!
//! With the fix the clashing names are taken first, and only a variable
//! without one is renamed: a variable never gets a name another one prints.

use crate::rt::JHashMap;
use crate::var_names_collector::VarNamesCollector;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;

/// `refreshVarNames` over `names` (the method's variable names, in order),
/// keeping the variables that print a clashing name out of the renaming.
pub fn refresh(
    varproc: VarProcessor,
    vc: VarNamesCollector,
    names: JHashMap<VarVersionPair, Option<String>>,
) {
    let names = names.entry_vec();
    for (key, _) in &names {
        if let Some(clashing) = varproc.get_clashing_name(*key) {
            vc.add_name(clashing);
        }
    }
    for (key, value) in names {
        if varproc.get_clashing_name(key).is_none() {
            varproc.set_var_name_opt(key, vc.get_free_name_opt(value.as_deref()));
        }
    }
}
