//! Execute weak-call output with the original declared ABI and both bindings.
use crate::common;
use std::collections::BTreeSet;
use std::process::Command;

#[test]
#[cfg(target_os = "linux")]
fn declared_weak_calls_preserve_null_guards_arguments_and_results() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let mut images = Vec::new();
    for compiler in ["47", "73"] {
        for abi in ["n32", "64"] {
            images.push((format!("octeon-relocations/pic-gcc{compiler}-{abi}.o"), None));
        }
    }
    for (abi, little) in [("n32", "MIPS:LE:64:64-32addr"), ("64", "MIPS:LE:64:default")] {
        images.push((format!("mips-weak-imports/page-{abi}-be.o"), None));
        images.push((format!("mips-weak-imports/page-{abi}-le.o"), Some(little)));
    }
    {
        for (name, target) in images {
            let image = common::repo_root().join("tests/fixtures").join(name);
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command.args([
                "decompile",
                image.to_str().unwrap(),
                "call_optional",
                "--assert-strict",
                "--assert",
                "prototype call_optional unsigned int call_optional(unsigned int x)",
                "--assert",
                "prototype optional unsigned int optional(unsigned int x)",
            ]);
            if let Some(target) = target {
                command.args(["--target", target]);
            }
            let out = common::process::required_output(&mut command);
            let code = String::from_utf8(out.stdout).unwrap();
            let slots: BTreeSet<_> = code
                .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .filter(|name| name.starts_with("dat_"))
                .collect();
            assert_eq!(slots.len(), 1, "one weak import slot: {code}");
            for bound in [false, true] {
                let source = common::scratch_file("weak-call", "c");
                let driver = source.with_extension("driver.c");
                let executable = source.with_extension("exe");
                let declarations = slots
                    .iter()
                    .map(|name| format!("extern uintptr_t {name};\n"))
                    .collect::<String>();
                std::fs::write(&source, format!("#include <stdint.h>\nextern unsigned int optional(unsigned int) __attribute__((weak));\n{declarations}{code}")).unwrap();
                let definitions = slots
                    .iter()
                    .map(|name| format!("uintptr_t {name};\n"))
                    .collect::<String>();
                let initialize = slots
                    .iter()
                    .map(|name| format!("{name}=(uintptr_t)optional;\n"))
                    .collect::<String>();
                let helper = if bound {
                    "unsigned int optional(unsigned int x) { calls++; last=x; return x^0x12345678u; }"
                } else {
                    ""
                };
                let expected = if bound { "0x12345678u" } else { "0x42u" };
                let count = if bound { "i+1" } else { "0" };
                let argument = if bound { " || last!=x" } else { "" };
                std::fs::write(&driver, format!("#include <stdint.h>\nextern unsigned int optional(unsigned int) __attribute__((weak));\nunsigned int call_optional(unsigned int);\nunsigned calls,last;\n{definitions}{helper}\nint main(void) {{ {initialize}for(uint32_t i=0;i<10000;i++) {{uint32_t x=i*2654435761u^0x89123456u; uint32_t y=call_optional(x); if(y!=(x^{expected}) || calls!={count}{argument}) return 1; }} return 0; }}\n")).unwrap();
                common::process::required_output(
                    Command::new("cc")
                        .args([
                            "-std=c11",
                            "-O2",
                            "-fno-lto",
                            "-Wall",
                            "-Werror",
                            "-Wuninitialized",
                        ])
                        .arg(&source)
                        .arg(&driver)
                        .arg("-o")
                        .arg(&executable),
                );
                common::process::required_output(&mut Command::new(&executable));
                for file in [source, driver, executable] {
                    std::fs::remove_file(file).unwrap();
                }
            }
        }
    }
}
