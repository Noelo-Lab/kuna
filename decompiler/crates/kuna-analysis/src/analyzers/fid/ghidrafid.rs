//! Ghidra FID database interop — read a real `.fidbf` (raw) or `.fidb` (packed)
//! database into kuna's [`FidDatabase`] surface.
//!
//! Ghidra ships its library fingerprints in these files, so reading them lets a
//! kuna run match against the stock `vs2019_x64` corpus instead of requiring a
//! database rebuilt by `kuna fid build`.
//!
//! # What of a Ghidra FID database kuna uses
//!
//! Read the primary function, string and library tables. The function's library
//! determines whether its fingerprint is usable for the image language; only
//! records from compatible libraries are indexed. The full flag byte accompanies
//! each record so automatic naming can enforce Ghidra's restrictions.
//! Both version-6 library schemas are supported. Compiler IDs are descriptive:
//! the newer metadata column can list several or leave them unspecified.
//!
//! # Packed (`.fidb`) container
//!
//! `ItemSerializer` writes a Java `ObjectOutputStream` header carrying a magic
//! long, a format version, the item name, the content type, the file type and
//! the payload length, followed by a `ZipOutputStream` holding one deflated
//! `FOLDER_ITEM` entry. [`unpack`] reads that framing and inflates the entry;
//! the raw `.fidbf` is the payload itself.

#[expect(clippy::disallowed_types, reason = "FID lookup indices; input order is preserved")]
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;

use kuna_base::error::{KunaError, KunaResult};

use super::db::{FidDatabase, FidRecord};
use super::ghidradb::{Database, FieldValue, TableRef};
use super::schema::FieldType::{self, Byte, Int, Long, Short, Str};
use super::schema::Schema;

const SCHEMA_VERSION: i32 = 6;
const LEGACY_COMPILER_COLUMN: &str = "Ghidra Compiler Spec ID";
const METADATA_COLUMN: &str = "Library Metadata";
const LIBRARY_FIELDS: &[(FieldType, &str)] = &[
    (Str, "Library Family Name"),
    (Str, "Library Version"),
    (Str, "Library Variant"),
    (Str, "Ghidra Version"),
    (Str, "Ghidra Language ID"),
    (Int, "Ghidra Language Version"),
    (Int, "Ghidra Language Minor Version"),
];
const FUNCTION_FIELDS: &[(FieldType, &str)] = &[
    (Short, "Code Unit Size"),
    (Long, "Full Hash"),
    (Byte, "Specific Hash Additional Size"),
    (Long, "Specific Hash"),
    (Long, "Library ID"),
    (Long, "Name ID"),
    (Long, "Entry Point"),
    (Long, "Domain Path ID"),
    (Byte, "Flags"),
];

/// The `ItemSerializer` packed-file magic (`0x2e30212634e92c20`).
pub const PACKED_MAGIC: u64 = 0x2e30_2126_34e9_2c20;
/// The `ItemSerializer` format version.
pub const PACKED_FORMAT_VERSION: i32 = 1;

// Ghidra FunctionRecord flag definitions. AUTO_PASS affects Ghidra's score,
// not eligibility; it never overrides any of the other restrictions.
pub(super) const HAS_TERMINATOR: u8 = 0x01;
pub(super) const AUTO_PASS: u8 = 0x02;
pub(super) const AUTO_FAIL: u8 = 0x04;
pub(super) const FORCE_SPECIFIC: u8 = 0x08;
pub(super) const FORCE_RELATION: u8 = 0x10;
pub(super) const KNOWN_FLAGS: u8 =
    HAS_TERMINATOR | AUTO_PASS | AUTO_FAIL | FORCE_SPECIFIC | FORCE_RELATION;

/// One library an image can be fingerprinted against.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GhidraLibrary {
    pub id: i64,
    pub family: String,
    pub version: String,
    pub language_id: String,
    /// A legacy compiler ID or a comma-separated metadata list, empty if unrestricted.
    pub compiler_spec_id: String,
}

