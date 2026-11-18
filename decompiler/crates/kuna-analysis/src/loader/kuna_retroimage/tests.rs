use super::*;
use kuna_sleigh::loadimage::section_flags;

macro_rules! fixture {
    ($name:literal) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../tests/fixtures/retro-formats/", $name))
    };
}

const FIXTURES: &[(&str, &[u8])] = &[
    ("hello.psx", fixture!("hello.psx")),
    ("hello.z64", fixture!("hello.z64")),
    ("hello-m68k.aout", fixture!("hello-m68k.aout")),
    ("hello-i386.aout", fixture!("hello-i386.aout")),
    ("hello.hunk", fixture!("hello.hunk")),
    ("hello.pef", fixture!("hello.pef")),
];

fn load(name: &str) -> RetroImage {
    let bytes = FIXTURES.iter().find(|(fixture, _)| *fixture == name).unwrap().1;
    assert!(is_retro_image(&bytes[..RETRO_PROBE_LEN]), "{name} is claimed from its probe prefix");
    parse(name, bytes).unwrap_or_else(|error| panic!("{name}: {}", error.explain()))
}

fn address(image: &RetroImage, name: &str) -> u64 {
    let symbol = image.symbols.iter().find(|symbol| symbol.name == name);
    image.base + symbol.unwrap_or_else(|| panic!("no symbol {name}")).offset
}

fn word_at(image: &RetroImage, address: u64, big: bool) -> u32 {
    word(&image.module, address - image.base, big).unwrap()
}

#[test]
fn psx_maps_the_text_at_t_addr_and_fixes_gp() {
    let image = load("hello.psx");
    assert_eq!((image.format, image.language), ("PS-X EXE", "MIPS:LE:32:default"));
    assert_eq!(image.base, 0x8001_0000);
    assert!(image.entry > image.base && image.entry < image.base + 0x800);
    assert_eq!(image.tracked.len(), 1);
    assert_eq!(image.tracked[0].0, "gp");
    assert_eq!(image.sections[0].file_range, Some((0x800, 0x800)));
    assert_eq!(image.sections[1].name, ".bss");
    assert_eq!(image.sections[1].offset, 0x800);
    assert_eq!(image.module.len() as u64, 0x800 + image.sections[1].size);
}

#[test]
fn psx_rejects_an_entry_outside_the_stored_text() {
    let mut bytes = fixture!("hello.psx").to_vec();
    bytes[0x10..0x14].copy_from_slice(&0x8002_0000u32.to_le_bytes());
    let error = parse("bad.psx", &bytes).unwrap_err();
    assert!(error.explain().contains("pc0 is outside"), "{}", error.explain());
    let error = parse("short.psx", &fixture!("hello.psx")[..0x800]).unwrap_err();
    assert!(error.explain().contains("no text follows"), "{}", error.explain());
}

#[test]
fn n64_reads_every_dump_byte_order_alike() {
    let big = fixture!("hello.z64").to_vec();
    let mut swapped = big.clone();
    swapped.chunks_exact_mut(2).for_each(|pair| pair.swap(0, 1));
    let mut little = big.clone();
    little.chunks_exact_mut(4).for_each(|word| word.reverse());
    let image = load("hello.z64");
    assert_eq!((image.language, image.base, image.entry), ("MIPS:BE:64:64-32addr", 0x8000_0400, 0x8000_0400));
    assert_eq!(image.module, big[0x1000..]);
    for dump in [swapped, little] {
        assert!(is_retro_image(&dump));
        let other = parse("dump", &dump).unwrap();
        assert_eq!((other.base, other.entry, &other.module), (image.base, image.entry, &image.module));
    }
}

#[test]
fn n64_maps_at_most_the_megabyte_the_boot_code_copies() {
    let mut rom = fixture!("hello.z64").to_vec();
    rom.resize(0x1000 + 0x20_0000, 0);
    assert_eq!(parse("big.z64", &rom).unwrap().module.len(), 0x10_0000);
}

#[test]
fn aout_executable_loads_at_its_page_with_its_symbols() {
    let image = load("hello-m68k.aout");
    assert_eq!((image.language, image.base), ("68000:BE:32:MC68020", 0x2000));
    assert_eq!(image.entry, address(&image, "main"));
    let names: Vec<&str> = image.sections.iter().map(|section| section.name.as_str()).collect();
    assert_eq!(names, [".text", ".data", ".bss"]);
    assert_eq!(image.sections[1].offset, 0x2000, "data begins on the page after the text");
    assert_eq!(word_at(&image, address(&image, "counter"), true), 7);
    assert_eq!(u64::from(word_at(&image, address(&image, "handler"), true)), address(&image, "scale"));
    assert!(image.symbols.iter().find(|symbol| symbol.name == "sum").unwrap().code);
    assert!(!image.symbols.iter().find(|symbol| symbol.name == "table").unwrap().code);
}

