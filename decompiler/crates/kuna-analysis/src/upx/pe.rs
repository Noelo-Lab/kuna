//! The PE arm of the static UPX unpacker: rebuild a `win32/pe` image from the
//! single compressed block UPX leaves in the packed file.
//!
//! Modelled on `PeFile::unpack0` + `PeFile::rebuild{Imports,Resources}` in
//! UPX's `src/pefile.cpp` (GPL-2.0-or-later). The layout is nothing like the
//! ELF one, which is why it needs its own walk:
//!
//! ```text
//!   [DOS hdr+stub][PE hdr][sections][PackHeader][ compressed block ][loader]
//!                                                        |
//!   obuf = [ image content from rvamin ][import block][original PE hdr +
//!            section table][extra info][le32 -> the original PE hdr]
//! ```
//!
//! Two things follow from that shape. The `PackHeader` is *not* in the tail --
//! it sits in the header padding immediately before the compressed data, so the
//! ELF tail scan never sees it. And the decompressed buffer is not the original
//! file: it is the original *memory image* with the import descriptors, the
//! import name strings and the resource directory stripped out, plus a trailer
//! that says how to put them back. Recovering the file means replaying those
//! rebuilds and then laying the sections out at their file offsets.
//!
//! # What is refused, and why
//!
//! The trailer is a stream that each rebuild step consumes a known number of
//! bytes from, in a fixed order, and which steps run at all depends on which
//! data directories the *original* image had. A directory this module does not
//! rebuild would silently desynchronize every later read, so the walk tallies
//! its own consumption against the trailer's real length and refuses a
//! mismatch. Base relocations are named up front as well, but only the ones the
//! packer kept: UPX strips an EXE's relocations by default and marks the header
//! `RELOCS_STRIPPED`, which costs no trailer bytes and no rebuild, while the
//! relocations it keeps -- every DLL's, every ASLR image's -- travel as an
//! optimized stream plus a trailer record this module does not replay. A TLS
//! directory needs nothing; UPX's own `rebuildTls` is empty. Everything else is
//! checked the way the ELF arm checks its stream: both Adler-32s, the rebuilt
//! directories confined to the sections that own them, and a reconstructed file
//! whose length must equal the original's to the byte.

use std::collections::BTreeSet;

use super::filter;
use super::{adler32, Block, PackInfo, UpxError};

/// `IMAGE_SIZEOF_SECTION_HEADER`.
const SECTION_HEADER_SIZE: usize = 40;
/// `IMAGE_IMPORT_DESCRIPTOR`: oft, timestamp, forwarder, name, iat.
const IMPORT_DESC_SIZE: usize = 20;
/// `IMAGE_RESOURCE_DATA_ENTRY`: offset, size, codepage, reserved.
const RESOURCE_DATA_ENTRY_SIZE: u32 = 16;

const DIR_IMPORT: usize = 1;
const DIR_RESOURCE: usize = 2;
const DIR_BASERELOC: usize = 5;
const DIR_BOUND_IMPORT: usize = 11;
const DIR_DELAY_IMPORT: usize = 13;

const RT_GROUP_ICON: u32 = 14;

/// `IMAGE_FILE_RELOCS_STRIPPED` in the COFF characteristics.
const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001;

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

/// A parsed PE header -- used both for the packed image and for the original
/// one recovered out of the compressed block, which have identical layout.
pub(super) struct PeHeader {
    /// Offset of the `PE\0\0` signature within its own buffer.
    pub at: usize,
    pub opt_size: usize,
    pub optional_magic: u16,
    /// COFF characteristics.
    pub flags: u16,
    pub code_size: u32,
    pub code_base: u32,
    pub sections: Vec<Section>,
    pub dirs: Vec<(u32, u32)>,
}

impl PeHeader {
    /// Total size of the signature + COFF header + optional header.
    fn header_size(&self) -> usize {
        24 + self.opt_size
    }

    fn section_table_at(&self) -> usize {
        self.at + self.header_size()
    }

