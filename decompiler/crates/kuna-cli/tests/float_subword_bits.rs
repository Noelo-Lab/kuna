//! Float subwords preserve bit patterns, including signed zero and NaN payloads.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn extracted_float_words_round_trip_as_bits() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() { return; }
    let source = common::scratch_file("float-subword", "c");
    let binary = source.with_extension("so");
    std::fs::write(&source, r#"
#include <stdint.h>
#include <string.h>
float low_component(uint64_t bits) { float p[2]; memcpy(p,&bits,8); return p[0]; }
float high_component(uint64_t bits) { float p[2]; memcpy(p,&bits,8); return p[1]; }
float signed_component(int64_t bits) { float p[2]; memcpy(p,&bits,8); return p[0]; }
float numeric_component(uint64_t bits) { return (float)bits; }
float loaded_pair(const unsigned char *buf) { uint64_t w; memcpy(&w,buf,8); float p[2]; memcpy(p,&w,8); return p[0] + p[1]; }
"#).unwrap();
    // Pin the sample's codegen: a host gcc that vectorizes p[0]+p[1] (AVX shuffles)
    // makes kuna emit vshufps_avx/SUB164 pseudo-ops, which cannot round-trip.
    let output = Command::new("gcc").args(["-O2", "-g", "-fPIC", "-shared", "-march=x86-64", "-fno-tree-vectorize"])
        .arg(&source).arg("-o").arg(&binary).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let mut rust_code = String::new();
    let mut code = String::from("#include <stdint.h>\n#include <string.h>\n");
    for name in ["low_component", "high_component", "signed_component", "numeric_component", "loaded_pair"] {
        let (body, err, status) = common::run_kuna(&["decompile", binary.to_str().unwrap(), name]);
        assert_eq!(status, 0, "{err}");
        let (rust, err, status) = common::run_kuna(&["decompile", binary.to_str().unwrap(), name, "--language", "rust"]);
        assert_eq!(status, 0, "{err}");
        rust_code.push_str(&rust);
        code.push_str(&body.replace(&format!("{name}("), &format!("rebuilt_{name}(")));
    }
    code.push_str(r#"
extern float low_component(uint64_t), high_component(uint64_t), numeric_component(uint64_t);
extern float signed_component(int64_t), loaded_pair(const unsigned char *);
static uint32_t word(float value) { uint32_t bits; memcpy(&bits,&value,4); return bits; }
static int same(float a, float b) { return word(a) == word(b) || (a != a && b != b); }
int main(void) {
    uint64_t inputs[] = {0, UINT64_C(0x8000000080000000), UINT64_C(0x400000003f800000),
        UINT64_C(0x7f800000ff800000), UINT64_C(0x7fc123457fc54321), UINT64_C(0x0000000100000001)};
    for (unsigned i=0;i<sizeof(inputs)/sizeof(inputs[0]);++i) {
        uint64_t bits=inputs[i];
        if (word(rebuilt_low_component(bits)) != word(low_component(bits))) return 1;
        if (word(rebuilt_high_component(bits)) != word(high_component(bits))) return 2;
        if (word(rebuilt_signed_component((int64_t)bits)) != word(signed_component((int64_t)bits))) return 4;
        if (rebuilt_numeric_component(bits) != numeric_component(bits)) return 3;
        if (!same(rebuilt_loaded_pair((unsigned char *)&bits), loaded_pair((unsigned char *)&bits))) return 5;
    }
    return 0;
}
"#);
    let regenerated = source.with_extension("roundtrip.c");
    let executable = source.with_extension("exe");
    std::fs::write(&regenerated, &code).unwrap();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() { continue; }
        for level in ["-O0", "-O2"] {
            let output = Command::new(compiler).args(["-std=c11", level]).arg(&regenerated)
                .arg(&binary).arg("-o").arg(&executable).output().unwrap();
            assert!(output.status.success(), "{}\n{code}", String::from_utf8_lossy(&output.stderr));
            assert!(Command::new(&executable).status().unwrap().success(), "{compiler} {level}\n{code}");
        }
    }
    if process::optional_output(Command::new("rustc").arg("--version")).is_some() {
        rust_code.push_str(r#"
fn main() { unsafe {
    for bits in [0u64, 0x8000000080000000, 0x400000003f800000, 0x7f800000ff800000,
                 0x7fc123457fc54321, 0x0000000100000001] {
        assert_eq!(low_component(bits).to_bits(), bits as u32);
        assert_eq!(high_component(bits).to_bits(), (bits >> 32) as u32);
        assert_eq!(signed_component(bits as i64).to_bits(), bits as u32);
        assert_eq!(numeric_component(bits), bits as f32);
        let mut word = bits;
        let pair = loaded_pair((&mut word as *mut u64).cast());
        let want = f32::from_bits(bits as u32) + f32::from_bits((bits >> 32) as u32);
        assert!(pair.to_bits() == want.to_bits() || pair.is_nan() && want.is_nan());
    }
} }
"#);
        let rust = source.with_extension("rs");
        std::fs::write(&rust, &rust_code).unwrap();
        for level in ["0", "2"] {
            let output = Command::new("rustc").args(["--crate-name", "float_subwords", "-C"])
                .arg(format!("opt-level={level}")).arg(&rust).arg("-o").arg(&executable).output().unwrap();
            assert!(output.status.success(), "{}\n{rust_code}", String::from_utf8_lossy(&output.stderr));
            assert!(Command::new(&executable).status().unwrap().success(), "{rust_code}");
        }
        std::fs::remove_file(rust).unwrap();
    }
    for path in [source, binary, regenerated, executable] { std::fs::remove_file(path).unwrap(); }
}
