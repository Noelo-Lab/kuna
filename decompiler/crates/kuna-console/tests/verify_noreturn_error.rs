//! End-to-end gate for the `error(nonzero,…)`-conditional no-return recognizer
//! (`noreturn_error`, decbench F2) — the value-conditional slice of Ghidra's
//! discovered-no-return analyzer, folded into the `noreturn_propagate` consumer.
//!
//! Fixture: `noreturn_error_x86_64` (`+.c`), a non-PIE x86-64 ELF modelling GNU
//! diffutils `pfatal_with_name`:
//!
//! ```c
//! __attribute__((noinline, noreturn)) static void err_fatal(const char *m) {
//!     error(2, errno, "%s", m);          // status 2 (nonzero) -> exit(2)
//!     __builtin_unreachable();
//! }
//! __attribute__((noinline)) static int err_warn(const char *m) {
//!     error(0, errno, "%s", m);          // status 0 -> error() RETURNS
//!     return 1;
//! }
//! long compute(long x)      { if (x < 0) err_fatal("negative"); return x*2+7; }
//! long compute_warn(long x) { if (x < 0) return err_warn("neg"); return x+1; }
//! ```
//!
//! Built `gcc -O2 -no-pie -fno-pic`: gcc constprops the wrappers to
//! `err_fatal.constprop.0` / `err_warn.constprop.0`. Because `err_fatal` is
//! `_Noreturn` in source, gcc emits its tail as `mov edi,0x2; call error@plt`
//! with **no `ret`** + NOP alignment padding. But the *stripped* binary carries
//! no annotation, and `error` is NOT a Known no-return (it returns for
//! `error(0,…)`), so:
//!
//!  - the **Known** list never marks `error` / the wrapper, and
//!  - `noreturn_propagate` alone cannot conclude the wrapper: its last real
//!    instruction is `call error@plt`, whose callee is not in the terminal set.
//!
//! `noreturn_error` closes exactly this: it recognises `call error(status,…)`
//! with a **nonzero constant** status (arg0 = x86-64 SysV `EDI` = `0x2`) as a
//! terminal transfer, so the propagation concludes `err_fatal.constprop.0`
//! no-return (called only ONCE — below `noreturn_disc`'s ≥3 threshold), and the
//! single `call err_fatal` in `compute` drops its dead fall-through.
//!
//! The CONTROL is `err_warn` (`error(0,…)`): status 0 ⇒ `error` returns ⇒ the
//! recognizer must NOT conclude it (a false positive would drop live caller
//! code). `compute_warn` therefore never gains a spurious no-return terminator.
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
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/noreturn_error_x86_64")
}

/// Whether the `error(nonzero,…)` recognizer is enabled. In BOTH modes the
/// Listing is built and `noreturn_propagate` is on (its default) — the ONLY
/// difference is `noreturn_error`, so the two modes isolate exactly this feature.
enum Mode {
    /// `listing on`, `noreturn_propagate on`, **`noreturn_error off`** — the
    /// wrapper is NOT concluded (the pre-F2 behaviour).
    ErrorOff,
    /// `listing on`, `noreturn_propagate on`, `noreturn_error on` (default) — the
    /// wrapper IS concluded no-return via the error(nonzero) recognizer.
    ErrorOn,
}

