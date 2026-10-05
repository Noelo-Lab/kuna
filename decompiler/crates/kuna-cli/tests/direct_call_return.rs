mod common;

use common::process;
use std::process::Command;

const ASM: &str = include_str!("fixtures/direct_call_return.S");
const DRIVER: &str = r#"
#include <stdint.h>
#include <assert.h>
#include <string.h>
void * __attribute__((ms_abi)) checked_return(void *, uint64_t);
void * __attribute__((ms_abi)) simple_checked_return(void *, uint64_t);
int main(int argc, char **argv) {
    if (argc > 1) return checked_return(0, UINT64_MAX) != 0;
    int value = 42;
    assert(checked_return(&value, 0) == &value);
    assert(simple_checked_return(&value, 0) == &value);
    return 0;
}
"#;

fn build(source: &str) -> (std::path::PathBuf, std::path::PathBuf, String) {
    let asm = common::scratch_file("direct-call-return", "S");
    let elf = asm.with_extension("elf");
    let assertions = asm.with_extension("kuna");
    std::fs::write(&asm, source).unwrap();
    process::required_output(
        Command::new("clang")
            .args([
                "-nostdlib",
                "-no-pie",
                "-Wl,--build-id=none",
                "-Wl,-Ttext=0x401000",
                "-Wl,-e,checked_return",
            ])
            .arg(&asm)
            .arg("-o")
            .arg(&elf),
    );
    let nm = process::required_output(Command::new("nm").args(["-S", "--defined-only"]).arg(&elf));
    let mut declarations = String::new();
    for line in String::from_utf8(nm.stdout).unwrap().lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        if words.len() != 4 || words[2] != "T" {
            continue;
        }
        let addr = u64::from_str_radix(words[0], 16).unwrap();
        let size = u64::from_str_radix(words[1], 16).unwrap();
        let proto = match words[3] {
            "checked_return" | "simple_checked_return" => format!(
                "void * MSABI {}(void *value, unsigned long long cookie)",
                words[3]
            ),
            "check_cookie" | "simple_check" => {
                format!("void MSABI {}(unsigned long long cookie)", words[3])
            }
            _ => format!("void MSABI {}(void)", words[3]),
        };
        declarations.push_str(&format!(
            "function {addr:#x}-{:#x}={}\nprototype {addr:#x} {proto}\n",
            addr + size,
            words[3]
        ));
    }
    std::fs::write(&assertions, declarations).unwrap();
    (asm, elf, format!("@{}", assertions.display()))
}

