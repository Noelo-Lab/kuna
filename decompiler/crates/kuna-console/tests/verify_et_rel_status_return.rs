//! Cross-architecture ET_REL proof for instruction relocations and conservative
//! status-return recovery across a terminal no-return error path.
//!
//! Both synthetic objects define `guarded_status`, which leaves a status in the
//! ABI return/first-argument register. The normal path returns it; a separate
//! guard-failure path passes the same value to an undefined no-return function.
//! `status_caller` consumes the result through a relocated direct call.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
}

fn decompile_pair(name: &str) -> (String, String) {
    let path = fixture(name);
    assert!(path.is_file(), "missing fixture {path:?}");
    let specs = repo_root().join("specs");
    let roots = vec![specs.to_string_lossy().into_owned()];
    let mut program = bootstrap_from_object(path.to_str().unwrap(), "", &roots)
        .unwrap_or_else(|error| panic!("{name}: bootstrap failed: {}", error.explain()));
    program.commit_pending_analysis().expect("analysis commit");
    let entries = program
        .function_entries_canonical()
        .into_iter()
        .filter(|entry| matches!(entry.name.as_str(), "guarded_status" | "status_caller"))
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 2, "{name}: missing synthetic functions");
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
            .unwrap_or_else(|| panic!("{name}: no decompile for {function}"))
    };
    (code_for("guarded_status"), code_for("status_caller"))
}

#[test]
fn arm_and_aarch64_status_helpers_keep_their_return_values() {
    let specs = repo_root().join("specs/Ghidra/Processors");
    if !specs.join("ARM/data/languages/ARM8_le.sla").is_file()
        || !specs.join("AARCH64/data/languages/AARCH64.sla").is_file()
    {
        eprintln!("verify_et_rel_status_return: skipping (required `.sla` files are absent)");
        return;
    }

    for name in ["et_rel_status_arm.o", "et_rel_status_aarch64.o"] {
        let (helper, caller) = decompile_pair(name);
        assert!(
            !helper.starts_with("void guarded_status"),
            "{name}: helper was forced to void:\n{helper}"
        );
        assert!(
            helper.contains("return "),
            "{name}: helper has no value-bearing return:\n{helper}"
        );
        assert!(
            helper.contains("__stack_chk_fail("),
            "{name}: terminal no-return control was not retained:\n{helper}"
        );
        assert!(
            caller.contains("guarded_status(") && caller.contains("return "),
            "{name}: caller did not consume the named relocated result:\n{caller}"
        );
        assert!(
            !caller.contains("sub_"),
            "{name}: relocated call lost callee identity:\n{caller}"
        );
    }
}
