//! The Ghidra `db` container reader — the buffer file plus its B-tree tables.
//!
//! Ghidra's `.fidbf` (raw) and `.fidb` (packed) files both wrap the same
//! `db.DBHandle` database: a **buffer file** of fixed-size blocks, where block 0
//! is the file header and buffer 0 onwards are user buffers. One user buffer
//! (the `DBParms` block) holds the database parameters, of which parameter 0 is
//! the root buffer id of the **master table**; every other table is described by
//! one master-table record that carries its name, schema and root buffer id.
//!
//! ```text
//! block 0        | magic(8) fileId(8) version(4) blockSize(4) firstFree(4)
//!                | parmCount(4) [ nameLen(4) name value(4) ]*
//! block 1+i      | flags(1) id(4) | payload(blockSize - 5)
//! ```
//!
//! Each B-tree node is one buffer payload whose first byte is the node type and
//! whose next four bytes are the key count. All integers are **big-endian**; the
//! user payload starts 5 bytes into its block (the prefix is Ghidra's
//! `BUFFER_PREFIX_SIZE`).
//!
//! # Origin (upstream Ghidra)
//!
//! - `Ghidra/Framework/DB/src/main/java/db/buffers/LocalBufferFile.java` — the
//!   file header, the block framing and the free-list encoding.
//! - `.../db/NodeMgr.java` — the node-type byte and the per-node header layout.
//! - `.../db/VarRecNode.java`, `.../db/FixedRecNode.java`,
//!   `.../db/LongKeyInteriorNode.java` — leaf and interior node layouts.
//! - `.../db/DBRecord.java` plus `Field.java`/`StringField.java`/… — the
//!   schema-driven record encoding.
//!
//! Every read here is bounds-checked and every structural surprise is an
//! `Err`: a foreign or corrupt database must fail to load, never panic.

use kuna_base::error::{KunaError, KunaResult};

use super::schema::{FieldType, Schema};

/// The `LocalBufferFile.MAGIC_NUMBER` that opens every buffer file (`0x2f30312c34292c2a`).
pub const BUFFER_FILE_MAGIC: u64 = 0x2f30_312c_3429_2c2a;
/// `LocalBufferFile.HEADER_FORMAT_VERSION` — the only header format in use.
pub const BUFFER_FILE_HEADER_VERSION: i32 = 1;
/// `LocalBufferFile.BUFFER_PREFIX_SIZE` — the block prefix ahead of the payload.
pub const BUFFER_PREFIX_SIZE: usize = 5;
/// `DBParms.MASTER_TABLE_ROOT_BUFFER_ID_PARM` — the parameter naming the master table.
pub const PARM_MASTER_TABLE_ROOT: usize = 0;

/// A node type byte (`NodeMgr`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    LongKeyInterior,
    LongKeyVarRec,
    LongKeyFixedRec,
    VarKeyInterior,
    VarKeyRecord,
    FixedKeyInterior,
    FixedKeyVarRec,
    FixedKeyFixedRec,
    ChainedBufferIndex,
    ChainedBufferData,
}

impl NodeType {
    fn from_byte(b: u8) -> KunaResult<Self> {
        Ok(match b {
            0 => NodeType::LongKeyInterior,
            1 => NodeType::LongKeyVarRec,
            2 => NodeType::LongKeyFixedRec,
            3 => NodeType::VarKeyInterior,
            4 => NodeType::VarKeyRecord,
            5 => NodeType::FixedKeyInterior,
            6 => NodeType::FixedKeyVarRec,
            7 => NodeType::FixedKeyFixedRec,
            8 => NodeType::ChainedBufferIndex,
            9 => NodeType::ChainedBufferData,
            other => {
                return Err(KunaError::bad_data(format!("ghidra db: unknown node type {other}")))
            }
        })
    }
}

/// A big-endian, bounds-checked view over one buffer payload.
struct Buf<'a> {
    data: &'a [u8],
}

