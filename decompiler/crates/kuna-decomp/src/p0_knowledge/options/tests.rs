//! In-module tests for `options` (the `ArchOption` dispatch surface, W4).
//!
//! Two test families:
//!   1. **Parsing matrices** — every option's on/off/bad-value/integer paths,
//!      asserting the *exact* C++ message/error text (the contract `run_tests`
//!      and the console exercise).
//!   2. **XML decode** — an `<optionslist>` stream driven through
//!      [`OptionDatabase::decode`], checking dispatch, param threading, and the
//!      bare-content (param1) path.
//!
//! The W5/W6/W8 subsystems are not alive, so a [`RecordingContext`] implements
//! [`ArchOptionContext`] by recording calls + holding the alive config fields,
//! which is enough to exercise every `apply()` end-to-end.

use super::*;

use kuna_base::marshal::{IdRegistry, XmlDecode};
use kuna_base::space::AddrSpaceManager;

// ---------------------------------------------------------------------------
// A test ArchOptionContext: holds the "alive" config fields and records calls
// into the W5/W6/W8-seamed methods so the apply() bodies can be exercised.
// ---------------------------------------------------------------------------

/// A recording / minimal-state [`ArchOptionContext`] for tests.
#[derive(Default)]
struct RecordingContext {
    readonly_propagate: bool,
    infer_pointers: bool,
    analyze_for_loops: bool,
    max_jumptable_size: uint4,
    max_instructions: int4,
    alias_block_level: int4,
    flowoptions: uint4,
    split_datatype_config: uint4,
    nan_ignore_all: bool,
    nan_ignore_compare: bool,
    // Printer (seamed): toggle whether the printer is C language.
    is_c_language: bool,
    header_comment: uint4,
    instruction_comment: uint4,
    current_action: String,
    // Lookup-failure injection for the STUB methods that can error in C++.
    unknown_function: bool,
    unknown_model: bool,
    bad_action_warning: bool,
    rule_present: bool,
    has_current_action: bool,
    // Recorded calls (name -> last value), for spot assertions.
    log: Vec<String>,
}

impl RecordingContext {
    fn c_lang() -> Self {
        RecordingContext { is_c_language: true, has_current_action: true, ..Default::default() }
    }
}

