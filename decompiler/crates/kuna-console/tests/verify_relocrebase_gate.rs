//! (kuna `relocrebase`, GH-289) The `relocrebase` option gate, in its own test
//! binary.
//!
//! The gate is a process-global environment variable (the analyzer tier runs
//! inside `load file`, upstream of every per-function `option`), so its two arms
//! cannot share a process with the default-on assertions in
//! `verify_relocrebase.rs` — cargo runs each integration-test target as its own
//! binary, which is exactly the isolation this needs.
//!
//! See `verify_relocrebase.rs` for the defect this option governs and for the
//! `.sla` precondition.

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

/// The gate is honoured through the process env bridge in BOTH directions, and
/// the off arm still reproduces the defect — which is what makes the on arm's
/// clean inventory a property of this option rather than of the fixture.
///
/// One test, not two: the gate is a process-global env var, so the arms must not
/// race each other under the test harness's thread pool.
#[test]
fn relocrebase_gate_governs_the_pre_link_phantoms() {
    use kuna_decomp::kuna_relocrebase::{set_relocrebase_env, RELOCREBASE_ENV};

    set_relocrebase_env(false);
    let off = boot("ptx.o").map(|p| inventory(&p));
    set_relocrebase_env(true);
    let on = boot("ptx.o").map(|p| inventory(&p));
    std::env::remove_var(RELOCREBASE_ENV);

    let (Some(off), Some(on)) = (off, on) else { return };

    let below = |inv: &[(u64, String)]| -> Vec<(u64, String)> {
        inv.iter().filter(|(a, _)| *a < RELOC_BASE).cloned().collect()
    };
    assert!(
        !below(&off).is_empty(),
        "`relocrebase off` must reproduce the pre-link phantoms (the defect)"
    );
    assert!(
        below(&on).is_empty(),
        "`relocrebase on` must leave no entry below RELOC_BASE, got {:x?}",
        below(&on)
    );

    // Rebasing only REMOVES the phantom half: every real entry the pre-fix run
    // reported is still reported, under the same name.
    let real_off: Vec<_> = off.iter().filter(|(a, _)| *a >= RELOC_BASE).cloned().collect();
    assert_eq!(real_off, on, "the real (rebased) entries must be untouched by the option");
}

