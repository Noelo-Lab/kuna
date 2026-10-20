//! Explicit x87 storage must not leave a model-generated hidden return argument.
use crate::common;
use common::process;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Assemble Windows fixtures with Clang so no Windows linker is required.
fn build_image(source: &Path) -> Option<PathBuf> {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return None;
    }
    let object = source.with_extension("obj");
    let output = Command::new("clang")
        .args(["--target=i686-pc-windows-gnu", "-c"])
        .arg(source)
        .arg("-o")
        .arg(&object)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Some(object)
}

#[test]
fn st0_assertions_keep_one_argument_and_the_callers_stack_offsets() {
    let source = common::scratch_file("x87-custom-storage", "s");
    std::fs::write(
        &source,
        r#"
.text
.globl _sample
.def _sample; .scl 2; .type 32; .endef
_sample:
 fldl 4(%esp)
 call _root_value
 mov 12(%esp),%eax
 fstpl (%eax)
 ret
.globl _root_value
.def _root_value; .scl 2; .type 32; .endef
_root_value:
 fsqrt
 ret
"#,
    )
    .unwrap();
    let Some(binary) = build_image(&source) else {
        std::fs::remove_file(&source).unwrap();
        return;
    };
    let parameter = "param _root_value::0 ST0 float10 value";
    let result = "return _root_value::ST0 float10";
    let regenerated = source.with_extension("c");
    let executable = source.with_extension("exe");
    for assertions in [[parameter, result], [result, parameter]] {
        let (code, err, status) = common::run_kuna(&[
            "decompile",
            binary.to_str().unwrap(),
            "_sample",
            "--assert",
            "prototype _sample void _sample(double x,double *out)",
            "--assert",
            assertions[0],
            "--assert",
            assertions[1],
        ]);
        assert_eq!(status, 0, "{err}");
        assert!(
            code.contains("*out = (double)_root_value((long double)x);"),
            "{code}"
        );
        assert!(!code.contains("CONCAT"), "{code}");
        std::fs::write(
            &regenerated,
            format!(
                r#"
#include <math.h>
long double _root_value(long double value) {{ return sqrtl(value); }}
{code}
int main(void) {{
    double values[]={{0.0, -0.0, 0.125, 1.0, 2.0, 17.25, 1e-200, 1e200}};
    for (unsigned i=0; i<sizeof(values)/sizeof(values[0]); ++i) {{
        double actual, expected=(double)sqrtl((long double)values[i]);
        _sample(values[i], &actual);
        if (actual != expected || signbit(actual) != signbit(expected)) return 1;
    }}
    return 0;
}}
"#
            ),
        )
        .unwrap();
        let mut ran = false;
        for cc in ["gcc", "clang"] {
            if process::optional_output(Command::new(cc).arg("--version")).is_none() {
                continue;
            }
            ran = true;
            for level in ["-O0", "-O2"] {
                let built = Command::new(cc)
                    .args(["-std=c11", level])
                    .arg(&regenerated)
                    .args(["-lm", "-o"])
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
                    "{cc} {level}\n{code}"
                );
            }
        }
        assert!(
            ran,
            "GCC or Clang is required for the regenerated-code check"
        );
    }
    for path in [source, binary, regenerated, executable] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn partial_st0_input_keeps_the_next_argument_at_the_first_stack_slot() {
    let source = common::scratch_file("x87-partial-storage", "s");
    std::fs::write(
        &source,
        r#"
.text
.globl _scaled
.def _scaled; .scl 2; .type 32; .endef
_scaled:
 fimull 4(%esp)
 ret
.globl _dispatch
.def _dispatch; .scl 2; .type 32; .endef
_dispatch:
 fldl 4(%esp)
 push 12(%esp)
 call _scaled
 add $4,%esp
 mov 16(%esp),%eax
 fstpt (%eax)
 ret
"#,
    )
    .unwrap();
    let Some(binary) = build_image(&source) else {
        std::fs::remove_file(source).unwrap();
        return;
    };
    let parameter = "param _scaled::0 ST0 float10 x";
    let result = "return _scaled::ST0 float10";
    let regenerated = source.with_extension("c");
    let reference = source.with_extension("reference.s");
    let executable = source.with_extension("elf");
    std::fs::write(
        &reference,
        r#"
.text
.globl original_scaled
original_scaled:
 fldt 4(%esp)
 push 16(%esp)
 call original_custom
 add $4,%esp
 ret
original_custom:
 fimull 4(%esp)
 ret
.section .note.GNU-stack,"",@progbits
"#,
    )
    .unwrap();
    for assertions in [[parameter, result], [result, parameter]] {
        let (code, err, status) = common::run_kuna(&[
            "decompile",
            binary.to_str().unwrap(),
            "_scaled",
            "--assert",
            "prototype _scaled float10 _scaled(float10 x,int n)",
            "--assert",
            assertions[0],
            "--assert",
            assertions[1],
        ]);
        assert_eq!(status, 0, "{err}");
        assert!(code.contains("(long double)n"), "{code}");
        let (caller, err, status) = common::run_kuna(&[
            "decompile",
            binary.to_str().unwrap(),
            "_dispatch",
            "--assert",
            "prototype _dispatch void _dispatch(double x,int n,float10 *out)",
            "--assert",
            "prototype _scaled float10 _scaled(float10 x,int n)",
            "--assert",
            assertions[0],
            "--assert",
            assertions[1],
        ]);
        assert_eq!(status, 0, "{err}");
        assert!(caller.contains("_scaled((long double)x,n)"), "{caller}");
        std::fs::write(&regenerated, format!(r#"
{code}
{caller}
extern long double original_scaled(long double, int);
static int check(void) {{
    static const long double values[] = {{0.0L, -0.0L, 0.125L, -1.25L, 1.0000000000000000001L,
                            1e-2000L, -1e2000L}};
    static const double caller_values[] = {{0.0, -0.0, 0.125, -1.25, 1.0000000000000002, 1e-200, -1e200}};
    static const int factors[] = {{0, 1, -1, 7, -12345, 2147483647, (-2147483647-1)}};
    for (unsigned i=0; i<sizeof(values)/sizeof(values[0]); ++i)
        for (unsigned j=0; j<sizeof(factors)/sizeof(factors[0]); ++j) {{
            long double a = _scaled(values[i], factors[j]);
            long double b = original_scaled(values[i], factors[j]);
            long double c;
            _dispatch(caller_values[i], factors[j], &c);
            long double d = original_scaled(caller_values[i], factors[j]);
            for (unsigned k=0; k<10; ++k) {{
                if (((unsigned char*)&a)[k] != ((unsigned char*)&b)[k]) return 1;
                if (((unsigned char*)&c)[k] != ((unsigned char*)&d)[k]) return 2;
            }}
        }}
    return 0;
}}
__attribute__((force_align_arg_pointer)) void _start(void) {{
    int status=check();
    __asm__ volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
    __builtin_unreachable();
}}
"#)).unwrap();
        for cc in ["gcc", "clang"] {
            if process::optional_output(Command::new(cc).arg("--version")).is_none() {
                continue;
            }
            for level in ["-O0", "-O2"] {
                let output = Command::new(cc)
                    .args([
                        "-m32",
                        "-std=c11",
                        level,
                        "-nostdlib",
                        "-fno-stack-protector",
                        "-fno-pie",
                        "-no-pie",
                        "-Wl,-e,_start",
                    ])
                    .arg(&regenerated)
                    .arg(&reference)
                    .arg("-o")
                    .arg(&executable)
                    .output()
                    .unwrap();
                assert!(
                    output.status.success(),
                    "{}\n{code}",
                    String::from_utf8_lossy(&output.stderr)
                );
                assert!(
                    Command::new(&executable).status().unwrap().success(),
                    "{cc} {level}\n{code}"
                );
            }
        }
    }
    for path in [source, binary, regenerated, reference, executable] {
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
    }
}
