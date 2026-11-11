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

#[test]
fn pe32_direct_mapped_load_preserves_all_bits() {
    check_case("i686-w64-windows-gnu", 4, true);
}
#[test]
fn pe64_direct_mapped_load_preserves_all_bits() {
    check_case("x86_64-w64-windows-gnu", 4, true);
}

fn check_target(target: &str, width: usize) {
    check_case(target, width, false);
}
fn check_case(target: &str, width: usize, direct: bool) {
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
    if direct {
        let text = std::fs::read_to_string(&generated).unwrap();
        let prefix = text.split("// Function: mix_words @").next().unwrap();
        let body = direct_load_body(&binary, read, mix);
        std::fs::write(&generated, format!("{prefix}\n{body}")).unwrap();
    }

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
    let raw = if direct { 1 } else { 0 };
    let mask = if direct {
        "0xffffffff"
    } else if width == 4 {
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
        if({raw} && i==2){{words[0]=0x7f800001;words[1]=0x7fc01234;}}
        if({raw} && i==3){{words[0]=0x7f800000;words[1]=0xff800000;}}
        if({raw} && i==4){{words[0]=0x7f7fffff;words[1]=0x00800000;}}
        {scalar} values[2];memcpy(values,words,sizeof(values));
        uint32_t first=words[0],second=words[{lanes}];
        if({lanes}==2){{first=first*257U ^ words[1];second=second*257U ^ words[3];}}
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

// Coalescing can make the memory read itself define the mapped frame word.
// Preserve the fresh fixture's storage and pointer, then render that state.
fn direct_load_body(binary: &std::path::Path, read: u64, mix: u64) -> String {
    use kuna_console::ifacedecomp::{
        execute, register_console_commands, register_decomp_commands, IfaceDecompData,
        DECOMPILE_MODULE,
    };
    use kuna_console::ifaceterm::IfaceTerm;
    use kuna_console::kuna_console::register_kuna_commands;
    use kuna_num::opcodes::OpCode;
    let script=format!("load file {}\noption stackviews on\nparse line typedef struct Packet {{unsigned int tag;unsigned char payload[4];void *context;}} Packet;\nmap function 0x{read:x} read_word\nmap function 0x{mix:x} mix_words\nmap prototype 0x{read:x} unsigned int __cdecl read_word(Packet *packet)\nmap prototype 0x{mix:x} unsigned int __cdecl mix_words(float *values)\nload function 0x{mix:x}\ndecompile\nretype v2 Packet\ndecompile\n",binary.display());
    let mut status = IfaceTerm::into_status("", script.as_bytes());
    register_decomp_commands(&mut status);
    register_kuna_commands(&mut status);
    register_console_commands(&mut status);
    let data = status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap();
    data.spec_roots = vec![common::repo_root()
        .join("specs")
        .to_string_lossy()
        .into_owned()];
    while !status.is_stream_finished() && !status.done {
        execute(&mut status);
        assert!(!status.is_in_error(), "{}", status.optr);
        status.optr.clear();
    }
    let data = status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap();
    let fd = data.fd.as_mut().unwrap();
    let store = fd
        .obank()
        .iter_alive()
        .find(|&id| {
            let Some(op) = fd.obank().get(id) else {
                return false;
            };
            if op.code() != OpCode::CPUI_COPY {
                return false;
            }
            let Some(value) = op.get_out().and_then(|vn| fd.vbank().get(vn)) else {
                return false;
            };
            value.get_size() == 4
                && value
                    .get_high()
                    .and_then(|h| fd.high_bank().get(h))
                    .is_some_and(|h| h.kuna_symbol_offset() == 4 && h.kuna_link_symbol().is_some())
        })
        .expect("payload copy");
    let mut source = fd.obank().get(store).unwrap().get_in(0).unwrap();
    for _ in 0..16 {
        let definition = fd
            .vbank()
            .get(source)
            .unwrap()
            .get_def()
            .expect("implied source definition");
        let operation = fd.obank().get(definition).unwrap();
        if !matches!(operation.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            break;
        }
        source = operation.get_in(0).unwrap();
    }
    let load = fd.vbank().get(source).unwrap().get_def().unwrap();
    let operation = fd.obank().get(load).unwrap();
    assert_eq!(operation.code(), OpCode::CPUI_LOAD);
    let inputs = [operation.get_in(0).unwrap(), operation.get_in(1).unwrap()];
    fd.op_set_opcode(
        store,
        kuna_decomp::context::TypeOp::new(OpCode::CPUI_LOAD, 0, "LOAD"),
    );
    fd.op_set_all_input(store, &inputs).unwrap();
    let arch = data.conf.as_mut().unwrap().arch_mut();
    arch.realtypes = true;
    arch.ctypes = true;
    arch.with_print(|printer, arch| {
        printer.options.partial_access = true;
        printer.doc_function_full(fd, arch)
    })
}
