//! The ELF arm of the static UPX unpacker: walk a packed image's compressed
//! block stream and rebuild the original file byte for byte.
//!
//! Modelled on `PackLinuxElf{32,64}::unpack` + `PackUnix::unpackExtent` in
//! UPX's `src/p_lx_elf.cpp` / `src/p_unix.cpp` (GPL-2.0-or-later), which is the
//! only normative description of where each block lands. The shape:
//!
//! ```text
//!   [stub Ehdr+Phdrs][l_info][p_info] b_info+data ... [loader] b_info+data ... [end][PackHeader]
//!                                     \___ one extent per original PT_LOAD ___/  \_ the gaps _/
//! ```
//!
//! The first block decompresses to the *original* Ehdr + Phdrs; those Phdrs
//! then say where every later block belongs, so the walk rewinds and replays
//! the stream as a series of extents. Between the PT_LOAD extents and the gap
//! extents sits the loader stub, whose length `l_info.l_lsize` gives -- the one
//! place the reader has to jump rather than stream.
//!
//! Nothing here trusts a size it read from the file: every block is bounds
//! checked, the decompressor must consume its input exactly, the stream must
//! terminate on the `UPX!` end marker at exactly the right offset, and the
//! total must equal the original file size the header declares. A run that
//! cannot satisfy all four returns `Err` rather than a partial image.

use super::filter;
use super::nrv::{self, BitOrder, Variant};
use super::{adler32, Block, PackInfo, UpxError};

/// UPX's own cap on a block's uncompressed size (`PackUnix::blocksize`), plus a
/// sanity ceiling so a corrupt `p_info` cannot ask for an unbounded allocation.
const MAX_BLOCKSIZE: u32 = 512 * 1024 * 1024;

const PT_LOAD: u32 = 1;
const PF_X: u32 = 1;

pub(super) struct Image<'a> {
    bytes: &'a [u8],
    be: bool,
    class64: bool,
}

impl<'a> Image<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Result<Self, UpxError> {
        if bytes.len() < 64 || &bytes[..4] != b"\x7fELF" {
            return Err(UpxError::Corrupt("packed image is not an ELF file".into()));
        }
        let class64 = match bytes[4] {
            1 => false,
            2 => true,
            other => {
                return Err(UpxError::Corrupt(format!("bad ELF class {other}")));
            }
        };
        let be = match bytes[5] {
            1 => false,
            2 => true,
            other => {
                return Err(UpxError::Corrupt(format!("bad ELF data encoding {other}")));
            }
        };
        Ok(Image { bytes, be, class64 })
    }

    fn u16_at(&self, off: usize) -> Result<u16, UpxError> {
        let s = self.slice(off, 2)?;
        Ok(if self.be {
            u16::from_be_bytes([s[0], s[1]])
        } else {
            u16::from_le_bytes([s[0], s[1]])
        })
    }

    fn u32_at(&self, off: usize) -> Result<u32, UpxError> {
        Ok(read_u32(self.slice(off, 4)?, self.be))
    }

    fn word_at(&self, off: usize) -> Result<u64, UpxError> {
        if self.class64 {
            let s = self.slice(off, 8)?;
            let a: [u8; 8] = s.try_into().expect("slice checked");
            Ok(if self.be { u64::from_be_bytes(a) } else { u64::from_le_bytes(a) })
        } else {
            Ok(u64::from(self.u32_at(off)?))
        }
    }

    fn slice(&self, off: usize, len: usize) -> Result<&'a [u8], UpxError> {
        self.bytes
            .get(off..off + len)
            .ok_or_else(|| UpxError::Corrupt(format!("read past end of file at {off:#x}")))
    }

    fn ehdr_size(&self) -> usize {
        if self.class64 {
            64
        } else {
            52
        }
    }

    fn phdr_size(&self) -> usize {
        if self.class64 {
            56
        } else {
            32
        }
    }
}

/// One program header, in whichever ELF class the image uses.
#[derive(Clone, Copy)]
struct Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_filesz: u64,
    p_memsz: u64,
}

