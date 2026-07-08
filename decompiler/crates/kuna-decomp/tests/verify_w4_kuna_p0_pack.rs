//! Adversarial verification tests for item `w4-kuna-p0-pack` (round 1).
//!
//! Independent verifier tests targeting the hunt-list-flagged fragile spots of
//! the P0 pack port (`kuna_phases`/`kuna_assert`/`kuna_restartlog`):
//!
//!   - the `values` pipe-split's INCLUSIVE-loop port (C++ `while(pos<=vals.size())`),
//!     where a trailing/lone `|` emits an extra empty `""` token — the highest
//!     byte-compat divergence risk in `emit_settable_json`;
//!   - `json_string` escaping (control chars < 0x20 collapse to one space, `\n`
//!     special-cased, `"`/`\\` escaped) and the C++ byte-iteration vs Rust
//!     char-iteration on multibyte input;
//!   - `KunaPhase::from_code` length/range boundaries and byte-index safety on a
//!     multibyte input (must not panic);
//!   - `RestartLog` per-function cap at exactly 32, monotone seq across funcs,
//!     and `(space_index, offset)` key ordering (the transcribed `operator<`);
//!   - `AssertLog::render_list`'s "applied via HARD mechanism" annotation gate.
//!
//! These are derived against the C++ in scope (`kuna_console.cc kunaJsonString`/
//! `kunaEmitSettableJson`, `kuna_stages.cc kunaStageFromCode`,
//! `kuna_restartlog.cc`, `kuna_assert.cc listAssertions`), not the porter notes.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_assert::{AssertLog, KunaAssertion};
use kuna_decomp::kuna_restartlog::{KunaRestartReason, RestartLog};
use kuna_decomp::kuna_phases::{
    emit_settable_json, KunaPhase, KunaStrength, KunaSettable,
};
use kuna_decomp::context::ArchContext;

// A bare settable used to drive the JSON emitter with hand-built `values`.
fn settable_with_values(values: &'static str, shipped: &'static str) -> KunaSettable {
    KunaSettable {
        option: "probe",
        values,
        shipped,
        destructive: false,
        phase: KunaPhase::S3,
        subphase: "simplification-quiescence",
        strength: KunaStrength::Hard,
        rewind: KunaPhase::S3,
        issue: "",
        summary: "",
        use_when: "",
        example: "",
        source_decompiler: "",
        inspiration: "",
        change_kind: "",
    }
}

