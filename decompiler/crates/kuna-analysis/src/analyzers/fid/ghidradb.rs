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
//! - `.../db/ChainedBuffer.java` — the overflow storage for large records.
//!
//! Every read here is bounds-checked and every structural surprise is an
//! `Err`: a foreign or corrupt database must fail to load, never panic.

#[expect(clippy::disallowed_types, reason = "bounded B-tree ownership checks")]
use std::collections::HashSet;

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
        let (text, next) = self.string_ref(off)?;
        Ok((text.map(str::to_owned), next))
    }

    fn string_ref(&self, off: usize) -> KunaResult<(Option<&'a str>, usize)> {
        let len = self.i32(off)?;
        if len == -1 {
            return Ok((None, off + 4));
        }
        if len < 0 {
            return Err(KunaError::bad_data("ghidra db: invalid string length"));
        }
        let len = len as usize;
        let raw = self.bytes(off + 4, len)?;
        let text = std::str::from_utf8(raw)
            .map_err(|_| KunaError::bad_data("ghidra db: non-UTF-8 string field"))?;
        Ok((Some(text), off + 4 + len))
    }

    /// A `BinaryField`: `i32` byte length followed by the bytes, or `-1` for null.
    fn binary(&self, off: usize) -> KunaResult<(Option<Vec<u8>>, usize)> {
        let len = self.i32(off)?;
        if len == -1 {
            return Ok((None, off + 4));
        }
        if len < 0 {
            return Err(KunaError::bad_data("ghidra db: invalid binary field length"));
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

struct TableDescriptor {
    name: String,
    version: i32,
    root_buffer_id: i32,
    key_type: u8,
    field_types: Vec<u8>,
    field_names: String,
    record_count: i32,
}

pub(crate) const PROGRAM_SYMBOLS_VERSION: i32 = 4;

impl TableDescriptor {
    fn decode(self, program: bool) -> KunaResult<TableRef> {
        let unsupported = |reason: &str| KunaError::bad_data(format!(
            "ghidra db: unsupported {} schema (v{}): {reason}", self.name, self.version
        ));
        let mut field_types = self.field_types.as_slice();
        if program && self.name == "Symbols" {
            if self.version != PROGRAM_SYMBOLS_VERSION {
                return Err(unsupported("expected the Ghidra 12.1.2 Symbols schema (v4)"));
            }
            // Schema encodes the sparse column indices from a HashSet.
            let (columns, extension) = field_types.split_at_checked(13)
                .ok_or_else(|| unsupported("short field-type list"))?;
            let valid = extension.strip_prefix(&[0xff, 1]).is_some_and(|indices| {
                indices.len() == 8 && (5..=12).all(|i| indices.contains(&i))
            });
            if !valid {
                return Err(unsupported("invalid sparse column list"));
            }
            field_types = columns;
        }
        let schema = Schema::decode(self.version, self.key_type, field_types, &self.field_names)
            .map_err(|error| if program { unsupported(error.explain()) } else { error })?;
        Ok(TableRef {
            name: self.name,
            version: self.version,
            root_buffer_id: self.root_buffer_id,
            schema,
            indexed_column: -1,
            record_count: self.record_count,
        })
    }
}

/// A view over a raw Ghidra database file (`db.DBHandle` on disk).
pub struct Database<'a> {
    data: &'a [u8],
    block_size: usize,
    buffer_count: usize,
    parms: Vec<i32>,
    tables: Vec<TableRef>,
    master_buffers: HashSet<i32>,
    table_roots: HashSet<i32>,
}

impl<'a> Database<'a> {
    /// Parse a raw buffer file (`LocalBufferFile`). `Err` for a file that is not
    /// a Ghidra database, is truncated, or whose master table cannot be read.
    pub fn parse(data: &'a [u8]) -> KunaResult<Self> {
        let (mut db, tables) = Self::parse_tables(data, None, |table| table.decode(false))?;
        db.tables = tables;
        Ok(db)
    }

    /// Check the saved Program metadata before interpreting version-dependent
    /// schemas. All declared roots are still checked, including skipped tables.
    #[cfg(feature = "ghidra-program")]
    pub(crate) fn parse_program<T>(
        data: &'a [u8],
        tables: &[&str],
        read_program: impl FnOnce(&Self, &mut HashSet<i32>) -> KunaResult<T>,
    ) -> KunaResult<(Self, HashSet<i32>, T)> {
        let (mut db, mut tables) = Self::parse_tables(data, Some(tables), Ok)?;
        db.tables.reserve_exact(tables.len());
        if let Some(index) = tables.iter().position(|table| table.name == "Program") {
            db.tables.push(tables.swap_remove(index).decode(true)?);
        }
        let mut used = db.occupied_buffers();
        let program = read_program(&db, &mut used)?;
        for table in tables {
            db.tables.push(table.decode(true)?);
        }
        Ok((db, used, program))
    }

    fn parse_tables<T>(
        data: &'a [u8],
        program_tables: Option<&[&str]>,
        decode: impl FnMut(TableDescriptor) -> KunaResult<T>,
    ) -> KunaResult<(Self, Vec<T>)> {
        if data.len() < 32 {
            return Err(KunaError::bad_data("ghidra db: file shorter than its header"));
        }
        let head = Buf::new(data);
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
        if block_size < 32 {
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
        let head = Buf::new(&data[..block_size]);

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

        let mut db = Database {
            data,
            block_size,
            buffer_count,
            parms: Vec::new(),
            tables: Vec::new(),
            master_buffers: HashSet::new(),
            table_roots: HashSet::new(),
        };
        db.parms = db.read_db_parms()?;
        let mut used = HashSet::from([0]); // DBParms is already owned.
        let (tables, roots) = db.read_master_table(&mut used, program_tables, decode)?;
        db.table_roots = roots;
        db.master_buffers = used;
        Ok((db, tables))
    }

    /// The payload of user buffer `id` (block `id + 1`, prefix stripped).
    fn buffer(&self, id: i32) -> KunaResult<Buf<'a>> {
        if id < 0 || id as usize >= self.buffer_count {
            return Err(KunaError::bad_data(format!(
                "ghidra db: buffer id {id} outside 0..{}",
                self.buffer_count
            )));
        }
        let start = (id as usize + 1) * self.block_size;
        let block = Buf::new(&self.data[start..start + self.block_size]);
        if block.u8(0)? != 0 || block.i32(1)? != id {
            return Err(KunaError::bad_data(format!(
                "ghidra db: invalid prefix for allocated buffer {id}"
            )));
        }
        Ok(Buf::new(&block.data[BUFFER_PREFIX_SIZE..]))
    }

    fn claim_buffer(
        &self,
        id: i32,
        used: &mut HashSet<i32>,
        allow_table_root: bool,
    ) -> KunaResult<Buf<'a>> {
        let buf = self.buffer(id)?;
        if (!allow_table_root && self.table_roots.contains(&id)) || !used.insert(id) {
            return Err(KunaError::bad_data(format!("ghidra db: repeated buffer {id}")));
        }
        Ok(buf)
    }

    /// Read a VarRecNode overflow record. Unlike general-purpose DBBuffers,
    /// these are written in full and never use obfuscation or sparse storage.
    fn chained_record<'b>(
        &self,
        id: i32,
        used: &mut HashSet<i32>,
        scratch: &'b mut Vec<u8>,
    ) -> KunaResult<&'b [u8]>
    where
        'a: 'b,
    {
        let first = self.claim_buffer(id, used, false)?;
        let len = first.i32(1)?;
        if len <= 0 {
            return Err(KunaError::bad_data("ghidra db: invalid chained record length or obfuscation"));
        }
        let len = len as usize;
        match NodeType::from_byte(first.u8(0)?)? {
            NodeType::ChainedBufferData => return first.bytes(5, len),
            NodeType::ChainedBufferIndex => {}
            _ => return Err(KunaError::bad_data("ghidra db: invalid chained record root")),
        }

        scratch.clear();
        self.indexed_chain(first, len, used, scratch, &[])?;
        Ok(scratch)
    }

    fn indexed_chain(
        &self,
        first: Buf<'a>,
        len: usize,
        used: &mut HashSet<i32>,
        output: &mut Vec<u8>,
        xor: &[u8],
    ) -> KunaResult<()> {
        let prepare = |out: &mut Vec<u8>| {
            out.try_reserve_exact(len)
                .map_err(|_| KunaError::bad_data("ghidra db: cannot allocate chained data"))
        };
        self.walk_indexed_chain(first, len, used, false, output, prepare, |out, _, bytes| {
            append_chain_block(out, bytes, xor);
            Ok(())
        })
    }

    #[expect(clippy::too_many_arguments, reason = "share chain traversal without copying data buffers")]
    fn walk_indexed_chain<T>(
        &self,
        first: Buf<'a>,
        len: usize,
        used: &mut HashSet<i32>,
        sparse: bool,
        context: &mut T,
        prepare: impl FnOnce(&mut T) -> KunaResult<()>,
        mut visit: impl FnMut(&mut T, usize, &[u8]) -> KunaResult<()>,
    ) -> KunaResult<()> {
        let data_space = first.data.len() - 1;
        let indexes_per_buffer = (first.data.len() - 9) / 4;
        let data_count = len.div_ceil(data_space);
        let index_count = data_count.div_ceil(indexes_per_buffer);
        // Every output byte must come from a distinct physical buffer claimed
        // by this record. Bound the allocation by the file before trusting len.
        let required_buffers = index_count - 1 + if sparse { 0 } else { data_count };
        if required_buffers > self.buffer_count - used.len() {
            return Err(KunaError::bad_data("ghidra db: chained record exceeds remaining file buffers"));
        }
        prepare(context)?;
        let mut index = first;
        let mut remaining = data_count;
        let mut bytes_left = len;
        loop {
            let entries = remaining.min(indexes_per_buffer);
            for slot in 0..entries {
                let id = index.i32(9 + slot * 4)?;
                let count = bytes_left.min(data_space);
                if !sparse || id != -1 {
                    let data = self.claim_buffer(id, used, false)?;
                    if NodeType::from_byte(data.u8(0)?)? != NodeType::ChainedBufferData {
                        return Err(KunaError::bad_data("ghidra db: invalid chained record data node"));
                    }
                    visit(context, len - bytes_left, data.bytes(1, count)?)?;
                }
                bytes_left -= count;
            }
            remaining -= entries;
            if remaining == 0 {
                // Ghidra may leave stale links and slots after shrinking a
                // record; only the index entries implied by len are live.
                return Ok(());
            }
            index = self.claim_buffer(index.i32(5)?, used, false)?;
            if NodeType::from_byte(index.u8(0)?)? != NodeType::ChainedBufferIndex {
                return Err(KunaError::bad_data("ghidra db: invalid chained record index node"));
            }
        }
    }

    /// Append one fully initialized File Bytes original chain. The XOR
    /// pattern restarts at the beginning of each physical data payload.
    #[cfg(feature = "ghidra-program")]
    pub(crate) fn append_file_chain(
        &self,
        id: i32,
        remaining: usize,
        used: &mut HashSet<i32>,
        output: &mut Vec<u8>,
        xor: &[u8],
    ) -> KunaResult<usize> {
        let first = self.claim_buffer(id, used, false)?;
        let encoded_len = first.i32(1)?;
        let len = (encoded_len & i32::MAX) as usize;
        if len == 0 || len > remaining {
            return Err(KunaError::bad_data("ghidra db: invalid File Bytes chain length"));
        }
        let xor = if encoded_len < 0 { xor } else { &[] };
        match NodeType::from_byte(first.u8(0)?)? {
            NodeType::ChainedBufferData => {
                let bytes = first.bytes(5, len)?;
                output.try_reserve(len)
                    .map_err(|_| KunaError::bad_data("ghidra db: cannot allocate File Bytes"))?;
                append_chain_block(output, bytes, xor);
            }
            NodeType::ChainedBufferIndex => self.indexed_chain(first, len, used, output, xor)?,
            _ => return Err(KunaError::bad_data("ghidra db: invalid File Bytes chain root")),
        }
        Ok(len)
    }

    /// Visit the stored blocks of a File Bytes overlay. Omitted blocks
    /// inherit the original, and the caller decides which bytes to verify.
    #[cfg(feature = "ghidra-program")]
    pub(crate) fn visit_file_layer(
        &self,
        id: i32,
        len: usize,
        used: &mut HashSet<i32>,
        xor: &[u8],
        mut visit: impl FnMut(usize, &[u8], &[u8]) -> KunaResult<()>,
    ) -> KunaResult<()> {
        let first = self.claim_buffer(id, used, false)?;
        let encoded_len = first.i32(1)?;
        if len == 0 || (encoded_len & i32::MAX) as usize != len {
            return Err(KunaError::bad_data("ghidra db: invalid layered File Bytes length"));
        }
        let xor = if encoded_len < 0 { xor } else { &[] };
        match NodeType::from_byte(first.u8(0)?)? {
            NodeType::ChainedBufferData => visit(0, first.bytes(5, len)?, xor),
            NodeType::ChainedBufferIndex => self.walk_indexed_chain(
                first, len, used, true, &mut (), |_| Ok(()),
                |(), offset, bytes| visit(offset, bytes, xor),
            ),
            _ => Err(KunaError::bad_data("ghidra db: invalid layered File Bytes root")),
        }
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
        if data_len < 1 || (data_len - 1) % 4 != 0 {
            return Err(KunaError::bad_data("ghidra db: invalid DBParms length"));
        }
        buf.bytes(5, data_len as usize)?;
        let count = (data_len as usize - 1) / 4;
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
            key_name: String::new(),
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

    fn read_master_table<T>(
        &self,
        used: &mut HashSet<i32>,
        program_tables: Option<&[&str]>,
        mut decode: impl FnMut(TableDescriptor) -> KunaResult<T>,
    ) -> KunaResult<(Vec<T>, HashSet<i32>)> {
        let root = self.parm(PARM_MASTER_TABLE_ROOT)?;
        if root < 0 {
            return Err(KunaError::bad_data("ghidra db: no master table root"));
        }
        let schema = Self::master_schema();
        let mut tables = Vec::new();
        let mut roots = HashSet::new();
        let mut names = HashSet::new();
        self.visit_tree(root, &schema, None, None, used, |rec| {
            let Ok([FieldValue::Str(Some(name)), FieldValue::Int(version),
                FieldValue::Int(root_buffer_id), FieldValue::Byte(key_type),
                FieldValue::Binary(Some(field_types)), FieldValue::Str(Some(field_names)),
                FieldValue::Int(indexed_column), FieldValue::Long(_), FieldValue::Int(record_count)]) =
                <[_; 9]>::try_from(rec.columns)
            else {
                return Err(KunaError::bad_data("ghidra db: invalid master-table record"));
            };
            if root_buffer_id >= 0
                && (root_buffer_id == 0 || root_buffer_id as usize >= self.buffer_count
                    || !roots.insert(root_buffer_id))
            {
                return Err(KunaError::bad_data(format!(
                    "ghidra db: invalid or repeated table root {root_buffer_id} for {name}"
                )));
            }
            // Index tables share a primary table's name but use an IndexKey schema.
            if indexed_column >= 0 {
                return Ok(());
            }
            if name.is_empty() || !names.insert(name.clone()) || indexed_column != -1
                || record_count < 0 || root_buffer_id < -1
                || (root_buffer_id == -1 && record_count != 0)
            {
                return Err(KunaError::bad_data(format!(
                    "ghidra db: invalid primary table metadata for {name:?}"
                )));
            }
            if program_tables.is_some_and(|tables| !tables.contains(&name.as_str())) {
                return Ok(());
            }
            tables.push(decode(TableDescriptor {
                name,
                version,
                root_buffer_id,
                key_type: key_type as u8,
                field_types,
                field_names,
                record_count,
            })?);
            Ok(())
        })?;
        if !roots.is_disjoint(used) {
            return Err(KunaError::bad_data("ghidra db: table root reuses a master table buffer"));
        }
        Ok((tables, roots))
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
        let mut records = Vec::new();
        self.visit_records(table, &mut self.occupied_buffers(), |record| {
            records.push(record);
            Ok(())
        })?;
        Ok(records)
    }

    /// Seed the ownership set for a scan of one or more primary tables.
    pub(crate) fn occupied_buffers(&self) -> HashSet<i32> {
        self.master_buffers.clone()
    }

    pub(crate) fn visit_records(
        &self,
        table: &TableRef,
        used: &mut HashSet<i32>,
        visit: impl FnMut(Record) -> KunaResult<()>,
    ) -> KunaResult<()> {
        self.visit_long_records(table, None, used, visit)
    }

    #[cfg(feature = "ghidra-program")]
    pub(crate) fn visit_program_symbols(
        &self,
        table: &TableRef,
        used: &mut HashSet<i32>,
        visit: impl FnMut(Record) -> KunaResult<()>,
    ) -> KunaResult<()> {
        self.visit_long_records(table, Some(5), used, visit)
    }

    fn visit_long_records(
        &self,
        table: &TableRef,
        sparse_from: Option<usize>,
        used: &mut HashSet<i32>,
        visit: impl FnMut(Record) -> KunaResult<()>,
    ) -> KunaResult<()> {
        let expected = usize::try_from(table.record_count)
            .map_err(|_| KunaError::bad_data("ghidra db: negative table record count"))?;
        if table.root_buffer_id == -1 && expected == 0 {
            return Ok(());
        }
        self.visit_tree(table.root_buffer_id, &table.schema, Some(expected), sparse_from, used, visit)
    }

    fn visit_tree(
        &self,
        root: i32,
        schema: &Schema,
        expected: Option<usize>,
        sparse_from: Option<usize>,
        used: &mut HashSet<i32>,
        mut visit: impl FnMut(Record) -> KunaResult<()>,
    ) -> KunaResult<()> {
        if schema.key_field != FieldType::Long {
            return Err(KunaError::bad_data("ghidra db: only long primary keys are supported"));
        }
        let fixed_len = if sparse_from.is_some() { None } else { schema.fixed_record_len() };
        let mut last_key = None;
        let mut count = 0;
        let mut scratch = Vec::new();
        self.walk_leaves(root, FieldType::Long, 0, used, &mut |_, buf, node, key_count, used| {
            let (entry_size, variable) = match (node, fixed_len) {
                (NodeType::LongKeyFixedRec, Some(len)) => (8 + len, false),
                (NodeType::LongKeyVarRec, None) => (13, true),
                _ => return Err(KunaError::bad_data("ghidra db: leaf does not match table schema")),
            };
            // Each entry needs its own bytes. Variable records additionally occupy
            // disjoint slices following the entry array, packed in reverse order.
            validate_entries(&buf, 13, entry_size, key_count)?;
            count += key_count;
            if expected.is_some_and(|n| count > n) {
                return Err(KunaError::bad_data("ghidra db: more records than the table declares"));
            }
            let metadata_end = 13 + key_count * entry_size;
            let mut data_end = buf.data.len();
            for i in 0..key_count {
                let base = 13 + i * entry_size;
                let key = buf.i64(base)?;
                if last_key.is_some_and(|previous| key <= previous) {
                    return Err(KunaError::bad_data("ghidra db: out-of-order or duplicate key"));
                }
                last_key = Some(key);
                let data = if variable {
                    let offset = buf.i32(base + 8)? as usize;
                    if offset < metadata_end || offset >= data_end {
                        return Err(KunaError::bad_data("ghidra db: overlapping or invalid record offset"));
                    }
                    let data = &buf.data[offset..data_end];
                    data_end = offset;
                    if buf.u8(base + 12)? != 0 {
                        let id = data.try_into().map(i32::from_be_bytes).map_err(|_| {
                            KunaError::bad_data("ghidra db: invalid chained record reference span")
                        })?;
                        self.chained_record(id, used, &mut scratch)?
                    } else {
                        data
                    }
                } else {
                    buf.bytes(base + 8, entry_size - 8)?
                };
                let columns = read_record(schema, &Buf::new(data), sparse_from)?;
                visit(Record { key, columns })?;
            }
            Ok(())
        })?;
        if expected.is_some_and(|n| count != n) {
            return Err(KunaError::bad_data("ghidra db: fewer records than the table declares"));
        }
        Ok(())
    }

    /// Scan the String-keyed Program and Property Table metadata.
    #[cfg(feature = "ghidra-program")]
    pub(crate) fn visit_program_strings(
        &self,
        table: &TableRef,
        used: &mut HashSet<i32>,
        mut visit: impl FnMut(&str, Vec<FieldValue>) -> KunaResult<()>,
    ) -> KunaResult<()> {
        if table.schema.key_field != FieldType::Str {
            return Err(KunaError::bad_data("ghidra db: expected string primary keys"));
        }
        let expected = usize::try_from(table.record_count)
            .map_err(|_| KunaError::bad_data("ghidra db: negative table record count"))?;
        if table.root_buffer_id == -1 && expected == 0 {
            return Ok(());
        }
        let mut count = 0;
        let mut last_key: Option<String> = None;
        let mut scratch = Vec::new();
        self.walk_leaves(table.root_buffer_id, FieldType::Str, 0, used,
            &mut |_, buf, _, key_count, used| {
                const HEADER: usize = 14;
                const ENTRY: usize = 5;
                let metadata_end = HEADER + key_count * ENTRY;
                let mut data_end = buf.data.len();
                count += key_count;
                if count > expected {
                    return Err(KunaError::bad_data("ghidra db: more records than the table declares"));
                }
                for i in 0..key_count {
                    let base = HEADER + i * ENTRY;
                    let offset = buf.i32(base)? as usize;
                    if offset < metadata_end || offset >= data_end {
                        return Err(KunaError::bad_data("ghidra db: invalid string record offset"));
                    }
                    let span = Buf::new(&buf.data[offset..data_end]);
                    data_end = offset;
                    let (key, start) = span.string(0)?;
                    let key = key.ok_or_else(|| KunaError::bad_data("ghidra db: null string key"))?;
                    if last_key.as_ref().is_some_and(|prev| prev.encode_utf16().ge(key.encode_utf16())) {
                        return Err(KunaError::bad_data("ghidra db: out-of-order or duplicate string key"));
                    }
                    let data = &span.data[start..];
                    let data = match buf.u8(base + 4)? {
                        0 => data,
                        1 => {
                            let id = data.try_into().map(i32::from_be_bytes).map_err(|_| {
                                KunaError::bad_data("ghidra db: invalid chained string record reference")
                            })?;
                            self.chained_record(id, used, &mut scratch)?
                        }
                        _ => return Err(KunaError::bad_data("ghidra db: invalid string-record flag")),
                    };
                    visit(&key, read_record(&table.schema, &Buf::new(data), None)?)?;
                    last_key = Some(key);
                }
                Ok(())
            },
        )?;
        if count != expected {
            return Err(KunaError::bad_data("ghidra db: fewer records than the table declares"));
        }
        Ok(())
    }

    /// Every buffer id reachable from `root` that is a leaf.
    pub fn leaf_buffers(&self, root: i32) -> KunaResult<Vec<i32>> {
        let mut leaves = Vec::new();
        self.walk_leaves(root, FieldType::Long, 0, &mut HashSet::new(), &mut |id, _, _, _, _| {
            leaves.push(id);
            Ok(())
        })?;
        Ok(leaves)
    }

    fn walk_leaves(
        &self,
        id: i32,
        key_field: FieldType,
        depth: u32,
        visited: &mut HashSet<i32>,
        visit: &mut impl FnMut(i32, Buf<'a>, NodeType, usize, &mut HashSet<i32>) -> KunaResult<()>,
    ) -> KunaResult<()> {
        if depth > 64 {
            return Err(KunaError::bad_data("ghidra db: B-tree deeper than 64 levels"));
        }
        let buf = self.claim_buffer(id, visited, depth == 0)?;
        let node = NodeType::from_byte(buf.u8(0)?)?;
        let count_offset = if cfg!(feature = "ghidra-program") && key_field == FieldType::Str {
            if buf.u8(1)? != key_field.code() {
                return Err(KunaError::bad_data("ghidra db: invalid string node key type"));
            }
            2
        } else {
            1
        };
        let key_count = usize::try_from(buf.i32(count_offset)?)
            .map_err(|_| KunaError::bad_data("ghidra db: negative key count"))?;
        match (key_field, node) {
            (FieldType::Long, NodeType::LongKeyInterior) => {
                validate_entries(&buf, 5, 12, key_count)?;
                for i in 0..key_count {
                    self.walk_leaves(buf.i32(5 + i * 12 + 8)?, key_field, depth + 1, visited, visit)?;
                }
                Ok(())
            }
            (FieldType::Long, NodeType::LongKeyVarRec | NodeType::LongKeyFixedRec) => {
                let min_entry = if node == NodeType::LongKeyVarRec { 13 } else { 8 };
                validate_entries(&buf, 13, min_entry, key_count)?;
                visit(id, buf, node, key_count, visited)
            }
            #[cfg(feature = "ghidra-program")]
            (FieldType::Str, NodeType::VarKeyInterior) => {
                const HEADER: usize = 6;
                const ENTRY: usize = 8;
                validate_entries(&buf, HEADER, ENTRY, key_count)?;
                let mut end = buf.data.len();
                let mut previous: Option<String> = None;
                for i in 0..key_count {
                    let base = HEADER + i * ENTRY;
                    let offset = buf.i32(base)? as usize;
                    if offset < HEADER + key_count * ENTRY || offset >= end {
                        return Err(KunaError::bad_data("ghidra db: invalid string separator offset"));
                    }
                    let span = Buf::new(&buf.data[offset..end]);
                    let (key, next) = span.string(0)?;
                    let key = key.ok_or_else(|| KunaError::bad_data("ghidra db: null string separator"))?;
                    if next != span.data.len()
                        || previous.as_ref().is_some_and(|prev| prev.encode_utf16().ge(key.encode_utf16()))
                    {
                        return Err(KunaError::bad_data("ghidra db: invalid string separator"));
                    }
                    previous = Some(key);
                    end = offset;
                    self.walk_leaves(buf.i32(base + 4)?, key_field, depth + 1, visited, visit)?;
                }
                Ok(())
            }
            #[cfg(feature = "ghidra-program")]
            (FieldType::Str, NodeType::VarKeyRecord) => {
                validate_entries(&buf, 14, 5, key_count)?;
                visit(id, buf, node, key_count, visited)
            }
            (_, other) => Err(KunaError::bad_data(format!(
                "ghidra db: unsupported B-tree node {other:?} for {key_field:?} keys"
            ))),
        }
    }
}

fn validate_entries(buf: &Buf<'_>, start: usize, len: usize, count: usize) -> KunaResult<()> {
    if start > buf.data.len() || count == 0 || count > (buf.data.len() - start) / len {
        return Err(KunaError::bad_data("ghidra db: invalid B-tree key count"));
    }
    Ok(())
}

fn append_chain_block(output: &mut Vec<u8>, bytes: &[u8], xor: &[u8]) {
    if !cfg!(feature = "ghidra-program") || xor.is_empty() {
        output.extend_from_slice(bytes);
    } else {
        output.extend(bytes.iter().zip(xor.iter().cycle()).map(|(b, x)| b ^ x));
    }
}

/// Decode dense columns, and validate but do not retain the Symbols v4 sparse tail.
fn read_record(
    schema: &Schema,
    buf: &Buf<'_>,
    sparse_from: Option<usize>,
) -> KunaResult<Vec<FieldValue>> {
    let dense = sparse_from.unwrap_or(schema.fields.len());
    if dense > schema.fields.len() || dense > buf.data.len() {
        return Err(KunaError::bad_data("ghidra db: record shorter than the table schema"));
    }
    let mut off = 0;
    let mut out = Vec::with_capacity(dense);
    for &field in &schema.fields[..dense] {
        out.push(read_field(buf, &mut off, field)?);
    }
    #[cfg(feature = "ghidra-program")]
    if sparse_from.is_some() {
        let count = usize::from(buf.u8(off)?);
        off += 1;
        let mut next_column = dense;
        for _ in 0..count {
            let column = usize::from(buf.u8(off)?);
            off += 1;
            if column < next_column || column >= schema.fields.len() {
                return Err(KunaError::bad_data("ghidra db: invalid sparse record column"));
            }
            next_column = column + 1;
            let field = schema.fields[column];
            if let Some(len) = field.fixed_size() {
                buf.bytes(off, len)?;
                off += len;
            } else if field == FieldType::Str {
                let (_, next) = buf.string_ref(off)?;
                off = next;
            } else {
                return Err(KunaError::bad_data("ghidra db: unsupported sparse Symbols field type"));
            }
        }
    }
    if off != buf.data.len() {
        return Err(KunaError::bad_data("ghidra db: record length does not match the table schema"));
    }
    Ok(out)
}

fn read_field(buf: &Buf<'_>, offset: &mut usize, field: FieldType) -> KunaResult<FieldValue> {
    let mut off = *offset;
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
        FieldType::Long | FieldType::LegacyIndexLong => {
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
    };
    *offset = off;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLOCK: usize = 512;
    const PAYLOAD: usize = BLOCK - BUFFER_PREFIX_SIZE;

    fn payload(id: usize) -> usize {
        (id + 1) * BLOCK + BUFFER_PREFIX_SIZE
    }

    /// Build a minimal Ghidra buffer file: header, `DBParms`, a master table
    /// holding one `Strings Table` record, and one leaf node of that table.
    fn synthetic_db() -> Vec<u8> {
        const BUFFERS: usize = 4; // 0 = DBParms, 1 = master, 3 = strings leaf
        let mut file = vec![0u8; BLOCK * (BUFFERS + 1)];
        let be32 = |v: i32| v.to_be_bytes();
        // Block 0: header.
        file[0..8].copy_from_slice(&BUFFER_FILE_MAGIC.to_be_bytes());
        file[16..20].copy_from_slice(&be32(BUFFER_FILE_HEADER_VERSION));
        file[20..24].copy_from_slice(&be32(BLOCK as i32));
        file[24..28].copy_from_slice(&be32(-1)); // first free index
        file[28..32].copy_from_slice(&be32(0)); // no header parameters
        for id in 0..BUFFERS {
            let p = payload(id);
            file[p - 4..p].copy_from_slice(&be32(id as i32));
        }
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
        let rec_end = o + 4;
        let packed_rec_off = BLOCK - BUFFER_PREFIX_SIZE - (rec_end - (m + rec_off));
        file.copy_within(m + rec_off..rec_end, m + packed_rec_off);
        file[m + 21..m + 25].copy_from_slice(&be32(packed_rec_off as i32));
        // Strings leaf (user buffer 3): one record.
        let s = payload(3);
        file[s] = 1;
        file[s + 1..s + 5].copy_from_slice(&be32(1));
        file[s + 5..s + 9].copy_from_slice(&be32(-1));
        file[s + 9..s + 13].copy_from_slice(&be32(-1));
        file[s + 13..s + 21].copy_from_slice(&42i64.to_be_bytes());
        let name = "kuna_reader_test";
        let mut so = (s + BLOCK - BUFFER_PREFIX_SIZE) - (4 + name.len());
        file[s + 21..s + 25].copy_from_slice(&be32((so - s) as i32));
        put_str(&mut file, &mut so, name);
        file
    }

    fn indexed_string_db() -> (Vec<u8>, String) {
        // Enough data nodes to require a second index node at this block size.
        let name = "nested_模板::".repeat(5000) + "probe";
        let mut record = Vec::with_capacity(4 + name.len());
        record.extend((name.len() as i32).to_be_bytes());
        record.extend_from_slice(name.as_bytes());
        let data_space = PAYLOAD - 1;
        let entries_per_index = (PAYLOAD - 9) / 4;
        let data_count = record.len().div_ceil(data_space);
        assert!(data_count > entries_per_index && data_count <= 2 * entries_per_index);

        let mut file = synthetic_db();
        const FIRST_INDEX: usize = 4;
        const SECOND_INDEX: usize = 5;
        const FIRST_DATA: usize = 6;
        file.resize((FIRST_DATA + data_count + 1) * BLOCK, 0);
        for id in FIRST_INDEX..FIRST_DATA + data_count {
            let p = payload(id);
            file[p - 4..p].copy_from_slice(&(id as i32).to_be_bytes());
        }
        let s = payload(3);
        file[s + 21..s + 25].copy_from_slice(&((PAYLOAD - 4) as i32).to_be_bytes());
        file[s + 25] = 1;
        file[s + PAYLOAD - 4..s + PAYLOAD].copy_from_slice(&(FIRST_INDEX as i32).to_be_bytes());

        let first = payload(FIRST_INDEX);
        let second = payload(SECOND_INDEX);
        file[first] = 8;
        file[first + 1..first + 5].copy_from_slice(&(record.len() as i32).to_be_bytes());
        file[first + 5..first + 9].copy_from_slice(&(SECOND_INDEX as i32).to_be_bytes());
        file[second] = 8;
        file[second + 1..second + 5].copy_from_slice(&(-1i32).to_be_bytes());
        file[second + 5..second + 9].copy_from_slice(&(-1i32).to_be_bytes());
        for (n, chunk) in record.chunks(data_space).enumerate() {
            let id = FIRST_DATA + n;
            let data = payload(id);
            file[data] = 9;
            file[data + 1..data + 1 + chunk.len()].copy_from_slice(chunk);
            let index = if n < entries_per_index { first } else { second };
            let entry = index + 9 + 4 * (n % entries_per_index);
            file[entry..entry + 4].copy_from_slice(&(id as i32).to_be_bytes());
        }
        (file, name)
    }

    #[test]
    fn a_record_spans_multiple_index_and_data_nodes() {
        let (mut bytes, name) = indexed_string_db();
        // The length, not a potentially stale terminal link, defines the chain.
        let terminal_next = payload(5) + 5;
        bytes[terminal_next..terminal_next + 4].copy_from_slice(&4i32.to_be_bytes());
        let db = Database::parse(&bytes).unwrap();
        let records = db.records(db.table("Strings Table").unwrap()).unwrap();
        assert_eq!(records[0].columns[0].as_str(), Some(name.as_str()));
    }

    #[test]
    fn a_shortened_record_can_retain_its_index_node() {
        let (mut bytes, _) = indexed_string_db();
        let name = "nested_模板::";
        let first = payload(4);
        let data = payload(6);
        bytes[first + 1..first + 5].copy_from_slice(&(4 + name.len() as i32).to_be_bytes());
        bytes[data + 1..data + 5].copy_from_slice(&(name.len() as i32).to_be_bytes());
        let db = Database::parse(&bytes).unwrap();
        let records = db.records(db.table("Strings Table").unwrap()).unwrap();
        assert_eq!(records[0].columns[0].as_str(), Some(name));
    }

    #[test]
    fn overflow_chains_require_live_distinct_buffers_and_exact_lengths() {
        let (bytes, _) = indexed_string_db();
        let first = payload(4);
        let data = payload(6);
        let string_len = i32::from_be_bytes(bytes[data + 1..data + 5].try_into().unwrap());
        for (offset, value, expected) in [
            (first + 1, i32::MAX, "remaining file buffers"),
            (first + 5, 4, "repeated buffer"),
            (first + 9, -1, "outside"),
            (first + 13, 6, "repeated buffer"),
            (first + 13, 3, "repeated buffer"),
            (data + 1, string_len - 1, "record length does not match"),
        ] {
            let mut bytes = bytes.clone();
            bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
            let db = Database::parse(&bytes).unwrap();
            let err = db.records(db.table("Strings Table").unwrap()).unwrap_err();
            assert!(err.explain().contains(expected), "{}", err.explain());
        }
    }

    #[test]
    fn a_synthetic_buffer_file_reads_its_table_and_records() {
        let bytes = synthetic_db();
        let db = Database::parse(&bytes).expect("parse");
        let table = db.table("Strings Table").expect("table");
        assert_eq!(table.schema.field_names, ["String Value"]);
        let records = db.records(table).expect("records");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].key, 42);
        assert_eq!(records[0].columns[0].as_str(), Some("kuna_reader_test"));
    }

    #[test]
    fn a_foreign_file_is_rejected_rather_than_panicking() {
        assert!(Database::parse(&[0u8; 64]).is_err());
        let mut bad = synthetic_db();
        bad[0] = 0; // break the magic
        assert!(Database::parse(&bad).is_err());
        let mut short = synthetic_db();
        short.truncate(40);
        assert!(Database::parse(&short).is_err());
    }

    #[test]
    fn record_walk_and_leaf_listing_reject_reused_nodes() {
        let node = 4 * BLOCK + BUFFER_PREFIX_SIZE;
        for child in [2i32, 3] {
            let mut bytes = synthetic_db();
            // Buffer 2 contains the genuine leaf, but the root either repeats
            // its child or links to itself.
            bytes.copy_within(node..5 * BLOCK, node - BLOCK);
            bytes[node] = 0;
            bytes[node + 1..node + 5].copy_from_slice(&2i32.to_be_bytes());
            for slot in [13, 25] {
                bytes[node + slot..node + slot + 4].copy_from_slice(&child.to_be_bytes());
            }
            let db = Database::parse(&bytes).unwrap();
            assert!(db.leaf_buffers(3).unwrap_err().explain().contains("repeated"));
            assert!(db.records(db.table("Strings Table").unwrap())
                .unwrap_err().explain().contains("repeated"));
        }
    }

    #[test]
    fn primary_table_scans_share_buffer_ownership() {
        let bytes = synthetic_db();
        let db = Database::parse(&bytes).unwrap();
        let table = db.table("Strings Table").unwrap();
        let mut used = db.occupied_buffers();
        db.visit_records(table, &mut used, |_| Ok(())).unwrap();
        let error = db.visit_records(table, &mut used, |_| Ok(())).unwrap_err();
        assert!(error.explain().contains("repeated"));
    }

    #[test]
    fn table_record_counts_and_spans_are_validated() {
        let mut bytes = synthetic_db();
        let db = Database::parse(&bytes).unwrap();
        let mut table = db.table("Strings Table").unwrap().clone();
        table.record_count = 2;
        assert!(db.records(&table).unwrap_err().explain().contains("fewer records"));

        let node = 4 * BLOCK + BUFFER_PREFIX_SIZE;
        bytes[node + 21..node + 25].copy_from_slice(&13i32.to_be_bytes());
        let db = Database::parse(&bytes).unwrap();
        assert!(db.records(db.table("Strings Table").unwrap()).is_err());
    }
}
