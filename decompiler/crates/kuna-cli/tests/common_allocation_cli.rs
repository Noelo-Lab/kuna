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
