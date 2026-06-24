//! End-to-end gate for the **cross-arch `_start`→`main` idiom** (oracle 4,
//! Increment 23) — the AArch64 / ARM / RISC-V analog of `verify_s1_entry`'s
//! x86-64 gate.
//!
//! Each fixture is a tiny `int main(int,char**){return argc;}` built DYNAMIC
//! (real crt1 `_start` → `__libc_start_main(main,…)`), with unwind tables
//! disabled, `-fvisibility=hidden` (so `main` is NOT exported in `.dynsym`), and
//! stripped — so `main` is recoverable ONLY by the `_start`→`main` idiom, never
//! via a symbol. The PIE crt1 of all three loads `main` *indirectly* from a GOT
//! slot bearing an `R_*_RELATIVE` relocation whose target is `main`'s VMA
//! (AArch64 `adrp x0;ldr x0`, ARM/Thumb GOT-relative `ldr r0`, RISC-V
//! `auipc a0;ld a0`).
//!
//! The proof per arch: `main` is NOT in any symbol table; the discovery pass +
//! commit seam register it as `sub_<addr>` (angr-style naming), and a
//! `load function sub_<addr>` → `decompile` → `print C` WITHOUT a supplied
//! `--addr` produces a real body. The ARM case additionally exercises the
//! Thumb-mode (`TMode=1`) decode-mode paint the discovery pass derives from the
//! GOT pointer's Thumb LSB (no `$t` mapping symbol survives stripping).
//!
//! ## `.sla` precondition
//!
//! Like `verify_s1_entry`, bootstrapping needs the built per-arch `.sla` under
//! `specs/` (gitignored; `make specs`, or the three specs this gate needs:
//! `AARCH64.slaspec`, `ARM8_le.slaspec`, `riscv.lp64d.slaspec`). When absent the
//! bootstrap fails; the test prints that and returns early (a specs-less CI is a
//! visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_elf;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap `fixture`, commit analysis, and assert the discovered `main`
/// (`sub_<main_vma>`) is registered and decompiles to a real body without a
/// supplied address. Returns `false` (visible skip) only when the `.sla` is
/// absent (bootstrap fails). Any other failure is a hard assertion.
fn assert_discovered_main_decompiles(fixture_name: &str, main_vma: u64) -> bool {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = match fixture(fixture_name).to_str() {
        Some(s) => s.to_string(),
        None => return false,
    };

    let mut prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_crossarch_entry_main[{fixture_name}]: skipping (bootstrap \
                 failed, build `.sla` with `make specs`): {}",
                e.explain()
            );
            return false;
        }
    };

    // The analysis facts (entries + ARM TMode paints) are committed at the
    // `read symbols` seam, not eagerly at bootstrap.
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    // The discovered `main` is registered under its angr-style `sub_<addr>` name
    // even though it is in NO symbol table of this stripped, hidden-visibility
    // binary — recovered purely by the `_start`→`main` libc-start idiom.
    let sym = format!("sub_{main_vma:x}");
    assert!(
        prog.lookup_symbol(&sym).is_some(),
        "[{fixture_name}] discovered main ({sym}) not registered — oracle 4 failed"
    );

    let cmds: Vec<String> = [format!("load function {sym}"), "decompile".into(), "print C".into()]
        .into_iter()
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

    // A real decompilation of a discovered, never-symboled function: it names the
    // function and emits a C body (braces) — not the CLI "no function" error.
    assert!(
        out.contains(&sym),
        "[{fixture_name}] expected a decompiled body for {sym}, got:\n{out}"
    );
    assert!(
        out.contains('{') && out.contains('}'),
        "[{fixture_name}] expected a C function body for {sym}, got:\n{out}"
    );
    assert!(
        !out.contains("Unknown function") && !out.contains("no function"),
        "[{fixture_name}] {sym} should resolve to a function, got:\n{out}"
    );
    // The fixture's `int main(int c,char**v){return c;}` returns its first arg —
    // a real (non-degenerate) body returns `a0` rather than `void`/`return;`.
    // (ARM relies on the Thumb `TMode=1` paint here, else the A32 misdecode emits
    // a `void … {return;}` stub.)
    assert!(
        out.contains("return a0") || out.contains("return (int8)a0"),
        "[{fixture_name}] expected `return a0` body (the recovered main), got:\n{out}"
    );
    true
}

/// AArch64: `_start` (0x600) → `main` (0x714) via `adrp x0,0x10000; ldr x0,[x0,#4080]`
/// → GOT slot 0x10ff0 (`R_AARCH64_RELATIVE` addend 0x714).
#[test]
fn aarch64_discovered_main_decompiles() {
    assert_discovered_main_decompiles("entrymain_aarch64", 0x714);
}

/// ARM/Thumb: `_start` (0x3dd) → `main` (0x4d8) GOT-relatively (`.got`+0x28 = slot
/// 0x10ff8, `R_ARM_RELATIVE` in-place value 0x4d9 = main|Thumb-LSB). Also proves
/// the discovery-derived `TMode=1` Thumb decode-mode paint.
#[test]
fn arm_thumb_discovered_main_decompiles() {
    assert_discovered_main_decompiles("entrymain_arm", 0x4d8);
}

/// RISC-V: `_start` (0x550) → `main` (0x608) via `auipc a0,0x2; ld a0,-1318(a0)`
/// → GOT slot 0x2030 (`R_RISCV_RELATIVE` addend 0x608).
#[test]
fn riscv_discovered_main_decompiles() {
    assert_discovered_main_decompiles("entrymain_riscv64", 0x608);
}
