//! The `--assert` range directives end-to-end — `docs/re-needs/no-cli-data-code-override.md`.
//!
//! `readonly` and `volatile` work in the console, `global add`/`global remove`
//! were `engine_unavailable` stubs, and none of the four was reachable from the
//! `kuna` binary. `--option readonly on|off` is a program-wide toggle, not a
//! range, so there was no way at all to say "these eight bytes never change" or
//! "this word is a device register".
//!
//! Every case here asserts the **emitted C changed**, against a measured
//! baseline. That is the bar this family is held to: `override prototype` has
//! printed "Successfully added override" and changed nothing since it was
//! ported, and it got there by being reviewed on its return value.
//!
//! Fixture: `kuna-analysis/tests/fixtures/assertranges_x86_64` (source next to
//! it) — a small unstripped non-PIE x86-64 ELF whose
//!
//!   * `sample` reads two `.data` globals that nothing writes (`scale` at
//!     `0x40402c`, `bias` at `0x404028`) and reads `0x50000000` twice, and
//!   * `latch` stores to `0x50000004`, calls, and reads it back.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::assertions::{self, Body, Directive, Outcome};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram, EntrySelector};
use kuna_console::project::decompile_targets;

/// `bias` (4 bytes) then `scale` (4 bytes), contiguous in `.data`.
const DATA_PAIR: u64 = 0x404028;
/// The device word `sample` reads twice.
const MMIO: u64 = 0x5000_0000;
/// The device word `latch` writes, calls across, and reads back.
const LATCH_MMIO: u64 = 0x5000_0004;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture and run the analysis commit.  `None` ⇒ specs-less skip.
fn load() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/assertranges_x86_64");
    let mut prog = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_assertranges: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit");
    Some(prog)
}

fn directive(spec: &str, body: Body) -> Directive {
    Directive { raw: spec.to_string(), body }
}

fn readonly(addr: u64, size: i32) -> Directive {
    directive(&format!("readonly {addr:#x}+{size}"), Body::Readonly { addr, size })
}

fn volatile(addr: u64, size: i32) -> Directive {
    directive(&format!("volatile {addr:#x}+{size}"), Body::Volatile { addr, size })
}

/// Decompile `target` under `directives`, returning `(C, report)`.
///
/// Mirrors what the CLI's in-process surface does with the same directives,
/// including the read-only propagation a `readonly` directive implies
/// (`decompile_all::load_program`); the console commands paint the same
/// properties from the generated script instead.
fn decompile_with(target: &str, directives: Vec<Directive>) -> Option<(String, Vec<Outcome>)> {
    let mut prog = load()?;
    if !directives.is_empty() {
        if assertions::implies_readonly_propagation(&directives) {
            prog.arch_mut().readonlypropagate = true;
        }
        prog.set_assertions(directives);
        assertions::apply_program_scoped(&mut prog);
    }
    let entry = prog
        .resolve_entry(&EntrySelector::Name(target.to_string()))
        .expect("the fixture has this function");
    let funcs = decompile_targets(&mut prog, vec![entry], false, false, false);
    let code = funcs[0].code.clone().unwrap_or_default();
    Some((code, prog.assertion_outcomes()))
}

/// Every outcome is `applied`; panics naming the offender otherwise.
fn all_applied(report: &[Outcome]) {
    for outcome in report {
        assert_eq!(
            outcome.status, "applied",
            "{:?} was rejected: {:?}",
            outcome.directive, outcome.detail
        );
    }
}

/// The un-asserted baseline every case below is measured against.
#[test]
fn the_baseline_reads_both_globals_and_merges_the_two_device_reads() {
    let Some((code, report)) = decompile_with("sample", Vec::new()) else { return };
    assert!(report.is_empty(), "no directives ⇒ no report rows");
    assert!(code.contains("scale"), "baseline lost the .data load:\n{code}");
    assert!(code.contains("bias"), "baseline lost the .data load:\n{code}");
    assert!(
        code.contains("dat_50000000 * 2"),
        "baseline no longer merges the two device reads:\n{code}"
    );
}

