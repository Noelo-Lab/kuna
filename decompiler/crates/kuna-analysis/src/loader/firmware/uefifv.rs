//! UEFI firmware volumes: walk FV -> FFS file -> section, decompressing and
//! descending into nested volumes, and collect the PE32 and TE modules.
//!
//! Layouts are from the UEFI PI specification, volume 3. A volume is found by
//! its `_FVH` signature at offset 0x28 of an 8-byte-aligned header anywhere in
//! the input, so a whole SPI flash image, a capsule body or a bare `.fv` all
//! work. GUID-defined sections are decoded for EDK2's LZMA (and its x86 BCJ
//! variant), Tiano and CRC32 GUIDs, and standard compression sections for the
//! EFI 1.1 format (Tiano as a fallback, which some firmware stores there) or no
//! compression. A Brotli section is reported as undecoded.

use crate::upx::lzma as lzma1;

/// One executable module found in a volume.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    /// FFS file name GUID, registry form.
    pub guid: String,
    /// The user-interface section's name, when the file has one.
    pub name: Option<String>,
    /// FFS file type (0x04 PEI core, 0x05 DXE core, 0x06 PEIM, 0x07 driver, ...).
    pub file_type: u8,
    /// True for a TE image, false for PE32.
    pub te: bool,
    pub image: Vec<u8>,
}

/// What a walk found.
#[derive(Debug, Default)]
pub struct Walk {
    pub modules: Vec<Module>,
    pub volumes: usize,
    /// Sections that could not be decoded, as `guid: reason`.
    pub undecoded: Vec<String>,
}

const LZMA: [u8; 16] = guid(0xEE4E5898, 0x3914, 0x4259, [0x9D, 0x6E, 0xDC, 0x7B, 0xD7, 0x94, 0x03, 0xCF]);
const LZMA_F86: [u8; 16] = guid(0xD42AE6BD, 0x1352, 0x4BFB, [0x90, 0x9A, 0xCA, 0x72, 0xA6, 0xEA, 0xE8, 0x89]);
const TIANO: [u8; 16] = guid(0xA31280AD, 0x481E, 0x41B6, [0x95, 0xE8, 0x12, 0x7F, 0x4C, 0x98, 0x47, 0x79]);
const CRC32: [u8; 16] = guid(0xFC1BCDB0, 0x7D31, 0x49AA, [0x93, 0x6A, 0xA4, 0x60, 0x0D, 0x9D, 0xD0, 0x83]);
const MAX_DEPTH: usize = 8;
const MAX_OUTPUT: usize = 256 << 20;

const fn guid(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> [u8; 16] {
    let a = d1.to_le_bytes();
    let b = d2.to_le_bytes();
    let c = d3.to_le_bytes();
    [a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]]
}

/// The registry form `XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX` of a GUID's bytes.
pub fn guid_string(g: &[u8]) -> String {
    let d1 = u32::from_le_bytes([g[0], g[1], g[2], g[3]]);
    let d2 = u16::from_le_bytes([g[4], g[5]]);
    let d3 = u16::from_le_bytes([g[6], g[7]]);
    let tail: String = g[10..16].iter().map(|b| format!("{b:02X}")).collect();
    format!("{d1:08X}-{d2:04X}-{d3:04X}-{:02X}{:02X}-{tail}", g[8], g[9])
}

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}
fn u24_at(b: &[u8], at: usize) -> Option<usize> {
    let s = b.get(at..at + 3)?;
    Some(s[0] as usize | (s[1] as usize) << 8 | (s[2] as usize) << 16)
}
fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}
fn u64_at(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?))
}

/// Does `bytes` hold a firmware volume header anywhere?
pub fn has_volume(bytes: &[u8]) -> bool {
    (0..bytes.len().saturating_sub(0x38)).step_by(8).any(|off| volume_len(bytes, off).is_some())
}

