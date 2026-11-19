//! Firmware containers that carry a header or a record format instead of an
//! object file: U-Boot legacy images, Intel HEX, Motorola S-records, uClinux
//! bFLT and Qualcomm MBN wrappers. An Espressif application image is not
//! one of them: its segments load far apart, so the engine loads it as a
//! segmented image ([`super::esp`]).
//!
//! Each becomes either one flat image (the bytes between the lowest and the
//! highest loaded address, gaps filled) with the processor, base and entries
//! its header states, or the object file it wraps.

use std::io::Read;

/// A flat image to load as a raw image.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Flat {
    pub format: &'static str,
    /// SLEIGH language id, when the container names the processor.
    pub target: Option<String>,
    pub base: u64,
    pub bytes: Vec<u8>,
    pub entries: Vec<u64>,
    /// 32-bit ARM code starts in Thumb state.
    pub thumb: bool,
    /// Function names the container carries.
    pub symbols: Vec<(u64, String)>,
    /// Why the processor is not stated although the container was recognized.
    pub note: Option<String>,
}

/// What a container holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Container {
    Flat(Flat),
    /// An object file wrapped in a vendor header.
    Embedded { format: &'static str, bytes: Vec<u8> },
}

/// Largest span a flattened image may cover.
const MAX_SPAN: u64 = 256 << 20;

/// Recognize `bytes` as one of the containers.
pub fn detect(bytes: &[u8]) -> Result<Option<Container>, String> {
    if bytes.starts_with(&[0x27, 0x05, 0x19, 0x56]) {
        return uimage(bytes).map(Some);
    }
    if bytes.starts_with(b"bFLT") {
        return bflt(bytes).map(Some);
    }
    if bytes.first() == Some(&b':') {
        if let Some(c) = ihex(bytes)? {
            return Ok(Some(c));
        }
    }
    if bytes.first() == Some(&b'S') && bytes.get(1).is_some_and(|c| c.is_ascii_digit()) {
        if let Some(c) = srec(bytes)? {
            return Ok(Some(c));
        }
    }
    if let Some(c) = md1img(bytes)? {
        return Ok(Some(c));
    }
    if let Some(c) = mbn(bytes) {
        return Ok(Some(c));
    }
    Ok(bios(bytes))
}

fn be32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(at..at + 4)?.try_into().ok()?))
}
fn le32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// Lay `(address, bytes)` chunks out flat from the lowest address.
fn flatten(chunks: &[(u64, Vec<u8>)], fill: u8) -> Result<(u64, Vec<u8>), String> {
    let lo = chunks.iter().map(|c| c.0).min().ok_or("no data")?;
    let hi = chunks.iter().map(|c| c.0 + c.1.len() as u64).max().unwrap_or(lo);
    if hi - lo > MAX_SPAN {
        return Err(format!("loaded addresses span {:#x} bytes", hi - lo));
    }
    let mut out = vec![fill; (hi - lo) as usize];
    for (addr, data) in chunks {
        let at = (addr - lo) as usize;
        out[at..at + data.len()].copy_from_slice(data);
    }
    Ok((lo, out))
}

/// U-Boot legacy image (`include/image.h`): a 64-byte big-endian header, then
/// the payload loaded at `ih_load`, entered at `ih_ep`.
fn uimage(b: &[u8]) -> Result<Container, String> {
    let size = be32(b, 12).ok_or("short uImage header")? as usize;
    let load = u64::from(be32(b, 16).ok_or("short uImage header")?);
    let ep = u64::from(be32(b, 20).ok_or("short uImage header")?);
    let (arch, comp) = (b[29], b[31]);
    let payload = b.get(64..64 + size).ok_or("uImage payload runs past the file")?;
    let bytes = match comp {
        0 => payload.to_vec(),
        1 => {
            let mut out = Vec::new();
            flate2::read::GzDecoder::new(payload)
                .take(MAX_SPAN)
                .read_to_end(&mut out)
                .map_err(|e| format!("uImage gzip payload: {e}"))?;
            out
        }
        other => return Err(format!("uImage compression {other} (bzip2/lzma/lzo/lz4/zstd) is not decoded")),
    };
    let target = match arch {
        2 => Some("ARM:LE:32:v7".to_string()),
        22 => Some("AARCH64:LE:64:v8A".into()),
        3 => Some("x86:LE:32:default".into()),
        24 => Some("x86:LE:64:default".into()),
        5 => Some(format!("MIPS:{}:32:default", mips_endian(&bytes))),
        6 => Some(format!("MIPS:{}:64:default", mips_endian(&bytes))),
        7 => Some("PowerPC:BE:32:default".into()),
        9 => Some("SuperH4:LE:32:default".into()),
        10 => Some("sparc:BE:32:default".into()),
        11 => Some("sparc:BE:64:default".into()),
        12 => Some("68000:BE:32:default".into()),
        25 => Some("Xtensa:LE:32:default".into()),
        26 => Some("RISCV:LE:64:default".into()),
        _ => None,
    };
    Ok(Container::Flat(Flat {
        format: "U-Boot uImage",
        target,
        base: load,
        bytes,
        entries: vec![ep & !u64::from(arch == 2)],
        thumb: arch == 2 && ep & 1 == 1,
        ..Flat::default()
    }))
}

