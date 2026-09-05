//! End-to-end two-pass gate for `ppclocalentry` — a PPC64 ELFv2 local entry
//! point is a point inside a function, not a function (P1 code/data partition).
//!
//! Fixture: `plt_ppc64le`, an ordinary `gcc` ppc64le PIE. The OpenPOWER ELFv2
//! ABI gives each of its `crtstuff.c` routines two entries — the symbol's
//! `st_value`, which materialises the TOC pointer `r2` from `r12`, and a local
//! entry 8 bytes later (`readelf -sW` prints `[<localentry>: 8]`) where an
//! intra-module `bl` lands.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** the walk takes each `bl <sym>+8` as an ordinary
//!   CALL target and mints a function there, which `funcdisc_recursive` /
//!   `fast_funcdisc` commit. Every locally called routine is listed twice: the
//!   named symbol truncated to its 8-byte TOC prologue plus the whole real body
//!   under an anonymous `sub_<hex>`.
//! * **default (the fix):** an address a defined `STT_FUNC` symbol declares to
//!   be its own local entry is never claimed, so each routine is one entry
//!   spanning its whole body.
//!
//! The third test is the property that keeps this from losing code: the fold
//! only ever REMOVES the duplicate second entry — it adds nothing, and every
//! named function keeps or grows its extent.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `PowerPC` `.sla` under `specs/` (gitignored;
//! `make specs`). When it is absent the bootstrap fails; the test prints that
//! and returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// `(global entry symbol, the husk's anonymous twin, the whole routine's size)`.
/// Sizes are `objdump`'s: each routine runs to the next symbol.
const SPLIT: &[(&str, &str, u64)] = &[
    ("deregister_tm_clones", "sub_6f8", 96),
    ("__do_global_dtors_aux", "sub_7c8", 240),
    ("frame_dummy", "sub_8b8", 12),
    ("__do_global_ctors_aux", "sub_948", 120),
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture the way every `kuna` driver does on a non-x86-64 image
/// (DIV-20/DIV-68: the Listing plus the discovery bundle), with `ppclocalentry`
/// in the requested state. `None` is a visible skip when the `.sla` is missing.
fn bootstrap(ppclocalentry: bool) -> Option<ConsoleProgram> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_ppclocalentry: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut()
        .set_kuna_option("funcstart_patterns", "on")
        .expect("funcstart_patterns flips on");
    prog.arch_mut()
        .set_kuna_option("ppclocalentry", if ppclocalentry { "on" } else { "off" })
        .expect("ppclocalentry flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn entries(prog: &ConsoleProgram) -> Vec<(String, u64)> {
    prog.function_entries_canonical().into_iter().map(|e| (e.name, e.size)).collect()
}

fn size_of(entries: &[(String, u64)], name: &str) -> Option<u64> {
    entries.iter().find(|(n, _)| n == name).map(|(_, s)| *s)
}

/// BEFORE (`--option ppclocalentry off`): every locally called routine is an
/// 8-byte named husk plus an anonymous body 8 bytes later.
#[test]
fn local_entries_split_every_function_with_the_option_off() {
    let Some(prog) = bootstrap(false) else {
        return;
    };
    let got = entries(&prog);
    for &(named, anon, whole) in SPLIT {
        assert_eq!(
            size_of(&got, named),
            Some(8),
            "with ppclocalentry off {named} must still be the 8-byte TOC-prologue husk \
             — the option must restore the previous discovery set exactly; got {got:?}"
        );
        assert_eq!(
            size_of(&got, anon),
            Some(whole - 8),
            "and its body must still be filed under {anon}; got {got:?}"
        );
    }
}

/// AFTER (default): the anonymous twins are gone and each named symbol spans
/// its whole routine.
#[test]
fn named_functions_span_their_whole_body_by_default() {
    let Some(prog) = bootstrap(true) else {
        return;
    };
    let got = entries(&prog);
    for &(named, anon, whole) in SPLIT {
        assert_eq!(
            size_of(&got, anon),
            None,
            "{anon} is {named}'s ELFv2 local entry, not a function; got {got:?}"
        );
        assert_eq!(
            size_of(&got, named),
            Some(whole),
            "{named} must span its whole routine; got {got:?}"
        );
    }
}

/// The property that keeps the fold from losing code: it only ever withholds
/// the duplicate second entry. Nothing is added, nothing else is removed, and
/// no surviving function shrinks.
#[test]
fn the_fold_only_removes_the_duplicate_entries() {
    let (Some(off), Some(on)) = (bootstrap(false), bootstrap(true)) else {
        return;
    };
    let before = entries(&off);
    let after = entries(&on);
    let mut removed: Vec<&str> = before
        .iter()
        .map(|(n, _)| n.as_str())
        .filter(|n| size_of(&after, n).is_none())
        .collect();
    let added: Vec<&str> = after
        .iter()
        .map(|(n, _)| n.as_str())
        .filter(|n| size_of(&before, n).is_none())
        .collect();
    let mut want: Vec<&str> = SPLIT.iter().map(|&(_, a, _)| a).collect();
    removed.sort_unstable();
    want.sort_unstable();
    assert_eq!(removed, want, "exactly the local-entry twins may be withheld");
    assert!(added.is_empty(), "the fold never adds an entry; got {added:?}");
    for (name, size) in &before {
        if let Some(now) = size_of(&after, name) {
            assert!(now >= *size, "{name} must not shrink: {size} -> {now}");
        }
    }
}
