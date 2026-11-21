//! (kuna `cil`) A managed .NET assembly as a decodable image: the PE's CLI
//! header and ECMA-335 metadata read into the methods, their bodies and the
//! references they name, laid out for the `CIL:LE:64:default` language.
//!
//! The PE is mapped by RVA at [`CIL_BASE`]. A method's function starts at the
//! first instruction of its body, after the tiny or fat header. CIL leaves two
//! things out of the instruction bytes that a SLEIGH decoder needs, and the
//! loader supplies both:
//!
//! - operand types and stack depth: [`body::type_body`] walks each body and
//!   the result is painted over it as the `cilctx` decode context;
//! - what a metadata token names: every token operand is rewritten in the
//!   mapped image to the address of the method, static field or string it
//!   resolves to.
//!
//! Methods of other assemblies and methods with no body get an address in an
//! unmapped range at [`EXTERN_BASE`] and are named and prototyped, not listed.
//! Static fields without initial data and `ldstr` strings (as NUL-terminated
//! UTF-16) are laid out after the image. A body the walk refuses is listed but
//! left unpainted, so it decodes as unimplemented instead of as a guess.

pub mod body;
pub mod image;
pub mod metadata;
pub mod sig;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_decomp::architecture::Architecture;
use kuna_decomp::dtype::{type_metatype, Datatype, TypeField};
use kuna_decomp::fspec::PrototypePieces;
use kuna_sleigh::loadimage::section_flags;

use crate::pass::{AnalysisOutput, ContextPaint, NoReturnFact, StringFact, SymFact, SymKind, TypedDataFact};
use body::{BodyHeader, Callee, Frame, Kind, Resolve, Slot};
use metadata::{Metadata, PeLayout, FIELD, FIELD_RVA, MEMBER_REF, METHOD_DEF, METHOD_SPEC, PARAM, TYPE_DEF, TYPE_REF, TYPE_SPEC};
use sig::{CilType, MethodSig};

/// The language a managed assembly decodes as.
pub const CIL_LANGUAGE: &str = "CIL:LE:64:default";
/// Where the PE is mapped: an RVA `r` is at `CIL_BASE + r`.
pub const CIL_BASE: u64 = 0x40_0000;
/// The no-return helper a `throw` calls, as the language spells it.
pub const THROW_ADDR: u64 = 0x1000;
/// The first address handed to a method with no body in this image.
pub const EXTERN_BASE: u64 = 0x1_0000;
/// The decode-context variable painted over typed instructions.
pub const CONTEXT_VAR: &str = "cilctx";

/// The largest image the loader maps: the PE plus what it lays out after it
/// must stay addressable by a 32-bit operand.
const MAX_IMAGE: u64 = 0x4000_0000;

/// How many times its file size a PE's file-backed extent may map to. Section
/// data is aligned to `FileAlignment` in the file and `SectionAlignment` in
/// memory, 0x200 and 0x2000 in a managed image, a ratio of 16; a larger
/// spread is a section table that names addresses the file cannot back.
const MAX_SPREAD: u64 = 64;

/// How much of a file's start [`has_cli_header`] is given before the whole
/// file is read: the headers of a PE end well inside it.
pub const CLI_PROBE_LEN: usize = 0x1000;

pub use metadata::has_cli_header;

/// Whether `bytes` is a PE with a CLI header and readable metadata.
pub fn is_cil_image(bytes: &[u8]) -> bool {
    PeLayout::parse(bytes).is_some_and(|pe| Metadata::parse(bytes, &pe).is_some())
}

/// A method with a body in this image.
#[derive(Debug)]
pub struct CilMethod {
    /// Address of the first instruction.
    pub addr: u64,
    /// Address past the last instruction byte.
    pub end: u64,
    pub name: String,
    /// The lift covers the body.
    pub typed: bool,
}

struct Proto {
    addr: u64,
    name: String,
    /// The type of the instance argument, for an instance method.
    this: Option<CilType>,
    sig: MethodSig,
    param_names: Vec<String>,
}

/// The instance fields of one `TypeDef`, laid out for the lift.
///
/// The runtime chooses a class's real layout; the image only orders the
/// fields. Each field gets the next naturally aligned offset after an 8-byte
/// object header (none in a value type) or after a base class of this image,
/// so an access is a load at a fixed offset the struct type names again.
#[derive(Debug)]
struct Record {
    name: String,
    size: u32,
    /// `TypeDef` rid of a base class laid out in front.
    base: Option<u32>,
    fields: Vec<(String, u32, CilType)>,
}

/// A managed assembly, mapped and resolved.
pub struct CilImage {
    /// The mapped bytes, starting at [`CIL_BASE`].
    pub image: Vec<u8>,
    pub methods: Vec<CilMethod>,
    /// The entry point method, when the CLI header names one with a body.
    pub entry: Option<u64>,
    /// The mapped bytes as `(address, size, flags)` sections: the method
    /// bodies of each PE section are one code run, everything else is data.
    pub sections: Vec<(u64, u64, u32)>,
    protos: Vec<Proto>,
    externs: Vec<(u64, String)>,
    statics: Vec<(u64, String, CilType)>,
    strings: Vec<(u64, u32)>,
    paints: Vec<(u64, u64, u32)>,
    enums: HashMap<u32, CilType>,
    primitives: HashMap<u32, CilType>,
    /// By `TypeDef` rid.
    records: BTreeMap<u32, Record>,
}

/// The loader's view of the metadata while bodies are typed.
struct Loader<'a> {
    md: Metadata<'a>,
    type_names: Vec<String>,
    /// `TypeDef` rid of an enum to its underlying type.
    enums: HashMap<u32, CilType>,
    /// `TypeRef` rid of a `System` primitive.
    primitives: HashMap<u32, CilType>,
    /// `TypeRef` rids of `System` handle value types, pointer-sized.
    handles: HashSet<u32>,
    value_types: Vec<bool>,
    field_owner: Vec<u32>,
    method_addr: Vec<u32>,
    field_rva: HashMap<u32, u32>,
    tail: u32,
    tail_data: Vec<u8>,
    static_slots: BTreeMap<u32, u32>,
    string_slots: BTreeMap<u32, (u32, u32)>,
    statics: Vec<(u64, String, CilType)>,
    records: BTreeMap<u32, Record>,
    /// Offset of an instance field, by `Field` rid.
    field_offset: Vec<Option<u32>>,
    /// `TypeDef` rids whose layout was attempted.
    layout_tried: Vec<bool>,
}

/// The nesting of value-type fields and base classes a layout may reach.
const MAX_LAYOUT_DEPTH: u32 = 32;