/// MIPS byte order by which reading finds more `jr ra` (0x03e00008).
fn mips_endian(code: &[u8]) -> &'static str {
    let count = |be: bool| {
        code.chunks_exact(4)
            .filter(|w| {
                let v = if be { u32::from_be_bytes([w[0], w[1], w[2], w[3]]) } else { u32::from_le_bytes([w[0], w[1], w[2], w[3]]) };
                v == 0x03e0_0008
            })
            .count()
    };
    if count(true) >= count(false) { "BE" } else { "LE" }
}

fn hex_line(line: &str) -> Option<Vec<u8>> {
    let digits = line.strip_prefix(':')?;
    if digits.len() % 2 != 0 {
        return None;
    }
    let rec: Option<Vec<u8>> = (0..digits.len()).step_by(2).map(|i| u8::from_str_radix(&digits[i..i + 2], 16).ok()).collect();
    let rec = rec?;
    let n = *rec.first()? as usize;
    (rec.len() == n + 5 && rec.iter().fold(0u8, |a, &b| a.wrapping_add(b)) == 0).then_some(rec)
}

/// Intel HEX: data records relative to the extended segment or linear base,
/// entry from the start-address record.
fn ihex(b: &[u8]) -> Result<Option<Container>, String> {
    let Ok(text) = std::str::from_utf8(b) else { return Ok(None) };
    let mut chunks = Vec::new();
    let mut upper = 0u64;
    let mut entries = Vec::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let Some(rec) = hex_line(line) else { return Ok(None) };
        let addr = u64::from(u16::from_be_bytes([rec[1], rec[2]]));
        let data = &rec[4..rec.len() - 1];
        match rec[3] {
            0 => chunks.push((upper + addr, data.to_vec())),
            1 => break,
            2 if data.len() == 2 => upper = u64::from(u16::from_be_bytes([data[0], data[1]])) << 4,
            3 if data.len() == 4 => {
                let cs = u64::from(u16::from_be_bytes([data[0], data[1]]));
                entries.push((cs << 4) + u64::from(u16::from_be_bytes([data[2], data[3]])));
            }
            4 if data.len() == 2 => upper = u64::from(u16::from_be_bytes([data[0], data[1]])) << 16,
            5 if data.len() == 4 => entries.push(u64::from(u32::from_be_bytes([data[0], data[1], data[2], data[3]]))),
            _ => return Ok(None),
        }
    }
    if chunks.is_empty() {
        return Ok(None);
    }
    let (base, bytes) = flatten(&chunks, 0xff)?;
    Ok(Some(Container::Flat(Flat { format: "Intel HEX", base, bytes, entries, ..Flat::default() })))
}

