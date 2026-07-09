//! PE CodeView debug-record extractor — the kuna port of Ghidra's
//! `PdbInfoCodeView` (the NB10 record) and `PdbInfoDotNet` (the RSDS record),
//! `Ghidra/Features/Base/.../format/pdb/PdbInfo{CodeView,DotNet}.java`.
//!
//! ## What this is (and is not)
//!
//! A linked PE that was built with debug info carries — in its
//! `IMAGE_DIRECTORY_ENTRY_DEBUG` data directory — an `IMAGE_DEBUG_TYPE_CODEVIEW`
//! entry whose payload is **not** the debug info itself but a tiny *fingerprint*:
//! a magic, a signature/GUID + age, and the path to the external `.pdb` file that
//! actually holds the names and types. This module decodes that fingerprint. It
//! is the PE analog of the ELF CodeView/`.gnu_debuglink` pointer — the lone
//! external-file case in the metadata tier (the PE references the debug info; it
//! does not contain it).
//!
//! Two record formats exist in the wild and both are decoded here:
//!
//! - **RSDS** (`PdbInfoDotNet`) — the modern form: `RSDS` magic, a 16-byte
//!   `GUID`, a `u32` age, then the NUL-terminated `.pdb` path.
//! - **NB10** (`PdbInfoCodeView`) — the older form: `NB10` magic, a `u32` offset
//!   (always 0), a `u32` signature, a `u32` age, then the path.
//!
//! ## Scope (PR-P0)
//!
//! This PR ships **only** the extractor + its test: given a PE image, return the
//! CodeView record ([`CodeViewInfo`]) or `None`. It registers no analysis pass,
//! adds no `--option`, and pulls in no `pdb` crate — those land in the
//! PR-P1 `.pdb`-consuming pass (which calls this `pub fn` to learn which `.pdb`
//! to open and which GUID/age to gate on). Because there is no pass and no
//! settable, this module cannot perturb the parity oracles.
//!
//! ## Faithfulness
//!
//! The two record layouts and their field order match Ghidra's `read()` methods
//! byte-for-byte (`PdbInfoDotNet.read`: magic[4], guid[16], age:u32, path;
//! `PdbInfoCodeView.read`: magic[4], offset:u32, sig:u32, age:u32, path). The
//! magic is matched as the raw four ASCII bytes (Ghidra's big-endian `MAGIC`
//! compare is the same four bytes in stream order). Everything is total: a
//! non-PE input, a PE with no debug directory, no CodeView entry, an unknown
//! magic, or a truncated record all yield `None` — never a panic, never an error.

use object::pe::{ImageDebugDirectory, IMAGE_DIRECTORY_ENTRY_DEBUG};
use object::pe::{ImageNtHeaders32, ImageNtHeaders64, IMAGE_DEBUG_TYPE_CODEVIEW};
use object::read::pe::{ImageNtHeaders, PeFile, PeFile32, PeFile64};
use object::{pod, FileKind, LittleEndian as LE};

/// A decoded PE CodeView debug record — the fingerprint linking a PE to its
/// external `.pdb`. The two variants mirror Ghidra's `PdbInfoDotNet` (RSDS) and
/// `PdbInfoCodeView` (NB10); the PR-P1 pass uses the GUID/sig + age to gate a
/// supplied `.pdb` (the "don't apply the wrong debug file" check) and the path as
/// the tier-1/tier-2 sidecar hint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodeViewInfo {
    /// The modern GUID-keyed record (`RSDS` magic) — `PdbInfoDotNet`.
    Rsds {
        /// The 16 raw GUID bytes, exactly as stored (a Microsoft mixed-endian
        /// GUID: `{u32 LE, u16 LE, u16 LE, [u8;8]}`). Kept raw so the PR-P1 gate
        /// can byte-compare against the `.pdb`'s own GUID; [`guid_string`] renders
        /// the canonical `XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX` text form.
        ///
        /// [`guid_string`]: CodeViewInfo::guid_string
        guid: [u8; 16],
        /// The PDB age (a small monotonically-increasing rebuild counter).
        age: u32,
        /// The recorded path to the external `.pdb` (as the linker wrote it —
        /// often just a filename, sometimes an absolute build path).
        pdb_path: String,
    },
    /// The older 32-bit-signature record (`NB10` magic) — `PdbInfoCodeView`.
    Nb10 {
        /// The 32-bit PDB signature (a timestamp-style hash; the NB10 analog of
        /// the RSDS GUID).
        signature: u32,
        /// The PDB age.
        age: u32,
        /// The recorded path to the external `.pdb`.
        pdb_path: String,
    },
}

