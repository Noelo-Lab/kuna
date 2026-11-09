//! Port of `org/vineflower/variablerenaming/VariableRenamingOptions.java`. The
//! annotated option fields Vineflower reads by reflection are the table
//! `OPTIONS`.

use crate::i_fernflower_preferences::{OptionType, PreferenceOption};
use crate::plugin_options::AddDefaults;

pub struct VariableRenamingOptions;

impl VariableRenamingOptions {
    pub const VARIABLE_RENAMER: &'static str = "variable-renaming";
    pub const RENAME_PARAMETERS: &'static str = "rename-parameters";
    pub const USE_JAD_VARNAMING: &'static str = "jad-style-variable-naming";
    pub const USE_JAD_PARAMETER_NAMING: &'static str = "jad-style-parameter-naming";

    /// The option fields, in declaration order.
    pub const OPTIONS: &'static [PreferenceOption] = &[
        PreferenceOption {
            field: "VARIABLE_RENAMER",
            key: VariableRenamingOptions::VARIABLE_RENAMER,
            short_name: None,
            name: Some("Variable Renaming"),
            description: Some(
                "Use a custom renamer for variable names. Built-in options include \"jad\" and \"tiny\".",
            ),
            type_: Some(OptionType::String),
            dynamic_default_value: None,
        },
        PreferenceOption {
            field: "RENAME_PARAMETERS",
            key: VariableRenamingOptions::RENAME_PARAMETERS,
            short_name: None,
            name: Some("Rename Parameters"),
            description: Some("Use the custom renamer for parameters in addition to locals."),
            type_: Some(OptionType::Boolean),
            dynamic_default_value: None,
        },
        PreferenceOption {
            field: "USE_JAD_VARNAMING",
            key: VariableRenamingOptions::USE_JAD_VARNAMING,
            short_name: None,
            name: Some("[Deprecated] JAD-Style Variable Naming"),
            description: Some(
                "Use JAD-style variable naming. Deprecated, set \"variable-renamer=jad\" instead.",
            ),
            type_: Some(OptionType::Boolean),
            dynamic_default_value: None,
        },
        PreferenceOption {
            field: "USE_JAD_PARAMETER_NAMING",
            key: VariableRenamingOptions::USE_JAD_PARAMETER_NAMING,
            short_name: None,
            name: Some("[Deprecated] JAD-Style Parameter Naming"),
            description: Some(
                "Alias for \"rename-parameters\". Deprecated, use that option instead.",
            ),
            type_: Some(OptionType::Boolean),
            dynamic_default_value: None,
        },
    ];

    /// Java puts `null` for `VARIABLE_RENAMER`; with `putIfAbsent` on a map
    /// read through `getProperty` that is the same as leaving the key absent.
    pub fn add_defaults(cons: &mut dyn AddDefaults) {
        cons.add_default(VariableRenamingOptions::RENAME_PARAMETERS, "0");
        cons.add_default(VariableRenamingOptions::USE_JAD_VARNAMING, "0");
        cons.add_default(VariableRenamingOptions::USE_JAD_PARAMETER_NAMING, "0");
    }
}
