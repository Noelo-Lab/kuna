//! (kuna `blobs`) The archive and compressed-stream containers: each walk
//! returns the file's exact length when `d` starts with a whole, well-formed
//! file. Only gzip is decompressed (its length is not otherwise knowable), into
//! a fixed buffer that is checksummed and discarded.

use flate2::{Decompress, FlushDecompress, Status};
use memchr::memmem;

use super::{crc32, le16, le32, Work};

pub(super) const ZIP_END: &[u8] = b"PK\x05\x06";
const ZIP_LOCAL: &[u8] = b"PK\x03\x04";
const ZIP_CENTRAL: &[u8] = b"PK\x01\x02";

/// Zip, from the end-of-central-directory record at `end`: the archive's
/// `(offset, length)` when the directory sits right before the record, starts
/// where the record says, and names a local header for every entry.
pub(super) fn zip(d: &[u8], end: usize, work: &mut Work) -> Option<(usize, usize)> {
    let record = d.get(end..end.checked_add(22)?)?;
    let entries = le16(record, 10)?;
    if le16(record, 4)? != 0 || le16(record, 6)? != 0 || le16(record, 8)? != entries || entries == 0 {
        return None;
    }
    let directory = end.checked_sub(le32(record, 12)? as usize)?;
    let start = directory.checked_sub(le32(record, 16)? as usize)?;
    let stop = end.checked_add(22 + le16(record, 20)?).filter(|&e| e <= d.len())?;
    if !d.get(start..)?.starts_with(ZIP_LOCAL) {
        return None;
    }
    let mut p = directory;
    let mut first = false;
    for _ in 0..entries {
        work.spend(46)?;
        let header = d.get(p..p.checked_add(46)?)?;
        if !header.starts_with(ZIP_CENTRAL) {
            return None;
        }
        let local = le32(header, 42)? as usize;
        let at = start.checked_add(local).filter(|&a| a < directory)?;
        if !d[at..].starts_with(ZIP_LOCAL) {
            return None;
        }
        first |= local == 0;
        p += 46 + le16(header, 28)? + le16(header, 30)? + le16(header, 32)?;
    }
    (p == end && first).then_some((start, stop - start))
}

/// Gzip (RFC 1952): one member, inflated to check the trailer's CRC-32 and
/// length. The inflated bytes count against the scan's work at one eighth.
pub(super) fn gzip(d: &[u8], work: &mut Work) -> Option<usize> {
    let flags = *d.get(3)?;
    if !d.starts_with(b"\x1f\x8b\x08") || flags & 0xe0 != 0 || !matches!(*d.get(8)?, 0 | 2 | 4) {
        return None;
    }
    let mut p = 10usize;
    if flags & 4 != 0 {
        p += 2 + le16(d, p)?;
    }
    for field in [8, 16] {
        if flags & field != 0 {
            p += memchr::memchr(0, d.get(p..)?)? + 1;
        }
    }
    if flags & 2 != 0 {
        p += 2;
    }
    let stream = d.get(p..)?;
    let mut inflate = Decompress::new(false);
    let mut crc = flate2::Crc::new();
    let mut buffer = vec![0u8; 32 << 10];
    loop {
        let (read, written) = (inflate.total_in(), inflate.total_out());
        let input = stream.get(read as usize..)?;
        let status = inflate.decompress(input, &mut buffer, FlushDecompress::None).ok()?;
        let produced = (inflate.total_out() - written) as usize;
        let consumed = (inflate.total_in() - read) as usize;
        crc.update(&buffer[..produced]);
        work.spend(consumed + produced / 8 + 1)?;
        match status {
            Status::StreamEnd => break,
            _ if consumed == 0 && produced == 0 => return None,
            _ => {}
        }
    }
    let trailer = p + inflate.total_in() as usize;
    if le32(d, trailer)? != crc.sum() || le32(d, trailer + 4)? != inflate.total_out() as u32 {
        return None;
    }
    Some(trailer + 8)
}

/// Xz: the first stream footer after the header whose CRC-32 matches, whose
/// flags repeat the header's, and whose index (itself checksummed) sums to
/// exactly the bytes between header and index.
pub(super) fn xz(d: &[u8], work: &mut Work) -> Option<usize> {
    let flags = d.get(6..8)?;
    if !d.starts_with(b"\xfd7zXZ\x00") || flags[0] != 0 || flags[1] > 0x0f || crc32(flags) != le32(d, 8)? {
        return None;
    }
    let mut searched = 24usize;
    for at in memmem::find_iter(d.get(searched..)?, b"YZ") {
        let end = 24 + at + 2;
        work.spend(end - searched)?;
        searched = end;
        if end % 4 == 0 && xz_footer(d, end, flags).is_some() {
            return Some(end);
        }
    }
    work.spend(d.len() - searched)?;
    None
}

