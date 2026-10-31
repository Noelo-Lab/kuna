//! End-to-end gate for Go `pclntab` function-name recovery (`pclntab`, the
//! kuna analog of Ghidra's `GolangSymbolAnalyzer` name-recovery half).
//!
//! The proof: a Go binary's functions render NAMED (`main.main`, `main.compute`,
//! `runtime.*`) instead of `sub_<addr>`, recovered from the embedded pclntab — and
//! crucially this works even on a STRIPPED Go binary (no `.symtab`), where the
//! pclntab is the *only* name source. Without the pclntab pass a stripped Go
//! `main.main` is just `sub_<addr>`.
//!
//! ## How the fixture is produced
//!
//! Go ELF binaries are ~1 MB (they embed the whole runtime), too large to vendor.
//! Following Increment 15's pattern (the real-Go no-return e2e), this test BUILDS
//! a tiny Go program at runtime in an isolated temp dir with a private
//! `GOCACHE`/`GOPATH` (hermetic — never touches the user's environment), guarded
//! on `go` being on PATH. An installed compiler must build the fixture successfully.
//! The hermetic parser logic is pinned separately by the unit tests in
//! `kuna-analysis/src/pclntab/tests.rs` (no `go` needed), so the merge-blocking
//! gate does not depend on a Go toolchain.

use std::path::PathBuf;
use std::process::{Command, Stdio};

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// `true` if a runnable `go` toolchain is on PATH.
fn go_available() -> bool {
    Command::new("go")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// A small Go program with a named, non-inlined function (`compute`) called from
/// `main`. `//go:noinline` keeps `main.compute` a distinct function with its own
/// pclntab entry. The `os.Args` use defeats dead-code elimination of `compute`.
const MAIN_GO: &str = r#"package main

import (
	"fmt"
	"os"
)

//go:noinline
func compute(x int) int {
	return x*x + len(os.Args)
}

func main() {
	fmt.Println(compute(7))
}
"#;

/// Build `MAIN_GO` into `out_bin` hermetically for `(goos, goarch)`. `strip`
/// toggles `-ldflags=-s -w` (drops the symbol table + DWARF, but KEEPS
/// `.gopclntab`/`__gopclntab`). Returns `true` on a successful build that produced
/// the binary, `false` for any failure. Cross-compiling is pure-Go work
/// (`CGO_ENABLED=0`), so a Linux runner can build the darwin/arm64 fixture.
fn build_go(
    dir: &std::path::Path,
    out_bin: &std::path::Path,
    goos: &str,
    goarch: &str,
    strip: bool,
) -> bool {
    build_go_flags(dir, out_bin, goos, goarch, if strip { &["-ldflags=-s -w"] } else { &[] })
}

/// [`build_go`] with explicit `go build` flags.
fn build_go_flags(
    dir: &std::path::Path,
    out_bin: &std::path::Path,
    goos: &str,
    goarch: &str,
    flags: &[&str],
) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    if std::fs::write(dir.join("main.go"), MAIN_GO).is_err() {
        return false;
    }
    let mut cmd = Command::new("go");
    // Flags MUST precede the package argument (`main.go`); `-ldflags=-s -w` drops
    // the symbol table + DWARF but keeps the pclntab.
    cmd.arg("build").args(flags);
    cmd.arg("-o")
        .arg(out_bin)
        .arg("main.go")
        .current_dir(dir)
        .env("GOOS", goos)
        .env("GOARCH", goarch)
        .env("GOCACHE", dir.join(".gocache"))
        .env("GOPATH", dir.join(".gopath"))
        .env("CGO_ENABLED", "0")
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    matches!(cmd.status(), Ok(s) if s.success()) && out_bin.exists()
}

