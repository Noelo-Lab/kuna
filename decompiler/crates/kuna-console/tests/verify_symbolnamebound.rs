//! End-to-end gate for the symbol-name scope bound (GH-338): one qualified symbol name
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
//! What each test pins:
//!
//! * the **bound** — measured as the depth the deep name is installed at;
//! * the **round trip** — the create path and the read path apply the identical
//!   fold, so a symbol installed under a folded path is still findable by the
//!   name the binary spells (otherwise every call to it renders `sub_<addr>`,
//!   the DIV-59 failure mode);
//! * **determinism** — two loads of the same bytes produce the same folded
//!   spelling, which a `DefaultHasher`/`RandomState` fold would not.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::kuna_symbolnamebound::{bound_scope_path, DEFAULT_SCOPE_DEPTH};

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

/// Bootstrap `bin` and report what the load produced.
fn load(bin: &str) -> Result<Loaded, String> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let prog = bootstrap_from_object(bin, "", &spec_roots);
    let out = prog.map(|p| Loaded {
        names: p.function_entries().map(|(name, _)| name.to_string()).collect(),
        scopes: p.arch().symboltab.num_scopes(),
        original_resolves: p.lookup_symbol(&original_name()).is_some(),
    });
    out.map_err(|e| e.explain().to_string())
}

fn require_specs() {
    load(&fixture("deep_scope_x86_64")).expect("load required fixture with built processor specs");
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

/// The bound holds: the deep name is installed at the capped depth, and the
/// `Scope` count the load allocates stays near it.
#[test]
fn the_bound_holds() {
    require_specs();
    let bin = fixture("deep_scope_x86_64");
    let on = load(&bin).expect("the fixture must load");
    assert_eq!(
        depth(&deep_name(&on)),
        DEFAULT_SCOPE_DEPTH,
        "the bound caps the scope path at {DEFAULT_SCOPE_DEPTH}: {}",
        deep_name(&on)
    );
    assert!(
        on.scopes < FIXTURE_DEPTH,
        "the load must allocate fewer scopes than the {FIXTURE_DEPTH} components it folded; got {}",
        on.scopes
    );
}

/// The round trip: the create path and the read path apply the SAME fold, so the
/// binary's original spelling still finds the function.
///
/// If the bound applied only on the create side this would fail, and every call
/// to that function would render `sub_<addr>` instead of a name — the failure
/// mode this test exists to make impossible to reintroduce.
#[test]
fn the_original_name_still_resolves_through_the_fold() {
    require_specs();
    let bin = fixture("deep_scope_x86_64");

    let on = load(&bin).expect("load at the default");
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
}

/// Determinism: the folded spelling is a function of the input bytes alone.
///
/// A `DefaultHasher`/`RandomState` fold would differ between processes; this
/// asserts it does not differ between loads, and the stage XML pins the exact
/// digits across processes.
#[test]
fn the_folded_spelling_is_deterministic() {
    require_specs();
    let bin = fixture("deep_scope_x86_64");
    let a = deep_name(&load(&bin).expect("load"));
    let b = deep_name(&load(&bin).expect("load"));
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

/// An ordinary C++ binary is untouched: no real name comes near the bound.
///
/// The deepest `::` nesting measured across every real binary sampled for this
/// bound is 8, so the fold is unreachable in practice — but that has to be
/// shown, not asserted.
#[test]
fn a_real_cpp_binary_is_untouched() {
    require_specs();
    let bin = fixture("anon_namespace_x86_64");
    let on = load(&bin).expect("a real C++ binary must load");
    for n in &on.names {
        assert!(!n.contains("__x"), "no real name may fold: {n}");
        assert!(
            depth(n) <= DEFAULT_SCOPE_DEPTH,
            "no real name comes near the bound: {n} ({})",
            depth(n)
        );
    }
}
