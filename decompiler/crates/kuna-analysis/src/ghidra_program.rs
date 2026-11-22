//! Read an immutable Ghidra 12.1.2 ProgramDB snapshot without a Ghidra runtime.

mod address;
mod elf_arm_targets;
mod elf_relocation_arm;
mod elf_sections;
mod file_bytes;

#[expect(clippy::disallowed_types, reason = "database buffer ownership")]
use std::collections::HashSet;
use std::collections::BTreeSet;

use kuna_base::error::{KunaError, KunaResult};

use crate::fid::ghidradb::{Database, FieldValue, PROGRAM_SYMBOLS_VERSION, TableRef};
use crate::fid::ghidrafid::{is_packed, unpack_bounded};
use crate::fid::schema::FieldType::{self, Binary, Bool, Byte, Int, Long, Str};

use self::address::{AddressMap, Metadata, ProgramInfo};

/// Original executable bytes and functions stored in the default address space.
#[derive(Debug)]
pub struct ProgramSnapshot {
    pub original: Vec<u8>,
    pub functions: Vec<ProgramFunction>,
    /// Default-space functions omitted because they have no meaningful saved name.
    pub skipped_unnamed: usize,
    /// Saved processor language, without a compiler specification or display version.
    pub language_id: String,
}

/// A Ghidra function entry in the original image and its unqualified name.
#[derive(Debug, PartialEq, Eq)]
pub struct ProgramFunction {
    pub location: ProgramEntry,
    pub name: String,
}

/// Original coordinates, independent of the saved program's load addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProgramEntry {
    LinkedAddress(u64),
    ElfSection { section_index: usize, offset: u64 },
}

const TABLES: &[&str] = &[
    "Program", "Property Table", "File Bytes", "ADDRESS MAP", "Overlay Spaces", "Function Data",
    "Symbols", "Memory Blocks", "Sub Memory Blocks",
    "Relocations",
];

/// Read a raw `.gbf` or packed `.gzf` Ghidra 12.1.2 ProgramDB. The original
/// must be a single whole-file import. Only supported flat default spaces are
/// returned; an unsupported map, rather than a guessed entry, is an error.
/// The database limit applies to both the input and the unpacked database.
pub fn read(
    bytes: &[u8], max_database_size: usize, max_original_size: usize,
) -> KunaResult<ProgramSnapshot> {
    if bytes.len() > max_database_size {
        return Err(bad(format_args!("database exceeds the {max_database_size}-byte limit")));
    }
    let packed;
    let raw = if is_packed(bytes) {
        packed = unpack_bounded(bytes, max_database_size)?;
        packed.as_slice()
    } else {
        bytes
    };
    let (db, mut used, program) = Database::parse_program(raw, TABLES, ProgramInfo::read)?;
    let metadata = Metadata::read(&db, &mut used, program)?;
    let map = AddressMap::read(&db, &metadata, &mut used)?;
    let file = file_bytes::read(&db, &metadata, &map, &mut used, raw.len(), max_original_size)?;
    let (mut functions, skipped_unnamed) = read_functions(&db, &map, &mut used)?;
    file.project_entries(&mut functions)?;
    Ok(ProgramSnapshot {
        original: file.original, functions, skipped_unnamed, language_id: metadata.language_id,
    })
}

fn bad(message: impl std::fmt::Display) -> KunaError {
    KunaError::bad_data(format!("ghidra program: {message}"))
}

fn require_table<'a>(
    db: &'a Database<'_>,
    name: &str,
    version: i32,
    key: (FieldType, &str),
    columns: &[(FieldType, &str)],
) -> KunaResult<&'a TableRef> {
    let table = db.table(name).ok_or_else(|| bad(format_args!("missing {name} table")))?;
    let schema = &table.schema;
    if table.version != version {
        return Err(bad(format_args!(
            "unsupported {name} schema (v{}); expected v{version}", table.version
        )));
    }
    if (schema.key_field, schema.key_name.as_str()) != key
        || !schema.fields.iter().copied().zip(schema.field_names.iter().map(String::as_str))
            .eq(columns.iter().copied())
    {
        return Err(bad(format_args!("unsupported {name} schema (v{version}): field layout differs")));
    }
    Ok(table)
}

const SYMBOL_SOURCE_MASK: i8 = 0x0b;
const DEFAULT_SOURCE: i8 = 2;
const FUNCTION_SYMBOL: i8 = 5;

