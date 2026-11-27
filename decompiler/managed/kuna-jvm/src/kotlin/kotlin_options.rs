//! Port of `org/vineflower/kotlin/KotlinOptions.java`. The annotated option
//! fields Vineflower reads by reflection are the table `OPTIONS`.

use crate::i_fernflower_preferences::{OptionType, PreferenceOption};

pub struct KotlinOptions;

impl KotlinOptions {
    pub const DECOMPILE_KOTLIN: &'static str = "kt-enable";
    pub const UNKNOWN_DEFAULT_ARG_STRING: &'static str = "kt-unknown-defaults";

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
    ];

    /// What `Fernflower::new` puts for each key a caller did not set (`addDefaults`).
    pub const DEFAULTS: [(&'static str, &'static str); 2] = [
        (KotlinOptions::DECOMPILE_KOTLIN, "1"),
        (KotlinOptions::UNKNOWN_DEFAULT_ARG_STRING, "..."),
    ];
}
