//! Ghidra `db.Schema` decoding — field type codes and the packed field-name list.
//!
//! A table's schema is stored in the master-table record as the primary key
//! type, a byte array of column type codes, and a `;`-terminated list of column
//! names. Decoding it is what lets the reader walk any Ghidra table without
//! knowing its shape in advance.
//!
//! # Origin (upstream Ghidra)
//!
//! - `Ghidra/Framework/DB/src/main/java/db/Schema.java` — `decodeSchema`,
//!   `getEncodedFieldTypes`, `getPackedFieldNames`, `getEncodedKeyFieldType`.
//! - `.../db/Field.java` — the `*_TYPE` constants (`BYTE_TYPE` 0 … `BOOLEAN_TYPE` 6,
//!   `FIXED_10_TYPE` 7, `LEGACY_INDEX_LONG_TYPE` 8, `FIELD_TYPE_MASK` `0x0F`).
//! - `.../db/FieldFactory.java` — the type-code-to-`Field` mapping.

use kuna_base::error::{KunaError, KunaResult};

/// One column's storage type, by Ghidra's `Field` type code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldType {
    Byte,
    Short,
    Int,
    Long,
    Str,
    Binary,
    Bool,
    /// `FIXED_10_TYPE` — a 10-byte fixed value, never used by FID tables.
    Fixed10,
    /// `LEGACY_INDEX_LONG_TYPE` — a long stored by the legacy index path.
    LegacyIndexLong,
}

impl FieldType {
    /// The number of bytes this field occupies when fixed-length; `None` for the
    /// variable-length types (`String`/`Binary`).
    pub fn fixed_size(self) -> Option<usize> {
        match self {
            FieldType::Byte | FieldType::Bool => Some(1),
            FieldType::Short => Some(2),
            FieldType::Int => Some(4),
            FieldType::Long | FieldType::LegacyIndexLong => Some(8),
            FieldType::Fixed10 => Some(10),
            FieldType::Str | FieldType::Binary => None,
        }
    }

    /// The `Field` type code this maps to.
    pub fn code(self) -> u8 {
        match self {
            FieldType::Byte => 0,
            FieldType::Short => 1,
            FieldType::Int => 2,
            FieldType::Long => 3,
            FieldType::Str => 4,
            FieldType::Binary => 5,
            FieldType::Bool => 6,
            FieldType::Fixed10 => 7,
            FieldType::LegacyIndexLong => 8,
        }
    }

    fn from_code(code: u8) -> KunaResult<Self> {
        Ok(match code {
            0 => FieldType::Byte,
            1 => FieldType::Short,
            2 => FieldType::Int,
            3 => FieldType::Long,
            4 => FieldType::Str,
            5 => FieldType::Binary,
            6 => FieldType::Bool,
            7 => FieldType::Fixed10,
            8 => FieldType::LegacyIndexLong,
            other => {
                return Err(KunaError::bad_data(format!(
                    "ghidra db: unknown field type code {other}"
                )))
            }
        })
    }
}

/// A decoded table schema: its version, primary key type and columns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schema {
    pub version: i32,
    pub key_field: FieldType,
    pub key_name: String,
    pub fields: Vec<FieldType>,
    pub field_names: Vec<String>,
}

impl Schema {
    /// Decode a master-table record's schema columns.
    ///
    /// `key_type` is the encoded key field byte (`getEncodedKeyFieldType`),
    /// `field_types` the encoded column types. Index tables and field extensions
    /// (such as sparse columns) have different encodings and are not supported.
    /// `packed_field_names` is the `;`-terminated list whose first entry is
    /// the primary key's name and whose remainder are the column names.
    pub fn decode(
        version: i32,
        key_type: u8,
        field_types: &[u8],
        packed_field_names: &str,
    ) -> KunaResult<Self> {
        let key_field = FieldType::from_code(key_type)?;
        let fields = field_types
            .iter()
            .map(|&code| FieldType::from_code(code))
            .collect::<KunaResult<Vec<_>>>()?;
        let packed = packed_field_names.strip_suffix(';').ok_or_else(|| {
            KunaError::bad_data("ghidra db: schema field names are not terminated")
        })?;
        let mut names = packed.split(';');
        let key_name = names.next().filter(|name| !name.is_empty()).ok_or_else(|| {
            KunaError::bad_data("ghidra db: schema is missing its key name")
        })?;
        let bad_names = || KunaError::bad_data(
            "ghidra db: schema column names do not match the encoded field types",
        );
        let mut field_names = Vec::with_capacity(fields.len());
        for _ in &fields {
            let name = names.next().filter(|name| !name.is_empty()).ok_or_else(bad_names)?;
            field_names.push(name.to_owned());
        }
        if names.next().is_some() {
            return Err(bad_names());
        }
        Ok(Self { version, key_field, key_name: key_name.to_owned(), fields, field_names })
    }

