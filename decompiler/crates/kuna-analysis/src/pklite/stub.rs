//! Reading the decompression parameters out of the PKLITE stub's machine code.
//!
//! The version word in the header is not trustworthy, so every parameter comes
//! from known byte patterns walked component by component from the entry point:
//! the *intro* (which may jump over a *descrambler* that un-XORs or un-ADDs the
//! rest of the stub with a key from `DX`), the *copier* that moves the stub to
//! high memory, and the *decompressor*, whose immediates give the compressed
//! data's paragraph and whose instruction choices give the compression mode.
//! Patterns use `?` as a wildcard byte.

use super::decode::Extra;
use super::PkliteError;

/// How many bytes at the entry point the walk can look at.
pub(super) const EP_LEN: usize = 1000;

/// The handful of MZ header fields the walk needs, in deark's signed arithmetic.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Exe {
    pub num_relocs: i64,
    pub ss: i64,
    pub sp: i64,
    pub cs: i64,
    pub ip: i64,
    pub reloc_table_pos: i64,
    pub start_of_code: i64,
    pub entry: i64,
    pub end_of_code: i64,
    pub overlay_len: i64,
}

pub(super) fn u16_at(b: &[u8], at: i64) -> i64 {
    if at < 0 {
        return 0;
    }
    let at = at as usize;
    i64::from(u16::from_le_bytes([
        b.get(at).copied().unwrap_or(0),
        b.get(at + 1).copied().unwrap_or(0),
    ]))
}

pub(super) fn i16_at(b: &[u8], at: i64) -> i64 {
    i64::from(u16_at(b, at) as u16 as i16)
}

impl Exe {
    pub fn parse(f: &[u8]) -> Exe {
        let lfb = u16_at(f, 2);
        let nblocks = u16_at(f, 4) & 0x7ff;
        let hdr = u16_at(f, 8);
        let mut e = Exe {
            num_relocs: u16_at(f, 6),
            ss: i16_at(f, 14),
            sp: u16_at(f, 16),
            ip: u16_at(f, 20),
            cs: i16_at(f, 22),
            reloc_table_pos: u16_at(f, 24),
            start_of_code: hdr * 16,
            ..Exe::default()
        };
        e.entry = (hdr + e.cs) * 16 + e.ip;
        e.end_of_code = nblocks * 512;
        if (1..=511).contains(&lfb) {
            e.end_of_code = e.end_of_code - 512 + lfb;
        }
        e.overlay_len = (f.len() as i64 - e.end_of_code).max(0);
        e
    }
}

fn matches(mem: &[u8], at: i64, pat: &[u8]) -> bool {
    if at < 0 {
        return false;
    }
    let at = at as usize;
    match mem.get(at..at + pat.len()) {
        Some(m) => m.iter().zip(pat).all(|(&b, &p)| p == b'?' || b == p),
        None => false,
    }
}

fn search(mem: &[u8], start: i64, end: i64, pat: &[u8]) -> Option<i64> {
    let start = start.max(0);
    let end = end.min(mem.len() as i64);
    if pat.len() as i64 > end - start {
        return None;
    }
    (start..=end - pat.len() as i64).find(|&i| matches(mem, i, pat))
}

/// The entry-point fingerprint, before any variant has been confirmed, among
/// the families `flags` admits: `1` the released versions (and UN2PACK's
/// patch), `2` the 1990 betas, `4` Megalite.
pub(super) fn intro_class(ep: &[u8], flags: u8) -> u32 {
    const CLASSES: [(&[u8], u32, u8); 7] = [
        (b"\xb8??\xba??\x8c\xdb\x03\xd8\x3b", 100, 1),
        (b"\xb8??\xba??\x05\x00\x00\x3b\x06", 112, 1),
        (b"\x50\xb8??\xba??\x05\x00\x00\x3b", 150, 1),
        (b"\x9c\xba?\x00\x2d?\x00\x81\xe1?\x00\x81", 250, 1),
        (b"\x2e\x8c\x1e??\x8b\x1e\x02\x00\x8c\xda\x81", 90, 2),
        (b"\x2e\x8c\x1e??\xfc\x8c\xc8\x2e\x2b\x06", 91, 2),
        (b"\xb8??\xba??\x05\x00\x00\x3b\x2d", 251, 4),
    ];
    CLASSES
        .iter()
        .find(|(p, _, family)| flags & family != 0 && matches(ep, 0, p))
        .map_or(0, |&(_, c, _)| c)
}

