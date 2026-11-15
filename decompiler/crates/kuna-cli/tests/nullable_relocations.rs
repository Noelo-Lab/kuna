//! Nullable identities survive address fragments, addends and ordinary import calls.
use crate::common;
use std::collections::BTreeSet;
use std::process::Command;

fn decompile(root: &std::path::Path, file: &str, name: &str, prototype: &str) -> String {
    let (code, error, rc) = common::run_kuna(&[
        "decompile",
        root.join(file).to_str().unwrap(),
        name,
        "--assert-strict",
        "--assert",
        &format!("prototype {name} {prototype}"),
        "--assert",
        "prototype optional int optional(int value)",
    ]);
    assert_eq!(rc, 0, "{error}");
    assert!(!code.contains("Nullable weak"), "{code}");
    code
}

#[test]
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn weak_imports_remain_nullable_across_non_pic_and_mixed_forms() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let root = common::repo_root().join("tests/fixtures/nullable-relocations");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("provenance.json")).unwrap()).unwrap();
    for object in manifest["objects"].as_array().unwrap() {
        let file = object["file"].as_str().unwrap();
        let abi = object["abi"].as_str().unwrap();
        let kind = object["kind"].as_str().unwrap();
        let wide = matches!(abi, "64" | "x86-64-large" | "x86-64-rip" | "x86-64-rodata");
        let word = if wide { "uint64_t" } else { "uint32_t" };
        let mut functions = Vec::new();
        if kind == "address" {
            let output = if abi == "n32" {
                "unsigned int"
            } else {
                "unsigned long"
            };
            for name in [
                "address_value",
                "address_offset",
                "address_negative",
                "unrelated_literal",
            ] {
                functions.push((name.to_string(), format!("{output} {name}(void)")));
            }
        } else {
            for prefix in if kind == "mixed" {
                vec!["", "pic_"]
            } else {
                vec![""]
            } {
                for (name, args) in [
                    ("guard", "int value"),
                    ("offset_guard", "void"),
                    ("defined_guard", "int value"),
                    ("present", "int value"),
                ] {
                    functions.push((
                        format!("{prefix}{name}"),
                        format!("int {prefix}{name}({args})"),
                    ));
                }
            }
        }
        let codes: Vec<_> = functions
            .iter()
            .map(|(name, prototype)| decompile(&root, file, name, prototype))
            .collect();
        let code = codes.join("\n");
        let names: BTreeSet<_> = code
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .filter(|name| {
                name.starts_with("dat_") && name[4..].chars().all(|c| c.is_ascii_hexdigit())
            })
            .collect();
        let source = common::scratch_file("nullable-relocations", "c");
        let driver = source.with_extension("driver.c");
        let executable = source.with_extension("exe");
        let globals: String = names
            .iter()
            .map(|name| format!("extern {word} {name};\n"))
            .collect();
        let declarations: String = codes
            .iter()
            .map(|code| format!("{};\n", code.split_once('{').unwrap().0.trim()))
            .collect();
        std::fs::write(&source, format!("#include <stdint.h>\n#include <stdbool.h>\nextern int optional(int) __attribute__((weak));\n{globals}{declarations}{code}")).unwrap();
        let definitions: String = names
            .iter()
            .map(|name| format!("{word} {name};\n"))
            .collect();
        let setters: String = names
            .iter()
            .map(|name| format!("{name}=({word})bits;"))
            .collect();
        let bindings = if kind == "address" {
            vec![0]
        } else {
            vec![0, 1]
        };
        for bound in bindings {
            let body = if kind == "address" {
                let values = if abi == "x86-64" {
                    "0x8000,0x400000,0x7fff7ff0"
                } else if abi == "x86-64-rip" {
                    "0,1,0x7fff,0x8000,0x400000,0x7fff7ff0"
                } else if abi == "n32" {
                    "0,1,0x7fff,0x8000,0x400000,0x7fff7ff0,0x80000000,0xfffffff0"
                } else {
                    "0,1,0x7fff,0x8000,0x400000,0x7fffffffffffffff,0x8000000000000000,0xffffffff80007ff0,0xffffffffffffffff"
                };
                let result = if abi == "n32" { "uint32_t" } else { "uint64_t" };
                format!("int main(void){{uint64_t values[]={{{values}}};for(unsigned i=0;i<sizeof(values)/sizeof(values[0]);i++){{uint64_t bits=values[i];{setters}if(address_value()!=({result})bits||address_offset()!=({result})(bits+0x8000u)||address_negative()!=({result})(bits-0x8000u)||unrelated_literal()!=0x402000u)return 1;}}return 0;}}")
            } else {
                let checks = if kind == "mixed" {
                    "||pic_guard(x)!=want||pic_defined_guard(x)!=x+11||pic_offset_guard()!=!BOUND"
                } else {
                    ""
                };
                let calls = if kind == "mixed" { 20000 } else { 10000 };
                format!("int calls,seen;\n#if BOUND\n__attribute__((section(\".weakbinding\"))) int optional(int x){{calls++;seen=x;return x+11;}}\n#else\nextern int optional(int) __attribute__((weak));\n#endif\nint main(void){{uintptr_t bits=(uintptr_t)optional;{setters}for(int x=-5000;x<5000;x++){{int want=x+(BOUND?11:7);if(guard(x)!=want||defined_guard(x)!=x+11||offset_guard()!=!BOUND{checks})return 1;}}return calls!=(BOUND?{calls}:0)||(BOUND&&seen!=4999);}}")
            };
            std::fs::write(
                &driver,
                format!("#include <stdint.h>\n{declarations}{definitions}{body}\n"),
            )
            .unwrap();
            for opt in ["-O0", "-O2"] {
                common::process::required_output(
                    Command::new("cc")
                        .args([
                            "-std=c11",
                            "-no-pie",
                            opt,
                            "-Wall",
                            "-Wextra",
                            "-Werror",
                            "-Wno-parentheses",
                            "-Wuninitialized",
                            "-fno-lto",
                            "-fsanitize=undefined",
                            "-fno-sanitize-recover=undefined",
                            &format!("-DBOUND={bound}"),
                        ])
                        .arg(&source)
                        .arg(&driver)
                        .arg("-Wl,--section-start=.weakbinding=0x407ff0")
                        .arg("-o")
                        .arg(&executable),
                );
                let output = common::process::output_with_timeout(
                    &mut Command::new(&executable),
                    std::time::Duration::from_secs(10),
                    std::time::Duration::from_millis(10),
                )
                .unwrap();
                assert!(
                    output.status.success(),
                    "{file}: {code}\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        for path in [source, driver, executable] {
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn unmatched_weak_operands_report_the_missing_model() {
    let root = common::repo_root().join("tests/fixtures/nullable-relocations");
    let (code, error, rc) = common::run_kuna(&[
        "decompile",
        root.join("rip-x86-64.elf").to_str().unwrap(),
        "unsupported_offset",
    ]);
    assert_eq!(rc, 0, "{error}");
    assert!(
        code.contains("Nullable weak relocation operand could not be modeled"),
        "{code}"
    );
}

#[test]
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn readonly_and_got_weak_references_share_identity() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    for abi in ["n32", "64"] {
        let image = common::repo_root().join(format!(
            "tests/fixtures/mips-weak-imports/identity-{abi}-be-mixed.o"
        ));
        let (code, error, rc) = common::run_kuna(&[
            "decompile",
            image.to_str().unwrap(),
            "same",
            "--assert-strict",
            "--assert",
            "prototype same unsigned int same(void)",
            "--assert",
            "prototype optional int optional(int value)",
        ]);
        assert_eq!(rc, 0, "{error}");
        // One immutable weak binding on both sides makes this equality true,
        // including the unbound zero case. No arbitrary synthetic address fits.
        assert!(
            code.contains("return 1;") || code.contains("return true;"),
            "{code}"
        );
        let source = common::scratch_file("weak-identity", "c");
        let exe = source.with_extension("exe");
        std::fs::write(
            &source,
            format!("#include <stdbool.h>\n{code}\nint main(void){{return same()!=1;}}\n"),
        )
        .unwrap();
        for opt in ["-O0", "-O2"] {
            common::process::required_output(
                Command::new("cc")
                    .args([
                        opt,
                        "-Wall",
                        "-Wextra",
                        "-Werror",
                        "-fsanitize=undefined",
                        "-fno-sanitize-recover=undefined",
                    ])
                    .arg(&source)
                    .arg("-o")
                    .arg(&exe),
            );
            let output = common::process::output_with_timeout(
                &mut Command::new(&exe),
                std::time::Duration::from_secs(5),
                std::time::Duration::from_millis(10),
            )
            .unwrap();
            assert!(output.status.success());
        }
        for path in [source, exe] {
            std::fs::remove_file(path).unwrap();
        }
    }
}