impl CodeViewInfo {
    /// The recorded `.pdb` path, common to both variants.
    pub fn pdb_path(&self) -> &str {
        match self {
            CodeViewInfo::Rsds { pdb_path, .. } | CodeViewInfo::Nb10 { pdb_path, .. } => pdb_path,
        }
    }

    /// The PDB age, common to both variants.
    pub fn age(&self) -> u32 {
        match self {
            CodeViewInfo::Rsds { age, .. } | CodeViewInfo::Nb10 { age, .. } => *age,
        }
    }

    /// Render an RSDS GUID in the canonical Microsoft text form
    /// `XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX` (the form `llvm-readobj`/DIA print).
    ///
    /// The 16 raw bytes are a mixed-endian `GUID` struct (Ghidra's
    /// `ghidra.app.util.datatype.microsoft.GUID`): `data1` (u32) and `data2`/
    /// `data3` (u16) are little-endian, `data4[8]` is byte-order. Returns `None`
    /// for an NB10 record (which has no GUID).
    pub fn guid_string(&self) -> Option<String> {
        let g = match self {
            CodeViewInfo::Rsds { guid, .. } => guid,
            CodeViewInfo::Nb10 { .. } => return None,
        };
        let d1 = u32::from_le_bytes([g[0], g[1], g[2], g[3]]);
        let d2 = u16::from_le_bytes([g[4], g[5]]);
        let d3 = u16::from_le_bytes([g[6], g[7]]);
        Some(format!(
            "{d1:08X}-{d2:04X}-{d3:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            g[8], g[9], g[10], g[11], g[12], g[13], g[14], g[15]
        ))
    }
}

/// Extract the PE CodeView debug record (RSDS / NB10) from a parsed PE image.
///
/// Walks `IMAGE_DIRECTORY_ENTRY_DEBUG`, finds the first
/// `IMAGE_DEBUG_TYPE_CODEVIEW` entry, and decodes its payload. Returns `None`
/// when the input is not a PE, carries no debug directory, has no CodeView entry,
/// or the record's magic is neither `RSDS` nor `NB10` (or is truncated). Never
/// panics, never errors — the PR-P1 pass treats `None` as "no PDB fingerprint".
///
/// `bytes` is the raw image: the typed `PeFile32`/`PeFile64` parsers expose the
/// data-directory + section table the neutral `object::File` view does not, so
/// the caller passes the bytes (mirroring [`crate::loader::pe_iat`]).
pub fn extract_codeview(bytes: &[u8]) -> Option<CodeViewInfo> {
    // Branch on the concrete PE width. A `FileKind` we do not recognize (not a
    // PE) or a parse failure yields `None` — never an error.
    match FileKind::parse(bytes) {
        Ok(FileKind::Pe64) => PeFile64::parse(bytes)
            .ok()
            .and_then(|pe| codeview_of::<ImageNtHeaders64>(&pe)),
        Ok(FileKind::Pe32) => PeFile32::parse(bytes)
            .ok()
            .and_then(|pe| codeview_of::<ImageNtHeaders32>(&pe)),
        _ => None,
    }
}