impl ArchOptionContext for RecordingContext {
    fn set_readonly_propagate(&mut self, val: bool) {
        self.readonly_propagate = val;
    }
    fn set_infer_pointers(&mut self, val: bool) {
        self.infer_pointers = val;
    }
    fn set_analyze_for_loops(&mut self, val: bool) {
        self.analyze_for_loops = val;
    }
    fn set_max_jumptable_size(&mut self, val: uint4) {
        self.max_jumptable_size = val;
    }
    fn set_max_instructions(&mut self, val: int4) {
        self.max_instructions = val;
    }
    fn alias_block_level(&self) -> int4 {
        self.alias_block_level
    }
    fn set_alias_block_level(&mut self, val: int4) {
        self.alias_block_level = val;
    }
    fn flow_options(&self) -> uint4 {
        self.flowoptions
    }
    fn set_flow_options(&mut self, val: uint4) {
        self.flowoptions = val;
    }
    fn split_datatype_config(&self) -> uint4 {
        self.split_datatype_config
    }
    fn set_split_datatype_config(&mut self, val: uint4) {
        self.split_datatype_config = val;
    }
    fn nan_ignore_all(&self) -> bool {
        self.nan_ignore_all
    }
    fn set_nan_ignore_all(&mut self, val: bool) {
        self.nan_ignore_all = val;
    }
    fn nan_ignore_compare(&self) -> bool {
        self.nan_ignore_compare
    }
    fn set_nan_ignore_compare(&mut self, val: bool) {
        self.nan_ignore_compare = val;
    }
    fn set_default_extra_pop(&mut self, expop: int4) {
        self.log.push(format!("default_extrapop={expop}"));
    }
    fn set_function_extra_pop(&mut self, name: &str, expop: int4) -> KunaResult<()> {
        if self.unknown_function {
            return Err(KunaError::recov(format!("Unknown function name: {name}")));
        }
        self.log.push(format!("fn_extrapop[{name}]={expop}"));
        Ok(())
    }
    fn set_default_model(&mut self, name: &str) -> KunaResult<()> {
        if self.unknown_model {
            return Err(KunaError::lowlevel(format!("Unknown prototype model :{name}")));
        }
        self.log.push(format!("default_model={name}"));
        Ok(())
    }
    fn set_eval_current_model(&mut self, name: &str) -> KunaResult<()> {
        if self.unknown_model {
            return Err(KunaError::parse(format!("Unknown prototype model: {name}")));
        }
        self.log.push(format!("eval_model={name}"));
        Ok(())
    }
    fn set_function_inline(&mut self, name: &str, val: bool) -> KunaResult<()> {
        if self.unknown_function {
            return Err(KunaError::recov(format!("Unknown function name: {name}")));
        }
        self.log.push(format!("inline[{name}]={val}"));
        Ok(())
    }
    fn set_function_no_return(&mut self, name: &str, val: bool) -> KunaResult<()> {
        if self.unknown_function {
            return Err(KunaError::recov(format!("Unknown function name: {name}")));
        }
        self.log.push(format!("noreturn[{name}]={val}"));
        Ok(())
    }
    fn print_is_c_language(&self) -> bool {
        self.is_c_language
    }
    fn set_null_printing(&mut self, val: bool) {
        self.log.push(format!("null_printing={val}"));
    }
    fn set_inplace_ops(&mut self, val: bool) {
        self.log.push(format!("inplace_ops={val}"));
    }
    fn set_convention_printing(&mut self, val: bool) {
        self.log.push(format!("convention={val}"));
    }
    fn set_no_cast_printing(&mut self, val: bool) {
        self.log.push(format!("no_cast={val}"));
    }
    fn set_hide_implied_exts(&mut self, val: bool) {
        self.log.push(format!("hide_exts={val}"));
    }
    fn set_max_line_size(&mut self, val: int4) {
        self.log.push(format!("max_line={val}"));
    }
    fn set_indent_increment(&mut self, val: int4) {
        self.log.push(format!("indent={val}"));
    }
    fn set_line_comment_indent(&mut self, val: int4) {
        self.log.push(format!("comment_indent={val}"));
    }
    fn set_comment_style(&mut self, style: &str) {
        self.log.push(format!("comment_style={style}"));
    }
    fn header_comment_flags(&self) -> uint4 {
        self.header_comment
    }
    fn set_header_comment_flags(&mut self, flags: uint4) {
        self.header_comment = flags;
    }
    fn instruction_comment_flags(&self) -> uint4 {
        self.instruction_comment
    }
    fn set_instruction_comment_flags(&mut self, flags: uint4) {
        self.instruction_comment = flags;
    }
    fn set_integer_format(&mut self, fmt: &str) {
        self.log.push(format!("int_format={fmt}"));
    }
    fn set_namespace_strategy(&mut self, strategy: NamespaceStrategy) {
        self.log.push(format!("namespace={strategy:?}"));
    }
    fn set_brace_format(&mut self, category: BraceCategory, style: BraceStyle) {
        self.log.push(format!("brace[{category:?}]={style:?}"));
    }
    fn set_print_language(&mut self, language: &str) {
        self.log.push(format!("language={language}"));
    }
    fn set_action_warning(&mut self, val: bool, name: &str) -> bool {
        if self.bad_action_warning {
            return false;
        }
        self.log.push(format!("warning[{name}]={val}"));
        true
    }
    fn clone_action_group(&mut self, from: &str, to: &str) {
        self.log.push(format!("clone {from}->{to}"));
    }
    fn set_current_action(&mut self, name: &str) {
        self.current_action = name.to_string();
    }
    fn current_action_name(&self) -> String {
        self.current_action.clone()
    }
    fn toggle_action(&mut self, group: &str, sub: &str, val: bool) {
        self.log.push(format!("toggle[{group}.{sub}]={val}"));
    }
    fn enable_rule(&mut self, path: &str) -> bool {
        self.log.push(format!("enable {path}"));
        self.rule_present
    }
    fn disable_rule(&mut self, path: &str) -> bool {
        self.log.push(format!("disable {path}"));
        self.rule_present
    }
    fn has_current_action(&self) -> bool {
        self.has_current_action
    }
    fn allow_context_set(&mut self, val: bool) {
        self.log.push(format!("allow_context={val}"));
    }
}

/// Run an option apply() by name through a fresh database + context, returning
/// the Result message/error.
fn apply_named(
    glb: &mut RecordingContext,
    name: &str,
    p1: &str,
    p2: &str,
    p3: &str,
) -> KunaResult<String> {
    // Build name->id from the upstream + framing elements, then dispatch.
    let id = lookup_upstream_id(name).unwrap_or_else(|| panic!("no element id for {name}"));
    let db = OptionDatabase::new();
    db.set(glb, id, p1, p2, p3)
}

/// Resolve an upstream option name to its element id (mirrors the C++
/// `ElementId::find(name,0)`), for the test harness only.
fn lookup_upstream_id(name: &str) -> Option<uint4> {
    let mut reg = IdRegistry::new();
    register_option_elements(&mut reg);
    let id = reg.find_element(name, 0);
    if id == ELEM_UNKNOWN.get_id() {
        None
    } else {
        Some(id)
    }
}

// ---------------------------------------------------------------------------
// on_or_off
// ---------------------------------------------------------------------------

#[test]
fn test_on_or_off_matrix() {
    assert!(on_or_off("").unwrap()); // empty => true (C++ size()==0)
    assert!(on_or_off("on").unwrap());
    assert!(!on_or_off("off").unwrap());
    let e = on_or_off("yes").unwrap_err();
    assert_eq!(e.explain(), "Must specify toggle value, on/off");
    assert!(matches!(e, KunaError::Parse { .. }));
    // case-sensitive: "On" is not "on".
    assert_eq!(
        on_or_off("On").unwrap_err().explain(),
        "Must specify toggle value, on/off"
    );
}

// ---------------------------------------------------------------------------
// Integer base auto-detection (the istringstream >> int knob).
// ---------------------------------------------------------------------------

