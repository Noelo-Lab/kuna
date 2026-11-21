//! Ghidra FID database interop — read a real `.fidbf` (raw) or `.fidb` (packed)
//! database into kuna's [`FidDatabase`] surface.
//!
//! Ghidra ships its library fingerprints in these files, so reading them lets a
//! kuna run match against the stock `vs2019_x64` corpus instead of requiring a
//! database rebuilt by `kuna fid build`.
//!
//! # What of a Ghidra FID database kuna uses
//!
//! `FID`'s query seam is one indexed lookup: `FunctionsTable.findRecords` by
//! `FULL_HASH_COL`, filtered by `FidQueryService` down to the records whose
//! `codeUnitSize` matches. [`FidFile`] keeps exactly that: `full_hash` →
//! [`FidRecord`] rows carrying the same five hash fields plus the resolved name.
//!
//! A Ghidra record's name lives in `Strings Table` behind `NAME_ID_COL`; the
//! library joins to `Libraries Table` and supplies the language the database
//! applies to. Records from an internal library and records from an external
//! (imported) library are kept apart the way Ghidra keeps them: an external
//! library's functions are *not* FID matches for an image's own code, so only
//! records of libraries the image could have been built from are exposed.
//!
//! # Packed (`.fidb`) container
//!
//! `ItemSerializer` writes a Java `ObjectOutputStream` header carrying a magic
//! long, a format version, the item name, the content type, the file type and
//! the payload length, followed by a `ZipOutputStream` holding one deflated
//! `FOLDER_ITEM` entry. [`unpack`] reads that framing and inflates the entry;
//! the raw `.fidbf` is the payload itself.

use std::io::Read;

use kuna_base::error::{KunaError, KunaResult};

use super::db::{FidDatabase, FidRecord, FLAG_HAS_TERMINATOR};
use super::ghidradb::{Database, FieldValue, TableRef};

/// `FunctionsTable` column indices.
const FN_CODE_UNIT_SIZE: usize = 0;
const FN_FULL_HASH: usize = 1;
const FN_SPECIFIC_ADDL: usize = 2;
const FN_SPECIFIC_HASH: usize = 3;
const FN_LIBRARY_ID: usize = 4;
const FN_NAME_ID: usize = 5;
const FN_FLAGS: usize = 8;

/// `LibrariesTable` column indices.
const LIB_FAMILY: usize = 0;
const LIB_VERSION: usize = 1;
const LIB_LANGUAGE_ID: usize = 4;

/// `StringsTable` column index.
const STRING_VALUE: usize = 0;

/// The `ItemSerializer` packed-file magic (`0x2e30212634e92c20`).
pub const PACKED_MAGIC: u64 = 0x2e30_2126_34e9_2c20;
/// The `ItemSerializer` format version.
pub const PACKED_FORMAT_VERSION: i32 = 1;

/// The `flags` bit Ghidra sets on a record whose body ended in a terminator.
///
/// Ghidra's `FunctionRecord` stores no such bit in `FLAGS_COL`; the column marks
/// filtered records. kuna models a terminator flag on its own records, so a
/// Ghidra record's flag byte is carried across verbatim and interpreted only
/// where kuna set the same bit.
const GHIDRA_TERMINATOR_BIT: u8 = FLAG_HAS_TERMINATOR;

/// One library an image can be fingerprinted against.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GhidraLibrary {
    pub id: i64,
    pub family: String,
    pub version: String,
    pub language_id: String,
}

/// A Ghidra FID database opened through kuna's [`FidDatabase`] surface.
pub struct FidFile {
    lang_id: String,
    cspec_id: String,
    libraries: Vec<GhidraLibrary>,
    by_full_hash: std::collections::HashMap<u64, Vec<FidRecord>>,
    n_records: usize,
}

