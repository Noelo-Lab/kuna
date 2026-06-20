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
    split_datatype, ArchOption, ArchOptionContext, NamespaceStrategy, OptionDatabase,
    OptionExtraPop, OptionJumpTableMax, OptionMaxInstruction, OptionMaxLineWidth,
    OptionSplitDatatypes, BraceCategory, BraceStyle,
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

// ---------------------------------------------------------------------------
// ROUND 2 — F1-fix boundary regression guards (these MUST PASS: they prove the
// new target-width saturation is faithful at the exact i32::MIN / unsigned
// negate-modulo / >u64 edges, verified against a C++11 `istringstream >> T`
// oracle).
// ---------------------------------------------------------------------------

/// The exact negative `int4` boundary.  C++ oracle: `"-2147483648">>int =
/// -2147483648 fail=0` (parses exactly, NO failbit), `"-2147483649">>int =
/// -2147483648 fail=1` (saturates to INT_MIN).  `maxinstruction` rejects both
/// (`newMax < 0`), so we observe the value through `extrapop` instead (sentinel
/// -300; neither boundary equals it).
#[test]
fn extrapop_int_min_boundary_exact_then_saturates() {
    // |i32::MIN| parses exactly -> stored value is i32::MIN, not the sentinel.
    let mut g = Ctx::default();
    OptionExtraPop
        .apply(&mut g, "-2147483648", "", "")
        .expect("-2147483648 is exactly representable, must parse");
    assert_eq!(
        g.default_extra_pop,
        int4::MIN,
        "C++ stores INT_MIN for `-2147483648`; port stored {}",
        g.default_extra_pop
    );
    // one past the boundary saturates to INT_MIN (failbit) — still not -300.
    let mut g2 = Ctx::default();
    OptionExtraPop
        .apply(&mut g2, "-2147483649", "", "")
        .expect("-2147483649 saturates to INT_MIN (not the -300 sentinel)");
    assert_eq!(
        g2.default_extra_pop,
        int4::MIN,
        "C++ saturates `-2147483649` to INT_MIN; port stored {}",
        g2.default_extra_pop
    );
}

/// Unsigned negate-modulo quirk at the in-range boundary.  C++ oracle:
/// `"-4294967295">>unsigned = 1 fail=0` (in-range magnitude negated mod 2^32,
/// NO failbit) but `"-0x100000000">>unsigned = 4294967295 fail=1` (magnitude
/// overflows u32 before the negate, saturates).  `jumptablemax` accepts both
/// (non-zero).
#[test]
fn jumptablemax_unsigned_negate_modulo_vs_overflow() {
    // -4294967295: magnitude 4294967295 is in range, negate mod 2^32 == 1.
    let mut g = Ctx::default();
    OptionJumpTableMax
        .apply(&mut g, "-4294967295", "", "")
        .expect("magnitude in range -> negate-modulo == 1, accepted");
    assert_eq!(
        g.max_jumptable_size, 1,
        "C++ `-4294967295`>>unsigned == 1; port stored {}",
        g.max_jumptable_size
    );
    // -0x100000000: magnitude 2^32 overflows u32 -> saturates to UINT_MAX.
    let mut g2 = Ctx::default();
    OptionJumpTableMax
        .apply(&mut g2, "-0x100000000", "", "")
        .expect("magnitude overflow -> UINT_MAX, non-zero, accepted");
    assert_eq!(
        g2.max_jumptable_size,
        uint4::MAX,
        "C++ `-0x100000000`>>unsigned saturates to UINT_MAX; port stored {}",
        g2.max_jumptable_size
    );
}

/// A magnitude above u64 (the original F1 saturate path) still works after the
/// width-saturation rework.  C++ oracle: `"99999999999999999999">>int =
/// 2147483647 fail=1` (INT_MAX) and `>>unsigned = 4294967295`.
#[test]
fn above_u64_still_saturates_after_rework() {
    let mut g = Ctx::default();
    OptionMaxInstruction
        .apply(&mut g, "99999999999999999999", "", "")
        .expect("u64-overflow magnitude saturates to INT_MAX (>=0), accepted");
    assert_eq!(g.max_instructions, int4::MAX);

    let mut g2 = Ctx::default();
    OptionJumpTableMax
        .apply(&mut g2, "99999999999999999999", "", "")
        .expect("u64-overflow magnitude saturates to UINT_MAX (!=0), accepted");
    assert_eq!(g2.max_jumptable_size, uint4::MAX);
}

