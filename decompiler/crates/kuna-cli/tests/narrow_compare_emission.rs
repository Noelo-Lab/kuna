//! A 16-bit machine sum must wrap before C compares it, including on a wider host.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common::process;
use std::{fmt::Write, fs, process::Command};

#[test]
fn narrow_word_equality_compares_the_wrapped_result() {
    let dir = tempfile::tempdir().unwrap();
    let raw = dir.path().join("sum.bin");
    let mut c = String::new();
    for (name, compare, set, word) in [
        ("sum_is_zero", [0x85, 0xc0], 0x94, "unsigned short"), // TEST AX,AX; SETZ
        ("sum_is_word", [0x39, 0xd8], 0x94, "unsigned short"), // CMP AX,BX; SETZ
        ("sum_is_not_word", [0x39, 0xd8], 0x95, "unsigned short"), // CMP AX,BX; SETNZ
        ("sum_is_short", [0x39, 0xd8], 0x94, "short"), // CMP AX,BX; SETZ beside a short
    ] {
        fs::write(&raw, [0x89, 0xc8, 0x01, 0xd0, compare[0], compare[1],
            0x0f, set, 0xc0, 0x0f, 0xb6, 0xc0, 0xc3]).unwrap();
        let out = process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&raw).args([
                "0x1000", "--addr", "--raw-image", "--base", "0x1000", "--assert-strict",
                "--target", "x86:LE:16:Real Mode:default",
                "--assert", "param 0 %CX unsigned short x",
                "--assert", &format!("param 1 %DX {word} y"),
                "--assert", &format!("param 2 %BX {word} z"),
                "--assert", "return %AX unsigned short",
            ]));
        writeln!(c, "#define sub_1000 {name}\n{}\n#undef sub_1000", std::str::from_utf8(&out.stdout).unwrap()).unwrap();
    }
    c.push_str(r#"
int main(void) {
    static volatile unsigned short values[] = { 0, 1, 2, 0x7fff, 0x8000, 0xfffe, 0xffff };
    for (unsigned i = 0; i < sizeof values / sizeof *values; ++i) {
        for (unsigned j = 0; j < sizeof values / sizeof *values; ++j) {
            unsigned short x = values[i], y = values[j], sum = (unsigned short)(x + y);
            if (sum_is_zero(x, y, 0) != (sum == 0)) return 1;
            for (unsigned k = 0; k < sizeof values / sizeof *values; ++k) {
                unsigned short z = values[k];
                if (sum_is_word(x, y, z) != (sum == z)) return 2;
                if (sum_is_not_word(x, y, z) != (sum != z)) return 3;
                if (sum_is_short(x, (short)y, (short)z) != (sum == z)) return 4;
            }
        }
    }
    return 0;
}
"#);
    let source = dir.path().join("sum.c");
    let binary = dir.path().join("sum");
    fs::write(&source, &c).unwrap();
    for opt in ["-O0", "-O2"] {
        process::required_output(Command::new("cc").args([
            opt, "-std=c11", "-fsanitize=undefined", "-fno-sanitize-recover=undefined",
        ]).arg(&source).arg("-o").arg(&binary));
        process::required_output(&mut Command::new(&binary));
    }
}
