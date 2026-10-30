//! (kuna `swiftcall`) Swift's calling convention on the functions the
//! metadata and the symbol table name, with their `self` typed.
//!
//! Swift passes a method's `self` in the `swiftself` register (x20 on
//! arm64, r13 on x86-64) when it is one pointer: a class instance, the
//! address of an `inout` (mutating) or indirect value, or a thick
//! metatype; a non-mutating method of a loadable struct gets its `self`
//! exploded into the trailing argument registers instead. A throwing
//! function hands its error back in the `swifterror` register (x21 / r12),
//! which a caller zeroes before the call and tests after it. Neither
//! register is a parameter under the platform's C model, so the `self` a
//! method reads is an unnamed register read, and a call drops the `self`
//! it passes.
//!
//! The engine derives `__swiftcall` from the compiler spec's default model
//! (`kuna_decomp::kuna_swiftcall`): the C model with `swiftself` added as an
//! input resource section of its own, so recovery declares `self` exactly
//! when the body reads it and never forces the argument registers around
//! it; `__swiftcall_throws` also returns the error register to the caller.
//! This arm binds one of them to every function it can place in a type,
//! with no signature (the arity stays recovered), and hints its `self`:
//!
//! - a class's vtable slots and overrides: `Class *` for an instance member,
//!   `swift.class_metadata *` for a class member or allocating initializer;
//! - a conformance's witnesses: the address of the value, `T *` for a
//!   struct and `Class **` for a class, or the metadata for a static
//!   requirement; a protocol's default implementations take the model only;
//! - a symbol the demangler places in a class or struct (an image that kept
//!   its symbols): a method, accessor, initializer or deinitializer, with
//!   `throws` read off its function type.
//!
//! Only `self` is typed, and only when the body reads the register at its
//! full width; a struct method whose `self` is exploded declares no `self`.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_decomp::dtype::Datatype;

use super::descriptors::{ContextKind, Target, TypeRef};
use super::kuna_swifttypes::{Registers, Types};
use crate::demangle::kuna_swift::demangle_swift_tree;
use crate::demangle::kuna_swift::node::{Kind, Node, NodeRef};
use crate::demangle::kuna_swift::print::print_full;
use crate::pass::{AnalysisOutput, InputHintFact, ModelFact};

use kuna_decomp::kuna_swiftcall::{SWIFTCALL, SWIFTCALL_THROWS};

#[derive(Default, Debug)]
pub struct Stats {
    pub bound: usize,
    pub typed_self: usize,
    pub throws: usize,
}

/// What a function's `self` is.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SelfIs {
    Instance,
    Witness,
    Metatype,
}

struct Binding {
    model: &'static str,
    self_t: Option<Rc<Datatype>>,
    maybe_self: bool,
    /// Two bindings named different `self` types.
    conflicted: bool,
}

struct Binder<'t, 'a> {
    types: &'t Types<'a>,
    bound: BTreeMap<u64, Binding>,
}

impl Binder<'_, '_> {
    fn put(&mut self, f: u64, model: &'static str, self_t: Option<Rc<Datatype>>) {
        self.put_with(f, model, self_t, false);
    }

    fn put_with(&mut self, f: u64, model: &'static str, self_t: Option<Rc<Datatype>>, maybe_self: bool) {
        if !self.types.namer.is_code(f) {
            return;
        }
        match self.bound.entry(f) {
            std::collections::btree_map::Entry::Vacant(v) => {
                v.insert(Binding {
                    model,
                    self_t,
                    maybe_self,
                    conflicted: false,
                });
            }
            // One body serving members of different types (identical code
            // folding) names no single `self` type.
            std::collections::btree_map::Entry::Occupied(mut o) => {
                let b = o.get_mut();
                match (&b.self_t, self_t) {
                    (Some(a), Some(c)) if a.get_name() != c.get_name() => {
                        b.self_t = None;
                        b.conflicted = true;
                    }
                    (None, Some(c)) if !b.conflicted => b.self_t = Some(c),
                    _ => {}
                }
            }
        }
    }

    /// The type `self` has in a member of the nominal type at `addr`.
    fn self_type(&self, addr: u64, is: SelfIs) -> Option<Rc<Datatype>> {
        let types = self.types;
        let kind = types.md.contexts.get(&addr)?.kind;
        if !matches!(kind, ContextKind::Class | ContextKind::Struct) {
            return None;
        }
        let path = types.unique_path(addr)?;
        match is {
            SelfIs::Instance => types.self_pointer(addr, &path),
            SelfIs::Witness => types.witness_self(addr, &path),
            SelfIs::Metatype if kind == ContextKind::Class => {
                types.pointer_to(types.class_metadata_struct()?)
            }
            SelfIs::Metatype => types.pointer_to(types.metadata_struct()?),
        }
    }

