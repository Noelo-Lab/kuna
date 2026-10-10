//! `syscallregs auto`, the default, gives an inline `svc` its result register
//! on an image built for an operating system's user space, and leaves the `svc`
//! of an image that says nothing about its system alone.
use crate::common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

/// The GH-812 shape as an ARM object: `pair(a, b)` moves b into r0 and a into
/// r1, sets r7 = 4 and returns what the `svc` leaves in r0. `userland` adds the
/// GNU ABI-tag note a Linux toolchain links into every program.
fn image(userland: bool) -> Vec<u8> {
    let words: [u32; 7] = [
        0xe92d4080, 0xe1a02001, 0xe1a01000, 0xe1a00002, 0xe3a07004, 0xef000000, 0xe8bd8080,
    ];
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let bytes: Vec<u8> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    object.append_section_data(text, &bytes, 4);
    object.add_symbol(Symbol {
        name: b"pair".to_vec(),
        value: 0,
        size: bytes.len() as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    if userland {
        let note = object.add_section(Vec::new(), b".note.ABI-tag".to_vec(), SectionKind::Note);
        let mut data = Vec::new();
        for v in [4u32, 16, 1] {
            data.extend_from_slice(&v.to_le_bytes());
        }
        data.extend_from_slice(b"GNU\0");
        for v in [0u32, 3, 2, 0] {
            data.extend_from_slice(&v.to_le_bytes());
        }
        object.append_section_data(note, &data, 4);
    }
    object.write().unwrap()
}

fn decompile(userland: bool, option: Option<&str>) -> String {
    let path = common::scratch_file("syscall-regs", "o");
    std::fs::write(&path, image(userland)).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args(["decompile", path.to_str().unwrap(), "pair"]);
    if let Some(value) = option {
        command.args(["--option", "syscallregs", value]);
    }
    let output = command.output().unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    text
}

/// By default a user-space image's `svc` returns its result, and reads the r7,
/// r0, r1 and r2 the function wrote before it.
#[test]
fn userland_image_returns_the_svc_result_by_default() {
    let text = decompile(true, None);
    assert!(
        text.contains("return software_interrupt(0,4,a1,a0,a1);"),
        "{text}"
    );
    assert!(!text.contains("CONCAT44"), "{text}");
}

/// Without a sign of an operating system, `auto` keeps the vendored model, as
/// `off` does on a user-space image; `on` models the `svc` on any image.
#[test]
fn auto_needs_the_userland_image() {
    for text in [decompile(false, None), decompile(true, Some("off"))] {
        assert!(text.contains("return CONCAT44(a0,a1);"), "{text}");
    }
    let forced = decompile(false, Some("on"));
    assert!(
        forced.contains("return software_interrupt(0,4,a1,a0,a1);"),
        "{forced}"
    );
}
