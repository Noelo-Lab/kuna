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
//! The region is mapped read-only rather than executable, which is both what
//! Windows does and what keeps the executable-region scans away from the MZ/PE
//! bytes of every PE in the corpus, so function discovery cannot invent entries
//! in a header. Reaching the entry there stays an explicit act — a name, an
//! address, or a `--define-function`.

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
    Some(HeaderRegion { vma: base, len: len as usize })
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
