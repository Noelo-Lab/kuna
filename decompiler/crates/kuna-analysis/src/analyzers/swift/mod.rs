//! P1 Swift runtime-metadata recovery (`SwiftMetadataPass`, option
//! `swiftmetadata`).
//!
//! A stripped Swift image keeps its reflection sections, and they name a
//! great deal of its code: every nominal type's descriptor carries the
//! type's name, its metadata accessor function and (for a class) its vtable
//! and override table; every protocol conformance carries its witness
//! table and, for a resilient protocol, each witness implementation next to
//! the requirement it satisfies. The pass reads them
//! (`descriptors.rs`) and names what they point at, through the demangler's
//! name-only conventions so a name here reads like a symbol-table name would:
//!
//! - `Module.Type.type_metadata_accessor` — the metadata accessor;
//! - `Module.Class.vtable#3.getter` — a vtable slot's implementation, by
//!   position and kind (the slot carries no selector);
//! - `Module.Sub.viewDidLoad()` — an override whose overridden method
//!   descriptor is an import (`…Tq`), so the member name is known;
//! - `<Module.Type:Swift.Hashable>.hash(into:)` — a resilient witness whose
//!   requirement descriptor is an import; `<T:P>.requirement#2` for a local
//!   protocol, `Module.P.requirement#2.default` for a default implementation;
//! - the records themselves: `….nominal_type_descriptor`,
//!   `….protocol_descriptor`, `<T:P>.protocol_conformance_descriptor`,
//!   `<T:P>.protocol_witness_table`, `….reflection_field_descriptor`.
//!
//! Function names use the FID-precedent label-gated rename: a real symbol
//! is never overwritten, only an engine placeholder. Registered for Mach-O
//! and ELF (the two formats Swift ships in); an image without the sections
//! produces nothing, so every other binary is byte-identical.

pub mod descriptors;
pub mod kuna_swifttypes;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use descriptors::{
    Conformance,
    Context,
    ContextKind,
    ELF_SECTIONS,
    MACHO_SECTIONS,
    Metadata,
    Reader,
    Target,
    TypeRef,
};
use object::read::{Object, ObjectSection, ObjectSymbol, ObjectSymbolTable};

use crate::demangle::kuna_swift::demangle_swift_name;
use crate::objc::sections::MachoImage;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, FidMatch, Phase, SymFact, SymKind};

#[derive(Default)]
pub struct SwiftMetadataPass;

impl AnalysisPass for SwiftMetadataPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "swiftmetadata"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let names = match ctx.file.format() {
            object::BinaryFormat::MachO => &MACHO_SECTIONS,
            object::BinaryFormat::Elf => &ELF_SECTIONS,
            _ => return out,
        };
        if [names.types, names.protocols, names.conformances]
            .iter()
            .all(|s| ctx.file.section_by_name(s).is_none())
        {
            return out;
        }
        let mut fixups = crate::loader::format::macho::resolve_chained_fixups(ctx.file, ctx.bytes);
        if ctx.file.format() == object::BinaryFormat::Elf {
            fixups = crate::loader::format::macho::ChainedFixups::from_rebases(
                elf_relative_relocs(ctx.file),
            );
        }
        let mut imports: HashMap<u64, String> =
            crate::loader::format::resolve_imports(ctx.file, ctx.bytes)
                .into_iter()
                .filter_map(|s| String::from_utf8(s.name).ok().map(|n| (s.addr, n)))
                .collect();
        // A chained-fixup Mach-O binds other images' descriptors into `__const`
        // slots the indirect-symbol table does not list; an ELF binds them
        // through GOT slots whose dynamic relocations name the symbol.
        for (slot, bind) in fixups.binds() {
            if bind.name.is_empty() {
                continue;
            }
            if let Ok(name) = std::str::from_utf8(&bind.name) {
                imports.entry(slot).or_insert_with(|| name.to_string());
            }
        }
        let mut bind_slots: std::collections::HashSet<u64> =
            fixups.binds().map(|(slot, _)| slot).collect();
        for (addr, reloc) in ctx.file.dynamic_relocations().into_iter().flatten() {
            if let object::RelocationTarget::Symbol(idx) = reloc.target() {
                let sym = ctx
                    .file
                    .dynamic_symbol_table()
                    .and_then(|t| t.symbol_by_index(idx).ok());
                if let Some(name) = sym.and_then(|s| s.name().ok()) {
                    imports.entry(addr).or_insert_with(|| name.to_string());
                    bind_slots.insert(addr);
                }
            }
        }
        let img = MachoImage::with_fixups(ctx.file, fixups.clone());
        let reader = Reader {
            img: &img,
            bind_slots: &bind_slots,
        };
        let md = reader.read_all(ctx.file, names, ctx.bytes.len());
        // Only a code address may be named as a function: a base-protocol
        // or associated-type witness row points at data.
        let code: Vec<(u64, u64)> = ctx
            .file
            .sections()
            .filter(|s| s.kind() == object::SectionKind::Text)
            .map(|s| (s.address(), s.address().saturating_add(s.size())))
            .collect();
        let namer = Namer {
            md: &md,
            imports: &imports,
            memo: Default::default(),
            code,
            claims: Default::default(),
            budget: std::cell::Cell::new(NAME_BYTES.saturating_add(ctx.bytes.len())),
        };
        namer.emit(&mut out);
        if ctx.arch.analysis_swifttypes {
            let mut types = kuna_swifttypes::Types::new(
                &md,
                &reader,
                &imports,
                &namer,
                ctx.arch.types(),
                ctx.file.architecture(),
                ctx.file.format() == object::BinaryFormat::MachO,
            );
            types.locate_all();
            let regs = kuna_swifttypes::Registers::for_arch(ctx.file.architecture());
            types.emit(regs.as_ref(), &mut out);
        }
        out
    }
}