/// A Ghidra FID database opened through kuna's [`FidDatabase`] surface.
pub struct FidFile {
    compatible_library: Option<usize>,
    libraries: Vec<GhidraLibrary>,
    by_full_hash: HashMap<u64, Vec<FidRecord>>,
    n_records: usize,
}

impl FidFile {
    /// Read either Ghidra container and index only functions belonging to a
    /// library that passes the image's language check.
    pub(super) fn load(
        bytes: &[u8],
        mut applies_to_image: impl FnMut(&str) -> bool,
    ) -> KunaResult<Self> {
        let packed;
        let raw = if is_packed(bytes) {
            packed = unpack(bytes)?;
            packed.as_slice()
        } else {
            bytes
        };
        let db = Database::parse(raw)?;
        let libraries_table = require_table(&db, "Libraries Table", "Library ID", |schema| {
            let mut columns = schema.columns();
            matches!(columns.next_back(), Some((Str, LEGACY_COMPILER_COLUMN | METADATA_COLUMN)))
                && columns.eq(LIBRARY_FIELDS.iter().copied())
        })?;
        let has_metadata = libraries_table.schema.field_names.last()
            .is_some_and(|name| name == METADATA_COLUMN);
        let strings_table = require_table(&db, "Strings Table", "String ID", |schema| {
            schema.columns().eq([(Str, "String Value")])
        })?;
        let functions_table = require_table(&db, "Functions Table", "Function ID", |schema| {
            schema.columns().eq(FUNCTION_FIELDS.iter().copied())
        })?;
        let mut used = db.occupied_buffers();

        let mut libraries = Vec::new();
        let mut compatible_library = None;
        let mut library_applies = HashMap::new();
        db.visit_records(libraries_table, &mut used, |rec| {
            let mut fields = rec.columns.into_iter();
            let family = take_text(fields.next(), "library family")?;
            let version = take_text(fields.next(), "library version")?;
            let language_id = take_text(fields.nth(2), "library language ID")?;
            let compiler_spec_id = library_compiler_spec(fields.nth(2), has_metadata)?;
            if language_id.is_empty() {
                return Err(KunaError::bad_data("ghidra fid: empty library language ID"));
            }
            let applies = applies_to_image(&language_id);
            if applies && compatible_library.is_none() {
                compatible_library = Some(libraries.len());
            }
            library_applies.insert(rec.key, applies);
            libraries.push(GhidraLibrary { id: rec.key, family, version, language_id, compiler_spec_id });
            Ok(())
        })?;

        let mut strings = HashMap::<_, Arc<str>>::new();
        db.visit_records(strings_table, &mut used, |rec| {
            let value = take_text(rec.columns.into_iter().next(), "string table value")?;
            strings.insert(rec.key, Arc::from(value));
            Ok(())
        })?;

        let mut by_full_hash = HashMap::<_, Vec<_>>::new();
        let mut n_records = 0;
        db.visit_records(functions_table, &mut used, |rec| {
            let [FieldValue::Short(code_unit_size), FieldValue::Long(full_hash),
                FieldValue::Byte(specific_addl), FieldValue::Long(specific_hash),
                FieldValue::Long(library_id), FieldValue::Long(name_id), FieldValue::Long(_),
                FieldValue::Long(domain_path_id), FieldValue::Byte(flags)] = rec.columns.as_slice()
            else {
                return Err(KunaError::bad_data("ghidra fid: invalid function record"));
            };
            let name = strings.get(name_id).filter(|name| !name.is_empty()).ok_or_else(|| {
                KunaError::bad_data(format!("ghidra fid: function {} has invalid name reference {name_id}", rec.key))
            })?;
            if !strings.contains_key(domain_path_id) {
                return Err(KunaError::bad_data(format!(
                    "ghidra fid: function {} has missing domain path reference {domain_path_id}", rec.key
                )));
            }
            let applies = library_applies.get(library_id).copied().ok_or_else(|| {
                KunaError::bad_data(format!("ghidra fid: function {} has missing library reference {library_id}", rec.key))
            })?;
            if applies {
                let full_hash = *full_hash as u64;
                by_full_hash.entry(full_hash).or_default().push(FidRecord {
                    full_hash,
                    specific_hash: *specific_hash as u64,
                    code_unit_size: *code_unit_size,
                    specific_addl: *specific_addl as u8,
                    flags: *flags as u8,
                    name: Arc::clone(name),
                });
                n_records += 1;
            }
            Ok(())
        })?;
        Ok(Self { compatible_library, libraries, by_full_hash, n_records })
    }

