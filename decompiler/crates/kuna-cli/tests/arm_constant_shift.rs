//! Authored ARM LSL bodies round-trip unchanged through C11, including UBSan.
mod common;

use common::process;
use std::process::Command;

struct Probe {
    name: String,
    prototype: String,
    words: Vec<u32>,
    check: String,
}

fn probes() -> Vec<Probe> {
    let mut probes = vec![
        Probe {
            name: "helper".into(),
            prototype: "int helper(int value)".into(),
            words: vec![0xe12fff1e],
            check: "CHECK((uint32_t)helper(value), raw);".into(),
        },
        Probe {
            name: "negative_helper".into(),
            prototype: "int negative_helper(int value)".into(),
            words: vec![0xe3500000, 0xb3a00001, 0xa3a00000, 0xe12fff1e],
            check: "CHECK((uint32_t)negative_helper(value), raw >> 31);".into(),
        },
    ];
    for count in [0, 1, 2, 3, 31] {
        let lsl = 0xe1a00000 | (count << 7);
        for (kind, result, parameter, tail, expected) in [
            ("unsigned", "unsigned int", "int", vec![], "bits"),
            ("signed", "int", "int", vec![], "bits"),
            ("healthy", "unsigned int", "unsigned int", vec![], "bits"),
            (
                "asr",
                "int",
                "int",
                vec![0xe1a00fc0],
                "(bits & 0x80000000u) ? UINT32_MAX : 0",
            ),
            ("negative", "int", "int", vec![], "bits >> 31"),
        ] {
            let name = format!("{kind}_{count}");
            let mut words = vec![lsl];
            words.extend(tail);
            words.push(0xe12fff1e);
            if kind == "negative" {
                words = vec![0xe92d4010, lsl, 0xeb000000, 0xe8bd8010];
            }
            probes.push(Probe {
                prototype: format!("{result} {name}({parameter} value)"),
                check: format!(
                    "{{ uint32_t bits = (uint32_t)((uint64_t)raw << {count}); \
                     CHECK((uint32_t){name}(({parameter})value), {expected}); }}"
                ),
                name,
                words,
            });
        }
    }
    probes.push(Probe {
        name: "store".into(),
        prototype: "int store(int value, int *output)".into(),
        words: vec![0xe1a00100, 0xe5810000, 0xe12fff1e],
        check: "{ int out = 123; uint32_t want = (uint32_t)((uint64_t)raw << 2); \
                CHECK((uint32_t)store(value, &out), want); CHECK((uint32_t)out, want); }"
            .into(),
    });
    probes.push(Probe {
        name: "load".into(),
        prototype: "unsigned int load(void)".into(),
        words: vec![0xe3a01802, 0xe5910000, 0xe1a00100, 0xe12fff1e],
        check: "{ shared = value; CHECK(load(), (uint32_t)((uint64_t)raw << 2)); }".into(),
    });
    for (name, parameter) in [("variable", "unsigned int"), ("variable_signed", "int")] {
        probes.push(Probe {
            name: name.into(),
            prototype: format!("unsigned int {name}({parameter} value, unsigned int count)"),
            words: vec![0xe1a00110, 0xe12fff1e],
            check: format!(
                "for (unsigned c = 0; c < 300; ++c) {{ unsigned low = c & 255; \
                 uint32_t want = low < 32 ? (uint32_t)((uint64_t)raw << low) : 0; \
                 CHECK({name}(({parameter})value, c), want); }}"
            ),
        });
    }
    probes.push(Probe {
        name: "repeat".into(),
        prototype: "int repeat(int value, unsigned int count, int *output)".into(),
        words: vec![0xe3510000, 0x012fff1e, 0xe1a00100, 0xe5820000,
            0xe2511001, 0x1afffffb, 0xe12fff1e],
        check: "for (unsigned c = 0; c < 20; ++c) { int out = 123; uint32_t bits = raw; \
                for (unsigned k = 0; k < c; ++k) bits = (uint32_t)((uint64_t)bits << 2); \
                CHECK((uint32_t)repeat(value, c, &out), bits); CHECK((uint32_t)out, c ? bits : 123u); }".into(),
    });
    let displacement = (-(probes.len() as i32 * 64 + 12) / 4) as u32 & 0x00ffffff;
    probes.push(Probe {
        name: "call_shift".into(),
        prototype: "unsigned int call_shift(int value)".into(),
        words: vec![
            0xe92d4010,
            0xeb000000 | displacement,
            0xe1a00180,
            0xe8bd8010,
        ],
        check: "CHECK(call_shift(value), (uint32_t)((uint64_t)raw << 3));".into(),
    });
    probes
}