/// Bootstrap the fixture, set the mode's flags, decompile `func`, return the C
/// (`None` ⇒ specs-less skip).
fn decompile(func: &str, mode: Mode) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_noreturn_error: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    // The live-CLI ordering: `option` lines precede `read symbols` (the deferred
    // commit). Build the Listing (`listing on`) and keep `noreturn_propagate` on
    // in BOTH modes; toggle only `noreturn_error`.
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut()
        .set_kuna_option("noreturn_propagate", "on")
        .expect("noreturn_propagate flips on");
    match mode {
        Mode::ErrorOff => {
            prog.arch_mut()
                .set_kuna_option("noreturn_error", "off")
                .expect("noreturn_error flips off");
        }
        Mode::ErrorOn => {
            prog.arch_mut()
                .set_kuna_option("noreturn_error", "on")
                .expect("noreturn_error flips on");
        }
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

/// THE PAYOFF: with `noreturn_error` on, the `error(2,…)` wrapper is concluded
/// no-return, so the single `call err_fatal` in `compute` carries the no-return
/// terminator and the dead fall-through is gone. With it off (propagate still
/// on) the wrapper is treated as returning and the dead code remains.
#[test]
fn error_nonzero_wrapper_concludes_no_return() {
    let Some(off) = decompile("compute", Mode::ErrorOff) else {
        return; // specs-less skip
    };
    let on = decompile("compute", Mode::ErrorOn)
        .expect("second bootstrap succeeds if the first did");

    eprintln!("---- compute (noreturn_error OFF) ----\n{off}");
    eprintln!("---- compute (noreturn_error ON) ----\n{on}");

    // 1. The recognizer changed the output.
    assert_ne!(
        on, off,
        "noreturn_error must change compute's decompilation (it did not):\n\
         --- off ---\n{off}\n--- on ---\n{on}"
    );
    // 2. ON: the wrapper is no-return, so the terminator appears at its call site.
    assert!(
        on.contains("WARNING: Subroutine does not return"),
        "error-on output must carry the no-return terminator after err_fatal():\n{on}"
    );
    // 3. OFF: the wrapper is treated as returning — no terminator.
    assert!(
        !off.contains("WARNING: Subroutine does not return"),
        "error-off output must NOT mark the wrapper no-return (dead code present):\n{off}"
    );
}

/// THE CONTROL: `compute_warn` calls `err_warn`, whose tail is `error(0,…)` —
/// status 0, so `error` RETURNS. The recognizer must NOT conclude it no-return
/// (the zero-status / has-a-`ret` guard); `compute_warn` gains no terminator even
/// with `noreturn_error` on. A false positive here would drop live caller code.
#[test]
fn error_zero_status_wrapper_is_not_concluded() {
    let Some(on) = decompile("compute_warn", Mode::ErrorOn) else {
        return; // specs-less skip
    };
    eprintln!("---- compute_warn (noreturn_error ON) ----\n{on}");
    assert!(
        !on.contains("WARNING: Subroutine does not return"),
        "error(0,…) returns, so err_warn must NOT be marked no-return:\n{on}"
    );
}

/// THE DIFFERENTIAL vs `noreturn_disc`: the wrapper has a SINGLE call site (below
/// the ≥3 evidence threshold), so `noreturn_disc` (the existing discovered
/// consumer) cannot conclude it — only the `error(nonzero)` recognizer does. This
/// mirrors the `noreturn_propagate` differential and shows F2 is a genuine new
/// capability over the name-list / evidence-count seams.
#[test]
fn discovered_consumer_does_not_fix_single_site_error_wrapper() {
    // Bootstrap with listing on + noreturn_disc on, but BOTH propagate and error
    // off, so only the discovered consumer is armed.
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let Some(bin) = fixture().to_str().map(str::to_string) else { return };
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(_) => return, // specs-less skip
    };
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing on");
    prog.arch_mut().set_kuna_option("noreturn_disc", "on").expect("noreturn_disc on");
    prog.arch_mut().set_kuna_option("noreturn_propagate", "off").expect("propagate off");
    prog.arch_mut().set_kuna_option("noreturn_error", "off").expect("error off");
    prog.commit_pending_analysis().expect("commit");

    let cmds: Vec<String> =
        ["load function compute".into(), "decompile".into(), "print C".into()].to_vec();
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
    let disc = status.optr.clone();
    eprintln!("---- compute (noreturn_disc ON only) ----\n{disc}");
    assert!(
        !disc.contains("WARNING: Subroutine does not return"),
        "noreturn_disc must NOT conclude the single-site error(nonzero) wrapper — \
         that is the gap noreturn_error closes:\n{disc}"
    );
}
