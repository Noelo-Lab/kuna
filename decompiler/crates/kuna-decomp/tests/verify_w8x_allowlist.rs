//! Verifier adversarial tests for item `w8x-universalaction-wire`
//! (and the rule-body items it closes: `w8x-subflow-splits`,
//! `w8x-constseq-strings`, `kuna_stackprobeloop`, `kuna_loweredswitch` install).
//!
//! The verdict file is `docs/rust-port/reviews/w8x-allowlist-closure.md`.  These
//! tests target the three load-bearing claims of the wave:
//!
//!   1. `UNPORTED_ALLOWLIST` is genuinely **empty** (not just "small").
//!   2. The materialized decompile tree is **byte-equal** to the C++ oracle dump
//!      with NO line-stripping or renumbering — i.e. every previously-allowlisted
//!      pass is present, and the gate is not silently weakened.
//!   3. Each newly-wired pass sits at its **exact C++ `universalAction`
//!      registration position** (verified against its immediate neighbours, which
//!      is the property a misplaced rule would silently break while still passing
//!      a name-set check).
//!
//! They are intentionally redundant with the in-tree
//! `universalaction_listing::decompile_tree_dump_is_byte_equal_to_oracle`: a
//! verifier owns an independent oracle-derived check so a future edit that
//! weakens the in-tree gate (re-adds stripping, drops a pass) cannot pass review
//! unnoticed.

use kuna_decomp::universalaction::{universal_sched, ActionListFilter, UNPORTED_ALLOWLIST};

/// The C++ `decompile` group members (verbatim from
/// `universalaction_listing.rs` — the filtered decompile-root view).
const DECOMPILE_GROUPS: &[&str] = &[
    "base", "protorecovery", "protorecovery_a", "deindirect", "localrecovery",
    "deadcode", "typerecovery", "stackptrflow",
    "blockrecovery", "stackvars", "deadcontrolflow", "switchnorm",
    "cleanup", "splitcopy", "splitpointer", "merge", "dynamic", "casts", "analysis",
    "canonicalcompare", "presentcompare",
    "fixateglobals", "fixateproto", "constsequence", "bitfields",
    "segment", "returnsplit", "nodejoin", "doubleload", "doubleprecis",
    "unreachable", "subvar", "floatprecision",
    "conditionalexe",
];

const ORACLE: &str = include_str!("fixtures/list_action_decompile_oracle.txt");

/// Render the Rust decompile-root dump exactly as the B0 gate does.
fn rust_dump() -> String {
    let sched = universal_sched(None, None, vec![]);
    let filter = ActionListFilter::from_names(DECOMPILE_GROUPS.iter().copied());
    sched.list_action_dump(&filter)
}

/// The trailing whitespace-delimited token (the pass name) of a dump line.
fn last_token(line: &str) -> Option<&str> {
    line.split_whitespace().last()
}

/// Index of the first dump line whose trailing token is `name` (panics if
/// absent — a missing pass is a hard failure for these tests).
fn pos_of(lines: &[&str], name: &str) -> usize {
    lines
        .iter()
        .position(|l| last_token(l) == Some(name))
        .unwrap_or_else(|| panic!("pass {name:?} absent from the rendered decompile dump"))
}

/// Assert `b` is registered immediately after `a` in the dump's NON-BLANK line
/// stream (the C++ `universalAction` order; blank group/pool separators are
/// ignored, exactly as `addRule(a); addRule(b);` adjacency means).
fn assert_immediately_after(name_a: &str, name_b: &str) {
    let dump = rust_dump();
    // Filter to named lines only; blanks are group/pool structure, not passes.
    let named: Vec<&str> = dump
        .lines()
        .filter(|l| last_token(l).is_some())
        .collect();
    let ia = pos_of(&named, name_a);
    let ib = pos_of(&named, name_b);
    assert_eq!(
        ib,
        ia + 1,
        "registration position wrong: expected {name_b:?} immediately after {name_a:?}, \
         got {name_a:?}@{ia} and {name_b:?}@{ib}\n  ... {} {} ...",
        named.get(ia).copied().unwrap_or(""),
        named.get(ia + 1).copied().unwrap_or(""),
    );
}

// ---------------------------------------------------------------------------
// 1. The allowlist is genuinely empty.
// ---------------------------------------------------------------------------

#[test]
fn w8x_allowlist_is_empty() {
    // `is_empty()` on the const is `const_is_empty`-flagged precisely because the
    // const is empty; routing through `.len()` keeps the assertion honest while
    // staying a runtime check on the actual data.
    let n = UNPORTED_ALLOWLIST.len();
    assert_eq!(
        n, 0,
        "UNPORTED_ALLOWLIST must be empty after w8x wiring, still has {n}: {:?}",
        UNPORTED_ALLOWLIST.iter().map(|e| e.name).collect::<Vec<_>>()
    );
}