fn xz_footer(d: &[u8], end: usize, flags: &[u8]) -> Option<()> {
    let footer = d.get(end - 12..end)?;
    if &footer[8..10] != flags || crc32(&footer[4..10]) != le32(footer, 0)? {
        return None;
    }
    let size = (le32(footer, 4)? as usize).checked_add(1)?.checked_mul(4)?;
    let start = (end - 12).checked_sub(size).filter(|&s| s >= 12)?;
    let index = d.get(start..end - 12)?;
    let (body, crc) = index.split_at(size - 4);
    if body.first() != Some(&0) || crc32(body) != le32(crc, 0)? {
        return None;
    }
    let mut p = 1usize;
    let records = xz_varint(body, &mut p)?;
    let mut blocks = 0u64;
    for _ in 0..records {
        let unpadded = xz_varint(body, &mut p)?;
        xz_varint(body, &mut p)?;
        blocks = blocks.checked_add(unpadded.checked_add(3)? & !3)?;
    }
    (blocks == (start - 12) as u64).then_some(())
}

fn xz_varint(d: &[u8], p: &mut usize) -> Option<u64> {
    let mut value = 0u64;
    for shift in (0..63).step_by(7) {
        let byte = *d.get(*p)?;
        *p += 1;
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Some(value);
        }
    }
    None
}

fn le24(d: &[u8], at: usize) -> Option<u32> {
    let b = d.get(at..at.checked_add(3)?)?;
    Some(u32::from(b[0]) | u32::from(b[1]) << 8 | u32::from(b[2]) << 16)
}

const ZSTD_BLOCK_MAX: u64 = 128 << 10;

/// Zstandard (RFC 8878): one frame, its header and every block header walked
/// to the last block. A frame needs a compressed block whose literals and
/// sequences headers fit it, or raw and RLE blocks summing to the declared
/// content size.
pub(super) fn zstd(d: &[u8], work: &mut Work) -> Option<usize> {
    let descriptor = *d.get(4)?;
    if !d.starts_with(b"\x28\xb5\x2f\xfd") || descriptor & 0x08 != 0 {
        return None;
    }
    let single = descriptor & 0x20 != 0;
    let mut p = 5usize;
    let mut window = None;
    if !single {
        let byte = u64::from(*d.get(p)?);
        let base = 1u64 << (10 + (byte >> 3));
        window = Some(base + (base / 8) * (byte & 7));
        p += 1;
    }
    let dictionary = [0usize, 1, 2, 4][(descriptor & 3) as usize];
    p += dictionary;
    let content = match (descriptor >> 6, single) {
        (0, false) => None,
        (0, true) => Some(u64::from(*d.get(p)?)),
        (1, _) => Some(le16(d, p)? as u64 + 256),
        (2, _) => Some(u64::from(le32(d, p)?)),
        _ => Some(u64::from_le_bytes(d.get(p..p.checked_add(8)?)?.try_into().ok()?)),
    };
    p += match (descriptor >> 6, single) {
        (0, false) => 0,
        (0, true) => 1,
        (1, _) => 2,
        (2, _) => 4,
        _ => 8,
    };
    let limit = window.or(content)?.min(ZSTD_BLOCK_MAX);
    let (mut plain, mut compressed) = (0u64, false);
    loop {
        let header = le24(d, p)?;
        p += 3;
        let size = (header >> 3) as usize;
        if size as u64 > limit {
            return None;
        }
        let stored = match (header >> 1) & 3 {
            0 => size,
            1 => 1,
            2 => {
                zstd_compressed_block(d.get(p..p.checked_add(size)?)?, !compressed && dictionary == 0)?;
                compressed = true;
                size
            }
            _ => return None,
        };
        if (header >> 1) & 3 != 2 {
            plain += size as u64;
        }
        work.spend(stored + 3)?;
        p = p.checked_add(stored).filter(|&e| e <= d.len())?;
        if header & 1 != 0 {
            break;
        }
    }
    if descriptor & 4 != 0 {
        p = p.checked_add(4).filter(|&e| e <= d.len())?;
    }
    let sums = content.is_some_and(|c| c == plain && c > 0);
    (compressed || sums).then_some(p)
}

/// A compressed block's literals-section and sequences-section headers must
/// fit the block; the frame's first one cannot reuse an earlier table.
fn zstd_compressed_block(block: &[u8], first: bool) -> Option<()> {
    let head = *block.first()?;
    let format = (head >> 2) & 3;
    let literals = match (head & 3, format) {
        (0 | 1, 0 | 2) => 1 + if head & 3 == 0 { usize::from(head >> 3) } else { 1 },
        (0 | 1, 1) => 2 + if head & 3 == 0 { le16(block, 0)? >> 4 } else { 1 },
        (0 | 1, _) => 3 + if head & 3 == 0 { le24(block, 0)? as usize >> 4 } else { 1 },
        (3, _) if first => return None,
        (_, 0 | 1) => 3 + ((le32(block, 0)? >> 14) & 0x3ff) as usize,
        (_, 2) => 4 + (le32(block, 0)? >> 18) as usize,
        _ => 5 + ((u64::from(le32(block, 0)?) | u64::from(*block.get(4)?) << 32) >> 22 & 0x3_ffff) as usize,
    };
    let sequences = block.get(literals..)?;
    let count = *sequences.first()?;
    if count == 0 {
        return (sequences.len() == 1).then_some(());
    }
    let modes = *sequences.get(match count {
        0..=127 => 1,
        128..=254 => 2,
        _ => 3,
    })?;
    let repeats = (1..4).any(|field| (modes >> (2 * field)) & 3 == 3);
    (modes & 3 == 0 && !(first && repeats)).then_some(())
}