    /// The libraries this database was built from.
    pub fn libraries(&self) -> &[GhidraLibrary] {
        &self.libraries
    }

    pub(super) fn check_compatibility(&self, image_language: &str) -> Result<(), String> {
        if self.compatible_library.is_some() {
            return Ok(());
        }
        let mut supported: Vec<_> = self.libraries.iter().map(|l| l.language_id.as_str()).collect();
        supported.sort_unstable();
        supported.dedup();
        let languages = if supported.is_empty() { "none".into() } else { supported.join(", ") };
        Err(format!(
            "database has no compatible library for image language {image_language} (it carries {languages})"
        ))
    }

    fn representative_library(&self) -> Option<&GhidraLibrary> {
        self.libraries.get(self.compatible_library.unwrap_or(0))
    }
}

impl FidDatabase for FidFile {
    fn lang_id(&self) -> &str {
        self.representative_library().map_or("", |lib| lib.language_id.as_str())
    }
    fn cspec_id(&self) -> &str {
        self.representative_library().map_or("", |lib| lib.compiler_spec_id.as_str())
    }
    fn find_by_full_hash(&self, h: u64) -> &[FidRecord] {
        self.by_full_hash.get(&h).map(Vec::as_slice).unwrap_or(&[])
    }
    fn record_count(&self) -> usize {
        self.n_records
    }
}

/// Require the version, primary key, and ordered names and types we interpret.
fn require_table<'db>(
    db: &'db Database<'_>,
    name: &str,
    key: &str,
    matches_columns: impl FnOnce(&Schema) -> bool,
) -> KunaResult<&'db TableRef> {
    let table = db.table(name)
        .ok_or_else(|| KunaError::bad_data(format!("ghidra fid: missing {name}")))?;
    let schema = &table.schema;
    if schema.version != SCHEMA_VERSION || schema.key_field != Long || schema.key_name != key
        || !matches_columns(schema)
    {
        return Err(KunaError::bad_data(format!(
            "ghidra fid: incompatible {name} schema (version {})", schema.version
        )));
    }
    Ok(table)
}

fn library_compiler_spec(value: Option<FieldValue>, has_metadata: bool) -> KunaResult<String> {
    if has_metadata {
        let Some(FieldValue::Str(metadata)) = value else {
            return Err(KunaError::bad_data("ghidra fid: missing library metadata column"));
        };
        let mut metadata = metadata.unwrap_or_default();
        if let Some(separator) = metadata.find(':') {
            metadata.truncate(separator);
        }
        return Ok(metadata);
    }
    let compiler = take_text(value, "library compiler spec ID")?;
    if compiler.is_empty() {
        return Err(KunaError::bad_data("ghidra fid: empty library compiler spec ID"));
    }
    Ok(compiler)
}

fn take_text(value: Option<FieldValue>, field: &str) -> KunaResult<String> {
    match value {
        Some(FieldValue::Str(Some(value))) => Ok(value),
        _ => Err(KunaError::bad_data(format!("ghidra fid: missing {field}"))),
    }
}

const JAVA_STREAM_HEADER: &[u8] = &[0xac, 0xed, 0, 5];

// ObjectOutputStream emits TC_BLOCKDATA for up to 255 bytes and
// TC_BLOCKDATALONG otherwise, draining at most 1024 bytes at a time.
fn java_block(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    let (len, data) = match bytes {
        [0x77, len, data @ ..] => (usize::from(*len), data),
        [0x7a, a, b, c, d, data @ ..] => (
            usize::try_from(i32::from_be_bytes([*a, *b, *c, *d])).ok()?,
            data,
        ),
        _ => return None,
    };
    data.split_at_checked(len).filter(|(block, _)| !block.is_empty())
}

/// True when `bytes` start with an ItemSerializer object stream.
pub fn is_packed(bytes: &[u8]) -> bool {
    first_packed_block(bytes).is_some()
}

