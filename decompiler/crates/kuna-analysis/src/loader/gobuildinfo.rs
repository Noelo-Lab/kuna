//! (kuna) The Go toolchain version off the `.go.buildinfo` blob.
//!
//! The runtime's own struct layouts drift release to release (go1.23 inserted
//! `g.syscallbp`, go1.25 dropped `gobuf.ret`), so the `goreg` shells must know
//! which release built the image. The version string is embedded in the
//! build-info blob the detection section carries (`debug/buildinfo` format):
//! a 32-byte header — the 14-byte magic `"\xff Go buildinf:"`, a pointer-size
//! byte, a flags byte — then, when flags bit 1 is set (every toolchain since
//! go1.18, the whole register-ABI era), two varint-length-prefixed strings:
//! the version (`"go1.22.10"`) and the modinfo. The pointer-indirection form
//! older toolchains used is not parsed for the version: those binaries predate
//! the register ABI, so nothing downstream would consume the answer.
//!
//! The blob is also how a Go **PE** is recognized at all. Go's linker gives
//! ELF and Mach-O a build-info section of their own, but on Windows it merges
//! the blob into the head of `.data`, so a Go PE carries no Go-named section.
//! [`pe_buildinfo_blob`] finds it the way `debug/buildinfo` (`go version`)
//! does — a 16-byte-aligned magic inside the first writable initialized-data
//! section — and accepts only a blob whose whole header validates
//! ([`blob_is_buildinfo`]), so a stray copy of the magic string is not
//! evidence.

use object::read::{Object, ObjectSection};
use object::SectionFlags;

/// Section names the build-info blob lives under, per format.
const BUILDINFO_SECTIONS: [&str; 2] = [".go.buildinfo", "__go_buildinfo"];

/// The build-info magic (14 bytes).
const MAGIC: &[u8; 14] = b"\xff Go buildinf:";

/// The blob's fixed header size and alignment (`debug/buildinfo`'s
/// `buildInfoHeaderSize` / `buildInfoAlign`).
const HEADER_SIZE: usize = 32;
const ALIGN: u64 = 16;

/// Longest version string a real toolchain stamps (`"go1.22.10"`, `"devel
/// go1.23-0a1b2c3 Tue ..."`, plus any `X:` experiment suffix).
const MAX_VERSION_LEN: usize = 128;

/// Parse `"go<maj>.<min>[...]"` into `(maj, min)`.
fn parse_go_version(s: &str) -> Option<(u16, u16)> {
    let rest = s.strip_prefix("go")?;
    let mut it = rest.split('.');
    let maj: u16 = it.next()?.parse().ok()?;
    let min_tok = it.next()?;
    // A devel or rc suffix stops the digits ("1.25rc1"): take the digit run.
    let digits: String = min_tok.chars().take_while(|c| c.is_ascii_digit()).collect();
    let min: u16 = digits.parse().ok()?;
    Some((maj, min))
}

/// Read a uvarint; returns `(value, len)`.
fn uvarint(b: &[u8]) -> Option<(u64, usize)> {
    let mut x: u64 = 0;
    let mut s = 0u32;
    for (i, &c) in b.iter().take(10).enumerate() {
        if c < 0x80 {
            return Some((x | (u64::from(c) << s), i + 1));
        }
        x |= u64::from(c & 0x7f) << s;
        s += 7;
    }
    None
}

/// The toolchain version from a raw build-info blob (the section's bytes).
pub fn version_from_blob(blob: &[u8]) -> Option<(u16, u16)> {
    if blob.len() < 34 || &blob[..14] != MAGIC {
        return None;
    }
    let flags = blob[15];
    if flags & 0x2 == 0 {
        return None; // pointer-indirection form: pre-go1.18, pre-register-ABI
    }
    let (len, n) = uvarint(&blob[32..])?;
    let start = 32 + n;
    let end = start.checked_add(usize::try_from(len).ok()?)?;
    let s = core::str::from_utf8(blob.get(start..end)?).ok()?;
    parse_go_version(s.trim())
}

/// `true` when `s` reads as a toolchain version string: a release (`go1.x`)
/// or a development build (`devel ...`), printable ASCII throughout.
fn is_version_string(s: &[u8]) -> bool {
    (2..=MAX_VERSION_LEN).contains(&s.len())
        && (s.starts_with(b"go") || s.starts_with(b"devel"))
        && s.iter().all(|&c| c == b' ' || c.is_ascii_graphic())
}