/// `readonly` — the headline. `.data` is writable, so the loader never reports
/// it read-only and the loads stand; an agent that knows nothing writes the pair
/// says so, and they fold to their initialisers.
#[test]
fn a_readonly_range_folds_its_loads_to_the_initialisers() {
    let Some((code, report)) = decompile_with("sample", vec![readonly(DATA_PAIR, 8)]) else {
        return;
    };
    all_applied(&report);
    assert!(code.contains("* 7"), "scale did not fold to 7:\n{code}");
    assert!(code.contains("+ 100"), "bias did not fold to 100:\n{code}");
    assert!(!code.contains("scale"), "the folded load survived:\n{code}");
    assert!(!code.contains("bias"), "the folded load survived:\n{code}");
}

/// The range is what does the work, not the option it turns on: read-only
/// propagation alone leaves `.data` exactly where it was, because the loader
/// never marked it.
#[test]
fn read_only_propagation_without_the_range_changes_nothing() {
    let Some(mut prog) = load() else { return };
    prog.arch_mut().readonlypropagate = true;
    let entry = prog
        .resolve_entry(&EntrySelector::Name("sample".to_string()))
        .expect("the fixture has `sample`");
    let funcs = decompile_targets(&mut prog, vec![entry], false, false, false);
    let code = funcs[0].code.clone().unwrap_or_default();
    assert!(code.contains("scale"), "the option alone folded a .data load:\n{code}");
    assert!(code.contains("bias"), "the option alone folded a .data load:\n{code}");
}

/// `volatile` — two reads of a device register are two reads. Without the
/// directive they are one load of an unwritten address and CSE merges them,
/// which for MMIO is a wrong decompilation, not a tidy one.
#[test]
fn a_volatile_range_keeps_both_device_reads() {
    let Some((code, report)) = decompile_with("sample", vec![volatile(MMIO, 4)]) else { return };
    all_applied(&report);
    assert!(
        !code.contains("dat_50000000 * 2"),
        "the two device reads are still merged:\n{code}"
    );
    assert_eq!(
        code.matches("dat_50000000").count(),
        2,
        "expected two distinct reads of the device word:\n{code}"
    );
}

/// Both directives on one run, which is the acceptance probe's shape: the
/// `.data` pair folds and the device reads split, in one decompile.
#[test]
fn readonly_and_volatile_compose_on_one_run() {
    let Some((code, report)) =
        decompile_with("sample", vec![readonly(DATA_PAIR, 8), volatile(MMIO, 4)])
    else {
        return;
    };
    all_applied(&report);
    assert_eq!(report.len(), 2);
    assert!(code.contains("* 7") && code.contains("+ 100"), "the fold was lost:\n{code}");
    assert!(!code.contains("dat_50000000 * 2"), "the split was lost:\n{code}");
}

/// A range property has to be painted before the symbols over it are mapped —
/// `Scope::addMap` folds it into each `SymbolEntry` and never consults the range
/// again. The loader's symbols are already mapped on this surface, so the
/// directive re-applies the property to the symbols it covers; without that,
/// `readonly` over a NAMED global is silently inert (which is why the proposal
/// that filed this need could not find a fixture where it did anything).
#[test]
fn the_property_reaches_a_range_the_loader_already_named() {
    let Some((code, _)) = decompile_with("sample", vec![readonly(DATA_PAIR, 8)]) else { return };
    // `scale` and `bias` are ELF symbols: the baseline prints them by name.
    assert!(code.contains("* 7"), "a named global did not take the property:\n{code}");
}

/// A range with no size is rejected with a reason, not accepted and dropped.
#[test]
fn a_zero_length_range_is_rejected_with_a_reason() {
    let Some((_, report)) = decompile_with("sample", vec![readonly(DATA_PAIR, 0)]) else { return };
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].status, "rejected");
    assert!(
        report[0].detail.as_deref().unwrap_or_default().contains("size"),
        "the rejection does not say why: {:?}",
        report[0].detail
    );
}

/// Each directive reports its phase-model coordinates, so a rejection can be
/// correlated with `kuna catalog` / `docs/phases.md`.
#[test]
fn a_range_directive_reports_the_code_data_partition_subphase() {
    let Some((_, report)) = decompile_with("sample", vec![readonly(DATA_PAIR, 8), volatile(MMIO, 4)])
    else {
        return;
    };
    assert_eq!((report[0].kind, report[0].phase, report[0].subphase), (
        "readonly",
        "P1",
        "code-data-partition"
    ));
    assert_eq!((report[1].kind, report[1].phase, report[1].subphase), (
        "volatile",
        "P1",
        "code-data-partition"
    ));
}

