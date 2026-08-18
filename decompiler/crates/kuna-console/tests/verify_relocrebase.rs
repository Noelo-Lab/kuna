//! (kuna `relocrebase`, GH-289) The load-time analysis facts of a relocatable
//! object live in the loaded image's address space.
//!
//! ## The defect this pins
//!
//! `relocobjects` (DIV-8/DIV-70) lays an ELF `ET_REL` `.o` / COFF `.obj` out
//! synthetically above `RELOC_BASE`, so every address the engine holds is a
//! post-layout one.  The load-time analysis passes re-parse the same file through
//! their own `object::File` and computed **pre-link, section-relative** addresses,
//! so the two spaces mixed in one inventory:
//!
//! ```text
//!   before:  kuna functions      ptx.o  →  95 entries (26 real, 27 phantoms at
//!                                          0x0/0x20/0x34/…, 42 externs)
//!            kuna decompile-all  ptx.o  →  26 functions
//!   after:   kuna functions      ptx.o  →  68 entries, none below RELOC_BASE
//!            kuna decompile-all  ptx.o  →  26 functions   (the two agree)
//! ```
//!
//! The phantoms were the `.eh_frame` FDE oracle reading `initial_location` fields
//! whose PC-relative relocation the linker had not applied — each reads back as
//! its own section offset — plus one DWARF `DW_AT_low_pc` that reads 0 for every
//! subprogram, which is also why the whole object's DWARF landed on a single
//! function at address 0.
//!
//! ## `.sla` precondition
//!
//! Like the sibling console gates, this needs the built `x86` `.sla` under
//! `specs/` (gitignored; `make specs`).  If it is absent the bootstrap fails and
//! the test prints a visible skip.

use std::path::PathBuf;

use kuna_analysis::loader::reloc_object::RELOC_BASE;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap a fixture and run the analysis commit (`read symbols`), returning
/// `None` (a visible skip) when the `.sla` is absent.
fn boot(name: &str) -> Option<ConsoleProgram> {
    let path = fixture(name);
    assert!(path.exists(), "missing fixture {path:?}");
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(mut prog) => {
            prog.commit_pending_analysis().expect("analysis commit");
            Some(prog)
        }
        Err(e) => {
            eprintln!(
                "verify_relocrebase: skipping {name} (bootstrap failed; build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

/// The whole inventory of a bootstrapped program, as `(vma, name)`.
fn inventory(prog: &ConsoleProgram) -> Vec<(u64, String)> {
    prog.function_entries_canonical()
        .into_iter()
        .map(|e| (e.addr.get_offset(), e.name))
        .collect()
}

/// The DIV-68 invariant the defect broke: the enumeration surface and the
/// decompiling surface must agree about a `.o`'s functions.  Every entry the
/// inventory reports is either a real, laid-out code address the decompiler can
/// reach or an extern stub above the sections — never a pre-link offset.
#[test]
fn inventory_agrees_with_the_loaded_image() {
    for name in ["ptx.o", "coff_obj.obj", "coff_comdat_i386.obj", "arm_thumb_le32.o"] {
        let Some(prog) = boot(name) else { continue };
        let inv = inventory(&prog);
        assert!(!inv.is_empty(), "{name}: empty inventory");
        for (vma, entry_name) in &inv {
            assert!(
                *vma >= RELOC_BASE,
                "{name}: `{entry_name}` at {vma:#x} is a pre-link address"
            );
        }
    }
}

/// The strict improvement the rebase buys: a `-g` object's DWARF now lands on the
/// real functions instead of collapsing onto address 0, so the recovered source
/// parameter names and types actually apply.
#[test]
fn dwarf_types_apply_to_the_rebased_functions() {
    let Some(mut prog) = boot("ptx.o") else { return };
    let entry = prog
        .function_entries_canonical()
        .into_iter()
        .find(|e| e.name == "to_uchar")
        .expect("ptx.o carries to_uchar");
    let out = kuna_console::project::decompile_targets(
        &mut prog,
        vec![entry],
        /* no_vars= */ true,
        /* want_proto= */ false,
        /* want_provenance= */ false,
    );
    let code = out.first().and_then(|f| f.code.clone()).unwrap_or_default();
    assert!(
        code.contains("char ch"),
        "the DWARF parameter name+type must reach the rebased function, got:\n{code}"
    );
}
