//! Bounds traps survive optimization without acquiring ordinary call effects.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn repeated_bounds_checks_keep_signed_comparisons_and_live_registers() {
    let assembler = "clang";
    if process::optional_output(Command::new(assembler).arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("bound-checks", "s");
    let binary = source.with_extension("dll");
    std::fs::write(
        &source,
        r#"
.text
.globl _sample
.def _sample; .scl 2; .type 32; .endef
_sample:
 push %ebx
 push %esi
 mov 12(%esp),%eax
 mov 16(%esp),%ecx
 mov 20(%esp),%ebx
 test %ebx,%ebx
 jz 2f
1:
 mov %ecx,%esi
.rept 4
 bound %eax,(%esi)
 add $8,%esi
 inc %eax
.endr
 dec %ebx
 jne 1b
2:
 pop %esi
 pop %ebx
 ret
"#,
    )
    .unwrap();
    let built = Command::new(assembler)
        .args([
            "--target=i686-w64-windows-gnu",
            "-shared",
            "-nostdlib",
            "-Wl,--entry,0",
            "-Wl,--export-all-symbols",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let (code, err, status) = common::run_kuna(&[
        "decompile",
        binary.to_str().unwrap(),
        "_sample",
        "--assert",
        "prototype _sample void _sample(int start,int *bounds,int rounds)",
    ]);
    assert_eq!(status, 0, "{err}");
    assert_eq!(code.matches("x86_bound_check32(").count(), 4, "{code}");
    let regenerated = source.with_extension("c");
    let executable = source.with_extension("exe");
    let driver = r#"
int main(void) {
    int bounds[8];
    const int starts[] = {INT_MIN, -30, -1, 0, 1, 30, INT_MAX-16};
    const int lower[] = {INT_MIN, -3, 0, 5, -3, INT_MAX};
    const int upper[] = {INT_MAX, 3, 0, -5, 7, INT_MIN};
    for (unsigned s=0; s<sizeof(starts)/sizeof(*starts); ++s)
        for (int rounds=0; rounds<=3; ++rounds)
            for (int mode=0; mode<6; ++mode) {
                for (int i=0; i<4; ++i) {
                    bounds[2*i]=lower[(mode+i)%6];
                    bounds[2*i+1]=upper[(mode+i)%6];
                }
                unsigned expected=0;
                for (int r=0; r<rounds; ++r)
                    for (int i=0; i<4; ++i) {
                        int x=starts[s]+r*4+i;
                        expected += x<bounds[2*i] || x>bounds[2*i+1];
                    }
                faults=0;
                _sample(starts[s], bounds, rounds);
                if (faults!=expected) return 1;
            }
    return 0;
}
"#;
    std::fs::write(&regenerated, format!("#include <limits.h>\nstatic unsigned faults;\nstatic void x86_bound_check32(int index, int lower, int upper) {{ if (index < lower || index > upper) ++faults; }}\n{code}\n{driver}")).unwrap();
    let mut ran = false;
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        ran = true;
        for opt in ["-O0", "-O2"] {
            let built = Command::new(cc)
                .args(["-std=c11", "-Wall", "-Werror", opt])
                .arg(&regenerated)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{}\n{code}",
                String::from_utf8_lossy(&built.stderr)
            );
            assert!(
                Command::new(&executable).status().unwrap().success(),
                "{cc} {opt}\n{code}"
            );
        }
    }
    assert!(
        ran,
        "GCC or Clang is required for the regenerated-code check"
    );
    if process::optional_output(Command::new("wine").arg("--version")).is_some()
        && windows_sysroot_available(assembler)
    {
        let original = source.with_extension("original.c");
        let original_exe = source.with_extension("original.exe");
        let prelude = r#"
#include <limits.h>
#include <windows.h>
static unsigned faults;
typedef void (__cdecl *Sample)(int,int*,int);
static Sample original;
static void _sample(int start,int *bounds,int rounds) { original(start,bounds,rounds); }
static LONG CALLBACK exception_handler(EXCEPTION_POINTERS *e) {
    if (e->ExceptionRecord->ExceptionCode != EXCEPTION_ARRAY_BOUNDS_EXCEEDED)
        return EXCEPTION_CONTINUE_SEARCH;
    if (*(unsigned char *)e->ContextRecord->Eip != 0x62)
        return EXCEPTION_CONTINUE_SEARCH;
    ++faults;
    e->ContextRecord->Eip += 2;
    return EXCEPTION_CONTINUE_EXECUTION;
}
"#;
        let entry = r#"int main(int argc,char **argv) {
    if (argc != 2) return 2;
    HMODULE mod=LoadLibraryA(argv[1]);
    if (!mod) return 3;
    original=(Sample)GetProcAddress(mod,"sample");
    if (!original) original=(Sample)GetProcAddress(mod,"_sample");
    if (!original) return 4;
    if (!AddVectoredExceptionHandler(1,exception_handler)) return 5;
"#;
        std::fs::write(
            &original,
            format!("{prelude}{}", driver.replace("int main(void) {", entry)),
        )
        .unwrap();
        let built = Command::new(assembler)
            .args([
                "--target=i686-w64-windows-gnu",
                "-std=c11",
                "-O2",
                "-Wall",
                "-Werror",
            ])
            .arg(&original)
            .arg("-o")
            .arg(&original_exe)
            .output()
            .unwrap();
        assert!(
            built.status.success(),
            "{}",
            String::from_utf8_lossy(&built.stderr)
        );
        let dll = format!(
            "Z:{}",
            binary
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .replace('/', "\\")
        );
        let actual = Command::new("wine")
            .env("WINEDEBUG", "-all")
            .arg(&original_exe)
            .arg(dll)
            .output()
            .unwrap();
        assert!(
            actual.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&actual.stdout),
            String::from_utf8_lossy(&actual.stderr)
        );
        std::fs::remove_file(original).unwrap();
        std::fs::remove_file(original_exe).unwrap();
    }
    for path in [source, binary, regenerated, executable] {
        std::fs::remove_file(path).unwrap();
    }
}

/// The native Windows replay needs a MinGW sysroot behind the i686 Windows target, not just wine.
fn windows_sysroot_available(compiler: &str) -> bool {
    let probe = common::scratch_file("bound-checks-sysroot", "c");
    std::fs::write(&probe, "#include <limits.h>\n#include <windows.h>\nint main(void) { return 0; }\n").unwrap();
    let status = Command::new(compiler)
        .args(["--target=i686-w64-windows-gnu", "-fsyntax-only"])
        .arg(&probe)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    std::fs::remove_file(probe).unwrap();
    status
}