/// `true` when `blob` opens with a complete, well-formed build-info header:
/// the magic, a pointer size of 4 or 8, no unknown flag bits, and a version
/// string that decodes. The inline form (go1.18+) pads the header with zeros
/// and follows it with the varint-prefixed version; the pointer form (older
/// toolchains) points at a Go string header, resolved through `read`.
pub fn blob_is_buildinfo<'a>(blob: &[u8], read: impl FnMut(u64, usize) -> Option<&'a [u8]>) -> bool {
    let mut read = read;
    if blob.len() < HEADER_SIZE || &blob[..14] != MAGIC {
        return false;
    }
    let (ptr_size, flags) = (usize::from(blob[14]), blob[15]);
    if !matches!(ptr_size, 4 | 8) || flags & !0x3 != 0 {
        return false;
    }
    if flags & 0x2 != 0 {
        if blob[16..HEADER_SIZE].iter().any(|&b| b != 0) {
            return false;
        }
        let Some((len, n)) = uvarint(&blob[HEADER_SIZE..]) else { return false };
        let start = HEADER_SIZE + n;
        let end = usize::try_from(len).ok().and_then(|l| start.checked_add(l));
        return end.and_then(|e| blob.get(start..e)).is_some_and(is_version_string);
    }
    let word = |b: &[u8]| {
        let mut w = [0u8; 8];
        if flags & 0x1 != 0 {
            w[8 - ptr_size..].copy_from_slice(&b[..ptr_size]);
            u64::from_be_bytes(w)
        } else {
            w[..ptr_size].copy_from_slice(&b[..ptr_size]);
            u64::from_le_bytes(w)
        }
    };
    let Some(header) = read(word(&blob[16..]), 2 * ptr_size) else { return false };
    let (ptr, len) = (word(header), word(&header[ptr_size..]));
    usize::try_from(len)
        .ok()
        .filter(|&l| l <= MAX_VERSION_LEN)
        .and_then(|l| read(ptr, l))
        .is_some_and(is_version_string)
}

/// The build-info blob of a Go **PE**: the first 16-byte-aligned magic in the
/// first writable initialized-data section whose header validates — the
/// section `debug/buildinfo`'s `peExe.DataStart` reads. The scan touches one
/// byte per aligned slot, and the blob is the section's first object on an
/// internally linked image, so a Go PE answers at once and a non-Go PE costs a
/// strided pass over its `.data`. `None` on any other format.
pub fn pe_buildinfo_blob<'a>(file: &'a object::File<'a>) -> Option<&'a [u8]> {
    use object::pe::{
        IMAGE_SCN_ALIGN_MASK, IMAGE_SCN_CNT_INITIALIZED_DATA, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE,
    };
    if file.format() != object::BinaryFormat::Pe {
        return None;
    }
    let want = IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE;
    let sec = file.sections().find(|s| {
        s.address() != 0
            && s.size() != 0
            && matches!(s.flags(), SectionFlags::Coff { characteristics }
                if characteristics & !IMAGE_SCN_ALIGN_MASK == want)
    })?;
    let data = sec.data().ok()?;
    let lead = ((ALIGN - sec.address() % ALIGN) % ALIGN) as usize;
    let mut img = None;
    (lead..data.len().saturating_sub(HEADER_SIZE - 1))
        .step_by(ALIGN as usize)
        .filter(|&off| data[off] == MAGIC[0] && data[off..].starts_with(MAGIC))
        .map(|off| &data[off..])
        .find(|blob| {
            blob_is_buildinfo(blob, |vma, len| {
                img.get_or_insert_with(|| crate::pclntab::ImageBytes::new(file)).read(vma, len)
            })
        })
}

/// The build-info blob of a Go image in any format: the named section on ELF
/// and Mach-O, the [`pe_buildinfo_blob`] scan on a PE.
pub fn buildinfo_blob<'a>(file: &'a object::File<'a>) -> Option<&'a [u8]> {
    BUILDINFO_SECTIONS
        .iter()
        .find_map(|name| file.section_by_name(name).and_then(|s| s.data().ok()))
        .or_else(|| pe_buildinfo_blob(file))
}

