//! Mach-O load tolerance: a segment or section that claims bytes past the
//! end of the file, and an image whose code is still FairPlay-encrypted.
//!
//! `object` reads a segment's file range eagerly when the loader maps it, so a
//! segment whose `fileoff + filesize` runs past the end of a truncated or
//! hand-edited file used to fail the whole load ("unreadable segment data").
//! dyld itself would refuse such a file, but a static reader can still use
//! every byte that is there. [`clamp_out_of_file_ranges`]
//! clamps each such segment's `filesize` (and each such section's `size`) to the
//! bytes the file holds, in a copy of the image, so the loader and every pass
//! see the same repaired view; the virtual sizes are untouched, so the rest of
//! each segment still reads as zero-filled memory.
//!
//! An App Store binary carries `LC_ENCRYPTION_INFO[_64]` with `cryptid != 0`
//! until it is decrypted: its `__TEXT` range is ciphertext and decompiles to
//! noise without any error. [`encryption_note`] names that state once.

const MH_MAGIC: u32 = 0xfeed_face;
const MH_MAGIC_64: u32 = 0xfeed_facf;
const LC_SEGMENT: u32 = 0x1;
const LC_SEGMENT_64: u32 = 0x19;
const LC_SYMTAB: u32 = 0x2;
const LC_ENCRYPTION_INFO: u32 = 0x21;
const LC_ENCRYPTION_INFO_64: u32 = 0x2c;
const ZEROFILL_TYPES: [u32; 3] = [0x1, 0xc, 0x12];

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn u64_at(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?))
}

fn is_64(bytes: &[u8]) -> bool {
    u32_at(bytes, 0) == Some(MH_MAGIC_64)
}

/// The load commands of a little-endian thin Mach-O, as `(cmd, offset)`.
fn commands(bytes: &[u8]) -> Vec<(u32, usize)> {
    let wide = match u32_at(bytes, 0) {
        Some(MH_MAGIC_64) => true,
        Some(MH_MAGIC) => false,
        _ => return Vec::new(),
    };
    let (Some(ncmds), Some(sizeofcmds)) = (u32_at(bytes, 16), u32_at(bytes, 20)) else {
        return Vec::new();
    };
    let start = if wide { 32 } else { 28 };
    let end = (start + sizeofcmds as usize).min(bytes.len());
    let mut out = Vec::new();
    let mut at = start;
    for _ in 0..ncmds {
        let (Some(cmd), Some(size)) = (u32_at(bytes, at), u32_at(bytes, at + 4)) else {
            break;
        };
        if size < 8 || at + size as usize > end {
            break;
        }
        out.push((cmd, at));
        at += size as usize;
    }
    out
}

/// Does a segment, section or symbol-table range run past the end of the file,
/// so that [`clamp_out_of_file_ranges`] would rewrite the header?
pub fn runs_past_the_file(bytes: &[u8]) -> bool {
    !out_of_file_edits(bytes).0.is_empty()
}

/// Clamp every segment `filesize` and section `size` that runs past the end of
/// the file to the bytes present. Returns the bytes unchanged (no copy) and no
/// note when nothing runs past the end.
pub fn clamp_out_of_file_ranges(bytes: Vec<u8>) -> (Vec<u8>, Option<String>) {
    let len = bytes.len() as u64;
    let (edits, clamped) = out_of_file_edits(&bytes);
    if edits.is_empty() {
        return (bytes, None);
    }
    let mut bytes = bytes;
    for (at, value, wide) in edits {
        if wide {
            bytes[at..at + 8].copy_from_slice(&value.to_le_bytes());
        } else {
            bytes[at..at + 4].copy_from_slice(&(value as u32).to_le_bytes());
        }
    }
    let note = if clamped.is_empty() {
        format!("Mach-O sections extend past the end of the {len}-byte file; clamped to the bytes present")
    } else {
        format!(
            "Mach-O {} extend past the end of the {len}-byte file; clamped to the bytes present, \
             the rest reads as zero",
            clamped.join(", ")
        )
    };
    (bytes, Some(note))
}

