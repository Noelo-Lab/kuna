//! Static unpacking of PKLITE-compressed DOS executables (EXE and COM).
//!
//! PKLITE (PKWARE, 1990-1996) replaces a program's load module with a small
//! decompressor stub followed by the compressed module, a compressed relocation
//! table and an 8-byte footer holding the original `SS:SP` and `CS:IP`; when
//! extra compression (`-e`) is off, the stub's header also keeps a copy of the
//! original MZ header. Everything an analysis needs is therefore in the file, but
//! *where* it is is not declared anywhere trustworthy: the compression mode and
//! the data's position are immediates inside the stub's machine code, which
//! differs per release and may itself be scrambled. [`stub`] reads them out of
//! known byte patterns; [`decode`] is the LZ77/Huffman decoder.
//!
//! # Refusing beats guessing
//!
//! A stub variant that matches no known pattern is a named
//! [`PkliteError::Unsupported`]; a stream or table that does not decode to the
//! end it should is [`PkliteError::Corrupt`]. The format carries no checksum, so
//! the proofs are structural: the stream must end on its stop code, the
//! relocation table must end exactly where an 8-byte footer still fits before
//! the end of the load module, and a kept copy of the original header is used
//! only if its registers and relocation count agree with the footer and table.
//!
//! Ported from deark (<https://github.com/jsummers/deark>), `modules/pklite.c`
//! and `src/fmtutil-exe.c`, which carry this notice:
//!
//! > Copyright (C) 2016-2026 Jason Summers
//! >
//! > Permission is hereby granted, free of charge, to any person obtaining a copy
//! > of this software and associated documentation files (the "Software"), to
//! > deal in the Software without restriction, including without limitation the
//! > rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
//! > sell copies of the Software, and to permit persons to whom the Software is
//! > furnished to do so, subject to the following conditions:
//! >
//! > The above copyright notice and this permission notice shall be included in
//! > all copies or substantial portions of the Software.
//! >
//! > THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! > IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! > FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! > AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! > LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
//! > FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
//! > IN THE SOFTWARE.

mod decode;
mod stub;

use std::fmt;

pub use decode::Extra;
use stub::{i16_at, u16_at, Exe, EP_LEN};

/// deark's ceiling on a relocation table: one more than an MZ header can count.
const MAX_RELOCS: usize = 65535;

/// The largest file a COM loader accepts.
const MAX_COM: usize = 65280;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PkliteError {
    /// No PKLITE stub at the entry point.
    NotPacked,
    /// Recognized as PKLITE, but a variant this build cannot unpack -- and says why.
    Unsupported(String),
    /// The stub is PKLITE's but the data behind it does not decode consistently.
    Corrupt(String),
}

impl fmt::Display for PkliteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PkliteError::NotPacked => write!(f, "no PKLITE decompressor stub found"),
            PkliteError::Unsupported(what) => write!(f, "unsupported PKLITE file: {what}"),
            PkliteError::Corrupt(what) => write!(f, "corrupt PKLITE file: {what}"),
        }
    }
}

impl std::error::Error for PkliteError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Container {
    Exe,
    Com,
}

/// Where the recovered MZ header came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderSource {
    /// The copy of the original header PKLITE kept, at this file offset.
    Original(usize),
    /// Rebuilt from the footer and the decoded sizes: extra compression keeps
    /// no copy (or the copy disagreed with the footer). `MINALLOC` is estimated
    /// from the stub's memory check and `MAXALLOC` is the packed file's.
    Reconstructed,
    /// A COM file has no header.
    None,
}

