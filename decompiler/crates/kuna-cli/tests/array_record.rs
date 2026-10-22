//! A local record keeps its complete declaration when its first member is an array.
#![cfg(target_os = "linux")]

use crate::common;
use common::process;
use object::Object;
use std::process::Command;

#[test]
fn pe32_declared_record_retains_leading_byte_array() {
    check_target("i686-w64-windows-gnu");
}

#[test]
fn pe64_declared_record_retains_leading_byte_array() {
    check_target("x86_64-w64-windows-gnu");
}

fn check_target(target: &str) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("array-record", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(&source, include_str!("fixtures/array_record/sample.c")).unwrap();
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
    let read = address(b"read_packet");
    let wrap = address(b"wrap_word");
    let contracts = [
        format!("function 0x{read:x}=read_packet"),
        format!("function 0x{wrap:x}=wrap_word"),
        "typedef struct BytePacket {unsigned char payload[4];void *context;} BytePacket;"
            .to_owned(),
        format!("prototype 0x{read:x} float __cdecl read_packet(BytePacket *packet)"),
        format!("prototype 0x{wrap:x} float __cdecl wrap_word(unsigned int bits)"),
        "type wrap_word::v1 BytePacket".to_owned(),
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
            &format!("0x{wrap:x}"),
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
        r#"
#include "sample.dll.h"
#include <stdint.h>
#include <string.h>
int main(void) {
    uint32_t state = 0x3fa00000;
    const uint32_t special[] = {0,0x80000000,0x3fa00000,0xbf400000};
    for (unsigned i=0;i<4096;++i) {
        state ^= state << 13; state ^= state >> 17; state ^= state << 5;
        uint32_t bits = i<4 ? special[i] : state & 0xff7fffff;
        float result=wrap_word(bits); uint32_t actual;
        memcpy(&actual,&result,4);
        if (actual!=bits) return 1;
    }
    return 0;
}
"#,
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
