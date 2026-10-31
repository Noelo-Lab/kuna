//! The PE arm of the static UPX unpacker: rebuild a `win32/pe` or `win64/pe`
//! image from the single compressed block UPX leaves in the packed file.
//!
//! Modelled on `PeFile::unpack0` + `PeFile::rebuild{Imports,Relocs,Exports,
//! Resources}` in UPX's `src/pefile.cpp` and `Packer::unoptimizeReloc` in
//! `src/packer_r.cpp` (GPL-2.0-or-later). The layout is nothing like the ELF
//! one, which is why it needs its own walk:
//!
//! ```text
//!   [DOS hdr+stub][PE hdr][sections][PackHeader][ compressed block ][loader]
//!                                                        |
//!   obuf = [ image content from rvamin ][import block][reloc stream]
//!          [original PE hdr + section table][extra info][le32 -> the PE hdr]
//! ```
//!
//! Two things follow from that shape. The `PackHeader` is *not* in the tail --
//! it sits in the header padding immediately before the compressed data, so the
//! ELF tail scan never sees it. And the decompressed buffer is not the original
//! file: it is the original *memory image* with the import descriptors, the
//! import name strings, the base relocations and the resource directory
//! stripped out, plus a trailer that says how to put them back. Recovering the
//! file means replaying those rebuilds and then laying the sections out at their
//! file offsets.
//!
//! The two targets differ only in word size. `win64/pe` is the same walk over a
//! PE32+ header: eight-byte import thunks whose ordinal flag is bit 63, `DIR64`
//! relocations instead of `HIGHLOW`, and UPX's one amd64 filter (`0x49`).
//!
//! # What is refused, and why
//!
//! The trailer is a stream that each rebuild step consumes a known number of
//! bytes from, in a fixed order, and which steps run at all depends on which
//! data directories the *original* image had. A directory this module does not
//! rebuild would silently desynchronize every later read, so the walk tallies
//! its own consumption against the trailer's real length and refuses a
//! mismatch. Everything else is checked the way the ELF arm checks its stream:
//! both Adler-32s, the rebuilt directories confined to the sections that own
//! them, and a reconstructed file whose length is the original's -- short of it
//! only by the trailing relocation section UPX drops when an image keeps no
//! relocations. The targets and layouts UPX writes but this module does not
//! rebuild -- `arm/wince-pe`, `win64/pe-arm64`, the exact-layout PackHeader
//! version 14, and PackHeaders older than UPX's own unpacker reads -- are named.

use std::collections::BTreeSet;

use super::filter;
use super::{adler32, Block, PackInfo, UpxError};

/// `IMAGE_SIZEOF_SECTION_HEADER`.
const SECTION_HEADER_SIZE: usize = 40;
/// `IMAGE_IMPORT_DESCRIPTOR`: oft, timestamp, forwarder, name, iat.
const IMPORT_DESC_SIZE: usize = 20;
/// `IMAGE_RESOURCE_DATA_ENTRY`: offset, size, codepage, reserved.
const RESOURCE_DATA_ENTRY_SIZE: u32 = 16;
/// `IMAGE_EXPORT_DIRECTORY`.
const EXPORT_DIR_SIZE: usize = 40;

const DIR_EXPORT: usize = 0;
const DIR_IMPORT: usize = 1;
const DIR_RESOURCE: usize = 2;
const DIR_BASERELOC: usize = 5;
const DIR_DEBUG: usize = 6;
const DIR_BOUND_IMPORT: usize = 11;

const RT_GROUP_ICON: u32 = 14;

/// `IMAGE_FILE_RELOCS_STRIPPED` in the COFF characteristics.
const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001;

/// `IMAGE_REL_BASED_*`.
const REL_HIGH: u16 = 1;
const REL_LOW: u16 = 2;
const REL_HIGHLOW: u16 = 3;
const REL_DIR64: u16 = 10;

/// `UPX_F_WIN32_PE` / `UPX_F_WIN64_PE`.
const FORMAT_WIN32_PE: u8 = 9;
const FORMAT_WIN64_PE: u8 = 36;

/// Upper bound on descriptors in a candidate loader import table, so a table
/// that repeats a wanted name forever cannot walk the whole image.
const MAX_LOADER_DLLS: usize = 256;

fn corrupt(what: impl Into<String>) -> UpxError {
    UpxError::Corrupt(what.into())
}

fn unsupported(what: impl Into<String>) -> UpxError {
    UpxError::Unsupported(what.into())
}

fn le16(b: &[u8], off: usize) -> Result<u16, UpxError> {
    let s = b.get(off..off + 2).ok_or_else(|| corrupt(format!("read past end at {off:#x}")))?;
    Ok(u16::from_le_bytes([s[0], s[1]]))
}

fn le32(b: &[u8], off: usize) -> Result<u32, UpxError> {
    let s = b.get(off..off + 4).ok_or_else(|| corrupt(format!("read past end at {off:#x}")))?;
    Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

fn le64(b: &[u8], off: usize) -> Result<u64, UpxError> {
    let s = b.get(off..off + 8).ok_or_else(|| corrupt(format!("read past end at {off:#x}")))?;
    Ok(u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]))
}

/// PE32 or PE32+: everything the two rebuilds disagree on is the word size.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Width {
    Pe32,
    Pe32Plus,
}

impl Width {
    /// Bytes in an import thunk or a relocated pointer.
    fn word(self) -> usize {
        match self {
            Width::Pe32 => 4,
            Width::Pe32Plus => 8,
        }
    }

    /// `IMAGE_ORDINAL_FLAG32` / `IMAGE_ORDINAL_FLAG64`.
    fn ordinal_flag(self) -> u64 {
        match self {
            Width::Pe32 => 1 << 31,
            Width::Pe32Plus => 1 << 63,
        }
    }

    /// Offset of the data directory table inside the optional header.
    fn dirs_at(self) -> usize {
        match self {
            Width::Pe32 => 96,
            Width::Pe32Plus => 112,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Width::Pe32 => "PE32",
            Width::Pe32Plus => "PE32+",
        }
    }
}

/// One entry of the section table, in either the packed or the recovered header.
#[derive(Clone, Copy)]
pub(super) struct Section {
    pub vsize: u32,
    pub vaddr: u32,
    pub rawsize: u32,
    pub rawptr: u32,
}

impl Section {
    fn parse(b: &[u8], off: usize) -> Result<Self, UpxError> {
        Ok(Section {
            vsize: le32(b, off + 8)?,
            vaddr: le32(b, off + 12)?,
            rawsize: le32(b, off + 16)?,
            rawptr: le32(b, off + 20)?,
        })
    }

    /// The file offset holding image byte `rva`, if this section maps it.
    fn file_offset(&self, rva: u32) -> Option<u32> {
        let delta = rva.checked_sub(self.vaddr)?;
        if delta >= self.rawsize {
            return None;
        }
        self.rawptr.checked_add(delta)
    }
}

/// Where a header's section table sits. A file's is wherever its COFF header
/// says the optional header ends, which is where a loader looks. The copy UPX
/// stores in the trailer is `sizeof_ih` bytes -- the optional header only up to
/// the data directories in use -- with the section table right behind it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Layout {
    File,
    Trailer,
}

/// A parsed PE header -- used both for the packed image and for the original
/// one recovered out of the compressed block.
pub(super) struct PeHeader {
    /// Offset of the `PE\0\0` signature within its own buffer.
    pub at: usize,
    pub width: Width,
    pub opt_size: usize,
    /// Signature + COFF header + optional header through the last data
    /// directory in use: UPX's `sizeof_ih`.
    pub hdr_len: usize,
    /// COFF characteristics.
    pub flags: u16,
    pub code_size: u32,
    pub code_base: u32,
    pub image_base: u64,
    pub file_align: u32,
    pub sections: Vec<Section>,
    pub dirs: Vec<(u32, u32)>,
    table_at: usize,
}

impl PeHeader {
    fn dir(&self, i: usize) -> (u32, u32) {
        self.dirs.get(i).copied().unwrap_or((0, 0))
    }

    /// Offset of data directory `i` in the header's own buffer, if it has one.
    fn dir_at(&self, i: usize) -> Option<usize> {
        (i < self.dirs.len()).then(|| self.at + 24 + self.width.dirs_at() + 8 * i)
    }

    /// Parse the headers whose `PE\0\0` signature sits at `at`.
    fn parse(b: &[u8], at: usize, layout: Layout) -> Result<Self, UpxError> {
        if b.get(at..at + 4) != Some(b"PE\0\0") {
            return Err(corrupt("no PE signature where the header should be"));
        }
        let nsec = le16(b, at + 6)? as usize;
        let opt_size = le16(b, at + 20)? as usize;
        let opt = at + 24;
        let width = match le16(b, opt)? {
            0x10b => Width::Pe32,
            0x20b => Width::Pe32Plus,
            magic => return Err(unsupported(format!("PE optional header magic {magic:#x}"))),
        };
        if nsec == 0 || nsec > 96 {
            return Err(corrupt(format!("PE header declares {nsec} sections")));
        }
        let dirs_at = width.dirs_at();
        if opt_size < dirs_at {
            return Err(corrupt(format!("optional header is {opt_size} bytes")));
        }
        // UPX reads at most sixteen directories and ignores a larger count.
        let ndirs = (le32(b, opt + dirs_at - 4)? as usize).min(16);
        if opt_size < dirs_at + 8 * ndirs {
            return Err(corrupt(format!("optional header declares {ndirs} data directories")));
        }
        let mut dirs = Vec::with_capacity(ndirs);
        for i in 0..ndirs {
            let d = opt + dirs_at + 8 * i;
            dirs.push((le32(b, d)?, le32(b, d + 4)?));
        }
        let hdr_len = 24 + dirs_at + 8 * ndirs;
        let table_at = match layout {
            Layout::File => at + 24 + opt_size,
            Layout::Trailer => at + hdr_len,
        };
        let mut sections = Vec::with_capacity(nsec);
        for i in 0..nsec {
            sections.push(Section::parse(b, table_at + i * SECTION_HEADER_SIZE)?);
        }
        let image_base = match width {
            Width::Pe32 => u64::from(le32(b, opt + 28)?),
            Width::Pe32Plus => le64(b, opt + 24)?,
        };
        Ok(PeHeader {
            at,
            width,
            opt_size,
            hdr_len,
            flags: le16(b, at + 22)?,
            code_size: le32(b, opt + 4)?,
            code_base: le32(b, opt + 20)?,
            image_base,
            file_align: le32(b, opt + 36)?,
            sections,
            dirs,
            table_at,
        })
    }
}

/// The packed image: its DOS header, its PE headers, and its raw bytes.
pub(super) struct Image<'a> {
    bytes: &'a [u8],
    pe_off: usize,
    hdr: PeHeader,
}

impl<'a> Image<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Result<Self, UpxError> {
        if bytes.len() < 0x40 || &bytes[..2] != b"MZ" {
            return Err(corrupt("packed image is not a PE file"));
        }
        let pe_off = le32(bytes, 0x3c)? as usize;
        let hdr = PeHeader::parse(bytes, pe_off, Layout::File)?;
        Ok(Image { bytes, pe_off, hdr })
    }

    /// The file offset holding image address `rva`, if any section maps it.
    fn file_offset(&self, rva: u32) -> Option<usize> {
        self.hdr.sections.iter().find_map(|s| s.file_offset(rva)).map(|o| o as usize)
    }

    /// Read `len` bytes of the packed image at image address `rva`.
    fn at_rva(&self, rva: u32, len: usize) -> Option<&'a [u8]> {
        let off = self.file_offset(rva)?;
        self.bytes.get(off..off.checked_add(len)?)
    }

    /// The NUL-terminated string at image address `rva`, capped so a missing
    /// terminator cannot run the whole file into a symbol name.
    fn cstr_at_rva(&self, rva: u32) -> Result<&'a [u8], UpxError> {
        let tail = self
            .file_offset(rva)
            .and_then(|off| self.bytes.get(off..self.bytes.len().min(off + 512)))
            .ok_or_else(|| corrupt(format!("import name at {rva:#x} is outside the image")))?;
        let end = tail
            .iter()
            .position(|&c| c == 0)
            .ok_or_else(|| corrupt(format!("unterminated import name at {rva:#x}")))?;
        Ok(&tail[..end])
    }
}

