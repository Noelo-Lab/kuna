//! Port of `org/vineflower/kotlin/KotlinOptions.java`. The annotated option
//! fields Vineflower reads by reflection are the table `OPTIONS`.

use crate::i_fernflower_preferences::{OptionType, PreferenceOption};
use crate::plugin_options::AddDefaults;

pub struct KotlinOptions;

impl KotlinOptions {
    pub const SHOW_PUBLIC_VISIBILITY: &'static str = "kt-show-public";
    pub const DECOMPILE_KOTLIN: &'static str = "kt-enable";
    pub const UNKNOWN_DEFAULT_ARG_STRING: &'static str = "kt-unknown-defaults";
    pub const COLLAPSE_STRING_CONCATENATION: &'static str = "kt-collapse-string-concat";
    pub const ALWAYS_EXPORT_METADATA: &'static str = "kt-export-metadata";
    pub const DECOMPILE_FACADES: &'static str = "kt-decompile-facades";

    /// The option fields, in declaration order.
    pub const OPTIONS: &'static [PreferenceOption] = &[
        PreferenceOption {
            field: "SHOW_PUBLIC_VISIBILITY",
            key: KotlinOptions::SHOW_PUBLIC_VISIBILITY,
            short_name: None,
            name: Some("Show public visibility"),
            description: Some("If a construct is public, show the public keyword"),
            type_: Some(OptionType::Boolean),
            dynamic_default_value: None,
        },
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
            field: "COLLAPSE_STRING_CONCATENATION",
            key: KotlinOptions::COLLAPSE_STRING_CONCATENATION,
            short_name: None,
            name: Some("Collapse string concatenation"),
            description: Some("Convert string concatenations to Kotlin string templates."),
            type_: Some(OptionType::Boolean),
            dynamic_default_value: None,
        },
        PreferenceOption {
            field: "ALWAYS_EXPORT_METADATA",
            key: KotlinOptions::ALWAYS_EXPORT_METADATA,
            short_name: None,
            name: Some("Always export metadata"),
            description: Some(
                "If Kotlin decompilation is disabled, metadata will not be parsed. If enabled, this will always parse Kotlin metadata for use by other plugins.",
            ),
            type_: Some(OptionType::Boolean),
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
        cons.add_default(KotlinOptions::SHOW_PUBLIC_VISIBILITY, "1");
        cons.add_default(KotlinOptions::DECOMPILE_KOTLIN, "1");
        cons.add_default(KotlinOptions::UNKNOWN_DEFAULT_ARG_STRING, "...");
        cons.add_default(KotlinOptions::COLLAPSE_STRING_CONCATENATION, "1");
        cons.add_default(KotlinOptions::ALWAYS_EXPORT_METADATA, "0");
        cons.add_default(KotlinOptions::DECOMPILE_FACADES, "0");
    }
}