// ---------------------------------------------------------------------------
// 2. Byte-equality with the raw oracle, AND the ten formerly-allowlisted passes
//    are present (so the byte-equality is non-vacuous, not achieved by stripping
//    both sides).
// ---------------------------------------------------------------------------

#[test]
fn w8x_dump_is_byte_equal_to_raw_oracle() {
    // Independent of the in-tree gate: compare verbatim, no stripping, no
    // renumbering.  A length or content divergence fails.
    let dump = rust_dump();
    assert_eq!(
        dump, ORACLE,
        "rendered decompile dump is not byte-equal to the raw C++ oracle"
    );
}

#[test]
fn w8x_all_ten_formerly_allowlisted_passes_are_present() {
    // The exact ten names the wave claims to have wired in.  Each must appear in
    // BOTH the oracle and the Rust dump (proving the byte-equality above is not
    // vacuous — i.e. not achieved by both sides omitting them).
    const WIRED: &[&str] = &[
        "splitflow", "subfloat_convert", "stackprobeloop", "lowerswitchinstall",
        "dumptyhumplate", "splitcopy", "splitload", "splitstore", "stringcopy",
        "stringstore",
    ];
    let dump = rust_dump();
    let named_dump: Vec<&str> = dump.lines().filter(|l| last_token(l).is_some()).collect();
    let named_oracle: Vec<&str> = ORACLE.lines().filter(|l| last_token(l).is_some()).collect();
    for name in WIRED {
        assert!(
            named_dump.iter().any(|l| last_token(l) == Some(name)),
            "{name:?} missing from the Rust dump (allowlist re-introduced?)"
        );
        assert!(
            named_oracle.iter().any(|l| last_token(l) == Some(name)),
            "{name:?} missing from the C++ oracle (stale fixture?)"
        );
    }
}

// ---------------------------------------------------------------------------
// 3. Exact registration positions (C++ universalAction neighbours).  A rule
//    moved one slot is invisible to a name-set check but corrupts datatest
//    output; this is the property the focus item flags CRITICAL.
// ---------------------------------------------------------------------------

#[test]
fn w8x_lowerswitchinstall_is_between_varnodeprops_and_heritage() {
    // coreaction.cc:5751-5756 — install runs pre-heritage.
    assert_immediately_after("varnodeprops", "lowerswitchinstall");
    assert_immediately_after("lowerswitchinstall", "heritage");
}

#[test]
fn w8x_splitflow_is_between_subvar_subpiece_and_ptrflow() {
    // coreaction.cc:5892-5894.
    assert_immediately_after("subvar_subpiece", "splitflow");
    assert_immediately_after("splitflow", "ptrflow");
}

#[test]
fn w8x_subfloat_convert_is_between_funcptrencoding_and_floatcast() {
    // coreaction.cc:5902-5904.
    assert_immediately_after("funcptrencoding", "subfloat_convert");
    assert_immediately_after("subfloat_convert", "floatcast");
}

#[test]
fn w8x_stackprobeloop_is_immediately_after_doubleout() {
    // coreaction.cc:5916-5917 — last oppool1 rule before extra_pool_rules.
    assert_immediately_after("doubleout", "stackprobeloop");
}

#[test]
fn w8x_cleanup_split_string_block_is_in_exact_cpp_order() {
    // coreaction.cc:5976-5989 — the full contiguous run, in order.
    assert_immediately_after("2comp2sub", "dumptyhumplate");
    assert_immediately_after("dumptyhumplate", "subright");
    assert_immediately_after("piecestructure", "splitcopy");
    assert_immediately_after("splitcopy", "splitload");
    assert_immediately_after("splitload", "splitstore");
    assert_immediately_after("splitstore", "stringcopy");
    assert_immediately_after("stringcopy", "stringstore");
    assert_immediately_after("stringstore", "memsetcopy");
}

// ---------------------------------------------------------------------------
// 4. Re-derivation guard: the dump index is zero-padded width-4 (the sticky
//    setfill('0') the C++ oracle carries).  A regression to right-aligned
//    `{:>4}` would silently desync the byte comparison; pin it directly.
// ---------------------------------------------------------------------------

#[test]
fn w8x_dump_index_is_zero_padded_width4() {
    let dump = rust_dump();
    let first = dump.lines().next().expect("non-empty dump");
    assert!(
        first.starts_with("0000"),
        "first dump line must start with the zero-padded index 0000, got {first:?}"
    );
    // And the oracle agrees (guards against a fixture re-capture drifting format).
    assert!(
        ORACLE.lines().next().map(|l| l.starts_with("0000")).unwrap_or(false),
        "oracle first line is not zero-padded width-4"
    );
}
