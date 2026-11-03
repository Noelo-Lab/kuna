//! (kuna `swiftmetadata`) The Swift 5 runtime metadata records, read from
//! the reflection sections (`__swift5_types` / `swift5_type_metadata`,
//! `__swift5_protos`, `__swift5_proto`, `__swift5_fieldmd`) per the ABI
//! (`swift/include/swift/ABI/Metadata.h`, `MetadataValues.h`).
//!
//! Everything in these sections is addressed through 32-bit self-relative
//! pointers, which need no fixup resolution; an *indirectable* pointer
//! (low bit set) names a pointer-sized slot instead, which is a rebase
//! (resolved through the chained-fixup overlay) or a bind to another image
//! (named through the import table). A malformed record is skipped; a
//! section that is absent yields nothing.

use std::collections::HashMap;

use object::read::{Object, ObjectSection};

use crate::objc::sections::MachoImage;

/// `ContextDescriptorKind` (the low 5 bits of the flags word).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextKind {
    Module,
    Extension,
    Anonymous,
    Protocol,
    OpaqueType,
    Class,
    Struct,
    Enum,
    Other(u8),
}

impl ContextKind {
    fn from_flags(flags: u32) -> ContextKind {
        match flags & 0x1f {
            0 => ContextKind::Module,
            1 => ContextKind::Extension,
            2 => ContextKind::Anonymous,
            3 => ContextKind::Protocol,
            4 => ContextKind::OpaqueType,
            16 => ContextKind::Class,
            17 => ContextKind::Struct,
            18 => ContextKind::Enum,
            k => ContextKind::Other(k as u8),
        }
    }

    pub fn is_nominal(self) -> bool {
        matches!(
            self,
            ContextKind::Class | ContextKind::Struct | ContextKind::Enum
        )
    }
}

/// `MethodDescriptorFlags::Kind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MethodKind {
    Method,
    Init,
    Getter,
    Setter,
    Modify,
    Read,
    Other(u8),
}

impl MethodKind {
    fn from_flags(flags: u32) -> MethodKind {
        match flags & 0xf {
            0 => MethodKind::Method,
            1 => MethodKind::Init,
            2 => MethodKind::Getter,
            3 => MethodKind::Setter,
            4 => MethodKind::Modify,
            5 => MethodKind::Read,
            k => MethodKind::Other(k as u8),
        }
    }

    pub fn tag(self) -> &'static str {
        match self {
            MethodKind::Method => "method",
            MethodKind::Init => "init",
            MethodKind::Getter => "getter",
            MethodKind::Setter => "setter",
            MethodKind::Modify => "modify",
            MethodKind::Read => "read",
            MethodKind::Other(_) => "entry",
        }
    }
}

/// One class vtable slot: its position, kind and implementation (absent for
/// a method the optimizer removed).
#[derive(Clone, Debug)]
pub struct VTableEntry {
    pub index: usize,
    pub kind: MethodKind,
    pub addr: u64,
    pub impl_addr: Option<u64>,
}

/// One override-table row: the overridden method descriptor (a local
/// vtable slot or an imported `…Tq` slot) and the overriding implementation.
#[derive(Clone, Debug)]
pub struct OverrideEntry {
    pub index: usize,
    pub base_class: Target,
    pub method: Target,
    pub impl_addr: Option<u64>,
}

/// Where an indirectable pointer lands: a record in this image, or a
/// pointer slot the loader fills (an import of another image, or a rebase
/// the fixup overlay could not resolve).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Direct(u64),
    Slot(u64),
}

/// A context descriptor: module, nominal type, protocol, extension or
/// anonymous context.
#[derive(Clone, Debug)]
pub struct Context {
    pub addr: u64,
    pub kind: ContextKind,
    pub flags: u32,
    pub parent: Option<Target>,
    /// The module name, the nominal/protocol name, or an extension's
    /// mangled extended-context type name.
    pub name: Option<String>,
    pub accessor: Option<u64>,
    pub fields: Option<u64>,
    pub vtable: Vec<VTableEntry>,
    pub overrides: Vec<OverrideEntry>,
    /// A protocol's requirement rows: `(descriptor address, default
    /// implementation)`.
    pub requirements: Vec<(u64, Option<u64>)>,
}