impl<'a> Buf<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    fn bytes(&self, off: usize, len: usize) -> KunaResult<&'a [u8]> {
        let end = off
            .checked_add(len)
            .ok_or_else(|| KunaError::bad_data("ghidra db: offset overflow"))?;
        self.data.get(off..end).ok_or_else(|| {
            KunaError::bad_data(format!(
                "ghidra db: read of {len} bytes at {off} past buffer end {}",
                self.data.len()
            ))
        })
    }

    fn u8(&self, off: usize) -> KunaResult<u8> {
        Ok(self.bytes(off, 1)?[0])
    }

    fn i16(&self, off: usize) -> KunaResult<i16> {
        let b = self.bytes(off, 2)?;
        Ok(i16::from_be_bytes([b[0], b[1]]))
    }

    fn i32(&self, off: usize) -> KunaResult<i32> {
        let b = self.bytes(off, 4)?;
        Ok(i32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn i64(&self, off: usize) -> KunaResult<i64> {
        let b = self.bytes(off, 8)?;
        Ok(i64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    }

    /// A `StringField`: `i32` byte length followed by UTF-8, or `-1` for null.
    fn string(&self, off: usize) -> KunaResult<(Option<String>, usize)> {
        let len = self.i32(off)?;
        if len < 0 {
            return Ok((None, off + 4));
        }
        let len = len as usize;
        let raw = self.bytes(off + 4, len)?;
        let text = std::str::from_utf8(raw)
            .map_err(|_| KunaError::bad_data("ghidra db: non-UTF-8 string field"))?
            .to_owned();
        Ok((Some(text), off + 4 + len))
    }

    /// A `BinaryField`: `i32` byte length followed by the bytes, or `-1` for null.
    fn binary(&self, off: usize) -> KunaResult<(Option<Vec<u8>>, usize)> {
        let len = self.i32(off)?;
        if len < 0 {
            return Ok((None, off + 4));
        }
        let len = len as usize;
        Ok((Some(self.bytes(off + 4, len)?.to_vec()), off + 4 + len))
    }
}

/// One decoded database record: its primary key and the schema-ordered field
/// column values it carries.
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub key: i64,
    pub columns: Vec<FieldValue>,
}

/// A decoded field value in schema order.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Str(Option<String>),
    Binary(Option<Vec<u8>>),
    Bool(bool),
}

impl FieldValue {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            FieldValue::Byte(v) => Some(i64::from(*v)),
            FieldValue::Short(v) => Some(i64::from(*v)),
            FieldValue::Int(v) => Some(i64::from(*v)),
            FieldValue::Long(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            FieldValue::Str(Some(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            FieldValue::Binary(Some(b)) => Some(b.as_slice()),
            _ => None,
        }
    }
}

/// A table located through the master table: its name, schema version, schema
/// and the buffer id of its B-tree root.
#[derive(Clone, Debug, PartialEq)]
pub struct TableRef {
    pub name: String,
    pub version: i32,
    pub root_buffer_id: i32,
    pub schema: Schema,
    pub indexed_column: i32,
    pub record_count: i32,
}

/// A memory-mapped Ghidra database file (`db.DBHandle` on disk).
pub struct Database {
    data: Vec<u8>,
    block_size: usize,
    buffer_count: usize,
    parms: Vec<i32>,
    tables: Vec<TableRef>,
}

impl Database {
    /// Parse a raw buffer file (`LocalBufferFile`). `Err` for a file that is not
    /// a Ghidra database, is truncated, or whose master table cannot be read.
    pub fn parse(data: Vec<u8>) -> KunaResult<Self> {
        if data.len() < 32 {
            return Err(KunaError::bad_data("ghidra db: file shorter than its header"));
        }
        let head = Buf::new(&data);
        let magic = head.i64(0)?;
        if magic as u64 != BUFFER_FILE_MAGIC {
            return Err(KunaError::bad_data(format!(
                "ghidra db: bad magic {magic:#x} (expected {BUFFER_FILE_MAGIC:#x})"
            )));
        }
        let version = head.i32(16)?;
        if version != BUFFER_FILE_HEADER_VERSION {
            return Err(KunaError::bad_data(format!(
                "ghidra db: header format version {version} != {BUFFER_FILE_HEADER_VERSION}"
            )));
        }
        let block_size = head.i32(20)?;
        if block_size <= BUFFER_PREFIX_SIZE as i32 {
            return Err(KunaError::bad_data(format!("ghidra db: block size {block_size} too small")));
        }
        let block_size = block_size as usize;
        if data.len() % block_size != 0 {
            return Err(KunaError::bad_data(format!(
                "ghidra db: file length {} is not a multiple of block size {block_size}",
                data.len()
            )));
        }
        let buffer_count = data.len() / block_size - 1;

        let parm_count = head.i32(28)?;
        if parm_count < 0 {
            return Err(KunaError::bad_data("ghidra db: negative header parameter count"));
        }
        let mut off = 32;
        for _ in 0..parm_count {
            // Header parameters are name/value pairs; values are not used by the
            // FID reader, but the walk keeps the offsets honest.
            let (_, next) = head.string(off)?;
            off = next + 4;
            if off > block_size {
                return Err(KunaError::bad_data("ghidra db: header parameter runs past block 0"));
            }
        }

        let mut db = Database { data, block_size, buffer_count, parms: Vec::new(), tables: Vec::new() };
        db.parms = db.read_db_parms()?;
        db.tables = db.read_master_table()?;
        Ok(db)
    }

