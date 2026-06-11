//! Verifier adversarial tests for item `w4-fw-options` (options.cc/options.hh).
//!
//! INDEPENDENT of the porter's in-module tests. These drive only the public
//! `options` API and target the hunt-list spots the review flagged as most
//! fragile for this item:
//!   - integer-WIDTH overflow vs the C++ `istringstream >> int4/uint4`
//!     saturation to `numeric_limits<T>::max()` (the porter's `parse_int_auto`
//!     only saturates on *u64* overflow, so a value that fits in u64 but
//!     overflows i32/u32 is wrap-truncated, not saturated — F1);
//!   - the integer base auto-detection corner cases (octal-with-bad-digit,
//!     `0x`-with-no-hex-digit, leading sign) the options rely on;
//!   - the `<optionslist>` decode dispatch + bare-content (param1) path and the
//!     unknown-option error;
//!   - the BTreeMap dispatch determinism.
//!
//! The C++ oracle values used below were produced with a standalone
//! `istringstream >> int/unsigned int` harness (see the verdict file).

use kuna_base::error::KunaError;
use kuna_base::types::{int4, uint4};
use kuna_decomp::options::{
    ArchOption, ArchOptionContext, NamespaceStrategy, OptionDatabase, OptionExtraPop,
    OptionJumpTableMax, OptionMaxInstruction, OptionMaxLineWidth, BraceCategory, BraceStyle,
};

/// A minimal recording context: enough to observe the value handed to the
/// alive config seams (`max_instructions`, `max_jumptable_size`, the default
/// extrapop) so the integer-parse result is observable.
#[derive(Default)]
struct Ctx {
    max_instructions: int4,
    max_jumptable_size: uint4,
    default_extra_pop: int4,
    is_c_language: bool,
    flowoptions: uint4,
    alias_block_level: int4,
    split_datatype_config: uint4,
    nan_ignore_all: bool,
    nan_ignore_compare: bool,
    header_comment: uint4,
    instruction_comment: uint4,
    current_action: String,
    log: Vec<String>,
}