/// Bootstrap a binary, commit the deferred analysis (so the pclntab pass's facts land), and
/// return the `ConsoleProgram`.
fn bootstrap(bin: &str) -> kuna_console::engine::ConsoleProgram {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    // The analysis-pass facts are committed at `read symbols` (gated by the per-pass
    // flags), not eagerly at bootstrap — trigger that commit here.
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

/// `load function <func>` → `decompile` → `print C` over an already-committed
/// program, returning the console transcript (which carries the C).
fn decompile(prog: kuna_console::engine::ConsoleProgram, func: &str) -> String {
    let cmds: Vec<String> = [format!("load function {func}"), "decompile".into(), "print C".into()].to_vec();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// The headline e2e: a STRIPPED Go binary (no `.symtab`) recovers `main.main` and
/// `main.compute` as NAMED functions purely from the pclntab. This is the BEFORE
/// (`sub_<addr>`) / AFTER (`main.main`, `main.compute`) proof — on a stripped
/// binary the pclntab is the only possible name source, so a recovered name can
/// only have come from this pass.
#[test]
fn stripped_go_functions_recovered_from_pclntab() {
    if !go_available() {
        eprintln!("verify_go_pclntab: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_pclntab_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin, "linux", "amd64", /* strip */ true) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path").to_string();

    let prog = bootstrap(&bin);

    // AFTER: the user functions are recovered by name from the pclntab. On a
    // stripped binary there is NO `.symtab`, so these names can ONLY come from the
    // pclntab pass (the BEFORE state would be a bare `sub_<addr>`).
    assert!(
        prog.lookup_symbol("main.main").is_some(),
        "main.main not recovered from the pclntab (stripped binary, no .symtab)"
    );
    assert!(
        prog.lookup_symbol("main.compute").is_some(),
        "main.compute not recovered from the pclntab (stripped binary, no .symtab)"
    );
    // A headline runtime function is recovered too (proves the runtime.* set, not
    // just the user package, is named).
    assert!(
        prog.lookup_symbol("runtime.main").is_some(),
        "runtime.main not recovered from the pclntab"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// The gating proof: with `--option gopclntab off` the recovery is suppressed, so
/// the same stripped binary's `main.compute` is NOT named (it falls back to
/// `sub_<addr>`). This pins that the names came from the pass, not some other
/// seam, and that the flag actually gates it.
#[test]
fn gopclntab_off_suppresses_recovery() {
    if !go_available() {
        eprintln!("verify_go_pclntab: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_pclntab_off_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin, "linux", "amd64", /* strip */ true) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path").to_string();

    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(&bin, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    // Turn the pass OFF before the deferred commit consults the flag.
    prog.arch_mut()
        .set_kuna_option("gopclntab", "off")
        .expect("gopclntab off applies");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    // With the pass gated off, the pclntab names are NOT installed on this stripped
    // binary — `main.compute` has no symbol.
    assert!(
        prog.lookup_symbol("main.compute").is_none(),
        "gopclntab off must suppress pclntab name recovery (main.compute should be absent)"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// An internally linked ELF PIE. go1.25 and older put its table in
/// `.data.rel.ro.gopclntab` (the relro spelling of `.gopclntab`, go1.26+ moved it
/// back), so on those toolchains the names come only from that section.
#[test]
fn stripped_go_pie_functions_recovered_from_pclntab() {
    if !go_available() {
        eprintln!("verify_go_pclntab: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_pclntab_pie_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped_pie");
    if !build_go_flags(&dir, &out_bin, "linux", "amd64", &["-buildmode=pie", "-ldflags=-s -w"]) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the PIE fixture");
    }
    let prog = bootstrap(out_bin.to_str().expect("utf8 path"));
    for name in ["main.main", "main.compute", "runtime.main"] {
        assert!(prog.lookup_symbol(name).is_some(), "{name} not recovered from a PIE pclntab");
    }
    let c = decompile(prog, "main.main");
    assert!(c.contains("main.compute("), "PIE main.main must call main.compute by name:\n{c}");

    let _ = std::fs::remove_dir_all(&dir);
}

/// The Mach-O arm (the pass used to return before parsing). `GOOS=darwin GOARCH=arm64`
/// is a pure-Go cross build, so a Linux runner covers it. Also pins the *extent*:
/// `main.main` must not be clipped, and nothing may be claimed inside it (an arm64
/// Go jump-table case block used to become a `sub_<addr>` of its own there).
#[test]
fn stripped_go_macho_functions_recovered_from_pclntab() {
    if !go_available() {
        eprintln!("verify_go_pclntab: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_pclntab_macho_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped_macho");
    if !build_go(&dir, &out_bin, "darwin", "arm64", /* strip */ true) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to cross-build the darwin/arm64 fixture");
    }
    let prog = bootstrap(out_bin.to_str().expect("utf8 path"));

    for name in ["main.main", "main.compute", "runtime.main"] {
        assert!(prog.lookup_symbol(name).is_some(), "{name} not recovered from a Mach-O pclntab");
    }
    let entries = prog.function_entries_canonical();
    let main = entries.iter().find(|e| e.name == "main.main").expect("main.main in inventory");
    let (start, end) = (main.addr.get_offset(), main.addr.get_offset() + main.size);
    assert!(main.size > 32, "main.main clipped to {} bytes", main.size);
    let interior: Vec<&str> = entries
        .iter()
        .filter(|e| e.addr.get_offset() > start && e.addr.get_offset() < end)
        .map(|e| e.name.as_str())
        .collect();
    assert!(interior.is_empty(), "entries claimed inside main.main: {interior:?}");

    let _ = std::fs::remove_dir_all(&dir);
}

/// The PE arm. A `go build` for Windows carries no Go-named section at all: the
/// build-info blob heads `.data` and the pclntab rides inside `.rdata`, so the
/// image is only recognized as Go, and its table only found, by reading
/// content. Cross-built pure-Go for each Windows architecture kuna decodes; on
/// each, the names must come back and `main.main` must decompile calling
/// `main.compute` by name. On amd64 the Go-gated rules must act too: the
/// stack-growth guard folds away (`gomorestack auto`, keyed on the detected
/// language).
#[test]
fn stripped_go_pe_functions_recovered_from_pclntab() {
    if !go_available() {
        eprintln!("verify_go_pclntab: skipping (`go` not on PATH)");
        return;
    }
    for goarch in ["amd64", "386", "arm64"] {
        let dir = std::env::temp_dir().join(format!("kuna_go_pclntab_pe_{goarch}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let out_bin = dir.join("go_stripped.exe");
        if !build_go(&dir, &out_bin, "windows", goarch, /* strip */ true) {
            let _ = std::fs::remove_dir_all(&dir);
            panic!("installed Go compiler failed to cross-build the windows/{goarch} fixture");
        }
        let prog = bootstrap(out_bin.to_str().expect("utf8 path"));
        for name in ["main.main", "main.compute", "runtime.main"] {
            assert!(prog.lookup_symbol(name).is_some(), "{name} not recovered from a windows/{goarch} pclntab");
        }
        let c = decompile(prog, "main.main");
        assert!(c.contains("main.compute("), "windows/{goarch} main.main must call main.compute by name:\n{c}");
        if goarch == "amd64" {
            assert!(!c.contains("morestack"), "windows/amd64 main.main keeps its stack-growth guard:\n{c}");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}

/// `gowalkprune`: the fast inventory walk leaves the functions the pclntab names
/// to the table. On a stripped Go ELF the walk roots are pclntab functions found
/// by the entry oracles; on a Go PE every pclntab function with a `.pdata` record
/// is one. With the option off the walk follows calls through the Go runtime,
/// and a function it reaches only that way gains a `sub_<addr>` alias beside its
/// pclntab name; on, those are not walked, and only the roots keep the alias
/// their entry source gave them. The inventory (addresses, names, sizes) is the
/// same either way. With `gopclntab` off nothing commits the table's functions,
/// so the walk keeps every root.
#[test]
fn fast_inventory_walk_leaves_pclntab_functions_to_the_table() {
    if !go_available() {
        eprintln!("verify_go_pclntab: skipping (`go` not on PATH)");
        return;
    }
    for (goos, file) in [("linux", "go_stripped"), ("windows", "go_stripped.exe")] {
        let dir = std::env::temp_dir().join(format!("kuna_go_walkprune_{goos}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let out_bin = dir.join(file);
        if !build_go(&dir, &out_bin, goos, "amd64", /* strip */ true) {
            let _ = std::fs::remove_dir_all(&dir);
            panic!("installed Go compiler failed to build the {goos}/amd64 fixture");
        }
        let bin = out_bin.to_str().expect("utf8 path").to_string();
        let specs = repo_root().join("specs");
        let spec_roots = vec![specs.to_str().unwrap().to_string()];
        let inventory = |options: &[(&str, &str)]| {
            let mut prog = bootstrap_from_object(&bin, "", &spec_roots)
                .expect("bootstrap fixture with built processor specs");
            for &(option, value) in [("fast_funcdisc", "on")].iter().chain(options) {
                prog.arch_mut().set_kuna_option(option, value).expect("option applies");
            }
            prog.commit_pending_analysis().expect("analysis commit succeeds");
            prog.function_entries_canonical()
        };
        let walked = inventory(&[("gowalkprune", "off")]);
        let pruned = inventory(&[("gowalkprune", "on")]);

        let shape = |entries: &[kuna_console::engine::FunctionEntry]| {
            entries.iter().map(|e| (e.addr.get_offset(), e.name.clone(), e.size)).collect::<Vec<_>>()
        };
        assert_eq!(shape(&walked), shape(&pruned), "{goos}: the inventory must not change");
        assert!(walked.iter().any(|e| e.name == "main.compute"), "{goos}: pclntab names recovered");
        let generated_aliases = |entries: &[kuna_console::engine::FunctionEntry]| {
            entries
                .iter()
                .filter(|e| !e.name.starts_with("sub_") && e.aliases.iter().any(|a| a.starts_with("sub_")))
                .map(|e| e.addr.get_offset())
                .collect::<std::collections::BTreeSet<u64>>()
        };
        let (walked_aliases, pruned_aliases) = (generated_aliases(&walked), generated_aliases(&pruned));
        assert!(
            pruned_aliases.is_subset(&walked_aliases) && pruned_aliases.len() < walked_aliases.len(),
            "{goos}: gowalkprune on must not walk the pclntab functions the roots call \
             ({} aliased pruned, {} walked)",
            pruned_aliases.len(),
            walked_aliases.len()
        );
        let tableless = |prune| shape(&inventory(&[("gopclntab", "off"), ("gowalkprune", prune)]));
        assert_eq!(tableless("on"), tableless("off"), "{goos}: without gopclntab every root is walked");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
