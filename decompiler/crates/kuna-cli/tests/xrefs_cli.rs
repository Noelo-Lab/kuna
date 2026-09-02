//! CLI end-to-end gate for `kuna xrefs` — drives the built `kuna` binary over the
//! real vendored fixtures and asserts the cross-reference surface an RE agent
//! consumes: both directions, every `kind`, and the JSON shape.
//!
//! The load-bearing case is the promoted acceptance probe
//! ([`the_acceptance_probe`]): `--to 0x1030` on the stripped PIE
//! `aif_gap_x86_64`, whose `.plt.got` thunk is called from exactly one site
//! (`0x1102`). Before `kuna xrefs` existed there was no way to ask that question
//! at all.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` / `ARM` `.sla` under `specs/` (gitignored;
//! `make specs`). When one is absent the command cannot build an architecture;
//! the test prints that and returns early — a specs-less CI is a visible skip,
//! never a false green.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

/// The stripped x86-64 PIE the acceptance probe names. `0x1030` is its
/// `.plt.got` `__cxa_finalize` thunk; `0x4010`/`0x4014` are its two globals.
fn aif_gap() -> String {
    fixture("aif_gap_x86_64")
}

/// The vendored non-stripped `fauxware`: named functions, real library calls, and
/// `.rodata` strings the `strings` pass names `s_<addr>` — the binary the
/// string-to-its-users workflow needs.
fn fauxware() -> String {
    fixture("fauxware")
}

/// Run the built `kuna` binary, returning `(stdout, stderr, exit code)`.
fn run_kuna(args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

/// `true` when the failure is a missing-`.sla` bootstrap failure (a legitimate
/// skip), not a real bug.
fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

/// Run `kuna xrefs ...`, returning its stdout, or `None` on a missing-`.sla` skip.
fn xrefs(args: &[&str]) -> Option<String> {
    let mut argv = vec!["xrefs"];
    argv.extend_from_slice(args);
    let (stdout, stderr, code) = run_kuna(&argv);
    if code != 0 {
        if is_specs_skip(&stderr) {
            eprintln!("skipping: {stderr}");
            return None;
        }
        panic!("kuna xrefs {args:?} failed ({code}): {stderr}");
    }
    Some(stdout)
}

/// The integer value of the first `"key": N` in a document.
fn json_int(doc: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{key}\":");
    let i = doc.find(&needle)? + needle.len();
    doc[i..].trim_start().split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()
}

/// The string value of the first `"key": "..."` in a document.
fn json_str(doc: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\": \"");
    let i = doc.find(&needle)? + needle.len();
    Some(doc[i..].split('"').next()?.to_string())
}

/// Every `"kind": "..."` value in a document, in order.
fn kinds(doc: &str) -> Vec<String> {
    doc.match_indices("\"kind\": \"")
        .filter_map(|(i, m)| doc[i + m.len()..].split('"').next().map(str::to_string))
        .collect()
}

/// The data rows of the human surface (everything past the `#` header).
fn rows(text: &str) -> Vec<&str> {
    text.lines().filter(|l| !l.starts_with('#') && !l.is_empty()).collect()
}

/// **The acceptance probe** (`docs/re-needs/`): the exact invocation the RE loop
/// recorded as the definition of done. Exit 0, valid JSON, `count > 0`, and every
/// row carrying an `address_hex`.
#[test]
fn the_acceptance_probe() {
    let Some(doc) = xrefs(&[&aif_gap(), "--to", "0x1030", "--json"]) else {
        return;
    };
    assert!(json_int(&doc, "count").unwrap() > 0, "no references found:\n{doc}");
    assert!(doc.contains("\"address_hex\":"), "no address_hex in a row:\n{doc}");
    assert_eq!(json_str(&doc, "direction").as_deref(), Some("to"));
    // The one call site: `_FINI_0` @ 0x10e0 calls the `.plt.got` thunk at 0x1102.
    assert_eq!(kinds(&doc), vec!["call"]);
    assert!(doc.contains("\"address_hex\": \"0x1102\""), "wrong call site:\n{doc}");
    assert!(doc.contains("\"name\": \"_FINI_0\""), "call site unattributed:\n{doc}");
}

/// Both ends of every edge are always spelled out, whichever direction was
/// asked for, so a consumer never has to infer which one `address` meant.
#[test]
fn every_row_names_both_ends_of_the_edge() {
    let Some(doc) = xrefs(&[&aif_gap(), "--to", "0x1030", "--json"]) else {
        return;
    };
    for key in ["from_address_hex", "to_address_hex", "from_function", "to_function"] {
        assert!(doc.contains(&format!("\"{key}\":")), "missing {key}:\n{doc}");
    }
    assert_eq!(json_str(&doc, "to_address_hex").as_deref(), Some("0x1030"));
}

/// `--to` on a named function finds its call sites and attributes each to the
/// function it sits in.
#[test]
fn to_a_named_function_finds_its_call_sites() {
    let Some(doc) = xrefs(&[&fauxware(), "--to", "authenticate", "--json"]) else {
        return;
    };
    assert_eq!(json_int(&doc, "count"), Some(1), "{doc}");
    assert_eq!(kinds(&doc), vec!["call"]);
    assert!(doc.contains("\"name\": \"main\""), "call site is not attributed to main:\n{doc}");
}