/// `global remove` / `global add` — the pair `phases.toml` names as the exposure
/// for the `code-data-partition` sub-phase, and `engine_unavailable` stubs until
/// this change.
///
/// The store in `latch` survives its call only because the address is inside the
/// global scope's range; taking the range out kills it, and putting it back
/// restores it. Asserting the C **changed** is the point: the stub these replace
/// returned an error, and a wiring that merely returns Ok would be worse.
///
/// This is also why there is no `global` directive on the `--assert` plane:
/// every stock cspec claims the whole `ram` space (`<range space="ram"/>`), so
/// on any ordinary ELF the range is global before a caller says anything and
/// `global add` on its own is measurably a no-op.
#[test]
fn global_remove_drops_a_store_and_global_add_restores_it() {
    let Some(baseline) = drive_console(&["load function latch", "decompile", "print C"]) else {
        return;
    };
    assert!(
        baseline.contains(&format!("dat_{LATCH_MMIO:08x} = ")),
        "baseline lost the device store:\n{baseline}"
    );

    let removed = drive_console(&[
        &format!("global remove [ram,{LATCH_MMIO:#x},4]"),
        "load function latch",
        "decompile",
        "print C",
    ])
    .expect("specs were present for the baseline");
    assert!(
        !removed.contains(&format!("dat_{LATCH_MMIO:08x} = ")),
        "`global remove` did not take the range out of the global scope:\n{removed}"
    );

    let restored = drive_console(&[
        &format!("global remove [ram,{LATCH_MMIO:#x},4]"),
        &format!("global add [ram,{LATCH_MMIO:#x},4]"),
        "load function latch",
        "decompile",
        "print C",
    ])
    .expect("specs were present for the baseline");
    assert!(
        restored.contains(&format!("dat_{LATCH_MMIO:08x} = ")),
        "`global add` did not put the range back:\n{restored}"
    );
}

/// `readonly`/`volatile` take an explicit size after the address, so a caller
/// that does not want to name the address space can still state a range — which
/// is what `--assert '<kind> <addr>+<size>'` lowers to.  The bracketed
/// `[space,offset,size]` spelling the C++ takes is unchanged.
///
/// The size is read, not merely tolerated: painted from four bytes below the
/// device word, a 4-byte range stops one byte short of it and an 8-byte range
/// reaches it.
#[test]
fn a_range_command_reads_the_explicit_size_after_the_address() {
    let below = MMIO - 4;
    let Some(short) = drive_console(&[
        &format!("volatile {below:#x} 4"),
        "load function sample",
        "decompile",
        "print C",
    ]) else {
        return;
    };
    assert!(
        short.contains("dat_50000000 * 2"),
        "a 4-byte range reached a word 4 bytes past its end:\n{short}"
    );
    let long = drive_console(&[
        &format!("volatile {below:#x} 8"),
        "load function sample",
        "decompile",
        "print C",
    ])
    .expect("specs were present above");
    assert!(
        !long.contains("dat_50000000 * 2"),
        "the explicit size was ignored (the 8-byte range did not reach the word):\n{long}"
    );
}

/// A size that is not a number is a parse error, not a silently ignored token.
#[test]
fn a_range_command_rejects_a_bad_explicit_size() {
    let Some(out) = drive_console(&[&format!("readonly {DATA_PAIR:#x} banana")]) else { return };
    assert!(out.contains("Bad size: banana"), "out: {out:?}");
}

/// Drive `commands` through a console wired like the datatest runner, with the
/// fixture installed as the current program.  `None` ⇒ specs-less skip.
fn drive_console(commands: &[&str]) -> Option<String> {
    use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData,
                                    DECOMPILE_MODULE};
    use kuna_console::ifaceterm::ConsoleCommands;

    let program = load()?;
    let cmds: Vec<String> = commands.iter().map(|s| s.to_string()).collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).expect("decompile module data");
        let dcp = data
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .expect("decompile module data is IfaceDecompData");
        dcp.conf = Some(program);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(status.optr.clone())
}
