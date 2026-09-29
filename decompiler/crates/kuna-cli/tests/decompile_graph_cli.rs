//! CLI contract checks for `kuna decompile-graph`.
//!
//! The ones that carry weight: an address in a PE's import table must not be
//! handed a decompiled body even when it is named explicitly, an address-taken
//! callee must still be an edge (or `main` is an orphan on every glibc ELF),
//! every edge endpoint must be a row in `functions`, `codeC` must be C, and two
//! runs of the same command must produce the same bytes.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

fn specs() -> String {
    repo_root().join("specs").to_string_lossy().into_owned()
}

/// Run the graph command and return its document.
fn graph(args: &[&str]) -> String {
    run(args).0
}

/// [`graph`], keeping stderr, for the warnings the document itself cannot carry.
fn run(args: &[&str]) -> (String, String) {
    let specs = specs();
    let mut argv = vec!["decompile-graph"];
    argv.extend_from_slice(args);
    argv.extend_from_slice(&["--sleighpath", &specs]);
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(&argv)
        .output()
        .expect("spawn kuna decompile-graph");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "decompile-graph failed: {stderr}");
    (String::from_utf8_lossy(&output.stdout).into_owned(), stderr.into_owned())
}

fn rows(document: &str, array: &str) -> Vec<Value> {
    let mut document: Value = serde_json::from_str(document).expect("valid graph JSON");
    match document.as_object_mut().expect("graph object").remove(array).expect("graph array") {
        Value::Array(rows) => rows,
        _ => panic!("{array} must be an array"),
    }
}

#[test]
fn the_document_carries_the_schema_and_both_arrays() {
    let stdout = graph(&[&fixture("fauxware"), "--label", "fixture-label"]);
    let parsed: Value = serde_json::from_str(&stdout).expect("valid graph JSON");
    assert_eq!(parsed.get("schemaVersion").and_then(Value::as_u64), Some(4));
    assert_eq!(parsed.pointer("/binary/label").and_then(Value::as_str), Some("fixture-label"));
    for key in [
        "\"schemaVersion\": 4",
        "\"label\": \"fixture-label\"",
        "\"analysisImageBase\": ",
        "\"functions\": [",
        "\"edges\": [",
        "\"kind\": ",
        "\"error\": ",
        "\"hasIndirectCalls\": ",
        "\"forwardsTo\": ",
        "\"callerAddress\": ",
        "\"calleeAddress\": ",
        "\"calleeOrder\": ",
    ] {
        assert!(stdout.contains(key), "missing {key} from:\n{stdout}");
    }
    assert!(!stdout.contains("\"address\": \"0x"), "addresses must be JSON numbers");
}

#[test]
fn a_file_export_writes_nothing_to_stdout() {
    let path = std::env::temp_dir().join(format!("kuna-decompile-graph-{}.json", std::process::id()));
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-graph",
            &fixture("fauxware"),
            "-o",
            path.to_str().unwrap(),
            "--sleighpath",
            &specs(),
        ])
        .output()
        .expect("spawn kuna decompile-graph");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "decompile-graph failed: {stderr}");
    assert!(output.stdout.is_empty(), "file output must not mix JSON into stdout");
    let document = std::fs::read_to_string(&path).expect("exported JSON file");
    let _ = std::fs::remove_file(path);
    assert!(document.contains("\"schemaVersion\": 4"));
}

/// A PE's import pointer slots are named, callable addresses in `.idata`. They
/// are not code, and lifting them produces a plausible-looking body out of a
/// pointer table — so the row says `import` and carries no C at all, and naming
/// one with `--addr` does not buy an exception
/// ([`an_explicitly_named_import_slot_still_gets_no_body`]).
#[test]
fn a_pe_import_slot_gets_a_label_not_a_body() {
    let stdout = graph(&[&fixture("pe_imports.exe")]);
    let rows = rows(&stdout, "functions");
    let imports: Vec<&Value> =
        rows.iter().filter(|r| r.get("kind").and_then(Value::as_str) == Some("import")).collect();
    assert!(
        !rows.iter().any(|r| r.get("kind").and_then(Value::as_str) == Some("data")),
        "every bodyless row in this fixture is a named import, not a data symbol"
    );
    assert!(!imports.is_empty(), "the fixture's import slots vanished from the inventory");
    for row in &imports {
        assert_eq!(row.get("codeC"), Some(&Value::Null), "invented a body:\n{row}");
        assert_eq!(row.get("assembly"), Some(&Value::Null), "invented a listing:\n{row}");
        assert_eq!(row.get("error"), Some(&Value::Null), "reported a failure:\n{row}");
    }
    assert!(
        imports.iter().any(|r| r.get("name").and_then(Value::as_str) == Some("DeleteCriticalSection")),
        "DeleteCriticalSection is a KERNEL32 import slot, not a function of this program"
    );
    // Every other row's body and listing arrive together.
    for row in &rows {
        if row.get("codeC") != Some(&Value::Null) {
            assert_ne!(row.get("assembly"), Some(&Value::Null), "C without asm:\n{row}");
        }
    }
}

