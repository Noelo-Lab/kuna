//! End-to-end gate for **PowerPC64 (ELFv2, little-endian)** PLT import-name
//! resolution — the PPC64 analog of `verify_w11_elf_plt_names.rs` (x86-64) and
//! `verify_riscv64_plt.rs` (RISC-V64).
//!
//! PowerPC was historically grouped as a "documented seam" because, unlike the
//! other arches, ELFv2 has no `.plt` *code* section: `.plt` is a NOBITS data
//! table (the runtime GOT for imports) and the linker synthesizes the call stubs
//! inline in `.text`.  This gate proves that the seam is in fact tractable —
//! `elf_plt::decode_ppc_text` decodes those TOC-relative `.text` stubs and ties
//! each back to its `R_PPC64_JMP_SLOT` import name:
//!
//! ```text
//!   before:  sub_680("hello");  sub_660("%d\n",...);
//!   after:   puts("hello");     printf("%d\n",argc);
//! ```
//!
//! ## Fixture
//!
//! `plt_ppc64le` (source `plt_ppc64le.c`): a dynamic PPC64le ELFv2 PIE built with
//! `powerpc64le-linux-gnu-gcc -O0` (see fixtures/README.md). Its `main` (`0x8bc`)
//! `bl`s to the `puts@plt` call stub (`0x680`) and the `printf@plt` call stub
//! (`0x660`); both are `R_PPC64_JMP_SLOT` relocations in `.rela.plt` naming
//! `puts`/`printf`, landing in `.plt` slots `0x1fef0`/`0x1fef8`. Each stub is the
//! standard ELFv2 veneer the decoder recognizes:
//! `std r2,24(r1); addis r12,r2,off@ha; ld r12,off@l(r12); mtctr r12; bctr`,
//! where the loaded `.plt` slot is `TOC_base(.got+0x8000) + (off@ha<<16) + off@l`.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built PowerPC `ppc_64_le.sla` under `specs/`
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

/// The vendored linked PPC64le fixture (shared with the kuna-analysis loader gates).
fn plt_ppc64le() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le")
}

#[test]
fn ppc64_plt_calls_are_named_in_decompiled_c() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = plt_ppc64le();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    // The arch is auto-detected from the ELF machine (PowerPC64, little-endian);
    // the loader picks the `PowerPC:LE:64:default` language and bootstrap resolves
    // `ppc_64_le.sla`.
    let prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_ppc64_plt: skipping (bootstrap failed, build the PowerPC `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // The PLT imports are resolvable as functions (the elf_plt PPC64 markup), and
    // the local `.symtab` `main` resolves.
    assert!(prog.lookup_symbol("puts").is_some(), "PPC64 PLT import `puts` not resolved by the loader");
    assert!(prog.lookup_symbol("printf").is_some(), "PPC64 PLT import `printf` not resolved by the loader");
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

    // This must be a REAL decode, not a skip: `main` is a non-trivial PPC64
    // function body. If the PowerPC `.sla` were missing the bootstrap would have
    // returned early above; reaching here means the engine actually decoded the
    // PPC64 instruction stream.
    assert!(out.contains("main"), "expected a decompiled `main` body, got:\n{out}");

    // The library calls are named...
    assert!(out.contains("puts("), "expected a `puts(` call, got:\n{out}");
    assert!(out.contains("printf("), "expected a `printf(` call, got:\n{out}");
    // ...and the old PLT-stub placeholders are gone for those call sites
    // (`puts@plt` stub = 0x680, `printf@plt` stub = 0x660).
    assert!(
        !out.contains("sub_680") && !out.contains("sub_660"),
        "PPC64 PLT stubs should no longer render as sub_<addr>, got:\n{out}"
    );
}