/// One `__swift5_proto` conformance descriptor.
#[derive(Clone, Debug)]
pub struct Conformance {
    pub addr: u64,
    pub protocol: Target,
    pub type_ref: TypeRef,
    pub witness_table: Option<u64>,
    pub flags: u32,
    /// Resilient witnesses: `(requirement descriptor, implementation)`.
    pub witnesses: Vec<(Target, Option<u64>)>,
}

/// `TypeReferenceKind` with its payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeRef {
    Descriptor(Target),
    ObjCClassName(u64),
    ObjCClass(u64),
}

/// A field descriptor (`__swift5_fieldmd`): the type's mangled name, its
/// superclass's, and the field records.
#[derive(Clone, Debug)]
pub struct FieldDescriptor {
    pub addr: u64,
    pub mangled_type_name: Option<u64>,
    pub superclass: Option<u64>,
    pub kind: u16,
    pub records: Vec<FieldRecord>,
}

#[derive(Clone, Debug)]
pub struct FieldRecord {
    pub flags: u32,
    pub mangled_type_name: Option<u64>,
    pub name: Option<String>,
}

/// The per-format spellings of the reflection sections.
pub struct SectionNames {
    pub types: &'static str,
    pub protocols: &'static str,
    pub conformances: &'static str,
    pub fieldmd: &'static str,
}

pub const MACHO_SECTIONS: SectionNames = SectionNames {
    types: "__swift5_types",
    protocols: "__swift5_protos",
    conformances: "__swift5_proto",
    fieldmd: "__swift5_fieldmd",
};

pub const ELF_SECTIONS: SectionNames = SectionNames {
    types: "swift5_type_metadata",
    protocols: "swift5_protocols",
    conformances: "swift5_protocol_conformances",
    fieldmd: "swift5_fieldmd",
};

/// The parsed metadata of one image.
#[derive(Default, Debug)]
pub struct Metadata {
    pub contexts: HashMap<u64, Context>,
    /// The `__swift5_types` entries in section order.
    pub types: Vec<u64>,
    pub protocols: Vec<u64>,
    pub conformances: Vec<Conformance>,
    pub fields: Vec<FieldDescriptor>,
    /// Requirement descriptor address → its `ProtocolRequirementFlags`.
    pub requirement_flags: HashMap<u64, u32>,
    /// Pointer words read from local witness tables, by slot.
    pub words: HashMap<u64, u64>,
    pub ptr_size: u64,
}

impl Metadata {
    /// `ProtocolRequirementFlags::Kind` (0 base protocol, 1 method, 2 init,
    /// 3 getter, 4 setter, 5 read, 6 modify, 7 associated type, 8 associated
    /// conformance), or 0xff when unknown.
    pub fn requirement_kind(&self, requirement: u64) -> u32 {
        self.requirement_flags
            .get(&requirement)
            .map_or(0xff, |f| f & 0xf)
    }
}

const MAX_VTABLE: u32 = 1 << 14;
const MAX_REQUIREMENTS: u32 = 1 << 12;
const MAX_FIELDS: u32 = 1 << 12;

pub struct Reader<'a> {
    pub img: &'a MachoImage<'a>,
    /// Pointer slots the loader binds to another image.
    pub bind_slots: &'a std::collections::HashSet<u64>,
}

impl<'a> Reader<'a> {
    /// A self-relative `i32` pointer at `at`; `None` when null.
    fn rel(&self, at: u64) -> Option<u64> {
        let off = self.img.read_i32(at)?;
        if off == 0 {
            return None;
        }
        Some(at.wrapping_add(off as i64 as u64))
    }