fn decompile(elf: &std::path::Path, assertions: &str, functions: &str, options: &[&str]) -> String {
    let mut args = vec![
        "decompile-all",
        elf.to_str().unwrap(),
        "--functions",
        functions,
        "--mode",
        "reliable",
        "--assert-strict",
        "--assert",
        assertions,
        "--json",
    ];
    args.extend_from_slice(options);
    let binary =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let output = process::required_output(Command::new(binary).args(args));
    let doc: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    for function in doc["functions"].as_array().unwrap() {
        assert!(function["error"].is_null(), "{function}");
    }
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["name"] == "checked_return" || f["name"] == "simple_checked_return")
        .map(|f| {
            assert!(f["error"].is_null(), "{f}");
            f["code"].as_str().unwrap().to_owned()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn terminal_failure_call_preserves_the_pre_call_result() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let (asm, elf, assertions) = build(ASM);
    let c = asm.with_extension("c");
    let native = asm.with_extension("native");
    std::fs::write(&c, DRIVER).unwrap();
    process::required_output(
        Command::new("clang")
            .arg(&asm)
            .arg(&c)
            .arg("-o")
            .arg(&native),
    );
    process::required_output(&mut Command::new(&native));
    assert_eq!(
        Command::new(&native).arg("fail").status().unwrap().code(),
        Some(75)
    );

    let plain = decompile(
        &elf,
        &assertions,
        "checked_return,simple_checked_return",
        &[],
    );
    assert_eq!(plain.matches("return value;").count(), 1, "{plain}");
    let unmarked_ordered = decompile(
        &elf,
        &assertions,
        "check_cookie,checked_return",
        &["--sort", "name"],
    );
    assert!(
        !unmarked_ordered.contains("return value;"),
        "{unmarked_ordered}"
    );
    for extra in [vec![], vec!["--option", "msvcstackguard", "on"]] {
        let mut options = vec!["--option", "noreturn", "cookie_failure"];
        options.extend(extra);
        let code = decompile(
            &elf,
            &assertions,
            "checked_return,simple_checked_return",
            &options,
        );
        assert_eq!(code.matches("return value;").count(), 2, "{code}");
        let emitted = asm.with_extension("emitted.c");
        std::fs::write(&emitted, format!("#include <stdint.h>\ntypedef uint64_t uint8;\nvoid __attribute__((ms_abi)) check_cookie(uint8);\nvoid __attribute__((ms_abi)) simple_check(uint8);\n{DRIVER}\n{code}")).unwrap();
        let helpers = asm.with_extension("helpers.S");
        let start = ASM.find(".globl simple_check\n").unwrap();
        std::fs::write(&helpers, format!(".text\n{}", &ASM[start..])).unwrap();
        for level in ["-O0", "-O2"] {
            let exe = asm.with_extension("emitted");
            process::required_output(
                Command::new("clang")
                    .arg(level)
                    .arg(&emitted)
                    .arg(&helpers)
                    .arg("-o")
                    .arg(&exe),
            );
            process::required_output(&mut Command::new(&exe));
            assert_eq!(
                Command::new(&exe).arg("fail").status().unwrap().code(),
                Some(75)
            );
        }
    }
    let off = decompile(
        &elf,
        &assertions,
        "checked_return,simple_checked_return",
        &[
            "--option",
            "noreturn",
            "cookie_failure",
            "--option",
            "calleeretpreserves",
            "off",
            "--option",
            "calleepreserves",
            "off",
        ],
    );
    assert!(!off.contains("return value;"), "{off}");
    let ordered = decompile(
        &elf,
        &assertions,
        "check_cookie,checked_return",
        &["--option", "noreturn", "cookie_failure", "--sort", "name"],
    );
    assert!(ordered.contains("return value;"), "{ordered}");
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn returning_writes_calls_and_incomplete_bodies_do_not_prove_preservation() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    for source in [
        ASM.replace(
            "    ret\n.Lfailure:",
            "    movq $99, %rax\n    ret\n.Lfailure:",
        ),
        ASM.replace(
            "    call cookie_failure",
            "    xorl %eax, %eax\n    call cookie_failure",
        ),
        ASM.replace("    call cookie_failure", "    call simple_check"),
        ASM.replace("    call cookie_failure", "    call *%r11"),
        ASM.replace(
            "check_cookie:\n    addq $1, %rcx\n",
            "check_cookie:\n    addq $1, %rcx\n    .byte 0x0f, 0xff\n",
        ),
    ] {
        let (_, elf, assertions) = build(&source);
        let code = decompile(
            &elf,
            &assertions,
            "checked_return",
            &["--option", "noreturn", "cookie_failure"],
        );
        assert!(!code.contains("return value;"), "{code}");
    }
    let (_, known_elf, known_assertions) = build(&ASM.replace("cookie_failure", "abort"));
    let known = decompile(&known_elf, &known_assertions, "checked_return", &[]);
    assert!(known.contains("return value;"), "{known}");
    let source = ASM.replace(
        "    call cookie_failure\n    int3",
        "    addq $40, %rsp\n    ret",
    );
    let (_, elf, assertions) = build(&source);
    let code = decompile(&elf, &assertions, "checked_return", &[]);
    assert!(code.contains("return value;"), "{code}");
}