/// `--from` is the other direction: a function's callees, named.
#[test]
fn from_a_function_lists_its_callees() {
    let Some(doc) = xrefs(&[&fauxware(), "--from", "main", "--json", "--kind", "call"]) else {
        return;
    };
    for callee in ["authenticate", "accepted", "rejected", "puts", "read"] {
        assert!(doc.contains(&format!("\"name\": \"{callee}\"")), "no call to {callee}:\n{doc}");
    }
    assert!(kinds(&doc).iter().all(|k| k == "call"), "--kind call leaked another kind:\n{doc}");
}

/// The reference kinds an agent navigates by are all populated, not just calls:
/// a `.bss` byte that one function reads and another writes, and a `.rodata`
/// string whose address is taken.
#[test]
fn data_read_and_write_references_are_all_reported() {
    let Some(doc) = xrefs(&[&aif_gap(), "--to", "0x4010", "--json"]) else {
        return;
    };
    let found = kinds(&doc);
    for want in ["data", "read", "write"] {
        assert!(found.iter().any(|k| k == want), "no {want} reference:\n{doc}");
    }

    let Some(doc) = xrefs(&[&fauxware(), "--to", "s_400915", "--json"]) else {
        return;
    };
    assert!(json_int(&doc, "count").unwrap() > 0, "a used string has no users:\n{doc}");
    assert_eq!(kinds(&doc), vec!["data"], "{doc}");
}

/// A call's own return address is materialized by the lifter on every
/// architecture (x86 stores it, ARM copies it into `LR`). It is not a reference
/// and must never appear as one.
#[test]
fn a_calls_return_address_is_not_reported_as_a_reference() {
    let Some(doc) = xrefs(&[&aif_gap(), "--from", "_FINI_0", "--json"]) else {
        return;
    };
    // `_FINI_0` calls 0x1030 at 0x1102 (5 bytes) and 0x1070 at 0x1107 (5 bytes).
    for after_a_call in ["0x1107", "0x110c"] {
        assert!(
            !doc.contains(&format!("\"to_address_hex\": \"{after_a_call}\"")),
            "a return address leaked as a reference:\n{doc}"
        );
    }
}

/// An address target works as well as a name, and resolves the name back.
#[test]
fn an_address_target_resolves_its_name() {
    let Some(doc) = xrefs(&[&fauxware(), "--to", "0x400664", "--json"]) else {
        return;
    };
    assert_eq!(json_str(&doc, "name").as_deref(), Some("authenticate"), "{doc}");
    assert_eq!(json_int(&doc, "address"), Some(0x400664));
}

/// The human surface: a `#` header naming the query, then one tab-separated row
/// per reference — greppable, and never the JSON document.
#[test]
fn the_human_surface_is_a_header_plus_tab_separated_rows() {
    let Some(text) = xrefs(&[&aif_gap(), "--to", "0x1030"]) else {
        return;
    };
    let mut lines = text.lines();
    let header = lines.next().expect("a header line");
    assert!(header.starts_with("# 1 reference to __cxa_finalize @ 0x1030"), "{text}");
    assert!(!text.contains('{'), "the human surface emitted JSON:\n{text}");
    let rows = rows(&text);
    assert_eq!(rows.len(), 1, "{text}");
    let cols: Vec<&str> = rows[0].split('\t').collect();
    assert_eq!(cols[0], "0x1102");
    assert_eq!(cols[1], "call");
    assert_eq!(cols[2], "_FINI_0+0x22");
    assert!(cols[3].contains("0x1030"), "the instruction column is missing: {:?}", cols);
}

/// An empty answer is an answer: a target nothing references is exit 0 with
/// `count: 0`, not an error a caller has to distinguish from a broken run.
#[test]
fn a_target_with_no_references_is_an_empty_success() {
    let Some(doc) = xrefs(&[&aif_gap(), "--to", "0x2000", "--json"]) else {
        return;
    };
    assert_eq!(json_int(&doc, "count"), Some(0), "{doc}");
    assert!(doc.contains("\"xrefs\": []"), "{doc}");
}

/// A name that resolves to nothing is a failed query (exit 1), and says so —
/// distinct from a usage error (exit 2).
#[test]
fn an_unresolvable_target_fails_with_a_reason() {
    let (_, stderr, code) = run_kuna(&["xrefs", &fauxware(), "--to", "no_such_symbol_here"]);
    if is_specs_skip(&stderr) {
        return;
    }
    assert_eq!(code, 1, "{stderr}");
    assert!(stderr.contains("no symbol named"), "{stderr}");
}

/// Usage errors are exit 2 with the usage block, never a silent empty answer.
#[test]
fn usage_errors_exit_two() {
    for args in [
        vec!["xrefs"],
        vec!["xrefs", "/nonexistent"],
        vec!["xrefs", "/nonexistent", "--to", "main", "--from", "main"],
        vec!["xrefs", "/nonexistent", "--to"],
        vec!["xrefs", "/nonexistent", "--to", "main", "--kind", "sideways"],
    ] {
        let (_, stderr, code) = run_kuna(&args);
        assert_eq!(code, 2, "{args:?} should be a usage error, got {code}: {stderr}");
        assert!(stderr.contains("usage: kuna xrefs"), "{args:?}: {stderr}");
    }
}
