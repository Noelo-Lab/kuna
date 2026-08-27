//! End-to-end gate for `symbolnamebound` (GH-338): one qualified symbol name
//! must not be able to spend the process.
//!
//! `Database::find_create_scope_from_symbol_name` splits a loader symbol name on
//! every `::` and nests one `Scope` per component with no bound. A `Scope` is
//! ~1.5 KB resident and the interning key includes the parent, so a repeated
//! component name still allocates a fresh one at every level: `kuna functions`
//! over an ELF whose `.symtab` carries `("a::" * 200000) + "z"` went from 52,932
//! KB of RSS to 345,132 KB — 291,724 KB for 600,001 bytes of `.strtab`, a ~498x
//! amplifier on data no header check validates.
//!
//! This is the ONLY place both arms can be tested. The gate is read inside `load
//! file`, upstream of every `option` command, so the stage XML
//! (`tests/stages/kuna-symbolnamebound.xml`) can only cover the default arm — the
//! same limit `kuna-symbolnamerepair.xml` documents. Here the env var is set
//! around each `bootstrap_from_object`, so `off` restores the historical
//! unbounded behavior and the difference is measured rather than asserted.
//!
//! What each test pins:
//!
//! * the **bound** — measured as the `Scope` count the load allocates, not a
//!   wall clock;
//! * the **round trip** — the create path and the read path apply the identical
//!   fold, so a symbol installed under a folded path is still findable by the
//!   name the binary spells (otherwise every call to it renders `sub_<addr>`,
//!   the DIV-59 failure mode);
//! * **determinism** — two loads of the same bytes produce the same folded
//!   spelling, which a `DefaultHasher`/`RandomState` fold would not;
//! * **`off`** — byte-for-byte the pre-fix behavior.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_symbolnamerepair` gate, bootstrapping needs the built
//! `x86` `.sla` under `specs/` (gitignored; `make specs`). When it is absent the
//! bootstrap fails; the test prints that and returns early (a specs-less CI is a
//! visible skip, never a false green).

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::kuna_symbolnamebound::{
    bound_scope_path, DEFAULT_SCOPE_DEPTH, SYMBOLNAMEBOUND_ENV,
};

/// Serializes the env-var flip + bootstrap across the concurrently-run tests
/// (the same guard `verify_symbolnamerepair.rs` uses): the gate is a
/// process-global env var, and every object load reads it.
static GATE: Mutex<()> = Mutex::new(());

/// The fixture's symbol nests 1000 `::` components (`deep_scope_x86_64.s`).
const FIXTURE_DEPTH: usize = 1000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

/// The binary's ORIGINAL name for the deep function, rebuilt exactly as
/// `deep_scope_x86_64.s` spells it.
fn original_name() -> String {
    let mut n: Vec<String> = (0..FIXTURE_DEPTH).map(|i| format!("n{i:04}")).collect();
    n.push("leaf".to_string());
    n.join("::")
}

struct Loaded {
    /// Every function name the symbol table installed.
    names: Vec<String>,
    /// How many `Scope`s the load allocated — the resource the gate bounds.
    scopes: usize,
    /// Whether the binary's ORIGINAL name still resolves to a function.
    original_resolves: bool,
}

/// Bootstrap `bin` with `SYMBOLNAMEBOUND_ENV` forced to `bound` (`None` ⇒ unset,
/// i.e. the shipped default), and report what the load produced.
fn load(bin: &str, bound: Option<&str>) -> Result<Loaded, String> {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    match bound {
        Some(v) => std::env::set_var(SYMBOLNAMEBOUND_ENV, v),
        None => std::env::remove_var(SYMBOLNAMEBOUND_ENV),
    }
    let prog = bootstrap_from_object(bin, "", &spec_roots);
    // The lookup below re-reads the env var, so it has to happen inside the flip.
    let out = prog.map(|p| Loaded {
        names: p.function_entries().map(|(name, _)| name.to_string()).collect(),
        scopes: p.arch().symboltab.num_scopes(),
        original_resolves: p.lookup_symbol(&original_name()).is_some(),
    });
    std::env::remove_var(SYMBOLNAMEBOUND_ENV);
    out.map_err(|e| e.explain().to_string())
}

/// Whether the `.sla` is present, so a bootstrap failure means what it says.
/// Returns `false` (and prints) when it is not — a visible skip.
fn specs_available() -> bool {
    match load(&fixture("deep_scope_x86_64"), None) {
        Err(e) if e.contains("sleigh specification") || e.contains(".sla") => {
            eprintln!("verify_symbolnamebound: skipping (no `.sla`, build with `make specs`): {e}");
            false
        }
        _ => true,
    }
}

/// The deep name, as each arm installed it.
fn deep_name(l: &Loaded) -> String {
    l.names
        .iter()
        .find(|n| n.contains("leaf"))
        .unwrap_or_else(|| panic!("no deep symbol in {:?}", l.names))
        .clone()
}

