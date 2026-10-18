//! Explicit x87 storage must not leave a model-generated hidden return argument.
use crate::common;
use common::process;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Link the fixture as a MinGW DLL, or assemble it as a COFF object with Clang
/// when no MinGW toolchain is installed.
fn build_image(source: &Path) -> Option<PathBuf> {
    let mingw = "i686-w64-mingw32-gcc";
    if process::optional_output(Command::new(mingw).arg("--version")).is_some() {
        let binary = source.with_extension("dll");
        process::optional_output(
            Command::new(mingw)
                .args([
                    "-shared",
                    "-nostdlib",
                    "-Wl,--entry,0",
                    "-Wl,--export-all-symbols",
                ])
                .arg(source)
                .arg("-o")
                .arg(&binary),
        )?;
        return Some(binary);
    }
    let object = source.with_extension("obj");
    process::optional_output(
        Command::new("clang")
            .args(["--target=i686-pc-windows-gnu", "-c"])
            .arg(source)
            .arg("-o")
            .arg(&object),
    )?;
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