/// Where UPX left the `PackHeader` in a PE: immediately before the compressed
/// block, which is the raw data of the second section (`UPX1`). UPX itself
/// reads a 1 KiB window at `isection[1].rawdataptr - 64` and scans it, so this
/// does the same.
///
/// A candidate has to pass the header's own checksum and name a PE target; when
/// more than one does, the compressed stream's Adler-32 picks between them. A
/// stream that checksums wrong does *not* disqualify the header, though -- a
/// damaged UPX file has to report as a damaged UPX file, not as "this file is
/// not one".
pub(super) fn find_pack_header(bytes: &[u8]) -> Option<usize> {
    let img = Image::new(bytes).ok()?;
    let rawptr = img.hdr.sections.get(1)?.rawptr as usize;
    let base = rawptr.saturating_sub(64);
    let end = bytes.len().min(base + 1024);
    let window = bytes.get(base..end)?;
    let magic = super::UPX_MAGIC_LE32.to_le_bytes();
    let mut fallback = None;
    for i in 0..window.len().saturating_sub(super::PACK_HEADER_SIZE) {
        if window[i..i + 4] != magic {
            continue;
        }
        let p = &window[i..];
        if p[4] < 10 || !is_pe_format(p[5]) {
            continue;
        }
        if p[super::PACK_HEADER_SIZE - 1]
            != super::pack_header_checksum(&p[..super::PACK_HEADER_SIZE - 1])
        {
            continue;
        }
        let c_len = u32::from_le_bytes([p[20], p[21], p[22], p[23]]) as usize;
        let c_adler = u32::from_le_bytes([p[12], p[13], p[14], p[15]]);
        let at = base + i;
        let data = at + super::PACK_HEADER_SIZE;
        let Some(stream) = bytes.get(data..data + c_len) else { continue };
        if adler32(1, stream) == c_adler {
            return Some(at);
        }
        fallback = Some(at);
    }
    fallback
}

pub(super) fn is_pe_format(format: u8) -> bool {
    matches!(format, 9 | 21 | 36 | 43 | 44)
}

/// The word size a PackHeader's target format implies, checked against the
/// packed image it was found in.
fn target_width(img: &Image<'_>, info: &PackInfo) -> Result<Width, UpxError> {
    let width = match info.format {
        FORMAT_WIN32_PE => Width::Pe32,
        FORMAT_WIN64_PE => Width::Pe32Plus,
        f => {
            return Err(unsupported(format!(
                "target format {f} ({}) -- only win32/pe and win64/pe (amd64) are implemented",
                super::format_name(f)
            )))
        }
    };
    if img.hdr.width != width {
        return Err(unsupported(format!(
            "PackHeader names {} but the packed image is {}",
            super::format_name(info.format),
            img.hdr.width.name()
        )));
    }
    Ok(width)
}

/// UPX's own PE unpacker reads PackHeader versions 12 through 14. Version 14 is
/// its exact-layout mode, which stores the original headers and the gaps
/// between sections verbatim instead of the rebuild trailer this module replays.
fn check_version(version: u8) -> Result<(), UpxError> {
    match version {
        12 | 13 => Ok(()),
        14 => Err(unsupported(
            "PackHeader version 14 is UPX's exact-layout PE mode, which this build does not rebuild",
        )),
        v if v < 12 => Err(unsupported(format!(
            "PackHeader version {v} predates every PE layout UPX itself can still unpack"
        ))),
        v => Err(unsupported(format!(
            "PackHeader version {v} is newer than any PE layout this build knows"
        ))),
    }
}

fn align_up(v: usize, align: u32) -> usize {
    let a = (align as usize).max(1);
    v.div_ceil(a) * a
}

/// Rebuild the original file. `info` is the `PackHeader` [`find_pack_header`]
/// located.
pub(super) fn unpack(img: &Image<'_>, info: &PackInfo) -> Result<(Vec<u8>, Vec<Block>), UpxError> {
    let width = target_width(img, info)?;
    check_version(info.version)?;

    // Data past the last packed section is the original's overlay, which UPX
    // carries through verbatim and its unpacker appends again.
    let last = img.hdr.sections.last().expect("parse rejects an empty section table");
    let packed_end = last.rawptr as usize + last.rawsize as usize;
    if packed_end > img.bytes.len() {
        return Err(corrupt("packed file is shorter than its last section"));
    }
    let tail = img.bytes.get(align_up(packed_end, img.hdr.file_align)..).unwrap_or(&[]);

    // The one compressed block, immediately after the PackHeader.
    let data = info.pack_header_offset + super::PACK_HEADER_SIZE;
    let src = img
        .bytes
        .get(data..data + info.c_len as usize)
        .ok_or_else(|| corrupt("compressed block runs past the end of the file"))?;
    let mut obuf = super::elf::method_codec(info.method)?
        .decompress(src, info.u_len as usize)
        .map_err(|e| corrupt(format!("compressed block at {data:#x}: {e}")))?;
    if adler32(1, &obuf) != info.u_adler {
        return Err(corrupt("uncompressed checksum mismatch"));
    }

    // The trailer: a le32 at the very end points at the original PE header,
    // which is followed by the original section table and the rebuild stream.
    let trailer_end = obuf
        .len()
        .checked_sub(4)
        .ok_or_else(|| corrupt("block is too short to carry a trailer"))?;
    let skip = le32(&obuf, trailer_end)? as usize;
    let mut oh = PeHeader::parse(&obuf, skip, Layout::Trailer)?;
    if oh.width != width {
        return Err(unsupported(format!(
            "PackHeader names {} but the recovered header is {}",
            info.format_name(),
            oh.width.name()
        )));
    }
    let rvamin = oh.sections[0].vaddr;
    let extra = oh.table_at + oh.sections.len() * SECTION_HEADER_SIZE;
    if extra > trailer_end {
        return Err(corrupt("recovered section table overruns the compressed block"));
    }

    // UPX filters only the code, and biases every rewritten target by the
    // code's distance from the image's first section.
    if info.filter != 0 {
        let start =
            oh.code_base.checked_sub(rvamin).ok_or_else(|| {
                corrupt(format!("code base {:#x} lies below the image", oh.code_base))
            })? as usize;
        let end = start
            .checked_add(oh.code_size as usize)
            .filter(|e| *e <= obuf.len())
            .ok_or_else(|| corrupt("filtered code runs past the recovered image"))?;
        filter::unfilter_at(info.filter, info.filter_cto, start as u32, &mut obuf[start..end])
            .map_err(|e| UpxError::Unsupported(e.to_string()))?;
    }

    let records = trailer_records(trailer_end - extra).ok_or_else(|| {
        unsupported(format!(
            "trailer has {} bytes, which no set of UPX's import, relocation and resource records adds up to -- the image uses a PE feature UPX rebuilds and kuna does not",
            trailer_end - extra
        ))
    })?;
    let mut trailer = extra;
    rebuild_imports(img, &mut obuf, &oh, rvamin, records.imports, &mut trailer)?;
    rebuild_relocs(&mut obuf, &mut oh, img.hdr.flags, rvamin, records.relocs, &mut trailer)?;
    rebuild_exports(img, &mut obuf, &oh, rvamin)?;
    rebuild_resources(img, &mut obuf, &oh, rvamin, records.resources, &mut trailer)?;
    if trailer != trailer_end {
        return Err(corrupt("rebuild records do not end where the trailer does"));
    }
    finish_header(&mut obuf, &mut oh);

    let bytes = assemble(img, &obuf, &oh, rvamin, skip, tail, info.u_file_size as usize)?;
    check_length(&bytes, &oh, info)?;
    let block = Block {
        offset: data,
        u_len: info.u_len,
        c_len: info.c_len,
        method: info.method,
        filter: info.filter,
        stored: false,
    };
    Ok((bytes, vec![block]))
}

/// Which of the trailer's optional records the packer wrote: imports (8 bytes),
/// relocations (5) and resources (2), in that order.
struct Records {
    imports: bool,
    relocs: bool,
    resources: bool,
}

/// The packer writes a record only when it had something to put in it, while
/// UPX's own unpacker predicts each one from the original's data directories --
/// and the two disagree on an image whose import directory names no DLL, or
/// whose relocations are all of a kind UPX drops, which `upx -d` then fails on.
/// No two sets of the three record lengths sum alike, so the trailer's length
/// alone says which records are there.
fn trailer_records(len: usize) -> Option<Records> {
    (0u8..8)
        .map(|m| Records { imports: m & 1 != 0, relocs: m & 2 != 0, resources: m & 4 != 0 })
        .find(|r| {
            8 * usize::from(r.imports) + 5 * usize::from(r.relocs) + 2 * usize::from(r.resources)
                == len
        })
}

/// The header fields `upx -d` rewrites once the rebuilds are done. The bound
/// import table lived in the original header's slack, which the packed file's
/// own headers took over, so it is gone rather than restorable; and the
/// checksum covers bytes the rebuilds re-laid out, so the stored one is stale.
fn finish_header(obuf: &mut [u8], oh: &mut PeHeader) {
    if let Some(at) = oh.dir_at(DIR_BOUND_IMPORT) {
        obuf[at..at + 8].fill(0);
        oh.dirs[DIR_BOUND_IMPORT] = (0, 0);
    }
    let checksum = oh.at + 24 + 64;
    obuf[checksum..checksum + 4].fill(0);
}

/// The rebuilt file must be exactly the original's length, or short of it by
/// what UPX itself drops: an image packed without relocations loses its
/// trailing `.reloc` section along with the directory naming it, and debug data
/// sitting where the overlay starts is stripped with the debug directory left
/// in place. `upx -d` writes the shorter file too.
fn check_length(bytes: &[u8], oh: &PeHeader, info: &PackInfo) -> Result<(), UpxError> {
    let want = info.u_file_size as usize;
    let dropped = oh.dir(DIR_BASERELOC) == (0, 0) || oh.dir(DIR_DEBUG) != (0, 0);
    if bytes.len() == want || (bytes.len() < want && dropped) {
        return Ok(());
    }
    Err(corrupt(format!(
        "rebuilt {} bytes, PackHeader declares an original of {}",
        bytes.len(),
        info.u_file_size
    )))
}

/// One DLL's entry in the trailer's import block.
struct ImportDll {
    /// Offset of the DLL's name from the start of the packed loader's own
    /// import descriptor table.
    name_off: u32,
    /// Image address of the thunk array this DLL's imports are written to;
    /// the trailer stores it relative to the image's first section.
    iat: u32,
    thunks: Vec<Thunk>,
}

enum Thunk {
    Name(Vec<u8>),
    Ordinal(u16),
    /// A `KERNEL32` ordinal, which UPX imports through its own loader table:
    /// the record is the offset of that loader thunk, whose stored value is the
    /// ordinal thunk itself.
    LoaderSlot(u32),
}

