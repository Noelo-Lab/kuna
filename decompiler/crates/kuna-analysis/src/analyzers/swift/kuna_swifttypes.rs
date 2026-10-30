//! (kuna `swifttypes`) Swift type layouts from static type metadata.
//!
//! A non-generic type whose metadata needs no runtime initialization has it
//! laid out statically, and its accessor is the one-line function that
//! returns the address (`adrp x0, …; add x0, x0, …; mov x1, #0; ret` on
//! arm64, `lea rax, [rip + …]; xor edx, edx; ret` on x86-64; a class with
//! Objective-C heritage tail-calls the runtime with the same address). From
//! the metadata: a struct's size and alignment (its value witness table, or
//! the builtin table an import names), its field offset vector
//! (32-bit entries); a class's instance size and alignment and its field
//! offset vector (pointer-sized entries). From the reflection field
//! descriptor: each stored property's name and mangled type, whose
//! symbolic references the descriptor table resolves. Together they give a
//! kuna struct per type — `Module.Point { long x; long y; }`, a class as the
//! object layout with a local superclass embedded as `super` — interned in
//! the type factory under the type's dotted name.
//!
//! A field type outside the mapping (an imported value type, a generic
//! instantiation without a known layout, an enum) is `undefined<n>` for the
//! bytes up to the next field: the layout is always right, the names are
//! always real, only the field's own type is withheld.
//!
//! What binds today: the typed metadata records and value witness tables,
//! the accessor prototypes (`swift.type_metadata *f(long request)`), the
//! `self` of a class's vtable getter (the `swiftself` register, x20 / r13,
//! which no calling-convention model describes, so it is pinned on the
//! function as a register-locked parameter) and the `self` of a local
//! protocol's witness getter (self indirect in `swiftself`, the type
//! metadata and witness table in the first two argument registers).

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_decomp::dtype::{Datatype, TypeFactory, TypeField, type_metatype};
use kuna_decomp::fspec::PrototypePieces;

use super::Namer;
use super::descriptors::{
    Context,
    ContextKind,
    FieldDescriptor,
    Metadata,
    MethodKind,
    Reader,
    Target,
};
use crate::demangle::kuna_swift::node::{Kind, Node, NodeRef};
use crate::demangle::kuna_swift::parse::mk_type;
use crate::demangle::kuna_swift::print::print_full;
use crate::demangle::kuna_swift::{demangle_swift_tree, demangle_swift_type};
use crate::objc::sections::MachoImage;
use crate::pass::{AnalysisOutput, ParamLockFact, TypedDataFact};

