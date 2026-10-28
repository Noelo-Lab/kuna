use crate::common;
use object::Object as _;
use std::process::Command;

#[derive(Clone, Copy, Debug)]
enum Plt {
    Bss,
    Short,
    Long,
    Absolute,
}

struct Fixture {
    plt: Plt,
    import: &'static str,
    symbolized: bool,
    padded: bool,
    little: bool,
    relocation: u32,
    addend: u32,
    got2: bool,
    got_tag: bool,
    bad_stub: bool,
}
impl Fixture {
    fn new(plt: Plt) -> Self {
        Self {
            plt,
            import: "__stack_chk_fail",
            symbolized: false,
            padded: false,
            little: false,
            relocation: object::elf::R_PPC_JMP_SLOT,
            addend: 0,
            got2: false,
            got_tag: true,
            bad_stub: false,
        }
    }
    fn target(&self) -> u32 {
        if matches!(self.plt, Plt::Bss) {
            0x3048
        } else {
            0x1800
        }
    }
    fn word(&self, n: u32) -> [u8; 4] {
        if self.little {
            n.to_le_bytes()
        } else {
            n.to_be_bytes()
        }
    }
    fn half(&self, n: u16) -> [u8; 2] {
        if self.little {
            n.to_le_bytes()
        } else {
            n.to_be_bytes()
        }
    }
    fn words(&self, words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|&n| self.word(n)).collect()
    }
    fn symbol(&self, name: u32, value: u32, size: u32, info: u8, section: u16) -> Vec<u8> {
        let mut b = self.words(&[name, value, size]);
        b.extend([info, 0]);
        b.extend(self.half(section));
        b
    }

    /// Authored ELF32 image, independent of an external assembler or linker.
    fn bytes(&self) -> Vec<u8> {
        struct Section {
            name: &'static str,
            kind: u32,
            flags: u32,
            addr: u32,
            data: Vec<u8>,
            size: u32,
            link: u32,
            info: u32,
            entsize: u32,
        }
        let mut sections = Vec::new();
        let mut section = |name, kind, flags, addr, data: Vec<u8>, size, link, info, entsize| {
            sections.push(Section {
                name,
                kind,
                flags,
                addr,
                data,
                size,
                link,
                info,
                entsize,
            });
        };
        let returning = self.import == "returning_import";
        let mut code = self.words(&[
            0x2c030000,
            0x4082000c,
            0x38600007,
            0x4e800020,
            0x48000001 | ((self.target() - 0x1010) & 0x03fffffc),
        ]);
        if returning {
            code.extend(self.words(&[0x38600009, 0x4e800020]));
        }
        let code_size = code.len() as u32;
        if self.padded {
            code.resize(0x100, 0);
        }
        section(".text", 1, 6, 0x1000, code, 0, 0, 0, 0);
        let got = if matches!(self.plt, Plt::Long) {
            0x20000u32
        } else {
            0x2800u32
        };
        let delta = 0x3000u32.wrapping_sub(got);
        let mut stub = match self.plt {
            Plt::Bss => vec![],
            Plt::Short => self.words(&[
                0x817e0000 | delta as u16 as u32,
                0x7d6903a6,
                0x4e800420,
                0x60000000,
            ]),
            Plt::Long => self.words(&[
                0x3d7e0000 | (delta.wrapping_add(0x8000) >> 16),
                0x816b0000 | delta as u16 as u32,
                0x7d6903a6,
                0x4e800420,
            ]),
            Plt::Absolute => self.words(&[0x3d600000, 0x816b3000, 0x7d6903a6, 0x4e800420]),
        };
        if self.bad_stub && !stub.is_empty() {
            let end = stub.len();
            stub[end - 8..end - 4].copy_from_slice(&self.word(0x4e800020));
        }
        section(".glink", 1, 6, 0x1800, stub, 0, 0, 0, 0);
        section(
            ".plt",
            if matches!(self.plt, Plt::Bss) { 8 } else { 1 },
            if matches!(self.plt, Plt::Bss) { 7 } else { 3 },
            0x3000,
            if matches!(self.plt, Plt::Bss) {
                vec![]
            } else {
                self.words(&[0x1900])
            },
            if matches!(self.plt, Plt::Bss) {
                0x54
            } else {
                4
            },
            0,
            0,
            0,
        );
        section(".got", 1, 3, got, self.words(&[0x2900, 0, 0]), 0, 0, 0, 0);
        section(
            ".got2",
            1,
            3,
            0x4000,
            vec![0; if self.got2 { 4 } else { 0 }],
            0,
            0,
            0,
            0,
        );
        section(
            ".dynamic",
            6,
            3,
            0x2900,
            self.words(&[
                3,
                0x3000,
                if self.got_tag { 0x70000000 } else { 0 },
                got,
                0,
                0,
            ]),
            0,
            8,
            0,
            8,
        );
        let mut syms = vec![0; 16];
        syms.extend(self.symbol(1, 0, 0, 0x10, 0));
        syms.extend(self.symbol(self.import.len() as u32 + 2, 0x1000, code_size, 0x12, 1));
        section(".dynsym", 11, 2, 0x5000, syms, 0, 8, 1, 16);
        let names = format!("\0{}\0check\0", self.import).into_bytes();
        section(".dynstr", 3, 2, 0x5100, names, 0, 0, 0, 0);
        let slot = if matches!(self.plt, Plt::Bss) {
            0x3048
        } else {
            0x3000
        };
        section(
            ".rela.plt",
            4,
            2,
            0x5200,
            self.words(&[slot, 0x100 | self.relocation, self.addend]),
            0,
            7,
            3,
            12,
        );
        if self.symbolized {
            let mut syms = vec![0; 16];
            syms.extend(self.symbol(
                1,
                self.target(),
                16,
                2,
                if matches!(self.plt, Plt::Bss) { 3 } else { 2 },
            ));
            section(".symtab", 2, 0, 0, syms, 0, 11, 2, 16);
            section(
                ".strtab",
                3,
                0,
                0,
                format!("\000000000.plt_pic32.{}\0", self.import).into_bytes(),
                0,
                0,
                0,
                0,
            );
        }
        let mut names = vec![0];
        let mut offsets = Vec::new();
        for s in &sections {
            offsets.push(names.len() as u32);
            names.extend(s.name.as_bytes());
            names.push(0);
        }
        let shstr = sections.len() + 1;
        offsets.push(names.len() as u32);
        names.extend(b".shstrtab\0");
        sections.push(Section {
            name: ".shstrtab",
            kind: 3,
            flags: 0,
            addr: 0,
            data: names,
            size: 0,
            link: 0,
            info: 0,
            entsize: 0,
        });
        let loads = sections
            .iter()
            .filter(|s| s.flags & 2 != 0 && (s.size != 0 || !s.data.is_empty()))
            .count();
        let mut bytes = vec![0; 52 + loads * 32];
        bytes[..7].copy_from_slice(&[
            0x7f,
            b'E',
            b'L',
            b'F',
            1,
            if self.little { 1 } else { 2 },
            1,
        ]);
        bytes[16..18].copy_from_slice(&self.half(3));
        bytes[18..20].copy_from_slice(&self.half(20));
        bytes[20..24].copy_from_slice(&self.word(1));
        bytes[24..28].copy_from_slice(&self.word(0x1000));
        bytes[28..32].copy_from_slice(&self.word(52));
        for (off, n) in [
            (40, 52),
            (42, 32),
            (44, loads as u16),
            (46, 40),
            (48, sections.len() as u16 + 1),
            (50, shstr as u16),
        ] {
            bytes[off..off + 2].copy_from_slice(&self.half(n));
        }
        let mut headers = vec![0; 40];
        let mut ph = 52;
        for (i, s) in sections.iter().enumerate() {
            while bytes.len() % 4 != 0 {
                bytes.push(0);
            }
            let offset = bytes.len() as u32;
            let size = if s.size != 0 {
                s.size
            } else {
                s.data.len() as u32
            };
            bytes.extend(&s.data);
            headers.extend(self.words(&[
                offsets[i], s.kind, s.flags, s.addr, offset, size, s.link, s.info, 4, s.entsize,
            ]));
            if s.flags & 2 != 0 && size != 0 {
                let p = self.words(&[
                    1,
                    offset,
                    s.addr,
                    s.addr,
                    if s.kind == 8 { 0 } else { size },
                    size,
                    4 | (if s.flags & 1 != 0 { 2 } else { 0 })
                        | (if s.flags & 4 != 0 { 1 } else { 0 }),
                    1,
                ]);
                bytes[ph..ph + 32].copy_from_slice(&p);
                ph += 32;
            }
        }
        while bytes.len() % 4 != 0 {
            bytes.push(0);
        }
        let shoff = bytes.len() as u32;
        bytes[32..36].copy_from_slice(&self.word(shoff));
        bytes.extend(headers);
        bytes
    }
}

