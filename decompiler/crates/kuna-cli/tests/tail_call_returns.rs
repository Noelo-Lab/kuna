//! `passthrough` hands back the result of a tail call to a callee with no
//! parameters (x86-64 `jmp getv`, ARM `b getv`, AArch64 `b getv`) the way it
//! does for a callee that takes one, and on x86-64 also when the function wrote
//! the return register before the call (`mov (%rdi),%rax; mov %eax,%edi; jmp
//! getk`), which the call overwrites. A write to an argument register that is
//! also the return register (ARM `mov r0,#5; b getk`) is not relaxed.
mod common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

/// One function: its bytes, and the branches in it to patch, as `(offset,
/// target)`.
struct Func {
    name: &'static str,
    code: Vec<u8>,
    branches: Vec<(usize, &'static str)>,
}

fn func(name: &'static str, code: Vec<u8>, branches: &[(usize, &'static str)]) -> Func {
    Func { name, code, branches: branches.to_vec() }
}

fn words(code: &[u32]) -> Vec<u8> {
    code.iter().flat_map(|w| w.to_le_bytes()).collect()
}

/// A relocatable object whose `.text` is `funcs` back to back, each branch
/// encoded by `patch(code, at, from, to)` against those offsets.
fn object(arch: Architecture, funcs: &[Func], patch: impl Fn(&mut [u8], usize, u64, u64)) -> Vec<u8> {
    let mut text = Vec::new();
    let mut at = Vec::new();
    for f in funcs {
        at.push(text.len() as u64);
        text.extend_from_slice(&f.code);
    }
    let start = |name: &str| at[funcs.iter().position(|f| f.name == name).unwrap()];
    for (f, &base) in funcs.iter().zip(&at) {
        for &(off, target) in &f.branches {
            let from = base + off as u64;
            patch(&mut text, from as usize, from, start(target));
        }
    }
    let mut object = Object::new(BinaryFormat::Elf, arch, Endianness::Little);
    let section = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    object.append_section_data(section, &text, 16);
    for (f, &value) in funcs.iter().zip(&at) {
        object.add_symbol(Symbol {
            name: f.name.as_bytes().to_vec(),
            value,
            size: f.code.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    object.write().unwrap()
}

fn x86_image() -> Vec<u8> {
    let rel32 = [0, 0, 0, 0];
    let funcs = [
        // mov $42,%eax; ret
        func("getv", vec![0xb8, 0x2a, 0, 0, 0, 0xc3], &[]),
        // jmp getv
        func("f", [&[0xe9][..], &rel32].concat(), &[(0, "getv")]),
        // lea (%rdi,%rdi,2),%eax; ret
        func("getk", vec![0x8d, 0x04, 0x7f, 0xc3], &[]),
        // mov (%rdi),%rax; mov %eax,%edi; jmp getk
        func("fk", [&[0x48, 0x8b, 0x07, 0x89, 0xc7, 0xe9][..], &rel32].concat(), &[(5, "getk")]),
        // call getv; mov %eax,%edi; jmp getk
        func(
            "twice",
            [&[0xe8][..], &rel32, &[0x89, 0xc7, 0xe9], &rel32].concat(),
            &[(0, "getv"), (7, "getk")],
        ),
        // call getv; mov $7,%eax; ret
        func("after", [&[0xe8][..], &rel32, &[0xb8, 7, 0, 0, 0, 0xc3]].concat(), &[(0, "getv")]),
        // mov %esi,(%rdi); ret
        func("sink", vec![0x89, 0x37, 0xc3], &[]),
        // jmp sink
        func("fs", [&[0xe9][..], &rel32].concat(), &[(0, "sink")]),
    ];
    object(Architecture::X86_64, &funcs, |text, at, from, to| {
        let rel = (to as i64 - (from as i64 + 5)) as i32;
        text[at + 1..at + 5].copy_from_slice(&rel.to_le_bytes());
    })
}

fn arm_image() -> Vec<u8> {
    let funcs = [
        // mov r0,#42; bx lr
        func("getv", words(&[0xe3a0002a, 0xe12fff1e]), &[]),
        // b getv
        func("f", words(&[0xea000000]), &[(0, "getv")]),
        // add r0,r0,r0,lsl #1; bx lr
        func("getk", words(&[0xe0800080, 0xe12fff1e]), &[]),
        // mov r0,#5; b getk
        func("fc", words(&[0xe3a00005, 0xea000000]), &[(4, "getk")]),
    ];
    object(Architecture::Arm, &funcs, |text, at, from, to| {
        let word = 0xea000000u32 | (((to as i64 - from as i64 - 8) >> 2) as u32 & 0xffffff);
        text[at..at + 4].copy_from_slice(&word.to_le_bytes());
    })
}

fn aarch64_image() -> Vec<u8> {
    let funcs = [
        // mov w0,#42; ret
        func("getv", words(&[0x52800540, 0xd65f03c0]), &[]),
        // b getv
        func("f", words(&[0x14000000]), &[(0, "getv")]),
    ];
    object(Architecture::Aarch64, &funcs, |text, at, from, to| {
        let word = 0x14000000u32 | (((to as i64 - from as i64) >> 2) as u32 & 0x3ffffff);
        text[at..at + 4].copy_from_slice(&word.to_le_bytes());
    })
}

fn decompile(bytes: &[u8], passthrough: bool) -> String {
    let path = common::scratch_file("tail-call-returns", "o");
    std::fs::write(&path, bytes).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args(["decompile-all", path.to_str().unwrap(), "--mode", "aggressive"]);
    if !passthrough {
        command.args(["--option", "passthrough", "off"]);
    }
    let output = command.output().unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "{text}\n{}", String::from_utf8_lossy(&output.stderr));
    text
}

fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let marker = format!("// Function: {name} @");
    text.split(&marker)
        .nth(1)
        .unwrap_or_else(|| panic!("{text}"))
        .split("// Function:")
        .next()
        .unwrap()
}

#[test]
fn a_tail_call_to_a_callee_with_no_parameters_returns_its_result() {
    for image in [x86_image(), arm_image(), aarch64_image()] {
        let text = decompile(&image, true);
        assert!(function(&text, "f").contains("return getv();"), "{text}");
        assert!(!function(&text, "f").contains("void f("), "{text}");
        let off = decompile(&image, false);
        assert!(function(&off, "f").contains("void f(void)"), "{off}");
    }
}

#[test]
fn an_x86_64_write_to_the_return_register_before_the_call_does_not_hide_its_result() {
    let text = decompile(&x86_image(), true);
    assert!(function(&text, "fk").contains("return getk("), "{text}");
    assert!(function(&text, "twice").contains("return getk("), "{text}");
    assert!(function(&text, "twice").contains("getv()"), "{text}");
    assert!(function(&text, "after").contains("return 7;"), "{text}");
    assert!(function(&text, "fs").contains("void fs("), "{text}");
    let off = decompile(&x86_image(), false);
    assert!(function(&off, "fk").contains("void fk("), "{off}");
    assert!(function(&off, "twice").contains("void twice("), "{off}");
}

#[test]
fn an_arm_argument_written_to_the_return_register_is_not_relaxed() {
    let text = decompile(&arm_image(), true);
    assert!(function(&text, "fc").contains("void fc("), "{text}");
    assert!(function(&text, "fc").contains("getk(5)"), "{text}");
}