/// Which of the bodies `(start, size)` overlap a body with another range.
/// Two methods may share one body; bodies that only partly coincide would be
/// typed, painted and rewritten against each other, so both are refused.
fn overlapping(bodies: &[(u32, u32)]) -> Vec<bool> {
    let mut order: Vec<usize> = (0..bodies.len()).collect();
    order.sort_unstable_by_key(|&i| bodies[i]);
    let mut out = vec![false; bodies.len()];
    let mut reach: Option<usize> = None;
    for i in order {
        let (start, size) = bodies[i];
        let end = u64::from(start) + u64::from(size);
        let reach_end = |r: usize| u64::from(bodies[r].0) + u64::from(bodies[r].1);
        if let Some(r) = reach.filter(|&r| u64::from(start) < reach_end(r) && bodies[r] != bodies[i]) {
            out[i] = true;
            out[r] = true;
        }
        if reach.is_none_or(|r| end > reach_end(r)) {
            reach = Some(i);
        }
    }
    out
}

/// Sections covering `[0, end)` of the mapped image: each run of `runs`
/// (offsets from [`CIL_BASE`]) is code and the bytes between them are data.
fn section_table(mut runs: Vec<(u64, u64)>, end: u64) -> Vec<(u64, u64, u32)> {
    runs.sort_unstable();
    let mut out = Vec::new();
    let mut at = 0;
    for (start, stop) in runs {
        let start = start.max(at);
        if stop <= start || stop > end {
            continue;
        }
        if start > at {
            out.push((CIL_BASE + at, start - at, section_flags::DATA));
        }
        out.push((CIL_BASE + start, stop - start, section_flags::CODE));
        at = stop;
    }
    if end > at {
        out.push((CIL_BASE + at, end - at, section_flags::DATA));
    }
    out
}

fn addr32(addr: u64) -> Option<u32> {
    u32::try_from(addr).ok()
}

impl<'a> Loader<'a> {
    fn type_name(&self, table: usize, rid: u32) -> String {
        match table {
            TYPE_DEF => self.type_names.get(rid as usize).cloned().unwrap_or_default(),
            TYPE_REF => sig::type_ref_name(&self.md, rid),
            TYPE_SPEC => self
                .md
                .row(TYPE_SPEC, rid)
                .and_then(|row| sig::type_spec(self.md.blob(row[0])))
                .map(|ty| self.display(&ty))
                .unwrap_or_default(),
            _ => String::new(),
        }
    }

    /// A type as C# spells it, for the name of a member reached through it.
    fn display(&self, ty: &CilType) -> String {
        match ty {
            CilType::Void => "void".into(),
            CilType::Bool => "bool".into(),
            CilType::Char => "char".into(),
            CilType::I1 => "sbyte".into(),
            CilType::U1 => "byte".into(),
            CilType::I2 => "short".into(),
            CilType::U2 => "ushort".into(),
            CilType::I4 => "int".into(),
            CilType::U4 => "uint".into(),
            CilType::I8 => "long".into(),
            CilType::U8 => "ulong".into(),
            CilType::R4 => "float".into(),
            CilType::R8 => "double".into(),
            CilType::Native { signed: true } => "nint".into(),
            CilType::Native { signed: false } => "nuint".into(),
            CilType::String => "string".into(),
            CilType::Object => "object".into(),
            CilType::TypedByRef => "TypedReference".into(),
            CilType::Ptr(t) => format!("{}*", self.display(t)),
            CilType::ByRef(t) => format!("{}&", self.display(t)),
            CilType::SzArray(t) => format!("{}[]", self.display(t)),
            CilType::Array(t, rank) => format!("{}[{}]", self.display(t), ",".repeat(rank.saturating_sub(1) as usize)),
            CilType::Named { table, rid, .. } => self.type_name(*table, *rid),
            CilType::Inst(base, args) => {
                let base = self.display(base);
                let base = base.split('`').next().unwrap_or_default();
                let args: Vec<String> = args.iter().map(|a| self.display(a)).collect();
                format!("{base}<{}>", args.join(","))
            }
            CilType::Var(n) => format!("!{n}"),
            CilType::MVar(n) => format!("!!{n}"),
            CilType::FnPtr => "fnptr".into(),
        }
    }

    /// How a value of `ty` sits on the evaluation stack; `None` for a value
    /// type whose size the metadata of this image does not give.
    fn slot(&self, ty: &CilType) -> Option<Slot> {
        let int = |class| Some(Slot { kind: Kind::I4, class });
        match ty {
            CilType::Void | CilType::TypedByRef => None,
            CilType::Bool | CilType::U1 => int(body::LOAD_U1),
            CilType::I1 => int(body::LOAD_S1),
            CilType::Char | CilType::U2 => int(body::LOAD_U2),
            CilType::I2 => int(body::LOAD_S2),
            CilType::I4 | CilType::U4 => int(body::LOAD_S4),
            CilType::R4 => Some(Slot { kind: Kind::R4, class: body::LOAD_F4 }),
            CilType::R8 => Some(Slot { kind: Kind::R8, class: body::LOAD_8 }),
            CilType::Named { value_type: true, table, rid } => match *table {
                TYPE_DEF => self.slot(self.enums.get(rid)?),
                TYPE_REF if self.handles.contains(rid) => Some(Slot::REF),
                TYPE_REF => self.slot(self.primitives.get(rid)?),
                _ => None,
            },
            CilType::Inst(base, _) => match **base {
                CilType::Named { value_type: false, .. } => Some(Slot::REF),
                _ => None,
            },
            _ => Some(Slot::REF),
        }
    }

    fn generic_args(&self, parent_table: usize, parent_rid: u32) -> Vec<CilType> {
        if parent_table != TYPE_SPEC {
            return Vec::new();
        }
        match self.md.row(TYPE_SPEC, parent_rid).and_then(|row| sig::type_spec(self.md.blob(row[0]))) {
            Some(CilType::Inst(_, args)) => args,
            _ => Vec::new(),
        }
    }

