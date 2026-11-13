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
//!    non-zero landing pad is committed as a discovered entry; the FDE-interior
//!    rule then rejects the pads strictly inside their function's FDE, which
//!    `verify_fdeinterior` covers.
//!
//! Like the sibling analysis-pass gates (`verify_go_pclntab`, `verify_noreturn_disc`),
//! the analysis facts are committed at `read symbols` (`commit_pending_analysis`),
//! gated by the per-pass flags — so the option must be flipped BEFORE that commit.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored stripped C++ try/catch fixture (shared with the kuna-analysis
/// LSDA unit tests).
fn eh_lsda() -> String {
    let p = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/eh_lsda_x86_64");
    p.to_str().expect("UTF-8 fixture path").to_string()
}

/// Bootstrap the fixture, optionally flip `eh_frame_full`, then commit the (deferred)
/// analysis facts.
fn bootstrap_with_eh_frame_full(on: bool) -> kuna_console::engine::ConsoleProgram {
    let bin = eh_lsda();
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(&bin, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    if on {
        prog.arch_mut()
            .set_kuna_option("eh_frame_full", "on")
            .expect("eh_frame_full flips on");
    }
    // Analysis facts commit at `read symbols` (gated by the per-pass flags), not
    // eagerly at bootstrap — trigger that commit after the option is applied.
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

/// BEFORE (the default-off gate): with the option OFF, the landing pad is NOT
/// discovered — `sub_4012e2` is absent from the symbol table. This is the parity
/// proof: a default run's discovery set excludes the LSDA landing pads, so the
/// option is purely additive (no parity gate moves).
#[test]
fn landing_pad_absent_with_option_off() {
    let prog = bootstrap_with_eh_frame_full(false);
    assert!(
        prog.lookup_symbol("sub_4012e2").is_none(),
        "landing pad 0x4012e2 must NOT be a registered function by default \
         (eh_frame_full off) — discovery must be byte-identical to FDE-pcBegin only"
    );
}
