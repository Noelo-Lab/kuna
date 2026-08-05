//! End-to-end gate for **C++ (namespaced) symbol names at call sites and in the
//! `load addr` function header** — the scope-shadowing defect the discovered-entry
//! commit arm used to introduce (DIV-59).
//!
//! The seam: `read_loader_symbols` installs a demangled C++ funcsym in its
//! **namespace scope** (`std::terminate` lands in scope `std`, base `terminate`).
//! The analysis commit's discovered-entry arm then re-reports the same VMA and
//! named it with the generic `sub_<addr>`; because that synthetic name carries no
//! `::`, its idempotence probe only ever looked in the GLOBAL scope, found
//! nothing, and installed a **duplicate** generic FunctionSymbol beside the real
//! one. `Database::find_function_across_scopes` — the resolver `FlowInfo::
//! queryCall` uses — searches global first, so the duplicate shadowed the real
//! name and every call site rendered `sub_<addr>`.
//!
//! Only the surfaces that enable a discovery pass were affected, which is why the
//! defect was invisible to the existing console gates: the shipped console default
//! runs no discovery pass, but `kuna decompile`/`decompile-all` do (`--mode auto`
//! picks `aggressive` under 500 KiB, which turns `fast_funcdisc` and
//! `funcstart_patterns` on).
//!
//! The second, independent defect gated here: `load addr <vma>` jumped straight to
//! the generic `Architecture::name_function`, so `kuna decompile <bin> --addr
//! <vma>` printed a `sub_<addr>` header even on an UNSTRIPPED binary whose
//! by-name path (`load function`) printed the real name.
//!
//! The fixture is the vendored `cpp_noreturn_x86_64` (`g++ -O0 -no-pie -fno-pic`),
//! already used by `verify_noreturn_demangle`: `fail()` tail-calls
//! `std::terminate()`, whose PLT stub is at 0x401070.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_funcstart_patterns`/`verify_noreturn_demangle` gates,
//! the bootstrap needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cpp_noreturn_x86_64")
}

/// `std::terminate`'s PLT-stub VMA (pinned by `verify_noreturn_demangle`).
const TERMINATE_STUB: u64 = 0x401070;

/// Bootstrap the fixture with the discovery passes the `aggressive` preset turns
/// on, applied BEFORE the deferred analysis commit (the live-CLI `option` <
/// `read symbols` ordering). `None` ⇒ specs-less skip.
fn bootstrap_aggressive() -> Option<ConsoleProgram> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_cppcallnames: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    for opt in ["listing", "fast_funcdisc", "funcstart_patterns"] {
        prog.arch_mut().set_kuna_option(opt, "on").expect("the discovery gate flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// Run a console script against an already-bootstrapped program and return the
/// captured console output.
fn run_console(prog: ConsoleProgram, cmds: &[&str]) -> String {
    let cmds: Vec<String> = cmds.iter().map(|s| s.to_string()).collect();
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

/// THE PAYOFF: with the discovery passes on, the call to the demangled
/// `std::terminate` still renders its qualified name — the duplicate generic
/// FunctionSymbol must not be installed over it.
#[test]
fn namespaced_callee_renders_qualified_with_discovery_on() {
    let Some(prog) = bootstrap_aggressive() else { return };
    let out = run_console(prog, &["load function fail", "decompile", "print C"]);

    assert!(
        out.contains("std::terminate"),
        "the demangled callee must render its qualified name, got:\n{out}"
    );
    assert!(
        !out.contains(&format!("sub_{TERMINATE_STUB:x}")),
        "no generic placeholder may shadow the known C++ name, got:\n{out}"
    );
}

/// The `load addr` header defect: an UNSTRIPPED entry addressed by VMA must
/// report the FunctionSymbol installed there (qualified across scopes), not the
/// generic `sub_<addr>` — the by-name path already did.
#[test]
fn load_addr_header_uses_the_known_symbol_name() {
    let Some(prog) = bootstrap_aggressive() else { return };
    let out = run_console(
        prog,
        &[&format!("load addr 0x{TERMINATE_STUB:x}"), "decompile", "print C"],
    );

    assert!(
        out.contains("std::terminate("),
        "`load addr` must name the function from the symbol table, got:\n{out}"
    );
    assert!(
        !out.contains(&format!("sub_{TERMINATE_STUB:x}(")),
        "`load addr` must not fall back to the generic name on a known entry, got:\n{out}"
    );
}

/// The control: suppressing the placeholder must NOT suppress discovery itself.
/// A genuinely unnamed VMA the discovery passes find still gets its `sub_<addr>`
/// name, and an explicit `load addr <vma> <name>` still wins.
#[test]
fn discovery_still_names_genuinely_unknown_entries() {
    let Some(prog) = bootstrap_aggressive() else { return };
    let generic: Vec<String> = prog
        .function_entries_canonical()
        .into_iter()
        .flat_map(|e| std::iter::once(e.name).chain(e.aliases))
        .filter(|n| n.starts_with("sub_"))
        .collect();
    assert!(
        !generic.is_empty(),
        "the discovery passes must still name their finds `sub_<addr>`"
    );

    let out = run_console(
        prog,
        &[&format!("load addr 0x{TERMINATE_STUB:x} my_hand_name"), "decompile", "print C"],
    );
    assert!(
        out.contains("my_hand_name("),
        "an explicit `load addr <vma> <name>` still wins, got:\n{out}"
    );
}
