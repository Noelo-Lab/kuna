#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;
use std::process::Command;

#[test]
fn zero_extended_right_shifts_keep_the_original_count_range() {
    let source = common::scratch_file("wide-right-shift", "c");
    std::fs::write(
        &source,
        r#"
#include <stdint.h>
__attribute__((noinline)) uint64_t wide_shift8(const uint8_t *p, unsigned k) {
    return (uint64_t)*p >> (k & 63);
}
__attribute__((noinline)) uint64_t wide_shift16(const uint16_t *p, unsigned k) {
    return (uint64_t)*p >> (k & 63);
}
__attribute__((noinline)) uint64_t wide_shift32(const uint32_t *p, unsigned k) {
    return (uint64_t)*p >> (k & 63);
}
__attribute__((noinline)) uint64_t bounded_shift16(const uint16_t *p, unsigned k) {
    return (uint64_t)*p >> (k & 15);
}
__attribute__((noinline)) uint64_t promoted_shift8(const uint8_t *p, unsigned k) {
    return (uint64_t)*p >> (k & 31);
}
"#,
    )
    .unwrap();
    for compiler in ["gcc", "clang"] {
        if Command::new(compiler).arg("--version").output().is_err() {
            continue;
        }
        for input_level in ["-O0", "-O2"] {
            let binary = common::scratch_file("wide-right-shift", "so");
            let compiled = Command::new(compiler)
                .args([input_level, "-fPIC", "-shared", "-fno-stack-protector"])
                .arg(&source)
                .arg("-o")
                .arg(&binary)
                .output()
                .unwrap();
            assert!(
                compiled.status.success(),
                "{}",
                String::from_utf8_lossy(&compiled.stderr)
            );
            let reference = common::scratch_file("wide-right-shift-reference", "o");
            let compiled = Command::new(compiler)
                .args([
                    input_level,
                    "-fno-stack-protector",
                    "-c",
                    "-Dwide_shift8=reference_wide_shift8",
                    "-Dwide_shift16=reference_wide_shift16",
                    "-Dwide_shift32=reference_wide_shift32",
                    "-Dbounded_shift16=reference_bounded_shift16",
                    "-Dpromoted_shift8=reference_promoted_shift8",
                ])
                .arg(&source)
                .arg("-o")
                .arg(&reference)
                .output()
                .unwrap();
            assert!(
                compiled.status.success(),
                "{}",
                String::from_utf8_lossy(&compiled.stderr)
            );
            let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .arg("decompile-all")
                .arg(&binary)
                .args([
                    "--functions",
                    "wide_shift8,wide_shift16,wide_shift32,bounded_shift16,promoted_shift8",
                    "--json",
                    "--jobs",
                    "1",
                ])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            let functions = document["functions"].as_array().unwrap();
            assert_eq!(functions.len(), 5, "{document}");
            let mut bodies = String::new();
            for function in functions {
                assert!(function["error"].is_null(), "{function}");
                bodies.push_str(function["code"].as_str().unwrap());
                bodies.push('\n');
            }
            let emitted = common::scratch_file("wide-right-shift-emitted", "c");
            std::fs::write(
                &emitted,
                format!(
                    r#"
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
uint64_t reference_wide_shift8(const uint8_t *, unsigned);
uint64_t reference_wide_shift16(const uint16_t *, unsigned);
uint64_t reference_wide_shift32(const uint32_t *, unsigned);
uint64_t reference_bounded_shift16(const uint16_t *, unsigned);
uint64_t reference_promoted_shift8(const uint8_t *, unsigned);
{bodies}
int main(void) {{
    uint32_t values[] = {{0, 1, 127, 128, 255, 256, 32767, 32768, 65535,
                         0x7fffffff, 0x80000000, 0xffffffff}};
    for (unsigned i = 0; i < sizeof(values) / sizeof(values[0]); ++i) {{
        uint8_t a = values[i]; uint16_t b = values[i]; uint32_t c = values[i];
        for (unsigned k = 0; k < 128; ++k) {{
            if ((uint64_t)wide_shift8(&a, k) != reference_wide_shift8(&a, k) ||
                (uint64_t)wide_shift16(&b, k) != reference_wide_shift16(&b, k) ||
                (uint64_t)wide_shift32(&c, k) != reference_wide_shift32(&c, k) ||
                (uint64_t)bounded_shift16(&b, k) != reference_bounded_shift16(&b, k) ||
                (uint64_t)promoted_shift8(&a, k) != reference_promoted_shift8(&a, k)) {{
                fprintf(stderr, "value=%u shift=%u\n", values[i], k);
                return 1;
            }}
        }}
    }}
    return 0;
}}
"#
                ),
            )
            .unwrap();
            for output_level in ["-O0", "-O2"] {
                let executable = common::scratch_file("wide-right-shift-emitted", "exe");
                let compiled = Command::new(compiler)
                    .args(["-std=c11", output_level])
                    .arg(&emitted)
                    .arg(&reference)
                    .arg("-o")
                    .arg(&executable)
                    .output()
                    .unwrap();
                assert!(
                    compiled.status.success(),
                    "{compiler} {input_level} -> {output_level}: {}\n{bodies}",
                    String::from_utf8_lossy(&compiled.stderr)
                );
                let ran = Command::new(&executable).output().unwrap();
                assert!(
                    ran.status.success(),
                    "{compiler} {input_level} -> {output_level}: {}\n{bodies}",
                    String::from_utf8_lossy(&ran.stderr)
                );
                std::fs::remove_file(executable).unwrap();
            }
            std::fs::remove_file(emitted).unwrap();
            std::fs::remove_file(reference).unwrap();
            std::fs::remove_file(binary).unwrap();
        }
    }
    std::fs::remove_file(source).unwrap();
}
