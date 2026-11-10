//! A call whose fall-through is the function's own literal pool does not return.
//!
//! gcc places the PC-relative constants a body loads right after its last
//! instruction, so a function that ends in a call kuna cannot prove no-return
//! is followed by its pool. The flow walk took the call's fall-through,
//! decoded the pool words as instructions and stopped at the first that
//! decodes as nothing (`halt_baddata(); // warn: Bad instruction - Truncating
//! control flow here`), or ran on into code the call never returns to, while
//! `kuna disassemble` lists the same words as data.
//!
//! Fixtures: `poolfallthru_arm_{le,be}32` (source `poolfallthru.c`),
//! `poolguards_arm_{le,be}32` (source `poolguards.s`), whose functions each
//! violate one condition of the bound, and `bejoin_unreach_{arm32,thumb}_be_O2`,
//! whose `u_pool` calls `hi` ahead of `__builtin_unreachable()`.

use crate::common;

const BOUND: &str = "funcboundflow: the call's fall-through is this function's literal pool";

/// The C of `function` from the in-process `--json` surface, so the engine
/// under test is the one that decides the flow.
fn decompile(fixture: &str, function: &str, extra: &[&str]) -> String {
    let binary = common::fixture(fixture);
    let mut args = vec!["decompile", binary.as_str(), function, "--json"];
    args.extend(extra);
    let (doc, stderr, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{fixture} {extra:?}: {stderr}");
    let parsed: serde_json::Value = serde_json::from_str(&doc).unwrap_or_else(|e| panic!("{e}: {stderr}\n{doc}"));
    let code = parsed
        .pointer("/functions/0/code")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("{fixture} {function}: no code: {stderr}\n{doc}"));
    assert!(!code.trim().is_empty(), "{fixture} {function}: empty code: {stderr}");
    code.to_string()
}

#[test]
fn a_call_followed_by_its_literal_pool_ends_the_function() {
    for fixture in ["poolfallthru_arm_le32", "poolfallthru_arm_be32"] {
        let code = decompile(fixture, "probe", &[]);
        assert!(!code.contains("halt_baddata") && !code.contains("Bad instruction"), "{fixture}:\n{code}");
        let last = code.lines().rev().find(|line| line.trim() != "}").unwrap_or_default();
        assert!(last.contains("();") && last.contains(BOUND), "{fixture}:\n{code}");
        let off = decompile(fixture, "probe", &["--option", "funcboundflow", "off"]);
        assert!(off.contains("halt_baddata(); // warn: Bad instruction - Truncating control flow here"), "{fixture}:\n{off}");
    }
}

/// The listing kuna already prints calls the same words data. Its rule is its
/// own (`kuna-console/src/litpool.rs` folds any width the range's rows tile);
/// this only pins that the pool the bound acts on is visible as data there too.
#[test]
fn disassemble_lists_the_same_words_as_data() {
    let binary = common::fixture("poolfallthru_arm_le32");
    let (text, stderr, status) = common::run_kuna(&["disassemble", binary.as_str(), "probe"]);
    assert_eq!(status, 0, "{stderr}");
    let rows: Vec<&str> = text.lines().filter(|line| line.starts_with("0x")).collect();
    let call = rows.iter().rposition(|row| row.contains(" bl ")).expect("a call");
    assert_eq!(rows[call + 1..].len(), 5, "{text}");
    assert!(rows[call + 1..].iter().all(|row| row.contains(".word")), "{text}");
}

/// `u_pool`'s `a4 == 5` path ends at `hi`; read through its pool, it printed
/// `msg = "other"; return a4 + a1;` after the call, the other path's code.
#[test]
fn a_call_ahead_of_its_pool_does_not_run_into_the_other_path() {
    let code = decompile("bejoin_unreach_arm32_be_O2", "u_pool", &[]);
    let call = code.lines().position(|line| line.trim_start().starts_with("hi(")).expect("the call");
    assert!(code.lines().nth(call).is_some_and(|line| line.contains(BOUND)), "{code}");
    assert!(!code.lines().skip(call + 1).any(|line| line.contains("return") || line.contains("\"other\"")), "{code}");
    let thumb = decompile("bejoin_unreach_thumb_be_O2", "u_pool", &[]);
    assert!(thumb.contains(BOUND), "{thumb}");
}

/// Each condition of the bound, violated alone, keeps the fall-through: the
/// call is conditional, the word after it is a branch target, it is read a
/// halfword at a time, the gap before it is live code, the gap is wider than the
/// alignment padding, the word is in a writable executable section, the callee
/// provably returns and the pool's only load sits past a call (so the word is
/// reached and is code), or the pool's only reader sits past a call nothing
/// proves returns and may be data itself.
/// `bounded_call` has none of them, `pad_call` has one instruction of real
/// padding, `indirect_call` reaches its pool through a `blx`, `two_pools` has
/// two such calls, `wide_literal` loads a VFP doubleword, `ordered_loads` reads
/// its pool from a block after the call, and `junk_pointer`'s pool word spells a
/// load of live code, which stays.
#[test]
fn a_function_that_violates_one_condition_keeps_its_fall_through() {
    for fixture in ["poolguards_arm_le32", "poolguards_arm_be32"] {
        for function in [
            "cond_call",
            "labelled_word",
            "halfword_read",
            "live_gap",
            "wide_gap",
            "data_word",
            "returning_callee",
            "unproven_reader",
        ] {
            let code = decompile(fixture, function, &[]);
            assert!(!code.contains(BOUND), "{fixture} {function}:\n{code}");
        }
        for function in ["bounded_call", "pad_call", "indirect_call", "wide_literal", "ordered_loads"] {
            let code = decompile(fixture, function, &[]);
            assert_eq!(code.matches(BOUND).count(), 1, "{fixture} {function}:\n{code}");
        }
        let code = decompile(fixture, "two_pools", &[]);
        assert_eq!(code.matches(BOUND).count(), 2, "{fixture} two_pools:\n{code}");
        let code = decompile(fixture, "junk_pointer", &[]);
        assert_eq!(code.matches(BOUND).count(), 1, "{fixture} junk_pointer:\n{code}");
        assert!(code.contains("+ 1"), "{fixture} junk_pointer keeps the live code after the returning call:\n{code}");
    }
}

/// A data word after a non-returning `svc` decodes as a load of the returning
/// call's fall-through. The word is past a user operation, so its load is no
/// witness: nothing is bounded at a pool and `return 5` stays, as it does
/// without the bound.
#[test]
fn a_load_decoded_past_a_user_operation_witnesses_nothing() {
    for fixture in ["poolguards_arm_le32", "poolguards_arm_be32"] {
        let code = decompile(fixture, "svc_junk", &[]);
        let off = decompile(fixture, "svc_junk", &["--option", "funcboundflow", "off"]);
        assert!(!code.contains(BOUND), "{fixture}:\n{code}");
        assert!(code.contains("return 5;"), "{fixture}:\n{code}");
        assert!(off.contains("return 5;"), "{fixture}:\n{off}");
    }
}
