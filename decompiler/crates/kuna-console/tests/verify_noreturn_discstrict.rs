//! Cross-crate gate for the positive-evidence no-return tally (GH-312): a decoder
//! gap must not be able to forge a no-return verdict and delete a caller's live
//! code.
//!
//! Fixture: `noreturn_discstrict_x86_64` (`+.s`, rebuildable with the one-line
//! `gcc` command in its header). `retseven` is `mov $7,%eax ; ret` — it returns on
//! every path. Three callers (`gap1`/`gap2`/`gap3`) follow their `call retseven`
//! with the single byte `0x06`, which is invalid in 64-bit mode. The Listing walk
//! pushes every call's fall-through onto its worklist unconditionally, so each of
//! those successors IS attempted; `decode_one` returns `Err` and the address never
//! becomes an instruction start.
//!
//! The legacy tally read that as "this call has no valid fall-through" at all
//! three sites — three votes, exactly the evidence threshold — and concluded
//! `retseven` never returns. The engine's inherited flow repair then deleted the
//! code after every call to it, including `usesret`, whose `return retseven() + 3`
//! is ordinary live code. Now only positive evidence counts — the successor is
//! data, or another function's entry — so `retseven` collects no votes and
//! `usesret` renders as `return retseven() + 3`.
//!
//! Every test runs with `--option listing on --option noreturn_disc on`, because
//! the whole no-return-discovery family is a Listing consumer and the Listing is
//! only built on the real-object bootstrap path — never on the XML datatest path.
//! That is why the `tests/stages/kuna-noreturn-discstrict.xml` datatest can only
//! prove the parent option is wired, and the behavior lives here (the same split
//! `verify_noreturn_error.rs` / `ghangr-noreturn-error.xml` use).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/noreturn_discstrict_x86_64")
}

/// Bootstrap the fixture with the Listing + discovered-no-return consumer on,
/// decompile `func`, and return the captured C.
fn decompile(func: &str) -> String {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str().expect("UTF-8 fixture path").to_string();
    let mut prog = bootstrap_from_object(&bin, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");

    // The live-CLI ordering: every `option` line precedes `read symbols` (the
    // deferred analysis commit), so the Listing build + the consumer see the flags.
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut().set_kuna_option("noreturn_disc", "on").expect("noreturn_disc flips on");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        [format!("load function {func}"), "decompile".into(), "print C".into()].to_vec();
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

/// THE PAYOFF: three undecodable bytes must not delete `usesret`'s live tail.
#[test]
fn decode_gap_does_not_forge_a_noreturn_verdict() {
    let out = decompile("usesret");
    eprintln!("---- usesret ----\n{out}");
    assert!(
        out.contains("retseven() + 3"),
        "usesret must keep its live tail (`return retseven() + 3`):\n{out}"
    );
    assert!(
        !out.contains("// no-return"),
        "the plainly-returning retseven must not be marked no-return:\n{out}"
    );
}

/// `retseven` itself is `return 7` — the callee's own body is never in question.
#[test]
fn the_victim_returns_a_value() {
    let out = decompile("retseven");
    assert!(out.contains("return 7"), "retseven must decompile to `return 7`:\n{out}");
}

/// No forged fact propagates: `_start` keeps its whole call chain.
#[test]
fn the_whole_call_chain_survives() {
    let out = decompile("_start");
    eprintln!("---- _start ----\n{out}");
    for callee in ["gap1", "gap2", "gap3", "usesret"] {
        assert!(out.contains(callee), "_start must still call {callee}:\n{out}");
    }
}

/// The existing `noreturn_disc` gate's fixture takes ALL of its evidence from the
/// "successor is another function's entry" arm, which the narrowing keeps — so a
/// GENUINE no-return wrapper is still discovered. This is the no-regression half
/// of the change, asserted here so a future edit to the predicate cannot silently
/// drop the positive arms.
#[test]
fn a_genuine_noreturn_wrapper_is_still_discovered() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/noreturn_disc_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let mut prog = bootstrap_from_object(&bin, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut().set_kuna_option("noreturn_disc", "on").expect("noreturn_disc flips on");
    // Isolate the discovered-no-return tally from the propagation consumer, so the
    // conclusion below can only have come from the evidence tally.
    prog.arch_mut()
        .set_kuna_option("noreturn_propagate", "off")
        .expect("noreturn_propagate flips off");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        ["load function compute_a".into(), "decompile".into(), "print C".into()].to_vec();
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
    let out = status.optr.clone();
    eprintln!("---- compute_a (propagate off) ----\n{out}");
    assert!(
        out.contains("// no-return"),
        "the `die` wrapper's evidence is the surviving `next function's entry` arm, \
         so it must still be concluded no-return:\n{out}"
    );
}
