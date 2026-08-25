//! End-to-end gate for the symbol-name → scope seam: a degenerate qualified
//! symbol name must not abort the load of an entire binary.
//!
//! Two independent defects meet at `Database::find_create_scope_from_symbol_name`,
//! which splits a loader symbol name on every `::` and nests one Scope per
//! component. An **empty** component reaches `attach_scope`, which rejects it with
//! `Non-global scope has empty name` — and because the symbol table is installed
//! inside `load file`, that error escapes `bootstrap_from_object` instead of
//! costing one symbol. Every command then answers `could not build an
//! architecture` and emits nothing at all.
//!
//! * **The root cause** (`kuna-analysis` `analyzers/demangle`): the name-only
//!   demangling of a C++ **anonymous namespace** used to delete the whole
//!   `(anonymous namespace)` group, leaving exactly that empty component. An
//!   anonymous namespace is the ordinary way C++ gives a definition internal
//!   linkage, so this refused a large share of real unstripped C++ binaries.
//!   Fixture: `anon_namespace_x86_64`. No option gates it — it is a strict fix.
//! * **The backstop** (`--option symbolnamerepair`, default on): a name can be
//!   degenerate without any demangler involved, and symbol-name bytes are
//!   attacker-controlled data that no header check validates. Fixture:
//!   `hostile_scope_x86_64`, whose `.symtab` carries a function literally named
//!   `a::::b`.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_noreturn_demangle` gate, bootstrapping needs the built
//! `x86` `.sla` under `specs/` (gitignored; `make specs`). When it is absent the
//! bootstrap fails; the test prints that and returns early (a specs-less CI is a
//! visible skip, never a false green).

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::kuna_symbolnamerepair::SYMBOLNAMEREPAIR_ENV;

/// Serializes the env-var flip + bootstrap across the concurrently-run tests
/// (the same guard `verify_typedepth.rs` uses): the gate is a process-global env
/// var, and every object load reads it.
static GATE: Mutex<()> = Mutex::new(());

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

/// Bootstrap `bin` with the backstop forced to `repair`, and return every
/// function name the symbol table installed, or the `LowlevelError` text the
/// load failed with.
fn load_function_names(bin: &str, repair: bool) -> Result<Vec<String>, String> {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    std::env::set_var(SYMBOLNAMEREPAIR_ENV, if repair { "on" } else { "off" });
    let prog = bootstrap_from_object(bin, "", &spec_roots);
    std::env::remove_var(SYMBOLNAMEREPAIR_ENV);
    let prog = prog.map_err(|e| e.explain().to_string())?;
    Ok(prog.function_entries().map(|(name, _)| name.to_string()).collect())
}

/// Whether the `.sla` is present, so a bootstrap failure means what it says.
/// Returns `false` (and prints) when it is not — a visible skip.
///
/// The match is on the spec-resolution text only, so the `Non-global scope has
/// empty name` failure this file exists to catch can never be mistaken for a
/// missing-specs skip.
fn specs_available() -> bool {
    match load_function_names(&fixture("anon_namespace_x86_64"), true) {
        Err(e) if e.contains("sleigh specification") || e.contains(".sla") => {
            eprintln!(
                "verify_symbolnamerepair: skipping (no `.sla`, build with `make specs`): {e}"
            );
            false
        }
        _ => true,
    }
}

/// The root-cause gate: an ordinary unstripped C++ binary whose definitions live
/// in anonymous namespaces loads, and each one is nested under the
/// `anonymous_namespace` identifier rather than collapsing to an empty component.
///
/// This holds with the backstop OFF — the demangler fix stands on its own, and
/// this asserts the two are genuinely independent rather than one masking the
/// other.
#[test]
fn anonymous_namespace_binary_loads_without_the_backstop() {
    if !specs_available() {
        return;
    }
    // Backstop OFF: the demangler fix alone must carry this, with nothing to
    // fall back on. If the two were entangled, this arm would fail the load.
    let names = match load_function_names(&fixture("anon_namespace_x86_64"), false) {
        Ok(n) => n,
        Err(e) => panic!("an anonymous-namespace C++ binary must load with the backstop OFF: {e}"),
    };
    for expected in [
        "anonymous_namespace::top_helper",
        "outer::anonymous_namespace::nested_helper",
        "outer::anonymous_namespace::Widget::emit",
    ] {
        assert!(
            names.iter().any(|n| n == expected),
            "missing {expected:?}; got {:?}",
            names.iter().filter(|n| n.contains("anonymous")).collect::<Vec<_>>()
        );
    }
    // The pre-fix shape must be gone entirely: no name may carry an empty
    // `::`-component, which is what aborted the load.
    for n in &names {
        assert!(!n.split("::").any(|c| c.is_empty()), "empty scope component survived: {n}");
    }
}

/// The backstop gate, both arms.
///
/// `a::::b` is not mangled, so the demangler never sees it — this is the failure
/// the root-cause fix does NOT cover, and the reason the gate exists.
#[test]
fn hostile_symbol_name_aborts_the_load_only_with_the_gate_off() {
    if !specs_available() {
        return;
    }
    let bin = fixture("hostile_scope_x86_64");

    // On (also the shipped default): the degenerate component is skipped and the
    // binary loads, with both functions present. The listing keeps the raw
    // `a::::b` the binary claims -- the repair places the SCOPE, it does not
    // rewrite what the symbol table said.
    let names = load_function_names(&bin, true).expect("repair on must load a hostile symbol name");
    assert!(names.iter().any(|n| n == "main"), "got {names:?}");
    assert!(names.iter().any(|n| n == "a::::b"), "got {names:?}");

    // Off: the historical hard error, and it takes the whole architecture with
    // it -- one symbol, zero output.
    let err = load_function_names(&bin, false).expect_err("gate off must restore the hard error");
    assert!(err.contains("Non-global scope has empty name"), "got {err:?}");
}
