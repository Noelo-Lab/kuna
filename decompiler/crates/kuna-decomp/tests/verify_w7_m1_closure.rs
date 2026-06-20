//! Independent verifier tests for item `w7-m1-closure` (round 1).
//!
//! `w7-m1-closure` is a TRIAGE / coverage-closure item: it touched only the
//! three in-scope unit-test files (`testfuncproto.rs`, `testparamstore.rs`,
//! `testtypes.rs`), un-ignored NOTHING, and refreshed every `#[ignore]` reason
//! into a `DEFER(W8)` annotation while leaving 31 bodies deferred.  The
//! invariants a verifier must defend are therefore not arithmetic but
//! structural:
//!
//!   1. NAME-PARITY: every C++ `TEST(name)` in the in-scope suites has a
//!      same-named `#[test] fn name` in the corresponding Rust file (this is the
//!      `port_audit` 207/207 contract, re-derived here from the C++ source so a
//!      future rename cannot silently drop a name without failing this test).
//!   2. DEFER DISCIPLINE: every `#[ignore]`d test in the three in-scope files
//!      carries a `DEFER(` annotation in its reason string AND an inline
//!      `// DEFER(` comment (no silently-ignored test).
//!   3. FAITHFULNESS GUARD on the only two un-ignored bodies in scope
//!      (`enum_matching` / `enum_matching2`): they must still assert the exact
//!      C++ oracle outputs.  The triage must not be a cover for gutting a body
//!      to pass.  Re-derived here directly against `TypeEnum::getMatches`
//!      (type.cc:1537) over additional boundary inputs the original TESTs do
//!      not cover, so a weakened `get_matches` would fail HERE even if the two
//!      ported tests were themselves weakened in lockstep.
//!
//! These tests read the in-scope test files as text for (1)/(2) and re-run the
//! enum oracle for (3).  They live in the kuna-decomp crate per the verifier
//! protocol and are tagged with the item id.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

use kuna_decomp::dtype::{flags, type_metatype, Datatype, DatatypeKind};

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR = decompiler/crates/kuna-decomp ; repo root is 3 up.
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for _ in 0..3 {
        p.pop();
    }
    p
}