/// A successful unpack.
#[derive(Debug, Clone)]
pub struct Unpacked {
    /// The reconstructed original file.
    pub bytes: Vec<u8>,
    pub container: Container,
    /// The header's version word, e.g. `1.00/l` (`/l` large, `/e` extra) --
    /// what the file claims, which is not always what wrote it.
    pub reported_version: String,
    /// What the stub's own code narrows the release to, when it can.
    pub detected_version: Option<String>,
    pub large: bool,
    pub extra: Extra,
    pub v120: bool,
    pub scrambled: bool,
    /// File range of the compressed load module.
    pub compressed_offset: usize,
    pub compressed_end: usize,
    /// Bytes of decompressed load module (the whole file, for a COM).
    pub load_module: usize,
    pub relocations: usize,
    pub header: HeaderSource,
    pub cs: u16,
    pub ip: u16,
    pub ss: u16,
    pub sp: u16,
    pub min_alloc: u16,
    pub max_alloc: u16,
    /// Bytes past the packed load module carried over to the end of the output.
    pub overlay: usize,
    /// Facts about the result a reader should know; none of them stop it.
    pub warnings: Vec<String>,
}

fn is_mz(image: &[u8]) -> bool {
    matches!(image.get(..2), Some(b"MZ") | Some(b"ZM"))
}

/// A Windows or OS/2 program: `e_lfanew` names a `PE`, `NE`, `LE` or `LX`
/// header. Its DOS stub may be PKLITE-compressed, but the program is the
/// new-style executable, which PKLITE never packs.
fn is_new_exe(image: &[u8]) -> bool {
    let Some(&[a, b, c, d]) = image.get(0x3c..0x40) else {
        return false;
    };
    let at = u32::from_le_bytes([a, b, c, d]) as usize;
    image.get(at..at.saturating_add(4)) == Some(b"PE\0\0")
        || matches!(image.get(at..at.saturating_add(2)), Some(b"NE" | b"LE" | b"LX"))
}

fn entry_bytes(image: &[u8], at: i64) -> Option<Vec<u8>> {
    let at = usize::try_from(at).ok().filter(|&a| a < image.len())?;
    let mut ep = image[at..image.len().min(at + EP_LEN)].to_vec();
    ep.resize(EP_LEN, 0);
    Some(ep)
}

/// Does `image` carry a PKLITE stub? Cheap: the header and 64 bytes at the
/// entry point. Stricter than [`unpack`] (the header must have the shape
/// PKLITE writes), because it answers for files nobody asked to unpack.
pub fn detect(image: &[u8]) -> bool {
    if is_mz(image) {
        if is_new_exe(image) {
            return false;
        }
        let host = Exe::parse(image);
        let shape = stub::host_shape(&host);
        return shape != 0
            && entry_bytes(image, host.entry)
                .is_some_and(|ep| stub::intro_class(&ep[..64], shape) != 0);
    }
    image.len() <= MAX_COM && com_layout(image).is_some_and(|v| com_has_copyright(image, v))
}

/// Unpack `image`, returning the original file.
pub fn unpack(image: &[u8]) -> Result<Unpacked, PkliteError> {
    if is_mz(image) {
        unpack_exe(image)
    } else {
        unpack_com(image)
    }
}

/// The version word as PKLITE's own `-v` prints it.
fn version_string(word: u16) -> String {
    let v = word & 0x0fff;
    format!(
        "{}.{:02}{}{}{}",
        v >> 8,
        v & 0xff,
        if word & 0x4000 != 0 { "/h" } else { "" },
        if word & 0x2000 != 0 { "/l" } else { "/s" },
        if word & 0x1000 != 0 { "/e" } else { "" },
    )
}

/// PKLAXFIX-style startup patch: a 17-byte jump appended at the end of the load
/// module that the header's entry point now names. The real stub is where it
/// jumps; the patch itself is dropped.
fn startup_patch(image: &[u8], host: &Exe) -> Option<i64> {
    let at = host.end_of_code - 17;
    if host.entry != at || at < 0 {
        return None;
    }
    let byte = |o: i64| image.get((at + o) as usize).copied();
    (byte(8) == Some(0x05) && byte(12) == Some(0xb8)).then_some(at)
}

fn corrupt(what: String) -> PkliteError {
    PkliteError::Corrupt(what)
}

struct Footer {
    ss: i64,
    sp: i64,
    cs: i64,
    ip: i64,
}