/// Parse `count` program headers out of a decompressed header blob (or out of
/// the packed image itself, which has the same layout).
fn parse_phdrs(img: &Image<'_>, buf: &[u8], count: usize) -> Result<Vec<Phdr>, UpxError> {
    let (eh, ph) = (img.ehdr_size(), img.phdr_size());
    let need = eh + count * ph;
    if buf.len() < need {
        return Err(UpxError::Corrupt(format!(
            "program header table needs {need} bytes, header blob has {}",
            buf.len()
        )));
    }
    let be = img.be;
    let mut out = Vec::with_capacity(count);
    for j in 0..count {
        let o = eh + j * ph;
        let (p_type, p_flags, p_offset, p_vaddr, p_filesz, p_memsz) = if img.class64 {
            (
                read_u32(&buf[o..o + 4], be),
                read_u32(&buf[o + 4..o + 8], be),
                read_u64(&buf[o + 8..o + 16], be),
                read_u64(&buf[o + 16..o + 24], be),
                read_u64(&buf[o + 32..o + 40], be),
                read_u64(&buf[o + 40..o + 48], be),
            )
        } else {
            (
                read_u32(&buf[o..o + 4], be),
                read_u32(&buf[o + 24..o + 28], be),
                u64::from(read_u32(&buf[o + 4..o + 8], be)),
                u64::from(read_u32(&buf[o + 8..o + 12], be)),
                u64::from(read_u32(&buf[o + 16..o + 20], be)),
                u64::from(read_u32(&buf[o + 20..o + 24], be)),
            )
        };
        out.push(Phdr { p_type, p_flags, p_offset, p_vaddr, p_filesz, p_memsz });
    }
    Ok(out)
}

fn read_u32(s: &[u8], be: bool) -> u32 {
    let a: [u8; 4] = s[..4].try_into().expect("caller sized the slice");
    if be {
        u32::from_be_bytes(a)
    } else {
        u32::from_le_bytes(a)
    }
}

fn read_u64(s: &[u8], be: bool) -> u64 {
    let a: [u8; 8] = s[..8].try_into().expect("caller sized the slice");
    if be {
        u64::from_be_bytes(a)
    } else {
        u64::from_le_bytes(a)
    }
}

/// The 12-byte `b_info` that prefixes every compressed block.
#[derive(Clone, Copy)]
struct BInfo {
    sz_unc: u32,
    sz_cpr: u32,
    method: u8,
    ftid: u8,
    cto8: u8,
}

/// What the walk accumulates as it consumes blocks: the block inventory plus
/// the packer's two running Adler-32s, which are the only true integrity check
/// a compressed stream carries.
struct Walk {
    blocks: Vec<Block>,
    u_adler: u32,
    c_adler: u32,
}

impl Walk {
    fn new() -> Self {
        Walk { blocks: Vec::new(), u_adler: 1, c_adler: 1 }
    }
}

/// Streaming cursor over the packed image's block region.
struct Reader<'a, 'i> {
    img: &'i Image<'a>,
    pos: usize,
}

impl<'a, 'i> Reader<'a, 'i> {
    fn b_info(&mut self) -> Result<BInfo, UpxError> {
        let s = self.img.slice(self.pos, 12)?;
        let info = BInfo {
            sz_unc: read_u32(&s[0..4], self.img.be),
            sz_cpr: read_u32(&s[4..8], self.img.be),
            method: s[8],
            ftid: s[9],
            cto8: s[10],
        };
        self.pos += 12;
        Ok(info)
    }
}

