//! GH-407: in ghidra mode a cspec `<callotherfixup>` must be fetched from the
//! host, not compiled locally.
//!
//! There is no `.sla` in ghidra mode, so no injection payload ever gets a
//! compiled template.  Before the fix the first CALLOTHER carrying a fixup
//! hard-errored the whole function, which is why `ARM.cspec`'s `setISAMode`
//! (raised by every interworking BX/BLX) and its nine MIPS twins took out most
//! of both architectures.  The answer now comes from a `getCallOtherFixup`
//! query, and it is p-code already lifted against ONE call site — never a
//! template, so nothing here may be cached.
//!
//! The traffic assertions are the point.  A "fix" that quietly degraded the
//! user-op to a black box would also produce C for every target below, so each
//! drive pins that the query really fired (and that AArch64, whose cspec has no
//! fixup at all, still issues none).

mod ghidra_sim;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use kuna_base::marshal::PackedEncode;
use kuna_ghidra::ids::ELEM_COMMAND_GETCALLOTHERFIXUP;
use kuna_ghidra::process::GhidraProcess;

use ghidra_sim::oracle::{generate_tspec, repo_root, InjectFault, SimOracle};
use ghidra_sim::{
    cmd_decompile_at, cmd_deregister_program, cmd_register_program, cmd_set_action,
    parse_decompile_doc, trace_session, MockReader, MockState, MockWriter, SessionTrace,
};

const ARM_LANG: &str = "specs/Ghidra/Processors/ARM/data/languages";
const MIPS_LANG: &str = "specs/Ghidra/Processors/MIPS/data/languages";
const AARCH64_LANG: &str = "specs/Ghidra/Processors/AARCH64/data/languages";

/// Every `fmt_arm` function #407 reports failing, plus `frame_dummy` — the one
/// that succeeded before the fix, and so the control that says the drive itself
/// is not what changed.
const ARM_TARGETS: &[&str] = &[
    "main",
    "register_tm_clones",
    "deregister_tm_clones",
    "__do_global_dtors_aux",
    "_start",
    "call_weak_fn",
    "frame_dummy",
];

struct Run {
    oracle: SimOracle,
    trace: SessionTrace,
    /// One entry per driven target, in wire order: the `decompileAt` payload
    /// size and the C it flattens to (empty when the payload was empty).
    results: Vec<(usize, String)>,
}

impl Run {
    /// How many `getCallOtherFixup` queries the whole session issued.
    fn callother_fixups(&self) -> u64 {
        self.oracle
            .log
            .counts
            .get(&ELEM_COMMAND_GETCALLOTHERFIXUP.get_id())
            .copied()
            .unwrap_or(0)
    }
}

/// Drive registerProgram → setAction → one decompileAt per target →
/// deregisterProgram against the in-process [`GhidraProcess`], with the host
/// end answered by the sim oracle.
///
/// `None` when the `.sla` specs are not built (the visible skip the CI canary
/// greps for).
fn run(
    binary: &Path,
    lang_dir: &str,
    pspec_name: &str,
    cspec_name: &str,
    targets: &[&str],
    fault: Option<InjectFault>,
) -> Option<Run> {
    let mut oracle = SimOracle::bootstrap(binary)?;
    oracle.inject_fault = fault;
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
    cmd_set_action(&mut commands, "0", "decompile", "c");
    for target in targets {
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
        cmd_decompile_at(&mut commands, "0", &packed_addr);
    }
    cmd_deregister_program(&mut commands, "0");
    let n = targets.len() + 3;

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
    let results = (0..targets.len())
        .map(|i| match trace.responses[i + 2].payload.as_deref() {
            Some(p) if !p.is_empty() => {
                (p.len(), parse_decompile_doc(p, &oracle.manager).c_text)
            }
            other => (other.map(|p| p.len()).unwrap_or(0), String::new()),
        })
        .collect();
    Some(Run {
        trace,
        oracle,
        results,
    })
}

/// Assert every driven target produced a function, and say which did not.
fn assert_all_decompiled(r: &Run, targets: &[&str], what: &str) {
    let failed: Vec<String> = targets
        .iter()
        .zip(&r.results)
        .filter(|(_, (size, _))| *size == 0)
        .enumerate()
        .map(|(i, (name, _))| {
            format!("{name} ({})", r.trace.responses[i + 2].warnings.trim())
        })
        .collect();
    assert!(
        failed.is_empty(),
        "{what}: {} of {} functions came back with an EMPTY decompileAt payload, \
         which Ghidra reports as `Expecting <doc> but did not scan an element`: {failed:?}",
        failed.len(),
        targets.len()
    );
    for (i, name) in targets.iter().enumerate() {
        assert!(
            r.trace.responses[i + 2].warnings.trim().is_empty(),
            "{what}/{name} warned: {}",
            r.trace.responses[i + 2].warnings
        );
    }
}

