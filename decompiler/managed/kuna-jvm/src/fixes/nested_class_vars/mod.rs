//! Fix family `nested_class_vars`: captured values and variable names in
//! nested classes and lambdas. In this file, `lambda_catch_var_rename` gives
//! the catch variables of a lambda body the renamed table entry its other uses
//! get; `nested_param_rename` declares a nested class method's parameter by the
//! table name its uses are written by; `refresh_printed_var_names` makes
//! `refreshVarNames` take the clashing names first and rename only the
//! variables without one; `header_parameter_names` keeps a parameter the
//! method header declares by its `MethodParameters` name under that name;
//! `constant_capture_final` writes a captured local with a constant
//! initializer without `final`.

pub(crate) mod captured_var_clash_name;
pub(crate) mod captured_var_through_cast;
pub(crate) mod deduplicated_lambda_captures;

use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::var_type_processor::FinalType;
use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::JHashMap;
use crate::statement::Statement;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::struct_method::StructMethod;
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

/// The parameters of `mt` its `MethodParameters` attribute names, with the
/// name: the method header declares each by it.
pub fn header_parameter_names(mt: StructMethod) -> Vec<(VarVersionPair, String)> {
    if !DecompilerContext::get_option(IFernflowerPreferences::USE_DEBUG_VAR_NAMES) {
        return Vec::new();
    }
    let Some(attr) = mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_METHOD_PARAMETERS) else {
        return Vec::new();
    };
    let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
    let mut index = if mt.has_modifier(ACC_STATIC) { 0 } else { 1 };
    let mut named = Vec::new();
    for (param, entry) in md.params.iter().zip(attr.get_entries()) {
        if let Some(name) = entry.my_name {
            named.push((VarVersionPair::new(index, 0), name));
        }
        index += param.stack_size;
    }
    named
}

/// `refreshVarNames` over `names` (the method's variable names, in order),
/// keeping out of the renaming the variables that print a clashing name and
/// the parameters the header declares by the name they have.
pub fn refresh(
    varproc: VarProcessor,
    vc: VarNamesCollector,
    names: JHashMap<VarVersionPair, Option<String>>,
) {
    let names = names.entry_vec();
    let declared = header_parameter_names(varproc.get_method());
    let kept = |key: &VarVersionPair, value: &Option<String>| {
        varproc.get_clashing_name(*key).is_some()
            || declared.iter().any(|(pair, name)| pair == key && Some(name) == value.as_ref())
    };
    for (key, value) in &names {
        if let Some(clashing) = varproc.get_clashing_name(*key) {
            vc.add_name(clashing);
        } else if let (true, Some(name)) = (kept(key, value), value) {
            vc.add_name(name.clone());
        }
    }
    for (key, value) in names {
        if !kept(&key, &value) {
            varproc.set_var_name_opt(key, vc.get_free_name_opt(value.as_deref()));
        }
    }
}

/// The declaration `left = right` of a local a nested class captures, which
/// is marked `final`: when `right` is a constant of a primitive or `String`
/// type, `final` would make the local a constant variable, which javac
/// inlines instead of capturing. The capture in the class file proves it is
/// not one, so from Java 8 on (where effectively final suffices) it is written
/// without `final`.
pub fn unmark_constant_capture(left: Exprent, right: Exprent) {
    let Some(var) = left.try_as_var().filter(|v| v.is_definition()) else {
        return;
    };
    let Some(processor) = var.get_processor() else {
        return;
    };
    let pair = var.get_var_version_pair();
    let constant = |t: &crate::var_type::VarType| {
        t.array_dim == 0
            && (t.type_ != CodeType::Object && t.type_ != CodeType::Null
                || t.value.as_deref() == Some("java/lang/String"))
    };
    if processor.get_var_final(pair) == FinalType::ExplicitFinal
        && processor.get_method().get_bytecode_version().has_lambdas()
        && right.try_as_const().is_some_and(|c| constant(&c.get_const_type()))
        && constant(&var.get_definition_var_type())
    {
        processor.set_var_final(pair, FinalType::Final);
    }
}
