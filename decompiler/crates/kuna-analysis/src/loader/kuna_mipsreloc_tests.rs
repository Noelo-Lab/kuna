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

/// Apply `reloc` alone to `field` at `place` through the section path the
/// loader uses, resolving every symbol to `t`; the first failure, if any.
fn apply_one(
    gp: &mut Gp,
    arch: Architecture,
    reloc: Relocation,
    field: &mut [u8],
    t: Target,
    place: u64,
    lo: Option<i64>,
) -> Result<(), RelocationFailure> {
    let records = [(0u64, reloc)];
    let applied = gp.apply_section(arch, field, place, &records, &[lo], Ok(0), &mut |_| Some(t));
    applied.failures.first().map_or(Ok(()), |&(_, reason)| Err(reason))
}

fn apply_be(
    gp: &mut Gp,
    arch: Architecture,
    reloc: Relocation,
    insn: u32,
    t: Target,
    place: u64,
    lo: Option<i64>,
) -> Result<u32, RelocationFailure> {
    let mut field = insn.to_be_bytes();
    apply_one(gp, arch, reloc, &mut field, t, place, lo)?;
    Ok(u32::from_be_bytes(field))
}

#[test]
fn field_widths_and_code_targets_follow_the_last_and_first_operation() {
    assert_eq!(field_width(R64), Ok(8));
    assert_eq!(field_width(SUB), Ok(8));
    for op in [R16, R32, REL32, R26, HI16, LO16, GPREL16, GOT16, CALL16, GPREL32, GOT_DISP, HIGHER, HIGHEST, PC32] {
        assert_eq!(field_width(op), Ok(4), "{op}");
    }
    assert_eq!(field_width(NONE), Err(RelocationFailure::Unsupported));
    assert_eq!(field_width(42), Err(RelocationFailure::Unsupported), "TLS_GD is unsupported");
    assert_eq!(field_width(JALR), Err(RelocationFailure::Unsupported), "a JALR hint patches nothing");
    for op in [R26, PC16, CALL16, CALL_HI16, CALL_LO16] {
        assert!(targets_code(op), "{op}");
    }
    for op in [HI16, LO16, GOT16, R32, R32 | (R26 << 8)] {
        assert!(!targets_code(op), "{op}");
    }
}

#[test]
fn jal_encodes_its_word_index_and_keeps_the_opcode() {
    let mut gp = Gp::new(0x40_1000, 4, 0, false);
    let o32 = Architecture::Mips;
    let jal = apply_be(&mut gp, o32, relocation(R26, 0, true), 0x0c00_0000, target(0x40_0100), 0x40_0020, None).unwrap();
    assert_eq!(jal, 0x0c10_0040);
    // A local `jal` against `.text` keeps the callee's offset as its addend.
    let local = Target { local: true, ..target(0x40_0000) };
    let jal = apply_be(&mut gp, o32, relocation(R26, 0, true), 0x0c00_0010, local, 0x40_0020, None).unwrap();
    assert_eq!(jal, 0x0c10_0010);
    // The target must share the place's 256 MiB region.
    assert_eq!(
        apply_be(&mut gp, o32, relocation(R26, 0, true), 0x0c00_0000, target(0x1000_0000), 0x40_0020, None),
        Err(RelocationFailure::OutOfRange)
    );
}

#[test]
fn hi16_takes_the_low_half_of_its_addend_from_the_paired_lo16() {
    let mut gp = Gp::new(0x40_1000, 4, 0, false);
    let o32 = Architecture::Mips;
    // `lui` addend 0, `addiu` addend -4: the address is S - 4 = 0x41_7ffc.
    let lui = apply_be(&mut gp, o32, relocation(HI16, 0, true), 0x3c02_0000, target(0x41_8000), 0x40_0000, Some(-4)).unwrap();
    assert_eq!(lui, 0x3c02_0041);
    let addiu = apply_be(&mut gp, o32, relocation(LO16, 0, true), 0x2442_fffc, target(0x41_8000), 0x40_0004, None).unwrap();
    assert_eq!(addiu, 0x2442_7ffc);
}