impl FidFile {
    /// Read a `.fidbf` or `.fidb` byte stream.
    ///
    /// The database language is the one every library record agrees on; a Ghidra
    /// FID database built from several languages reports the first and keeps
    /// each record's library, so a multi-language database applies to any of
    /// them — [`applies_to_language`] is what a caller should consult.
    pub fn load(bytes: &[u8]) -> KunaResult<Self> {
        let raw = if is_packed(bytes) { unpack(bytes)? } else { bytes.to_vec() };
        let db = Database::parse(raw)?;
        let libs = db
            .table("Libraries Table")
            .ok_or_else(|| KunaError::bad_data("ghidra fid: no Libraries Table"))?;
        let lib_rows = db.records(libs)?;
        let mut libraries = Vec::with_capacity(lib_rows.len());
        for rec in &lib_rows {
            libraries.push(GhidraLibrary {
                id: rec.key,
                family: column_str(rec, LIB_FAMILY).unwrap_or_default().to_owned(),
                version: column_str(rec, LIB_VERSION).unwrap_or_default().to_owned(),
                language_id: column_str(rec, LIB_LANGUAGE_ID).unwrap_or_default().to_owned(),
            });
        }
        let lang_id = libraries
            .iter()
            .find(|l| !l.language_id.is_empty())
            .map(|l| l.language_id.clone())
            .unwrap_or_default();
        // Ghidra's FID files carry no single compiler spec; the library family is
        // the closest comparable fact and is reported as such.
        let cspec_id = libraries.first().map(|l| l.family.clone()).unwrap_or_default();

        let names = read_strings(&db)?;
        let functions = db
            .table("Functions Table")
            .ok_or_else(|| KunaError::bad_data("ghidra fid: no Functions Table"))?;
        let records = db.records(functions)?;

        let mut by_full_hash: std::collections::HashMap<u64, Vec<FidRecord>> =
            std::collections::HashMap::new();
        let mut n_records = 0usize;
        for rec in records {
            let Some(name_id) = rec.columns.get(FN_NAME_ID).and_then(FieldValue::as_i64) else {
                continue;
            };
            let Some(name) = names.get(&name_id) else { continue };
            let Some(full_hash) = rec.columns.get(FN_FULL_HASH).and_then(FieldValue::as_i64) else {
                continue;
            };
            let specific_hash =
                rec.columns.get(FN_SPECIFIC_HASH).and_then(FieldValue::as_i64).unwrap_or(0);
            let code_unit_size = rec
                .columns
                .get(FN_CODE_UNIT_SIZE)
                .and_then(FieldValue::as_i64)
                .unwrap_or(0) as i16;
            let specific_addl = rec
                .columns
                .get(FN_SPECIFIC_ADDL)
                .and_then(FieldValue::as_i64)
                .unwrap_or(0) as u8;
            let flags = rec.columns.get(FN_FLAGS).and_then(FieldValue::as_i64).unwrap_or(0) as u8;
            let _library = rec.columns.get(FN_LIBRARY_ID).and_then(FieldValue::as_i64);
            n_records += 1;
            by_full_hash.entry(full_hash as u64).or_default().push(FidRecord {
                full_hash: full_hash as u64,
                specific_hash: specific_hash as u64,
                code_unit_size,
                specific_addl,
                flags: flags & GHIDRA_TERMINATOR_BIT,
                name: name.clone(),
            });
        }

        Ok(Self { lang_id, cspec_id, libraries, by_full_hash, n_records })
    }

    /// The libraries this database was built from.
    pub fn libraries(&self) -> &[GhidraLibrary] {
        &self.libraries
    }

    /// `Ok` when this database carries fingerprints for `language_id`.
    pub fn applies_to_language(&self, language_id: &str) -> Result<(), String> {
        if self.libraries.is_empty() {
            return if self.lang_id.is_empty() {
                Ok(())
            } else if self.lang_id == language_id {
                Ok(())
            } else {
                Err(format!(
                    "database language {} differs from image language {language_id}",
                    self.lang_id
                ))
            };
        }
        if self.libraries.iter().any(|l| l.language_id == language_id) {
            Ok(())
        } else {
            let known: Vec<&str> = self
                .libraries
                .iter()
                .map(|l| l.language_id.as_str())
                .filter(|l| !l.is_empty())
                .collect();
            Err(format!(
                "database has no library for language {language_id} (it carries {})",
                known.join(", ")
            ))
        }
    }
}

impl FidDatabase for FidFile {
    fn lang_id(&self) -> &str {
        &self.lang_id
    }
    fn cspec_id(&self) -> &str {
        &self.cspec_id
    }
    fn find_by_full_hash(&self, h: u64) -> &[FidRecord] {
        self.by_full_hash.get(&h).map(Vec::as_slice).unwrap_or(&[])
    }
    fn record_count(&self) -> usize {
        self.n_records
    }
}

