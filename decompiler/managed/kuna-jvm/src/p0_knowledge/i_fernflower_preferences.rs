//! Port of `org/jetbrains/java/decompiler/main/extern/IFernflowerPreferences.java`.
//!
//! The option keys the engine reads, as `&'static str` constants on
//! [`IFernflowerPreferences`], and their defaults.

use crate::rt::JHashMap;

pub struct IFernflowerPreferences;

impl IFernflowerPreferences {
    pub const REMOVE_BRIDGE: &str = "remove-bridge";
    pub const REMOVE_SYNTHETIC: &str = "remove-synthetic";
    pub const DECOMPILE_INNER: &str = "decompile-inner";
    pub const DECOMPILE_GENERIC_SIGNATURES: &str = "decompile-generics";
    pub const LITERALS_AS_IS: &str = "keep-literals";
    pub const ASCII_STRING_CHARACTERS: &str = "ascii-strings";
    pub const SYNTHETIC_NOT_SET: &str = "synthetic-not-set";
    pub const USE_DEBUG_VAR_NAMES: &str = "use-lvt-names";
    pub const FINALLY_DEINLINE: &str = "decompile-finally";
    pub const IGNORE_INVALID_BYTECODE: &str = "ignore-invalid-bytecode";
    pub const VERIFY_ANONYMOUS_CLASSES: &str = "verify-anonymous-classes";
    pub const PATTERN_MATCHING: &str = "pattern-matching";
    pub const SWITCH_EXPRESSIONS: &str = "decompile-switch-expressions";
    pub const VERIFY_VARIABLE_MERGES: &str = "verify-merges";
    pub const VERIFY_PRE_POST_VARIABLE_MERGES: &str = "verify-pre-post-merges";
    pub const OLD_TRY_DEDUP: &str = "old-try-dedup";
    pub const INCLUDE_JAVA_RUNTIME: &str = "include-runtime";
    pub const MAX_PROCESSING_METHOD: &str = "max-time-per-method";
    pub const RENAME_ENTITIES: &str = "rename-members";
    pub const INDENT_STRING: &str = "indent-string";
    pub const PREFERRED_LINE_LENGTH: &str = "preferred-line-length";
    pub const THREADS: &str = "thread-count";
    pub const SKIP_EXTRA_FILES: &str = "skip-extra-files";
    pub const DUMP_BYTECODE_ON_ERROR: &str = "dump-bytecode-on-error";
    pub const DECOMPILER_COMMENTS: &str = "decompiler-comments";
    pub const FORCE_JSR_INLINE: &str = "force-jsr-inline";
    pub const MARK_CORRESPONDING_SYNTHETICS: &str = "mark-corresponding-synthetics";
    pub const METHOD_TO_DECOMPILE: &str = "method-to-decompile";

    /// `IFernflowerPreferences.DEFAULTS`, computed once per thread.
    #[allow(non_snake_case)]
    pub fn DEFAULTS() -> JHashMap<String, String> {
        thread_local! {
            static DEFAULTS: JHashMap<String, String> = IFernflowerPreferences::get_defaults();
        }
        DEFAULTS.with(|d| d.clone())
    }

    /// A fresh `HashMap` filled in the Java's insertion order, so it iterates
    /// in the JDK's order for the same puts.
    pub fn get_defaults() -> JHashMap<String, String> {
        let mut defaults = JHashMap::new();
        for (key, value) in Self::default_entries() {
            defaults.put(key.to_string(), value);
        }
        defaults
    }

    /// The `defaults.put` calls of `getDefaults()`, in source order.
    pub fn default_entries() -> Vec<(&'static str, String)> {
        vec![
            (IFernflowerPreferences::REMOVE_BRIDGE, "1".to_string()),
            (IFernflowerPreferences::REMOVE_SYNTHETIC, "1".to_string()),
            (IFernflowerPreferences::DECOMPILE_INNER, "1".to_string()),
            (IFernflowerPreferences::DECOMPILE_GENERIC_SIGNATURES, "1".to_string()),
            (IFernflowerPreferences::LITERALS_AS_IS, "0".to_string()),
            (IFernflowerPreferences::ASCII_STRING_CHARACTERS, "0".to_string()),
            (IFernflowerPreferences::SYNTHETIC_NOT_SET, "0".to_string()),
            (IFernflowerPreferences::USE_DEBUG_VAR_NAMES, "1".to_string()),
            (IFernflowerPreferences::FINALLY_DEINLINE, "1".to_string()),
            (IFernflowerPreferences::IGNORE_INVALID_BYTECODE, "0".to_string()),
            (IFernflowerPreferences::VERIFY_ANONYMOUS_CLASSES, "0".to_string()),
            (IFernflowerPreferences::PATTERN_MATCHING, "1".to_string()),
            (IFernflowerPreferences::SWITCH_EXPRESSIONS, "1".to_string()),
            (IFernflowerPreferences::VERIFY_VARIABLE_MERGES, "0".to_string()),
            (IFernflowerPreferences::VERIFY_PRE_POST_VARIABLE_MERGES, "0".to_string()),
            (IFernflowerPreferences::OLD_TRY_DEDUP, "0".to_string()),
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "1".to_string()),
            (IFernflowerPreferences::MAX_PROCESSING_METHOD, "0".to_string()),
            (IFernflowerPreferences::RENAME_ENTITIES, "0".to_string()),
            (IFernflowerPreferences::INDENT_STRING, "   ".to_string()),
            (IFernflowerPreferences::PREFERRED_LINE_LENGTH, "160".to_string()),
            (IFernflowerPreferences::THREADS, available_processors().to_string()),
            (IFernflowerPreferences::SKIP_EXTRA_FILES, "0".to_string()),
            (IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR, "1".to_string()),
            (IFernflowerPreferences::DECOMPILER_COMMENTS, "1".to_string()),
            (IFernflowerPreferences::FORCE_JSR_INLINE, "0".to_string()),
            (IFernflowerPreferences::MARK_CORRESPONDING_SYNTHETICS, "0".to_string()),
            (IFernflowerPreferences::METHOD_TO_DECOMPILE, "".to_string()),
        ]
    }
}

fn available_processors() -> i32 {
    std::thread::available_parallelism().map_or(1, |n| n.get() as i32)
}