fn read_functions(
    db: &Database<'_>, map: &AddressMap, used: &mut HashSet<i32>,
) -> KunaResult<(Vec<ProgramFunction>, usize)> {
    let table = require_table(db, "Function Data", 3, (Long, "ID"), &[
        (Long, "Return DataType ID"), (Int, "StackPurge"), (Int, "StackReturnOffset"),
        (Int, "StackLocalSize"), (Byte, "Flags"), (Byte, "Calling Convention ID"),
        (Str, "Return Storage"),
    ])?;
    let mut function_ids = BTreeSet::new();
    db.visit_records(table, used, |record| {
        function_ids.insert(record.key);
        Ok(())
    })?;

    let table = require_table(db, "Symbols", PROGRAM_SYMBOLS_VERSION, (Long, "Key"), &[
        (Str, "Name"), (Long, "Address"), (Long, "Namespace"), (Byte, "Symbol Type"),
        (Byte, "Flags"), (Long, "Locator Hash"), (Long, "Primary"), (Long, "Datatype"),
        (Int, "Variable Offset"), (Str, "ExtOrigImportName"), (Str, "ExtProgAddr"),
        (Str, "Comment"), (Str, "LibPath"),
    ])?;
    let mut functions = Vec::new();
    let mut skipped_unnamed = 0;
    db.visit_program_symbols(table, used, |mut record| {
        let [FieldValue::Str(name), FieldValue::Long(key), FieldValue::Long(_),
            FieldValue::Byte(kind), FieldValue::Byte(flags)] =
            record.columns.as_mut_slice()
        else {
            return Err(bad("invalid Symbols record"));
        };
        if *kind != FUNCTION_SYMBOL {
            return Ok(());
        }
        if !function_ids.remove(&record.key) {
            return Err(bad(format_args!("function symbol {} has no Function Data", record.key)));
        }
        let Some(address) = map.decode(*key)? else { return Ok(()) };
        let Some(name) = saved_name(*flags, name.take()) else {
            skipped_unnamed += 1;
            return Ok(());
        };
        functions.push(ProgramFunction { location: ProgramEntry::LinkedAddress(address), name });
        Ok(())
    })?;
    if let Some(id) = function_ids.first() {
        return Err(bad(format_args!("Function Data {id} has no function symbol")));
    }

    functions.sort_unstable_by_key(|function| function.location);
    if functions.windows(2).any(|pair| pair[0].location == pair[1].location) {
        return Err(bad("ambiguous function entries in the default address space"));
    }
    Ok((functions, skipped_unnamed))
}

fn saved_name(flags: i8, name: Option<String>) -> Option<String> {
    if flags & SYMBOL_SOURCE_MASK == DEFAULT_SOURCE {
        return None;
    }
    name.filter(|name| !name.is_empty() && !is_placeholder(name))
}

fn is_placeholder(name: &str) -> bool {
    let name = name.strip_prefix("thunk_").unwrap_or(name);
    ["FUN_", "EXT_FUN_", "LAB_", "sub_", "func_"].iter().any(|prefix| {
        name.strip_prefix(prefix).is_some_and(|suffix| {
            let digits = suffix.strip_prefix("0x").unwrap_or(suffix);
            !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_hexdigit())
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const STOCK_GZF: &[u8] = include_bytes!("../../../../tests/fixtures/bsim/reference.gzf");
    const STOCK_ORIGINAL: &[u8] = include_bytes!("../../../../tests/fixtures/fid/callees/prog");

    #[test]
    fn stock_fixed_base_keeps_saved_names_and_original_backing() {
        let snapshot = read(STOCK_GZF, 2 << 20, STOCK_ORIGINAL.len()).unwrap();
        assert_eq!(snapshot.language_id, "x86:LE:64:default");
        assert_eq!(snapshot.original, STOCK_ORIGINAL);
        assert!(snapshot.functions.contains(&ProgramFunction {
            location: ProgramEntry::LinkedAddress(0x401000), name: "project_scale_a".into(),
        }));
        assert!(snapshot.functions.contains(&ProgramFunction {
            location: ProgramEntry::LinkedAddress(0x401020), name: "project_scale_b".into(),
        }));
    }

    #[test]
    fn saved_names_ignore_default_provenance_and_numeric_placeholders() {
        for name in ["FUN_00401000", "thunk_FUN_00401000", "sub_401000", "func_0x401000", ""] {
            assert_eq!(saved_name(3, Some(name.into())), None, "{name}");
        }
        assert_eq!(saved_name(DEFAULT_SOURCE | 4, Some("inherited_thunk_name".into())), None);
        for (source, name) in [(0, "sub_process"), (1, "entry"), (3, "FUN_validation"), (8, "ai_name")] {
            assert_eq!(saved_name(source, Some(name.into())).as_deref(), Some(name));
        }
    }

    #[test]
    fn saved_program_version_is_checked_before_sparse_symbols() {
        let mut raw = unpack_bounded(STOCK_GZF, 2 << 20).unwrap();
        let symbols = b"\0\0\0\x07Symbols\0\0\0\x04";
        let offset = raw.windows(symbols.len()).position(|span| span == symbols).unwrap();
        raw[offset + symbols.len() - 1] = 3;
        let error = read(&raw, raw.len(), STOCK_ORIGINAL.len()).unwrap_err();
        assert!(error.explain().contains("unsupported Symbols schema (v3)"));

        let program_version = b"\0\0\0\x0aDB Version\0\0\0\x0232";
        let offset = raw.windows(program_version.len()).position(|span| span == program_version).unwrap();
        let end = offset + program_version.len();
        raw[end - 2..end].copy_from_slice(b"28");
        let error = read(&raw, raw.len(), STOCK_ORIGINAL.len()).unwrap_err();
        assert!(error.explain().contains(
            "unsupported Program DB Version \"28\"; only saved version 32 (Ghidra 12.1.2) is supported"
        ));
    }

    #[test]
    fn caps_raw_database_packed_payload_and_original_independently() {
        let raw = unpack_bounded(STOCK_GZF, 2 << 20).unwrap();
        let db_limit = raw.len() - 1;
        assert!(STOCK_GZF.len() < db_limit);
        let error = read(&raw, db_limit, STOCK_ORIGINAL.len()).unwrap_err();
        assert!(error.explain().contains("database exceeds"));
        let error = read(STOCK_GZF, db_limit, STOCK_ORIGINAL.len()).unwrap_err();
        assert!(error.explain().contains("packed payload length"));
        let error = read(&raw, raw.len(), STOCK_ORIGINAL.len() - 1).unwrap_err();
        assert!(error.explain().contains("original File Bytes exceed"));
    }
}