/// The whole reconstruction. `info` is the already-parsed trailing PackHeader.
pub(super) fn unpack(
    img: &Image<'_>,
    info: &PackInfo,
) -> Result<(Vec<u8>, Vec<Block>), UpxError> {
    // `l_info` sits immediately before the overlay; `p_info` starts it.
    let l_off = (info.overlay_offset as usize)
        .checked_sub(12)
        .ok_or_else(|| UpxError::Corrupt("overlay_offset underflows l_info".into()))?;
    // l_magic is stored le32 whatever the target's endianness.
    let l_magic = img.slice(l_off + 4, 4)?;
    if u32::from_le_bytes([l_magic[0], l_magic[1], l_magic[2], l_magic[3]])
        != super::UPX_MAGIC_LE32
    {
        return Err(UpxError::Corrupt("l_info magic missing before the overlay".into()));
    }
    let lsize = usize::from(img.u16_at(l_off + 8)?);
    let orig_file_size = img.u32_at(info.overlay_offset as usize + 4)?;
    let blocksize = img.u32_at(info.overlay_offset as usize + 8)?;
    if orig_file_size != info.u_file_size {
        return Err(UpxError::Corrupt(format!(
            "p_info file size {orig_file_size} disagrees with PackHeader {}",
            info.u_file_size
        )));
    }
    if blocksize == 0 || blocksize > orig_file_size || blocksize > MAX_BLOCKSIZE {
        return Err(UpxError::Corrupt(format!("implausible block size {blocksize}")));
    }

    // Old (2001-era) loaders wrote an 8-byte b_info with no method or filter id;
    // the method then comes from the file-level PackHeader and the filter from a
    // whole-file rule. Refusing is honest -- guessing would corrupt the output.
    if info.version <= 11 {
        return Err(UpxError::Unsupported(format!(
            "UPX loader version {} (pre-12 block header layout)",
            info.version
        )));
    }

    let e_phoff = img.word_at(if img.class64 { 32 } else { 28 })?;
    if e_phoff != img.ehdr_size() as u64 {
        return Err(UpxError::Unsupported(
            "packed image has non-contiguous Ehdr/Phdrs (packed shared library?)".into(),
        ));
    }

    let c_phnum = usize::from(img.u16_at(if img.class64 { 56 } else { 44 })?);
    let c_phdrs = parse_phdrs(img, img.bytes, c_phnum)?;
    // In a packed executable the overlay begins immediately after the stub's own
    // program headers. A packed *shared library* keeps the original headers and
    // splices the overlay in further down (UPX's `xct_off`), which this walk does
    // not implement -- so when the first block will not decode, that offset is
    // what says which of the two we are looking at.
    let shlib_layout = l_off != img.ehdr_size() + c_phnum * img.phdr_size();
    let shlib = || {
        UpxError::Unsupported(
            "packed shared library: the overlay does not start at the program headers".into(),
        )
    };

    let mut out = vec![0u8; orig_file_size as usize];
    let mut walk = Walk::new();
    let mut written = 0usize;
    let start = info.overlay_offset as usize + 12;
    let mut rd = Reader { img, pos: start };

    // Block 1 is the original Ehdr + Phdrs -- the map for everything after it.
    let first = rd.b_info()?;
    let hdr_blob = decode_block(img, &mut rd, &first, blocksize, &mut walk)
        .map_err(|e| if shlib_layout { shlib() } else { e })?;
    let ehsize = img.ehdr_size();
    if hdr_blob.len() < ehsize || hdr_blob[..4] != *b"\x7fELF" {
        return Err(if shlib_layout {
            shlib()
        } else {
            UpxError::Corrupt("first block is not an ELF header".into())
        });
    }
    for k in [4usize, 5, 6, 16, 17, 18, 19] {
        // EI_CLASS/EI_DATA/EI_VERSION, then e_type and e_machine: the packer
        // preserves all of them, so a mismatch means we decoded the wrong bytes.
        if hdr_blob[k] != img.bytes[k] {
            return Err(UpxError::Corrupt(
                "recovered ELF header disagrees with the packed stub".into(),
            ));
        }
    }
    // e_phnum: 8 bytes before the end of the Ehdr in both ELF classes.
    let pn = ehsize - 8;
    let u_phnum = usize::from(if img.be {
        u16::from_be_bytes([hdr_blob[pn], hdr_blob[pn + 1]])
    } else {
        u16::from_le_bytes([hdr_blob[pn], hdr_blob[pn + 1]])
    });
    let phdrs = parse_phdrs(img, &hdr_blob, u_phnum)?;

    // Replay the stream from the top: the header block is also the first bytes
    // of the first PT_LOAD's extent.
    rd.pos = start;
    walk = Walk::new();
    for p in &phdrs {
        if p.p_type == PT_LOAD {
            written += unpack_extent(
                img,
                &mut rd,
                p.p_filesz,
                p.p_offset,
                blocksize,
                &mut out,
                &mut walk,
            )?;
        }
    }

    // Skip the loader stub, which sits between the PT_LOAD extents and the
    // compressed "gaps" (the parts of the original file no PT_LOAD covered).
    let e_entry = img.word_at(24)?;
    let mut off_entry = 0u64;
    for p in &c_phdrs {
        if p.p_type == PT_LOAD && e_entry.wrapping_sub(p.p_vaddr) < p.p_filesz {
            off_entry = e_entry.wrapping_sub(p.p_vaddr) + p.p_offset;
            break;
        }
    }
    let file_size = img.bytes.len() as u64;
    if c_phdrs.len() >= 2
        && c_phdrs[0].p_filesz == 0x1000
        && c_phdrs[0].p_offset == 0
        && c_phdrs[1].p_offset == 0
        && c_phdrs[1].p_filesz == c_phdrs[1].p_memsz
    {
        // "C_BASE" layout: the loader is the whole of the second PT_LOAD.
        rd.pos = up4(c_phdrs[1].p_memsz) as usize;
    } else if off_entry + up4(lsize as u64) + 32 + 4 < up4(file_size) {
        let sz_d_info = d_info_size(img, &c_phdrs)?;
        let loader_offset = off_entry
            .checked_sub(sz_d_info)
            .ok_or_else(|| UpxError::Corrupt("loader offset underflows".into()))?;
        rd.pos = (loader_offset + lsize as u64) as usize;
    }

    for j in 0..phdrs.len() {
        let gap = find_load_gap(&phdrs, j, info.u_file_size)?;
        if gap != 0 {
            let where_ = phdrs[j].p_offset + phdrs[j].p_filesz;
            written += unpack_extent(
                img,
                &mut rd,
                gap,
                where_,
                blocksize,
                &mut out,
                &mut walk,
            )?;
        }
    }

    // The stream must end here, on the marker UPX writes after the last block.
    let end = rd.b_info()?;
    let raw = img.slice(rd.pos - 12 + 4, 4)?;
    let end_magic = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    if end.sz_unc != 0 || end_magic != super::UPX_MAGIC_LE32 {
        return Err(UpxError::Corrupt(format!(
            "expected the UPX end marker at {:#x}, found sz_unc={} sz_cpr={:#x}",
            rd.pos - 12,
            end.sz_unc,
            end.sz_cpr
        )));
    }
    // Nothing but 4-byte alignment padding may separate the end marker from the
    // trailing PackHeader; anything more means the walk stopped in the wrong place.
    let after_marker = rd.pos;
    if info.pack_header_offset < after_marker || info.pack_header_offset - after_marker >= 4 {
        return Err(UpxError::Corrupt(format!(
            "end marker at {:#x} is not adjacent to the PackHeader at {:#x}",
            after_marker - 12,
            info.pack_header_offset
        )));
    }
    if written != orig_file_size as usize {
        return Err(UpxError::Corrupt(format!(
            "reconstructed {written} bytes, original file size is {orig_file_size}"
        )));
    }
    // The packer's own end-to-end check. Structure alone cannot catch a flipped
    // literal byte -- it decodes to a wrong image of exactly the right size --
    // so this is what makes the output trustworthy rather than merely plausible.
    if walk.c_adler != info.c_adler || walk.u_adler != info.u_adler {
        return Err(UpxError::Corrupt(format!(
            "checksum mismatch: compressed {:#010x} (header {:#010x}), \
             uncompressed {:#010x} (header {:#010x})",
            walk.c_adler, info.c_adler, walk.u_adler, info.u_adler
        )));
    }
    Ok((out, walk.blocks))
}

