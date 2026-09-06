//! pdb end-to-end gate — THE HEADLINE: a Windows PE whose matching `.pdb` sits
//! beside it decompiles with its real function names, with no option line and no
//! environment variable (DIV-129).
//!
//! Fixtures (vendored under `kuna-analysis/tests/fixtures`):
//!   - `pdb_prog.exe` — a freestanding x86-64 PE built `clang -target
//!     x86_64-pc-windows-msvc -g -gcodeview -fuse-ld=lld`, so lld-link emitted a
//!     matching `pdb_prog.pdb` + the RSDS CodeView record naming it. Its function
//!     `pdb_demo_compute` carries NO leftover symbol kuna's loader names from (the
//!     COFF symtab is not a function-naming source), so without the `.pdb` it is a
//!     stripped `FUN_<addr>`.
//!   - `pdb_prog.pdb` — the matching `.pdb`, vendored **beside** the EXE, which is
//!     what the sidecar search finds.
//!   - `pdb_prog_mismatch.pdb` — a DIFFERENT `.pdb` (a different content-hash GUID),
//!     for the fingerprint-gate negative tests.
//! The build recipe + the pinned VMA/GUID are in `tests/fixtures/README.md`.
//!
//! ## What is proved
//!
//!  - **the shipped default, fixture dir**: `pdb_demo_compute` at `0x140001000`,
//!    found through the CodeView record's own filename resolved beside the image.
//!  - **`--option pdb off`**: the same function is an engine `FUN_*`/`sub_*`
//!    placeholder — proving the name is NOT a leftover symbol and that the PDB
//!    pass's value is what is visible.
//!  - **the EXE alone in a directory**: placeholder again — the sidecar, not the
//!    EXE, is where the name comes from.
//!  - **a stale `.pdb` beside the EXE** (the mismatch fixture copied to the
//!    sidecar name): placeholder — the GUID/age gate rejects it, which is what
//!    makes the automatic search safe to run by default.
//!  - **`kuna_pdb_path`**: an EXE with no sidecar still recovers its names from an
//!    explicitly supplied `.pdb`; and an explicit path that does NOT match falls
//!    through to a sidecar that does.
//!
//! Everything runs on the real-PE path (loading an actual PE), so the XML
//! datatest 675/158 oracles never reach this.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// Serializes the `kuna_pdb_path` env-var dance: the pass reads the process-global
/// env at LOAD, so two tests setting it in parallel would clobber each other. Each
/// `#[test]` holds this lock for its whole body, the `verify_macho_fat` `ENV_LOCK`
/// precedent.
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// The pinned VMA of `pdb_demo_compute` in `pdb_prog.exe` (ImageBase 0x140000000 +
/// RVA 0x1000). See `tests/fixtures/README.md`.
const COMPUTE_VMA: u64 = 0x140001000;

/// The function name the PDB pass recovers (the un-mangled source name).
const COMPUTE_NAME: &str = "pdb_demo_compute";

/// The env var naming the `.pdb` to apply (the fid `kuna_fid_db` precedent).
const PDB_PATH_ENV: &str = "kuna_pdb_path";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

/// A scratch directory holding a copy of `pdb_prog.exe` and nothing else, so the
/// sidecar tiers find only what a test puts there.
fn isolated_exe(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("kuna-verify-pdb-{tag}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the scratch dir");
    let exe = dir.join("pdb_prog.exe");
    std::fs::copy(fixtures().join("pdb_prog.exe"), &exe).expect("copy the PE");
    exe
}

/// How to drive one run: which EXE to load, what `kuna_pdb_path` holds, and
/// whether the `pdb` option is explicitly turned off before the commit.
struct Run {
    exe: PathBuf,
    env: Option<PathBuf>,
    force_off: bool,
}

impl Run {
    /// The vendored fixture, in its own directory (the sidecar is present).
    fn fixture() -> Run {
        Run { exe: fixtures().join("pdb_prog.exe"), env: None, force_off: false }
    }
    /// A copy of the fixture alone in a scratch directory (no sidecar).
    fn isolated(tag: &str) -> Run {
        Run { exe: isolated_exe(tag), env: None, force_off: false }
    }
    fn env(mut self, pdb: PathBuf) -> Run {
        self.env = Some(pdb);
        self
    }
    fn option_off(mut self) -> Run {
        self.force_off = true;
        self
    }
    /// Place `src` beside the EXE under the sidecar name the CodeView record uses.
    fn with_sidecar(self, src: &Path) -> Run {
        let dst = self.exe.parent().expect("the EXE has a parent").join("pdb_prog.pdb");
        std::fs::copy(src, dst).expect("install the sidecar");
        self
    }
}

