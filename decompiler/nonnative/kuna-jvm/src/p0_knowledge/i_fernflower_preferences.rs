//! Port of `org/jetbrains/java/decompiler/main/extern/IFernflowerPreferences.java`.
//!
//! Every option key is a `&'static str` constant on [`IFernflowerPreferences`];
//! the field annotations (`@Name`, `@Description`, `@ShortName`, `@Type`,
//! `@DynamicDefaultValue`) are harvested into [`IFernflowerPreferences::all_options`].

use crate::i_fernflower_logger::Severity;
use crate::rt::JHashMap;

pub struct IFernflowerPreferences;

impl IFernflowerPreferences {
    pub const REMOVE_BRIDGE: &str = "remove-bridge";
    pub const REMOVE_SYNTHETIC: &str = "remove-synthetic";
    pub const DECOMPILE_INNER: &str = "decompile-inner";
    pub const DECOMPILE_CLASS_1_4: &str = "decompile-java4";
    pub const DECOMPILE_ASSERTIONS: &str = "decompile-assert";
    pub const HIDE_EMPTY_SUPER: &str = "hide-empty-super";
    pub const HIDE_DEFAULT_CONSTRUCTOR: &str = "hide-default-constructor";
    pub const DECOMPILE_GENERIC_SIGNATURES: &str = "decompile-generics";
    pub const INCORPORATE_RETURNS: &str = "incorporate-returns";
    pub const ENSURE_SYNCHRONIZED_MONITOR: &str = "ensure-synchronized-monitors";
    pub const DECOMPILE_ENUM: &str = "decompile-enums";
    pub const DECOMPILE_PREVIEW: &str = "decompile-preview";
    pub const REMOVE_GET_CLASS_NEW: &str = "remove-getclass";
    pub const LITERALS_AS_IS: &str = "keep-literals";
    pub const BOOLEAN_TRUE_ONE: &str = "boolean-as-int";
    pub const ASCII_STRING_CHARACTERS: &str = "ascii-strings";
    pub const SYNTHETIC_NOT_SET: &str = "synthetic-not-set";
    pub const UNDEFINED_PARAM_TYPE_OBJECT: &str = "undefined-as-object";
    pub const USE_DEBUG_VAR_NAMES: &str = "use-lvt-names";
    pub const USE_METHOD_PARAMETERS: &str = "use-method-parameters";
    pub const REMOVE_EMPTY_RANGES: &str = "remove-empty-try-catch";
    pub const FINALLY_DEINLINE: &str = "decompile-finally";
    pub const LAMBDA_TO_ANONYMOUS_CLASS: &str = "lambda-to-anonymous-class";
    pub const BYTECODE_SOURCE_MAPPING: &str = "bytecode-source-mapping";
    pub const DUMP_CODE_LINES: &str = "dump-code-lines";
    pub const IGNORE_INVALID_BYTECODE: &str = "ignore-invalid-bytecode";
    pub const VERIFY_ANONYMOUS_CLASSES: &str = "verify-anonymous-classes";
    pub const TERNARY_CONSTANT_SIMPLIFICATION: &str = "ternary-constant-simplification";
    pub const PATTERN_MATCHING: &str = "pattern-matching";
    pub const TRY_LOOP_FIX: &str = "try-loop-fix";
    pub const TERNARY_CONDITIONS: &str = "ternary-in-if";
    pub const SWITCH_EXPRESSIONS: &str = "decompile-switch-expressions";
    pub const SHOW_HIDDEN_STATEMENTS: &str = "show-hidden-statements";
    pub const OVERRIDE_ANNOTATION: &str = "override-annotation";
    pub const SIMPLIFY_STACK_SECOND_PASS: &str = "simplify-stack";
    pub const VERIFY_VARIABLE_MERGES: &str = "verify-merges";
    pub const VERIFY_PRE_POST_VARIABLE_MERGES: &str = "verify-pre-post-merges";
    pub const OLD_TRY_DEDUP: &str = "old-try-dedup";
    pub const INCLUDE_ENTIRE_CLASSPATH: &str = "include-classpath";
    pub const INCLUDE_JAVA_RUNTIME: &str = "include-runtime";
    pub const EXPLICIT_GENERIC_ARGUMENTS: &str = "explicit-generics";
    pub const INLINE_SIMPLE_LAMBDAS: &str = "inline-simple-lambdas";
    pub const LOG_LEVEL: &str = "log-level";
    pub const MAX_PROCESSING_METHOD: &str = "max-time-per-method";
    pub const RENAME_ENTITIES: &str = "rename-members";
    pub const USER_RENAMER_CLASS: &str = "user-renamer-class";
    pub const NEW_LINE_SEPARATOR: &str = "new-line-separator";
    pub const INDENT_STRING: &str = "indent-string";
    pub const PREFERRED_LINE_LENGTH: &str = "preferred-line-length";
    pub const BANNER: &str = "banner";
    pub const ERROR_MESSAGE: &str = "error-message";
    pub const THREADS: &str = "thread-count";
    pub const DUMP_ORIGINAL_LINES: &str = "__dump_original_lines__";
    pub const UNIT_TEST_MODE: &str = "__unit_test_mode__";
    pub const LINE_SEPARATOR_WIN: &str = "\r\n";
    pub const LINE_SEPARATOR_UNX: &str = "\n";
    pub const SKIP_EXTRA_FILES: &str = "skip-extra-files";
    pub const WARN_INCONSISTENT_INNER_CLASSES: &str = "warn-inconsistent-inner-attributes";
    pub const DUMP_BYTECODE_ON_ERROR: &str = "dump-bytecode-on-error";
    pub const DUMP_EXCEPTION_ON_ERROR: &str = "dump-exception-on-error";
    pub const DECOMPILER_COMMENTS: &str = "decompiler-comments";
    pub const SOURCE_FILE_COMMENTS: &str = "sourcefile-comments";
    pub const DECOMPILE_COMPLEX_CONDYS: &str = "decompile-complex-constant-dynamic";
    pub const FORCE_JSR_INLINE: &str = "force-jsr-inline";
    pub const DUMP_TEXT_TOKENS: &str = "dump-text-tokens";
    pub const REMOVE_IMPORTS: &str = "remove-imports";
    pub const MARK_CORRESPONDING_SYNTHETICS: &str = "mark-corresponding-synthetics";
    pub const EXCLUDED_CLASSES: &str = "excluded-classes";
    pub const INCLUDED_CLASSES: &str = "included-classes";
    pub const VALIDATE_INNER_CLASSES_NAMES: &str = "validate-inner-classes-names";
    pub const PRETTIFY_IFS: &str = "prettify-ifs";
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
            (IFernflowerPreferences::DECOMPILE_CLASS_1_4, "1".to_string()),
            (IFernflowerPreferences::DECOMPILE_ASSERTIONS, "1".to_string()),
            (IFernflowerPreferences::HIDE_EMPTY_SUPER, "1".to_string()),
            (IFernflowerPreferences::HIDE_DEFAULT_CONSTRUCTOR, "1".to_string()),
            (IFernflowerPreferences::DECOMPILE_GENERIC_SIGNATURES, "1".to_string()),
            (IFernflowerPreferences::INCORPORATE_RETURNS, "1".to_string()),
            (IFernflowerPreferences::ENSURE_SYNCHRONIZED_MONITOR, "1".to_string()),
            (IFernflowerPreferences::DECOMPILE_ENUM, "1".to_string()),
            (IFernflowerPreferences::REMOVE_GET_CLASS_NEW, "1".to_string()),
            (IFernflowerPreferences::LITERALS_AS_IS, "0".to_string()),
            (IFernflowerPreferences::BOOLEAN_TRUE_ONE, "1".to_string()),
            (IFernflowerPreferences::ASCII_STRING_CHARACTERS, "0".to_string()),
            (IFernflowerPreferences::SYNTHETIC_NOT_SET, "0".to_string()),
            (IFernflowerPreferences::UNDEFINED_PARAM_TYPE_OBJECT, "1".to_string()),
            (IFernflowerPreferences::USE_DEBUG_VAR_NAMES, "1".to_string()),
            (IFernflowerPreferences::USE_METHOD_PARAMETERS, "1".to_string()),
            (IFernflowerPreferences::REMOVE_EMPTY_RANGES, "1".to_string()),
            (IFernflowerPreferences::FINALLY_DEINLINE, "1".to_string()),
            (IFernflowerPreferences::LAMBDA_TO_ANONYMOUS_CLASS, "0".to_string()),
            (IFernflowerPreferences::BYTECODE_SOURCE_MAPPING, "0".to_string()),
            (IFernflowerPreferences::DUMP_CODE_LINES, "0".to_string()),
            (IFernflowerPreferences::IGNORE_INVALID_BYTECODE, "0".to_string()),
            (IFernflowerPreferences::VERIFY_ANONYMOUS_CLASSES, "0".to_string()),
            (IFernflowerPreferences::TERNARY_CONSTANT_SIMPLIFICATION, "0".to_string()),
            (IFernflowerPreferences::OVERRIDE_ANNOTATION, "1".to_string()),
            (IFernflowerPreferences::PATTERN_MATCHING, "1".to_string()),
            (IFernflowerPreferences::TRY_LOOP_FIX, "1".to_string()),
            (IFernflowerPreferences::TERNARY_CONDITIONS, "1".to_string()),
            (IFernflowerPreferences::SWITCH_EXPRESSIONS, "1".to_string()),
            (IFernflowerPreferences::SHOW_HIDDEN_STATEMENTS, "0".to_string()),
            (IFernflowerPreferences::SIMPLIFY_STACK_SECOND_PASS, "1".to_string()),
            (IFernflowerPreferences::VERIFY_VARIABLE_MERGES, "0".to_string()),
            (IFernflowerPreferences::VERIFY_PRE_POST_VARIABLE_MERGES, "0".to_string()),
            (IFernflowerPreferences::OLD_TRY_DEDUP, "0".to_string()),
            (IFernflowerPreferences::DECOMPILE_PREVIEW, "1".to_string()),
            (IFernflowerPreferences::INCLUDE_ENTIRE_CLASSPATH, "0".to_string()),
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "1".to_string()),
            (IFernflowerPreferences::EXPLICIT_GENERIC_ARGUMENTS, "0".to_string()),
            (IFernflowerPreferences::INLINE_SIMPLE_LAMBDAS, "1".to_string()),
            (IFernflowerPreferences::LOG_LEVEL, Severity::Info.name().to_string()),
            (IFernflowerPreferences::MAX_PROCESSING_METHOD, "0".to_string()),
            (IFernflowerPreferences::RENAME_ENTITIES, "0".to_string()),
            (IFernflowerPreferences::NEW_LINE_SEPARATOR, "1".to_string()),
            (IFernflowerPreferences::INDENT_STRING, "   ".to_string()),
            (IFernflowerPreferences::PREFERRED_LINE_LENGTH, "160".to_string()),
            (IFernflowerPreferences::BANNER, "".to_string()),
            (IFernflowerPreferences::ERROR_MESSAGE, "Please report this to the Vineflower issue tracker, at https://github.com/Vineflower/vineflower/issues with a copy of the class file (if you have the rights to distribute it!)".to_string()),
            (IFernflowerPreferences::UNIT_TEST_MODE, "0".to_string()),
            (IFernflowerPreferences::DUMP_ORIGINAL_LINES, "0".to_string()),
            (IFernflowerPreferences::THREADS, available_processors().to_string()),
            (IFernflowerPreferences::SKIP_EXTRA_FILES, "0".to_string()),
            (IFernflowerPreferences::WARN_INCONSISTENT_INNER_CLASSES, "1".to_string()),
            (IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR, "1".to_string()),
            (IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, "1".to_string()),
            (IFernflowerPreferences::DECOMPILER_COMMENTS, "1".to_string()),
            (IFernflowerPreferences::SOURCE_FILE_COMMENTS, "0".to_string()),
            (IFernflowerPreferences::DECOMPILE_COMPLEX_CONDYS, "0".to_string()),
            (IFernflowerPreferences::FORCE_JSR_INLINE, "0".to_string()),
            (IFernflowerPreferences::DUMP_TEXT_TOKENS, "0".to_string()),
            (IFernflowerPreferences::REMOVE_IMPORTS, "0".to_string()),
            (IFernflowerPreferences::MARK_CORRESPONDING_SYNTHETICS, "0".to_string()),
            (IFernflowerPreferences::EXCLUDED_CLASSES, "".to_string()),
            (IFernflowerPreferences::INCLUDED_CLASSES, "".to_string()),
            (IFernflowerPreferences::VALIDATE_INNER_CLASSES_NAMES, "1".to_string()),
            (IFernflowerPreferences::PRETTIFY_IFS, "1".to_string()),
            (IFernflowerPreferences::METHOD_TO_DECOMPILE, "".to_string()),
        ]
    }

    /// Every option key in declaration order, with its annotations.
    pub fn all_options() -> &'static [PreferenceOption] {
        OPTIONS
    }

    /// The option whose key or short name is `id`.
    pub fn find_option(id: &str) -> Option<&'static PreferenceOption> {
        OPTIONS
            .iter()
            .find(|o| o.key == id || o.short_name == Some(id))
    }
}

