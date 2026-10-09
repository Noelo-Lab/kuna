//! A signed stack index must not forward the initializer past the indexed writes.

mod common;
use common::process;
use std::process::Command;

#[test]
fn emitted_signed_stack_index_matches_the_stored_word() {
    let xml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/stages/kuna-signed-stack-index.xml"
    ));
    let chunk = regex::Regex::new(r#"<bytechunk[^>]*>([0-9a-f]+)</bytechunk>"#).unwrap();
    let hex = &chunk.captures(xml).unwrap()[1];
    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap())
        .collect();
    let binary = common::scratch_file("signed-stack-index", "bin");
    std::fs::write(&binary, bytes).unwrap();
    let (stdout, stderr, status) = common::run_kuna(&[
        "decompile",
        binary.to_str().unwrap(),
        "0x1000",
        "--raw-image",
        "--base",
        "0x1000",
        "--target",
        "sparc:BE:32:default:default",
        "--assert",
        "function 0x1000=signed_stack_index",
        "--assert",
        "prototype signed_stack_index unsigned int signed_stack_index(unsigned int seed)",
        "--assert-strict",
        "--json",
    ]);
    assert_eq!(status, 0, "{stderr}");
    let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(result["functions"][0]["error"].is_null(), "{stdout}");
    let code = result["functions"][0]["code"].as_str().unwrap();
    assert!(!code.contains("return 0;"), "{code}");
    let program = format!(
        r#"
{code}
int main(void) {{
    unsigned int seeds[] = {{0, 1, 2047, 2048, 0xffffffffu, 0x80000000u}};
    for (unsigned int i = 0; i < sizeof(seeds) / sizeof(seeds[0]); ++i)
        if (signed_stack_index(seeds[i]) != (seeds[i] & 2047u)) return 1;
    return 0;
}}
"#
    );
    let source = common::scratch_file("signed-stack-index", "c");
    std::fs::write(&source, program).unwrap();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            let executable = common::scratch_file("signed-stack-index", "exe");
            let compiled = Command::new(compiler)
                .args([optimization, "-std=c11"])
                .arg(&source)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                compiled.status.success(),
                "{compiler} {optimization}: {}",
                String::from_utf8_lossy(&compiled.stderr)
            );
            let run = Command::new(&executable).output().unwrap();
            assert!(
                run.status.success(),
                "{compiler} {optimization}: {}",
                String::from_utf8_lossy(&run.stderr)
            );
        }
    }
}