/// `PackUnix::unpackExtent`: consume whole blocks until `wanted` uncompressed
/// bytes have landed at `offset` in the output.
#[allow(clippy::too_many_arguments)]
fn unpack_extent(
    img: &Image<'_>,
    rd: &mut Reader<'_, '_>,
    wanted: u64,
    offset: u64,
    blocksize: u32,
    out: &mut [u8],
    walk: &mut Walk,
) -> Result<usize, UpxError> {
    let mut done = 0u64;
    while done < wanted {
        let hdr = rd.b_info()?;
        let data = decode_block(img, rd, &hdr, blocksize, walk)?;
        let at = (offset + done) as usize;
        let end = at
            .checked_add(data.len())
            .filter(|e| *e <= out.len())
            .ok_or_else(|| {
                UpxError::Corrupt(format!(
                    "block at {at:#x}+{} runs past the original file",
                    data.len()
                ))
            })?;
        out[at..end].copy_from_slice(&data);
        done += data.len() as u64;
    }
    if done != wanted {
        return Err(UpxError::Corrupt(format!(
            "extent overshot: produced {done} of {wanted} bytes"
        )));
    }
    Ok(done as usize)
}

/// Decompress (or copy, for a stored block) one `b_info`-headed block and
/// reverse its filter.
fn decode_block(
    img: &Image<'_>,
    rd: &mut Reader<'_, '_>,
    hdr: &BInfo,
    blocksize: u32,
    walk: &mut Walk,
) -> Result<Vec<u8>, UpxError> {
    let at = rd.pos - 12;
    if hdr.sz_unc == 0 || hdr.sz_cpr == 0 {
        return Err(UpxError::Corrupt(format!("zero-length block at {at:#x}")));
    }
    if hdr.sz_cpr > hdr.sz_unc || hdr.sz_unc > blocksize {
        return Err(UpxError::Corrupt(format!(
            "block at {at:#x} declares sz_unc={} sz_cpr={} against block size {blocksize}",
            hdr.sz_unc, hdr.sz_cpr
        )));
    }
    let src = img.slice(rd.pos, hdr.sz_cpr as usize)?;
    rd.pos += hdr.sz_cpr as usize;
    walk.c_adler = adler32(walk.c_adler, src);

    let stored = hdr.sz_cpr == hdr.sz_unc;
    let mut data = if stored {
        src.to_vec()
    } else {
        let (variant, order) = method_codec(hdr.method)?;
        nrv::decompress(variant, order, src, hdr.sz_unc as usize)
            .map_err(|e| UpxError::Corrupt(format!("block at {at:#x}: {e}")))?
    };
    // A stored block was never filtered: UPX only records a filter id on a
    // block it actually compressed.
    if !stored && hdr.ftid != 0 {
        filter::unfilter(hdr.ftid, hdr.cto8, &mut data)
            .map_err(|e| UpxError::Unsupported(e.to_string()))?;
    }
    walk.u_adler = adler32(walk.u_adler, &data);
    walk.blocks.push(Block {
        offset: at,
        u_len: hdr.sz_unc,
        c_len: hdr.sz_cpr,
        method: hdr.method,
        filter: hdr.ftid,
        stored,
    });
    Ok(data)
}

