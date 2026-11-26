//! The low word of a machine product remains defined for unsigned inputs above C's INT_MAX product bound.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common::process;
use std::{fs, process::Command};

const IMAGE: &str = r#"<binaryimage arch="x86:LE:64:default:gcc">
<bytechunk space="ram" offset="0x1000" readonly="true">89f8660fafc6c3</bytechunk>
<bytechunk space="ram" offset="0x1010" readonly="true">89f8660fafc6660fafc2c3</bytechunk>
<bytechunk space="ram" offset="0x1020" readonly="true">0fb707660fafc6668907c3</bytechunk>
<symbol space="ram" offset="0x1000" name="mul_low"/>
<symbol space="ram" offset="0x1010" name="mul_chain"/>
<symbol space="ram" offset="0x1020" name="mul_store"/>
</binaryimage>"#;

const DRIVER: &str = r#"
int main(void) {
    static volatile uint16_t samples[] = {0, 1, 0x7fff, 0x8000, 43946, 52719, 0xfffe, 0xffff};
    for (unsigned i = 0; i < sizeof samples / sizeof *samples; ++i) {
        for (unsigned j = 0; j < sizeof samples / sizeof *samples; ++j) {
            uint16_t a = samples[i], b = samples[j];
            uint16_t expected = (uint16_t)((uint32_t)a * b), stored = a;
            if (mul_low(a, b) != expected) return 1;
            if (mul_low_16(a, b) != expected) return 4;
            mul_store(&stored, b);
            if (stored != expected) return 2;
            for (unsigned k = 0; k < sizeof samples / sizeof *samples; ++k) {
                uint16_t c = samples[k];
                if (mul_chain(a, b, c) != (uint16_t)((uint32_t)expected * c)) return 3;
            }
        }
    }
    return 0;
}
"#;

#[test]
fn narrow_unsigned_multiplication_matches_machine_low_words() {
    let dir = tempfile::tempdir().unwrap();
    let image = dir.path().join("narrow-multiply.xml");
    fs::write(&image, IMAGE).unwrap();
    let mut c = String::from("#include <stdint.h>\ntypedef uint16_t uint2; typedef uint32_t uint4;\n\
                             typedef unsigned short ushort; typedef unsigned int uint;\n");
    for (name, signature) in [
        ("mul_low", "unsigned short mul_low(unsigned short a,unsigned short b)"),
        ("mul_chain", "unsigned short mul_chain(unsigned short a,unsigned short b,unsigned short c)"),
        ("mul_store", "void mul_store(unsigned short *a,unsigned short b)"),
    ] {
        let output = process::required_output(
            Command::new(env!("CARGO_BIN_EXE_kuna"))
                .arg("decompile").arg(&image).arg(name)
                .args(["--assert-strict", "--assert", &format!("prototype {name} {signature}")]),
        );
        c.push_str(std::str::from_utf8(&output.stdout).unwrap());
        c.push('\n');
    }
    let raw = dir.path().join("narrow-multiply-16.bin");
    fs::write(&raw, [0x89, 0xc8, 0x0f, 0xaf, 0xc2, 0xc3]).unwrap();
    let output = process::required_output(
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&raw)
            .args(["0x1000", "--addr", "--raw-image", "--base", "0x1000", "--target",
                "x86:LE:16:Real Mode:default", "--assert-strict",
                "--assert", "param 0 %CX unsigned short a",
                "--assert", "param 1 %DX unsigned short b",
                "--assert", "return %AX unsigned short"]),
    );
    c.push_str("#define sub_1000 mul_low_16\n");
    c.push_str(std::str::from_utf8(&output.stdout).unwrap());
    c.push_str("\n#undef sub_1000\n");
    c.push_str(DRIVER);
    let source = dir.path().join("printed.c");
    let executable = dir.path().join("printed");
    fs::write(&source, &c).unwrap();
    for opt in ["-O0", "-O2"] {
        process::required_output(
            Command::new("cc")
                .args([opt, "-std=c11", "-Werror=implicit-function-declaration",
                    "-Werror=return-type", "-fsanitize=undefined", "-fno-sanitize-recover=undefined"])
                .arg(&source).arg("-o").arg(&executable),
        );
        let output = process::output_with_timeout(
            &mut Command::new(&executable),
            std::time::Duration::from_secs(10),
            std::time::Duration::from_millis(10),
        ).unwrap();
        assert!(output.status.success(), "{opt}: {}\n{c}", String::from_utf8_lossy(&output.stderr));
    }
}