/// The `swiftself` register and the first two argument registers.
pub struct Registers {
    pub self_reg: &'static str,
    pub args: [&'static str; 2],
}

impl Registers {
    pub fn for_arch(arch: object::Architecture) -> Option<Registers> {
        Some(match arch {
            object::Architecture::Aarch64 => Registers {
                self_reg: "x20",
                args: ["x0", "x1"],
            },
            object::Architecture::X86_64 => Registers {
                self_reg: "R13",
                args: ["RDI", "RSI"],
            },
            _ => return None,
        })
    }
}

/// The metadata address a non-generic accessor returns, decoded from its
/// first instructions, or `None` for any other body (a cache lookup, a
/// runtime call with the pattern as argument).
pub fn locate_metadata(img: &MachoImage, arch: object::Architecture, accessor: u64) -> Option<u64> {
    match arch {
        object::Architecture::Aarch64 => locate_arm64(img, accessor),
        object::Architecture::X86_64 => locate_x86_64(img, accessor),
        _ => None,
    }
}

fn locate_arm64(img: &MachoImage, accessor: u64) -> Option<u64> {
    let bytes = img.read(accessor, 20)?;
    let word = |i: usize| u32::from_le_bytes(bytes[i * 4..i * 4 + 4].try_into().ok()?).into();
    let words: Vec<Option<u32>> = (0..5).map(word).collect();
    let mut i = 0;
    // `stp x29, x30, [sp, #-16]!; mov x29, sp` opens the accessor that
    // calls the runtime.
    if words[0] == Some(0xa9bf_7bfd) && words[1] == Some(0x9100_03fd) {
        i = 2;
    }
    let adrp = words[i]?;
    if adrp & 0x9f00_0000 != 0x9000_0000 || adrp & 31 != 0 {
        return None;
    }
    let immlo = (adrp >> 29) & 3;
    let immhi = (adrp >> 5) & 0x7_ffff;
    let imm = ((immhi << 2) | immlo) as i64;
    let imm = if imm & (1 << 20) != 0 {
        imm - (1 << 21)
    } else {
        imm
    };
    let page = ((accessor + 4 * i as u64) & !0xfff).wrapping_add((imm * 4096) as u64);
    let add = words[i + 1]?;
    if add & 0xff80_0000 != 0x9100_0000 || add & 31 != 0 || (add >> 5) & 31 != 0 {
        return None;
    }
    let mut imm12 = ((add >> 10) & 0xfff) as u64;
    if (add >> 22) & 1 != 0 {
        imm12 <<= 12;
    }
    let next = words[i + 2]?;
    let plain = next == 0x5280_0001 || next == 0xd280_0001 || next == 0xd65f_03c0;
    let call = next & 0xfc00_0000 == 0x9400_0000 || next & 0xfc00_0000 == 0x1400_0000;
    (plain || call).then_some(page.wrapping_add(imm12))
}

fn locate_x86_64(img: &MachoImage, accessor: u64) -> Option<u64> {
    let bytes = img.read(accessor, 16)?;
    let mut i = 0;
    if bytes.starts_with(&[0x55, 0x48, 0x89, 0xe5]) {
        i = 4;
    }
    if bytes.get(i..i + 3)? != [0x48, 0x8d, 0x05] {
        return None;
    }
    let disp = i32::from_le_bytes(bytes.get(i + 3..i + 7)?.try_into().ok()?);
    let next = accessor + (i + 7) as u64;
    let tail = bytes.get(i + 7..)?;
    let ok = tail.starts_with(&[0x31, 0xd2])
        || tail.starts_with(&[0xc3])
        || tail.starts_with(&[0xe8])
        || tail.starts_with(&[0xe9])
        || tail.starts_with(&[0xeb]);
    ok.then_some(next.wrapping_add(disp as i64 as u64))
}

/// Offsets in a value witness table: the size word after the eight
/// function pointers, and the flags word (bits 0-7 the alignment mask).
const VWT_SIZE: u64 = 64;
const VWT_FLAGS: u64 = 80;

/// The size of the value a builtin value witness table an import names
/// describes (`$sBi64_WV`, `$sBoWV`, `$sytWV`).
fn builtin_witness_size(import: &str) -> Option<u32> {
    let body = import
        .trim_start_matches('_')
        .strip_prefix("$s")?
        .strip_suffix("WV")?;
    Some(match body {
        "Bi8_" | "Bi1_" => 1,
        "Bi16_" => 2,
        "Bi32_" | "Bf32_" => 4,
        "Bi64_" | "Bf64_" | "Bo" | "BO" | "Bb" | "Bp" => 8,
        "yt" => 0,
        _ => return None,
    })
}

pub struct Types<'a> {
    pub md: &'a Metadata,
    pub reader: &'a Reader<'a>,
    pub imports: &'a HashMap<u64, String>,
    pub namer: &'a Namer<'a>,
    pub tf: &'a dyn TypeFactory,
    pub arch: object::Architecture,
    /// Whether class metadata carries the Objective-C `cache` and `data`
    /// words (a Mach-O; an ELF has no Objective-C interop).
    pub objc_interop: bool,
    /// Context descriptor address → its static metadata address.
    pub metadata: HashMap<u64, u64>,
    by_metadata: HashMap<u64, u64>,
    fields_by_addr: HashMap<u64, &'a FieldDescriptor>,
    /// Dotted path → the one descriptor spelled that way; a path two
    /// descriptors share (private types of the same name in different
    /// files) names neither, so no layout is ever read through the wrong
    /// name.
    by_path: HashMap<String, u64>,
    ambiguous: HashSet<String>,
    built: RefCell<HashMap<u64, Option<Rc<Datatype>>>>,
    in_progress: RefCell<HashSet<u64>>,
    ptr_size: i32,
}

/// What the pass did, for the dump and the report.
#[derive(Default, Debug)]
pub struct Stats {
    pub located: usize,
    pub structs: usize,
    pub classes: usize,
    pub fields: usize,
    pub typed_fields: usize,
    pub vtable_getters: usize,
    pub witness_getters: usize,
}

