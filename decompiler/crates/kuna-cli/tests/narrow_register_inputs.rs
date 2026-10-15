//! Mixed-width reads preserve distinct ABI arguments and destination fields.
use crate::common;
use std::process::Command;

#[test]
fn narrow_sparc_argument_reads_preserve_all_stored_values() {
    let bytes = "da2a2011d82a2012d83a0000d832201081c3e00801000000";
    let image: Vec<u8> = (0..bytes.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&bytes[i..i + 2], 16).unwrap())
        .collect();
    let path = common::scratch_file("narrow-register-inputs", "bin");
    std::fs::write(&path, image).unwrap();
    let assertions = common::scratch_file("narrow-register-inputs", "assertions");
    let record = "typedef struct Record { unsigned long long wide; unsigned long long untouched; unsigned short half; unsigned char byte; } Record;";
    std::fs::write(
        &assertions,
        format!("{record}\nprototype store_parts void store_parts(Record *dst,unsigned int b,unsigned int c,unsigned int d,unsigned int e,unsigned int f)\n"),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile",
            path.to_str().unwrap(),
            "0x1000",
            "--raw-image",
            "--base",
            "0x1000",
            "--target",
            "sparc:BE:32:default",
            "--define-function",
            "0x1000=store_parts",
            "--assert",
        ])
        .arg(format!("@{}", assertions.display()))
        .args(["--assert-strict", "--json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(json["functions"][0]["error"].is_null(), "{json}");
    let code = json["functions"][0]["code"].as_str().unwrap();
    assert!(code.contains("CONCAT44(e,f)"), "{code}");
    assert!(
        !code.contains("in_register") && !code.contains("unaff_"),
        "{code}"
    );
    let source = format!(
        r#"
#include <stdint.h>
{record}
#define CONCAT44(high,low) (((unsigned long long)(unsigned)(high) << 32) | (unsigned)(low))
{code}
int main(void) {{
    const unsigned int edges[] = {{0,1,0xff,0x100,0xffff,0x10000,0x80000000,0xffffffff}};
    uint32_t random = 0x31415926;
    for (unsigned i=0; i<4000; ++i) {{
        random = random * 1664525u + 1013904223u;
        unsigned e = i < 8 ? edges[i] : random;
        random = random * 1664525u + 1013904223u;
        unsigned f = i < 8 ? edges[7-i] : random;
        Record value = {{0xdeadbeef,0x1122334455667788ull,0x1357,0x42}};
        store_parts(&value,123,456,789,e,f);
        if (value.wide != (((unsigned long long)e << 32) | f)
            || value.half != (unsigned short)e || value.byte != (unsigned char)e
            || value.untouched != 0x1122334455667788ull) return 1;
    }}
    return 0;
}}
"#
    );
    let c_path = common::scratch_file("narrow-register-inputs", "c");
    std::fs::write(&c_path, source).unwrap();
    for compiler in ["gcc", "clang"] {
        if Command::new(compiler).arg("--version").output().is_err() {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            let executable = common::scratch_file("narrow-register-inputs", "exe");
            let build = Command::new(compiler)
                .args([
                    "-std=c11",
                    "-Wall",
                    "-Wextra",
                    "-Werror",
                    "-Wno-unused-parameter",
                    optimization,
                ])
                .arg(&c_path)
                .args(["-o", executable.to_str().unwrap()])
                .output()
                .unwrap();
            assert!(
                build.status.success(),
                "{compiler} {optimization}: {}",
                String::from_utf8_lossy(&build.stderr)
            );
            let run = Command::new(&executable).output().unwrap();
            assert!(run.status.success(), "{compiler} {optimization}: {run:?}");
            std::fs::remove_file(executable).unwrap();
        }
    }
    for file in [path, assertions, c_path] {
        std::fs::remove_file(file).unwrap();
    }
}