fn available_processors() -> i32 {
    std::thread::available_parallelism().map_or(1, |n| n.get() as i32)
}

/// `DecompilerOption.Type`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum OptionType {
    Boolean,
    String,
    Integer,
}

impl OptionType {
    /// The enum constant's name (`BOOLEAN`, ...).
    pub fn name(self) -> &'static str {
        match self {
            OptionType::Boolean => "BOOLEAN",
            OptionType::String => "STRING",
            OptionType::Integer => "INTEGER",
        }
    }

    /// `DecompilerOption.Type.toString()`.
    pub fn friendly_name(self) -> &'static str {
        match self {
            OptionType::Boolean => "bool",
            OptionType::String => "string",
            OptionType::Integer => "int",
        }
    }
}

/// One option field of the interface and its annotations; `None` where the
/// field carries no such annotation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PreferenceOption {
    pub field: &'static str,
    pub key: &'static str,
    pub short_name: Option<&'static str>,
    pub name: Option<&'static str>,
    pub description: Option<&'static str>,
    pub type_: Option<OptionType>,
    pub dynamic_default_value: Option<&'static str>,
}

impl PreferenceOption {
    /// The entry of `DEFAULTS`, as `DecompilerOption` reads it.
    pub fn default_value(&self) -> Option<String> {
        IFernflowerPreferences::DEFAULTS().get_cloned(&self.key.to_string())
    }
}