#[test]
fn test_parse_int_auto_bases() {
    // decimal, hex (0x), octal (leading 0), sign.
    assert_eq!(parse_int_auto::<int4>("42"), Some(42));
    assert_eq!(parse_int_auto::<int4>("0x2a"), Some(0x2a));
    assert_eq!(parse_int_auto::<int4>("0X2A"), Some(0x2a));
    assert_eq!(parse_int_auto::<int4>("0755"), Some(0o755));
    assert_eq!(parse_int_auto::<int4>("-7"), Some(-7));
    assert_eq!(parse_int_auto::<int4>("+9"), Some(9));
    // "0" alone parses as 0 (octal base, single digit).
    assert_eq!(parse_int_auto::<int4>("0"), Some(0));
    // "0x" with no hex digit: subject sequence is just "0".
    assert_eq!(parse_int_auto::<int4>("0x"), Some(0));
    // trailing garbage stops extraction at the first non-digit.
    assert_eq!(parse_int_auto::<int4>("12ab"), Some(12));
    // Non-empty field with no leading digit: C++11 num_get sets failbit AND
    // stores 0 (verified against g++/clang++ -std=c++11). The target is
    // overwritten with 0, NOT left at the caller's sentinel.
    assert_eq!(parse_int_auto::<int4>("abc"), Some(0));
    assert_eq!(parse_int_auto::<int4>("unknown"), Some(0));
    assert_eq!(parse_int_auto::<int4>("-"), Some(0));
    assert_eq!(parse_int_auto::<int4>("+"), Some(0));
    assert_eq!(parse_int_auto::<int4>("0xZ"), Some(0));
    // Empty OR whitespace-only field: C++11 leaves the target unchanged
    // (extraction fails before storing anything), so the sentinel survives.
    assert_eq!(parse_int_auto::<int4>(""), None);
    assert_eq!(parse_int_auto::<int4>("   "), None);
    assert_eq!(parse_int_auto::<int4>(" \t\n"), None);
    // Leading whitespace then non-digit is a non-empty field => stores 0.
    assert_eq!(parse_int_auto::<int4>("  abc"), Some(0));
    // unsigned path
    assert_eq!(parse_int_auto::<uint4>("0x100"), Some(0x100));
    assert_eq!(parse_int_auto::<uint4>("notanint"), Some(0));
    assert_eq!(parse_int_auto::<uint4>(""), None);
}

// ---------------------------------------------------------------------------
// extrapop
// ---------------------------------------------------------------------------

#[test]
fn test_extrapop() {
    let mut g = RecordingContext::default();
    // "unknown" => extrapop_unknown, global path.
    assert_eq!(
        apply_named(&mut g, "extrapop", "unknown", "", "").unwrap(),
        "Global extrapop set"
    );
    assert!(g.log.iter().any(|s| s == "default_extrapop=32768"));
    // integer, global path.
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "extrapop", "8", "", "").unwrap(),
        "Global extrapop set"
    );
    assert!(g.log.iter().any(|s| s == "default_extrapop=8"));
    // Non-numeric (not "unknown"): C++11 num_get stores 0 (not the -300
    // sentinel), so expop=0 != -300 and the option is ACCEPTED with value 0
    // (oracle: `int expop=-300; iss("garbage")>>expop => expop=0`).
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "extrapop", "garbage", "", "").unwrap(),
        "Global extrapop set"
    );
    assert!(g.log.iter().any(|s| s == "default_extrapop=0"));
    // Empty (or whitespace-only) parameter: extraction leaves expop at the -300
    // sentinel => ParseError (the only branch that still throws).
    let mut g = RecordingContext::default();
    let e = apply_named(&mut g, "extrapop", "", "", "").unwrap_err();
    assert_eq!(e.explain(), "Bad extrapop adjustment parameter");
    let mut g = RecordingContext::default();
    let e = apply_named(&mut g, "extrapop", "   ", "", "").unwrap_err();
    assert_eq!(e.explain(), "Bad extrapop adjustment parameter");
    // per-function path.
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "extrapop", "4", "myfunc", "").unwrap(),
        "ExtraPop set for function myfunc"
    );
    // per-function unknown name => RecovError.
    let mut g = RecordingContext { unknown_function: true, ..Default::default() };
    let e = apply_named(&mut g, "extrapop", "4", "missing", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown function name: missing");
    assert!(matches!(e, KunaError::Recov { .. }));
}

// ---------------------------------------------------------------------------
// readonly / inferconstptr / analyzeforloops
// ---------------------------------------------------------------------------

#[test]
fn test_readonly() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "readonly", "on", "", "").unwrap(),
        "Read-only memory locations now propagate as constants"
    );
    assert!(g.readonly_propagate);
    assert_eq!(
        apply_named(&mut g, "readonly", "off", "", "").unwrap(),
        "Read-only memory locations now do not propagate"
    );
    assert!(!g.readonly_propagate);
    // empty p1 => the dedicated ParseError (NOT onOrOff's default-true).
    let e = apply_named(&mut g, "readonly", "", "", "").unwrap_err();
    assert_eq!(e.explain(), "Read-only option must be set \"on\" or \"off\"");
    // bad value => onOrOff error.
    let e = apply_named(&mut g, "readonly", "maybe", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify toggle value, on/off");
}

