mod common;

use common::process;
use std::process::Command;

#[test]
fn guarded_arm_branch_tables_keep_cases_and_the_shared_frame() {
    for tool in ["arm-none-eabi-as", "arm-none-eabi-ld"] {
        if process::optional_output(Command::new(tool).arg("--version")).is_none() {
            eprintln!("skipping ARM assembly fixture: {tool} is unavailable");
            return;
        }
    }
    let dir = tempfile::tempdir().unwrap();
    let assembly = dir.path().join("switch.S");
    let object = dir.path().join("switch.o");
    let elf = dir.path().join("switch.elf");
    std::fs::write(&assembly, include_str!("fixtures/arm_conditional_switch.S")).unwrap();
    process::required_output(
        Command::new("arm-none-eabi-as")
            .args(["-march=armv5t", "-o"])
            .arg(&object)
            .arg(&assembly),
    );
    process::required_output(
        Command::new("arm-none-eabi-ld")
            .args(["-Ttext=0x10000", "-e", "arm_switch_conditional", "-o"])
            .arg(&elf)
            .arg(&object),
    );
    let specs = common::repo_root().join("specs");
    let functions = [
        ("arm_switch_conditional", 0x10000, 0x10054, "int input"),
        ("arm_switch_unconditional", 0x10054, 0x100a4, "int input"),
        ("arm_switch_addls", 0x100a4, 0x100e8, "int input"),
        ("arm_switch_frame", 0x100e8, 0x10148, "int input, int bias"),
        (
            "arm_indirect_call",
            0x10148,
            0x10160,
            "struct Callback *target, int input",
        ),
    ];
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for tail in ["on", "off"] {
        let binary = env!("CARGO_BIN_EXE_kuna");
        let runner = std::env::var_os("KUNA_CLI_WRAPPER").unwrap_or_else(|| binary.into());
        let mut command = Command::new(runner);
        command.env("KUNA_BINARY", binary).env("KUNA_SPECS", &specs).args([
            "decompile-all", elf.to_str().unwrap(), "--target", "ARM:LE:32:v5t:default",
            "--isa", "arm", "--mode", "reliable", "--jobs", "1", "--json",
            "--assert-strict", "--sleighpath", specs.to_str().unwrap(),
            "--functions", "arm_switch_conditional,arm_switch_unconditional,arm_switch_addls,arm_switch_frame,arm_indirect_call",
            "--option", "tailcalljump", tail,
            "--assert", "typedef struct Callback { int (*invoke)(int); };",
        ]);
        for (name, start, end, params) in functions {
            command.args(["--assert", &format!("function {start:#x}-{end:#x}={name}")]);
            command.args([
                "--assert",
                &format!("prototype {name} int {name}({params})"),
            ]);
        }
        let output = process::required_output(&mut command);
        let doc: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(
            doc["assertions"]
                .as_array()
                .unwrap()
                .iter()
                .all(|a| a["status"] != "rejected"),
            "{doc}"
        );
        let results = doc["functions"].as_array().unwrap();
        assert_eq!(results.len(), functions.len(), "{doc}");
        let mut printed = String::new();
        for result in results {
            assert!(result["error"].is_null(), "{doc}");
            let code = result["code"].as_str().unwrap();
            assert!(!code.contains("jump-as-call"), "{code}");
            if code.contains("arm_indirect_call(") {
                assert!(
                    code.contains("target->invoke") && code.contains(")(input)"),
                    "{code}"
                );
                assert!(!code.contains("switch"), "{code}");
            } else {
                assert!(code.contains("switch"), "{code}");
                for case in 0..4 {
                    assert!(
                        code.contains(&format!("case {case}:"))
                            || code.contains(&format!("case {}:", case + 58))
                            || code.contains(&format!("case {:#x}:", case + 58)),
                        "{code}"
                    );
                }
                assert!(!code.contains("code *"), "{code}");
            }
            printed.push_str(code);
            printed.push('\n');
        }
        let source = dir.path().join("printed.c");
        std::fs::write(
            &source,
            format!(
                r#"
#include <assert.h>
#include <limits.h>
typedef int int4;
typedef unsigned int uint4;
typedef struct Callback {{ int (*invoke)(int); }} Callback;
{printed}
static int callback(int input) {{ return input * 2; }}
static struct Callback callbacks = {{callback}};
int main(void) {{
    const int boundaries[] = {{INT_MIN, INT_MIN + 57, -59, -58, -1, 0, 3, 4,
                               57, 58, 59, 60, 61, 62, INT_MAX}};
    for (unsigned j = 0; j < sizeof boundaries / sizeof *boundaries; ++j) {{
        int input = boundaries[j];
        int conditional = input >= 58 && input <= 61 ? input - 58 + 11 : -1;
        int unconditional = input >= 0 && input <= 3 ? input + 21 : -1;
        int addls = input >= 0 && input <= 3 ? input + 31 : -1;
        assert(arm_switch_conditional(input) == conditional);
        assert(arm_switch_unconditional(input) == unconditional);
        assert(arm_switch_addls(input) == addls);
        for (int bias = -10; bias <= 10; ++bias) {{
            int framed = input >= 58 && input <= 61 ? input - 58 + 41 : -1;
            assert(arm_switch_frame(input, bias) == framed + bias);
        }}
    }}
    for (int input = 0; input <= 62; ++input) {{
        int expected = input >= 58 && input <= 61 ? input - 58 + 11 : -1;
        assert(arm_switch_conditional(input) == expected);
        assert(arm_indirect_call(&callbacks, input) == input * 2 + 1);
    }}
}}
"#
            ),
        )
        .unwrap();
        if compilers.is_empty() {
            eprintln!("skipping emitted C execution: GCC and Clang are unavailable");
        }
        for cc in &compilers {
            for opt in ["-O0", "-O2"] {
                let executable = dir.path().join("round-trip");
                process::required_output(
                    Command::new(cc)
                        .args([
                            opt,
                            "-std=c11",
                            "-Wall",
                            "-Wextra",
                            "-Werror",
                            "-fsanitize=undefined",
                            "-fno-sanitize-recover=all",
                        ])
                        .arg(&source)
                        .arg("-o")
                        .arg(&executable),
                );
                process::required_output(&mut Command::new(&executable));
            }
        }
    }
}
