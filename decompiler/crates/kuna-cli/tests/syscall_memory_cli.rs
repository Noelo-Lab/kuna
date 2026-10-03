//! An OS system call may replace a writable global through its buffer argument.
mod common;

use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

fn image(userland: bool) -> Vec<u8> {
    let words = [
        0xe92d4080u32,
        0xe2813007,
        0xe59f1024,
        0xe3a02004,
        0xe3a07003,
        0xe08f1001,
        0xe5813000,
        0xef000000,
        0xe59f1010,
        0xe0830103,
        0xe79f1001,
        0xe0810000,
        0xe8bd8080,
        0,
        0,
        0xe92d4080,
        0xe1a01000,
        0xe5903000,
        0xe3a00011,
        0xe3a07003,
        0xe3a02004,
        0xef000000,
        0xe5911000,
        0xe0830103,
        0xe0810000,
        0xe8bd8080,
        0xe92d4080,
        0xe1a01000,
        0xe3a00011,
        0xe3a07003,
        0xe3a02004,
        0xef000000,
        0xe3a07003,
        0xe3a02004,
        0xe5913000,
        0xe3a00011,
        0xef000000,
        0xe5911000,
        0xe0830103,
        0xe0810000,
        0xe8bd8080,
        0xe92d4080,
        0xe59f1028,
        0xe2803007,
        0xe3a00011,
        0xe3a07003,
        0xe08f1001,
        0xe3a02004,
        0xe5813000,
        0xef000000,
        0xe3a00000,
        0xe5810000,
        0xe1a00003,
        0xe8bd8080,
        0,
        0xe92d40b0,
        0xe1a05000,
        0xe3a07003,
        0xe3a00011,
        0xe1a01005,
        0xe3a02004,
        0xef000000,
        0xe1a04000,
        0xe3a07003,
        0xe3a00012,
        0xe1a01005,
        0xe3a02002,
        0xef000000,
        0xe6bf0070,
        0xe5951000,
        0xe0844204,
        0xe0811004,
        0xe0810000,
        0xe8bd80b0,
    ];
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let bytes: Vec<u8> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    object.append_section_data(text, &bytes, 4);
    object.add_symbol(Symbol {
        name: b"rd_into_global".to_vec(),
        value: 0,
        size: 0x3c,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    for (name, value, size) in [
        ("pointer_before", 0x3c, 0x2c),
        ("pointer_twice", 0x68, 0x3c),
        ("write_global", 0xa4, 0x38),
        ("return_snapshot", 0xdc, 0x4c),
    ] {
        object.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    let bss = object.add_section(Vec::new(), b".bss".to_vec(), SectionKind::UninitializedData);
    object.append_section_bss(bss, 4, 4);
    let sink = object.add_symbol(Symbol {
        name: b"sink".to_vec(),
        value: 0,
        size: 4,
        kind: SymbolKind::Data,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(bss),
        flags: SymbolFlags::None,
    });
    for (offset, addend) in [(0x34, 24), (0x38, 8), (0xd8, 24)] {
        object
            .add_relocation(
                text,
                Relocation {
                    offset,
                    symbol: sink,
                    addend,
                    flags: object::RelocationFlags::Elf {
                        r_type: object::elf::R_ARM_REL32,
                    },
                },
            )
            .unwrap();
    }
    if userland {
        add_userland_note(&mut object);
    }
    object.write().unwrap()
}

fn add_userland_note(object: &mut Object) {
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

fn narrow_image64(userland: bool) -> Vec<u8> {
    let words = [
        0xa9bf53f3u32,
        0xaa0003f3,
        0xd28007e8,
        0xd2800220,
        0xaa1303e1,
        0xd2800082,
        0xd4000001,
        0xaa0003f4,
        0xd28007e8,
        0xd2800240,
        0xaa1303e1,
        0xd2800042,
        0xd4000001,
        0x13003c00,
        0xb9400261,
        0x8b141294,
        0x0b140021,
        0x0b000020,
        0xa8c153f3,
        0xd65f03c0,
    ];
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Aarch64, Endianness::Little);
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let bytes: Vec<u8> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    object.append_section_data(text, &bytes, 4);
    object.add_symbol(Symbol {
        name: b"return_snapshot".to_vec(),
        value: 0,
        size: bytes.len() as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    if userland {
        add_userland_note(&mut object);
    }
    object.write().unwrap()
}

fn decompile(function: &str, userland: bool, option: Option<&str>) -> String {
    decompile_image(function, image(userland), option)
}

fn decompile_image(function: &str, image: Vec<u8>, option: Option<&str>) -> String {
    let path = common::scratch_file("syscall-memory", "o");
    std::fs::write(&path, image).unwrap();
    let mut args = vec!["decompile", path.to_str().unwrap(), function];
    if let Some(value) = option {
        args.extend(["--option", "syscallregs", value]);
    }
    let (text, err, rc) = common::run_kuna(&args);
    assert_eq!(rc, 0, "{text}\n{err}");
    if let Some(global) = text
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .find(|word| word.starts_with("dat_"))
    {
        text.replace(global, "sink")
    } else {
        text
    }
}

#[test]
fn system_call_reloads_the_memory_it_may_replace() {
    for text in [
        decompile("rd_into_global", true, None),
        decompile("rd_into_global", false, Some("on")),
    ] {
        assert!(text.contains("* 5"), "{text}");
        assert!(!text.contains("* 6"), "{text}");
        assert_eq!(text.matches("sink").count(), 2, "{text}");
        if Command::new("cc").arg("--version").output().is_err() {
            continue;
        }
        let source = common::scratch_file("syscall-memory-roundtrip", "c");
        let binary = common::scratch_file("syscall-memory-roundtrip", "bin");
        std::fs::write(&source, format!(
            "int sink, seen;\nvoid software_interrupt(int a, ...) {{ seen = sink; sink = 41; }}\n{text}\nint main(void) {{ int r = rd_into_global(0, 2); return r != 86 || sink != 41 || seen != 9; }}\n"
        )).unwrap();
        let output = Command::new("cc")
            .args(["-O2", "-x", "c"])
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{text}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(Command::new(&binary).status().unwrap().success(), "{text}");
    }
}

#[test]
fn unclassified_image_and_off_keep_the_original_memory_model() {
    for text in [
        decompile("rd_into_global", false, None),
        decompile("rd_into_global", true, Some("off")),
    ] {
        assert!(text.contains("* 6"), "{text}");
        assert_eq!(text.matches("sink").count(), 1, "{text}");
    }
}

#[test]
fn pointer_values_are_saved_before_each_memory_changing_call() {
    if Command::new("cc").arg("--version").output().is_err() {
        return;
    }
    for (function, expected) in [("pointer_before", 115), ("pointer_twice", 305)] {
        let text = decompile(function, true, None);
        let source = common::scratch_file("syscall-pointer-roundtrip", "c");
        let binary = common::scratch_file("syscall-pointer-roundtrip", "bin");
        std::fs::write(&source, format!(
            "int calls;\nvoid software_interrupt(int a,int b,int c,int *p,int n,...) {{ *p = ++calls == 1 && {expected} == 305 ? 41 : 100; }}\n{text}\nint main(void) {{ int v = 3; int r = {function}(&v); return r != {expected} || v != 100; }}\n"
        )).unwrap();
        let output = Command::new("cc")
            .args(["-O2", "-x", "c"])
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{text}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(Command::new(&binary).status().unwrap().success(), "{text}");
    }
}

#[test]
fn the_kernel_observes_a_store_overwritten_after_the_call() {
    let text = decompile("write_global", true, None);
    assert_eq!(text.matches("sink =").count(), 2, "{text}");
    if Command::new("cc").arg("--version").output().is_err() {
        return;
    }
    let source = common::scratch_file("syscall-store-roundtrip", "c");
    let binary = common::scratch_file("syscall-store-roundtrip", "bin");
    std::fs::write(&source, format!(
        "int sink, seen;\nvoid software_interrupt(int a, ...) {{ seen = sink; sink = 41; }}\n{text}\nint main(void) {{ int r = write_global(2); return r != 9 || sink != 0 || seen != 9; }}\n"
    )).unwrap();
    let output = Command::new("cc")
        .args(["-O2", "-x", "c"])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(Command::new(&binary).status().unwrap().success(), "{text}");
}

#[test]
fn returned_lengths_are_saved_before_reading_the_final_buffer() {
    for text in [
        decompile("return_snapshot", true, None),
        decompile("return_snapshot", false, Some("on")),
        decompile_image("return_snapshot", narrow_image64(true), None),
        decompile_image("return_snapshot", narrow_image64(false), Some("on")),
    ] {
        let calls: Vec<_> = text
            .lines()
            .filter(|line| line.contains("software_interrupt(") || line.contains("CallSupervisor("))
            .collect();
        assert_eq!(calls.len(), 2, "{text}");
        assert!(calls.iter().all(|line| line.contains(" = ")), "{text}");
        if Command::new("cc").arg("--version").output().is_err() {
            continue;
        }
        let source = common::scratch_file("syscall-returned-lengths", "c");
        std::fs::write(&source, format!(
            "int calls;\nlong kernel(long fd,int *p,long size) {{ int value = fd == 17 ? 100 : 500; ++calls; __builtin_memcpy(p, &value, size); return size; }}\n#define software_interrupt(imm,num,fd,p,size,...) kernel(fd,p,size)\n#define CallSupervisor(imm,num,fd,p,size,...) kernel(fd,p,size)\n{text}\nint main(void) {{ int p = 3; return return_snapshot(&p) != 570 || p != 500 || calls != 2; }}\n"
        )).unwrap();
        for optimization in ["-O0", "-O2"] {
            let binary = common::scratch_file("syscall-returned-lengths", "bin");
            let output = Command::new("cc")
                .args([optimization, "-x", "c"])
                .arg(&source)
                .arg("-o")
                .arg(&binary)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{text}\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(Command::new(&binary).status().unwrap().success(), "{text}");
        }
    }
}