#[test]
fn aout_object_gives_undefined_symbols_addresses() {
    let image = load("hello-i386.aout");
    assert_eq!((image.language, image.base), ("x86:LE:32:default", 0));
    assert_eq!(image.entry, address(&image, "scale"), "an object's entry is its first function");
    let puts = address(&image, "puts");
    let external = image.sections.last().unwrap();
    assert_eq!(external.name, ".extern");
    assert_eq!(puts, external.offset);
    let main = address(&image, "main");
    let text = &image.module[main as usize..image.sections[0].size as usize];
    let call = text.windows(5).enumerate().find_map(|(at, bytes)| {
        let next = main + at as u64 + 5;
        let displacement = i32::from_le_bytes(bytes[1..].try_into().unwrap());
        (bytes[0] == 0xe8 && next.wrapping_add_signed(displacement.into()) == puts).then_some(at)
    });
    assert!(call.is_some(), "main's call to puts lands on the slot the loader gave it");
    assert_eq!(u64::from(word_at(&image, address(&image, "handler"), false)), address(&image, "scale"));
}

#[test]
fn aout_dialects_place_the_text_where_their_headers_say() {
    fn header(magic: [u8; 4], big: bool, entry: u32) -> Vec<u8> {
        let mut bytes = vec![0u8; 0x3000];
        bytes[..4].copy_from_slice(&magic);
        let put = |bytes: &mut [u8], at: usize, value: u32| {
            bytes[at..at + 4].copy_from_slice(&if big { value.to_be_bytes() } else { value.to_le_bytes() });
        };
        put(&mut bytes, 4, 0x1000);
        put(&mut bytes, 20, entry);
        bytes
    }
    let cases: &[([u8; 4], bool, u32, &str, u64)] = &[
        ([0x00, 0x87, 0x01, 0x0b], true, 0x2020, "68000:BE:32:MC68020", 0x2000),
        ([0x00, 0x88, 0x01, 0x0b], true, 0x1020, "68000:BE:32:MC68020", 0x1000),
        ([0x00, 0x86, 0x01, 0x0b], false, 0x1020, "x86:LE:32:default", 0x1000),
        ([0x00, 0x8a, 0x01, 0x08], true, 0x2000, "sparc:BE:32:default", 0x2000),
        ([0x00, 0x03, 0x01, 0x0b], true, 0x2020, "sparc:BE:32:default", 0x2000),
        ([0x80, 0x02, 0x01, 0x07], true, 0x2000, "68000:BE:32:MC68020", 0x2000),
        ([0x81, 0x03, 0x01, 0x0b], true, 0x2020, "sparc:BE:32:default", 0x2000),
        ([0x00, 0x8f, 0x01, 0x0b], false, 0x1020, "ARM:LE:32:v4", 0x1000),
        ([0x0b, 0x01, 0x64, 0x00], false, 0, "x86:LE:32:default", 0),
        ([0xcc, 0x00, 0x64, 0x00], false, 0x1020, "x86:LE:32:default", 0x1000),
        ([0xcc, 0x00, 0x86, 0x00], false, 0x1020, "x86:LE:32:default", 0x1000),
        ([0x07, 0x01, 0x64, 0x00], false, 0x10_0000, "x86:LE:32:default", 0x10_0000),
    ];
    for &(magic, big, entry, language, base) in cases {
        let bytes = header(magic, big, entry);
        assert!(is_retro_image(&bytes[..RETRO_PROBE_LEN]), "{magic:02x?}");
        let image = parse("a.out", &bytes).unwrap_or_else(|error| panic!("{magic:02x?}: {}", error.explain()));
        assert_eq!((image.language, image.base, image.entry), (language, base, u64::from(entry)), "{magic:02x?}");
    }
    for magic in [[0x00, 0x87, 0x01, 0x0c], [0x00, 0x95, 0x01, 0x0b], [0x0b, 0x01, 0x87, 0x00], [0x0b, 0x01, 0x00, 0x00]] {
        assert!(!is_retro_image(&header(magic, true, 0)), "{magic:02x?} is not a known a.out");
    }
}