    /// A self-relative pointer whose low bit selects indirection through a
    /// pointer-sized slot.
    fn rel_indirectable(&self, at: u64) -> Option<Target> {
        let off = self.img.read_i32(at)?;
        if off == 0 {
            return None;
        }
        let target = at.wrapping_add((off & !1) as i64 as u64);
        if off & 1 == 0 {
            return Some(Target::Direct(target));
        }
        if self.bind_slots.contains(&target) {
            return Some(Target::Slot(target));
        }
        match self.img.read_ptr(target) {
            Some(p) if p != 0 && self.img.read(p, 4).is_some() => Some(Target::Direct(p)),
            _ => Some(Target::Slot(target)),
        }
    }

    fn cstr(&self, at: u64) -> Option<String> {
        let s = self.img.read_cstr(at)?;
        if s.is_empty() {
            return None;
        }
        Some(s)
    }

    /// Every 4-byte self-relative entry of the section named `name`.
    fn section_entries(&self, file: &object::File, name: &str) -> Vec<u64> {
        let Some(sec) = file.section_by_name(name) else {
            return Vec::new();
        };
        let addr = sec.address();
        let Ok(data) = sec.data() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for i in 0..data.len() / 4 {
            let slot = addr.wrapping_add((i * 4) as u64);
            let Some(off) = self.img.read_i32(slot) else {
                continue;
            };
            if off == 0 {
                continue;
            }
            out.push(slot.wrapping_add(off as i64 as u64));
        }
        out
    }

    pub fn read_all(&self, file: &object::File, names: &SectionNames) -> Metadata {
        let mut md = Metadata {
            ptr_size: self.img.ptr_size(),
            ..Metadata::default()
        };
        md.types = self.section_entries(file, names.types);
        md.protocols = self.section_entries(file, names.protocols);
        for &addr in md.types.iter().chain(md.protocols.iter()) {
            self.context_closure(addr, &mut md.contexts, 0);
        }
        for addr in self.section_entries(file, names.conformances) {
            if let Some(c) = self.conformance(addr) {
                if let TypeRef::Descriptor(Target::Direct(t)) = c.type_ref {
                    self.context_closure(t, &mut md.contexts, 0);
                }
                if let Target::Direct(p) = c.protocol {
                    self.context_closure(p, &mut md.contexts, 0);
                    if let (Some(w), Some(proto)) = (c.witness_table, md.contexts.get(&p)) {
                        for k in 0..=proto.requirements.len() as u64 {
                            let slot = w + md.ptr_size * k;
                            if let Some(v) = self.img.read_ptr(slot) {
                                md.words.insert(slot, v);
                            }
                        }
                    }
                }
                md.conformances.push(c);
            }
        }
        for ctx in md.contexts.values() {
            for (row, _) in &ctx.requirements {
                if let Some(f) = self.img.read_u32(*row) {
                    md.requirement_flags.insert(*row, f);
                }
            }
        }
        if let Some(sec) = file.section_by_name(names.fieldmd) {
            let base = sec.address();
            let size = sec.size();
            let mut at = base;
            while at < base.saturating_add(size) {
                let Some(fd) = self.field_descriptor(at) else {
                    break;
                };
                let len = 16u64 + fd.records.len() as u64 * 12;
                md.fields.push(fd);
                at = at.saturating_add(len);
            }
        }
        md
    }

    /// Read the descriptor at `addr` and its parent chain into `out`.
    fn context_closure(&self, addr: u64, out: &mut HashMap<u64, Context>, depth: usize) {
        if depth > 32 || out.contains_key(&addr) {
            return;
        }
        let Some(ctx) = self.context(addr) else {
            return;
        };
        let parent = ctx.parent;
        out.insert(addr, ctx);
        if let Some(Target::Direct(p)) = parent {
            self.context_closure(p, out, depth + 1);
        }
    }