fn first_packed_block(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    let (block, rest) = java_block(bytes.strip_prefix(JAVA_STREAM_HEADER)?)?;
    Some((block.strip_prefix(&PACKED_MAGIC.to_be_bytes())?, rest))
}

struct JavaHeader<'a> {
    block: &'a [u8],
    rest: &'a [u8],
}

impl<'a> JavaHeader<'a> {
    fn take(&mut self, len: usize) -> KunaResult<&'a [u8]> {
        if self.block.is_empty() {
            (self.block, self.rest) = java_block(self.rest)
                .ok_or_else(|| KunaError::bad_data("ghidra fid: missing or truncated Java data block"))?;
        }
        let (head, tail) = self.block.split_at(len.min(self.block.len()));
        self.block = tail;
        Ok(head)
    }

    fn read<const N: usize>(&mut self) -> KunaResult<[u8; N]> {
        let mut bytes = [0u8; N];
        let mut remaining = bytes.as_mut_slice();
        while !remaining.is_empty() {
            let chunk = self.take(remaining.len())?;
            remaining[..chunk.len()].copy_from_slice(chunk);
            remaining = &mut remaining[chunk.len()..];
        }
        Ok(bytes)
    }

    fn skip_utf(&mut self) -> KunaResult<()> {
        let mut remaining = usize::from(u16::from_be_bytes(self.read()?));
        while remaining != 0 {
            remaining -= self.take(remaining)?.len();
        }
        Ok(())
    }
}

/// Read a Ghidra `ItemSerializer` packed file's single payload entry.
///
/// The metadata is Java block data containing a magic(8), formatVersion(4),
/// UTF item name, UTF content type, fileType(4), and length(8). After the
/// final block comes a streamed zip member: a local file header, the deflate
/// stream, and a data descriptor, without a central directory.
pub fn unpack(bytes: &[u8]) -> KunaResult<Vec<u8>> {
    const MAX_PAYLOAD: usize = 1 << 31;
    unpack_bounded(bytes, MAX_PAYLOAD)
}