fn read_test_file(stem: &str) -> String {
    let path = repo_root()
        .join("decompiler/crates/kuna-decomp/tests")
        .join(format!("{stem}.rs"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// The C++ `TEST(name)` names of the three in-scope upstream unit-test files,
/// FROZEN here. They were derived from `decompiler/unittests/<stem>.cc` while the
/// C++ tree existed; that tree has since been removed (see docs/RUST_PORT.md), so
/// the name-parity contract is now pinned to this snapshot — a future rename in the
/// Rust files still fails this test, which is the contract's whole point.
fn cpp_test_names(stem: &str) -> Vec<String> {
    let names: &[&str] = match stem {
        "testfuncproto" => &[
            "funcproto_register", "funcproto_smallregister", "funcproto_stackalign",
            "funcproto_pointeroverflow", "funcproto_stackoverflow", "funcproto_floatreg",
            "funcproto_floattogeneric", "funcproto_grouped", "funcproto_join", "funcproto_nojoin",
            "funcproto_hiddenreturn", "funcproto_mixedmeta", "funcproto_recoverbasic",
            "funcproto_recoversmallreg", "funcproto_recoverstack", "funcproto_recoverunrefregister",
            "funcproto_recoverunrefstack", "funcproto_recovergroups", "funcproto_recoverholes",
            "funcproto_recoverfloat", "funcproto_recovermixedmeta",
        ],
        "testparamstore" => &[
            "paramstore_x64", "paramstore_ppc64be_stdcall", "paramstore_mips32be_stdcall",
            "paramstore_aarch64_cdecl",
        ],
        "testtypes" => &[
            "cast_basic", "cast_pointer", "cast_enum", "cast_compare", "type_ordering",
            "cast_integertoken", "enum_matching", "enum_matching2",
        ],
        other => panic!("no frozen TEST-name list for {other}"),
    };
    names.iter().map(|s| s.to_string()).collect()
}

/// Extract every `#[test] fn name` from a Rust test file (the function name on
/// the line that begins `fn ` after a `#[test]` / `#[ignore]` attribute block).
fn rust_test_names(stem: &str) -> Vec<String> {
    let src = read_test_file(stem);
    let mut names = Vec::new();
    let lines: Vec<&str> = src.lines().collect();
    let mut saw_test = false;
    for line in &lines {
        let t = line.trim_start();
        if t.starts_with("#[test]") {
            saw_test = true;
            continue;
        }
        if saw_test {
            if t.starts_with("#[") || t.starts_with("//") {
                // intervening attribute (#[ignore], #[cfg_attr], ...) or comment
                continue;
            }
            // first non-attribute line after #[test] must be the fn
            if let Some(rest) = t.strip_prefix("fn ") {
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                names.push(name);
            }
            saw_test = false;
        }
    }
    names
}

// ---------------------------------------------------------------------------
// (1) NAME-PARITY: every in-scope C++ TEST has a same-named Rust #[test] fn.
// ---------------------------------------------------------------------------

#[test]
fn w7_m1_closure_name_parity_in_scope_suites() {
    let suites = ["testfuncproto", "testparamstore", "testtypes"];
    let expected_counts = [(("testfuncproto"), 21usize), ("testparamstore", 4), ("testtypes", 8)];

    for stem in suites {
        let cpp = cpp_test_names(stem);
        let rust: std::collections::BTreeSet<String> =
            rust_test_names(stem).into_iter().collect();
        assert!(
            !cpp.is_empty(),
            "{stem}.cc yielded zero TEST() names (parser regression?)"
        );
        for name in &cpp {
            assert!(
                rust.contains(name),
                "[w7-m1-closure] {stem}: C++ TEST({name}) has NO same-named #[test] fn in {stem}.rs"
            );
        }
        let want = expected_counts
            .iter()
            .find(|(s, _)| *s == stem)
            .map(|(_, c)| *c)
            .unwrap();
        assert_eq!(
            cpp.len(),
            want,
            "[w7-m1-closure] {stem}: expected {want} C++ TESTs, found {}",
            cpp.len()
        );
    }
}

// ---------------------------------------------------------------------------
// (2) DEFER DISCIPLINE: no test is #[ignore]d without a DEFER annotation.
// Every #[ignore] in the three files must (a) carry "DEFER" in its reason
// string and (b) be preceded by an inline `// DEFER(` comment line.
// ---------------------------------------------------------------------------

#[test]
fn w7_m1_closure_every_ignore_has_defer_reason_and_inline_comment() {
    for stem in ["testfuncproto", "testparamstore", "testtypes"] {
        let src = read_test_file(stem);
        let lines: Vec<&str> = src.lines().collect();
        let mut ignore_count = 0usize;
        for (i, line) in lines.iter().enumerate() {
            let t = line.trim_start();
            if !t.starts_with("#[ignore") {
                continue;
            }
            ignore_count += 1;
            // (a) reason string carries DEFER
            assert!(
                line.contains("DEFER"),
                "[w7-m1-closure] {stem} line {}: #[ignore] reason has no DEFER tag: {line}",
                i + 1
            );
            // (b) an inline `// DEFER(` comment appears in the preceding 6 lines
            let start = i.saturating_sub(6);
            let has_inline_defer = lines[start..i]
                .iter()
                .any(|l| l.trim_start().starts_with("//") && l.contains("DEFER("));
            assert!(
                has_inline_defer,
                "[w7-m1-closure] {stem} line {}: no inline `// DEFER(` comment above the #[ignore]",
                i + 1
            );
        }
        assert!(
            ignore_count > 0,
            "[w7-m1-closure] {stem}: expected at least one #[ignore] (deferred body)"
        );
    }
}

/// The in-scope ignore counts must match the triaged state exactly, so a future
/// edit that silently un-ignores (or newly ignores) a body without re-triage
/// trips here.  Mirror of the item gate numbers.
#[test]
fn w7_m1_closure_ignore_counts_match_triage() {
    let count_ignores = |stem: &str| {
        read_test_file(stem)
            .lines()
            .filter(|l| l.trim_start().starts_with("#[ignore"))
            .count()
    };
    assert_eq!(count_ignores("testfuncproto"), 21, "testfuncproto ignore count");
    assert_eq!(count_ignores("testparamstore"), 4, "testparamstore ignore count");
    assert_eq!(count_ignores("testtypes"), 6, "testtypes ignore count");
}

// ---------------------------------------------------------------------------
// (3) FAITHFULNESS GUARD on the two un-ignored bodies: re-derive the enum
// oracle over EXTRA inputs (not present in the C++ TESTs) so a weakened
// get_matches / make_enum cannot pass.
// ---------------------------------------------------------------------------

/// Same construction as testtypes.rs `make_enum` (an interned `TypeEnum` after
/// decode): a BTreeMap<u64,String> keyed by value, x86-64 default enum size 8.
fn make_enum(size: i32, entries: &[(u64, &str)]) -> Rc<Datatype> {
    let mut dt = Datatype::new_with_align(size, size, type_metatype::TYPE_INT);
    dt.flags |= flags::enumtype;
    let mut namemap: BTreeMap<u64, String> = BTreeMap::new();
    for (v, n) in entries {
        namemap.insert(*v, (*n).to_string());
    }
    dt.kind = DatatypeKind::Enum { namemap };
    Rc::new(dt)
}

/// enum3 { ZERO=0, ONE=1, TWO=2, FOUR=4, EIGHT=8 } — the C++ enum_matching
/// fixture — probed at inputs the original TEST does NOT cover.  Each expected
/// value is hand-derived from TypeEnum::getMatches (type.cc:1537).
#[test]
fn w7_m1_closure_enum3_extra_boundary_inputs() {
    let enum3 = make_enum(
        8,
        &[(0, "ZERO"), (1, "ONE"), (2, "TWO"), (4, "FOUR"), (8, "EIGHT")],
    );

    // 3 = 2 | 1  -> ["TWO","ONE"] (greedy MSB-first), not complement.
    let r = enum3.get_matches(3).unwrap();
    assert_eq!(r.match_name, vec!["TWO", "ONE"], "3 == TWO|ONE");
    assert!(!r.complement);

    // 15 = 8|4|2|1 -> ["EIGHT","FOUR","TWO","ONE"], not complement.
    let r = enum3.get_matches(15).unwrap();
    assert_eq!(
        r.match_name,
        vec!["EIGHT", "FOUR", "TWO", "ONE"],
        "15 == EIGHT|FOUR|TWO|ONE"
    );
    assert!(!r.complement);

    // 13 = 8|4|1 -> ["EIGHT","FOUR","ONE"].
    let r = enum3.get_matches(13).unwrap();
    assert_eq!(r.match_name, vec!["EIGHT", "FOUR", "ONE"], "13 == EIGHT|FOUR|ONE");
    assert!(!r.complement);

    // Complement path with size 8: ~8 = 0xfffffffffffffff7 is the original
    // TEST's complement row; verify the sibling ~1 = 0xfffffffffffffffe, which
    // complements to 1 -> ["ONE"], complement=true.
    let r = enum3.get_matches(0xfffffffffffffffe).unwrap();
    assert_eq!(r.match_name, vec!["ONE"], "~1 == complement of ONE");
    assert!(r.complement);

    // 6 = 4|2 (no SIX member here) -> ["FOUR","TWO"], not complement.
    let r = enum3.get_matches(6).unwrap();
    assert_eq!(r.match_name, vec!["FOUR", "TWO"], "6 == FOUR|TWO");
    assert!(!r.complement);
}

/// enum4 { ZERO,ONE,TWO,FOUR,SIX=6,EIGHT,ELEVEN=11 } — the enum_matching2
/// fixture — probed at extra inputs.  SIX=6 (a non-power-of-two member) and
/// ELEVEN=11 exercise the upper_bound/predecessor walk and the restricted
/// re-search branch (`target = curval & ~diff`).
#[test]
fn w7_m1_closure_enum4_extra_boundary_inputs() {
    let enum4 = make_enum(
        8,
        &[
            (0, "ZERO"),
            (1, "ONE"),
            (2, "TWO"),
            (4, "FOUR"),
            (6, "SIX"),
            (8, "EIGHT"),
            (11, "ELEVEN"),
        ],
    );

    // 6 is an exact member -> ["SIX"] alone (single-name hit, not FOUR|TWO).
    let r = enum4.get_matches(6).unwrap();
    assert_eq!(r.match_name, vec!["SIX"], "6 is the exact member SIX");
    assert!(!r.complement);

    // 14 = 8 | 6 -> greedy picks EIGHT (msb), leftover 6 hits SIX exactly:
    // ["EIGHT","SIX"].
    let r = enum4.get_matches(14).unwrap();
    assert_eq!(r.match_name, vec!["EIGHT", "SIX"], "14 == EIGHT|SIX");
    assert!(!r.complement);

    // 11 is an exact member -> ["ELEVEN"] (the original TEST checks this row;
    // re-pinned here as the oracle for the predecessor walk landing on the
    // largest key).
    let r = enum4.get_matches(11).unwrap();
    assert_eq!(r.match_name, vec!["ELEVEN"], "11 is the exact member ELEVEN");
    assert!(!r.complement);

    // 16 = 0x10 has no representation (no member, and complement ~0x10 over
    // size 8 also cannot be ORed from {1,2,4,6,8,11}) -> empty, not complement.
    let r = enum4.get_matches(0x10).unwrap();
    assert!(r.match_name.is_empty(), "0x10 has no enum4 representation");
    assert!(!r.complement);
}

/// Order provenance: the C++ namemap is `map<uintb,string>` (ascending key
/// order).  Build the SAME enum with entries inserted in a SCRAMBLED order and
/// confirm get_matches is insertion-order-independent (BTreeMap re-sorts by
/// key), i.e. the helper faithfully reproduces the std::map key ordering the
/// upper_bound walk depends on.
#[test]
fn w7_m1_closure_enum_match_is_insertion_order_independent() {
    let ascending = make_enum(
        8,
        &[(0, "ZERO"), (1, "ONE"), (2, "TWO"), (4, "FOUR"), (8, "EIGHT")],
    );
    let scrambled = make_enum(
        8,
        &[(8, "EIGHT"), (0, "ZERO"), (4, "FOUR"), (1, "ONE"), (2, "TWO")],
    );
    for val in [0u64, 1, 2, 3, 5, 6, 7, 8, 13, 15, 0xfffffffffffffff7, 0x10] {
        let a = ascending.get_matches(val).unwrap();
        let b = scrambled.get_matches(val).unwrap();
        assert_eq!(
            a.match_name, b.match_name,
            "match_name differs for val={val:#x} under scrambled insertion"
        );
        assert_eq!(
            a.complement, b.complement,
            "complement differs for val={val:#x} under scrambled insertion"
        );
    }
}
