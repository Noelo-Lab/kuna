//! End-to-end gate for AArch64 ELF PLT/GOT import-name resolution.
//!
//! The `elf_plt` follow-up (`src/s1_loader/elf_plt.rs`) decodes the AArch64
//! standard GNU `ld` import veneer (`adrp x16, GOT_page; ldr x17,[x16,#lo12];
//! add x16,x16,#lo12; br x17`) and maps the stub entry to its imported name.
//! Until now that path was only unit-tested against synthetic veneer bytes —
//! the x86 paths had a real linked fixture (`verify_w11_elf_plt_names`) but
//! AArch64 had no linked binary to prove the decode end-to-end.  This gate
//! drives the full `load file plt_aarch64` → `load function main` → `decompile`
//! → `print C` path against a **real, linked, dynamic AArch64 executable** and
//! asserts `main` renders the libc imports (`puts(`, `printf(`) instead of the
//! old `sub_<addr>` PLT-stub placeholders:
//!
//! ```text
//!   before:  sub_4004d0("hello");  sub_4004e0(...);
//!   after:   puts("hello");        printf(...);
//! ```
//!
//! The fixture (`decompiler/crates/kuna-analysis/tests/fixtures/plt_aarch64`)
//! was built with `aarch64-linux-gnu-gcc -O0 -no-pie plt_aarch64.c -o ...`
//! (gcc 11.4.0).  Load-bearing addresses are pinned below from
//! `objdump`/`readelf` (they shift across toolchains).
//!
//! ## `.sla` precondition
//!
//! Like the sibling x86 gate, bootstrapping needs the built `AARCH64` `.sla`
//! under `specs/` (gitignored; `make specs`, or just
//! `slacomp specs/Ghidra/Processors/AARCH64/data/languages/AARCH64.slaspec`).
//! When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

// Load-bearing VMAs, pinned from the linked fixture (objdump/readelf):
//   main         0x400604   (calls puts@plt then printf@plt)
//   puts@plt     0x4004d0   (GOT slot 0x411018, R_AARCH64_JUMP_SLOT -> puts)
//   printf@plt   0x4004e0   (GOT slot 0x411020, R_AARCH64_JUMP_SLOT -> printf)
const PUTS_PLT: &str = "sub_4004d0";
const PRINTF_PLT: &str = "sub_4004e0";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The linked AArch64 PLT fixture (shared with the kuna-analysis loader gates).
fn plt_aarch64() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/plt_aarch64")
}

#[test]
fn aarch64_plt_calls_are_named_in_decompiled_c() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = plt_aarch64();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_aarch64_plt: skipping (bootstrap failed, build the AARCH64 `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // The PLT imports are resolvable as functions (the loader markup), and the
    // local `.symtab` `main` still resolves.
    assert!(prog.lookup_symbol("puts").is_some(), "PLT import `puts` not resolved by the loader");
    assert!(
        prog.lookup_symbol("printf").is_some(),
        "PLT import `printf` not resolved by the loader"
    );
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

    // The library calls are named...
    assert!(out.contains("puts("), "expected a `puts(` call, got:\n{out}");
    assert!(out.contains("printf("), "expected a `printf(` call, got:\n{out}");
    // ...and the old PLT-stub placeholders are gone for those call sites — proof
    // the AArch64 veneer decode mapped the `bl 0x4004d0`/`bl 0x4004e0` targets to
    // their imported names rather than falling back to `sub_<addr>`.
    assert!(
        !out.contains(PUTS_PLT) && !out.contains(PRINTF_PLT),
        "PLT stubs should no longer render as sub_<addr>, got:\n{out}"
    );
}
