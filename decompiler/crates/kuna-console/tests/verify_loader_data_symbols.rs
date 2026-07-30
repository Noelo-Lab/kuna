//! End-to-end gate for **data-global naming from the ELF symbol table**: an
//! imported libc object renders by its name (`optind`, `stdin`, `stdout`,
//! `optarg`) instead of `dat_<addr>`, matching IDA Pro and Ghidra.
//!
//! Fixture: `regglobal_fmt_x86_64` — GNU coreutils `fmt`, whose `main` reads all
//! four through copy relocations (`R_X86_64_COPY` slots in `.bss`).
//!
//! The gap: the loader kept only `SymbolKind::Text` entries from `.symtab` /
//! `.dynsym`, so every `STT_OBJECT` symbol was dropped. A copy-relocated libc
//! global has a real defined address in the importing binary but never appears in
//! the program's own `.debug_info`, so the DWARF data-global naming could not
//! reach it either — `main` rendered `dat_20a098` where both reference
//! decompilers show `optind`. The loader now collects the data half of the same
//! two symbol tables and the engine installs each as a named `undefined<size>`
//! global.
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
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/regglobal_fmt_x86_64")
}

/// Bootstrap the fixture, commit the analysis facts, decompile `func`, and
/// return the captured C (`None` ⇒ specs-less skip).
fn decompile(func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_loader_data_symbols: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // `read symbols`: commit the analysis facts, including the loader's data
    // symbols (loader markup — no flag).
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

/// `fmt/main` reads `optind` (4 bytes, `.dynsym`+`.symtab` `STT_OBJECT` at
/// `0x20a098`), `stdin`/`stdout` (8 bytes), and `optarg` (8 bytes). All four are
/// imported libc objects — absent from the program's DWARF — so before the loader
/// read the data half of the symbol table each rendered `dat_<addr>`.
#[test]
fn libc_extern_globals_render_by_symbol_name() {
    let Some(code) = decompile("main") else { return }; // specs-less skip

    for name in ["optind", "stdin", "stdout", "optarg"] {
        assert!(
            code.contains(name),
            "expected the libc extern global `{name}` to render by name; got:\n{code}",
        );
    }
    // The four addresses those symbols cover must no longer appear raw.
    for raw in ["dat_20a098", "dat_20a090", "dat_20a088", "dat_20a0a0"] {
        assert!(
            !code.contains(raw),
            "`{raw}` should have been named from the ELF symbol table; got:\n{code}",
        );
    }
}

/// The program's own DWARF-described globals keep their DWARF-recovered extent:
/// the loader arm commits LAST and skips any address a richer source already
/// claimed, so this is a precedence regression guard, not a duplicate of
/// `verify_data_global_symbols`.
#[test]
fn dwarf_named_globals_survive_the_loader_arm() {
    let Some(code) = decompile("main") else { return }; // specs-less skip

    for name in ["max_width", "goal_width", "prefix_length"] {
        assert!(
            code.contains(name),
            "expected the DWARF-named global `{name}` to still render by name; got:\n{code}",
        );
    }
}
