//! INDEPENDENT VERIFIER (item: w10-rsp-8-guardfold) — the jump-table guard-fold
//! (`JumpBasic::foldInGuards` / `foldInOneGuard`, `jumptable.cc:1391/1709`) plus
//! the post-fold constant-CBRANCH collapse (`ActionDeterminedBranch::removeBranch`,
//! `coreaction.cc:3688`).
//!
//! The fold absorbs a switch's bound-check guard (`if ((uint4)val <= 10)`) into
//! the jump table as a folded `default:` case.  Two arms of `foldInOneGuard`:
//!   * `pos != nout` (switchind): the guard CBRANCH predicate is set constant —
//!     an `if (1)` residue that the subsequent `ActionDeterminedBranch` collapses
//!     to an unconditional branch (the C++ does this via the re-run of the full
//!     action loop after `getStructure().clear()`).
//!   * `pos == nout` (switchmulti): the guard CBRANCH is pushed into the switch
//!     as an unconditional branch (`pushBranch`), the out-of-range target added as
//!     a new no-label `default:` destination.
//!
//! In BOTH the default out-edge must structure as a real `default:` arm (not a
//! `case 0xbad1abe1bad1abe1:` NO_LABEL leak — the dual-arena structuring resolves
//! the case→default edge against the stable bblocks topology, not the sblocks
//! copy that can lose a wrapped case's in-edge).
//!
//! Adversarial pins (re-derived from the C++ oracle, NOT the Rust port):
//!   1. switchind: the `default:` arm folds in (Switch Indirect #8 → green; 16/16).
//!   2. switchmulti: UNPERTURBED — exactly 7 `case` lines + one `default:`
//!      (Switch Multi #1 stays green; the fold does not pull an 8th labeled case).
//!   3. NO `if (1)` constant-guard residue survives in either render.
//!   4. NO `0xbad1abe1` NO_LABEL leaks into a folded default.

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_harness::testfunction::run_test_files_with_specs;

/// Serializes the `KUNA_DUMP` env set/remove across the parallel tests in this
/// (single) test binary — the dump hook is process-global, so two tests racing
/// on the var could make one observe it unset and capture an empty render.
static DUMP_ENV_LOCK: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn x86_64_sla() -> PathBuf {
    repo_root().join("specs/Ghidra/Processors/x86/data/languages/x86-64.sla")
}

fn specs_dir_string() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

/// Run one datatest file through the engine, returning its full Success/FAIL +
/// KUNA_DUMP `print C` output (the dump hook routes the rendered C into `out`).
fn run_one(stem: &str) -> Option<String> {
    if !x86_64_sla().exists() {
        eprintln!("SKIP: x86-64.sla not built (run `make specs`)");
        return None;
    }
    let path = repo_root().join("decompiler/datatests").join(format!("{stem}.xml"));
    let path = path.to_str().expect("utf8 path").to_string();
    let specs = specs_dir_string();
    let files = vec![path];
    let mut out = String::new();
    // The KUNA_DUMP triage hook routes the rendered `print C` into `out`.  Hold
    // the env lock across set→run→remove so parallel tests in this binary don't
    // race on the process-global var (which would yield an empty render).
    let _guard = DUMP_ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    std::env::set_var("KUNA_DUMP", "1");
    run_test_files_with_specs(&files, std::slice::from_ref(&specs), &mut out);
    std::env::remove_var("KUNA_DUMP");
    drop(_guard);
    assert!(
        !out.contains("Execution failed") && !out.contains("Did not apply tests in"),
        "{stem}.xml EXEC-FAILED under the guard-fold:\n{out}"
    );
    Some(out)
}

/// Extract just the rendered `print C` body from the KUNA_DUMP envelope.
fn dump_body(out: &str) -> String {
    match (out.find("===KUNA_DUMP_BEGIN"), out.find("===KUNA_DUMP_MID===")) {
        (Some(b), Some(m)) => out[b..m].to_string(),
        _ => out.to_string(),
    }
}

/// (1) switchind: the bound-check guard folds into the switch as `default:`, so
/// `Switch Indirect #8` (`default:`) passes — switchind reaches 16/16.
#[test]
fn guardfold_switchind_default_folds_in_16_of_16() {
    let Some(out) = run_one("switchind") else { return };
    for i in 1..=16 {
        let name = format!("Switch Indirect #{i}");
        assert!(
            out.contains(&format!("Success -- {name}\n")),
            "switchind must be 16/16 — `{name}` failed (guard-fold/collapse \
             regressed):\n{out}"
        );
        assert!(
            !out.contains(&format!("FAIL -- {name}\n")),
            "switchind `{name}` FAILed:\n{out}"
        );
    }
    let body = dump_body(&out);
    // The folded `default:` arm replaces the `casefuncdefault();` epilogue.
    assert!(body.contains("default:"), "switchind missing the folded `default:`:\n{body}");
}

/// (2) switchmulti UNPERTURBED: the fold takes the `pushBranch` arm here; the
/// out-of-range target must structure as exactly ONE `default:` arm, leaving the
/// labeled `case` count at SEVEN (`Switch Multi #1` keys on `min=max=7` of
/// `case .*:`).  A regression (the seam the brief warned about) pulls the default
/// in as an 8th labeled `case 0xbad1abe1...`, breaking #1.
#[test]
fn guardfold_switchmulti_first_assertion_unperturbed() {
    let Some(out) = run_one("switchmulti") else { return };
    assert!(
        out.contains("Success -- Switch Multi #1\n"),
        "Switch Multi #1 regressed — the guard-fold pulled in an extra labeled \
         case (must stay exactly 7 `case` lines + one `default:`):\n{out}"
    );
    assert!(
        !out.contains("FAIL -- Switch Multi #1\n"),
        "Switch Multi #1 FAILed under the guard-fold:\n{out}"
    );
    let body = dump_body(&out);
    let case_lines = body.matches("case ").count();
    assert_eq!(
        case_lines, 7,
        "switchmulti must render exactly 7 labeled `case` lines (default folds \
         separately as `default:`), got {case_lines}:\n{body}"
    );
    assert!(body.contains("default:"), "switchmulti missing the folded `default:`:\n{body}");
}

/// (3) NO `if (1)` constant-guard residue: after the fold sets the guard CBRANCH
/// predicate constant, `ActionDeterminedBranch` must collapse it to an
/// unconditional branch.  Neither render may show an `if (1)` (nor `if (0)`).
#[test]
fn guardfold_no_constant_if_residue() {
    for stem in ["switchind", "switchmulti"] {
        let Some(out) = run_one(stem) else { return };
        let body = dump_body(&out);
        assert!(
            !body.contains("if (1)") && !body.contains("if (0)"),
            "{stem}: a constant-guard `if (1)`/`if (0)` residue survived the fold \
             (ActionDeterminedBranch did not collapse it):\n{body}"
        );
    }
}

/// (4) NO `0xbad1abe1` NO_LABEL leak: a folded default that fails to resolve its
/// switch out-edge in the dual-arena structuring would render the NO_LABEL
/// sentinel as a bogus `case 0xbad1abe1bad1abe1:` instead of `default:`.  Neither
/// switch render may contain it.
#[test]
fn guardfold_no_no_label_default_leak() {
    for stem in ["switchind", "switchmulti"] {
        let Some(out) = run_one(stem) else { return };
        let body = dump_body(&out);
        assert!(
            !body.contains("0xbad1abe1"),
            "{stem}: a NO_LABEL (0xbad1abe1...) leaked as a labeled case instead \
             of folding to `default:`:\n{body}"
        );
    }
}
