//! Windows i386 argument slots retain four-byte alignment for wider objects.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn wide_arguments_follow_an_integer_or_hidden_return_pointer_without_padding() {
    let compiler = "i686-w64-mingw32-gcc";
    if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("windows-stack-alignment", "c");
    let binary = source.with_extension("dll");
    std::fs::write(&source, "struct wide { double real, imag; };\nstruct wide copy_wide(struct wide value) { return value; }\ndouble after_integer(int tag,double value) { return tag+value; }\ndouble pair_after_integer(int tag,struct wide value) { return tag+value.real+value.imag; }\n").unwrap();
    let output = Command::new(compiler)
        .args(["-O0", "-g", "-shared"])
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
    let specs = common::repo_root().join("specs");
    let mut code = String::from("typedef struct { double real,imag; } wide;\n");
    for name in ["copy_wide", "after_integer", "pair_after_integer"] {
        let (json, err, status) = common::run_kuna(&[
            "decompile",
            binary.to_str().unwrap(),
            name,
            "--json",
            "--sleighpath",
            specs.to_str().unwrap(),
        ]);
        assert_eq!(status, 0, "{err}");
        let json: serde_json::Value = serde_json::from_str(&json).unwrap();
        let function = &json["functions"][0];
        let value = function["variables"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["name"] == "value")
            .unwrap();
        assert_eq!(value["stack_offset"], 8, "{function}");
        code.push_str(function["code"].as_str().unwrap());
        code.push('\n');
    }
    code.push_str("int main(void) { wide input={0.5,-17.25}, output; copy_wide(&output,input); return output.real!=input.real || output.imag!=input.imag || after_integer(3,0.5)!=3.5 || pair_after_integer(3,input)!=-13.75; }\n");
    let regenerated = source.with_extension("roundtrip.c");
    let executable = source.with_extension("exe");
    std::fs::write(&regenerated, &code).unwrap();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let output = Command::new(compiler)
                .args(["-std=c11", "-fno-strict-aliasing", level])
                .arg(&regenerated)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}\n{code}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(
                Command::new(&executable).status().unwrap().success(),
                "{compiler} {level}\n{code}"
            );
        }
    }
    for path in [source, binary, regenerated, executable] {
        std::fs::remove_file(path).unwrap();
    }
}
