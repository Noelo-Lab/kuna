//! The DEX container: header, id tables, class definitions, class data, code
//! items, encoded values and annotations (source.android.com, "Dalvik
//! executable format"). Tables are validated against the file size once;
//! every item is read on demand and every offset is checked when it is read.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::reader::{mutf8_at, Reader};
use crate::{err, Result};

pub const NO_INDEX: u32 = u32::MAX;
const MAX_VALUE_DEPTH: u32 = 32;

#[derive(Clone, Copy, Debug, Default)]
struct Table {
    off: usize,
    count: u32,
    size: usize,
}

impl Table {
    fn new(data: &[u8], what: &str, count: u32, off: u32, size: usize) -> Result<Table> {
        let end = (off as usize).checked_add((count as usize).saturating_mul(size));
        if count != 0 && end.is_none_or(|e| e > data.len()) {
            return Err(err(format!("{what} table ({count} items at {off:#x}) is out of bounds")));
        }
        Ok(Table {
            off: off as usize,
            count,
            size,
        })
    }

    fn item<'a>(&self, data: &'a [u8], what: &str, idx: u32) -> Result<Reader<'a>> {
        if idx >= self.count {
            return Err(err(format!("{what} index {idx} out of range ({})", self.count)));
        }
        Reader::at(data, self.off + idx as usize * self.size)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FieldId {
    pub class: u32,
    pub ty: u32,
    pub name: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct MethodId {
    pub class: u32,
    pub proto: u32,
    pub name: u32,
}

#[derive(Clone, Debug)]
pub struct Proto {
    pub ret: u32,
    pub params: Vec<u32>,
}

#[derive(Clone, Copy, Debug)]
pub struct ClassDef {
    pub class: u32,
    pub access: u32,
    pub superclass: u32,
    pub interfaces_off: u32,
    pub source_file: u32,
    pub annotations_off: u32,
    pub class_data_off: u32,
    pub static_values_off: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct EncodedField {
    pub field: u32,
    pub access: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct EncodedMethod {
    pub method: u32,
    pub access: u32,
    pub code_off: u32,
}

#[derive(Clone, Debug, Default)]
pub struct ClassData {
    pub static_fields: Vec<EncodedField>,
    pub instance_fields: Vec<EncodedField>,
    pub direct_methods: Vec<EncodedMethod>,
    pub virtual_methods: Vec<EncodedMethod>,
}

/// One try item: `[start, start + count)` in code units, its handlers in
/// order (`None` is the catch-all, always last).
#[derive(Clone, Debug)]
pub struct Try {
    pub start: u32,
    pub count: u32,
    pub handlers: Vec<(Option<u32>, u32)>,
}

#[derive(Clone, Debug)]
pub struct CodeItem<'a> {
    pub registers: u16,
    pub ins: u16,
    pub outs: u16,
    pub debug_off: u32,
    /// The instruction array as raw little-endian code units.
    pub insns: &'a [u8],
    pub tries: Vec<Try>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Byte(i8),
    Short(i16),
    Char(u16),
    Int(i32),
    Long(i64),
    Float(u32),
    Double(u64),
    MethodType(u32),
    MethodHandle(u32),
    String(u32),
    Type(u32),
    Field(u32),
    Method(u32),
    Enum(u32),
    Array(Vec<Value>),
    Annotation(EncodedAnnotation),
    Null,
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EncodedAnnotation {
    pub ty: u32,
    pub elements: Vec<(u32, Value)>,
}

#[derive(Clone, Debug)]
pub struct Annotation {
    pub visibility: u8,
    pub value: EncodedAnnotation,
}

pub const VISIBILITY_BUILD: u8 = 0;
pub const VISIBILITY_RUNTIME: u8 = 1;
pub const VISIBILITY_SYSTEM: u8 = 2;

#[derive(Clone, Debug, Default)]
pub struct AnnotationsDirectory {
    pub class: u32,
    pub fields: Vec<(u32, u32)>,
    pub methods: Vec<(u32, u32)>,
    pub parameters: Vec<(u32, u32)>,
}

#[derive(Clone, Copy, Debug)]
pub struct MethodHandle {
    pub kind: u16,
    pub member: u16,
}

/// A parsed DEX file borrowing its bytes.
pub struct Dex<'a> {
    pub data: &'a [u8],
    pub version: u32,
    strings: Table,
    types: Table,
    protos: Table,
    fields: Table,
    methods: Table,
    classes: Table,
    call_sites: Table,
    method_handles: Table,
    by_type: OnceLock<BTreeMap<u32, u32>>,
    nesting: OnceLock<crate::class::Nesting>,
}

impl<'a> Dex<'a> {
    /// Parses the header and validates the id tables; versions 035 to 039.
    pub fn parse(data: &'a [u8]) -> Result<Dex<'a>> {
        if data.len() < 0x70 || &data[..4] != b"dex\n" || data[7] != 0 {
            return Err(err("not a DEX file (bad magic)".to_string()));
        }
        let digits = &data[4..7];
        if !digits.iter().all(u8::is_ascii_digit) {
            return Err(err("not a DEX file (bad version)".to_string()));
        }
        let version = digits
            .iter()
            .fold(0u32, |v, d| v * 10 + u32::from(d - b'0'));
        if !(35..=39).contains(&version) {
            let what = if version >= 41 {
                " (a multi-dex container)"
            } else {
                ""
            };
            return Err(err(format!(
                "unsupported DEX version {}{what}; 035 to 039 are supported",
                String::from_utf8_lossy(digits)
            )));
        }
        let mut r = Reader::at(data, 0x20)?;
        let file_size = r.u32()?;
        let header_size = r.u32()?;
        let endian = r.u32()?;
        if endian != 0x1234_5678 {
            return Err(err("big-endian or corrupt DEX (bad endian tag)".to_string()));
        }
        if header_size != 0x70 {
            return Err(err(format!("unexpected header size {header_size:#x}")));
        }
        if (file_size as usize) > data.len() {
            return Err(err(format!(
                "truncated: header says {file_size} bytes, file has {}",
                data.len()
            )));
        }
        let data = &data[..file_size as usize];
        let _link = (r.u32()?, r.u32()?);
        let map_off = r.u32()?;
        let mut table = |what: &str, size: usize| -> Result<Table> {
            let count = r.u32()?;
            let off = r.u32()?;
            Table::new(data, what, count, off, size)
        };
        let strings = table("string_ids", 4)?;
        let types = table("type_ids", 4)?;
        let protos = table("proto_ids", 12)?;
        let fields = table("field_ids", 8)?;
        let methods = table("method_ids", 8)?;
        let classes = table("class_defs", 32)?;
        let mut dex = Dex {
            data,
            version,
            strings,
            types,
            protos,
            fields,
            methods,
            classes,
            call_sites: Table::default(),
            method_handles: Table::default(),
            by_type: OnceLock::new(),
            nesting: OnceLock::new(),
        };
        if map_off != 0 {
            dex.read_map(map_off)?;
        }
        Ok(dex)
    }

    fn read_map(&mut self, off: u32) -> Result<()> {
        let mut r = Reader::at(self.data, off as usize)?;
        let size = r.u32()?;
        for _ in 0..size {
            let ty = r.u16()?;
            r.u16()?;
            let count = r.u32()?;
            let item_off = r.u32()?;
            match ty {
                0x0007 => {
                    self.call_sites = Table::new(self.data, "call_site_ids", count, item_off, 4)?
                }
                0x0008 => {
                    self.method_handles =
                        Table::new(self.data, "method_handles", count, item_off, 8)?
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn class_count(&self) -> u32 {
        self.classes.count
    }

    /// A string's MUTF-8 bytes.
    pub fn string(&self, idx: u32) -> Result<&'a [u8]> {
        let off = self.strings.item(self.data, "string", idx)?.u32()?;
        mutf8_at(self.data, off as usize)
    }

    /// A type's descriptor (`I`, `Lcom/a/B;`, `[J`).
    pub fn type_desc(&self, idx: u32) -> Result<&'a [u8]> {
        let s = self.types.item(self.data, "type", idx)?.u32()?;
        let desc = self.string(s)?;
        if desc.is_empty() {
            return Err(err(format!("type {idx} has an empty descriptor")));
        }
        Ok(desc)
    }

    pub fn proto(&self, idx: u32) -> Result<Proto> {
        let mut r = self.protos.item(self.data, "proto", idx)?;
        let _shorty = r.u32()?;
        let ret = r.u32()?;
        let params_off = r.u32()?;
        Ok(Proto {
            ret,
            params: self.type_list(params_off)?,
        })
    }

    /// A `type_list` (0 is the empty list).
    pub fn type_list(&self, off: u32) -> Result<Vec<u32>> {
        if off == 0 {
            return Ok(Vec::new());
        }
        let mut r = Reader::at(self.data, off as usize)?;
        let size = r.u32()?;
        if size as usize > r.remaining() / 2 {
            return Err(err(format!("type list at {off:#x} is out of bounds")));
        }
        (0..size).map(|_| Ok(u32::from(r.u16()?))).collect()
    }

    pub fn field(&self, idx: u32) -> Result<FieldId> {
        let mut r = self.fields.item(self.data, "field", idx)?;
        Ok(FieldId {
            class: u32::from(r.u16()?),
            ty: u32::from(r.u16()?),
            name: r.u32()?,
        })
    }

    pub fn method(&self, idx: u32) -> Result<MethodId> {
        let mut r = self.methods.item(self.data, "method", idx)?;
        Ok(MethodId {
            class: u32::from(r.u16()?),
            proto: u32::from(r.u16()?),
            name: r.u32()?,
        })
    }

    pub fn class_def(&self, idx: u32) -> Result<ClassDef> {
        let mut r = self.classes.item(self.data, "class_def", idx)?;
        Ok(ClassDef {
            class: r.u32()?,
            access: r.u32()?,
            superclass: r.u32()?,
            interfaces_off: r.u32()?,
            source_file: r.u32()?,
            annotations_off: r.u32()?,
            class_data_off: r.u32()?,
            static_values_off: r.u32()?,
        })
    }

    /// The class definition index of a type defined in this file.
    pub fn class_of_type(&self, ty: u32) -> Option<u32> {
        self.by_type
            .get_or_init(|| {
                let mut map = BTreeMap::new();
                for i in 0..self.classes.count {
                    if let Ok(def) = self.class_def(i) {
                        map.entry(def.class).or_insert(i);
                    }
                }
                map
            })
            .get(&ty)
            .copied()
    }

    /// Inner-class facts for every class of the file, built on first use.
    pub(crate) fn nesting(&self) -> &crate::class::Nesting {
        self.nesting.get_or_init(|| crate::class::Nesting::build(self))
    }

    pub fn class_data(&self, off: u32) -> Result<ClassData> {
        if off == 0 {
            return Ok(ClassData::default());
        }
        let mut r = Reader::at(self.data, off as usize)?;
        let counts = [r.uleb()?, r.uleb()?, r.uleb()?, r.uleb()?];
        if counts.iter().map(|&c| c as usize).sum::<usize>() > r.remaining() / 2 {
            return Err(err(format!("class data at {off:#x} is out of bounds")));
        }
        let mut fields = |n: u32| -> Result<Vec<EncodedField>> {
            let mut idx = 0u32;
            (0..n)
                .map(|_| {
                    idx = idx.wrapping_add(r.uleb()?);
                    Ok(EncodedField {
                        field: idx,
                        access: r.uleb()?,
                    })
                })
                .collect()
        };
        let static_fields = fields(counts[0])?;
        let instance_fields = fields(counts[1])?;
        let mut methods = |n: u32| -> Result<Vec<EncodedMethod>> {
            let mut idx = 0u32;
            (0..n)
                .map(|_| {
                    idx = idx.wrapping_add(r.uleb()?);
                    Ok(EncodedMethod {
                        method: idx,
                        access: r.uleb()?,
                        code_off: r.uleb()?,
                    })
                })
                .collect()
        };
        let direct_methods = methods(counts[2])?;
        let virtual_methods = methods(counts[3])?;
        Ok(ClassData {
            static_fields,
            instance_fields,
            direct_methods,
            virtual_methods,
        })
    }

    pub fn code_item(&self, off: u32) -> Result<CodeItem<'a>> {
        let mut r = Reader::at(self.data, off as usize)?;
        let registers = r.u16()?;
        let ins = r.u16()?;
        let outs = r.u16()?;
        let tries_size = r.u16()?;
        let debug_off = r.u32()?;
        let units = r.u32()?;
        if ins > registers {
            return Err(err(format!("code item at {off:#x}: ins {ins} > registers {registers}")));
        }
        let insns = r.bytes((units as usize).saturating_mul(2))?;
        let mut tries = Vec::new();
        if tries_size != 0 {
            if units % 2 == 1 {
                r.u16()?;
            }
            let mut raw = Vec::with_capacity(usize::from(tries_size).min(r.remaining() / 8));
            for _ in 0..tries_size {
                raw.push((r.u32()?, u32::from(r.u16()?), r.u16()?));
            }
            let list_start = r.pos();
            r.uleb()?;
            for (start, count, handler_off) in raw {
                let mut h = Reader::at(self.data, list_start + usize::from(handler_off))?;
                let size = h.sleb()?;
                let typed = size.unsigned_abs();
                if typed as usize > h.remaining() / 2 {
                    return Err(err(format!("catch handler at {off:#x} is out of bounds")));
                }
                let mut handlers = Vec::new();
                for _ in 0..typed {
                    let ty = h.uleb()?;
                    handlers.push((Some(ty), h.uleb()?));
                }
                if size <= 0 {
                    handlers.push((None, h.uleb()?));
                }
                if start.checked_add(count).is_none_or(|end| end > units) {
                    return Err(err(format!("try range at {off:#x} is outside the code")));
                }
                tries.push(Try {
                    start,
                    count,
                    handlers,
                });
            }
        }
        Ok(CodeItem {
            registers,
            ins,
            outs,
            debug_off,
            insns,
            tries,
        })
    }

    pub fn encoded_array(&self, off: u32) -> Result<Vec<Value>> {
        if off == 0 {
            return Ok(Vec::new());
        }
        let mut r = Reader::at(self.data, off as usize)?;
        read_array(&mut r, 0)
    }

    pub fn call_site(&self, idx: u32) -> Result<Vec<Value>> {
        let off = self.call_sites.item(self.data, "call_site", idx)?.u32()?;
        self.encoded_array(off)
    }

    pub fn method_handle(&self, idx: u32) -> Result<MethodHandle> {
        let mut r = self.method_handles.item(self.data, "method_handle", idx)?;
        let kind = r.u16()?;
        r.u16()?;
        Ok(MethodHandle {
            kind,
            member: r.u16()?,
        })
    }

    pub fn annotations_directory(&self, off: u32) -> Result<AnnotationsDirectory> {
        if off == 0 {
            return Ok(AnnotationsDirectory::default());
        }
        let mut r = Reader::at(self.data, off as usize)?;
        let class = r.u32()?;
        let counts = [r.u32()?, r.u32()?, r.u32()?];
        if counts.iter().map(|&c| c as usize).sum::<usize>() > r.remaining() / 8 {
            return Err(err(format!("annotations directory at {off:#x} is out of bounds")));
        }
        let mut pairs = |n: u32| -> Result<Vec<(u32, u32)>> {
            (0..n).map(|_| Ok((r.u32()?, r.u32()?))).collect()
        };
        Ok(AnnotationsDirectory {
            class,
            fields: pairs(counts[0])?,
            methods: pairs(counts[1])?,
            parameters: pairs(counts[2])?,
        })
    }

    pub fn annotation_set(&self, off: u32) -> Result<Vec<Annotation>> {
        if off == 0 {
            return Ok(Vec::new());
        }
        let mut r = Reader::at(self.data, off as usize)?;
        let size = r.u32()?;
        if size as usize > r.remaining() / 4 {
            return Err(err(format!("annotation set at {off:#x} is out of bounds")));
        }
        let mut out = Vec::new();
        for _ in 0..size {
            let item = r.u32()?;
            let mut a = Reader::at(self.data, item as usize)?;
            let visibility = a.u8()?;
            out.push(Annotation {
                visibility,
                value: read_annotation(&mut a, 0)?,
            });
        }
        Ok(out)
    }

    /// An `annotation_set_ref_list`: one annotation set per parameter.
    pub fn annotation_set_list(&self, off: u32) -> Result<Vec<Vec<Annotation>>> {
        let mut r = Reader::at(self.data, off as usize)?;
        let size = r.u32()?;
        if size as usize > r.remaining() / 4 {
            return Err(err(format!("annotation set list at {off:#x} is out of bounds")));
        }
        let offs: Vec<u32> = (0..size).map(|_| r.u32()).collect::<Result<_>>()?;
        offs.into_iter().map(|o| self.annotation_set(o)).collect()
    }
}

fn read_annotation(r: &mut Reader, depth: u32) -> Result<EncodedAnnotation> {
    let ty = r.uleb()?;
    let size = r.uleb()?;
    if size as usize > r.remaining() / 2 {
        return Err(err(format!("annotation at {:#x} is out of bounds", r.pos())));
    }
    let mut elements = Vec::new();
    for _ in 0..size {
        let name = r.uleb()?;
        elements.push((name, read_value(r, depth + 1)?));
    }
    Ok(EncodedAnnotation { ty, elements })
}

fn read_array(r: &mut Reader, depth: u32) -> Result<Vec<Value>> {
    let size = r.uleb()?;
    if size as usize > r.remaining() {
        return Err(err(format!("encoded array at {:#x} is out of bounds", r.pos())));
    }
    (0..size).map(|_| read_value(r, depth + 1)).collect()
}

fn read_value(r: &mut Reader, depth: u32) -> Result<Value> {
    if depth > MAX_VALUE_DEPTH {
        return Err(err("encoded value nested too deeply".to_string()));
    }
    let head = r.u8()?;
    let (ty, arg) = (head & 0x1f, usize::from(head >> 5));
    let mut raw = |signed: bool| -> Result<u64> {
        let b = r.bytes(arg + 1)?;
        let mut v = 0u64;
        for (i, &x) in b.iter().enumerate() {
            v |= u64::from(x) << (8 * i);
        }
        let bits = 8 * (arg + 1);
        if signed && bits < 64 && v & (1 << (bits - 1)) != 0 {
            v |= u64::MAX << bits;
        }
        Ok(v)
    };
    let index = |v: u64| -> Result<u32> {
        u32::try_from(v).map_err(|_| err("encoded index out of range".to_string()))
    };
    let width = |max: usize| -> Result<()> {
        if arg >= max {
            return Err(err(format!("encoded value type {ty:#x} with size {}", arg + 1)));
        }
        Ok(())
    };
    Ok(match ty {
        0x00 => {
            width(1)?;
            Value::Byte(raw(true)? as i8)
        }
        0x02 => {
            width(2)?;
            Value::Short(raw(true)? as i16)
        }
        0x03 => {
            width(2)?;
            Value::Char(raw(false)? as u16)
        }
        0x04 => {
            width(4)?;
            Value::Int(raw(true)? as i32)
        }
        0x06 => {
            width(8)?;
            Value::Long(raw(true)? as i64)
        }
        0x10 => {
            width(4)?;
            Value::Float((raw(false)? << (8 * (3 - arg))) as u32)
        }
        0x11 => {
            width(8)?;
            Value::Double(raw(false)? << (8 * (7 - arg)))
        }
        0x15..=0x1b => {
            width(4)?;
            let i = index(raw(false)?)?;
            match ty {
                0x15 => Value::MethodType(i),
                0x16 => Value::MethodHandle(i),
                0x17 => Value::String(i),
                0x18 => Value::Type(i),
                0x19 => Value::Field(i),
                0x1a => Value::Method(i),
                _ => Value::Enum(i),
            }
        }
        0x1c => Value::Array(read_array(r, depth)?),
        0x1d => Value::Annotation(read_annotation(r, depth)?),
        0x1e => Value::Null,
        0x1f => Value::Boolean(arg != 0),
        _ => return Err(err(format!("bad encoded value type {ty:#x}"))),
    })
}
