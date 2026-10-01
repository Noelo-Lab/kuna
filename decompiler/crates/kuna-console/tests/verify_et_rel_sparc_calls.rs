//! 32-bit SPARC ET_REL proof for instruction relocations.
//!
//! `et_rel_calls_sparc.o` reaches a local function and an undefined extern
//! through `R_SPARC_WDISP30` calls, and a `.data` global and a `.rodata`
//! literal through `R_SPARC_HI22`/`R_SPARC_LO10` pairs. Without the SPARC
//! encoders every call targets its own instruction and every global reads
//! address zero, so the calls render as `sub_<addr>` and the global is lost.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

#[test]
fn sparc_calls_and_split_immediates_resolve_to_their_targets() {
    let specs = repo_root().join("specs");
    assert!(
        specs
            .join("Ghidra/Processors/Sparc/data/languages/SparcV9_32.sla")
            .is_file(),
        "required processor specs must be available"
    );
    let path = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/et_rel_calls_sparc.o");
    let roots = vec![specs.to_string_lossy().into_owned()];
    let mut program = bootstrap_from_object(path.to_str().unwrap(), "", &roots)
        .unwrap_or_else(|error| panic!("bootstrap failed: {}", error.explain()));
    program.commit_pending_analysis().expect("analysis commit");
    let entries = program
        .function_entries_canonical()
        .into_iter()
        .filter(|entry| matches!(entry.name.as_str(), "counter_bump" | "report_total"))
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 2, "missing synthetic functions");
    let output = kuna_console::project::decompile_targets(
        &mut program,
        entries,
        /* no_vars= */ false,
        /* want_proto= */ false,
        /* want_provenance= */ false,
    );
    let code_for = |function: &str| {
        output
            .iter()
            .find(|item| item.name == function)
            .and_then(|item| item.code.clone())
            .unwrap_or_else(|| panic!("no decompile for {function}"))
    };

    let bump = code_for("counter_bump");
    assert!(
        bump.contains("dat_400038 += a0;"),
        "HI22/LO10 global did not resolve to the laid-out .data word:\n{bump}"
    );

    let caller = code_for("report_total");
    assert!(
        caller.contains("counter_bump(a0);"),
        "WDISP30 call lost its local callee:\n{caller}"
    );
    assert!(
        caller.contains("printf(0x400040"),
        "WDISP30 extern call or HI22/LO10 literal address did not resolve:\n{caller}"
    );
    assert!(!caller.contains("sub_"), "a relocated call lost callee identity:\n{caller}");
}