/// Find + decode the CodeView entry of an already-typed `PeFile` of either width.
fn codeview_of<Pe: ImageNtHeaders>(pe: &PeFile<Pe>) -> Option<CodeViewInfo> {
    // The debug data directory (entry 6). `data_directory` returns `None` for a
    // missing entry or a zero virtual address.
    let dir = pe.data_directory(IMAGE_DIRECTORY_ENTRY_DEBUG)?;
    // The directory points at an array of `ImageDebugDirectory` records (RVA +
    // size); `data` resolves it to the file bytes via the section table.
    let debug_bytes = dir.data(pe.data(), &pe.section_table()).ok()?;
    let debug_dirs = pod::slice_from_all_bytes::<ImageDebugDirectory>(debug_bytes).ok()?;

    for entry in debug_dirs {
        if entry.typ.get(LE) != IMAGE_DEBUG_TYPE_CODEVIEW {
            continue;
        }
        // The CodeView payload lives at `pointer_to_raw_data` (a file offset),
        // `size_of_data` bytes long. Read it straight out of the raw image — the
        // payload is in the file even when the debug directory is not mapped to a
        // section the RVA resolver can reach.
        let off = entry.pointer_to_raw_data.get(LE) as usize;
        let size = entry.size_of_data.get(LE) as usize;
        let record = pe.data().get(off..off.checked_add(size)?)?;
        if let Some(info) = parse_record(record) {
            return Some(info);
        }
        // A CodeView entry whose payload we cannot decode (unknown magic /
        // truncated) — keep scanning; a later entry might be the real one.
    }
    None
}

/// Decode one CodeView record payload (the bytes at `pointer_to_raw_data`).
///
/// Ports `PdbInfoDotNet.read` (RSDS) and `PdbInfoCodeView.read` (NB10): dispatch
/// on the four magic bytes, then read the fixed fields little-endian and the
/// NUL-terminated ASCII path. `None` on an unknown magic or a record too short to
/// hold its fixed fields — the boundary the PR-P1 pass treats as "no fingerprint".
fn parse_record(rec: &[u8]) -> Option<CodeViewInfo> {
    let magic = rec.get(0..4)?;
    match magic {
        b"RSDS" => {
            // magic[4], guid[16], age:u32, path (PdbInfoDotNet).
            let guid: [u8; 16] = rec.get(4..20)?.try_into().ok()?;
            let age = read_u32_le(rec, 20)?;
            let pdb_path = read_c_string(rec, 24);
            Some(CodeViewInfo::Rsds { guid, age, pdb_path })
        }
        b"NB10" => {
            // magic[4], offset:u32 (always 0), sig:u32, age:u32, path
            // (PdbInfoCodeView). The `offset` field is read past, not stored.
            let signature = read_u32_le(rec, 8)?;
            let age = read_u32_le(rec, 12)?;
            let pdb_path = read_c_string(rec, 16);
            Some(CodeViewInfo::Nb10 { signature, age, pdb_path })
        }
        _ => None,
    }
}

/// Read a little-endian `u32` at `off`, or `None` if the slice is too short.
fn read_u32_le(buf: &[u8], off: usize) -> Option<u32> {
    let b: [u8; 4] = buf.get(off..off.checked_add(4)?)?.try_into().ok()?;
    Some(u32::from_le_bytes(b))
}