    /// The address and signature of the method `(table, rid)` names, its
    /// generic parameters replaced by `method_args` and its parent's.
    fn method(&self, table: usize, rid: u32, method_args: &[CilType], depth: u32) -> Option<(u32, MethodSig)> {
        match table {
            METHOD_DEF => {
                let row = self.md.row(METHOD_DEF, rid)?;
                let sig = sig::method_sig(self.md.blob(row[4]))?;
                let addr = *self.method_addr.get(rid as usize)?;
                Some((addr, substitute(&sig, &[], method_args)))
            }
            MEMBER_REF => {
                let row = self.md.row(MEMBER_REF, rid)?;
                let sig = sig::method_sig(self.md.blob(row[2]))?;
                let (parent, parent_rid) = metadata::member_ref_parent(row[0])?;
                let class_args = self.generic_args(parent, parent_rid);
                let addr = addr32(EXTERN_BASE + 8 * u64::from(rid - 1))?;
                Some((addr, substitute(&sig, &class_args, method_args)))
            }
            METHOD_SPEC if depth == 0 => {
                let row = self.md.row(METHOD_SPEC, rid)?;
                let (inner, inner_rid) = metadata::method_def_or_ref(row[0])?;
                let args = sig::method_spec(self.md.blob(row[1]))?;
                self.method(inner, inner_rid, &args, 1)
            }
            _ => None,
        }
    }

    fn member_name(&self, row: &metadata::Row) -> String {
        let name = self.md.string(row[1]);
        match metadata::member_ref_parent(row[0]) {
            Some((METHOD_DEF, rid)) => self.method_name(rid),
            Some((table, rid)) => format!("{}::{name}", self.type_name(table, rid)),
            None => name.to_string(),
        }
    }

    fn method_name(&self, rid: u32) -> String {
        let name = self.md.row(METHOD_DEF, rid).map_or("", |row| self.md.string(row[3]));
        format!("{}::{name}", self.type_names.get(self.method_owner(rid) as usize).map_or("", String::as_str))
    }

    fn method_owner(&self, rid: u32) -> u32 {
        let types = self.md.table(TYPE_DEF);
        types.partition_point(|row| row[5] <= rid) as u32
    }

    fn alloc(&mut self, bytes: &[u8], align: usize) -> Option<u32> {
        let pad = (align - self.tail_data.len() % align) % align;
        self.tail_data.resize(self.tail_data.len() + pad, 0);
        let addr = self.tail.checked_add(u32::try_from(self.tail_data.len()).ok()?)?;
        self.tail_data.extend_from_slice(bytes);
        (u64::from(addr) + (bytes.len() as u64) < CIL_BASE + MAX_IMAGE).then_some(addr)
    }

    /// Lay out the instance fields of `TypeDef` `rid`; `None` for an
    /// interface, an enum and an explicit layout. Each type is tried once,
    /// and its field offsets stand only when the whole layout does.
    fn layout(&mut self, rid: u32, depth: u32) -> Option<u32> {
        if let Some(record) = self.records.get(&rid) {
            return Some(record.size);
        }
        if depth > MAX_LAYOUT_DEPTH || std::mem::replace(self.layout_tried.get_mut(rid as usize)?, true) {
            return None;
        }
        let row = *self.md.row(TYPE_DEF, rid)?;
        let value_type = *self.value_types.get(rid as usize)?;
        if row[0] & 0x20 != 0 || row[0] & 0x18 == 0x10 || self.enums.contains_key(&rid) {
            return None;
        }
        let base = match metadata::type_def_or_ref(row[3]) {
            Some((TYPE_DEF, base)) if !value_type && base != rid => Some(base),
            _ => None,
        };
        let mut end = match base {
            Some(base) => self.layout(base, depth + 1)?,
            None if value_type => 0,
            None => 8,
        };
        let mut fields = Vec::new();
        let mut offsets = Vec::new();
        for field in self.md.owned(TYPE_DEF, rid, 4, FIELD) {
            let Some(frow) = self.md.row(FIELD, field).copied() else { continue };
            if frow[0] & 0x10 != 0 {
                continue;
            }
            let ty = sig::field_sig(self.md.blob(frow[2]))?;
            let size = match (self.slot(&ty), &ty) {
                (Some(slot), _) => 1 << slot.log2_size(),
                (None, CilType::Named { value_type: true, table: TYPE_DEF, rid: inner }) => {
                    self.layout(*inner, depth + 1).unwrap_or(8).max(1)
                }
                _ => 8,
            };
            let align = size.min(8).next_power_of_two();
            let offset = end.checked_add(align - 1)? & !(align - 1);
            end = offset.checked_add(size)?;
            offsets.push((field, offset));
            fields.push((self.md.string(frow[1]).to_string(), offset, ty));
        }
        let size = (end.checked_add(7)? & !7).max(8);
        for (field, offset) in offsets {
            if let Some(slot) = self.field_offset.get_mut(field as usize) {
                *slot = Some(offset);
            }
        }
        let name = self.type_name(TYPE_DEF, rid);
        self.records.insert(rid, Record { name, size, base, fields });
        Some(size)
    }