// ---------------------------------------------------------------------------
// ROUND 2 — F3 (NEW): C++11 `num_get` stores 0 (NOT the sentinel) on a failed
// extraction whose input is non-empty but has no leading digit (`"abc"`).  The
// port's `parse_int_auto` returns `None` for such input, so the caller falls
// back to the sentinel and the option ERRORS where C++ ACCEPTS with value 0.
//
// C++ oracle (g++ and clang++, -std=c++11):
//   int    val=-1;   iss("abc") >> val;   => val = 0   fail=1   (overwrites -1)
//   int    expop=-300; iss("abc") >> ...; => expop = 0 fail=1   (overwrites -300)
//   int    val=-1;   iss("")    >> val;   => val = -1  fail=1   (empty: unchanged)
//   unsigned val=0;  iss("abc") >> val;   => val = 0   fail=1
// So `maxlinewidth "abc"` / `maxinstruction "abc"` / `extrapop "abc"` all
// ACCEPT in C++ (value 0, which is >=0 / not the -300 sentinel), but the port
// returns Err.  These tests document the divergence (currently FAILING).
// ---------------------------------------------------------------------------

/// `maxlinewidth "abc"`: C++ stores 0 (`val==-1` false) and ACCEPTS; the port
/// returns `Err("Must specify integer linewidth")`.
#[test]
fn maxlinewidth_nonnumeric_input_accepts_as_zero_like_cpp() {
    let mut g = Ctx { is_c_language: true, ..Default::default() };
    let res = OptionMaxLineWidth.apply(&mut g, "abc", "", "");
    assert!(
        res.is_ok(),
        "C++ `maxlinewidth abc` stores 0 (num_get failbit-with-zero) and accepts; \
         port returned {res:?}"
    );
    assert!(
        g.log.iter().any(|s| *s == "max_line=0"),
        "C++ hands 0 to setMaxLineSize on non-numeric input; port log={:?}",
        g.log
    );
}

/// `maxinstruction "abc"`: C++ stores 0 (`newMax<0` false) and ACCEPTS; the port
/// returns `Err("Bad maxinstruction parameter")`.
#[test]
fn maxinstruction_nonnumeric_input_accepts_as_zero_like_cpp() {
    let mut g = Ctx::default();
    let res = OptionMaxInstruction.apply(&mut g, "notanum", "", "");
    assert!(
        res.is_ok(),
        "C++ `maxinstruction notanum` stores 0 (>=0) and accepts; port returned {res:?}"
    );
    assert_eq!(
        g.max_instructions, 0,
        "C++ stores 0 on non-numeric input; port stored {}",
        g.max_instructions
    );
}

/// `extrapop "abc"`: C++ stores 0 (`expop==-300` false) and ACCEPTS; the port
/// returns `Err("Bad extrapop adjustment parameter")`.
#[test]
fn extrapop_nonnumeric_input_accepts_as_zero_like_cpp() {
    let mut g = Ctx::default();
    let res = OptionExtraPop.apply(&mut g, "abc", "", "");
    assert!(
        res.is_ok(),
        "C++ `extrapop abc` stores 0 (not the -300 sentinel) and accepts; \
         port returned {res:?}"
    );
    assert_eq!(
        g.default_extra_pop, 0,
        "C++ stores 0 on non-numeric input; port stored {}",
        g.default_extra_pop
    );
}

