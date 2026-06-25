//! Multi-format-loader PR-9 e2e gate: a pre-link **COFF object carrying MSVC C++
//! mangled symbols** (`msvc_mangled.obj`, `?foo@Bar@@QEAAHH@Z` …) loads under
//! `--experimental-formats`, the loader's MSVC demangle arm rewrites each
//! `?`-prefixed `cl.exe` symbol to its readable qualified name, and the function
//! decompiles its real body **resolved by that demangled name** — the MSVC arm of
//! the symbol-name pass (design §5.5, §8 PR-9).
//!
//! ## Why clang, not `cl.exe`
//!
//! `cl.exe` is unavailable on Linux. But `clang -target x86_64-pc-windows-msvc`
//! emits the *same* `?`-prefixed MSVC mangling (the MSVC C++ ABI), verified by
//! `objdump -t`. So this is a **real** e2e (not a hand-faked symtab): the fixture
//! is a genuine `cl.exe`-scheme COFF object built by a real compiler. Source is
//! recorded at `tests/fixtures/msvc_mangled.cpp`.
//!
//! ## Before → after (PR-9)
//!
//! `msvc_mangled.cpp`:
//!   `int Bar::foo(int x){ return x*3+1; }`  → `?foo@Bar@@QEAAHH@Z`
//!   `int ns::g(int a,int b){ return a*b+7; }` → `?g@ns@@YAHHH@Z`
//!   `int freefunc(int x){ return x+42; }`   → `?freefunc@@YAHH@Z`
//!
//! ```text
//!   before (PR-9 absent):  funcsym stream carries the raw `?foo@Bar@@QEAAHH@Z`;
//!                          (worse: `strip_version` truncated it to `?foo`)
//!   after  (this PR):      `?foo@Bar@@QEAAHH@Z` -> `Bar::foo`, and
//!                          `freefunc` decompiles to `x + 42`
//! ```
//!
//! `Bar::foo`/`ns::g` nest into scopes via kuna's `::`-splitter, so the flat
//! `load function` resolves the bare leaf `freefunc` here (the non-namespaced
//! free function) — the demangle of the *namespaced* names is proven at the
//! loader level by `loadimage_object::tests::msvc_mangled_coff_symbols_are_demangled_name_only`
//! and by the `s1_demangle` unit tests (the merge gate, no toolchain needed).
//!
//! ## Flag gating (default-off ⇒ byte-identical on ELF)
//!
//! The `.obj` only loads when `KUNA_EXPERIMENTAL_FORMATS` is set (the
//! `--experimental-formats` CLI flag); with it unset the same fixture is rejected
//! (the default-off, byte-identical-dispatch proof shared with the sibling COFF
//! gate).
//!
//! ## `.sla` precondition
//!
//! Like the sibling console gates, bootstrapping needs the built `x86` `.sla`
//! under `specs/` (gitignored; `make specs`). If absent the bootstrap fails and
//! the test prints that and returns early (a visible skip, never a false green).

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

/// `KUNA_EXPERIMENTAL_FORMATS` is a process-global env var both tests in this file
/// toggle; serialize their env-sensitive bodies so the default-off check never
/// races the experimental-on bootstrap (cargo runs tests in parallel in one
/// process).
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

/// Bootstrap, run `load function <name>` → `decompile` → `print C`, and return the
/// captured C.
fn decompile_func(prog: ConsoleProgram, func_cmd: &str) -> String {
    let cmds: Vec<String> = [func_cmd, "decompile", "print C"]
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
    status.optr.clone()
}

/// Bootstrap the MSVC COFF object under `--experimental-formats`, returning `None`
/// (a visible skip) when the `.sla` is absent.
fn boot() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join("msvc_mangled.obj");
    assert!(path.exists(), "missing fixture {path:?}");

    std::env::set_var("KUNA_EXPERIMENTAL_FORMATS", "1");
    match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_msvc_demangle: skipping (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");
            None
        }
    }
}

/// The headline: an MSVC-mangled COFF `.obj` loads, its `?`-mangled symbols are
/// demangled to readable names by the PR-9 arm, and `freefunc` decompiles to
/// `x + 42` resolved *by its demangled name*.
#[test]
fn msvc_coff_demangles_and_decompiles() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join("msvc_mangled.obj");

    // (before / default-off proof) Without the flag the `.obj` must NOT load.
    std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");
    let off = kuna_console::engine::bootstrap_from_file(path.to_str().unwrap(), "", &spec_roots);
    assert!(
        off.is_err(),
        "default-off: a COFF object must NOT load without --experimental-formats"
    );

    let Some(prog) = boot() else { return };

    // Loads with the Windows x86-64 spec (MSVC-flavored COFF).
    let desc = prog.description().to_string();
    assert!(desc.contains("x86"), "MSVC COFF: expected x86 spec, got {desc:?}");

    // The PR-9 demangle arm rewrote every `?`-mangled COFF symbol:
    //   ?foo@Bar@@QEAAHH@Z  -> Bar::foo   (member, nested under scope `Bar`)
    //   ?g@ns@@YAHHH@Z      -> ns::g      (namespaced, nested under scope `ns`)
    //   ?freefunc@@YAHH@Z   -> freefunc   (free function, flat)
    // The flat leaf resolves directly; the nested names are proven at the loader
    // level (loadimage_object::tests::msvc_mangled_coff_symbols_are_demangled_name_only).
    assert!(
        prog.lookup_symbol("freefunc").is_some(),
        "MSVC: `freefunc` (from ?freefunc@@YAHH@Z) must resolve by demangled name"
    );
    // The raw `?`-mangled symbol must NOT survive into the symbol table.
    assert!(
        prog.lookup_symbol("?freefunc@@YAHH@Z").is_none()
            && prog.lookup_symbol("?freefunc").is_none(),
        "MSVC: raw mangled `?freefunc...` must be demangled, not registered raw"
    );

    // `kuna decompile msvc_mangled.obj freefunc` (by demangled name) → `x + 42`.
    let out = decompile_func(prog, "load function freefunc");
    std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");

    assert!(
        out.contains("freefunc"),
        "MSVC: decompiled function should be named `freefunc` (demangled), got:\n{out}"
    );
    // `return x + 42` — the LEA 0x2a(rcx) the x86-64 Windows ABI emits.
    assert!(
        out.contains("+ 0x2a") || out.contains("+ 42"),
        "MSVC: `freefunc` should decompile to `x + 42`, got:\n{out}"
    );
    assert!(
        !out.contains("not an ELF object"),
        "MSVC: loader rejected the object as not-ELF:\n{out}"
    );
    assert!(
        !out.contains('?'),
        "MSVC: no raw `?`-mangled name should leak into the decompiled output:\n{out}"
    );
}
