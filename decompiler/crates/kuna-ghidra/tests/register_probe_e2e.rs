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
//!
//! The other two drives pin what the seam must keep working: an x86-64 one that
//! the direction-flag assertion still lands (its output is free of the string-op
//! stride the assertion exists to fold away), and an x86-32 one that the
//! syscall pass's ABI registers are already in the cache the probe reads.

mod ghidra_sim;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use kuna_base::marshal::PackedEncode;
use kuna_decomp::kuna_linuxsyscall::{ARG_REGISTERS, NUM_REGISTER};
use kuna_ghidra::process::GhidraProcess;

use ghidra_sim::oracle::{generate_tspec, repo_root, SimOracle};
use ghidra_sim::{
    cmd_decompile_at, cmd_deregister_program, cmd_flush_native, cmd_register_program,
    cmd_set_action, parse_decompile_doc, trace_session, MockReader, MockState, MockWriter,
    ParsedDoc, SessionTrace,
};

struct Run {
    oracle: SimOracle,
    trace: SessionTrace,
    /// The C flattened from each NON-EMPTY `decompileAt` response, in wire
    /// order (empty when the drive registered the program only, and short when
    /// a decompile came back empty — GH-407 on ARM/MIPS).
    docs: Vec<ParsedDoc>,
}

/// Drive the DecompInterface lifecycle against the in-process [`GhidraProcess`]
/// with the host end answered by the sim oracle.
///
/// With a `target` the sequence is the full registerProgram → setAction →
/// decompileAt → flushNative → decompileAt → deregisterProgram; without one it
/// is registerProgram → deregisterProgram, so every logged query is one
/// registerProgram itself issued.
///
/// `None` when the `.sla` specs are not built (the visible skip the CI canary
/// greps for).
fn run(
    binary: &Path,
    lang_dir: &str,
    pspec_name: &str,
    cspec_name: &str,
    target: Option<&str>,
) -> Option<Run> {
    let oracle = SimOracle::bootstrap(binary)?;
    let tspec = generate_tspec(&oracle.manager, oracle.big_endian, oracle.unique_base);
    let dir = repo_root().join(lang_dir);
    let pspec = std::fs::read(dir.join(pspec_name)).expect("vendored pspec");
    let cspec = std::fs::read(dir.join(cspec_name)).expect("vendored cspec");

    let mut commands = Vec::new();
    cmd_register_program(
        &mut commands,
        &pspec,
        &cspec,
        &tspec,
        ghidra_sim::DEFAULT_CORETYPES_XML,
    );
    if let Some(target) = target {
        let addr = oracle
            .prog
            .find_entry_by_name(target)
            .unwrap_or_else(|| panic!("{binary:?}: no function named {target}"))
            .addr;
        let mut packed_addr = Vec::new();
        {
            let mut e = PackedEncode::new(&mut packed_addr);
            addr.encode(&mut e).expect("entry addr encodes");
        }
        cmd_set_action(&mut commands, "0", "decompile", "c");
        cmd_decompile_at(&mut commands, "0", &packed_addr);
        cmd_flush_native(&mut commands, "0");
        cmd_decompile_at(&mut commands, "0", &packed_addr);
    }
    cmd_deregister_program(&mut commands, "0");
    let n = if target.is_some() { 6 } else { 2 };

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
        assert_eq!(
            status,
            if i == n - 1 { 1 } else { 0 },
            "command #{i} status"
        );
    }
    let _ = process.into_inner();
    let state = match Rc::try_unwrap(shared) {
        Ok(cell) => cell.into_inner(),
        Err(_) => panic!("mock state still shared"),
    };
    let oracle = state.source;
    let trace = trace_session(&state.from_process);
    // With a target the decompileAt spans are responses 2 and 4.
    let mut docs = Vec::new();
    if target.is_some() {
        for i in [2usize, 4] {
            match trace.responses[i].payload.as_deref() {
                Some(payload) if !payload.is_empty() => {
                    docs.push(parse_decompile_doc(payload, &oracle.manager))
                }
                _ => {}
            }
        }
    }
    Some(Run {
        trace,
        oracle,
        docs,
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
        Some("main"),
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

/// The other direction, asserted on the OUTPUT rather than on the query log:
/// `grep`'s `sub_17400` clears a `struct stat` with `rep stosq`, so if the
/// direction-flag assertion stops landing the string-op stride survives as
/// `&v7[(unsigned long)v9 * -2 + 1]` with `v9` a live `df` variable, instead of
/// the `&v8[1]` kuna emits.  That assertion is the reason `probe_register_varnode`
/// has to answer `DF` at all in ghidra mode.
#[test]
fn x86_64_folds_the_direction_flag_stride() {
    let binary = repo_root().join("tests/bug-repro/grep");
    let Some(r) = run(
        &binary,
        "specs/Ghidra/Processors/x86/data/languages",
        "x86-64.pspec",
        "x86-64-gcc.cspec",
        Some("sub_17400"),
    ) else {
        return;
    };

    assert!(
        r.oracle.log.register_probe_failures.is_empty(),
        "x86-64 asked for undefined registers: {:?}",
        r.oracle.log.register_probe_failures
    );
    assert_eq!(
        r.docs.len(),
        2,
        "both decompileAt spans must carry a document"
    );
    for (i, doc) in r.docs.iter().enumerate() {
        assert!(
            !doc.c_text.trim().is_empty(),
            "decompileAt #{i} produced no C"
        );
        assert!(
            !doc.c_text.contains("* -2 + 1"),
            "decompileAt #{i}: the direction flag survived into the output as the \
             string-op stride `1 - 2*DF`; the DF-unaffected assertion did not land:\n{}",
            doc.c_text
        );
    }
}

/// The i386 syscall pass reads `EAX`/`EBX`/`ECX`/`EDX`/`ESI`/`EDI`/`EBP` through
/// the same probe, and in ghidra mode a probe sees only what the register cache
/// already holds.  What fills it is the compiler spec: every `<prototype>` in
/// `x86gcc.cspec` is decoded during registerProgram, before any function is
/// lifted.  This drive registers the program and nothing else, so every name in
/// the log below was resolved by that decode — move it after the first lift and
/// the syscall pass silently stops recognizing `int 0x80`.
#[test]
fn x86_32_caches_the_syscall_abi_registers_at_register_program() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/picbase_i386");
    let Some(r) = run(
        &binary,
        "specs/Ghidra/Processors/x86/data/languages",
        "x86.pspec",
        "x86gcc.cspec",
        None,
    ) else {
        return;
    };

    assert!(
        r.oracle.log.register_probe_failures.is_empty(),
        "x86-32 asked for undefined registers: {:?}",
        r.oracle.log.register_probe_failures
    );
    let probed = &r.oracle.log.register_probes;
    for nm in std::iter::once(NUM_REGISTER).chain(ARG_REGISTERS) {
        assert!(
            probed.iter().any(|n| n == nm),
            "registerProgram never resolved {nm}, so the i386 syscall ABI probe would \
             miss it in ghidra mode: {probed:?}"
        );
    }
    assert_eq!(r.trace.responses.len(), 2, "one response span per command");
}