/// Control: a truly EMPTY string `""` DOES leave the sentinel unchanged in C++11
/// (no characters extracted), so `maxlinewidth ""` errors in BOTH — this is the
/// case the port's model is correct for, and isolates F3 to the non-empty case.
#[test]
fn maxlinewidth_empty_input_errors_in_both() {
    let mut g = Ctx { is_c_language: true, ..Default::default() };
    let res = OptionMaxLineWidth.apply(&mut g, "", "", "");
    assert!(
        res.is_err(),
        "C++ `maxlinewidth \"\"` leaves val==-1 (empty extraction) and errors; \
         port must also error"
    );
    assert_eq!(res.unwrap_err().explain(), "Must specify integer linewidth");
}

// ===========================================================================
// ROUND 3 (independent verifier) — new adversarial coverage.
//
// The round-3 repair (f8c688f) closed F3 by distinguishing the empty/
// whitespace-only field (target unchanged -> sentinel survives -> reject) from
// the non-empty no-leading-digit field (`num_get` stores 0 -> accept).  These
// tests pin the SENTINEL-INTERPLAY edges that the F3 fix touches but that were
// not yet covered, plus the octal/hex base auto-detection, plus the F2
// split-datatype partial-state divergence.  Every expected value is from a
// standalone `istringstream >> int/unsigned` C++11 oracle (g++ 11 + clang++).
// ===========================================================================

/// `extrapop` sentinel interplay — the F3 fix must NOT disturb the four
/// distinct num_get outcomes the `-300` sentinel rides on.  C++ oracle:
///   - `"-300"`  -> expop = -300 (digits parsed, no failbit) -> `== -300` TRUE
///                  -> throw "Bad extrapop adjustment parameter"
///   - `""`      -> expop unchanged at -300 (empty extraction)  -> throw
///   - `"0"`     -> expop = 0 (digits parsed) -> `!= -300` -> ACCEPT, stores 0
///   - `"abc"`   -> expop = 0 (failbit-with-zero) -> `!= -300` -> ACCEPT, 0
/// The empty case (sentinel survives -> error) and the non-numeric case
/// (stored 0 -> accept) are the two halves the round-3 fix separates; `"-300"`
/// proves a *legitimately parsed* -300 still trips the sentinel exactly as C++.
#[test]
fn extrapop_minus_300_and_empty_throw_zero_and_abc_accept() {
    // "-300": digits parse to exactly the sentinel value -> C++ throws.
    let mut g = Ctx::default();
    let e = OptionExtraPop.apply(&mut g, "-300", "", "").unwrap_err();
    assert_eq!(
        e.explain(),
        "Bad extrapop adjustment parameter",
        "C++ parses -300 (no failbit) and the `expop==-300` sentinel fires"
    );
    // "": empty extraction leaves expop at the -300 sentinel -> throw.
    let mut g = Ctx::default();
    let e = OptionExtraPop.apply(&mut g, "", "", "").unwrap_err();
    assert_eq!(
        e.explain(),
        "Bad extrapop adjustment parameter",
        "C++ empty extraction leaves the sentinel -> throw (NOT a stored 0)"
    );
    // "0": digits parse to 0 -> not the sentinel -> accept, stores 0.
    let mut g = Ctx::default();
    OptionExtraPop
        .apply(&mut g, "0", "", "")
        .expect("C++ parses 0, not -300 -> accept");
    assert_eq!(g.default_extra_pop, 0);
    // "abc": failbit-with-zero -> 0 -> not the sentinel -> accept, stores 0.
    let mut g = Ctx::default();
    OptionExtraPop
        .apply(&mut g, "abc", "", "")
        .expect("C++ num_get stores 0 on non-numeric -> not -300 -> accept");
    assert_eq!(g.default_extra_pop, 0);
}

