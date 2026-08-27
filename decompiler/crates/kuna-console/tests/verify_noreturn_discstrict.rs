//! Cross-crate gate for `noreturn_discstrict` (GH-312): a decoder gap must not be
//! able to forge a no-return verdict and delete a caller's live code.
//!
//! Fixture: `noreturn_discstrict_x86_64` (`+.s`, rebuildable with the one-line
//! `gcc` command in its header). `retseven` is `mov $7,%eax ; ret` — it returns on
//! every path. Three callers (`gap1`/`gap2`/`gap3`) follow their `call retseven`
//! with the single byte `0x06`, which is invalid in 64-bit mode. The Listing walk
//! pushes every call's fall-through onto its worklist unconditionally, so each of
//! those successors IS attempted; `decode_one` returns `Err` and the address never
//! becomes an instruction start.
//!
//! The legacy tally reads that as "this call has no valid fall-through" at all
//! three sites — three votes, exactly the evidence threshold — and concludes
//! `retseven` never returns. The engine's inherited flow repair then deletes the
//! code after every call to it, including `usesret`, whose `return retseven() + 3`
//! is ordinary live code.
//!
//! The two arms:
//!
//!  - **default** (`noreturn_discstrict` on): only positive evidence counts — the
//!    successor is data, or another function's entry. The successor of each `call
//!    retseven` is an undecoded byte inside `.text`, which is neither, so
//!    `retseven` collects no votes and `usesret` renders as `return retseven() + 3`.
//!  - **`--option noreturn_discstrict off`**: the legacy three-arm tally, verbatim
//!    — `usesret` collapses to `retseven(); // no-return` and loses its tail.
//!
//! Both arms run with `--option listing on --option noreturn_disc on`, because the
//! whole no-return-discovery family is a Listing consumer and the Listing is only
//! built on the real-object bootstrap path — never on the XML datatest path. That
//! is why the `tests/stages/kuna-noreturn-discstrict.xml` datatest can only prove
//! the option is wired, and the behavior lives here (the same split
//! `verify_noreturn_error.rs` / `ghangr-noreturn-error.xml` use).
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

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

/// Which tally the discovered-no-return consumer uses.
enum Tally {
    /// The shipped default: positive evidence only.
    Strict,
    /// `--option noreturn_discstrict off` — the legacy three-arm tally.
    Legacy,
}

/// Bootstrap the fixture with the Listing + discovered-no-return consumer on,
/// select the tally, decompile `func`, and return the captured C (`None` ⇒
/// specs-less skip).
fn decompile(func: &str, tally: Tally) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_noreturn_discstrict: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    // The live-CLI ordering: every `option` line precedes `read symbols` (the
    // deferred analysis commit), so the Listing build + the consumer see the flags.
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut().set_kuna_option("noreturn_disc", "on").expect("noreturn_disc flips on");
    if let Tally::Legacy = tally {
        prog.arch_mut()
            .set_kuna_option("noreturn_discstrict", "off")
            .expect("noreturn_discstrict flips off");
    }
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
    Some(status.optr.clone())
}

/// THE PAYOFF: three undecodable bytes must not delete `usesret`'s live tail.
#[test]
fn decode_gap_no_longer_forges_a_noreturn_verdict() {
    let Some(strict) = decompile("usesret", Tally::Strict) else {
        return; // specs-less skip
    };
    let legacy = decompile("usesret", Tally::Legacy).expect("second bootstrap succeeds");

    eprintln!("---- usesret (default / strict) ----\n{strict}");
    eprintln!("---- usesret (noreturn_discstrict off / legacy) ----\n{legacy}");

    // Default: the live tail survives and the function still returns a value.
    assert!(
        strict.contains("retseven() + 3"),
        "default output must keep usesret's live tail (`return retseven() + 3`):\n{strict}"
    );
    assert!(
        !strict.contains("// no-return"),
        "default output must not mark the plainly-returning retseven no-return:\n{strict}"
    );

    // Legacy: the forged verdict truncates usesret at the call.
    assert!(
        legacy.contains("// no-return"),
        "legacy tally must forge the no-return verdict (the GH-312 bug):\n{legacy}"
    );
    assert!(
        !legacy.contains("retseven() + 3"),
        "legacy tally must have deleted usesret's tail (the GH-312 bug):\n{legacy}"
    );

    assert_ne!(strict, legacy, "the two tallies must differ on this fixture");
}

/// `retseven` itself is `return 7` under both tallies — the callee's own body is
/// never in question; what changes is whether the tally believes it returns.
#[test]
fn the_victim_returns_a_value_under_both_tallies() {
    let Some(strict) = decompile("retseven", Tally::Strict) else {
        return; // specs-less skip
    };
    let legacy = decompile("retseven", Tally::Legacy).expect("second bootstrap succeeds");
    for (name, body) in [("strict", &strict), ("legacy", &legacy)] {
        assert!(
            body.contains("return 7"),
            "retseven must decompile to `return 7` under the {name} tally:\n{body}"
        );
    }
}

/// The forged fact propagates: under the legacy tally the fixpoint promotes the
/// three gap callers too, and `_start` loses everything after its first call. The
/// default keeps the whole call chain.
#[test]
fn the_forged_fact_propagates_to_the_whole_call_chain() {
    let Some(strict) = decompile("_start", Tally::Strict) else {
        return; // specs-less skip
    };
    let legacy = decompile("_start", Tally::Legacy).expect("second bootstrap succeeds");
    eprintln!("---- _start (default / strict) ----\n{strict}");
    eprintln!("---- _start (legacy) ----\n{legacy}");

    for callee in ["gap1", "gap2", "gap3", "usesret"] {
        assert!(
            strict.contains(callee),
            "default _start must still call {callee}:\n{strict}"
        );
    }
    assert!(
        legacy.contains("gap1") && !legacy.contains("usesret"),
        "legacy _start must be truncated after the first call (the GH-312 bug):\n{legacy}"
    );
}

/// The existing `noreturn_disc` gate's fixture takes ALL of its evidence from the
/// "successor is another function's entry" arm, which the narrowing keeps — so a
/// GENUINE no-return wrapper is still discovered under the default. This is the
/// no-regression half of the change, asserted here so a future edit to the strict
/// predicate cannot silently drop the positive arms.
#[test]
fn a_genuine_noreturn_wrapper_is_still_discovered_under_the_default() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/noreturn_disc_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_noreturn_discstrict: skipping (bootstrap failed): {}",
                e.explain()
            );
            return;
        }
    };
    // Defaults everywhere except the Listing gates: `noreturn_discstrict` is ON.
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut().set_kuna_option("noreturn_disc", "on").expect("noreturn_disc flips on");
    // Isolate the discovered-no-return tally from the propagation consumer, so the
    // conclusion below can only have come from the evidence tally this option
    // narrows.
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
    eprintln!("---- compute_a (strict default, propagate off) ----\n{out}");
    assert!(
        out.contains("// no-return"),
        "the `die` wrapper's evidence is the surviving `next function's entry` arm, \
         so it must still be concluded no-return under the default:\n{out}"
    );
}
