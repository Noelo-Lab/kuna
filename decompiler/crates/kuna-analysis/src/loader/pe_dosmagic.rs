//! (kuna) PE DOS-magic tolerance: load an image whose `e_magic` is not `MZ` when
//! its `e_lfanew` still points at a `PE\0\0` signature, because every header the
//! loader maps from lives behind that pointer.
//!
//! The DOS header is a 64-byte stub whose only load-bearing fields are `e_magic`
//! (`MZ`) and `e_lfanew` (the file offset of the NT headers). `object` sniffs the
//! format from the first bytes alone, so an image whose first byte was overwritten
//! -- a CTF challenge that ships `0x15 'Z'` instead of `MZ` -- is "Unknown file
//! magic" and every kuna surface exits 1 before a byte of code is mapped, although
//! `e_lfanew` still leads straight to an intact PE signature, COFF header, optional
//! header and section table.
//!
//! This module recovers those images: when the file is no format `object`
//! recognizes and `e_lfanew` points at `PE\0\0`, write `MZ` over `e_magic` in a
//! copy of the bytes and hand that copy downstream. The file on disk is never
//! touched. Any other file, including every image that already parses, is
//! returned verbatim with no parse performed.
//!
//! Like the data-directory clamp, the repair is kept even when the copy still does
//! not parse: a PE signature at `e_lfanew` settles which format the file is, so the
//! caller then reports what is *actually* unreadable in it instead of a magic
//! number that was never the whole story.

/// Return `bytes` with a corrupted DOS `e_magic` rewritten to `MZ`, plus the
/// one-line diagnostic describing the repair (`None` when nothing was repaired).
pub fn tolerate_corrupt_dos_magic(bytes: Vec<u8>) -> (Vec<u8>, Option<String>) {
    let Some((magic, lfanew)) = corrupt_magic_site(&bytes) else {
        return (bytes, None);
    };
    let mut repaired = bytes;
    repaired[..2].copy_from_slice(b"MZ");
    let note = format!(
        "PE DOS header e_magic is {magic:#06x}, not MZ (0x5a4d); loaded via e_lfanew {lfanew:#x} \
         -> PE signature (repaired in memory; the file is unchanged)"
    );
    (repaired, Some(note))
}

/// Whether [`tolerate_corrupt_dos_magic`] would repair `bytes`: a format sniff
/// that must admit the image as a PE before the repair runs.
pub fn is_pe_with_corrupt_dos_magic(bytes: &[u8]) -> bool {
    corrupt_magic_site(bytes).is_some()
}

/// The declared `e_magic` and `e_lfanew` of an image `object` cannot identify
/// whose `e_lfanew` nonetheless points at a `PE\0\0` signature clear of the
/// magic itself. `None` for everything else.
fn corrupt_magic_site(bytes: &[u8]) -> Option<(u16, usize)> {
    if bytes.len() < 0x40 || bytes.get(..2)? == b"MZ" {
        return None;
    }
    if object::FileKind::parse(bytes).is_ok() {
        return None;
    }
    let lfanew = u32::from_le_bytes(bytes.get(0x3c..0x40)?.try_into().ok()?) as usize;
    if lfanew < 2 || bytes.get(lfanew..lfanew.checked_add(4)?)? != b"PE\0\0" {
        return None;
    }
    Some((u16::from_le_bytes([bytes[0], bytes[1]]), lfanew))
}

#[cfg(test)]
mod tests;