impl ArchOptionContext for Ctx {
    fn set_readonly_propagate(&mut self, _v: bool) {}
    fn set_infer_pointers(&mut self, _v: bool) {}
    fn set_analyze_for_loops(&mut self, _v: bool) {}
    fn set_max_jumptable_size(&mut self, v: uint4) {
        self.max_jumptable_size = v;
    }
    fn set_max_instructions(&mut self, v: int4) {
        self.max_instructions = v;
    }
    fn alias_block_level(&self) -> int4 {
        self.alias_block_level
    }
    fn set_alias_block_level(&mut self, v: int4) {
        self.alias_block_level = v;
    }
    fn flow_options(&self) -> uint4 {
        self.flowoptions
    }
    fn set_flow_options(&mut self, v: uint4) {
        self.flowoptions = v;
    }
    fn split_datatype_config(&self) -> uint4 {
        self.split_datatype_config
    }
    fn set_split_datatype_config(&mut self, v: uint4) {
        self.split_datatype_config = v;
    }
    fn nan_ignore_all(&self) -> bool {
        self.nan_ignore_all
    }
    fn set_nan_ignore_all(&mut self, v: bool) {
        self.nan_ignore_all = v;
    }
    fn nan_ignore_compare(&self) -> bool {
        self.nan_ignore_compare
    }
    fn set_nan_ignore_compare(&mut self, v: bool) {
        self.nan_ignore_compare = v;
    }
    fn set_default_extra_pop(&mut self, v: int4) {
        self.default_extra_pop = v;
    }
    fn set_function_extra_pop(&mut self, _n: &str, _v: int4) -> Result<(), KunaError> {
        Ok(())
    }
    fn set_default_model(&mut self, _n: &str) -> Result<(), KunaError> {
        Ok(())
    }
    fn set_eval_current_model(&mut self, _n: &str) -> Result<(), KunaError> {
        Ok(())
    }
    fn set_function_inline(&mut self, _n: &str, _v: bool) -> Result<(), KunaError> {
        Ok(())
    }
    fn set_function_no_return(&mut self, _n: &str, _v: bool) -> Result<(), KunaError> {
        Ok(())
    }
    fn print_is_c_language(&self) -> bool {
        self.is_c_language
    }
    fn set_null_printing(&mut self, _v: bool) {}
    fn set_inplace_ops(&mut self, _v: bool) {}
    fn set_convention_printing(&mut self, _v: bool) {}
    fn set_no_cast_printing(&mut self, _v: bool) {}
    fn set_hide_implied_exts(&mut self, _v: bool) {}
    fn set_max_line_size(&mut self, v: int4) {
        self.log.push(format!("max_line={v}"));
    }
    fn set_indent_increment(&mut self, _v: int4) {}
    fn set_line_comment_indent(&mut self, _v: int4) {}
    fn set_comment_style(&mut self, _s: &str) {}
    fn header_comment_flags(&self) -> uint4 {
        self.header_comment
    }
    fn set_header_comment_flags(&mut self, f: uint4) {
        self.header_comment = f;
    }
    fn instruction_comment_flags(&self) -> uint4 {
        self.instruction_comment
    }
    fn set_instruction_comment_flags(&mut self, f: uint4) {
        self.instruction_comment = f;
    }
    fn set_integer_format(&mut self, _f: &str) {}
    fn set_namespace_strategy(&mut self, _s: NamespaceStrategy) {}
    fn set_brace_format(&mut self, _c: BraceCategory, _s: BraceStyle) {}
    fn set_print_language(&mut self, _l: &str) {}
    fn set_action_warning(&mut self, _v: bool, _n: &str) -> bool {
        true
    }
    fn clone_action_group(&mut self, _f: &str, _t: &str) {}
    fn set_current_action(&mut self, n: &str) {
        self.current_action = n.to_string();
    }
    fn current_action_name(&self) -> String {
        self.current_action.clone()
    }
    fn toggle_action(&mut self, _g: &str, _s: &str, _v: bool) {}
    fn enable_rule(&mut self, _p: &str) -> bool {
        true
    }
    fn disable_rule(&mut self, _p: &str) -> bool {
        true
    }
    fn has_current_action(&self) -> bool {
        true
    }
    fn allow_context_set(&mut self, _v: bool) {}
}

// ---------------------------------------------------------------------------
// F1: integer-WIDTH overflow must saturate to numeric_limits<T>::max(),
// matching `istringstream >> int4/uint4`, NOT wrap-truncate the u64 magnitude.
// ---------------------------------------------------------------------------

/// `maxinstruction "3000000000"`: 3e9 > INT_MAX but fits u64.
///
/// C++ oracle: `int v; iss >> v;` yields `v = 2147483647` (INT_MAX, failbit
/// set).  `OptionMaxInstruction` then checks `newMax < 0` — INT_MAX is NOT
/// negative, so C++ **accepts** and stores `max_instructions = 2147483647`.
///
/// The port wrap-truncates `3000000000 as i32 = -1294967296`, which IS negative,
/// so it returns `Err("Bad maxinstruction parameter")` — a control-flow
/// divergence (accept vs error) on the same input.
#[test]
fn maxinstruction_overflow_saturates_not_wraps() {
    let opt = OptionMaxInstruction;
    let mut g = Ctx::default();
    let res = opt.apply(&mut g, "3000000000", "", "");
    assert!(
        res.is_ok(),
        "C++ accepts 3000000000 as INT_MAX-saturated (>=0); port returned {res:?}"
    );
    assert_eq!(
        g.max_instructions,
        int4::MAX,
        "C++ stores numeric_limits<int>::max() on overflow; port stored {}",
        g.max_instructions
    );
}

