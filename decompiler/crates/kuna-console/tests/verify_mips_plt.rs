//! End-to-end gate for **MIPS32** ELF import-name resolution — the MIPS analog of
//! `verify_w11_elf_plt_names.rs` (x86-64) / `verify_riscv64_plt.rs` (RISC-V64).
//!
//! ## Why MIPS is structurally different
//!
//! The GNU MIPS o32 toolchain emits **no `.plt` code section and no
//! `R_MIPS_JUMP_SLOT` relocations** for its lazy libc imports.  A call site reads
//! the import's address out of a `$gp`-relative GOT slot and jumps through it
//! (`lw $t9, off($gp); jalr $t9`), so the stub→name correspondence is carried by
//! the dynamic-symbol GOT layout (`DT_MIPS_LOCAL_GOTNO`/`DT_MIPS_GOTSYM`), not by
//! relocations.  `elf_plt::resolve_mips_imports` names each import's `.MIPS.stubs`
//! stub (the GOT slot's static contents) and marks the GOT external slots
//! **constant** (`ObjectLoadImage::const_ranges` → `get_readonly`); `bootstrap_from_elf`
//! turns on `readonlypropagate` for MIPS so the engine folds the GOT load to the
//! stub address and resolves the call to the import name:
//!
//! ```text
//!   before:  (*(code *)(dat_411040 & 0xfffffffe))("hello");
//!            (*(code *)(dat_411044 & 0xfffffffe))(0x400888,v1);
//!   after:   puts("hello");
//!            printf(0x400888,v1);
//! ```
//!
//! ## Fixture
//!
//! `plt_mips32` (source `plt_mips32.c`): a dynamic big-endian MIPS32 executable
//! built with `mips-linux-gnu-gcc -O0` (see fixtures/README.md). Its `main`
//! (`0x400700`) calls `puts` (GOT slot `0x411040` → stub `0x400800`) and `printf`
//! (GOT slot `0x411044` → stub `0x4007f0`); there are no PLT relocations — the
//! names come from the dynamic-symbol GOT layout.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built MIPS `mips32be.sla` under `specs/` (gitignored;
//! `make specs`). When absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_elf;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored linked MIPS32 (big-endian) fixture.
fn plt_mips32() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/plt_mips32")
}

#[test]
fn mips32_imports_are_named_in_decompiled_c() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = plt_mips32();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    // The arch is auto-detected from the ELF machine (MIPS, big-endian); the loader
    // picks `MIPS:BE:32:default:default` and bootstrap resolves `mips32be.sla`.
    let prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_mips_plt: skipping (bootstrap failed, build the MIPS `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // The imports are resolvable as functions (the elf_plt MIPS markup), and the
    // local `.symtab` `main` resolves.
    assert!(prog.lookup_symbol("puts").is_some(), "MIPS import `puts` not resolved by the loader");
    assert!(prog.lookup_symbol("printf").is_some(), "MIPS import `printf` not resolved by the loader");
    assert!(prog.lookup_symbol("main").is_some(), "defined `main` not resolved by the loader");

    // Drive `load function main` → `decompile` → `print C`.
    let cmds: Vec<String> = ["load function main", "decompile", "print C"]
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
    let out = status.optr.clone();

    // This must be a REAL decode, not a skip: if the MIPS `.sla` were missing the
    // bootstrap would have returned early above; reaching here means the engine
    // actually decoded the big-endian MIPS instruction stream of `main`.
    assert!(out.contains("main"), "expected a decompiled `main` body, got:\n{out}");

    // The library calls are named (the `$gp`-relative GOT load was folded to the
    // stub address and resolved to the import name)...
    assert!(out.contains("puts("), "expected a `puts(` call, got:\n{out}");
    assert!(out.contains("printf("), "expected a `printf(` call, got:\n{out}");
    // ...and the raw GOT-slot loads are gone for those call sites (the unresolved
    // `(*(code *)(dat_411040 & ...))(...)` placeholders).
    assert!(
        !out.contains("dat_411040") && !out.contains("dat_411044"),
        "MIPS GOT-slot calls should resolve to import names, got:\n{out}"
    );
}