/// Map a UPX `b_method` onto an NRV variant + bit-buffer layout, or refuse it
/// by name.
pub(super) fn method_codec(method: u8) -> Result<(Variant, BitOrder), UpxError> {
    Ok(match method {
        2 => (Variant::Nrv2b, BitOrder::Le32),
        3 => (Variant::Nrv2b, BitOrder::Byte),
        4 => (Variant::Nrv2b, BitOrder::Le16),
        5 => (Variant::Nrv2d, BitOrder::Le32),
        6 => (Variant::Nrv2d, BitOrder::Byte),
        7 => (Variant::Nrv2d, BitOrder::Le16),
        8 => (Variant::Nrv2e, BitOrder::Le32),
        9 => (Variant::Nrv2e, BitOrder::Byte),
        10 => (Variant::Nrv2e, BitOrder::Le16),
        other => {
            return Err(UpxError::Unsupported(format!(
                "compression method {other} ({})",
                super::method_name(other)
            )))
        }
    })
}

/// The per-architecture `d_info` prologue the loader stub is preceded by, whose
/// size tells the walk where the stub really starts.
fn d_info_size(img: &Image<'_>, c_phdrs: &[Phdr]) -> Result<u64, UpxError> {
    let first_is_exec = c_phdrs.first().is_some_and(|p| p.p_flags & PF_X != 0);
    if !first_is_exec {
        // Modern layout: the whole `d_info` array precedes the stub.
        return Ok(24);
    }
    let e_machine = img.u16_at(18)?;
    Ok(match e_machine {
        3 | 62 => 8,            // EM_386, EM_X86_64
        40 | 183 => 16,         // EM_ARM, EM_AARCH64
        20 | 21 => 12,          // EM_PPC, EM_PPC64
        8 => 4,                 // EM_MIPS
        other => {
            return Err(UpxError::Unsupported(format!(
                "ELF machine {other} (no known loader prologue size)"
            )))
        }
    })
}

/// `PackLinuxElf::find_LOAD_gap`: the run of original-file bytes after PT_LOAD
/// `k` that no other PT_LOAD covers -- what the "gap" extents carry.
fn find_load_gap(phdrs: &[Phdr], k: usize, u_file_size: u32) -> Result<u64, UpxError> {
    if phdrs[k].p_type != PT_LOAD {
        return Ok(0);
    }
    let hi = phdrs[k].p_offset + phdrs[k].p_filesz;
    let mut lo = u64::from(u_file_size);
    if lo < hi {
        return Err(UpxError::Corrupt("PT_LOAD extends beyond end of file".into()));
    }
    let n = phdrs.len();
    let mut j = k;
    loop {
        j += 1;
        if j == n {
            j = 0;
        }
        if j == k {
            break;
        }
        if phdrs[j].p_type == PT_LOAD {
            let t = phdrs[j].p_offset;
            if t.wrapping_sub(hi) < lo.wrapping_sub(hi) {
                lo = t;
                if hi == lo {
                    break;
                }
            }
        }
    }
    Ok(lo - hi)
}

fn up4(v: u64) -> u64 {
    (v + 3) & !3
}