/// Integer base auto-detection (the `unsetf(dec|hex|oct)` path) at the
/// boundary digits, observed through `maxinstruction` (`int4`, `newMax<0`
/// reject).  C++ `istringstream >> int` oracle:
///   "08"   -> 0   (octal: '0' parsed, '8' >= base 8 stops; failbit NOT set,
///                  digits were extracted)
///   "019"  -> 1   (octal: 0,1 parsed, '9' stops -> 01 octal = 1)
///   "0x10" -> 16  (hex)
///   "0xG"  -> 0   (the byte after 0x is not a hex digit: subject seq is "0")
///   "0x"   -> 0   (no hex digit at all: subject seq is "0")
/// All are >= 0, so `maxinstruction` ACCEPTS each and stores the parsed value.
#[test]
fn maxinstruction_octal_and_hex_base_edges_match_cpp() {
    for (input, want) in [
        ("08", 0i32),
        ("019", 1),
        ("0x10", 16),
        ("0xG", 0),
        ("0x", 0),
        ("0177", 127),
        ("00", 0),
    ] {
        let mut g = Ctx::default();
        OptionMaxInstruction
            .apply(&mut g, input, "", "")
            .unwrap_or_else(|e| panic!("maxinstruction {input:?} should accept: {}", e.explain()));
        assert_eq!(
            g.max_instructions, want,
            "C++ `>> int` of {input:?} == {want}; port stored {}",
            g.max_instructions
        );
    }
}

/// `jumptablemax` `uint4` sentinel is `val == 0`, and the F3 fix routes a
/// non-numeric input to a stored 0 — which for *this* option collides with the
/// sentinel and therefore REJECTS (unlike maxinstruction/extrapop which accept
/// 0).  C++ oracle: `unsigned v=0; iss("abc")>>v => v=0 fail=1`; `OptionJump-
/// TableMax` then sees `val==0` and throws.  An empty input also reaches v=0
/// (the caller's `.unwrap_or(0)` default == the num_get-unchanged 0), so both
/// "abc" and "" reject here — the same observable outcome in C++ and the port.
#[test]
fn jumptablemax_zero_and_nonnumeric_both_reject_via_zero_sentinel() {
    for input in ["abc", "", "0", "00", "0x0"] {
        let mut g = Ctx::default();
        let e = OptionJumpTableMax.apply(&mut g, input, "", "").unwrap_err();
        assert_eq!(
            e.explain(),
            "Must specify integer maximum",
            "C++ jumptablemax {input:?} -> stored/default 0 -> `val==0` sentinel -> throw"
        );
    }
    // a real value still gets through and is stored.
    let mut g = Ctx::default();
    OptionJumpTableMax.apply(&mut g, "5", "", "").expect("5 != 0");
    assert_eq!(g.max_jumptable_size, 5);
}

/// F2 (LOSS-047 in the verdict): `OptionSplitDatatypes::apply` partial-state on
/// a malformed p2.  C++ writes `glb->split_datatype_config = getOptionBit(p1)`
/// BEFORE `getOptionBit(p2)` can throw, so a bad p2 leaves the field at
/// `bit(p1)`; the port accumulates into a local and only writes glb after all
/// three parse, so on the same input glb stays at `old_config`.
///
/// This documents the divergence (kept as an either/or assertion so the test
/// stays green): the C++ leaves `option_struct` (=1) behind, the port leaves
/// the prior value (here 0).  The error itself (`Unknown data-type split
/// option: garbage`) is identical; only the surviving field differs.
#[test]
fn splitdatatype_bad_p2_partial_state_divergence_documented() {
    let opt = OptionSplitDatatypes;
    let mut g = Ctx {
        split_datatype_config: 0,
        ..Default::default()
    };
    // p1="struct" (valid, bit 1), p2="garbage" (throws in getOptionBit).
    let e = opt.apply(&mut g, "struct", "garbage", "").unwrap_err();
    assert_eq!(
        e.explain(),
        "Unknown data-type split option: garbage",
        "the error text must match C++ getOptionBit on a bad option"
    );
    let cpp_partial = split_datatype::OPTION_STRUCT; // C++ leaves bit(p1) behind
    let port_value = g.split_datatype_config; // port leaves old_config (0)
    assert!(
        port_value == 0 || port_value == cpp_partial,
        "either the documented divergence (port=0) or a future fix (=bit(p1)=1); got {port_value}"
    );
    // Pin the *current* divergent behavior so a regression/fix is visible:
    assert_eq!(
        port_value, 0,
        "port currently leaves split_datatype_config at old_config (0) on a bad p2; \
         C++ leaves option_struct (1) — LOSS-047. If a fix lands, update this assertion."
    );
}
