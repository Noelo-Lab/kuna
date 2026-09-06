//! (kuna) PE data-directory tolerance: clamp a `NumberOfRvaAndSizes` that cannot
//! fit its own optional header, because the directories that are physically
//! present still describe the image.
//!
//! A PE's optional header ends with an array of `IMAGE_DATA_DIRECTORY` entries
//! whose length is declared, separately, by `NumberOfRvaAndSizes`. The two can
//! disagree: `SizeOfOptionalHeader` bounds how many entries are actually there,
//! and Windows itself trusts the bound, reading `min(NumberOfRvaAndSizes, what
//! fits)` directories and ignoring the field's excess. `object` does not — it
//! slices exactly `NumberOfRvaAndSizes` entries out of the optional header inside
//! `ImageNtHeaders::parse`, so one oversized `u32` rejects the whole file with
//! "Invalid PE number of RVA and sizes" and every kuna surface (`functions`,
//! `decompile-all`, `strings`, `disassemble`) exits 1 before a single byte of
//! code is mapped. Packers write that field over deliberately: an Invius-packed
//! image carried 1531532893 in a 224-byte optional header holding the 16 real
//! directories.
//!
//! This module recovers those images: when the declared count exceeds what the
//! optional header can hold, rewrite it to the count that fits in a copy of the
//! bytes and hand that copy downstream. The imports are then read from the real
//! table, not fabricated. A file whose count already fits is returned verbatim,
//! byte for byte, with no parse performed.
//!
//! Unlike the ELF section-table repair next door, the clamp is kept even when the
//! rewritten copy still does not parse. A count larger than its own header is
//! unambiguously wrong however the rest of the file reads, so keeping it lets the
//! caller report whatever is *actually* unreadable instead of a header count that
//! was never the whole story.

use object::read::Object;

/// Where the count lives and what it may legally be.
struct Site {
    /// File offset of the `NumberOfRvaAndSizes` `u32`.
    nrva: usize,
    /// `(SizeOfOptionalHeader - fixed part) / 8` — the number of
    /// `IMAGE_DATA_DIRECTORY` entries the optional header has room for.
    max: u32,
    /// `SizeOfOptionalHeader`, for the diagnostic.
    optsize: usize,
}

/// Size of the fixed part of the optional header, i.e. everything before the
/// data-directory array. `object` subtracts exactly these from
/// `SizeOfOptionalHeader` to bound the array.
const PE32_FIXED: usize = 96;
const PE32PLUS_FIXED: usize = 112;

/// Return `bytes` with an over-declared PE data-directory count clamped to what
/// the optional header holds, plus the one-line diagnostic describing the clamp
/// (`None` when nothing was clamped, or when the clamped copy still does not
/// parse and the caller's own error is the better message).
///
/// The check is pure header arithmetic and runs before any parse, so a
/// well-formed image costs a handful of bounds tests and is returned untouched.
pub fn tolerate_oversized_data_directories(bytes: Vec<u8>) -> (Vec<u8>, Option<String>) {
    let Some(site) = nrva_site(&bytes) else {
        return (bytes, None);
    };
    let declared = read_u32(&bytes, site.nrva);
    if declared <= site.max {
        return (bytes, None);
    }

    let mut repaired = bytes;
    write_u32(&mut repaired, site.nrva, site.max);
    let note = object::File::parse(repaired.as_slice()).ok().map(|file| {
        format!(
            "PE NumberOfRvaAndSizes is {declared}, but only {} data directories fit in a \
             {}-byte optional header; clamped to {} (entry {:#x}, {} section(s))",
            site.max,
            site.optsize,
            site.max,
            file.entry(),
            file.sections().count()
        )
    });
    (repaired, note)
}

/// The count's file offset and legal maximum for an image whose DOS/PE headers
/// are fully present and whose optional-header magic is one `object` accepts.
/// `None` for anything else (an ELF, a Mach-O, a truncated stub, a ROM image),
/// which is returned untouched.
fn nrva_site(bytes: &[u8]) -> Option<Site> {
    if bytes.len() < 0x40 || bytes.get(..2)? != b"MZ" {
        return None;
    }
    let nt = read_u32(bytes, 0x3c) as usize;
    if bytes.get(nt..nt.checked_add(4)?)? != b"PE\0\0" {
        return None;
    }
    let coff = nt + 4;
    // IMAGE_FILE_HEADER: SizeOfOptionalHeader is the 17th..18th byte of 20.
    let optsize = read_u16(bytes, coff.checked_add(16)?)? as usize;
    let opt = coff + 20;
    let fixed = match read_u16(bytes, opt)? {
        0x10b => PE32_FIXED,
        0x20b => PE32PLUS_FIXED,
        _ => return None,
    };
    // A header too small to hold even the fixed part is `object`'s "PE optional
    // header size is too small", which no clamp can repair.
    let room = optsize.checked_sub(fixed)?;
    let nrva = opt + fixed - 4;
    if bytes.len() < nrva + 4 {
        return None;
    }
    Some(Site {
        nrva,
        max: (room / 8) as u32,
        optsize,
    })
}

fn read_u16(bytes: &[u8], off: usize) -> Option<u16> {
    let slice = bytes.get(off..off.checked_add(2)?)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], off: usize) -> u32 {
    let Some(slice) = bytes.get(off..off.saturating_add(4)) else {
        return 0;
    };
    u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]])
}

fn write_u32(bytes: &mut [u8], off: usize, value: u32) {
    let Some(slice) = bytes.get_mut(off..off.saturating_add(4)) else {
        return;
    };
    slice.copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests;
