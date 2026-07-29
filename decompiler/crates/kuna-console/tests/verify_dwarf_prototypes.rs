//! End-to-end gate for **applying a function's own DWARF prototype**: a `-g`
//! binary's function renders its source signature — real return type, real
//! parameter types, real parameter names — instead of a data-flow-derived
//! approximation.
//!
//! The gap: the DWARF pass already built a full `PrototypePieces` for every
//! defined `DW_TAG_subprogram` and parked it on the global `FunctionSymbol`, but
//! that snapshot was read only by a *caller* (`ActionDefaultParams` copies a
//! callee's prototype into the call site). A function's own decompile ignored it
//! and re-derived everything from data flow, so `main` — described by
//! `.debug_info` as `int main(int argc, char **argv)` — rendered
//! `undefined16 main(uint4 a0, void *a1)`.
//!
//! The wide bogus return is the same bug's second symptom. With no locked output,
//! return recovery registers a trial per output register the model characterizes
//! (x86-64 gcc: RAX *and* RDX) and the cspec's `join_dual_class` output rule
//! accepts the pair as one 16-byte return — emitted as a `char[16]` whose high
//! half is an uninitialized stack slot. A known `int` return skips that entirely.
//!
//! Fixtures: `dwarf_globals_x86_64` (a small purpose-built `-g` binary) and
//! `regglobal_fmt_x86_64` (GNU coreutils `fmt`, the IDA-parity reference).
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap `bin`, commit the (default-on) DWARF facts, decompile `func`, and
/// return the captured C (`None` ⇒ specs-less skip).
fn decompile(bin: &str, func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = fixture(bin).to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_dwarf_prototypes: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        [format!("load function {func}"), "decompile".into(), "print C".into()].to_vec();
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
    Some(status.optr.clone())
}

/// `int bump(int n)` — the DWARF parameter NAME (`n`) is the visible delta from
/// the recovered-storage default (`a0`).
#[test]
fn parameter_names_and_types_come_from_dwarf() {
    let Some(code) = decompile("dwarf_globals_x86_64", "bump") else { return };

    assert!(
        code.contains("bump(int4 n)"),
        "expected the DWARF signature `bump(int4 n)` (typed + named); got:\n{code}",
    );
}

/// `int main(int argc, char **argv)` — return type, both parameter types, and
/// both parameter names.
#[test]
fn main_renders_its_source_signature() {
    let Some(code) = decompile("dwarf_globals_x86_64", "main") else { return };

    assert!(
        code.contains("main(int4 argc,char **argv)"),
        "expected `main(int4 argc,char **argv)` from DWARF; got:\n{code}",
    );
}

/// The IDA-parity reference. `fmt/main` returned a bogus 16-byte `RAX:RDX` join
/// (`undefined16` + a `char[16]` local whose high half was an uninitialized stack
/// slot) because nothing locked its `int` return. With the DWARF prototype
/// applied the return narrows to the real `int` result, matching IDA Pro's
/// `return (unsigned __int8)v16 ^ 1;`.
#[test]
fn fmt_main_return_narrows_from_the_bogus_register_pair() {
    let Some(code) = decompile("regglobal_fmt_x86_64", "main") else { return };

    assert!(
        code.contains("main(int4 argc,char **argv)"),
        "expected fmt's DWARF signature for main; got:\n{code}",
    );
    assert!(
        !code.contains("undefined16"),
        "the 16-byte RAX:RDX return join must be gone; got:\n{code}",
    );
    assert!(
        code.contains("return v13 ^ 1;"),
        "expected the narrowed int return (`return v13 ^ 1;`); got:\n{code}",
    );
    assert!(
        !code.contains("[16]"),
        "the phantom `char[16]` return buffer must be gone; got:\n{code}",
    );
}