/// The issue's own repro: six of `fmt_arm`'s seven functions failed entirely,
/// each on `ARM.cspec`'s `setISAMode` fixup.
#[test]
fn arm32_fetches_the_setisamode_fixup_from_the_host() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fmt_arm");
    let Some(r) = run(
        &binary,
        ARM_LANG,
        "ARMt.pspec",
        "ARM.cspec",
        ARM_TARGETS,
        None,
    ) else {
        return;
    };
    assert_all_decompiled(&r, ARM_TARGETS, "ARM:LE:32 fmt_arm");

    let main_c = &r.results[0].1;
    assert!(
        main_c.contains("printf("),
        "ARM main did not recover the printf call:\n{main_c}"
    );
    // A degrade would leave the user-op standing as a black-box call instead.
    for (name, (_, c)) in ARM_TARGETS.iter().zip(&r.results) {
        assert!(
            !c.contains("setISAMode"),
            "{name}: the setISAMode CALLOTHER survived into the output:\n{c}"
        );
    }
    assert!(
        r.callother_fixups() > 0,
        "no getCallOtherFixup query was ever issued, so the C above did not come \
         from the host's fixup: {:?}",
        r.oracle.log.counts
    );
    assert!(
        r.trace.responses[0].warnings.trim().is_empty(),
        "registerProgram warnings: {}",
        r.trace.responses[0].warnings
    );
}

/// The second architecture family, and the only one whose fixup body is
/// `v0 = v0` rather than `r0 = r0`: all nine vendored MIPS cspecs carry a
/// `<callotherfixup targetop="setISAMode">`.
#[test]
fn mips32_fetches_the_setisamode_fixup_from_the_host() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/plt_mips32");
    let targets = ["main"];
    let Some(r) = run(
        &binary,
        MIPS_LANG,
        "mips32.pspec",
        "mips32be.cspec",
        &targets,
        None,
    ) else {
        return;
    };
    assert_all_decompiled(&r, &targets, "MIPS:BE:32 plt_mips32");
    assert!(
        r.callother_fixups() > 0,
        "no getCallOtherFixup query was issued on MIPS: {:?}",
        r.oracle.log.counts
    );
}

/// The negative control: `AARCH64.cspec` declares no `<callotherfixup>` at all,
/// so the seam must stay entirely off the wire there — and the output must be
/// the one AArch64 already produced.
#[test]
fn aarch64_issues_no_inject_query() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fmt_aarch64");
    let targets = ["main", "frame_dummy", "_start"];
    let Some(r) = run(
        &binary,
        AARCH64_LANG,
        "AARCH64.pspec",
        "AARCH64.cspec",
        &targets,
        None,
    ) else {
        return;
    };
    assert_all_decompiled(&r, &targets, "AArch64 fmt_aarch64");
    assert_eq!(
        r.callother_fixups(),
        0,
        "AArch64 has no cspec fixup, so it must issue no getCallOtherFixup query"
    );
}

/// The host throwing (an unregistered payload name) must NOT come back as a
/// Java exception: that would abort the whole decompileAt command as a
/// `DecompileException` instead of the clean incomplete-function shape.
/// Upstream catches it for exactly this reason (inject_ghidra.cc:58-59).
#[test]
fn a_thrown_inject_stays_a_low_level_error() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fmt_arm");
    let targets = ["main", "frame_dummy"];
    let Some(r) = run(
        &binary,
        ARM_LANG,
        "ARMt.pspec",
        "ARM.cspec",
        &targets,
        Some(InjectFault::NotFound),
    ) else {
        return;
    };
    assert_eq!(r.results[0].0, 0, "main should not have decompiled");
    assert!(
        r.trace.responses[2]
            .warnings
            .contains("Injection error: No p-code injection with name: setISAMode"),
        "main's warning did not carry the host's exception message: {}",
        r.trace.responses[2].warnings
    );
    // The session survived: the next command was still answered, with C.
    assert!(
        r.results[1].0 > 0 && r.results[1].1.contains("void"),
        "frame_dummy after the thrown inject: {:?}",
        r.results[1]
    );
}

/// The host's other decline — an empty response, which it sends when there is
/// no instruction at the base address or the payload lifted to nothing.
#[test]
fn an_empty_inject_response_fails_only_that_function() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fmt_arm");
    let targets = ["main", "frame_dummy"];
    let Some(r) = run(
        &binary,
        ARM_LANG,
        "ARMt.pspec",
        "ARM.cspec",
        &targets,
        Some(InjectFault::NoPcode),
    ) else {
        return;
    };
    assert_eq!(r.results[0].0, 0, "main should not have decompiled");
    assert!(
        r.trace.responses[2]
            .warnings
            .contains("Could not retrieve injection: setISAMode"),
        "main's warning did not name the unanswered injection: {}",
        r.trace.responses[2].warnings
    );
    assert!(
        r.results[1].0 > 0,
        "frame_dummy after the empty inject: {:?}",
        r.results[1]
    );
}