    /// The `TypeDef` a member reference's parent names, with the generic
    /// arguments it is instantiated by.
    fn local_parent(&self, parent: usize, rid: u32) -> Option<(u32, Vec<CilType>)> {
        match parent {
            TYPE_DEF => Some((rid, Vec::new())),
            TYPE_SPEC => match sig::type_spec(self.md.blob(self.md.row(TYPE_SPEC, rid)?[0]))? {
                CilType::Inst(base, args) => match *base {
                    CilType::Named { table: TYPE_DEF, rid, .. } => Some((rid, args)),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }

    fn static_slot(&mut self, token: u32, name: String, ty: CilType) -> Option<u32> {
        if let Some(&addr) = self.static_slots.get(&token) {
            return Some(addr);
        }
        let addr = self.alloc(&[0; 8], 8)?;
        self.static_slots.insert(token, addr);
        self.statics.push((u64::from(addr), name, ty));
        Some(addr)
    }
}

fn substitute(sig: &MethodSig, class: &[CilType], method: &[CilType]) -> MethodSig {
    MethodSig {
        ret: sig.ret.substitute(class, method),
        params: sig.params.iter().map(|p| p.substitute(class, method)).collect(),
        ..sig.clone()
    }
}

impl Resolve for Loader<'_> {
    fn callee(&mut self, token: u32) -> Option<Callee> {
        let (addr, sig) = self.method((token >> 24) as usize, token & 0x00ff_ffff, &[], 0)?;
        if sig.vararg || sig.params.iter().any(|p| self.slot(p).is_none()) {
            return None;
        }
        let ret = match sig.ret {
            CilType::Void => None,
            ref ty => Some(self.slot(ty)?),
        };
        Some(Callee { addr, args: sig.params.len() + usize::from(sig.has_this), ret })
    }

    fn static_field(&mut self, token: u32) -> Option<(u32, Slot)> {
        let rid = token & 0x00ff_ffff;
        match (token >> 24) as usize {
            FIELD => {
                let row = *self.md.row(FIELD, rid)?;
                (row[0] & 0x10 != 0).then_some(())?;
                let ty = sig::field_sig(self.md.blob(row[2]))?;
                let slot = self.slot(&ty)?;
                if let Some(&rva) = self.field_rva.get(&rid) {
                    return Some((addr32(CIL_BASE + u64::from(rva))?, slot));
                }
                let owner = self.field_owner.get(rid as usize).copied().unwrap_or(0);
                let name = format!("{}::{}", self.type_name(TYPE_DEF, owner), self.md.string(row[1]));
                Some((self.static_slot(token, name, ty)?, slot))
            }
            MEMBER_REF => {
                let row = *self.md.row(MEMBER_REF, rid)?;
                let (parent, parent_rid) = metadata::member_ref_parent(row[0])?;
                let ty = sig::field_sig(self.md.blob(row[2]))?.substitute(&self.generic_args(parent, parent_rid), &[]);
                let slot = self.slot(&ty)?;
                let name = self.member_name(&row);
                Some((self.static_slot(token, name, ty)?, slot))
            }
            _ => None,
        }
    }

    fn element(&mut self, token: u32) -> Option<Slot> {
        let rid = token & 0x00ff_ffff;
        match (token >> 24) as usize {
            TYPE_DEF if *self.value_types.get(rid as usize)? => self.slot(self.enums.get(&rid)?),
            TYPE_DEF => Some(Slot::REF),
            TYPE_REF => self.slot(self.primitives.get(&rid)?),
            TYPE_SPEC => self.slot(&sig::type_spec(self.md.blob(self.md.row(TYPE_SPEC, rid)?[0]))?),
            _ => None,
        }
    }

    fn string(&mut self, token: u32) -> Option<u32> {
        (token >> 24 == 0x70).then_some(())?;
        let offset = token & 0x00ff_ffff;
        if let Some(&(addr, _)) = self.string_slots.get(&offset) {
            return Some(addr);
        }
        let units = self.md.user_string(offset)?;
        let mut bytes: Vec<u8> = units.iter().flat_map(|u| u.to_le_bytes()).collect();
        bytes.extend_from_slice(&[0, 0]);
        let addr = self.alloc(&bytes, 2)?;
        self.string_slots.insert(offset, (addr, u32::try_from(bytes.len()).ok()?));
        Some(addr)
    }

    fn instance_field(&mut self, token: u32) -> Option<(u32, Slot)> {
        let rid = token & 0x00ff_ffff;
        let (field, class_args) = match (token >> 24) as usize {
            FIELD => (rid, Vec::new()),
            MEMBER_REF => {
                let row = *self.md.row(MEMBER_REF, rid)?;
                let (parent, parent_rid) = metadata::member_ref_parent(row[0])?;
                let (owner, args) = self.local_parent(parent, parent_rid)?;
                let name = self.md.string(row[1]);
                let field = self
                    .md
                    .owned(TYPE_DEF, owner, 4, FIELD)
                    .find(|&f| self.md.row(FIELD, f).is_some_and(|r| self.md.string(r[1]) == name))?;
                (field, args)
            }
            _ => return None,
        };
        let row = *self.md.row(FIELD, field)?;
        (row[0] & 0x10 == 0).then_some(())?;
        let offset = (*self.field_offset.get(field as usize)?)?;
        let ty = sig::field_sig(self.md.blob(row[2]))?.substitute(&class_args, &[]);
        Some((offset, self.slot(&ty)?))
    }

    fn handle(&mut self, token: u32) -> u32 {
        let rva = (token >> 24 == FIELD as u32).then(|| self.field_rva.get(&(token & 0x00ff_ffff))).flatten();
        rva.and_then(|&rva| addr32(CIL_BASE + u64::from(rva))).unwrap_or(token)
    }
}

impl CilImage {
    /// Map the assembly, resolve its tokens and type its method bodies.
    pub fn parse(file: &[u8]) -> KunaResult<CilImage> {
        let bad = |what: &str| KunaError::lowlevel(format!("managed .NET assembly: {what}"));
        let pe = PeLayout::parse(file).ok_or_else(|| bad("no CLI header"))?;
        let md = Metadata::parse(file, &pe).ok_or_else(|| bad("unreadable metadata"))?;
        let backed = |s: &metadata::Section| (s.raw_size as usize).min(file.len().saturating_sub(s.raw as usize));
        let mapped = pe.sections.iter().map(|s| u64::from(s.rva) + backed(s) as u64).max().unwrap_or(0);
        let image_size = (mapped + 0xfff) & !0xfff;
        let limit = MAX_IMAGE.min(MAX_SPREAD * file.len() as u64 + 0x1_0000);
        if image_size == 0 || image_size > limit {
            return Err(bad("image size out of range"));
        }
        let mut image = vec![0u8; image_size as usize];
        let headers = pe.sections.iter().map(|s| s.raw as usize).min().unwrap_or(0).min(file.len()).min(image.len());
        image[..headers].copy_from_slice(&file[..headers]);
        for s in &pe.sections {
            let (rva, raw, len) = (s.rva as usize, s.raw as usize, backed(s));
            if len > 0 {
                image[rva..rva + len].copy_from_slice(&file[raw..raw + len]);
            }
        }

        let type_count = md.table(TYPE_DEF).len() as u32;
        let nested: HashMap<u32, u32> = md.table(metadata::NESTED_CLASS).iter().map(|row| (row[0], row[1])).collect();
        let type_names: Vec<String> =
            std::iter::once(String::new()).chain((1..=type_count).map(|rid| sig::type_def_name(&md, &nested, rid))).collect();
        let mut primitives = HashMap::new();
        let mut handles = HashSet::new();
        for (i, row) in md.table(TYPE_REF).iter().enumerate() {
            if md.string(row[2]) != "System" {
                continue;
            }
            let name = md.string(row[1]);
            if let Some(ty) = sig::system_primitive(name) {
                primitives.insert(i as u32 + 1, ty);
            } else if matches!(name, "RuntimeFieldHandle" | "RuntimeTypeHandle" | "RuntimeMethodHandle") {
                handles.insert(i as u32 + 1);
            }
        }
        let extends = |row: &metadata::Row, name: &str| {
            metadata::type_def_or_ref(row[3]).is_some_and(|(table, rid)| {
                table == TYPE_REF
                    && md.row(TYPE_REF, rid).is_some_and(|r| md.string(r[2]) == "System" && md.string(r[1]) == name)
            })
        };
        let mut enums = HashMap::new();
        let mut value_types = vec![false];
        let mut field_owner = vec![0u32; md.table(FIELD).len() + 1];
        for rid in 1..=type_count {
            let row = &md.table(TYPE_DEF)[rid as usize - 1];
            let fields = md.owned(TYPE_DEF, rid, 4, FIELD);
            for field in fields.clone() {
                field_owner[field as usize] = rid;
            }
            let is_enum = extends(row, "Enum");
            value_types.push(is_enum || extends(row, "ValueType"));
            if is_enum {
                let underlying = fields
                    .filter_map(|f| md.row(FIELD, f))
                    .find(|f| f[0] & 0x10 == 0)
                    .and_then(|f| sig::field_sig(md.blob(f[2])));
                if let Some(ty) = underlying {
                    enums.insert(rid, ty);
                }
            }
        }
        let field_rva: HashMap<u32, u32> = md.table(FIELD_RVA).iter().map(|row| (row[1], row[0])).collect();

        let member_refs = md.table(MEMBER_REF).len() as u64;
        let method_count = md.table(METHOD_DEF).len();
        let mut method_addr = vec![0u32; method_count + 1];
        let mut bodies: Vec<(u32, u32, u32, BodyHeader)> = Vec::new();
        for (i, row) in md.table(METHOD_DEF).iter().enumerate() {
            let rid = i as u32 + 1;
            let header = (row[0] != 0 && row[1] & 3 == 0)
                .then(|| pe.offset(row[0]).and_then(|off| BodyHeader::parse(file.get(off..)?)))
                .flatten()
                .filter(|h| {
                    let end = u64::from(row[0]) + u64::from(h.header_size) + u64::from(h.code_size);
                    h.code_size > 0 && end <= image_size
                });
            let extern_addr = EXTERN_BASE + 8 * (member_refs + u64::from(rid) - 1);
            method_addr[rid as usize] = match &header {
                Some(h) => addr32(CIL_BASE + u64::from(row[0]) + u64::from(h.header_size)),
                None => addr32(extern_addr),
            }
            .ok_or_else(|| bad("too many methods"))?;
            if let Some(h) = header {
                bodies.push((rid, row[0] + h.header_size, h.code_size, h));
            }
        }

        let tail = addr32(CIL_BASE + image_size).ok_or_else(|| bad("image size out of range"))?;
        let mut loader = Loader {
            md,
            type_names,
            enums,
            primitives,
            handles,
            value_types,
            field_owner,
            method_addr,
            field_rva,
            tail,
            tail_data: Vec::new(),
            static_slots: BTreeMap::new(),
            string_slots: BTreeMap::new(),
            statics: Vec::new(),
            records: BTreeMap::new(),
            field_offset: Vec::new(),
            layout_tried: vec![false; type_count as usize + 1],
        };
        loader.field_offset = vec![None; loader.md.table(FIELD).len() + 1];
        for rid in 1..=type_count {
            loader.layout(rid, 0);
        }

        let overlapped = overlapping(&bodies.iter().map(|(_, rva, size, _)| (*rva, *size)).collect::<Vec<_>>());
        let typed: Vec<Option<Vec<body::Insn>>> = bodies
            .iter()
            .zip(&overlapped)
            .map(|((rid, code_rva, code_size, header), overlapped)| {
                let code = &image[*code_rva as usize..(*code_rva + *code_size) as usize];
                (!overlapped).then(|| loader.type_method(*rid, header, code)).flatten()
            })
            .collect();
        let mut methods = Vec::with_capacity(bodies.len());
        let mut protos = Vec::new();
        let mut paints = Vec::new();
        for ((rid, code_rva, code_size, _), typed) in bodies.iter().zip(&typed) {
            let addr = CIL_BASE + u64::from(*code_rva);
            for insn in typed.iter().flatten() {
                let at = addr + u64::from(insn.offset);
                paints.push((at, at + u64::from(insn.len), insn.context));
                if let Some((offset, value)) = insn.patch {
                    let at = (*code_rva + offset) as usize;
                    image[at..at + 4].copy_from_slice(&value.to_le_bytes());
                }
            }
            let name = loader.method_name(*rid);
            methods.push(CilMethod { addr, end: addr + u64::from(*code_size), name, typed: typed.is_some() });
        }
        for rid in 1..=method_count as u32 {
            let Some(row) = loader.md.row(METHOD_DEF, rid) else { continue };
            let Some(sig) = sig::method_sig(loader.md.blob(row[4])) else { continue };
            let mut param_names = vec![String::new(); sig.params.len()];
            for param in loader.md.owned(METHOD_DEF, rid, 5, PARAM) {
                let Some(p) = loader.md.row(PARAM, param) else { continue };
                if let Some(slot) = (p[1] as usize).checked_sub(1).and_then(|i| param_names.get_mut(i)) {
                    *slot = loader.md.string(p[2]).to_string();
                }
            }
            let owner = loader.method_owner(rid);
            let value_type = loader.value_types.get(owner as usize).copied().unwrap_or(false);
            protos.push(Proto {
                addr: u64::from(loader.method_addr[rid as usize]),
                name: loader.method_name(rid),
                this: sig.has_this.then_some(CilType::Named { value_type, table: TYPE_DEF, rid: owner }),
                sig,
                param_names,
            });
        }
        let mut externs = Vec::new();
        for (i, row) in loader.md.table(MEMBER_REF).iter().enumerate() {
            let Some((addr, sig)) = loader.method(MEMBER_REF, i as u32 + 1, &[], 0) else { continue };
            let name = loader.member_name(row);
            externs.push((u64::from(addr), name.clone()));
            let param_names = vec![String::new(); sig.params.len()];
            let this = sig.has_this.then(|| {
                let local = metadata::member_ref_parent(row[0]).and_then(|(table, rid)| loader.local_parent(table, rid));
                local.map_or(CilType::Object, |(rid, _)| CilType::Named { value_type: false, table: TYPE_DEF, rid })
            });
            protos.push(Proto { addr: u64::from(addr), name, this, sig, param_names });
        }
        for rid in 1..=method_count as u32 {
            let addr = u64::from(loader.method_addr[rid as usize]);
            if addr < CIL_BASE {
                externs.push((addr, loader.method_name(rid)));
            }
        }

        let entry_token = loader.md.entry_token;
        let entry = (entry_token >> 24 == METHOD_DEF as u32)
            .then(|| loader.method_addr.get((entry_token & 0x00ff_ffff) as usize).copied())
            .flatten()
            .map(u64::from)
            .filter(|&addr| addr >= CIL_BASE);
        image.extend_from_slice(&loader.tail_data);
        let mut runs = Vec::new();
        for s in &pe.sections {
            let (low, high) = (u64::from(s.rva), u64::from(s.rva) + backed(s) as u64);
            let inside = bodies
                .iter()
                .map(|(_, rva, size, _)| (u64::from(*rva), u64::from(*rva) + u64::from(*size)))
                .filter(|&(start, end)| start >= low && end <= high);
            let (first, last) = inside.fold((u64::MAX, 0), |(first, last), (start, end)| (first.min(start), last.max(end)));
            if first < last {
                runs.push((first, last));
            }
        }
        let sections = section_table(runs, image.len() as u64);
        Ok(CilImage {
            image,
            methods,
            entry,
            sections,
            protos,
            externs,
            statics: loader.statics,
            strings: loader.string_slots.values().map(|&(addr, len)| (u64::from(addr), len)).collect(),
            paints,
            enums: loader.enums,
            primitives: loader.primitives,
            records: loader.records,
        })
    }

    /// The facts the image states: its functions, the names and prototypes of
    /// everything a call can reach, the decode context, static fields and strings.
    pub fn facts(&self, arch: &Architecture) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let types = Types { image: self, arch, records: self.record_types(arch) };
        for method in &self.methods {
            out.symbols.push(SymFact { addr: method.addr, name: method.name.clone(), kind: SymKind::Function });
        }
        out.import_slot_names.extend(self.externs.iter().cloned());
        out.import_slot_names.push((THROW_ADDR, "throw".into()));
        out.noreturn.push(NoReturnFact { addr: THROW_ADDR, name: "throw".into() });
        for &(addr, end, value) in &self.paints {
            out.context_paints.push(ContextPaint { addr, end: Some(end), var: CONTEXT_VAR, value });
        }
        for (addr, name, ty) in &self.statics {
            if let Ok(type_) = types.datatype(ty) {
                out.typed_data.push(TypedDataFact { addr: *addr, name: name.clone(), type_ });
            }
        }
        for &(addr, len) in &self.strings {
            out.wide_strings.push(StringFact { addr, len });
            out.readonly.push((addr, addr + u64::from(len)));
        }
        for proto in &self.protos {
            if let Some(pieces) = types.pieces(proto) {
                out.prototypes_at.push((proto.addr, pieces));
            }
        }
        if let Ok(object) = types.datatype(&CilType::Object) {
            out.prototypes_at.push((
                THROW_ADDR,
                PrototypePieces {
                    name: "throw".into(),
                    outtype: arch.types().get_type_void().ok(),
                    intypes: vec![object],
                    innames: vec!["exception".into()],
                    first_var_arg_slot: -1,
                    ..Default::default()
                },
            ));
        }
        out
    }

    /// A struct per laid-out `TypeDef`: every record is declared first, so a
    /// field may point at any of them, then completed with its fields.
    fn record_types(&self, arch: &Architecture) -> HashMap<u32, Rc<Datatype>> {
        let factory = arch.types();
        let mut records = HashMap::new();
        for (&rid, record) in &self.records {
            let declared = match factory.find_by_name(&record.name) {
                Ok(Some(held)) => Some(held),
                _ => factory.kuna_declare_record(&record.name, false).ok(),
            };
            if let Some(shell) = declared {
                records.insert(rid, shell);
            }
        }
        let mut done = HashSet::new();
        for &rid in self.records.keys() {
            self.complete_record(rid, arch, &mut records, &mut done, 0);
        }
        records
    }

    fn complete_record(
        &self,
        rid: u32,
        arch: &Architecture,
        records: &mut HashMap<u32, Rc<Datatype>>,
        done: &mut HashSet<u32>,
        depth: u32,
    ) {
        let (Some(record), Some(shell)) = (self.records.get(&rid), records.get(&rid).cloned()) else { return };
        if depth > MAX_LAYOUT_DEPTH || !shell.is_incomplete() || !done.insert(rid) {
            return;
        }
        let mut embedded: Vec<u32> = record.base.into_iter().collect();
        embedded.extend(record.fields.iter().filter_map(|(_, _, ty)| match ty {
            CilType::Named { value_type: true, table: TYPE_DEF, rid } => Some(*rid),
            _ => None,
        }));
        for inner in embedded {
            self.complete_record(inner, arch, records, done, depth + 1);
        }
        let types = Types { image: self, arch, records: std::mem::take(records) };
        let mut fields = Vec::new();
        if let Some(base) = record.base.and_then(|base| types.records.get(&base)) {
            if !base.is_incomplete() {
                fields.push(TypeField::new(0, 0, "base".to_string(), Rc::clone(base)));
            }
        }
        for (name, offset, ty) in &record.fields {
            let field = match ty {
                CilType::Named { value_type: true, table: TYPE_DEF, rid } if self.records.contains_key(rid) => {
                    types.records.get(rid).filter(|held| !held.is_incomplete()).cloned()
                }
                _ => types.datatype(ty).ok().filter(|held| held.get_size() > 0),
            };
            let field = field.or_else(|| arch.types().get_base(8, type_metatype::TYPE_UNKNOWN).ok());
            if let Some(field) = field {
                fields.push(TypeField::new(*offset as i32, *offset as i32, name.clone(), field));
            }
        }
        *records = types.records;
        if fields.is_empty() {
            return;
        }
        if let Ok(complete) = arch.types().set_fields_struct_raw(&shell, fields, Vec::new(), record.size as i32, 8, 0) {
            records.insert(rid, complete);
        }
    }
}

/// The C types standing for CIL types on one architecture.
struct Types<'a> {
    image: &'a CilImage,
    arch: &'a Architecture,
    records: HashMap<u32, Rc<Datatype>>,
}