static OPTIONS: &[PreferenceOption] = &[
    PreferenceOption { field: "REMOVE_BRIDGE", key: IFernflowerPreferences::REMOVE_BRIDGE, short_name: Some("rbr"), name: Some("Remove Bridge Methods"), description: Some("Removes any methods that are marked as bridge from the decompiled output."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "REMOVE_SYNTHETIC", key: IFernflowerPreferences::REMOVE_SYNTHETIC, short_name: Some("rsy"), name: Some("Remove Synthetic Methods And Fields"), description: Some("Removes any methods and fields that are marked as synthetic from the decompiled output."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DECOMPILE_INNER", key: IFernflowerPreferences::DECOMPILE_INNER, short_name: Some("din"), name: Some("Decompile Inner Classes"), description: Some("Process inner classes and add them to the decompiled output."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DECOMPILE_CLASS_1_4", key: IFernflowerPreferences::DECOMPILE_CLASS_1_4, short_name: Some("dc4"), name: Some("Decompile Java 4 class references"), description: Some("Resugar the Java 1-4 class reference format instead of leaving the synthetic code."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DECOMPILE_ASSERTIONS", key: IFernflowerPreferences::DECOMPILE_ASSERTIONS, short_name: Some("das"), name: Some("Decompile Assertions"), description: Some("Decompile assert statements."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "HIDE_EMPTY_SUPER", key: IFernflowerPreferences::HIDE_EMPTY_SUPER, short_name: Some("hes"), name: Some("Hide Empty super()"), description: Some("Hide super() calls with no parameters."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "HIDE_DEFAULT_CONSTRUCTOR", key: IFernflowerPreferences::HIDE_DEFAULT_CONSTRUCTOR, short_name: Some("hdc"), name: Some("Hide Default Constructor"), description: Some("Hide constructors with no parameters and no code."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DECOMPILE_GENERIC_SIGNATURES", key: IFernflowerPreferences::DECOMPILE_GENERIC_SIGNATURES, short_name: Some("dgs"), name: Some("Decompile Generics"), description: Some("Decompile generics in classes, methods, fields, and variables."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "INCORPORATE_RETURNS", key: IFernflowerPreferences::INCORPORATE_RETURNS, short_name: Some("ner"), name: Some("Incorporate returns in try-catch blocks"), description: Some("Integrate returns better in try-catch blocks instead of storing them in a temporary variable."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "ENSURE_SYNCHRONIZED_MONITOR", key: IFernflowerPreferences::ENSURE_SYNCHRONIZED_MONITOR, short_name: Some("esm"), name: Some("Ensure synchronized ranges are complete"), description: Some("If a synchronized block has a monitorenter without any corresponding monitorexit, try to deduce where one should be to ensure the synchronized is correctly decompiled."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DECOMPILE_ENUM", key: IFernflowerPreferences::DECOMPILE_ENUM, short_name: Some("den"), name: Some("Decompile Enums"), description: Some("Decompile enums."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DECOMPILE_PREVIEW", key: IFernflowerPreferences::DECOMPILE_PREVIEW, short_name: Some("dpr"), name: Some("Decompile Preview Features"), description: Some("Decompile features marked as preview or incubating in the latest Java versions."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "REMOVE_GET_CLASS_NEW", key: IFernflowerPreferences::REMOVE_GET_CLASS_NEW, short_name: Some("rgn"), name: Some("Remove reference getClass()"), description: Some("Remove synthetic getClass() calls created by code such as 'obj.new Inner()'."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "LITERALS_AS_IS", key: IFernflowerPreferences::LITERALS_AS_IS, short_name: Some("lit"), name: Some("Keep Literals As Is"), description: Some("Keep NaN, infinities, and pi values as is without resugaring them."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "BOOLEAN_TRUE_ONE", key: IFernflowerPreferences::BOOLEAN_TRUE_ONE, short_name: Some("bto"), name: Some("Represent boolean as 0/1"), description: Some("Represent integers 0 and 1 as booleans."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "ASCII_STRING_CHARACTERS", key: IFernflowerPreferences::ASCII_STRING_CHARACTERS, short_name: Some("asc"), name: Some("ASCII String Characters"), description: Some("Encode non-ASCII characters in string and character literals as Unicode escapes."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "SYNTHETIC_NOT_SET", key: IFernflowerPreferences::SYNTHETIC_NOT_SET, short_name: Some("nns"), name: Some("Synthetic Not Set"), description: Some("Treat some known structures as synthetic even when not explicitly set."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "UNDEFINED_PARAM_TYPE_OBJECT", key: IFernflowerPreferences::UNDEFINED_PARAM_TYPE_OBJECT, short_name: Some("uto"), name: Some("Treat Undefined Param Type As Object"), description: Some("Treat nameless types as java.lang.Object."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "USE_DEBUG_VAR_NAMES", key: IFernflowerPreferences::USE_DEBUG_VAR_NAMES, short_name: Some("udv"), name: Some("Use LVT Names"), description: Some("Use LVT names for local variables and parameters instead of var<index>_<version>."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "USE_METHOD_PARAMETERS", key: IFernflowerPreferences::USE_METHOD_PARAMETERS, short_name: Some("ump"), name: Some("Use Method Parameters"), description: Some("Use method parameter names, as given in the MethodParameters attribute."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "REMOVE_EMPTY_RANGES", key: IFernflowerPreferences::REMOVE_EMPTY_RANGES, short_name: Some("rer"), name: Some("Remove Empty try-catch blocks"), description: Some("Remove try-catch blocks with no code."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "FINALLY_DEINLINE", key: IFernflowerPreferences::FINALLY_DEINLINE, short_name: Some("fdi"), name: Some("Decompile Finally"), description: Some("Decompile finally blocks."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "LAMBDA_TO_ANONYMOUS_CLASS", key: IFernflowerPreferences::LAMBDA_TO_ANONYMOUS_CLASS, short_name: Some("lac"), name: Some("Decompile Lambdas as Anonymous Classes"), description: Some("Decompile lambda expressions as anonymous classes."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "BYTECODE_SOURCE_MAPPING", key: IFernflowerPreferences::BYTECODE_SOURCE_MAPPING, short_name: Some("bsm"), name: Some("Bytecode to Source Mapping"), description: Some("Map Bytecode to source lines."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DUMP_CODE_LINES", key: IFernflowerPreferences::DUMP_CODE_LINES, short_name: Some("dcl"), name: Some("Dump Code Lines"), description: Some("Dump line mappings to output archive zip entry extra data."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "IGNORE_INVALID_BYTECODE", key: IFernflowerPreferences::IGNORE_INVALID_BYTECODE, short_name: Some("iib"), name: Some("Ignore Invalid Bytecode"), description: Some("Ignore bytecode that is malformed."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "VERIFY_ANONYMOUS_CLASSES", key: IFernflowerPreferences::VERIFY_ANONYMOUS_CLASSES, short_name: Some("vac"), name: Some("Verify Anonymous Classes"), description: Some("Verify that anonymous classes are local."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "TERNARY_CONSTANT_SIMPLIFICATION", key: IFernflowerPreferences::TERNARY_CONSTANT_SIMPLIFICATION, short_name: Some("tcs"), name: Some("Ternary Constant Simplification"), description: Some("Fold branches of ternary expressions that have boolean true and false constants."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "PATTERN_MATCHING", key: IFernflowerPreferences::PATTERN_MATCHING, short_name: Some("pam"), name: Some("Pattern Matching"), description: Some("Decompile with if and switch pattern matching enabled."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "TRY_LOOP_FIX", key: IFernflowerPreferences::TRY_LOOP_FIX, short_name: Some("tlf"), name: Some("Try-Loop fix"), description: Some("Fixes rare cases of malformed decompilation when try blocks are found inside of while loops"), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "TERNARY_CONDITIONS", key: IFernflowerPreferences::TERNARY_CONDITIONS, short_name: Some("tco"), name: Some("[Experimental] Ternary In If Conditions"), description: Some("Tries to collapse if statements that have a ternary in their condition."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "SWITCH_EXPRESSIONS", key: IFernflowerPreferences::SWITCH_EXPRESSIONS, short_name: Some("swe"), name: Some("Decompile Switch Expressions"), description: Some("Decompile switch expressions in modern Java class files."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "SHOW_HIDDEN_STATEMENTS", key: IFernflowerPreferences::SHOW_HIDDEN_STATEMENTS, short_name: Some("shs"), name: Some("[Debug] Show hidden statements"), description: Some("Display hidden code blocks for debugging purposes."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "OVERRIDE_ANNOTATION", key: IFernflowerPreferences::OVERRIDE_ANNOTATION, short_name: Some("ovr"), name: Some("Override Annotation"), description: Some("Display override annotations for methods known to the decompiler."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "SIMPLIFY_STACK_SECOND_PASS", key: IFernflowerPreferences::SIMPLIFY_STACK_SECOND_PASS, short_name: Some("ssp"), name: Some("Second-Pass Stack Simplification"), description: Some("Simplify variables across stack bounds to resugar complex statements."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "VERIFY_VARIABLE_MERGES", key: IFernflowerPreferences::VERIFY_VARIABLE_MERGES, short_name: Some("vvm"), name: Some("[Experimental] Verify Variable Merges"), description: Some("Tries harder to verify the validity of variable merges. If there are strange variable recompilation issues, this is a good place to start."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "VERIFY_PRE_POST_VARIABLE_MERGES", key: IFernflowerPreferences::VERIFY_PRE_POST_VARIABLE_MERGES, short_name: Some("pvm"), name: Some("[Experimental] Verify Pre Post Variable Merges"), description: Some("Will try to validate that code before and after variable merges is equivalent"), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "OLD_TRY_DEDUP", key: IFernflowerPreferences::OLD_TRY_DEDUP, short_name: None, name: Some("[Experimental] Use old try deduplication"), description: Some("Use the old try deduplication algorithm for methods with obfuscated exceptions, which inserts dummy exception handlers instead of duplicating blocks"), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "INCLUDE_ENTIRE_CLASSPATH", key: IFernflowerPreferences::INCLUDE_ENTIRE_CLASSPATH, short_name: Some("iec"), name: Some("Include Entire Classpath"), description: Some("Give the decompiler information about every jar on the classpath."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "INCLUDE_JAVA_RUNTIME", key: IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, short_name: Some("jrt"), name: Some("Include Java Runtime"), description: Some("Give the decompiler information about the Java runtime, either 1 or current for the current runtime, or a path to another runtime"), type_: Some(OptionType::String), dynamic_default_value: None },
    PreferenceOption { field: "EXPLICIT_GENERIC_ARGUMENTS", key: IFernflowerPreferences::EXPLICIT_GENERIC_ARGUMENTS, short_name: Some("ega"), name: Some("Explicit Generic Arguments"), description: Some("Put explicit diamond generic arguments on method calls."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "INLINE_SIMPLE_LAMBDAS", key: IFernflowerPreferences::INLINE_SIMPLE_LAMBDAS, short_name: Some("isl"), name: Some("Inline Simple Lambdas"), description: Some("Remove braces on simple, one line, lambda expressions."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "LOG_LEVEL", key: IFernflowerPreferences::LOG_LEVEL, short_name: Some("log"), name: Some("Logging Level"), description: Some("Logging level. Must be one of: 'info', 'debug', 'warn', 'error'."), type_: Some(OptionType::String), dynamic_default_value: None },
    PreferenceOption { field: "MAX_PROCESSING_METHOD", key: IFernflowerPreferences::MAX_PROCESSING_METHOD, short_name: Some("mpm"), name: Some("[DEPRECATED] Max time to process method"), description: Some("Maximum time in seconds to process a method. This is deprecated, do not use."), type_: Some(OptionType::Integer), dynamic_default_value: None },
    PreferenceOption { field: "RENAME_ENTITIES", key: IFernflowerPreferences::RENAME_ENTITIES, short_name: Some("ren"), name: Some("Rename Members"), description: Some("Rename classes, fields, and methods with a number suffix to help in deobfuscation."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "USER_RENAMER_CLASS", key: IFernflowerPreferences::USER_RENAMER_CLASS, short_name: Some("urc"), name: Some("User Renamer Class"), description: Some("Path to a class that implements IIdentifierRenamer."), type_: Some(OptionType::String), dynamic_default_value: None },
    PreferenceOption { field: "NEW_LINE_SEPARATOR", key: IFernflowerPreferences::NEW_LINE_SEPARATOR, short_name: Some("nls"), name: Some("[DEPRECATED] New Line Seperator"), description: Some("Use \\n instead of \\r\\n for new lines. Deprecated, do not use."), type_: Some(OptionType::Boolean), dynamic_default_value: Some("Disabled on Windows, enabled on other systems") },
    PreferenceOption { field: "INDENT_STRING", key: IFernflowerPreferences::INDENT_STRING, short_name: Some("ind"), name: Some("Indent String"), description: Some("A string of spaces or tabs that is placed for each indent level."), type_: Some(OptionType::String), dynamic_default_value: None },
    PreferenceOption { field: "PREFERRED_LINE_LENGTH", key: IFernflowerPreferences::PREFERRED_LINE_LENGTH, short_name: Some("pll"), name: Some("Preferred line length"), description: Some("Max line length before formatting is applied."), type_: Some(OptionType::Integer), dynamic_default_value: None },
    PreferenceOption { field: "BANNER", key: IFernflowerPreferences::BANNER, short_name: Some("ban"), name: Some("Banner"), description: Some("A message to display at the top of the decompiled file."), type_: Some(OptionType::String), dynamic_default_value: None },
    PreferenceOption { field: "ERROR_MESSAGE", key: IFernflowerPreferences::ERROR_MESSAGE, short_name: Some("erm"), name: Some("Error Message"), description: Some("Message to display when an error occurs in the decompiler."), type_: Some(OptionType::String), dynamic_default_value: None },
    PreferenceOption { field: "THREADS", key: IFernflowerPreferences::THREADS, short_name: Some("thr"), name: Some("Thread Count"), description: Some("How many threads to use to decompile."), type_: Some(OptionType::Integer), dynamic_default_value: Some("all available processors") },
    PreferenceOption { field: "DUMP_ORIGINAL_LINES", key: IFernflowerPreferences::DUMP_ORIGINAL_LINES, short_name: None, name: None, description: None, type_: None, dynamic_default_value: None },
    PreferenceOption { field: "UNIT_TEST_MODE", key: IFernflowerPreferences::UNIT_TEST_MODE, short_name: None, name: None, description: None, type_: None, dynamic_default_value: None },
    PreferenceOption { field: "SKIP_EXTRA_FILES", key: IFernflowerPreferences::SKIP_EXTRA_FILES, short_name: Some("sef"), name: Some("Skip Extra Files"), description: Some("Skip copying non-class files from the input folder or file to the output"), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "WARN_INCONSISTENT_INNER_CLASSES", key: IFernflowerPreferences::WARN_INCONSISTENT_INNER_CLASSES, short_name: Some("win"), name: Some("Warn about inconsistent inner attributes"), description: Some("Warn about inconsistent inner class attributes"), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DUMP_BYTECODE_ON_ERROR", key: IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR, short_name: Some("dbe"), name: Some("Dump Bytecode On Error"), description: Some("Put the bytecode in the method body when an error occurs."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DUMP_EXCEPTION_ON_ERROR", key: IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, short_name: Some("dee"), name: Some("Dump Exceptions On Error"), description: Some("Put the exception message in the method body or source file when an error occurs."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DECOMPILER_COMMENTS", key: IFernflowerPreferences::DECOMPILER_COMMENTS, short_name: Some("dec"), name: Some("Decompiler Comments"), description: Some("Sometimes, odd behavior of the bytecode or unfixable problems occur. This enables or disables the adding of those to the decompiled output."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "SOURCE_FILE_COMMENTS", key: IFernflowerPreferences::SOURCE_FILE_COMMENTS, short_name: Some("sfc"), name: Some("SourceFile comments"), description: Some("Add debug comments showing the class SourceFile attribute if present."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DECOMPILE_COMPLEX_CONDYS", key: IFernflowerPreferences::DECOMPILE_COMPLEX_CONDYS, short_name: Some("dcc"), name: Some("Decompile complex constant-dynamic expressions"), description: Some("Some constant-dynamic expressions can't be converted to a single Java expression with identical run-time behaviour. This decompiles them to a similar non-lazy expression, marked with a comment."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "FORCE_JSR_INLINE", key: IFernflowerPreferences::FORCE_JSR_INLINE, short_name: Some("fji"), name: Some("Force JSR inline"), description: Some("Forces the processing of JSR instructions even if the class files shouldn't contain it (Java 7+)"), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "DUMP_TEXT_TOKENS", key: IFernflowerPreferences::DUMP_TEXT_TOKENS, short_name: Some("dtt"), name: Some("Dump Text Tokens"), description: Some("Dump Text Tokens on each class file"), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "REMOVE_IMPORTS", key: IFernflowerPreferences::REMOVE_IMPORTS, short_name: Some("rim"), name: Some("Remove Imports"), description: Some("Remove import statements from the decompiled code"), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "MARK_CORRESPONDING_SYNTHETICS", key: IFernflowerPreferences::MARK_CORRESPONDING_SYNTHETICS, short_name: Some("mcs"), name: Some("Mark Corresponding Synthetics"), description: Some("Mark lambdas and anonymous and local classes with their respective synthetic constructs"), type_: None, dynamic_default_value: None },
    PreferenceOption { field: "EXCLUDED_CLASSES", key: IFernflowerPreferences::EXCLUDED_CLASSES, short_name: None, name: Some("Excluded Classes"), description: Some("Exclude classes from decompilation if their fully qualified names match the specified regular expression."), type_: Some(OptionType::String), dynamic_default_value: None },
    PreferenceOption { field: "INCLUDED_CLASSES", key: IFernflowerPreferences::INCLUDED_CLASSES, short_name: None, name: Some("Included Classes"), description: Some("Include classes in decompilation only if their fully qualified names match the specified regular expression."), type_: Some(OptionType::String), dynamic_default_value: None },
    PreferenceOption { field: "VALIDATE_INNER_CLASSES_NAMES", key: IFernflowerPreferences::VALIDATE_INNER_CLASSES_NAMES, short_name: None, name: Some("Validate inner classes names"), description: Some("Validates that the inner class name is correct (if it is separated using \"$\" for example BaseClass$InnerClass). If not then inner class won't be processed."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "PRETTIFY_IFS", key: IFernflowerPreferences::PRETTIFY_IFS, short_name: None, name: Some("Prettify Ifs"), description: Some("Use heuristics to restructure if statements to make them clearer to read."), type_: Some(OptionType::Boolean), dynamic_default_value: None },
    PreferenceOption { field: "METHOD_TO_DECOMPILE", key: IFernflowerPreferences::METHOD_TO_DECOMPILE, short_name: None, name: Some("Method to decompile"), description: Some("Option to decompile a single method. Set to owner + \".\" + name + descriptor, e.g. foo/Bar.baz()V, or simply name + descriptor if you're only decompiling a single file and the method belongs to the root class."), type_: Some(OptionType::String), dynamic_default_value: None },
];
