//! GH-388: ghidra mode must not ask the Java host for registers the language
//! does not define.
//!
//! A by-name register lookup in ghidra mode is a `getRegister` query, and the
//! host answers an undefined name by THROWING (`DecompileCallback.java:756-762`
//! → `No Register Defined: <name>`), which `DecompileProcess.readResponse`
//! turns into an exception frame plus an `Unexpected Exception: …` ERROR
//! record in Ghidra's log.  So a pass merely testing "does this language happen
//! to have X?" has to go through the probe seam, which answers from the
//! register cache and issues no query.
//!
//! The sim oracle answers an undefined name the same way Java does (an
//! exception frame) and records every probed / failed name, so the ARM drive
//! below sees exactly what the GUI user's log would.

mod ghidra_sim;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use kuna_base::marshal::PackedEncode;
use kuna_ghidra::process::GhidraProcess;

use ghidra_sim::oracle::{generate_tspec, repo_root, SimOracle};
use ghidra_sim::{
    cmd_decompile_at, cmd_deregister_program, cmd_flush_native, cmd_register_program,
    cmd_set_action, trace_session, MockReader, MockState, MockWriter, SessionTrace,
};

struct Run {
    oracle: SimOracle,
    trace: SessionTrace,
}

/// Drive the DecompInterface lifecycle — registerProgram, setAction,
/// decompileAt, flushNative, decompileAt, deregisterProgram — against the
/// in-process [`GhidraProcess`] with the host end answered by the sim oracle.
///
/// `None` when the `.sla` specs are not built (the visible skip the CI canary
/// greps for).
fn run(binary: &Path, lang_dir: &str, pspec_name: &str, cspec_name: &str, target: &str) -> Option<Run> {
    let oracle = SimOracle::bootstrap(binary)?;
    let addr = oracle
        .prog
        .find_entry_by_name(target)
        .unwrap_or_else(|| panic!("{binary:?}: no function named {target}"))
        .addr;

    let tspec = generate_tspec(&oracle.manager, oracle.big_endian, oracle.unique_base);
    let dir = repo_root().join(lang_dir);
    let pspec = std::fs::read(dir.join(pspec_name)).expect("vendored pspec");
    let cspec = std::fs::read(dir.join(cspec_name)).expect("vendored cspec");

    let mut packed_addr = Vec::new();
    {
        let mut e = PackedEncode::new(&mut packed_addr);
        addr.encode(&mut e).expect("entry addr encodes");
    }

    let mut commands = Vec::new();
    cmd_register_program(
        &mut commands,
        &pspec,
        &cspec,
        &tspec,
        ghidra_sim::DEFAULT_CORETYPES_XML,
    );
    cmd_set_action(&mut commands, "0", "decompile", "c");
    cmd_decompile_at(&mut commands, "0", &packed_addr);
    cmd_flush_native(&mut commands, "0");
    cmd_decompile_at(&mut commands, "0", &packed_addr);
    cmd_deregister_program(&mut commands, "0");
    let n = 6;

    let shared = Rc::new(RefCell::new(MockState::new(commands, oracle)));
    let reader = MockReader {
        shared: Rc::clone(&shared),
    };
    let writer = MockWriter {
        shared: Rc::clone(&shared),
    };
    let mut process = GhidraProcess::new(reader, writer);
    for i in 0..n {
        let status = process
            .read_command()
            .unwrap_or_else(|e| panic!("command #{i} failed: {e:?}"));
        assert_eq!(status, if i == n - 1 { 1 } else { 0 }, "command #{i} status");
    }
    let _ = process.into_inner();
    let state = match Rc::try_unwrap(shared) {
        Ok(cell) => cell.into_inner(),
        Err(_) => panic!("mock state still shared"),
    };
    Some(Run {
        trace: trace_session(&state.from_process),
        oracle: state.source,
    })
}

/// GH-388: an ARM:LE:32 program must not make the host throw.  Before the fix
/// this reports `["ST0", "DF", "DF", "DF"]` — one `ST0` from the msvcftol
/// language test and one `DF` per `<prototype>` in `ARM.cspec`.
#[test]
fn arm32_asks_the_host_for_no_undefined_register() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fmt_arm");
    let Some(r) = run(
        &binary,
        "specs/Ghidra/Processors/ARM/data/languages",
        "ARMt.pspec",
        "ARM.cspec",
        "main",
    ) else {
        return;
    };

    let failures = &r.oracle.log.register_probe_failures;
    eprintln!(
        "ARM getRegister probes: {} total, {} undefined: {failures:?}",
        r.oracle.log.register_probes.len(),
        failures.len()
    );
    assert!(
        failures.is_empty(),
        "GH-388: ghidra mode on ARM:LE:32 asked the host for {} register(s) the language \
         does not define ({failures:?}); each one throws `No Register Defined` in \
         DecompileCallback and logs `Unexpected Exception` in Ghidra",
        failures.len()
    );

    // The session is otherwise unchanged: every command answered and the
    // registerProgram warnings frame stayed empty.
    assert_eq!(r.trace.responses.len(), 6, "one response span per command");
    assert!(
        r.trace.responses[0].warnings.trim().is_empty(),
        "registerProgram warnings: {}",
        r.trace.responses[0].warnings
    );
}

/// The other direction: x86-64 still resolves `DF` (the pspec `<tracked_set>`
/// caches it during registerProgram), so the direction-flag assertion the probe
/// feeds keeps applying where it matters.
#[test]
fn x86_64_still_resolves_the_direction_flag() {
    let binary = repo_root().join("tests/bug-repro/faillog");
    let Some(r) = run(
        &binary,
        "specs/Ghidra/Processors/x86/data/languages",
        "x86-64.pspec",
        "x86-64-gcc.cspec",
        "sub_2620",
    ) else {
        return;
    };

    let probes = &r.oracle.log.register_probes;
    eprintln!("x86-64 getRegister probes: {}", probes.len());
    assert!(
        r.oracle.log.register_probe_failures.is_empty(),
        "x86-64 asked for undefined registers: {:?}",
        r.oracle.log.register_probe_failures
    );
    assert!(
        probes.iter().any(|n| n == "DF"),
        "x86-64 never resolved DF, so the direction-flag assertion cannot apply: {probes:?}"
    );
}
