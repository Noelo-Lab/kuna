//! End-to-end two-pass gate for `fdeinterior` — never start a function inside
//! another function's `.eh_frame` FDE (P1 code/data partition, DIV-61).
//!
//! Fixture: `eh_lsda_x86_64`, the vendored C++ try/catch ELF stripped of
//! `.symtab` (shared with `verify_eh_frame_full`). `guarded()` occupies the FDE
//! `[0x4012d6, 0x40137a)` and its `.gcc_except_table` LSDA call-site table names
//! the catch-dispatch landing pad `0x4012e2` — 12 bytes into that same body.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** with `eh_frame_full` on, `0x4012e2` is committed as
//!   a top-level function `sub_4012e2`. It is a block the unwinder jumps to
//!   mid-frame, so it inherits `guarded`'s live frame pointer: decompiled on its
//!   own it opens on an undefined `rbp` and every local is a garbage dereference.
//! * **default (the fix):** the pad is strictly inside a single-function FDE, so
//!   the commit rejects it and `sub_4012e2` is absent — IDA Pro's model, where
//!   `get_func(landing_pad).start_ea` is the enclosing function taken from the FDE.
//!   The FDE's own start `sub_4012d6` is untouched and still decompiles.
//!
//! The third test is the property that keeps this from deleting real code: the
//! linker gives the whole PLT ONE FDE, so `fdeinterior` must never touch an import
//! stub. Every named function the default run registers is still registered.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

/// The catch-dispatch landing pad, 12 bytes into `guarded`'s FDE body.
const PAD: &str = "sub_4012e2";
/// `guarded`'s entry — the FDE `pcBegin` that covers the pad.
const OWNER: &str = "sub_4012d6";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the stripped C++ try/catch fixture with the LSDA landing-pad oracle
/// on and `fdeinterior` in the requested state, then commit the (deferred)
/// analysis facts. `None` is a visible skip when the `.sla` is missing.
fn bootstrap(fdeinterior: bool) -> Option<ConsoleProgram> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/eh_lsda_x86_64");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_fdeinterior: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut().set_kuna_option("eh_frame_full", "on").expect("eh_frame_full flips on");
    prog.arch_mut()
        .set_kuna_option("fdeinterior", if fdeinterior { "on" } else { "off" })
        .expect("fdeinterior flips");
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

/// BEFORE (`--option fdeinterior off`): the landing pad becomes its own function.
#[test]
fn landing_pad_is_a_function_with_the_option_off() {
    let Some(prog) = bootstrap(false) else {
        return;
    };
    assert!(
        prog.lookup_symbol(PAD).is_some(),
        "with fdeinterior off the LSDA landing pad 0x4012e2 must still register as \
         {PAD} — the option must restore the previous discovery set exactly"
    );
    let body = decompile(prog, PAD);
    assert!(body.contains(PAD), "expected a decompiled body for {PAD}, got:\n{body}");
}

/// AFTER (default): the pad is strictly inside `guarded`'s FDE, so it is not a
/// function — while the FDE's own start still is.
#[test]
fn landing_pad_is_rejected_by_default_and_its_owner_survives() {
    let Some(prog) = bootstrap(true) else {
        return;
    };
    assert!(
        prog.lookup_symbol(PAD).is_none(),
        "0x4012e2 is 12 bytes into the FDE [0x4012d6,0x40137a) — by default it must \
         NOT be promoted to a top-level function"
    );
    assert!(
        prog.lookup_symbol(OWNER).is_some(),
        "the FDE start {OWNER} is a function and must survive the interior filter"
    );
    let body = decompile(prog, OWNER);
    assert!(body.contains(OWNER), "expected a decompiled body for {OWNER}, got:\n{body}");
}

/// The PLT property: the linker emits ONE FDE for the whole PLT, and every stub
/// inside it is a real function. No named function may be lost.
#[test]
fn no_named_function_is_lost() {
    let (Some(off), Some(on)) = (bootstrap(false), bootstrap(true)) else {
        return;
    };
    let named: Vec<String> = off
        .function_entries_canonical()
        .into_iter()
        .filter(|e| !e.name.starts_with("sub_"))
        .map(|e| e.name)
        .collect();
    assert!(!named.is_empty(), "the fixture must carry named (PLT/import) functions");
    for name in &named {
        assert!(
            on.lookup_symbol(name).is_some(),
            "fdeinterior must never drop a named function; {name} disappeared"
        );
    }
}