#[test]
fn test_inferconstptr() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "inferconstptr", "on", "", "").unwrap(),
        "Constant pointers are now inferred"
    );
    assert!(g.infer_pointers);
    assert_eq!(
        apply_named(&mut g, "inferconstptr", "off", "", "").unwrap(),
        "Constant pointers must now be set explicitly"
    );
    assert!(!g.infer_pointers);
    // empty => onOrOff => true.
    assert_eq!(
        apply_named(&mut g, "inferconstptr", "", "", "").unwrap(),
        "Constant pointers are now inferred"
    );
}

#[test]
fn test_analyzeforloops() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "analyzeforloops", "on", "", "").unwrap(),
        "Recovery of for-loops is on"
    );
    assert!(g.analyze_for_loops);
    // message echoes p1 verbatim even for the empty (=>true) case.
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "analyzeforloops", "", "", "").unwrap(),
        "Recovery of for-loops is "
    );
    assert!(g.analyze_for_loops);
}

// ---------------------------------------------------------------------------
// inline / noreturn (true/false parsing, unknown function)
// ---------------------------------------------------------------------------

#[test]
fn test_inline() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "inline", "foo", "true", "").unwrap(),
        "Inline property for function foo = true"
    );
    // p2 empty => true.
    assert_eq!(
        apply_named(&mut g, "inline", "foo", "", "").unwrap(),
        "Inline property for function foo = true"
    );
    // p2 anything-but-"true" => false.
    assert_eq!(
        apply_named(&mut g, "inline", "foo", "false", "").unwrap(),
        "Inline property for function foo = false"
    );
    assert_eq!(
        apply_named(&mut g, "inline", "foo", "nope", "").unwrap(),
        "Inline property for function foo = false"
    );
    // unknown function.
    let mut g = RecordingContext { unknown_function: true, ..Default::default() };
    let e = apply_named(&mut g, "inline", "ghost", "true", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown function name: ghost");
}

#[test]
fn test_noreturn() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "noreturn", "bar", "true", "").unwrap(),
        "No return property for function bar = true"
    );
    let mut g = RecordingContext { unknown_function: true, ..Default::default() };
    let e = apply_named(&mut g, "noreturn", "bar", "true", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown function name: bar");
}

// ---------------------------------------------------------------------------
// warning
// ---------------------------------------------------------------------------

#[test]
fn test_warning() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "warning", "RuleFoo", "on", "").unwrap(),
        "Warnings for RuleFoo turned on"
    );
    // p2 empty => true => "on".
    assert_eq!(
        apply_named(&mut g, "warning", "RuleFoo", "", "").unwrap(),
        "Warnings for RuleFoo turned on"
    );
    // empty p1 => ParseError.
    let e = apply_named(&mut g, "warning", "", "on", "").unwrap_err();
    assert_eq!(e.explain(), "No action/rule specified");
    // bad action/rule specifier => RecovError.
    let mut g = RecordingContext { bad_action_warning: true, ..Default::default() };
    let e = apply_named(&mut g, "warning", "RuleBad", "on", "").unwrap_err();
    assert_eq!(e.explain(), "Bad action/rule specifier: RuleBad");
}

// ---------------------------------------------------------------------------
// printer-gated options (c-language vs not)
// ---------------------------------------------------------------------------

#[test]
fn test_nullprinting_lang_gate() {
    // non-C printer: special message, no mutation.
    let mut g = RecordingContext::default(); // is_c_language=false
    assert_eq!(
        apply_named(&mut g, "nullprinting", "on", "", "").unwrap(),
        "Only a known output language accepts the null printing option"
    );
    assert!(g.log.is_empty());
    // C printer: applies.
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "nullprinting", "on", "", "").unwrap(),
        "Null printing turned on"
    );
    assert!(g.log.iter().any(|s| s == "null_printing=true"));
}

#[test]
fn test_inplaceops_and_convention_gates() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "inplaceops", "on", "", "").unwrap(),
        "Can only set inplace operators for a known output language"
    );
    assert_eq!(
        apply_named(&mut g, "conventionprinting", "on", "", "").unwrap(),
        "Can only set convention printing for a known output language"
    );
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "inplaceops", "off", "", "").unwrap(),
        "Inplace operators turned off"
    );
    assert_eq!(
        apply_named(&mut g, "conventionprinting", "on", "", "").unwrap(),
        "Convention printing turned on"
    );
}

#[test]
fn test_nocastprinting_gate() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "nocastprinting", "on", "", "").unwrap(),
        "Can only set no cast printing for a known output language"
    );
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "nocastprinting", "on", "", "").unwrap(),
        "No cast printing turned on"
    );
}

#[test]
fn test_hideextensions_gate() {
    // OptionHideExtensions is NOT registered in OptionDatabase::new (faithful to
    // upstream; it is reachable only via the console `option` command by name),
    // so it is exercised by calling apply() on the struct directly.
    let opt = OptionHideExtensions;
    let mut g = RecordingContext::default();
    assert_eq!(
        opt.apply(&mut g, "on", "", "").unwrap(),
        "Can only toggle extension hiding for a known output language"
    );
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        opt.apply(&mut g, "off", "", "").unwrap(),
        "Implied extension hiding turned off"
    );
    assert!(g.log.iter().any(|s| s == "hide_exts=false"));
}

// ---------------------------------------------------------------------------
// integer-valued printer options
// ---------------------------------------------------------------------------