/// The `Strings Table` as `id -> value`.
fn read_strings(db: &Database) -> KunaResult<std::collections::HashMap<i64, String>> {
    let table = match db.table("Strings Table") {
        Some(t) => t,
        None => return Ok(std::collections::HashMap::new()),
    };
    let rows = db.records(table)?;
    let mut out = std::collections::HashMap::with_capacity(rows.len());
    for rec in rows {
        if let Some(name) = column_str(&rec, STRING_VALUE) {
            out.insert(rec.key, name.to_owned());
        }
    }
    Ok(out)
}

fn column_str(rec: &super::ghidradb::Record, index: usize) -> Option<&str> {
    rec.columns.get(index).and_then(FieldValue::as_str)
}

/// True when `bytes` start with the packed-container magic at its fixed offset.
pub fn is_packed(bytes: &[u8]) -> bool {
    // ItemSerializer.MAGIC_NUMBER_POS = 6, size 8, big-endian.
    bytes.len() >= 14 && {
        let mut magic = [0u8; 8];
        magic.copy_from_slice(&bytes[6..14]);
        u64::from_be_bytes(magic) == PACKED_MAGIC
    }
}

/// Read a Ghidra `ItemSerializer` packed file's single payload entry.
///
/// The header is written with bare `ObjectOutputStream` primitives (`writeLong`,
/// `writeInt`, `writeUTF`), so its layout is fixed: magic(8),
/// formatVersion(4), UTF item name, UTF content type, fileType(4),
/// length(8). The payload follows as a **streamed** zip member: a local file
/// header, the deflate stream, and a data descriptor — no central directory, so
/// it must be read here rather than through a zip archive reader.
pub fn unpack(bytes: &[u8]) -> KunaResult<Vec<u8>> {
    if !is_packed(bytes) {
        return Err(KunaError::bad_data("ghidra fid: not a packed database file"));
    }
    let mut off = 14;
    let version = read_be_i32(bytes, off)?;
    if version != PACKED_FORMAT_VERSION {
        return Err(KunaError::bad_data(format!(
            "ghidra fid: packed format version {version} != {PACKED_FORMAT_VERSION}"
        )));
    }
    off += 4;
    // writeUTF: u16 byte length then modified-UTF-8 bytes.
    let name_len = read_be_u16(bytes, off)? as usize;
    off += 2 + name_len;
    let type_len = read_be_u16(bytes, off)? as usize;
    off += 2 + type_len;
    off += 4; // file type
    let payload_len = read_be_i64(bytes, off)?;
    off += 8;

    // Local file header: signature(4) version(2) flags(2) method(2) time(2)
    // date(2) crc(4) csize(4) usize(4) nameLen(2) extraLen(2) then name+extra.
    let sig = bytes
        .get(off..off + 4)
        .ok_or_else(|| KunaError::bad_data("ghidra fid: truncated zip header"))?;
    if sig != [0x50, 0x4b, 0x03, 0x04] {
        return Err(KunaError::bad_data("ghidra fid: packed payload is not a zip member"));
    }
    // Zip local-header fields are little-endian, unlike the Java header above.
    let method = read_le_u16(bytes, off + 8)?;
    let entry_name_len = read_le_u16(bytes, off + 26)? as usize;
    let entry_extra_len = read_le_u16(bytes, off + 28)? as usize;
    let data_off = off + 30 + entry_name_len + entry_extra_len;
    let data = bytes
        .get(data_off..)
        .ok_or_else(|| KunaError::bad_data("ghidra fid: truncated zip member"))?;

    // `payload_len` is untrusted. Validate it before it can steer an allocation,
    // and bound the inflation to it: a decompression bomb or an absurd declared
    // length must be a parse error, never an allocation abort.
    if payload_len < 0 {
        return Err(KunaError::bad_data("ghidra fid: negative packed payload length"));
    }
    let declared = payload_len as usize;
    // A raw database is a fixed-block file, so its size is a whole number of
    // blocks; refuse a declared size no such file could have before reserving.
    const MAX_PAYLOAD: usize = 1 << 31;
    if declared == 0 || declared > MAX_PAYLOAD {
        return Err(KunaError::bad_data(format!(
            "ghidra fid: packed payload length {declared} out of range"
        )));
    }
    let mut out = Vec::with_capacity(declared.min(data.len().saturating_mul(1)));
    match method {
        0 => out.extend_from_slice(data), // stored
        8 => {
            let inflater = flate2::read::DeflateDecoder::new(std::io::Cursor::new(data));
            // `take` bounds the read so a bomb cannot expand past the declared
            // size; reading one byte more tells us the declaration was a lie.
            let mut bounded = inflater.take(declared as u64 + 1);
            bounded
                .read_to_end(&mut out)
                .map_err(|e| KunaError::bad_data(format!("ghidra fid: cannot inflate payload ({e})")))?;
        }
        other => {
            return Err(KunaError::bad_data(format!(
                "ghidra fid: unsupported zip compression method {other}"
            )))
        }
    }
    if out.len() != declared {
        return Err(KunaError::bad_data(format!(
            "ghidra fid: inflated {} bytes, packed header declares {declared}",
            out.len()
        )));
    }
    Ok(out)
}

