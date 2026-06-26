//! End-to-end gate for `.eh_frame` LSDA landing-pad discovery (the gated
//! `eh_frame_full` analysis option — the GccExceptionAnalyzer full-markup product).
//!
//! Fixture: `eh_lsda_x86_64`, a C++ try/catch ELF (non-PIE x86-64) **stripped of
//! `.symtab`** so the exception landing pads are not otherwise named. The
//! `guarded()` function's `.gcc_except_table` LSDA call-site table carries three
//! landing pads (the catch dispatch + two cleanup blocks); the `may_throw()` LSDA
//! one. The headline landing pad is `0x4012e2` (the catch-dispatch `endbr64`
//! inside `guarded`).
//!
//! ## The proof (discovery, not `--addr`)
//!
//! A landing pad sits **mid-function** (it is reached only by the unwinder), so the
//! FDE-pcBegin / prologue / libc-start oracles never see it — only the LSDA
//! call-site table does. The gate:
//!
//!  - **default (option OFF):** `0x4012e2` is NOT registered as a function symbol
//!    (`sub_4012e2` absent). The discovery set is byte-identical to the
//!    FDE-pcBegin-only behavior — every parity gate is untouched.
//!  - **`--option eh_frame_full on`:** the LSDA call-site table is decoded and each
//!    non-zero landing pad is committed as a discovered entry, so `sub_4012e2`
//!    (and the other landing pads) register and decompile by name.
//!
//! Like the sibling analysis-pass gates (`verify_go_pclntab`, `verify_noreturn_disc`),
//! the analysis facts are committed at `read symbols` (`commit_pending_analysis`),
//! gated by the per-pass flags — so the option must be flipped BEFORE that commit.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored stripped C++ try/catch fixture (shared with the kuna-analysis
/// LSDA unit tests).
fn eh_lsda() -> Option<String> {
    let p = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/eh_lsda_x86_64");
    p.to_str().map(|s| s.to_string())
}

/// Bootstrap the fixture, optionally flip `eh_frame_full`, then commit the
/// (deferred) analysis facts. Returns `None` (a visible skip) if the `.sla` is
/// missing.
fn bootstrap_with_eh_frame_full(on: bool) -> Option<kuna_console::engine::ConsoleProgram> {
    let bin = eh_lsda()?;
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_eh_frame_full: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    if on {
        prog.arch_mut()
            .set_kuna_option("eh_frame_full", "on")
            .expect("eh_frame_full flips on");
    }
    // Analysis facts commit at `read symbols` (gated by the per-pass flags), not
    // eagerly at bootstrap — trigger that commit after the option is applied.
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// AFTER: with `--option eh_frame_full on`, the `guarded` catch-dispatch landing
/// pad `0x4012e2` is discovered from the `.gcc_except_table` LSDA and registers as
/// `sub_4012e2` — a function reached only by the unwinder, never by any other
/// oracle. It then decompiles by name (a real C body), with no `--addr`.
#[test]
fn landing_pad_discovered_and_decompiles_with_option_on() {
    let Some(prog) = bootstrap_with_eh_frame_full(true) else {
        return;
    };

    // The discovered landing pad registered under its angr-style `sub_<addr>` name.
    assert!(
        prog.lookup_symbol("sub_4012e2").is_some(),
        "landing pad 0x4012e2 (guarded's catch dispatch) must be discovered + \
         registered as sub_4012e2 with eh_frame_full on"
    );

    // Drive `load function sub_4012e2` -> `decompile` -> `print C` WITHOUT --addr:
    // the discovered name resolves to a real function with a C body.
    let cmds: Vec<String> = ["load function sub_4012e2", "decompile", "print C"]
        .iter()
        .map(|s| s.to_string())
        .collect();
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

    assert!(
        out.contains("sub_4012e2"),
        "expected a decompiled body for the discovered sub_4012e2, got:\n{out}"
    );
    assert!(
        out.contains('{') && out.contains('}'),
        "expected a C function body for sub_4012e2, got:\n{out}"
    );
    assert!(
        !out.contains("Unknown function") && !out.contains("no function"),
        "sub_4012e2 should resolve to a function, got:\n{out}"
    );
}

/// BEFORE (the default-off gate): with the option OFF, the landing pad is NOT
/// discovered — `sub_4012e2` is absent from the symbol table. This is the parity
/// proof: a default run's discovery set excludes the LSDA landing pads, so the
/// option is purely additive (no parity gate moves).
#[test]
fn landing_pad_absent_with_option_off() {
    let Some(prog) = bootstrap_with_eh_frame_full(false) else {
        return;
    };
    assert!(
        prog.lookup_symbol("sub_4012e2").is_none(),
        "landing pad 0x4012e2 must NOT be a registered function by default \
         (eh_frame_full off) — discovery must be byte-identical to FDE-pcBegin only"
    );
}