#[test]
fn test_maxlinewidth() {
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "maxlinewidth", "120", "", "").unwrap(),
        "Maximum line width set to 120"
    );
    assert!(g.log.iter().any(|s| s == "max_line=120"));
    // Empty (or whitespace-only) => the -1 sentinel survives => ParseError.
    let e = apply_named(&mut g, "maxlinewidth", "", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify integer linewidth");
    let e = apply_named(&mut g, "maxlinewidth", "   ", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify integer linewidth");
    // Non-empty non-numeric ("wide"): C++11 num_get stores 0 (val=0 != -1), so
    // the option is ACCEPTED with width 0 (oracle: `int val=-1; iss("wide")>>val
    // => val=0`).
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "maxlinewidth", "wide", "", "").unwrap(),
        "Maximum line width set to wide"
    );
    assert!(g.log.iter().any(|s| s == "max_line=0"));
}

#[test]
fn test_indentincrement_and_commentindent() {
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "indentincrement", "4", "", "").unwrap(),
        "Characters per indent level set to 4"
    );
    assert_eq!(
        apply_named(&mut g, "commentindent", "20", "", "").unwrap(),
        "Comment indent set to 20"
    );
    // Empty (or whitespace-only) => the -1 sentinel survives => ParseError.
    let e = apply_named(&mut g, "indentincrement", "", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify integer increment");
    let e = apply_named(&mut g, "commentindent", "  ", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify integer comment indent");
    // Non-empty non-numeric ("x"): C++11 num_get stores 0 (val=0 != -1), so the
    // option is ACCEPTED with value 0 (oracle: `iss("x")>>val => val=0`).
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "indentincrement", "x", "", "").unwrap(),
        "Characters per indent level set to x"
    );
    assert_eq!(
        apply_named(&mut g, "commentindent", "x", "", "").unwrap(),
        "Comment indent set to x"
    );
}

// ---------------------------------------------------------------------------
// comment style / type
// ---------------------------------------------------------------------------

#[test]
fn test_commentstyle_and_integerformat() {
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "commentstyle", "cplusplus", "", "").unwrap(),
        "Comment style set to cplusplus"
    );
    assert_eq!(
        apply_named(&mut g, "integerformat", "hex", "", "").unwrap(),
        "Integer format set to hex"
    );
}

#[test]
fn test_commentheader_and_instruction() {
    let mut g = RecordingContext::c_lang();
    // turn on "warning" header bit (warning=16).
    assert_eq!(
        apply_named(&mut g, "commentheader", "warning", "on", "").unwrap(),
        "Header comment type warning turned on"
    );
    assert_eq!(g.header_comment, comment_type::WARNING);
    // turn it off again.
    assert_eq!(
        apply_named(&mut g, "commentheader", "warning", "off", "").unwrap(),
        "Header comment type warning turned off"
    );
    assert_eq!(g.header_comment, 0);
    // unknown comment type => LowlevelError.
    let e = apply_named(&mut g, "commentheader", "bogus", "on", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown comment type: bogus");
    assert!(matches!(e, KunaError::Lowlevel { .. }));
    // instruction variant.
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "commentinstruction", "user1", "on", "").unwrap(),
        "Instruction comment type user1 turned on"
    );
    assert_eq!(g.instruction_comment, comment_type::USER1);
}

// ---------------------------------------------------------------------------
// braceformat
// ---------------------------------------------------------------------------

#[test]
fn test_braceformat() {
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "braceformat", "function", "next", "").unwrap(),
        "Brace formatting for function set to next"
    );
    assert!(g.log.iter().any(|s| s == "brace[Function]=NextLine"));
    // unknown style.
    let e = apply_named(&mut g, "braceformat", "function", "weird", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown brace style: weird");
    // unknown category (with a valid style so we reach the category check).
    let e = apply_named(&mut g, "braceformat", "weird", "same", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown brace format category: weird");
    // non-C printer.
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "braceformat", "function", "same", "").unwrap(),
        "Can only set brace formatting for a known output language"
    );
}

// ---------------------------------------------------------------------------
// setaction / currentaction
// ---------------------------------------------------------------------------

#[test]
fn test_setaction() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "setaction", "decompile", "", "").unwrap(),
        "Set current action to decompile"
    );
    assert_eq!(g.current_action, "decompile");
    // clone path (p2 present).
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "setaction", "decompile", "mycopy", "").unwrap(),
        "Created mycopy by cloning decompile and made it current"
    );
    assert_eq!(g.current_action, "mycopy");
    // empty p1.
    let e = apply_named(&mut g, "setaction", "", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify preexisting action");
}

#[test]
fn test_currentaction() {
    // two-param form: toggle subgroup in current action.
    let mut g = RecordingContext { current_action: "decompile".into(), ..Default::default() };
    assert_eq!(
        apply_named(&mut g, "currentaction", "deadcode", "off", "").unwrap(),
        "Toggled deadcode in action decompile"
    );
    assert!(g.log.iter().any(|s| s == "toggle[decompile.deadcode]=false"));
    // three-param form: set current then toggle.
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "currentaction", "myroot", "deadcode", "on").unwrap(),
        "Toggled deadcode in action myroot"
    );
    assert_eq!(g.current_action, "myroot");
    // missing args.
    let e = apply_named(&mut g, "currentaction", "x", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify subaction, on/off");
}

