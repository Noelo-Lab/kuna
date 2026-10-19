//! Qualified register declarations must bind identically by name and address.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn address_qualified_register_parameters_and_returns_reach_the_caller() {
    let compiler = "clang";
    if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("qualified-register-address", "s");
    let binary = source.with_extension("dll");
    std::fs::write(
        &source,
        r#"
.text
.globl _sample
.def _sample; .scl 2; .type 32; .endef
_sample:
 mov 4(%esp),%ecx
 call _adjust_value
 mov %edx,%eax
 ret
.globl _adjust_value
.def _adjust_value; .scl 2; .type 32; .endef
_adjust_value:
 lea 1(%ecx),%edx
 ret
"#,
    )
    .unwrap();
    let output = Command::new(compiler)
        .args([
            "--target=i686-w64-windows-gnu",
            "-shared",
            "-nostdlib",
            "-Wl,--entry,0",
            "-Wl,--export-all-symbols",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let (inventory, err, status) =
        common::run_kuna(&["functions", binary.to_str().unwrap(), "--json"]);
    assert_eq!(status, 0, "{err}");
    let inventory: serde_json::Value = serde_json::from_str(&inventory).unwrap();
    let helper = inventory["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"].as_str() == Some("_adjust_value"))
        .unwrap();
    let address = helper["address_hex"].as_str().unwrap();
    for declaration in [
        "param 0x1::0 ECX unsigned int value",
        "return 0x1::EDX unsigned int",
    ] {
        let (_, err, status) = common::run_kuna(&[
            "decompile",
            binary.to_str().unwrap(),
            "_sample",
            "--assert",
            declaration,
        ]);
        assert_eq!(status, 0, "rejected assertions remain warnings: {err}");
        assert!(err.contains("rejected: no function starts at 0x1"), "{err}");
    }
    let regenerated = source.with_extension("c");
    let executable = source.with_extension("exe");
    for (selector, return_selector, complete) in [
        ("_adjust_value", "_adjust_value", false),
        (address, address, false),
        ("_adjust_value", address, true),
        (address, "_adjust_value", true),
    ] {
        let param = format!("param {selector}::0 ECX unsigned int value");
        let result = format!("return {return_selector}::EDX unsigned int");
        for assertions in [
            [param.as_str(), result.as_str()],
            [result.as_str(), param.as_str()],
        ] {
            let prototype = format!(
                "prototype {selector} unsigned int __cdecl _adjust_value(unsigned int value)"
            );
            let mut args = vec![
                "decompile",
                binary.to_str().unwrap(),
                "_sample",
                "--assert",
                "prototype _sample unsigned int _sample(unsigned int x)",
            ];
            if complete {
                args.extend(["--assert", prototype.as_str()]);
            }
            args.extend(["--assert", assertions[0], "--assert", assertions[1]]);
            let (body, err, status) = common::run_kuna(&args);
            assert_eq!(status, 0, "{err}");
            assert!(
                body.contains("return _adjust_value(x);"),
                "{selector}: {body}"
            );
            std::fs::write(
                &regenerated,
                format!(
                    r#"
#include <assert.h>
#include <stdint.h>
static unsigned int _adjust_value(unsigned int x) {{ return x + 1u; }}
{body}
int main(void) {{
    uint32_t samples[] = {{0, 1, 0x7fffffff, 0x80000000, 0xffffffff}};
    for (unsigned i=0; i<sizeof(samples)/sizeof(samples[0]); ++i)
        assert(_sample(samples[i]) == samples[i] + 1u);
}}
"#
                ),
            )
            .unwrap();
            let mut ran = false;
            for cc in ["gcc", "clang"] {
                if process::optional_output(Command::new(cc).arg("--version")).is_none() {
                    continue;
                }
                for opt in ["-O0", "-O2"] {
                    let output = Command::new(cc)
                        .arg(opt)
                        .arg(&regenerated)
                        .arg("-o")
                        .arg(&executable)
                        .output()
                        .unwrap();
                    assert!(
                        output.status.success(),
                        "{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    assert!(Command::new(&executable).status().unwrap().success());
                    ran = true;
                }
            }
            assert!(ran, "a host C compiler is required for the semantic check");
        }
    }
    for path in [source, binary, regenerated, executable] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn same_named_helpers_keep_independent_address_qualified_storage() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let first = common::scratch_file("register-address-left", "s");
    let second = common::scratch_file("register-address-right", "s");
    let binary = first.with_extension("so");
    for (path, caller, input, output, amount) in [
        (&first, "sample_left", "ecx", "edx", 1),
        (&second, "sample_right", "eax", "ecx", 2),
    ] {
        std::fs::write(
            path,
            format!(
                r#"
.text
.globl {caller}
.type {caller},@function
{caller}:
 mov 4(%esp),%{input}
 call adjust_value
 mov %{output},%eax
 ret
.size {caller},.-{caller}
.local adjust_value
.type adjust_value,@function
adjust_value:
 lea {amount}(%{input}),%{output}
 ret
.size adjust_value,.-adjust_value
.section .note.GNU-stack,"",@progbits
"#
            ),
        )
        .unwrap();
    }
    let output = Command::new("gcc")
        .args(["-m32", "-shared", "-nostdlib"])
        .arg(&first)
        .arg(&second)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let (inventory, err, status) =
        common::run_kuna(&["functions", binary.to_str().unwrap(), "--json"]);
    assert_eq!(status, 0, "{err}");
    let inventory: serde_json::Value = serde_json::from_str(&inventory).unwrap();
    let mut helpers: Vec<_> = inventory["functions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["name"].as_str() == Some("adjust_value"))
        .collect();
    helpers.sort_by_key(|f| f["address"].as_u64().unwrap());
    assert_eq!(helpers.len(), 2, "{inventory}");
    let left = helpers[0]["address_hex"].as_str().unwrap();
    let right = helpers[1]["address_hex"].as_str().unwrap();
    let directives = [
        format!("param {left}::0 ECX unsigned int left_value"),
        format!("return {left}::EDX unsigned int"),
        format!("param {right}::0 EAX unsigned int right_value"),
        format!("return {right}::ECX unsigned int"),
    ];
    for caller in ["sample_left", "sample_right"] {
        let prototype = format!("prototype {caller} unsigned int {caller}(unsigned int x)");
        let mut args = vec![
            "decompile",
            binary.to_str().unwrap(),
            caller,
            "--assert",
            prototype.as_str(),
        ];
        for directive in &directives {
            args.extend(["--assert", directive.as_str()]);
        }
        let (body, err, status) = common::run_kuna(&args);
        assert_eq!(status, 0, "{err}");
        assert!(body.contains("return adjust_value(x);"), "{caller}: {body}");
    }
    for path in [first, second, binary] {
        std::fs::remove_file(path).unwrap();
    }
}