impl Types<'_> {
    fn pieces(&self, proto: &Proto) -> Option<PrototypePieces> {
        let mut intypes = Vec::new();
        let mut innames = Vec::new();
        if let Some(this) = &proto.this {
            let pointee = match this {
                CilType::Named { table: TYPE_DEF, rid, .. } => self.records.get(rid).cloned(),
                _ => None,
            };
            intypes.push(match pointee {
                Some(record) => self.arch.types().get_type_pointer(8, record, 1).ok()?,
                None => self.datatype(&CilType::Object).ok()?,
            });
            innames.push("this".to_string());
        }
        for (ty, name) in proto.sig.params.iter().zip(&proto.param_names) {
            intypes.push(self.datatype(ty).ok()?);
            innames.push(name.clone());
        }
        Some(PrototypePieces {
            name: proto.name.clone(),
            outtype: Some(self.datatype(&proto.sig.ret).ok()?),
            intypes,
            innames,
            first_var_arg_slot: -1,
            ..Default::default()
        })
    }

    /// The C type standing for a CIL type: a reference is a pointer (to the
    /// struct of a class this image lays out), an array reference a pointer
    /// to its element and a string one to UTF-16 text.
    fn datatype(&self, ty: &CilType) -> KunaResult<Rc<Datatype>> {
        let types = self.arch.types();
        let pointer = |to: Rc<Datatype>| types.get_type_pointer(8, to, 1);
        let int = |size, signed| types.get_base(size, if signed { type_metatype::TYPE_INT } else { type_metatype::TYPE_UINT });
        match ty {
            CilType::Void => types.get_type_void(),
            CilType::Bool => types.get_base(1, type_metatype::TYPE_BOOL),
            CilType::Char => types.get_type_char(2),
            CilType::I1 => int(1, true),
            CilType::U1 => int(1, false),
            CilType::I2 => int(2, true),
            CilType::U2 => int(2, false),
            CilType::I4 => int(4, true),
            CilType::U4 => int(4, false),
            CilType::I8 => int(8, true),
            CilType::U8 => int(8, false),
            CilType::R4 => types.get_base(4, type_metatype::TYPE_FLOAT),
            CilType::R8 => types.get_base(8, type_metatype::TYPE_FLOAT),
            CilType::Native { signed } => int(8, *signed),
            CilType::String => pointer(types.get_type_char(2)?),
            CilType::SzArray(elem) | CilType::ByRef(elem) | CilType::Ptr(elem) if **elem != CilType::Void => {
                pointer(self.datatype(elem)?)
            }
            CilType::Inst(base, _) => self.datatype(base),
            CilType::Named { value_type, table, rid } => {
                let known = if *table == TYPE_DEF { self.image.enums.get(rid) } else { self.image.primitives.get(rid) };
                let record = (*table == TYPE_DEF).then(|| self.records.get(rid)).flatten();
                match (known, record) {
                    (Some(ty), _) => self.datatype(ty),
                    (None, Some(record)) if *value_type && !record.is_incomplete() => Ok(Rc::clone(record)),
                    (None, Some(record)) if !*value_type => pointer(Rc::clone(record)),
                    _ => pointer(types.get_type_void()?),
                }
            }
            _ => pointer(types.get_type_void()?),
        }
    }
}

