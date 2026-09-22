//! Two-pass end-to-end gate for the `slotptr` option (P6, JSON surface).
//!
//! At -O0 each C local has its own spill slot.  The dataflow folds the value out
//! of the slot, `framelayout` reports the emptied slot from an early pass as the
//! width-only `undefinedN`, and the C body prints the value itself as `char *`.
//! `slotptr` gives the slot the pointer type of the value stored into it.
//!
//! Pass 1 (`slotptr off`) and pass 2 (`on`) run the same decompile of the same
//! functions.  The gate is that the two arms differ in exactly one field -- a
//! stack row's `type_name` -- that every row that differs moved from a width-only
//! `undefinedN` to a pointer of the same size, and that the emitted C is
//! byte-identical.
//!
//! ## `.sla` precondition
//!
//! Like the sibling gates, bootstrapping needs the built `x86` `.sla` under
//! `specs/` (gitignored; `make specs`).  When it is absent the bootstrap fails;
//! the test prints that and returns early (a specs-less CI is a visible skip,
//! never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::decompile_drive::{
    decompile_func_full_with_override_dyn, extract_variables, print_c, VarInfo,
};
use kuna_decomp::kuna_slotptr::SlotPtrMode;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

/// Every function's `extract_variables` output and emitted C for one setting.
fn run(mode: SlotPtrMode) -> Option<Vec<(String, Vec<VarInfo>, String)>> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fauxware().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_slotptr: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("read symbols (analysis commit) must succeed");
    prog.arch_mut().framelayout = true;
    prog.arch_mut().slot_ptr = mode;

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
        let vars = extract_variables(prog.arch(), &fd);
        let code = print_c(prog.arch_mut(), &fd);
        out.push((name, vars, code));
    }
    Some(out)
}

fn is_pointer_spelling(t: &str) -> bool {
    t.ends_with('*') || t.contains("(*)")
}

#[test]
fn slotptr_types_filler_slots_from_their_stores_and_changes_nothing_else() {
    let Some(off) = run(SlotPtrMode::Off) else { return };
    let Some(on) = run(SlotPtrMode::On) else { return };
    assert_eq!(off.len(), on.len(), "the two arms must decompile the same functions");

    let mut respelled = Vec::new();
    for ((fname, ovars, ocode), (fname2, nvars, ncode)) in off.iter().zip(on.iter()) {
        assert_eq!(fname, fname2, "function order must match across arms");
        assert_eq!(ocode, ncode, "{fname}: slotptr changed the emitted C");
        assert_eq!(ovars.len(), nvars.len(), "{fname}: slotptr changed the variable COUNT");
        for (a, b) in ovars.iter().zip(nvars.iter()) {
            assert_eq!(a.name, b.name, "{fname}: slotptr renamed a variable");
            assert_eq!(a.stack_offset, b.stack_offset, "{fname}: slotptr moved a variable");
            assert_eq!(a.size, b.size, "{fname}: slotptr resized `{}`", a.name);
            assert_eq!(a.is_param, b.is_param, "{fname}: slotptr reclassified `{}`", a.name);
            assert_eq!(a.arg_index, b.arg_index, "{fname}: slotptr re-indexed `{}`", a.name);
            if a.type_name == b.type_name {
                continue;
            }
            assert!(!a.is_param, "{fname}: slotptr retyped the parameter `{}`", a.name);
            assert!(
                a.type_name.starts_with("undefined"),
                "{fname}: `{}` was `{}`, not a width-only filler slot",
                a.name,
                a.type_name
            );
            assert!(
                is_pointer_spelling(&b.type_name),
                "{fname}: `{}` took the non-pointer type `{}`",
                a.name,
                b.type_name
            );
            respelled.push((fname.clone(), b.name.clone(), b.type_name.clone()));
        }
    }

    assert!(
        respelled.iter().any(|(f, _, t)| f == "authenticate" && t == "char *"),
        "authenticate's two -O0 string slots must take `char *`; re-spelled: {respelled:?}"
    );
}
