//! Authored ELF32 fixtures; compile this file with rustc to regenerate the stage image.
#[derive(Clone, Copy, Debug)]
pub enum Plt {
    Bss,
    Short,
    Long,
    Absolute,
}

pub struct Fixture {
    pub plt: Plt,
    pub import: &'static str,
    pub symbolized: bool,
    pub padded: bool,
    pub little: bool,
    pub relocation: u32,
    pub addend: u32,
    pub got2: bool,
    pub got_tag: bool,
    pub bad_stub: bool,
    pub pic_collision: bool,
    pub missing_got2: bool,
    pub caller: Option<Vec<u32>>,
    pub second_caller: Option<Vec<u32>>,
    pub second_stub: bool,
}
impl Fixture {
    pub fn new(plt: Plt) -> Self {
        Self {
            plt,
            import: "__stack_chk_fail",
            symbolized: false,
            padded: false,
            little: false,
            relocation: 21,
            addend: 0,
            got2: false,
            got_tag: true,
            bad_stub: false,
            pic_collision: false,
            missing_got2: false,
            caller: None,
            second_caller: None,
            second_stub: false,
        }
    }
    pub fn target(&self) -> u32 {
        if matches!(self.plt, Plt::Bss) {
            0x3048
        } else {
            0x1800
        }
    }
    pub fn word(&self, n: u32) -> [u8; 4] {
        if self.little {
            n.to_le_bytes()
        } else {
            n.to_be_bytes()
        }
    }
    pub fn half(&self, n: u16) -> [u8; 2] {
        if self.little {
            n.to_le_bytes()
        } else {
            n.to_be_bytes()
        }
    }
    pub fn words(&self, words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|&n| self.word(n)).collect()
    }
    pub fn symbol(&self, name: u32, value: u32, size: u32, info: u8, section: u16) -> Vec<u8> {
        let mut b = self.words(&[name, value, size]);
        b.extend([info, 0]);
        b.extend(self.half(section));
        b
    }

    /// Authored ELF32 image, independent of an external assembler or linker.
    pub fn bytes(&self) -> Vec<u8> {
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
        if self.pic_collision {
            let base = if matches!(self.plt, Plt::Long) {
                0x20004u32
            } else {
                0x2804
            };
            let delta = base.wrapping_sub(0x1014);
            code = self.words(&[
                0x7c0802a6,
                0x9421fff0,
                0x90010014,
                0x93c10008,
                0x429f0005,
                0x7fc802a6,
                0x3fde0000 | (delta.wrapping_add(0x8000) >> 16),
                0x3bde0000 | delta as u16 as u32,
                0x48000001 | ((self.target() - 0x1020) & 0x03fffffc),
                0x38600007,
                0x80010014,
                0x83c10008,
                0x38210010,
                0x7c0803a6,
                0x4e800020,
            ]);
        }
        if let Some(words) = &self.caller {
            code = self.words(words);
        }
        let code_size = code.len() as u32;
        if let Some(words) = &self.second_caller {
            code.extend(self.words(words));
        }
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
        if self.second_stub {
            assert!(matches!(self.plt, Plt::Short));
            stub.extend(self.words(&[
                0x817e0000 | delta.wrapping_sub(4) as u16 as u32,
                0x7d6903a6,
                0x4e800420,
                0x60000000,
            ]));
        }
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
                self.words(if self.pic_collision {
                    &[0x1900, 0x1904]
                } else {
                    &[0x1900]
                })
            },
            if matches!(self.plt, Plt::Bss) {
                0x54
            } else if self.pic_collision {
                8
            } else {
                4
            },
            0,
            0,
            0,
        );
        section(".got", 1, 3, got, self.words(&[0x2900, 0, 0]), 0, 0, 0, 0);
        section(
            if self.missing_got2 {
                ".unused"
            } else {
                ".got2"
            },
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
        if self.pic_collision {
            syms.extend(self.symbol(self.import.len() as u32 + 8, 0, 0, 0x10, 0));
        }
        if let Some(words) = &self.second_caller {
            syms.extend(self.symbol(
                self.import.len() as u32 + 14,
                0x1000 + code_size,
                words.len() as u32 * 4,
                0x12,
                1,
            ));
        }
        section(".dynsym", 11, 2, 0x5000, syms, 0, 8, 1, 16);
        let names = format!("\0{}\0check\0extra\0check2\0", self.import).into_bytes();
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
            if self.pic_collision {
                self.words(&[
                    slot,
                    0x100 | self.relocation,
                    self.addend,
                    slot + 4,
                    0x315,
                    0,
                ])
            } else {
                self.words(&[slot, 0x100 | self.relocation, self.addend])
            },
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
                format!(
                    "\000000000.plt_pic32.{}\0",
                    if self.pic_collision {
                        "extra"
                    } else {
                        self.import
                    }
                )
                .into_bytes(),
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

#[allow(dead_code)]
fn main() {
    let path = std::env::args_os().nth(1).expect("output path required");
    let mut fixture = Fixture::new(Plt::Short);
    fixture.pic_collision = true;
    std::fs::write(path, fixture.bytes()).unwrap();
}