/// An ELF's relocated pointer slots (`R_*_RELATIVE`, the addend the loader
/// stores), so the reader resolves a pointer word the same way it does a
/// Mach-O chained rebase.
fn elf_relative_relocs(file: &object::File) -> HashMap<u64, u64> {
    let relative = match file.architecture() {
        object::Architecture::X86_64 => object::elf::R_X86_64_RELATIVE,
        object::Architecture::Aarch64 => object::elf::R_AARCH64_RELATIVE,
        _ => return HashMap::new(),
    };
    let mut out = HashMap::new();
    for (addr, reloc) in file.dynamic_relocations().into_iter().flatten() {
        if let object::RelocationFlags::Elf { r_type } = reloc.flags() {
            if r_type == relative && !reloc.has_implicit_addend() {
                out.insert(addr, reloc.addend() as u64);
            }
        }
    }
    out
}

/// The name-only rendering of an import slot's symbol, with the trailing
/// `.tag` (a descriptor kind) removed when it is the expected one.
fn import_name(imports: &HashMap<u64, String>, slot: u64, tag: &str) -> Option<String> {
    let raw = imports.get(&slot)?;
    let name = demangle_swift_name(raw).or_else(|| demangle_swift_name(&format!("_{raw}")))?;
    let stripped = name.strip_suffix(tag)?.strip_suffix('.')?;
    Some(stripped.to_string())
}

/// `Proto.member` → `member`, given the method descriptor's name-only
/// form `Module.Proto.member`.
fn member_after_protocol(name: &str) -> Option<String> {
    // The protocol path is everything up to the member; members start
    // after the last `.` that is not inside parentheses and not part of
    // a `.getter`/`.setter`-style accessor tag.
    let bytes = name.as_bytes();
    let mut depth = 0i32;
    let mut cut = None;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' | b'<' | b'[' => depth += 1,
            b')' | b'>' | b']' => depth -= 1,
            b'.' if depth == 0 => cut = Some(i),
            _ => {}
        }
    }
    let mut member = &name[cut? + 1..];
    // An accessor tag belongs to the member before it.
    if matches!(
        member,
        "getter" | "setter" | "modify" | "read" | "unsafeMutableAddressor" | "unsafeAddressor"
    ) {
        let head = &name[..cut?];
        let prev = head.rfind('.')?;
        member = &name[prev + 1..];
    }
    if member.is_empty() {
        return None;
    }
    Some(member.to_string())
}

/// The bytes of names the pass builds beyond the image's own length. A
/// real image's names come to a fraction of its size; the budget stops a
/// crafted one whose long paths repeat across many records.
const NAME_BYTES: usize = 16 << 20;

/// One metadata record's claim on a function: the name it would give it,
/// the owning type and, for a witness, the protocol member it implements.
#[derive(Clone, Debug)]
pub struct Claim {
    pub name: String,
    pub owner: String,
    pub requirement: Option<(String, String)>,
    pub getter: bool,
}