    fn context(&self, addr: u64) -> Option<Context> {
        let flags = self.img.read_u32(addr)?;
        let kind = ContextKind::from_flags(flags);
        let parent = self.rel_indirectable(addr + 4);
        let mut ctx = Context {
            addr,
            kind,
            flags,
            parent,
            name: None,
            accessor: None,
            fields: None,
            vtable: Vec::new(),
            overrides: Vec::new(),
            requirements: Vec::new(),
        };
        match kind {
            ContextKind::Module => {
                ctx.name = self.rel(addr + 8).and_then(|p| self.cstr(p));
            }
            ContextKind::Extension => {
                ctx.name = self.rel(addr + 8).and_then(|p| self.mangled_name(p));
            }
            ContextKind::Protocol => {
                ctx.name = self.rel(addr + 8).and_then(|p| self.cstr(p));
                let num_sig = self.img.read_u32(addr + 12)?;
                let num_req = self.img.read_u32(addr + 16)?;
                if num_req > MAX_REQUIREMENTS {
                    return None;
                }
                // Trailing: one 12-byte GenericRequirementDescriptor per
                // signature requirement, then the requirement rows.
                let mut at = addr + 24 + num_sig as u64 * 12;
                for _ in 0..num_req {
                    let default_impl = self.rel(at + 4);
                    ctx.requirements.push((at, default_impl));
                    at += 8;
                }
                // Default implementations of base-protocol / associated-type
                // rows are not functions; the row flags tell the kind.
                for (row, d) in ctx.requirements.iter_mut() {
                    let kind = self.img.read_u32(*row).map_or(0xff, |f| f & 0xf);
                    if !matches!(kind, 1..=6 | 8) {
                        *d = None;
                    }
                }
            }
            ContextKind::Class | ContextKind::Struct | ContextKind::Enum => {
                ctx.name = self.rel(addr + 8).and_then(|p| self.cstr(p));
                ctx.accessor = self.rel(addr + 12);
                ctx.fields = self.rel(addr + 16);
                if kind == ContextKind::Class {
                    self.class_tables(&mut ctx)?;
                }
            }
            _ => {}
        }
        Some(ctx)
    }

    /// The vtable and override table that trail a class descriptor.
    fn class_tables(&self, ctx: &mut Context) -> Option<()> {
        let flags = ctx.flags;
        let specific = flags >> 16;
        let mut at = ctx.addr + 44;
        if flags & 0x80 != 0 {
            // TypeGenericContextDescriptorHeader: two instantiation words,
            // then {numParams, numRequirements, numKeyArguments, flags} as
            // u16s, the parameter bytes (padded to 4), the requirements
            // (12 bytes each), then pack shapes and values when flagged.
            let counts = self.img.read_u32(at + 8)?;
            let (num_params, num_reqs) = (counts & 0xffff, counts >> 16);
            let header_flags = self.img.read_u32(at + 12)? >> 16;
            at += 16 + ((num_params as u64 + 3) & !3) + num_reqs as u64 * 12;
            if header_flags & 0x2 != 0 {
                // Conditional inverted protocols: a layout this reader
                // does not follow.
                return None;
            }
            if header_flags & 0x1 != 0 {
                let num_packs = self.img.read_u32(at)? & 0xffff;
                at += 4 + num_packs as u64 * 8;
            }
            if header_flags & 0x4 != 0 {
                let num_values = self.img.read_u32(at)?;
                at += 4 + num_values as u64 * 8;
            }
        }
        if specific & (1 << 13) != 0 {
            at += 4; // ResilientSuperclass
        }
        match specific & 3 {
            1 => at += 12, // SingletonMetadataInitialization
            2 => at += 4,  // ForeignMetadataInitialization
            _ => {}
        }
        if specific & (1 << 15) != 0 {
            let _vtable_offset = self.img.read_u32(at)?;
            let size = self.img.read_u32(at + 4)?;
            if size > MAX_VTABLE {
                return None;
            }
            at += 8;
            for index in 0..size as usize {
                let mflags = self.img.read_u32(at)?;
                let impl_addr = self.rel(at + 4);
                ctx.vtable.push(VTableEntry {
                    index,
                    kind: MethodKind::from_flags(mflags),
                    addr: at,
                    impl_addr,
                });
                at += 8;
            }
        }
        if specific & (1 << 14) != 0 {
            let count = self.img.read_u32(at)?;
            if count > MAX_VTABLE {
                return None;
            }
            at += 4;
            for index in 0..count as usize {
                let base_class = self.rel_indirectable(at)?;
                let method = self.rel_indirectable(at + 4)?;
                let impl_addr = self.rel(at + 8);
                ctx.overrides.push(OverrideEntry {
                    index,
                    base_class,
                    method,
                    impl_addr,
                });
                at += 12;
            }
        }
        Some(())
    }

