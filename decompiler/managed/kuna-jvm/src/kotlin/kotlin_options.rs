//! Port of `org/vineflower/kotlin/KotlinOptions.java`: the Kotlin plugin's keys
//! and their defaults.

pub struct KotlinOptions;

impl KotlinOptions {
    pub const DECOMPILE_KOTLIN: &'static str = "kt-enable";
    pub const UNKNOWN_DEFAULT_ARG_STRING: &'static str = "kt-unknown-defaults";

    /// What `Fernflower::new` puts for each key a caller did not set (`addDefaults`).
    pub const DEFAULTS: [(&'static str, &'static str); 2] = [
        (KotlinOptions::DECOMPILE_KOTLIN, "1"),
        (KotlinOptions::UNKNOWN_DEFAULT_ARG_STRING, "..."),
    ];
}