    fn classes(&mut self) {
        let md = self.types.md;
        let mut contexts: Vec<_> = md.contexts.values().filter(|c| c.kind == ContextKind::Class).collect();
        contexts.sort_by_key(|c| c.addr);
        for ctx in contexts {
            for e in &ctx.vtable {
                let Some(f) = e.impl_addr else { continue };
                let is = if e.instance { SelfIs::Instance } else { SelfIs::Metatype };
                let t = self.self_type(ctx.addr, is);
                self.put(f, SWIFTCALL, t);
            }
            for o in &ctx.overrides {
                let Some(f) = o.impl_addr else { continue };
                let is = match (o.base_class, o.method) {
                    (Target::Direct(b), Target::Direct(m)) => {
                        let Some(e) = md.contexts.get(&b).and_then(|base| base.vtable.iter().find(|e| e.addr == m))
                        else {
                            continue;
                        };
                        if e.instance { SelfIs::Instance } else { SelfIs::Metatype }
                    }
                    (_, Target::Slot(slot)) => match self.types.imports.get(&slot).and_then(|n| imported_is(n)) {
                        Some(is) => is,
                        None => continue,
                    },
                    _ => continue,
                };
                let t = self.self_type(ctx.addr, is);
                self.put(f, SWIFTCALL, t);
            }
        }
    }

    fn conformances(&mut self) {
        let md = self.types.md;
        for c in &md.conformances {
            let TypeRef::Descriptor(Target::Direct(t)) = c.type_ref else {
                continue;
            };
            let local_rows = c.witnesses.is_empty() && c.flags & (1 << 17) == 0;
            if let (true, Some(w), Target::Direct(p)) = (local_rows, c.witness_table, c.protocol) {
                let Some(proto) = md.contexts.get(&p) else { continue };
                for (k, (req, _)) in proto.requirements.iter().enumerate() {
                    let Some(is) = requirement_is(md.requirement_flags.get(req).copied()) else {
                        continue;
                    };
                    let slot = w + md.ptr_size * (1 + k as u64);
                    if let Some(&f) = md.words.get(&slot) {
                        let is = if is == SelfIs::Instance { SelfIs::Witness } else { is };
                        let ty = self.self_type(t, is);
                        self.put(f, SWIFTCALL, ty);
                    }
                }
            }
            for (req, impl_addr) in &c.witnesses {
                let Some(f) = *impl_addr else { continue };
                let is = match *req {
                    Target::Direct(r) => requirement_is(md.requirement_flags.get(&r).copied()),
                    Target::Slot(slot) => self.types.imports.get(&slot).and_then(|n| imported_is(n)),
                };
                let Some(is) = is else { continue };
                let is = if is == SelfIs::Instance { SelfIs::Witness } else { is };
                let ty = self.self_type(t, is);
                self.put(f, SWIFTCALL, ty);
            }
        }
        let mut protocols: Vec<_> = md.contexts.values().filter(|c| c.kind == ContextKind::Protocol).collect();
        protocols.sort_by_key(|c| c.addr);
        for proto in protocols {
            for (req, default_impl) in &proto.requirements {
                if let (Some(f), Some(_)) = (*default_impl, requirement_is(md.requirement_flags.get(req).copied())) {
                    self.put(f, SWIFTCALL, None);
                }
            }
        }
    }

    fn symbols(&mut self, symbols: &[(u64, String)]) {
        for (addr, name) in symbols {
            let Some(tree) = demangle_swift_tree(name) else { continue };
            let [entity] = tree.children.as_slice() else { continue };
            let Some((owner, is, throws)) = member(entity) else { continue };
            let model = if throws { SWIFTCALL_THROWS } else { SWIFTCALL };
            let value_type = matches!(owner_kind(&owner), Some(Kind::Structure | Kind::Enum));
            if value_type && is == SelfIs::Metatype {
                continue;
            }
            let ty = match nominal(&owner) {
                Some(path) => self.types.nominal_at(&path).and_then(|a| self.self_type(a, is)),
                None => None,
            };
            self.put_with(*addr, model, ty, value_type && is == SelfIs::Instance);
        }
    }
}

/// `self` of a requirement with these `ProtocolRequirementFlags`: kinds 1-6
/// (method, initializer, accessors); `IsInstance` is bit 4. An
/// initializer requirement is static.
fn requirement_is(flags: Option<u32>) -> Option<SelfIs> {
    let flags = flags?;
    if !matches!(flags & 0xf, 1..=6) {
        return None;
    }
    Some(if flags & 0x10 != 0 { SelfIs::Instance } else { SelfIs::Metatype })
}