/// Both ends of every edge must be rows of this document, or a consumer cannot
/// build the graph without a containment model of its own.
#[test]
fn every_edge_endpoint_is_a_function_row() {
    for name in ["pe_imports.exe", "fauxware", "plt_ppc64le"] {
        let stdout = graph(&[&fixture(name)]);
        let known: BTreeSet<u64> = rows(&stdout, "functions")
            .iter()
            .map(|row| row.get("address").and_then(Value::as_u64).expect("numeric function address"))
            .collect();
        for edge in rows(&stdout, "edges") {
            for end in ["callerAddress", "calleeAddress"] {
                let address = edge.get(end).and_then(Value::as_u64).expect("edge endpoint");
                assert!(known.contains(&address), "{name}: {end} {address} is not a function");
            }
            assert!(
                matches!(edge.get("kind").and_then(Value::as_str), Some("call" | "jump" | "data")),
                "{name}: unknown edge kind in {edge}"
            );
        }
    }
}

/// Two runs of one command must be byte-identical: the document is an input to
/// diffs and to caches downstream, and unordered iteration would silently break
/// both.
#[test]
fn two_runs_produce_the_same_bytes() {
    let first = graph(&[&fixture("pe_imports.exe")]);
    let second = graph(&[&fixture("pe_imports.exe")]);
    assert_eq!(first, second, "two runs disagreed");
}

/// A sharded graph names every synthesized structure as the serial one does:
/// `fb`, decided again by the convergence sweep, and `fc` share a structure
/// that `fa` cannot reach.
#[test]
fn a_sharded_graph_keeps_the_serial_structure_names() {
    let bin = fixture("structsynthchain_x86_64");
    let serial = graph(&[&bin, "--max-fn-seconds", "0"]);
    assert!(serial.contains("struct_1 *"), "the fixture stopped synthesizing:\n{serial}");
    for pool in [&["--jobs", "2", "--jobs-chunk", "1"][..], &["--jobs", "4"][..]] {
        let args = [&[bin.as_str(), "--max-fn-seconds", "0"][..], pool].concat();
        let (sharded, stderr) = run(&args);
        assert!(stderr.contains("[kuna --jobs] structsynth: "), "{pool:?}:\n{stderr}");
        assert_eq!(sharded, serial, "{pool:?} moved the document");
    }
}

/// The address-taken edge. `_start` never calls `main`; it loads its address and
/// hands it to `__libc_start_main`, which is the shape of every glibc program.
/// Dropping that reference leaves the one function an analyst opens the document
/// for with no caller at all.
#[test]
fn an_address_taken_callee_is_still_an_edge() {
    let stdout = graph(&[&fixture("fauxware")]);
    let functions = rows(&stdout, "functions");
    let address = |name: &str| {
        functions
            .iter()
            .find(|r| r.get("name").and_then(Value::as_str) == Some(name))
            .and_then(|r| r.get("address").and_then(Value::as_u64))
            .unwrap_or_else(|| panic!("{name} is not a row"))
    };
    let (start, main) = (address("_start"), address("main"));
    let edge = rows(&stdout, "edges")
        .into_iter()
        .find(|e| {
            e.get("callerAddress").and_then(Value::as_u64) == Some(start)
                && e.get("calleeAddress").and_then(Value::as_u64) == Some(main)
        })
        .expect("_start -> main is not in the edge list");
    assert_eq!(edge.get("kind").and_then(Value::as_str), Some("data"), "wrong kind: {edge}");
}