/// Motorola S-record: S1/S2/S3 data with 16/24/32-bit addresses, S7/S8/S9 entry.
fn srec(b: &[u8]) -> Result<Option<Container>, String> {
    let Ok(text) = std::str::from_utf8(b) else { return Ok(None) };
    let mut chunks = Vec::new();
    let mut entries = Vec::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let (Some(kind), Some(rest)) = (line.get(1..2), line.get(2..)) else { return Ok(None) };
        if line.as_bytes()[0] != b'S' || rest.len() % 2 != 0 {
            return Ok(None);
        }
        let rec: Option<Vec<u8>> = (0..rest.len()).step_by(2).map(|i| u8::from_str_radix(&rest[i..i + 2], 16).ok()).collect();
        let Some(rec) = rec else { return Ok(None) };
        if rec.first().map(|&n| n as usize + 1) != Some(rec.len()) || rec.iter().fold(0u8, |a, &x| a.wrapping_add(x)) != 0xff {
            return Ok(None);
        }
        let width = match kind {
            "1" | "9" => 2,
            "2" | "8" => 3,
            "3" | "7" => 4,
            "0" | "5" | "6" => continue,
            _ => return Ok(None),
        };
        let addr = rec[1..1 + width].iter().fold(0u64, |a, &x| a << 8 | u64::from(x));
        match kind {
            "1" | "2" | "3" => chunks.push((addr, rec[1 + width..rec.len() - 1].to_vec())),
            _ => entries.push(addr),
        }
    }
    if chunks.is_empty() {
        return Ok(None);
    }
    let (base, bytes) = flatten(&chunks, 0xff)?;
    Ok(Some(Container::Flat(Flat { format: "Motorola S-record", base, bytes, entries, ..Flat::default() })))
}

/// uClinux bFLT v4 (`include/uapi/linux/flat.h`): big-endian header; text,
/// data and bss at their file offsets from 0, relocations and GOT entries
/// relative to the end of the 64-byte header.
fn bflt(b: &[u8]) -> Result<Container, String> {
    let field = |i: usize| be32(b, 4 * i).ok_or("short bFLT header");
    if field(1)? != 4 {
        return Err(format!("bFLT revision {} (only 4 is read)", field(1)?));
    }
    let (entry, data_start, data_end, bss_end) = (field(2)?, field(3)? as usize, field(4)? as usize, field(5)? as usize);
    let (reloc_start, reloc_count, flags) = (field(7)? as usize, field(8)? as usize, field(9)?);
    if flags & 0x4 != 0 {
        return Err("gzip-compressed bFLT is not decoded".into());
    }
    let mut image = b.get(..data_end).ok_or("bFLT data runs past the file")?.to_vec();
    image.resize(bss_end.max(data_end), 0);
    const HDR: u32 = 0x40;
    let mut patch = |at: usize| {
        if let Some(v) = be32(&image, at).filter(|&v| v != 0 && v != u32::MAX) {
            image[at..at + 4].copy_from_slice(&(v + HDR).to_be_bytes());
        }
    };
    if flags & 0x2 != 0 {
        let mut at = data_start;
        while be32(b, at).is_some_and(|v| v != u32::MAX) && at + 4 <= data_end {
            patch(at);
            at += 4;
        }
    }
    for i in 0..reloc_count {
        let Some(r) = be32(b, reloc_start + 4 * i) else { break };
        patch(r as usize + HDR as usize);
    }
    Ok(Container::Flat(Flat {
        format: "uClinux bFLT",
        target: Some("68000:BE:32:Coldfire".into()),
        base: 0,
        bytes: image,
        entries: vec![u64::from(entry)],
        thumb: false,
        ..Flat::default()
    }))
}

/// A Qualcomm MBN / SBL image: a vendor header (or a GPT disk image) in front
/// of an ELF for Hexagon, ARM or AArch64; the ELF is what loads.
fn mbn(b: &[u8]) -> Option<Container> {
    if b.starts_with(b"\x7fELF") {
        return None;
    }
    let sbl = be32(b, 0) == Some(0xd1dc_4b84) || le32(b, 0) == Some(0x844b_dcd1);
    let gpt = b.get(0x200..0x208) == Some(b"EFI PART");
    let header = matches!(le32(b, 4), Some(3 | 5 | 6 | 7)) && le32(b, 0).is_some_and(|id| id < 0x40);
    if !(sbl || gpt || header) {
        return None;
    }
    let limit = b.len().min(1 << 20);
    (0..limit).step_by(4).find_map(|at| {
        let e = b.get(at..at + 0x34)?;
        let machine = u16::from_le_bytes([e[18], e[19]]);
        (e.starts_with(b"\x7fELF") && e[4] == 1 && e[5] == 1 && matches!(machine, 0x28 | 0xa4 | 0xb7))
            .then(|| Container::Embedded { format: "Qualcomm MBN", bytes: b[at..].to_vec() })
    })
}

const MD1_MAGIC: u32 = 0x5888_1688;
const MD1_EXT_MAGIC: u32 = 0x5889_1689;
/// The MediaTek section header (`magic, dsize, name[32], maddr, mode,
/// ext_magic, hdr_size, hdr_version, img_type, img_list_end, align_size,
/// dsize_extend, maddr_extend`).
const MD1_HEADER: usize = 0x50;
/// Where the modem ROM runs.
pub const MD1ROM_BASE: u64 = 0x9000_0000;