/// `self` of the member an imported method descriptor (`…Tq`) names.
fn imported_is(import: &str) -> Option<SelfIs> {
    let tree = demangle_swift_tree(import).or_else(|| demangle_swift_tree(&format!("_{import}")))?;
    let top = tree.first()?;
    if top.kind != Kind::MethodDescriptor {
        return None;
    }
    let entity = top.first()?;
    Some(match entity.kind {
        Kind::Static | Kind::Allocator => SelfIs::Metatype,
        _ => SelfIs::Instance,
    })
}

/// The owner, `self` and `throws` of a member entity, `None` for anything
/// that is not a method, accessor, initializer or deinitializer.
fn member(entity: &NodeRef) -> Option<(NodeRef, SelfIs, bool)> {
    match entity.kind {
        Kind::ProtocolWitness => {
            let conformance = entity.first()?;
            let ty = conformance.first()?;
            let inner = entity.child(1)?;
            let (_, is, throws) = member(inner)?;
            let is = if is == SelfIs::Instance { SelfIs::Witness } else { is };
            Some((Rc::clone(ty), is, throws))
        }
        Kind::Static => {
            let (owner, _, throws) = member(entity.first()?)?;
            Some((owner, SelfIs::Metatype, throws))
        }
        Kind::Function => Some((Rc::clone(entity.first()?), SelfIs::Instance, throws(entity))),
        Kind::Allocator => Some((Rc::clone(entity.first()?), SelfIs::Metatype, throws(entity))),
        Kind::Constructor | Kind::Destructor | Kind::Deallocator => {
            Some((Rc::clone(entity.first()?), SelfIs::Instance, throws(entity)))
        }
        Kind::Getter
        | Kind::Setter
        | Kind::ModifyAccessor
        | Kind::Modify2Accessor
        | Kind::ReadAccessor
        | Kind::Read2Accessor => {
            let storage = entity.first()?;
            if !matches!(storage.kind, Kind::Variable | Kind::Subscript) {
                return None;
            }
            Some((Rc::clone(storage.first()?), SelfIs::Instance, false))
        }
        _ => None,
    }
    .filter(|(owner, _, _)| owner_kind(owner).is_some())
}

/// The nominal kind a member's owner names, through an extension.
fn owner_kind(owner: &NodeRef) -> Option<Kind> {
    let n = match owner.kind {
        Kind::Extension => owner.child(1)?,
        Kind::Type => owner.first()?,
        _ => owner,
    };
    let n = if n.kind == Kind::Type { n.first()? } else { n };
    matches!(n.kind, Kind::Class | Kind::Structure | Kind::Enum | Kind::Protocol).then_some(n.kind)
}

/// The dotted path of a member's owner, when it is a class or struct.
fn nominal(owner: &NodeRef) -> Option<String> {
    let n = match owner.kind {
        Kind::Extension => owner.child(1)?,
        _ => owner,
    };
    let n = if n.kind == Kind::Type { Rc::clone(n.first()?) } else { Rc::clone(n) };
    if !matches!(n.kind, Kind::Class | Kind::Structure) {
        return None;
    }
    print_full(&Node::with(Kind::Type, vec![n]))
}

/// Whether the function type of an entity throws.
fn throws(entity: &NodeRef) -> bool {
    let Some(mut t) = entity.child_of_kind(Kind::Type) else { return false };
    loop {
        let Some(inner) = t.first() else { return false };
        match inner.kind {
            Kind::Type => t = inner,
            Kind::DependentGenericType => match inner.child_of_kind(Kind::Type) {
                Some(next) => t = next,
                None => return false,
            },
            Kind::FunctionType | Kind::NoEscapeFunctionType => {
                return inner
                    .children
                    .iter()
                    .any(|c| matches!(c.kind, Kind::ThrowsAnnotation | Kind::TypedThrowsAnnotation));
            }
            _ => return false,
        }
    }
}

/// Bind the convention to every member the image names, and hint `self`.
pub fn bind(types: &Types, regs: &Registers, symbols: &[(u64, String)], out: &mut AnalysisOutput) -> Stats {
    let mut b = Binder {
        types,
        bound: BTreeMap::new(),
    };
    b.symbols(symbols);
    b.classes();
    b.conformances();
    let mut stats = Stats::default();
    if !b.bound.is_empty() {
        out.swift_types.regs = Some((regs.self_reg, regs.error_reg));
    }
    for (f, binding) in b.bound {
        stats.bound += 1;
        if binding.model == SWIFTCALL_THROWS {
            stats.throws += 1;
        }
        out.swift_types.models.push(ModelFact {
            func_addr: f,
            model: binding.model,
            maybe_self: binding.maybe_self,
        });
        if binding.self_t.is_some() {
            stats.typed_self += 1;
        }
        out.swift_types.input_hints.push(InputHintFact {
            func_addr: f,
            reg: regs.self_reg,
            name: "self".to_string(),
            type_: binding.self_t,
        });
    }
    stats
}
