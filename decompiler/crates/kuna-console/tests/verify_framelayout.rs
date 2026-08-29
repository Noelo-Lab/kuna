//! Two-pass end-to-end gate for the `framelayout` option (P6, JSON surface).
//!
//! `restructure_varnode` re-derives the stack frame from the LIVE stack Varnodes
//! on every pass and clears the previous pass's unlocked symbols first, so a slot
//! whose spill store/load pair the dataflow later folded away is present in an
//! early layout and gone from the last one.  `extract_variables` reads only the
//! final pass, so those slots never reach `decompile-all --json`'s `variables`
//! array -- the surface decbench's `type_match` metric scores and the one IDA's
//! stack view and Binary Ninja's variable list are the analogues of.
//!
//! The corpus number this exists for: over 82,049 scored functions kuna reports
//! 154,716 of the 184,522 ground-truth stack variables (84%), against angr's
//! 203,329 and IDA's 178,337.
//!
//! Pass 1 (`framelayout off`, the pre-feature behaviour) and pass 2 (`on`) run the
//! same decompile of the same functions.  The gate is that ON is a strict superset
//! of OFF, that everything it adds is a stack slot at an offset OFF did not report,
//! and that nothing it adds carries Ghidra's internal `$$undefNNNNNNNN` placeholder
//! name or the misleading `char[N]` spelling for a type the recovery never
//! committed to.
//!
//! ## `.sla` precondition
//!
//! Like the sibling loader gates, bootstrapping needs the built `x86` `.sla` under
//! `specs/` (gitignored; `make specs`).  When it is absent the bootstrap fails; the
//! test prints that and returns early (a specs-less CI is a visible skip, never a
//! false green).

use std::collections::BTreeSet;
use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::decompile_drive::{
    decompile_func_full_with_override_dyn, extract_variables, VarInfo,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

/// Every function's `extract_variables` output for one setting of `framelayout`.
fn variables_for(framelayout: bool) -> Option<Vec<(String, Vec<VarInfo>)>> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fauxware().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_framelayout: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("read symbols (analysis commit) must succeed");
    prog.arch_mut().framelayout = framelayout;

    let entries: Vec<(String, _)> =
        prog.function_entries().map(|(n, a)| (n.to_string(), a.clone())).collect();
    let mut out = Vec::new();
    for (name, addr) in entries {
        let mapped = prog.dwarf_locals_for(addr.get_offset());
        let Ok(fd) = decompile_func_full_with_override_dyn(
            prog.arch_mut(), &name, addr, 0, &mapped, &[], &[], None, &[], &[], &[],
        ) else {
            continue;
        };
        out.push((name, extract_variables(prog.arch(), &fd)));
    }
    Some(out)
}

#[test]
fn framelayout_reports_the_frame_slots_the_final_restructure_pass_lost() {
    let Some(off) = variables_for(false) else { return };
    let Some(on) = variables_for(true) else { return };
    assert_eq!(off.len(), on.len(), "the two arms must decompile the same functions");

    let mut added_total = 0usize;
    for ((fname, ovars), (fname2, nvars)) in off.iter().zip(on.iter()) {
        assert_eq!(fname, fname2, "function order must match across arms");

        // ON is a strict superset: every variable OFF reported is still reported,
        // unchanged. The option only ever APPENDS.
        assert!(
            nvars.len() >= ovars.len(),
            "{fname}: framelayout on dropped variables ({} -> {})",
            ovars.len(),
            nvars.len()
        );
        for (a, b) in ovars.iter().zip(nvars.iter()) {
            assert_eq!(a.name, b.name, "{fname}: framelayout rewrote an existing variable");
            assert_eq!(a.type_name, b.type_name, "{fname}: framelayout retyped an existing variable");
            assert_eq!(a.stack_offset, b.stack_offset, "{fname}: framelayout moved a variable");
        }

        let old_offsets: BTreeSet<i64> = ovars.iter().filter_map(|v| v.stack_offset).collect();
        for v in &nvars[ovars.len()..] {
            added_total += 1;
            // Everything added is a stack slot, never a parameter.
            assert!(!v.is_param, "{fname}: framelayout added a parameter `{}`", v.name);
            assert_eq!(v.arg_index, None, "{fname}: added slot `{}` carries an arg_index", v.name);
            let Some(off) = v.stack_offset else {
                panic!("{fname}: framelayout added `{}` with no stack offset", v.name);
            };
            assert!(
                !old_offsets.contains(&off),
                "{fname}: framelayout re-reported offset {off} that `off` already had"
            );
            // Ghidra's internal placeholder must never reach a public surface.
            assert!(
                !v.name.starts_with("$$undef"),
                "{fname}: framelayout leaked the `$$undef` placeholder as `{}`",
                v.name
            );
            assert!(!v.name.is_empty(), "{fname}: framelayout added an unnamed slot");
            // A slot the type system never committed to reports its WIDTH, not a
            // fabricated element type.
            assert!(!v.type_name.is_empty(), "{fname}: added slot `{}` has no type", v.name);
            assert!(
                !v.type_name.starts_with("char["),
                "{fname}: added slot `{}` reports the array-of-unknown carrier `{}` \
                 instead of the width-only undefined<N>",
                v.name,
                v.type_name
            );
        }
    }

    // The fixture must actually exercise the path, or this gate proves nothing.
    assert!(
        added_total > 0,
        "framelayout added no frame slots anywhere in fauxware -- the gate is vacuous"
    );
}