/// One `md1img` section: name and data.
fn md1_sections(b: &[u8]) -> Option<Vec<(String, &[u8])>> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off + MD1_HEADER <= b.len() {
        if le32(b, off)? != MD1_MAGIC || le32(b, off + 0x30)? != MD1_EXT_MAGIC {
            break;
        }
        let dsize = le32(b, off + 4)? as usize;
        let hdr = le32(b, off + 0x34)? as usize;
        let align = (le32(b, off + 0x44)? as usize).max(1);
        if hdr < MD1_HEADER {
            break;
        }
        let name_bytes = b.get(off + 8..off + 40)?;
        let name = String::from_utf8_lossy(&name_bytes[..name_bytes.iter().position(|&c| c == 0).unwrap_or(32)]).to_string();
        let data = b.get(off + hdr..off + hdr + dsize)?;
        out.push((name, data));
        off += hdr + dsize;
        off += (align - dsize % align) % align;
    }
    (!out.is_empty()).then_some(out)
}

fn cstring(b: &[u8], at: &mut usize) -> Option<String> {
    let rest = b.get(*at..)?;
    let n = rest.iter().position(|&c| c == 0)?;
    *at += n + 1;
    Some(String::from_utf8_lossy(&rest[..n]).to_string())
}

/// `md1_dbginfo`: an LZMA-alone stream holding 0x1c bytes, four strings
/// (target, platform, version, build time), a word, the symbol-area end, then
/// `(name, start, end)` records.
pub fn md1_symbols(packed: &[u8]) -> Result<Vec<(u64, String)>, String> {
    let data = super::uefifv::lzma(packed).map_err(|e| format!("md1_dbginfo: {e}"))?;
    let mut at = 0x1c;
    for _ in 0..4 {
        cstring(&data, &mut at).ok_or("md1_dbginfo: short header")?;
    }
    let end = le32(&data, at + 4).ok_or("md1_dbginfo: short header")? as usize;
    at += 8;
    let mut out = Vec::new();
    while at < end.min(data.len()) {
        let Some(name) = cstring(&data, &mut at) else { break };
        let (Some(start), Some(_stop)) = (le32(&data, at), le32(&data, at + 4)) else { break };
        at += 8;
        if !name.is_empty() {
            out.push((u64::from(start), name));
        }
    }
    Ok(out)
}

/// MediaTek modem image (`md1img.img`): a chain of `0x58881688` sections;
/// `md1rom` runs at 0x90000000, `md1_dbginfo` names its functions. A ROM that
/// starts with nanoMIPS code (`c0 60`, a load-immediate) has no processor
/// here: there is no nanoMIPS SLEIGH specification.
fn md1img(b: &[u8]) -> Result<Option<Container>, String> {
    let Some(sections) = md1_sections(b) else { return Ok(None) };
    let Some((_, rom)) = sections.iter().find(|(n, _)| n == "md1rom") else {
        return Err("md1img has no md1rom section".into());
    };
    let symbols = match sections.iter().find(|(n, _)| n == "md1_dbginfo") {
        Some((_, packed)) => md1_symbols(packed)?,
        None => Vec::new(),
    };
    let nanomips = rom.starts_with(&[0xc0, 0x60]);
    Ok(Some(Container::Flat(Flat {
        format: "MediaTek md1img",
        target: (!nanomips).then(|| "MIPS:LE:32:default".to_string()),
        base: MD1ROM_BASE,
        bytes: rom.to_vec(),
        entries: vec![MD1ROM_BASE],
        symbols,
        note: nanomips.then(|| "md1rom is nanoMIPS code, which no SLEIGH specification here decodes".to_string()),
        ..Flat::default()
    })))
}

/// The part of a flash dump that is the BIOS: a size that is not 4 KiB-aligned
/// loses its trailer to a 64 KiB boundary.
fn bios_size(b: &[u8]) -> usize {
    if b.len() & 0xfff != 0 { b.len() & !0xffff } else { b.len() }
}

