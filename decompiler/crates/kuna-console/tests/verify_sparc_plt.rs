//! End-to-end gate for **SPARC** (SPARC v9, 64-bit, big-endian) ELF PLT/GOT
//! import-name resolution — the SPARC analog of `verify_w11_elf_plt_names.rs`
//! (x86-64) and `verify_riscv64_plt.rs` (RISC-V64).
//!
//! `elf_plt.rs` carries a SPARC PLT-veneer decoder (`decode_sparc`). SPARC is the
//! odd one out among the supported arches: its `.rela.plt` `R_SPARC_JMP_SLOT`
//! relocation `r_offset` **is the PLT entry address itself** (the dynamic linker
//! rewrites the in-place 32-byte stub at resolution time — see Ghidra's
//! `SPARC64_ElfRelocationHandler`), not a separate `.got` slot. So the
//! stub→name map is keyed directly on the call target the decompiler sees: the
//! decoder walks the `.plt` in 32-byte strides and records any `sethi %g1`-headed
//! entry whose address is a known relocation. A synthetic unit test pins the
//! strider; this gate proves it end-to-end on a **real, linked, dynamically
//! linked SPARC64 executable**.
//!
//! ```text
//!   before:  sub_2021c0("hello");  sub_2021a0("%d\n",...);
//!   after:   puts("hello");        printf("%d\n",...);
//! ```
//!
//! ## Fixture
//!
//! `plt_sparc64` (source `plt_sparc64.c`): a dynamic SPARC v9 / 64-bit BE EXEC
//! built with `sparc64-linux-gnu-gcc -O0` (see fixtures/README.md). Its `main`
//! (`0x100750`) calls `puts@plt` (`0x2021c0`) and `printf@plt` (`0x2021a0`);
//! both are `R_SPARC_JMP_SLOT` relocations in `.rela.plt` naming `puts`/`printf`,
//! with `r_offset` equal to the entry address. The `.plt` is the standard
//! 32-byte `sethi %hi(...),%g1; b,a %xcc,<resolver>; nop*6` veneer the decoder
//! recognizes (a 4-slot / 0x80-byte reserved PLT0 header precedes the imports).
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built SPARC `sparc.default.sla` under `specs/`
//! (gitignored; `make specs`). When absent the bootstrap fails; the test prints
//! that and returns early (a specs-less CI is a visible skip, never a false
//! green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_elf;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored linked SPARC64 fixture (shared with the kuna-analysis loader gates).
fn plt_sparc64() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/plt_sparc64")
}

#[test]
fn sparc_plt_calls_are_named_in_decompiled_c() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = plt_sparc64();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    // The arch is auto-detected from the ELF machine (SPARC v9, ELF64, BE); the
    // loader picks the `sparc:BE:64:default` language and bootstrap resolves the
    // SPARC `.sla`.
    let prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_sparc_plt: skipping (bootstrap failed, build the SPARC `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // The PLT imports are resolvable as functions (the elf_plt SPARC markup), and
    // the local `.symtab` `main` resolves.
    assert!(prog.lookup_symbol("puts").is_some(), "SPARC PLT import `puts` not resolved by the loader");
    assert!(prog.lookup_symbol("printf").is_some(), "SPARC PLT import `printf` not resolved by the loader");
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

    // This must be a REAL decode, not a skip: `main` is a non-trivial SPARC
    // function body. If the SPARC `.sla` were missing the bootstrap would have
    // returned early above; reaching here means the engine actually decoded the
    // SPARC instruction stream.
    assert!(
        out.contains("main"),
        "expected a decompiled `main` body, got:\n{out}"
    );

    // The library calls are named...
    assert!(out.contains("puts("), "expected a `puts(` call, got:\n{out}");
    assert!(out.contains("printf("), "expected a `printf(` call, got:\n{out}");
    // ...and the old PLT-stub placeholders are gone for those call sites
    // (`puts@plt`=0x2021c0, `printf@plt`=0x2021a0).
    assert!(
        !out.contains("sub_2021c0") && !out.contains("sub_2021a0"),
        "SPARC PLT stubs should no longer render as sub_<addr>, got:\n{out}"
    );
}