#[test]
fn hunk_relocations_join_separately_placed_hunks() {
    let image = load("hello.hunk");
    assert_eq!((image.language, image.base, image.entry), ("68000:BE:32:default", hunk::BASE, hunk::BASE));
    assert_eq!(address(&image, "main"), image.entry);
    let kinds: Vec<u32> = image.sections.iter().map(RetroSection::flags).collect();
    assert_eq!(kinds, [section_flags::CODE, section_flags::DATA, section_flags::DATA]);
    assert_eq!(image.sections[2].file_range, None, "the bss hunk holds no file bytes");
    let data = image.base + image.sections[1].offset;
    let (counter, handler, scratch) = (address(&image, "counter"), address(&image, "handler"), address(&image, "scratch"));
    assert!(counter >= data && handler >= data, "data symbols are placed in the data hunk");
    assert!(scratch >= image.base + image.sections[2].offset, "the zeroed word is in the bss hunk");
    assert_eq!(word_at(&image, counter, true), 7);
    assert_eq!(u64::from(word_at(&image, handler, true)), address(&image, "scale"), "RELOC32SHORT, data to code");
    let text = &image.module[..image.sections[0].size as usize];
    let loads_counter = text.windows(4).any(|bytes| u64::from(u32::from_be_bytes(bytes.try_into().unwrap())) == counter);
    assert!(loads_counter, "RELOC32, code to data");
}

#[test]
fn hunk_stops_at_an_overlay_and_refuses_an_unknown_block() {
    let mut overlay = fixture!("hello.hunk").to_vec();
    let end = overlay.len() - 4;
    overlay[end..].copy_from_slice(&0x3f5u32.to_be_bytes());
    let image = parse("overlay", &overlay).unwrap();
    assert!(image.notes.iter().any(|note| note.contains("overlay")), "{:?}", image.notes);
    overlay[end..].copy_from_slice(&0x3e7u32.to_be_bytes());
    let error = parse("unit", &overlay).unwrap_err();
    assert!(error.explain().contains("unknown block type 0x3e7"), "{}", error.explain());
}

#[test]
fn pef_relocates_vectors_and_names_code_from_three_sources() {
    let image = load("hello.pef");
    assert_eq!((image.language, image.base), ("PowerPC:BE:32:default", pef::BASE));
    let main = address(&image, "main");
    assert_eq!(image.entry, main, "the main symbol is a transition vector; the entry is the code it names");
    let (register, toc) = image.tracked[0];
    assert_eq!(register, "r2");
    let data = image.base + image.sections[1].offset;
    assert!(toc > data && toc < data + image.sections[1].size, "the TOC is in the data section");
    assert_eq!(word_at(&image, address(&image, "counter"), true), 7);
    for name in ["scale", "checksum", "sum", "main"] {
        let symbol = image.symbols.iter().find(|symbol| symbol.name == name).unwrap();
        assert!(symbol.code, "{name} is exported as a transition vector and named at its code");
    }
    let glue = image.symbols.iter().find(|symbol| symbol.name == "puts" && symbol.code).unwrap();
    assert_eq!(&image.module[glue.offset as usize..][..2], [0x81, 0x82], "the glue stub loads its vector from the TOC");
    let slot = image.symbols.iter().find(|symbol| symbol.name == "puts" && !symbol.code).unwrap();
    assert_eq!(image.sections.last().unwrap().name, "imports");
    assert_eq!(slot.offset, image.sections.last().unwrap().offset);
    let handler = word_at(&image, address(&image, "handler"), true);
    assert_eq!(u64::from(word_at(&image, u64::from(handler), true)), address(&image, "scale"));
}

#[test]
fn pef_names_a_function_from_its_traceback_table() {
    let mut stripped = fixture!("hello.pef").to_vec();
    let image = load("hello.pef");
    let loader_at = u32::from_be_bytes(stripped[40 + 2 * 28 + 20..][..4].try_into().unwrap()) as usize;
    stripped[loader_at + 52..loader_at + 56].fill(0);
    let unexported = parse("stripped.pef", &stripped).unwrap();
    assert_eq!(address(&unexported, "checksum"), address(&image, "checksum"));
    assert!(!unexported.symbols.iter().any(|symbol| symbol.name == "counter"), "data names came from the exports");
}

#[test]
fn pef_pattern_data_expands_every_opcode() {
    let mut out = [0xffu8; 32];
    let packed = [
        0x03, // zero 3
        0x22, 0xaa, 0xbb, // blockCopy 2
        0x42, 0x01, 0x11, 0x22, // repeatedBlock size 2, 2 times
        0x61, 0x02, 0x02, 0xc0, 0x01, 0x02, 0x03, 0x04, // interleave copy: common 1, custom 2, 2 repeats
        0x82, 0x01, 0x02, 0xd1, 0xd2, // interleave zero: common 2, custom 1, 2 repeats
        0x00, 0x81, 0x00, // zero 128: past the end
    ];
    assert_eq!(pef::unpack_pattern(&packed, &mut out), None, "a run past the section's end is malformed");
    let expected = [
        0, 0, 0, 0xaa, 0xbb, 0x11, 0x22, 0x11, 0x22, 0xc0, 0x01, 0x02, 0xc0, 0x03, 0x04, 0xc0, 0, 0, 0xd1, 0, 0, 0xd2, 0, 0,
    ];
    assert_eq!(out[..24], expected);
    assert_eq!(pef::unpack_pattern(&[0x40, 0x00, 0xff, 0xff, 0xff, 0x7f], &mut out), None, "an empty block cannot repeat");
}