impl Loader<'_> {
    /// Type one method's body; `None` when the lift does not cover it.
    fn type_method(&mut self, rid: u32, header: &BodyHeader, code: &[u8]) -> Option<Vec<body::Insn>> {
        if header.has_clauses {
            return None;
        }
        let row = *self.md.row(METHOD_DEF, rid)?;
        let sig = sig::method_sig(self.md.blob(row[4]))?;
        let mut args = Vec::new();
        if sig.has_this {
            args.push(Slot::REF);
        }
        for param in &sig.params {
            args.push(self.slot(param)?);
        }
        let ret = match sig.ret {
            CilType::Void => None,
            ref ty => Some(self.slot(ty)?),
        };
        let locals: Vec<Option<Slot>> = match header.locals_token {
            0 => Vec::new(),
            token => {
                (token >> 24 == metadata::STANDALONE_SIG as u32).then_some(())?;
                let blob = self.md.blob(self.md.row(metadata::STANDALONE_SIG, token & 0x00ff_ffff)?[0]);
                sig::locals_sig(blob)?.iter().map(|ty| self.slot(ty)).collect()
            }
        };
        body::type_body(code, &Frame { args: &args, locals: &locals, ret }, self)
    }
}

#[cfg(test)]
mod tests {
    use super::body::{type_body, Callee, Frame, Insn, Kind, Resolve, Slot, LOAD_S4};
    use super::*;

