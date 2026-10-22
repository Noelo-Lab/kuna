//! Floating values retain their bits when copied into a declared byte-array member.
#![cfg(target_os = "linux")]

use crate::common;
use common::process;
use object::Object;
use std::process::Command;

#[test]
fn pe32_float_byte_member_preserves_bits() {
    check_target("i686-w64-windows-gnu", 4, 4);
}
#[test]
fn pe64_float_byte_member_preserves_bits() {
    check_target("x86_64-w64-windows-gnu", 4, 4);
}
#[test]
fn pe32_double_byte_member_preserves_bits() {
    check_target("i686-w64-windows-gnu", 8, 8);
}
#[test]
fn pe64_double_byte_member_preserves_bits() {
    check_target("x86_64-w64-windows-gnu", 8, 8);
}

#[test]
fn pe32_float_partial_byte_member_preserves_bits() {
    check_target("i686-w64-windows-gnu", 4, 12);
}
#[test]
fn pe64_float_partial_byte_member_preserves_bits() {
    check_target("x86_64-w64-windows-gnu", 4, 12);
}
#[test]
fn pe32_double_partial_byte_member_preserves_bits() {
    check_target("i686-w64-windows-gnu", 8, 16);
}
#[test]
fn pe64_double_partial_byte_member_preserves_bits() {
    check_target("x86_64-w64-windows-gnu", 8, 16);
}

#[test]
fn pe32_double_register_halves_preserve_bits() {
    check_target_cpu("i686-w64-windows-gnu", "i686", 8, 8);
}
#[test]
fn pe32_double_partial_register_halves_preserve_bits() {
    check_target_cpu("i686-w64-windows-gnu", "i686", 8, 16);
}

/// i686 pins `-march=pentium4`, the default of current clang, so older clang also copies a
/// double through SSE; the `register_halves` cases pin `-march=i686`, which copies it as
/// two general-register words.
fn check_target(target: &str, width: usize, capacity: usize) {
    let cpu = if target.starts_with("i686") { "pentium4" } else { "x86-64" };
    check_target_cpu(target, cpu, width, capacity);
}

fn check_target_cpu(target: &str, cpu: &str, width: usize, capacity: usize) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("array-member-bits", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(&source, include_str!("fixtures/array_member_bits/sample.c")).unwrap();
    process::required_output(
        Command::new("clang")
            .arg(format!("--target={target}"))
            .arg(format!("-DPAYLOAD_BYTES={width}"))
            .arg(format!("-DPAYLOAD_CAPACITY={capacity}"))
            .arg(format!("-march={cpu}"))
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
    let wrap = address(b"wrap_float");
    let scalar = if width == 4 { "float" } else { "double" };
    let contracts = [
        format!("function 0x{read:x}=read_packet"),
        format!("function 0x{wrap:x}=wrap_float"),
        format!(
            "typedef struct Packet {{unsigned int tag;unsigned char payload[{capacity}];}} Packet;"
        ),
        format!("prototype 0x{read:x} {scalar} __cdecl read_packet(Packet *packet)"),
        format!("prototype 0x{wrap:x} {scalar} __cdecl wrap_float({scalar} value)"),
        "type wrap_float::v1 Packet".to_owned(),
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
    let driver_body = if width == 4 {
        r#"
#include "sample.dll.h"
#include <stdint.h>
#include <string.h>
int main(void) {
    uint32_t state=0x3fa00000;
    const uint32_t special[]={0,0x80000000,0x3fa00000,0xbf400000};
    for(unsigned i=0;i<4096;i++) {
        state^=state<<13;state^=state>>17;state^=state<<5;
        uint32_t bits=i<4?special[i]:state&0xff7fffff,actual;
        float input;memcpy(&input,&bits,4);
        float result=wrap_float(input);memcpy(&actual,&result,4);
        if(actual!=bits)return 1;
    }return 0;
}
"#
    } else {
        r#"
#include "sample.dll.h"
#include <stdint.h>
#include <string.h>
int main(void) {
    uint64_t state=UINT64_C(0x3ff4000000000000);
    const uint64_t special[]={0,UINT64_C(0x8000000000000000),UINT64_C(0x3ff4000000000000),UINT64_C(0xbfe8000000000000)};
    for(unsigned i=0;i<4096;i++) {
        state^=state<<13;state^=state>>7;state^=state<<17;
        uint64_t bits=i<4?special[i]:state&UINT64_C(0xffefffffffffffff),actual;
        double input;memcpy(&input,&bits,8);
        double result=wrap_float(input);memcpy(&actual,&result,8);
        if(actual!=bits)return 1;
    }return 0;
}
"#
    };
    std::fs::write(&driver, driver_body).unwrap();
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
