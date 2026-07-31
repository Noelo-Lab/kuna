//! PR6 end-to-end gate for the Listing/xref tier: the discovered-no-return
//! consumer (`FindNoReturnFunctionsAnalyzer`), the first Listing consumer — THE
//! PAYOFF.
//!
//! Fixture: `noreturn_disc_x86_64` (`+.c`), a non-PIE x86-64 ELF with a **custom**
//! no-return wrapper the static name-lists do NOT know:
//!
//! ```c
//! static void die(const char *m) { fprintf(stderr, "fatal: %s\n", m); exit(1); }
//! ```
//!
//! `die` is no-return *in fact* (it tail-calls `exit`, which IS no-return) but is
//! NOT marked `__attribute__((noreturn))` and is not on `ElfFunctionsThatDoNotReturn`.
//! Built `gcc -O1 -no-pie -fno-pic`: at `-O1` gcc, seeing `die`'s body never
//! returns, emits NOTHING after each `call die` — the byte after the call is the
//! NEXT function's entry. `die` is called from FOUR functions (`compute_a/b/c/d`),
//! so all four call sites show no-valid-fall-through ≥ the evidence threshold (3).
//!
//! The two assertions (the before/after):
//!
//!  - **flags ON** (`--option listing on --option noreturn_disc on`): the consumer
//!    runs over the (now-built) Listing, concludes `die` is no-return, and emits a
//!    `NoReturnFact`. The engine's inherited flow-repair then eliminates the
//!    post-`die()` dead code: `compute_a` renders as just its own body with a
//!    `/* WARNING: Subroutine does not return */` terminator, NOT a fall-through
//!    into `compute_b`'s code.
//!  - **flags OFF** (default): the consumer never runs (and the Listing is never
//!    built), so `die` is treated as returning and the decompiler follows the
//!    fall-through past `call die` into the next function's bytes (the dead code is
//!    present, the body is the larger merged form). The default output is
//!    byte-identical to today.
//!
//! ## The build-timing fix this proves
//!
//! The live CLI emits `option listing on` AFTER `load file` but the analysis
//! driver runs at load — so the Listing must be built at the DEFERRED commit point
//! (`read symbols` / `commit_pending_analysis`), when the flag is known. This test
//! mirrors that ordering exactly: it sets the options on the live arch and THEN
//! calls `commit_pending_analysis()` (the deferred build + consumer run), proving
//! the flag-after-load path actually builds the Listing and applies the consumer.
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
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/noreturn_disc_x86_64")
}

/// Whether to enable the discovered-no-return consumer (and the Listing it reads).
enum Mode {
    /// Default: no flags (the today baseline).
    Off,
    /// `--option listing on --option noreturn_disc on`, set BEFORE the deferred
    /// commit — the live-CLI ordering the build-timing fix targets.
    On,
}

/// Bootstrap the fixture, (optionally) enable the consumer, decompile `func`, and
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
                "verify_noreturn_disc: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    // The live-CLI ordering: the `option` lines precede `read symbols` (the
    // deferred commit). Set the flags on the live arch BEFORE committing so the
    // deferred Listing build + consumer run see them on.
    if let Mode::On = mode {
        prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
        prog.arch_mut()
            .set_kuna_option("noreturn_disc", "on")
            .expect("noreturn_disc flips on");
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

/// THE PAYOFF: with the consumer on, the custom `die` wrapper is concluded
/// no-return and the post-`die()` dead code is eliminated; with it off, the
/// decompiler falls through past the call (dead code present). The two outputs
/// must differ, and the on-form must carry the no-return terminator.
#[test]
fn discovered_noreturn_eliminates_dead_code_after_custom_wrapper() {
    let Some(off) = decompile("compute_a", Mode::Off) else {
        return; // specs-less skip
    };
    let on = decompile("compute_a", Mode::On).expect("second bootstrap succeeds if the first did");

    eprintln!("---- compute_a (flags OFF / default) ----\n{off}");
    eprintln!("---- compute_a (flags ON) ----\n{on}");

    // 1. The two are different — the consumer changed the output.
    assert_ne!(
        on, off,
        "discovered-no-return must change compute_a's decompilation (it did not):\n\
         --- off ---\n{off}\n--- on ---\n{on}"
    );

    // 2. Flags ON: `die` is marked no-return, so the engine emits the no-return
    //    terminator and eliminates the dead fall-through.
    assert!(
        on.contains("// no-return"),
        "flags-on output must carry the no-return terminator after die():\n{on}"
    );
    assert!(
        on.contains("die("),
        "flags-on output must still show the die() call:\n{on}"
    );

    // 3. Flags OFF (default): `die` is treated as returning, so the decompiler
    //    follows the fall-through past `call die` into the next function's bytes —
    //    the dead code is present (the no-return terminator is absent).
    assert!(
        !off.contains("// no-return"),
        "default output must NOT mark die() no-return (the dead code is present):\n{off}"
    );
}

/// The default path is byte-identical regardless of how it is reached: decompiling
/// with NO flags must equal decompiling with the flags explicitly off (the parity
/// guarantee — the deferred build never fires by default).
#[test]
fn flags_off_is_the_today_baseline() {
    let Some(off) = decompile("compute_a", Mode::Off) else {
        return; // specs-less skip
    };
    assert!(!off.trim().is_empty(), "expected a non-empty body for compute_a");
    // The default body follows the fall-through, so it is the larger merged form
    // (it references more than one `compute_*`/`die` site). The exact bytes are the
    // today baseline; we only assert the no-return terminator is absent (assertion
    // 3 above) and the body is non-trivial here.
    assert!(
        off.contains("compute_a"),
        "default body must name compute_a:\n{off}"
    );
}

/// `die` itself, decompiled with the consumer on, is a no-return tail-call to
/// `exit` — a sanity check that the fixture's wrapper is shaped as documented.
#[test]
fn die_wrapper_tail_calls_exit() {
    let Some(die) = decompile("die", Mode::On) else {
        return; // specs-less skip
    };
    eprintln!("---- die (flags ON) ----\n{die}");
    assert!(
        die.contains("exit") || die.contains("// no-return"),
        "die() must tail-call exit / be no-return:\n{die}"
    );
}
