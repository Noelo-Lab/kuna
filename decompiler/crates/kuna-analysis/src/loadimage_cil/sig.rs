//! ECMA-335 signature blobs (II.23.2): method, field and local-variable
//! signatures, and the type grammar they share.

use std::collections::HashMap;

use super::metadata::{self, compressed, Metadata};

/// A type as a signature spells it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CilType {
    Void,
    Bool,
    Char,
    I1,
    U1,
    I2,
    U2,
    I4,
    U4,
    I8,
    U8,
    R4,
    R8,
    /// `native int` / `native unsigned int`.
    Native { signed: bool },
    String,
    Object,
    TypedByRef,
    Ptr(Box<CilType>),
    ByRef(Box<CilType>),
    /// A single-dimensional, zero-based array.
    SzArray(Box<CilType>),
    /// A general array of the given rank.
    Array(Box<CilType>, u32),
    /// A class or value type by `(table, rid)`.
    Named { value_type: bool, table: usize, rid: u32 },
    /// A generic instantiation of a named type.
    Inst(Box<CilType>, Vec<CilType>),
    /// A generic parameter of the enclosing type (`!n`).
    Var(u32),
    /// A generic parameter of the enclosing method (`!!n`).
    MVar(u32),
    FnPtr,
}

/// A method signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodSig {
    pub has_this: bool,
    pub generic_params: u32,
    pub ret: CilType,
    pub params: Vec<CilType>,
    /// A call-site signature with a vararg sentinel.
    pub vararg: bool,
}

/// The nesting a hostile signature may reach before it is refused.
const MAX_DEPTH: u32 = 32;

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn byte(&mut self) -> Option<u8> {
        let v = *self.b.get(self.at)?;
        self.at += 1;
        Some(v)
    }

    fn peek(&self) -> Option<u8> {
        self.b.get(self.at).copied()
    }

    fn uint(&mut self) -> Option<u32> {
        let (v, used) = compressed(self.b.get(self.at..)?)?;
        self.at += used;
        Some(v)
    }

    fn skip_mods(&mut self) -> Option<()> {
        while matches!(self.peek(), Some(0x1f | 0x20)) {
            self.at += 1;
            self.uint()?;
        }
        Some(())
    }

    fn ty(&mut self, depth: u32) -> Option<CilType> {
        if depth > MAX_DEPTH {
            return None;
        }
        self.skip_mods()?;
        let boxed = |r: &mut Self| r.ty(depth + 1).map(Box::new);
        Some(match self.byte()? {
            0x01 => CilType::Void,
            0x02 => CilType::Bool,
            0x03 => CilType::Char,
            0x04 => CilType::I1,
            0x05 => CilType::U1,
            0x06 => CilType::I2,
            0x07 => CilType::U2,
            0x08 => CilType::I4,
            0x09 => CilType::U4,
            0x0a => CilType::I8,
            0x0b => CilType::U8,
            0x0c => CilType::R4,
            0x0d => CilType::R8,
            0x0e => CilType::String,
            0x0f => CilType::Ptr(boxed(self)?),
            0x10 => CilType::ByRef(boxed(self)?),
            kind @ (0x11 | 0x12) => {
                let (table, rid) = metadata::type_def_or_ref(self.uint()?)?;
                CilType::Named { value_type: kind == 0x11, table, rid }
            }
            0x13 => CilType::Var(self.uint()?),
            0x14 => {
                let elem = boxed(self)?;
                let rank = self.uint()?;
                for _ in 0..self.uint()? {
                    self.uint()?;
                }
                for _ in 0..self.uint()? {
                    self.uint()?;
                }
                CilType::Array(elem, rank)
            }
            0x15 => {
                let base = boxed(self)?;
                let count = self.uint()?;
                let mut args = Vec::new();
                for _ in 0..count {
                    args.push(self.ty(depth + 1)?);
                }
                CilType::Inst(base, args)
            }
            0x16 => CilType::TypedByRef,
            0x18 => CilType::Native { signed: true },
            0x19 => CilType::Native { signed: false },
            0x1b => {
                self.method(depth + 1)?;
                CilType::FnPtr
            }
            0x1c => CilType::Object,
            0x1d => CilType::SzArray(boxed(self)?),
            0x1e => CilType::MVar(self.uint()?),
            0x45 => return self.ty(depth + 1),
            _ => return None,
        })
    }

    fn method(&mut self, depth: u32) -> Option<MethodSig> {
        let conv = self.byte()?;
        let generic_params = if conv & 0x10 != 0 { self.uint()? } else { 0 };
        let count = self.uint()?;
        let ret = self.ty(depth)?;
        let mut params = Vec::new();
        let mut vararg = false;
        for _ in 0..count {
            if self.peek() == Some(0x41) {
                self.at += 1;
                vararg = true;
            }
            params.push(self.ty(depth)?);
        }
        Some(MethodSig {
            has_this: conv & 0x20 != 0 && conv & 0x40 == 0,
            generic_params,
            ret,
            params,
            vararg: vararg || conv & 0x0f == 5,
        })
    }
}

