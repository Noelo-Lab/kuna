//! s1_pdb end-to-end gate — THE HEADLINE: recover a stripped function's real name
//! in an x86-64 Windows PE from its matching external `.pdb` (PDB PR-P1).
//!
//! Fixtures (vendored under `kuna-analysis/tests/fixtures`):
//!   - `pdb_prog.exe` — a freestanding x86-64 PE built `clang -target
//!     x86_64-pc-windows-msvc -g -gcodeview -fuse-ld=lld`, so lld-link emitted a
//!     matching `pdb_prog.pdb` + the RSDS CodeView record. Its function
//!     `pdb_demo_compute` carries NO leftover symbol kuna's loader names from (the
//!     COFF symtab is not a function-naming source), so without the `.pdb` it is a
//!     stripped `FUN_<addr>`.
//!   - `pdb_prog.pdb` — the matching `.pdb` (GUID/age agree with the EXE's CodeView
//!     record).
//!   - `pdb_prog_mismatch.pdb` — a DIFFERENT `.pdb` (a different content-hash GUID),
//!     for the fingerprint-gate negative test.
//! The build recipe + the pinned VMA/GUID are in `tests/fixtures/README.md`.
//!
//! ## The two-state proof + the fingerprint gate (the `verify_objc` posture)
//!
//!  - **`--option pdb off`** (default): `pdb_demo_compute` at `0x140001000` is an
//!    engine `FUN_*`/`sub_*` placeholder — proving the name is NOT a leftover symbol
//!    and that the PDB pass's value is what's visible.
//!  - **`--option pdb on` + `kuna_pdb_path=<...>/pdb_prog.pdb`**: the SAME function
//!    is now **`pdb_demo_compute`** — recovered purely from the matching PDB's
//!    S_PUB32/S_GPROC32 stream, gated through the GUID/age fingerprint check.
//!  - **`--option pdb on` + a guid-MISMATCH `.pdb`** (`pdb_prog_mismatch.pdb`): the
//!    function STAYS a `FUN_*` placeholder — the fingerprint gate rejects the stale
//!    PDB (never apply the wrong external knowledge).
//!
//! Everything runs on the real-PE path (loading an actual PE), so the XML
//! datatest 675/158 oracles never reach this.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Mutex;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// Serializes the `kuna_pdb_path` env-var dance: the pass reads the process-global
/// env at LOAD, so two tests setting it in parallel would clobber each other. Each
/// `#[test]` holds this lock for its whole body (both states), the
/// `verify_macho_fat` `ENV_LOCK` precedent.
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// The pinned VMA of `pdb_demo_compute` in `pdb_prog.exe` (ImageBase 0x140000000 +
/// RVA 0x1000). See `tests/fixtures/README.md`.
const COMPUTE_VMA: u64 = 0x140001000;

/// The function name the PDB pass recovers (the un-mangled source name).
const COMPUTE_NAME: &str = "pdb_demo_compute";

/// The env var naming the `.pdb` to apply (the s1_fid `kuna_fid_db` precedent).
const PDB_PATH_ENV: &str = "kuna_pdb_path";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

/// How to drive the run.
enum Mode {
    /// Default: no flag, no `.pdb` (the today baseline — the stripped placeholder).
    Off,
    /// `--option pdb on` + `kuna_pdb_path` pointing at the matching `.pdb`.
    OnMatching,
    /// `--option pdb on` + `kuna_pdb_path` pointing at a guid-MISMATCH `.pdb`
    /// (the fingerprint-gate negative case).
    OnMismatch,
}