    /// The payload of user buffer `id` (block `id + 1`, prefix stripped).
    fn buffer(&self, id: i32) -> KunaResult<Buf<'_>> {
        if id < 0 || id as usize >= self.buffer_count {
            return Err(KunaError::bad_data(format!(
                "ghidra db: buffer id {id} outside 0..{}",
                self.buffer_count
            )));
        }
        let start = (id as usize + 1) * self.block_size + BUFFER_PREFIX_SIZE;
        let end = (id as usize + 2) * self.block_size;
        Ok(Buf::new(self.data.get(start..end).ok_or_else(|| {
            KunaError::bad_data(format!("ghidra db: buffer {id} payload outside the file"))
        })?))
    }

    /// Decode the `DBParms` block (user buffer 0): node type `CHAINED_BUFFER_DATA`,
    /// a data-length word, a version byte, then 4-byte parameter values.
    fn read_db_parms(&self) -> KunaResult<Vec<i32>> {
        let buf = self.buffer(0)?;
        if NodeType::from_byte(buf.u8(0)?)? != NodeType::ChainedBufferData {
            return Err(KunaError::bad_data("ghidra db: DBParms buffer is not a data node"));
        }
        // DATA_LENGTH_OFFSET = 1, VERSION_OFFSET = 5, PARM_BASE_OFFSET = 6.
        if buf.u8(5)? != 1 {
            return Err(KunaError::bad_data("ghidra db: unsupported DBParms version"));
        }
        let data_len = buf.i32(1)?;
        if data_len < 1 {
            return Err(KunaError::bad_data("ghidra db: empty DBParms block"));
        }
        // `data_len` is untrusted: clamp the derived count to what the payload can
        // actually hold before reserving, so a corrupt length cannot request a
        // huge allocation. The per-parameter read below is still bounds-checked.
        let declared = (data_len as usize - 1) / 4;
        let available = buf.data.len().saturating_sub(6) / 4;
        let count = declared.min(available);
        let mut out = Vec::with_capacity(count);
        for i in 0..count {
            out.push(buf.i32(6 + i * 4)?);
        }
        Ok(out)
    }

    fn parm(&self, index: usize) -> KunaResult<i32> {
        self.parms
            .get(index)
            .copied()
            .ok_or_else(|| KunaError::bad_data(format!("ghidra db: parameter {index} not allocated")))
    }

    /// The master table record schema (`TableRecord.getTableRecordSchema`):
    /// name, schema version, root buffer id, key type, encoded field types,
    /// packed field names, indexed column, max key, record count.
    fn master_schema() -> Schema {
        Schema {
            version: 0,
            key_field: FieldType::Long,
            fields: vec![
                FieldType::Str,
                FieldType::Int,
                FieldType::Int,
                FieldType::Byte,
                FieldType::Binary,
                FieldType::Str,
                FieldType::Int,
                FieldType::Long,
                FieldType::Int,
            ],
            field_names: vec![],
        }
    }