fn unpack_exe(image: &[u8]) -> Result<Unpacked, PkliteError> {
    if is_new_exe(image) {
        return Err(PkliteError::NotPacked);
    }
    let mut host = Exe::parse(image);
    let patch = startup_patch(image, &host);
    if let Some(at) = patch {
        host.entry = host.start_of_code + i16_at(image, at + 9) * 16 + u16_at(image, at + 13);
    }
    let mut ep = entry_bytes(image, host.entry).ok_or(PkliteError::NotPacked)?;
    let class = stub::intro_class(&ep, 0xff);
    if class == 0 {
        return Err(PkliteError::NotPacked);
    }
    let reported = u16_at(image, 28) as u16;
    let a = stub::analyze(&host, &mut ep, class, patch)?;
    let mut warnings = Vec::new();

    if a.cmpr_data_area_end > image.len() as i64 {
        return Err(corrupt(format!(
            "the header declares {} bytes of load module but the file holds {}",
            a.cmpr_data_area_end,
            image.len()
        )));
    }
    let params = decode::Params {
        large: a.large,
        v120: a.v120,
        extra: a.extra,
        offset_xor: a.offset_xor,
        allow_prehistory: reported & 0x0fff == 0x100,
    };
    let dec = decode::decompress(
        image,
        a.cmpr_data_pos as usize,
        host.start_of_code.max(0) as usize,
        &params,
    )?;
    if dec.uncompressed_area {
        warnings.push(
            "the stream holds an uncompressed area; its 8-byte signature and length byte are \
             restored as PKLITE's documentation describes them"
                .into(),
        );
    }

    let reloc_start = dec.end as i64;
    let reloc_len = a.cmpr_data_area_end - 8 - reloc_start;
    let (relocs, reloc_end) = if a.extra != Extra::None {
        relocs_long(image, reloc_start, reloc_len, a.scramble == 2)?
    } else {
        relocs_short(image, reloc_start, reloc_len)?
    };
    let footer_room = a.cmpr_data_area_end - reloc_end;
    if footer_room > 8 + 15 {
        warnings.push(format!(
            "{} unexplained bytes between the footer and the end of the load module",
            footer_room - 8
        ));
    }
    let footer = Footer {
        ss: i16_at(image, reloc_end),
        sp: u16_at(image, reloc_end + 2),
        cs: i16_at(image, reloc_end + 4),
        ip: u16_at(image, reloc_end + 6),
    };

    let (hdr, source) =
        match original_header(image, &host, &a, &footer, &relocs, &dec.bytes, &mut warnings) {
            Some(found) => found,
            None => (
                reconstruct_header(image, &host, &a, &footer, relocs.len(), dec.bytes.len()),
                HeaderSource::Reconstructed,
            ),
        };
    let mut guest = Exe::parse(&hdr);
    if guest.reloc_table_pos < 28 {
        guest.reloc_table_pos = 28;
    }
    let beta_size_bug =
        matches!(a.intro, stub::Intro::Beta | stub::Intro::BetaLoadHigh) && u16_at(&hdr, 2) == 0;

    let reloc_pos = guest.reloc_table_pos as usize;
    let code_pos = guest.start_of_code as usize;
    let mut out = hdr[..hdr.len().min(reloc_pos)].to_vec();
    out.resize(reloc_pos, 0);
    for &(off, seg) in &relocs {
        out.extend_from_slice(&off.to_le_bytes());
        out.extend_from_slice(&seg.to_le_bytes());
    }
    if out.len() > code_pos && !relocs.is_empty() {
        return Err(corrupt(format!(
            "the original header puts code at {code_pos:#x}, inside its own relocation table \
             ({:#x}..{:#x})",
            reloc_pos,
            out.len()
        )));
    }
    out.resize(code_pos, 0);
    out.extend_from_slice(&dec.bytes);

    let mut overlay = 0;
    if let Some(from) = usize::try_from(host.end_of_code).ok().filter(|_| host.overlay_len > 0) {
        if beta_size_bug || out.len() as i64 == guest.end_of_code {
            out.extend_from_slice(&image[from..]);
            overlay = image.len() - from;
        } else {
            warnings.push(format!(
                "{} bytes of overlay were not copied: the recovered header does not end where \
                 the decompressed module does",
                host.overlay_len
            ));
        }
    }
    if let Some(sig) = a.psp_sig {
        warnings.push(format!(
            "the program checks for a \"{sig}\" signature PKLITE writes into its PSP, so the \
             unpacked file may refuse to run (it still analyzes)"
        ));
    }

    let fin = Exe::parse(&out);
    Ok(Unpacked {
        container: Container::Exe,
        reported_version: version_string(reported),
        detected_version: stub::detect_version(&a, reported),
        large: a.large,
        extra: a.extra,
        v120: a.v120,
        scrambled: a.scramble != 0,
        compressed_offset: a.cmpr_data_pos as usize,
        compressed_end: dec.end,
        load_module: dec.bytes.len(),
        relocations: relocs.len(),
        header: source,
        cs: fin.cs as u16,
        ip: fin.ip as u16,
        ss: fin.ss as u16,
        sp: fin.sp as u16,
        min_alloc: u16_at(&out, 10) as u16,
        max_alloc: u16_at(&out, 12) as u16,
        overlay,
        warnings,
        bytes: out,
    })
}

