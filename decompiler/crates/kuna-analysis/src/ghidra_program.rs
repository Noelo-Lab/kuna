//! Read an immutable Ghidra 12.1.2 ProgramDB snapshot without a Ghidra runtime.

mod address;
mod file_bytes;

#[expect(clippy::disallowed_types, reason = "database buffer ownership")]
use std::collections::HashSet;
use std::collections::{BTreeMap, BTreeSet};

use kuna_base::error::{KunaError, KunaResult};

use crate::fid::ghidradb::{Database, FieldValue, TableRef};
use crate::fid::ghidrafid::{is_packed, unpack_bounded};
use crate::fid::schema::FieldType::{self, Binary, Bool, Byte, Int, Long, Str};

use self::address::{AddressMap, Layout, Metadata};

/// Original executable bytes and functions stored in the default address space.
#[derive(Debug)]
pub struct ProgramSnapshot {
    pub original: Vec<u8>,
    pub functions: Vec<ProgramFunction>,
    /// Saved processor language, without a compiler specification or display version.
    pub language_id: String,
}

/// A Ghidra function entry in the original image's address space and its unqualified name.
#[derive(Debug, PartialEq, Eq)]
pub struct ProgramFunction {
    pub address: u64,
    pub name: String,
}

const TABLES: &[&str] = &[
    "Program", "Property Table", "File Bytes", "ADDRESS MAP", "Overlay Spaces", "Function Data",
    "Symbols", "Thunk Functions", "Memory Blocks", "Sub Memory Blocks",
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
    let db = Database::parse_program(raw, TABLES)?;
    let mut used = db.occupied_buffers();
    let metadata = Metadata::read(&db, &mut used)?;
    let map = AddressMap::read(&db, &metadata, &mut used)?;
    let file = file_bytes::read(&db, &metadata, &map, &mut used, raw.len(), max_original_size)?;
    let mut functions = read_functions(&db, &map, &mut used)?;
    file.relocate_entries(&mut functions)?;
    Ok(ProgramSnapshot { original: file.original, functions, language_id: metadata.language_id })
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
    if table.version != version
        || (schema.key_field, schema.key_name.as_str()) != key
        || !schema.fields.iter().copied().zip(schema.field_names.iter().map(String::as_str))
            .eq(columns.iter().copied())
    {
        return Err(bad(format_args!("unsupported {name} schema (v{})", table.version)));
    }
    Ok(table)
}

const DEFAULT_SOURCE: i8 = 2;
const FUNCTION_SYMBOL: i8 = 5;

struct FunctionSymbol {
    address: Option<u64>,
    name: Option<String>,
    default_name: bool,
    external_address: Option<String>,
    thunk: Option<i64>,
}

fn read_functions(
    db: &Database<'_>, map: &AddressMap, used: &mut HashSet<i32>,
) -> KunaResult<Vec<ProgramFunction>> {
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

    let table = require_table(db, "Symbols", 4, (Long, "Key"), &[
        (Str, "Name"), (Long, "Address"), (Long, "Namespace"), (Byte, "Symbol Type"),
        (Byte, "Flags"), (Long, "Locator Hash"), (Long, "Primary"), (Long, "Datatype"),
        (Int, "Variable Offset"), (Str, "ExtOrigImportName"), (Str, "ExtProgAddr"),
        (Str, "Comment"), (Str, "LibPath"),
    ])?;
    let mut symbols = BTreeMap::new();
    db.visit_program_symbols(table, used, |mut record| {
        let [FieldValue::Str(name), FieldValue::Long(key), FieldValue::Long(_),
            FieldValue::Byte(kind), FieldValue::Byte(flags), FieldValue::Str(external_address)] =
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
        let address = map.decode(*key)?;
        let default_name = *flags & 3 == DEFAULT_SOURCE;
        if !default_name && name.as_ref().is_none_or(String::is_empty) {
            return Err(bad(format_args!("function symbol {} has no explicit name", record.key)));
        }
        symbols.insert(record.key, FunctionSymbol {
            address, name: name.take(), default_name, thunk: None,
            external_address: if (*key as u64) >> 60 == 5 { external_address.take() } else { None },
        });
        Ok(())
    })?;
    if let Some(id) = function_ids.first() {
        return Err(bad(format_args!("Function Data {id} has no function symbol")));
    }

    let table = require_table(db, "Thunk Functions", 0, (Long, "ID"), &[(Long, "Linked Function ID")])?;
    db.visit_records(table, used, |record| {
        let [FieldValue::Long(target)] = record.columns.as_slice() else {
            return Err(bad("invalid Thunk Functions record"));
        };
        if !symbols.contains_key(target) {
            return Err(bad(format_args!("missing thunk target {target}")));
        }
        let thunk = symbols.get_mut(&record.key).ok_or_else(|| bad("missing thunk function"))?;
        thunk.thunk = Some(*target);
        Ok(())
    })?;

    let mut functions = Vec::new();
    for (&id, symbol) in &symbols {
        if let Some(address) = symbol.address {
            let name = display_name(id, &symbols, map.layout)?;
            functions.push(ProgramFunction { address, name });
        }
    }
    functions.sort_unstable_by_key(|function| function.address);
    if functions.windows(2).any(|pair| pair[0].address == pair[1].address) {
        return Err(bad("ambiguous function entries in the default address space"));
    }
    Ok(functions)
}

// FunctionSymbol.doGetName: explicit names win. A chain of default thunks
// inherits the terminal name, with the prefix only for a default non-thunk.
fn display_name(id: i64, symbols: &BTreeMap<i64, FunctionSymbol>, layout: Layout) -> KunaResult<String> {
    let mut current = id;
    for _ in 0..symbols.len() {
        let symbol = &symbols[&current];
        if !symbol.default_name {
            return symbol.name.clone().ok_or_else(|| bad("missing explicit function name"));
        }
        if let Some(target) = symbol.thunk {
            current = target;
            continue;
        }
        let thunk_prefix = if id == current { "" } else { "thunk_" };
        if let Some(address) = symbol.address {
            let digits = usize::from(layout.bits.div_ceil(4).min(8));
            return Ok(format!("{thunk_prefix}FUN_{address:0digits$x}"));
        }
        if let Some(external) = symbol.external_address.as_deref().filter(|addr| !addr.is_empty()) {
            return Ok(format!("{thunk_prefix}EXT_FUN_{}", external.replace(':', "_")));
        }
        return Err(bad(format_args!("unsupported default name for thunk target {current}")));
    }
    Err(bad(format_args!("cyclic thunk chain for function {id}")))
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
            address: 0x401000, name: "project_scale_a".into(),
        }));
        assert!(snapshot.functions.contains(&ProgramFunction {
            address: 0x401020, name: "project_scale_b".into(),
        }));
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
