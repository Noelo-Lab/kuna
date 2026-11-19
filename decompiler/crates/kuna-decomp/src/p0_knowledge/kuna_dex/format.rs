//! (kuna `dexmodel`) The DEX container format, read straight from the bytes.
//!
//! Follows the format document (source.android.com/docs/core/runtime/dex-format):
//! the header, the five id tables, the class definitions with their
//! `class_data_item`, and each method's `code_item`. Every offset is checked
//! against the file, so a truncated or hostile file is an error, not a panic.
//! Strings are decoded from MUTF-8 when asked for.

use kuna_base::error::{KunaError, KunaResult};

/// `NO_INDEX`: an absent type or string reference.
pub const NO_INDEX: u32 = 0xffff_ffff;
/// `ACC_STATIC`.
pub const ACC_STATIC: u32 = 0x8;
/// `ACC_CONSTRUCTOR`.
pub const ACC_CONSTRUCTOR: u32 = 0x1_0000;

const HEADER_SIZE: usize = 0x70;
const ENDIAN_CONSTANT: u32 = 0x1234_5678;

/// Is `bytes` a DEX file: `dex\n` followed by a three-digit version and NUL?
pub fn is_dex(bytes: &[u8]) -> bool {
    bytes.len() >= HEADER_SIZE
        && bytes.starts_with(b"dex\n")
        && bytes[4..7].iter().all(u8::is_ascii_digit)
        && bytes[7] == 0
}

/// One `method_id_item`.
#[derive(Clone, Copy, Debug)]
pub struct MethodId {
    pub class_idx: u32,
    pub proto_idx: u32,
    pub name_idx: u32,
}

/// One `field_id_item`.
#[derive(Clone, Copy, Debug)]
pub struct FieldId {
    pub class_idx: u32,
    pub type_idx: u32,
    pub name_idx: u32,
}

/// One `proto_id_item` with its parameter list.
#[derive(Clone, Debug)]
pub struct ProtoId {
    pub return_type_idx: u32,
    pub parameters: Vec<u32>,
}

/// The header fields of a `code_item`, and where its instructions are.
#[derive(Clone, Copy, Debug)]
pub struct CodeItem {
    pub registers_size: u16,
    pub ins_size: u16,
    pub outs_size: u16,
    pub tries_size: u16,
    /// File offset of the first instruction unit.
    pub insns_off: u32,
    /// Length of the instruction array in 16-bit units.
    pub insns_units: u32,
}

impl CodeItem {
    /// File offset of the `try_item` array, which follows the instructions
    /// (padded to four bytes).
    pub fn tries_off(&self) -> usize {
        let end = self.insns_off as usize + self.insns_units as usize * 2;
        (end + 3) & !3
    }
}

/// One catch clause of a `try_item`: the exception type (`None` for the
/// catch-all) and the handler, in 16-bit units from the method's first
/// instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CatchHandler {
    pub type_idx: Option<u32>,
    pub addr: u32,
}

/// One `try_item`: the covered instructions, in 16-bit units from the method's
/// first instruction, and its catch clauses in match order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TryItem {
    pub start: u32,
    pub count: u16,
    pub handlers: Vec<CatchHandler>,
}

/// One method from a `class_data_item`.
#[derive(Clone, Copy, Debug)]
pub struct EncodedMethod {
    pub method_idx: u32,
    pub access_flags: u32,
    pub code: Option<CodeItem>,
}

impl EncodedMethod {
    pub fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC != 0
    }
}

/// One `class_def_item` and its methods.
#[derive(Clone, Debug)]
pub struct ClassDef {
    pub class_idx: u32,
    pub access_flags: u32,
    pub superclass_idx: u32,
    pub methods: Vec<EncodedMethod>,
}

#[derive(Clone, Copy, Debug, Default)]
struct Table {
    off: usize,
    count: usize,
}

/// A parsed DEX file. Owns its bytes; the id tables are read on demand.
#[derive(Debug)]
pub struct DexFile {
    bytes: Vec<u8>,
    /// The three-digit format version (`35` for `dex\n035\0`).
    pub version: u32,
    strings: Table,
    types: Table,
    protos: Table,
    fields: Table,
    methods: Table,
    pub classes: Vec<ClassDef>,
}

fn bad(what: &str) -> KunaError {
    KunaError::lowlevel(format!("malformed DEX file: {what}"))
}