/// The "short" table (normal compression): groups of a count byte, a segment
/// word and that many offset words, ended by a zero count.
fn relocs_short(image: &[u8], start: i64, len: i64) -> Result<(Vec<(u16, u16)>, i64), PkliteError> {
    let end = start + len;
    let mut pos = start;
    let mut out = Vec::new();
    loop {
        if pos + 1 > end {
            return Err(reloc_overrun());
        }
        let count = image[pos as usize] as usize;
        pos += 1;
        if count == 0 {
            return Ok((out, pos));
        }
        if out.len() + count > MAX_RELOCS || pos + 2 + count as i64 * 2 > end {
            return Err(reloc_overrun());
        }
        let seg = u16_at(image, pos) as u16;
        pos += 2;
        for _ in 0..count {
            out.push((u16_at(image, pos) as u16, seg));
            pos += 2;
        }
    }
}

/// The "long" table (extra compression): groups of a count word and that many
/// offsets, the segment advancing by `0x0fff` per group, ended by `0xffff`.
/// The ADD-scrambled stubs store the offsets big-endian.
fn relocs_long(
    image: &[u8],
    start: i64,
    len: i64,
    big_endian: bool,
) -> Result<(Vec<(u16, u16)>, i64), PkliteError> {
    let end = start + len;
    let mut pos = start;
    let mut seg: i64 = 0;
    let mut out = Vec::new();
    loop {
        if pos + 2 > end {
            return Err(reloc_overrun());
        }
        let count = u16_at(image, pos) as usize;
        pos += 2;
        if count == 0xffff {
            return Ok((out, pos));
        }
        if seg > 0xffff || out.len() + count > MAX_RELOCS || pos + count as i64 * 2 > end {
            return Err(reloc_overrun());
        }
        for _ in 0..count {
            let w = u16_at(image, pos) as u16;
            out.push((if big_endian { w.swap_bytes() } else { w }, seg as u16));
            pos += 2;
        }
        seg += 0x0fff;
    }
}

fn reloc_overrun() -> PkliteError {
    corrupt("the relocation table does not end before the 8-byte footer".into())
}