// ---------------------------------------------------------------------------
// allowcontextset / flow-flag options
// ---------------------------------------------------------------------------

#[test]
fn test_allowcontextset() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "allowcontextset", "on", "", "").unwrap(),
        "Toggled allowcontextset to on"
    );
    assert!(g.log.iter().any(|s| s == "allow_context=true"));
}

#[test]
fn test_flow_flag_options() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "ignoreunimplemented", "on", "", "").unwrap(),
        "Unimplemented instructions are now ignored (treated as nop)"
    );
    assert_ne!(g.flowoptions & flow_flags::ignore_unimplemented, 0);
    assert_eq!(
        apply_named(&mut g, "ignoreunimplemented", "off", "", "").unwrap(),
        "Unimplemented instructions now generate warnings"
    );
    assert_eq!(g.flowoptions & flow_flags::ignore_unimplemented, 0);

    assert_eq!(
        apply_named(&mut g, "errorunimplemented", "on", "", "").unwrap(),
        "Unimplemented instructions are now a fatal error"
    );
    assert_ne!(g.flowoptions & flow_flags::error_unimplemented, 0);

    assert_eq!(
        apply_named(&mut g, "errorreinterpreted", "on", "", "").unwrap(),
        "Instruction reinterpretation is now a fatal error"
    );
    assert_ne!(g.flowoptions & flow_flags::error_reinterpreted, 0);

    assert_eq!(
        apply_named(&mut g, "errortoomanyinstructions", "off", "", "").unwrap(),
        "Too many instructions are now NOT a fatal error"
    );

    assert_eq!(
        apply_named(&mut g, "jumpload", "on", "", "").unwrap(),
        "Jumptable analysis will record loads required to calculate jump address"
    );
    assert_ne!(g.flowoptions & flow_flags::record_jumploads, 0);
}

// ---------------------------------------------------------------------------
// protoeval / defaultprototype / setlanguage
// ---------------------------------------------------------------------------

#[test]
fn test_protoeval() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "protoeval", "default", "", "").unwrap(),
        "Set current evaluation to default"
    );
    let e = apply_named(&mut g, "protoeval", "", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify prototype model");
    let mut g = RecordingContext { unknown_model: true, ..Default::default() };
    let e = apply_named(&mut g, "protoeval", "weird", "", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown prototype model: weird");
}

#[test]
fn test_defaultprototype() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "defaultprototype", "__stdcall", "", "").unwrap(),
        "Set default prototype to __stdcall"
    );
    let mut g = RecordingContext { unknown_model: true, ..Default::default() };
    let e = apply_named(&mut g, "defaultprototype", "weird", "", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown prototype model :weird");
}

#[test]
fn test_setlanguage() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "setlanguage", "c-language", "", "").unwrap(),
        "Decompiler produces c-language"
    );
    assert!(g.log.iter().any(|s| s == "language=c-language"));
}

// ---------------------------------------------------------------------------
// jumptablemax / maxinstruction / aliasblock / namespacestrategy
// ---------------------------------------------------------------------------

#[test]
fn test_jumptablemax() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "jumptablemax", "1024", "", "").unwrap(),
        "Maximum jumptable size set to 1024"
    );
    assert_eq!(g.max_jumptable_size, 1024);
    // sentinel 0 => ParseError.
    let e = apply_named(&mut g, "jumptablemax", "0", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify integer maximum");
    let e = apply_named(&mut g, "jumptablemax", "lots", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify integer maximum");
}

#[test]
fn test_maxinstruction() {
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "maxinstruction", "100000", "", "").unwrap(),
        "Maximum instructions per function set"
    );
    assert_eq!(g.max_instructions, 100000);
    // empty p1 => ParseError (number-of-instructions).
    let e = apply_named(&mut g, "maxinstruction", "", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify number of instructions");
    // negative => ParseError (bad maxinstruction).
    let e = apply_named(&mut g, "maxinstruction", "-5", "", "").unwrap_err();
    assert_eq!(e.explain(), "Bad maxinstruction parameter");
}

