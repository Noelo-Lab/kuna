//! Static unpacking of UPX-packed executables.
//!
//! A UPX-packed binary shows the loader only: one tiny stub segment plus a blob
//! of compressed bytes. Every analysis in this crate -- function discovery,
//! strings, DWARF, the Listing walk -- therefore sees a program with almost no
//! code in it, and reports that honestly and uselessly. This module recovers the
//! *original* image so the rest of the tier has something to analyze.
//!
//! It is a reimplementation, not a wrapper: `upx -d` cannot be assumed present
//! on a machine running a release `kuna`, and shelling out to a packer to look
//! at a hostile binary is not a thing an analyzer should do. The compressed
//! stream is UCL NRV2B/NRV2D/NRV2E ([`nrv`]) under a per-block branch filter
//! ([`filter`]); the reconstruction walk lives in `elf`.
//!
//! # Refusing beats guessing
//!
//! The output of this module is a file a human will disassemble and believe. A
//! subtly wrong one -- an unreversed filter leaves every call target wrong while
//! every size still adds up -- is far more expensive than no output at all. So
//! every unimplemented method, filter, format and layout is a named
//! [`UpxError::Unsupported`], and the walk additionally requires the stream to
//! terminate on the `UPX!` end marker at exactly the offset the last block ends
//! and to total exactly the original file size the header declares.
//!
//! Ported from UPX (GPL-2.0-or-later): `src/p_lx_elf.cpp`, `src/p_unix.cpp`,
//! `src/packhead.cpp`, `src/filter/*.h`, and UCL's `src/n2{b,d,e}_d.c`.

pub mod filter;
pub mod nrv;

mod elf;

use std::fmt;

/// `"UPX!"` as a little-endian u32 -- the magic on both `l_info` and the
/// trailing `PackHeader`, stored le32 regardless of target endianness.
pub const UPX_MAGIC_LE32: u32 = 0x2158_5055;

/// Size of the modern (version >= 10, non-DOS) `PackHeader`.
const PACK_HEADER_SIZE: usize = 32;

/// What the trailing `PackHeader` says about the packed file.
#[derive(Debug, Clone)]
pub struct PackInfo {
    /// File offset of the `UPX!` PackHeader.
    pub pack_header_offset: usize,
    /// Loader format version (`p[4]`), not the UPX release number.
    pub version: u8,
    /// `UPX_F_*` target format id (`p[5]`).
    pub format: u8,
    /// `M_*` compression method id (`p[6]`).
    pub method: u8,
    /// Compression level 1..=10 (`p[7]`).
    pub level: u8,
    /// File-level filter id and its `cto` byte (per-block ids override these).
    pub filter: u8,
    pub filter_cto: u8,
    /// Last block's uncompressed / compressed length, as UPX records them.
    pub u_len: u32,
    pub c_len: u32,
    /// Running Adler-32 over every uncompressed / compressed block, in stream
    /// order -- the packer's own end-to-end integrity check ([`adler32`]).
    pub u_adler: u32,
    pub c_adler: u32,
    /// Size of the original, unpacked file.
    pub u_file_size: u32,
    /// Offset of the `p_info` that starts the compressed overlay.
    pub overlay_offset: u32,
}

impl PackInfo {
    pub fn format_name(&self) -> &'static str {
        format_name(self.format)
    }

    pub fn method_name(&self) -> &'static str {
        method_name(self.method)
    }
}

/// One compressed block of the packed stream, as consumed.
#[derive(Debug, Clone)]
pub struct Block {
    /// File offset of the block's `b_info` header.
    pub offset: usize,
    pub u_len: u32,
    pub c_len: u32,
    pub method: u8,
    pub filter: u8,
    /// A block UPX could not compress, stored verbatim.
    pub stored: bool,
}

/// A successful unpack.
#[derive(Debug)]
pub struct Unpacked {
    pub info: PackInfo,
    /// The reconstructed original file.
    pub bytes: Vec<u8>,
    pub blocks: Vec<Block>,
}

impl Unpacked {
    /// Total compressed payload consumed, excluding block headers.
    pub fn compressed_bytes(&self) -> u64 {
        self.blocks.iter().map(|b| u64::from(b.c_len)).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpxError {
    /// No UPX PackHeader: the file is not UPX-packed (or not packed at all).
    NotPacked,
    /// Recognized as UPX, but this build cannot unpack it -- and says why.
    Unsupported(String),
    /// The headers are UPX's but do not describe a consistent file.
    Corrupt(String),
}

impl fmt::Display for UpxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpxError::NotPacked => write!(f, "no UPX PackHeader found"),
            UpxError::Unsupported(what) => write!(f, "unsupported UPX image: {what}"),
            UpxError::Corrupt(what) => write!(f, "corrupt UPX image: {what}"),
        }
    }
}