fn u16_at(bytes: &[u8], off: usize) -> KunaResult<u16> {
    bytes
        .get(off..off + 2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .ok_or_else(|| bad("read past the end of the file"))
}

fn u32_at(bytes: &[u8], off: usize) -> KunaResult<u32> {
    bytes
        .get(off..off + 4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or_else(|| bad("read past the end of the file"))
}

fn uleb128(bytes: &[u8], pos: &mut usize) -> KunaResult<u32> {
    let mut result: u32 = 0;
    for shift in (0..35).step_by(7) {
        let byte = *bytes.get(*pos).ok_or_else(|| bad("truncated uleb128"))?;
        *pos += 1;
        result |= u32::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
    }
    Err(bad("uleb128 longer than five bytes"))
}

fn sleb128(bytes: &[u8], pos: &mut usize) -> KunaResult<i32> {
    let mut result: u32 = 0;
    let mut shift = 0;
    loop {
        let byte = *bytes.get(*pos).ok_or_else(|| bad("truncated sleb128"))?;
        *pos += 1;
        result |= u32::from(byte & 0x7f) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            if shift < 32 && byte & 0x40 != 0 {
                result |= u32::MAX << shift;
            }
            return Ok(result as i32);
        }
        if shift >= 35 {
            return Err(bad("sleb128 longer than five bytes"));
        }
    }
}

/// Decode a NUL-terminated MUTF-8 string (CESU-8 surrogates, `C0 80` for NUL).
fn mutf8(bytes: &[u8]) -> String {
    let mut units: Vec<u16> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i];
        if b0 == 0 {
            break;
        }
        let cont = |k: usize| bytes.get(i + k).map_or(0, |b| u16::from(b & 0x3f));
        if b0 < 0x80 {
            units.push(u16::from(b0));
            i += 1;
        } else if b0 & 0xe0 == 0xc0 {
            units.push((u16::from(b0 & 0x1f) << 6) | cont(1));
            i += 2;
        } else if b0 & 0xf0 == 0xe0 {
            units.push((u16::from(b0 & 0x0f) << 12) | (cont(1) << 6) | cont(2));
            i += 3;
        } else {
            units.push(0xfffd);
            i += 1;
        }
    }
    String::from_utf16_lossy(&units)
}

impl DexFile {
    /// Parse the header, the class definitions and their code items.
    pub fn parse(bytes: Vec<u8>) -> KunaResult<DexFile> {
        if !is_dex(&bytes) {
            return Err(bad("no dex magic"));
        }
        let version = (bytes[4] - b'0') as u32 * 100
            + (bytes[5] - b'0') as u32 * 10
            + (bytes[6] - b'0') as u32;
        if u32_at(&bytes, 0x28)? != ENDIAN_CONSTANT {
            return Err(bad("unsupported endian tag"));
        }
        let table = |at: usize, width: usize, what: &str| -> KunaResult<Table> {
            let count = u32_at(&bytes, at)? as usize;
            let off = u32_at(&bytes, at + 4)? as usize;
            let end = count
                .checked_mul(width)
                .and_then(|len| off.checked_add(len))
                .ok_or_else(|| bad(what))?;
            if count != 0 && end > bytes.len() {
                return Err(bad(what));
            }
            Ok(Table { off, count })
        };
        let strings = table(0x38, 4, "string_ids")?;
        let types = table(0x40, 4, "type_ids")?;
        let protos = table(0x48, 12, "proto_ids")?;
        let fields = table(0x50, 8, "field_ids")?;
        let methods = table(0x58, 8, "method_ids")?;
        let class_defs = table(0x60, 32, "class_defs")?;
        let mut dex = DexFile {
            bytes,
            version,
            strings,
            types,
            protos,
            fields,
            methods,
            classes: Vec::with_capacity(class_defs.count),
        };
        for index in 0..class_defs.count {
            let class = dex.read_class_def(class_defs.off + index * 32)?;
            dex.classes.push(class);
        }
        Ok(dex)
    }