fn write_word(
    obuf: &mut [u8],
    rvamin: u32,
    rva: u32,
    value: u64,
    width: Width,
) -> Result<(), UpxError> {
    match width {
        Width::Pe32 => write_at(obuf, rvamin, rva, &(value as u32).to_le_bytes()),
        Width::Pe32Plus => write_at(obuf, rvamin, rva, &value.to_le_bytes()),
    }
}

fn read_word(b: &[u8], off: usize, width: Width) -> Result<u64, UpxError> {
    match width {
        Width::Pe32 => le32(b, off).map(u64::from),
        Width::Pe32Plus => le64(b, off),
    }
}

/// `PeFile::rebuildImports`: UPX strips the import descriptors, the thunk
/// arrays and the hint/name entries out of the image and replaces them with a
/// compact list in the trailer; the DLL *names* it keeps in the packed file, in
/// the loader's own import table. Put all three back.
///
/// Where the original's names were one contiguous run, UPX records where it
/// started and the names are written back there in order. Where they were
/// scattered -- every MinGW image -- it records zero, keeps each descriptor's
/// name address and each thunk's hint/name address in place, and only the
/// strings themselves have to be put back where those already point.
fn rebuild_imports(
    img: &Image<'_>,
    obuf: &mut [u8],
    oh: &PeHeader,
    rvamin: u32,
    present: bool,
    trailer: &mut usize,
) -> Result<(), UpxError> {
    let (imp_rva, imp_size) = oh.dir(DIR_IMPORT);
    let expected = imp_rva != 0 && imp_size as usize > IMPORT_DESC_SIZE;
    if present && !expected {
        return Err(corrupt("trailer carries imports for an image with no import directory"));
    }
    // A directory naming no DLL gives the packer nothing to record, and the
    // original's empty table is still in the image.
    if !present {
        return Ok(());
    }
    let idata = le32(obuf, *trailer)? as usize;
    let inamespos = le32(obuf, *trailer + 4)?;
    *trailer += 8;

    let dlls = parse_import_block(obuf, idata, rvamin)?;
    if dlls.is_empty() {
        return Ok(());
    }
    if dlls.len() * IMPORT_DESC_SIZE > imp_size as usize {
        return Err(corrupt(format!(
            "{} rebuilt import descriptors do not fit the original {imp_size}-byte directory",
            dlls.len()
        )));
    }
    let desc_base = loader_import_table(img, &dlls)?;
    let width = oh.width;

    let mut names = Vec::with_capacity(dlls.len());
    for d in &dlls {
        let at = desc_base
            .checked_add(d.name_off)
            .ok_or_else(|| corrupt("DLL name offset overflows the image"))?;
        names.push(img.cstr_at_rva(at)?);
    }
    let dll_names_len: usize = names.iter().map(|n| n.len() + 1).sum();
    let mut dll_name_rva = inamespos;
    let imported_start = inamespos.saturating_add((dll_names_len as u32).saturating_add(1) & !1);
    let mut imported = imported_start;

    let mut descs: Vec<u8> = Vec::with_capacity((dlls.len() + 1) * IMPORT_DESC_SIZE);
    for (k, (d, name)) in dlls.iter().zip(&names).enumerate() {
        let this_name = if inamespos != 0 {
            let at = dll_name_rva;
            dll_name_rva = dll_name_rva.saturating_add(name.len() as u32 + 1);
            at
        } else {
            le32(obuf, rva_off(rvamin, imp_rva)? + k * IMPORT_DESC_SIZE + 12)?
        };
        write_at(obuf, rvamin, this_name, name)?;
        write_at(obuf, rvamin, this_name.saturating_add(name.len() as u32), &[0])?;

        let mut thunk = d.iat;
        for t in &d.thunks {
            let value = match t {
                Thunk::Ordinal(ord) => u64::from(*ord) | width.ordinal_flag(),
                Thunk::LoaderSlot(off) => {
                    let slot =
                        img.at_rva(desc_base.saturating_add(*off), width.word()).ok_or_else(
                            || corrupt("KERNEL32 ordinal slot is outside the loader table"),
                        )?;
                    let v = read_word(slot, 0, width)?;
                    if v & width.ordinal_flag() == 0 {
                        return Err(corrupt("KERNEL32 ordinal slot does not hold an ordinal"));
                    }
                    v
                }
                Thunk::Name(n) if inamespos == 0 => {
                    let at = read_word(obuf, rva_off(rvamin, thunk)?, width)?;
                    let at = u32::try_from(at).map_err(|_| {
                        corrupt(format!("import thunk at {thunk:#x} is not a name"))
                    })?;
                    write_at(obuf, rvamin, at.saturating_add(2), n)?;
                    write_at(obuf, rvamin, at.saturating_add(2 + n.len() as u32), &[0])?;
                    u64::from(at)
                }
                Thunk::Name(n) => {
                    // The hint word in front of each name is aligned even, the
                    // way UPX writes it back.
                    if (imported - imported_start) & 1 != 0 {
                        imported -= 1;
                    }
                    write_at(obuf, rvamin, imported, &[0, 0])?;
                    write_at(obuf, rvamin, imported.saturating_add(2), n)?;
                    write_at(obuf, rvamin, imported.saturating_add(2 + n.len() as u32), &[0])?;
                    let at = imported;
                    imported = imported.saturating_add(2 + n.len() as u32 + 1);
                    u64::from(at)
                }
            };
            write_word(obuf, rvamin, thunk, value, width)?;
            thunk = thunk.saturating_add(width.word() as u32);
        }
        write_word(obuf, rvamin, thunk, 0, width)?;

        // `OriginalFirstThunk` is set to the same array as `FirstThunk`: the
        // original's value is not recorded anywhere, and an unbound image where
        // the two agree is exactly what a loader -- or a disassembler reading
        // the names back -- expects.
        descs.extend_from_slice(&d.iat.to_le_bytes());
        descs.extend_from_slice(&[0u8; 8]);
        descs.extend_from_slice(&this_name.to_le_bytes());
        descs.extend_from_slice(&d.iat.to_le_bytes());
    }
    descs.extend_from_slice(&[0u8; IMPORT_DESC_SIZE]);
    write_at(obuf, rvamin, imp_rva, &descs)?;
    if inamespos != 0 {
        confine_to_section(oh, inamespos, imported - inamespos, "rebuilt import names")?;
    }
    Ok(())
}

/// Offset in the recovered image of address `rva`.
fn rva_off(rvamin: u32, rva: u32) -> Result<usize, UpxError> {
    rva.checked_sub(rvamin)
        .map(|o| o as usize)
        .ok_or_else(|| corrupt(format!("rebuild targets {rva:#x}, below the image base")))
}

/// Walk the trailer's import list: per DLL a name offset and a thunk-array
/// address, then its imports as `01 name 00`, `ff` and a 2-byte ordinal, or any
/// other tag and a 4-byte loader-thunk offset, then a `00`; a zero name offset
/// ends the list.
fn parse_import_block(obuf: &[u8], mut at: usize, rvamin: u32) -> Result<Vec<ImportDll>, UpxError> {
    let mut dlls = Vec::new();
    loop {
        let name_off = le32(obuf, at)?;
        if name_off == 0 {
            return Ok(dlls);
        }
        let iat = le32(obuf, at + 4)?.wrapping_add(rvamin);
        at += 8;
        let mut thunks = Vec::new();
        loop {
            let tag = *obuf.get(at).ok_or_else(|| corrupt("import block runs past the block"))?;
            match tag {
                0 => {
                    at += 1;
                    break;
                }
                1 => {
                    let start = at + 1;
                    let end = obuf[start..]
                        .iter()
                        .position(|&c| c == 0)
                        .ok_or_else(|| corrupt("unterminated name in the import block"))?;
                    thunks.push(Thunk::Name(obuf[start..start + end].to_vec()));
                    at = start + end + 1;
                }
                0xff => {
                    thunks.push(Thunk::Ordinal(le16(obuf, at + 1)?));
                    at += 3;
                }
                _ => {
                    thunks.push(Thunk::LoaderSlot(le32(obuf, at + 1)?));
                    at += 5;
                }
            }
        }
        dlls.push(ImportDll { name_off, iat, thunks });
    }
}

/// Does a candidate loader table name exactly the DLLs the trailer asks for?
///
/// A set comparison. The loader table holds one descriptor per distinct DLL,
/// while the original image may import one DLL across several descriptors, so
/// the two lists differ in length whenever a binary does that. The packer's own
/// `KERNEL32.DLL` descriptor is not part of `table`; see [`loader_import_table`].
fn table_names_dlls(table: &[u32], base: u32, wanted: &BTreeSet<u32>) -> bool {
    !table.is_empty()
        && table.iter().map(|n| n.checked_sub(base)).collect::<Option<BTreeSet<u32>>>().as_ref()
            == Some(wanted)
}

/// Find the packed loader's own import descriptor table, which is what the
/// trailer's DLL-name offsets are relative to.
///
/// UPX reaches it through the packed image's import data directory. That is
/// exactly the field a repacker retargets when it bolts a decoy import table
/// onto a UPX image, so the directory is a hint here and not the answer: the
/// table is the one naming exactly the DLLs the trailer's offsets point at,
/// and a candidate is accepted only if it is unique.
///
/// Matched as a SET, not position-by-position. The loader table holds one
/// descriptor per distinct DLL, while the original image may import the same
/// DLL from several descriptors, so the two lists differ in length whenever a
/// binary does that and a positional walk can never agree. It also holds the
/// stub's own `KERNEL32.DLL` descriptor (`LoadLibraryA`, `GetProcAddress`, ...)
/// even when the original imports nothing from it, so one descriptor naming
/// that DLL is allowed beyond the set the trailer asks for.
///
/// The scan reads only section bytes the file holds, and gives up after two
/// descriptor reads per four file bytes. An honest image, whose sections do not
/// overlap in the file, rejects nearly every candidate on its first descriptor,
/// so it needs about half that; a region of descriptors that all name wanted
/// DLLs would otherwise cost a full table walk per candidate.
fn loader_import_table(img: &Image<'_>, dlls: &[ImportDll]) -> Result<u32, UpxError> {
    let wanted: BTreeSet<u32> = dlls.iter().map(|d| d.name_off).collect();
    if wanted.len() > MAX_LOADER_DLLS {
        return Err(unsupported(format!(
            "trailer names {} DLLs; this build follows loader tables of at most {MAX_LOADER_DLLS}",
            wanted.len()
        )));
    }
    let mut reads = img.bytes.len() / 2 + img.hdr.sections.len() + 2 * (MAX_LOADER_DLLS + 1);
    let mut agrees = |base: u32| -> Result<bool, UpxError> {
        let mut names = Vec::new();
        let mut stub_kernel32 = false;
        for k in 0..=MAX_LOADER_DLLS {
            reads = reads
                .checked_sub(1)
                .ok_or_else(|| corrupt("packed image holds too many candidate loader tables to scan"))?;
            let Some(at) = base.checked_add((k * IMPORT_DESC_SIZE) as u32) else {
                return Ok(false);
            };
            let Some(desc) = img.at_rva(at, IMPORT_DESC_SIZE) else {
                return Ok(false);
            };
            if desc.iter().all(|&b| b == 0) {
                return Ok(table_names_dlls(&names, base, &wanted));
            }
            let name = u32::from_le_bytes([desc[12], desc[13], desc[14], desc[15]]);
            // Nearly every position a scan tries fails on its first descriptor;
            // the set comparison alone would walk each one on to a terminator.
            if !name.checked_sub(base).is_some_and(|o| wanted.contains(&o)) {
                let kernel32 =
                    img.cstr_at_rva(name).is_ok_and(|n| n.eq_ignore_ascii_case(b"KERNEL32.DLL"));
                if stub_kernel32 || !kernel32 {
                    return Ok(false);
                }
                stub_kernel32 = true;
                continue;
            }
            names.push(name);
        }
        Ok(false)
    };
    let declared = img.hdr.dir(DIR_IMPORT).0;
    if declared != 0 && agrees(declared)? {
        return Ok(declared);
    }
    let mut found = None;
    for s in &img.hdr.sections {
        let present = img.bytes.len().saturating_sub(s.rawptr as usize);
        let end = (u64::from(s.vaddr) + (s.rawsize as usize).min(present) as u64).min(1 << 32);
        let mut rva = u64::from(s.vaddr);
        while rva < end {
            let at = rva as u32;
            if agrees(at)? {
                if found.is_some_and(|f| f != at) {
                    return Err(corrupt("packed image has more than one loader import table"));
                }
                found = Some(at);
            }
            rva += 4;
        }
    }
    found.ok_or_else(|| corrupt("cannot find the loader import table the DLL names live in"))
}

