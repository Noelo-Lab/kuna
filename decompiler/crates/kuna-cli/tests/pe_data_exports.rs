#![cfg(target_os = "linux")]

use crate::common;
use common::process;
use object::Object;
use std::process::Command;

#[test]
fn pe32_exported_bytes_remain_data() {
    check_target("i686-w64-windows-gnu");
}

#[test]
fn pe64_exported_bytes_remain_data() {
    check_target("x86_64-w64-windows-gnu");
}

fn check_target(target: &str) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("pe-data-export", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(
        &source,
        r#"
__declspec(dllexport) unsigned char exported_bytes[16];
__declspec(dllexport) void write_byte(void) { exported_bytes[0] = 7; }
"#,
    )
    .unwrap();
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .args(["-shared", "-nostdlib", "-O0", "-Wl,--entry,0"])
            .arg(&source)
            .arg("-o")
            .arg(&binary),
    );
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let exports = image.exports().unwrap();
    let address = |name: &[u8]| {
        exports
            .iter()
            .find(|entry| entry.name() == name)
            .unwrap()
            .address()
    };
    let data = address(b"exported_bytes");
    let code = address(b"write_byte");
    let (inventory, stderr, status) =
        common::run_kuna(&["functions", binary.to_str().unwrap(), "--json"]);
    assert_eq!(status, 0, "{stderr}");
    let inventory: serde_json::Value = serde_json::from_str(&inventory).unwrap();
    let functions = inventory["functions"].as_array().unwrap();
    assert!(
        !functions
            .iter()
            .any(|entry| entry["address"].as_u64() == Some(data)),
        "{inventory}"
    );
    let name = functions
        .iter()
        .find(|entry| entry["address"].as_u64() == Some(code))
        .unwrap()["name"]
        .as_str()
        .unwrap();
    let (xrefs, stderr, status) =
        common::run_kuna(&["xrefs", binary.to_str().unwrap(), "--to", "exported_bytes"]);
    assert_eq!(status, 0, "{stderr}");
    assert!(xrefs.contains(&format!("exported_bytes @ 0x{data:x}")), "{xrefs}");
    let project = root.join("project");
    let declaration = format!("data 0x{data:x} unsigned char exported_bytes[16]");
    let (_, stderr, status) = common::run_kuna(&[
        "decompile-project",
        binary.to_str().unwrap(),
        "--addr",
        &format!("0x{code:x}"),
        "--assert",
        &declaration,
        "--assert-strict",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{stderr}");
    let header = std::fs::read_to_string(project.join("sample.dll.h")).unwrap();
    assert!(
        header.contains("extern unsigned char exported_bytes[16];"),
        "{header}"
    );
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .args(["-std=gnu11", "-Werror", "-c"])
            .arg(project.join("sample.dll.c"))
            .arg("-o")
            .arg(root.join("generated.o")),
    );
    let driver = project.join("driver.c");
    std::fs::write(
        &driver,
        format!(
            r#"
#include "sample.dll.h"
unsigned char exported_bytes[16];
int main(void) {{
    for (unsigned i=0;i<16;++i) exported_bytes[i]=(unsigned char)(23+i);
    {name}();
    if (exported_bytes[0]!=7) return 1;
    for (unsigned i=1;i<16;++i) if (exported_bytes[i]!=(unsigned char)(23+i)) return 2;
    return 0;
}}
"#
        ),
    )
    .unwrap();
    for optimization in ["-O0", "-O2"] {
        let executable = root.join(format!("check{optimization}"));
        process::required_output(
            Command::new("clang")
                .args(["-std=gnu11", optimization])
                .arg(project.join("sample.dll.c"))
                .arg(&driver)
                .arg("-o")
                .arg(&executable),
        );
        process::required_output(&mut Command::new(executable));
    }
}