    /// The whole file.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn read_class_def(&self, at: usize) -> KunaResult<ClassDef> {
        let b = &self.bytes;
        let mut class = ClassDef {
            class_idx: u32_at(b, at)?,
            access_flags: u32_at(b, at + 4)?,
            superclass_idx: u32_at(b, at + 8)?,
            methods: Vec::new(),
        };
        let data_off = u32_at(b, at + 24)? as usize;
        if data_off == 0 {
            return Ok(class);
        }
        let mut pos = data_off;
        let static_fields = uleb128(b, &mut pos)?;
        let instance_fields = uleb128(b, &mut pos)?;
        let direct = uleb128(b, &mut pos)?;
        let virtual_ = uleb128(b, &mut pos)?;
        for _ in 0..u64::from(static_fields) + u64::from(instance_fields) {
            uleb128(b, &mut pos)?;
            uleb128(b, &mut pos)?;
        }
        for group in [direct, virtual_] {
            let mut method_idx: u32 = 0;
            for _ in 0..group {
                method_idx = method_idx.wrapping_add(uleb128(b, &mut pos)?);
                let access_flags = uleb128(b, &mut pos)?;
                let code_off = uleb128(b, &mut pos)? as usize;
                let code = if code_off == 0 { None } else { Some(self.read_code_item(code_off)?) };
                class.methods.push(EncodedMethod { method_idx, access_flags, code });
            }
        }
        Ok(class)
    }

    fn read_code_item(&self, at: usize) -> KunaResult<CodeItem> {
        let b = &self.bytes;
        let item = CodeItem {
            registers_size: u16_at(b, at)?,
            ins_size: u16_at(b, at + 2)?,
            outs_size: u16_at(b, at + 4)?,
            tries_size: u16_at(b, at + 6)?,
            insns_units: u32_at(b, at + 12)?,
            insns_off: (at + 16) as u32,
        };
        let end = (item.insns_off as u64) + u64::from(item.insns_units) * 2;
        if end > b.len() as u64 || item.ins_size > item.registers_size {
            return Err(bad("code_item"));
        }
        Ok(item)
    }

    /// The `try_item`s of a code item with their catch clauses.
    pub fn tries(&self, code: &CodeItem) -> KunaResult<Vec<TryItem>> {
        let b = &self.bytes;
        let tries_off = code.tries_off();
        let list_off = tries_off + usize::from(code.tries_size) * 8;
        let mut out = Vec::with_capacity(usize::from(code.tries_size));
        for k in 0..usize::from(code.tries_size) {
            let at = tries_off + k * 8;
            let start = u32_at(b, at)?;
            let count = u16_at(b, at + 4)?;
            let mut pos = list_off + usize::from(u16_at(b, at + 6)?);
            let size = sleb128(b, &mut pos)?;
            let mut handlers = Vec::new();
            for _ in 0..size.unsigned_abs().min(0xffff) {
                let type_idx = uleb128(b, &mut pos)?;
                let addr = uleb128(b, &mut pos)?;
                handlers.push(CatchHandler { type_idx: Some(type_idx), addr });
            }
            if size <= 0 {
                handlers.push(CatchHandler { type_idx: None, addr: uleb128(b, &mut pos)? });
            }
            out.push(TryItem { start, count, handlers });
        }
        Ok(out)
    }

    pub fn num_strings(&self) -> usize {
        self.strings.count
    }
    pub fn num_types(&self) -> usize {
        self.types.count
    }
    pub fn num_methods(&self) -> usize {
        self.methods.count
    }
    pub fn num_fields(&self) -> usize {
        self.fields.count
    }

    /// The string at `idx`, decoded from MUTF-8.
    pub fn string(&self, idx: u32) -> Option<String> {
        let idx = idx as usize;
        if idx >= self.strings.count {
            return None;
        }
        let data = u32_at(&self.bytes, self.strings.off + idx * 4).ok()? as usize;
        let mut pos = data;
        uleb128(&self.bytes, &mut pos).ok()?;
        Some(mutf8(self.bytes.get(pos..)?))
    }

    /// The descriptor of type `idx` (`I`, `Ljava/lang/String;`, `[J`, ...).
    pub fn type_descriptor(&self, idx: u32) -> Option<String> {
        let idx = idx as usize;
        if idx >= self.types.count {
            return None;
        }
        self.string(u32_at(&self.bytes, self.types.off + idx * 4).ok()?)
    }

    pub fn method_id(&self, idx: u32) -> Option<MethodId> {
        let idx = idx as usize;
        if idx >= self.methods.count {
            return None;
        }
        let at = self.methods.off + idx * 8;
        Some(MethodId {
            class_idx: u32::from(u16_at(&self.bytes, at).ok()?),
            proto_idx: u32::from(u16_at(&self.bytes, at + 2).ok()?),
            name_idx: u32_at(&self.bytes, at + 4).ok()?,
        })
    }

    pub fn field_id(&self, idx: u32) -> Option<FieldId> {
        let idx = idx as usize;
        if idx >= self.fields.count {
            return None;
        }
        let at = self.fields.off + idx * 8;
        Some(FieldId {
            class_idx: u32::from(u16_at(&self.bytes, at).ok()?),
            type_idx: u32::from(u16_at(&self.bytes, at + 2).ok()?),
            name_idx: u32_at(&self.bytes, at + 4).ok()?,
        })
    }

    pub fn proto_id(&self, idx: u32) -> Option<ProtoId> {
        let idx = idx as usize;
        if idx >= self.protos.count {
            return None;
        }
        let at = self.protos.off + idx * 12;
        let return_type_idx = u32_at(&self.bytes, at + 4).ok()?;
        let params_off = u32_at(&self.bytes, at + 8).ok()? as usize;
        let mut parameters = Vec::new();
        if params_off != 0 {
            let size = u32_at(&self.bytes, params_off).ok()? as usize;
            if params_off + 4 + size * 2 > self.bytes.len() {
                return None;
            }
            for k in 0..size {
                parameters.push(u32::from(u16_at(&self.bytes, params_off + 4 + k * 2).ok()?));
            }
        }
        Some(ProtoId { return_type_idx, parameters })
    }
}