/// The Go toolchain version that built `bytes`, if it is a Go image with an
/// inline-strings build-info blob. Pure and total over arbitrary bytes.
pub fn go_toolchain_version(bytes: &[u8]) -> Option<(u16, u16)> {
    let file = object::File::parse(bytes).ok()?;
    buildinfo_blob(&file).and_then(version_from_blob)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blob(version: &str, flags: u8) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(MAGIC);
        b.push(8); // ptr size
        b.push(flags);
        b.resize(32, 0);
        b.push(u8::try_from(version.len()).unwrap()); // uvarint (short)
        b.extend_from_slice(version.as_bytes());
        b
    }

    fn unmapped(_: u64, _: usize) -> Option<&'static [u8]> {
        None
    }

    #[test]
    fn only_a_complete_header_is_buildinfo() {
        assert!(blob_is_buildinfo(&blob("go1.21.3", 2), unmapped));
        assert!(blob_is_buildinfo(&blob("devel go1.23-0a1b2c3", 2), unmapped));
        let mut ptr4 = blob("go1.22.1", 2);
        ptr4[14] = 4;
        assert!(blob_is_buildinfo(&ptr4, unmapped));

        let bare: Vec<u8> = MAGIC.iter().copied().chain([0u8; 18]).collect();
        assert!(!blob_is_buildinfo(&bare, unmapped), "the magic string alone");
        let mut bad = blob("go1.21.3", 2);
        bad[14] = 3;
        assert!(!blob_is_buildinfo(&bad, unmapped), "pointer size 3");
        let mut bad = blob("go1.21.3", 2);
        bad[15] = 0x12;
        assert!(!blob_is_buildinfo(&bad, unmapped), "unknown flag bits");
        let mut bad = blob("go1.21.3", 2);
        bad[20] = 1;
        assert!(!blob_is_buildinfo(&bad, unmapped), "nonzero header padding");
        assert!(!blob_is_buildinfo(&blob("", 2), unmapped), "empty version");
        assert!(!blob_is_buildinfo(&blob("v1.2.3", 2), unmapped), "not a toolchain version");
        assert!(!blob_is_buildinfo(&blob("go1\x01", 2), unmapped), "control bytes");
        let mut cut = blob("go1.21.3", 2);
        cut.truncate(36);
        assert!(!blob_is_buildinfo(&cut, unmapped), "version runs off the end");
        assert!(!blob_is_buildinfo(&MAGIC[..], unmapped), "short of a header");
    }

    #[test]
    fn pointer_form_resolves_its_version_string() {
        let mut b = Vec::new();
        b.extend_from_slice(MAGIC);
        b.extend_from_slice(&[8, 0]);
        b.extend_from_slice(&0x5000u64.to_le_bytes());
        b.extend_from_slice(&0x6000u64.to_le_bytes());
        let header: Vec<u8> = [0x7000u64.to_le_bytes(), 9u64.to_le_bytes()].concat();
        let text = b"go1.16.15".to_vec();
        let read = |vma: u64, len: usize| match vma {
            0x5000 => header.get(..len),
            0x7000 => text.get(..len),
            _ => None,
        };
        assert!(blob_is_buildinfo(&b, read));
        let dangling = |vma: u64, len: usize| if vma == 0x5000 { header.get(..len) } else { None };
        assert!(!blob_is_buildinfo(&b, dangling), "the version pointer leads nowhere");
        assert!(!blob_is_buildinfo(&b, unmapped));
    }

    #[test]
    fn inline_blob_parses_and_old_forms_refuse() {
        assert_eq!(version_from_blob(&blob("go1.22.10", 2)), Some((1, 22)));
        assert_eq!(version_from_blob(&blob("go1.25.3", 2)), Some((1, 25)));
        assert_eq!(version_from_blob(&blob("go1.25rc1", 2)), Some((1, 25)));
        // pointer-indirection form (pre-1.18) is deliberately unparsed
        assert_eq!(version_from_blob(&blob("go1.16.5", 0)), None);
        // wrong magic / truncation / junk
        assert_eq!(version_from_blob(b"not a blob"), None);
        assert_eq!(version_from_blob(&blob("devel +abc", 2)), None);
        assert_eq!(version_from_blob(&[]), None);
    }

    #[test]
    fn corpus_binaries_state_their_toolchains() {
        // The campaign corpus carries both eras; missing files skip (the
        // corpus is machine-local state, not a repo fixture).
        for (p, want) in [
            ("/home/mahaloz/kwt/multilang/corpus/go/hello", (1u16, 22u16)),
        ] {
            if let Ok(bytes) = std::fs::read(p) {
                assert_eq!(go_toolchain_version(&bytes), Some(want), "{p}");
            }
        }
    }
}