/// `PeFile::rebuildRelocs`. UPX strips an EXE's relocations by default and
/// marks both headers `RELOCS_STRIPPED`; its unpacker then zeroes the recovered
/// header's relocation directory instead of rebuilding anything, and so does
/// this. An eight-byte directory is an empty block, written back in place of
/// the zeros UPX packed. Every other image -- every DLL's, every ASLR image's --
/// had its relocations rebased to zero, byte-swapped in place, and recorded as
/// a delta-coded position stream; the trailer names where that stream starts.
/// Undo both and lay the positions out as a fresh `.reloc` table, the way
/// `PeFile::Reloc::finish` does. Entries of a kind UPX does not keep -- anything
/// but `DIR64` in a PE32+, anything but `HIGHLOW`/`HIGH`/`LOW` in a PE32 -- were
/// dropped when it packed, so `upx -d` does not have them either.
fn rebuild_relocs(
    obuf: &mut [u8],
    oh: &mut PeHeader,
    packed_flags: u16,
    rvamin: u32,
    present: bool,
    trailer: &mut usize,
) -> Result<(), UpxError> {
    if packed_flags & IMAGE_FILE_RELOCS_STRIPPED != 0 {
        oh.flags |= IMAGE_FILE_RELOCS_STRIPPED;
        obuf[oh.at + 22..oh.at + 24].copy_from_slice(&oh.flags.to_le_bytes());
        if let Some(at) = oh.dir_at(DIR_BASERELOC) {
            oh.dirs[DIR_BASERELOC] = (0, 0);
            obuf[at..at + 8].fill(0);
        }
    }
    let (rva, size) = oh.dir(DIR_BASERELOC);
    let expected = rva != 0 && size != 0 && size != 8 && oh.flags & IMAGE_FILE_RELOCS_STRIPPED == 0;
    if present && !expected {
        return Err(corrupt("trailer carries relocations for an image that keeps none"));
    }
    if rva == 0 || size == 0 || oh.flags & IMAGE_FILE_RELOCS_STRIPPED != 0 {
        return Ok(());
    }
    if size == 8 {
        return write_at(obuf, rvamin, rva, &[0, 0, 0, 0, 8, 0, 0, 0]);
    }
    if !present {
        // The packer kept none of them -- a PE32+ image's `HIGHLOW` entries are
        // dropped, not rebased -- and cleared the table: an empty rebuild.
        oh.dirs[DIR_BASERELOC].1 = 0;
        if let Some(d) = oh.dir_at(DIR_BASERELOC) {
            obuf[d + 4..d + 8].fill(0);
        }
        return Ok(());
    }
    let crelocs = le32(obuf, *trailer)? as usize;
    let big = *obuf
        .get(*trailer + 4)
        .ok_or_else(|| corrupt("trailer ends inside its relocation record"))?;
    *trailer += 5;

    let width = oh.width;
    let (mut relocs, mut at) = unoptimize_relocs(obuf, crelocs, width, oh.image_base, rvamin)?;
    // 16-bit relocations (PE32 only) follow the stream as zero-terminated
    // position lists: `LOW` first when the record says so, then `HIGH`.
    if big & 6 != 0 {
        let first = if big & 4 != 0 { REL_LOW } else { REL_HIGH };
        let lists: &[u16] = if big & 6 == 6 { &[first, REL_HIGH] } else { &[first] };
        for &ty in lists {
            loop {
                let pos = le32(obuf, at)?;
                at += 4;
                if pos == 0 {
                    break;
                }
                relocs.push((pos.wrapping_add(rvamin), ty));
            }
        }
    }
    let table = build_reloc_table(relocs)?;
    write_at(obuf, rvamin, rva, &table)?;
    confine_to_section(oh, rva, table.len() as u32, "rebuilt relocations")?;
    let len = table.len() as u32;
    oh.dirs[DIR_BASERELOC].1 = len;
    if let Some(d) = oh.dir_at(DIR_BASERELOC) {
        obuf[d + 4..d + 8].copy_from_slice(&len.to_le_bytes());
    }
    Ok(())
}

/// `Packer::unoptimizeReloc` plus the rebase `rebuildRelocs` applies: decode the
/// delta stream at `crelocs`, and at each position it names turn the
/// big-endian, zero-based pointer the packer left back into the original
/// little-endian one. Returns the relocations and the offset past the stream.
fn unoptimize_relocs(
    obuf: &mut [u8],
    crelocs: usize,
    width: Width,
    image_base: u64,
    rvamin: u32,
) -> Result<(Vec<(u32, u16)>, usize), UpxError> {
    let ty = match width {
        Width::Pe32 => REL_HIGHLOW,
        Width::Pe32Plus => REL_DIR64,
    };
    let word = width.word();
    let mut relocs = Vec::new();
    let mut at = crelocs;
    let mut pc = u32::MAX - 3;
    loop {
        let b = *obuf.get(at).ok_or_else(|| corrupt("relocation stream runs past the block"))?;
        if b == 0 {
            return Ok((relocs, at + 1));
        }
        let delta = if b < 0xf0 {
            at += 1;
            u32::from(b)
        } else {
            let d = u32::from(b & 0x0f) * 0x10000 + u32::from(le16(obuf, at + 1)?);
            at += 3;
            if d == 0 {
                let d = le32(obuf, at)?;
                at += 4;
                d
            } else {
                d
            }
        };
        if (delta as i32) < 4 {
            return Err(corrupt("relocation stream has overlapping fixups"));
        }
        pc = pc.wrapping_add(delta);
        let p = pc as usize;
        if p.checked_add(word).is_none_or(|e| e > crelocs) {
            return Err(corrupt(format!("relocation at {pc:#x} lies outside the image")));
        }
        let slot = &mut obuf[p..p + word];
        slot.reverse();
        match width {
            Width::Pe32 => {
                let v = u32::from_le_bytes([slot[0], slot[1], slot[2], slot[3]])
                    .wrapping_add(image_base as u32)
                    .wrapping_add(rvamin);
                slot.copy_from_slice(&v.to_le_bytes());
            }
            Width::Pe32Plus => {
                let mut w = [0u8; 8];
                w.copy_from_slice(slot);
                let v =
                    u64::from_le_bytes(w).wrapping_add(image_base).wrapping_add(u64::from(rvamin));
                slot.copy_from_slice(&v.to_le_bytes());
            }
        }
        relocs.push((rvamin.wrapping_add(pc), ty));
    }
}

/// `PeFile::Reloc::finish`: one `IMAGE_BASE_RELOCATION` block per 4 KiB page,
/// entries sorted, each block padded to four bytes with `ABSOLUTE` entries.
fn build_reloc_table(mut relocs: Vec<(u32, u16)>) -> Result<Vec<u8>, UpxError> {
    relocs.sort_unstable();
    if relocs.windows(2).any(|w| w[0].0 == w[1].0) {
        return Err(corrupt("relocation stream names one address twice"));
    }
    fn finish(table: &mut Vec<u8>, start: usize) {
        while (table.len() - start) & 3 != 0 {
            table.push(0);
        }
        let sob = (table.len() - start) as u32;
        table[start + 4..start + 8].copy_from_slice(&sob.to_le_bytes());
    }
    let mut table = Vec::with_capacity(relocs.len() * 2 + 64);
    let mut block: Option<(usize, u32)> = None;
    for (pos, ty) in relocs {
        if block.is_none_or(|(_, page)| pos - page >= 0x1000) {
            if let Some((start, _)) = block {
                finish(&mut table, start);
            }
            let page = pos & !0xfff;
            block = Some((table.len(), page));
            table.extend_from_slice(&page.to_le_bytes());
            table.extend_from_slice(&8u32.to_le_bytes());
        }
        table.extend_from_slice(&((ty << 12) | (pos & 0xfff) as u16).to_le_bytes());
    }
    if let Some((start, _)) = block {
        finish(&mut table, start);
    }
    Ok(table)
}

/// `PeFile::rebuildExports`. A DLL's export directory -- or an EXE's, packed
/// with `--compress-exports=0` -- is moved out of the compressed image into the
/// packed file, re-laid out compactly by `PeFile::Export::build`. Rebuild that
/// same compact layout at the original directory's address. An export
/// directory left in place needs nothing.
fn rebuild_exports(
    img: &Image<'_>,
    obuf: &mut [u8],
    oh: &PeHeader,
    rvamin: u32,
) -> Result<(), UpxError> {
    let (orva, osize) = oh.dir(DIR_EXPORT);
    let (irva, isize) = img.hdr.dir(DIR_EXPORT);
    if osize == 0 || orva == irva || isize == 0 {
        return Ok(());
    }
    let moved = img
        .at_rva(irva, isize as usize)
        .ok_or_else(|| corrupt("packed export directory is outside the image"))?;
    let room = obuf.len().saturating_sub(rva_off(rvamin, orva)?);
    let built = rebuild_export_dir(moved, irva, orva, room)?;
    write_at(obuf, rvamin, orva, &built)?;
    confine_to_section(oh, orva, built.len() as u32, "rebuilt export directory")
}

