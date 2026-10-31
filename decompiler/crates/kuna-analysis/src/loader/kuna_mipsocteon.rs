//! Octeon ELF identity from standard machine, ABI and floating-point evidence.
use super::kuna_mipsabi::{abi_flags, fp_abi, AbiFlags};
use object::{Architecture, BinaryFormat, FileFlags, Object};

const MACH_MASK: u32 = 0x00ff_0000;
const ABI2: u32 = 0x20;
const ABI_MASK: u32 = 0xf000;

fn machine_variant(machine: u32) -> Option<&'static str> {
    match machine {
        0x008b_0000 => Some("octeon"),
        0x008d_0000 => Some("octeon2"),
        0x008e_0000 => Some("octeon3"),
        _ => None,
    }
}

fn extension_variant(extension: u32) -> Option<&'static str> {
    match extension {
        5 => Some("octeon"),
        3 => Some("octeonplus"),
        2 => Some("octeon2"),
        19 => Some("octeon3"),
        _ => None,
    }
}

fn id(variant: &str, n32: bool, hard: bool) -> String {
    format!(
        "MIPS:BE:64:{variant}{}:{}",
        if n32 { "-32addr" } else { "" },
        if hard { "hard" } else { "default" }
    )
}

/// `None` leaves non-Octeon selection alone. Identified but unqualified or
/// contradictory profiles require an explicit target, never an inferred ABI.
pub fn language_id(file: &object::File<'_>) -> Result<Option<String>, &'static str> {
    if file.format() != BinaryFormat::Elf
        || !matches!(
            file.architecture(),
            Architecture::Mips | Architecture::Mips64 | Architecture::Mips64_N32
        )
    {
        return Ok(None);
    }
    let FileFlags::Elf { e_flags, .. } = file.flags() else {
        return Ok(None);
    };
    let machine = e_flags & MACH_MASK;
    let from_machine = machine_variant(machine);
    let record = match abi_flags(file) {
        Ok(record) => record,
        Err(error) if from_machine.is_some() => return Err(error),
        Err(_) => return Ok(None),
    };
    let extension = record
        .map(|a| a.extension(file.is_little_endian()))
        .unwrap_or(0);
    let from_extension = extension_variant(extension);
    if from_machine.is_none() && from_extension.is_none() {
        return Ok(None);
    }
    let variant = match (from_machine, from_extension) {
        (Some("octeon"), Some("octeonplus")) => "octeonplus",
        (Some(a), Some(b)) if a == b => a,
        (Some(a), None) if extension == 0 => a,
        (None, Some(b)) if machine == 0 => b,
        _ => return Err("conflicting Octeon machine and ISA-extension declarations"),
    };
    if file.is_little_endian() {
        return Err("little-endian Octeon requires an explicit target");
    }
    let n32 = !file.is_64();
    if e_flags & ABI_MASK != 0 || (e_flags & ABI2 != 0) != n32 {
        return Err("Octeon ELF class and N32/N64 ABI flags disagree or name an unsupported ABI");
    }
    if e_flags & 0xf000_0000 != 0x8000_0000 {
        return Err("Octeon automatic selection requires MIPS64r2 ISA evidence");
    }
    if e_flags & 0x0f00_0000 != 0 {
        return Err("Octeon image declares an unqualified MIPS ISA extension");
    }
    let hard = match fp_abi(file, record)? {
        Some(1) => true,
        Some(3) => false,
        _ => return Err("Octeon floating-point ABI is missing or unsupported; specify a target"),
    };
    if let Some(AbiFlags(raw)) = record {
        let revision = raw[3] == 2 || (variant == "octeon3" && raw[3] == 5);
        if raw[2] != 64
            || !revision
            || raw[4] != 2
            || raw[5] != if hard { 2 } else { 0 }
            || raw[6] != 0
        {
            return Err(
                "Octeon ABI-flags ISA or register widths disagree with its calling convention",
            );
        }
        let ases = u32::from_be_bytes(raw[12..16].try_into().unwrap());
        let generation_ases = if variant == "octeon3" { 0x100 } else { 0 };
        if ases & !generation_ases != 0
            || u32::from_be_bytes(raw[16..20].try_into().unwrap()) & !1 != 0
            || raw[20..24] != [0; 4]
        {
            return Err("Octeon ABI-flags declare unqualified ISA extensions or flags");
        }
    }
    if hard && variant != "octeon3" {
        return Err("hard-float automatic selection is qualified only for Octeon III");
    }
    Ok(Some(id(variant, n32, hard)))
}

pub fn language_ids() -> Vec<String> {
    let mut ids = Vec::new();
    for variant in ["octeon", "octeonplus", "octeon2", "octeon3"] {
        for n32 in [false, true] {
            ids.push(id(variant, n32, false));
            if variant == "octeon3" {
                ids.push(id(variant, n32, true));
            }
        }
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use object::write::Object as Writer;
    use object::{Endianness, SectionKind};

    fn flags(extension: u32, hard: bool) -> Vec<u8> {
        let mut bytes = vec![0; 24];
        bytes[2..5].copy_from_slice(&[64, if extension == 19 { 5 } else { 2 }, 2]);
        bytes[5] = if hard { 2 } else { 0 };
        bytes[7] = if hard { 1 } else { 3 };
        bytes[8..12].copy_from_slice(&extension.to_be_bytes());
        if extension == 19 {
            bytes[12..16].copy_from_slice(&0x100u32.to_be_bytes());
        }
        bytes
    }

    fn elf(n32: bool, e_flags: u32, sections: &[(&[u8], Vec<u8>)]) -> Vec<u8> {
        elf_endian(n32, e_flags, sections, Endianness::Big)
    }

    fn elf_endian(
        n32: bool,
        e_flags: u32,
        sections: &[(&[u8], Vec<u8>)],
        endian: Endianness,
    ) -> Vec<u8> {
        let arch = if n32 {
            Architecture::Mips
        } else {
            Architecture::Mips64
        };
        let mut object = Writer::new(BinaryFormat::Elf, arch, endian);
        object.flags = FileFlags::Elf {
            os_abi: 0,
            abi_version: 0,
            e_flags,
        };
        for (name, bytes) in sections {
            let section = object.add_section(Vec::new(), name.to_vec(), SectionKind::Other);
            object.append_section_data(section, bytes, 1);
        }
        object.write().unwrap()
    }

    fn select(bytes: &[u8]) -> Result<Option<String>, &'static str> {
        language_id(&object::File::parse(bytes).unwrap())
    }

    fn attributes(fp: u8) -> Vec<u8> {
        let mut bytes = vec![b'A'];
        bytes.extend(15u32.to_be_bytes());
        bytes.extend(b"gnu\0\x01");
        bytes.extend(7u32.to_be_bytes());
        bytes.extend([4, fp]);
        bytes
    }

    fn with_segments(
        mut bytes: Vec<u8>,
        n32: bool,
        records: &[Vec<u8>],
        sectionless: bool,
    ) -> Vec<u8> {
        fn put16(bytes: &mut [u8], offset: usize, value: u16) {
            bytes[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
        }
        fn put(bytes: &mut [u8], offset: usize, value: usize, n32: bool) {
            if n32 {
                bytes[offset..offset + 4].copy_from_slice(&(value as u32).to_be_bytes());
            } else {
                bytes[offset..offset + 8].copy_from_slice(&(value as u64).to_be_bytes());
            }
        }
        bytes.resize((bytes.len() + 7) & !7, 0);
        let table = bytes.len();
        let stride = if n32 { 32 } else { 56 };
        put(&mut bytes, if n32 { 28 } else { 32 }, table, n32);
        put16(&mut bytes, if n32 { 42 } else { 54 }, stride as u16);
        put16(&mut bytes, if n32 { 44 } else { 56 }, records.len() as u16);
        bytes.resize(table + stride * records.len(), 0);
        for (index, record) in records.iter().enumerate() {
            let ph = table + stride * index;
            let offset = bytes.len();
            bytes[ph..ph + 4].copy_from_slice(&object::elf::PT_MIPS_ABIFLAGS.to_be_bytes());
            put(&mut bytes, ph + if n32 { 4 } else { 8 }, offset, n32);
            put(
                &mut bytes,
                ph + if n32 { 16 } else { 32 },
                record.len(),
                n32,
            );
            bytes.extend(record);
        }
        if sectionless {
            put(&mut bytes, if n32 { 32 } else { 40 }, 0, n32);
            put16(&mut bytes, if n32 { 48 } else { 60 }, 0);
            put16(&mut bytes, if n32 { 50 } else { 62 }, 0);
        }
        bytes
    }

    #[test]
    fn segment_evidence_survives_missing_sections_and_conflicts_are_rejected() {
        for n32 in [false, true] {
            let header = 0x808e0000 | if n32 { ABI2 } else { 0 };
            let soft = flags(19, false);
            let hard = flags(19, true);
            for sectionless in [false, true] {
                let bytes = with_segments(
                    elf(n32, header, &[(b".MIPS.abiflags", soft.clone())]),
                    n32,
                    &[soft.clone(), soft.clone()],
                    sectionless,
                );
                assert_eq!(select(&bytes), Ok(Some(id("octeon3", n32, false))));
                assert_eq!(
                    super::super::kuna_floatabi::float_arg_registers(
                        &object::File::parse(bytes.as_slice()).unwrap()
                    ),
                    Some(false)
                );
                assert!(select(&bytes[..bytes.len() - 1]).is_err());
                let bytes = with_segments(
                    elf(n32, header, &[(b".MIPS.abiflags", soft.clone())]),
                    n32,
                    &[soft.clone(), hard.clone()],
                    sectionless,
                );
                assert!(select(&bytes).is_err());
            }
        }
    }

    #[test]
    fn indistinguishable_legacy_plus_flags_select_only_the_base_generation() {
        let bytes = elf(true, 0x808b0020, &[(b".gnu.attributes", attributes(3))]);
        assert_eq!(select(&bytes), Ok(Some(id("octeon", true, false))));
        let bytes = elf(true, 0x80000020, &[(b".MIPS.abiflags", flags(3, false))]);
        assert_eq!(select(&bytes), Ok(Some(id("octeonplus", true, false))));
    }

    #[test]
    fn modern_generations_and_both_abis_select_distinct_models() {
        for n32 in [false, true] {
            for (machine, extension, variant) in [
                (0x8b0000, 5, "octeon"),
                (0x8b0000, 3, "octeonplus"),
                (0x8d0000, 2, "octeon2"),
                (0x8e0000, 19, "octeon3"),
            ] {
                for hard in [false, true] {
                    let bytes = elf(
                        n32,
                        0x80000000 | machine | if n32 { ABI2 } else { 0 },
                        &[(b".MIPS.abiflags", flags(extension, hard))],
                    );
                    if hard && variant != "octeon3" {
                        assert!(select(&bytes).is_err());
                    } else {
                        assert_eq!(select(&bytes), Ok(Some(id(variant, n32, hard))));
                    }
                }
            }
        }
    }

    #[test]
    fn legacy_vendor_float_bits_do_not_override_standard_attributes() {
        for n32 in [false, true] {
            for vendor_bit in [0, 0x200, 0x800] {
                for (fp, hard) in [(1, true), (3, false)] {
                    let bytes = elf(
                        n32,
                        0x808e0000 | vendor_bit | if n32 { ABI2 } else { 0 },
                        &[(b".gnu.attributes", attributes(fp))],
                    );
                    assert_eq!(select(&bytes), Ok(Some(id("octeon3", n32, hard))));
                }
            }
        }
    }

    #[test]
    fn ambiguous_missing_and_conflicting_evidence_is_not_a_soft_default() {
        for sections in [
            vec![],
            vec![(b".gnu.attributes".as_slice(), attributes(0))],
            vec![(b".MIPS.abiflags".as_slice(), flags(2, false))],
            vec![
                (b".MIPS.abiflags".as_slice(), flags(19, false)),
                (b".gnu.attributes".as_slice(), attributes(1)),
            ],
            vec![(b".MIPS.abiflags".as_slice(), vec![0; 8])],
        ] {
            assert!(select(&elf(false, 0x808e0000, &sections)).is_err());
        }
        for e_flags in [
            0x808e0020, 0x808e1000, 0x708e0000, 0x808f0000, 0x848e0000, 0x828e0000,
        ] {
            assert!(select(&elf(
                false,
                e_flags,
                &[(b".MIPS.abiflags", flags(19, false))]
            ))
            .is_err());
        }
        let mut bad = flags(19, false);
        bad[4] = 1;
        assert!(select(&elf(false, 0x808e0000, &[(b".MIPS.abiflags", bad)])).is_err());
        for byte in [6, 12, 19, 23] {
            let mut bad = flags(19, false);
            bad[byte] = 2;
            assert!(select(&elf(false, 0x808e0000, &[(b".MIPS.abiflags", bad)])).is_err());
        }
        assert_eq!(select(&elf(false, 0x80000000, &[])), Ok(None));
    }

    #[test]
    fn unknown_fp_attribute_can_be_refined_by_an_explicit_declaration() {
        let bytes = elf(
            true,
            0x808e0020,
            &[
                (b".gnu.attributes", attributes(0)),
                (b".MIPS.abiflags", flags(19, false)),
            ],
        );
        assert_eq!(select(&bytes), Ok(Some(id("octeon3", true, false))));
        assert_eq!(
            super::super::kuna_floatabi::float_arg_registers(
                &object::File::parse(bytes.as_slice()).unwrap()
            ),
            Some(false)
        );
    }

    #[test]
    fn little_endian_metadata_is_recognized_but_requires_an_explicit_target() {
        for n32 in [false, true] {
            for hard in [false, true] {
                let mut raw = flags(19, hard);
                raw[8..12].copy_from_slice(&19u32.to_le_bytes());
                raw[12..16].copy_from_slice(&0x100u32.to_le_bytes());
                let bytes = elf_endian(
                    n32,
                    0x80000000 | if n32 { ABI2 } else { 0 },
                    &[(b".MIPS.abiflags", raw)],
                    Endianness::Little,
                );
                assert_eq!(
                    select(&bytes),
                    Err("little-endian Octeon requires an explicit target")
                );
                assert_eq!(
                    super::super::kuna_floatabi::float_arg_registers(
                        &object::File::parse(bytes.as_slice()).unwrap()
                    ),
                    Some(hard)
                );
            }
        }
    }
}
