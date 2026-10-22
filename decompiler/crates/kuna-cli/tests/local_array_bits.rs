//! Numeric conversions retain their result bits in a recovered local byte array.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::process::Command;

#[test]
fn pe32_float_local_array_preserves_conversion() {
    check_target("i686-w64-windows-gnu", 4);
}
#[test]
fn pe64_float_local_array_preserves_conversion() {
    check_target("x86_64-w64-windows-gnu", 4);
}
#[test]
fn pe64_double_local_array_preserves_conversion() {
    check_target("x86_64-w64-windows-gnu", 8);
}

fn check_target(target: &str, width: usize) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("local-array-bitcopy", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(&source, include_str!("fixtures/local_array_bits/sample.c")).unwrap();
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .arg(format!("-DPAYLOAD_BYTES={width}"))
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
    let read = address(b"read_packet");
    let convert = address(b"convert_packet");
    let scalar = if width == 4 { "float" } else { "double" };
    let contracts = [
        format!("function 0x{read:x}=read_packet"),
        format!("function 0x{convert:x}=convert_packet"),
        format!(
            "typedef struct Packet {{unsigned int tag;unsigned char payload[{width}];}} Packet;"
        ),
        format!("prototype 0x{read:x} {scalar} __cdecl read_packet(Packet *packet)"),
        format!("prototype 0x{convert:x} {scalar} __cdecl convert_packet(int value)"),
        "type convert_packet::v1 Packet".to_owned(),
    ];
    let project = root.join("project");
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .arg("decompile-project")
        .arg(&binary)
        .args([
            "--addr",
            &format!("0x{read:x}"),
            "--addr",
            &format!("0x{convert:x}"),
            "--assert-strict",
            "--option",
            "stackviews",
            "on",
            "--sleighpath",
        ])
        .arg(common::repo_root().join("specs"))
        .arg("-o")
        .arg(&project);
    for contract in &contracts {
        command.args(["--assert", contract]);
    }
    process::required_output(&mut command);
    let generated = project.join("sample.dll.c");
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .args(["-std=gnu11", "-Werror", "-include", "stddef.h", "-c"])
            .arg(&generated)
            .arg("-o")
            .arg(root.join("generated.o")),
    );
    let driver = project.join("driver.c");
    std::fs::write(
        &driver,
        format!(
            r#"
#include "sample.dll.h"
#include <stdint.h>
#include <string.h>
int main(void) {{
    uint32_t state=0x3fa00000;
    const int special[]={{0,1,-1,2147483647,(-2147483647-1)}};
    for(unsigned i=0;i<4096;i++) {{
        state^=state<<13;state^=state>>17;state^=state<<5;
        int input;memcpy(&input,&state,4);if(i<5)input=special[i];
        {scalar} expected=({scalar})input,actual=convert_packet(input);
        if(memcmp(&actual,&expected,sizeof(actual)))return 1;
    }}return 0;
}}
"#
        ),
    )
    .unwrap();
    for optimization in ["-O0", "-O2"] {
        let executable = root.join(format!("check{optimization}"));
        process::required_output(
            Command::new("clang")
                .args(["-std=gnu11", optimization, "-include", "stddef.h"])
                .arg(&generated)
                .arg(&driver)
                .arg("-o")
                .arg(&executable),
        );
        process::required_output(&mut Command::new(executable));
    }
}