    fn read_master_table(&self) -> KunaResult<Vec<TableRef>> {
        let root = self.parm(PARM_MASTER_TABLE_ROOT)?;
        if root < 0 {
            return Err(KunaError::bad_data("ghidra db: no master table root"));
        }
        let schema = Self::master_schema();
        let mut tables = Vec::new();
        for rec in self.table_records(root, &schema, None, 0)? {
            let name = rec.columns.first().and_then(FieldValue::as_str).unwrap_or("").to_owned();
            let version = rec.columns.get(1).and_then(FieldValue::as_i64).unwrap_or(0) as i32;
            let root_buffer_id = rec.columns.get(2).and_then(FieldValue::as_i64).unwrap_or(-1) as i32;
            let key_type = rec.columns.get(3).and_then(FieldValue::as_i64).unwrap_or(0) as u8;
            let field_types = rec.columns.get(4).and_then(FieldValue::as_binary).unwrap_or(&[]).to_vec();
            let field_names = rec.columns.get(5).and_then(FieldValue::as_str).unwrap_or("");
            let indexed_column = rec.columns.get(6).and_then(FieldValue::as_i64).unwrap_or(-1) as i32;
            let record_count = rec.columns.get(8).and_then(FieldValue::as_i64).unwrap_or(0) as i32;
            // Index tables (indexed_column >= 0) carry the indexed table's name
            // but an `IndexKey` schema; only primary tables are interesting here.
            if indexed_column >= 0 {
                continue;
            }
            let schema = Schema::decode(version, key_type, &field_types, field_names)?;
            tables.push(TableRef {
                name,
                version,
                root_buffer_id,
                schema,
                indexed_column,
                record_count,
            });
        }
        Ok(tables)
    }

    /// The table with `name` (primary tables only).
    pub fn table(&self, name: &str) -> Option<&TableRef> {
        self.tables.iter().find(|t| t.name == name)
    }

    /// Every primary table.
    pub fn tables(&self) -> &[TableRef] {
        &self.tables
    }

    /// Read every record of `table` in key order.
    pub fn records(&self, table: &TableRef) -> KunaResult<Vec<Record>> {
        if table.root_buffer_id < 0 {
            return Ok(Vec::new());
        }
        self.table_records(table.root_buffer_id, &table.schema, None, 0)
    }

    /// The fixed record length for a `LONGKEY_FIXED_REC_NODE` table, computed
    /// from its schema; `None` for variable-length records.
    fn table_records(
        &self,
        root: i32,
        schema: &Schema,
        fixed_len: Option<usize>,
        depth: u32,
    ) -> KunaResult<Vec<Record>> {
        if depth > 64 {
            return Err(KunaError::bad_data("ghidra db: B-tree deeper than 64 levels"));
        }
        let buf = self.buffer(root)?;
        let node = NodeType::from_byte(buf.u8(0)?)?;
        let key_count = buf.i32(1)?;
        if key_count < 0 {
            return Err(KunaError::bad_data("ghidra db: negative key count"));
        }
        let key_count = key_count as usize;
        let mut out = Vec::new();
        match node {
            NodeType::LongKeyInterior => {
                // | type(1) | keyCount(4) | Key0(8) ID0(4) | ...
                for i in 0..key_count {
                    let base = 5 + i * 12;
                    let child = buf.i32(base + 8)?;
                    out.extend(self.table_records(child, schema, fixed_len, depth + 1)?);
                }
            }
            NodeType::LongKeyVarRec => {
                // | type(1) | keyCount(4) | prev(4) | next(4) |
                //   KeyN(8) RecOffsetN(4) IndFlagN(1) ... | free | RecN ... Rec0 |
                for i in 0..key_count {
                    let base = 13 + i * 13;
                    let key = buf.i64(base)?;
                    let rec_off = buf.i32(base + 8)?;
                    let indirect = buf.u8(base + 12)? != 0;
                    if indirect {
                        return Err(KunaError::bad_data(
                            "ghidra db: indirect (chained-buffer) record storage is not supported",
                        ));
                    }
                    if rec_off < 0 {
                        return Err(KunaError::bad_data("ghidra db: negative record offset"));
                    }
                    let columns = read_record(schema, &buf, rec_off as usize, None)?;
                    out.push(Record { key, columns });
                }
            }
            NodeType::LongKeyFixedRec => {
                let len = match fixed_len {
                    Some(len) => len,
                    None => schema.fixed_record_len().ok_or_else(|| {
                        KunaError::bad_data(
                            "ghidra db: fixed-record node in a variable-length table",
                        )
                    })?,
                };
                let entry = 8 + len;
                for i in 0..key_count {
                    let base = 13 + i * entry;
                    let key = buf.i64(base)?;
                    let columns = read_record(schema, &buf, base + 8, Some(len))?;
                    out.push(Record { key, columns });
                }
            }
            other => {
                return Err(KunaError::bad_data(format!(
                    "ghidra db: unsupported B-tree node {other:?} in a long-key table"
                )))
            }
        }
        Ok(out)
    }