/// A PC BIOS / flash dump: the reset vector, the last 16 bytes, is a far jump
/// to segment F000 or above, a near jump, or a backward short jump, after any
/// `nop`s and a `wbinvd`.
fn bios_reset_vector(b: &[u8]) -> bool {
    let size = bios_size(b);
    if size < 16 {
        return false;
    }
    let mut v = &b[size - 16..size];
    while v.first() == Some(&0x90) {
        v = &v[1..];
    }
    if v.starts_with(&[0x0f, 0x09]) {
        v = &v[2..];
    }
    match v {
        [0xea, _, _, _, seg, ..] => (0xf0..=0xfe).contains(seg),
        [0xe9, _, _, ..] => true,
        [0xeb, d, ..] => d & 0x80 != 0,
        _ => false,
    }
}

/// The real-mode view of a BIOS image: its last 128 KiB (or less) mapped below
/// 1 MiB as segments E000 and F000, entered at F000:FFF0.
fn bios(b: &[u8]) -> Option<Container> {
    if !bios_reset_vector(b) || b.len() > 64 << 20 {
        return None;
    }
    let size = bios_size(b);
    let len = size.min(0x20000);
    Some(Container::Flat(Flat {
        format: "PC BIOS image",
        target: Some("x86:LE:16:Real Mode".into()),
        base: 0x10_0000 - len as u64,
        bytes: b[size - len..size].to_vec(),
        entries: vec![0xf_fff0],
        ..Flat::default()
    }))
}

/// The protected-mode view of a BIOS image: the whole flash mapped so it ends
/// at 4 GiB, as the chipset aliases it.
pub fn bios_flash(b: &[u8]) -> Option<Flat> {
    if !bios_reset_vector(b) {
        return None;
    }
    let size = bios_size(b);
    Some(Flat {
        format: "PC BIOS image (flash alias)",
        target: None,
        base: (1u64 << 32) - size as u64,
        bytes: b[..size].to_vec(),
        ..Flat::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intel_hex_flattens_with_the_linear_base_and_entry() {
        let hex = ":020000040800F2\n:0400000001020304F2\n:04000500AABBCCDDE9\n:0400000508000101ED\n:00000001FF\n";
        let Some(Container::Flat(f)) = detect(hex.as_bytes()).unwrap() else { panic!() };
        assert_eq!(f.base, 0x0800_0000);
        assert_eq!(f.bytes, [1, 2, 3, 4, 0xff, 0xaa, 0xbb, 0xcc, 0xdd]);
        assert_eq!(f.entries, [0x0800_0101]);
    }

    #[test]
    fn uimage_names_processor_base_and_thumb_entry() {
        let mut b = vec![0u8; 64];
        b[..4].copy_from_slice(&[0x27, 0x05, 0x19, 0x56]);
        b[12..16].copy_from_slice(&8u32.to_be_bytes());
        b[16..20].copy_from_slice(&0x8000u32.to_be_bytes());
        b[20..24].copy_from_slice(&0x8001u32.to_be_bytes());
        b[29] = 2;
        b.extend([0x70, 0x47, 0, 0, 0, 0, 0, 0]);
        let Some(Container::Flat(f)) = detect(&b).unwrap() else { panic!() };
        assert_eq!((f.target.as_deref(), f.base, f.entries.as_slice(), f.thumb), (Some("ARM:LE:32:v7"), 0x8000, &[0x8000u64][..], true));
    }

    #[test]
    fn a_bios_image_maps_below_one_megabyte() {
        let mut b = vec![0u8; 0x20000];
        b[0x1fff0..0x1fff5].copy_from_slice(&[0xea, 0x5b, 0xe0, 0x00, 0xf0]);
        let Some(Container::Flat(f)) = detect(&b).unwrap() else { panic!() };
        assert_eq!((f.base, f.entries.as_slice(), f.bytes.len()), (0xe0000, &[0xffff0u64][..], 0x20000));
        assert_eq!(bios_flash(&b).unwrap().base, 0xfffe_0000);
        b[0x1fff0] = 0x90;
        b[0x1fff1] = 0x90;
        assert!(detect(&b).unwrap().is_none());
    }

    #[test]
    fn srec_reads_s3_data_and_s7_entry() {
        let text = "S30900001000DEADBEEFAE\nS70500001000EA\n";
        let Some(Container::Flat(f)) = detect(text.as_bytes()).unwrap() else { panic!() };
        assert_eq!((f.base, f.bytes.as_slice(), f.entries.as_slice()), (0x1000, &[0xde, 0xad, 0xbe, 0xef][..], &[0x1000u64][..]));
    }
}
