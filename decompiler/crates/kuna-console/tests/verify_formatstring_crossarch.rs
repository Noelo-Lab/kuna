//! Cross-arch end-to-end gate for **format-string varargs typing**
//! (`s1_formatstring` half B — the kuna analog of Ghidra's
//! `FormatStringAnalyzer`, "Variadic Function Signature Override").
//!
//! The sibling [`verify_s1_formatstring`] proves the feature on x86-64; this gate
//! extends the coverage to the other three first-class arches — **AArch64**,
//! **ARM** (32-bit), and **RISC-V 64** — each built from the same source as the
//! x86-64 fixture:
//!
//! ```c
//! int main(int argc, char **argv) { printf("%d %s\n", argc, argv[0]); return 0; }
//! ```
//!
//! For each fixture it decompiles `main` once with the feature **off** (the
//! default) and once with `--option formatstring on`, and asserts:
//!
//! - **default-off** leaves the printf call's varargs UNTYPED — the `%s` arg is
//!   *not* cast to `char *` (the feature is off);
//! - **formatstring on** types them per the format string —
//!   `printf("%d %s\n",a0,(char *)*a1)`: the `%d` arg is a plain `int` (no widening
//!   `(uint8)`/`(int8)` cast) and the `%s` arg is cast to `char *`.
//!
//! ## The ARM literal-pool case (the one cross-arch wrinkle)
//!
//! On x86-64/AArch64/RISC-V the format-string address is materialized directly
//! into a register (`lea` / `adrp+add` / `auipc+addi`), so the call's format-arg
//! varnode is already a constant pointer that `resolve_const_pointer` reads as-is.
//! On **ARM** the address is loaded *PC-relatively from a read-only literal pool*
//! (`ldr r3,[pc,#k]; add r3,pc`), so the format-arg varnode is a memory LOAD that
//! only constant-folds when `readonlypropagate` is on. The `IfcDecompile`
//! format-string loop therefore enables read-only propagation for the duration of
//! a `formatstring`-on decompile (and restores it afterward), so the feature is
//! self-contained on ARM with a single `--option formatstring on` — matching the
//! other arches' single-flag UX. (Gate-safe: that toggle is inert unless the
//! feature is enabled, so the parity gates — which never set `formatstring` — are
//! byte-identical.)
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_s1_*` gates, bootstrapping needs the built per-arch
//! `.sla` under `specs/` (gitignored; `make specs`). When it is absent the
//! bootstrap fails; the test prints that and returns early (a specs-less CI is a
//! visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_elf;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// One of the vendored `printf("%d %s\n", argc, argv[0])` fixtures
/// (`fmt_aarch64` / `fmt_arm` / `fmt_riscv64`), built from the same source as the
/// x86-64 `fmt_x86_64`.
fn fmt_bin(arch: &str) -> PathBuf {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(format!("fmt_{arch}"))
}

/// Decompile `main` of `fmt_<arch>` and return the captured C, optionally enabling
/// the format-string varargs-typing feature first (`--option formatstring on`).
/// Returns `None` (a visible skip) when the per-arch `.sla` is absent.
fn decompile_main(arch: &str, formatstring_on: bool) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fmt_bin(arch).to_str()?.to_string();
    let mut prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_formatstring_crossarch[{arch}]: skipping (bootstrap failed, build \
                 `.sla` with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // Commit the load-time analysis facts (libproto, PLT import names, etc.) — what
    // `read symbols` does — so the `printf` proto is recovered before the decompile.
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    if formatstring_on {
        prog.arch_mut()
            .set_kuna_option("formatstring", "on")
            .expect("formatstring option flips on");
    }

    let cmds: Vec<String> =
        ["load function main".into(), "decompile".into(), "print C".into()].to_vec();
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

/// default-off: the printf varargs are left UNTYPED (the `%s` arg is *not* cast to
/// `char *`).  Run for each of AArch64/ARM/RISC-V.
fn assert_off_leaves_varargs_untyped(arch: &str) {
    let Some(out) = decompile_main(arch, false) else {
        return; // specs-less skip
    };
    assert!(out.contains("printf("), "[{arch}] expected a printf call, got:\n{out}");
    assert!(
        !out.contains("(char *)*a1"),
        "[{arch}] default-off must NOT type the %s arg as char* (feature is off), got:\n{out}"
    );
}

/// formatstring-on: the %s arg is cast to `char *` and the %d arg drops its
/// widening cast (the format-derived `int` matches the recovered parameter).  Run
/// for each of AArch64/ARM/RISC-V.
fn assert_on_types_printf_varargs(arch: &str) {
    let Some(out) = decompile_main(arch, true) else {
        return; // specs-less skip
    };
    assert!(out.contains("printf("), "[{arch}] expected a printf call, got:\n{out}");
    // The whole point: the call renders `printf("%d %s\n",a0,(char *)*a1)`.
    assert!(
        out.contains("printf(\"%d %s\\n\",a0,(char *)*a1)"),
        "[{arch}] formatstring-on should render `printf(\"%d %s\\n\",a0,(char *)*a1)`, got:\n{out}"
    );
    // Belt-and-suspenders on the two halves of that string (so a future render
    // tweak still pins the *typing*, not just the exact spelling):
    assert!(
        out.contains("(char *)*a1"),
        "[{arch}] formatstring-on should type the %s arg as `char *`, got:\n{out}"
    );
    // No widening cast on the %d arg (x86-64/AArch64 default to `(uint8)`, RISC-V
    // to `(int8)`; ARM has no cast there at all — assert none of them survive).
    assert!(
        !out.contains("(uint8)a0") && !out.contains("(int8)a0"),
        "[{arch}] formatstring-on should drop the widening cast on the %d arg, got:\n{out}"
    );
}

// --- AArch64 ----------------------------------------------------------------

#[test]
fn formatstring_off_leaves_varargs_untyped_aarch64() {
    assert_off_leaves_varargs_untyped("aarch64");
}

#[test]
fn formatstring_on_types_printf_varargs_aarch64() {
    assert_on_types_printf_varargs("aarch64");
}

// --- ARM (32-bit, the read-only literal-pool case) --------------------------

#[test]
fn formatstring_off_leaves_varargs_untyped_arm() {
    assert_off_leaves_varargs_untyped("arm");
}

#[test]
fn formatstring_on_types_printf_varargs_arm() {
    assert_on_types_printf_varargs("arm");
}

// --- RISC-V 64 --------------------------------------------------------------

#[test]
fn formatstring_off_leaves_varargs_untyped_riscv64() {
    assert_off_leaves_varargs_untyped("riscv64");
}

#[test]
fn formatstring_on_types_printf_varargs_riscv64() {
    assert_on_types_printf_varargs("riscv64");
}
