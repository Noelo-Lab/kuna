//! Replay a widened four-byte addition under `wrapadd on` across independent compiler inputs.
use crate::common;
use std::process::Command;

fn decompile(image: &std::path::Path, option: Option<&str>) -> String {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.arg("decompile").arg(image).arg("word_wrap");
    if let Some(value) = option {
        command.args(["--option", "wrapadd", value]);
    }
    command.args(["--assert-strict", "--assert", "prototype word_wrap long long word_wrap(int value)"]);
    String::from_utf8(common::process::required_output(&mut command).stdout).unwrap()
}

#[test]
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn widened_word_additions_wrap_like_the_machine() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let root = common::repo_root().join("tests/fixtures/wide-integer-emission");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("provenance.json")).unwrap()).unwrap();
    for object in manifest["objects"].as_array().unwrap() {
        let image = root.join(object["file"].as_str().unwrap());
        let code = decompile(&image, Some("on"));
        let source = common::scratch_file("wrap-add", "c");
        let driver = source.with_extension("driver.c");
        let executable = source.with_extension("exe");
        std::fs::write(&source, format!("#include <stdint.h>\n#include <stdbool.h>\n{code}")).unwrap();
        let declaration = code.split_once('{').unwrap().0.trim();
        std::fs::write(&driver, format!("#include <stdint.h>\n{declaration};\nint main(void){{uint32_t edges[]={{0,1,0x7fffffffu,0x80000000u,0xfffffffeu,0xffffffffu}};for(uint32_t i=0;i<10000;i++){{uint32_t word=i<6?edges[i]:i*2654435761u;if(word_wrap((int32_t)word)!=(int64_t)(int32_t)(word+1u))return 1;}}return 0;}}\n")).unwrap();
        for opt in ["-O0", "-O2"] {
            common::process::required_output(
                Command::new("cc")
                    .args(["-std=c11", opt, "-Wall", "-Wextra", "-Werror", "-Wno-parentheses",
                        "-Wuninitialized", "-fno-lto", "-fsanitize=undefined",
                        "-fno-sanitize-recover=undefined"])
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
            assert!(output.status.success(), "{code}\n{}", String::from_utf8_lossy(&output.stderr));
        }
        for path in [source, driver, executable] {
            std::fs::remove_file(path).unwrap();
        }
        let default = decompile(&image, None);
        assert!(!default.contains("(unsigned int)value"), "{default}");
    }
}
