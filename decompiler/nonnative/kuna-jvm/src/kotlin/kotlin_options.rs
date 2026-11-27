//! Port of `org/vineflower/kotlin/KotlinOptions.java`. The annotated option
//! fields Vineflower reads by reflection are the table `OPTIONS`.

use crate::i_fernflower_preferences::{OptionType, PreferenceOption};
use crate::plugin_options::AddDefaults;

pub struct KotlinOptions;

impl KotlinOptions {
    pub const DECOMPILE_KOTLIN: &'static str = "kt-enable";
    pub const UNKNOWN_DEFAULT_ARG_STRING: &'static str = "kt-unknown-defaults";
    pub const DECOMPILE_FACADES: &'static str = "kt-decompile-facades";

    /// The option fields, in declaration order.
    pub const OPTIONS: &'static [PreferenceOption] = &[
        PreferenceOption {
            field: "DECOMPILE_KOTLIN",
            key: KotlinOptions::DECOMPILE_KOTLIN,
            short_name: None,
            name: Some("Enable Kotlin plugin"),
            description: Some("Decompile Kotlin classes as Kotlin instead of Java"),
            type_: Some(OptionType::Boolean),
            dynamic_default_value: None,
        },
        PreferenceOption {
            field: "UNKNOWN_DEFAULT_ARG_STRING",
            key: KotlinOptions::UNKNOWN_DEFAULT_ARG_STRING,
            short_name: None,
            name: Some("Unknown default arg string"),
            description: Some(
                "String to use for unknown default arguments, or empty to not indicate unknown defaults",
            ),
            type_: Some(OptionType::String),
            dynamic_default_value: None,
        },
        PreferenceOption {
            field: "DECOMPILE_FACADES",
            key: KotlinOptions::DECOMPILE_FACADES,
            short_name: None,
            name: Some("Decompile facades"),
            description: Some(
                "Decompile multi-file facades into their wrapper-function classes.",
            ),
            type_: Some(OptionType::Boolean),
            dynamic_default_value: None,
        },
    ];

    pub fn add_defaults(cons: &mut dyn AddDefaults) {
        cons.add_default(KotlinOptions::DECOMPILE_KOTLIN, "1");
        cons.add_default(KotlinOptions::UNKNOWN_DEFAULT_ARG_STRING, "...");
        cons.add_default(KotlinOptions::DECOMPILE_FACADES, "0");
    }
}