impl std::error::Error for UpxError {}

/// Is this image UPX-packed, and if so what does its header claim? Cheap: it
/// only reads the tail.
pub fn detect(image: &[u8]) -> Option<PackInfo> {
    parse_pack_header(image).ok()
}

/// Unpack `image`, returning the original file.
pub fn unpack(image: &[u8]) -> Result<Unpacked, UpxError> {
    let info = parse_pack_header(image)?;
    if !is_elf_format(info.format) {
        return Err(UpxError::Unsupported(format!(
            "target format {} ({}) -- only the ELF formats are implemented",
            info.format,
            info.format_name()
        )));
    }
    // Reject an unimplemented method up front, by name, rather than after
    // writing part of an image.
    elf::method_codec(info.method)?;
    let img = elf::Image::new(image)?;
    let (bytes, blocks) = elf::unpack(&img, &info)?;
    Ok(Unpacked { info, bytes, blocks })
}

/// UPX's `find_overlay_offset` + `decodePackHeaderFromBuf`: the PackHeader sits
/// just before the trailing 4-byte overlay offset, after any zero padding.
fn parse_pack_header(image: &[u8]) -> Result<PackInfo, UpxError> {
    if image.len() < PACK_HEADER_SIZE + 4 {
        return Err(UpxError::NotPacked);
    }
    // Scan only the tail UPX could have written into, and take the last
    // candidate whose own checksum validates -- the file's own bytes may well
    // contain "UPX!" elsewhere.
    let window = 2 * 4096 + 2 * (PACK_HEADER_SIZE + 4) + 1;
    let base = image.len().saturating_sub(window);
    let tail = &image[base..];
    let magic = UPX_MAGIC_LE32.to_le_bytes();
    let mut found: Option<usize> = None;
    for i in 0..tail.len().saturating_sub(PACK_HEADER_SIZE + 4 - 1) {
        if tail[i..i + 4] != magic {
            continue;
        }
        let p = &tail[i..];
        if p.len() < PACK_HEADER_SIZE + 4 {
            continue;
        }
        if p[4] < 10 {
            // Pre-10 headers have no checksum to validate and a different size;
            // this unpacker refuses them later anyway.
            continue;
        }
        if p[PACK_HEADER_SIZE - 1] != pack_header_checksum(&p[..PACK_HEADER_SIZE - 1]) {
            continue;
        }
        found = Some(i);
    }
    let i = found.ok_or(UpxError::NotPacked)?;
    let p = &tail[i..];
    let big_endian = p[5] >= 128;
    let rd = |o: usize| -> u32 {
        let a = [p[o], p[o + 1], p[o + 2], p[o + 3]];
        if big_endian {
            u32::from_be_bytes(a)
        } else {
            u32::from_le_bytes(a)
        }
    };
    let (u_len, c_len, u_adler, c_adler, u_file_size) = if big_endian {
        (rd(8), rd(12), rd(16), rd(20), rd(24))
    } else {
        (rd(16), rd(20), rd(8), rd(12), rd(24))
    };
    // The overlay offset is the le32/be32 word right after the header.
    let overlay_offset = {
        let o = PACK_HEADER_SIZE;
        let a = [p[o], p[o + 1], p[o + 2], p[o + 3]];
        if big_endian {
            u32::from_be_bytes(a)
        } else {
            u32::from_le_bytes(a)
        }
    };
    if u_file_size == 0 || (overlay_offset as usize) >= image.len() {
        return Err(UpxError::Corrupt(
            "PackHeader declares an overlay outside the file".into(),
        ));
    }
    Ok(PackInfo {
        pack_header_offset: base + i,
        version: p[4],
        format: p[5],
        method: p[6],
        level: p[7] & 15,
        filter: p[28],
        filter_cto: p[29],
        u_len,
        c_len,
        u_adler,
        c_adler,
        u_file_size,
        overlay_offset,
    })
}

