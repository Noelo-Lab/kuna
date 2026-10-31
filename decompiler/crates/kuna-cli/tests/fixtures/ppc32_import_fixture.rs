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
    pub text_padding: Option<(usize, u32)>,
    pub data_size: usize,
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
    pub lazy_resolver: bool,
    pub unwind_only: bool,
    pub unwind_padding: u32,
    pub metadata_padding: u32,
}
impl Fixture {
    pub fn new(plt: Plt) -> Self {
        Self {
            plt,
            import: "__stack_chk_fail",
            symbolized: false,
            padded: false,
            text_padding: None,
            data_size: 4,
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
            lazy_resolver: false,
            unwind_only: false,
            unwind_padding: 0,
            metadata_padding: 0,
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
        if let Some((size, word)) = self.text_padding {
            while code.len() < size {
                code.extend(self.word(word));
            }
        } else if self.padded {
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
            let delta = delta.wrapping_sub(4);
            stub.extend(self.words(&match self.plt {
                Plt::Short => [
                    0x817e0000 | delta as u16 as u32,
                    0x7d6903a6, 0x4e800420, 0x60000000,
                ],
                Plt::Long => [
                    0x3d7e0000 | (delta.wrapping_add(0x8000) >> 16),
                    0x816b0000 | delta as u16 as u32,
                    0x7d6903a6, 0x4e800420,
                ],
                _ => panic!("second stub requires a PIC layout"),
            }));
        }
        if self.lazy_resolver {
            let pc = 0x1800 + stub.len() as u32;
            let delta = (got + 4).wrapping_sub(pc + 12);
            stub.extend(self.words(&[
                0x3d6b0000,
                0x7c0802a6,
                0x429f0005,
                0x396b001c,
                0x7d8802a6,
                0x7c0803a6,
                0x7d6c5850,
                0x3d8c0000 | (delta.wrapping_add(0x8000) >> 16),
                0x800c0000 | u32::from(delta as u16),
                0x818c0000 | u32::from(delta.wrapping_add(4) as u16),
                0x7c0903a6,
                0x7c0b5a14,
                0x7d605a14,
                0x4e800420,
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
            vec![0; if self.got2 { self.data_size } else { 0 }],
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
        syms.extend(self.symbol(
            self.import.len() as u32 + 2,
            0x1000,
            if self.unwind_only { 0 } else { code_size },
            0x12,
            1,
        ));
        if self.pic_collision {
            syms.extend(self.symbol(
                self.import.len() as u32 + 8 + self.metadata_padding,
                0,
                0,
                0x10,
                0,
            ));
        }
        if let Some(words) = &self.second_caller {
            syms.extend(self.symbol(
                self.import.len() as u32 + 14 + self.metadata_padding,
                0x1000 + code_size,
                words.len() as u32 * 4,
                0x12,
                1,
            ));
        }
        section(".dynsym", 11, 2, 0x5000, syms, 0, 8, 1, 16);
        let mut names = format!("\0{}\0check\0", self.import).into_bytes();
        names.resize(names.len() + self.metadata_padding as usize, 0);
        names.extend(b"extra\0check2\0");
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
        if self.unwind_only {
            let mut frames = self.words(&[16 + self.unwind_padding, 0]);
            frames.extend([1, b'z', b'R', 0, 1, 0x7c, 65, 1, 0x1b, 0, 0, 0]);
            frames.resize(frames.len() + self.unwind_padding as usize, 0);
            frames.extend(self.words(&[
                16,
                24 + self.unwind_padding,
                0x1000u32.wrapping_sub(0x601c + self.unwind_padding),
                code_size,
                0,
                0,
            ]));
            section(".eh_frame", 1, 2, 0x6000, frames, 0, 0, 0, 0);
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
    let mut args = std::env::args_os().skip(1);
    let path = args.next().expect("output path required");
    let bytes = if let Some(name) = args.next() {
        match name.to_str().unwrap() {
            "slot-write" => slot_write(false),
            "cache-original" => cache_pair().0,
            "cache-swapped" => cache_pair().1,
            "abi" => abi_caller(),
            "unwind" => {
                let mut f = Fixture::new(Plt::Short);
                f.pic_collision = true;
                f.metadata_padding = 4;
                f.unwind_only = true;
                f.bytes()
            }
            name => proof_gap(name, false),
        }
    } else {
        let mut fixture = Fixture::new(Plt::Short);
        fixture.pic_collision = true;
        fixture.metadata_padding = 4;
        fixture.bytes()
    };
    std::fs::write(path, bytes).unwrap();
}

/// Independent paths into a two-import PIC stub, including relocated code.
pub fn proof_gap(name: &str, little: bool) -> Vec<u8> {
    proof_gap_with_padding(name, little, 0)
}

pub fn proof_gap_padded(name: &str, little: bool) -> Vec<u8> {
    proof_gap_with_padding(name, little, 4)
}

fn proof_gap_with_padding(name: &str, little: bool, metadata_padding: u32) -> Vec<u8> {
    if matches!(
        name,
        "string-store"
            | "word-store"
            | "split-store"
            | "aligned-data"
            | "unaligned-data"
            | "composed-words"
            | "vector-copy"
            | "vector-no-store"
            | "note-valid"
            | "mapped-data"
    ) {
        let mut f = Fixture::new(Plt::Short);
        f.little = little;
        f.metadata_padding = metadata_padding;
        f.padded = true;
        f.text_padding = Some((0x800, 0x48000000));
        f.data_size = match name {
            "vector-copy" | "vector-no-store" => 16,
            "note-valid" => 20,
            _ => 8,
        };
        f.pic_collision = true;
        f.got2 = true;
        f.second_stub = true;
        f.caller = Some(vec![0x3bc02800, 0x4800080c]);
        let mut second = match name {
            "string-store" => vec![
                0x39601004,
                0x39804000,
                31 << 26 | 11 << 21 | 12 << 16 | 4 << 11 | 725 << 1,
            ],
            "word-store" => vec![0x39601004, 0x91604000],
            "composed-words" if little => vec![0x3d600400, 0x91604000, 0x39600010, 0x91604004],
            "composed-words" => vec![0x39600010, 0x91604000, 0x3d600400, 0x91604004],
            "vector-copy" | "vector-no-store" => vec![
                0x39804000,
                0x7c0060ce,
                0x1024038c,
                0x10000880,
                0x39804000,
                if name == "vector-copy" {
                    0x7c0061ce
                } else {
                    0x60000000
                },
            ],
            "split-store" => vec![
                0x39600010,
                0x99604000 | if little { 1 } else { 2 },
                0x39600004,
                0x99604000 | if little { 0 } else { 3 },
            ],
            _ => vec![],
        };
        second.extend((3..=10).map(|r| 0x38000000 | r << 21));
        let offset = match name {
            "unaligned-data" => 1,
            "composed-words" if little => 3,
            "composed-words" => 1,
            "note-valid" => 16,
            _ => 0,
        };
        second.extend([0x38604000 + offset, 0x3bc02804]);
        let pc = 0x1008 + second.len() as u32 * 4;
        second.push(0x48000000 | (0x1800 - pc));
        f.second_caller = Some(second);
        let mut bytes = f.bytes();
        let read = |bytes: &[u8], at: usize| {
            let word = bytes[at..at + 4].try_into().unwrap();
            (if little {
                u32::from_le_bytes(word)
            } else {
                u32::from_be_bytes(word)
            }) as usize
        };
        let headers = read(&bytes, 32);
        let stub = read(&bytes, headers + 2 * 40 + 16);
        bytes[stub..stub + 32].rotate_left(16);
        bytes[stub..stub + 16]
            .copy_from_slice(&f.words(&[0x3d600000, 0x816b3004, 0x7d6903a6, 0x4e800420]));
        if name.ends_with("data") {
            let data = read(&bytes, headers + 5 * 40 + 16) + usize::from(name == "unaligned-data");
            bytes[data..data + 4].copy_from_slice(&f.word(0x1004));
        }
        let data = read(&bytes, headers + 5 * 40 + 16);
        if name == "mapped-data" {
            bytes[headers + 5 * 40 + 8..headers + 5 * 40 + 12].copy_from_slice(&f.word(0));
        }
        if name.starts_with("vector-") {
            bytes[data..data + 16].copy_from_slice(&f.words(&[0x1000; 4]));
        }
        if name == "note-valid" {
            bytes[data..data + 12].copy_from_slice(&f.words(&[4, 4, 1]));
            bytes[data + 12..data + 16].copy_from_slice(b"CBK\0");
            bytes[data + 16..data + 20].copy_from_slice(&f.word(0x1004));
            bytes[headers + 5 * 40 + 4..headers + 5 * 40 + 8].copy_from_slice(&f.word(7));
        }
        return bytes;
    }
    if matches!(
        name,
        "callee-output" | "float-copy" | "unsupported-rfi" | "unsupported-tar"
    ) {
        let mut f = Fixture::new(Plt::Short);
        f.little = little;
        f.metadata_padding = metadata_padding;
        f.pic_collision = true;
        f.caller = Some(vec![0x3bc02800, 0x480007fc]);
        match name {
            "callee-output" => {
                f.symbolized = true;
                f.second_stub = true;
                f.second_caller = Some(vec![
                    0x7c0802a6, 0x9421ffe0, 0x90010024, 0x3bc02804, 0x48000029, 0x7d836378,
                    0x480007f1, 0x38600007, 0x80010024, 0x38210020, 0x7c0803a6, 0x4e800020,
                    0x60000000, 0x60000000, 0x39801004, 0x4e800020,
                ]);
            }
            "float-copy" => {
                f.caller = Some(vec![0x3bc02800, 0x48000009, 0x480007f8]);
                f.second_caller = Some(vec![
                    0x7c0802a6, 0x90010004, 0xc8010000, 0xd8002804, 0x4e800020,
                ]);
            }
            _ => {
                let mut words = if name == "unsupported-tar" {
                    vec![0x39801004, 0x7d8fcba6, 0x3bc02804, 0x4e800460]
                } else {
                    vec![0x39801004, 0x7d9a03a6, 0x3bc02804, 0x4c000064]
                };
                words.extend((3..=10).map(|r| 0x38000000 | r << 21));
                words.push(0x48000000);
                f.second_caller = Some(words);
            }
        }
        let mut bytes = f.bytes();
        if name == "callee-output" {
            let read = |bytes: &[u8], at: usize| {
                let word = bytes[at..at + 4].try_into().unwrap();
                (if little {
                    u32::from_le_bytes(word)
                } else {
                    u32::from_be_bytes(word)
                }) as usize
            };
            let headers = read(&bytes, 32);
            let dynamic = read(&bytes, headers + 7 * 40 + 16);
            let locals = read(&bytes, headers + 10 * 40 + 16);
            let stub = read(&bytes, headers + 2 * 40 + 16);
            bytes[dynamic + 72..dynamic + 76].copy_from_slice(&f.word(56));
            bytes[locals + 16..locals + 32].copy_from_slice(&f.symbol(1, 0x1040, 8, 2, 1));
            bytes[stub + 16..stub + 32]
                .copy_from_slice(&f.words(&[0x3d600000, 0x816b3004, 0x7d6903a6, 0x4e800420]));
        }
        return bytes;
    }
    if matches!(
        name,
        "callee-callback"
            | "callee-callback-shift"
            | "callee-callback-argument"
            | "return-export"
            | "return-export-explicit"
            | "return-private"
    ) {
        let mut f = Fixture::new(Plt::Short);
        f.little = little;
        f.metadata_padding = metadata_padding;
        f.pic_collision = true;
        f.symbolized = true;
        if name.starts_with("return-") {
            f.got2 = true;
            f.caller = Some(vec![
                0x7c0802a6, 0x9421ffe0, 0x90010024, 0x3bc02800, 0x48000021, 0x480007ed, 0x38600007,
                0x80010024, 0x38210020, 0x7c0803a6, 0x4e800020, 0x60000000,
            ]);
            f.second_caller = Some(vec![
                if name == "return-export-explicit" {
                    0x38001014
                } else {
                    0x7c0802a6
                },
                if name == "return-private" {
                    0x60000000
                } else {
                    0x90004000
                },
                0x4e800020,
            ]);
        } else {
            f.second_stub = true;
            f.caller = Some(vec![0x3bc02800, 0x480007fc]);
            let argument = name == "callee-callback-argument";
            f.second_caller = Some(vec![
                0x7c0802a6,
                0x9421ffe0,
                0x90010024,
                0x3bc02804,
                if argument { 0x38601004 } else { 0x39c01004 },
                0x4800001d,
                0x38600007,
                0x80010024,
                0x38210020,
                0x7c0803a6,
                0x4e800020,
                0x60000000,
                if argument { 0x60000000 } else { 0x7dc37378 },
                0x480007d4,
            ]);
            if name == "callee-callback-shift" {
                f.second_caller.as_mut().unwrap()[4..7]
                    .copy_from_slice(&[0x39c00401, 0x55ce103a, 0x48000019]);
            }
        }
        let mut bytes = f.bytes();
        let read = |bytes: &[u8], at: usize| {
            let word = bytes[at..at + 4].try_into().unwrap();
            (if little {
                u32::from_le_bytes(word)
            } else {
                u32::from_be_bytes(word)
            }) as usize
        };
        let headers = read(&bytes, 32);
        let dynamic = read(&bytes, headers + 7 * 40 + 16);
        let locals = read(&bytes, headers + 10 * 40 + 16);
        if name.starts_with("return-") {
            bytes[dynamic + 64..dynamic + 80].copy_from_slice(&f.symbol(
                f.import.len() as u32 + 14 + f.metadata_padding,
                0x4000,
                4,
                0x11,
                5,
            ));
            bytes[locals + 16..locals + 32].copy_from_slice(&f.symbol(1, 0x1030, 12, 2, 1));
        } else {
            let stub = read(&bytes, headers + 2 * 40 + 16);
            bytes[stub + 16..stub + 32]
                .copy_from_slice(&f.words(&[0x3d600000, 0x816b3004, 0x7d6903a6, 0x4e800420]));
            bytes[dynamic + 72..dynamic + 76].copy_from_slice(&f.word(48));
            bytes[locals + 16..locals + 32].copy_from_slice(&f.symbol(1, 0x1038, 8, 2, 1));
        }
        return bytes;
    }
    if matches!(
        name,
        "tail-callback"
            | "tail-callback-linked"
            | "local-alias"
            | "local-alias-nop"
            | "callback-shift"
            | "callback-direct"
            | "callback-call-clobber"
            | "callback-no-call"
    ) {
        let mut f = Fixture::new(Plt::Short);
        f.little = little;
        f.metadata_padding = metadata_padding;
        f.pic_collision = true;
        f.caller = Some(vec![0x3bc02800, 0x480007fc]);
        if name.starts_with("callback-") {
            f.second_stub = true;
            f.second_caller = Some(match name {
                "callback-call-clobber" | "callback-no-call" => vec![
                    0x7c0802a6,
                    0x9421ffe0,
                    0x90010024,
                    0x39c01004,
                    if name == "callback-call-clobber" {
                        0x480007f9
                    } else {
                        0x60000000
                    },
                    0x7dc37378,
                    0x80010024,
                    0x38210020,
                    0x7c0803a6,
                    0x3bc02804,
                    0x480007e0,
                ],
                _ => vec![
                    0x3bc02804,
                    if name == "callback-shift" {
                        0x38600401
                    } else {
                        0x38601004
                    },
                    if name == "callback-shift" {
                        0x5463103a
                    } else {
                        0x60000000
                    },
                    0x480007fc,
                ],
            });
        } else if name.starts_with("tail-callback") {
            f.second_stub = true;
            f.second_caller = Some(vec![
                0x3bc02804,
                0x38601004,
                0x48000800 | u32::from(name.ends_with("linked")),
            ]);
        } else {
            f.second_caller = Some(vec![
                0x7fe802a6,
                0x9421fff0,
                0x93e10014,
                0x39c10014,
                if name.ends_with("nop") {
                    0x60000000
                } else {
                    0x48000025
                },
                0x7dc37378,
                0x48000021,
                0x80010014,
                0x7fe803a6,
                0x38210010,
                0x3bc02804,
                0x7c0903a6,
                0x4e800420,
                0x4e800020,
                0x39800401,
                0x558c103a,
                0x91830000,
                0x4e800020,
            ]);
        }
        let mut bytes = f.bytes();
        if name.starts_with("tail-callback") || name.starts_with("callback-") {
            let read = |at: usize| {
                let word = bytes[at..at + 4].try_into().unwrap();
                if little {
                    u32::from_le_bytes(word)
                } else {
                    u32::from_be_bytes(word)
                }
            };
            let at = read(read(32) as usize + 2 * 40 + 16) as usize;
            bytes[at + 16..at + 32]
                .copy_from_slice(&f.words(&[0x3d600000, 0x816b3004, 0x7d6903a6, 0x4e800420]));
        }
        return bytes;
    }
    if matches!(
        name,
        "dynamic-init"
            | "dynamic-fini"
            | "dynamic-null"
            | "lr-alias"
            | "lr-alias-control"
            | "lr-alias-returning"
            | "lr-alias-returning-control"
            | "unproved-stub"
            | "unproved-stub-control"
    ) {
        let mut f = Fixture::new(Plt::Short);
        f.little = little;
        f.metadata_padding = metadata_padding;
        f.pic_collision = true;
        f.caller = Some(vec![0x3bc02800, 0x480007fc]);
        if name.starts_with("lr-alias-returning") {
            f.second_caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93e10008, 0x7fe802a6, 0x38800014, 0x7c612214,
                0x4800001d, 0x80010014, 0x7fe803a6, 0x83e10008, 0x38210010, 0x7c0903a6, 0x4e800420,
                0x39800401, 0x558c103a, 0x91830000, 0x4e800020,
            ]);
            if name.ends_with("control") {
                f.second_caller.as_mut().unwrap()[6] = 0x38610014;
            }
        } else if name.starts_with("lr-alias") {
            f.second_caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x3bc02804, 0x38800014, 0x7c612214, 0x48000015,
                0x80010014, 0x38210010, 0x7c0803a6, 0x4e800020, 0x39800401, 0x558c103a, 0x91830000,
                0x4e800020,
            ]);
            if name.ends_with("control") {
                f.second_caller.as_mut().unwrap()[5] = 0x38610014;
            }
        } else if name.starts_with("unproved-stub") {
            f.second_stub = true;
            f.second_caller = Some(vec![
                0x3bc02804, 0x39800401, 0x558c103a, 0x919e07f8, 0x7c0802a6, 0x9421fff0, 0x90010014,
                0x480007ed, 0x38600007, 0x80010014, 0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
        }
        let mut bytes = f.bytes();
        let read = |at: usize| {
            let word = bytes[at..at + 4].try_into().unwrap();
            if little {
                u32::from_le_bytes(word)
            } else {
                u32::from_be_bytes(word)
            }
        };
        let headers = read(32) as usize;
        if name.starts_with("dynamic-") {
            let at = read(headers + 6 * 40 + 16) as usize;
            let tag = match name {
                "dynamic-init" => 12,
                "dynamic-fini" => 13,
                _ => 0,
            };
            bytes[at + 8..at + 12].copy_from_slice(&f.word(tag));
            bytes[at + 12..at + 16].copy_from_slice(&f.word(
                if name == "dynamic-null" && metadata_padding != 0 {
                    0
                } else {
                    0x1004
                },
            ));
        } else if name.starts_with("unproved-stub") {
            let at = read(headers + 2 * 40 + 16) as usize;
            bytes[at + 16..at + 20].copy_from_slice(&f.word(0x817e07f8));
            if name.ends_with("control") {
                bytes[at + 28..at + 32].copy_from_slice(&f.word(0x60000001));
            }
        }
        return bytes;
    }
    if matches!(name, "nonexec-section" | "missing-section") {
        let mut bytes = proof_gap_with_padding("aligned-exec", little, metadata_padding);
        let mut f = Fixture::new(Plt::Short);
        f.little = little;
        f.metadata_padding = metadata_padding;
        let h = if little {
            u32::from_le_bytes(bytes[32..36].try_into().unwrap())
        } else {
            u32::from_be_bytes(bytes[32..36].try_into().unwrap())
        } as usize;
        bytes[h + 11 * 40 + 8..h + 11 * 40 + 12].copy_from_slice(&f.word(2));
        if name == "missing-section" {
            bytes[48..50].copy_from_slice(&f.half(11));
        }
        return bytes;
    }
    if matches!(
        name,
        "init-array" | "fini-array" | "preinit-array" | "pointer-array"
    ) {
        let mut bytes = proof_gap_with_padding("literal-data", little, metadata_padding);
        let mut f = Fixture::new(Plt::Short);
        f.little = little;
        f.metadata_padding = metadata_padding;
        let read = |at: usize| {
            let word = bytes[at..at + 4].try_into().unwrap();
            if little {
                u32::from_le_bytes(word)
            } else {
                u32::from_be_bytes(word)
            }
        };
        let h = read(32) as usize;
        let got = h + 4 * 40;
        let array = h + 5 * 40;
        let dynamic = read(h + 6 * 40 + 16) as usize;
        let offset = read(got + 16);
        let names = h
            + (if little {
                u16::from_le_bytes(bytes[50..52].try_into().unwrap())
            } else {
                u16::from_be_bytes(bytes[50..52].try_into().unwrap())
            }) as usize
                * 40;
        let at = read(names + 16) as usize;
        let mut strings = bytes[at..at + read(names + 20) as usize].to_vec();
        let name_offset = strings.len() as u32;
        strings.extend(b".init_array\0");
        let kind = match name {
            "init-array" => 14,
            "fini-array" => 15,
            "preinit-array" => 16,
            _ => 1,
        };
        for (at, value) in [
            (got + 8, 2),
            (got + 20, 8),
            (array, name_offset),
            (array + 4, kind),
            (array + 8, 2),
            (array + 12, 0x2808),
            (array + 16, offset + 8),
            (array + 20, 4),
            (array + 32, 4),
            (array + 36, 4),
            (dynamic + 8, 0),
            (names + 16, bytes.len() as u32),
            (names + 20, strings.len() as u32),
        ] {
            bytes[at..at + 4].copy_from_slice(&f.word(value));
        }
        bytes.extend(strings);
        return bytes;
    }
    let mut f = Fixture::new(Plt::Short);
    f.pic_collision = true;
    f.little = little;
    f.metadata_padding = metadata_padding;
    match name {
        "lr-join" => {
            f.caller = Some(vec![
                0x3bc02800, 0x2c030000, 0x41820008, 0x480007f5, 0x2c040000, 0x41820014, 0x3bc0100c,
                0x7fc803a6, 0x48000014, 0x60000000, 0x3bc01008, 0x7fc803a6, 0x60000000, 0x3bc02804,
                0x4e800020,
            ])
        }
        "padding" => {
            f.caller = Some(vec![0x3bc02800, 0x480007fc, 0x4e800020]);
            f.second_caller = Some(vec![0x3bc02804, 0x480007ec]);
        }
        "indirect" => {
            f.caller = Some(vec![0x3bc02800, 0x480007fc, 0x4e800020]);
            f.second_caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10008, 0x3bc02804, 0x39801800, 0x7d8903a6,
                0x4e800421, 0x38600007, 0x80010014, 0x83c10008, 0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
        }
        "indirect-branch" => {
            f.caller = Some(vec![0x3bc02800, 0x480007fc, 0x4e800020]);
            f.second_caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10008, 0x3bc02804, 0x398017fc, 0x48000008,
                0x39800000, 0x398c0004, 0x7d8903a6, 0x4e800421, 0x38600007, 0x80010014, 0x83c10008,
                0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
        }
        "literal" | "literal-data" | "literal-halfword" => {
            f.caller = Some(vec![0x3bc02800, 0x480007fc]);
            f.second_caller = Some(vec![
                0x7c0802a6,
                0x9421fff0,
                0x90010014,
                0x93c10008,
                0x3bc02804,
                0x39601044,
                if name == "literal-halfword" {
                    0xa18b0000 | if little { 2 } else { 0 }
                } else {
                    0x818b0000
                },
                0x7d8903a6,
                0x4e800421,
                0x38600007,
                0x83c10008,
                0x80010014,
                0x38210010,
                0x7c0803a6,
                0x4e800020,
                if name == "literal-halfword" {
                    0x18000000
                } else {
                    0x00001800
                },
                0x48000000,
            ]);
        }
        "resolver" | "resolver-direct" => {
            f.lazy_resolver = true;
            f.symbolized = true;
            f.caller = Some(vec![0x3bc02800, 0x480007fc]);
            f.second_caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10008, 0x3bc02804, 0x39801004, 0x7d8903a6,
                0x39601844, 0x7d6803a6, 0x4e800021, 0x38600007, 0x83c10008, 0x80010014, 0x38210010,
                0x7c0803a6, 0x4e800020,
            ]);
        }
        "store-halfword"
        | "store-word"
        | "store-word-halfword"
        | "store-word-halfword-interior" => {
            f.caller = Some(vec![0x3bc02800, 0x480007fc]);
            f.second_caller = Some(vec![
                0x7c0802a6,
                0x9421fff0,
                0x90010014,
                0x93c10008,
                0x3bc02804,
                match name {
                    "store-word-halfword" => 0x3d801800,
                    "store-word-halfword-interior" => 0x3d801004,
                    _ => 0x39801004,
                },
                0x39602800,
                if name != "store-halfword" {
                    0x918b0008
                } else {
                    0xb18b0008
                },
                if name == "store-word" {
                    0x818b0008
                } else {
                    0xa18b0008
                        | if little && name.starts_with("store-word-halfword") {
                            2
                        } else {
                            0
                        }
                },
                0x7d8903a6,
                0x4e800421,
                0x38600007,
                0x83c10008,
                0x80010014,
                0x38210010,
                0x7c0803a6,
                0x4e800020,
            ]);
        }
        "shift-spill" | "stack-bytes" | "ctr-spill" | "byte-composed" | "shift-no-spill" => {
            f.caller = Some(vec![0x3bc02800, 0x480007fc]);
            let (hi, lo) = if little { (9, 8) } else { (10, 11) };
            let mut body = vec![0x7c0802a6, 0x9421ffe0, 0x90010024, 0x93c10018, 0x3bc02804];
            match name {
                "byte-composed" => {
                    body = vec![
                        0x7c0802a6,
                        0x9421fff0,
                        0x90010014,
                        0x93c10008,
                        0x3bc02804,
                        0x39602800,
                        0x39800010,
                        0x998b0000 | hi,
                        0x39800004,
                        0x998b0000 | lo,
                        0x818b0008,
                        0x7d8903a6,
                        0x4e800421,
                        0x38600007,
                        0x83c10008,
                        0x80010014,
                        0x38210010,
                        0x7c0803a6,
                        0x4e800020,
                    ];
                }
                _ => {
                    if name == "stack-bytes" {
                        body.extend([
                            0x39800000,
                            0x91810008,
                            0x39800018,
                            0x99810000 | hi,
                            0x39800000,
                            0x99810000 | lo,
                        ]);
                    } else if name == "ctr-spill" {
                        body.extend([0x39801004, 0x7d8903a6, 0x7d6902a6, 0x91610008]);
                    } else {
                        body.extend([0x39800600, 0x558c103a, 0x91810008]);
                    }
                    body.extend([
                        0x81810008, 0x7d8903a6, 0x4e800421, 0x38600007, 0x83c10018, 0x80010024,
                        0x38210020, 0x7c0803a6, 0x4e800020,
                    ]);
                    if name == "shift-no-spill" {
                        body[7] = 0x60000000;
                        body[8] = 0x60000000;
                    }
                }
            }
            f.second_caller = Some(body);
        }
        "stub-interior"
        | "stub-interior-direct"
        | "stub-interior-control"
        | "stub-interior-literal" => {
            f.second_stub = true;
            f.symbolized = true;
            f.caller = Some(vec![0x3bc02804, 0x4800080c]);
            let mut words = vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10008, 0x3bc02808, 0x39801004, 0x7d8903a6,
            ];
            if name == "stub-interior-literal" {
                words.extend([0x39602808, 0x816b0000, 0x7d6803a6, 0x4e800021]);
            } else if name == "stub-interior" {
                words.extend([0x39601808, 0x7d6803a6, 0x4e800021]);
            } else {
                words.extend([
                    if name == "stub-interior-direct" {
                        0x480007e5
                    } else {
                        0x4bffffe1
                    },
                    0x60000000,
                    0x60000000,
                ]);
            }
            words.extend([
                0x38600007, 0x83c10008, 0x80010014, 0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
            f.second_caller = Some(words);
        }
        "unaligned-exec" | "aligned-exec" | "split-instruction" => {
            f.caller = Some(vec![0x3bc02800, 0x480007fc, 0x60000000]);
            f.second_caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10008, 0x3bc02804, 0x480007e1, 0x38600007,
                0x83c10008, 0x80010014, 0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
        }
        "internal-link" => {
            f.caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10008, 0x3bc02804, 0x48000009, 0x3bc02800,
                0x480007e5, 0x38600007, 0x83c10008, 0x80010014, 0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
        }
        "indirect-chain" => {
            f.caller = Some(vec![0x3bc02800, 0x480007fc]);
            f.second_caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10008, 0x3bc02804, 0x39801004, 0x39601048,
                0x7d6903a6, 0x4e800421, 0x38600007, 0x83c10008, 0x80010014, 0x38210010, 0x7c0803a6,
                0x4e800020, 0x39801000, 0x7d8903a6, 0x4e800420,
            ]);
        }
        "base-spill" => {
            f.caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x3bc0103c, 0x4800001d, 0x480007ed, 0x38600007,
                0x80010014, 0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
            f.second_caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x93c10004, 0x480007d5, 0x83c10004, 0x80010014,
                0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
            f.second_stub = true;
        }
        "ctr-decrement" => {
            f.caller = Some(vec![0x3bc02800, 0x480007fc]);
            f.second_caller = Some(vec![
                0x3bc02804, 0x39801008, 0x7d8903a6, 0x42000004, 0x42000004, 0x42000004, 0x42000004,
                0x4e800420,
            ]);
        }
        "local-fallthrough" => {
            f.symbolized = true;
            f.caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x3bc02800, 0x4800001d, 0x480007ed, 0x38600007,
                0x80010014, 0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
            f.second_caller = Some(vec![0x2c030000, 0x4d820020, 0x3bc02804, 0x4e800020]);
        }
        "relocated" => {
            f.second_stub = true;
            f.caller = Some(vec![
                0x7c0802a6, 0x9421fff0, 0x90010014, 0x3bc02804, 0x480007f1, 0x480007fd, 0x80010014,
                0x38210010, 0x7c0803a6, 0x4e800020,
            ]);
        }
        _ => panic!("unknown proof fixture"),
    }
    let mut bytes = f.bytes();
    if !matches!(
        name,
        "base-spill"
            | "shift-spill"
            | "stack-bytes"
            | "ctr-spill"
            | "byte-composed"
            | "shift-no-spill"
            | "ctr-decrement"
            | "internal-link"
            | "indirect-chain"
            | "stub-interior"
            | "stub-interior-direct"
            | "stub-interior-control"
            | "stub-interior-literal"
            | "unaligned-exec"
            | "aligned-exec"
            | "split-instruction"
    ) {
        bytes[16..18].copy_from_slice(&f.half(2));
    }
    if name == "stub-interior-literal" {
        let read = |at| {
            let word = bytes[at..at + 4].try_into().unwrap();
            if little {
                u32::from_le_bytes(word)
            } else {
                u32::from_be_bytes(word)
            }
        };
        let shoff = read(32) as usize;
        let got = read(shoff + 4 * 40 + 16) as usize;
        bytes[got + 8..got + 12].copy_from_slice(&f.word(0x1808));
    }
    if matches!(
        name,
        "unaligned-exec" | "aligned-exec" | "split-instruction"
    ) {
        let read = |at| {
            let word = bytes[at..at + 4].try_into().unwrap();
            if little {
                u32::from_le_bytes(word)
            } else {
                u32::from_be_bytes(word)
            }
        };
        let shoff = read(32) as usize;
        let text = read(shoff + 40 + 16);
        let symbols = read(shoff + 7 * 40 + 16) as usize;
        let mut section = bytes[shoff + 40..shoff + 80].to_vec();
        let offset = if name == "aligned-exec" { 12 } else { 9 };
        for (field, value) in [
            (12, 0x1000 + offset),
            (16, text + offset),
            (20, 60 - offset),
            (32, if offset == 9 { 1 } else { 4 }),
        ] {
            section[field..field + 4].copy_from_slice(&f.word(value));
        }
        bytes[shoff + 40 + 20..shoff + 40 + 24].copy_from_slice(&f.word(8));
        bytes[symbols + 2 * 16 + 8..symbols + 2 * 16 + 12].copy_from_slice(&f.word(8));
        bytes[symbols + 4 * 16 + 14..symbols + 5 * 16].copy_from_slice(&f.half(11));
        if name == "split-instruction" {
            let mut next = section.clone();
            for (field, value) in [(12, 0x1023), (16, text + 35), (20, 25)] {
                next[field..field + 4].copy_from_slice(&f.word(value));
            }
            section[20..24].copy_from_slice(&f.word(26));
            bytes[48..50].copy_from_slice(&f.half(13));
            bytes.extend(section);
            bytes.extend(next);
        } else {
            bytes[48..50].copy_from_slice(&f.half(12));
            bytes.extend(section);
        }
    }
    if matches!(name, "resolver" | "resolver-direct" | "literal-data") {
        let read = |at| {
            let word = bytes[at..at + 4].try_into().unwrap();
            if little {
                u32::from_le_bytes(word)
            } else {
                u32::from_be_bytes(word)
            }
        };
        let shoff = read(32) as usize;
        let text = read(shoff + 40 + 16) as usize;
        if name == "literal-data" {
            let got = read(shoff + 4 * 40 + 16) as usize;
            bytes[got + 8..got + 12].copy_from_slice(&f.word(0x1800));
            bytes[text + 0x1c..text + 0x20].copy_from_slice(&f.word(0x39602808));
            bytes[text + 0x44..text + 0x48].copy_from_slice(&f.word(0x60000000));
        } else {
            let symbols = read(shoff + 10 * 40 + 16) as usize;
            let strings = read(shoff + 11 * 40 + 16) as usize;
            bytes[symbols + 20..symbols + 24].copy_from_slice(&f.word(0x1810));
            bytes[symbols + 24..symbols + 28].copy_from_slice(&f.word(56));
            bytes[strings + 1..strings + 10].copy_from_slice(b"resolver\0");
            if name == "resolver-direct" {
                bytes[text + 0x2c..text + 0x30].copy_from_slice(&f.word(0x48000819));
            }
        }
    }
    if name == "base-spill" {
        let word = bytes[32..36].try_into().unwrap();
        let shoff = if little {
            u32::from_le_bytes(word)
        } else {
            u32::from_be_bytes(word)
        } as usize;
        let word = bytes[shoff + 2 * 40 + 16..shoff + 2 * 40 + 20]
            .try_into()
            .unwrap();
        let offset = if little {
            u32::from_le_bytes(word)
        } else {
            u32::from_be_bytes(word)
        } as usize;
        bytes[offset..offset + 4].copy_from_slice(&f.word(0x817e1fc4));
        bytes[offset + 16..offset + 32]
            .copy_from_slice(&f.words(&[0x3d600000, 0x816b3004, 0x7d6903a6, 0x4e800420]));
    }
    if name == "local-fallthrough" {
        let read = |at| {
            let word = bytes[at..at + 4].try_into().unwrap();
            if little {
                u32::from_le_bytes(word)
            } else {
                u32::from_be_bytes(word)
            }
        };
        let shoff = read(32) as usize;
        let symbols = read(shoff + 7 * 40 + 16) as usize;
        let locals = read(shoff + 10 * 40 + 16) as usize;
        let names = read(shoff + 11 * 40 + 16) as usize;
        bytes[symbols + 72..symbols + 76].copy_from_slice(&f.word(12));
        bytes[locals + 16..locals + 32].copy_from_slice(&f.symbol(1, 0x1038, 4, 2, 1));
        bytes[names + 1..names + 10].copy_from_slice(b"epilogue\0");
    }
    if matches!(name, "padding" | "relocated") {
        let read = |b: &[u8]| {
            if little {
                u32::from_le_bytes(b.try_into().unwrap())
            } else {
                u32::from_be_bytes(b.try_into().unwrap())
            }
        };
        let shoff = read(&bytes[32..36]) as usize;
        let oldaddr = if name == "padding" { 0x1800 } else { 0x5200 };
        let address = if name == "padding" { 0x17f8 } else { oldaddr };
        let section = (1..12)
            .map(|i| shoff + i * 40)
            .find(|&s| read(&bytes[s + 12..s + 16]) == oldaddr)
            .unwrap();
        let oldoff = read(&bytes[section + 16..section + 20]) as usize;
        let oldsize = read(&bytes[section + 20..section + 24]) as usize;
        let mut data = bytes[oldoff..oldoff + oldsize].to_vec();
        if name == "padding" {
            let mut prefix = f.words(&[0x48000000, 0x60000000]);
            prefix.extend(data);
            data = prefix;
        } else {
            data.extend(f.words(&[0x1800, 1, 0x3bc02808, 0x1804, 1, 0x4e800020]));
        }
        let offset = bytes.len() as u32;
        let size = data.len() as u32;
        bytes.extend(data);
        for (field, value) in [(12, address), (16, offset), (20, size)] {
            bytes[section + field..section + field + 4].copy_from_slice(&f.word(value));
        }
        let count = if little {
            u16::from_le_bytes(bytes[44..46].try_into().unwrap())
        } else {
            u16::from_be_bytes(bytes[44..46].try_into().unwrap())
        };
        for i in 0..usize::from(count) {
            let ph = 52 + i * 32;
            if read(&bytes[ph + 8..ph + 12]) == oldaddr {
                for (field, value) in [
                    (4, offset),
                    (8, address),
                    (12, address),
                    (16, size),
                    (20, size),
                ] {
                    bytes[ph + field..ph + field + 4].copy_from_slice(&f.word(value));
                }
            }
        }
    }
    bytes
}

/// ABI instructions in an unrelated function must not suppress a proven caller.
pub fn abi_caller() -> Vec<u8> {
    let mut f = Fixture::new(Plt::Short);
    f.pic_collision = true;
    f.metadata_padding = 4;
    f.second_caller = Some(vec![
        0x7d832214, 0x7c032000, 0x4cc63182, 0x60000000, 0x4e800020,
    ]);
    f.bytes()
}

/// Two authored images with different jump-slot identities and the same legacy
/// two-lane fingerprint. Only relocation records and ignored trailer bytes differ.
pub fn cache_pair() -> (Vec<u8>, Vec<u8>) {
    let mut fixture = Fixture::new(Plt::Short);
    fixture.pic_collision = true;
    fixture.metadata_padding = 4;
    let mut first = fixture.bytes();
    first.resize(first.len().next_multiple_of(16) + 16, 0);
    let mut second = first.clone();
    let word = |at: usize| u32::from_be_bytes(first[at..at + 4].try_into().unwrap()) as usize;
    let headers = word(32);
    let count = u16::from_be_bytes(first[48..50].try_into().unwrap()) as usize;
    let section = (1..count)
        .map(|i| headers + i * 40)
        .find(|&s| word(s + 4) == 4)
        .unwrap();
    let offset = word(section + 16);
    second[offset + 4..offset + 8].copy_from_slice(&first[offset + 16..offset + 20]);
    second[offset + 16..offset + 20].copy_from_slice(&first[offset + 4..offset + 8]);
    let fingerprint = |bytes: &[u8]| {
        let mut lanes = [0x243F_6A88_85A3_08D3u64, 0x1319_8A2E_0370_7344u64];
        for block in bytes.chunks_exact(16) {
            for (lane, word) in lanes.iter_mut().zip(block.chunks_exact(8)) {
                *lane = (*lane ^ u64::from_le_bytes(word.try_into().unwrap()))
                    .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                    .rotate_left(29);
            }
        }
        lanes
    };
    let end = first.len() - 16;
    let before = fingerprint(&second[..end]);
    let target = fingerprint(&first);
    let mut inverse = 1u64;
    for _ in 0..6 {
        inverse =
            inverse.wrapping_mul(2u64.wrapping_sub(0x9E37_79B9_7F4A_7C15u64.wrapping_mul(inverse)));
    }
    for i in 0..2 {
        let word = before[i] ^ target[i].rotate_right(29).wrapping_mul(inverse);
        second[end + i * 8..end + (i + 1) * 8].copy_from_slice(&word.to_le_bytes());
    }
    assert_eq!(fingerprint(&first), fingerprint(&second));
    assert_ne!(first, second);
    (first, second)
}

/// memcpy copies a returning target into the next call's writable abort slot.
pub fn slot_write(little: bool) -> Vec<u8> {
    let mut f = Fixture::new(Plt::Short);
    f.little = little;
    f.pic_collision = true;
    f.import = "memcpy";
    let mut code = vec![
        0x7c0802a6, 0x9421ffe0, 0x90010024, 0x93c10018,
        0x3bc02800, 0x39601100, 0x91610008, 0x38603004, 0x38810008, 0x38a00004,
        0x480007d9, 0x3bde0004, 0x480007d1,
        0x38600007, 0x83c10018, 0x80010024, 0x38210020, 0x7c0803a6, 0x4e800020,
    ];
    code.resize(0x100 / 4, 0x60000000);
    code.extend([0x38600009, 0x4e800020]);
    f.caller = Some(code);
    let mut bytes = f.bytes();
    let offset = bytes.windows(6).position(|b| b == b"extra\0").unwrap();
    bytes[offset..offset + 6].copy_from_slice(b"abort\0");
    bytes
}