#[test]
fn signed_constant_lsl_is_defined_and_preserves_signed_consumers() {
    let probes = probes();
    let dir = tempfile::tempdir().unwrap();
    let image = dir.path().join("lsl.bin");
    let assertions = dir.path().join("lsl.kuna");
    let mut bytes = Vec::new();
    let mut declarations = String::new();
    let mut entries = Vec::new();
    for probe in &probes {
        let start = 0x10000 + bytes.len();
        for (slot, word) in probe.words.iter().enumerate() {
            let word = if probe.name.starts_with("negative_")
                && probe.name != "negative_helper"
                && slot == 2
            {
                let displacement = (0x10040i32 - start as i32 - 16) / 4;
                0xeb000000 | (displacement as u32 & 0x00ffffff)
            } else {
                *word
            };
            bytes.extend(word.to_le_bytes());
        }
        let end = 0x10000 + bytes.len();
        declarations.push_str(&format!(
            "function {start:#x}-{end:#x}={}\nprototype {start:#x} {}\n",
            probe.name, probe.prototype
        ));
        entries.push(format!("{start:#x}"));
        bytes.resize(bytes.len().next_multiple_of(64), 0);
    }
    bytes.resize(0x10004, 0);
    declarations.push_str("data 0x20000 int shared\nvolatile 0x20000+4\n");
    std::fs::write(&image, bytes).unwrap();
    std::fs::write(&assertions, declarations).unwrap();
    let mut command = Command::new(
        std::env::var_os("KUNA_TEST_RUNNER").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
    );
    command.env("KUNA_BINARY", env!("CARGO_BIN_EXE_kuna"));
    command.env("KUNA_SPECS", common::repo_root().join("specs"));
    command.env("KUNA_DECODE_JOBS", "1");
    command.args([
        "decompile-all",
        image.to_str().unwrap(),
        "--raw-image",
        "--base",
        "0x10000",
        "--target",
        "ARM:LE:32:v5t:default",
        "--isa",
        "arm",
        "--mode",
        "reliable",
        "--option",
        "armconstshift",
        "on",
        "--jobs",
        "1",
        "--assert-strict",
        "--json",
        "--assert",
    ]);
    command.arg(format!("@{}", assertions.display()));
    for entry in entries {
        command.args(["--entry", &entry]);
    }
    let output = process::required_output(&mut command);
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(json["error"].is_null(), "{json}");
    let functions = json["functions"].as_array().unwrap();
    assert_eq!(functions.len(), probes.len(), "{json}");
    for assertion in json["assertions"].as_array().unwrap() {
        assert_eq!(assertion["status"], "applied", "{assertion}");
    }
    let mut emitted = String::new();
    for function in functions {
        assert!(function["error"].is_null(), "{function}");
        let code = function["code"].as_str().unwrap();
        if function["name"] == "load" {
            let body = code.split_once('{').unwrap().1;
            assert_eq!(body.matches("shared").count(), 1, "{code}");
        }
        if function["name"] == "call_shift" {
            assert_eq!(code.matches("helper(").count(), 1, "{code}");
        }
        if function["name"] == "repeat" {
            assert!(!code.contains("<<="), "signed compound assignment: {code}");
        }
        if function["name"] == "unsigned_2" {
            assert!(
                code.contains("int4 value"),
                "locked parameter changed: {code}"
            );
        }
        if function["name"] == "healthy_2" {
            assert!(
                code.contains("return value << 2;"),
                "healthy shift changed: {code}"
            );
        }
        if function["name"] == "variable" {
            assert!(
                code.contains("< 32u ?"),
                "register-count guard lost: {code}"
            );
        }
        emitted.push_str(code);
        emitted.push('\n');
    }
    let checks = probes
        .iter()
        .map(|p| p.check.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let source = format!(
        r#"
#include <stdint.h>
#include <limits.h>
#include <stdio.h>
#include <stdbool.h>
typedef int int4;
typedef unsigned int uint4;
typedef unsigned char byte;
volatile int4 shared;
_Static_assert(sizeof(int4) == 4 && sizeof(uint4) == 4 && INT_MIN == INT32_MIN &&
    INT_MAX == INT32_MAX && UINT_MAX == UINT32_MAX, "ARM32 integer contract required");
{emitted}
#define CHECK(got, want) do {{ uint32_t g = (got), w = (want); \
    if (g != w) {{ printf("%s input=%08x got=%08x want=%08x\n", #got, raw, g, w); return 1; }} }} while (0)
int main(void) {{
    const int boundaries[] = {{0, 1, -1, INT_MIN, INT_MAX, -16383, -16384,
        0x3fffffff, 0x40000000, 0x1fffffff, 0x20000000, 0x0fffffff, 0x10000000}};
    uint32_t seed = 0x12345678u;
    for (unsigned i = 0; i < 269; ++i) {{
        seed = seed * 1664525u + 1013904223u;
        int value = i < sizeof boundaries / sizeof boundaries[0] ? boundaries[i] : (int)seed;
        uint32_t raw = (uint32_t)value;
        {checks}
    }}
    return 0;
}}
"#
    );
    let cfile = common::scratch_file("arm-constant-shift", "c");
    std::fs::write(&cfile, &source).unwrap();
    std::fs::write(cfile.with_extension("json"), &output.stdout).unwrap();
    eprintln!("unchanged generated-C round trip: {}", cfile.display());
    let mut tested = 0;
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        tested += 1;
        for level in ["-O0", "-O2"] {
            for sanitize in [false, true] {
                let exe = dir.path().join(format!("{cc}{level}-{sanitize}"));
                let mut command = Command::new(cc);
                command.args(["-std=c11", "-pedantic-errors", level]);
                if sanitize {
                    command.args(["-fsanitize=undefined", "-fno-sanitize-recover=undefined"]);
                }
                command.arg(&cfile).arg("-o").arg(&exe);
                let compiled = command.output().unwrap();
                assert!(
                    compiled.status.success(),
                    "{cc} {level}: {}\n{source}",
                    String::from_utf8_lossy(&compiled.stderr)
                );
                let run = Command::new(&exe).output().unwrap();
                assert!(
                    run.status.success(),
                    "{cc} {level} sanitize={sanitize}: {}\n{}\n{source}",
                    String::from_utf8_lossy(&run.stdout),
                    String::from_utf8_lossy(&run.stderr)
                );
            }
        }
    }
    assert!(tested > 0, "round trip requires GCC or Clang");
}