/// Resolve the symbol-table name at `COMPUTE_VMA` (the engine placeholder, or the
/// PDB-recovered name).
fn name_at_compute(prog: &ConsoleProgram) -> String {
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(std::rc::Rc::clone(&code_space), COMPUTE_VMA);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// Bootstrap the run, commit the (gated) PDB facts, and return the symbol-table
/// name at `COMPUTE_VMA`. `None` ⇒ a specs-less skip.
///
/// The `.pdb` is located by the pass at LOAD (the facts are stashed during
/// bootstrap), so `kuna_pdb_path` is set BEFORE `bootstrap_from_object` and always
/// cleared after, so runs do not leak into one another. The `option` line is
/// applied before the deferred commit, matching live-CLI ordering.
fn run(r: Run) -> Option<String> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    assert!(r.exe.exists(), "missing fixture {:?}", r.exe);

    match &r.env {
        Some(p) => std::env::set_var(PDB_PATH_ENV, p),
        None => std::env::remove_var(PDB_PATH_ENV),
    }
    let prog = bootstrap_from_object(r.exe.to_str().unwrap(), "", &spec_roots);
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

    if r.force_off {
        prog.arch_mut().set_kuna_option("pdb", "off").expect("pdb flips off");
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

/// THE HEADLINE: the shipped default names `0x140001000` from the `.pdb` sitting
/// beside the EXE — no option line, no environment variable — and `--option pdb
/// off` puts the stripped placeholder back.
#[test]
fn pdb_sidecar_names_the_function_by_default() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(on) = run(Run::fixture()) else {
        return; // specs-less skip
    };
    let off = run(Run::fixture().option_off()).expect("second bootstrap succeeds if the first did");

    eprintln!("==== 0x{COMPUTE_VMA:x}  default: {on:>24}   option off: {off:>24} ====");

    assert_eq!(
        on, COMPUTE_NAME,
        "the shipped default must name 0x{COMPUTE_VMA:x} {COMPUTE_NAME} from the sidecar .pdb"
    );
    assert!(
        is_placeholder(&off),
        "`option pdb off` must leave 0x{COMPUTE_VMA:x} a generic placeholder, got `{off}`"
    );
    assert_ne!(
        off, COMPUTE_NAME,
        "`option pdb off` must NOT name the stripped function {COMPUTE_NAME} (proves it is not a leftover symbol)"
    );
}

/// The name comes from the SIDECAR, not the EXE: the same PE alone in a directory
/// keeps its placeholder, and gains the name back as soon as the `.pdb` is put
/// beside it.
#[test]
fn pdb_name_follows_the_sidecar_file() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(alone) = run(Run::isolated("alone")) else {
        return; // specs-less skip
    };
    let beside = run(Run::isolated("beside").with_sidecar(&fixtures().join("pdb_prog.pdb")))
        .expect("bootstrap succeeds if the first did");

    eprintln!("==== EXE alone: {alone:>24}   EXE + .pdb: {beside:>24} ====");

    assert!(is_placeholder(&alone), "a PE with no .pdb beside it must stay stripped, got `{alone}`");
    assert_eq!(beside, COMPUTE_NAME, "dropping the matching .pdb beside the PE must name it");
}

/// THE FINGERPRINT GATE on the automatic tier: a STALE `.pdb` sitting beside the
/// EXE under the recorded name is rejected — the function stays a placeholder.
/// This is what makes the sidecar search safe to run by default.
#[test]
fn pdb_fingerprint_gate_rejects_a_stale_sidecar() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let mismatch = fixtures().join("pdb_prog_mismatch.pdb");
    assert!(mismatch.exists(), "missing mismatch fixture {mismatch:?}");

    let Some(stale) = run(Run::isolated("stale").with_sidecar(&mismatch)) else {
        return; // specs-less skip
    };

    eprintln!("==== stale sidecar: {stale:>24} ====");

    assert!(
        is_placeholder(&stale),
        "a guid-mismatch .pdb beside the EXE must be rejected — 0x{COMPUTE_VMA:x} must stay a placeholder, got `{stale}`"
    );
    assert_ne!(
        stale, COMPUTE_NAME,
        "the stale .pdb must NOT recover {COMPUTE_NAME} (the fingerprint gate rejected it)"
    );
}

/// `kuna_pdb_path` still reaches a `.pdb` that is NOT beside the image, and an
/// explicit path that fails the fingerprint gate falls through to a sidecar that
/// passes it.
#[test]
fn pdb_explicit_path_is_tried_first_and_falls_through() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let matching = fixtures().join("pdb_prog.pdb");
    let mismatch = fixtures().join("pdb_prog_mismatch.pdb");

    // No sidecar anywhere near the EXE: only the env var can supply the `.pdb`.
    let Some(explicit) = run(Run::isolated("explicit").env(matching.clone())) else {
        return; // specs-less skip
    };
    // An explicit path that does not match, with a matching sidecar present.
    let fell_through = run(Run::fixture().env(mismatch.clone()))
        .expect("bootstrap succeeds if the first did");
    // An explicit path that does not match, with nothing else to fall back to.
    let nothing_left =
        run(Run::isolated("nofallback").env(mismatch)).expect("bootstrap succeeds if the first did");

    eprintln!(
        "==== env only: {explicit:>24}   env mismatch + sidecar: {fell_through:>24}   env mismatch alone: {nothing_left:>24} ===="
    );

    assert_eq!(explicit, COMPUTE_NAME, "kuna_pdb_path must still reach an out-of-tree .pdb");
    assert_eq!(
        fell_through, COMPUTE_NAME,
        "a mismatching explicit path must not veto a sidecar that does match"
    );
    assert!(
        is_placeholder(&nothing_left),
        "with every candidate rejected the function stays a placeholder, got `{nothing_left}`"
    );
}
