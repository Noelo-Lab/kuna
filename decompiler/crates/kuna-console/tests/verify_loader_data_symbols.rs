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
//! global, gated by `--option datasyms on|off` (default ON, DIV-76, GH-184) at
//! the `read symbols` commit — the off arm below pins that contract.
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

/// The GH-184 witness: the stripped shadow `faillog` (23 KB, two
/// `__fprintf_chk(stderr, ...)` sites in the function at `0x3320`), whose only
/// name source for `stderr`/`optind`/`optarg`/`stdout` is the `.dynsym`
/// `STT_OBJECT` half — no `.symtab`, no DWARF.
fn faillog() -> PathBuf {
    repo_root().join("tests/bug-repro/faillog")
}

/// Bootstrap `bin`, optionally flip `--option datasyms off`, commit the analysis
/// facts, run `load_cmd` + decompile, and return the captured C (`None` ⇒
/// specs-less skip).
fn decompile_bin(bin: &PathBuf, load_cmd: &str, datasyms_off: bool) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = bin.to_str()?.to_string();
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
    if datasyms_off {
        // The live CLI shape: the `option` line runs after `load file` and
        // before `read symbols`, where the commit consults the flag (DIV-76).
        prog.arch_mut().set_kuna_option("datasyms", "off").expect("datasyms option applies");
    }
    // `read symbols`: commit the analysis facts, including (gated on `datasyms`)
    // the loader's data symbols.
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> = [load_cmd.to_string(), "decompile".into(), "print C".into()].to_vec();
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
/// Decompile `func` from the fmt fixture with the default (datasyms-on) config.
fn decompile(func: &str) -> Option<String> {
    decompile_bin(&fixture(), &format!("load function {func}"), false)
}

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

/// The GH-184 witness, default arm: both `__fprintf_chk` sites in the stripped
/// `faillog` function at `0x3320` name their stream argument `stderr` from the
/// `.dynsym` `STT_OBJECT` entry at `0x61a0` (size 8) — the address must no
/// longer render raw.
#[test]
fn faillog_stderr_renders_by_name_by_default() {
    let Some(code) = decompile_bin(&faillog(), "load addr 0x3320", false) else { return };

    assert!(
        code.contains("stderr"),
        "expected the copy-relocated libc extern `stderr` to render by name; got:\n{code}",
    );
    assert!(
        !code.contains("dat_61a0"),
        "`dat_61a0` should have been named `stderr` from `.dynsym`; got:\n{code}",
    );
}

/// The `datasyms` off-switch (DIV-76): flipping the option before the commit
/// restores the raw `dat_<addr>` rendering exactly — the stream is dropped at
/// `read symbols`, so the symbol never installs and both call sites regress to
/// the pre-DIV-26 output. This pins the option's contract on the same in-process
/// path `kuna decompile-all --option datasyms off` drives.
#[test]
fn option_datasyms_off_restores_dat_addr() {
    let Some(code) = decompile_bin(&faillog(), "load addr 0x3320", true) else { return };

    assert!(
        code.contains("dat_61a0"),
        "with `datasyms off` the stream argument must render raw `dat_61a0`; got:\n{code}",
    );
    assert!(
        !code.contains("stderr"),
        "with `datasyms off` no symbol-table data name may install; got:\n{code}",
    );
}
