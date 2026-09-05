//! End-to-end two-pass gate for `overlapbranch` — the conditional branch whose own
//! target lies strictly inside its fall-through instruction (P2 flow
//! classification).
//!
//! Fixture: `overlapbranch_i386` (`overlapbranch_i386.py` beside it is the
//! generator), a 2 KiB ELF32 carrying the same idiom, at the same address, as the
//! round-2 RE-friction crackme `pcme0` — a `75 01` short JNZ hopping over a junk
//! `e8` lead byte at 0x8048824, so a follower that decodes the fall-through first
//! sees one bogus 5-byte `CALL` that swallows the branch target at 0x8048825.
//!
//! This case is the promoted acceptance probe of RE-need
//! `overlapping-anti-disassembly-sequence` (`tests/cli/`) executed by
//! `make rust-test`: the two `stdout_absent` clauses are asserted verbatim, and
//! the `option overlapbranch off` pass proves the clauses are discriminating
//! rather than vacuous — with the option off both artefacts are emitted.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/overlapbranch_i386")
        .to_string_lossy()
        .into_owned()
}

fn specs() -> String {
    repo_root().join("specs").to_string_lossy().into_owned()
}

fn specs_missing(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
        || stderr.contains(".sla")
}

/// Decompile the fixture's overlapping function, optionally flipping the option.
fn decompile(extra: &[&str]) -> Option<String> {
    let binary = fixture();
    let sleigh = specs();
    let mut args: Vec<&str> =
        vec!["decompile", &binary, "0x804881c", "--addr", "--sleighpath", &sleigh];
    args.extend_from_slice(extra);
    let out = Command::new(env!("CARGO_BIN_EXE_kuna")).args(&args).output().expect("spawn kuna");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !out.status.success() {
        if specs_missing(&stderr) {
            eprintln!("skipping: specs not built ({stderr})");
            return None;
        }
        panic!("kuna decompile failed: {stderr}");
    }
    Some(stdout)
}

/// The acceptance clauses of `a-52c2ad89b522`, verbatim: neither the out-of-image
/// call nor the invented global may appear, and the run must succeed.
#[test]
fn default_emits_neither_artefact_of_the_swallowed_decode() {
    let Some(c) = decompile(&[]) else { return };
    assert!(
        !regex_sub_call(&c),
        "the swallowed decode's out-of-image sub_<hex>() call is still emitted:\n{c}"
    );
    assert!(
        !regex_dat_add(&c),
        "the swallowed decode's invented dat_<hex> global is still emitted:\n{c}"
    );
    // ... and the real stream the branch target owns is what is recovered.
    assert!(c.contains("a0 + 7"), "the real add-chain stream was not recovered:\n{c}");
    assert!(
        c.contains("overlapbranch:"),
        "the truncation must be attributable by a warning:\n{c}"
    );
}

/// The same clauses with the option off must FAIL — otherwise the default pass is
/// asserting nothing.
#[test]
fn option_off_reproduces_both_artefacts() {
    let Some(c) = decompile(&["--option", "overlapbranch", "off"]) else { return };
    assert!(regex_sub_call(&c), "expected the out-of-image call with the option off:\n{c}");
    assert!(regex_dat_add(&c), "expected the invented global with the option off:\n{c}");
}

/// `sub_[0-9a-f]{7}\(\)` — a call, with no arguments, to a seven-hex-digit address.
fn regex_sub_call(c: &str) -> bool {
    c.match_indices("sub_").any(|(i, _)| {
        let rest = &c[i + 4..];
        let hex: String = rest.chars().take_while(|ch| ch.is_ascii_hexdigit()).collect();
        hex.len() == 7 && rest[7..].starts_with("()")
    })
}

/// `dat_[0-9a-f]{8} \+=` — an eight-hex-digit global being incremented.
fn regex_dat_add(c: &str) -> bool {
    c.match_indices("dat_").any(|(i, _)| {
        let rest = &c[i + 4..];
        let hex: String = rest.chars().take_while(|ch| ch.is_ascii_hexdigit()).collect();
        hex.len() == 8 && rest[8..].starts_with(" +=")
    })
}