/// HUNT(off-by-one / inclusive-loop): C++ `kunaEmitSettableJson` walks the
/// pipe-separated values with `while(pos <= vals.size())`, so a string ENDING in
/// `|` emits a trailing empty `""` token, a LONE `|` emits two `""`, and an EMPTY
/// string emits one `""`. The Rust `loop { ... match next { None => break } }`
/// must reproduce each exactly, or the catalog `values` array diverges.
#[test]
fn w4_kuna_p0_pack_values_split_inclusive_loop_edges() {
    // C++: vals="a|", size=2. pos0 -> "a", pos2<=2 -> "" -> ["a",""].
    let mut out = String::new();
    emit_settable_json(&mut out, &settable_with_values("a|", "a"), None);
    assert!(
        out.contains(r#""values": ["a", ""]"#),
        "trailing '|' must emit a trailing empty token; got: {out}"
    );

    // C++: vals="", size=0. pos0<=0 -> "" -> [""].
    let mut out = String::new();
    emit_settable_json(&mut out, &settable_with_values("", ""), None);
    assert!(
        out.contains(r#""values": [""]"#),
        "empty values must emit a single empty token; got: {out}"
    );

    // C++: vals="|", size=1. pos0 -> "" (bar at 0), pos1<=1 -> "" -> ["",""].
    let mut out = String::new();
    emit_settable_json(&mut out, &settable_with_values("|", ""), None);
    assert!(
        out.contains(r#""values": ["", ""]"#),
        "lone '|' must emit two empty tokens; got: {out}"
    );

    // Sanity: the normal two-value case is unaffected.
    let mut out = String::new();
    emit_settable_json(&mut out, &settable_with_values("on|off", "on"), None);
    assert!(
        out.contains(r#""values": ["on", "off"]"#),
        "normal split regressed; got: {out}"
    );
}

/// HUNT(width/escaping): `kunaJsonString` escapes `"` and `\\`, special-cases
/// `\n` to `\n`, collapses every other control char < 0x20 to a single space,
/// and passes everything >= 0x20 through. The Rust port iterates `chars()` where
/// C++ iterates bytes; for multibyte UTF-8 the re-encoded bytes must be identical
/// (every continuation byte is >= 0x80, so neither side touches it).
#[test]
fn w4_kuna_p0_pack_json_string_escaping_and_control_chars() {
    // Build values with quote/backslash via a stable settable summary path: the
    // emitter routes `summary`/`use_when`/... through json_string too. Use a
    // settable whose summary contains the adversarial bytes.
    let st = KunaSettable {
        summary: "q\"b\\s\tt\u{1}c\nL\u{00e9}",
        ..settable_with_values("on|off", "on")
    };
    let mut out = String::new();
    emit_settable_json(&mut out, &st, None);
    // Expected escaping: " -> \" ; \\ -> \\\\ ; \t (0x09, <0x20) -> single space;
    // 0x01 -> single space; \n -> \n ; é (U+00E9) passes through verbatim.
    let expect = r#""summary": "q\"b\\s t c\nLé""#;
    assert!(
        out.contains(expect),
        "json_string escaping diverged.\n got: {out}\n want substring: {expect}"
    );
}

/// HUNT(off-by-one / byte-index safety): `KunaPhase::from_code` requires exactly
/// 2 chars, `P0`/`p0`, or `[sS][1-9]`. The boundary chars `'0'` and `':'` (just
/// outside `'1'..='9'`) must reject; a 1- or 3-char code rejects; and a 2-BYTE
/// but multi-CODEPOINT input (e.g. "é" is 2 bytes) must not panic on byte
/// indexing — C++ indexes `code[0]/code[1]` on the byte string and simply fails
/// the char comparisons.
#[test]
fn w4_kuna_p0_pack_from_code_boundaries_and_byte_safety() {
    // Valid forms.
    assert_eq!(KunaPhase::from_code("P0"), Some(KunaPhase::P0));
    assert_eq!(KunaPhase::from_code("p0"), Some(KunaPhase::P0));
    assert_eq!(KunaPhase::from_code("S1"), Some(KunaPhase::S1));
    assert_eq!(KunaPhase::from_code("s9"), Some(KunaPhase::S9));

    // Range boundaries: '0' and ':' bracket '1'..='9'.
    assert_eq!(KunaPhase::from_code("S0"), None, "S0 below range");
    assert_eq!(KunaPhase::from_code("S:"), None, "S: above '9'");
    // P only pairs with '0'.
    assert_eq!(KunaPhase::from_code("P1"), None);
    assert_eq!(KunaPhase::from_code("P9"), None);

    // Length boundaries.
    assert_eq!(KunaPhase::from_code("S"), None);
    assert_eq!(KunaPhase::from_code("S12"), None);
    assert_eq!(KunaPhase::from_code(""), None);

    // Multibyte: "é" is exactly 2 bytes in UTF-8 but len()==2; the C++ would
    // read two bytes and fail every comparison. Rust must not panic on
    // `as_bytes()[0/1]` and must return None.
    assert_eq!(KunaPhase::from_code("é"), None, "2-byte multibyte must not panic");
    // A single non-ascii char (1 codepoint, 2 bytes) -> len 2 -> compared as bytes.
    assert_eq!(KunaPhase::from_code("S\u{0080}"), None);
}

/// HUNT(off-by-one / iteration order): the `RestartLog` per-function ring cap is
/// `>= 32` (C++ `(int4)events.size() >= MAX_EVENTS_PER_FUNC`), the seq counter is
/// monotone ACROSS functions (C++ file-static `restartSeq++` shared by the whole
/// table), and the dump numbers events 0..=N-1 in INSERTION order. Verify the
/// 32nd record (index 31) is the last kept and the 33rd is dropped, while a
/// second function still records (proving the cap is per-key, not global).
#[test]
fn w4_kuna_p0_pack_restartlog_cap_is_exactly_32_per_function() {
    let fd_a = build_fd("alpha", 0x1000);
    let fd_b = build_fd("beta", 0x2000);
    let mut log = RestartLog::new();

    // 33 records into A: indices 0..=31 kept, the 33rd dropped.
    for i in 0..33 {
        let detail = format!("a{i}");
        log.record(&fd_a, KunaRestartReason::DeadcodeBump, &detail);
    }
    // B still records after A is full (per-function cap, not global).
    log.record(&fd_b, KunaRestartReason::ProtoForced, "b0");

    let out_a = log.render(&fd_a);
    assert!(out_a.contains("\n  31: "), "index 31 must be retained");
    assert!(!out_a.contains("\n  32: "), "index 32 must be dropped at the cap");
    // The dropped record's detail (a32) must not appear; the kept boundary (a31) must.
    assert!(out_a.contains("[a31]"), "the 32nd kept record's detail present");
    assert!(!out_a.contains("[a32]"), "the dropped record's detail absent");

    let out_b = log.render(&fd_b);
    assert!(
        out_b.contains("  0: prototype forced late at call site (S4, edge 5)  [b0]\n"),
        "second function records despite A being capped; got: {out_b}"
    );
}

/// HUNT(comparator totality / key ordering): `KunaFuncKey`'s derived `Ord` is the
/// transcribed C++ `operator<` minus the owner-implied arch pointer: space_index
/// (i32) then offset (u64). Two functions in the same space sort by offset; the
/// dump for each reads only its own bucket in insertion order, so distinct keys
/// never cross-contaminate regardless of insertion interleaving.
#[test]
fn w4_kuna_p0_pack_restartlog_keys_do_not_collide_interleaved() {
    let fd_lo = build_fd("lo", 0x10);
    let fd_hi = build_fd("hi", 0x20);
    let mut log = RestartLog::new();
    // Interleave records across the two keys.
    log.record(&fd_hi, KunaRestartReason::MultistageJump, "hi-0");
    log.record(&fd_lo, KunaRestartReason::DeadcodeBump, "lo-0");
    log.record(&fd_hi, KunaRestartReason::ProtoDeindirect, "hi-1");

    let out_lo = log.render(&fd_lo);
    let out_hi = log.render(&fd_hi);
    // lo has exactly one event at index 0; hi has two at 0,1 in insertion order.
    assert!(out_lo.contains("  0: deadcode-delay bump (S3 dead-definition gate, edge 3)  [lo-0]\n"));
    assert!(!out_lo.contains("  1: "), "lo bucket must hold only its own event");
    assert!(out_hi.contains("  0: multistage jump-table hint (S2 switch model, edge 2)  [hi-0]\n"));
    assert!(out_hi.contains("  1: prototype discovered late at indirect call (S4, edge 5)  [hi-1]\n"));
}

/// HUNT(exception->Result / conditional rendering): `IfcKunaAssert::listAssertions`
/// prints " (applied via HARD mechanism)" ONLY when the recorded strength is HINT
/// and the applied mechanism is HARD. Every other (strength, applied) pairing must
/// omit it; the "-" strength label is reserved for `kstrength_none`.
#[test]
fn w4_kuna_p0_pack_assertlog_hint_via_hard_annotation_gate() {
    let mk = |strength, applied| KunaAssertion {
        func_name: "(global)".into(),
        phase: KunaPhase::S3,
        subphase: "simplification-quiescence".into(),
        args: String::new(),
        strength,
        applied,
        rewind: KunaPhase::S3,
    };

    // HINT requested, HARD applied -> annotation present.
    let mut log = AssertLog::new();
    log.push(mk(KunaStrength::Hint, KunaStrength::Hard));
    let out = log.render_list();
    assert!(out.contains("strength=HINT (applied via HARD mechanism)  rewind->S3"), "got: {out}");

    // HARD requested, HARD applied -> NO annotation, label HARD.
    let mut log = AssertLog::new();
    log.push(mk(KunaStrength::Hard, KunaStrength::Hard));
    let out = log.render_list();
    assert!(out.contains("strength=HARD  rewind->S3"));
    assert!(!out.contains("applied via HARD"));

    // HINT requested, HINT applied -> NO annotation (applied not HARD).
    let mut log = AssertLog::new();
    log.push(mk(KunaStrength::Hint, KunaStrength::Hint));
    let out = log.render_list();
    assert!(out.contains("strength=HINT  rewind->S3"));
    assert!(!out.contains("applied via HARD"));

    // NONE strength -> "-" label.
    let mut log = AssertLog::new();
    log.push(mk(KunaStrength::None, KunaStrength::Hard));
    let out = log.render_list();
    assert!(out.contains("strength=-  rewind->S3"), "got: {out}");
}

// --- test harness (mirrors src/kuna_restartlog/tests.rs) ---------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd(name: &str, off: u64) -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, off);
    Funcdata::new(name, name, glb, addr, 0x10000000, 0x40).unwrap()
}