#[test]
fn pef_relocation_program_runs_every_instruction() {
    let sections = [
        pef::Section::placed_at(0, 0x100, 0),
        pef::Section::placed_at(0x100, 0x100, 0x40),
    ];
    let code = pef::BASE as u32;
    let data = code + 0x100 - 0x40;
    let import = |index: u32| code + 0x200 + index * 8;
    let mut module = vec![0u8; 0x200 + 4 * 8];
    let program = [
        0x0042, // skip 1 word, 2 words by sectionD
        0x4001, // 2 words by sectionC
        0x4400, // one 12-byte vector
        0x4600, // one 8-byte vector
        0x4800, // one vtable word and a skipped one
        0x4a01, // two imports
        0x6003, // import 3
        0x6600, // by section 0
        0x8003, // skip 4 bytes
        0x6201, // sectionC = section 1
        0x4000, // by sectionC
        0x9001, // repeat the last block twice
        0xa000, 0x00f0, // position 0xf0
        0xa400, 0x0002, // import 2
        0xb400, 0x0000, // by section 0
        0xb0c0, 0x0001, // repeat the last four blocks once: import 2, section 0
    ];
    let mut relocator = pef::Relocator::new(&mut module, &sections, 1, 0x200, 4).unwrap();
    relocator.run(&program).unwrap();
    let words: Vec<u32> = (0x100..0x200).step_by(4).map(|at| word(&module, at, true).unwrap()).collect();
    let expected = [
        0, data, data, code, code, code, data, 0, code, data, data, 0, import(0), import(1), import(3), code, 0, data,
        data, data,
    ];
    assert_eq!(words[..expected.len()], expected);
    assert_eq!(words[0xf0 / 4..], [import(2), code, import(2), code]);
    let mut relocator = pef::Relocator::new(&mut module, &sections, 1, 0x200, 4).unwrap();
    assert_eq!(relocator.run(&[0x6005]), None, "an import index past the table is malformed");
    let mut relocator = pef::Relocator::new(&mut module, &sections, 1, 0x200, 4).unwrap();
    assert_eq!(relocator.run(&[0x6201, 0xb03f, 0xffff]), None, "a repeat that patches nothing runs out of budget");
}

/// A container is untrusted input: no prefix of one and no single corrupted
/// byte may panic, hang or map more than the size limit.
#[test]
fn truncated_and_corrupted_containers_never_panic() {
    for &(name, bytes) in FIXTURES {
        for cut in (0..bytes.len()).step_by(if bytes.len() > 2048 { 7 } else { 1 }) {
            if is_retro_image(&bytes[..cut]) {
                parse(name, &bytes[..cut]).ok();
            }
        }
        let mut mutated = bytes.to_vec();
        let header = bytes.len().min(0x60);
        let tail = bytes.len().saturating_sub(0x200);
        for at in (0..header).chain(tail..bytes.len()) {
            for value in [0x00, 0x01, 0x7f, 0x80, 0xff] {
                let original = std::mem::replace(&mut mutated[at], value);
                if let Ok(image) = parse(name, &mutated) {
                    assert!(image.module.len() as u64 <= MAX_MODULE, "{name} byte {at:#x} = {value:#x}");
                    assert!(image.entry >= image.base && image.entry - image.base < image.module.len() as u64);
                }
                mutated[at] = original;
            }
        }
    }
}

/// The five probes read a few header bytes each; none may claim a file of a
/// format another loader owns.
#[test]
fn no_other_fixture_is_claimed() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut pending = vec![root.join("tests/fixtures"), root.join("../../../tests/fixtures"), root.join("../../../tests/datatests")];
    let mut seen = 0;
    while let Some(dir) = pending.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                if !path.ends_with("retro-formats") {
                    pending.push(path);
                }
                continue;
            }
            let bytes = std::fs::read(&path).unwrap();
            seen += 1;
            assert!(!is_retro_image(&bytes[..bytes.len().min(RETRO_PROBE_LEN)]), "{} is claimed", path.display());
        }
    }
    assert!(seen > 1000, "the walk reached the fixture corpus ({seen} files)");
}