pub struct Namer<'a> {
    md: &'a Metadata,
    imports: &'a HashMap<u64, String>,
    memo: std::cell::RefCell<HashMap<u64, Option<String>>>,
    code: Vec<(u64, u64)>,
    /// Every claim on each function address, in record order.
    claims: std::cell::RefCell<std::collections::BTreeMap<u64, Vec<Claim>>>,
    /// Name bytes left to build ([`NAME_BYTES`] plus the image length).
    budget: std::cell::Cell<usize>,
}

impl Namer<'_> {
    /// The dotted path of the context at `addr` (`Module.Outer.Inner`), or
    /// `None` when a link of the chain cannot be named.
    pub fn path(&self, addr: u64, depth: usize) -> Option<String> {
        if depth > 32 {
            return None;
        }
        if let Some(cached) = self.memo.borrow().get(&addr) {
            return cached.clone();
        }
        let ctx = self.md.contexts.get(&addr);
        let result = ctx
            .and_then(|c| self.path_of(c, depth))
            .filter(|p| self.spend(p));
        self.memo.borrow_mut().insert(addr, result.clone());
        result
    }

    fn path_of(&self, ctx: &Context, depth: usize) -> Option<String> {
        match ctx.kind {
            ContextKind::Module => ctx.name.clone(),
            ContextKind::Anonymous => self.parent_path(ctx, depth),
            ContextKind::Extension => {
                let mangled = ctx.name.as_ref()?;
                demangle_swift_name(&format!("$s{mangled}"))
            }
            ContextKind::Class
            | ContextKind::Struct
            | ContextKind::Enum
            | ContextKind::Protocol => {
                let name = ctx.name.as_ref()?;
                if name.is_empty() {
                    return None;
                }
                let parent = self.parent_path(ctx, depth)?;
                Some(format!("{parent}.{name}"))
            }
            _ => None,
        }
    }

    fn parent_path(&self, ctx: &Context, depth: usize) -> Option<String> {
        match ctx.parent? {
            Target::Direct(p) => self.path(p, depth + 1),
            Target::Slot(slot) => import_name(self.imports, slot, "module_descriptor")
                .or_else(|| import_name(self.imports, slot, "nominal_type_descriptor")),
        }
    }

    fn target_path(&self, t: Target, tag: &str) -> Option<String> {
        match t {
            Target::Direct(p) => self.path(p, 0),
            Target::Slot(slot) => import_name(self.imports, slot, tag),
        }
    }

    /// The conforming type's path and the protocol's path.
    fn conformance_parts(&self, c: &Conformance) -> Option<(String, String)> {
        let ty = match c.type_ref {
            TypeRef::Descriptor(t) => self.target_path(t, "nominal_type_descriptor")?,
            TypeRef::ObjCClassName(_) | TypeRef::ObjCClass(_) => return None,
        };
        let proto = self.target_path(c.protocol, "protocol_descriptor")?;
        Some((ty, proto))
    }

    pub fn is_code(&self, addr: u64) -> bool {
        self.code.iter().any(|&(lo, hi)| addr >= lo && addr < hi)
    }

    /// Charge `name` to the budget; `false` (and nothing more is named)
    /// once it is spent.
    fn spend(&self, name: &str) -> bool {
        match self.budget.get().checked_sub(name.len()) {
            Some(left) => {
                self.budget.set(left);
                true
            }
            None => {
                self.budget.set(0);
                false
            }
        }
    }

    /// Record a metadata record's claim on the code at `addr`.
    fn claim(
        &self,
        addr: u64,
        name: String,
        owner: &str,
        requirement: Option<(String, String)>,
        getter: bool,
    ) {
        if !self.is_code(addr)
            || !self.spend(&name)
            || !self.spend(owner)
            || requirement
                .as_ref()
                .is_some_and(|(p, m)| !self.spend(p) || !self.spend(m))
        {
            return;
        }
        self.claims
            .borrow_mut()
            .entry(addr)
            .or_default()
            .push(Claim {
                name,
                owner: owner.to_string(),
                requirement,
                getter,
            });
    }

    /// Whether the function at `addr` cannot take one type's `self`: the
    /// linker folded it with code of another type, or with a member that is
    /// not a getter.
    pub fn ambiguous_self(&self, addr: u64) -> bool {
        let claims = self.claims.borrow();
        let Some(claims) = claims.get(&addr) else {
            return false;
        };
        let owner = &claims[0].owner;
        claims.iter().any(|c| &c.owner != owner || !c.getter)
    }

    fn emit(&self, out: &mut AnalysisOutput) {
        let mut data: Vec<SymFact> = Vec::new();
        let mut contexts: Vec<&Context> = self.md.contexts.values().collect();
        contexts.sort_by_key(|c| c.addr);
        for ctx in &contexts {
            let Some(path) = self.path(ctx.addr, 0) else {
                continue;
            };
            match ctx.kind {
                ContextKind::Module => data.push(SymFact {
                    addr: ctx.addr,
                    name: format!("{path}.module_descriptor"),
                    kind: SymKind::Data,
                }),
                ContextKind::Protocol => {
                    data.push(SymFact {
                        addr: ctx.addr,
                        name: format!("{path}.protocol_descriptor"),
                        kind: SymKind::Data,
                    });
                    for (k, (_, default_impl)) in ctx.requirements.iter().enumerate() {
                        if let Some(f) = default_impl {
                            self.claim(
                                *f,
                                format!("{path}.requirement#{k}.default"),
                                &path,
                                None,
                                false,
                            );
                        }
                    }
                }
                ContextKind::Class | ContextKind::Struct | ContextKind::Enum => {
                    data.push(SymFact {
                        addr: ctx.addr,
                        name: format!("{path}.nominal_type_descriptor"),
                        kind: SymKind::Data,
                    });
                    if let Some(a) = ctx.accessor {
                        self.claim(
                            a,
                            format!("{path}.type_metadata_accessor"),
                            &path,
                            None,
                            false,
                        );
                    }
                    if let Some(f) = ctx.fields {
                        data.push(SymFact {
                            addr: f,
                            name: format!("{path}.reflection_field_descriptor"),
                            kind: SymKind::Data,
                        });
                    }
                    for e in &ctx.vtable {
                        if let Some(f) = e.impl_addr {
                            let getter = e.kind == descriptors::MethodKind::Getter && e.instance;
                            self.claim(
                                f,
                                format!("{path}.vtable#{}.{}", e.index, e.kind.tag()),
                                &path,
                                None,
                                getter,
                            );
                        }
                    }
                    for o in &ctx.overrides {
                        let Some(f) = o.impl_addr else { continue };
                        if let Some(name) = self.override_name(&path, o) {
                            let getter = name.ends_with(".getter");
                            self.claim(f, name, &path, None, getter);
                        }
                    }
                }
                _ => {}
            }
        }
        for c in &self.md.conformances {
            let Some((ty, proto)) = self.conformance_parts(c) else {
                continue;
            };
            let cn = format!("<{ty}:{proto}>");
            if !self.spend(&cn) {
                continue;
            }
            data.push(SymFact {
                addr: c.addr,
                name: format!("{cn}.protocol_conformance_descriptor"),
                kind: SymKind::Data,
            });
            if let Some(w) = c.witness_table {
                data.push(SymFact {
                    addr: w,
                    name: format!("{cn}.protocol_witness_table"),
                    kind: SymKind::Data,
                });
            }
            let witness = |addr: u64, member: String, getter: bool| {
                let name = format!("{cn}.{member}");
                self.claim(addr, name, &ty, Some((proto.clone(), member)), getter);
            };
            // A local, non-resilient conformance's witness table is a plain
            // pointer table: the descriptor, then one word per requirement.
            if c.witnesses.is_empty() && c.flags & (1 << 17) == 0 {
                if let (Some(w), Target::Direct(p)) = (c.witness_table, c.protocol) {
                    if let Some(proto_ctx) = self.md.contexts.get(&p) {
                        for (k, (req, _)) in proto_ctx.requirements.iter().enumerate() {
                            let flags = self.md.requirement_flags.get(req).copied().unwrap_or(0xff);
                            let kind = flags & 0xf;
                            if !matches!(kind, 1..=6 | 8) {
                                continue;
                            }
                            let slot = w.wrapping_add(self.md.ptr_size * (1 + k as u64));
                            if let Some(f) = self.md.words.get(&slot) {
                                witness(
                                    *f,
                                    format!("requirement#{k}"),
                                    kind == 3 && flags & 0x10 != 0,
                                );
                            }
                        }
                    }
                }
            }
            for (k, (req, impl_addr)) in c.witnesses.iter().enumerate() {
                let Some(f) = impl_addr else { continue };
                if !self.is_code(*f) {
                    continue;
                }
                let member = match *req {
                    Target::Slot(slot) => {
                        match self.imports.get(&slot).and_then(|r| demangle_swift_name(r)) {
                            Some(n) if n.ends_with(".method_descriptor") => {
                                member_after_protocol(&n[..n.len() - ".method_descriptor".len()])
                            }
                            Some(n) if n.ends_with(".associated_conformance_descriptor") => {
                                Some(format!("associated_conformance#{k}"))
                            }
                            _ => None,
                        }
                    }
                    Target::Direct(r) => self
                        .requirement_index(c.protocol, r)
                        .map(|i| format!("requirement#{i}")),
                };
                let member = member.unwrap_or_else(|| format!("witness#{k}"));
                let getter = match *req {
                    Target::Slot(_) => member.ends_with(".getter"),
                    Target::Direct(r) => {
                        let flags = self.md.requirement_flags.get(&r).copied().unwrap_or(0xff);
                        flags & 0xf == 3 && flags & 0x10 != 0
                    }
                };
                witness(*f, member, getter);
            }
        }
        self.resolve_claims(out);
        // Dedup by address: the first (lowest-addressed descriptor) wins.
        let mut seen = std::collections::HashSet::new();
        for d in data {
            if seen.insert(d.addr) && self.spend(&d.name) {
                out.symbols.push(d);
            }
        }
    }

    /// One name per claimed function. A function several records claim is
    /// code the linker folded (identical code folding): claims of one type
    /// name it after the first member with the count of members it serves
    /// (`<T:P>.requirement#10.merged#4`); witnesses of one protocol member
    /// from different types name it after the member alone
    /// (`<*:Swift.RawRepresentable>.init(rawValue:)`, a later such body
    /// `.2`, `.3`, …); anything else keeps the engine placeholder.
    fn resolve_claims(&self, out: &mut AnalysisOutput) {
        let dump = std::env::var("KUNA_SWIFTMETADATA_DUMP").ok();
        let mut dumped = String::new();
        let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
        for (&addr, claims) in self.claims.borrow().iter() {
            if dump.is_some() {
                for c in claims {
                    dumped.push_str(&format!(
                        "{addr:x}\t{}\t{}\t{}\n",
                        c.name, c.owner, c.getter
                    ));
                }
            }
            let mut names: Vec<&str> = claims.iter().map(|c| c.name.as_str()).collect();
            names.sort_unstable();
            names.dedup();
            let mut name = if names.len() == 1 {
                names[0].to_string()
            } else if claims.iter().all(|c| c.owner == claims[0].owner) {
                format!("{}.merged#{}", names[0], names.len())
            } else if let Some((proto, member)) = claims[0]
                .requirement
                .as_ref()
                .filter(|r| claims.iter().all(|c| c.requirement.as_ref() == Some(r)))
            {
                format!("<*:{proto}>.{member}")
            } else {
                continue;
            };
            let base = name.clone();
            let mut k = 2;
            while !used.insert(name.clone()) {
                name = format!("{base}.{k}");
                k += 1;
            }
            out.symbols.push(SymFact {
                addr,
                name: name.clone(),
                kind: SymKind::Function,
            });
            out.fid_names.push(FidMatch { addr, name });
        }
        if let Some(path) = dump {
            let _ = std::fs::write(path, dumped);
        }
    }

    fn requirement_index(&self, protocol: Target, requirement: u64) -> Option<usize> {
        let Target::Direct(p) = protocol else {
            return None;
        };
        let proto = self.md.contexts.get(&p)?;
        proto
            .requirements
            .iter()
            .position(|(addr, _)| *addr == requirement)
    }

    fn override_name(&self, class_path: &str, o: &descriptors::OverrideEntry) -> Option<String> {
        match o.method {
            Target::Slot(slot) => {
                let desc = import_name(self.imports, slot, "method_descriptor")?;
                let member = member_after_protocol(&desc)?;
                Some(format!("{class_path}.{member}"))
            }
            Target::Direct(m) => {
                let Target::Direct(b) = o.base_class else {
                    return None;
                };
                let base = self.md.contexts.get(&b)?;
                let e = base.vtable.iter().find(|e| e.addr == m)?;
                Some(format!("{class_path}.vtable#{}.{}", e.index, e.kind.tag()))
            }
        }
    }
}