#[test]
fn gp_disp_is_gp_less_the_place_and_the_prologue_rebuilds_gp() {
    let mut gp = Gp::new(0x40_1000, 4, 0, false);
    let o32 = Architecture::Mips;
    let g = gp.gp();
    let disp = Target { gp_disp: true, ..target(0) };
    let entry = 0x40_0100u64;
    let lui = apply_be(&mut gp, o32, relocation(HI16, 0, true), 0x3c1c_0000, disp, entry, Some(0)).unwrap();
    let addiu = apply_be(&mut gp, o32, relocation(LO16, 0, true), 0x279c_0000, disp, entry + 4, None).unwrap();
    let rebuilt = ((lui & 0xffff) << 16).wrapping_add(addiu as u16 as i16 as i32 as u32).wrapping_add(entry as u32);
    assert_eq!(u64::from(rebuilt), g);
}

#[test]
fn call16_reads_its_symbol_from_a_got_entry_gp_reaches() {
    let mut gp = Gp::new(0x40_1000, 4, 4, false);
    let o32 = Architecture::Mips;
    let lw = apply_be(&mut gp, o32, relocation(CALL16, 0, true), 0x8f99_0000, target(0x40_0200), 0x40_0010, None).unwrap();
    let off = i64::from(lw as u16 as i16);
    let slot = (gp.gp() as i64 + off) as u64;
    assert_eq!(slot, 0x40_1000);
    // The same symbol shares its entry; another takes the next.
    assert_eq!(apply_be(&mut gp, o32, relocation(CALL16, 0, true), 0x8f99_0000, target(0x40_0200), 0x40_0020, None).unwrap(), lw);
    let lw2 = apply_be(&mut gp, o32, relocation(CALL16, 0, true), 0x8f99_0000, target(0x40_0300), 0x40_0030, None).unwrap();
    assert_eq!(i64::from(lw2 as u16 as i16) - off, 4);
    let bytes = gp.bytes();
    assert_eq!(&bytes[..8], &[0x00, 0x40, 0x02, 0x00, 0x00, 0x40, 0x03, 0x00]);
}

#[test]
fn a_local_got16_names_the_page_and_its_lo16_the_offset() {
    let mut gp = Gp::new(0x40_1000, 4, 4, true);
    let local = Target { local: true, ..target(0x41_0000) };
    let mut field = 0x8f82_0000u32.to_le_bytes();
    apply_one(&mut gp, Architecture::Mips, relocation(GOT16, 0, true), &mut field, local, 0x40_0000, Some(0x1234)).unwrap();
    let slot = (gp.gp() as i64 + i64::from(u32::from_le_bytes(field) as u16 as i16)) as u64;
    assert_eq!(slot, 0x40_1000);
    assert_eq!(&gp.bytes()[..4], &0x41_0000u32.to_le_bytes());
}

#[test]
fn n64_chains_hi16_and_lo16_of_minus_gp_rel() {
    let mut gp = Gp::new(0x40_1000, 8, 0, false);
    let n64 = Architecture::Mips64;
    let f = 0x40_0040u64;
    let hi = GPREL16 | (SUB << 8) | (HI16 << 16);
    let lo = GPREL16 | (SUB << 8) | (LO16 << 16);
    let lui = apply_be(&mut gp, n64, relocation(hi, 0, false), 0x3c01_0000, target(f), f, None).unwrap();
    let daddiu = apply_be(&mut gp, n64, relocation(lo, 0, false), 0x643c_0000, target(f), f + 8, None).unwrap();
    let rebuilt = (i64::from(lui as u16 as i16) << 16) + i64::from(daddiu as u16 as i16) + f as i64;
    assert_eq!(rebuilt as u64, gp.gp());
}

#[test]
fn gpdword_writes_the_gp_relative_doubleword() {
    let mut gp = Gp::new(0x40_1000, 8, 0, false);
    let t = GPREL32 | (R64 << 8);
    let mut field = [0u8; 8];
    apply_one(&mut gp, Architecture::Mips64, relocation(t, 0x10, false), &mut field, target(0x40_0000), 0x40_0800, None).unwrap();
    assert_eq!(i64::from_be_bytes(field), 0x40_0010 - gp.gp() as i64);
}

