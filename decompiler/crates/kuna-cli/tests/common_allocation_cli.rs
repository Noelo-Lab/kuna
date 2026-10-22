use crate::common;
use object::{
    write, Architecture, BinaryFormat, Endianness, RelocationFlags, SymbolFlags, SymbolKind,
    SymbolScope,
};

#[test]
fn distinct_common_objects_do_not_turn_a_seven_return_into_zero() {
    let mut object =
        write::Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let text = object.section_id(write::StandardSection::Text);
    let code = [
        0xc7, 0x05, 0, 0, 0, 0, 7, 0, 0, 0, 0xc7, 0x05, 0, 0, 0, 0, 0, 0, 0, 0, 0x8b, 0x05, 0, 0,
        0, 0, 0xc3,
    ];
    object.append_section_data(text, &code, 1);
    object.add_symbol(write::Symbol {
        name: b"touch".to_vec(),
        value: 0,
        size: code.len() as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: write::SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    let mut symbols = Vec::new();
    for (name, size, alignment) in [("buffer", 64, 32), ("state", 32, 64)] {
        symbols.push(object.add_symbol(write::Symbol {
            name: name.as_bytes().to_vec(),
            value: alignment,
            size,
            kind: SymbolKind::Data,
            scope: SymbolScope::Linkage,
            weak: false,
            section: write::SymbolSection::Common,
            flags: SymbolFlags::None,
        }));
    }
    for (offset, symbol, addend) in [
        (2, symbols[0], 8),
        (12, symbols[1], -8),
        (22, symbols[0], 12),
    ] {
        object
            .add_relocation(
                text,
                write::Relocation {
                    offset,
                    symbol,
                    addend,
                    flags: RelocationFlags::Elf {
                        r_type: object::elf::R_X86_64_PC32,
                    },
                },
            )
            .unwrap();
    }
    let path = common::scratch_file("common-allocation", "o");
    std::fs::write(&path, object.write().unwrap()).unwrap();
    let (code, stderr, status) = common::run_kuna(&["decompile", path.to_str().unwrap(), "touch"]);
    assert_eq!(status, 0, "{stderr}");
    assert!(code.contains("return 7;"), "{code}");
    assert!(code.contains(" = 7;"), "{code}");
    assert!(code.contains(" = 0;"), "{code}");
    let _ = std::fs::remove_file(path);
}

#[test]
fn common_globals_are_declared_in_recompilable_project_headers() {
    use std::{fs, process::Command};
    if Command::new("clang").arg("--version").output().is_err() {
        return;
    }
    let dir = common::scratch_file("common-project-header", "dir");
    fs::create_dir_all(&dir).unwrap();
    let source = dir.join("counter.c");
    fs::write(
        &source,
        "int counter; void bump(int delta) { counter += delta; }\n",
    )
    .unwrap();
    for target in [
        "i686-w64-windows-gnu",
        "x86_64-w64-windows-gnu",
        "i386-unknown-linux-gnu",
        "x86_64-unknown-linux-gnu",
    ] {
        let object = dir.join(format!("{target}.o"));
        let built = Command::new("clang")
            .args(["-target", target, "-fcommon", "-fno-pic", "-fno-pie", "-O1", "-c"])
            .arg(&source)
            .arg("-o")
            .arg(&object)
            .output()
            .unwrap();
        assert!(
            built.status.success(),
            "{}",
            String::from_utf8_lossy(&built.stderr)
        );
        let export = dir.join(target);
        let (_, stderr, status) = common::run_kuna(&[
            "decompile-project",
            object.to_str().unwrap(),
            "-o",
            export.to_str().unwrap(),
        ]);
        assert_eq!(status, 0, "{stderr}");
        let header = fs::read_to_string(export.join(format!("{target}.o.h"))).unwrap();
        assert!(
            header
                .lines()
                .any(|line| line.starts_with("extern ") && line.contains("dat_")),
            "{target}: {header}"
        );
        let compiled = Command::new("clang")
            .args(["-target", target, "-std=c11", "-Werror", "-c"])
            .arg(export.join(format!("{target}.o.c")))
            .arg("-o")
            .arg(dir.join(format!("{target}-regenerated.o")))
            .output()
            .unwrap();
        assert!(
            compiled.status.success(),
            "{target}: {}",
            String::from_utf8_lossy(&compiled.stderr)
        );
    }
    fs::remove_dir_all(&dir).unwrap();
}
