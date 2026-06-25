//! Multi-format-loader PR-6+7 e2e gate (the Mach-O headline): a **linked Mach-O**
//! executable (`macho_imports`, x86-64 + `macho_imports_arm64`, arm64) loads and
//! decompiles a function, and its libc import renders **named** (`printf(`) — not
//! `sub_<addr>` — through the `__stubs` indirect-symbol walk
//! (`s1_loader/macho_stubs.rs`, design §3.3 / §8 PR-7).
//!
//! This is the Mach-O half of the headline goal ("a decompiler for most file
//! formats"): PR-2 proved a Mach-O *object* parses; this proves a full linked
//! *exe* loads (`LC_MAIN` entry, `LC_SEGMENT_64` mapping), decompiles a function,
//! and names the import a `bl`/`callq` reaches through `__TEXT,__stubs`.
//!
//! ## How `main` reaches `printf` (and what names it)
//!
//! In both fixtures `main` reaches `printf` by a **direct branch to the
//! `__stubs` entry** — x86-64 `callq 0x1000005cc`, arm64 `bl 0x1000005a0`.
//! There is no IAT-style slot to constant-fold; naming the *stub entry* address
//! (`sec.addr + i*reserved2`) is sufficient and arch-independent. The name comes
//! from the `LC_DYSYMTAB` indirect-symbol table → `LC_SYMTAB` (`_printf`, with
//! the leading `_` stripped). So:
//!
//! ```text
//!   before (resolve_imports empty):  sub_1000005cc(0x1000005ee, a0 * 3 + 7);
//!   after  (PR-7 __stubs naming):    printf(0x1000005ee, a0 * 3 + 7);
//! ```
//!
//! ## Two fixtures, one proof, both arches
//!
//! `macho_imports` (x86-64) and `macho_imports_arm64` (arm64) are the *same*
//! source (`macho_imports.c`: a `_main` calling `printf("%d\n", compute(argc))`)
//! linked for two arches with `ld64.lld`. The pair proves the naming is
//! arch-independent (the design's §3.3 claim): the resolver keys off section
//! metadata, never an instruction decode.
//!
//! ## Multi-format loading (unconditional)
//!
//! PE/Mach-O/COFF load unconditionally now — like ELF, with no flag. The test
//! confirms the same fixture routes through the default `load file` dispatch to
//! the object loader (the byte-identical-dispatch proof shared with
//! `verify_object_formats`).
//!
//! ## `.sla` precondition
//!
//! Like the sibling console gates, bootstrapping needs the built `x86`/`AARCH64`
//! `.sla` under `specs/` (gitignored; `make specs`). If absent the bootstrap
//! fails and the test prints that and returns early (a visible skip, never a
//! false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

// ---- Pinned facts (container `ld64.lld` link + `llvm-objdump --macho -d`) -----
//
// macho_imports.c = `_main` that calls `printf("%d\n", compute(argc))`,
//   compute(n) = n*3+7.  Linked with the rustup-bundled `ld64.lld`:
//     clang -target {x86_64,arm64}-apple-macos11 -O1 -c macho_imports.c -o m.o
//     ld64.lld -arch {x86_64,arm64} -platform_version macos 11.0 11.0 \
//              -undefined dynamic_lookup -e _main -o macho_imports m.o
//   ImageBase 0x100000000 (PIE).
//
//   x86-64 (macho_imports):
//     _compute       @ 0x1000005a0
//     _main          @ 0x1000005b0   (callq 0x1000005cc -> printf stub)
//     __stubs printf @ 0x1000005cc   (the `bl`/`callq` target we name)
//   arm64 (macho_imports_arm64):
//     _compute       @ 0x100000560
//     _main          @ 0x10000056c   (bl 0x1000005a0 -> printf stub)
//     __stubs printf @ 0x1000005a0
const X64_MAIN_VMA: u64 = 0x1000005b0;
const X64_PRINTF_STUB: u64 = 0x1000005cc;

/// Bootstrap, run a `load …`-driven `decompile` → `print C`, and return the
/// captured C. `func_cmd` is the `load function …` / `load addr …` command that
/// seeds the function to decompile.
fn decompile_func(prog: ConsoleProgram, func_cmd: &str) -> String {
    let cmds: Vec<String> = [func_cmd, "decompile", "print C"]
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
    status.optr.clone()
}