#[test]
fn pc16_is_relative_to_its_branch() {
    let mut gp = Gp::new(0x40_1000, 4, 0, false);
    // `bal f` against an external symbol carries the -4 of the delay slot.
    let bal = apply_be(&mut gp, Architecture::Mips, relocation(PC16, 0, true), 0x0411_ffff, target(0x40_0100), 0x40_0000, None).unwrap();
    assert_eq!(bal, 0x0411_003f);
}

#[test]
fn a_full_got_and_an_out_of_range_offset_fail_without_writing() {
    let mut gp = Gp::new(0x40_1000, 4, 1, false);
    let o32 = Architecture::Mips;
    apply_be(&mut gp, o32, relocation(CALL16, 0, true), 0x8f99_0000, target(0x40_0200), 0, None).unwrap();
    assert_eq!(
        apply_be(&mut gp, o32, relocation(CALL16, 0, true), 0x8f99_0000, target(0x40_0300), 0, None),
        Err(RelocationFailure::OutOfRange)
    );
    assert_eq!(
        apply_be(&mut gp, o32, relocation(GPREL16, 0, true), 0x8f82_0000, target(0x50_0000), 0, None),
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

fn fixture(name: &str) -> Vec<u8> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures/octeon-relocations");
    std::fs::read(dir.join(name)).unwrap()
}

fn layout(bytes: &[u8]) -> super::super::reloc_object::RelocLayout {
    let file = object::File::parse(bytes).unwrap();
    super::super::reloc_object::layout_relocatable(&file, &super::super::format::elf::ElfFormat)
}

fn read_at(data: &[u8], at: u64, width: usize, le: bool) -> u64 {
    read(&data[at as usize..at as usize + width], le)
}

/// The `_gp` a GNU-linked image defines.
fn gnu_gp(linked: &object::File) -> u64 {
    linked.symbols().find(|s| s.name() == Ok("_gp")).unwrap().address()
}

#[test]
fn pc32_fields_and_composed_results_match_gnu_in_both_abis_and_byte_orders() {
    for abi in ["n32", "64"] {
        for endian in ["be", "le"] {
            let stem = format!("pc32-{abi}-{endian}");
            let bytes = fixture(&format!("{stem}.o"));
            let linked_bytes = fixture(&format!("{stem}.elf"));
            let file = object::File::parse(&*bytes).unwrap();
            let linked = object::File::parse(&*linked_bytes).unwrap();
            let arch = if abi == "n32" { Architecture::Mips64_N32 } else { Architecture::Mips64 };
            assert_eq!(file.architecture(), arch);
            let section = file.section_by_name(".text").unwrap();
            let records: Vec<_> = section.relocations().collect();
            assert!(records.iter().all(|(_, r)| !r.has_implicit_addend()));
            assert_eq!(records[0].1.addend(), -4);
            if abi == "n32" {
                assert_eq!((records[0].0, records[1].0), (0, 0));
                assert_eq!(records[1].1.addend(), -4);
                assert_eq!(records[1].1.flags(), object::RelocationFlags::Elf { r_type: PC32 });
            } else {
                assert_eq!(records[0].1.flags(), object::RelocationFlags::Elf { r_type: R32 | PC32 << 8 });
            }
            let le = endian == "le";
            assert_eq!(read_at(section.data().unwrap(), 0, 4, le), 0xffff_fffc);
            let expected = linked.section_by_name(".text").unwrap();
            assert_eq!(expected.address(), 0x40_0000);
            assert_eq!(read_at(expected.data().unwrap(), 0, 4, le), 0xffbf_fffc);
            assert_eq!(read_at(expected.data().unwrap(), 4, 4, le), 0x7fbf_fffc);
            if abi == "64" {
                assert_eq!(read_at(expected.data().unwrap(), 8, 8, le), 0xffbf_fff4);
                assert_eq!(read_at(expected.data().unwrap(), 16, 4, le), 0x9a7c_dee0);
            }
            let layout = layout(&bytes);
            let address = layout.section_vma[&section.index()];
            assert_eq!(address, expected.address());
            let (_, actual) = layout.segments.iter().find(|(base, _)| *base == address).unwrap();
            assert_eq!(actual, expected.data().unwrap(), "{stem}");
            assert!(layout.diagnostics.is_empty(), "{stem}: {:?}", layout.diagnostics.report_lines());
        }
    }
}

#[test]
fn pc32_wraps_boundaries_without_relaxing_narrow_relocation_checks() {
    for entry in [4, 8] {
        let arch = if entry == 8 { Architecture::Mips64 } else { Architecture::Mips64_N32 };
        for le in [false, true] {
            let mut gp = Gp::new(0x40_1000, entry, 0, le);
            for (symbol, addend, place, expected) in [
                (0, -4, 0x40_0000, 0xffbf_fffc),
                (0, 0, 0x8000_0004, 0x7fff_fffc),
                (0x8000_0000, 0, 0, 0x8000_0000),
                (0x1234_5678_9abc_def0, 0, 0x40_0000, 0x9a7c_def0),
                (0, 0x1_0000_0007, 0x40_0000, 0xffc0_0007),
            ] {
                let mut field = [0xa5; 4];
                apply_one(&mut gp, arch, relocation(PC32, addend, false), &mut field, target(symbol), place, None).unwrap();
                assert_eq!(read(&field, le), expected, "entry {entry}, le {le}, addend {addend}");
            }
            for (op, addend, reason) in [
                (R16, -0x8001, RelocationFailure::OutOfRange),
                (PC16, 0x4_0000, RelocationFailure::OutOfRange),
                (PC16, 2, RelocationFailure::Misaligned),
            ] {
                let mut field = [0xa5; 4];
                assert_eq!(apply_one(&mut gp, arch, relocation(op, addend, false), &mut field, target(0), 0, None), Err(reason));
                assert_eq!(field, [0xa5; 4]);
            }
        }
    }
}

/// Consecutive N32 REL records at one offset chain as GNU ld chains them: the
/// first record's implicit addend seeds the expression, each result is the
/// next record's addend, and only the last record writes. Two `R_MIPS_32`
/// accumulate; an `R_MIPS_32` followed by `HI16` or `LO16` keeps the field's
/// other half; `GPREL16`, `SUB`, `HI16` spell `%hi(%neg(%gp_rel(x)))`.
#[test]
fn n32_rel_same_offset_chains_match_gnu_in_both_byte_orders() {
    for endian in ["be", "le"] {
        let le = endian == "le";
        let bytes = fixture(&format!("rel-chain-n32-{endian}.o"));
        let linked_bytes = fixture(&format!("rel-chain-n32-{endian}.elf"));
        let file = object::File::parse(&*bytes).unwrap();
        let linked = object::File::parse(&*linked_bytes).unwrap();
        assert_eq!(file.architecture(), Architecture::Mips64_N32);
        let section = file.section_by_name(".rodata").unwrap();
        let records: Vec<_> = section.relocations().collect();
        assert_eq!(records.iter().map(|(at, _)| *at).collect::<Vec<_>>(), [0, 0, 4, 4, 8, 8, 0xc, 0xc, 0x10, 0x10, 0x10]);
        assert!(records.iter().all(|(_, r)| r.has_implicit_addend()));
        let types: Vec<u32> = records.iter().map(|(_, r)| r_type_of(r).unwrap()).collect();
        assert_eq!(types, [R32, R32, R32, R32, R32, HI16, R32, LO16, GPREL16, SUB, HI16]);
        let initial = section.data().unwrap();
        assert_eq!(read_at(initial, 0, 4, le), 7);
        assert_eq!(read_at(initial, 4, 4, le), 0xffff_fff0);
        assert_eq!(read_at(initial, 8, 4, le), 7);
        assert_eq!(read_at(initial, 0xc, 4, le), 0x0005_0007);
        assert_eq!(read_at(initial, 0x10, 4, le), 0x3c01_0000);
        let expected = linked.section_by_name(".rodata").unwrap();
        let words: Vec<u64> = (0..5).map(|i| read_at(expected.data().unwrap(), i * 4, 4, le)).collect();
        let gp = gnu_gp(&linked);
        let neg_gp_rel = 0u64.wrapping_sub(0x1234_0000u64.wrapping_sub(gp));
        assert_eq!(words, [0x68ac_0007, 0x10, 0x68ac, 0x0005_123b, 0x3c01_0000 | (neg_gp_rel.wrapping_add(0x8000) >> 16 & 0xffff)]);
        let layout = layout(&bytes);
        assert_eq!(gp, layout.gp.unwrap(), "{endian}: the script's _gp is the loader's");
        let address = layout.section_vma[&section.index()];
        let (_, actual) = layout.segments.iter().find(|(base, _)| *base == address).unwrap();
        assert_eq!(actual, expected.data().unwrap(), "{endian}");
        assert!(layout.diagnostics.is_empty(), "{endian}: {:?}", layout.diagnostics.report_lines());
    }
}

/// A record that cannot be applied fails the whole chain it belongs to, which
/// writes nothing and reports each of its records; the chain at the next
/// offset is unaffected.
#[test]
fn a_damaged_n32_rel_record_fails_its_whole_chain() {
    for endian in ["be", "le"] {
        for bad_record in [0, 1] {
            let mut bytes = fixture(&format!("rel-chain-n32-{endian}.o"));
            let file = object::File::parse(&*bytes).unwrap();
            let table = file.section_by_name(".rel.rodata").unwrap().file_range().unwrap().0;
            let at = table + bad_record * 8 + 4;
            let info = read_at(&bytes, at, 4, endian == "le");
            write(&mut bytes[at as usize..at as usize + 4], endian == "le", (info & !0xff) | 42);
            let file = object::File::parse(&*bytes).unwrap();
            let section = file.section_by_name(".rodata").unwrap();
            let layout = layout(&bytes);
            let address = layout.section_vma[&section.index()];
            let (_, data) = layout.segments.iter().find(|(base, _)| *base == address).unwrap();
            assert_eq!(read_at(data, 0, 4, endian == "le"), 7, "{endian} record {bad_record}");
            assert_eq!(read_at(data, 4, 4, endian == "le"), 0x10);
            assert_eq!(read_at(data, 8, 4, endian == "le"), 0x68ac);
            assert_eq!(layout.diagnostics.total(), 2, "{endian} record {bad_record}: both records of the chain");
        }
    }
}

/// GNU ld reads a section's REL and RELA records into one array and chains by
/// offset alone, so a RELA record continuing a REL chain takes the saved value,
/// not its own addend.
#[test]
fn same_offset_records_chain_across_rel_and_rela_as_gnu_ld() {
    let records = [(0, relocation(R32, 0, true)), (0, relocation(R32, 11, false)), (0, relocation(R32, 0, true))];
    for le in [false, true] {
        let mut field = if le { 7u32.to_le_bytes() } else { 7u32.to_be_bytes() };
        let mut gp = Gp::new(0x40_0000, 4, 0, le);
        let applied = gp.apply_section(
            Architecture::Mips64_N32, &mut field, 0x40_0000, &records, &[None; 3],
            Ok(0), &mut |_| Some(target(0x100)),
        );
        assert_eq!(read(&field, le), 0x307);
        assert!(applied.failures.is_empty());
    }
}

/// Original N32/N64 and o32 objects whose GNU-linked images put `.text` and
/// `$gp` where the loader does: composed N32 records, packed and spanning N64
/// records, REL HI/LO pairs and same-offset chains, GP-relative and literal
/// fields, a partial link's input `$gp`, and calls through composed expressions
/// all match GNU ld. The gp objects define `_gp` themselves, far from the
/// layout; the linker script's `_gp` overrides it, as the default script's does.
#[test]
fn relocated_text_matches_the_gnu_linked_originals() {
    let mut stems = vec!["implicit-32-be".to_string(), "gp-32-be".to_string()];
    for kind in ["expressions", "call-composed", "gp", "gp-partial"] {
        for abi in ["n32", "64"] {
            for endian in ["be", "le"] {
                stems.push(format!("{kind}-{abi}-{endian}"));
            }
        }
    }
    for stem in stems {
        let object_bytes = fixture(&format!("{stem}.o"));
        let linked_bytes = fixture(&format!("{stem}.elf"));
        let linked = object::File::parse(&*linked_bytes).unwrap();
        let layout = layout(&object_bytes);
        assert!(layout.diagnostics.is_empty(), "{stem}: {:?}", layout.diagnostics.report_lines());
        if stem.starts_with("gp-") {
            let file = object::File::parse(&*object_bytes).unwrap();
            let own = file.symbols().find(|s| s.name() == Ok("_gp")).unwrap();
            assert!(!own.is_undefined(), "{stem}: the object defines _gp");
            if !stem.starts_with("gp-partial") {
                assert_eq!(own.address() as u32, 0x8000_8000, "{stem}: far from the layout");
            }
            assert_eq!(gnu_gp(&linked), layout.gp.unwrap(), "{stem}: the linker's _gp, not the object's");
        }
        let text = linked.section_by_name(".text").unwrap();
        let (_, laid) = layout.segments.iter().find(|(a, _)| *a == text.address()).unwrap();
        assert_eq!(laid, text.data().unwrap(), "{stem}");
    }
}

/// Each GOT load reaches a slot holding what the GNU-linked GOT's slot holds;
/// the slots may be ordered differently, and every other bit matches.
#[test]
fn got_loads_reach_the_values_gnu_linked() {
    let mut stems = vec!["got-32-be".to_string()];
    for abi in ["n32", "64"] {
        for endian in ["be", "le"] {
            stems.push(format!("got-{abi}-{endian}"));
        }
    }
    for stem in stems {
        let object_bytes = fixture(&format!("{stem}.o"));
        let linked_bytes = fixture(&format!("{stem}.elf"));
        let file = object::File::parse(&*object_bytes).unwrap();
        let linked = object::File::parse(&*linked_bytes).unwrap();
        let le = file.is_little_endian();
        let width = if file.is_64() { 8 } else { 4 };
        let layout = layout(&object_bytes);
        assert!(layout.diagnostics.is_empty(), "{stem}: {:?}", layout.diagnostics.report_lines());
        let gp = layout.got_range.unwrap().0 + GP_BIAS;
        let gnu_gp = gnu_gp(&linked);
        let gnu_got = linked.section_by_name(".got").unwrap();
        let text = linked.section_by_name(".text").unwrap();
        let mut expected = text.data().unwrap().to_vec();
        let (_, laid) = layout.segments.iter().find(|(a, _)| *a == text.address()).unwrap();
        let mut actual = laid.clone();
        let value = |address: u64| {
            let (base, data) = layout
                .segments
                .iter()
                .find(|(base, data)| address >= *base && address + width <= base + data.len() as u64)
                .unwrap();
            read_at(data, address - base, width as usize, le)
        };
        let mut slots = 0;
        for (offset, reloc) in file.section_by_name(".text").unwrap().relocations() {
            let RelocationFlags::Elf { r_type } = reloc.flags() else { unreachable!() };
            let weak = match reloc.target() {
                RelocationTarget::Symbol(i) => file.symbol_by_index(i).is_ok_and(|s| s.is_undefined() && s.is_weak()),
                _ => false,
            };
            // A split offset's high half is 0 in both small GOTs, so its low half
            // alone locates the slot; the `lui` halves are compared as other bits.
            if weak || !matches!(r_type & 0xff, GOT16 | CALL16 | GOT_DISP | GOT_PAGE | GOT_LO16 | CALL_LO16) {
                continue;
            }
            let ours = read_at(&actual, offset, 4, le);
            let gnu = read_at(&expected, offset, 4, le);
            let slot = gp.wrapping_add(ours as u16 as i16 as i64 as u64);
            let gnu_slot = gnu_gp.wrapping_add(gnu as u16 as i16 as i64 as u64);
            let gnu_value = read_at(gnu_got.data().unwrap(), gnu_slot - gnu_got.address(), width as usize, le);
            assert_eq!(value(slot), gnu_value, "{stem} at {offset:#x}");
            for word in [&mut actual, &mut expected] {
                let at = offset as usize;
                let masked = read_at(word, offset, 4, le) & !0xffff;
                write(&mut word[at..at + 4], le, masked);
            }
            slots += 1;
        }
        assert!(slots >= 5, "{stem}");
        assert_eq!(actual, expected, "{stem}: bits outside the GOT offsets");
    }
}

fn synthetic(arch: object::Architecture, endian: Endianness, relocations: &[(u64, u32, u64)]) -> Vec<u8> {
    let mut obj = Writer::new(BinaryFormat::Elf, arch, endian);
    let data = obj.section_id(object::write::StandardSection::Data);
    obj.append_section_data(data, &[0xa5; 16], 8);
    for (index, &(offset, r_type, value)) in relocations.iter().enumerate() {
        let symbol = obj.add_symbol(Symbol {
            name: format!("target{index}").into_bytes(),
            value,
            size: 0,
            kind: SymbolKind::Data,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Absolute,
            flags: SymbolFlags::None,
        });
        obj.add_relocation(data, WriteRelocation { offset, symbol, addend: 0, flags: RelocationFlags::Elf { r_type } })
            .unwrap();
    }
    obj.write().unwrap()
}

/// A chain with an unsupported operation fails as a whole, leaving its field
/// alone; an independent field beside it is still written.
#[test]
fn an_unsupported_operation_fails_its_whole_expression() {
    const TLS_GD: u32 = 42;
    for (arch, records) in [
        (Architecture::Mips64_N32, vec![(0, R32, 0x1234_5678), (0, TLS_GD, 0), (8, R32, 0x1234_5678)]),
        (Architecture::Mips64, vec![(0, R32 | (TLS_GD << 8), 0x1234_5678), (8, R32, 0x1234_5678)]),
        (Architecture::Mips64, vec![(0, R32 | (SUB << 8) | (HI16 << 16) | (19 << 24), 0x1234_5678), (8, R32, 0x1234_5678)]),
    ] {
        for endian in [Endianness::Big, Endianness::Little] {
            let layout = layout(&synthetic(arch, endian, &records));
            let data = &layout.segments[0].1;
            assert_eq!(&data[..8], &[0xa5; 8], "{arch:?} {endian:?}");
            assert_eq!(read_at(data, 8, 4, endian == Endianness::Little), 0x1234_5678);
            assert!(!layout.diagnostics.is_empty());
        }
    }
}

/// N64 operations after an `R_MIPS_NONE` in the same record are not a second
/// expression: the record is malformed and writes nothing.
#[test]
fn n64_operations_after_none_reject_the_record() {
    for endian in [Endianness::Big, Endianness::Little] {
        for r_type in [R64 << 8, R64 | (SUB << 16), (SUB << 8) | (R64 << 16), R64 | (255 << 16)] {
            let layout = layout(&synthetic(Architecture::Mips64, endian, &[(0, r_type, 0x1234_8000), (8, R32, 0x5678_1234)]));
            let data = &layout.segments[0].1;
            assert_eq!(&data[..8], &[0xa5; 8], "{endian:?} {r_type:#x}");
            assert_eq!(read_at(data, 8, 4, endian == Endianness::Little), 0x5678_1234);
            assert_eq!(layout.diagnostics.total(), 1);
        }
    }
}

/// A malformed record at the offset of a complete packed record does not
/// continue that record's expression: the expression is written and only the
/// malformed record is reported.
#[test]
fn a_malformed_packed_record_leaves_the_expression_before_it_written() {
    for endian in [Endianness::Big, Endianness::Little] {
        let complete = R64 | (SUB << 8) | (R64 << 16);
        let layout = layout(&synthetic(Architecture::Mips64, endian, &[(8, complete, 0x1234_8000), (8, HI16 << 8, 0x5678_1234)]));
        let data = &layout.segments[0].1;
        assert_eq!(&data[..8], &[0xa5; 8], "{endian:?}");
        assert_eq!(read_at(data, 8, 8, endian == Endianness::Little), 0u64.wrapping_sub(0x1234_8000), "{endian:?}");
        assert_eq!(layout.diagnostics.total(), 1, "{endian:?}");
    }
}

/// An N64 record's `r_ssym` names `0`, `$gp`, the input `$gp` or the place,
/// not a symbol-table entry; `$gp` is the layout's, not the `_gp` the object
/// defines.
#[test]
fn n64_special_symbols_are_linker_values() {
    for endian in [Endianness::Big, Endianness::Little] {
        for ssym in 0..4u32 {
            let mut obj = Writer::new(BinaryFormat::Elf, Architecture::Mips64, endian);
            let data = obj.section_id(object::write::StandardSection::Data);
            obj.append_section_data(data, &[0xa5; 16], 8);
            let add = |obj: &mut Writer, name: &str, value| {
                obj.add_symbol(Symbol {
                    name: name.as_bytes().to_vec(),
                    value,
                    size: 0,
                    kind: SymbolKind::Data,
                    scope: SymbolScope::Linkage,
                    weak: false,
                    section: SymbolSection::Absolute,
                    flags: SymbolFlags::None,
                })
            };
            add(&mut obj, "_gp", 0x60_0088);
            add(&mut obj, "decoy", 0x9876_5432);
            let target = add(&mut obj, "target", 0x1234_8000);
            let options = obj.add_section(Vec::new(), b".MIPS.options".to_vec(), SectionKind::ReadOnlyData);
            let mut record = [0u8; 40];
            record[0] = 1;
            record[1] = 40;
            let gp0 = if endian == Endianness::Big { 0x70_00a0u64.to_be_bytes() } else { 0x70_00a0u64.to_le_bytes() };
            record[32..].copy_from_slice(&gp0);
            obj.append_section_data(options, &record, 8);
            let r_type = R64 | (SUB << 8) | (R64 << 16) | (ssym << 24);
            obj.add_relocation(data, WriteRelocation { offset: 8, symbol: target, addend: 0, flags: RelocationFlags::Elf { r_type } })
                .unwrap();
            let layout = layout(&obj.write().unwrap());
            assert!(layout.diagnostics.is_empty(), "{:?}", layout.diagnostics.report_lines());
            let gp = layout.gp.unwrap();
            assert_ne!(gp, 0x60_0088, "the object's own _gp is overridden");
            let special = [0u64, gp, 0x70_00a0, 0x40_0008][ssym as usize];
            let data = &layout.segments[0].1;
            assert_eq!(&data[..8], &[0xa5; 8]);
            assert_eq!(read_at(data, 8, 8, endian == Endianness::Little), special.wrapping_sub(0x1234_8000), "{endian:?} r_ssym {ssym}");
        }
    }
}

/// Only expressions that read the input `$gp` fail when its record is
/// malformed (a `.reginfo` of the wrong size, which still occupies its place
/// in the layout); the partial link's other fields still match GNU ld.
#[test]
fn a_malformed_input_gp_fails_only_the_fields_that_use_it() {
    for abi in ["n32", "64"] {
        for endian in ["be", "le"] {
            let stem = format!("gp-partial-{abi}-{endian}");
            let mut bytes = fixture(&format!("{stem}.o"));
            let linked_bytes = fixture(&format!("{stem}.elf"));
            let file = object::File::parse(&*bytes).unwrap();
            assert_eq!(input_gp(&file), Ok(if abi == "n32" { 0x40_8050 } else { 0x40_8060 }), "{stem}");
            let le = endian == "le";
            let text = file.section_by_name(".text").unwrap();
            let original = text.data().unwrap().to_vec();
            let dependent: Vec<usize> = text
                .relocations()
                .filter(|(_, r)| match r.target() {
                    RelocationTarget::Symbol(i) => file.symbol_by_index(i).unwrap().name() == Ok("local_target"),
                    _ => false,
                })
                .map(|(offset, _)| offset as usize)
                .collect();
            assert_eq!(dependent.len(), 2, "{stem}: a local literal and a GPREL32");
            let linked = object::File::parse(&*linked_bytes).unwrap();
            let mut expected = linked.section_by_name(".text").unwrap().data().unwrap().to_vec();
            for at in dependent {
                expected[at..at + 4].copy_from_slice(&original[at..at + 4]);
            }
            let section = file.section_by_name(if abi == "n32" { ".reginfo" } else { ".MIPS.options" }).unwrap();
            if abi == "n32" {
                let table = read_at(&bytes, 32, 4, le) as usize;
                let stride = read_at(&bytes, 46, 2, le) as usize;
                let size = table + stride * section.index().0 + 20;
                write(&mut bytes[size..size + 4], le, 23);
            } else {
                let (start, _) = section.file_range().unwrap();
                bytes[start as usize + 1] = 0;
            }
            let file = object::File::parse(&*bytes).unwrap();
            assert!(input_gp(&file).is_err());
            let layout = layout(&bytes);
            assert_eq!(layout.segments[0].1, expected, "{stem}");
            assert_eq!(layout.diagnostics.total(), 2);
        }
    }
}