#[test]
fn ppc32_import_names_follow_jump_slots_and_complete_stubs() {
    for plt in [Plt::Bss, Plt::Short, Plt::Long, Plt::Absolute] {
        for little in [false, true] {
            for symbolized in [false, true] {
                let mut fixture = Fixture::new(plt);
                fixture.little = little;
                fixture.symbolized = symbolized;
                let bytes = fixture.bytes();
                let file = object::File::parse(&*bytes).unwrap();
                assert!(file.dynamic_relocations().unwrap().any(|(slot, _)| slot
                    == if matches!(plt, Plt::Bss) {
                        0x3048
                    } else {
                        0x3000
                    }));
                let imports = kuna_analysis::loader::format::resolve_imports(&file, &bytes);
                assert!(
                    imports
                        .iter()
                        .any(|s| s.addr == u64::from(fixture.target())
                            && s.name == b"__stack_chk_fail"),
                    "{plt:?} little={little} symbolized={symbolized}"
                );
            }
        }
    }
}

#[test]
fn ppc32_no_return_contract_stops_flow_at_the_import() {
    for plt in [Plt::Bss, Plt::Short, Plt::Long, Plt::Absolute] {
        for symbolized in [false, true] {
            for padded in [false, true] {
                for import in ["__stack_chk_fail", "returning_import"] {
                    let mut fixture = Fixture::new(plt);
                    fixture.symbolized = symbolized;
                    fixture.padded = padded;
                    fixture.import = import;
                    let path = common::scratch_file("ppc32-import", "so");
                    std::fs::write(&path, fixture.bytes()).unwrap();
                    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                        .args([
                            "decompile",
                            path.to_str().unwrap(),
                            "check",
                            "--mode",
                            "fast",
                        ])
                        .output()
                        .unwrap();
                    let text = String::from_utf8_lossy(&out.stdout);
                    assert!(
                        out.status.success(),
                        "{plt:?} {import}: {text}\n{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    assert!(
                        text.contains(&format!("{import}(")) && text.contains("return 7;"),
                        "{plt:?}: {text}"
                    );
                    assert!(
                        !text.contains("halt_baddata") && !text.contains("Bad instruction"),
                        "{text}"
                    );
                    assert_eq!(
                        text.contains("no-return"),
                        import == "__stack_chk_fail",
                        "{text}"
                    );
                    if import == "returning_import" {
                        assert!(text.contains("return 9;"), "{text}");
                    }
                    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                        .args(["xrefs", path.to_str().unwrap(), "--to", import, "--json"])
                        .output()
                        .unwrap();
                    assert!(
                        out.status.success(),
                        "{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let rows: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
                    assert!(rows.to_string().contains("0x1010"), "{rows}");
                    std::fs::remove_file(path).unwrap();
                }
            }
        }
    }
}

#[test]
fn ppc32_declines_unproven_import_contracts() {
    for plt in [Plt::Bss, Plt::Short, Plt::Long, Plt::Absolute] {
        for change in 0..5 {
            if change >= 2 && matches!(plt, Plt::Bss | Plt::Absolute) {
                continue;
            }
            let mut fixture = Fixture::new(plt);
            match change {
                0 => fixture.relocation = object::elf::R_PPC_ADDR32,
                1 => fixture.addend = 4,
                2 => fixture.got_tag = false,
                3 => fixture.got2 = true,
                _ => fixture.bad_stub = true,
            }
            let bytes = fixture.bytes();
            let file = object::File::parse(&*bytes).unwrap();
            let imports = kuna_analysis::loader::format::resolve_imports(&file, &bytes);
            assert!(imports.is_empty(), "{plt:?} change={change}");
        }
    }
}

#[test]
fn ppc32_rejects_malformed_metadata_and_out_of_range_slots() {
    use object::ObjectSection as _;
    for plt in [Plt::Bss, Plt::Short, Plt::Long, Plt::Absolute] {
        for change in 0..10 {
            if change >= 7 && matches!(plt, Plt::Bss | Plt::Absolute) {
                continue;
            }
            let fixture = Fixture::new(plt);
            let mut bytes = fixture.bytes();
            let file = object::File::parse(&*bytes).unwrap();
            let section = |name| {
                let sec = file.section_by_name(name).unwrap();
                (
                    sec.index().0,
                    sec.file_range().map(|p| p.0 as usize).unwrap_or(0),
                )
            };
            let (plt_index, _) = section(".plt");
            let (_, rela) = section(".rela.plt");
            let (_, sym) = section(".dynsym");
            let (_, got) = section(".got");
            let (_, dynamic) = section(".dynamic");
            let (stub_index, _) = section(".glink");
            let (rela_index, _) = section(".rela.plt");
            let shoff = u32::from_be_bytes(bytes[32..36].try_into().unwrap()) as usize;
            match change {
                0 => bytes[16..18].copy_from_slice(&1u16.to_be_bytes()),
                1 => bytes[shoff + plt_index * 40 + 8..shoff + plt_index * 40 + 12]
                    .copy_from_slice(&0u32.to_be_bytes()),
                2 => bytes[rela..rela + 4].copy_from_slice(&0x3001u32.to_be_bytes()),
                3 => bytes[rela..rela + 4].copy_from_slice(&0x3050u32.to_be_bytes()),
                4 => bytes[sym + 28] = 0x11,
                5 => bytes[shoff + rela_index * 40 + 20..shoff + rela_index * 40 + 24]
                    .copy_from_slice(&0u32.to_be_bytes()),
                6 => {
                    let entry = bytes[rela..rela + 12].to_vec();
                    let off = bytes.len() as u32;
                    bytes.extend(&entry);
                    bytes.extend(&entry);
                    bytes[shoff + rela_index * 40 + 16..shoff + rela_index * 40 + 20]
                        .copy_from_slice(&off.to_be_bytes());
                    bytes[shoff + rela_index * 40 + 20..shoff + rela_index * 40 + 24]
                        .copy_from_slice(&24u32.to_be_bytes());
                }
                7 => bytes[got..got + 4].copy_from_slice(&0xdead0000u32.to_be_bytes()),
                8 => {
                    bytes[dynamic + 16..dynamic + 20].copy_from_slice(&0x70000000u32.to_be_bytes())
                }
                _ => bytes[shoff + stub_index * 40 + 20..shoff + stub_index * 40 + 24]
                    .copy_from_slice(&12u32.to_be_bytes()),
            }
            let file = object::File::parse(&*bytes).unwrap();
            assert!(
                kuna_analysis::loader::format::resolve_imports(&file, &bytes).is_empty(),
                "{plt:?} change={change}"
            );
        }
    }
}

#[test]
fn ppc32_empty_got2_must_not_supply_a_conflicting_slot() {
    use object::ObjectSection as _;
    let fixture = Fixture::new(Plt::Short);
    let mut bytes = fixture.bytes();
    let file = object::File::parse(&*bytes).unwrap();
    let plt = file.section_by_name(".plt").unwrap().index().0;
    let got2 = file.section_by_name(".got2").unwrap().index().0;
    let rela = file.section_by_name(".rela.plt").unwrap();
    let index = rela.index().0;
    let mut entries = rela.data().unwrap().to_vec();
    entries.extend(fixture.words(&[0x3004, 0x115, 0]));
    let shoff = u32::from_be_bytes(bytes[32..36].try_into().unwrap()) as usize;
    let off = bytes.len() as u32;
    bytes.extend(entries);
    for (section, field, value) in [
        (plt, 20, 8),
        (got2, 12, 0xffffa804),
        (index, 16, off),
        (index, 20, 24),
    ] {
        let at = shoff + section * 40 + field;
        bytes[at..at + 4].copy_from_slice(&value.to_be_bytes());
    }
    let file = object::File::parse(&*bytes).unwrap();
    assert!(kuna_analysis::loader::format::resolve_imports(&file, &bytes).is_empty());
}
