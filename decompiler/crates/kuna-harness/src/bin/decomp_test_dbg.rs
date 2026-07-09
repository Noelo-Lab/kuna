//! The Rust `decomp_test_dbg`: the unit-test + XML datatest harness (the same
//! command surface as the C++ binary of the same name).
//!
//! Port of `decompiler/cpp/test.cc` `main()`:
//!
//! ```text
//!   parse -path <datatestdir> / -sleighpath <sleighdir> / -usesleighenv
//!   parse the `unittests`/`datatests` selector (+ optional test names)
//!   startDecompilerLibrary(sleighdir)
//!   if runUnitTests: UnitTest::run(names)            -> stderr grammar
//!   if runDataTests: gatherDataTests + FunctionTestCollection::runTestFiles(s)
//!                                                     -> stdout grammar
//!   return failedTests (clamped at 255 by add_exit_code)
//! ```
//!
//! The two output streams the Python harness (`kuna/run_tests.py`) parses:
//!   - **stderr**: `testing : <name> ...` / `  passed.` / `  fail` /
//!     `N/M tests passed.` (the unit-test grammar).
//!   - **stdout**: `Success -- <name>` / `FAIL -- <name>` /
//!     `Total tests applied = N` / `Total passing tests = N` (the datatest
//!     grammar) — produced by the ported [`run_test_files_with_specs`].
//!
//! The unit-test registry is the documented boundary: the engine `TEST(...)` bodies
//! live in the crates' own `#[test]` functions (run by `cargo test`), not a
//! constructor-time global registry, so an empty [`UnitTestSuite`] is run here
//! (it prints the faithful `0/0 tests passed.` footer).  The DATATEST runner is
//! fully wired (the `w9x-arch-engine-glue` engine assembly drives `load
//! function`/`decompile`/`print C`), which is what this item's gate exercises.

use std::path::Path;
use std::process::ExitCode;

use kuna_harness::testfunction::run_test_files_with_specs;
use kuna_harness::unittest::{add_exit_code, UnitTestSuite};

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();

    let mut run_unit_tests = true;
    let mut run_data_tests = true;
    let mut unit_test_names: Vec<String> = Vec::new();
    let mut data_test_names: Vec<String> = Vec::new();
    // C++ defaults; the Python harness always passes -path and -sleighpath.
    let mut dirname = String::from("../datatests");
    let mut spec_roots: Vec<String> = Vec::new();

    // C++ `argc -= 1; argv += 1;` — skip the program name.
    let mut i = 1usize;
    while i < argv.len() {
        let command = argv[i].as_str();
        match command {
            "-path" => {
                if i + 1 >= argv.len() {
                    return usage();
                }
                dirname = argv[i + 1].clone();
                run_data_tests = true;
                i += 2;
            }
            "-sleighpath" => {
                if i + 1 >= argv.len() {
                    return usage();
                }
                spec_roots.push(argv[i + 1].clone());
                i += 2;
            }
            "-usesleighenv" => {
                if let Ok(home) = std::env::var("SLEIGHHOME") {
                    println!("Using SLEIGHHOME={home}");
                    spec_roots.push(home);
                } else {
                    println!("No SLEIGHHOME environment variable");
                }
                i += 1;
            }
            "unittests" => {
                run_unit_tests = true;
                run_data_tests = false; // Run only unit tests
                unit_test_names.extend(argv[i + 1..].iter().cloned());
                break;
            }
            "datatests" => {
                run_unit_tests = false; // Run only data-tests
                run_data_tests = true;
                data_test_names.extend(argv[i + 1..].iter().cloned());
                break;
            }
            _ => return usage(),
        }
    }

    // SLEIGHHOME is the load-bearing spec-root the Python harness sets even
    // without -usesleighenv; honor it (kuna/run_tests.py sets it indirectly via
    // -sleighpath, but allow either).
    if let Ok(home) = std::env::var("SLEIGHHOME") {
        if !home.is_empty() && !spec_roots.contains(&home) {
            spec_roots.push(home);
        }
    }

    // startDecompilerLibrary(sleighdirname) — the id/capability bootstrap is
    // per-program inside `engine::bootstrap_*`; the roots are recorded here.

    let mut failed_tests = 0i32;

    if run_unit_tests {
        // The engine unit tests run under `cargo test`; the harness binary runs an
        // empty suite (the documented registry boundary) and prints the faithful
        // `0/0 tests passed.` footer the Python harness greps off stderr.
        let suite = UnitTestSuite::new();
        let mut buf = String::new();
        let errors = suite.run(unit_test_names.clone(), &mut buf);
        eprint!("{buf}");
        failed_tests = add_exit_code(failed_tests, errors);
    }

    if run_data_tests {
        let test_files = gather_data_tests(&dirname, &data_test_names);
        // C++ `cout << endl << endl;`
        println!();
        println!();
        let mut buf = String::new();
        let errors = run_test_files_with_specs(&test_files, &spec_roots, &mut buf);
        print!("{buf}");
        failed_tests = add_exit_code(failed_tests, errors);
    }

    ExitCode::from(failed_tests.clamp(0, 255) as u8)
}

/// C++ usage line + the `-1` (=255) exit.
fn usage() -> ExitCode {
    println!(
        "USAGE: ghidra_test [-usesleighenv] [-sleighpath <sleighdir>] [-path <datatestdir>] \
         [[unittests|datatests] [testname1 testname2 ...]]"
    );
    ExitCode::from(255)
}

/// C++ `gatherDataTests(dirname, testNames, testFiles)`: build the list of
/// absolute paths of the `.xml` test files to run.
///
/// If `test_names` is non-empty, only the files whose absolute path matches
/// `<dirname>/<name>` are taken; otherwise every `.xml` in `dirname` is taken.
fn gather_data_tests(dirname: &str, test_names: &[String]) -> Vec<String> {
    // fullNames = { dirname + '/' + name : name in testNames }
    let mut full_names: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for name in test_names {
        let mut val = dirname.to_string();
        if !val.ends_with('/') {
            val.push('/');
        }
        val.push_str(name);
        full_names.insert(val);
    }

    // matchList(testFiles, ".xml", true): every *.xml under dirname, sorted.
    let mut all_xml: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dirname) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("xml") {
                if let Some(s) = path.to_str() {
                    all_xml.push(s.to_string());
                }
            }
        }
    }
    all_xml.sort();

    if full_names.is_empty() {
        return all_xml; // Take all test files
    }
    // Take tests matching into the list (compare on the joined dirname/name and
    // on the bare basename, so either form the caller passes resolves).
    all_xml
        .into_iter()
        .filter(|f| {
            full_names.contains(f)
                || Path::new(f)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|base| test_names.iter().any(|n| n == base))
                    .unwrap_or(false)
        })
        .collect()
}
