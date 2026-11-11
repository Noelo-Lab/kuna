//! Synthetic PKLITE files: a compressor for PKLITE's bitstream and a builder
//! that lays a packed EXE or COM out around a *fingerprint skeleton* of the
//! decompressor stub -- only the byte patterns a static unpacker keys on, zero
//! filled everywhere else, so no PKWARE code is involved. A test-time generator,
//! not a fixture source: included by `kuna_analysis::pklite`'s tests and by
//! `kuna-cli/tests/unpack_pklite_cli.rs`. Files it builds are accepted by deark's
//! `pklite` module, which unpacks them to the same bytes as [`expected`].
#![allow(dead_code)]

/// `(code length << 12) | code`, the four length alphabets and two distance
/// alphabets PKLITE's decompressors hard-code.
const LENGTHS_LARGE: [u16; 24] = [
    0x2003, 0x3000, 0x4002, 0x4003, 0x4004, 0x500a, 0x500b, 0x500c, 0x601a, 0x601b, 0x703a, 0x703b,
    0x703c, 0x807a, 0x807b, 0x807c, 0x90fa, 0x90fb, 0x90fc, 0x90fd, 0x90fe, 0x90ff, 0x601c, 0x2002,
];
const LENGTHS_SMALL: [u16; 9] =
    [0x2000, 0x3004, 0x3005, 0x400c, 0x400d, 0x400e, 0x400f, 0x3003, 0x3002];
const LENGTHS_V120_LARGE: [u16; 21] = [
    0x2003, 0x3000, 0x4005, 0x4006, 0x5006, 0x5007, 0x6008, 0x6009, 0x7020, 0x7021, 0x7022, 0x7023,
    0x8048, 0x8049, 0x804a, 0x9096, 0x9097, 0x6013, 0x2002, 0x4007, 0x5005,
];
const LENGTHS_V120_SMALL: [u16; 11] =
    [0x2003, 0x3000, 0x4004, 0x4005, 0x500e, 0x601e, 0x601f, 0x4006, 0x2002, 0x4003, 0x4002];
const OFFSETS: [u16; 32] = [
    0x1001, 0x4000, 0x4001, 0x5004, 0x5005, 0x5006, 0x5007, 0x6010, 0x6011, 0x6012, 0x6013, 0x6014,
    0x6015, 0x6016, 0x702e, 0x702f, 0x7030, 0x7031, 0x7032, 0x7033, 0x7034, 0x7035, 0x7036, 0x7037,
    0x7038, 0x7039, 0x703a, 0x703b, 0x703c, 0x703d, 0x703e, 0x703f,
];
const OFFSETS_V120: [u16; 32] = [
    0x1001, 0x3000, 0x5004, 0x5005, 0x5006, 0x5007, 0x6010, 0x6011, 0x6012, 0x6013, 0x6014, 0x6015,
    0x702c, 0x702d, 0x702e, 0x702f, 0x7030, 0x7031, 0x7032, 0x7033, 0x7034, 0x7035, 0x7036, 0x7037,
    0x7038, 0x7039, 0x703a, 0x703b, 0x703c, 0x703d, 0x703e, 0x703f,
];

pub const UNCOMPRESSED_SIG: [u8; 8] = *b"PKLITE\x26\xa3";

/// The compression mode a stream is written in.
#[derive(Debug, Clone, Copy)]
pub struct Mode {
    pub large: bool,
    pub v120: bool,
    /// `0` none, `1` literal XOR remaining-control-bits, `2` literal XOR `0xff`.
    pub extra: u8,
    pub offset_xor: u8,
}

struct Alphabet {
    table: &'static [u16],
    long: usize,
    two: usize,
    two_high: Option<usize>,
    zero_literal: Option<usize>,
    bias: usize,
}

fn alphabet(m: &Mode) -> Alphabet {
    match (m.large, m.v120) {
        (true, false) => Alphabet {
            table: &LENGTHS_LARGE,
            long: 22,
            two: 23,
            two_high: None,
            zero_literal: None,
            bias: 25,
        },
        (true, true) => Alphabet {
            table: &LENGTHS_V120_LARGE,
            long: 17,
            two: 18,
            two_high: Some(19),
            zero_literal: Some(20),
            bias: 20,
        },
        (false, false) => Alphabet {
            table: &LENGTHS_SMALL,
            long: 7,
            two: 8,
            two_high: None,
            zero_literal: None,
            bias: 10,
        },
        (false, true) => Alphabet {
            table: &LENGTHS_V120_SMALL,
            long: 7,
            two: 8,
            two_high: Some(9),
            zero_literal: Some(10),
            bias: 10,
        },
    }
}

