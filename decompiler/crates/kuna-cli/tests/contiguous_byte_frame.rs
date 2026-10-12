//! A byte loop spans a word header and an interior payload pointer in one frame.
#![cfg(all(target_os = "linux", target_endian = "little"))]

use crate::common;
use common::process;
use std::process::Command;

fn emitted_frame(mutable: bool, options: &[&str]) -> String {
    let hex = include_str!("fixtures/contiguous_byte_frame.hex").trim();
    let bytes: Vec<_> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect();
    let image = common::scratch_file("contiguous-byte-frame", "bin");
    let assertions = common::scratch_file("contiguous-byte-frame", "kuna");
    std::fs::write(&image, bytes).unwrap();
    let declarations = include_str!("fixtures/contiguous_byte_frame.kuna");
    std::fs::write(
        &assertions,
        if mutable {
            declarations.replace("const unsigned char", "unsigned char")
        } else {
            declarations.to_owned()
        },
    )
    .unwrap();
    let directive = format!("@{}", assertions.display());
    let mut args = vec![
        "decompile-all",
        image.to_str().unwrap(),
        "--raw-image",
        "--base",
        "0x10000",
        "--addr",
        "0x10000",
        "--target",
        "ARM:LE:32:v5t:default",
        "--isa",
        "arm",
        "--mode",
        "reliable",
        "--jobs",
        "1",
        "--no-vars",
        "--json",
        "--assert-strict",
        "--assert",
        &directive,
    ];
    args.extend_from_slice(options);
    let (stdout, stderr, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{stderr}\n{stdout}");
    let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(result["functions"][0]["error"].is_null(), "{stdout}");
    result["functions"][0]["code"].as_str().unwrap().to_owned()
}

#[test]
fn emitted_frame_preserves_bytes_and_object_bounds() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "a C compiler is required");
    for mutable in [false, true] {
        let code = emitted_frame(mutable, &[]);
        let pieces = regex::Regex::new(r"(\w+)\._0_4_ = ([^;\n]+);").unwrap();
        let compilable = pieces.replace_all(&code, "store_word($1, $2);");
        let source = common::scratch_file("contiguous-byte-frame", "c");
        std::fs::write(
            &source,
            format!(
                r#"
#include <stdint.h>
#include <string.h>
typedef uint8_t uint1;
typedef uint32_t uint4;
typedef int32_t int4;
static void store_word(uint1 *p, uint4 value) {{ memcpy(p, &value, sizeof(value)); }}
void FillPayload(uint1 *p, uint4 n) {{ for (uint4 i = 0; i < n; ++i) p[i] = i + 1; }}
uint4 PayloadSum({qualifier}uint1 *p, uint4 n) {{
    uint4 sum = 0; for (uint4 i = 0; i < n; ++i) sum += p[i]; return sum;
}}
void CopyFrame(uint1 *d, {qualifier}uint1 *p, uint4 n) {{ memcpy(d, p, n); }}
{compilable}
int main(void) {{
    const uint1 expected[22] = {{153, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
                                10, 11, 12, 13, 14, 15, 16, 17, 0}};
    uint1 output[54]; memset(output, 0xa5, sizeof(output));
    BuildFrame(output + 16);
    if (memcmp(output + 16, expected, sizeof(expected))) return 1;
    for (unsigned i = 0; i < 16; ++i)
        if (output[i] != 0xa5 || output[38 + i] != 0xa5) return 2;
    return 0;
}}
"#,
                qualifier = if mutable { "" } else { "const " }
            ),
        )
        .unwrap();
        for cc in &compilers {
            for optimization in ["-O0", "-O2"] {
                let executable = common::scratch_file("contiguous-byte-frame", "exe");
                let compiled = Command::new(cc)
                    .args([
                        "-std=c11",
                        optimization,
                        "-fsanitize=address,undefined",
                        "-fno-sanitize-recover=all",
                        "-fno-pie",
                        "-no-pie",
                    ])
                    .arg(&source)
                    .arg("-o")
                    .arg(&executable)
                    .output()
                    .unwrap();
                assert!(
                    compiled.status.success(),
                    "{cc} {optimization}: {}\n{code}",
                    String::from_utf8_lossy(&compiled.stderr)
                );
                let run = Command::new(&executable).output().unwrap();
                assert!(
                    run.status.success(),
                    "{cc} {optimization} mutable={mutable}: {}\n{code}",
                    String::from_utf8_lossy(&run.stderr)
                );
            }
        }
    }
}

#[test]
fn disabling_range_proof_or_shared_store_layout_retains_the_split() {
    for option in ["loadguardrange", "stackstoreguard"] {
        let code = emitted_frame(false, &["--option", option, "off"]);
        assert!(code.contains("+ -4]"), "{option}: {code}");
        assert!(
            code.contains("CopyFrame(destination,(uint1 *)&"),
            "{option}: {code}"
        );
    }
}
