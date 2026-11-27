//! Octeon ELF identity from standard machine, ABI and floating-point evidence.
use super::kuna_mipsabi::{abi_flags, fp_abi, AbiFlags};
use kuna_decomp::kuna_octeonid::{language_id as id, o64_language_id as o64_id};
use object::{Architecture, BinaryFormat, FileFlags, Object, ObjectSection};

const MACH_MASK: u32 = 0x00ff_0000;
const ABI2: u32 = 0x20;
const ABI_MASK: u32 = 0xf000;
const ABI_O64: u32 = 0x2000;
const MODE_32BIT: u32 = 0x100;
const FLOAT_BITS: u32 = 0xe00;

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


/// `None` leaves non-Octeon selection alone. Identified but unqualified or
/// contradictory N32/N64 profiles require an explicit target; an ELF32 O64
/// profile that does not qualify keeps generic selection, and [`notes`] says why.
pub fn language_id(file: &object::File<'_>) -> Result<Option<String>, &'static str> {
    if file.format() != BinaryFormat::Elf {
        return Ok(None);
    }
    if file.architecture() == Architecture::Mips {
        return Ok(o64_profile(file).and_then(Result::ok));
    }
    if !matches!(file.architecture(), Architecture::Mips64 | Architecture::Mips64_N32) {
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
    if let Some(record) = record {
        check_record(record, variant, hard)?;
    }
    if hard && variant != "octeon3" {
        return Err("hard-float automatic selection is qualified only for Octeon III");
    }
    Ok(Some(id(variant, n32, hard)))
}

/// Why an ELF32 image with an Octeon machine tag that declares O64 or no ABI
/// did not load as Octeon O64, for the loader's diagnostics; empty when it did,
/// when `explicit`, or for any other image.
pub fn notes(file: &object::File<'_>, language: &str, explicit: bool) -> Vec<String> {
    match o64_profile(file) {
        Some(Err(reason)) if !explicit => {
            let closest = if file.is_little_endian() {
                String::new()
            } else {
                format!(" ({} decodes the original Octeon instructions with O64)", o64_id())
            };
            vec![format!(
                "Octeon ELF32 image not loaded as Octeon O64: {reason}; using {language}; use --target to choose another language{closest}"
            )]
        }
        _ => Vec::new(),
    }
}

/// `None` for every image but a non-N32 ELF32 image with an Octeon machine tag
/// that declares O64 or no ABI; for that image, the O64 language when its
/// metadata is consistent with soft-float O64 on a big-endian original Octeon,
/// else the contradiction. The original Octeon has no FPU, so an image without
/// a float declaration is soft-float; with no attribute or ABI-flags record,
/// the header's float bits are that declaration.
fn o64_profile(file: &object::File<'_>) -> Option<Result<String, &'static str>> {
    let FileFlags::Elf { e_flags, .. } = file.flags() else {
        return None;
    };
    if file.format() != BinaryFormat::Elf
        || file.architecture() != Architecture::Mips
        || !matches!(e_flags & ABI_MASK, 0 | ABI_O64)
    {
        return None;
    }
    let variant = machine_variant(e_flags & MACH_MASK)?;
    Some(o64(file, e_flags, variant))
}

fn o64(file: &object::File<'_>, e_flags: u32, variant: &str) -> Result<String, &'static str> {
    if file.is_little_endian() {
        return Err("little-endian Octeon requires an explicit target");
    }
    if variant != "octeon" {
        return Err("only the original Octeon has an O64 model");
    }
    let record = abi_flags(file)?;
    match record.map(|r| r.extension(false)).unwrap_or(0) {
        0 | 5 => {}
        3 => return Err("Octeon Plus has no O64 model"),
        _ => return Err("conflicting Octeon machine and ISA-extension declarations"),
    }
    if e_flags & 0xf000_0000 != 0x8000_0000 {
        return Err("Octeon automatic selection requires MIPS64r2 ISA evidence");
    }
    if e_flags & 0x0f00_0000 != 0 {
        return Err("Octeon image declares an unqualified MIPS ISA extension");
    }
    if e_flags & MODE_32BIT != 0 {
        return Err("it declares 32-bit registers, which O64 does not use");
    }
    if file
        .sections()
        .filter_map(|section| section.name().ok())
        .any(|name| name.starts_with(".mdebug.") && name != ".mdebug.abiO64")
    {
        return Err("its .mdebug ABI marker names an ABI other than O64");
    }
    match fp_abi(file, record)? {
        Some(3) => {}
        None if e_flags & FLOAT_BITS == 0 => {}
        None => return Err("its header declares float registers, but O64 is soft-float"),
        Some(_) => return Err("it declares a floating-point ABI other than soft float, the only O64 model"),
    }
    if let Some(record) = record {
        check_record(record, variant, false)?;
    }
    Ok(o64_id())
}

/// An ABI-flags record must declare the MIPS64r2 ISA, 64-bit GPRs, the float
/// registers `hard` implies and no extension or flag beyond the variant's own.
fn check_record(AbiFlags(raw): AbiFlags, variant: &str, hard: bool) -> Result<(), &'static str> {
    let revision = raw[3] == 2 || (variant == "octeon3" && raw[3] == 5);
    if raw[2] != 64
        || !revision
        || raw[4] != 2
        || raw[5] != if hard { 2 } else { 0 }
        || raw[6] != 0
    {
        return Err("Octeon ABI-flags ISA or register widths disagree with its calling convention");
    }
    let ases = u32::from_be_bytes(raw[12..16].try_into().unwrap());
    let generation_ases = if variant == "octeon3" { 0x100 } else { 0 };
    if ases & !generation_ases != 0
        || u32::from_be_bytes(raw[16..20].try_into().unwrap()) & !1 != 0
        || raw[20..24] != [0; 4]
    {
        return Err("Octeon ABI-flags declare unqualified ISA extensions or flags");
    }
    Ok(())
}

pub fn language_ids() -> Vec<String> {
    let mut ids = vec![o64_id()];
    for variant in kuna_decomp::kuna_octeonid::VARIANTS {
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
    fn original_octeon_elf32_declaring_o64_or_no_abi_selects_the_o64_model() {
        for abi in [0, ABI_O64] {
            for sections in [
                vec![],
                vec![(b".mdebug.abiO64".as_slice(), vec![])],
                vec![(b".gnu.attributes".as_slice(), attributes(3))],
                vec![(b".MIPS.abiflags".as_slice(), flags(5, false))],
                vec![(b".mdebug".as_slice(), vec![0; 4])],
            ] {
                let bytes = elf(true, 0x808b0001 | abi, &sections);
                assert_eq!(select(&bytes), Ok(Some(o64_id())));
                assert!(notes(&object::File::parse(bytes.as_slice()).unwrap(), &o64_id(), false).is_empty());
            }
        }
    }

    fn refused(bytes: &[u8]) -> bool {
        let file = object::File::parse(bytes).unwrap();
        language_id(&file) == Ok(None)
            && matches!(o64_profile(&file), Some(Err(_)))
            && notes(&file, "MIPS:BE:32:default", false).len() == 1
            && notes(&file, "MIPS:BE:32:default", true).is_empty()
    }

    #[test]
    fn contradicting_elf32_octeon_metadata_keeps_generic_selection_with_a_note() {
        let none = 0x808b0001;
        for (e_flags, sections) in [
            (none, vec![(b".mdebug.abi32".as_slice(), vec![])]),
            (ABI_O64 | none, vec![(b".mdebug.abiN32".as_slice(), vec![])]),
            (none, vec![(b".mdebug.abiO64".as_slice(), vec![]), (b".mdebug.abi64".as_slice(), vec![])]),
            (none, vec![(b".gnu.attributes".as_slice(), attributes(1))]),
            (none, vec![(b".MIPS.abiflags".as_slice(), flags(3, false))]),
            (none, vec![(b".MIPS.abiflags".as_slice(), flags(2, false))]),
            (none, vec![(b".MIPS.abiflags".as_slice(), vec![0; 8])]),
            (none | MODE_32BIT, vec![]),
            (0x708b0001, vec![]),
            (0x608b0001, vec![]),
            (0xa08b0001, vec![]),
            (0x848b0001, vec![]),
            (ABI_O64 | 0x808d0001, vec![]),
            (0x808e0001, vec![]),
        ] {
            assert!(refused(&elf(true, e_flags, &sections)), "{e_flags:#x}");
        }
        let mut narrow = flags(5, false);
        narrow[4] = 1;
        assert!(refused(&elf(true, none, &[(b".MIPS.abiflags", narrow)])));
        assert!(refused(&elf(true, none, &[(b".MIPS.abiflags", flags(5, true))])));
        assert!(refused(&elf_endian(true, none, &[], Endianness::Little)));
        let little = elf_endian(true, none, &[], Endianness::Little);
        let note = notes(&object::File::parse(little.as_slice()).unwrap(), "MIPS:LE:32:default", false);
        assert!(!note[0].contains("MIPS:BE:"), "{note:?}");
        for bit in [0x200, 0x400, 0x800] {
            assert!(refused(&elf(true, none | bit, &[])), "{bit:#x}");
            let soft = [(b".gnu.attributes".as_slice(), attributes(3))];
            assert_eq!(select(&elf(true, none | bit, &soft)), Ok(Some(o64_id())), "{bit:#x}");
        }
    }

    #[test]
    fn elf32_without_an_octeon_identity_or_declaring_another_abi_keeps_generic_selection() {
        for (e_flags, sections) in [
            (0x80000001, vec![]),
            (0x80002001, vec![]),
            (0x80000001, vec![(b".MIPS.abiflags".as_slice(), flags(5, false))]),
            (0x808b1001, vec![]),
            (0x808b3001, vec![]),
            (0x808b4001, vec![]),
        ] {
            let bytes = elf(true, e_flags, &sections);
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert_eq!(language_id(&file), Ok(None), "{e_flags:#x}");
            assert!(notes(&file, "MIPS:BE:32:default", false).is_empty(), "{e_flags:#x}");
        }
    }

    #[test]
    fn valid_o32_octeon_headers_retain_generic_selection() {
        for endian in [Endianness::Big, Endianness::Little] {
            for machine in [0x8b0000, 0x8d0000, 0x8e0000] {
                let bytes = elf_endian(true, 0x80001107 | machine, &[], endian);
                assert_eq!(select(&bytes), Ok(None));
            }
        }
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