/// Control bits go into 16-bit little-endian words, least significant first,
/// and the next word is reserved the moment one fills -- before any data byte
/// that follows the bit that filled it, which is when the decoder reads it.
struct Writer {
    out: Vec<u8>,
    word: usize,
    used: u32,
}

impl Writer {
    fn new() -> Writer {
        Writer { out: vec![0, 0], word: 0, used: 0 }
    }

    fn bit(&mut self, b: bool) {
        if b {
            self.out[self.word + (self.used / 8) as usize] |= 1 << (self.used % 8);
        }
        self.used += 1;
        if self.used == 16 {
            self.word = self.out.len();
            self.out.extend([0, 0]);
            self.used = 0;
        }
    }

    fn code(&mut self, entry: u16) {
        let len = u32::from(entry >> 12);
        for k in (0..len).rev() {
            self.bit((entry >> k) & 1 != 0);
        }
    }

    fn left(&self) -> u8 {
        (16 - self.used) as u8
    }
}

/// How many of each token kind a stream used, so a round trip can show it
/// exercised the whole format rather than one easy corner of it.
#[derive(Debug, Default, Clone)]
pub struct Coverage {
    pub literals: usize,
    pub zero_literals: usize,
    pub short_matches: usize,
    pub long_matches: usize,
    pub two: usize,
    pub two_high: usize,
    pub separators: usize,
    pub stored: usize,
    pub high_bits_seen: u32,
}

pub fn compress(data: &[u8], m: &Mode, stored: &[(usize, usize)], separators: &[usize]) -> Vec<u8> {
    compress_with_coverage(data, m, stored, separators).0
}

/// Compress `data` into a PKLITE stream ending in the stop code. `stored` are
/// `(offset, len)` uncompressed areas (large mode only): `data` must hold
/// [`UNCOMPRESSED_SIG`] and the length byte at `offset`, followed by `len`
/// stored bytes, and nothing after an area is matched against anything before
/// its end. `separators` emit the no-op `0xfe` code before those offsets.
pub fn compress_with_coverage(
    data: &[u8],
    m: &Mode,
    stored: &[(usize, usize)],
    separators: &[usize],
) -> (Vec<u8>, Coverage) {
    let mut cov = Coverage::default();
    let a = alphabet(m);
    let offsets: &[u16] = if m.v120 { &OFFSETS_V120 } else { &OFFSETS };
    let max_len = a.bias + 0xfc;
    let mut w = Writer::new();
    let mut heads: std::collections::BTreeMap<[u8; 3], Vec<usize>> = Default::default();
    let mut floor = 0usize;
    let mut i = 0usize;
    let mut toggle = false;

    let literal = |w: &mut Writer, b: u8| {
        w.bit(false);
        let key = match m.extra {
            1 => w.left(),
            2 => 0xff,
            _ => 0,
        };
        w.out.push(b ^ key);
    };

    while i < data.len() {
        let next_event = stored
            .iter()
            .map(|a| a.0)
            .chain(separators.iter().copied())
            .filter(|&at| at > i)
            .min()
            .unwrap_or(data.len());
        if separators.contains(&i) {
            cov.separators += 1;
            w.bit(true);
            w.code(a.table[a.long]);
            w.out.push(0xfe);
        }
        if let Some(&(_, len)) = stored.iter().find(|(at, _)| *at == i) {
            assert_eq!(&data[i..i + 8], &UNCOMPRESSED_SIG);
            assert_eq!(usize::from(data[i + 8]), len);
            w.bit(true);
            w.code(a.table[a.long]);
            w.out.push(0xfd);
            w.out.push(len as u8);
            w.out.extend_from_slice(&data[i + 9..i + 9 + len]);
            cov.stored += 1;
            i += 9 + len;
            floor = i;
            continue;
        }

        let mut best = (0usize, 0usize);
        if i + 3 <= data.len() {
            if let Some(cands) = heads.get(&[data[i], data[i + 1], data[i + 2]]) {
                for &c in cands.iter().rev().take(48) {
                    let dist = i - c;
                    if dist > 8191 || c < floor {
                        break;
                    }
                    let mut n = 0;
                    while i + n < next_event && n < max_len && data[c + n] == data[i + n] {
                        n += 1;
                    }
                    if n > best.0 {
                        best = (n, dist);
                    }
                }
            }
        }
        if best.0 < 3 && i + 2 <= next_event {
            let reach = if m.v120 { 511 } else { 255 };
            for dist in 1..=reach.min(i - floor) {
                if data[i - dist] == data[i] && data[i - dist + 1] == data[i + 1] {
                    best = (2, dist);
                    break;
                }
            }
        }

        let (len, dist) = best;
        if len < 2 {
            let b = data[i];
            toggle = !toggle;
            match a.zero_literal {
                Some(z) if b == 0 && toggle => {
                    cov.zero_literals += 1;
                    w.bit(true);
                    w.code(a.table[z]);
                }
                _ => {
                    cov.literals += 1;
                    literal(&mut w, b)
                }
            }
        } else {
            w.bit(true);
            if len == 2 {
                if dist <= 255 {
                    cov.two += 1;
                    w.code(a.table[a.two]);
                } else {
                    cov.two_high += 1;
                    w.code(a.table[a.two_high.expect("v1.20 alphabet")]);
                }
            } else {
                if len < 3 + a.long {
                    cov.short_matches += 1;
                    w.code(a.table[len - 3]);
                } else {
                    cov.long_matches += 1;
                    w.code(a.table[a.long]);
                    w.out.push((len - a.bias) as u8);
                }
                cov.high_bits_seen |= 1 << (dist >> 8);
                w.code(offsets[dist >> 8]);
            }
            w.out.push((dist & 0xff) as u8 ^ m.offset_xor);
        }
        let step = len.max(1);
        for k in i..i + step {
            if k + 3 <= data.len() {
                heads.entry([data[k], data[k + 1], data[k + 2]]).or_default().push(k);
            }
        }
        i += step;
    }
    w.bit(true);
    w.code(a.table[a.long]);
    w.out.push(0xff);
    (w.out, cov)
}