/// The copy of the original header PKLITE keeps between the stub's header and
/// its code (normal compression only). `None` when there is none, or when it
/// disagrees with the footer or the relocation table -- then it is not used.
fn original_header(
    image: &[u8],
    host: &Exe,
    a: &stub::Analysis,
    footer: &Footer,
    relocs: &[(u16, u16)],
    code: &[u8],
    warnings: &mut Vec<String>,
) -> Option<(Vec<u8>, HeaderSource)> {
    if a.extra != Extra::None {
        return None;
    }
    let pos = host.reloc_table_pos + 4 * host.num_relocs;
    let mut len = host.start_of_code - pos;
    if len < 26 {
        return None;
    }
    let orig_reloc = u16_at(image, pos + 22);
    if orig_reloc >= 28 && orig_reloc < 2 + len {
        len = orig_reloc - 2;
    }
    let bad = |warnings: &mut Vec<String>, why: &str| {
        warnings.push(format!(
            "the kept copy of the original header {why}; the header was rebuilt instead"
        ));
        None
    };
    if u16_at(image, pos) > 511 || u16_at(image, pos + 2) == 0 {
        return bad(warnings, "has an impossible size");
    }
    let mut hdr = b"MZ".to_vec();
    hdr.extend((0..len).map(|i| image.get((pos + i) as usize).copied().unwrap_or(0)));
    let guest = Exe::parse(&hdr);
    if (guest.ss, guest.sp, guest.cs, guest.ip) != (footer.ss, footer.sp, footer.cs, footer.ip) {
        return bad(warnings, "disagrees with the footer's SS:SP / CS:IP");
    }
    if guest.num_relocs != relocs.len() as i64 {
        return bad(warnings, "counts a different number of relocations than the table holds");
    }
    let expected = guest.end_of_code - guest.start_of_code;
    if code.len() as i64 != expected {
        warnings.push(format!(
            "the original header declares {expected} bytes of load module but {} decompressed{}",
            code.len(),
            if (code.len() as i64) < expected {
                " (the original file may have claimed more than it held)"
            } else {
                ""
            }
        ));
    }
    Some((hdr, HeaderSource::Original(pos as usize)))
}

/// Rebuild a 28-byte MZ header from the footer and the decoded sizes, code
/// aligned to 16 bytes.
fn reconstruct_header(
    image: &[u8],
    host: &Exe,
    a: &stub::Analysis,
    footer: &Footer,
    nrelocs: usize,
    code_len: usize,
) -> Vec<u8> {
    let mut minmem = u16_at(image, 10);
    let mut maxmem = u16_at(image, 12);
    if maxmem == 0 {
        maxmem = 65535;
    }
    if !a.data_before_decoder {
        let mut pos = host.entry;
        let mut b = image.get(pos as usize).copied().unwrap_or(0);
        pos += 1;
        if b == 0x50 {
            b = image.get(pos as usize).copied().unwrap_or(0);
            pos += 1;
        }
        if b == 0xb8 {
            let n = (u16_at(image, pos) << 4) + 0x100 - code_len as i64;
            if n >= 0 {
                minmem = (n + 0xf) >> 4;
            }
        }
    }
    if maxmem < minmem {
        maxmem = minmem;
    }
    let nrelocs = nrelocs as i64;
    let start = (28 + nrelocs * 4 + 15) / 16 * 16;
    let end = start + code_len as i64;
    let fields = [
        end % 512,
        (end + 511) / 512,
        nrelocs,
        start / 16,
        minmem,
        maxmem,
        footer.ss,
        footer.sp,
        0,
        footer.ip,
        footer.cs,
        28,
        0,
    ];
    let mut hdr = b"MZ".to_vec();
    for f in fields {
        hdr.extend_from_slice(&(f as u16).to_le_bytes());
    }
    hdr
}

/// The COM stub's version-word offset, when its first ten bytes have the shape
/// of one.
fn com_layout(image: &[u8]) -> Option<i64> {
    let b = image.get(..10)?;
    if b[0] == 0xb8 && b[3] == 0xba && b[6] == 0x3b && b[7] == 0xc4 {
        match b[9] {
            0x67 => Some(44),
            0x69 => Some(46),
            _ => None,
        }
    } else if b[0] == 0x50 && b[1] == 0xb8 && b[4] == 0xba && b[7] == 0x3b {
        Some(46)
    } else if b[0] == 0xba && b[3] == 0xa1 && b[6] == 0x2d && b[7] == 0x20 {
        Some(36)
    } else {
        None
    }
}

fn com_has_copyright(image: &[u8], verpos: i64) -> bool {
    let at = verpos as usize + 2;
    if verpos == 38 {
        return image.get(at..at + 8) == Some(b"PK Copyr");
    }
    match image.get(at..at + 4) {
        Some(&[b'P', k, l, i]) => {
            matches!(k, b'K' | b'k') && matches!(l, b'L' | b'l') && matches!(i, b'I' | b'i')
        }
        _ => false,
    }
}