/// `Export::convert` over the directory at `base` (whose bytes are `dir`), then
/// `Export::build` at `newbase`: header, address table, name pointers,
/// ordinals, the DLL name, the export names, then the forwarder strings. Every
/// name pointer may name the same long string, so the layout is sized as the
/// strings are read and refused once it outgrows the `room` left in the image.
fn rebuild_export_dir(
    dir: &[u8],
    base: u32,
    newbase: u32,
    room: usize,
) -> Result<Vec<u8>, UpxError> {
    let local = |rva: u32, len: usize| -> Result<usize, UpxError> {
        let off = rva.checked_sub(base).map(|o| o as usize);
        off.filter(|o| o.checked_add(len).is_some_and(|e| e <= dir.len()))
            .ok_or_else(|| corrupt(format!("export table at {rva:#x} runs outside its directory")))
    };
    let cstr = |rva: u32| -> Result<&[u8], UpxError> {
        let off = local(rva, 1)?;
        let n = dir[off..]
            .iter()
            .position(|&c| c == 0)
            .ok_or_else(|| corrupt(format!("unterminated export string at {rva:#x}")))?;
        Ok(&dir[off..off + n])
    };
    if dir.len() < EXPORT_DIR_SIZE {
        return Err(corrupt("packed export directory is shorter than its header"));
    }
    let mut edir = dir[..EXPORT_DIR_SIZE].to_vec();
    let ename = cstr(le32(&edir, 12)?)?;
    let nfuncs = le32(&edir, 20)? as usize;
    let nnames = le32(&edir, 24)? as usize;
    let funcs_at = local(le32(&edir, 28)?, nfuncs.saturating_mul(4))?;
    let names_at = local(le32(&edir, 32)?, nnames.saturating_mul(4))?;
    let ords_at = local(le32(&edir, 36)?, nnames.saturating_mul(2))?;
    let mut size = EXPORT_DIR_SIZE + 4 * nfuncs + 6 * nnames;
    let mut grow = |s: &[u8]| -> Result<(), UpxError> {
        size += s.len() + 1;
        if size > room {
            return Err(corrupt("rebuilt export directory outgrows the image"));
        }
        Ok(())
    };
    grow(ename)?;
    let mut funcs: Vec<u32> =
        (0..nfuncs).map(|i| le32(dir, funcs_at + 4 * i)).collect::<Result<_, _>>()?;
    let mut names = Vec::with_capacity(nnames);
    for i in 0..nnames {
        let n = cstr(le32(dir, names_at + 4 * i)?)?;
        grow(n)?;
        names.push(n);
    }
    let end = base.saturating_add(dir.len() as u32);
    let mut forwarders: Vec<Option<&[u8]>> = Vec::with_capacity(nfuncs);
    for &f in &funcs {
        let fw = if f >= base && f < end { Some(cstr(f)?) } else { None };
        if let Some(s) = fw {
            grow(s)?;
        }
        forwarders.push(fw);
    }

    let functionp = EXPORT_DIR_SIZE;
    let namep = functionp + 4 * nfuncs;
    let ordinalp = namep + 4 * nnames;
    let enamep = ordinalp + 2 * nnames;
    let mut out = edir.clone();
    out.resize(enamep, 0);
    out[ordinalp..enamep].copy_from_slice(&dir[ords_at..ords_at + 2 * nnames]);
    out.extend_from_slice(ename);
    out.push(0);
    let rva_of = |off: usize| newbase.wrapping_add(off as u32);
    for (i, n) in names.iter().enumerate() {
        let at = rva_of(out.len());
        out[namep + 4 * i..namep + 4 * i + 4].copy_from_slice(&at.to_le_bytes());
        out.extend_from_slice(n);
        out.push(0);
    }
    for (f, fw) in funcs.iter_mut().zip(&forwarders) {
        if let Some(s) = fw {
            *f = rva_of(out.len());
            out.extend_from_slice(s);
            out.push(0);
        }
    }
    for (i, f) in funcs.iter().enumerate() {
        out[functionp + 4 * i..functionp + 4 * i + 4].copy_from_slice(&f.to_le_bytes());
    }
    edir[12..16].copy_from_slice(&rva_of(enamep).to_le_bytes());
    edir[28..32].copy_from_slice(&rva_of(functionp).to_le_bytes());
    edir[32..36].copy_from_slice(&rva_of(namep).to_le_bytes());
    edir[36..40].copy_from_slice(&rva_of(ordinalp).to_le_bytes());
    out[..EXPORT_DIR_SIZE].copy_from_slice(&edir);
    out.resize(align_up(out.len(), 4), 0);
    Ok(out)
}

/// `PeFile::rebuildResources`: UPX keeps the resource *directory* uncompressed
/// in the packed file and moves the leaves it compressed into the image. Copy
/// the directory back to where the original had it, repointing every leaf UPX
/// moved at the address recorded in front of its data. A directory UPX could
/// not clear -- one whose pieces were not contiguous -- is still in the image,
/// and is left there the way UPX's unpacker leaves it. Unlike the import and
/// relocation records, the packer writes this one exactly when it gives the
/// packed header a resource directory, so a trailer without it for an image
/// whose headers both have one is damaged, not a variant.
fn rebuild_resources(
    img: &Image<'_>,
    obuf: &mut [u8],
    oh: &PeHeader,
    rvamin: u32,
    present: bool,
    trailer: &mut usize,
) -> Result<(), UpxError> {
    let expected = oh.dir(DIR_RESOURCE).1 != 0 && img.hdr.dir(DIR_RESOURCE).1 != 0;
    if present && !expected {
        return Err(corrupt("trailer carries resources for an image with no resource directory"));
    }
    if !present && expected {
        return Err(corrupt("trailer has no resource record for an image with resources"));
    }
    if !present {
        return Ok(());
    }
    let res_rva = oh.dir(DIR_RESOURCE).0;
    let icondir_count = le16(obuf, *trailer)?;
    *trailer += 2;

    let packed_rva = img.hdr.dir(DIR_RESOURCE).0;
    let packed_size = img.hdr.dir(DIR_RESOURCE).1;
    let tree_src = img
        .at_rva(packed_rva, packed_size as usize)
        .ok_or_else(|| corrupt("packed resource directory is outside the image"))?;
    let mut tree = tree_src.to_vec();

    let mut walk = ResourceWalk::default();
    walk_resource_dir(&tree, 0, 0, 0, &mut walk)?;
    confine_to_section(oh, res_rva, walk.extent, "rebuilt resource directory")?;

    let mut icondir_count = icondir_count;
    let mut moved = 0usize;
    for (leaf, ty) in walk.leaves {
        let at = leaf as usize;
        let (data_rva, size) = (le32(&tree, at)?, le32(&tree, at + 4)?);
        if data_rva <= packed_rva {
            // A leaf UPX compressed: its data is already in the image, at the
            // address the directory still names.
            continue;
        }
        let orig = le32(
            data_rva
                .checked_sub(4)
                .and_then(|r| img.at_rva(r, 4))
                .ok_or_else(|| corrupt("moved resource has no back-pointer"))?,
            0,
        )?;
        moved = moved.saturating_add(size as usize);
        if moved > img.bytes.len() {
            return Err(corrupt("moved resources add up to more than the packed file"));
        }
        let body = img
            .at_rva(data_rva, size as usize)
            .ok_or_else(|| corrupt("moved resource runs past the packed image"))?;
        tree[at..at + 4].copy_from_slice(&orig.to_le_bytes());
        write_at(obuf, rvamin, orig, body)?;
        if icondir_count != 0 && ty == RT_GROUP_ICON {
            write_at(obuf, rvamin, orig.saturating_add(4), &icondir_count.to_le_bytes())?;
            icondir_count = 0;
        }
    }
    let entries = le32(obuf, rva_off(rvamin, res_rva)? + 12)?;
    if entries == 0 {
        let dir = tree
            .get(..walk.extent as usize)
            .ok_or_else(|| corrupt("resource directory runs past its packed copy"))?;
        write_at(obuf, rvamin, res_rva, dir)?;
    }
    Ok(())
}

/// What a walk of a resource tree reached: one past the last byte any
/// directory, entry or data entry occupies, and each data entry's offset with
/// the top-level type it sits under, in walk order.
#[derive(Default)]
struct ResourceWalk {
    extent: u32,
    leaves: Vec<(u32, u32)>,
    seen: BTreeSet<u32>,
}

/// Depth-first walk of an `IMAGE_RESOURCE_DIRECTORY` tree. The depth cap is
/// UPX's own three levels (type / name / language) plus slack; the tree is
/// attacker-controlled, so a cycle must terminate the walk, and a directory or
/// data entry reached twice -- which no linker writes, and which would let a
/// small tree fan out into an exponential walk -- is refused.
fn walk_resource_dir(
    tree: &[u8],
    at: u32,
    depth: u32,
    ty: u32,
    walk: &mut ResourceWalk,
) -> Result<(), UpxError> {
    if depth > 4 {
        return Err(corrupt("resource directory nests deeper than a PE allows"));
    }
    if !walk.seen.insert(at | 0x8000_0000) {
        return Err(corrupt("resource directory reaches one subdirectory twice"));
    }
    let named = u32::from(le16(tree, at as usize + 12)?);
    let ids = u32::from(le16(tree, at as usize + 14)?);
    let count = named + ids;
    let entries = at.saturating_add(16);
    walk.extent = walk.extent.max(entries.saturating_add(8 * count));
    for i in 0..count {
        let e = (entries + 8 * i) as usize;
        let child = le32(tree, e + 4)?;
        if child & 0x8000_0000 != 0 {
            let ty = if depth == 0 { le32(tree, e)? & 0x7fff_ffff } else { ty };
            walk_resource_dir(tree, child & 0x7fff_ffff, depth + 1, ty, walk)?;
        } else {
            if !walk.seen.insert(child) {
                return Err(corrupt("resource directory reaches one data entry twice"));
            }
            walk.extent = walk.extent.max(child.saturating_add(RESOURCE_DATA_ENTRY_SIZE));
            walk.leaves.push((child, ty));
        }
    }
    Ok(())
}

/// Write into the recovered image at address `rva`, refusing anything that
/// lands outside the decompressed block.
fn write_at(obuf: &mut [u8], rvamin: u32, rva: u32, data: &[u8]) -> Result<(), UpxError> {
    let off = rva
        .checked_sub(rvamin)
        .ok_or_else(|| corrupt(format!("rebuild targets {rva:#x}, below the image base")))?
        as usize;
    let end = off
        .checked_add(data.len())
        .filter(|e| *e <= obuf.len())
        .ok_or_else(|| corrupt(format!("rebuild at {rva:#x} runs past the recovered image")))?;
    obuf[off..end].copy_from_slice(data);
    Ok(())
}

/// A rebuilt directory has to stay inside the section the original header says
/// owns it; a rebuild that spills into the next one would look plausible and
/// disassemble wrong.
fn confine_to_section(oh: &PeHeader, rva: u32, len: u32, what: &str) -> Result<(), UpxError> {
    let end = rva.checked_add(len).ok_or_else(|| corrupt(format!("{what} overflows")))?;
    let ok = oh
        .sections
        .iter()
        .any(|s| rva >= s.vaddr && end <= s.vaddr.saturating_add(s.vsize.max(s.rawsize)));
    if ok {
        Ok(())
    } else {
        Err(corrupt(format!("{what} at {rva:#x}..{end:#x} spans no single section")))
    }
}

/// The original's overlay, out of the bytes the packed file carries past its
/// last section -- normally all of them. With nothing dropped from the image,
/// the original's length says how long the overlay was, and two things can
/// stand beside it. UPX 3.9x writes its stub's relocation block twice; the
/// second copy spills past the last section's declared end, and that spill,
/// padded with zeros to the packed file's alignment, is the first `spill` bytes
/// of the tail. Bytes appended to the packed file afterwards (a signature, a
/// payload) come after the overlay. So the overlay starts past the spill and
/// stops at its own length. When the rebuilt sections fall short of the
/// original (see [`check_length`]) the whole tail is the overlay.
fn overlay_of(tail: &[u8], sections_end: usize, original_len: usize, spill: usize) -> &[u8] {
    match original_len.checked_sub(sections_end) {
        Some(want) if want <= tail.len() => {
            let lead = if tail.len() - want >= spill { spill } else { 0 };
            &tail[lead..lead + want]
        }
        _ => tail,
    }
}

/// Is `front` the end of the packed stub's relocation table followed by zeros?
/// The stub's table is a few fixups and the padding is under one file-alignment
/// unit, so anything larger is not that spill.
fn is_stub_reloc_spill(front: &[u8], relocs: &[u8]) -> bool {
    if front.len() > 0x1_0000 || relocs.len() > 0x1000 {
        return false;
    }
    let nonzero = front.iter().rposition(|&b| b != 0).map_or(0, |i| i + 1);
    (nonzero..=relocs.len().min(front.len())).any(|k| front[..k] == relocs[relocs.len() - k..])
}

