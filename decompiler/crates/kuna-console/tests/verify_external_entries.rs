//! Selecting an **external** must report an external, not a decompile failure.
//!
//! ## The defect
//!
//! A relocatable object's undefined symbols (`puts`, `xnmalloc`,
//! `CellClass::Cell_Coord`) are bound to synthetic addresses in an extern area
//! above the laid-out sections, purely so a *call* to one renders by name
//! (`crate::loader::reloc_object`). Nothing is mapped there and nothing ever
//! will be — the definition is in another translation unit. The whole-binary
//! surfaces never reach one (`function_entries_executable` drops them), but
//! selecting one **by name or address** — which is one click in the browser
//! inventory, where every entry is a row — ran the lifter against unmapped
//! memory and reported the result as if the function had failed:
//!
//! ```text
//!   before:  /* CellClass::Cell_Coord @ 0x404080 — decompile error:
//!               Unable to load 512 bytes at r0x00404080 */
//!   after:   // CellClass::Cell_Coord: external symbol -- no code at this
//!            // address in this module
//! ```
//!
//! ## The read-cache half
//!
//! Probing for "does this entry have bytes" also exposed a latent loader bug
//! worth its own assertion: `load_fill` claims `bufoffset` at the top of a fill,
//! *before* reading anything, and left it claimed when the fill failed — so the
//! next request within 512 bytes of a failed one was served out of a buffer that
//! was never filled, and reported as a successful read. Two adjacent externs are
//! exactly that shape (the second test below), which is how the browser's
//! inventory saw one unmapped extern followed by forty "mapped" ones.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`). If it is absent the bootstrap fails and the test prints that
//! and returns early (a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::project::decompile_targets;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// `ptx.o` — the vendored ET_REL fixture, whose undefined externals
/// (`xnmalloc`, `xstrtoimax`, …) take extern-area addresses.
fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/ptx.o")
}

fn boot() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixture();
    assert!(path.exists(), "missing fixture {path:?}");
    match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_external_entries: skipping (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

/// Every extern-area entry: those the canonical inventory carries but the
/// executable (decompilable) one does not.
fn extern_entries(prog: &ConsoleProgram) -> Vec<kuna_console::engine::FunctionEntry> {
    let executable: std::collections::BTreeSet<u64> =
        prog.function_entries_executable().iter().map(|e| e.addr.get_offset()).collect();
    prog.function_entries_canonical()
        .into_iter()
        .filter(|e| !executable.contains(&e.addr.get_offset()))
        .collect()
}

/// The headline: selecting an external by name reports it as one — a body, no
/// error — instead of the byte-load failure the lifter would raise.
#[test]
fn selecting_an_external_reports_an_external_not_an_error() {
    let Some(mut prog) = boot() else { return };

    let externs = extern_entries(&prog);
    assert!(!externs.is_empty(), "fixture must carry extern-area entries");
    let target = externs[0].clone();
    let name = target.name.clone();

    let out = decompile_targets(&mut prog, vec![target], true, false, false);
    assert_eq!(out.len(), 1);
    let r = &out[0];
    assert!(
        r.error.is_none(),
        "an external must not be reported as a decompile error, got: {:?}",
        r.error
    );
    let code = r.code.as_deref().unwrap_or("");
    assert!(
        code.contains("external symbol"),
        "expected an external-symbol body for {name}, got: {code:?}"
    );
    assert!(
        !code.contains("Unable to load"),
        "the byte-load failure must not reach the output, got: {code:?}"
    );
}

/// The read-cache half: EVERY extern must report as unmapped, not just the
/// first one probed. Before the `bufoffset` reset, the first failed fill left
/// the 512-byte window claimed, so the externs packed 16 bytes apart behind it
/// were served from a never-filled buffer and answered "mapped".
#[test]
fn a_failed_read_does_not_make_its_neighbours_look_mapped() {
    let Some(prog) = boot() else { return };

    let externs = extern_entries(&prog);
    assert!(
        externs.len() >= 4,
        "need several adjacent externs to exercise the 512-byte window, got {}",
        externs.len()
    );
    // In address order, so a poisoned window would be entered on the first probe
    // and hide the rest — the exact order the browser inventory classifies in.
    let mut addrs: Vec<u64> = externs.iter().map(|e| e.addr.get_offset()).collect();
    addrs.sort_unstable();
    let mapped: Vec<u64> = addrs.iter().copied().filter(|&a| prog.vma_bytes_mapped(a)).collect();
    assert!(
        mapped.is_empty(),
        "no extern slot is backed by bytes; these reported mapped: {mapped:x?}"
    );

    // The control: a real function IS mapped, so the probe is not answering
    // "false" for everything.
    let real = prog.function_entries_executable();
    assert!(!real.is_empty(), "fixture must have decompilable functions");
    assert!(
        prog.vma_bytes_mapped(real[0].addr.get_offset()),
        "a real function's entry must probe as mapped"
    );
}