impl<'a> Types<'a> {
    pub fn new(
        md: &'a Metadata,
        reader: &'a Reader<'a>,
        imports: &'a HashMap<u64, String>,
        namer: &'a Namer<'a>,
        tf: &'a dyn TypeFactory,
        arch: object::Architecture,
        objc_interop: bool,
    ) -> Types<'a> {
        let fields_by_addr = md.fields.iter().map(|f| (f.addr, f)).collect();
        let mut by_path: HashMap<String, u64> = HashMap::new();
        let mut ambiguous = HashSet::new();
        for ctx in md.contexts.values() {
            if ctx.kind.is_nominal() {
                if let Some(p) = namer.path(ctx.addr, 0) {
                    if by_path.insert(p.clone(), ctx.addr).is_some() {
                        ambiguous.insert(p);
                    }
                }
            }
        }
        for p in &ambiguous {
            by_path.remove(p);
        }
        Types {
            md,
            reader,
            imports,
            namer,
            tf,
            arch,
            objc_interop,
            metadata: HashMap::new(),
            by_metadata: HashMap::new(),
            fields_by_addr,
            by_path,
            ambiguous,
            built: RefCell::new(HashMap::new()),
            in_progress: RefCell::new(HashSet::new()),
            ptr_size: md.ptr_size as i32,
        }
    }

    fn img(&self) -> &'a MachoImage<'a> {
        self.reader.img
    }

    /// The byte offset of a class metadata's `flags` word: after `isa`,
    /// `superclass` and, with Objective-C interop, the two cache words and
    /// the `data` word.
    fn class_flags_offset(&self) -> u64 {
        let words = if self.objc_interop { 5 } else { 2 };
        words * self.md.ptr_size
    }

    /// Find every statically laid-out type's metadata and check it points
    /// back at the descriptor.
    pub fn locate_all(&mut self) {
        for ctx in self.md.contexts.values() {
            if !ctx.kind.is_nominal() || ctx.flags & 0x80 != 0 || (ctx.flags >> 16) & 3 != 0 {
                continue;
            }
            let Some(accessor) = ctx.accessor else {
                continue;
            };
            let Some(meta) = locate_metadata(self.img(), self.arch, accessor) else {
                continue;
            };
            let descriptor_slot = match ctx.kind {
                ContextKind::Class => meta + self.class_flags_offset() + 24,
                _ => meta + self.md.ptr_size,
            };
            let kind_ok = match ctx.kind {
                ContextKind::Class => true,
                _ => matches!(self.img().read_ptr(meta), Some(0x200..=0x202)),
            };
            if kind_ok && self.img().read_ptr(descriptor_slot) == Some(ctx.addr) {
                self.metadata.insert(ctx.addr, meta);
                self.by_metadata.insert(meta, ctx.addr);
            }
        }
    }

    /// The dotted path of the type at `addr`, unless another type shares it.
    fn unique_path(&self, addr: u64) -> Option<String> {
        let path = self.namer.path(addr, 0)?;
        (!self.ambiguous.contains(&path)).then_some(path)
    }

    fn word(&self, at: u64) -> Option<u64> {
        self.img().read_ptr(at)
    }

    /// A struct's `(size, alignment)` from its value witness table.
    fn struct_extent(&self, meta: u64) -> Option<(u32, u32)> {
        let slot = meta - self.md.ptr_size;
        if self.reader.bind_slots.contains(&slot) {
            let size = builtin_witness_size(self.imports.get(&slot)?)?;
            return Some((size, size.max(1)));
        }
        let vwt = self.word(slot)?;
        let size = u32::try_from(self.word(vwt + VWT_SIZE)?).ok()?;
        let flags = self.img().read_u32(vwt + VWT_FLAGS)?;
        Some((size, (flags & 0xff) + 1))
    }

    /// The local value witness table of the struct or enum at `meta`.
    pub fn local_witness_table(&self, meta: u64) -> Option<u64> {
        let slot = meta - self.md.ptr_size;
        if self.reader.bind_slots.contains(&slot) {
            return None;
        }
        self.word(slot)
    }

    fn records(&self, ctx: &Context) -> Option<Vec<(String, Vec<u8>, u64)>> {
        let fd = self.fields_by_addr.get(&ctx.fields?)?;
        if fd.records.len() != ctx.num_fields as usize {
            return None;
        }
        let mut out = Vec::new();
        for r in &fd.records {
            let name = r.name.clone()?;
            let (bytes, at) = match r.mangled_type_name {
                Some(at) => (self.reader.mangled_bytes(at)?, at),
                None => (Vec::new(), 0),
            };
            out.push((name, bytes, at));
        }
        Some(out)
    }

    /// The kuna struct of the nominal type at `addr` (built on first use),
    /// `None` when its layout is not static.
    pub fn struct_type(&self, addr: u64) -> Option<Rc<Datatype>> {
        if let Some(t) = self.built.borrow().get(&addr) {
            return t.clone();
        }
        if !self.in_progress.borrow_mut().insert(addr) {
            return None;
        }
        let t = self.build(addr);
        self.in_progress.borrow_mut().remove(&addr);
        self.built.borrow_mut().insert(addr, t.clone());
        t
    }

    fn build(&self, addr: u64) -> Option<Rc<Datatype>> {
        let ctx = self.md.contexts.get(&addr)?;
        let meta = *self.metadata.get(&addr)?;
        let path = self.unique_path(addr)?;
        let existing = self.tf.find_by_name(&path).ok().flatten();
        let shell = match existing {
            Some(t) if t.get_metatype() == type_metatype::TYPE_STRUCT && t.get_size() == 0 => t,
            Some(_) => return None,
            None => self.tf.get_type_struct(&path).ok()?,
        };
        match ctx.kind {
            ContextKind::Struct => self.build_struct(ctx, meta, &shell),
            ContextKind::Class => self.build_class(ctx, meta, &shell),
            _ => None,
        }
    }

    fn build_struct(&self, ctx: &Context, meta: u64, shell: &Rc<Datatype>) -> Option<Rc<Datatype>> {
        let (size, align) = self.struct_extent(meta)?;
        if size == 0 || size > 1 << 20 {
            return None;
        }
        let records = self.records(ctx)?;
        let vector = meta + ctx.field_offset_vector as u64 * self.md.ptr_size;
        let mut offsets = Vec::new();
        for i in 0..records.len() {
            offsets.push(self.img().read_u32(vector + 4 * i as u64)?);
        }
        let fields = self.fields(&records, &offsets, size)?;
        self.tf
            .set_fields_struct_raw(
                shell,
                fields,
                Vec::new(),
                size as i32,
                align.min(16) as i32,
                0,
            )
            .ok()
    }

    fn build_class(&self, ctx: &Context, meta: u64, shell: &Rc<Datatype>) -> Option<Rc<Datatype>> {
        let flags = meta + self.class_flags_offset();
        let size = self.img().read_u32(flags + 8)?;
        let align = (self.img().read_u32(flags + 12)? & 0xffff) + 1;
        if size == 0 || size > 1 << 20 {
            return None;
        }
        let records = self.records(ctx)?;
        let vector = meta + ctx.field_offset_vector as u64 * self.md.ptr_size;
        let mut offsets = Vec::new();
        for i in 0..records.len() {
            offsets.push(u32::try_from(self.word(vector + self.md.ptr_size * i as u64)?).ok()?);
        }
        let mut fields = self.superclass_field(meta);
        let header_end = fields
            .iter()
            .map(|f| (f.offset + f.field_type.get_size()) as u32)
            .max()
            .unwrap_or(0);
        let own = self.fields(&records, &offsets, size)?;
        fields.extend(own.into_iter().filter(|f| f.offset as u32 >= header_end));
        self.tf
            .set_fields_struct_raw(
                shell,
                fields,
                Vec::new(),
                size as i32,
                align.min(16) as i32,
                0,
            )
            .ok()
    }

    /// A local superclass with a static layout embedded as `super` at 0;
    /// otherwise the `isa` word, and for a Swift-native root (no
    /// superclass, or the runtime's `SwiftObject`) the reference counts
    /// after it.
    fn superclass_field(&self, meta: u64) -> Vec<TypeField> {
        let slot = meta + self.md.ptr_size;
        let bound = self.reader.bind_slots.contains(&slot);
        let sup_meta = self.word(slot).filter(|&p| p != 0 && !bound);
        let local = sup_meta.and_then(|m| self.by_metadata.get(&m).copied());
        if let Some(sup) = local
            .and_then(|a| self.struct_type(a))
            .filter(|t| t.get_size() > 0)
        {
            return vec![TypeField::new(0, 0, "super".to_string(), sup)];
        }
        let Some(ptr) = self.void_ptr() else {
            return Vec::new();
        };
        let mut header = vec![TypeField::new(0, 0, "isa".to_string(), ptr)];
        let swift_root = sup_meta.is_none()
            && (!bound
                || self
                    .imports
                    .get(&slot)
                    .is_some_and(|n| n.ends_with("_TtCs12_SwiftObject")));
        if swift_root {
            if let Ok(word) = self.tf.get_base(8, type_metatype::TYPE_UINT) {
                header.push(TypeField::new(1, 8, "refcounts".to_string(), word));
            }
        }
        header
    }

    fn void_ptr(&self) -> Option<Rc<Datatype>> {
        let void = self.tf.get_type_void().ok()?;
        self.tf.get_type_pointer(self.ptr_size, void, 1).ok()
    }

    fn unknown(&self, n: u32) -> Option<Rc<Datatype>> {
        match n {
            0 => None,
            1 | 2 | 4 | 8 => self.tf.get_base(n as i32, type_metatype::TYPE_UNKNOWN).ok(),
            n => {
                let byte = self.tf.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?;
                self.tf.get_type_array(n as i32, byte).ok()
            }
        }
    }

    /// The typed fields of a layout: each stored property at its offset,
    /// typed from its mangled name or as the bytes up to the next field.
    fn fields(
        &self,
        records: &[(String, Vec<u8>, u64)],
        offsets: &[u32],
        size: u32,
    ) -> Option<Vec<TypeField>> {
        let mut order: Vec<usize> = (0..records.len()).collect();
        order.sort_by_key(|&i| offsets[i]);
        let mut fields = Vec::new();
        let mut prev_end = 0u32;
        for (k, &i) in order.iter().enumerate() {
            let offset = offsets[i];
            let next = order.get(k + 1).map_or(size, |&j| offsets[j]);
            if offset > size || next < offset {
                return None;
            }
            let hint = next - offset;
            let (name, mangled, at) = &records[i];
            let ty = self
                .field_type(mangled, *at, hint)
                .or_else(|| self.unknown(hint));
            let Some(ty) = ty else { continue };
            let end = offset + ty.get_size() as u32;
            if offset < prev_end || end > size {
                return None;
            }
            prev_end = end;
            fields.push(TypeField::new(i as i32, offset as i32, name.clone(), ty));
        }
        Some(fields)
    }

    /// The node a symbolic reference in a mangled name at `base` stands for.
    fn resolve(&self, base: u64, kind: u8, at: usize, value: u64) -> Option<NodeRef> {
        let here = base + at as u64;
        let offset = value as u32 as i32 as i64 as u64;
        match kind {
            0x01 => self.context_node(here.wrapping_add(offset), 0),
            0x02 => {
                let slot = here.wrapping_add(offset);
                if self.reader.bind_slots.contains(&slot) {
                    return imported_node(self.imports.get(&slot)?);
                }
                let target = self.word(slot)?;
                self.context_node(target, 0)
            }
            _ => None,
        }
    }

    /// The demangler tree of a local context: `Type(Structure(Module,
    /// Identifier))` for a nominal type, a `Module` node for a module.
    fn context_node(&self, addr: u64, depth: usize) -> Option<NodeRef> {
        if depth > 32 {
            return None;
        }
        let ctx = self.md.contexts.get(&addr)?;
        let parent = |this: &Self| -> Option<NodeRef> {
            match ctx.parent? {
                Target::Direct(p) => this.context_node(p, depth + 1),
                Target::Slot(slot) => imported_node(this.imports.get(&slot)?),
            }
        };
        match ctx.kind {
            ContextKind::Module => Some(Node::text(Kind::Module, ctx.name.clone()?)),
            ContextKind::Anonymous => parent(self),
            ContextKind::Class
            | ContextKind::Struct
            | ContextKind::Enum
            | ContextKind::Protocol => {
                let kind = match ctx.kind {
                    ContextKind::Class => Kind::Class,
                    ContextKind::Struct => Kind::Structure,
                    ContextKind::Enum => Kind::Enum,
                    _ => Kind::Protocol,
                };
                let name = ctx.name.clone().filter(|n| !n.is_empty())?;
                let p = parent(self)?;
                let p = if p.kind == Kind::Type {
                    Rc::clone(p.first()?)
                } else {
                    p
                };
                Some(mk_type(Node::with(
                    kind,
                    vec![p, Node::text(Kind::Identifier, name)],
                )))
            }
            _ => None,
        }
    }

    fn field_type(&self, mangled: &[u8], at: u64, hint: u32) -> Option<Rc<Datatype>> {
        if mangled.is_empty() {
            return None;
        }
        let resolver = |kind: u8, pos: usize, value: u64| self.resolve(at, kind, pos, value);
        let node = demangle_swift_type(mangled, &resolver)?;
        self.kuna_type(&node, hint)
    }

    /// The kuna type of a demangled Swift type, sized `hint` bytes in its
    /// container, or `None` for one outside the mapping.
    fn kuna_type(&self, node: &NodeRef, hint: u32) -> Option<Rc<Datatype>> {
        let inner = if node.kind == Kind::Type {
            node.first()?
        } else {
            node
        };
        let tf = self.tf;
        match inner.kind {
            Kind::Structure | Kind::Class | Kind::Enum => {
                let full = print_full(node)?;
                if let Some(t) = self.known_type(&full, hint) {
                    return Some(t);
                }
                if inner.kind == Kind::Class {
                    let st = match self.by_path.get(&full) {
                        Some(&addr) => self
                            .struct_type(addr)
                            .or_else(|| tf.get_type_struct(&full).ok())?,
                        None => tf.get_type_struct(&full).ok()?,
                    };
                    return tf.get_type_pointer(self.ptr_size, st, 1).ok();
                }
                let addr = *self.by_path.get(&full)?;
                if inner.kind == Kind::Structure {
                    let t = self.struct_type(addr)?;
                    (t.get_size() as u32 <= hint).then_some(t)
                } else {
                    None
                }
            }
            Kind::BoundGenericEnum => {
                let base = print_full(inner.first()?)?;
                if base != "Swift.Optional" {
                    return None;
                }
                let arg = inner.child(1)?.first()?;
                let t = self.kuna_type(arg, hint)?;
                let nil_is_zero = t.get_metatype() == type_metatype::TYPE_PTR
                    || matches!(t.get_name(), "Swift.String" | "swift.closure")
                    || t.get_name().starts_with("Swift.Array<")
                    || t.get_name().starts_with("Swift.Dictionary<")
                    || t.get_name().starts_with("Swift.Set<");
                (nil_is_zero && t.get_size() as u32 <= hint).then_some(t)
            }
            Kind::BoundGenericStructure => {
                let base = print_full(inner.first()?)?;
                match base.as_str() {
                    "Swift.Array" | "Swift.Dictionary" | "Swift.Set" | "Swift.ContiguousArray" => {
                        let full = print_full(node)?.replace(' ', "");
                        (hint >= 8).then(|| self.storage_struct(&full)).flatten()
                    }
                    "Swift.UnsafePointer"
                    | "Swift.UnsafeMutablePointer"
                    | "Swift.Unmanaged"
                    | "Swift.AutoreleasingUnsafeMutablePointer" => {
                        (hint >= 8).then(|| self.void_ptr()).flatten()
                    }
                    _ => None,
                }
            }
            Kind::BoundGenericClass => {
                let full = print_full(node)?.replace(' ', "");
                let st = tf.get_type_struct(&full).ok()?;
                tf.get_type_pointer(self.ptr_size, st, 1).ok()
            }
            Kind::FunctionType | Kind::NoEscapeFunctionType => {
                (hint >= 16).then(|| self.closure_struct()).flatten()
            }
            Kind::Metatype => (hint >= 8).then(|| self.void_ptr()).flatten(),
            _ => None,
        }
    }

    fn known_type(&self, full: &str, hint: u32) -> Option<Rc<Datatype>> {
        use type_metatype::{TYPE_BOOL, TYPE_FLOAT, TYPE_INT, TYPE_UINT};
        let (size, meta) = match full {
            "Swift.Int" | "Swift.Int64" => (8, TYPE_INT),
            "Swift.UInt" | "Swift.UInt64" => (8, TYPE_UINT),
            "Swift.Int32" => (4, TYPE_INT),
            "Swift.UInt32" => (4, TYPE_UINT),
            "Swift.Int16" => (2, TYPE_INT),
            "Swift.UInt16" => (2, TYPE_UINT),
            "Swift.Int8" => (1, TYPE_INT),
            "Swift.UInt8" => (1, TYPE_UINT),
            "Swift.Double" | "CoreGraphics.CGFloat" => (8, TYPE_FLOAT),
            "Swift.Float" => (4, TYPE_FLOAT),
            "Swift.Bool" => (1, TYPE_BOOL),
            "Swift.String" | "Swift.Character" => {
                return (hint >= 16).then(|| self.string_struct(full)).flatten();
            }
            "Swift.UnsafeRawPointer"
            | "Swift.UnsafeMutableRawPointer"
            | "Swift.OpaquePointer"
            | "Swift.ObjectIdentifier"
            | "Swift.AnyObject" => return (hint >= 8).then(|| self.void_ptr()).flatten(),
            _ => return None,
        };
        if size > hint {
            return None;
        }
        self.base(size, meta)
    }

    /// A sized integer or float; the 1-byte integers by their core names,
    /// which an unnamed match would otherwise shadow.
    fn base(&self, size: u32, meta: type_metatype) -> Option<Rc<Datatype>> {
        match (size, meta) {
            (1, type_metatype::TYPE_INT) => self.tf.get_base_named(1, meta, "int1").ok(),
            (1, type_metatype::TYPE_UINT) => self.tf.get_base_named(1, meta, "uint1").ok(),
            _ => self.tf.get_base(size as i32, meta).ok(),
        }
    }

    fn interned_struct(
        &self,
        name: &str,
        fields: &[(i32, &str, Rc<Datatype>)],
        size: i32,
    ) -> Option<Rc<Datatype>> {
        if let Some(t) = self.tf.find_by_name(name).ok().flatten() {
            return Some(t);
        }
        let shell = self.tf.get_type_struct(name).ok()?;
        let fields: Vec<TypeField> = fields
            .iter()
            .enumerate()
            .map(|(i, (off, n, t))| TypeField::new(i as i32, *off, n.to_string(), Rc::clone(t)))
            .collect();
        self.tf
            .set_fields_struct_raw(&shell, fields, Vec::new(), size, 8, 0)
            .ok()
    }

    /// `Swift.String` / `Swift.Character`: the count-and-flags word and the
    /// string object (a tagged pointer or a small-string payload).
    fn string_struct(&self, name: &str) -> Option<Rc<Datatype>> {
        let word = self.tf.get_base(8, type_metatype::TYPE_UINT).ok()?;
        let ptr = self.void_ptr()?;
        self.interned_struct(
            name,
            &[(0, "_countAndFlagsBits", word), (8, "_object", ptr)],
            16,
        )
    }

    /// A collection's single storage word (`Swift.Array<Swift.String>`).
    fn storage_struct(&self, name: &str) -> Option<Rc<Datatype>> {
        let ptr = self.void_ptr()?;
        self.interned_struct(name, &[(0, "_storage", ptr)], 8)
    }

    /// `swift.closure`: the function and its context.
    fn closure_struct(&self) -> Option<Rc<Datatype>> {
        let ptr = self.void_ptr()?;
        self.interned_struct(
            "swift.closure",
            &[(0, "function", Rc::clone(&ptr)), (8, "context", ptr)],
            16,
        )
    }

    /// `swift.type_metadata`: the kind word and the descriptor of a struct
    /// or enum.
    pub fn metadata_struct(&self) -> Option<Rc<Datatype>> {
        let word = self.tf.get_base(8, type_metatype::TYPE_UINT).ok()?;
        let ptr = self.void_ptr()?;
        self.interned_struct(
            "swift.type_metadata",
            &[(0, "kind", word), (8, "nominal_type_descriptor", ptr)],
            16,
        )
    }

    /// `swift.class_metadata`: `isa`, `superclass`, with Objective-C
    /// interop the cache and `data` words, then the instance extent, the
    /// descriptor and the ivar destroyer.
    pub fn class_metadata_struct(&self) -> Option<Rc<Datatype>> {
        let ptr = self.void_ptr()?;
        let u32t = self.tf.get_base(4, type_metatype::TYPE_UINT).ok()?;
        let u16t = self.tf.get_base(2, type_metatype::TYPE_UINT).ok()?;
        let mut fields: Vec<(i32, &str, Rc<Datatype>)> = vec![
            (0, "isa", Rc::clone(&ptr)),
            (8, "superclass", Rc::clone(&ptr)),
        ];
        if self.objc_interop {
            fields.push((16, "cache0", Rc::clone(&ptr)));
            fields.push((24, "cache1", Rc::clone(&ptr)));
            fields.push((32, "data", Rc::clone(&ptr)));
        }
        let f = self.class_flags_offset() as i32;
        fields.extend([
            (f, "flags", Rc::clone(&u32t)),
            (f + 4, "instanceAddressPoint", Rc::clone(&u32t)),
            (f + 8, "instanceSize", Rc::clone(&u32t)),
            (f + 12, "instanceAlignMask", Rc::clone(&u16t)),
            (f + 14, "reserved", u16t),
            (f + 16, "classSize", Rc::clone(&u32t)),
            (f + 20, "classAddressPoint", u32t),
            (f + 24, "nominal_type_descriptor", Rc::clone(&ptr)),
            (f + 32, "ivarDestroyer", ptr),
        ]);
        self.interned_struct("swift.class_metadata", &fields, f + 40)
    }

    /// `swift.value_witness_table`.
    pub fn witness_table_struct(&self) -> Option<Rc<Datatype>> {
        let ptr = self.void_ptr()?;
        let word = self.tf.get_base(8, type_metatype::TYPE_UINT).ok()?;
        let u32t = self.tf.get_base(4, type_metatype::TYPE_UINT).ok()?;
        let names = [
            "initializeBufferWithCopyOfBuffer",
            "destroy",
            "initializeWithCopy",
            "assignWithCopy",
            "initializeWithTake",
            "assignWithTake",
            "getEnumTagSinglePayload",
            "storeEnumTagSinglePayload",
        ];
        let mut fields: Vec<(i32, &str, Rc<Datatype>)> = names
            .iter()
            .enumerate()
            .map(|(i, n)| (8 * i as i32, *n, Rc::clone(&ptr)))
            .collect();
        fields.push((64, "size", Rc::clone(&word)));
        fields.push((72, "stride", word));
        fields.push((80, "flags", Rc::clone(&u32t)));
        fields.push((84, "extraInhabitantCount", u32t));
        self.interned_struct("swift.value_witness_table", &fields, 88)
    }

    /// Build every static layout, type the metadata records, the accessors
    /// and the getters' `self`.
    pub fn emit(&self, regs: Option<&Registers>, out: &mut AnalysisOutput) -> Stats {
        let mut stats = Stats {
            located: self.metadata.len(),
            ..Stats::default()
        };
        let dump_path = std::env::var("KUNA_SWIFTTYPES_DUMP").ok();
        let mut dump = String::new();
        let mut addrs: Vec<u64> = self.metadata.keys().copied().collect();
        addrs.sort_unstable();
        let metadata_t = self.metadata_struct();
        let class_metadata_t = self.class_metadata_struct();
        let vwt_t = self.witness_table_struct();
        let long = self.tf.get_base(8, type_metatype::TYPE_INT).ok();
        for addr in addrs {
            let Some(ctx) = self.md.contexts.get(&addr) else {
                continue;
            };
            let meta = self.metadata[&addr];
            let Some(path) = self.unique_path(addr) else {
                continue;
            };
            if let Some(t) = self.struct_type(addr) {
                match ctx.kind {
                    ContextKind::Struct => stats.structs += 1,
                    ContextKind::Class => stats.classes += 1,
                    _ => {}
                }
                let mut i = 0;
                if dump_path.is_some() {
                    dump.push_str(&format!("{path} size={}:", t.get_size()));
                }
                while let Some(f) = t.get_field(i) {
                    if dump_path.is_some() {
                        dump.push_str(&format!(
                            " {}@{}:{}",
                            f.name,
                            f.offset,
                            f.field_type.get_name()
                        ));
                    }
                    if f.name != "super" && f.name != "isa" && f.name != "refcounts" {
                        stats.fields += 1;
                        if f.field_type.get_metatype() != type_metatype::TYPE_UNKNOWN
                            && f.field_type.get_metatype() != type_metatype::TYPE_ARRAY
                        {
                            stats.typed_fields += 1;
                        }
                    }
                    i += 1;
                }
                if dump_path.is_some() {
                    dump.push('\n');
                }
            } else if dump_path.is_some() {
                dump.push_str(&format!("{path}: no layout\n"));
            }
            let (record_t, name) = match ctx.kind {
                ContextKind::Class => (class_metadata_t.clone(), "class_metadata"),
                _ => (metadata_t.clone(), "type_metadata"),
            };
            if let Some(t) = record_t.clone() {
                out.swift_types.typed_data.push(TypedDataFact {
                    addr: meta,
                    name: format!("{path}.{name}"),
                    type_: t,
                });
            }
            if ctx.kind != ContextKind::Class {
                if let (Some(w), Some(t)) = (self.local_witness_table(meta), vwt_t.clone()) {
                    out.swift_types.typed_data.push(TypedDataFact {
                        addr: w,
                        name: format!("{path}.value_witness_table"),
                        type_: t,
                    });
                }
            }
            if let (Some(_), Some(t), Some(long)) = (ctx.accessor, record_t, long.clone()) {
                if let Ok(ret) = self.tf.get_type_pointer(self.ptr_size, t, 1) {
                    out.swift_types.prototypes.push(PrototypePieces {
                        name: format!("{path}.type_metadata_accessor"),
                        outtype: Some(ret),
                        intypes: vec![long],
                        innames: vec!["request".to_string()],
                        first_var_arg_slot: -1,
                        output_storage: None,
                        input_storage: Vec::new(),
                        kuna_library_names: None,
                    });
                }
            }
        }
        if let Some(regs) = regs {
            (stats.vtable_getters, stats.witness_getters) = self.lock_getters(regs, out);
        }
        if let Some(path) = dump_path {
            let _ = std::fs::write(path, format!("{stats:?}\n{dump}"));
        }
        stats
    }

    /// `self` of every class vtable getter, and of every local protocol's
    /// witness getter.
    fn lock_getters(&self, regs: &Registers, out: &mut AnalysisOutput) -> (usize, usize) {
        let mut vtable = 0;
        let mut witness = 0;
        let mut contexts: Vec<&Context> = self.md.contexts.values().collect();
        contexts.sort_by_key(|c| c.addr);
        for ctx in contexts {
            if ctx.kind != ContextKind::Class {
                continue;
            }
            let Some(path) = self.unique_path(ctx.addr) else {
                continue;
            };
            let Some(self_t) = self.self_pointer(ctx.addr, &path) else {
                continue;
            };
            for e in &ctx.vtable {
                let Some(f) = e.impl_addr else { continue };
                if e.kind != MethodKind::Getter
                    || !e.instance
                    || !self.namer.is_code(f)
                    || self.namer.ambiguous_self(f)
                {
                    continue;
                }
                out.swift_types.param_locks.push(ParamLockFact {
                    func_addr: f,
                    index: 0,
                    name: "self".to_string(),
                    reg: regs.self_reg,
                    type_: Rc::clone(&self_t),
                });
                vtable += 1;
            }
        }
        let metadata_ptr = self
            .metadata_struct()
            .and_then(|t| self.tf.get_type_pointer(self.ptr_size, t, 1).ok());
        for c in &self.md.conformances {
            let Target::Direct(p) = c.protocol else {
                continue;
            };
            let Some(proto) = self.md.contexts.get(&p) else {
                continue;
            };
            let (Some(w), super::descriptors::TypeRef::Descriptor(Target::Direct(t))) =
                (c.witness_table, c.type_ref)
            else {
                continue;
            };
            let Some(path) = self.unique_path(t) else {
                continue;
            };
            let Some(self_t) = self.self_pointer(t, &path) else {
                continue;
            };
            if !c.witnesses.is_empty() || c.flags & (1 << 17) != 0 {
                continue;
            }
            for (k, (req, _)) in proto.requirements.iter().enumerate() {
                let flags = self.md.requirement_flags.get(req).copied().unwrap_or(0xff);
                if flags & 0xf != 3 || flags & 0x10 == 0 {
                    continue;
                }
                let slot = w + self.md.ptr_size * (1 + k as u64);
                let Some(&f) = self.md.words.get(&slot) else {
                    continue;
                };
                if !self.namer.is_code(f) || self.namer.ambiguous_self(f) {
                    continue;
                }
                let (Some(mp), Some(vp)) = (metadata_ptr.clone(), self.void_ptr()) else {
                    continue;
                };
                for fact in [
                    ParamLockFact {
                        func_addr: f,
                        index: 0,
                        name: "Self".to_string(),
                        reg: regs.args[0],
                        type_: mp,
                    },
                    ParamLockFact {
                        func_addr: f,
                        index: 1,
                        name: "witness_table".to_string(),
                        reg: regs.args[1],
                        type_: vp,
                    },
                    ParamLockFact {
                        func_addr: f,
                        index: 2,
                        name: "self".to_string(),
                        reg: regs.self_reg,
                        type_: Rc::clone(&self_t),
                    },
                ] {
                    out.swift_types.param_locks.push(fact);
                }
                witness += 1;
            }
        }
        (vtable, witness)
    }

    /// `Type *`: the pointer a class or struct `self` is passed as.
    fn self_pointer(&self, addr: u64, path: &str) -> Option<Rc<Datatype>> {
        let st = self
            .struct_type(addr)
            .or_else(|| self.tf.find_by_name(path).ok().flatten())
            .or_else(|| self.tf.get_type_struct(path).ok())?;
        if st.get_metatype() != type_metatype::TYPE_STRUCT {
            return None;
        }
        self.tf.get_type_pointer(self.ptr_size, st, 1).ok()
    }
}

/// The type (or module) node an imported descriptor symbol names
/// (`$s7SwiftUI4ViewPMp`, `$s10Foundation4DateVMn`, `$s7SwiftUIMXM`).
fn imported_node(import: &str) -> Option<NodeRef> {
    let tree =
        demangle_swift_tree(import).or_else(|| demangle_swift_tree(&format!("_{import}")))?;
    let top = tree.first()?;
    match top.kind {
        Kind::NominalTypeDescriptor | Kind::ProtocolDescriptor => Some(Rc::clone(top.first()?)),
        Kind::ModuleDescriptor => Some(Rc::clone(top.first()?)),
        _ => None,
    }
}