/// Adler-32 (RFC 1950), seeded the way UPX seeds it (`upx_adler32(nullptr, 0)`
/// is 1). This is what makes a static unpack *verifiable*: an LZ stream has no
/// internal integrity, so a flipped literal byte decodes to a wrong image of
/// exactly the right length -- only the packer's own running checksum catches it.
pub fn adler32(seed: u32, data: &[u8]) -> u32 {
    const BASE: u32 = 65521;
    // The zlib bound: the largest run that cannot overflow the u32 accumulators.
    const NMAX: usize = 5552;
    let mut s1 = seed & 0xffff;
    let mut s2 = (seed >> 16) & 0xffff;
    for chunk in data.chunks(NMAX) {
        for &b in chunk {
            s1 += u32::from(b);
            s2 += s1;
        }
        s1 %= BASE;
        s2 %= BASE;
    }
    (s2 << 16) | s1
}

/// `get_packheader_checksum`: sum of every header byte after the magic, mod 251.
fn pack_header_checksum(after_magic: &[u8]) -> u8 {
    let sum: u32 = after_magic[4..].iter().map(|b| u32::from(*b)).sum();
    (sum % 251) as u8
}

fn is_elf_format(format: u8) -> bool {
    matches!(format, 12 | 20 | 22 | 23 | 25 | 30 | 39 | 42 | 45 | 132 | 133 | 137 | 140)
}

/// `UPX_F_*` names, from UPX's `src/conf.h`. Unknown ids report as such rather
/// than being guessed into a neighbour's name.
pub fn format_name(format: u8) -> &'static str {
    match format {
        1 => "dos/com",
        2 => "dos/sys",
        3 | 7 => "dos/exe",
        4 => "djgpp2/coff",
        5 => "watcom/le",
        8 => "tmt/adam",
        9 => "win32/pe",
        10 => "linux/i386",
        12 => "linux/elf386",
        14 => "linux/sh-i386",
        15 => "vmlinuz/i386",
        16 => "bvmlinuz/i386",
        18 => "ps1/exe",
        19 => "vmlinux/i386",
        20 => "linux/elfi386",
        21 => "arm/wince-pe",
        22 => "linux/amd64",
        23 => "linux/arm",
        24 => "bsd/i386",
        25 => "bsd/elf386",
        26 => "bsd/sh-i386",
        27 => "vmlinux/amd64",
        28 => "vmlinux/arm",
        29 => "mach/i386",
        30 => "linux/mipsel",
        31 => "vmlinuz/arm",
        32 => "mach/arm",
        33 => "dylib/i386",
        34 => "mach/amd64",
        35 => "dylib/amd64",
        36 => "win64/pe",
        37 => "mach/arm64",
        39 => "linux/ppc64le",
        40 => "vmlinux/ppc64le",
        42 => "linux/arm64",
        43 => "win64/pe-arm64",
        45 => "linux/riscv64",
        46 => "cpm86/cmd",
        129 => "atari/tos",
        131 => "mach/ppc32",
        132 => "linux/ppc32",
        133 => "linux/armeb",
        134 => "mach/fat",
        135 => "vmlinux/armeb",
        136 => "vmlinux/ppc32",
        137 => "linux/mips",
        138 => "dylib/ppc32",
        139 => "mach/ppc64",
        140 => "linux/ppc64",
        141 => "vmlinux/ppc64",
        142 => "dylib/ppc64",
        _ => "unknown",
    }
}