fn volume_len(bytes: &[u8], off: usize) -> Option<usize> {
    if bytes.get(off + 0x28..off + 0x2c)? != b"_FVH" {
        return None;
    }
    let len = usize::try_from(u64_at(bytes, off + 0x20)?).ok()?;
    let hlen = u16_at(bytes, off + 0x30)? as usize;
    (hlen >= 0x38 && len >= hlen && off.checked_add(len)? <= bytes.len()).then_some(len)
}

/// Walk every volume in `bytes`.
pub fn walk(bytes: &[u8]) -> Walk {
    let mut w = Walk::default();
    scan_volumes(bytes, 0, &mut w);
    w
}

fn scan_volumes(bytes: &[u8], depth: usize, w: &mut Walk) {
    let mut off = 0;
    while off + 0x38 <= bytes.len() {
        match volume_len(bytes, off) {
            Some(len) => {
                volume(&bytes[off..off + len], depth, w);
                off += len.max(8);
            }
            None => off += 8,
        }
    }
}

fn volume(fv: &[u8], depth: usize, w: &mut Walk) {
    w.volumes += 1;
    let hlen = u16_at(fv, 0x30).unwrap_or(0x48) as usize;
    let ext = u16_at(fv, 0x34).unwrap_or(0) as usize;
    let mut off = hlen;
    if ext != 0 {
        if let Some(size) = u32_at(fv, ext + 16) {
            off = off.max(ext + size as usize);
        }
    }
    let erased = if u32_at(fv, 0x2c).unwrap_or(0) & 0x800 != 0 { 0xff } else { 0x00 };
    loop {
        off = (off + 7) & !7;
        let Some(head) = fv.get(off..off + 24) else { break };
        if head.iter().all(|&b| b == erased) {
            break;
        }
        let large = head[0x13] & 0x01 != 0;
        let (size, hsize) = if large {
            (u64_at(fv, off + 24).and_then(|s| usize::try_from(s).ok()).unwrap_or(0), 32)
        } else {
            (u24_at(head, 0x14).unwrap_or(0), 24)
        };
        if size < hsize || off + size > fv.len() {
            break;
        }
        let file = File { guid: guid_string(&head[..16]), file_type: head[0x12] };
        let body = &fv[off + hsize..off + size];
        match file.file_type {
            0xf0 => {}
            0x01 if depth < MAX_DEPTH => scan_volumes(body, depth + 1, w),
            _ => {
                let mut found = Found::default();
                sections(body, &file, depth, w, &mut found);
                for (te, image) in found.images {
                    w.modules.push(Module {
                        guid: file.guid.clone(),
                        name: found.name.clone(),
                        file_type: file.file_type,
                        te,
                        image,
                    });
                }
            }
        }
        off += size;
    }
}

struct File {
    guid: String,
    file_type: u8,
}

#[derive(Default)]
struct Found {
    name: Option<String>,
    images: Vec<(bool, Vec<u8>)>,
}

fn sections(mut data: &[u8], file: &File, depth: usize, w: &mut Walk, found: &mut Found) {
    while data.len() >= 4 {
        let mut size = u24_at(data, 0).unwrap_or(0);
        let kind = data[3];
        let mut hsize = 4;
        if size == 0xff_ffff {
            size = u32_at(data, 4).unwrap_or(0) as usize;
            hsize = 8;
        }
        if size < hsize || size > data.len() {
            return;
        }
        let body = &data[hsize..size];
        match kind {
            0x01 if depth < MAX_DEPTH => match (u32_at(body, 0), body.get(4)) {
                (Some(_), Some(0)) => sections(&body[5..], file, depth + 1, w, found),
                (Some(_), Some(1)) => match super::tiano::decompress_either(&body[5..], super::tiano::Variant::Efi) {
                    Ok(out) => sections(&out, file, depth + 1, w, found),
                    Err(reason) => w.undecoded.push(format!("{}: {reason}", file.guid)),
                },
                _ => w.undecoded.push(format!("{}: unknown compression type", file.guid)),
            },
            0x02 if depth < MAX_DEPTH => guided(body, file, depth, w, found),
            0x10 => found.images.push((false, body.to_vec())),
            0x12 => found.images.push((true, body.to_vec())),
            0x15 => {
                let units: Vec<u16> = body.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).take_while(|&u| u != 0).collect();
                found.name = String::from_utf16(&units).ok().filter(|n| !n.is_empty());
            }
            0x17 if depth < MAX_DEPTH => scan_volumes(body, depth + 1, w),
            _ => {}
        }
        let step = (size + 3) & !3;
        if step >= data.len() {
            return;
        }
        data = &data[step..];
    }
}

