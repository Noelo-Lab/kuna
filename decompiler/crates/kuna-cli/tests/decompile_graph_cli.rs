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

fn missing_specs(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

/// Run the command, or `None` as a visible skip when the `.sla` files are absent.
fn graph(args: &[&str]) -> Option<String> {
    run(args).map(|(stdout, _)| stdout)
}

/// [`graph`], keeping stderr, for the warnings the document itself cannot carry.
fn run(args: &[&str]) -> Option<(String, String)> {
    let specs = specs();
    let mut argv = vec!["decompile-graph"];
    argv.extend_from_slice(args);
    argv.extend_from_slice(&["--sleighpath", &specs]);
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(&argv)
        .output()
        .expect("spawn kuna decompile-graph");
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() && missing_specs(&stderr) {
        eprintln!("decompile_graph_cli: skipping (no `.sla`; run `make specs`): {stderr}");
        return None;
    }
    assert!(output.status.success(), "decompile-graph failed: {stderr}");
    Some((String::from_utf8_lossy(&output.stdout).into_owned(), stderr.into_owned()))
}

/// One `"key": value` of a rendered object, unquoted — enough to walk this
/// document without a JSON dependency the CLI does not have.
fn field(object: &str, key: &str) -> Option<String> {
    let at = object.find(&format!("\"{key}\":"))? + key.len() + 3;
    let rest = object[at..].trim_start();
    if let Some(body) = rest.strip_prefix('"') {
        return Some(body[..body.find('"')?].to_string());
    }
    let end = rest.find([',', '\n', '}']).unwrap_or(rest.len());
    Some(rest[..end].trim().to_string())
}

/// Split the `functions` / `edges` arrays into their top-level objects. The
/// documents are `dumps_indent2`-rendered, so a row starts at a `    {` line and
/// ends at the matching `    }`.
fn rows(document: &str, array: &str) -> Vec<String> {
    let start = document.find(&format!("\"{array}\": [")).expect("array present");
    let body = &document[start..];
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    for line in body.lines().skip(1) {
        if line == "    {" {
            current = Some(String::new());
        } else if line == "    }" || line == "    }," {
            if let Some(row) = current.take() {
                out.push(row);
            }
        } else if line == "  ]," || line == "  ]" {
            break;
        } else if let Some(row) = current.as_mut() {
            row.push_str(line);
            row.push('\n');
        }
    }
    out
}

#[test]
fn the_document_carries_the_schema_and_both_arrays() {
    let Some(stdout) = graph(&[&fixture("fauxware"), "--label", "fixture-label"]) else { return };
    assert!(stdout.starts_with("{\n"), "not JSON: {stdout}");
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
    if !output.status.success() && missing_specs(&stderr) {
        return;
    }
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
    let Some(stdout) = graph(&[&fixture("pe_imports.exe")]) else { return };
    let rows = rows(&stdout, "functions");
    let imports: Vec<&String> =
        rows.iter().filter(|r| field(r, "kind").as_deref() == Some("import")).collect();
    assert!(
        !rows.iter().any(|r| field(r, "kind").as_deref() == Some("data")),
        "every bodyless row in this fixture is a named import, not a data symbol"
    );
    assert!(!imports.is_empty(), "the fixture's import slots vanished from the inventory");
    for row in &imports {
        assert_eq!(field(row, "codeC").as_deref(), Some("null"), "invented a body:\n{row}");
        assert_eq!(field(row, "assembly").as_deref(), Some("null"), "invented a listing:\n{row}");
        assert_eq!(field(row, "error").as_deref(), Some("null"), "reported a failure:\n{row}");
    }
    assert!(
        imports.iter().any(|r| field(r, "name").as_deref() == Some("DeleteCriticalSection")),
        "DeleteCriticalSection is a KERNEL32 import slot, not a function of this program"
    );
    // Every other row's body and listing arrive together.
    for row in &rows {
        if field(row, "codeC").as_deref() != Some("null") {
            assert_ne!(field(row, "assembly").as_deref(), Some("null"), "C without asm:\n{row}");
        }
    }
}

/// Both ends of every edge must be rows of this document, or a consumer cannot
/// build the graph without a containment model of its own.
#[test]
fn every_edge_endpoint_is_a_function_row() {
    for name in ["pe_imports.exe", "fauxware", "plt_ppc64le"] {
        let Some(stdout) = graph(&[&fixture(name)]) else { return };
        let known: BTreeSet<String> =
            rows(&stdout, "functions").iter().filter_map(|r| field(r, "address")).collect();
        for edge in rows(&stdout, "edges") {
            for end in ["callerAddress", "calleeAddress"] {
                let address = field(&edge, end).expect("edge endpoint");
                assert!(known.contains(&address), "{name}: {end} {address} is not a function");
            }
            assert!(
                matches!(field(&edge, "kind").as_deref(), Some("call" | "jump" | "data")),
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
    let Some(first) = graph(&[&fixture("pe_imports.exe")]) else { return };
    let Some(second) = graph(&[&fixture("pe_imports.exe")]) else { return };
    assert_eq!(first, second, "two runs disagreed");
}

/// The address-taken edge. `_start` never calls `main`; it loads its address and
/// hands it to `__libc_start_main`, which is the shape of every glibc program.
/// Dropping that reference leaves the one function an analyst opens the document
/// for with no caller at all.
#[test]
fn an_address_taken_callee_is_still_an_edge() {
    let Some(stdout) = graph(&[&fixture("fauxware")]) else { return };
    let functions = rows(&stdout, "functions");
    let address = |name: &str| {
        functions
            .iter()
            .find(|r| field(r, "name").as_deref() == Some(name))
            .and_then(|r| field(r, "address"))
            .unwrap_or_else(|| panic!("{name} is not a row"))
    };
    let (start, main) = (address("_start"), address("main"));
    let edge = rows(&stdout, "edges")
        .into_iter()
        .find(|e| {
            field(e, "callerAddress").as_deref() == Some(&start)
                && field(e, "calleeAddress").as_deref() == Some(&main)
        })
        .expect("_start -> main is not in the edge list");
    assert_eq!(field(&edge, "kind").as_deref(), Some("data"), "wrong kind: {edge}");
}

/// `--addr` selects which bodies are rendered, never whether the target policy
/// applies: a PE import slot named outright is still a labelled row, and the run
/// says on stderr that it exported no body for it.
#[test]
fn an_explicitly_named_import_slot_still_gets_no_body() {
    let Some((stdout, stderr)) = run(&[&fixture("pe_imports.exe"), "--addr", "0x14000d1dc"]) else {
        return;
    };
    for row in rows(&stdout, "functions") {
        assert_eq!(field(&row, "codeC").as_deref(), Some("null"), "invented a body:\n{row}");
        assert_eq!(field(&row, "assembly").as_deref(), Some("null"), "invented a listing:\n{row}");
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
        if missing_specs(&stderr) {
            return;
        }
        assert!(!output.status.success(), "{flag:?} was accepted: {stderr}");
        assert!(stderr.contains("C-only"), "{flag:?} failed for another reason: {stderr}");
    }
}

/// A rustc-built binary trips the auto-language policy on every other
/// whole-binary surface. It must not trip it here, or a Rust program's document
/// would carry Rust in `codeC` with no flag given at all.
#[test]
fn a_rust_binary_is_still_exported_as_c() {
    let Some(stdout) = graph(&[&fixture("rust_hello_x86_64")]) else { return };
    let bodies: Vec<String> = rows(&stdout, "functions")
        .into_iter()
        .filter_map(|r| field(&r, "codeC"))
        .filter(|c| c != "null")
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
    let Some(document) = graph(&[&fixture("macho_stripped_main")]) else { return };
    let flagged: Vec<String> = rows(&document, "functions")
        .into_iter()
        .filter(|row| field(row, "isEntryPoint").as_deref() == Some("true"))
        .filter_map(|row| field(&row, "name"))
        .collect();
    assert_eq!(flagged, vec!["main".to_string()], "exactly the LC_MAIN entry: {document}");
}