pub(crate) fn unpack_bounded(bytes: &[u8], max_payload: usize) -> KunaResult<Vec<u8>> {
    let (block, rest) = first_packed_block(bytes)
        .ok_or_else(|| KunaError::bad_data("ghidra fid: not a packed database file"))?;
    let mut header = JavaHeader { block, rest };
    let version = i32::from_be_bytes(header.read()?);
    if version != PACKED_FORMAT_VERSION {
        return Err(KunaError::bad_data(format!(
            "ghidra fid: packed format version {version} != {PACKED_FORMAT_VERSION}"
        )));
    }
    header.skip_utf()?;
    header.skip_utf()?;
    let _: [u8; 4] = header.read()?; // file type
    let payload_len = i64::from_be_bytes(header.read()?);
    if !header.block.is_empty() {
        return Err(KunaError::bad_data("ghidra fid: extra Java header bytes"));
    }
    let bytes = header.rest;

    // Local file header: signature(4) version(2) flags(2) method(2) time(2)
    // date(2) crc(4) csize(4) usize(4) nameLen(2) extraLen(2) then name+extra.
    let sig = bytes.get(..4)
        .ok_or_else(|| KunaError::bad_data("ghidra fid: truncated zip header"))?;
    if sig != [0x50, 0x4b, 0x03, 0x04] {
        return Err(KunaError::bad_data("ghidra fid: packed payload is not a zip member"));
    }
    // Zip local-header fields are little-endian, unlike the Java header above.
    let method = read_le_u16(bytes, 8)?;
    let entry_name_len = read_le_u16(bytes, 26)? as usize;
    let entry_extra_len = read_le_u16(bytes, 28)? as usize;
    let data_off = 30 + entry_name_len + entry_extra_len;
    let data = bytes
        .get(data_off..)
        .ok_or_else(|| KunaError::bad_data("ghidra fid: truncated zip member"))?;

    // `payload_len` is untrusted. Validate it before it can steer an allocation,
    // and bound the inflation to it: a decompression bomb or an absurd declared
    // length must be a parse error, never an allocation abort.
    let declared = usize::try_from(payload_len).map_err(|_| {
        KunaError::bad_data(format!("ghidra fid: packed payload length {payload_len} out of range"))
    })?;
    if declared == 0 || declared > max_payload {
        return Err(KunaError::bad_data(format!(
            "ghidra fid: packed payload length {declared} out of range (limit {max_payload})"
        )));
    }
    let mut out = Vec::new();
    out.try_reserve_exact(declared.min(data.len()))
        .map_err(|_| KunaError::bad_data("ghidra fid: cannot allocate packed payload"))?;
    match method {
        0 => {
            let stored = data.get(..declared)
                .ok_or_else(|| KunaError::bad_data("ghidra fid: truncated stored zip member"))?;
            out.extend_from_slice(stored);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_detection_reads_the_magic_at_its_offset() {
        let mut bytes = vec![0u8; 32];
        bytes[..4].copy_from_slice(JAVA_STREAM_HEADER);
        bytes[4..6].copy_from_slice(&[0x77, 8]);
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
        bytes[..4].copy_from_slice(JAVA_STREAM_HEADER);
        bytes[4..6].copy_from_slice(&[0x77, 28]);
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

    const RAW_FIXTURE: &[u8] = include_bytes!("../../../tests/fixtures/fid_audit_ghidra.fidbf");
    const PACKED_FIXTURE: &[u8] = include_bytes!("../../../tests/fixtures/fid_audit_ghidra.fidb");
    const METADATA_FIXTURE: &[u8] =
        include_bytes!("../../../tests/fixtures/fid_audit_ghidra_metadata.fidbf");

    #[test]
    fn packed_header_handles_short_long_and_split_object_stream_blocks() {
        // Keep the real ItemSerializer zip payload; write the metadata in
        // stock ObjectOutputStream framing with a variable-length item name.
        let zip_start = 6 + usize::from(PACKED_FIXTURE[5]);
        let expected = unpack(PACKED_FIXTURE).unwrap();
        for name_len in [220u16, 221, 988, 989, 990, 2048] {
            let mut metadata = Vec::new();
            metadata.extend_from_slice(&PACKED_MAGIC.to_be_bytes());
            metadata.extend_from_slice(&PACKED_FORMAT_VERSION.to_be_bytes());
            metadata.extend_from_slice(&name_len.to_be_bytes());
            metadata.resize(metadata.len() + usize::from(name_len), b'n');
            metadata.extend_from_slice(b"\0\x07Program");
            metadata.extend_from_slice(&0i32.to_be_bytes());
            metadata.extend_from_slice(&(expected.len() as i64).to_be_bytes());

            let mut packed = JAVA_STREAM_HEADER.to_vec();
            for block in metadata.chunks(1024) {
                if let Ok(len) = u8::try_from(block.len()) {
                    packed.extend_from_slice(&[0x77, len]);
                } else {
                    packed.push(0x7a);
                    packed.extend_from_slice(&(block.len() as u32).to_be_bytes());
                }
                packed.extend_from_slice(block);
            }
            packed.extend_from_slice(&PACKED_FIXTURE[zip_start..]);
            assert!(is_packed(&packed), "name length {name_len}");
            assert_eq!(unpack_bounded(&packed, expected.len()).unwrap(), expected);
            assert!(unpack_bounded(&packed, expected.len() - 1).unwrap_err()
                .explain().contains("out of range"));
        }
    }

    #[test]
    fn stock_raw_and_packed_resolve_shared_overflow_names() {
        let expected_name = format!("kuna::audit::{}probe", "nested_template::".repeat(300));
        for bytes in [RAW_FIXTURE, PACKED_FIXTURE] {
            let db = FidFile::load(bytes, |lang| lang == "AARCH64:LE:64:v8A").unwrap();
            assert_eq!(db.record_count(), 2);
            let mix = &db.find_by_full_hash(0x25d1_0671_fc24_68cf)[0].name;
            let fold = &db.find_by_full_hash(0xeecb_018e_b729_863b)[0].name;
            assert_eq!(mix.as_ref(), expected_name.as_str());
            assert!(Arc::ptr_eq(mix, fold));
        }

        let expected_path = format!("/fixture/{}input", "generated/".repeat(2400));
        let raw = Database::parse(RAW_FIXTURE).unwrap();
        let strings = raw.records(raw.table("Strings Table").unwrap()).unwrap();
        assert!(strings.iter().any(|r| r.columns[0].as_str() == Some(expected_path.as_str())));
    }

    #[test]
    fn v6_library_metadata_reports_only_the_compiler_list() {
        for (language, compilers, count) in [
            ("x86:LE:64:compat32", "gcc,windows", 3),
            ("x86:LE:32:default", "default", 1),
            ("AARCH64:LE:64:v8A", "", 2),
        ] {
            let db = FidFile::load(METADATA_FIXTURE, |lang| lang == language).unwrap();
            assert_eq!(db.lang_id(), language);
            assert_eq!(db.cspec_id(), compilers);
            assert_eq!(db.record_count(), count);
        }
        assert_eq!(library_compiler_spec(Some(FieldValue::Str(None)), true).unwrap(), "");
        assert!(library_compiler_spec(Some(FieldValue::Str(None)), false).is_err());
    }

    #[test]
    fn an_incompatible_image_gets_no_candidates() {
        let db = FidFile::load(RAW_FIXTURE, |_| false).unwrap();
        let err = db.check_compatibility("MIPS:BE:32:default").unwrap_err();
        assert!(err.contains("MIPS:BE:32:default"), "{err}");
        assert!(err.contains("x86:LE:64:compat32"), "{err}");
        assert_eq!(db.record_count(), 0);
        assert!(db.find_by_full_hash(0x25d1_0671_fc24_68cf).is_empty());
    }

    #[test]
    fn records_and_metadata_follow_the_selected_library() {
        let db = FidFile::load(RAW_FIXTURE, |lang| lang == "x86:LE:32:default").unwrap();
        assert!(db.check_compatibility("x86:LE:32:default").is_ok());
        assert_eq!(db.lang_id(), "x86:LE:32:default");
        assert_eq!(db.cspec_id(), "default");
        assert_eq!(db.record_count(), 1);
        let records = db.find_by_full_hash(0x25d1_0671_fc24_68cf);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name.as_ref(), "foreign_alias");
        assert!(db.find_by_full_hash(0xeecb_018e_b729_863b).is_empty());
    }

    #[test]
    fn required_tables_columns_and_references_cannot_disappear_silently() {
        for (before, after, error) in [
            (b"Strings Table".as_slice(), b"Missing Table".as_slice(), "missing Strings Table"),
            (b"Domain Path ID", b"Wrong  Path ID", "incompatible Functions Table schema"),
            (
                b"Ghidra Compiler Spec ID",
                b"Ghidra Compiler Spec IX",
                "incompatible Libraries Table schema",
            ),
        ] {
            let mut bytes = RAW_FIXTURE.to_vec();
            let mut rest = bytes.as_mut_slice();
            while let Some(offset) = rest.windows(before.len()).position(|s| s == before) {
                let start = offset + before.len();
                rest[offset..start].copy_from_slice(after);
                rest = &mut rest[start..];
            }
            let err = FidFile::load(&bytes, |_| true).err().expect("malformed schema");
            assert!(err.explain().contains(error), "{}", err.explain());
        }

        let db = Database::parse(RAW_FIXTURE).unwrap();
        let root = db.table("Functions Table").unwrap().root_buffer_id as usize;
        let block = i32::from_be_bytes(RAW_FIXTURE[20..24].try_into().unwrap()) as usize;
        let first_row = (root + 1) * block + 5 + 13;
        for (offset, error) in [
            (27, "library reference"),
            (35, "name reference"),
            (51, "domain path reference"),
        ] {
            let mut bytes = RAW_FIXTURE.to_vec();
            bytes[first_row + offset..first_row + offset + 8].fill(0);
            let err = FidFile::load(&bytes, |_| false).err().expect("dangling reference");
            assert!(err.explain().contains(error), "{}", err.explain());
        }
    }
}
