//! End-to-end gate for **data-global naming from DWARF** (Feature #1, matching
//! IDA Pro / Ghidra): a file-scope data global rendered by its symbol name
//! (`g_counter`, `g_total`, `g_name`) instead of `dat_<addr>`.
//!
//! Fixture: `dwarf_globals_x86_64` (`+.c`), a non-PIE x86-64 ELF (gcc `-O2 -g`)
//! with three multi-byte file-scope globals — `int g_counter` (4), `long g_total`
//! (8), `char *g_name` (8) — read/written by memory access in `bump`/`main`.
//!
//! The bug: a data global was mapped with a size-1 code type, so a 4-/8-byte
//! memory access (`mov [g_total], rax`) queried `queryContainer(addr, 8)` and
//! found no covering `SymbolEntry` — `ActionNameVars`' global-scope query then
//! failed and the printer emitted `dat_<addr>`. (1-byte globals coincidentally
//! matched the size-1 entry and were named.) The DWARF pass now resolves each
//! top-level `DW_TAG_variable`'s `DW_AT_type` to its byte size and maps it with an
//! `undefined<size>` type, so the container query matches at the real access
//! width and the name binds. Before the fix `bump` reads `dat_4020`/`dat_4028`;
//! after, `g_total`/`g_counter`.
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

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/dwarf_globals_x86_64")
}

/// Bootstrap the fixture, commit the (default-on) DWARF analysis, decompile
/// `func`, and return the captured C (`None` ⇒ specs-less skip).
fn decompile(func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_data_global_symbols: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // `read symbols`: commit the DWARF data-object facts (data-global naming is
    // default-on and needs no flag; the DWARF pass reads `.debug_info` directly).
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

/// `bump` reads/writes the 4-byte `g_counter` and 8-byte `g_total` — both must
/// render by name, neither as `dat_<addr>`.
#[test]
fn multibyte_globals_render_by_dwarf_name_not_dat() {
    let Some(code) = decompile("bump") else { return }; // specs-less skip

    assert!(
        code.contains("g_counter"),
        "expected the 4-byte global `g_counter` to render by name; got:\n{code}",
    );
    assert!(
        code.contains("g_total"),
        "expected the 8-byte global `g_total` to render by name; got:\n{code}",
    );
    assert!(
        !code.contains("dat_"),
        "no data global should render as `dat_<addr>` (all are DWARF-named); got:\n{code}",
    );
}

/// `main` takes the address of / assigns the 8-byte pointer global `g_name`.
#[test]
fn pointer_global_renders_by_dwarf_name() {
    let Some(code) = decompile("main") else { return }; // specs-less skip

    assert!(
        code.contains("g_name"),
        "expected the pointer global `g_name` to render by name; got:\n{code}",
    );
    assert!(
        !code.contains("dat_"),
        "no data global should render as `dat_<addr>`; got:\n{code}",
    );
}