fn unpack_com(image: &[u8]) -> Result<Unpacked, PkliteError> {
    if image.len() > MAX_COM {
        return Err(PkliteError::NotPacked);
    }
    let verpos = com_layout(image).ok_or(PkliteError::NotPacked)?;
    let mut ep = image[..image.len().min(EP_LEN)].to_vec();
    ep.resize(EP_LEN, 0);
    let fail = |what: &str| {
        if com_has_copyright(image, verpos) {
            PkliteError::Unsupported(format!("COM stub: {what}"))
        } else {
            PkliteError::NotPacked
        }
    };
    let m = |at: i64, pat: &[u8]| {
        ep.get(at as usize..at as usize + pat.len())
            .is_some_and(|w| w.iter().zip(pat).all(|(&b, &p)| p == b'?' || b == p))
    };

    let position2 = if m(0, b"\xb8??\xba??\x3b\xc4\x73") {
        10
    } else if m(0, b"\x50\xb8??\xba??\x3b\xc4\x73") {
        11
    } else if m(0, b"\xba??\xa1\x02\x00\x2d??\x8c\xcb??????\x77") {
        19 + i64::from(ep[18])
    } else {
        return Err(fail("the intro matches no known PKLITE COM intro"));
    };
    if position2 + 100 > EP_LEN as i64 {
        return Err(fail("the copier lies outside the first 1000 bytes"));
    }
    let decompr_pos = if m(position2, b"\x8b\xc4\x2d??\x25\xf0\xff\x8b\xf8\xb9??\xbe") {
        u16_at(&ep, position2 + 14) - 0x100
    } else if m(position2, b"\x8b\xc4\x2d??\x90\x25\xf0\xff\x8b\xf8\xb9??\x90\xbe") {
        u16_at(&ep, position2 + 16) - 0x100
    } else if m(position2, b"\xfa\xbc\x00\x02\x8e\xd0\xfb") {
        position2 + 24
    } else {
        return Err(fail("the copier matches no known PKLITE COM copier"));
    };
    if decompr_pos <= 0 || decompr_pos + 100 > EP_LEN as i64 {
        return Err(fail("no decompressor where the copier hands over"));
    }
    let key = if m(decompr_pos, b"\xfd\x8b\xf8\x4f\x4f\xbe") {
        decompr_pos + 6
    } else if m(decompr_pos, b"\xfd\xbe??\x03\xf2\x8b\xfa\x4f\x4f") {
        decompr_pos + 2
    } else {
        return Err(fail("the decompressor matches no known PKLITE COM decompressor"));
    };
    let data = u16_at(&ep, key) + 2 - 0x100;
    if data < 1 || data > image.len() as i64 {
        return Err(fail("the decompressor names data outside the file"));
    }

    let reported = u16_at(image, verpos) as u16;
    let params = decode::Params {
        large: false,
        v120: false,
        extra: Extra::None,
        offset_xor: 0,
        allow_prehistory: reported & 0x0fff == 0x100,
    };
    let dec = decode::decompress(image, data as usize, 0, &params)?;
    let detected = match data {
        500 => Some("1.00 beta"),
        448 => Some(match image.get(260) {
            Some(0x1d) => "1.00-1.03",
            Some(0x1c) => "1.05-1.14",
            _ => "1.00-1.14",
        }),
        450 => Some("1.15"),
        464 => Some("1.50-2.01"),
        _ => None,
    };
    Ok(Unpacked {
        container: Container::Com,
        reported_version: version_string(reported),
        detected_version: detected.map(str::to_string),
        large: false,
        extra: Extra::None,
        v120: false,
        scrambled: false,
        compressed_offset: data as usize,
        compressed_end: dec.end,
        load_module: dec.bytes.len(),
        relocations: 0,
        header: HeaderSource::None,
        cs: 0,
        ip: 0x100,
        ss: 0,
        sp: 0,
        min_alloc: 0,
        max_alloc: 0,
        overlay: 0,
        warnings: Vec::new(),
        bytes: dec.bytes,
    })
}

#[cfg(test)]
#[path = "../../tests/fixtures/pklite_synth.rs"]
mod synth;

#[cfg(test)]
mod tests;