/// Read the NUL-terminated ASCII path starting at `off` (lossy-UTF8 of the bytes
/// up to the first NUL, or to end-of-buffer if unterminated). Mirrors Ghidra's
/// `readNextAsciiString`. Returns an empty string when `off` is past the end.
fn read_c_string(buf: &[u8], off: usize) -> String {
    let tail = match buf.get(off..) {
        Some(t) => t,
        None => return String::new(),
    };
    let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
    String::from_utf8_lossy(&tail[..end]).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `parse_record` decodes a hand-built RSDS payload (the exact field order
    /// Ghidra's `PdbInfoDotNet.read` reads) and `guid_string` renders the
    /// canonical Microsoft text form.
    #[test]
    fn parse_rsds_record() {
        let mut rec = Vec::new();
        rec.extend_from_slice(b"RSDS");
        let guid: [u8; 16] = [
            0x63, 0x39, 0xAC, 0x61, 0x48, 0xFF, 0x24, 0x90, 0x4C, 0x4C, 0x44, 0x20, 0x50, 0x44,
            0x42, 0x2E,
        ];
        rec.extend_from_slice(&guid);
        rec.extend_from_slice(&7u32.to_le_bytes());
        rec.extend_from_slice(b"foo.pdb\0trailing-garbage");

        let info = parse_record(&rec).expect("RSDS decodes");
        assert_eq!(
            info,
            CodeViewInfo::Rsds { guid, age: 7, pdb_path: "foo.pdb".to_string() }
        );
        assert_eq!(info.age(), 7);
        assert_eq!(info.pdb_path(), "foo.pdb");
        // Microsoft mixed-endian render: data1/2/3 LE, data4 byte-order.
        assert_eq!(info.guid_string().as_deref(), Some("61AC3963-FF48-9024-4C4C-44205044422E"));
    }

    /// `parse_record` decodes a hand-built NB10 payload (magic, offset:u32,
    /// sig:u32, age:u32, path — Ghidra's `PdbInfoCodeView.read`), skipping the
    /// `offset` field.
    #[test]
    fn parse_nb10_record() {
        let mut rec = Vec::new();
        rec.extend_from_slice(b"NB10");
        rec.extend_from_slice(&0u32.to_le_bytes()); // offset (always 0)
        rec.extend_from_slice(&0xDEADBEEFu32.to_le_bytes()); // signature
        rec.extend_from_slice(&3u32.to_le_bytes()); // age
        rec.extend_from_slice(b"bar.pdb\0");

        let info = parse_record(&rec).expect("NB10 decodes");
        assert_eq!(
            info,
            CodeViewInfo::Nb10 { signature: 0xDEAD_BEEF, age: 3, pdb_path: "bar.pdb".to_string() }
        );
        // NB10 has no GUID.
        assert_eq!(info.guid_string(), None);
    }

    /// An unknown magic / a truncated record yields `None` (never a panic).
    #[test]
    fn parse_rejects_unknown_and_truncated() {
        assert_eq!(parse_record(b"NOPE12345678"), None);
        assert_eq!(parse_record(b"RS"), None); // shorter than the magic
        assert_eq!(parse_record(b"RSDS\x01\x02"), None); // RSDS but no room for the GUID
        assert_eq!(parse_record(&[]), None);
    }

    /// A non-PE byte blob yields `None` from the top-level extractor.
    #[test]
    fn extract_non_pe_is_none() {
        assert_eq!(extract_codeview(b"\x7fELF not a PE at all"), None);
        assert_eq!(extract_codeview(&[]), None);
    }

    /// End-to-end over the real `pdb_min.exe` fixture (a freestanding PE built
    /// `clang -target x86_64-pc-windows-msvc -g -gcodeview -fuse-ld=lld`, so
    /// lld-link wrote an RSDS CodeView record + the `.pdb` path into the debug
    /// directory). The extractor must pull the exact `{guid, age, pdb_path}`
    /// `llvm-readobj --coff-debug-directory pdb_min.exe` reports:
    ///   GUID  (raw) = 63 39 AC 61 48 FF 24 90 4C 4C 44 20 50 44 42 2E
    ///   Age   = 1
    ///   PDB   = pdb_min.pdb
    /// (`docs/`-pinned in tests/fixtures/README.md). This proves the directory
    /// walk + RSDS decode on a genuine MSVC-ABI CodeView record, not a hand-blob.
    #[test]
    fn extract_pdb_min_exe_rsds_record() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/pdb_min.exe");
        let bytes = std::fs::read(path).expect("read pdb_min.exe fixture");
        let info = extract_codeview(&bytes).expect("pdb_min.exe carries a CodeView RSDS record");

        let expected_guid: [u8; 16] = [
            0x63, 0x39, 0xAC, 0x61, 0x48, 0xFF, 0x24, 0x90, 0x4C, 0x4C, 0x44, 0x20, 0x50, 0x44,
            0x42, 0x2E,
        ];
        assert_eq!(
            info,
            CodeViewInfo::Rsds {
                guid: expected_guid,
                age: 1,
                pdb_path: "pdb_min.pdb".to_string(),
            },
            "extracted RSDS record must match llvm-readobj"
        );
        // The canonical text GUID (the form llvm-readobj/DIA print) the PR-P1 gate
        // would log.
        assert_eq!(
            info.guid_string().as_deref(),
            Some("61AC3963-FF48-9024-4C4C-44205044422E")
        );
    }
}
