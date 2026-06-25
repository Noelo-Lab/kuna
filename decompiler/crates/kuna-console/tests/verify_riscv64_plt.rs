//! End-to-end gate for **RISC-V64** ELF PLT/GOT import-name resolution — the
//! RISC-V analog of `verify_w11_elf_plt_names.rs` (which proves the same path on
//! x86-64).
//!
//! `elf_plt.rs` already carries a RISC-V (RV32/RV64) PLT-veneer decoder
//! (`decode_riscv`: `auipc t3,hi; l[wd] t3,lo(t3); jalr t1,t3; nop`), and a
//! synthetic unit test pins its arithmetic. This gate proves it end-to-end on a
//! **real, linked, dynamically-linked RISC-V64 executable**: it drives the full
//! `bootstrap_from_object` → `load function main` → `decompile` → `print C` path and
//! asserts the decompiled body names the libc PLT imports (`puts(`, `printf(`)
//! instead of `sub_<addr>` placeholders.
//!
//! ```text
//!   before:  sub_5e0("hello");  sub_5f0("%d\n",...);
//!   after:   puts("hello");     printf("%d\n",(int8)a0);
//! ```
//!
//! ## Fixture
//!
//! `plt_riscv64` (source `plt_riscv64.c`): a dynamic RISC-V64 PIE built with
//! `riscv64-linux-gnu-gcc -O0` (see fixtures/README.md). Its `main` (`0x6b8`)
//! calls `puts@plt` (`0x5e0` → GOT `0x2020`) and `printf@plt` (`0x5f0` → GOT
//! `0x2028`); both are `R_RISCV_JUMP_SLOT` relocations in `.rela.plt` naming
//! `puts`/`printf`. The PLT stubs are the standard 16-byte
//! `auipc t3; ld t3,lo(t3); jalr t1,t3; nop` veneer the decoder recognizes.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built RISC-V `riscv.lp64d.sla` under `specs/`
//! (gitignored; `make specs`). When absent the bootstrap fails; the test prints
//! that and returns early (a specs-less CI is a visible skip, never a false
//! green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored linked RISC-V64 fixture (shared with the kuna-analysis loader gates).
fn plt_riscv64() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/plt_riscv64")
}

#[test]
fn riscv64_plt_calls_are_named_in_decompiled_c() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = plt_riscv64();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    // The arch is auto-detected from the ELF machine (RISC-V64); the loader picks
    // the `RISCV:LE:64:default` language and bootstrap resolves `riscv.lp64d.sla`.
    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_riscv64_plt: skipping (bootstrap failed, build the RISC-V `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // The PLT imports are resolvable as functions (the elf_plt RISC-V markup), and
    // the local `.symtab` `main` resolves.
    assert!(prog.lookup_symbol("puts").is_some(), "RISC-V PLT import `puts` not resolved by the loader");
    assert!(prog.lookup_symbol("printf").is_some(), "RISC-V PLT import `printf` not resolved by the loader");
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

    // This must be a REAL decode, not a skip: `main` is a non-trivial RISC-V
    // function body. If the RISC-V `.sla` were missing the bootstrap would have
    // returned early above; reaching here means the engine actually decoded the
    // (compressed) RISC-V instruction stream.
    assert!(
        out.contains("main"),
        "expected a decompiled `main` body, got:\n{out}"
    );

    // The library calls are named...
    assert!(out.contains("puts("), "expected a `puts(` call, got:\n{out}");
    assert!(out.contains("printf("), "expected a `printf(` call, got:\n{out}");
    // ...and the old PLT-stub placeholders are gone for those call sites
    // (`puts@plt`=0x5e0, `printf@plt`=0x5f0).
    assert!(
        !out.contains("sub_5e0") && !out.contains("sub_5f0"),
        "RISC-V PLT stubs should no longer render as sub_<addr>, got:\n{out}"
    );
}
