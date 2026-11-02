use super::*;
use object::write::{Object as Writer, Relocation as WriteRelocation, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};

/// Parse one relocation of type `r_type` (REL when `implicit`) out of a
/// one-symbol object, the way the loader sees it.
fn relocation(r_type: u32, addend: i64, implicit: bool) -> Relocation {
    let (arch, endian) = if implicit {
        (Architecture::Mips, Endianness::Big)
    } else {
        (Architecture::Mips64, Endianness::Big)
    };
    let mut obj = Writer::new(BinaryFormat::Elf, arch, endian);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &[0; 16], 4);
    let sym = obj.add_symbol(Symbol {
        name: b"f".to_vec(),
        value: 0,
        size: 0,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Undefined,
        flags: SymbolFlags::None,
    });
    obj.add_relocation(
        text,
        WriteRelocation {
            offset: 0,
            symbol: sym,
            addend,
            flags: object::RelocationFlags::Elf { r_type },
        },
    )
    .unwrap();
    let bytes = obj.write().unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let section = file.section_by_name(".text").unwrap();
    let (_, reloc) = section.relocations().next().unwrap();
    assert_eq!(reloc.has_implicit_addend(), implicit);
    reloc
}

fn target(address: u64) -> Target {
    Target { address, local: false, gp_disp: false }
}

fn apply_be(gp: &mut Gp, r_type: u32, reloc: &Relocation, insn: u32, t: Target, place: u64, lo: Option<i64>) -> Result<u32, RelocationFailure> {
    let mut field = insn.to_be_bytes();
    gp.apply(r_type, reloc, &mut field, t, place, lo)?;
    Ok(u32::from_be_bytes(field))
}

#[test]
fn classify_names_widths_no_ops_and_code_targets() {
    assert_eq!(classify(R26).unwrap(), Some((4, true)));
    assert_eq!(classify(CALL16).unwrap(), Some((4, true)));
    assert_eq!(classify(HI16).unwrap(), Some((4, false)));
    assert_eq!(classify(R64).unwrap(), Some((8, false)));
    assert_eq!(classify(GPREL32 | (R64 << 8)).unwrap(), Some((8, false)));
    assert_eq!(classify(GPREL16 | (SUB << 8) | (HI16 << 16)).unwrap(), Some((4, false)));
    assert_eq!(classify(JALR).unwrap(), None);
    assert_eq!(classify(NONE).unwrap(), None);
    assert!(classify(42).is_err(), "TLS_GD is unsupported");
    assert!(classify(GPREL16 | (SUB << 8)).is_err(), "a chain cannot end in SUB");
    assert!(classify(GPREL16 | (HI16 << 16)).is_err(), "type3 needs a type2");
}

#[test]
fn jal_encodes_its_word_index_and_keeps_the_opcode() {
    let mut gp = Gp::new(0x40_1000, 4, 0, false);
    let r = relocation(R26, 0, true);
    let jal = apply_be(&mut gp, R26, &r, 0x0c00_0000, target(0x40_0100), 0x40_0020, None).unwrap();
    assert_eq!(jal, 0x0c10_0040);
    // A local `jal` against `.text` keeps the callee's offset as its addend.
    let local = Target { local: true, ..target(0x40_0000) };
    let jal = apply_be(&mut gp, R26, &r, 0x0c00_0010, local, 0x40_0020, None).unwrap();
    assert_eq!(jal, 0x0c10_0010);
    // The target must share the place's 256 MiB region.
    assert_eq!(
        apply_be(&mut gp, R26, &r, 0x0c00_0000, target(0x1000_0000), 0x40_0020, None),
        Err(RelocationFailure::OutOfRange)
    );
}

#[test]
fn hi16_takes_the_low_half_of_its_addend_from_the_paired_lo16() {
    let mut gp = Gp::new(0x40_1000, 4, 0, false);
    let r = relocation(HI16, 0, true);
    // `lui` addend 0, `addiu` addend -4: the address is S - 4 = 0x41_7ffc.
    let lui = apply_be(&mut gp, HI16, &r, 0x3c02_0000, target(0x41_8000), 0x40_0000, Some(-4)).unwrap();
    assert_eq!(lui, 0x3c02_0041);
    let r = relocation(LO16, 0, true);
    let addiu = apply_be(&mut gp, LO16, &r, 0x2442_fffc, target(0x41_8000), 0x40_0004, None).unwrap();
    assert_eq!(addiu, 0x2442_7ffc);
}

#[test]
fn gp_disp_is_gp_less_the_place_and_the_prologue_rebuilds_gp() {
    let mut gp = Gp::new(0x40_1000, 4, 0, false);
    let g = gp.gp();
    let disp = Target { gp_disp: true, ..target(0) };
    let entry = 0x40_0100u64;
    let lui = apply_be(&mut gp, HI16, &relocation(HI16, 0, true), 0x3c1c_0000, disp, entry, Some(0)).unwrap();
    let addiu = apply_be(&mut gp, LO16, &relocation(LO16, 0, true), 0x279c_0000, disp, entry + 4, None).unwrap();
    let rebuilt = ((lui & 0xffff) << 16).wrapping_add(addiu as u16 as i16 as i32 as u32).wrapping_add(entry as u32);
    assert_eq!(u64::from(rebuilt), g);
}

