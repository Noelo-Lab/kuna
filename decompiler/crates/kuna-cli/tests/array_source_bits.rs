//! Array-typed memory reads retain their bytes in a declared frame member.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::process::Command;

#[test]
fn pe32_float_array_source_preserves_bits() {
    check_target("i686-w64-windows-gnu", 4);
}
#[test]
fn pe64_float_array_source_preserves_bits() {
    check_target("x86_64-w64-windows-gnu", 4);
}
#[test]
fn pe32_double_array_source_preserves_bits() {
    check_target("i686-w64-windows-gnu", 8);
}
#[test]
fn pe64_double_array_source_preserves_bits() {
    check_target("x86_64-w64-windows-gnu", 8);
}

fn check_target(target: &str, width: usize) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("array-source-bits", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(&source, include_str!("fixtures/array_source_bits/sample.c")).unwrap();
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
    let read = address(b"read_word");
    let mix = address(b"mix_words");
    let scalar = if width == 4 { "float" } else { "double" };
    let contracts=[format!("function 0x{read:x}=read_word"),format!("function 0x{mix:x}=mix_words"),
        format!("typedef struct Packet {{unsigned int tag;unsigned char payload[{width}];void *context;}} Packet;"),
        format!("prototype 0x{read:x} unsigned int __cdecl read_word(Packet *packet)"),
        format!("prototype 0x{mix:x} unsigned int __cdecl mix_words({scalar} *values)"),"type mix_words::v2 Packet".to_owned()];
    let project = root.join("project");
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .arg("decompile-project")
        .arg(&binary)
        .args([
            "--addr",
            &format!("0x{read:x}"),
            "--addr",
            &format!("0x{mix:x}"),
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
    let lanes = width / 4;
    let mask = if width == 4 {
        "0xff7fffff"
    } else {
        "0xffefffff"
    };
    std::fs::write(
        &driver,
        format!(
            r#"
#include "sample.dll.h"
#include <stdint.h>
#include <string.h>
int main(void) {{
    uint32_t state=0x3fa00000;
    for(unsigned i=0;i<4096;i++) {{
        uint32_t words[4]={{0}};
        for(unsigned j=0;j<{lanes}*2;j++) {{
            state^=state<<13;state^=state>>17;state^=state<<5;
            words[j]=state;
            if(j%{lanes}=={lanes}-1)words[j]&={mask};
            if(i==0)words[j]=0;
            if(i==1)words[j]=(j%{lanes}=={lanes}-1)?0x80000000:0;
        }}
        {scalar} values[2];memcpy(values,words,sizeof(values));
        uint32_t first=words[0],second=words[{lanes}];
        if({lanes}==2){{first^=words[1];second^=words[3];}}
        if(mix_words(values)!=(first*33U ^ second))return 1;
    }}return 0;
}}
"#
        ),
    )
    .unwrap();
    for optimization in ["-O0", "-O2"] {
        let exe = root.join(format!("check{optimization}"));
        process::required_output(
            Command::new("clang")
                .args(["-std=gnu11", optimization, "-include", "stddef.h"])
                .arg(&generated)
                .arg(&driver)
                .arg("-o")
                .arg(&exe),
        );
        process::required_output(&mut Command::new(exe));
    }
}