/// The original, uncompressed MZ program.
#[derive(Debug, Clone)]
pub struct Program {
    /// Header bytes from `0x1c` up to the relocation table (a linker's stamp).
    pub stamp: Vec<u8>,
    pub relocs: Vec<(u16, u16)>,
    pub header_paras: u16,
    pub code: Vec<u8>,
    pub ss: u16,
    pub sp: u16,
    pub cs: u16,
    pub ip: u16,
    pub min_alloc: u16,
    pub max_alloc: u16,
    pub overlay: Vec<u8>,
    pub stored: Vec<(usize, usize)>,
}

fn put16(b: &mut Vec<u8>, v: u16) {
    b.extend_from_slice(&v.to_le_bytes());
}

fn mz_header(end: usize, nrelocs: usize, paras: u16, fields: [u16; 7], reloc_pos: u16) -> Vec<u8> {
    let mut h = b"MZ".to_vec();
    put16(&mut h, (end % 512) as u16);
    put16(&mut h, end.div_ceil(512) as u16);
    put16(&mut h, nrelocs as u16);
    put16(&mut h, paras);
    for f in &fields[..4] {
        put16(&mut h, *f);
    }
    put16(&mut h, 0);
    for f in &fields[4..6] {
        put16(&mut h, *f);
    }
    put16(&mut h, reloc_pos);
    put16(&mut h, fields[6]);
    h
}

impl Program {
    pub fn reloc_pos(&self) -> u16 {
        (28 + self.stamp.len()) as u16
    }

    /// The program as its linker wrote it.
    pub fn image(&self) -> Vec<u8> {
        let start = usize::from(self.header_paras) * 16;
        let end = start + self.code.len();
        let mut f = mz_header(
            end,
            self.relocs.len(),
            self.header_paras,
            [self.min_alloc, self.max_alloc, self.ss, self.sp, self.ip, self.cs, 0],
            self.reloc_pos(),
        );
        f.extend_from_slice(&self.stamp);
        for &(off, seg) in &self.relocs {
            put16(&mut f, off);
            put16(&mut f, seg);
        }
        assert!(f.len() <= start, "relocation table overruns the header");
        f.resize(start, 0);
        f.extend_from_slice(&self.code);
        f.extend_from_slice(&self.overlay);
        f
    }
}