    const HELLO: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/cil_hello.dll"));

    #[test]
    fn the_fixture_lists_its_method_bodies() {
        let image = CilImage::parse(HELLO).unwrap();
        let listed: Vec<(u64, &str, bool)> =
            image.methods.iter().map(|m| (m.addr, m.name.as_str(), m.typed)).collect();
        assert_eq!(
            listed,
            [
                (0x40_2051, "Program::<Main>$", true),
                (0x40_206e, "Program::.ctor", true),
                (0x40_2084, "Program::<<Main>$>g__Sum|0_0", true),
            ]
        );
        assert_eq!(image.entry, Some(0x40_2051));
    }

    #[test]
    fn only_the_method_bodies_are_a_code_section() {
        let image = CilImage::parse(HELLO).unwrap();
        let (code, data) = (section_flags::CODE, section_flags::DATA);
        assert_eq!(
            image.sections,
            [(0x40_0000, 0x2051, data), (0x40_2051, 0x4f, code), (0x40_20a0, image.image.len() as u64 - 0x20a0, data)]
        );
        assert_eq!(section_table(vec![(8, 12), (4, 10), (20, 99)], 32), [
            (CIL_BASE, 4, data),
            (CIL_BASE + 4, 6, code),
            (CIL_BASE + 10, 2, code),
            (CIL_BASE + 12, 20, data),
        ]);
        assert_eq!(section_table(Vec::new(), 0), []);
    }

    #[test]
    fn bodies_that_partly_coincide_are_refused_and_a_shared_body_is_not() {
        assert_eq!(overlapping(&[(0, 10), (20, 80), (5, 3)]), [true, false, true]);
        assert_eq!(overlapping(&[(0, 10), (1, 49), (5, 3)]), [true, true, true]);
        assert_eq!(overlapping(&[(8, 4), (8, 4), (12, 4)]), [false, false, false]);
        assert_eq!(overlapping(&[(0, 10), (0, 50)]), [true, true]);
        assert_eq!(overlapping(&[]), [false; 0]);
    }

    #[test]
    fn a_call_token_becomes_the_address_of_its_target() {
        let image = CilImage::parse(HELLO).unwrap();
        let call = 0x2051 + 0x11;
        assert_eq!(image.image[call], 0x28);
        assert_eq!(image.image[call + 1..call + 5], 0x40_2084u32.to_le_bytes());
        assert_eq!(HELLO[0x251 + 0x11], 0x28, "the file holds the call at its raw offset");
        assert_ne!(HELLO[0x251 + 0x12..0x251 + 0x16], 0x40_2084u32.to_le_bytes(), "and still names a token");
    }

