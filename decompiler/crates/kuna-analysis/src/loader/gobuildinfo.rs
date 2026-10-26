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
//! older toolchains used is deliberately not parsed: those binaries predate
//! the register ABI, so nothing downstream would consume the answer.

use object::read::{Object, ObjectSection};

/// Section names the build-info blob lives under, per format.
const BUILDINFO_SECTIONS: [&str; 2] = [".go.buildinfo", "__go_buildinfo"];

/// The build-info magic (14 bytes).
const MAGIC: &[u8; 14] = b"\xff Go buildinf:";

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

/// The Go toolchain version that built `bytes`, if it is a Go image with an
/// inline-strings build-info blob. Pure and total over arbitrary bytes.
pub fn go_toolchain_version(bytes: &[u8]) -> Option<(u16, u16)> {
    let file = object::File::parse(bytes).ok()?;
    for name in BUILDINFO_SECTIONS {
        if let Some(data) = file.section_by_name(name).and_then(|s| s.data().ok()) {
            if let Some(v) = version_from_blob(data) {
                return Some(v);
            }
        }
    }
    None
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
