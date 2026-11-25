//! Replay fixed 64-bit shifts under `armconstshift all` across independent compiler inputs.
use crate::common;
use std::process::Command;

#[test]
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn fixed_wide_shifts_execute_as_bitvectors() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let root = common::repo_root().join("tests/fixtures/wide-integer-emission");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("provenance.json")).unwrap()).unwrap();
    for object in manifest["objects"].as_array().unwrap() {
        let call = object["call"].as_bool().unwrap();
        let image = root.join(object["file"].as_str().unwrap());
        let mut functions = vec![
            (
                "shift31",
                "long long shift31(long long value, long long *echo)",
            ),
            ("shift63", "long long shift63(long long value)"),
            ("widened_shift31", "long long widened_shift31(int value)"),
        ];
        if call {
            functions.push(("shifted_call", "long long shifted_call(void)"));
        }
        let mut codes = Vec::new();
        let mut declarations = Vec::new();
        for (name, prototype) in functions {
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command.arg("decompile").arg(&image).arg(name).args([
                "--option",
                "armconstshift",
                "all",
                "--assert-strict",
                "--assert",
                &format!("prototype {name} {prototype}"),
            ]);
            if call {
                command.args([
                    "--assert",
                    "prototype source_value long long source_value(void)",
                ]);
            }
            let output = common::process::required_output(&mut command);
            let code = String::from_utf8(output.stdout).unwrap();
            assert!(!code.contains("more targets"), "{code}");
            declarations.push(code.split_once('{').unwrap().0.trim().to_string());
            codes.push(code);
        }
        let source = common::scratch_file("wide-integer", "c");
        let driver = source.with_extension("driver.c");
        let executable = source.with_extension("exe");
        std::fs::write(
            &source,
            format!(
                "#include <stdint.h>\n#include <stdbool.h>\nlong long source_value(void);\n{}",
                codes.join("\n")
            ),
        )
        .unwrap();
        let integer = declarations[0].split_once("shift31").unwrap().0.trim();
        let mut prelude = declarations
            .iter()
            .map(|decl| format!("{decl};\n"))
            .collect::<String>();
        let call_check = if call {
            prelude.push_str("static long long seed;static int calls;long long source_value(void){calls++;return seed;}\n");
            "seed=(long long)bits;calls=0;if((uint64_t)shifted_call()!=(bits<<31)||calls!=1)return 4;"
        } else {
            ""
        };
        std::fs::write(&driver, format!("#include <stdint.h>\n{prelude}\nint main(void){{uint64_t edges[]={{0,1,UINT64_MAX,INT64_MAX,UINT64_C(1)<<63,0x7fffffff,0x80000000,0xffffffff}};for(uint64_t i=0;i<10000;i++){{uint64_t bits=i<8?edges[i]:i*UINT64_C(11400714819323198485);{integer} value=({integer})bits,echo=0;if((uint64_t)shift31(value,&echo)!=(bits<<31)||(uint64_t)echo!=bits)return 1;if((uint64_t)shift63(value)!=(bits<<63))return 2;uint32_t word=(uint32_t)bits;if((uint64_t)widened_shift31((int32_t)word)!=((uint64_t)(int64_t)(int32_t)word<<31))return 5;{call_check}}}return 0;}}\n")).unwrap();
        for opt in ["-O0", "-O2"] {
            common::process::required_output(
                Command::new("cc")
                    .args([
                        "-std=c11",
                        opt,
                        "-Wall",
                        "-Wextra",
                        "-Werror",
                        "-Wno-parentheses",
                        "-Wuninitialized",
                        "-fno-lto",
                        "-fsanitize=undefined",
                        "-fno-sanitize-recover=undefined",
                    ])
                    .arg(&source)
                    .arg(&driver)
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
                "{codes:?}\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        for path in [source, driver, executable] {
            std::fs::remove_file(path).unwrap();
        }
    }
}

/// The default keeps upstream's compact shift on every object.
#[test]
fn fixed_wide_shifts_keep_their_spelling_by_default() {
    let root = common::repo_root().join("tests/fixtures/wide-integer-emission");
    for file in ["x86-64.elf", "aarch64.elf", "ppc64.elf", "mips64.elf"] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.arg("decompile").arg(root.join(file)).arg("shift63").args([
            "--assert-strict",
            "--assert",
            "prototype shift63 long long shift63(long long value)",
        ]);
        let code = String::from_utf8(common::process::required_output(&mut command).stdout).unwrap();
        assert!(code.contains("return value << 0x3f;"), "{file}: {code}");
        assert!(!code.contains("unsigned long"), "{file}: {code}");
    }
}