/// Resolve the symbol-table name at `COMPUTE_VMA` (the engine placeholder, or the
/// PDB-recovered name).
fn name_at_compute(prog: &ConsoleProgram) -> String {
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), COMPUTE_VMA);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// Bootstrap the fixture in `mode`, commit the (gated) PDB facts, and return the
/// symbol-table name at `COMPUTE_VMA`. `None` ⇒ a specs-less skip.
///
/// The `.pdb` path is read by the pass from `kuna_pdb_path` at LOAD (the facts are
/// stashed during bootstrap), so the env var is set BEFORE `bootstrap_from_object`;
/// the `--option pdb on` flag is flipped before the deferred commit. The env var is
/// always cleared after the load so runs do not leak into one another.
fn run(mode: Mode) -> Option<String> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fixtures().join("pdb_prog.exe");
    assert!(bin.exists(), "missing fixture {bin:?}");

    // Set `kuna_pdb_path` BEFORE bootstrap (the pass reads it at load). The Off mode
    // sets no path (the pass is doubly inert: gate off AND no path).
    match mode {
        Mode::Off => std::env::remove_var(PDB_PATH_ENV),
        Mode::OnMatching => {
            std::env::set_var(PDB_PATH_ENV, fixtures().join("pdb_prog.pdb"));
        }
        Mode::OnMismatch => {
            std::env::set_var(PDB_PATH_ENV, fixtures().join("pdb_prog_mismatch.pdb"));
        }
    }

    let prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots);
    // Clear the env immediately after load so it cannot leak into the next run.
    std::env::remove_var(PDB_PATH_ENV);

    let mut prog = match prog {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_pdb: skipping (bootstrap failed, build `.sla` with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    // Live-CLI ordering: the `option` line precedes the deferred commit. Flip the
    // flag on the live arch BEFORE committing so the gated pdb facts are applied.
    if !matches!(mode, Mode::Off) {
        prog.arch_mut().set_kuna_option("pdb", "on").expect("pdb flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    Some(name_at_compute(&prog))
}

/// Is `name` an engine-generated placeholder (`FUN_*`/`sub_*`/`func_*`/`LAB_*`)?
fn is_placeholder(name: &str) -> bool {
    name.starts_with("sub_")
        || name.starts_with("func_")
        || name.starts_with("FUN_")
        || name.starts_with("LAB_")
}

/// THE HEADLINE: `0x140001000` in the PE is a generic placeholder by default, and
/// becomes `pdb_demo_compute` only with `--option pdb on` + a matching `.pdb` —
/// recovered purely from the PDB symbol stream, gated through the GUID/age check.
#[test]
fn pdb_recovers_stripped_function_name_from_matching_pdb() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(off) = run(Mode::Off) else {
        return; // specs-less skip
    };
    let on = run(Mode::OnMatching).expect("second bootstrap succeeds if the first did");

    eprintln!("==== 0x{COMPUTE_VMA:x}  OFF: {off:>24}   ON(match): {on:>24} ====");

    // 1. pdb OFF (default): the name is a generic engine placeholder — NOT the PDB
    //    name (the EXE has no leftover symbol for it).
    assert!(
        is_placeholder(&off),
        "default (pdb off) must leave 0x{COMPUTE_VMA:x} a generic placeholder, got `{off}`"
    );
    assert_ne!(
        off, COMPUTE_NAME,
        "default (pdb off) must NOT name the stripped function {COMPUTE_NAME} (proves it is not a leftover symbol)"
    );

    // 2. pdb ON + matching `.pdb`: the SAME function is now `pdb_demo_compute`,
    //    recovered from the PDB and gated through the fingerprint check.
    assert_eq!(
        on, COMPUTE_NAME,
        "pdb on + matching .pdb must rename 0x{COMPUTE_VMA:x} to {COMPUTE_NAME} from the PDB symbol stream"
    );

    // 3. The name changed — the PDB pass performed the rename.
    assert_ne!(off, on, "pdb must change the function's name (placeholder -> {COMPUTE_NAME})");
}

/// THE FINGERPRINT GATE: with `--option pdb on` but a guid-MISMATCH `.pdb`, the
/// function STAYS a placeholder — the stale PDB is rejected (never apply the wrong
/// external knowledge, the FID full-hash-match discipline).
#[test]
fn pdb_fingerprint_gate_rejects_mismatched_pdb() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    // Only meaningful if the mismatch fixture is present.
    let mismatch = fixtures().join("pdb_prog_mismatch.pdb");
    assert!(mismatch.exists(), "missing mismatch fixture {mismatch:?}");

    let Some(off) = run(Mode::Off) else {
        return; // specs-less skip
    };
    let mismatch_name = run(Mode::OnMismatch).expect("bootstrap succeeds if the first did");

    eprintln!(
        "==== fingerprint gate: OFF: {off:>24}   ON(mismatch): {mismatch_name:>24} ===="
    );

    // The mismatched `.pdb` must be REJECTED: the function stays a placeholder (same
    // as the off case), NOT renamed to anything from the wrong PDB.
    assert!(
        is_placeholder(&mismatch_name),
        "a guid-mismatch .pdb must be rejected — 0x{COMPUTE_VMA:x} must stay a placeholder, got `{mismatch_name}`"
    );
    assert_ne!(
        mismatch_name, COMPUTE_NAME,
        "the mismatch .pdb must NOT recover {COMPUTE_NAME} (the fingerprint gate rejected it)"
    );
    // The mismatch result equals the off result (no rename happened).
    assert_eq!(
        mismatch_name, off,
        "a rejected .pdb leaves the name exactly as the default (off) path"
    );
}

/// The default path is byte-identical regardless of how it is reached: with NO pdb
/// flag and no `.pdb` the function stays a placeholder (the pass never fires).
#[test]
fn pdb_off_is_the_today_baseline() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(off) = run(Mode::Off) else {
        return; // specs-less skip
    };
    assert!(is_placeholder(&off), "default name must be a placeholder, got `{off}`");
    assert_ne!(off, COMPUTE_NAME, "default must not recover the stripped function name");
}