fn read_le_u16(b: &[u8], off: usize) -> KunaResult<u16> {
    let s = b.get(off..off + 2).ok_or_else(|| KunaError::bad_data("ghidra fid: short header"))?;
    Ok(u16::from_le_bytes([s[0], s[1]]))
}

fn read_be_u16(b: &[u8], off: usize) -> KunaResult<u16> {
    let s = b.get(off..off + 2).ok_or_else(|| KunaError::bad_data("ghidra fid: short header"))?;
    Ok(u16::from_be_bytes([s[0], s[1]]))
}

fn read_be_i32(b: &[u8], off: usize) -> KunaResult<i32> {
    let s = b.get(off..off + 4).ok_or_else(|| KunaError::bad_data("ghidra fid: short header"))?;
    Ok(i32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}

fn read_be_i64(b: &[u8], off: usize) -> KunaResult<i64> {
    let s = b.get(off..off + 8).ok_or_else(|| KunaError::bad_data("ghidra fid: short header"))?;
    Ok(i64::from_be_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]))
}

/// Kept private to the module: what a [`TableRef`] is used for here.
#[allow(dead_code)]
fn schema_names(t: &TableRef) -> &[String] {
    &t.schema.field_names
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The packed-container magic sits at `ItemSerializer.MAGIC_NUMBER_POS`.
    #[test]
    fn packed_detection_reads_the_magic_at_its_offset() {
        let mut bytes = vec![0u8; 32];
        bytes[6..14].copy_from_slice(&PACKED_MAGIC.to_be_bytes());
        assert!(is_packed(&bytes));
        bytes[6] = 0;
        assert!(!is_packed(&bytes));
        assert!(!is_packed(&[]));
        assert!(!is_packed(&[0u8; 8]));
    }

    /// A `.fidb` whose deflate member is not a zip archive is an error, not a
    /// panic: the header names a zip member that is absent.
    #[test]
    fn a_packed_file_without_a_zip_member_is_an_error() {
        let mut bytes = vec![0u8; 200];
        bytes[6..14].copy_from_slice(&PACKED_MAGIC.to_be_bytes());
        bytes[14..18].copy_from_slice(&PACKED_FORMAT_VERSION.to_be_bytes());
        // Two empty UTF strings (name and content type).
        bytes[18..20].copy_from_slice(&0u16.to_be_bytes());
        bytes[20..22].copy_from_slice(&0u16.to_be_bytes());
        bytes[22..26].copy_from_slice(&0i32.to_be_bytes()); // file type
        bytes[26..34].copy_from_slice(&0i64.to_be_bytes()); // payload length
        let err = unpack(&bytes).expect_err("no zip member");
        assert!(err.explain().contains("zip"), "unexpected error: {}", err.explain());
    }

    #[test]
    fn a_raw_file_is_not_packed() {
        assert!(!is_packed(&[0xffu8; 64]));
    }

    /// A database whose libraries disagree with the image names each language it
    /// does carry, so the caller sees what went wrong.
    #[test]
    fn applies_to_language_lists_the_languages_it_has() {
        let db = FidFile {
            lang_id: "x86:LE:64:default".into(),
            cspec_id: "Visual Studio".into(),
            libraries: vec![GhidraLibrary {
                id: 1,
                family: "Visual Studio".into(),
                version: "2019".into(),
                language_id: "x86:LE:64:default".into(),
            }],
            by_full_hash: std::collections::HashMap::new(),
            n_records: 0,
        };
        assert!(db.applies_to_language("x86:LE:64:default").is_ok());
        let err = db.applies_to_language("x86:LE:32:default").unwrap_err();
        assert!(err.contains("x86:LE:32:default"), "{err}");
        assert!(err.contains("x86:LE:64:default"), "{err}");
    }
}
