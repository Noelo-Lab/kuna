//! Runtime stack reservations under platform ABIs that differ from the ELF
//! matrix in `stack_allocations_isa`.
//!
//! Apple arm64 passes variadic arguments on the stack, never in `x1`-`x7`.
//! `stackalloc_darwin_variadic.c` built with clang 22 (`-target
//! arm64-apple-macos11 -ffreestanding -fno-stack-protector`, `-O0` and `-O2`,
//! linked with ld64.lld): `sample_vla_variadic` calls
//! `sink(n, a, n, 1, ..., 8)` with a VLA live, so the ten variadic words are
//! stored through the lowered allocation, where input recovery cannot see them.

use std::process::Command;

#[test]
fn darwin_variadic_tail_after_a_vla_keeps_its_stack_arguments() {
    for (fixture, sample) in [
        ("stackalloc_darwin_variadic_O0", "0x100000340"),
        ("stackalloc_darwin_variadic_O2", "0x10000032c"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", crate::common::repo_root().join("specs"))
            .arg("decompile").arg(crate::common::fixture(fixture)).arg(sample)
            .args(["--option", "stackalloc", "on",
                "--assert", "prototype _sink int _sink(int n,...)",
                "--assert", "prototype _sample_vla_variadic int _sample_vla_variadic(int n)"])
            .output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let code = String::from_utf8(output.stdout).unwrap();
        assert!(code.contains("__builtin_alloca("), "{fixture}: the VLA was not recovered\n{code}");
        let call = code.lines().find(|line| line.contains("_sink(")).unwrap_or_default();
        assert_eq!(call.matches(',').count(), 10, "{fixture}: sink must receive n and ten variadic words\n{code}");
    }
}

/// Windows probes every reservation through `__chkstk`: on x64 it only touches
/// the pages and the caller lowers `rsp` itself (then reserves the 32-byte home
/// area for the next call); on x86 it lowers `esp` itself. `stackalloc_win_chkstk.c`
/// built with clang 22 (`--target={x86_64,i686}-pc-windows-msvc -O0
/// -ffreestanding -fno-builtin`), linked with lld-link against a minimal
/// `__chkstk` (`stackalloc_win32_chkstk.s`; a bare `ret` on x64). Under wine
/// the recovered C and the native code returned the same values for n in
/// 1..4000 on both targets.
#[test]
fn windows_chkstk_reservations_lower_to_c_allocations() {
    for (fixture, sample, observe, chkstk, cc) in [
        ("stackalloc_win64_chkstk.exe", "0x140001020", "0x140001000", "0x140001084", ""),
        ("stackalloc_win32_chkstk.exe", "0x401020", "0x401000", "0x401088", "__cdecl "),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", crate::common::repo_root().join("specs"))
            .arg("decompile").arg(crate::common::fixture(fixture)).arg(sample)
            .args(["--option", "stackalloc", "on",
                "--define-function", &format!("{observe}=observe"),
                "--define-function", &format!("{sample}=sample_one"),
                "--define-function", &format!("{chkstk}=__chkstk"),
                "--assert", &format!("prototype observe unsigned int {cc}observe(unsigned char *p,unsigned int n)"),
                "--assert", &format!("prototype sample_one unsigned int {cc}sample_one(unsigned int n)")])
            .output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let code = String::from_utf8(output.stdout).unwrap();
        assert!(code.contains("__builtin_alloca(") && !code.contains("__chkstk"),
            "{fixture}: the probed reservation was not recovered\n{code}");
        let call = code.lines().find(|line| line.contains("observe(")).unwrap_or_default();
        assert_eq!(call.matches(',').count(), 1, "{fixture}: observe lost an argument\n{code}");
    }
}