/// A deterministic load module that exercises every token kind: random bytes,
/// zero runs, short repeats, long repeats past the longest match, and enough of
/// it to wrap the 8 KiB window. `stored` uncompressed areas are spliced in at
/// the offsets given.
pub fn program(seed: u32, len: usize, relocs: Vec<(u16, u16)>, stored_at: &[usize]) -> Program {
    let mut s = seed;
    let mut rnd = move || {
        s = s.wrapping_mul(1_103_515_245).wrapping_add(12345);
        (s >> 16) as u8
    };
    let phrase = b"\x55\x8b\xec\x83\xec\x08\x56\x57\x8b\x76\x04\xb4\x09\xcd\x21";
    let mut code = Vec::with_capacity(len + 512);
    while code.len() < len {
        match rnd() % 8 {
            0 | 1 => (0..1 + rnd() % 64).for_each(|_| code.push(rnd())),
            2 => code.extend(std::iter::repeat_n(0u8, usize::from(rnd() % 24))),
            3 => code.extend_from_slice(phrase),
            4 if code.len() > 600 => {
                let back = 1 + usize::from(rnd()) * 31 % 8100.min(code.len() - 1);
                let n = 2 + usize::from(rnd() % 60);
                let from = code.len() - back;
                for k in 0..n {
                    code.push(code[from + k]);
                }
            }
            5 if rnd() % 8 == 0 => {
                code.extend(std::iter::repeat_n(0x90u8, 300 + usize::from(rnd())))
            }
            5 => code.extend(std::iter::repeat_n(0x90u8, 2 + usize::from(rnd() % 10))),
            6 => code.extend_from_slice(b"Hello from a synthetic DOS program$"),
            _ => code.push(rnd()),
        }
    }
    code.truncate(len);
    let mut stored = Vec::new();
    for &at in stored_at {
        let n = 40usize;
        let mut area = UNCOMPRESSED_SIG.to_vec();
        area.push(n as u8);
        area.extend((0..n).map(|k| (k as u8).wrapping_mul(37) ^ 0x5a));
        code.splice(at..at + area.len(), area);
        stored.push((at, n));
    }
    let header_paras = (28 + 6 + relocs.len() * 4).div_ceil(16) as u16;
    Program {
        stamp: vec![0x01, 0x00, 0xfb, 0x20, 0x72, 0x6a],
        relocs,
        header_paras: header_paras.max(2),
        code,
        ss: 0x2e68,
        sp: 0x0080,
        cs: 0x1b58,
        ip: 0x0010,
        min_alloc: 0x0042,
        max_alloc: 0xffff,
        overlay: Vec::new(),
        stored,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intro {
    V100,
    V112,
    V114,
    V150,
    Un2Pack,
    Megalite,
    Beta,
    BetaLoadHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scramble {
    None,
    /// The v1.14 descrambler: XOR chain.
    Xor114,
    /// The v1.50 descrambler, in its ADD form (which also byte-swaps the long
    /// relocation table).
    Add150,
    /// The descrambler of the v1.50 IBM build: an XOR chain.
    Ibm150,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Copier {
    Common,
    Scr150,
    Pklite201,
    V120Small,
    Megalite,
    Un2Pack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decompressor {
    Common,
    V115,
    V120Small,
    V120SmallOld,
}

/// How a synthetic packed file is laid out.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub intro: Intro,
    pub scramble: Scramble,
    pub copier: Copier,
    pub decompressor: Decompressor,
    pub mode: Mode,
    pub version_word: u16,
    pub psp_sig: bool,
    /// Append a PKLAXFIX-style startup patch and point the entry at it.
    pub startup_patch: bool,
    /// `MOV AX` immediate in the intro (the paragraph check `MINALLOC` is
    /// estimated from when no header copy is kept).
    pub ax: u16,
    pub key: u16,
}

impl Layout {
    pub fn new(intro: Intro, mode: Mode) -> Layout {
        Layout {
            intro,
            scramble: Scramble::None,
            copier: Copier::Common,
            decompressor: Decompressor::Common,
            mode,
            version_word: 0x0100
                | if mode.large { 0x2000 } else { 0 }
                | if mode.extra != 0 { 0x1000 } else { 0 },
            psp_sig: false,
            startup_patch: false,
            ax: 0x1400,
            key: 0x3a5c,
        }
    }

    pub fn data_before_decoder(&self) -> bool {
        matches!(self.intro, Intro::Beta | Intro::BetaLoadHigh)
    }
}

const STUB_LEN: usize = 0x200;

fn place(ep: &mut [u8], at: usize, pat: &[u8]) {
    for (k, &b) in pat.iter().enumerate() {
        if b != b'?' {
            ep[at + k] = b;
        }
    }
}

fn put_word(ep: &mut [u8], at: usize, v: u16) {
    ep[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

/// The fingerprint skeleton of the stub, `STUB_LEN` bytes, entry at offset 0.
/// IP-relative immediates assume the entry is the start of code, as PKLITE lays
/// out every non-beta file.
fn stub(l: &Layout) -> Vec<u8> {
    let mut ep = vec![0u8; STUB_LEN];
    let ip = |eprel: usize| (eprel + 0x100) as u16;

    if l.data_before_decoder() {
        if l.intro == Intro::Beta {
            place(&mut ep, 0, b"\x2e\x8c\x1e??\x8b\x1e\x02\x00\x8c\xda\x81");
            place(&mut ep, 0x59, b"\xf3\xa5\x2e\xa1????????\xcb\xfc");
        } else {
            place(&mut ep, 0, b"\x2e\x8c\x1e??\xfc\x8c\xc8\x2e\x2b\x06");
        }
        let d = if l.intro == Intro::Beta { 0x66 } else { 5 };
        place(&mut ep, d, b"\xfc\x8c\xc8\x2e\x2b\x06??\x8e\xd8\xbf");
        decompressor_body(&mut ep, l, d + 24);
        return ep;
    }

    let ax_intro = |ep: &mut Vec<u8>, at: usize| {
        put_word(ep, at + 1, l.ax);
        put_word(ep, at + 4, l.key);
    };
    let position2 = match l.intro {
        Intro::V100 => {
            place(&mut ep, 0, b"\xb8??\xba??\x8c\xdb\x03\xd8\x3b");
            ax_intro(&mut ep, 0);
            16
        }
        Intro::V112 => {
            place(&mut ep, 0, b"\xb8??\xba??\x05\x00\x00\x3b\x06");
            ax_intro(&mut ep, 0);
            ep[13] = 0x73;
            15
        }
        Intro::V114 | Intro::Megalite => {
            if l.intro == Intro::V114 {
                place(&mut ep, 0, b"\xb8??\xba??\x05\x00\x00\x3b\x06");
            } else {
                place(&mut ep, 0, b"\xb8??\xba??\x05\x00\x00\x3b\x2d");
            }
            ax_intro(&mut ep, 0);
            ep[13] = 0x72;
            ep[14] = 5;
            20
        }
        Intro::V150 => {
            place(&mut ep, 0, b"\x50\xb8??\xba??\x05\x00\x00\x3b");
            ax_intro(&mut ep, 1);
            ep[14] = 0x72;
            ep[15] = 4;
            20
        }
        Intro::Un2Pack => {
            place(&mut ep, 0, b"\x9c\xba?\x00\x2d?\x00\x81\xe1?\x00\x81");
            34
        }
        Intro::Beta | Intro::BetaLoadHigh => unreachable!(),
    };

    let copier_pos = if l.scramble == Scramble::None { position2 } else { position2 + 64 };
    let copier_at = copier_pos + 8;
    let d = copier_pos + 48;
    match l.copier {
        Copier::Common | Copier::Scr150 => {
            place(&mut ep, copier_at, b"\xb9??\x33\xff\x57\xbe??\xfc\xf3\xa5");
            ep[copier_at + 12] = if l.copier == Copier::Common { 0xcb } else { 0xca };
            put_word(&mut ep, copier_at + 7, ip(d));
        }
        Copier::Pklite201 => {
            place(&mut ep, copier_at, b"\xb9??\x33\xff\x57\xfc\xbe??\xf3\xa5\xcb");
            put_word(&mut ep, copier_at + 8, ip(d));
        }
        Copier::V120Small => {
            place(&mut ep, copier_at, b"\x57\xb9??\xbe??\xfc\xf3\xa5\xc3");
            put_word(&mut ep, copier_at + 5, ip(d));
        }
        Copier::Megalite => {
            place(&mut ep, copier_at, b"\xb9??\x33\xff\x56\xbe??\xfc\xf2\xa5\xca");
            put_word(&mut ep, copier_at + 7, ip(d));
        }
        Copier::Un2Pack => {
            place(&mut ep, copier_at, b"\xb9??\x2b\xff\x57\xbe??\xfc\xf3\xa5\xcb");
            put_word(&mut ep, copier_at + 7, ip(d));
        }
    }

    match l.decompressor {
        Decompressor::Common => {
            place(&mut ep, d, b"\xfd\x8c\xdb\x53\x83\xc3");
            ep[d + 6] = ((STUB_LEN + 0x100) / 16) as u8;
            ep[d + 9] = 0xbe;
        }
        Decompressor::V115 => {
            place(&mut ep, d, b"\xfd\x8c\xdb\x53\x81\xc3");
            put_word(&mut ep, d + 6, ((STUB_LEN + 0x100) / 16) as u16);
        }
        Decompressor::V120Small => {
            place(&mut ep, d, b"\xfd\x5f\xc7\x85????\x4f\x4f\xbe??\x03\xf2\x8b\xca\xd1\xe9\xf3");
            put_word(&mut ep, d + 11, ip(STUB_LEN - 2));
        }
        Decompressor::V120SmallOld => {
            place(&mut ep, d, b"\xfd\x5f\x4f\x4f\xbe??\x03\xf2\x8b\xca\xd1\xe9\xf3");
            put_word(&mut ep, d + 5, ip(STUB_LEN - 2));
        }
    }
    decompressor_body(&mut ep, l, d + 24);

    match l.scramble {
        Scramble::None => {}
        Scramble::Xor114 | Scramble::Add150 | Scramble::Ibm150 => {
            let start = copier_pos;
            let last = STUB_LEN - 2;
            let count = (last + 2 - start) / 2;
            let (pat, count_at, end_at, jmp_at): (&[u8], usize, usize, usize) = match l.scramble {
                Scramble::Xor114 => (
                    b"\x2d\x20\x00\x8e\xd0\x2d??\x50\x52\xb9??\xbe??\x8b\xfe\xfd\x90\x49\x74?\xad\x92\x33\xc2\xab\xeb\xf6",
                    11,
                    14,
                    22,
                ),
                Scramble::Ibm150 => (
                    b"\x59\x2d\x20\x00\x8e\xd0\x51\x2d??\x50\x52\xb9??\xbe??\x8b\xfe\xfd\x90\x49\x74?\xad\x92\x33",
                    13,
                    16,
                    24,
                ),
                _ => (
                    b"\x59\x2d\x20\x00\x8e\xd0\x51??\x00\x50\x80\x3e\x41\x01\xc3\x75\xe6\x52\xb8??\xbe??\x56\x56\x52\x50\x90???????\x74",
                    20,
                    23,
                    38,
                ),
            };
            place(&mut ep, position2, pat);
            if l.scramble == Scramble::Add150 {
                ep[position2 + 45] = 0x03;
            }
            put_word(&mut ep, position2 + count_at, (count + 1) as u16);
            put_word(&mut ep, position2 + end_at, ip(last));
            ep[position2 + jmp_at] = (copier_pos - (position2 + jmp_at) - 1) as u8;
            let words: Vec<u16> = (0..count)
                .map(|k| u16::from_le_bytes([ep[start + 2 * k], ep[start + 2 * k + 1]]))
                .collect();
            let mut next = l.key;
            for k in (0..count).rev() {
                let s = if l.scramble == Scramble::Add150 {
                    words[k].wrapping_sub(next)
                } else {
                    words[k] ^ next
                };
                put_word(&mut ep, start + 2 * k, s);
                next = s;
            }
        }
    }
    ep
}

/// The parts of the decompressor the mode is read from: the literal-copy loop
/// (normal or extra), the length table or the v1.20 register clear at its end,
/// the offset XOR, and the PSP signature store.
fn decompressor_body(ep: &mut [u8], l: &Layout, at: usize) {
    match l.mode.extra {
        0 => place(ep, at, b"\xad\x95\xb2\x10\x72\x08\xa4\xd1\xed\x4a\x74"),
        1 => place(ep, at, b"\xad\x95\xb2\x10\x72\x0b\xac\x32\xc2\xaa\xd1\xed\x4a\x74"),
        _ => place(ep, at, b"\xad\x95\xb2\x10\x72\x0b\xac\xf6\xd0\xaa\xd1\xed\x4a\x74"),
    }
    if l.psp_sig {
        place(ep, at + 20, b"\xc7\x06\x5c\x00\x50\x4b");
    }
    if l.mode.v120 && l.mode.offset_xor != 0 {
        place(ep, at + 200, b"\xac\x34?\x8a");
        ep[at + 202] = l.mode.offset_xor;
    }
    let small_v120 = matches!(l.decompressor, Decompressor::V120Small | Decompressor::V120SmallOld);
    if l.mode.v120 && l.mode.large {
        place(ep, STUB_LEN - 20, b"\x33\xc0\x8b\xd8\x8b\xc8\x8b\xd0\x8b\xe8\x8b\xf0\x8b");
    } else if !small_v120 {
        ep[STUB_LEN - 31] = if l.mode.large { 0x18 } else { 0x09 };
        place(
            ep,
            STUB_LEN - 30,
            b"\x01\x02\x00\x00\x03\x04\x05\x06\x00\x00\x00\x00\x00\x00\x00\x00\x07\x08\x09\x0a\x0b",
        );
    }
}

fn reloc_table(p: &Program, l: &Layout) -> Vec<u8> {
    let mut t = Vec::new();
    if l.mode.extra == 0 {
        let mut i = 0;
        while i < p.relocs.len() {
            let seg = p.relocs[i].1;
            let n = p.relocs[i..].iter().take(255).take_while(|r| r.1 == seg).count();
            t.push(n as u8);
            put16(&mut t, seg);
            for r in &p.relocs[i..i + n] {
                put16(&mut t, r.0);
            }
            i += n;
        }
        t.push(0);
    } else {
        let groups = p.relocs.iter().map(|r| usize::from(r.1) / 0x0fff + 1).max().unwrap_or(0);
        for g in 0..groups {
            let offs: Vec<u16> =
                p.relocs.iter().filter(|r| usize::from(r.1) == g * 0x0fff).map(|r| r.0).collect();
            put16(&mut t, offs.len() as u16);
            for o in offs {
                let o = if l.scramble == Scramble::Add150 { o.swap_bytes() } else { o };
                put16(&mut t, o);
            }
        }
        put16(&mut t, 0xffff);
    }
    t
}

fn footer(p: &Program) -> Vec<u8> {
    let mut f = Vec::new();
    for v in [p.ss, p.sp, p.cs, p.ip] {
        put16(&mut f, v);
    }
    f
}

/// Pack `p` the way `l` describes.
pub fn pack(p: &Program, l: &Layout) -> Vec<u8> {
    let separators = if l.mode.large { vec![p.code.len() / 2] } else { Vec::new() };
    let stream = compress(&p.code, &l.mode, &p.stored, &separators);
    let tail = [reloc_table(p, l), footer(p)].concat();
    let copy: Vec<u8> = if l.mode.extra == 0 {
        p.image()[2..usize::from(p.reloc_pos())].to_vec()
    } else {
        Vec::new()
    };
    let start = (0x50 + copy.len()).div_ceil(16) * 16;
    let stub = stub(l);
    let (body, entry_rel): (Vec<u8>, usize) = if l.data_before_decoder() {
        let mut body = [stream, tail].concat();
        while body.len() % 16 != 0 {
            body.push(0);
        }
        let entry = body.len();
        body.extend_from_slice(&stub);
        (body, entry)
    } else {
        ([stub, stream, tail].concat(), 0)
    };
    let mut body = body;
    let (cs, ip) = if l.startup_patch {
        let at = body.len();
        let mut patch = [0u8; 17];
        patch[8] = 0x05;
        patch[9..11].copy_from_slice(&0xfff0u16.to_le_bytes());
        patch[12] = 0xb8;
        patch[13..15].copy_from_slice(&0x0100u16.to_le_bytes());
        body.extend_from_slice(&patch);
        (((at >> 4) as u16).wrapping_sub(16), (at & 15) as u16 + 0x100)
    } else if l.data_before_decoder() {
        (((entry_rel as i64 - 0x100) / 16) as u16, 0x100)
    } else {
        (0xfff0, 0x100)
    };
    let end = start + body.len();
    let mut f =
        mz_header(end, 0, (start / 16) as u16, [0x0200, 0xffff, 0x1000, 0x0200, ip, cs, 0], 0x50);
    put16(&mut f, l.version_word);
    f.extend_from_slice(b"PKLITE-format test image, synthetic stub");
    f.resize(0x50, 0);
    f.extend_from_slice(&copy);
    f.resize(start, 0);
    f.extend_from_slice(&body);
    f.extend_from_slice(&p.overlay);
    f
}

/// What a correct unpacker writes for [`pack`]`(p, l)`: the original image when
/// the header copy is kept, otherwise the header deark's rules rebuild.
pub fn expected(p: &Program, l: &Layout) -> Vec<u8> {
    if l.mode.extra == 0 {
        return p.image();
    }
    let n = p.relocs.len();
    let start = (28 + n * 4).div_ceil(16) * 16;
    let end = start + p.code.len();
    let mut min = 0x0200i64;
    if !l.data_before_decoder() {
        let est = (i64::from(l.ax) << 4) + 0x100 - p.code.len() as i64;
        if est >= 0 {
            min = (est + 15) >> 4;
        }
    }
    let max = 0xffffi64.max(min);
    let mut f = mz_header(
        end,
        n,
        (start / 16) as u16,
        [min as u16, max as u16, p.ss, p.sp, p.ip, p.cs, 0],
        28,
    );
    for &(off, seg) in &p.relocs {
        put16(&mut f, off);
        put16(&mut f, seg);
    }
    f.resize(start, 0);
    f.extend_from_slice(&p.code);
    f.extend_from_slice(&p.overlay);
    f
}

/// A PKLITE COM file around `code`, v1.00-1.14 layout (data at offset 448).
pub fn pack_com(code: &[u8], version_word: u16) -> Vec<u8> {
    let data = 448usize;
    let mut f = vec![0u8; data];
    place(&mut f, 0, b"\xb8??\xba??\x3b\xc4\x73\x67");
    place(&mut f, 10, b"\x8b\xc4\x2d??\x25\xf0\xff\x8b\xf8\xb9??\xbe");
    let d = 64usize;
    put_word(&mut f, 24, (d + 0x100) as u16);
    put_word(&mut f, 44, version_word);
    f[46..52].copy_from_slice(b"PKLITE");
    place(&mut f, d, b"\xfd\x8b\xf8\x4f\x4f\xbe");
    put_word(&mut f, d + 6, (data - 2 + 0x100) as u16);
    f[260] = 0x1d;
    let mode = Mode { large: false, v120: false, extra: 0, offset_xor: 0 };
    f.extend_from_slice(&compress(code, &mode, &[], &[]));
    f
}

/// A named layout and the relocation table its program carries.
pub type Case = (&'static str, Layout, Vec<(u16, u16)>);

/// Every layout the unpacker is expected to handle, with a name for messages.
pub fn layouts() -> Vec<Case> {
    let m = |large, v120, extra, offset_xor| Mode { large, v120, extra, offset_xor };
    let short_relocs: Vec<(u16, u16)> = (0..58u16)
        .map(|k| (0x017d + 0x4d * (k % 7), [0x1b58, 0x0000, 0x0d2a][usize::from(k % 3)]))
        .collect();
    let long_relocs: Vec<(u16, u16)> = (0..40u16)
        .map(|k| (0x0100 + k * 9, [0x0000, 0x0fff, 0x1ffe][usize::from(k / 14)]))
        .collect();
    let mut out = Vec::new();
    let mut add = |name, l: Layout, extra: bool| {
        out.push((name, l, if extra { long_relocs.clone() } else { short_relocs.clone() }));
    };
    add("v1.00 large", Layout::new(Intro::V100, m(true, false, 0, 0)), false);
    add("v1.00 small", Layout::new(Intro::V100, m(false, false, 0, 0)), false);
    add("v1.00 small extra", Layout::new(Intro::V100, m(false, false, 1, 0)), true);
    add("v1.00 large extra", Layout::new(Intro::V100, m(true, false, 1, 0)), true);
    add("v1.23 complement", Layout::new(Intro::V100, m(true, false, 2, 0)), true);
    add(
        "v1.12 small, v1.15 decompressor",
        Layout {
            decompressor: Decompressor::V115,
            ..Layout::new(Intro::V112, m(false, false, 0, 0))
        },
        false,
    );
    add(
        "v1.14 XOR-scrambled large extra",
        Layout { scramble: Scramble::Xor114, ..Layout::new(Intro::V114, m(true, false, 1, 0)) },
        true,
    );
    add(
        "v1.50 ADD-scrambled large extra",
        Layout {
            scramble: Scramble::Add150,
            copier: Copier::Scr150,
            decompressor: Decompressor::V115,
            ..Layout::new(Intro::V150, m(true, false, 1, 0))
        },
        true,
    );
    add(
        "v1.50 ADD-scrambled large",
        Layout {
            scramble: Scramble::Add150,
            copier: Copier::Pklite201,
            ..Layout::new(Intro::V150, m(true, false, 0, 0))
        },
        false,
    );
    add(
        "v1.50 IBM-scrambled small extra",
        Layout { scramble: Scramble::Ibm150, ..Layout::new(Intro::V150, m(false, false, 1, 0)) },
        true,
    );
    add(
        "v1.20 small, offset key",
        Layout {
            copier: Copier::V120Small,
            decompressor: Decompressor::V120Small,
            ..Layout::new(Intro::V112, m(false, true, 1, 0x5c))
        },
        true,
    );
    add(
        "v1.20 small (old)",
        Layout {
            copier: Copier::V120Small,
            decompressor: Decompressor::V120SmallOld,
            ..Layout::new(Intro::V112, m(false, true, 1, 0))
        },
        true,
    );
    add("v1.20 large, offset key", Layout::new(Intro::V112, m(true, true, 1, 0xa7)), true);
    add(
        "Megalite",
        Layout { copier: Copier::Megalite, ..Layout::new(Intro::Megalite, m(true, false, 0, 0)) },
        false,
    );
    add(
        "UN2PACK",
        Layout { copier: Copier::Un2Pack, ..Layout::new(Intro::Un2Pack, m(false, false, 0, 0)) },
        false,
    );
    add("May 1990 beta", Layout::new(Intro::Beta, m(true, false, 0, 0)), false);
    add("beta, load-high", Layout::new(Intro::BetaLoadHigh, m(false, false, 1, 0)), true);
    add(
        "startup patch",
        Layout { startup_patch: true, ..Layout::new(Intro::V100, m(true, false, 0, 0)) },
        false,
    );
    add(
        "tamper check",
        Layout { psp_sig: true, ..Layout::new(Intro::V100, m(false, false, 0, 0)) },
        false,
    );
    out
}