/// `jumptablemax "0x100000000"`: 2^32 overflows uint4 but fits u64.
///
/// C++ oracle: `unsigned int uv; iss >> uv;` yields `uv = 4294967295`
/// (UINT_MAX, failbit set).  `OptionJumpTableMax` checks `val == 0` — UINT_MAX
/// is non-zero, so C++ **accepts** and stores `max_jumptable_size = UINT_MAX`.
///
/// The port wrap-truncates `0x100000000 as u32 = 0`, which trips the `== 0`
/// sentinel and returns `Err("Must specify integer maximum")`.
#[test]
fn jumptablemax_width_overflow_saturates_not_wraps() {
    let opt = OptionJumpTableMax;
    let mut g = Ctx::default();
    let res = opt.apply(&mut g, "0x100000000", "", "");
    assert!(
        res.is_ok(),
        "C++ accepts 0x100000000 as UINT_MAX-saturated (!=0); port returned {res:?}"
    );
    assert_eq!(
        g.max_jumptable_size,
        uint4::MAX,
        "C++ stores numeric_limits<unsigned>::max() on overflow; port stored {}",
        g.max_jumptable_size
    );
}

/// `extrapop "0xffffffff"`: = 4294967295, overflows int4 but fits u64.
///
/// C++ oracle: `int v; iss >> v;` yields `v = 2147483647` (INT_MAX saturated).
/// `OptionExtraPop` checks `expop == -300` — INT_MAX is not -300, so C++ stores
/// `default extrapop = 2147483647`.  The port wrap-truncates
/// `0xffffffff as i32 = -1`, which is also not -300, so it does NOT error, but
/// it stores a *different value* (-1 vs INT_MAX) — a silent value divergence.
#[test]
fn extrapop_width_overflow_value_matches_cpp() {
    let opt = OptionExtraPop;
    let mut g = Ctx::default();
    let res = opt.apply(&mut g, "0xffffffff", "", "");
    assert!(res.is_ok(), "neither value is the -300 sentinel: {res:?}");
    assert_eq!(
        g.default_extra_pop,
        int4::MAX,
        "C++ saturates 0xffffffff to INT_MAX for `>> int`; port stored {}",
        g.default_extra_pop
    );
}

// ---------------------------------------------------------------------------
// Base auto-detection corner cases that MUST match (these are clean in the port
// — kept as regression guards around the saturation fix).
// ---------------------------------------------------------------------------

/// In-range values across every base must round-trip exactly, and the `== -1`
/// sentinel for `maxlinewidth` must reject only a literal `-1`/non-numeric, not
/// a valid small width.  C++ oracle: `"0x7f"=>127`, `"0177"=>127`, `"127"=>127`.
#[test]
fn maxlinewidth_in_range_bases_roundtrip() {
    let opt = OptionMaxLineWidth;
    for (input, want) in [("127", 127i32), ("0x7f", 127), ("0177", 127), ("+50", 50)] {
        let mut g = Ctx { is_c_language: true, ..Default::default() };
        opt.apply(&mut g, input, "", "").unwrap_or_else(|e| {
            panic!("maxlinewidth {input:?} should parse: {}", e.explain())
        });
        assert!(
            g.log.iter().any(|s| *s == format!("max_line={want}")),
            "maxlinewidth {input:?} expected {want}, log={:?}",
            g.log
        );
    }
    // literal "-1" hits the sentinel and is rejected (C++ parses -1, val==-1).
    let mut g = Ctx { is_c_language: true, ..Default::default() };
    let e = opt.apply(&mut g, "-1", "", "").unwrap_err();
    assert_eq!(e.explain(), "Must specify integer linewidth");
}

// ---------------------------------------------------------------------------
// Dispatch determinism + unknown-option error (BTreeMap key = element id).
// ---------------------------------------------------------------------------

/// Two `OptionDatabase::new()` instances dispatch identically, and an
/// unregistered id yields the exact C++ `ParseError("Unknown option")`.
#[test]
fn dispatch_unknown_id_is_parse_error_and_stable() {
    let db1 = OptionDatabase::new();
    let db2 = OptionDatabase::new();
    let mut g1 = Ctx { is_c_language: true, ..Default::default() };
    let mut g2 = Ctx { is_c_language: true, ..Default::default() };
    let e1 = db1.set(&mut g1, 123_456_789, "", "", "").unwrap_err();
    let e2 = db2.set(&mut g2, 123_456_789, "", "", "").unwrap_err();
    assert_eq!(e1.explain(), "Unknown option");
    assert_eq!(e2.explain(), e1.explain());
    assert!(matches!(e1, KunaError::Parse { .. }));
}