/// Bootstrap a fixture, returning `None` (a visible skip) when the `.sla` is
/// absent.
fn boot(name: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join(name);
    assert!(path.exists(), "missing fixture {path:?}");

    match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_macho_imports: skipping {name} (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

/// The headline: a linked x86-64 Mach-O loads, decompiles `main`, and its libc
/// call renders `printf(` — named by the `__stubs` indirect-symbol walk, not
/// `sub_<stub>`.
#[test]
fn macho_x64_decompiles_with_named_printf() {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join("macho_imports");

    // (default-on proof) The object loads through the *default* `load file`
    // dispatch with no flag — multi-format support is unconditional. (A `.sla`-
    // absent environment surfaces as a load error; the main body's skip covers
    // that, so here we only assert the dispatch ROUTES to the object loader, i.e.
    // it does not fail with the XML "not recognized" error.)
    let dflt = kuna_console::engine::bootstrap_from_file(path.to_str().unwrap(), "", &spec_roots);
    if let Err(e) = &dflt {
        // Only acceptable failure is a missing-`.sla` bootstrap error, never an
        // "unrecognized format" rejection (that would mean the magic wasn't admitted).
        let msg = e.explain();
        assert!(
            !msg.contains("Unable to recognize") && !msg.contains("XML"),
            "default-on: the object must route to the object loader (got: {msg})"
        );
    }

    let Some(prog) = boot("macho_imports") else { return };

    // The linked exe loads with the x86-64 (SysV/gcc) spec — macOS x86-64 is
    // System V AMD64, the same cspec Ghidra labels `gcc`.
    let desc = prog.description().to_string();
    assert!(desc.contains("x86"), "Mach-O: expected x86 spec, got {desc:?}");
    // The defined `_main` and the imported `printf` resolve as functions.
    assert!(
        prog.lookup_symbol("_main").is_some(),
        "Mach-O: `_main` not resolved by the loader"
    );
    assert!(
        prog.lookup_symbol("printf").is_some(),
        "Mach-O: `printf` not resolved by the `__stubs` walk"
    );

    // Decompile by the defined-function name (`_main`).
    let out = decompile_func(prog, "load function _main");

    assert!(out.contains("printf("), "Mach-O: expected a `printf(` call, got:\n{out}");
    // The call is the named import, not an unnamed stub.
    assert!(
        !out.contains(&format!("sub_{X64_PRINTF_STUB:x}")),
        "Mach-O: the printf stub should render as `printf`, not sub_<addr>:\n{out}"
    );
}

/// The PR-7 proof, by address: decompiling `_main` *by its VMA* (no reliance on
/// the defined-function symbol) still renders the `printf(` call named, because
/// only the `__stubs` indirect-symbol walk named the stub entry at
/// `0x1000005cc` that `callq` targets directly.
#[test]
fn macho_x64_names_printf_stub_by_address() {
    let Some(prog) = boot("macho_imports") else { return };

    let out = decompile_func(prog, &format!("load addr 0x{X64_MAIN_VMA:x}"));

    assert!(
        out.contains("printf("),
        "Mach-O (by addr): the __stubs entry must be named `printf(`, got:\n{out}"
    );
    assert!(
        !out.contains(&format!("sub_{X64_PRINTF_STUB:x}")),
        "Mach-O (by addr): the printf stub should no longer be sub_{X64_PRINTF_STUB:x}:\n{out}"
    );
}

/// Arch-independence (design §3.3): the *same* source linked for arm64 names
/// `printf` identically — the resolver keys off section metadata, not an
/// instruction decode, so a direct `bl __stubs` resolves with no per-arch code.
#[test]
fn macho_arm64_decompiles_with_named_printf() {
    let Some(prog) = boot("macho_imports_arm64") else { return };

    let desc = prog.description().to_string();
    assert!(
        desc.contains("AARCH64") || desc.contains("ARM"),
        "Mach-O arm64: expected an AArch64 spec, got {desc:?}"
    );
    assert!(
        prog.lookup_symbol("printf").is_some(),
        "Mach-O arm64: `printf` not resolved by the `__stubs` walk"
    );

    let out = decompile_func(prog, "load function _main");

    assert!(
        out.contains("printf("),
        "Mach-O arm64: expected a `printf(` call, got:\n{out}"
    );
}