    pub(super) fn columns(&self) -> impl DoubleEndedIterator<Item = (FieldType, &str)> {
        self.fields.iter().copied().zip(self.field_names.iter().map(String::as_str))
    }

    /// The byte length of one record when every column is fixed-length; `None`
    /// when the schema has a variable-length column.
    pub fn fixed_record_len(&self) -> Option<usize> {
        let mut total = 0usize;
        for field in &self.fields {
            total += field.fixed_size()?;
        }
        Some(total)
    }

    /// The column index named `name`.
    pub fn column(&self, name: &str) -> Option<usize> {
        self.field_names.iter().position(|n| n == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_names_start_with_the_key_name() {
        // `Schema.getPackedFieldNames` writes the KEY name first, then columns.
        let schema = Schema::decode(
            6,
            3,
            &[1, 3, 0, 3, 3, 3, 3, 3, 0],
            "Function ID;Code Unit Size;Full Hash;Specific Hash Additional Size;\
             Specific Hash;Library ID;Name ID;Entry Point;Domain Path ID;Flags;",
        )
        .unwrap();
        assert_eq!(schema.key_field, FieldType::Long);
        assert_eq!(schema.key_name, "Function ID");
        assert_eq!(schema.fields.len(), 9);
        assert_eq!(schema.field_names[0], "Code Unit Size");
        assert_eq!(schema.column("Full Hash"), Some(1));
        assert_eq!(schema.column("Flags"), Some(8));
    }

    #[test]
    fn a_zero_column_table_decodes() {
        // The relation tables carry no columns: the packed list is the key alone.
        let schema = Schema::decode(6, 3, &[], "Relation Smash;").unwrap();
        assert!(schema.fields.is_empty());
        assert!(schema.field_names.is_empty());
        assert_eq!(schema.fixed_record_len(), Some(0));
    }

    #[test]
    fn a_column_count_that_disagrees_with_the_field_types_is_an_error() {
        // Two encoded columns but one column name after the key name.
        assert!(Schema::decode(1, 3, &[1, 3], "Key;OnlyOne;").is_err());
        // One column with no name at all (only the key name) also disagrees.
        assert!(Schema::decode(1, 3, &[1], "Key;").is_err());
    }

    #[test]
    fn empty_or_shifted_names_are_rejected() {
        assert!(Schema::decode(6, 3, &[], "").is_err());
        assert!(Schema::decode(6, 3, &[3], ";Key;Value;").is_err());
        assert!(Schema::decode(6, 3, &[3], "Key;;Value;").is_err());
    }

    #[test]
    fn fixed_record_len_is_none_when_a_column_is_variable() {
        let schema = Schema::decode(6, 3, &[4], "Key;String Value;").unwrap();
        assert_eq!(schema.fixed_record_len(), None);
        let fixed = Schema::decode(6, 3, &[1, 3], "Key;A;B;").unwrap();
        assert_eq!(fixed.fixed_record_len(), Some(10));
    }

    #[test]
    fn an_unknown_field_type_code_is_rejected() {
        assert!(Schema::decode(1, 3, &[0x0e], "Key;Bad;").is_err());
        assert!(Schema::decode(6, 0x13, &[3], "Key;Value;").is_err());
        assert!(Schema::decode(6, 3, &[3, 0xff, 1, 0], "Key;Value;").is_err());
    }
}
