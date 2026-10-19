//! A floating conversion must round before a subsequent integer truncation.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn narrowing_before_integer_truncation_matches_native() {
    let compilers: Vec<_> = ["clang", "gcc"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        return;
    }
    let dir = common::scratch_file("float-integer-truncation", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let image = dir.join("sample.bin");
    // cvtsd2ss xmm0,xmm0; cvttss2si eax,xmm0; ret
    std::fs::write(
        &image,
        [0xf2, 0x0f, 0x5a, 0xc0, 0xf3, 0x0f, 0x2c, 0xc0, 0xc3],
    )
    .unwrap();
    let (printed, stderr, rc) = common::run_kuna(&[
        "decompile",
        image.to_str().unwrap(),
        "0x1000",
        "--raw-image",
        "--base",
        "0x1000",
        "--target",
        "x86:LE:64:default",
        "--define-function",
        "0x1000=rounded_integer",
        "--assert",
        "prototype rounded_integer int rounded_integer(double value)",
        "--assert-strict",
    ]);
    assert_eq!(rc, 0, "{stderr}\n{printed}");
    let source = dir.join("roundtrip.c");
    std::fs::write(
        &source,
        format!(
            r#"
typedef float float4;
typedef double float8;
typedef int int4;
{printed}
#include <stdio.h>
__attribute__((noinline)) static int reference(double value) {{
    volatile float rounded = (float)value;
    return (int)rounded;
}}
int main(void) {{
    const double values[] = {{0.99999999, -0.99999999, 1.99999999, -1.99999999,
        16.9999999, -16.9999999, 0.0, 0.25, -0.25, 1.0, -1.0, 1234.5}};
    for (unsigned i=0;i<sizeof values/sizeof values[0];++i) {{
        int expected=reference(values[i]), actual=rounded_integer(values[i]);
        if (actual!=expected) {{
            fprintf(stderr, "value=%.17g expected=%d actual=%d\n",values[i],expected,actual);
            return 1;
        }}
    }}
    return 0;
}}
"#
        ),
    )
    .unwrap();
    for cc in compilers {
        for opt in ["-O0", "-O2"] {
            let executable = dir.join(format!("roundtrip-{cc}-{opt}"));
            process::required_output(
                Command::new(cc)
                    .args([opt, "-std=c11", "-fno-fast-math"])
                    .arg(&source)
                    .arg("-o")
                    .arg(&executable),
            );
            process::required_output(&mut Command::new(executable));
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
