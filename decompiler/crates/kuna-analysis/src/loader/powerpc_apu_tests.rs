use super::kuna_ppcaputarget::{language_id_when, notes_when};
use crate::loadimage_object::ObjectLoadImage;
use kuna_sleigh::loadimage::LoadImage;
use object::{write::Object, Architecture, BinaryFormat, Endianness, FileFlags, SectionKind};

fn image(arch: Architecture, little: bool, flags: u32, apus: Option<&[u32]>) -> Vec<u8> {
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
    if let Some(apus) = apus {
        let word = |n: u32| {
            if little {
                n.to_le_bytes()
            } else {
                n.to_be_bytes()
            }
        };
        let mut note: Vec<u8> = [8, apus.len() as u32 * 4, 2]
            .into_iter()
            .flat_map(word)
            .collect();
        note.extend_from_slice(b"APUinfo\0");
        note.extend(apus.iter().copied().flat_map(word));
        let section = obj.add_section(Vec::new(), b".PPC.EMB.apuinfo".to_vec(), SectionKind::Note);
        obj.append_section_data(section, &note, 4);
    }
    obj.write().unwrap()
}

fn generic(bytes: &[u8], little: bool) {
    let endian = if little { "LE" } else { "BE" };
    let loaded = ObjectLoadImage::from_bytes("synthetic-apu.elf", bytes).unwrap();
    assert_eq!(
        loaded.get_arch_type(),
        format!("PowerPC:{endian}:32:default:default").as_bytes()
    );
    assert!(loaded.fallback_arch_id().is_none());
}

#[test]
fn embedded_powerpc_uses_declared_apus_not_the_embedded_abi_bit() {
    for little in [false, true] {
        let endian = if little { "LE" } else { "BE" };
        for (apus, variant) in [
            (None, None),
            (Some(&[0x0040_0001][..]), None),
            (Some(&[0x0041_0001, 0x0042_0001, 0x0043_0001][..]), None),
            (Some(&[0x0040_0001, 0x0101_0001][..]), Some("e500")),
            (Some(&[0x0100_0001][..]), Some("e500")),
        ] {
            let bytes = image(Architecture::PowerPc, little, 0x8000_0000, apus);
            let file = object::File::parse(&bytes[..]).unwrap();
            assert_eq!(
                language_id_when(&file, true),
                variant.map(|v| format!("PowerPC:{endian}:32:{v}:default"))
            );
            assert!(notes_when(&file, "PowerPC", false, true).is_empty());
            // `ppcapu` is off by default.
            assert_eq!(language_id_when(&file, false), None);
            generic(&bytes, little);
        }
    }
}

#[test]
fn unsupported_or_malformed_apus_keep_the_generic_decoder_with_a_note() {
    for little in [false, true] {
        let mut cases: Vec<Vec<u8>> = [
            &[0x0104_0001][..],
            &[0x0102_0001],
            &[0x0040_0002],
            &[0xffff_0001],
            &[0x0100_0001, 0x0104_0001],
        ]
        .into_iter()
        .map(|apus| image(Architecture::PowerPc, little, 0x8000_0000, Some(apus)))
        .collect();
        let mut bytes = image(
            Architecture::PowerPc,
            little,
            0x8000_0000,
            Some(&[0x0100_0001]),
        );
        let owner = bytes.windows(8).position(|w| w == b"APUinfo\0").unwrap();
        let size = if little {
            u32::MAX.to_le_bytes()
        } else {
            u32::MAX.to_be_bytes()
        };
        bytes[owner - 8..owner - 4].copy_from_slice(&size);
        cases.push(bytes);
        for bytes in cases {
            let file = object::File::parse(&bytes[..]).unwrap();
            assert_eq!(language_id_when(&file, true), None);
            let notes = notes_when(&file, "PowerPC:BE:32:default:default", false, true);
            assert_eq!(notes.len(), 1);
            assert!(notes[0].contains("APU"), "{notes:?}");
            assert!(notes[0].contains("--target"), "{notes:?}");
            assert!(notes_when(&file, "PowerPC", true, true).is_empty());
            assert!(notes_when(&file, "PowerPC", false, false).is_empty());
            generic(&bytes, little);
        }
    }
}

#[test]
fn explicit_target_wins_over_spe_or_unknown_apus() {
    for apus in [&[0x0100_0001][..], &[0xffff_0001][..]] {
        let bytes = image(Architecture::PowerPc, false, 0x8000_0000, Some(apus));
        let target = "PowerPC:BE:32:default:default";
        let loaded =
            ObjectLoadImage::from_bytes_with_target("synthetic-apu.elf", &bytes, target).unwrap();
        assert_eq!(loaded.get_arch_type(), target.as_bytes());
        assert!(loaded.fallback_arch_id().is_none());
    }
}
