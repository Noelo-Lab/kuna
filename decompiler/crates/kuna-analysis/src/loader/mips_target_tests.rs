use crate::loadimage_object::ObjectLoadImage;
use kuna_sleigh::loadimage::LoadImage;
use object::{write::Object, Architecture, BinaryFormat, Endianness, FileFlags, SectionKind};

fn image(arch: Architecture, little: bool, flags: u32) -> Vec<u8> {
    let endian = if little {
        Endianness::Little
    } else {
        Endianness::Big
    };
    let mut obj = Object::new(BinaryFormat::Elf, arch, endian);
    obj.flags = FileFlags::Elf {
        os_abi: 0,
        abi_version: 0,
        e_flags: flags,
    };
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &[0; 8], 4);
    obj.write().unwrap()
}

#[test]
fn mips_elf_flags_select_register_width_address_width_and_abi() {
    for little in [false, true] {
        let endian = if little { "LE" } else { "BE" };
        for (arch, flags, suffix) in [
            (Architecture::Mips, 0x7000_1000, "32:default:default"),
            (Architecture::Mips, 0x7400_1000, "32:default:default"),
            (Architecture::Mips, 0x8000_0000, "64:64-32addr:default"),
            (Architecture::Mips, 0x8000_2000, "64:64-32addr:o64"),
            (Architecture::Mips, 0x8000_0020, "64:64-32addr:n32"),
            (Architecture::Mips, 0x8000_1000, "64:64-32addr-fp32:o32"),
            (Architecture::Mips, 0x8000_1100, "32:default:default"),
            (Architecture::Mips, 0x7000_3000, "32:default:eabi"),
            (Architecture::Mips64, 0x8000_0000, "64:default:default"),
            (Architecture::Mips, 0xa000_0020, "64:64-32R6addr:n32"),
            (Architecture::Mips, 0x9000_1000, "32:R6:default"),
            (Architecture::Mips, 0x9200_1405, "32:R6:default"),
            (Architecture::Mips, 0xa200_0020, "64:64-32R6addr:n32"),
            (Architecture::Mips64, 0xa200_0000, "64:R6:default"),
            (Architecture::Mips64, 0xa000_0000, "64:R6:default"),
            (Architecture::Mips, 0x7200_1000, "32:micro:default"),
            (Architecture::Mips, 0x8200_0020, "64:micro64-32addr:n32"),
        ] {
            let bytes = image(arch, little, flags);
            let loaded = ObjectLoadImage::from_bytes("synthetic-mips.elf", &bytes).unwrap();
            let expected = format!("MIPS:{endian}:{suffix}");
            assert_eq!(loaded.get_arch_type(), expected.as_bytes(), "{flags:#x}");
            assert!(
                loaded.fallback_arch_id().is_none(),
                "must not retry with a different ISA/ABI"
            );
        }
    }
}

#[test]
fn explicit_target_wins_over_elf_isa_and_abi_metadata() {
    for (arch, flags) in [
        (Architecture::Mips, 0x8000_0020),
        (Architecture::Mips, 0x8000_4000),
    ] {
        let bytes = image(arch, false, flags);
        let target = "MIPS:BE:64:64-32addr:n32";
        let loaded =
            ObjectLoadImage::from_bytes_with_target("synthetic.elf", &bytes, target).unwrap();
        assert_eq!(loaded.get_arch_type(), target.as_bytes());
        assert!(loaded.fallback_arch_id().is_none());
    }
}