    fn dir(&self, i: usize) -> (u32, u32) {
        self.dirs.get(i).copied().unwrap_or((0, 0))
    }

    /// Parse the headers whose `PE\0\0` signature sits at `at`.
    fn parse(b: &[u8], at: usize) -> Result<Self, UpxError> {
        if b.get(at..at + 4) != Some(b"PE\0\0") {
            return Err(corrupt("no PE signature where the header should be"));
        }
        let nsec = le16(b, at + 6)? as usize;
        let opt_size = le16(b, at + 20)? as usize;
        let opt = at + 24;
        let optional_magic = le16(b, opt)?;
        let (ndirs_offset, dirs_offset) = match optional_magic {
            0x10b => (92, 96),
            0x20b => (108, 112),
            magic => return Err(unsupported(format!("PE optional header magic {magic:#x}"))),
        };
        if nsec == 0 || nsec > 96 {
            return Err(corrupt(format!("PE header declares {nsec} sections")));
        }
        if opt_size < dirs_offset {
            return Err(corrupt(format!("optional header is {opt_size} bytes")));
        }
        let ndirs = le32(b, opt + ndirs_offset)? as usize;
        if ndirs > 16 || opt_size < dirs_offset + 8 * ndirs {
            return Err(corrupt(format!("optional header declares {ndirs} data directories")));
        }
        let mut dirs = Vec::with_capacity(ndirs);
        for i in 0..ndirs {
            dirs.push((le32(b, opt + dirs_offset + 8 * i)?, le32(b, opt + dirs_offset + 4 + 8 * i)?));
        }
        let table = at + 24 + opt_size;
        let mut sections = Vec::with_capacity(nsec);
        for i in 0..nsec {
            sections.push(Section::parse(b, table + i * SECTION_HEADER_SIZE)?);
        }
        Ok(PeHeader {
            at,
            opt_size,
            optional_magic,
            flags: le16(b, at + 22)?,
            code_size: le32(b, opt + 4)?,
            code_base: le32(b, opt + 20)?,
            sections,
            dirs,
        })
    }

