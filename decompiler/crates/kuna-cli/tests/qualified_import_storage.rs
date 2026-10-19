//! Imported thunk declarations must not update a same-named IAT sibling.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn imported_register_storage_reaches_the_executable_thunk() {
    let compiler = "clang";
    if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
        return;
    }
    let provider = common::scratch_file("register-import-provider", "s");
    let library = provider.with_extension("dll");
    let import_library = provider.with_extension("a");
    let caller = common::scratch_file("register-import-caller", "s");
    let binary = caller.with_extension("dll");
    std::fs::write(
        &provider,
        r#"
.text
.globl __double_value
.def __double_value; .scl 2; .type 32; .endef
__double_value:
 add %eax,%eax
 ret
"#,
    )
    .unwrap();
    std::fs::write(
        &caller,
        r#"
.text
.globl _sample
.def _sample; .scl 2; .type 32; .endef
_sample:
 mov 4(%esp),%eax
 call __double_value
 ret
.globl _sample_indirect
.def _sample_indirect; .scl 2; .type 32; .endef
_sample_indirect:
 mov 4(%esp),%eax
 call *__imp___double_value
 ret
"#,
    )
    .unwrap();
    let result = Command::new(compiler)
        .args([
            "--target=i686-w64-windows-gnu",
            "-shared",
            "-nostdlib",
            "-Wl,--entry,0",
            "-Wl,--export-all-symbols",
        ])
        .arg(format!("-Wl,--out-implib,{}", import_library.display()))
        .arg(&provider)
        .arg("-o")
        .arg(&library)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let result = Command::new(compiler)
        .args([
            "--target=i686-w64-windows-gnu",
            "-shared",
            "-nostdlib",
            "-Wl,--entry,0",
            "-Wl,--export-all-symbols",
        ])
        .arg(&caller)
        .arg(&import_library)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let (inventory, err, status) =
        common::run_kuna(&["functions", binary.to_str().unwrap(), "--json"]);
    assert_eq!(status, 0, "{err}");
    let inventory: serde_json::Value = serde_json::from_str(&inventory).unwrap();
    let helpers: Vec<_> = inventory["functions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["name"].as_str() == Some("_double_value"))
        .collect();
    assert_eq!(
        helpers.len(),
        2,
        "fixture must have an executable thunk and an IAT sibling"
    );
    let thunk = helpers
        .iter()
        .find(|f| f["size"].as_u64().unwrap() > 0)
        .unwrap()["address_hex"]
        .as_str()
        .unwrap();
    let slot = helpers
        .iter()
        .find(|f| f["size"].as_u64() == Some(0))
        .unwrap()["address_hex"]
        .as_str()
        .unwrap();
    for (caller_name, selector) in [
        ("_sample", "_double_value"),
        ("_sample", thunk),
        ("_sample_indirect", slot),
    ] {
        for complete in [false, true] {
            for reverse in [false, true] {
                let caller_proto =
                    format!("prototype {caller_name} unsigned int {caller_name}(unsigned int x)");
                let mut args = vec![
                    "decompile",
                    binary.to_str().unwrap(),
                    caller_name,
                    "--assert",
                    &caller_proto,
                ];
                let prototype =
                    format!("prototype {selector} unsigned int _double_value(unsigned int value)");
                if complete {
                    args.extend(["--assert", &prototype]);
                }
                let mut declarations = [
                    format!("param {selector}::0 EAX unsigned int value"),
                    format!("return {selector}::EAX unsigned int"),
                ];
                if reverse {
                    declarations.reverse();
                }
                for declaration in &declarations {
                    args.extend(["--assert", declaration.as_str()]);
                }
                let (body, err, status) = common::run_kuna(&args);
                assert_eq!(status, 0, "{err}");
                assert!(
                    body.contains("_double_value(x)"),
                    "caller={caller_name} selector={selector} complete={complete} reverse={reverse}: {body}"
                );
                let body = body.replace(&format!("{caller_name}("), "_sample(");
                let recovered = caller.with_extension("c");
                std::fs::write(&recovered,format!("static unsigned int _double_value(unsigned int x) {{ return x+x; }}\n{body}\nint main(void) {{ unsigned int cases[]={{0,1,0x80000000u,0xffffffffu,0x12345678u}}; for(unsigned i=0;i<5;i++) if(_sample(cases[i]) != cases[i]*2u) return 1; return 0; }}\n")).unwrap();
                for cc in ["gcc", "clang"] {
                    if process::optional_output(Command::new(cc).arg("--version")).is_none() {
                        continue;
                    }
                    for level in ["-O0", "-O2"] {
                        let executable = caller.with_extension("exe");
                        let result = Command::new(cc)
                            .args(["-std=c11", "-Wall", "-Werror", level])
                            .arg(&recovered)
                            .arg("-o")
                            .arg(&executable)
                            .output()
                            .unwrap();
                        assert!(
                            result.status.success(),
                            "{}",
                            String::from_utf8_lossy(&result.stderr)
                        );
                        assert!(Command::new(&executable).status().unwrap().success());
                    }
                }
            }
        }
    }
}