#[test]
fn call16_reads_its_symbol_from_a_got_entry_gp_reaches() {
    let mut gp = Gp::new(0x40_1000, 4, 4, false);
    let r = relocation(CALL16, 0, true);
    let lw = apply_be(&mut gp, CALL16, &r, 0x8f99_0000, target(0x40_0200), 0x40_0010, None).unwrap();
    let off = i64::from(lw as u16 as i16);
    let slot = (gp.gp() as i64 + off) as u64;
    assert_eq!(slot, 0x40_1000);
    // The same symbol shares its entry; another takes the next.
    assert_eq!(apply_be(&mut gp, CALL16, &r, 0x8f99_0000, target(0x40_0200), 0x40_0020, None).unwrap(), lw);
    let lw2 = apply_be(&mut gp, CALL16, &r, 0x8f99_0000, target(0x40_0300), 0x40_0030, None).unwrap();
    assert_eq!(i64::from(lw2 as u16 as i16) - off, 4);
    let bytes = gp.bytes();
    assert_eq!(&bytes[..8], &[0x00, 0x40, 0x02, 0x00, 0x00, 0x40, 0x03, 0x00]);
}

#[test]
fn a_local_got16_names_the_page_and_its_lo16_the_offset() {
    let mut gp = Gp::new(0x40_1000, 4, 4, true);
    let local = Target { local: true, ..target(0x41_0000) };
    let r = relocation(GOT16, 0, true);
    let mut field = 0x8f82_0000u32.to_le_bytes();
    gp.apply(GOT16, &r, &mut field, local, 0x40_0000, Some(0x1234)).unwrap();
    let slot = (gp.gp() as i64 + i64::from(u32::from_le_bytes(field) as u16 as i16)) as u64;
    assert_eq!(slot, 0x40_1000);
    assert_eq!(&gp.bytes()[..4], &0x41_0000u32.to_le_bytes());
}

#[test]
fn n64_chains_hi16_and_lo16_of_minus_gp_rel() {
    let mut gp = Gp::new(0x40_1000, 8, 0, false);
    let f = 0x40_0040u64;
    let hi = GPREL16 | (SUB << 8) | (HI16 << 16);
    let lo = GPREL16 | (SUB << 8) | (LO16 << 16);
    let lui = apply_be(&mut gp, hi, &relocation(hi, 0, false), 0x3c01_0000, target(f), f, None).unwrap();
    let daddiu = apply_be(&mut gp, lo, &relocation(lo, 0, false), 0x643c_0000, target(f), f + 8, None).unwrap();
    let rebuilt = (i64::from(lui as u16 as i16) << 16) + i64::from(daddiu as u16 as i16) + f as i64;
    assert_eq!(rebuilt as u64, gp.gp());
}

#[test]
fn gpdword_writes_the_gp_relative_doubleword() {
    let mut gp = Gp::new(0x40_1000, 8, 0, false);
    let t = GPREL32 | (R64 << 8);
    let mut field = [0u8; 8];
    gp.apply(t, &relocation(t, 0x10, false), &mut field, target(0x40_0000), 0x40_0800, None).unwrap();
    assert_eq!(i64::from_be_bytes(field), 0x40_0010 - gp.gp() as i64);
}

#[test]
fn pc16_is_relative_to_its_branch() {
    let mut gp = Gp::new(0x40_1000, 4, 0, false);
    // `bal f` against an external symbol carries the -4 of the delay slot.
    let r = relocation(PC16, 0, true);
    let bal = apply_be(&mut gp, PC16, &r, 0x0411_ffff, target(0x40_0100), 0x40_0000, None).unwrap();
    assert_eq!(bal, 0x0411_003f);
}

#[test]
fn a_full_got_and_an_out_of_range_offset_fail_without_writing() {
    let mut gp = Gp::new(0x40_1000, 4, 1, false);
    let r = relocation(CALL16, 0, true);
    apply_be(&mut gp, CALL16, &r, 0x8f99_0000, target(0x40_0200), 0, None).unwrap();
    assert_eq!(
        apply_be(&mut gp, CALL16, &r, 0x8f99_0000, target(0x40_0300), 0, None),
        Err(RelocationFailure::OutOfRange)
    );
    let r = relocation(GPREL16, 0, true);
    assert_eq!(
        apply_be(&mut gp, GPREL16, &r, 0x8f82_0000, target(0x50_0000), 0, None),
        Err(RelocationFailure::OutOfRange)
    );
}

#[test]
fn small_data_sections_are_recognized_by_name() {
    for name in [".sdata", ".sbss", ".lit4", ".lit8", ".srdata", ".sdata.x", ".sbss.y"] {
        assert!(is_small_data(name), "{name}");
    }
    for name in [".data", ".sdatax", ".bss", ".rodata"] {
        assert!(!is_small_data(name), "{name}");
    }
}