#[test]
fn unsupported_or_conflicting_mips_declarations_require_an_override() {
    for (arch, flags) in [
        (Architecture::Mips, 0x8000_4000),
        (Architecture::Mips, 0x8000_1020),
        (Architecture::Mips, 0x8000_0120),
        (Architecture::Mips, 0x8000_2100),
        (Architecture::Mips, 0x7000_0020),
        (Architecture::Mips, 0xb000_1000),
        (Architecture::Mips, 0x7600_1000),
        (Architecture::Mips, 0x9400_1000),
        (Architecture::Mips, 0x9600_1000),
        (Architecture::Mips, 0x7200_3000),
        (Architecture::Mips64, 0x8000_0020),
        (Architecture::Mips64, 0x8000_1000),
    ] {
        let bytes = image(arch, false, flags);
        let error = ObjectLoadImage::from_bytes("synthetic-invalid.elf", &bytes).unwrap_err();
        assert!(error.explain().contains("--target"), "{error:?}");
        assert!(ObjectLoadImage::from_bytes_with_target(
            "synthetic-invalid.elf",
            &bytes,
            "MIPS:BE:64:64-32addr:n32",
        )
        .is_ok());
    }
}

fn mips_float_image(little: bool, flags: u32, sections: &[(&[u8], Vec<u8>)]) -> Vec<u8> {
    let endian = if little {
        Endianness::Little
    } else {
        Endianness::Big
    };
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Mips, endian);
    obj.flags = FileFlags::Elf {
        os_abi: 0,
        abi_version: 0,
        e_flags: flags,
    };
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &[0; 8], 4);
    for (name, bytes) in sections {
        let section = obj.add_section(Vec::new(), name.to_vec(), SectionKind::Other);
        obj.append_section_data(section, bytes, 4);
    }
    obj.write().unwrap()
}

fn mips_fp_record(size: u8, abi: u8) -> Vec<u8> {
    let mut bytes = vec![0; 24];
    bytes[2..8].copy_from_slice(&[64, 2, 2, size, 0, abi]);
    bytes
}

fn mips_fp_attribute(little: bool, abi: u8) -> Vec<u8> {
    let word = |v: u32| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let mut bytes = vec![b'A'];
    bytes.extend(word(15));
    bytes.extend(b"gnu\0");
    bytes.push(1);
    bytes.extend(word(7));
    bytes.extend([4, abi]);
    bytes
}

#[test]
fn mips_floating_point_metadata_selects_fprs_independently() {
    for little in [false, true] {
        let endian = if little { "LE" } else { "BE" };
        for (flags, size, abi, suffix) in [
            (0x8000_1000, 1, 1, "64:64-32addr-fp32:o32"),
            (0x8000_1000, 2, 6, "64:64-32addr:o32"),
            (0x8000_1000, 2, 7, "64:64-32addr:o32"),
            (0x8200_1000, 1, 1, "64:micro64-32addr-fp32:o32"),
            (0x7000_1000, 1, 1, "32:default:default"),
            (0x9000_1000, 2, 6, "32:R6:default"),
            (0x8000_0020, 2, 1, "64:64-32addr:n32"),
        ] {
            let bytes = mips_float_image(
                little,
                flags,
                &[(b".MIPS.abiflags", mips_fp_record(size, abi))],
            );
            let loaded = ObjectLoadImage::from_bytes("synthetic-fpr.elf", &bytes).unwrap();
            assert_eq!(
                loaded.get_arch_type(),
                format!("MIPS:{endian}:{suffix}").as_bytes()
            );
        }
        for (flags, sections, suffix) in [
            (0x8000_1000, vec![], "64-32addr-fp32"),
            (0x8000_1200, vec![], "64-32addr"),
            (
                0x8000_1000,
                vec![(&b".gnu.attributes"[..], mips_fp_attribute(little, 1))],
                "64-32addr-fp32",
            ),
            (
                0x8000_1000,
                vec![(&b".gnu.attributes"[..], mips_fp_attribute(little, 6))],
                "64-32addr",
            ),
        ] {
            let bytes = mips_float_image(little, flags, &sections);
            let loaded = ObjectLoadImage::from_bytes("synthetic-fpr.elf", &bytes).unwrap();
            assert_eq!(
                loaded.get_arch_type(),
                format!("MIPS:{endian}:64:{suffix}:o32").as_bytes()
            );
        }
    }
}

