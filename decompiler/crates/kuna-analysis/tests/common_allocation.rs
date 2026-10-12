use kuna_analysis::loader::{format::elf::ElfFormat, reloc_object::layout_relocatable};
use object::{
    write, Architecture, BinaryFormat, Endianness, Object, ObjectSymbol, RelocationEncoding,
    RelocationFlags, RelocationKind, SymbolFlags, SymbolKind, SymbolScope,
};

#[test]
fn common_sizes_alignment_and_repeated_references_hold_across_architectures() {
    for (architecture, endian, width) in [
        (Architecture::X86_64, Endianness::Little, 8),
        (Architecture::I386, Endianness::Little, 4),
        (Architecture::Sparc, Endianness::Big, 4),
        (Architecture::Sparc64, Endianness::Big, 8),
        (Architecture::Arm, Endianness::Little, 4),
        (Architecture::Aarch64, Endianness::Little, 8),
        (Architecture::Mips, Endianness::Big, 4),
        (Architecture::PowerPc, Endianness::Big, 4),
    ] {
        let mut object = write::Object::new(BinaryFormat::Elf, architecture, endian);
        let section = object.section_id(write::StandardSection::Data);
        object.append_section_data(section, &vec![0; width * 4], width as u64);
        let mut symbols = Vec::new();
        for (name, size, value, storage, kind) in [
            (
                "buffer",
                64,
                32,
                write::SymbolSection::Common,
                SymbolKind::Data,
            ),
            (
                "state",
                32,
                64,
                write::SymbolSection::Common,
                SymbolKind::Data,
            ),
            (
                "callee",
                0,
                0,
                write::SymbolSection::Undefined,
                SymbolKind::Text,
            ),
        ] {
            symbols.push(object.add_symbol(write::Symbol {
                name: name.as_bytes().to_vec(),
                size,
                value,
                section: storage,
                kind,
                scope: SymbolScope::Linkage,
                weak: false,
                flags: SymbolFlags::None,
            }));
        }
        for (index, symbol) in [symbols[0], symbols[1], symbols[0], symbols[2]]
            .into_iter()
            .enumerate()
        {
            object
                .add_relocation(
                    section,
                    write::Relocation {
                        offset: (index * width) as u64,
                        symbol,
                        addend: 0,
                        flags: RelocationFlags::Generic {
                            kind: RelocationKind::Absolute,
                            encoding: RelocationEncoding::Generic,
                            size: (width * 8) as u8,
                        },
                    },
                )
                .unwrap();
        }
        let bytes = object.write().unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let layout = layout_relocatable(&file, &ElfFormat);
        assert!(
            layout.diagnostics.is_empty(),
            "{architecture:?}: {:?}",
            layout.diagnostics
        );
        let address = |name: &str| {
            let symbol = file
                .symbols()
                .find(|s| s.name().ok() == Some(name))
                .unwrap();
            layout.extern_addr[&symbol.index()]
        };
        let buffer = address("buffer");
        let state = address("state");
        assert_eq!(buffer % 32, 0, "{architecture:?}");
        assert_eq!(state % 64, 0, "{architecture:?}");
        assert!(state >= buffer + 64, "{architecture:?}");
        let (extern_start, extern_end) = layout.extern_range.unwrap();
        assert_eq!(address("callee"), extern_start, "{architecture:?}");
        assert_eq!(extern_end - extern_start, 16, "{architecture:?}");
        assert!(buffer >= extern_end, "{architecture:?}");
        let (common_start, common_end) = layout.common_range.unwrap();
        assert_eq!(common_start, buffer);
        assert_eq!(common_end, state + 32);
        let data_vma = layout
            .section_info
            .iter()
            .find(|s| s.name == ".data")
            .unwrap()
            .vma;
        let raw = &layout
            .segments
            .iter()
            .find(|(address, _)| *address == data_vma)
            .unwrap()
            .1;
        assert_eq!(
            &raw[..width],
            &raw[2 * width..3 * width],
            "{architecture:?}"
        );
    }
}