/// A `MethodDefSig` / `MethodRefSig`; `None` for a field signature.
pub fn method_sig(blob: &[u8]) -> Option<MethodSig> {
    if blob.first()? & 0x0f == 6 {
        return None;
    }
    Reader { b: blob, at: 0 }.method(0)
}

/// The type of a `FieldSig`.
pub fn field_sig(blob: &[u8]) -> Option<CilType> {
    let mut r = Reader { b: blob, at: 0 };
    (r.byte()? == 6).then_some(())?;
    r.ty(0)
}

/// The local types of a `LocalVarSig`.
pub fn locals_sig(blob: &[u8]) -> Option<Vec<CilType>> {
    let mut r = Reader { b: blob, at: 0 };
    (r.byte()? == 7).then_some(())?;
    let count = r.uint()?;
    let mut out = Vec::new();
    for _ in 0..count {
        out.push(r.ty(0)?);
    }
    Some(out)
}

/// The type a `TypeSpec` blob spells.
pub fn type_spec(blob: &[u8]) -> Option<CilType> {
    Reader { b: blob, at: 0 }.ty(0)
}

/// The instantiation of a `MethodSpec` blob.
pub fn method_spec(blob: &[u8]) -> Option<Vec<CilType>> {
    let mut r = Reader { b: blob, at: 0 };
    (r.byte()? == 0x0a).then_some(())?;
    let count = r.uint()?;
    let mut out = Vec::new();
    for _ in 0..count {
        out.push(r.ty(0)?);
    }
    Some(out)
}

impl CilType {
    /// Replace the generic parameters with the arguments of the instantiation
    /// a member is reached through; a parameter with no argument stays.
    pub fn substitute(&self, class: &[CilType], method: &[CilType]) -> CilType {
        let sub = |t: &CilType| Box::new(t.substitute(class, method));
        match self {
            CilType::Var(n) => class.get(*n as usize).cloned().unwrap_or_else(|| self.clone()),
            CilType::MVar(n) => method.get(*n as usize).cloned().unwrap_or_else(|| self.clone()),
            CilType::Ptr(t) => CilType::Ptr(sub(t)),
            CilType::ByRef(t) => CilType::ByRef(sub(t)),
            CilType::SzArray(t) => CilType::SzArray(sub(t)),
            CilType::Array(t, rank) => CilType::Array(sub(t), *rank),
            CilType::Inst(base, args) => CilType::Inst(
                base.clone(),
                args.iter().map(|a| a.substitute(class, method)).collect(),
            ),
            _ => self.clone(),
        }
    }
}

/// The primitive a `System` type name stands for in a type-token position.
pub fn system_primitive(name: &str) -> Option<CilType> {
    Some(match name {
        "Boolean" => CilType::Bool,
        "Char" => CilType::Char,
        "SByte" => CilType::I1,
        "Byte" => CilType::U1,
        "Int16" => CilType::I2,
        "UInt16" => CilType::U2,
        "Int32" => CilType::I4,
        "UInt32" => CilType::U4,
        "Int64" => CilType::I8,
        "UInt64" => CilType::U8,
        "Single" => CilType::R4,
        "Double" => CilType::R8,
        "IntPtr" => CilType::Native { signed: true },
        "UIntPtr" => CilType::Native { signed: false },
        "String" => CilType::String,
        "Object" => CilType::Object,
        _ => return None,
    })
}

fn qualified(namespace: &str, name: &str) -> String {
    if namespace.is_empty() {
        name.to_string()
    } else {
        format!("{namespace}.{name}")
    }
}

/// The full name of a `TypeDef` row. `nested` maps a nested type to the one
/// enclosing it; the two are joined with `.`.
pub fn type_def_name(md: &Metadata, nested: &HashMap<u32, u32>, rid: u32) -> String {
    let mut parts = Vec::new();
    let mut at = Some(rid);
    while let Some(row) = at.filter(|_| parts.len() <= MAX_DEPTH as usize).and_then(|rid| md.row(metadata::TYPE_DEF, rid)) {
        parts.push(qualified(md.string(row[2]), md.string(row[1])));
        at = at.and_then(|rid| nested.get(&rid).copied());
    }
    parts.reverse();
    parts.join(".")
}

/// The full name of a `TypeRef` row, a nested one under its enclosing type.
pub fn type_ref_name(md: &Metadata, rid: u32) -> String {
    let mut parts = Vec::new();
    let mut at = Some(rid);
    while let Some(row) = at.filter(|_| parts.len() <= MAX_DEPTH as usize).and_then(|rid| md.row(metadata::TYPE_REF, rid)) {
        parts.push(qualified(md.string(row[2]), md.string(row[1])));
        at = metadata::decode_coded(row[0], &[metadata::MODULE, 0x1a, 0x23, metadata::TYPE_REF], 2)
            .filter(|&(table, _)| table == metadata::TYPE_REF)
            .map(|(_, rid)| rid);
    }
    parts.reverse();
    parts.join(".")
}
