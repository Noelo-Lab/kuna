use kuna_analysis::loader::reloc_object::layout_relocatable;
use object::{
    write, Architecture, BinaryFormat, Endianness, Object, ObjectSymbol, RelocationEncoding,
    RelocationFlags, RelocationKind, SymbolFlags, SymbolKind, SymbolScope,
};

#[test]
fn common_sizes_alignment_and_repeated_references_hold_across_architectures() {
    for (format, architecture, endian, width) in [
        (Architecture::X86_64, Endianness::Little, 8),
        (Architecture::I386, Endianness::Little, 4),
        (Architecture::Sparc, Endianness::Big, 4),
        (Architecture::Sparc64, Endianness::Big, 8),
        (Architecture::Arm, Endianness::Little, 4),
        (Architecture::Aarch64, Endianness::Little, 8),
        (Architecture::Mips, Endianness::Big, 4),
        (Architecture::PowerPc, Endianness::Big, 4),
    ]
    .into_iter()
    .map(|(a, e, w)| (BinaryFormat::Elf, a, e, w))
    .chain([
        (
            BinaryFormat::Coff,
            Architecture::I386,
            Endianness::Little,
            4,
        ),
        (
            BinaryFormat::Coff,
            Architecture::X86_64,
            Endianness::Little,
            8,
        ),
    ]) {
        let mut object = write::Object::new(format, architecture, endian);
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
        let fmt = kuna_analysis::loader::format::detect(&file).unwrap();
        let layout = layout_relocatable(&file, fmt.as_ref());
        assert!(
            layout.diagnostics.is_empty(),
            "{architecture:?}: {:?}",
            layout.diagnostics
        );
        let address = |name: &str| {
            let symbol = file
                .symbols()
                .find(|s| s.name().ok().is_some_and(|n| n == name || (format == BinaryFormat::Coff && n.strip_prefix('_') == Some(name))))
                .unwrap();
            layout.extern_addr[&symbol.index()]
        };
        let buffer = address("buffer");
        let state = address("state");
        assert_eq!(
            buffer % if format == BinaryFormat::Elf { 32 } else { 16 },
            0,
            "{architecture:?}"
        );
        assert_eq!(
            state % if format == BinaryFormat::Elf { 64 } else { 16 },
            0,
            "{architecture:?}"
        );
        assert!(state >= buffer + 64, "{architecture:?}");
        let (extern_start, extern_end) = layout.extern_range.unwrap();
        assert_eq!(address("callee"), extern_start, "{architecture:?}");
        assert_eq!(extern_end - extern_start, 16, "{architecture:?}");
        assert!(buffer >= extern_end, "{architecture:?}");
        let (common_start, common_end) = layout.common_range.unwrap();
        assert_eq!(common_start, buffer);
        assert_eq!(common_end, state + 32);
        let image =
            kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes("common", &bytes).unwrap();
        let ranges = image.data_object_ranges();
        assert!(
            ranges
                .iter()
                .any(|&(lo, hi)| lo <= buffer && hi >= state + 31),
            "{format:?}: {ranges:?}"
        );
        assert!(
            !ranges
                .iter()
                .any(|&(lo, hi)| lo <= extern_start && extern_start <= hi),
            "{format:?}: extern slot is data"
        );
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

/// One object with `weak` undefined weak functions, each named by a full
/// read-only pointer, `unused` more that nothing names, `common` COMMON
/// objects and one ordinary extern.
fn weak_and_common_object(
    architecture: Architecture,
    endian: Endianness,
    width: usize,
    weak: usize,
    unused: usize,
    common: usize,
) -> Vec<u8> {
    let mut object = write::Object::new(BinaryFormat::Elf, architecture, endian);
    let rodata = object.section_id(write::StandardSection::ReadOnlyData);
    let data = object.section_id(write::StandardSection::Data);
    object.append_section_data(rodata, &vec![0; width * weak.max(1)], width as u64);
    object.append_section_data(data, &vec![0; width * (common + 1)], width as u64);
    let pointer = RelocationFlags::Generic {
        kind: RelocationKind::Absolute,
        encoding: RelocationEncoding::Generic,
        size: (width * 8) as u8,
    };
    let add = |object: &mut write::Object, name: String, size, value, section, kind, weak| {
        object.add_symbol(write::Symbol {
            name: name.into_bytes(),
            size,
            value,
            section,
            kind,
            scope: SymbolScope::Linkage,
            weak,
            flags: SymbolFlags::None,
        })
    };
    let undefined = write::SymbolSection::Undefined;
    let callee = add(&mut object, "callee".into(), 0, 0, undefined, SymbolKind::Text, false);
    let mut relocations = vec![(data, 0, callee)];
    for index in 0..weak + unused {
        let symbol = add(&mut object, format!("optional{index}"), 0, 0, undefined, SymbolKind::Text, true);
        if index < weak {
            relocations.push((rodata, index * width, symbol));
        }
    }
    for index in 0..common {
        let (size, alignment) = if index % 2 == 0 { (4, 4) } else { (24, 16) };
        let symbol = add(
            &mut object,
            format!("shared{index}"),
            size,
            alignment,
            write::SymbolSection::Common,
            SymbolKind::Data,
            false,
        );
        relocations.push((data, (index + 1) * width, symbol));
    }
    for (section, offset, symbol) in relocations {
        object
            .add_relocation(
                section,
                write::Relocation { offset: offset as u64, symbol, addend: 0, flags: pointer },
            )
            .unwrap();
    }
    object.write().unwrap()
}

/// Extern slots, weak import slots and COMMON storage never share an address,
/// and COMMON storage moves only by the slots an architecture can allocate.
#[test]
fn weak_import_slots_stay_clear_of_extern_slots_and_common_storage() {
    for (architecture, endian, width, slotted) in [
        (Architecture::X86_64, Endianness::Little, 8, true),
        (Architecture::Mips, Endianness::Big, 4, true),
        (Architecture::Mips, Endianness::Little, 4, true),
        (Architecture::Mips64, Endianness::Big, 8, true),
        (Architecture::Mips64, Endianness::Little, 8, true),
        (Architecture::Aarch64, Endianness::Little, 8, false),
        (Architecture::PowerPc, Endianness::Big, 4, false),
        (Architecture::I386, Endianness::Little, 4, false),
    ] {
        for (weak, unused, common) in [(1, 0, 2), (3, 2, 5), (0, 2, 3), (1000, 0, 1000)] {
            let bytes = weak_and_common_object(architecture, endian, width, weak, unused, common);
            let file = object::File::parse(&*bytes).unwrap();
            let fmt = kuna_analysis::loader::format::detect(&file).unwrap();
            let layout = layout_relocatable(&file, fmt.as_ref());
            let case = format!("{architecture:?} {endian:?} {weak}+{unused} weak, {common} common");
            assert!(layout.diagnostics.is_empty(), "{case}: {:?}", layout.diagnostics.report_lines());
            let mut externs = Vec::new();
            let mut commons = Vec::new();
            let (mut undefined, mut weak_undefined) = (0u64, 0u64);
            for symbol in file.symbols() {
                undefined += u64::from(symbol.is_undefined());
                weak_undefined += u64::from(symbol.is_undefined() && symbol.is_weak());
                let Some(&address) = layout.extern_addr.get(&symbol.index()) else { continue };
                if symbol.is_common() {
                    commons.push((address, address + symbol.size()));
                } else {
                    externs.push((address, address + 16));
                }
            }
            assert_eq!(externs.len(), weak + 1, "{case}");
            assert_eq!(commons.len(), common, "{case}");
            assert_eq!(layout.import_slots.len(), if slotted { weak } else { 0 }, "{case}");
            assert!(layout.import_slots.iter().all(|&(lo, hi)| hi - lo == width as u64), "{case}");
            let base = externs.iter().map(|range| range.0).min().unwrap();
            let reserved = if slotted { weak_undefined * width as u64 } else { 0 };
            let first_common = commons.iter().map(|range| range.0).min().unwrap();
            assert_eq!(first_common, (base + 16 * undefined + reserved + 3) & !3, "{case}");
            assert_eq!(layout.common_range.unwrap().0, first_common, "{case}");
            assert!(layout.import_slots.iter().all(|&(_, hi)| hi <= first_common), "{case}");
            let mut ranges: Vec<_> = externs
                .iter()
                .chain(&commons)
                .chain(&layout.import_slots)
                .copied()
                .collect();
            ranges.sort_unstable();
            assert!(ranges.windows(2).all(|pair| pair[0].1 <= pair[1].0), "{case}: {ranges:x?}");
            let slots: Vec<_> = layout.import_slots.iter().map(|&(lo, _)| lo).collect();
            for &(slot, entry) in &layout.function_address_aliases {
                assert!(slots.contains(&slot), "{case}");
                assert!(externs.contains(&(entry, entry + 16)), "{case}");
                assert_eq!(layout.funcsyms.iter().filter(|(address, _)| *address == entry).count(), 1, "{case}");
            }
            assert!(!layout.funcsyms.iter().any(|(address, _)| slots.contains(address)), "{case}");
        }
    }
}

/// A weak relocation whose offset cannot be an address in the section is
/// skipped; the relocation beside it still gets its slot.
#[test]
fn an_unrepresentable_weak_relocation_offset_is_skipped() {
    for (architecture, endian, width) in [
        (Architecture::X86_64, Endianness::Little, 8usize),
        (Architecture::Mips64, Endianness::Big, 8),
        (Architecture::Mips, Endianness::Big, 4),
    ] {
        for section in [write::StandardSection::ReadOnlyData, write::StandardSection::Text] {
            for offset in [u64::MAX - 0xff, u64::MAX, u64::MAX - width as u64 + 1] {
                let mut object = write::Object::new(BinaryFormat::Elf, architecture, endian);
                let rodata = object.section_id(write::StandardSection::ReadOnlyData);
                let target = object.section_id(section);
                object.append_section_data(rodata, &vec![0; width], width as u64);
                if target != rodata {
                    object.append_section_data(target, &[0; 16], 4);
                }
                let symbol = object.add_symbol(write::Symbol {
                    name: b"optional".to_vec(),
                    size: 0,
                    value: 0,
                    section: write::SymbolSection::Undefined,
                    kind: SymbolKind::Text,
                    scope: SymbolScope::Linkage,
                    weak: true,
                    flags: SymbolFlags::None,
                });
                let flags = RelocationFlags::Generic {
                    kind: RelocationKind::Absolute,
                    encoding: RelocationEncoding::Generic,
                    size: (width * 8) as u8,
                };
                for (section, offset) in [(rodata, 0), (target, offset)] {
                    object
                        .add_relocation(section, write::Relocation { offset, symbol, addend: 0, flags })
                        .unwrap();
                }
                let bytes = object.write().unwrap();
                let file = object::File::parse(&*bytes).unwrap();
                let fmt = kuna_analysis::loader::format::detect(&file).unwrap();
                let layout = layout_relocatable(&file, fmt.as_ref());
                let case = format!("{architecture:?} {section:?} {offset:#x}");
                assert_eq!(layout.import_slots.len(), 1, "{case}");
                let rodata_vma = layout.section_info.iter().find(|s| s.name == ".rodata").unwrap().vma;
                let fields: Vec<_> = layout.nullable_relocations.iter().map(|r| r.field).collect();
                assert_eq!(fields, [rodata_vma], "{case}");
            }
        }
    }
}
