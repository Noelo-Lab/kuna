//! (kuna) The PE **header page**: the file-backed bytes Windows maps at
//! `ImageBase` before the first section, which kuna's load map omitted entirely.
//!
//! A PE image is mapped by the Windows loader in two parts: `SizeOfHeaders`
//! bytes of the file are copied to `ImageBase` (read-only, `PAGE_READONLY`), and
//! then each section is copied to `ImageBase + VirtualAddress`. `object`'s
//! neutral view enumerates the sections only, so everything below the first
//! section's RVA — the MZ stub, the PE signature, the COFF and optional headers,
//! the section table — was absent from the map.
//!
//! That is normally invisible, because a compiler puts no code there. A hand-
//! built or packed image can, and one does: the witness for
//! `docs/re-needs/pe-header-entry-mapped.md` declares `AddressOfEntryPoint`
//! `0x154`, which is the byte immediately after its two-entry section table, so
//! the declared entry lived in the header page. Every kuna surface answered
//! "address 0x400154 is not mapped in this input" — including `decompile
//! --define-function`, i.e. an explicit definition could not reach it either.
//!
//! The region is mapped read-only, which is what Windows does, and as DATA,
//! which keeps the code scans away from the MZ/PE bytes of every PE in the
//! corpus so function discovery cannot invent entries in a header — with one
//! exception the image itself declares. When `AddressOfEntryPoint` points into
//! the header page the region is marked CODE
//! ([`declared_entry_in_header`]): no scan guessed that, the image named the
//! address the OS jumps to, and treating it as data is what left the witness for
//! `docs/re-needs/whole-binary-decompilation-treats.md` reporting zero functions
//! from `kuna decompile-all`.

use object::pe::{ImageNtHeaders32, ImageNtHeaders64};
use object::read::pe::{ImageNtHeaders, ImageOptionalHeader, PeFile32, PeFile64};
use object::read::{Object, ObjectSection};
use object::FileKind;

use super::format::HeaderRegion;

/// The header page of a PE, or `None` for any other input.
///
/// The extent is `SizeOfHeaders` clamped twice: to the file length, because only
/// file-backed bytes can be copied, and to the first section's RVA, so a
/// malformed `SizeOfHeaders` (a packer field kuna already has to distrust — see
/// [`super::pe_datadirs`]) can never shadow a real section. Both clamps can
/// yield zero, which is `None`: no region rather than an empty one.
///
/// Pure and total: an unparsable or non-PE input yields `None`, never an error.
pub(crate) fn header_region(file: &object::File, bytes: &[u8]) -> Option<HeaderRegion> {
    let declared = declared_headers(bytes)?;
    let (base, size_of_headers) = declared;
    let mut len = size_of_headers.min(bytes.len() as u64);
    if let Some(first) = file.sections().map(|sec| sec.address()).filter(|vma| *vma >= base).min() {
        len = len.min(first - base);
    }
    if len == 0 {
        return None;
    }
    let mut region = HeaderRegion { vma: base, len: len as usize, code: false };
    region.code = declared_entry_in_header(bytes, &region).is_some();
    Some(region)
}

/// The declared entry point, when the image puts it inside its own header page.
///
/// The composed form of [`header_region`] + [`declared_entry_in_header`], for
/// callers that have the file and not the region.
pub(crate) fn header_entry(file: &object::File, bytes: &[u8]) -> Option<u64> {
    declared_entry_in_header(bytes, &header_region(file, bytes)?)
}

/// The declared entry point when the image puts it inside its own header page.
///
/// `AddressOfEntryPoint` is the one address a PE names as code without any help
/// from a section flag, and a packer that lays its stub in the slack after the
/// section table names one in here: the witness for
/// `docs/re-needs/whole-binary-decompilation-treats.md` declares `0x154`, the
/// byte immediately after its two-entry table. Mapping those bytes as data left
/// whole-binary decompilation reporting zero functions on an image whose entry
/// `kuna decompile --addr 0x400154` renders in full.
///
/// An `AddressOfEntryPoint` of `0` is the "no entry point" encoding a
/// resource-only DLL uses. `object` still reports that as `ImageBase`, which is
/// inside the header page, so it is rejected here rather than declaring the `MZ`
/// signature to be code.
pub(crate) fn declared_entry_in_header(bytes: &[u8], region: &HeaderRegion) -> Option<u64> {
    let rva = declared_entry_rva(bytes)?;
    if rva == 0 {
        return None;
    }
    let entry = region.vma.checked_add(u64::from(rva))?;
    let end = region.vma.checked_add(region.len as u64)?;
    (entry < end).then_some(entry)
}

/// `AddressOfEntryPoint` as the header spells it (an RVA), off the typed
/// optional header the neutral `object::File` view does not expose.
pub(crate) fn declared_entry_rva(bytes: &[u8]) -> Option<u32> {
    match FileKind::parse(bytes).ok()? {
        FileKind::Pe32 => {
            let nt = PeFile32::parse(bytes).ok()?.nt_headers().optional_header();
            Some(nt.address_of_entry_point())
        }
        FileKind::Pe64 => {
            let nt = PeFile64::parse(bytes).ok()?.nt_headers().optional_header();
            Some(nt.address_of_entry_point())
        }
        _ => None,
    }
}

/// `(ImageBase, SizeOfHeaders)` off the typed optional header, whose width the
/// neutral `object::File` view does not expose.
fn declared_headers(bytes: &[u8]) -> Option<(u64, u64)> {
    match FileKind::parse(bytes).ok()? {
        FileKind::Pe32 => optional::<ImageNtHeaders32>(PeFile32::parse(bytes).ok()?.nt_headers()),
        FileKind::Pe64 => optional::<ImageNtHeaders64>(PeFile64::parse(bytes).ok()?.nt_headers()),
        _ => None,
    }
}

fn optional<Pe: ImageNtHeaders>(nt: &Pe) -> Option<(u64, u64)> {
    let opt = nt.optional_header();
    Some((opt.image_base(), u64::from(opt.size_of_headers())))
}

#[cfg(test)]
mod tests;
