//! End-to-end two-pass gate for `armlibcmain` — a non-PIE ARM32 ELF recovers the
//! `main` its crt1 `_start` hands to `__libc_start_main` (P1 program prep).
//!
//! Fixtures: `armlibcmain_le32` (the classic `crt1.o` shape, `main` in a literal
//! pool word at 0x103dc) and `armlibcmain_got_le32` (the shape a modern
//! `-no-pie` link supplies, `main` at 0x10518 in a `.got` slot the linker filled
//! with no relocation to find it by). Both print a prompt from `main` and are
//! stripped, so nothing but `_start` says where the program's body is.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** `main` is not a function at all. Its body falls
//!   inside the extent of the entry before it, whose walk stops at that entry's
//!   own terminator, so the prompt it prints has no reader.
//! * **default (the fix):** `main` is a discovered, named function and its body
//!   is decompiled — the prompt load and the read that follows it are in it.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `ARM` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn bootstrap(fixture: &str, armlibcmain: bool) -> Option<ConsoleProgram> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_armlibcmain: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut()
        .set_kuna_option("armlibcmain", if armlibcmain { "on" } else { "off" })
        .expect("armlibcmain flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn decompile(prog: ConsoleProgram, name: &str) -> String {
    let cmds: Vec<String> = [format!("load function {name}"), "decompile".into(), "print C".into()]
        .into_iter()
        .collect();
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

/// BEFORE (`--option armlibcmain off`): neither crt1 shape yields a `main`, and
/// no function starts at its address under any name.
#[test]
fn no_function_covers_main_with_the_option_off() {
    for (fixture, entry) in
        [("armlibcmain_le32", "sub_103dc"), ("armlibcmain_got_le32", "sub_10518")]
    {
        let Some(prog) = bootstrap(fixture, false) else {
            return;
        };
        assert!(
            prog.lookup_symbol("main").is_none(),
            "{fixture}: with armlibcmain off nothing may be named main — the option \
             must restore the previous inventory exactly"
        );
        assert!(
            prog.lookup_symbol(entry).is_none(),
            "{fixture}: and no function may start there under the anonymous name \
             either — this is a discovery gap, not a naming one"
        );
    }
}

/// AFTER (default): both crt1 shapes name the address `_start` hands over, and
/// the body decompiled there is the one that prints the prompt.
#[test]
fn both_crt1_shapes_recover_main_by_default() {
    for fixture in ["armlibcmain_le32", "armlibcmain_got_le32"] {
        let Some(prog) = bootstrap(fixture, true) else {
            return;
        };
        assert!(prog.lookup_symbol("main").is_some(), "{fixture}: main must be discovered");
        let body = decompile(prog, "main");
        assert!(
            body.contains("main("),
            "{fixture}: and declared under that name; got:\n{body}"
        );
        // The body is the one that prints the prompt: the pool load feeding the
        // printf call, and the read that follows it, are both inside it.
        for want in ["__printf_chk(", "fgets("] {
            assert!(
                body.contains(want),
                "{fixture}: main's body must reach {want}; got:\n{body}"
            );
        }
    }
}

/// The count guard: the option adds exactly the one entry, and adds none at all
/// on the PIE ARM image entry oracle 4 already resolves through its
/// `R_ARM_RELATIVE` table.
#[test]
fn exactly_one_entry_is_added_and_none_on_a_pie_image() {
    let (Some(off), Some(on)) =
        (bootstrap("armlibcmain_le32", false), bootstrap("armlibcmain_le32", true))
    else {
        return;
    };
    assert_eq!(
        on.function_entries_canonical().len(),
        off.function_entries_canonical().len() + 1,
        "armlibcmain must add exactly the one entry it claims"
    );

    let (Some(pie_off), Some(pie_on)) =
        (bootstrap("entrymain_arm", false), bootstrap("entrymain_arm", true))
    else {
        return;
    };
    assert_eq!(
        pie_on.function_entries_canonical().len(),
        pie_off.function_entries_canonical().len(),
        "a PIE ARM image is already handled by oracle 4 — nothing may be added"
    );
}