#[test]
fn conflicting_or_unsupported_mips_fp_metadata_requires_an_override() {
    for little in [false, true] {
        let mut truncated = mips_fp_record(1, 1);
        truncated.pop();
        let mut unknown_version = mips_fp_record(1, 1);
        unknown_version[0] = 1;
        let mut cases = vec![
            (0x8000_1000, vec![(&b".MIPS.abiflags"[..], truncated)]),
            (0x8000_1000, vec![(b".MIPS.abiflags", unknown_version)]),
            (
                0x8000_1000,
                vec![
                    (b".MIPS.abiflags", mips_fp_record(1, 1)),
                    (b".gnu.attributes", mips_fp_attribute(little, 6)),
                ],
            ),
        ];
        for (flags, size, abi) in [
            (0x8000_1000, 1, 6),
            (0x8000_1000, 2, 1),
            (0x8000_1200, 1, 1),
            (0x8000_1000, 3, 1),
            (0x8000_1000, 0, 8),
            (0xa000_1000, 1, 1),
            (0x8000_0020, 1, 1),
            (0x7000_1000, 2, 6),
        ] {
            cases.push((flags, vec![(b".MIPS.abiflags", mips_fp_record(size, abi))]));
        }
        for (flags, sections) in cases {
            let bytes = mips_float_image(little, flags, &sections);
            let error =
                ObjectLoadImage::from_bytes("synthetic-invalid-fpr.elf", &bytes).unwrap_err();
            assert!(error.explain().contains("--target"), "{error:?}");
            assert!(ObjectLoadImage::from_bytes_with_target(
                "synthetic-invalid-fpr.elf",
                &bytes,
                "MIPS:LE:64:64-32addr:o32"
            )
            .is_ok());
        }
    }
}

#[test]
fn mips_fp_any_does_not_conflict_with_concrete_declarations() {
    for little in [false, true] {
        let endian = if little { "LE" } else { "BE" };
        for (abi, suffix, carrier) in [
            (0, "64-32addr-fp32", None),
            (1, "64-32addr-fp32", Some(true)),
            (3, "64-32addr-fp32", Some(false)),
            (6, "64-32addr", Some(true)),
        ] {
            for (record, attribute) in [(0, abi), (abi, 0)] {
                let bytes = mips_float_image(
                    little,
                    0x8000_1000,
                    &[
                        (b".MIPS.abiflags", mips_fp_record(0, record)),
                        (b".gnu.attributes", mips_fp_attribute(little, attribute)),
                    ],
                );
                let loaded = ObjectLoadImage::from_bytes("synthetic-fp-any.elf", &bytes).unwrap();
                assert_eq!(
                    loaded.get_arch_type(),
                    format!("MIPS:{endian}:64:{suffix}:o32").as_bytes()
                );
                let file = object::File::parse(bytes.as_slice()).unwrap();
                assert_eq!(super::kuna_floatabi::float_arg_registers(&file), carrier);
            }
        }
        let bytes = mips_float_image(
            little,
            0x8000_1000,
            &[
                (b".MIPS.abiflags", mips_fp_record(2, 0)),
                (b".gnu.attributes", mips_fp_attribute(little, 0)),
            ],
        );
        let loaded = ObjectLoadImage::from_bytes("synthetic-fp-any.elf", &bytes).unwrap();
        assert_eq!(
            loaded.get_arch_type(),
            format!("MIPS:{endian}:64:64-32addr:o32").as_bytes()
        );
    }
}

