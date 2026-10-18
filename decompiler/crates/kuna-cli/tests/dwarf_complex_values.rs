//! Complex components retain their storage and argument/return registers.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = r#"#include <complex.h>
float complex pass_pair(float complex z) { return z; }
double complex pass_wide(double complex z) { return z; }
float pair_real(float complex z) { return crealf(z); }
float pair_imag(float complex z) { return cimagf(z); }
float complex make_pair(float x, float y) { return __builtin_complex(x,y); }
void store_pair(float complex *p, float x, float y) { *p = __builtin_complex(x,y); }
float complex exp_pair(float complex z) { return cexpf(z); }
double complex exp_wide(double complex z) { return cexp(z); }
void touch_pair(float complex *z) { *z=__builtin_complex(cimagf(*z),crealf(*z)); }
float local_pair(float x,float y) { float complex z=__builtin_complex(x,y); touch_pair(&z); return crealf(z)+cimagf(z); }
"#;
const DRIVER: &str = r#"
extern kuna_complex_float exp_pair(kuna_complex_float);
extern kuna_complex_double exp_wide(kuna_complex_double);
int main(void) {
    uint32_t fbits[] = {0,0x80000000u,0x3f000000u,0xbf800000u,0x7f800000u,0x7fc01234u,1};
    uint64_t dbits[] = {0,0x8000000000000000ull,0x3fe0000000000000ull,0xbff0000000000000ull,0x7ff0000000000000ull,0x7ff8000000001234ull,1};
    for (unsigned i=0;i<7;i++) for(unsigned j=0;j<7;j++) {
        kuna_complex_float input, result;
        memcpy(&input.real,&fbits[i],4); memcpy(&input.imag,&fbits[j],4);
        if (i < 4 && j < 4) {
            if (recovered_local_pair(input.real,input.imag) != input.real+input.imag) return 7;
            kuna_complex_float actual = recovered_exp_pair(input), expected = exp_pair(input);
            if (memcmp(&actual, &expected, sizeof actual)) return 6;
        }
        result=recovered_pass_pair(input);
        if(memcmp(&input,&result,8)) return 1;
        result=recovered_make_pair(input.real,input.imag);
        if(memcmp(&input,&result,8)) return 2;
        memset(&result,0xcc,8); recovered_store_pair(&result,input.real,input.imag);
        if(memcmp(&input,&result,8)) return 3;
        float r=recovered_pair_real(input), im=recovered_pair_imag(input);
        if(memcmp(&r,&input.real,4)||memcmp(&im,&input.imag,4)) return 4;
        kuna_complex_double wide, wresult;
        memcpy(&wide.real,&dbits[i],8); memcpy(&wide.imag,&dbits[j],8);
        if (i < 4 && j < 4) {
            kuna_complex_double actual=recovered_exp_wide(wide), expected=exp_wide(wide);
            if (memcmp(&actual,&expected,sizeof actual)) return 8;
        }
        wresult=recovered_pass_wide(wide);
        if(memcmp(&wide,&wresult,16)) return 5;
    }
    return 0;
}
"#;

#[test]
fn complex_values_and_imported_exponential_round_trip() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("complex-values", "c");
    let binary = source.with_extension("so");
    std::fs::write(&source, SOURCE).unwrap();
    let output = Command::new("gcc")
        .args(["-O0", "-g", "-fPIC", "-shared"])
        .arg(&source)
        .arg("-lm")
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut code = String::from("#include <string.h>\n#include <stdint.h>\ntypedef struct { float real,imag; } kuna_complex_float;\ntypedef struct { double real,imag; } kuna_complex_double;\nextern kuna_complex_float cexpf(kuna_complex_float);\nextern kuna_complex_double cexp(kuna_complex_double);\nextern void touch_pair(kuna_complex_float *);\n");
    for name in [
        "pass_pair",
        "pass_wide",
        "make_pair",
        "store_pair",
        "pair_real",
        "pair_imag",
        "exp_pair",
        "exp_wide",
        "local_pair",
    ] {
        let (body, err, status) = common::run_kuna(&["decompile", binary.to_str().unwrap(), name]);
        assert_eq!(status, 0, "{err}");
        code.push_str(&body.replace(&format!("{name}("), &format!("recovered_{name}(")));
    }
    code.push_str(DRIVER);
    let regenerated = source.with_extension("roundtrip.c");
    let executable = source.with_extension("exe");
    std::fs::write(&regenerated, &code).unwrap();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let output = Command::new(compiler)
                .args(["-std=c11", "-Wall", "-Werror", "-fno-builtin", level])
                .arg(&regenerated)
                .arg(&binary)
                .arg("-lm")
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
                "{compiler} {level}\n{code}"
            );
        }
    }
    for path in [source, binary, regenerated, executable] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn pe_complex_values_keep_their_complete_types() {
    for compiler in ["i686-w64-mingw32-gcc", "x86_64-w64-mingw32-gcc"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        let source = common::scratch_file("complex-values-pe", "c");
        let binary = source.with_extension("dll");
        std::fs::write(&source, SOURCE).unwrap();
        let output = Command::new(compiler)
            .args(["-O0", "-g", "-shared"])
            .arg(&source)
            .arg("-lm")
            .arg("-o")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        for (name, expected) in [
            ("pass_pair", "kuna_complex_float"),
            ("pass_wide", "kuna_complex_double"),
        ] {
            if compiler == "i686-w64-mingw32-gcc" && name == "pass_wide" { continue; }
            let (body, err, status) =
                common::run_kuna(&["decompile", binary.to_str().unwrap(), name]);
            assert_eq!(status, 0, "{err}");
            assert!(
                body.lines().next().unwrap().contains(expected),
                "{compiler}: {body}"
            );
            assert!(!body.contains("unaff_"), "{compiler}: {body}");
        }
        let (body, err, status) =
            common::run_kuna(&["decompile", binary.to_str().unwrap(), "local_pair"]);
        assert_eq!(status, 0, "{err}");
        assert!(body.contains("kuna_complex_float z;"), "{compiler}: {body}");
        assert!(!body.contains("((float *)&z)[1]"), "{compiler}: {body}");
        std::fs::remove_file(source).unwrap();
        std::fs::remove_file(binary).unwrap();
    }
}
