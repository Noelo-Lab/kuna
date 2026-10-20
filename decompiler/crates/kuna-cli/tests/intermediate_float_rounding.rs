//! Successive narrowing conversions preserve their separate rounding steps.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn successive_float_narrowing_matches_native() {
    let compilers: Vec<_> = ["clang", "gcc"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        return;
    }
    let dir = common::scratch_file("intermediate-float-rounding", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let original = dir.join("original.c");
    std::fs::write(
        &original,
        "float rounded_twice(long double value) { return (float)(double)value; }\n",
    )
    .unwrap();
    let native = dir.join("sample.so");
    process::required_output(
        Command::new(compilers[0])
            .args(["-shared", "-fPIC", "-O0"])
            .arg(&original)
            .arg("-o")
            .arg(&native),
    );
    let (printed, stderr, rc) = common::run_kuna(&[
        "decompile-all",
        native.to_str().unwrap(),
        "--functions",
        "rounded_twice",
        "--assert",
        "prototype rounded_twice float rounded_twice(long double value)",
        "--assert-strict",
    ]);
    assert_eq!(rc, 0, "{stderr}\n{printed}");
    let regenerated = dir.join("regenerated.c");
    std::fs::write(&regenerated,format!("typedef float float4;\ntypedef double float8;\ntypedef long double float10;\n#define rounded_twice printed_rounded_twice\n{printed}")).unwrap();
    let driver = dir.join("driver.c");
    std::fs::write(
        &driver,
        r#"
#include <stdio.h>
#include <float.h>
float rounded_twice(long double value);
float printed_rounded_twice(long double value);
int main(void) {
    const long double values[]={0x1.000001000000001p0L,-0x1.000001000000001p0L,
        0x1.000002fffffffffp0L,-0x1.000002fffffffffp0L,
        0x1.000001p0L,-0x1.000001p0L,0.0L,1.0L,-1.0L,1234.5L};
    if(LDBL_MANT_DIG!=64)return 0;
    for(unsigned i=0;i<sizeof values/sizeof values[0];i++) {
        float expected=rounded_twice(values[i]), actual=printed_rounded_twice(values[i]);
        if(expected!=actual) {
            fprintf(stderr,"case %u expected=%a actual=%a\n",i,(double)expected,(double)actual);
            return 1;
        }
    }
    return 0;
}
"#,
    )
    .unwrap();
    for cc in compilers {
        for opt in ["-O0", "-O2"] {
            let exe = dir.join(format!("roundtrip-{cc}-{opt}"));
            process::required_output(
                Command::new(cc)
                    .args([opt, "-fno-fast-math", "-ffp-contract=off"])
                    .arg(&original)
                    .arg(&regenerated)
                    .arg(&driver)
                    .arg("-o")
                    .arg(&exe),
            );
            process::required_output(&mut Command::new(exe));
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