/// The Java spelling of a descriptor: `int`, `java.lang.String`, `byte[][]`.
pub fn java_type_name(descriptor: &str) -> String {
    let dims = descriptor.bytes().take_while(|&b| b == b'[').count();
    let base = &descriptor[dims..];
    let mut name = match base {
        "V" => "void".to_string(),
        "Z" => "boolean".to_string(),
        "B" => "byte".to_string(),
        "S" => "short".to_string(),
        "C" => "char".to_string(),
        "I" => "int".to_string(),
        "J" => "long".to_string(),
        "F" => "float".to_string(),
        "D" => "double".to_string(),
        _ => base
            .strip_prefix('L')
            .and_then(|b| b.strip_suffix(';'))
            .unwrap_or(base)
            .replace('/', "."),
    };
    for _ in 0..dims {
        name.push_str("[]");
    }
    name
}

/// The last component of a class descriptor: `Ljava/lang/String;` -> `String`.
pub fn simple_class_name(descriptor: &str) -> String {
    let java = java_type_name(descriptor);
    match java.rfind('.') {
        Some(dot) => java[dot + 1..].to_string(),
        None => java,
    }
}

/// The length in 16-bit units of the Dalvik instruction with opcode `op`
/// (the format table of the bytecode document); `0x00` is a `nop` unless it
/// heads a payload, which [`DexModel::instruction_units`] sizes.
pub fn opcode_units(op: u8) -> u32 {
    match op {
        0x00..=0x01 | 0x04 | 0x07 | 0x0a..=0x12 | 0x1d | 0x1e | 0x21 | 0x27 | 0x28 => 1,
        0x3e..=0x43 | 0x73 | 0x79 | 0x7a | 0x7b..=0x8f | 0xb0..=0xcf | 0xe3..=0xf9 => 1,
        0x02 | 0x05 | 0x08 | 0x13 | 0x15 | 0x16 | 0x19 | 0x1a | 0x1c | 0x1f | 0x20 | 0x22 | 0x23 | 0x29 => 2,
        0x2d..=0x3d | 0x44..=0x6d | 0x90..=0xaf | 0xd0..=0xe2 | 0xfe | 0xff => 2,
        0x03 | 0x06 | 0x09 | 0x14 | 0x17 | 0x1b | 0x24..=0x26 | 0x2a..=0x2c | 0x6e..=0x72 | 0x74..=0x78 | 0xfc | 0xfd => 3,
        0xfa | 0xfb => 4,
        0x18 => 5,
    }
}

/// Does a value of this descriptor take a register pair (`J`, `D`)?
pub fn is_wide(descriptor: &str) -> bool {
    matches!(descriptor, "J" | "D")
}