#[test]
fn test_aliasblock() {
    let mut g = RecordingContext { alias_block_level: 2, ..Default::default() };
    // changing 2 -> 0.
    assert_eq!(
        apply_named(&mut g, "aliasblock", "none", "", "").unwrap(),
        "Alias block level set to none"
    );
    assert_eq!(g.alias_block_level, 0);
    // unchanged (already 0 == "none").
    assert_eq!(
        apply_named(&mut g, "aliasblock", "none", "", "").unwrap(),
        "Alias block level unchanged"
    );
    // each level.
    assert_eq!(
        apply_named(&mut g, "aliasblock", "all", "", "").unwrap(),
        "Alias block level set to all"
    );
    assert_eq!(g.alias_block_level, 3);
    // empty / unknown.
    let e = apply_named(&mut g, "aliasblock", "", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify alias block level");
    let e = apply_named(&mut g, "aliasblock", "weird", "", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown alias block level: weird");
}

#[test]
fn test_namespacestrategy() {
    let mut g = RecordingContext::c_lang();
    assert_eq!(
        apply_named(&mut g, "namespacestrategy", "all", "", "").unwrap(),
        "Namespace strategy set"
    );
    assert!(g.log.iter().any(|s| s == "namespace=All"));
    assert_eq!(
        apply_named(&mut g, "namespacestrategy", "none", "", "").unwrap(),
        "Namespace strategy set"
    );
    let e = apply_named(&mut g, "namespacestrategy", "weird", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify a valid strategy");
}

// ---------------------------------------------------------------------------
// togglerule
// ---------------------------------------------------------------------------

#[test]
fn test_togglerule() {
    // enable success.
    let mut g = RecordingContext { has_current_action: true, rule_present: true, ..Default::default() };
    assert_eq!(
        apply_named(&mut g, "togglerule", "decompile.rulepath", "on", "").unwrap(),
        "Successfully enabled rule"
    );
    // disable success.
    assert_eq!(
        apply_named(&mut g, "togglerule", "decompile.rulepath", "off", "").unwrap(),
        "Successfully disabled rule"
    );
    // enable failure (rule not present).
    let mut g = RecordingContext { has_current_action: true, rule_present: false, ..Default::default() };
    assert_eq!(
        apply_named(&mut g, "togglerule", "decompile.bad", "on", "").unwrap(),
        "Failed to enable rule"
    );
    assert_eq!(
        apply_named(&mut g, "togglerule", "decompile.bad", "off", "").unwrap(),
        "Failed to disable rule"
    );
    // missing args.
    let e = apply_named(&mut g, "togglerule", "", "on", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify rule path");
    let e = apply_named(&mut g, "togglerule", "path", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify on/off");
    // no current action.
    let mut g = RecordingContext { has_current_action: false, ..Default::default() };
    let e = apply_named(&mut g, "togglerule", "path", "on", "").unwrap_err();
    assert_eq!(e.explain(), "Missing current action");
}

// ---------------------------------------------------------------------------
// splitdatatype
// ---------------------------------------------------------------------------

#[test]
fn test_splitdatatype_bits() {
    assert_eq!(OptionSplitDatatypes::get_option_bit("").unwrap(), 0);
    assert_eq!(
        OptionSplitDatatypes::get_option_bit("struct").unwrap(),
        split_datatype::OPTION_STRUCT
    );
    assert_eq!(
        OptionSplitDatatypes::get_option_bit("array").unwrap(),
        split_datatype::OPTION_ARRAY
    );
    assert_eq!(
        OptionSplitDatatypes::get_option_bit("pointer").unwrap(),
        split_datatype::OPTION_POINTER
    );
    let e = OptionSplitDatatypes::get_option_bit("bogus").unwrap_err();
    assert_eq!(e.explain(), "Unknown data-type split option: bogus");
}

#[test]
fn test_splitdatatype_apply() {
    let mut g = RecordingContext { current_action: "decompile".into(), ..Default::default() };
    // struct+pointer => splitcopy on, splitpointer on.
    assert_eq!(
        apply_named(&mut g, "splitdatatype", "struct", "pointer", "").unwrap(),
        "Split data-type configuration set"
    );
    assert!(g.log.iter().any(|s| s == "toggle[decompile.splitcopy]=true"));
    assert!(g.log.iter().any(|s| s == "toggle[decompile.splitpointer]=true"));
    // re-apply same config => "unchanged".
    assert_eq!(
        apply_named(&mut g, "splitdatatype", "struct", "pointer", "").unwrap(),
        "Split data-type configuration unchanged"
    );
    // empty (config 0, no struct/array bit) => splitcopy/splitpointer off.
    let mut g = RecordingContext { current_action: "decompile".into(), ..Default::default() };
    assert_eq!(
        apply_named(&mut g, "splitdatatype", "", "", "").unwrap(),
        "Split data-type configuration unchanged"
    );
    assert!(g.log.iter().any(|s| s == "toggle[decompile.splitcopy]=false"));
}

// ---------------------------------------------------------------------------
// nanignore
// ---------------------------------------------------------------------------

#[test]
fn test_nanignore() {
    let mut g = RecordingContext { has_current_action: true, ..Default::default() };
    assert_eq!(
        apply_named(&mut g, "nanignore", "all", "", "").unwrap(),
        "Nan ignore configuration set to: all"
    );
    assert!(g.nan_ignore_all);
    assert!(g.nan_ignore_compare);
    assert!(g.log.iter().any(|s| s == "enable ignorenan"));
    // compare-only.
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "nanignore", "compare", "", "").unwrap(),
        "Nan ignore configuration set to: compare"
    );
    assert!(!g.nan_ignore_all);
    assert!(g.nan_ignore_compare);
    // none => disable rule, and if already none => unchanged.
    let mut g = RecordingContext::default();
    assert_eq!(
        apply_named(&mut g, "nanignore", "none", "", "").unwrap(),
        "NaN ignore configuration unchanged"
    );
    assert!(g.log.iter().any(|s| s == "disable ignorenan"));
    // unknown.
    let e = apply_named(&mut g, "nanignore", "weird", "", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown nanignore option: weird");
}

// ---------------------------------------------------------------------------
// OptionDatabase::set unknown id
// ---------------------------------------------------------------------------

#[test]
fn test_set_unknown_option() {
    let db = OptionDatabase::new();
    let mut g = RecordingContext::default();
    // an id never registered.
    let e = db.set(&mut g, 9_999_999, "", "", "").unwrap_err();
    assert_eq!(e.explain(), "Unknown option");
    assert!(matches!(e, KunaError::Parse { .. }));
}

// ---------------------------------------------------------------------------
// register_option (kuna-option plug-in door)
// ---------------------------------------------------------------------------

/// A stand-in kuna ArchOption to verify the registration door routes a custom
/// element id to a custom apply().  (Mirrors how kuna_*.rs modules will plug in.)
struct DummyKunaOption;
impl ArchOption for DummyKunaOption {
    fn get_name(&self) -> &str {
        "dummykuna"
    }
    fn apply(
        &self,
        _glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        Ok(format!("dummy got {p1}"))
    }
}

#[test]
fn test_register_custom_option() {
    let mut db = OptionDatabase::new();
    let elem = ElementId::new("dummykuna", 4099);
    db.register_option(elem, Box::new(DummyKunaOption));
    let mut g = RecordingContext::default();
    assert_eq!(
        db.set(&mut g, elem.get_id(), "hello", "", "").unwrap(),
        "dummy got hello"
    );
}

// ---------------------------------------------------------------------------
// XML decode: <optionslist> with param children and bare-content forms.
// ---------------------------------------------------------------------------

#[test]
fn test_decode_optionslist() {
    // An optionslist with:
    //   - <readonly> with bare content "on"  (no <param1>: content is param1)
    //   - <jumptablemax><param1>512</param1></jumptablemax>
    //   - <extrapop><param1>8</param1><param2>myfunc</param2></extrapop>
    let xml = br#"<optionslist>
  <readonly>on</readonly>
  <jumptablemax><param1>512</param1></jumptablemax>
  <extrapop><param1>8</param1><param2>myfunc</param2></extrapop>
</optionslist>"#;

    let manager = AddrSpaceManager::new();
    let mut registry = IdRegistry::with_base_ids();
    register_option_elements(&mut registry);

    let mut dec = XmlDecode::new(&manager, &registry);
    dec.ingest_stream(xml).unwrap();

    let db = OptionDatabase::new();
    let mut g = RecordingContext::default();
    db.decode(&mut dec, &mut g).unwrap();

    // readonly on
    assert!(g.readonly_propagate);
    // jumptablemax 512
    assert_eq!(g.max_jumptable_size, 512);
    // extrapop 8 for function myfunc
    assert!(g.log.iter().any(|s| s == "fn_extrapop[myfunc]=8"));
}

#[test]
fn test_decode_unknown_option_in_list() {
    // An <optionslist> child whose name has no registered element id resolves to
    // ELEM_UNKNOWN; set() then throws "Unknown option" (the C++ behavior).
    let xml = br#"<optionslist><notarealoption>x</notarealoption></optionslist>"#;
    let manager = AddrSpaceManager::new();
    let mut registry = IdRegistry::with_base_ids();
    register_option_elements(&mut registry);
    let mut dec = XmlDecode::new(&manager, &registry);
    dec.ingest_stream(xml).unwrap();
    let db = OptionDatabase::new();
    let mut g = RecordingContext::default();
    let e = db.decode(&mut dec, &mut g).unwrap_err();
    assert_eq!(e.explain(), "Unknown option");
}

#[test]
fn test_decode_lenient_skips_unknown_and_applies_known() {
    // (kuna, ghidra-mode) The lenient <optionslist> decode: unknown elements are
    // skipped with a returned "Warning" line while every known option still
    // applies — the documented setOptions divergence (docs/ghidra-integration.md
    // paragraph 8; upstream fails the whole list and Java then fails the program
    // open).
    let xml = br#"<optionslist>
        <maxinstruction>77777</maxinstruction>
        <notarealoption>x</notarealoption>
        <jumptablemax>512</jumptablemax>
    </optionslist>"#;
    let manager = AddrSpaceManager::new();
    let mut registry = IdRegistry::with_base_ids();
    register_option_elements(&mut registry);
    let mut dec = XmlDecode::new(&manager, &registry);
    dec.ingest_stream(xml).unwrap();
    let db = OptionDatabase::new();
    let mut g = RecordingContext::default();
    let warnings = db.decode_lenient(&mut dec, &mut g).unwrap();
    // Both known options applied despite the unknown element between them.
    assert_eq!(g.max_instructions, 77777);
    assert_eq!(g.max_jumptable_size, 512);
    // Exactly one warning, phrased to pass Java's isErrorMessage "warning" test.
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].starts_with("Warning"), "{}", warnings[0]);
}

/// `UPSTREAM_OPTION_ELEMENTS` is what `kuna-cli` validates a `--option NAME`
/// against before it spawns a run, while `OptionDatabase::new` is what actually
/// dispatches the name; a name in one and not the other is either a silently
/// swallowed option or a good name the CLI refuses, so the two lists must agree.
#[test]
fn the_element_list_and_the_dispatch_map_register_the_same_options() {
    let db = OptionDatabase::new();
    let dispatched: BTreeMap<uint4, &str> =
        db.optionmap.iter().map(|(id, opt)| (*id, opt.get_name())).collect();
    let listed: BTreeMap<uint4, &str> =
        UPSTREAM_OPTION_ELEMENTS.iter().map(|e| (e.get_id(), e.get_name())).collect();
    assert_eq!(dispatched, listed);
}