fn guided(body: &[u8], file: &File, depth: usize, w: &mut Walk, found: &mut Found) {
    let (Some(g), Some(data_off)) = (body.get(..16), u16_at(body, 16)) else { return };
    let Some(payload) = (data_off as usize).checked_sub(4).and_then(|o| body.get(o..)) else { return };
    let decoded = match <[u8; 16]>::try_from(g).ok() {
        Some(LZMA) => lzma(payload),
        Some(LZMA_F86) => lzma(payload).map(|mut out| {
            bcj_x86_decode(&mut out);
            out
        }),
        Some(TIANO) => super::tiano::decompress_either(payload, super::tiano::Variant::Tiano),
        Some(CRC32) => Ok(payload.to_vec()),
        _ => Err(format!("GUID-defined section {}", guid_string(g))),
    };
    match decoded {
        Ok(out) => sections(&out, file, depth + 1, w, found),
        Err(reason) => w.undecoded.push(format!("{}: {reason}", file.guid)),
    }
}

/// EDK2's LZMA: the 13-byte LZMA-alone header (properties, dictionary size,
/// 64-bit unpacked size) then the raw LZMA1 stream, decoded by the UPX
/// decoder behind its own two-byte property prefix.
pub fn lzma(data: &[u8]) -> Result<Vec<u8>, String> {
    let (Some(&props), Some(declared), Some(stream)) = (data.first(), u64_at(data, 5), data.get(13..)) else {
        return Err("short LZMA header".into());
    };
    if props >= 9 * 5 * 5 {
        return Err(format!("LZMA properties byte {props:#x} is out of range"));
    }
    let (lc, lp, pb) = (props % 9, props / 9 % 5, props / 45);
    let mut src = vec![pb, lc | lp << 4];
    src.extend_from_slice(stream);
    if declared == u64::MAX {
        return lzma1::decompress_exhaustive(&src, MAX_OUTPUT).map(|e| e.bytes).map_err(|e| format!("LZMA: {e}"));
    }
    let len = usize::try_from(declared).ok().filter(|&n| n <= MAX_OUTPUT).ok_or_else(|| format!("LZMA output {declared} bytes is too large"))?;
    lzma1::decompress(&src, len).map_err(|e| format!("LZMA: {e}"))
}

/// Undo the x86 branch-call-jump filter (LZMA SDK `x86_Convert`, decoding,
/// start address 0).
pub fn bcj_x86_decode(data: &mut [u8]) {
    let test86 = |b: u8| b.wrapping_add(1) & 0xfe == 0;
    if data.len() < 5 {
        return;
    }
    let size = data.len() - 4;
    let ip: u32 = 5;
    let mut mask: u32 = 0;
    let mut pos = 0usize;
    loop {
        let mut p = pos;
        while p < size && data[p] & 0xfe != 0xe8 {
            p += 1;
        }
        let d = p - pos;
        pos = p;
        if p >= size {
            return;
        }
        if d > 2 {
            mask = 0;
        } else {
            mask >>= d as u32;
            if mask != 0 && (mask > 4 || mask == 3 || test86(data[p + (mask >> 1) as usize + 1])) {
                mask = (mask >> 1) | 4;
                pos += 1;
                continue;
            }
        }
        if test86(data[p + 4]) {
            let mut v = u32::from_le_bytes([data[p + 1], data[p + 2], data[p + 3], data[p + 4]]);
            let cur = ip.wrapping_add(pos as u32);
            pos += 5;
            v = v.wrapping_sub(cur);
            if mask != 0 {
                let sh = (mask & 6) << 2;
                if test86((v >> sh) as u8) {
                    v ^= (0x100u32 << sh).wrapping_sub(1);
                    v = v.wrapping_sub(cur);
                }
                mask = 0;
            }
            data[p + 1] = v as u8;
            data[p + 2] = (v >> 8) as u8;
            data[p + 3] = (v >> 16) as u8;
            data[p + 4] = 0u8.wrapping_sub(((v >> 24) & 1) as u8);
        } else {
            mask = (mask >> 1) | 4;
            pos += 1;
        }
    }
}