/// The header words to rewrite, `(offset, value, 64-bit)`, and the names of
/// the segments and tables that run past the end of the file.
fn out_of_file_edits(bytes: &[u8]) -> (Vec<(usize, u64, bool)>, Vec<String>) {
    let len = bytes.len() as u64;
    let mut edits: Vec<(usize, u64, bool)> = Vec::new();
    let mut clamped = Vec::new();
    for (cmd, at) in commands(bytes) {
        if cmd == LC_SYMTAB {
            let entry = if is_64(&bytes) { 16 } else { 12 };
            let (Some(symoff), Some(nsyms), Some(stroff), Some(strsize)) =
                (u32_at(&bytes, at + 8), u32_at(&bytes, at + 12), u32_at(&bytes, at + 16), u32_at(&bytes, at + 20))
            else {
                continue;
            };
            let (symoff, stroff) = (u64::from(symoff), u64::from(stroff));
            if symoff + u64::from(nsyms) * entry > len {
                edits.push((at + 8, symoff.min(len), false));
                edits.push((at + 12, len.saturating_sub(symoff) / entry, false));
                clamped.push("symbol table".into());
            }
            if stroff + u64::from(strsize) > len {
                edits.push((at + 16, stroff.min(len), false));
                edits.push((at + 20, len.saturating_sub(stroff), false));
                clamped.push("string table".into());
            }
            continue;
        }
        let wide = match cmd {
            LC_SEGMENT_64 => true,
            LC_SEGMENT => false,
            _ => continue,
        };
        let word = |off: usize| if wide { u64_at(&bytes, at + off) } else { u32_at(&bytes, at + off).map(u64::from) };
        let (fileoff_at, filesize_at, nsects_at, sect0, sect_size) =
            if wide { (40, 48, 64, 72, 80) } else { (32, 36, 48, 56, 68) };
        let (Some(fileoff), Some(filesize), Some(nsects)) =
            (word(fileoff_at), word(filesize_at), u32_at(&bytes, at + nsects_at))
        else {
            continue;
        };
        if fileoff.saturating_add(filesize) > len {
            edits.push((at + fileoff_at, fileoff.min(len), wide));
            edits.push((at + filesize_at, len.saturating_sub(fileoff), wide));
            let name = &bytes[at + 8..at + 24];
            let name = String::from_utf8_lossy(&name[..name.iter().position(|&b| b == 0).unwrap_or(16)]).into_owned();
            clamped.push(format!("segment {name}"));
        }
        for i in 0..nsects as usize {
            let s = at + sect0 + i * sect_size;
            let (size_at, offset_at, flags_at) = if wide { (40, 48, 64) } else { (36, 40, 56) };
            let size = if wide { u64_at(&bytes, s + size_at) } else { u32_at(&bytes, s + size_at).map(u64::from) };
            let (Some(size), Some(offset), Some(flags)) = (size, u32_at(&bytes, s + offset_at), u32_at(&bytes, s + flags_at))
            else {
                break;
            };
            if ZEROFILL_TYPES.contains(&(flags & 0xff)) || u64::from(offset).saturating_add(size) <= len {
                continue;
            }
            edits.push((s + size_at, len.saturating_sub(u64::from(offset)), wide));
            edits.push((s + offset_at, u64::from(offset).min(len), false));
        }
    }
    (edits, clamped)
}

/// A one-line warning when the image's code is still encrypted
/// (`LC_ENCRYPTION_INFO[_64]` with a non-zero `cryptid`).
pub fn encryption_note(bytes: &[u8]) -> Option<String> {
    commands(bytes).into_iter().find_map(|(cmd, at)| {
        if cmd != LC_ENCRYPTION_INFO && cmd != LC_ENCRYPTION_INFO_64 {
            return None;
        }
        let (off, size, id) = (u32_at(bytes, at + 8)?, u32_at(bytes, at + 12)?, u32_at(bytes, at + 16)?);
        (id != 0).then(|| {
            format!(
                "Mach-O is encrypted (LC_ENCRYPTION_INFO cryptid {id}): file bytes {off:#x}..{:#x} are ciphertext \
                 and decompile to noise; decrypt the app first",
                u64::from(off) + u64::from(size)
            )
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        std::fs::read(format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap()
    }

    #[test]
    fn a_whole_image_is_returned_untouched() {
        let bytes = fixture("macho_unwind_only_arm64");
        let (out, note) = clamp_out_of_file_ranges(bytes.clone());
        assert_eq!(out, bytes);
        assert!(note.is_none());
    }

    #[test]
    fn a_truncated_image_parses_and_maps_after_the_clamp() {
        let mut bytes = fixture("macho_unwind_only_arm64");
        bytes.truncate(0x600);
        let (out, note) = clamp_out_of_file_ranges(bytes);
        let note = note.unwrap();
        assert!(note.contains("segment __TEXT") && note.contains("symbol table"), "{note}");
        let file = object::File::parse(out.as_slice()).unwrap();
        for seg in object::Object::segments(&file) {
            object::ObjectSegment::data(&seg).unwrap();
        }
        for sec in object::Object::sections(&file) {
            object::ObjectSection::data(&sec).unwrap();
        }
    }

    #[test]
    fn cryptid_one_is_named_and_zero_is_not() {
        let mut bytes = fixture("macho_unwind_only_arm64");
        assert!(encryption_note(&bytes).is_none());
        let (_, at) = commands(&bytes).into_iter().find(|c| c.0 == LC_SEGMENT_64).unwrap();
        bytes[at..at + 4].copy_from_slice(&LC_ENCRYPTION_INFO_64.to_le_bytes());
        bytes[at + 8..at + 20].copy_from_slice(&[0, 0x40, 0, 0, 0, 0x40, 0, 0, 1, 0, 0, 0]);
        assert!(encryption_note(&bytes).unwrap().contains("cryptid 1"));
    }
}