    #[test]
    fn only_a_pe_with_metadata_is_a_managed_image() {
        assert!(is_cil_image(HELLO));
        assert!(!is_cil_image(b"MZ"));
        assert!(!is_cil_image(b"\x7fELF\x02\x01\x01"));
        let mut no_cli = HELLO.to_vec();
        let pe = u32::from_le_bytes(no_cli[0x3c..0x40].try_into().unwrap()) as usize;
        let cli_dir = pe + 24 + 96 + 14 * 8;
        no_cli[cli_dir..cli_dir + 4].fill(0);
        assert!(!is_cil_image(&no_cli));
    }

    #[test]
    fn a_section_table_the_file_cannot_back_is_refused() {
        let pe = u32::from_le_bytes(HELLO[0x3c..0x40].try_into().unwrap()) as usize;
        let sections = pe + 24 + usize::from(u16::from_le_bytes(HELLO[pe + 20..pe + 22].try_into().unwrap()));
        let mut far = HELLO.to_vec();
        far[sections + 40 + 12..sections + 40 + 16].copy_from_slice(&0x3000_0000u32.to_le_bytes());
        assert!(CilImage::parse(&far).is_err(), "a section a gigabyte up in a 5 KiB file");
        assert!(CilImage::parse(HELLO).unwrap().image.len() <= 0x1_0000);
    }

    #[test]
    fn the_header_probe_decides_from_the_start_of_the_file() {
        assert_eq!(has_cli_header(&HELLO[..CLI_PROBE_LEN]), Some(true));
        assert_eq!(has_cli_header(&HELLO[..0x100]), None, "cut before the data directories");
        assert_eq!(has_cli_header(b"\x7fELF\x02\x01\x01"), Some(false));
        assert_eq!(has_cli_header(b""), Some(false));
        let pe = u32::from_le_bytes(HELLO[0x3c..0x40].try_into().unwrap()) as usize;
        let cli_dir = pe + 24 + 96 + 14 * 8;
        let mut native = HELLO[..CLI_PROBE_LEN].to_vec();
        native[cli_dir..cli_dir + 4].fill(0);
        assert_eq!(has_cli_header(&native), Some(false));
        for len in 0..CLI_PROBE_LEN {
            let _ = has_cli_header(&HELLO[..len]);
        }
    }

    #[test]
    fn truncated_and_corrupt_images_are_refused_without_a_panic() {
        for len in 0..HELLO.len() {
            let _ = CilImage::parse(&HELLO[..len]);
        }
        let mut bytes = HELLO.to_vec();
        for at in 0..bytes.len() {
            let kept = bytes[at];
            for value in [0x00, 0x7f, 0xff] {
                bytes[at] = value;
                let _ = CilImage::parse(&bytes);
            }
            bytes[at] = kept;
        }
        assert_eq!(bytes, HELLO, "every byte was put back");
        assert_eq!(CilImage::parse(&bytes).unwrap().methods.len(), 3);
    }

    struct NoTokens;

    impl Resolve for NoTokens {
        fn callee(&mut self, _: u32) -> Option<Callee> {
            None
        }
        fn static_field(&mut self, _: u32) -> Option<(u32, Slot)> {
            None
        }
        fn instance_field(&mut self, _: u32) -> Option<(u32, Slot)> {
            None
        }
        fn element(&mut self, _: u32) -> Option<Slot> {
            None
        }
        fn string(&mut self, _: u32) -> Option<u32> {
            None
        }
        fn handle(&mut self, token: u32) -> u32 {
            token
        }
    }

    fn walk(code: &[u8], args: &[Slot], ret: Option<Slot>) -> Option<Vec<Insn>> {
        type_body(code, &Frame { args, locals: &[], ret }, &mut NoTokens)
    }

    fn depths(insns: &[Insn]) -> Vec<(u32, u32, u32)> {
        insns.iter().map(|i| (i.offset, i.context >> 26, i.context >> 22 & 15)).collect()
    }

    const INT: Slot = Slot { kind: Kind::I4, class: LOAD_S4 };

    #[test]
    fn the_walk_records_depth_and_operand_class() {
        // ldc.i4.1; ldc.i4.2; add; ret
        let int_add = walk(&[0x17, 0x18, 0x58, 0x2a], &[], Some(INT)).unwrap();
        assert_eq!(depths(&int_add), [(0, 0, 0), (1, 1, 0), (2, 2, 0), (3, 1, 0)]);
        // ldc.r8 1.0; ldc.r8 2.0; add; pop; ret
        let mut code = vec![0x23];
        code.extend(1.0f64.to_le_bytes());
        code.push(0x23);
        code.extend(2.0f64.to_le_bytes());
        code.extend([0x58, 0x26, 0x2a]);
        let float_add = walk(&code, &[], None).unwrap();
        assert_eq!(depths(&float_add)[2], (18, 2, 5), "add of two float64");
        assert_eq!(depths(&float_add)[4], (20, 0, 15), "ret of nothing");
    }

    #[test]
    fn the_walk_follows_both_arms_and_skips_dead_code() {
        // ldarg.0; brfalse.s +2; ldc.i4.1; ret; ldc.i4.2; ret; nop
        let insns = walk(&[0x02, 0x2c, 0x02, 0x17, 0x2a, 0x18, 0x2a, 0x00], &[INT], Some(INT)).unwrap();
        let offsets: Vec<u32> = insns.iter().map(|i| i.offset).collect();
        assert_eq!(offsets, [0, 1, 3, 4, 5, 6], "the trailing nop is unreachable");
    }

    #[test]
    fn a_body_the_lift_does_not_cover_is_refused_whole() {
        assert!(walk(&[0x0f, 0x00, 0x26, 0x2a], &[INT], None).is_none(), "ldarga");
        assert!(walk(&[0x17, 0x2a], &[], None).is_none(), "a value left on the stack at ret");
        assert!(walk(&[0x58, 0x2a], &[], None).is_none(), "add on an empty stack");
        // ldarg.0; brfalse.s +1; ldc.i4.1; ret: the join is reached at depth 0 and 1
        assert!(walk(&[0x02, 0x2c, 0x01, 0x17, 0x2a], &[INT], None).is_none());
        // ldc.i4.1; ldc.r8 1.0; add
        let mut mixed = vec![0x17, 0x23];
        mixed.extend(1.0f64.to_le_bytes());
        mixed.extend([0x58, 0x26, 0x2a]);
        assert!(walk(&mixed, &[], None).is_none(), "int32 + float64");
        assert!(walk(&[0x2b, 0x7f], &[], None).is_none(), "a branch out of the body");
        assert!(walk(&[0x17], &[], None).is_none(), "the body runs off its end");
    }
}
