//! (kuna `blobs`) The image containers: each walk returns the file's exact
//! length when `d` starts with a whole, well-formed file.

use super::{be32, crc32, le16, le32, Work};

const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";

/// PNG: signature, then length/type/data/CRC chunks from `IHDR` to `IEND`.
pub(super) fn png(d: &[u8], work: &mut Work) -> Option<usize> {
    if !d.starts_with(PNG_SIGNATURE) {
        return None;
    }
    let mut p = PNG_SIGNATURE.len();
    let mut data_seen = false;
    loop {
        let len = be32(d, p)? as usize;
        let kind = d.get(p + 4..p + 8)?;
        if !kind.iter().all(u8::is_ascii_alphabetic) {
            return None;
        }
        let end = p.checked_add(len)?.checked_add(12)?;
        let body = d.get(p + 4..end - 4)?;
        work.spend(len + 12)?;
        if crc32(body) != be32(d, end - 4)? {
            return None;
        }
        match kind {
            b"IHDR" if p == PNG_SIGNATURE.len() => {
                if len != 13 || be32(body, 4)? == 0 || be32(body, 8)? == 0 {
                    return None;
                }
            }
            _ if p == PNG_SIGNATURE.len() => return None,
            b"IDAT" => data_seen = true,
            b"IEND" => return (len == 0 && data_seen).then_some(end),
            _ => {}
        }
        p = end;
    }
}

/// JPEG: marker segments from `SOI` to `EOI`, skipping each scan's
/// entropy-coded bytes.
pub(super) fn jpeg(d: &[u8], work: &mut Work) -> Option<usize> {
    if !d.starts_with(b"\xff\xd8") {
        return None;
    }
    let mut p = 2usize;
    let (mut frame, mut scan) = (false, false);
    loop {
        if *d.get(p)? != 0xff {
            return None;
        }
        while *d.get(p + 1)? == 0xff {
            p += 1;
        }
        let marker = d[p + 1];
        p += 2;
        match marker {
            0xd9 => return (frame && scan).then_some(p),
            0x00 | 0x01 | 0xd0..=0xd8 => return None,
            _ => {}
        }
        let len = u16::from_be_bytes(d.get(p..p + 2)?.try_into().ok()?) as usize;
        if len < 2 {
            return None;
        }
        work.spend(len)?;
        p = p.checked_add(len).filter(|&e| e <= d.len())?;
        match marker {
            0xc4 | 0xc8 | 0xcc => {}
            0xc0..=0xcf if len < 8 => return None,
            0xc0..=0xcf => frame = true,
            0xda if !frame => return None,
            0xda => {
                scan = true;
                p = jpeg_scan_end(d, p, work)?;
            }
            _ => {}
        }
    }
}

/// The offset of the marker that ends the entropy-coded bytes starting at `p`;
/// inside them `FF 00` and the restart markers are data.
fn jpeg_scan_end(d: &[u8], mut p: usize, work: &mut Work) -> Option<usize> {
    loop {
        let at = p + memchr::memchr(0xff, d.get(p..)?)?;
        work.spend(at - p + 1)?;
        match *d.get(at + 1)? {
            0x00 | 0xd0..=0xd7 => p = at + 2,
            0xff => p = at + 1,
            _ => return Some(at),
        }
    }
}

/// GIF: header, logical screen, then image and extension blocks to the trailer.
pub(super) fn gif(d: &[u8], work: &mut Work) -> Option<usize> {
    if !(d.starts_with(b"GIF87a") || d.starts_with(b"GIF89a")) {
        return None;
    }
    let table = |flags: u8| if flags & 0x80 != 0 { 3usize << ((flags & 7) + 1) } else { 0 };
    let mut p = 13 + table(*d.get(10)?);
    let mut images = 0usize;
    loop {
        let block = *d.get(p)?;
        p += 1;
        match block {
            0x3b => return (images > 0).then_some(p),
            0x2c => {
                let desc = d.get(p..p + 9)?;
                if desc[4..6] == [0, 0] || desc[6..8] == [0, 0] {
                    return None;
                }
                p += 9 + table(desc[8]);
                if !(2..=8).contains(d.get(p)?) {
                    return None;
                }
                p += 1;
                images += 1;
            }
            0x21 => p += 1,
            _ => return None,
        }
        loop {
            let n = *d.get(p)? as usize;
            work.spend(n + 1)?;
            p += n + 1;
            if n == 0 {
                break;
            }
        }
    }
}

/// BMP: the file header's size, checked against the DIB header's pixel array
/// for an uncompressed bitmap.
pub(super) fn bmp(d: &[u8]) -> Option<usize> {
    if !d.starts_with(b"BM") || le32(d, 6)? != 0 {
        return None;
    }
    let size = le32(d, 2)? as usize;
    let pixels = le32(d, 10)? as usize;
    let dib = le32(d, 14)? as usize;
    let (width, height, planes, bpp, compression) = match dib {
        12 => (le16(d, 18)? as u64, le16(d, 20)? as u64, le16(d, 22)?, le16(d, 24)?, 0),
        40 | 52 | 56 | 64 | 108 | 124 => (
            u64::from(le32(d, 18)?),
            u64::from((le32(d, 22)? as i32).unsigned_abs()),
            le16(d, 26)?,
            le16(d, 28)?,
            le32(d, 30)?,
        ),
        _ => return None,
    };
    if planes != 1 || !matches!(bpp, 1 | 4 | 8 | 16 | 24 | 32) {
        return None;
    }
    if width == 0 || height == 0 || width > 1 << 20 || height > 1 << 20 {
        return None;
    }
    if pixels < 14 + dib || pixels >= size || size > d.len() {
        return None;
    }
    match compression {
        0 | 3 => {
            let row = (width * bpp as u64).div_ceil(32) * 4;
            let need = pixels as u64 + row * height;
            (need <= size as u64 && size as u64 - need < 4).then_some(size)
        }
        1 if bpp == 8 => Some(size),
        2 if bpp == 4 => Some(size),
        _ => None,
    }
}

/// ICO: a directory whose images follow it back to back, each a PNG of the
/// entry's size or a DIB of the entry's dimensions.
pub(super) fn ico(d: &[u8], work: &mut Work) -> Option<usize> {
    if !d.starts_with(b"\x00\x00\x01\x00") {
        return None;
    }
    let count = le16(d, 4)?;
    if !(1..=64).contains(&count) {
        return None;
    }
    let mut end = 6 + 16 * count;
    let mut images: Vec<(usize, usize, usize, usize)> = Vec::new();
    for entry in d.get(6..end)?.chunks_exact(16) {
        if entry[3] != 0 || le16(entry, 4)? > 1 || !matches!(le16(entry, 6)?, 0 | 1 | 4 | 8 | 16 | 24 | 32) {
            return None;
        }
        let side = |b: u8| if b == 0 { 256usize } else { b as usize };
        images.push((le32(entry, 12)? as usize, le32(entry, 8)? as usize, side(entry[0]), side(entry[1])));
    }
    images.sort_unstable();
    for (offset, size, width, height) in images {
        if offset != end || size < 40 {
            return None;
        }
        end = offset.checked_add(size)?;
        let image = d.get(offset..end)?;
        if image.starts_with(PNG_SIGNATURE) {
            if png(image, work)? != size {
                return None;
            }
        } else if le32(image, 0)? != 40
            || le32(image, 4)? as usize != width
            || le32(image, 8)? as usize != 2 * height
            || le16(image, 12)? != 1
        {
            return None;
        }
    }
    Some(end)
}