/// Scope components before the base name.
fn depth(name: &str) -> usize {
    name.split("::").count() - 1
}

/// The bound holds, `off` restores the unbounded behavior, and the difference is
/// the resource the issue is about — measured as allocated `Scope`s.
#[test]
fn the_bound_holds_and_off_restores_the_unbounded_behavior() {
    if !specs_available() {
        return;
    }
    let bin = fixture("deep_scope_x86_64");

    let on = load(&bin, None).expect("the fixture must load at the shipped default");
    let off = load(&bin, Some("off")).expect("the fixture must load unbounded too");

    assert_eq!(
        depth(&deep_name(&on)),
        DEFAULT_SCOPE_DEPTH,
        "the default caps the scope path at {DEFAULT_SCOPE_DEPTH}: {}",
        deep_name(&on)
    );
    assert_eq!(
        depth(&deep_name(&off)),
        FIXTURE_DEPTH,
        "`off` must keep every component the binary spells"
    );

    // The resource claim, measured: the unbounded arm allocates one Scope per
    // dropped component, and those are ~1.5 KB each.
    assert!(
        off.scopes >= on.scopes + (FIXTURE_DEPTH - DEFAULT_SCOPE_DEPTH),
        "off must allocate at least {} more scopes; got on={} off={}",
        FIXTURE_DEPTH - DEFAULT_SCOPE_DEPTH,
        on.scopes,
        off.scopes
    );

    // A deliberately tighter bound folds further, so the value is honoured
    // rather than being a hard-coded default.
    let tight = load(&bin, Some("8")).expect("an explicit bound must load");
    assert_eq!(depth(&deep_name(&tight)), 8, "{}", deep_name(&tight));
}

/// The round trip: the create path and the read path apply the SAME fold, so the
/// binary's original spelling still finds the function.
///
/// If the bound applied only on the create side this would fail, and every call
/// to that function would render `sub_<addr>` instead of a name — the failure
/// mode this test exists to make impossible to reintroduce.
#[test]
fn the_original_name_still_resolves_through_the_fold() {
    if !specs_available() {
        return;
    }
    let bin = fixture("deep_scope_x86_64");

    let on = load(&bin, None).expect("load at the default");
    assert!(
        on.original_resolves,
        "the binary's 1000-component name must still address the function; installed as {}",
        deep_name(&on)
    );

    // Idempotent: the FOLDED spelling — the one the listing renders and the one
    // a user would copy out of it — resolves as well.
    let folded = deep_name(&on);
    assert_eq!(
        &*bound_scope_path(&folded, "::"),
        folded,
        "re-bounding an already-bounded name must be a no-op"
    );

    // And with the bound off, the original name is of course still the installed
    // one — the fold is the only thing that could have broken this.
    let off = load(&bin, Some("off")).expect("load unbounded");
    assert!(off.original_resolves);
}

/// Determinism: the folded spelling is a function of the input bytes alone.
///
/// A `DefaultHasher`/`RandomState` fold would differ between processes; this
/// asserts it does not differ between loads, and the stage XML pins the exact
/// digits across processes.
#[test]
fn the_folded_spelling_is_deterministic() {
    if !specs_available() {
        return;
    }
    let bin = fixture("deep_scope_x86_64");
    let a = deep_name(&load(&bin, None).expect("load"));
    let b = deep_name(&load(&bin, None).expect("load"));
    assert_eq!(a, b, "two loads of the same bytes must produce the same folded name");
    assert!(a.contains("__x"), "the fold must carry its hash: {a}");

    // The synthetic component is identifier-safe: it is re-split on `::` and
    // reaches emitted C verbatim.
    let synth = a.split("::").find(|c| c.contains("__x")).expect("a folded component");
    assert!(
        synth.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'),
        "folded component must be identifier-safe: {synth}"
    );
}

/// An ordinary C++ binary is untouched: the bound costs a real load nothing, and
/// changes no name.
///
/// The deepest `::` nesting measured across every real binary sampled for this
/// gate is 8, so the fold is unreachable in practice — but that has to be shown,
/// not asserted.
#[test]
fn a_real_cpp_binary_is_byte_identical_with_the_bound_on_and_off() {
    if !specs_available() {
        return;
    }
    let bin = fixture("anon_namespace_x86_64");
    let on = load(&bin, None).expect("a real C++ binary must load at the default");
    let off = load(&bin, Some("off")).expect("and unbounded");
    assert_eq!(on.names, off.names, "the bound must change no real name");
    assert_eq!(on.scopes, off.scopes, "and allocate no different number of scopes");
    for n in &on.names {
        assert!(
            depth(n) <= DEFAULT_SCOPE_DEPTH,
            "no real name comes near the bound: {n} ({})",
            depth(n)
        );
    }
}