    fn require_pe32(&self) -> Result<(), UpxError> {
        if self.optional_magic != 0x10b {
            return Err(unsupported("PE32+ image -- only 32-bit win32/pe is implemented"));
        }
        Ok(())
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
        let hdr = PeHeader::parse(bytes, pe_off)?;
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
    matches!(format, 9 | 21 | 36 | 43)
}

/// Rebuild the original file. `info` is the `PackHeader` [`find_pack_header`]
/// located.
pub(super) fn unpack(img: &Image<'_>, info: &PackInfo) -> Result<(Vec<u8>, Vec<Block>), UpxError> {
    if info.format != 9 {
        return Err(unsupported(format!(
            "target format {} ({}) -- only win32/pe is implemented",
            info.format,
            super::format_name(info.format)
        )));
    }
    img.hdr.require_pe32()?;
    let last = img.hdr.sections.last().expect("parse rejects an empty section table");
    let packed_end = last.rawptr as usize + last.rawsize as usize;
    if packed_end < img.bytes.len() {
        return Err(unsupported(format!(
            "packed file carries a {}-byte overlay past its last section",
            img.bytes.len() - packed_end
        )));
    }

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
    let mut oh = PeHeader::parse(&obuf, skip)?;
    oh.require_pe32()?;
    let rvamin = oh.sections[0].vaddr;
    let extra = oh.section_table_at() + oh.sections.len() * SECTION_HEADER_SIZE;
    if extra > trailer_end {
        return Err(corrupt("recovered section table overruns the compressed block"));
    }

    // UPX filters only the code, and initializes the filter with
    // `codebase - rvamin` as its addend; this unfilter folds that term out, so
    // an image whose code does not start at `rvamin` has to be refused rather
    // than unfiltered with the wrong bias.
    if oh.code_base != rvamin {
        return Err(unsupported(format!(
            "code base {:#x} is not the image's first section {rvamin:#x}",
            oh.code_base
        )));
    }
    if info.filter != 0 {
        let end = (oh.code_size as usize).min(obuf.len());
        filter::unfilter(info.filter, info.filter_cto, &mut obuf[..end])
            .map_err(|e| UpxError::Unsupported(e.to_string()))?;
    }

    for (dir, what) in [
        (DIR_BOUND_IMPORT, "bound imports"),
        (DIR_DELAY_IMPORT, "delay-loaded imports"),
    ] {
        if oh.dir(dir).0 != 0 && oh.dir(dir).1 != 0 {
            return Err(unsupported(format!("original image has {what}, which UPX packs into the trailer and this build does not rebuild")));
        }
    }

    let mut trailer = extra;
    rebuild_imports(img, &mut obuf, &oh, rvamin, &mut trailer)?;
    rebuild_relocs(&mut obuf, &mut oh, img.hdr.flags, skip, rvamin)?;
    rebuild_resources(img, &mut obuf, &oh, rvamin, &mut trailer)?;
    // Every rebuild consumes a fixed number of trailer bytes, so a leftover is
    // proof that the original image needed a step this build did not run -- and
    // that whatever was rebuilt after it read the wrong bytes.
    if trailer != trailer_end {
        let left = trailer_end
            .checked_sub(trailer)
            .ok_or_else(|| corrupt("trailer records run past the end of the trailer"))?;
        return Err(unsupported(format!(
            "trailer has {left} bytes this build cannot account for -- the image uses a PE feature UPX rebuilds and kuna does not"
        )));
    }

    let bytes = assemble(img, &obuf, &oh, rvamin, skip, info.u_file_size as usize)?;
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
    Ordinal(u32),
}

/// `PeFile::rebuildImports`: UPX strips the import descriptors, the thunk
/// arrays and the hint/name entries out of the image and replaces them with a
/// compact list in the trailer; the DLL *names* it keeps in the packed file, in
/// the loader's own import table. Put all three back.
fn rebuild_imports(
    img: &Image<'_>,
    obuf: &mut [u8],
    oh: &PeHeader,
    rvamin: u32,
    trailer: &mut usize,
) -> Result<(), UpxError> {
    let (imp_rva, imp_size) = oh.dir(DIR_IMPORT);
    if imp_rva == 0 || imp_size as usize <= IMPORT_DESC_SIZE {
        return Ok(());
    }
    let idata = le32(obuf, *trailer)? as usize;
    let inamespos = le32(obuf, *trailer + 4)?;
    *trailer += 8;
    if inamespos == 0 {
        return Err(unsupported(
            "import names were left in place rather than relocated -- not implemented",
        ));
    }

    let dlls = parse_import_block(obuf, idata, rvamin)?;
    if dlls.is_empty() {
        return Ok(());
    }
    let desc_base = loader_import_table(img, &dlls)?;

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
    for (d, name) in dlls.iter().zip(&names) {
        write_at(obuf, rvamin, dll_name_rva, name)?;
        write_at(obuf, rvamin, dll_name_rva.saturating_add(name.len() as u32), &[0])?;
        let this_name = dll_name_rva;
        dll_name_rva = dll_name_rva.saturating_add(name.len() as u32 + 1);

        let mut thunk = d.iat;
        for t in &d.thunks {
            let value = match t {
                Thunk::Ordinal(ord) => ord | 0x8000_0000,
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
                    at
                }
            };
            write_at(obuf, rvamin, thunk, &value.to_le_bytes())?;
            thunk = thunk.saturating_add(4);
        }
        write_at(obuf, rvamin, thunk, &0u32.to_le_bytes())?;

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
    if descs.len() > imp_size as usize {
        return Err(corrupt(format!(
            "{} rebuilt import descriptors do not fit the original {imp_size}-byte directory",
            dlls.len()
        )));
    }
    write_at(obuf, rvamin, imp_rva, &descs)?;
    confine_to_section(oh, inamespos, imported - inamespos, "rebuilt import names")?;
    Ok(())
}

/// Walk the trailer's import list: per DLL a name offset and a thunk-array
/// address, then its imports as `01 name 00` or a 2- or 4-byte ordinal, then a
/// `00`; a zero name offset ends the list.
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
                    thunks.push(Thunk::Ordinal(u32::from(le16(obuf, at + 1)?)));
                    at += 3;
                }
                _ => {
                    thunks.push(Thunk::Ordinal(le32(obuf, at + 1)?));
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
/// the two lists differ in length whenever a binary does that.
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
/// binary does that and a positional walk can never agree.
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
                return Ok(false);
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

/// `PeFile::rebuildRelocs`, reduced to the two branches that consume no trailer
/// bytes. UPX strips an EXE's relocations by default and marks both headers
/// `RELOCS_STRIPPED`; its unpacker then zeroes the recovered header's relocation
/// directory instead of rebuilding anything, and so does this. An eight-byte
/// directory is an empty block, written back in place of the zeros UPX packed.
/// Relocations the packer kept -- every DLL's, every ASLR image's -- travel as
/// an optimized stream plus a five-byte trailer record this build does not
/// replay; they are refused here, before the resource rebuild would read that
/// record as its icon count.
fn rebuild_relocs(
    obuf: &mut [u8],
    oh: &mut PeHeader,
    packed_flags: u16,
    skip: usize,
    rvamin: u32,
) -> Result<(), UpxError> {
    if packed_flags & IMAGE_FILE_RELOCS_STRIPPED != 0 {
        oh.flags |= IMAGE_FILE_RELOCS_STRIPPED;
        obuf[skip + 22..skip + 24].copy_from_slice(&oh.flags.to_le_bytes());
        if let Some(dir) = oh.dirs.get_mut(DIR_BASERELOC) {
            *dir = (0, 0);
            let at = skip + 24 + 96 + 8 * DIR_BASERELOC;
            obuf[at..at + 8].fill(0);
        }
    }
    let (rva, size) = oh.dir(DIR_BASERELOC);
    if rva == 0 || size == 0 || oh.flags & IMAGE_FILE_RELOCS_STRIPPED != 0 {
        return Ok(());
    }
    if size == 8 {
        return write_at(obuf, rvamin, rva, &[0, 0, 0, 0, 8, 0, 0, 0]);
    }
    Err(unsupported(
        "original image keeps its base relocations, which UPX packs as an optimized stream this build does not rebuild",
    ))
}

/// `PeFile::rebuildResources`: UPX keeps the resource *directory* uncompressed
/// in the packed file and moves the leaves it compressed into the image. Copy
/// the directory back to where the original had it, repointing every leaf UPX
/// moved at the address recorded in front of its data.
fn rebuild_resources(
    img: &Image<'_>,
    obuf: &mut [u8],
    oh: &PeHeader,
    rvamin: u32,
    trailer: &mut usize,
) -> Result<(), UpxError> {
    let (res_rva, res_size) = oh.dir(DIR_RESOURCE);
    if res_rva == 0 || res_size == 0 {
        return Ok(());
    }
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
    let dir = tree
        .get(..walk.extent as usize)
        .ok_or_else(|| corrupt("resource directory runs past its packed copy"))?;
    write_at(obuf, rvamin, res_rva, dir)?;
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
/// data entry reached twice is refused. UPX never writes one: its packer lays
/// the directory out afresh with every node once, even when the original shared
/// a subtree, and following one would let a small tree fan out into an
/// exponential walk.
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

/// Lay the recovered image back out as a file: the packed file's own DOS header
/// and stub (UPX preserves it), the recovered PE header and section table, then
/// each section's raw data read out of the image at its own address.
///
/// The DOS header is the one part of the original file UPX keeps rather than
/// packs, so a packed image that shrank it below the 64 bytes a DOS header needs
/// has thrown the original's away: `e_lfanew` no longer exists to be rewritten,
/// and the recovered PE header would sit at an offset nothing points at. That is
/// refused here rather than written out, and the assembled file is re-read at the
/// end to prove it is a PE a loader can follow. The layout is sized, and checked
/// against the `original_len` the PackHeader declares, before anything is
/// allocated for it.
fn assemble(
    img: &Image<'_>,
    obuf: &[u8],
    oh: &PeHeader,
    rvamin: u32,
    skip: usize,
    original_len: usize,
) -> Result<Vec<u8>, UpxError> {
    if img.pe_off < 0x40 {
        return Err(unsupported(format!(
            "packed image keeps only {} bytes of DOS header, so the original's is gone",
            img.pe_off
        )));
    }
    let table = oh.sections.len() * SECTION_HEADER_SIZE;
    let header = obuf
        .get(skip..skip + oh.header_size() + table)
        .ok_or_else(|| corrupt("recovered PE header runs past the block"))?;
    let mut end = img.pe_off + header.len();
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
    if end != original_len {
        return Err(corrupt(format!(
            "rebuilt {end} bytes, PackHeader declares an original of {original_len}"
        )));
    }
    let mut out = vec![0u8; end];
    out[..img.pe_off].copy_from_slice(&img.bytes[..img.pe_off]);
    out[img.pe_off..img.pe_off + header.len()].copy_from_slice(header);
    for (ptr, data) in raw {
        out[ptr..ptr + data.len()].copy_from_slice(data);
    }
    // A section whose raw data starts inside the headers would have overwritten
    // what was just written there, so read the result back the way a loader
    // would rather than trusting the layout that produced it.
    if le32(&out, 0x3c)? as usize != img.pe_off || out.get(img.pe_off..img.pe_off + 4) != Some(b"PE\0\0")
    {
        return Err(corrupt(
            "rebuilt file's headers were overwritten by a section's raw data",
        ));
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

    fn offsets(name_offs: &[u32]) -> BTreeSet<u32> {
        name_offs.iter().copied().collect()
    }

    /// The regression: a 32-bit MSVC crackme importing seven DLLs across
    /// thirteen descriptors (KERNEL32 four times, three others twice). Its
    /// loader table holds one descriptor per distinct DLL, so the two lists are
    /// 7 and 13 long and a position-by-position walk could never agree -- the
    /// unpack failed with "cannot find the loader import table".
    #[test]
    fn a_dll_imported_by_several_descriptors_still_matches_its_loader_table() {
        let base = 0x73a0c;
        let table: Vec<u32> = [224, 237, 250, 263, 273, 286, 297]
            .iter()
            .map(|o| base + o)
            .collect();
        let repeated = [224, 224, 224, 224, 237, 237, 250, 263, 273, 273, 286, 286, 297];
        assert!(table_names_dlls(&table, base, &offsets(&repeated)));
    }

    #[test]
    fn one_descriptor_per_dll_still_matches() {
        let base = 0x1000;
        let table: Vec<u32> = [16, 32, 48].iter().map(|o| base + o).collect();
        assert!(table_names_dlls(&table, base, &offsets(&[16, 32, 48])));
    }

    /// Set equality both ways: a table naming a DLL the trailer never asks for
    /// is the wrong table, and so is one missing a DLL the trailer wants.
    #[test]
    fn a_table_whose_names_differ_is_rejected() {
        let base = 0x1000;
        let table: Vec<u32> = [16, 32, 48].iter().map(|o| base + o).collect();
        assert!(!table_names_dlls(&table, base, &offsets(&[16, 32])));
        assert!(!table_names_dlls(&table, base, &offsets(&[16, 32, 48, 64])));
        assert!(!table_names_dlls(&[], base, &offsets(&[16])));
    }

    /// A minimal PE header at `skip` inside a block: one section, sixteen data
    /// directories, `flags` as the COFF characteristics and `reloc` in entry 5.
    fn block_with_header(skip: usize, flags: u16, reloc: (u32, u32)) -> Vec<u8> {
        let mut b = vec![0u8; skip + 24 + 224 + SECTION_HEADER_SIZE];
        b[skip..skip + 4].copy_from_slice(b"PE\0\0");
        b[skip + 6..skip + 8].copy_from_slice(&1u16.to_le_bytes());
        b[skip + 20..skip + 22].copy_from_slice(&224u16.to_le_bytes());
        b[skip + 22..skip + 24].copy_from_slice(&flags.to_le_bytes());
        let opt = skip + 24;
        b[opt..opt + 2].copy_from_slice(&0x10bu16.to_le_bytes());
        b[opt + 92..opt + 96].copy_from_slice(&16u32.to_le_bytes());
        let d = opt + 96 + 8 * DIR_BASERELOC;
        b[d..d + 4].copy_from_slice(&reloc.0.to_le_bytes());
        b[d + 4..d + 8].copy_from_slice(&reloc.1.to_le_bytes());
        b
    }

    fn relocs(
        flags: u16,
        packed_flags: u16,
        reloc: (u32, u32),
    ) -> (Result<(), UpxError>, PeHeader, Vec<u8>) {
        let skip = 0x100;
        let mut b = block_with_header(skip, flags, reloc);
        let mut oh = PeHeader::parse(&b, skip).unwrap();
        let r = rebuild_relocs(&mut b, &mut oh, packed_flags, skip, 0x1000);
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
        let written = PeHeader::parse(&b, 0x100).unwrap();
        assert_eq!(written.dir(DIR_BASERELOC), (0, 0));
        assert_ne!(written.flags & IMAGE_FILE_RELOCS_STRIPPED, 0);
    }

    /// Relocations the packer kept travel as an optimized stream plus a trailer
    /// record this build does not replay: with neither header flagged, the image
    /// is refused by name rather than walked into a desynchronized trailer.
    #[test]
    fn kept_relocations_are_refused() {
        let (r, _, _) = relocs(0, 0, (0x3b000, 0x5cc));
        let err = r.expect_err("kept relocations need a rebuild this build lacks");
        assert!(
            matches!(&err, UpxError::Unsupported(m) if m.contains("base relocations")),
            "{err}"
        );
    }

    #[test]
    fn an_image_without_relocations_needs_nothing() {
        let (r, oh, _) = relocs(0, 0, (0, 0));
        r.unwrap();
        assert_eq!(oh.flags, 0);
    }

    fn put(b: &mut [u8], at: usize, v: u32) {
        b[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }

    /// Where [`tiny_pe`] writes its one section header and its data directories.
    const TINY_SECTION: usize = 0x40 + 24 + 224;
    const TINY_DIRS: usize = 0x40 + 24 + 96;

    /// A packed image with one section mapping `body` at 0x1000 from file
    /// offset 0x200, and no data directories set.
    fn tiny_pe(body: &[u8]) -> Vec<u8> {
        let mut b = vec![0u8; 0x200];
        b[..2].copy_from_slice(b"MZ");
        put(&mut b, 0x3c, 0x40);
        b[0x40..0x44].copy_from_slice(b"PE\0\0");
        b[0x46..0x48].copy_from_slice(&1u16.to_le_bytes());
        b[0x54..0x56].copy_from_slice(&224u16.to_le_bytes());
        b[0x58..0x5a].copy_from_slice(&0x10bu16.to_le_bytes());
        put(&mut b, 0x58 + 92, 16);
        let len = body.len() as u32;
        for (field, v) in [(8, len), (12, 0x1000), (16, len), (20, 0x200)] {
            put(&mut b, TINY_SECTION + field, v);
        }
        b.extend_from_slice(body);
        b
    }

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
            opt_size: 224,
            optional_magic: 0x10b,
            flags: 0,
            code_size: 0,
            code_base: 0x1000,
            sections: vec![rsrc],
            dirs,
        };
        let mut obuf = vec![0u8; 0x20000];
        let mut trailer = obuf.len() - 2;
        rebuild_resources(&img, &mut obuf, &oh, 0x1000, &mut trailer)
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