    fn conformance(&self, addr: u64) -> Option<Conformance> {
        let protocol = self.rel_indirectable(addr)?;
        let flags = self.img.read_u32(addr + 12)?;
        let type_ref = match (flags >> 3) & 3 {
            0 => TypeRef::Descriptor(self.rel_indirectable(addr + 4)?),
            1 => TypeRef::Descriptor(self.rel_indirectable_slot(addr + 4)?),
            2 => TypeRef::ObjCClassName(self.rel(addr + 4)?),
            _ => TypeRef::ObjCClass(self.rel(addr + 4)?),
        };
        let witness_table = self.rel(addr + 8);
        let mut c = Conformance {
            addr,
            protocol,
            type_ref,
            witness_table,
            flags,
            witnesses: Vec::new(),
        };
        // Trailing objects: retroactive context, conditional requirements,
        // conditional pack descriptors, resilient witnesses header + rows.
        let mut at = addr + 16;
        if flags & (1 << 6) != 0 {
            at += 4;
        }
        let num_cond = (flags >> 8) & 0xff;
        at += num_cond as u64 * 12;
        let num_packs = (flags >> 18) & 3;
        at += num_packs as u64 * 4;
        if flags & (1 << 16) != 0 {
            let count = self.img.read_u32(at)?;
            if count > MAX_REQUIREMENTS {
                return None;
            }
            at += 4;
            for _ in 0..count {
                let requirement = self.rel_indirectable(at)?;
                let impl_addr = self.rel(at + 4);
                c.witnesses.push((requirement, impl_addr));
                at += 8;
            }
        }
        Some(c)
    }

    /// An always-indirect relative pointer (the `IndirectTypeDescriptor`
    /// reference kind).
    fn rel_indirectable_slot(&self, at: u64) -> Option<Target> {
        let slot = self.rel(at)?;
        if self.bind_slots.contains(&slot) {
            return Some(Target::Slot(slot));
        }
        match self.img.read_ptr(slot) {
            Some(p) if p != 0 && self.img.read(p, 4).is_some() => Some(Target::Direct(p)),
            _ => Some(Target::Slot(slot)),
        }
    }

    fn field_descriptor(&self, addr: u64) -> Option<FieldDescriptor> {
        let mangled_type_name = self.rel(addr);
        let superclass = self.rel(addr + 4);
        let kind = (self.img.read_u32(addr + 8)? & 0xffff) as u16;
        let record_size = self.img.read_u32(addr + 8)? >> 16;
        let num = self.img.read_u32(addr + 12)?;
        if record_size != 12 || num > MAX_FIELDS {
            return None;
        }
        let mut records = Vec::new();
        let mut at = addr + 16;
        for _ in 0..num {
            let flags = self.img.read_u32(at)?;
            let mangled = self.rel(at + 4);
            let name = self.rel(at + 8).and_then(|p| self.cstr(p));
            records.push(FieldRecord {
                flags,
                mangled_type_name: mangled,
                name,
            });
            at += 12;
        }
        Some(FieldDescriptor {
            addr,
            mangled_type_name,
            superclass,
            kind,
            records,
        })
    }

    /// A mangled type name at `at`: the bytes up to the NUL, with symbolic
    /// references (`\x01`–`\x17` + 4 bytes, `\x18`–`\x1f` + 8 bytes) kept
    /// intact. Returned as the raw bytes re-encoded for the demangler:
    /// `None` when a symbolic reference is present (not demangled yet).
    pub fn mangled_name(&self, at: u64) -> Option<String> {
        let mut s = String::new();
        let mut p = at;
        loop {
            let b = *self.img.read(p, 1)?.first()?;
            if b == 0 {
                break;
            }
            if b < 0x20 {
                return None;
            }
            if s.len() > 4096 {
                return None;
            }
            s.push(b as char);
            p += 1;
        }
        Some(s)
    }
}