#[test]
fn processor_specific_profiles_are_not_selected_as_generic_mips() {
    for little in [false, true] {
        for machine in [0x008b_0000, 0x008d_0000, 0x008e_0000] {
            let bytes = image(Architecture::Mips, little, 0x8000_0020 | machine);
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert_eq!(super::kuna_mipstarget::language_id(&file).unwrap(), None);
        }
        for extension in [2u32, 3, 5, 19] {
            let mut record = mips_fp_record(2, 3);
            record[8..12].copy_from_slice(&if little {
                extension.to_le_bytes()
            } else {
                extension.to_be_bytes()
            });
            let bytes = mips_float_image(little, 0x8000_0020, &[(b".MIPS.abiflags", record)]);
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert_eq!(super::kuna_mipstarget::language_id(&file).unwrap(), None);
        }
    }
}

fn mips_segment_image(little: bool, records: &[Vec<u8>]) -> Vec<u8> {
    let word = |n: u32| {
        if little {
            n.to_le_bytes()
        } else {
            n.to_be_bytes()
        }
    };
    let half = |n: u16| {
        if little {
            n.to_le_bytes()
        } else {
            n.to_be_bytes()
        }
    };
    let mut bytes = b"\x7fELF".to_vec();
    bytes.extend([1, if little { 1 } else { 2 }, 1]);
    bytes.resize(16, 0);
    bytes.extend([2, 8].into_iter().flat_map(half));
    bytes.extend([1, 0x1000, 52, 0, 0x8000_1000].into_iter().flat_map(word));
    bytes.extend(
        [52, 32, records.len() as u16 + 1, 40, 0, 0]
            .into_iter()
            .flat_map(half),
    );
    bytes.extend(
        [1, 0x100, 0x1000, 0x1000, 8, 8, 5, 4]
            .into_iter()
            .flat_map(word),
    );
    for (i, record) in records.iter().enumerate() {
        let offset = 0x110 + i as u32 * 32;
        bytes.extend(
            [
                object::elf::PT_MIPS_ABIFLAGS,
                offset,
                0x2000 + offset,
                0,
                record.len() as u32,
                record.len() as u32,
                4,
                8,
            ]
            .into_iter()
            .flat_map(word),
        );
    }
    for (i, record) in records.iter().enumerate() {
        bytes.resize(0x110 + i * 32, 0);
        bytes.extend(record);
    }
    bytes
}

#[test]
fn segment_only_abi_records_select_fpr_width_and_exclude_processor_extensions() {
    for little in [false, true] {
        let endian = if little { "LE" } else { "BE" };
        for (size, abi, variant) in [(1, 1, "64-32addr-fp32"), (2, 6, "64-32addr")] {
            let bytes = mips_segment_image(little, &[mips_fp_record(size, abi)]);
            let loaded = ObjectLoadImage::from_bytes("synthetic-abi-segment.elf", &bytes).unwrap();
            assert_eq!(
                loaded.get_arch_type(),
                format!("MIPS:{endian}:64:{variant}:o32").as_bytes()
            );
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert_eq!(super::kuna_floatabi::float_arg_registers(&file), Some(true));
        }
        let mut record = mips_fp_record(2, 3);
        record[8..12].copy_from_slice(&if little {
            5u32.to_le_bytes()
        } else {
            5u32.to_be_bytes()
        });
        let bytes = mips_segment_image(little, &[record]);
        let file = object::File::parse(bytes.as_slice()).unwrap();
        assert_eq!(super::kuna_mipstarget::language_id(&file).unwrap(), None);
    }
}

#[test]
fn conflicting_segment_abi_records_require_an_explicit_target() {
    for little in [false, true] {
        let bytes = mips_segment_image(little, &[mips_fp_record(1, 1), mips_fp_record(2, 6)]);
        let error =
            ObjectLoadImage::from_bytes("synthetic-conflicting-abi.elf", &bytes).unwrap_err();
        assert!(
            error.explain().contains("conflicting MIPS ABI-flags"),
            "{error:?}"
        );
        let target = "MIPS:BE:64:64-32addr:o32";
        assert!(ObjectLoadImage::from_bytes_with_target(
            "synthetic-conflicting-abi.elf",
            &bytes,
            target
        )
        .is_ok());
    }
}
