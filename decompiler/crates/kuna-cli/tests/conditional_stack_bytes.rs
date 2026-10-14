//! Conditional stores preserve byte extents in asserted frame layouts.
use crate::common;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

const ASSERTIONS: &[&str] = &[
    "typedef struct Pair { unsigned int low; unsigned int high; };",
    "prototype conditional_frame_store unsigned long long conditional_frame_store(int condition)",
    "prototype boolean_frame_store unsigned long long boolean_frame_store(int first,int second)",
    "prototype observe_pair void observe_pair(const Pair *source)",
    "prototype read_word unsigned long long read_word(const unsigned long long *source)",
];
const PRELUDE: &str = r#"
#include <stdint.h>
#include <string.h>
typedef uint8_t uint1, undefined1;
typedef uint16_t uint2; typedef uint32_t uint4; typedef uint64_t uint8;
typedef int32_t int4;
typedef struct Pair Pair;
void observe_pair(const Pair *source);
uint8 read_word(const uint8 *source);
"#;
const CHECKS: &str = r#"
int main(void) {
    const uint8 upper = UINT64_C(0x1122334455667700);
    for (int first = -1; first <= 1; ++first) {
        if (conditional_frame_store(first) != (upper | (first != 0))) return 1;
        for (int second = -1; second <= 1; ++second)
            if (boolean_frame_store(first, second) != (upper | (first != 0 && second != 0))) return 2;
    }
    return 0;
}
"#;

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn conditional_byte_stores_retain_locked_layouts_and_other_bytes() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|compiler| {
            common::process::optional_output(Command::new(compiler).arg("--version")).is_some()
        })
        .collect();
    if compilers.is_empty() {
        eprintln!("no C compiler available; skipping conditional byte-store execution");
        return;
    }
    let fixture =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/conditional_stack_bytes.S");
    let image = common::scratch_file("conditional-stack-bytes", "elf");
    common::process::required_output(
        Command::new(compilers[0])
            .args(["-nostdlib", "-no-pie", "-Wl,-e,conditional_frame_store"])
            .arg(&fixture)
            .arg("-o")
            .arg(&image),
    );
    let mut definitions = Vec::new();
    for function in ["conditional_frame_store", "boolean_frame_store"] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command
            .arg("decompile")
            .arg(&image)
            .arg(function)
            .args([
                "--mode",
                "reliable",
                "--assert-strict",
                "--option",
                "stackviews",
                "on",
                "--option",
                "stackalias",
                "off",
                "--option",
                "structdefs",
                "on",
                "--sleighpath",
            ])
            .arg(common::repo_root().join("specs"));
        for assertion in ASSERTIONS {
            command.args(["--assert", assertion]);
        }
        command.args([
            "--assert",
            &format!("type {function}::v1 struct Pair protected_pair"),
        ]);
        let output = common::process::output_with_timeout(
            &mut command,
            Duration::from_secs(30),
            Duration::from_millis(25),
        )
        .expect("bounded conditional byte-store decompilation");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let code = String::from_utf8(output.stdout).unwrap();
        assert!(code.contains("Pair protected_pair;"), "{code}");
        assert!(!code.contains("._0_1_"), "{code}");
        assert!(code.contains("__builtin_memcpy"), "{code}");
        let start = code
            .find(&format!("uint8 {function}("))
            .expect("emitted function signature");
        definitions.push(code[start..].to_owned());
    }
    let helpers = r#"
struct Pair { uint4 low; uint4 high; };
void observe_pair(const Pair *source) { (void)source; }
uint8 read_word(const uint8 *source) {
    uint8 value; memcpy(&value, source, sizeof value); return value;
}
"#;
    let source = common::scratch_file("conditional-stack-bytes-emitted", "c");
    std::fs::write(
        &source,
        format!("{PRELUDE}\n{helpers}\n{}\n{CHECKS}", definitions.join("\n")),
    )
    .unwrap();
    let native_source = common::scratch_file("conditional-stack-bytes-native", "c");
    std::fs::write(&native_source, format!(
        "{PRELUDE}\nuint8 conditional_frame_store(int4);\nuint8 boolean_frame_store(int4,int4);\n{CHECKS}"
    )).unwrap();
    let native = common::scratch_file("conditional-stack-bytes-native", "bin");
    common::process::required_output(
        Command::new(compilers[0])
            .arg(&native_source)
            .arg(&fixture)
            .arg("-o")
            .arg(&native),
    );
    common::process::required_output(&mut Command::new(native));
    for compiler in compilers {
        for optimization in ["-O0", "-O2"] {
            let binary = common::scratch_file("conditional-stack-bytes-roundtrip", "bin");
            common::process::required_output(
                Command::new(compiler)
                    .args([
                        "-std=c11",
                        optimization,
                        "-fstrict-aliasing",
                        "-Werror=int-conversion",
                        "-Werror=incompatible-pointer-types",
                    ])
                    .arg(&source)
                    .arg("-o")
                    .arg(&binary),
            );
            common::process::required_output(&mut Command::new(binary));
        }
    }
}
