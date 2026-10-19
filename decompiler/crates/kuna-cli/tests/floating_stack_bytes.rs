//! Replayed floating loads retain their storage representation in byte writes.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use crate::common;
use common::process;
use std::process::Command;

const CHECKS: &str = r#"
#include <stdint.h>
#include <string.h>
unsigned int read_float_bits(float *);
unsigned long read_double_bits(double *);
unsigned short read_float_low(float *);
unsigned short read_float_high(float *);
int main(void) {
    uint32_t state = 0x3fa00000;
    const uint32_t special[] = {0,0x80000000,0x3fa00000,0xbf400000,0x7f800000,0x7fc01234};
    for (unsigned i=0;i<4096;++i) {
        state ^= state << 13; state ^= state >> 17; state ^= state << 5;
        uint32_t bits = i<6 ? special[i] : state;
        if ((bits & 0x7f800000)==0x7f800000 && (bits & 0x7fffff)) bits |= 0x400000;
        float input; memcpy(&input,&bits,4);
        if (read_float_bits(&input)!=bits) return 1;
        if (read_float_low(&input)!=(uint16_t)bits) return 2;
        if (read_float_high(&input)!=(uint16_t)(bits>>16)) return 3;
        uint64_t wide=((uint64_t)bits<<32)|state;
        if ((wide & UINT64_C(0x7ff0000000000000))==UINT64_C(0x7ff0000000000000)) wide |= UINT64_C(0x8000000000000);
        double value; memcpy(&value,&wide,8);
        if (read_double_bits(&value)!=wide) return 4;
    }
    return 0;
}
"#;

#[test]
fn floating_loads_survive_full_and_partial_stack_byte_writes() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "a native C compiler is required");
    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/floating_stack_bytes.S");
    let image = common::scratch_file("floating-stack-bytes", "elf");
    process::required_output(
        Command::new(compilers[0])
            .args(["-nostdlib", "-no-pie", "-Wl,-e,read_float_bits"])
            .arg(&fixture)
            .arg("-o")
            .arg(&image),
    );
    let assertions = [
        "prototype read_float_bits unsigned int read_float_bits(float *input)",
        "prototype read_double_bits unsigned long read_double_bits(double *input)",
        "prototype read_float_low unsigned short read_float_low(float *input)",
        "prototype read_float_high unsigned short read_float_high(float *input)",
        "prototype observe_float void observe_float(float *value)",
        "prototype observe_double void observe_double(double *value)",
    ];
    let mut regenerated = String::from("typedef unsigned char undefined1;\nvoid observe_float(float *value) {(void)value;}\nvoid observe_double(double *value) {(void)value;}\n");
    for name in [
        "read_float_bits",
        "read_double_bits",
        "read_float_low",
        "read_float_high",
    ] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command
            .arg("decompile")
            .arg(&image)
            .arg(name)
            .args([
                "--assert-strict",
                "--option",
                "stackviews",
                "on",
                "--option",
                "structdefs",
                "on",
                "--option",
                "nanignore",
                "none",
                "--sleighpath",
            ])
            .arg(common::repo_root().join("specs"));
        for assertion in assertions {
            command.args(["--assert", assertion]);
        }
        let output = process::required_output(&mut command);
        regenerated.push_str(std::str::from_utf8(&output.stdout).unwrap());
        regenerated.push('\n');
    }
    let original = common::scratch_file("floating-stack-original", "c");
    let source = common::scratch_file("floating-stack-regenerated", "c");
    std::fs::write(&original, CHECKS).unwrap();
    std::fs::write(&source, format!("{regenerated}\n{CHECKS}")).unwrap();
    for cc in compilers {
        for opt in ["-O0", "-O2"] {
            for native in [true, false] {
                let executable = common::scratch_file("floating-stack-roundtrip", "exe");
                let mut command = Command::new(cc);
                command
                    .args(["-std=gnu11", "-Werror=incompatible-pointer-types", opt])
                    .arg(if native { &original } else { &source });
                if native {
                    command.arg(&fixture);
                }
                process::required_output(command.arg("-o").arg(&executable));
                let output = process::required_output(&mut Command::new(&executable));
                assert!(
                    output.status.success(),
                    "{cc} {opt} native={native}\n{regenerated}"
                );
                std::fs::remove_file(executable).unwrap();
            }
        }
    }
    for path in [image, original, source] {
        std::fs::remove_file(path).unwrap();
    }
}