/// `M_*` names, from UPX's `src/conf.h`.
pub fn method_name(method: u8) -> &'static str {
    match method {
        2 => "NRV2B_LE32",
        3 => "NRV2B_8",
        4 => "NRV2B_LE16",
        5 => "NRV2D_LE32",
        6 => "NRV2D_8",
        7 => "NRV2D_LE16",
        8 => "NRV2E_LE32",
        9 => "NRV2E_8",
        10 => "NRV2E_LE16",
        11 => "CL1B_LE32",
        12 => "CL1B_8",
        13 => "CL1B_LE16",
        14 => "LZMA",
        15 => "DEFLATE",
        16 => "ZSTD",
        17 => "BZIP2",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vendored UPX 3.96 witness (see its `.provenance` sidecar): the same
    /// binary that made `kuna functions` report zero functions.
    fn fixture() -> Vec<u8> {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/upx_packed_x86_64");
        std::fs::read(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
    }

    #[test]
    fn detects_the_vendored_witness() {
        let info = detect(&fixture()).expect("fixture is UPX-packed");
        assert_eq!(info.pack_header_offset, 0x2c40);
        assert_eq!(info.version, 13);
        assert_eq!(info.format, 22);
        assert_eq!(info.format_name(), "linux/amd64");
        assert_eq!(info.method, 2);
        assert_eq!(info.method_name(), "NRV2B_LE32");
        assert_eq!(info.level, 8);
        assert_eq!(info.filter, 0x49);
        assert_eq!(info.u_file_size, 31640);
        assert_eq!(info.overlay_offset, 0xf4);
    }

    /// The end-to-end contract: the recovered image is a complete, well-formed
    /// ELF -- section headers, symbols and all -- not merely a file of the right
    /// length.
    #[test]
    fn unpacks_the_vendored_witness_to_a_usable_elf() {
        let out = unpack(&fixture()).expect("fixture unpacks");
        assert_eq!(out.bytes.len(), 31640);
        assert_eq!(&out.bytes[..4], b"\x7fELF");
        assert_eq!(out.blocks.len(), 9);
        assert_eq!(out.compressed_bytes(), 8564);

        // e_type stays ET_DYN, e_machine stays EM_X86_64, and unlike the packed
        // stub the recovered file has a real section header table.
        assert_eq!(u16::from_le_bytes([out.bytes[16], out.bytes[17]]), 3);
        assert_eq!(u16::from_le_bytes([out.bytes[18], out.bytes[19]]), 0x3e);
        let e_shoff = u64::from_le_bytes(out.bytes[40..48].try_into().unwrap());
        let e_shnum = u16::from_le_bytes([out.bytes[60], out.bytes[61]]);
        assert_eq!(e_shoff, 29656);
        assert_eq!(e_shnum, 31);

        // The one block UPX filtered is the executable segment; unreversing it
        // would leave a file that still parses but whose calls all point wrong,
        // so assert the filter actually ran on it.
        let filtered: Vec<&Block> = out.blocks.iter().filter(|b| b.filter != 0).collect();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].filter, 0x49);
        // `_start` at 0x2580 begins `endbr64; xor ebp,ebp` and its `call *rip+..`
        // resolves only if the ctok unfilter ran.
        assert_eq!(&out.bytes[0x2580..0x2586], &[0xf3, 0x0f, 0x1e, 0xfa, 0x31, 0xed]);
        assert_eq!(&out.bytes[0x2598..0x259f], &[0x48, 0x8d, 0x3d, 0xb5, 0x24, 0x00, 0x00]);
    }

    /// A one-byte corruption inside the compressed stream must be refused, not
    /// silently turned into a plausible-looking image. `0x700` is a literal byte
    /// of the second block: flipping it changes exactly one output byte and
    /// leaves every size intact, so only the packer's Adler-32 can catch it --
    /// which is why the unpacker verifies it.
    #[test]
    fn a_corrupted_literal_is_caught_by_the_checksum() {
        let mut bytes = fixture();
        bytes[0x700] ^= 0xff;
        let err = unpack(&bytes).expect_err("a corrupted stream must not unpack");
        assert!(format!("{err}").contains("checksum mismatch"), "{err}");
    }

    /// A corrupt block header is caught earlier, by structure.
    #[test]
    fn a_corrupted_block_header_is_refused() {
        let mut bytes = fixture();
        bytes[0x1ee] ^= 0x40; // second block's sz_unc
        assert!(unpack(&bytes).is_err());
    }

    #[test]
    fn an_unpacked_file_is_not_packed() {
        assert!(detect(b"\x7fELF not packed at all, no magic here").is_none());
        assert_eq!(unpack(b"\x7fELF short").err(), Some(UpxError::NotPacked));
    }

    #[test]
    fn a_bare_magic_without_a_valid_checksum_is_not_a_pack_header() {
        let mut buf = vec![0u8; 256];
        buf[100..104].copy_from_slice(b"UPX!");
        buf[104] = 13;
        buf[105] = 22;
        buf[131] = 0xff; // deliberately wrong header checksum
        assert!(detect(&buf).is_none());
    }

    #[test]
    fn adler32_matches_the_rfc1950_vector() {
        assert_eq!(adler32(1, b""), 1);
        assert_eq!(adler32(1, b"Wikipedia"), 0x11E6_0398);
        // Chunked accumulation must equal the one-shot value.
        assert_eq!(adler32(adler32(1, b"Wiki"), b"pedia"), 0x11E6_0398);
    }

    #[test]
    fn unknown_ids_report_as_unknown_rather_than_a_neighbour() {
        assert_eq!(format_name(200), "unknown");
        assert_eq!(method_name(99), "unknown");
        assert_eq!(format_name(22), "linux/amd64");
        assert_eq!(method_name(2), "NRV2B_LE32");
        assert_eq!(method_name(14), "LZMA");
    }
}
