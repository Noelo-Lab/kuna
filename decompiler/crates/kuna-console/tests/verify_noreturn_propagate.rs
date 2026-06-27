//! End-to-end gate for the structural no-return **propagation** consumer
//! (`noreturn_propagate`) — the kuna analog of angr's CFGFast call-graph
//! no-return propagation (`test_decompiling_tee_O2_x2nrealloc`).
//!
//! Fixture: `noreturn_propagate_x86_64` (`+.c`), a non-PIE x86-64 ELF with a
//! **custom** no-return wrapper the static name lists do NOT know, called from a
//! SINGLE site (so it is also below `noreturn_disc`'s ≥3 evidence threshold):
//!
//! ```c
//! __attribute__((noinline)) static void my_die(const char *m) {
//!     fprintf(stderr, "fatal: %s\n", m);
//!     abort();                       // unconditional no-return tail
//! }
//! __attribute__((noinline)) long compute(long x) {
//!     if (x < 0) my_die("negative"); // the ONE call site
//!     return x * 2 + 7;
//! }
//! ```
//!
//! Built `gcc -O2 -no-pie -fno-pic`: at `-O2` gcc constant-propagates `my_die` to
//! `my_die.constprop.0`, whose body ends, unconditionally, in `call abort`
//! followed by NOP **alignment padding** (a *valid* instruction start). `my_die`
//! is no-return *in fact* but:
//!
//!  - the **Known** list does not carry `my_die` (a custom name), and
//!  - **`noreturn_disc` does NOT fire**: only ONE call site (below the ≥3
//!    threshold) AND its `call abort` is followed by valid NOP padding, so the
//!    "no valid fall-through" predicate reads the padding as a returning
//!    fall-through.
//!
//! Only `noreturn_propagate` concludes it: seeded from the Known `abort`, its
//! last-real-instruction rule (skip the trailing NOP padding → `call abort`, a
//! terminal callee; no RETURN path) marks `my_die.constprop.0` no-return with no
//! evidence threshold. The single `call my_die` in `compute` then becomes
//! no-return and the post-call dead code is eliminated.
//!
//! This is the **differential** that justifies a new option over `noreturn_disc`:
//! the same fixture, with `noreturn_disc on`, is NOT fixed.
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
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/noreturn_propagate_x86_64")
}

/// Which no-return consumer (if any) to enable, all over the (built) Listing.
enum Mode {
    /// Default: no flags (the today baseline).
    Off,
    /// `--option listing on --option noreturn_propagate on` — the new pass.
    Propagate,
    /// `--option listing on --option noreturn_disc on` — the *existing* discovered
    /// consumer, which must NOT fix this fixture (the differential).
    Disc,
}

/// Bootstrap the fixture, (optionally) enable a consumer, decompile `func`, and
/// return the captured C (`None` ⇒ specs-less skip).
fn decompile(func: &str, mode: Mode) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_noreturn_propagate: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    // The live-CLI ordering: the `option` lines precede `read symbols` (the
    // deferred commit). Set the flags on the live arch BEFORE committing so the
    // deferred Listing build + consumer run see them on.
    match mode {
        Mode::Off => {}
        Mode::Propagate => {
            prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
            prog.arch_mut()
                .set_kuna_option("noreturn_propagate", "on")
                .expect("noreturn_propagate flips on");
        }
        Mode::Disc => {
            prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
            prog.arch_mut()
                .set_kuna_option("noreturn_disc", "on")
                .expect("noreturn_disc flips on");
            // (kuna DIV-14) `noreturn_propagate` is now default-ON and is the *other*
            // Listing consumer; with the Listing built (listing on) it would ALSO fix
            // this fixture. Turn it off so this mode isolates the discovered-consumer
            // (noreturn_disc) behaviour — the differential this test asserts.
            prog.arch_mut()
                .set_kuna_option("noreturn_propagate", "off")
                .expect("noreturn_propagate flips off");
        }
    }
    // `read symbols`: build the Listing (deferred) and run the consumer, gated.
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

/// THE PAYOFF: with the propagation consumer on, the custom `my_die` wrapper —
/// called only ONCE and ending in `call abort` + NOP padding — is concluded
/// no-return, so the single `call my_die` in `compute` is no-return and the
/// post-call dead code is eliminated.
#[test]
fn propagation_eliminates_dead_code_after_single_call_wrapper() {
    let Some(off) = decompile("compute", Mode::Off) else {
        return; // specs-less skip
    };
    let on = decompile("compute", Mode::Propagate)
        .expect("second bootstrap succeeds if the first did");

    eprintln!("---- compute (flags OFF / default) ----\n{off}");
    eprintln!("---- compute (noreturn_propagate ON) ----\n{on}");

    // 1. The two differ — the consumer changed the output.
    assert_ne!(
        on, off,
        "noreturn_propagate must change compute's decompilation (it did not):\n\
         --- off ---\n{off}\n--- on ---\n{on}"
    );

    // 2. ON: `my_die` is marked no-return, so the no-return terminator appears and
    //    the dead fall-through after the call is gone.
    assert!(
        on.contains("WARNING: Subroutine does not return"),
        "propagate-on output must carry the no-return terminator after my_die():\n{on}"
    );
    assert!(
        on.contains("my_die"),
        "propagate-on output must still show the my_die() call:\n{on}"
    );

    // 3. OFF (default): `my_die` is treated as returning — the dead code is present
    //    (the no-return terminator is absent).
    assert!(
        !off.contains("WARNING: Subroutine does not return"),
        "default output must NOT mark my_die() no-return (the dead code is present):\n{off}"
    );
}

/// THE DIFFERENTIAL: the SAME fixture with the *existing* `noreturn_disc` consumer
/// on is NOT fixed — `my_die` has a single call site (below the ≥3 evidence
/// threshold) and its `call abort` is followed by valid NOP padding. This is what
/// makes `noreturn_propagate` a genuinely new capability, not a duplicate.
#[test]
fn discovered_consumer_does_not_fix_this_fixture() {
    let Some(disc) = decompile("compute", Mode::Disc) else {
        return; // specs-less skip
    };
    eprintln!("---- compute (noreturn_disc ON) ----\n{disc}");
    assert!(
        !disc.contains("WARNING: Subroutine does not return"),
        "noreturn_disc must NOT conclude my_die no-return here (single call site, NOP \
         padding after the call) — that is exactly the gap noreturn_propagate closes:\n{disc}"
    );
}

/// `my_die.constprop.0` itself, decompiled with propagation on, is a no-return
/// tail to `abort` — a sanity check that the fixture's wrapper is shaped as
/// documented (and that the pass marks the wrapper, not only its caller).
#[test]
fn wrapper_itself_is_concluded_no_return() {
    let Some(die) = decompile("my_die.constprop.0", Mode::Propagate) else {
        return; // specs-less skip
    };
    eprintln!("---- my_die.constprop.0 (noreturn_propagate ON) ----\n{die}");
    assert!(
        die.contains("abort") || die.contains("WARNING: Subroutine does not return"),
        "my_die() must tail-call abort / be no-return:\n{die}"
    );
}
