//! Fix family `nested_class_vars`: captured values and variable names in
//! nested classes and lambdas. In this file, `lambda_catch_var_rename` gives
//! the catch variables of a lambda body the renamed table entry its other uses
//! get; `nested_param_rename` declares a nested class method's parameter by the
//! table name its uses are written by; `refresh_printed_var_names` makes
//! `refreshVarNames` take the clashing names first and rename only the
//! variables without one.

pub(crate) mod captured_var_clash_name;
pub(crate) mod captured_var_through_cast;

use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::rt::JHashMap;
use crate::statement::Statement;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::var_exprent::VarExprent;
use crate::var_names_collector::VarNamesCollector;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;

/// Applies the lambda body's renamed entries to the catch variables under
/// `stat`, as the rename applies them to the variables of the body's graph.
pub fn rename_catch_vars(
    stat: Statement,
    lvts: &JHashMap<VarVersionPair, Option<LocalVariable>>,
    renamed: &dyn Fn(VarVersionPair) -> bool,
) {
    let vars: Vec<VarExprent> = if let Some(catch) = stat.try_as_catch() {
        catch.get_vars().to_vec()
    } else if let Some(catch_all) = stat.try_as_catch_all() {
        catch_all.get_vars().to_vec()
    } else {
        Vec::new()
    };
    for var in vars {
        let pair = var.get_var_version_pair();
        if let Some(lv) = lvts.get(&pair).cloned().flatten() {
            var.set_lvt(Some(lv));
        } else if renamed(pair) {
            var.set_lvt(None);
        }
    }
    for child in stat.get_stats().borrow().to_vec() {
        rename_catch_vars(child, lvts, renamed);
    }
}

/// The name the uses of parameter `pair` are written by when it is a
/// local-variable-table name other than `name`.
pub fn table_name(varproc: VarProcessor, pair: VarVersionPair, name: Option<&str>) -> Option<String> {
    if !DecompilerContext::get_option(IFernflowerPreferences::USE_DEBUG_VAR_NAMES) {
        return None;
    }
    let lvt = varproc.get_var_lvt(pair)?.get_name();
    (Some(lvt.as_str()) != name && VarExprent::is_valid_name(&lvt)).then_some(lvt)
}

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