    /// Every buffer id reachable from `root` that is a leaf, used by tests and
    /// diagnostics.
    pub fn leaf_buffers(&self, root: i32) -> KunaResult<Vec<i32>> {
        let mut leaves = Vec::new();
        self.collect_leaves(root, &mut leaves, 0)?;
        Ok(leaves)
    }

    fn collect_leaves(&self, id: i32, out: &mut Vec<i32>, depth: u32) -> KunaResult<()> {
        if depth > 64 {
            return Err(KunaError::bad_data("ghidra db: B-tree deeper than 64 levels"));
        }
        let buf = self.buffer(id)?;
        match NodeType::from_byte(buf.u8(0)?)? {
            NodeType::LongKeyInterior => {
                let n = buf.i32(1)?;
                for i in 0..n.max(0) {
                    self.collect_leaves(buf.i32(5 + i as usize * 12 + 8)?, out, depth + 1)?;
                }
            }
            _ => out.push(id),
        }
        Ok(())
    }
}

/// Decode one record's columns per `schema`, starting at `off`.
fn read_record(
    schema: &Schema,
    buf: &Buf<'_>,
    mut off: usize,
    fixed_len: Option<usize>,
) -> KunaResult<Vec<FieldValue>> {
    let start = off;
    let mut out = Vec::with_capacity(schema.fields.len());
    for &field in &schema.fields {
        let value = match field {
            FieldType::Byte => {
                let v = buf.u8(off)? as i8;
                off += 1;
                FieldValue::Byte(v)
            }
            FieldType::Short => {
                let v = buf.i16(off)?;
                off += 2;
                FieldValue::Short(v)
            }
            FieldType::Int => {
                let v = buf.i32(off)?;
                off += 4;
                FieldValue::Int(v)
            }
            FieldType::Long => {
                let v = buf.i64(off)?;
                off += 8;
                FieldValue::Long(v)
            }
            FieldType::Bool => {
                let v = buf.u8(off)? != 0;
                off += 1;
                FieldValue::Bool(v)
            }
            FieldType::Str => {
                let (v, next) = buf.string(off)?;
                off = next;
                FieldValue::Str(v)
            }
            FieldType::Binary => {
                let (v, next) = buf.binary(off)?;
                off = next;
                FieldValue::Binary(v)
            }
            FieldType::Fixed10 => {
                // FIXED_10_TYPE is a 10-byte opaque value; FID tables never use it.
                let raw = buf.bytes(off, 10)?;
                off += 10;
                FieldValue::Binary(Some(raw.to_vec()))
            }
            FieldType::LegacyIndexLong => {
                let v = buf.i64(off)?;
                off += 8;
                FieldValue::Long(v)
            }
        };
        out.push(value);
    }
    if let Some(len) = fixed_len {
        if off - start != len {
            return Err(KunaError::bad_data(format!(
                "ghidra db: record decoded to {} bytes, schema says {len}",
                off - start
            )));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal Ghidra buffer file: header, `DBParms`, a master table
    /// holding one `Strings Table` record, and one leaf node of that table.
    fn synthetic_db() -> Vec<u8> {
        const BLOCK: usize = 512;
        const BUFFERS: usize = 4; // 0 = DBParms, 1 = master, 2 = strings leaf
        let mut file = vec![0u8; BLOCK * (BUFFERS + 1)];
        let be32 = |v: i32| v.to_be_bytes();
        // Block 0: header.
        file[0..8].copy_from_slice(&BUFFER_FILE_MAGIC.to_be_bytes());
        file[16..20].copy_from_slice(&be32(BUFFER_FILE_HEADER_VERSION));
        file[20..24].copy_from_slice(&be32(BLOCK as i32));
        file[24..28].copy_from_slice(&be32(-1)); // first free index
        file[28..32].copy_from_slice(&be32(0)); // no header parameters
        // `Database::buffer(id)` reads block `id + 1`, prefix stripped.
        let payload = |id: usize| (id + 1) * BLOCK + BUFFER_PREFIX_SIZE;
        // DBParms (user buffer 0): data node, version 1, one parameter.
        let p = payload(0);
        file[p] = 9; // CHAINED_BUFFER_DATA_NODE
        file[p + 1..p + 5].copy_from_slice(&be32(5)); // data length = 1 + 4
        file[p + 5] = 1; // DBParms version
        file[p + 6..p + 10].copy_from_slice(&be32(1)); // master root = buffer 1
        // Master table (user buffer 1): var-rec leaf with one record.
        let m = payload(1);
        file[m] = 1; // LONGKEY_VAR_REC_NODE
        file[m + 1..m + 5].copy_from_slice(&be32(1)); // one key
        file[m + 5..m + 9].copy_from_slice(&be32(-1)); // prev leaf
        file[m + 9..m + 13].copy_from_slice(&be32(-1)); // next leaf
        // entry 0: key 7, record offset, no indirect storage
        let rec_off = 40usize;
        file[m + 13..m + 21].copy_from_slice(&7i64.to_be_bytes());
        file[m + 21..m + 25].copy_from_slice(&be32(rec_off as i32));
        file[m + 25] = 0;
        // the master record, in the field order of `Database::master_schema`
        let mut o = m + rec_off;
        let put_str = |f: &mut [u8], o: &mut usize, s: &str| {
            f[*o..*o + 4].copy_from_slice(&be32(s.len() as i32));
            f[*o + 4..*o + 4 + s.len()].copy_from_slice(s.as_bytes());
            *o += 4 + s.len();
        };
        put_str(&mut file, &mut o, "Strings Table");
        file[o..o + 4].copy_from_slice(&be32(6)); // schema version
        o += 4;
        file[o..o + 4].copy_from_slice(&be32(3)); // root buffer id = 3
        o += 4;
        file[o] = 3; // key type = long
        o += 1;
        file[o..o + 4].copy_from_slice(&be32(1)); // one field type
        o += 4;
        file[o] = 4; // STRING
        o += 1;
        put_str(&mut file, &mut o, "String ID;String Value;");
        file[o..o + 4].copy_from_slice(&be32(-1)); // indexed column (primary)
        o += 4;
        file[o..o + 8].copy_from_slice(&i64::MIN.to_be_bytes()); // max key
        o += 8;
        file[o..o + 4].copy_from_slice(&be32(1)); // record count
        // Strings leaf (user buffer 3): one record.
        let s = payload(3);
        file[s] = 1;
        file[s + 1..s + 5].copy_from_slice(&be32(1));
        file[s + 5..s + 9].copy_from_slice(&be32(-1));
        file[s + 9..s + 13].copy_from_slice(&be32(-1));
        file[s + 13..s + 21].copy_from_slice(&42i64.to_be_bytes());
        file[s + 21..s + 25].copy_from_slice(&be32(30));
        file[s + 25] = 0;
        let mut so = s + 30;
        put_str(&mut file, &mut so, "kuna_reader_test");
        file
    }

    #[test]
    fn a_synthetic_buffer_file_reads_its_table_and_records() {
        let db = Database::parse(synthetic_db()).expect("parse");
        let table = db.table("Strings Table").expect("table");
        assert_eq!(table.schema.field_names, ["String Value"]);
        let records = db.records(table).expect("records");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].key, 42);
        assert_eq!(records[0].columns[0].as_str(), Some("kuna_reader_test"));
    }

    #[test]
    fn a_foreign_file_is_rejected_rather_than_panicking() {
        assert!(Database::parse(vec![0u8; 64]).is_err());
        let mut bad = synthetic_db();
        bad[0] = 0; // break the magic
        assert!(Database::parse(bad).is_err());
        let mut short = synthetic_db();
        short.truncate(40);
        assert!(Database::parse(short).is_err());
    }
}