/// The FFS file-type name.
pub fn file_type_name(t: u8) -> &'static str {
    match t {
        0x03 => "SECURITY_CORE",
        0x04 => "PEI_CORE",
        0x05 => "DXE_CORE",
        0x06 => "PEIM",
        0x07 => "DRIVER",
        0x08 => "COMBINED_PEIM_DRIVER",
        0x09 => "APPLICATION",
        0x0a => "MM",
        0x0c => "COMBINED_MM_DXE",
        0x0d => "MM_CORE",
        0x0e => "MM_STANDALONE",
        0x0f => "MM_CORE_STANDALONE",
        _ => "OTHER",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_strings_use_registry_form() {
        assert_eq!(guid_string(&LZMA), "EE4E5898-3914-4259-9D6E-DC7BD79403CF");
    }

    fn volume_with(sections: &[u8]) -> Vec<u8> {
        let mut fv = vec![0u8; 0x48];
        fv[0x28..0x2c].copy_from_slice(b"_FVH");
        fv[0x30..0x32].copy_from_slice(&0x48u16.to_le_bytes());
        let mut file = vec![0x22u8; 16];
        file.extend([0, 0, 0x07, 0]);
        file.extend(&((24 + sections.len()) as u32).to_le_bytes()[..3]);
        file.push(0xf8);
        file.extend(sections);
        fv.extend(&file);
        while fv.len() % 8 != 0 {
            fv.push(0xff);
        }
        let total = fv.len() as u64;
        fv[0x20..0x28].copy_from_slice(&total.to_le_bytes());
        fv
    }

    fn section(kind: u8, body: &[u8]) -> Vec<u8> {
        let mut s = ((4 + body.len()) as u32).to_le_bytes()[..3].to_vec();
        s.push(kind);
        s.extend(body);
        while s.len() % 4 != 0 {
            s.push(0);
        }
        s
    }

    /// `section(0x10, "MZ-inner-image") + section(0x15, "DiskIoDxe")` as an
    /// LZMA-alone stream with an unknown size and an end marker.
    const PACKED: [u8; 65] = [
        0x5d, 0x00, 0x00, 0x01, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x09, 0x00, 0x2f, 0x20,
        0xdc, 0x4c, 0x19, 0x7d, 0xe8, 0xec, 0x6a, 0x91, 0x85, 0x0a, 0x55, 0x9e, 0xd9, 0x62, 0xc0, 0xdd, 0x87, 0xae,
        0x94, 0xf8, 0x71, 0xb0, 0x2a, 0xe9, 0x38, 0x29, 0xfc, 0x32, 0x9b, 0xf7, 0xb2, 0x8c, 0xa1, 0xba, 0x19, 0x56,
        0xee, 0x0e, 0x6b, 0xcb, 0x5e, 0x9f, 0xff, 0xf8, 0x06, 0x26, 0x00,
    ];

    #[test]
    fn an_lzma_section_is_decoded_and_its_name_kept() {
        let mut guided_body = LZMA.to_vec();
        guided_body.extend(24u16.to_le_bytes());
        guided_body.extend(1u16.to_le_bytes());
        guided_body.extend(PACKED);
        let w = walk(&volume_with(&section(0x02, &guided_body)));
        assert_eq!(w.modules.len(), 1, "{:?}", w.undecoded);
        assert_eq!(w.modules[0].image, b"MZ-inner-image");
        assert_eq!(w.modules[0].name.as_deref(), Some("DiskIoDxe"));
    }

    #[test]
    fn standard_and_tiano_compressed_files_are_decoded() {
        let fv = std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/uefi_tiano_efi_compressed.fv")).unwrap();
        let plain = std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/uefi_diskio_x64.efi")).unwrap();
        let w = walk(&fv);
        assert!(w.undecoded.is_empty(), "{:?}", w.undecoded);
        assert_eq!(w.modules.len(), 2);
        assert!(w.modules.iter().all(|m| m.image == plain));
        assert_eq!(w.modules[0].name.as_deref(), Some("DiskIoDxe"));
    }

    #[test]
    fn a_pad_file_does_not_end_the_volume() {
        let pe = b"MZ-after-the-pad";
        let mut fv = vec![0u8; 0x48];
        fv[0x28..0x2c].copy_from_slice(b"_FVH");
        fv[0x30..0x32].copy_from_slice(&0x48u16.to_le_bytes());
        fv[0x2c..0x30].copy_from_slice(&0x800u32.to_le_bytes());
        let mut pad = vec![0xffu8; 16];
        pad.extend([0, 0, 0xf0, 0]);
        pad.extend(&32u32.to_le_bytes()[..3]);
        pad.push(0xf8);
        pad.extend([0xffu8; 8]);
        fv.extend(&pad);
        let mut file = vec![0x33u8; 16];
        file.extend([0, 0, 0x06, 0]);
        file.extend(&((24 + 4 + pe.len()) as u32).to_le_bytes()[..3]);
        file.push(0xf8);
        file.extend(section(0x10, pe));
        fv.extend(&file);
        while fv.len() % 8 != 0 {
            fv.push(0xff);
        }
        let total = fv.len() as u64;
        fv[0x20..0x28].copy_from_slice(&total.to_le_bytes());
        let w = walk(&fv);
        assert_eq!(w.modules.len(), 1, "{:?}", w.undecoded);
        assert_eq!(w.modules[0].file_type, 0x06);
    }

    #[test]
    fn bcj_decode_restores_relative_calls() {
        let mut code = vec![0x90, 0xe8, 0x0b, 0x00, 0x00, 0x00, 0x90, 0x90];
        bcj_x86_decode(&mut code);
        assert_eq!(code, [0x90, 0xe8, 0x05, 0x00, 0x00, 0x00, 0x90, 0x90]);
    }

    #[test]
    fn a_volume_with_one_pe_file_yields_one_module() {
        let mut fv = vec![0u8; 0x48];
        fv[0x28..0x2c].copy_from_slice(b"_FVH");
        fv[0x30..0x32].copy_from_slice(&0x48u16.to_le_bytes());
        let pe = b"MZ-not-really";
        let section_len = 4 + pe.len();
        let file_len = 24 + section_len;
        let mut file = vec![0x11u8; 16];
        file.extend([0, 0, 0x07, 0]);
        file.extend(&(file_len as u32).to_le_bytes()[..3]);
        file.push(0xf8);
        file.extend(&(section_len as u32).to_le_bytes()[..3]);
        file.push(0x10);
        file.extend(pe);
        fv.extend(&file);
        while fv.len() % 8 != 0 {
            fv.push(0xff);
        }
        let total = fv.len() as u64;
        fv[0x20..0x28].copy_from_slice(&total.to_le_bytes());
        let w = walk(&fv);
        assert_eq!(w.volumes, 1);
        assert_eq!(w.modules.len(), 1);
        assert_eq!(w.modules[0].image, pe);
        assert_eq!(w.modules[0].file_type, 0x07);
    }
}