/// `forwardsTo` and the edge list are two views of one recovered veneer
/// relation. The function row points at its import slot, and that exact pair is
/// a jump edge so reachability can traverse it. PE already inventories the IAT
/// slot; ELF's loader names only the PLT veneer, so the graph must materialize a
/// GOT-slot import row from the decoded relation.
#[test]
fn forwarding_veneers_are_jump_edges_to_import_rows_on_pe_and_elf() {
    for (binary, veneer, slot) in [
        ("pe_noreturn_import.exe", 0x140001070u64, 0x140005038u64),
        ("aif_gap_x86_64", 0x1030, 0x3ff8),
    ] {
        let stdout = graph(&[&fixture(binary)]);
        let functions = rows(&stdout, "functions");
        let row = functions
            .iter()
            .find(|r| r.get("address").and_then(Value::as_u64) == Some(veneer))
            .unwrap_or_else(|| panic!("{binary}: import veneer row"));
        assert_eq!(
            row.get("forwardsTo").and_then(Value::as_u64),
            Some(slot),
            "{binary}: wrong forwarding row: {row}"
        );
        let slot_row = functions
            .iter()
            .find(|r| r.get("address").and_then(Value::as_u64) == Some(slot))
            .unwrap_or_else(|| panic!("{binary}: import slot row"));
        assert_eq!(slot_row.get("kind").and_then(Value::as_str), Some("import"), "{binary}: {slot_row}");

        let edge = rows(&stdout, "edges")
            .into_iter()
            .find(|e| {
                e.get("callerAddress").and_then(Value::as_u64) == Some(veneer)
                    && e.get("calleeAddress").and_then(Value::as_u64) == Some(slot)
            })
            .unwrap_or_else(|| panic!("{binary}: veneer -> import slot edge"));
        assert_eq!(edge.get("kind").and_then(Value::as_str), Some("jump"), "{binary}: {edge}");

        if binary == "aif_gap_x86_64" {
            let plt0 = 0x1020u64;
            let plt0_slot = 0x3fd0u64;
            let row = functions
                .iter()
                .find(|r| r.get("address").and_then(Value::as_u64) == Some(plt0_slot))
                .expect("ELF PLT0 fixed-slot target row");
            assert_eq!(row.get("kind").and_then(Value::as_str), Some("data"), "PLT0 target: {row}");
            let edge = rows(&stdout, "edges")
                .into_iter()
                .find(|e| {
                    e.get("callerAddress").and_then(Value::as_u64) == Some(plt0)
                        && e.get("calleeAddress").and_then(Value::as_u64) == Some(plt0_slot)
                })
                .expect("ELF PLT0 -> fixed-slot data edge");
            assert_eq!(edge.get("kind").and_then(Value::as_str), Some("jump"), "PLT0 edge: {edge}");
        }
    }
}

/// `--addr` selects which bodies are rendered, never whether the target policy
/// applies: a PE import slot named outright is still a labelled row, and the run
/// says on stderr that it exported no body for it.
#[test]
fn an_explicitly_named_import_slot_still_gets_no_body() {
    let (stdout, stderr) = run(&[&fixture("pe_imports.exe"), "--addr", "0x14000d1dc"]);
    for row in rows(&stdout, "functions") {
        assert_eq!(row.get("codeC"), Some(&Value::Null), "invented a body:\n{row}");
        assert_eq!(row.get("assembly"), Some(&Value::Null), "invented a listing:\n{row}");
    }
    assert!(
        stderr.contains("is not executable content"),
        "the dropped selection was silent: {stderr}"
    );
}

/// `codeC` names its language. `--language rust` reaches this command through
/// the shared parser, so it has to be refused here rather than answered with
/// Rust in a field called `codeC`.
#[test]
fn a_non_c_output_language_is_refused() {
    let (specs, binary) = (specs(), fixture("fauxware"));
    let spelling: [&[&str]; 2] =
        [&["--language", "rust"], &["--option", "setlanguage", "rust-language"]];
    for flag in spelling {
        let mut argv = vec!["decompile-graph", binary.as_str(), "--functions", "main"];
        argv.extend_from_slice(flag);
        argv.extend_from_slice(&["--sleighpath", specs.as_str()]);
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(&argv)
            .output()
            .expect("spawn kuna decompile-graph");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{flag:?} was accepted: {stderr}");
        assert!(stderr.contains("C-only"), "{flag:?} failed for another reason: {stderr}");
    }
}

/// A rustc-built binary trips the auto-language policy on every other
/// whole-binary surface. It must not trip it here, or a Rust program's document
/// would carry Rust in `codeC` with no flag given at all.
#[test]
fn a_rust_binary_is_still_exported_as_c() {
    let stdout = graph(&[&fixture("rust_hello_x86_64")]);
    let bodies: Vec<String> = rows(&stdout, "functions")
        .into_iter()
        .filter_map(|r| r.get("codeC").and_then(Value::as_str).map(str::to_owned))
        .collect();
    assert!(!bodies.is_empty(), "the fixture rendered no bodies at all");
    for body in bodies {
        assert!(!body.starts_with("#[allow("), "Rust in codeC: {body}");
    }
}

/// The graph's `isEntryPoint` roots the document, and it was rooted at the raw
/// Mach-O `LC_MAIN` entryoff -- an address no row carries, so NO row was flagged
/// on any `LC_MAIN` image.
#[test]
fn a_macho_entry_point_row_is_flagged() {
    let document = graph(&[&fixture("macho_stripped_main")]);
    let flagged: Vec<String> = rows(&document, "functions")
        .into_iter()
        .filter(|row| row.get("isEntryPoint").and_then(Value::as_bool) == Some(true))
        .filter_map(|row| row.get("name").and_then(Value::as_str).map(str::to_owned))
        .collect();
    assert_eq!(flagged, vec!["main".to_string()], "exactly the LC_MAIN entry: {document}");
}