/// Lay the recovered image back out as a file: the packed file's own DOS header
/// and stub (UPX preserves it), the recovered PE header and section table, then
/// each section's raw data read out of the image at its own address, then the
/// overlay the packed file carried.
///
/// The DOS header is the one part of the original file UPX keeps rather than
/// packs, so a packed image that shrank it below the 64 bytes a DOS header needs
/// has thrown the original's away: `e_lfanew` no longer exists to be rewritten,
/// and the recovered PE header would sit at an offset nothing points at. That is
/// refused here rather than written out, and the assembled file is re-read at the
/// end to prove it is a PE a loader can follow. The layout is sized before
/// anything is written, so no header field can make it longer than the original
/// the PackHeader declares.
fn assemble(
    img: &Image<'_>,
    obuf: &[u8],
    oh: &PeHeader,
    rvamin: u32,
    skip: usize,
    tail: &[u8],
    original_len: usize,
) -> Result<Vec<u8>, UpxError> {
    if img.pe_off < 0x40 {
        return Err(unsupported(format!(
            "packed image keeps only {} bytes of DOS header, so the original's is gone",
            img.pe_off
        )));
    }
    let header = obuf
        .get(skip..skip + oh.hdr_len)
        .ok_or_else(|| corrupt("recovered PE header runs past the block"))?;
    let table = oh.sections.len() * SECTION_HEADER_SIZE;
    let section_table = obuf
        .get(oh.table_at..oh.table_at + table)
        .ok_or_else(|| corrupt("recovered section table runs past the block"))?;
    // The section table goes where the COFF header says the optional header
    // ends, which is where a loader will look for it.
    let table_at = img.pe_off + 24 + oh.opt_size;
    let mut end = table_at + table;
    let mut raw = Vec::with_capacity(oh.sections.len());
    for s in &oh.sections {
        if s.rawsize == 0 {
            continue;
        }
        let (ptr, size) = (s.rawptr as usize, s.rawsize as usize);
        let src = (s.vaddr.checked_sub(rvamin).ok_or_else(|| {
            corrupt(format!("section at {:#x} lies below the image base", s.vaddr))
        })?) as usize;
        // The image content must not reach into the trailer: that region is
        // UPX's own bookkeeping, never a section's bytes.
        if src + size > skip {
            return Err(corrupt(format!(
                "section at {:#x} overlaps the packer's trailer",
                s.vaddr
            )));
        }
        if ptr.checked_add(size).is_none() || ptr + size > 512 * 1024 * 1024 {
            return Err(corrupt(format!("section raw range at {ptr:#x} is not credible")));
        }
        end = end.max(ptr + size);
        raw.push((ptr, &obuf[src..src + size]));
    }
    let (overlay_at, overlay) = if tail.is_empty() {
        (end, &[][..])
    } else {
        let at = align_up(end, oh.file_align);
        let (rva, size) = img.hdr.dir(DIR_BASERELOC);
        let stub_relocs = img.at_rva(rva, size as usize).unwrap_or(&[]);
        let spill = tail
            .get(..img.hdr.file_align as usize)
            .filter(|front| is_stub_reloc_spill(front, stub_relocs))
            .map_or(0, <[u8]>::len);
        (at, overlay_of(tail, at, original_len, spill))
    };
    let total = overlay_at + overlay.len();
    if total > original_len {
        return Err(corrupt(format!(
            "rebuilt {total} bytes, PackHeader declares an original of {original_len}"
        )));
    }
    let mut out = vec![0u8; total];
    out[..img.pe_off].copy_from_slice(&img.bytes[..img.pe_off]);
    out[img.pe_off..img.pe_off + header.len()].copy_from_slice(header);
    out[table_at..table_at + table].copy_from_slice(section_table);
    for (ptr, data) in raw {
        out[ptr..ptr + data.len()].copy_from_slice(data);
    }
    out[overlay_at..].copy_from_slice(overlay);
    // A section whose raw data starts inside the headers would have overwritten
    // what was just written there, so read the result back the way a loader
    // would rather than trusting the layout that produced it.
    if le32(&out, 0x3c)? as usize != img.pe_off
        || out.get(img.pe_off..img.pe_off + 4) != Some(b"PE\0\0")
    {
        return Err(corrupt("rebuilt file's headers were overwritten by a section's raw data"));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dlls(name_offs: &[u32]) -> Vec<ImportDll> {
        name_offs
            .iter()
            .map(|&name_off| ImportDll { name_off, iat: 0, thunks: Vec::new() })
            .collect()
    }

    fn offsets(dlls: &[ImportDll]) -> BTreeSet<u32> {
        dlls.iter().map(|d| d.name_off).collect()
    }

    /// The regression: a 32-bit MSVC crackme importing seven DLLs across
    /// thirteen descriptors (KERNEL32 four times, three others twice). Its
    /// loader table holds one descriptor per distinct DLL, so the two lists are
    /// 7 and 13 long and a position-by-position walk could never agree -- the
    /// unpack failed with "cannot find the loader import table".
    #[test]
    fn a_dll_imported_by_several_descriptors_still_matches_its_loader_table() {
        let base = 0x73a0c;
        let table: Vec<u32> =
            [224, 237, 250, 263, 273, 286, 297].iter().map(|o| base + o).collect();
        let repeated = dlls(&[224, 224, 224, 224, 237, 237, 250, 263, 273, 273, 286, 286, 297]);
        assert!(table_names_dlls(&table, base, &offsets(&repeated)));
    }

    #[test]
    fn one_descriptor_per_dll_still_matches() {
        let base = 0x1000;
        let table: Vec<u32> = [16, 32, 48].iter().map(|o| base + o).collect();
        assert!(table_names_dlls(&table, base, &offsets(&dlls(&[16, 32, 48]))));
    }

    /// Set equality both ways: a table naming a DLL the trailer never asks for
    /// is the wrong table, and so is one missing a DLL the trailer wants.
    #[test]
    fn a_table_whose_names_differ_is_rejected() {
        let base = 0x1000;
        let table: Vec<u32> = [16, 32, 48].iter().map(|o| base + o).collect();
        assert!(!table_names_dlls(&table, base, &offsets(&dlls(&[16, 32]))));
        assert!(!table_names_dlls(&table, base, &offsets(&dlls(&[16, 32, 48, 64]))));
        assert!(!table_names_dlls(&[], base, &offsets(&dlls(&[16]))));
    }

    /// A minimal PE header at `skip` inside a block, laid out the way UPX stores
    /// it in the trailer: one section, sixteen data directories, `flags` as the
    /// COFF characteristics and `reloc` in entry 5.
    fn block_with_header(skip: usize, flags: u16, reloc: (u32, u32), width: Width) -> Vec<u8> {
        let opt_size = width.dirs_at() + 128;
        let mut b = vec![0u8; skip + 24 + opt_size + SECTION_HEADER_SIZE + 16];
        b[skip..skip + 4].copy_from_slice(b"PE\0\0");
        b[skip + 6..skip + 8].copy_from_slice(&1u16.to_le_bytes());
        b[skip + 20..skip + 22].copy_from_slice(&(opt_size as u16).to_le_bytes());
        b[skip + 22..skip + 24].copy_from_slice(&flags.to_le_bytes());
        let opt = skip + 24;
        let magic: u16 = if width == Width::Pe32 { 0x10b } else { 0x20b };
        b[opt..opt + 2].copy_from_slice(&magic.to_le_bytes());
        let base = 0x1_4000_0000u64;
        if width == Width::Pe32 {
            b[opt + 28..opt + 32].copy_from_slice(&0x40_0000u32.to_le_bytes());
        } else {
            b[opt + 24..opt + 32].copy_from_slice(&base.to_le_bytes());
        }
        let dirs = opt + width.dirs_at();
        b[dirs - 4..dirs].copy_from_slice(&16u32.to_le_bytes());
        let d = dirs + 8 * DIR_BASERELOC;
        b[d..d + 4].copy_from_slice(&reloc.0.to_le_bytes());
        b[d + 4..d + 8].copy_from_slice(&reloc.1.to_le_bytes());
        // One section covering the first 0x4000 bytes of the image.
        let t = opt + opt_size;
        b[t + 8..t + 12].copy_from_slice(&0x4000u32.to_le_bytes());
        b[t + 12..t + 16].copy_from_slice(&0x1000u32.to_le_bytes());
        b[t + 16..t + 20].copy_from_slice(&0x4000u32.to_le_bytes());
        b
    }

    fn relocs(
        flags: u16,
        packed_flags: u16,
        reloc: (u32, u32),
    ) -> (Result<(), UpxError>, PeHeader, Vec<u8>) {
        let skip = 0x100;
        let mut b = block_with_header(skip, flags, reloc, Width::Pe32);
        let mut oh = PeHeader::parse(&b, skip, Layout::Trailer).unwrap();
        let mut trailer = oh.table_at + SECTION_HEADER_SIZE;
        let r = rebuild_relocs(&mut b, &mut oh, packed_flags, 0x1000, false, &mut trailer);
        (r, oh, b)
    }

    /// The witness shape: an EXE whose relocations UPX stripped. The stored
    /// header still names the directory and only the packed header carries the
    /// flag; UPX's own unpacker zeroes the entry rather than rebuilding, so the
    /// recovered header has to come out the way `upx -d` writes it.
    #[test]
    fn stripped_relocations_are_zeroed_not_refused() {
        let (r, oh, b) = relocs(0, IMAGE_FILE_RELOCS_STRIPPED, (0x3b000, 0x5cc));
        r.expect("a stripped image unpacks");
        assert_eq!(oh.dir(DIR_BASERELOC), (0, 0));
        assert_ne!(oh.flags & IMAGE_FILE_RELOCS_STRIPPED, 0);
        let written = PeHeader::parse(&b, 0x100, Layout::Trailer).unwrap();
        assert_eq!(written.dir(DIR_BASERELOC), (0, 0));
        assert_ne!(written.flags & IMAGE_FILE_RELOCS_STRIPPED, 0);
    }

    /// Relocations the packer kept are rebased to zero, byte-swapped in place
    /// and recorded as a delta stream. Rebuilding them restores both pointers
    /// and lays the positions out one block per page, the way `upx -d` does --
    /// for PE32+ that is `DIR64` entries over eight-byte pointers.
    #[test]
    fn kept_pe32plus_relocations_are_rebuilt() {
        let rvamin = 0x1000u32;
        let base = 0x1_4000_0000u64;
        let skip = 0x3000;
        let mut b = block_with_header(skip, 0, (0x2800, 0x40), Width::Pe32Plus);
        let targets = [(0x10usize, base + 0x1234), (0x1008usize, base + 0x3ff0)];
        for (at, target) in targets {
            let zero_based = target - base - u64::from(rvamin);
            b[at..at + 8].copy_from_slice(&zero_based.to_be_bytes());
        }
        // Positions 0x10 then 0x1008: one short delta, one three-byte one.
        let crelocs = 0x2000;
        b[crelocs..crelocs + 5].copy_from_slice(&[0x14, 0xf0, 0xf8, 0x0f, 0x00]);
        let mut oh = PeHeader::parse(&b, skip, Layout::Trailer).unwrap();
        let record = oh.table_at + SECTION_HEADER_SIZE;
        b[record..record + 4].copy_from_slice(&(crelocs as u32).to_le_bytes());
        b[record + 4] = 0;
        let mut trailer = record;
        rebuild_relocs(&mut b, &mut oh, 0, rvamin, true, &mut trailer)
            .expect("kept relocations rebuild");
        assert_eq!(trailer, record + 5, "the relocation record is five trailer bytes");
        for (at, target) in targets {
            assert_eq!(u64::from_le_bytes(b[at..at + 8].try_into().unwrap()), target);
        }
        let table = &b[0x1800..0x1818];
        assert_eq!(
            table,
            &[
                0x00, 0x10, 0, 0, 12, 0, 0, 0, 0x10, 0xa0, 0, 0, //
                0x00, 0x20, 0, 0, 12, 0, 0, 0, 0x08, 0xa0, 0, 0,
            ]
        );
        assert_eq!(oh.dir(DIR_BASERELOC), (0x2800, 24));
        let written = PeHeader::parse(&b, skip, Layout::Trailer).unwrap();
        assert_eq!(written.dir(DIR_BASERELOC), (0x2800, 24));
    }

    /// A PE32 image's `HIGHLOW` relocations take the same path, and the 16-bit
    /// `LOW`/`HIGH` lists UPX appends after the stream come back as their own
    /// entry types.
    #[test]
    fn kept_pe32_relocations_and_their_16_bit_lists_are_rebuilt() {
        let rvamin = 0x1000u32;
        let skip = 0x3000;
        let mut b = block_with_header(skip, 0, (0x2800, 0x40), Width::Pe32);
        b[0x20..0x24].copy_from_slice(&0x0000_0abcu32.to_be_bytes());
        let crelocs = 0x2000;
        // One HIGHLOW at 0x20, then a LOW list (0x40) and a HIGH list (0x44).
        let mut stream = vec![0x24, 0x00];
        for v in [0x40u32, 0, 0x44, 0] {
            stream.extend_from_slice(&v.to_le_bytes());
        }
        b[crelocs..crelocs + stream.len()].copy_from_slice(&stream);
        let mut oh = PeHeader::parse(&b, skip, Layout::Trailer).unwrap();
        let record = oh.table_at + SECTION_HEADER_SIZE;
        b[record..record + 4].copy_from_slice(&(crelocs as u32).to_le_bytes());
        b[record + 4] = 6;
        let mut trailer = record;
        rebuild_relocs(&mut b, &mut oh, 0, rvamin, true, &mut trailer)
            .expect("kept relocations rebuild");
        assert_eq!(
            u32::from_le_bytes(b[0x20..0x24].try_into().unwrap()),
            0x40_0000 + 0x1000 + 0xabc
        );
        let entries: Vec<u16> =
            (0..3).map(|i| u16::from_le_bytes([b[0x1808 + 2 * i], b[0x1809 + 2 * i]])).collect();
        assert_eq!(entries, [0x3020, 0x2040, 0x1044]);
        assert_eq!(oh.dir(DIR_BASERELOC).1, 16);
    }

    #[test]
    fn an_image_without_relocations_needs_nothing() {
        let (r, oh, _) = relocs(0, 0, (0, 0));
        r.unwrap();
        assert_eq!(oh.flags, 0);
    }

    /// The three optional records are 8, 5 and 2 bytes, and no two sets of them
    /// sum alike: every trailer length names exactly one set, and the lengths
    /// no set reaches are refused.
    #[test]
    fn the_trailer_length_alone_names_its_records() {
        let mut seen = BTreeSet::new();
        for len in 0..=20 {
            if let Some(r) = trailer_records(len) {
                assert_eq!(
                    8 * usize::from(r.imports)
                        + 5 * usize::from(r.relocs)
                        + 2 * usize::from(r.resources),
                    len
                );
                assert!(seen.insert((r.imports, r.relocs, r.resources)), "two lengths for one set");
            }
        }
        assert_eq!(seen.len(), 8);
        assert!(
            trailer_records(1).is_none()
                && trailer_records(3).is_none()
                && trailer_records(16).is_none()
        );
        let r = trailer_records(10).unwrap();
        assert!(r.imports && !r.relocs && r.resources);
    }

    /// The trailer's import list: a name, a 2-byte ordinal, and a `KERNEL32`
    /// ordinal recorded as the offset of the packer's own loader thunk -- which
    /// must not be read as the ordinal itself.
    #[test]
    fn the_import_block_distinguishes_loader_slots_from_ordinals() {
        let mut b = Vec::new();
        b.extend_from_slice(&0x30u32.to_le_bytes());
        b.extend_from_slice(&0x2000u32.to_le_bytes());
        b.extend_from_slice(b"\x01Sleep\x00");
        b.extend_from_slice(&[0xff, 0x07, 0x00]);
        b.extend_from_slice(&[0xfe, 0x40, 0x00, 0x00, 0x00]);
        b.push(0);
        b.extend_from_slice(&0u32.to_le_bytes());
        let dlls = parse_import_block(&b, 0, 0x1000).unwrap();
        assert_eq!(dlls.len(), 1);
        assert_eq!((dlls[0].name_off, dlls[0].iat), (0x30, 0x3000));
        assert!(matches!(&dlls[0].thunks[0], Thunk::Name(n) if n == b"Sleep"));
        assert!(matches!(dlls[0].thunks[1], Thunk::Ordinal(7)));
        assert!(matches!(dlls[0].thunks[2], Thunk::LoaderSlot(0x40)));
    }

    /// A one-section PE32 whose section (at RVA 0x1000) holds `body`.
    fn tiny_pe(body: &[u8]) -> Vec<u8> {
        let mut b = vec![0u8; 0x200];
        b[..2].copy_from_slice(b"MZ");
        b[0x3c..0x40].copy_from_slice(&0x40u32.to_le_bytes());
        let pe = 0x40;
        b[pe..pe + 4].copy_from_slice(b"PE\0\0");
        b[pe + 6..pe + 8].copy_from_slice(&1u16.to_le_bytes());
        b[pe + 20..pe + 22].copy_from_slice(&224u16.to_le_bytes());
        let opt = pe + 24;
        b[opt..opt + 2].copy_from_slice(&0x10bu16.to_le_bytes());
        b[opt + 92..opt + 96].copy_from_slice(&16u32.to_le_bytes());
        let t = opt + 224;
        let size = body.len() as u32;
        b[t + 8..t + 12].copy_from_slice(&size.to_le_bytes());
        b[t + 12..t + 16].copy_from_slice(&0x1000u32.to_le_bytes());
        b[t + 16..t + 20].copy_from_slice(&size.to_le_bytes());
        b[t + 20..t + 24].copy_from_slice(&0x200u32.to_le_bytes());
        b.extend_from_slice(body);
        b
    }

    /// The packer's loader table carries its own `KERNEL32.DLL` descriptor even
    /// when the original imports nothing from it -- a 32-bit image importing only
    /// `arr.dll` failed with "cannot find the loader import table". One such
    /// extra is allowed; any other DLL the trailer does not ask for is not.
    #[test]
    fn the_loader_table_may_carry_the_stubs_own_kernel32() {
        let table = |extra: &[u8]| {
            let mut body = vec![0u8; 0x100];
            body[12..16].copy_from_slice(&0x1080u32.to_le_bytes());
            body[32..36].copy_from_slice(&0x1090u32.to_le_bytes());
            body[0x80..0x88].copy_from_slice(b"arr.dll\0");
            body[0x90..0x90 + extra.len()].copy_from_slice(extra);
            tiny_pe(&body)
        };
        let wanted = vec![ImportDll { name_off: 0x80, iat: 0, thunks: Vec::new() }];
        let bytes = table(b"KERNEL32.DLL\0");
        let img = Image::new(&bytes).unwrap();
        assert_eq!(loader_import_table(&img, &wanted).unwrap(), 0x1000);
        let bytes = table(b"USER32.dll\0");
        let img = Image::new(&bytes).unwrap();
        assert!(loader_import_table(&img, &wanted).is_err());
    }

    /// `Export::build`'s compact layout, rebuilt at the original address: the
    /// header's tables are repointed, the names and the DLL name re-laid out,
    /// an ordinal-only export kept, and a forwarder's string moved with it.
    #[test]
    fn a_moved_export_directory_is_rebuilt_at_its_original_address() {
        let base = 0x9000u32;
        let mut d = vec![0u8; 40];
        let put =
            |d: &mut Vec<u8>, at: usize, v: u32| d[at..at + 4].copy_from_slice(&v.to_le_bytes());
        put(&mut d, 16, 1);
        put(&mut d, 20, 3);
        put(&mut d, 24, 2);
        // functions, name pointers, ordinals, DLL name, names, forwarder
        let funcs = d.len();
        d.resize(funcs + 12, 0);
        let names = d.len();
        d.resize(names + 8, 0);
        let ords = d.len();
        d.extend_from_slice(&[0, 0, 2, 0]);
        let ename = d.len();
        d.extend_from_slice(b"lib.dll\0");
        let n0 = d.len();
        d.extend_from_slice(b"alpha\0");
        let n1 = d.len();
        d.extend_from_slice(b"fwd\0");
        let fw = d.len();
        d.extend_from_slice(b"K32.Sleep\0");
        put(&mut d, 12, base + ename as u32);
        put(&mut d, 28, base + funcs as u32);
        put(&mut d, 32, base + names as u32);
        put(&mut d, 36, base + ords as u32);
        put(&mut d, funcs, 0x1100);
        put(&mut d, funcs + 4, 0x1200);
        put(&mut d, funcs + 8, base + fw as u32);
        put(&mut d, names, base + n0 as u32);
        put(&mut d, names + 4, base + n1 as u32);

        let newbase = 0x3000u32;
        let out = rebuild_export_dir(&d, base, newbase, 0x1000).unwrap();
        let rd = |at: usize| u32::from_le_bytes(out[at..at + 4].try_into().unwrap());
        let at = |rva: u32| (rva - newbase) as usize;
        let cstr = |rva: u32| {
            let o = at(rva);
            let n = out[o..].iter().position(|&c| c == 0).unwrap();
            String::from_utf8_lossy(&out[o..o + n]).into_owned()
        };
        assert_eq!(out.len() % 4, 0);
        assert_eq!((rd(28), rd(32), rd(36)), (newbase + 40, newbase + 52, newbase + 60));
        assert_eq!(cstr(rd(12)), "lib.dll");
        assert_eq!((rd(40), rd(44)), (0x1100, 0x1200), "code addresses are not moved");
        assert_eq!(cstr(rd(48)), "K32.Sleep", "the forwarder string moves with the table");
        assert_eq!((cstr(rd(52)), cstr(rd(56))), ("alpha".to_owned(), "fwd".to_owned()));
        assert_eq!(&out[60..64], &[0, 0, 2, 0]);
    }

    /// Every name pointer of a packed export directory may name the same long
    /// string; the rebuild stops once its layout outgrows the room it has
    /// instead of copying the string once per pointer.
    #[test]
    fn export_names_that_share_one_long_string_are_bounded_by_the_image() {
        let base = 0x9000u32;
        let n = 4000usize;
        let mut d = vec![0u8; 40 + 4 * n];
        let text = d.len() as u32;
        d.extend_from_slice(&[b'A'; 8000]);
        d.push(0);
        d[12..16].copy_from_slice(&(base + text).to_le_bytes());
        d[24..28].copy_from_slice(&(n as u32).to_le_bytes());
        for f in [28, 32, 36] {
            d[f..f + 4].copy_from_slice(&(base + 40).to_le_bytes());
        }
        for i in 0..n {
            d[40 + 4 * i..44 + 4 * i].copy_from_slice(&(base + text).to_le_bytes());
        }
        let err = rebuild_export_dir(&d, base, 0x3000, 0x10000).expect_err("32 MB of names");
        assert!(format!("{err}").contains("outgrows"), "{err}");
    }

    /// Five directories of a few hundred entries that all name the next one
    /// would be walked hundreds-to-the-fourth times; a shared subdirectory is
    /// refused the first time it is reached again.
    #[test]
    fn a_resource_tree_that_shares_a_subdirectory_is_refused() {
        let (n, levels) = (300usize, 5usize);
        let dsz = 16 + 8 * n;
        let mut tree = vec![0u8; dsz * levels];
        for k in 0..levels - 1 {
            let o = k * dsz;
            tree[o + 14..o + 16].copy_from_slice(&(n as u16).to_le_bytes());
            for i in 0..n {
                let e = o + 16 + 8 * i;
                tree[e..e + 4].copy_from_slice(&(i as u32 + 1).to_le_bytes());
                let child = 0x8000_0000 | ((k + 1) * dsz) as u32;
                tree[e + 4..e + 8].copy_from_slice(&child.to_le_bytes());
            }
        }
        let err = walk_resource_dir(&tree, 0, 0, 0, &mut ResourceWalk::default())
            .expect_err("a shared subdirectory");
        assert!(format!("{err}").contains("subdirectory twice"), "{err}");
    }

    /// A section mapped at the top of the address space must not wrap the
    /// loader-table scan back to address zero, and a section whose raw data
    /// lies past the end of the file names nothing.
    #[test]
    fn a_hostile_packed_section_table_cannot_wrap_or_overrun_the_scan() {
        let wanted = vec![ImportDll { name_off: 0x80, iat: 0, thunks: Vec::new() }];
        let mut bytes = tiny_pe(&[0x11u8; 0x40]);
        let t = 0x40 + 24 + 224;
        bytes[t + 12..t + 16].copy_from_slice(&0xffff_fff0u32.to_le_bytes());
        bytes[t + 16..t + 20].copy_from_slice(&0xfu32.to_le_bytes());
        let img = Image::new(&bytes).unwrap();
        assert!(loader_import_table(&img, &wanted).is_err());

        let mut bytes = tiny_pe(&[0x11u8; 0x40]);
        bytes[t + 20..t + 24].copy_from_slice(&0x10_0000u32.to_le_bytes());
        let img = Image::new(&bytes).unwrap();
        assert!(img.cstr_at_rva(0x1000).is_err());
        assert!(loader_import_table(&img, &wanted).is_err());
    }

    /// A huge `FileAlignment` in the recovered header used to pad the overlay
    /// out to gigabytes before the length check refused the file; the layout
    /// is now sized, and refused, before anything is allocated.
    #[test]
    fn a_huge_file_alignment_is_refused_before_it_is_allocated() {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/upx_packed_pe_x86_64.exe");
        let mut bytes = std::fs::read(p).unwrap();
        let info = super::super::detect(&bytes).expect("fixture is UPX-packed");
        let data = info.pack_header_offset + super::super::PACK_HEADER_SIZE;
        let obuf = super::super::elf::method_codec(info.method)
            .unwrap()
            .decompress(&bytes[data..data + info.c_len as usize], info.u_len as usize)
            .unwrap();
        bytes.extend_from_slice(b"overlay!");
        let img = Image::new(&bytes).unwrap();
        let skip = le32(&obuf, obuf.len() - 4).unwrap() as usize;
        let mut oh = PeHeader::parse(&obuf, skip, Layout::Trailer).unwrap();
        let rvamin = oh.sections[0].vaddr;
        let tail = &bytes[bytes.len() - 8..];
        let sane = assemble(&img, &obuf, &oh, rvamin, skip, tail, 3072 + 8).unwrap();
        assert_eq!(&sane[3072..], b"overlay!");
        oh.file_align = 0xf000_0000;
        let err = assemble(&img, &obuf, &oh, rvamin, skip, tail, 3072 + 8)
            .expect_err("a 4 GB alignment is not credible");
        assert!(format!("{err}").contains("rebuilt 4026531848 bytes"), "{err}");
    }

    /// The overlay is cut out of the packed file's tail by the original's
    /// length. UPX 3.9x's spilled second copy of the stub relocations (and its
    /// zero padding) comes off the front; bytes appended after packing -- a
    /// signature, a payload -- come off the end instead of shifting the overlay.
    #[test]
    fn the_overlay_is_cut_from_whichever_end_holds_something_else() {
        let relocs = [0, 0x10, 1, 0, 12, 0, 0, 0, 0x60, 0x39, 0, 0];
        let mut front = vec![0x60, 0x39, 0, 0];
        front.resize(512, 0);
        assert!(is_stub_reloc_spill(&front, &relocs));
        assert!(!is_stub_reloc_spill(b"COFF symbols", &relocs));
        assert!(!is_stub_reloc_spill(&[0x39, 0x60, 0, 0], &relocs));

        let overlay = b"COFF symbols and strings";
        let later = b"certificate appended later";
        let (end, len) = (0x1000, 0x1000 + overlay.len());
        let spilled = [&front[..], overlay].concat();
        assert_eq!(overlay_of(&spilled, end, len, 512), overlay);
        let both = [&front[..], overlay, later].concat();
        assert_eq!(overlay_of(&both, end, len, 512), overlay);
        let appended = [&overlay[..], later].concat();
        assert_eq!(overlay_of(&appended, end, len, 0), overlay);
        assert_eq!(overlay_of(overlay, end, len, 0), overlay);
        assert_eq!(overlay_of(overlay, end, len + 100, 0), overlay);
        assert_eq!(overlay_of(&appended, end, end, 0), b"");
    }

    fn put(b: &mut [u8], at: usize, v: u32) {
        b[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }

    /// Where [`tiny_pe`] writes its one section header and its data directories.
    const TINY_SECTION: usize = 0x40 + 24 + 224;
    const TINY_DIRS: usize = 0x40 + 24 + 96;

    /// Replay the resource rebuild of a packed image whose one section, at
    /// `packed_rva`, is the resource directory `tree`, into a recovered image
    /// whose `.rsrc` starts at 0x1000.
    fn rebuild_tree(tree: &[u8], packed_rva: u32) -> Result<(), UpxError> {
        let mut bytes = tiny_pe(tree);
        put(&mut bytes, TINY_SECTION + 12, packed_rva);
        put(&mut bytes, TINY_DIRS + 8 * DIR_RESOURCE, packed_rva);
        put(&mut bytes, TINY_DIRS + 8 * DIR_RESOURCE + 4, tree.len() as u32);
        let img = Image::new(&bytes).unwrap();
        let mut dirs = vec![(0, 0); 16];
        dirs[DIR_RESOURCE] = (0x1000, tree.len() as u32);
        let rsrc = Section { vsize: 0x10000, vaddr: 0x1000, rawsize: 0x10000, rawptr: 0x400 };
        let oh = PeHeader {
            at: 0,
            width: Width::Pe32,
            opt_size: 224,
            hdr_len: 24 + 224,
            flags: 0,
            code_size: 0,
            code_base: 0x1000,
            image_base: 0x40_0000,
            file_align: 0x200,
            sections: vec![rsrc],
            dirs,
            table_at: 24 + 224,
        };
        let mut obuf = vec![0u8; 0x20000];
        let mut trailer = obuf.len() - 2;
        rebuild_resources(&img, &mut obuf, &oh, 0x1000, true, &mut trailer)
    }

    /// A root directory with one entry naming the data entry at 24, which
    /// holds `data_rva` and a zero size; `len` pads the tree.
    fn one_leaf_tree(data_rva: u32, len: usize) -> Vec<u8> {
        let mut tree = vec![0u8; len];
        tree[14..16].copy_from_slice(&1u16.to_le_bytes());
        put(&mut tree, 16, 3);
        put(&mut tree, 20, 24);
        put(&mut tree, 24, data_rva);
        tree
    }

    /// A data entry whose last eight bytes lie past the packed directory's
    /// copy used to be sliced out of it anyway.
    #[test]
    fn a_resource_data_entry_past_its_packed_copy_is_refused() {
        let err = rebuild_tree(&one_leaf_tree(0, 32), 0x1000).expect_err("a truncated entry");
        assert!(format!("{err}").contains("runs past its packed copy"), "{err}");
        rebuild_tree(&one_leaf_tree(0, 40), 0x1000).expect("the whole entry is there");
    }

    /// A moved leaf's back-pointer sits four bytes before its data, so a leaf
    /// at address 1 has none -- it used to underflow the address instead.
    #[test]
    fn a_moved_resource_below_address_four_has_no_back_pointer() {
        let err = rebuild_tree(&one_leaf_tree(1, 40), 0).expect_err("no back-pointer");
        assert!(format!("{err}").contains("no back-pointer"), "{err}");
    }

    /// A section whose raw data lies past the end of the file maps no bytes,
    /// so a DLL name it claims to hold is an error, not a slice past the end.
    #[test]
    fn a_section_past_the_end_of_the_file_holds_no_import_name() {
        let mut bytes = tiny_pe(&[0x11; 0x40]);
        put(&mut bytes, TINY_SECTION + 20, 0x10_0000);
        let img = Image::new(&bytes).unwrap();
        let err = img.cstr_at_rva(0x1000).expect_err("nothing is there");
        assert!(format!("{err}").contains("outside the image"), "{err}");
    }

    /// A section mapped at the top of the address space must not wrap the
    /// loader-table scan back to address zero (endless in release) or
    /// overflow its end address.
    #[test]
    fn a_section_at_the_top_of_the_address_space_does_not_wrap_the_loader_scan() {
        for rawsize in [0xf, 0x40] {
            let mut bytes = tiny_pe(&[0x11; 0x40]);
            put(&mut bytes, TINY_SECTION + 12, 0xffff_fff0);
            put(&mut bytes, TINY_SECTION + 16, rawsize);
            let img = Image::new(&bytes).unwrap();
            let err = loader_import_table(&img, &dlls(&[0x80])).expect_err("no loader table");
            assert!(format!("{err}").contains("cannot find"), "{err}");
        }
    }

    /// More distinct DLLs than a candidate table is followed for is this
    /// build's limit, not damage: UPX packs up to 4096 import descriptors, and
    /// older releases any number.
    #[test]
    fn more_dlls_than_the_scan_follows_is_unsupported() {
        let img_bytes = tiny_pe(&[0; 0x40]);
        let img = Image::new(&img_bytes).unwrap();
        let wanted: Vec<u32> = (0..=MAX_LOADER_DLLS as u32).map(|k| 0x40 + 20 * k).collect();
        let err = loader_import_table(&img, &dlls(&wanted)).expect_err("too many DLLs");
        assert!(matches!(err, UpxError::Unsupported(_)), "{err}");
    }

    /// A section whose every word names a wanted DLL relative to the candidate
    /// four bytes before it would make each candidate a full table walk; the
    /// scan's reads are bounded by the file instead.
    #[test]
    fn a_region_of_matching_descriptors_exhausts_the_scan_not_the_clock() {
        let mut body = vec![0u8; 0x10000];
        for q in (0..body.len()).step_by(4) {
            put(&mut body, q, 0x1000 + q as u32 - 12 + 0x40);
        }
        let img_bytes = tiny_pe(&body);
        let img = Image::new(&img_bytes).unwrap();
        let wanted: Vec<u32> = (0..MAX_LOADER_DLLS as u32).map(|k| 0x40 + 20 * k).collect();
        let err = loader_import_table(&img, &dlls(&wanted)).expect_err("no table, many walks");
        assert!(format!("{err}").contains("too many candidate loader tables"), "{err}");
    }

    /// An eight-byte directory is the empty block "some tricky dlls use", which
    /// UPX writes back in place of the zeros it packed.
    #[test]
    fn an_empty_relocation_block_is_written_back() {
        let (r, _, b) = relocs(0, 0, (0x1010, 8));
        r.unwrap();
        assert_eq!(&b[0x10..0x18], &[0, 0, 0, 0, 8, 0, 0, 0]);
    }
}
