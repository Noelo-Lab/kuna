//! A `kuna decompile` budget stop lands at the budget (#1034).
//!
//! `kuna decompile` drives the console with `load addr` (or `load function`)
//! and then `decompile`, which adopts the IR the load followed. Two things
//! used to push a stop past the budget:
//!
//! 1. The adopting drive armed a fresh budget, so the follow the load made,
//!    every jump-table recovery included, came on top of it.
//! 2. When the drive stopped, the console followed the flow again just to
//!    keep an un-decompiled function for `print C`'s failure shell.
//!
//! On a 205 KB function a 120 s budget stopped after about 220 s, while the
//! same function decompiled with no budget in 210 s. This binary sets the
//! process-wide default budget `decomp_dbg --max-fn-seconds` sets, so it runs
//! on its own.

use std::path::PathBuf;
use std::time::Duration;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::interface::IfaceStatus;
use kuna_decomp::decompile_drive::{
    build_and_follow_flow_within_budget, decompile_func_full_with_override_dyn_prefollowed, is_budget_stop,
    set_default_fn_budget_secs, DrivePurpose,
};

const BUDGET: Duration = Duration::from_secs(600);

fn load() -> ConsoleProgram {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/fmt_x86_64");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

fn main_entry(prog: &ConsoleProgram) -> kuna_base::address::Address {
    prog.function_entries_canonical()
        .into_iter()
        .find(|e| e.name == "main")
        .expect("the fixture exports main")
        .addr
}

fn dcp(status: &mut IfaceStatus) -> &mut IfaceDecompData {
    status.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut().downcast_mut::<IfaceDecompData>().unwrap()
}

/// Run `cmds` one at a time, calling `between(i, dcp)` before command `i`.
fn drive(cmds: &[&str], mut between: impl FnMut(usize, &mut IfaceDecompData)) -> (String, IfaceStatus) {
    let prog = load();
    let cmds: Vec<String> = cmds.iter().map(|s| s.to_string()).collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    dcp(&mut status).conf = Some(prog);
    for i in 0..count {
        between(i, dcp(&mut status));
        execute(&mut status);
    }
    (status.optr.clone(), status)
}

/// The failure shell `print C` rendered last, from its prototype line on,
/// with its reason replaced.
fn shell(transcript: &str) -> String {
    let warn = transcript.rfind("/* WARNING: decompilation failed: ").expect("the shell names the failure");
    let proto = transcript[..warn].rfind("main(").expect("the shell prints the prototype");
    let line = transcript[..proto].rfind('\n').map_or(0, |i| i + 1);
    let end = warn + transcript[warn..].find("*/").unwrap() + 2;
    format!("{}<reason>{}", &transcript[line..warn], &transcript[end..]).trim().to_string()
}

#[test]
fn a_drive_charges_the_follow_it_adopts_to_its_budget() {
    let mut prog = load();
    let entry = main_entry(&prog);
    let arch = prog.arch_mut();
    arch.kuna_fn_budget = Some(BUDGET);
    let drive = |arch: &mut kuna_decomp::architecture::Architecture, spent: Option<Duration>| {
        let mut fd = build_and_follow_flow_within_budget(arch, "main", entry.clone(), 0, &[]).expect("main follows");
        assert!(fd.flow_spent() > Duration::ZERO, "the follow records how long it took");
        if let Some(spent) = spent {
            fd.set_flow_spent(spent);
        }
        decompile_func_full_with_override_dyn_prefollowed(
            arch,
            "main",
            entry.clone(),
            0,
            &[],
            &[],
            &[],
            None,
            &[],
            &[],
            &[],
            Some(fd),
            DrivePurpose::SingleFunction,
        )
    };
    assert!(drive(arch, None).is_ok(), "a quick follow leaves the drive its budget");
    let Err(error) = drive(arch, Some(BUDGET)) else {
        panic!("a follow that used the budget leaves the drive none");
    };
    assert!(is_budget_stop(&error), "{}", error.explain());
    assert!(error.explain().starts_with("per-function decompile budget exceeded (600 s)"), "{}", error.explain());
}

#[test]
fn a_budget_stopped_decompile_keeps_its_shell_without_following_again() {
    set_default_fn_budget_secs(BUDGET.as_secs());
    let (adopted, mut status) = drive(&["load function main", "decompile", "print C"], |i, dcp| {
        if i == 1 {
            dcp.fd.as_mut().expect("main is loaded").set_flow_spent(BUDGET);
        }
    });
    assert_eq!(dcp(&mut status).adopted_flows, 1, "the decompile adopts the loaded IR:\n{adopted}");
    assert!(adopted.contains("per-function decompile budget exceeded (600 s)"), "{adopted}");
    assert!(
        dcp(&mut status).fd.as_ref().expect("a shell is kept for print C").obank().empty(),
        "the shell is not followed again"
    );
    let (rebuilt, mut status) = drive(&["load function main", "echo x", "decompile", "print C"], |i, dcp| {
        if i == 2 {
            dcp.conf.as_mut().unwrap().arch_mut().kuna_fn_budget = Some(Duration::ZERO);
        }
    });
    assert_eq!(dcp(&mut status).adopted_flows, 0, "a command in between rebuilds the flow");
    assert!(rebuilt.contains("per-function decompile budget exceeded (0 s)"), "{rebuilt}");
    assert_eq!(shell(&adopted), shell(&rebuilt), "the stop prints the shell the loaded IR prints");
}