/// Which [`intro_class`] families a header of this shape could carry: `CS:IP`
/// at `FFF0:0100` with the entry at the start of code, the May 1990 beta's
/// entry past it, or Megalite's `0000:0000`. This is what keeps detection
/// from matching four instructions at the entry of an arbitrary program.
pub(super) fn host_shape(e: &Exe) -> u8 {
    if e.ip == 256 && e.cs == -16 && e.num_relocs <= 2 && e.entry == e.start_of_code {
        1
    } else if e.ip == 256 && e.cs > -16 && e.num_relocs == 0 && e.entry > e.start_of_code {
        2
    } else if e.cs == 0
        && e.ip == 0
        && e.num_relocs == 1
        && e.reloc_table_pos == 28
        && e.entry == e.start_of_code
    {
        4
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Intro {
    Beta,
    BetaLoadHigh,
    V100,
    V112,
    V114,
    V150,
    Un2Pack,
    Megalite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Decompressor {
    Common,
    V115,
    V120Small,
    V120SmallOld,
    Beta,
}

/// Everything the walk established.
#[derive(Debug, Clone)]
pub(super) struct Analysis {
    pub intro: Intro,
    pub data_before_decoder: bool,
    /// `0` none, `1` XOR, `2` ADD -- the latter also reverses the byte order of
    /// the long relocation table.
    pub scramble: u8,
    pub copier_byte17: u8,
    pub copier_byte23: u8,
    pub decompressor: Decompressor,
    pub decompr_byte9: u8,
    pub cmpr_data_pos: i64,
    /// Where the reloc table and footer must end: the end of the load module,
    /// or the entry point when the data precedes the decoder.
    pub cmpr_data_area_end: i64,
    pub large: bool,
    pub v120: bool,
    pub extra: Extra,
    pub offset_xor: u8,
    /// `"PK"` / `"pk"` written into the PSP: the program checks it at run time.
    pub psp_sig: Option<&'static str>,
}

fn unsupported(what: String) -> PkliteError {
    PkliteError::Unsupported(what)
}

/// A one-byte forward jump at `at`: its target, relative to the entry point.
fn follow_jump(ep: &[u8], at: i64) -> i64 {
    if !(0..EP_LEN as i64).contains(&at) {
        return 0;
    }
    at + 1 + i64::from(ep[at as usize])
}

/// Walk the stub. `ep` is a writable copy of the [`EP_LEN`] bytes at the entry
/// point, descrambled in place; `class` is [`intro_class`] of it (non-zero).
/// `patch` is a startup patch's file offset (the host's own entry point), which
/// ends the load module, or the decoder of a file whose data precedes it.
pub(super) fn analyze(
    host: &Exe,
    ep: &mut [u8],
    class: u32,
    patch: Option<i64>,
) -> Result<Analysis, PkliteError> {
    let ip_to_eprel = |ip: i64| (host.start_of_code + (ip - 0x100) - host.entry).max(0);

    let key = if matches(ep, 0, b"\xb8??\xba") {
        u16_at(ep, 4)
    } else if matches(ep, 0, b"\x50\xb8??\xba") {
        u16_at(ep, 5)
    } else {
        0
    };

    let (intro, position2, data_before_decoder) = match class {
        90 => (Intro::Beta, 0, true),
        91 => (Intro::BetaLoadHigh, 0, true),
        100 => (Intro::V100, 16, false),
        112 if ep[13] == 0x73 => (Intro::V112, 15, false),
        112 if ep[13] == 0x72 => (Intro::V114, follow_jump(ep, 14), false),
        150 if ep[14] == 0x72 => (Intro::V150, follow_jump(ep, 15), false),
        250 => (Intro::Un2Pack, 34, false),
        251 if ep[13] == 0x72 => (Intro::Megalite, follow_jump(ep, 14), false),
        _ => {
            return Err(unsupported(format!(
                "the entry code opens like PKLITE (class {class}) but continues like no known \
                 version"
            )))
        }
    };
    if !data_before_decoder && position2 == 0 {
        return Err(unsupported("the intro's jump leads nowhere".into()));
    }

    let mut scramble = 0u8;
    let mut copier_pos = position2;
    if matches!(intro, Intro::V112 | Intro::V114 | Intro::V150) && position2 + 200 <= EP_LEN as i64
    {
        if let Some(d) = descrambler(ep, position2) {
            scramble = match ep[d.op as usize] {
                0x33 => 1,
                0x03 => 2,
                op => {
                    return Err(unsupported(format!(
                        "the descrambler combines words with opcode {op:#04x}, neither XOR nor ADD"
                    )))
                }
            };
            let count = (u16_at(ep, d.count) - 1).max(0);
            let last = ip_to_eprel(u16_at(ep, d.endpos));
            copier_pos = follow_jump(ep, d.jmp);
            if count >= 1 {
                descramble(ep, scramble, count, last, key)?;
            }
        }
    }

    let mut decompr_pos = 0;
    let (mut copier_byte17, mut copier_byte23) = (0, 0);
    if !data_before_decoder {
        if copier_pos == 0 || copier_pos + 200 > EP_LEN as i64 {
            return Err(unsupported("the copier is outside the stub's first 1000 bytes".into()));
        }
        copier_byte17 = ep[(copier_pos + 17) as usize];
        copier_byte23 = ep[(copier_pos + 23) as usize];
        let field = copier_field(ep, copier_pos)
            .ok_or_else(|| unsupported("the copier matches no known PKLITE copier".into()))?;
        decompr_pos = ip_to_eprel(u16_at(ep, field));
    } else if matches(ep, 0x59, b"\xf3\xa5\x2e\xa1????????\xcb\xfc") {
        decompr_pos = 0x66;
    } else if matches(ep, 0x5b, b"\xf3\xa5\x85\xed????????????\xcb\xfc") {
        decompr_pos = 0x6c;
    } else if matches(ep, 0, b"\x2e\x8c\x1e??\xfc\x8c\xc8\x2e\x2b\x06") {
        decompr_pos = 5;
    }

    if decompr_pos == 0 || decompr_pos + 200 > EP_LEN as i64 {
        return Err(unsupported("no decompressor where the copier hands over".into()));
    }
    let pos = decompr_pos;
    let decompr_byte9 = ep[(pos + 9) as usize];
    let (decompressor, cmpr_data_pos) = if matches(ep, pos, b"\xfd\x8c\xdb\x53\x83\xc3") {
        let n = i64::from(ep[(pos + 6) as usize]) * 16;
        (Decompressor::Common, host.entry + ip_to_eprel(n))
    } else if matches(ep, pos, b"\xfd\x8c\xdb\x53\x81\xc3") {
        let n = u16_at(ep, pos + 6) * 16;
        (Decompressor::V115, host.entry + ip_to_eprel(n))
    } else if matches(ep, pos, b"\xfd\x5f\xc7\x85????\x4f\x4f\xbe??\x03\xf2\x8b\xca\xd1\xe9\xf3") {
        (Decompressor::V120Small, host.entry + 2 + ip_to_eprel(u16_at(ep, pos + 11)))
    } else if matches(ep, pos, b"\xfd\x5f\x4f\x4f\xbe??\x03\xf2\x8b\xca\xd1\xe9\xf3") {
        (Decompressor::V120SmallOld, host.entry + 2 + ip_to_eprel(u16_at(ep, pos + 5)))
    } else if matches(ep, pos, b"\xfc\x8c\xc8\x2e\x2b\x06??\x8e\xd8\xbf") {
        (Decompressor::Beta, host.start_of_code)
    } else {
        return Err(unsupported("the decompressor matches no known PKLITE decompressor".into()));
    };
    if cmpr_data_pos == 0 {
        return Err(unsupported("the decompressor names no compressed data".into()));
    }

    let (approx_end, cmpr_data_area_end) = if data_before_decoder {
        (patch.unwrap_or(host.end_of_code) - host.entry, host.entry)
    } else {
        (cmpr_data_pos - host.entry, patch.unwrap_or(host.end_of_code))
    };

    let extra = if search(ep, pos, approx_end, b"\xad\x95\xb2\x10\x72\x08\xa4\xd1\xed\x4a\x74")
        .is_some()
    {
        Extra::None
    } else {
        match search(ep, pos, approx_end, b"\xad\x95\xb2\x10\x72\x0b\xac??\xaa\xd1\xed\x4a\x74") {
            Some(f) if ep[f as usize + 7] == 0x32 && ep[f as usize + 8] == 0xc2 => Extra::BitCount,
            Some(f) if ep[f as usize + 7] == 0xf6 && ep[f as usize + 8] == 0xd0 => {
                Extra::Complement
            }
            _ => {
                return Err(unsupported(
                    "the literal-copy loop matches neither normal nor extra (-e) compression"
                        .into(),
                ))
            }
        }
    };

    let (large, v120) =
        if matches!(decompressor, Decompressor::V120Small | Decompressor::V120SmallOld) {
            (false, true)
        } else if let Some(f) = search(
            ep,
            approx_end - 60,
            approx_end,
            b"\x01\x02\x00\x00\x03\x04\x05\x06\x00\x00\x00\x00\x00\x00\x00\x00\x07\x08\x09\x0a\x0b",
        )
        .filter(|&f| f > 0)
        {
            match ep[f as usize - 1] {
                0x09 => (false, false),
                0x18 => (true, false),
                b => {
                    return Err(unsupported(format!(
                    "the length table is prefixed {b:#04x}, neither small (0x09) nor large (0x18)"
                )))
                }
            }
        } else if extra != Extra::None
            && search(
                ep,
                approx_end - 50,
                approx_end,
                b"\x33\xc0\x8b\xd8\x8b\xc8\x8b\xd0\x8b\xe8\x8b\xf0\x8b",
            )
            .is_some()
        {
            (true, true)
        } else {
            return Err(unsupported("no length table to tell small from large compression".into()));
        };

    let offset_xor = if v120 {
        search(ep, pos + 200, approx_end, b"\xac\x34?\x8a").map_or(0, |f| ep[f as usize + 2])
    } else {
        0
    };

    let psp_sig = if search(ep, pos, approx_end, b"\xc7\x06\x5c\x00\x70\x6b").is_some() {
        Some("pk")
    } else if search(ep, pos, approx_end, b"\xc7\x06\x5c\x00\x50\x4b").is_some() {
        Some("PK")
    } else {
        None
    };

    Ok(Analysis {
        intro,
        data_before_decoder,
        scramble,
        copier_byte17,
        copier_byte23,
        decompressor,
        decompr_byte9,
        cmpr_data_pos,
        cmpr_data_area_end,
        large,
        v120,
        extra,
        offset_xor,
        psp_sig,
    })
}

struct Descrambler {
    count: i64,
    endpos: i64,
    jmp: i64,
    op: i64,
}

fn descrambler(ep: &[u8], pos: i64) -> Option<Descrambler> {
    // (pattern, word-count field, end-position field, jump field, operation byte)
    const FORMS: [(&[u8], i64, i64, i64, i64); 8] = [
        (
            b"\x2d\x20\x00\x8e\xd0\x2d??\x50\x52\xb9??\xbe??\x8b\xfe\xfd\x90\x49\x74?\xad\x92\x33\xc2\xab\xeb\xf6",
            11,
            14,
            22,
            25,
        ),
        (
            b"\x8b\xfc\x81\xef??\x57\x57\x52\xb9??\xbe??\x8b\xfe\xfd\x49\x74?\xad\x92\x03\xc2\xab\xeb\xf6",
            10,
            13,
            20,
            23,
        ),
        (
            b"\x8b\xfc\x81\xef??\x57\x57\x52\xb9??\xbe??\x8b\xfe\xfd\x90\x49\x74?\xad\x92\x03\xc2\xab\xeb\xf6",
            10,
            13,
            21,
            24,
        ),
        (
            b"\x59\x2d\x20\x00\x8e\xd0\x51??\x00\x50\x80\x3e\x41\x01\xc3\x75\xe6\x52\xb8??\xbe??\x56\x56\x52\x50\x90???????\x74",
            20,
            23,
            38,
            45,
        ),
        (b"\x2d\x20\x00????????????\xb9??\xbe????????\x74???\x03", 16, 19, 28, 31),
        (b"\x2d\x20\x00????????????\xb9??\xbe?????????\x74???\x03", 16, 19, 29, 32),
        (
            b"\x2d\x20\x00?????????????????\xb9??\xbe??????????\x74???\x03",
            21,
            24,
            35,
            38,
        ),
        (b"\x8b\xfc\x81?????????????\xbb??\xbe??????\x74???\x03", 17, 20, 27, 30),
    ];
    const IBM: (&[u8], i64, i64, i64, i64) = (
        b"\x59\x2d\x20\x00\x8e\xd0\x51\x2d??\x50\x52\xb9??\xbe??\x8b\xfe\xfd\x90\x49\x74?\xad\x92\x33",
        13,
        16,
        24,
        27,
    );
    FORMS.iter().chain(std::iter::once(&IBM)).find(|(p, ..)| matches(ep, pos, p)).map(
        |&(_, count, endpos, jmp, op)| Descrambler {
            count: pos + count,
            endpos: pos + endpos,
            jmp: pos + jmp,
            op: pos + op,
        },
    )
}

/// Undo the descrambler: each word of the region is combined with the
/// *scrambled* word after it, and the last with the intro's `DX` key.
fn descramble(
    ep: &mut [u8],
    method: u8,
    count: i64,
    last: i64,
    key: i64,
) -> Result<(), PkliteError> {
    if last + 2 > EP_LEN as i64 {
        return Err(PkliteError::Corrupt("the scrambled region runs past the stub".into()));
    }
    let start = last + 2 - count * 2;
    if start < 0 {
        return Err(PkliteError::Corrupt("the scrambled region starts before the stub".into()));
    }
    let mut this = u16_at(ep, start) as u16;
    let mut pos = start;
    while pos <= last {
        let next = if pos == last { key as u16 } else { u16_at(ep, pos + 2) as u16 };
        let plain = if method == 2 { this.wrapping_add(next) } else { this ^ next };
        ep[pos as usize..pos as usize + 2].copy_from_slice(&plain.to_le_bytes());
        this = next;
        pos += 2;
    }
    Ok(())
}

/// The copier's `MOV SI, decompressor` immediate, searched in the 75 bytes
/// after `pos`.
fn copier_field(ep: &[u8], pos: i64) -> Option<i64> {
    let end = pos + 75;
    if let Some(f) = search(ep, pos, end, b"\xb9??\x33\xff\x57\xbe??\xfc\xf3\xa5") {
        return Some(f + 7);
    }
    const FORMS: [(&[u8], i64); 4] = [
        (b"\xb9??\x33\xff\x57\xfc\xbe??\xf3\xa5\xcb", 8),
        (b"\x57\xb9??\xbe??\xfc\xf3\xa5\xc3", 5),
        (b"\xb9??\x33\xff\x56\xbe??\xfc\xf2\xa5\xca", 7),
        (b"\xb9??\x2b\xff\x57\xbe??\xfc\xf3\xa5\xcb", 7),
    ];
    FORMS.iter().find_map(|(p, off)| search(ep, pos, end, p).map(|f| f + off))
}

/// Which PKLITE release wrote the stub, as far as the stub and the version word
/// can tell; `None` when nothing narrows it.
pub(super) fn detect_version(a: &Analysis, reported: u16) -> Option<String> {
    const NAMES: [(u32, &str); 12] = [
        (10005, "1.00 beta (May 1990)"),
        (10007, "1.00 beta (July 1990)"),
        (10050, "1.00"),
        (10350, "1.03"),
        (10550, "1.05"),
        (11150, "1.11"),
        (11250, "1.12"),
        (11350, "1.13"),
        (11450, "1.14"),
        (11550, "1.15"),
        (15050, "1.50"),
        (20150, "2.01"),
    ];
    #[derive(Default)]
    struct V {
        set: bool,
        special: bool,
        min: u32,
        max: u32,
    }
    impl V {
        fn range(&mut self, lo: u32, hi: u32, weak: bool) {
            if self.special {
                return;
            }
            if lo > hi || (self.set && (lo > self.max || hi < self.min)) {
                if !weak {
                    self.set = true;
                    self.special = true;
                }
                return;
            }
            if !self.set {
                *self = V { set: true, special: false, min: lo, max: hi };
            }
            self.min = self.min.max(lo);
            self.max = self.max.min(hi);
        }
    }
    let mut v = V::default();
    let extra = a.extra != Extra::None;
    'walk: {
        if a.v120 {
            v.set = true;
            v.special = true;
            break 'walk;
        }
        match a.intro {
            Intro::Beta | Intro::BetaLoadHigh => {
                v.range(10005, 10005, false);
                break 'walk;
            }
            Intro::V100 => {
                v.range(10007, 10550, false);
                if a.decompressor == Decompressor::Common {
                    if a.decompr_byte9 == 0xbe {
                        v.range(10007, 10350, false);
                        if a.large && !extra {
                            let exact = match a.copier_byte17 {
                                0x21 => Some(10007),
                                0x22 => Some(10050),
                                0x23 => Some(10350),
                                _ => None,
                            };
                            if let Some(x) = exact {
                                v.range(x, x, false);
                                break 'walk;
                            }
                        }
                        if !a.large && !extra {
                            if a.copier_byte17 == 0xc2 {
                                v.range(10007, 10007, false);
                                break 'walk;
                            }
                            v.range(10050, 10350, false);
                        }
                        if !a.large && extra {
                            if a.copier_byte17 == 0xc6 && a.copier_byte23 == 0x4c {
                                v.range(10007, 10007, false);
                                break 'walk;
                            }
                            v.range(10050, 10350, false);
                        }
                    } else if a.decompr_byte9 == 0x8c {
                        v.range(10550, 10550, false);
                        break 'walk;
                    }
                }
            }
            Intro::V112 => v.range(11150, 11350, false),
            Intro::V114 => v.range(11450, 11550, false),
            Intro::V150 => v.range(15050, 20150, false),
            Intro::Un2Pack | Intro::Megalite => {}
        }
        match reported & 0x0fff {
            0x100 => v.range(10005, 10050, true),
            0x103 => v.range(10350, 10350, true),
            0x10b => v.range(11150, 11150, true),
            0x10c | 0x10d => v.range(11250, 11350, true),
            _ => {}
        }
    }
    if !v.set {
        return None;
    }
    if v.special {
        return Some("a nonstandard or v1.20-style variant".into());
    }
    let name = |x: u32| NAMES.iter().find(|(k, _)| *k == x).map_or("?", |(_, n)| n);
    Some(if v.min == v.max {
        name(v.min).to_string()
    } else {
        format!("{}-{}", name(v.min), name(v.max))
    })
}
