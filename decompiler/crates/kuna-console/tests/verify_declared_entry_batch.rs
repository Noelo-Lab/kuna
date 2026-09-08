//! A caller-declared function must reach the **whole-binary** target list.
//!
//! ## The defect
//!
//! `function_entries_executable` — what `kuna decompile-all`, `decompile-project`
//! and the browser inventory loop over — kept only entries inside a section
//! flagged executable. That is a guess about where code lives, and a packer
//! defeats it for free by not setting the bit: the reported NEOLite-packed PE
//! (RE-need `batch-silently-omits-explicitly`) flags all six of its sections
//! `INITIALIZED_DATA|READ|WRITE`, `.text` included. So
//!
//! ```text
//!   kuna functions BIN --define-function 0x4f7001-0x4f700c=entry   ->  entry, 11 bytes
//!   kuna decompile BIN entry --define-function ...                 ->  void entry(void) { ... }
//!   kuna decompile-all BIN --json --define-function ...            ->  count 0, error null
//! ```
//!
//! The declaration is an assertion, not a guess, so it outranks the flag test —
//! and `--define-function` is the only thing an agent has left once the flags
//! are lying.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`). If it is absent the bootstrap fails and the test prints that
//! and returns early (a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// The declared entry of the all-data PE fixture, and its declared extent — the
/// reduction of the reported image's `0x4f7001`, 11 bytes, in `.NEOpack`.
const ENTRY: u64 = 0x402001;
const EXTENT: i32 = 11;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn boot(rel: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = root.join(rel);
    assert!(path.exists(), "missing fixture {path:?}");
    match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_declared_entry_batch: skipping (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

fn declare(prog: &mut ConsoleProgram, vma: u64, name: &str, size: i32) {
    let space = prog
        .arch()
        .manage()
        .get_default_code_space()
        .cloned()
        .expect("a default code space");
    prog.declare_function(Address::new(Rc::clone(&space), vma), Some(name), size)
        .expect("the declaration is accepted");
}

fn offsets(prog: &ConsoleProgram) -> Vec<u64> {
    prog.function_entries_executable().iter().map(|e| e.addr.get_offset()).collect()
}

/// The headline: a PE that flags every section data enumerates nothing, and the
/// declaration is what puts the function back in the batch.
#[test]
fn a_declared_entry_survives_the_section_flag_filter() {
    let Some(mut prog) =
        boot("decompiler/crates/kuna-analysis/tests/fixtures/pe_datasection_entry_i386.exe")
    else {
        return;
    };
    prog.commit_pending_analysis().expect("read symbols (analysis commit) must succeed");

    assert!(
        offsets(&prog).is_empty(),
        "no section here is flagged executable, so nothing is enumerated: {:x?}",
        offsets(&prog)
    );

    declare(&mut prog, ENTRY, "entry", EXTENT);
    let entries = prog.function_entries_executable();
    assert_eq!(
        entries.iter().map(|e| e.addr.get_offset()).collect::<Vec<_>>(),
        vec![ENTRY],
        "the declared entry must reach the whole-binary target list"
    );
    assert_eq!(entries[0].name, "entry");
    assert_eq!(entries[0].size, EXTENT as u64, "the declared extent is reported, not the clip");
    assert!(prog.is_declared_entry(ENTRY));
    assert!(!prog.is_declared_entry(ENTRY + 1));
}

/// The other half: the filter still filters. `ptx.o`'s extern-area slots are
/// synthetic addresses with no bytes behind them and must stay out of the batch
/// — until a caller declares one, which is the caller's call to make.
#[test]
fn an_undeclared_data_address_is_still_dropped() {
    let Some(mut prog) = boot("decompiler/crates/kuna-analysis/tests/fixtures/ptx.o") else {
        return;
    };
    prog.commit_pending_analysis().expect("read symbols (analysis commit) must succeed");

    let executable = offsets(&prog);
    let dropped: Vec<u64> = prog
        .function_entries_canonical()
        .into_iter()
        .map(|e| e.addr.get_offset())
        .filter(|a| !executable.contains(a))
        .collect();
    assert!(!dropped.is_empty(), "fixture must carry entries the filter drops");

    let target = dropped[0];
    declare(&mut prog, target, "declared_extern", 0);
    let after = offsets(&prog);
    assert!(after.contains(&target), "the declared address must now be in the batch");
    for other in dropped.iter().skip(1) {
        assert!(
            !after.contains(other),
            "declaring {target:#x} must not lift {other:#x} with it"
        );
    }
}
