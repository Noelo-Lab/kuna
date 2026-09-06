//! CLI end-to-end gate for `kuna xrefs` — drives the built `kuna` binary over the
//! real vendored fixtures and asserts the cross-reference surface an RE agent
//! consumes: both directions, every `kind`, and the JSON shape.
//!
//! Two promoted acceptance probes are load-bearing here. [`the_acceptance_probe`]
//! is `--to 0x1030` on the stripped PIE `aif_gap_x86_64`, whose `.plt.got` thunk
//! `_FINI_0` both guards on and calls — before `kuna xrefs` existed there was no
//! way to ask that question at all. [`a_pe_import_answers_the_same_at_its_veneer_and_at_its_slot`]
//! is the `xrefs-unify-pe-import` need: an import has two addresses under one
//! name, and asking either must answer the same.
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

/// The stripped Cortex-M firmware whose `0x800039c` is reachable ONLY through a
/// data path: no symbol, no `<patternpairs>` prologue pair, and no direct `bl`
/// points at it, so no recursive descent seeded from the inventory reaches it.
/// It is the witness for both halves of the query's discovery
/// ([`a_function_no_descent_reaches_still_answers_for_itself`] and
/// [`the_gap_walk_finds_call_sites_a_descent_cannot_reach`]).
fn cortexm_gap() -> String {
    fixture("cortexm_aifcorroborate_le32")
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

/// The vendored MinGW x86-64 PE. Every import in it has two addresses under one
/// name — an `FF 25` veneer in `.text` and an IAT slot in `.idata` — which is the
/// shape [`a_pe_import_answers_the_same_at_its_veneer_and_at_its_slot`] is about.
fn pe_imports() -> String {
    fixture("pe_imports.exe")
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
    // `_FINI_0` @ 0x10e0 uses `__cxa_finalize` twice, once through each of its
    // two addresses: it null-checks the GOT slot at 0x3ff8 (the weak-symbol
    // guard) and then calls the `.plt.got` veneer at 0x1030. Both are references
    // to the same import, and both are reported whichever address is asked for.
    assert_eq!(kinds(&doc), vec!["read", "call"]);
    assert!(doc.contains("\"address_hex\": \"0x1102\""), "wrong call site:\n{doc}");
    assert!(doc.contains("\"address_hex\": \"0x10ee\""), "missing the guard read:\n{doc}");
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
    // Both of the import's addresses show up as a row's `to_address_hex`: the
    // guard reads the GOT slot, the call goes through the veneer.
    for to in ["0x1030", "0x3ff8"] {
        assert!(doc.contains(&format!("\"to_address_hex\": \"{to}\"")), "no row to {to}:\n{doc}");
    }
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
    assert!(header.starts_with("# 2 references to __cxa_finalize @ 0x1030"), "{text}");
    assert!(!text.contains('{'), "the human surface emitted JSON:\n{text}");
    let rows = rows(&text);
    assert_eq!(rows.len(), 2, "{text}");
    let cols: Vec<&str> = rows[1].split('\t').collect();
    assert_eq!(cols[0], "0x1102");
    assert_eq!(cols[1], "call");
    assert_eq!(cols[2], "_FINI_0+0x22");
    assert!(cols[3].contains("0x1030"), "the instruction column is missing: {:?}", cols);
    // The other address the answer was taken over is named, so a count that does
    // not match a raw grep of the target explains itself on the spot.
    assert!(text.contains("# same import at 0x3ff8"), "the alias is unexplained:\n{text}");
}

/// **The `xrefs-unify-pe-import` acceptance probe** (`docs/re-needs/`), on the
/// vendored twin of the crackme it was recorded against.
///
/// A MinGW PE reaches an import through two addresses that both carry its name —
/// the `FF 25` veneer at `0x1400079b0` and the IAT slot at `0x14000d234` — and
/// `kuna functions --filter VirtualProtect` shows an agent both. Asking the
/// veneer used to answer `count: 0` although the program plainly calls
/// `VirtualProtect`, because every call site references the slot instead. Both
/// addresses must now answer with the same two real call sites.
#[test]
fn a_pe_import_answers_the_same_at_its_veneer_and_at_its_slot() {
    const VENEER: &str = "0x1400079b0";
    const SLOT: &str = "0x14000d234";
    let Some(at_veneer) = xrefs(&[&pe_imports(), "--to", VENEER, "--json"]) else {
        return;
    };
    let at_slot = xrefs(&[&pe_imports(), "--to", SLOT, "--json"]).expect("the slot query");

    assert_eq!(json_str(&at_veneer, "name").as_deref(), Some("VirtualProtect"), "{at_veneer}");
    assert_eq!(json_int(&at_veneer, "count"), Some(2), "{at_veneer}");
    assert_eq!(json_int(&at_slot, "count"), Some(2), "{at_slot}");

    // The two rows are the real uses of the import, in the two functions whose
    // decompilation calls it: `__write_memory.part.0` calls straight through the
    // slot, `_pei386_runtime_relocator` loads it into a register first.
    for doc in [&at_veneer, &at_slot] {
        for site in ["0x140001a9e", "0x140001cce"] {
            assert!(doc.contains(&format!("\"from_address_hex\": \"{site}\"")), "no {site}:\n{doc}");
        }
        // The veneer's own `jmp [slot]` is the other half of the import, not a
        // caller of it, and must never pad the count.
        assert!(
            !doc.contains(&format!("\"from_address_hex\": \"{VENEER}\"")),
            "the forwarding jump was counted as a reference:\n{doc}"
        );
        assert!(doc.contains("\"aliases\":"), "the alias is not disclosed:\n{doc}");
    }
    // Not merely the same count: the same rows, byte for byte. The `xrefs` array
    // is everything after the query's own `target` block, so slicing there
    // compares the answer without comparing which address was asked for.
    let answer = |doc: &str| doc[doc.find("\"xrefs\"").expect("an xrefs array")..].to_string();
    assert_eq!(answer(&at_veneer), answer(&at_slot), "the two addresses of one import disagree");
}

/// The mirror case, and the reason the unification has to run both ways: `puts`
/// is called through its veneer, so it is the *slot* that would otherwise report
/// no users at all.
#[test]
fn a_slot_reached_only_through_its_veneer_still_finds_its_callers() {
    let Some(doc) = xrefs(&[&pe_imports(), "--to", "0x14000d33c", "--json"]) else {
        return;
    };
    assert_eq!(json_str(&doc, "name").as_deref(), Some("puts"), "{doc}");
    assert_eq!(json_int(&doc, "count"), Some(1), "{doc}");
    assert_eq!(kinds(&doc), vec!["call"], "{doc}");
    assert!(doc.contains("\"from_address_hex\": \"0x1400015a5\""), "not main's call:\n{doc}");
    assert!(doc.contains("\"name\": \"main\""), "call site unattributed:\n{doc}");
}

/// The `name-based-xrefs-rejects` need: asking for the import by NAME must answer
/// the same as asking for either of its addresses.
///
/// Both addresses carry the name, so the selector model called `VirtualProtect`
/// ambiguous and exited 1 — refusing a question that has exactly one answer,
/// while `--to 0x1400079b0` and `--to 0x14000d234` both answered it. The two
/// candidates are one alias class, so the name settles on the class's code half,
/// the veneer, and the slot is disclosed as its alias.
#[test]
fn an_import_named_by_its_veneer_and_its_slot_resolves_by_name() {
    let Some(by_name) = xrefs(&[&pe_imports(), "--to", "VirtualProtect", "--json"]) else {
        return;
    };
    let by_veneer = xrefs(&[&pe_imports(), "--to", "0x1400079b0", "--json"]).expect("the veneer");

    assert_eq!(json_str(&by_name, "address_hex").as_deref(), Some("0x1400079b0"), "{by_name}");
    assert!(by_name.contains("\"0x14000d234\""), "the slot is not disclosed:\n{by_name}");
    // Not merely the same count: the same rows, byte for byte.
    let answer = |doc: &str| doc[doc.find("\"xrefs\"").expect("an xrefs array")..].to_string();
    assert_eq!(answer(&by_name), answer(&by_veneer), "the name and its veneer disagree");
}

/// The mirror: `puts` is reached through its veneer rather than its slot, so the
/// fold must not depend on which half the call sites happen to reference.
#[test]
fn a_name_folds_whichever_half_of_the_import_is_called() {
    let Some(doc) = xrefs(&[&pe_imports(), "--to", "puts", "--json"]) else {
        return;
    };
    assert_eq!(json_str(&doc, "address_hex").as_deref(), Some("0x140007240"), "{doc}");
    assert_eq!(json_int(&doc, "count"), Some(1), "{doc}");
    assert!(doc.contains("\"from_address_hex\": \"0x1400015a5\""), "not main's call:\n{doc}");
}

/// The fold is the alias class and nothing looser. Two static `duplicate_local`
/// definitions in one relocatable object share a name and no forwarding jump, so
/// the query still refuses them and still names both — picking one would be the
/// guess the selector model exists to refuse.
#[test]
fn two_genuinely_distinct_functions_of_one_name_are_still_refused() {
    let object = fixture("entry_selectors_x86_64.o");
    let (_, stderr, code) = run_kuna(&["xrefs", &object, "--to", "duplicate_local"]);
    if is_specs_skip(&stderr) {
        return;
    }
    assert_eq!(code, 1, "{stderr}");
    assert!(stderr.contains("ambiguous"), "{stderr}");
    assert!(stderr.contains(".text.selector_a+0x0"), "{stderr}");
    assert!(stderr.contains(".text.selector_b+0x0"), "{stderr}");
}

/// The alias class is built from the decoded forwarding jump, never from a shared
/// name: an ordinary function keeps an empty `aliases` list and its own answer.
#[test]
fn an_ordinary_function_is_never_aliased_to_anything() {
    let Some(doc) = xrefs(&[&fauxware(), "--to", "authenticate", "--json"]) else {
        return;
    };
    assert!(doc.contains("\"aliases\": []"), "authenticate acquired an alias:\n{doc}");
    assert_eq!(json_int(&doc, "count"), Some(1), "{doc}");
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

/// A reference query seeds its walk with the address it was ASKED about.
///
/// `0x800039c` has no inbound `bl`, no funcsym and no paired prologue, so the
/// recursive descent structurally cannot reach it and the answer used to be
/// `count: 0` about a function that plainly calls something. It is walked after
/// the seeded descent drains, so it can only add coverage — and it is answered
/// even with the speculative gap-walk off, because the caller naming an address
/// is a stronger fact than a fingerprint match.
#[test]
fn a_function_no_descent_reaches_still_answers_for_itself() {
    let Some(doc) = xrefs(&[&cortexm_gap(), "--from", "0x800039c", "--json"]) else {
        return;
    };
    assert_eq!(json_int(&doc, "count"), Some(1), "the focus seed did not walk:\n{doc}");
    assert_eq!(json_str(&doc, "name").as_deref(), Some("sub_800039c"), "{doc}");
    assert!(doc.contains("\"to_address_hex\": \"0x8000160\""), "wrong callee:\n{doc}");
    assert!(doc.contains("\"from_address_hex\": \"0x80003a0\""), "wrong call site:\n{doc}");

    let off = xrefs(&[&cortexm_gap(), "--from", "0x800039c", "--json", "--option", "aif", "off"])
        .expect("the gap-walk-off query");
    assert_eq!(json_int(&off, "count"), Some(1), "the focus seed needs the gap-walk:\n{off}");
}

/// The speculative gap-walk runs over the partition the reference walk itself
/// leaves behind, so the call sites inside a function only it discovers are in
/// the answer — without the analysis-tier Listing decoding the program a second
/// time to produce them.
///
/// `0x8000160` is called twice: once from `0x8000042`, which every descent
/// reaches, and once from `0x80003a0`, which lives inside the data-reachable
/// `0x800039c`. Turning the gap-walk off drops the second one, which is exactly
/// the recall this query would lose by dropping the Listing without replacing it.
#[test]
fn the_gap_walk_finds_call_sites_a_descent_cannot_reach() {
    let Some(doc) = xrefs(&[&cortexm_gap(), "--to", "0x8000160", "--json"]) else {
        return;
    };
    assert_eq!(json_int(&doc, "count"), Some(2), "a caller is missing:\n{doc}");
    for site in ["0x8000042", "0x80003a0"] {
        assert!(doc.contains(&format!("\"from_address_hex\": \"{site}\"")), "no {site}:\n{doc}");
    }

    let off = xrefs(&[&cortexm_gap(), "--to", "0x8000160", "--json", "--option", "aif", "off"])
        .expect("the gap-walk-off query");
    assert_eq!(json_int(&off, "count"), Some(1), "the gap-walk was not the reason:\n{off}");
    assert!(
        !off.contains("\"from_address_hex\": \"0x80003a0\""),
        "the gap-walk-off answer still has the gap call site:\n{off}"
    );
}
