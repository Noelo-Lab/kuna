//! (kuna `swifthelpers`) The helpers swiftc emits into every image that
//! uses generic types, named from their bodies, and the types they are
//! asked for, named at each call.
//!
//! - `__swift_instantiateConcreteTypeFromMangledName` (and its `Abstract`
//!   twin): test a cache word, else hand the mangled type name the cache
//!   (or a second argument) points at to
//!   `swift_getTypeByMangledNameInContext2` (`…InMetadataState2`) and store
//!   the metadata it returns. A stripped image keeps no name for it. The
//!   body is recognised by its one runtime call and the relative-pointer
//!   resolution (`add xD, xB, wN, sxtw`) that computes the name; whether
//!   the record holding the name is the first argument (the cache itself,
//!   `ldr x0, [x0]` holds the offset and the negated length until it is
//!   filled) or the second (a separate cache) is read off the register the
//!   resolution is based on. The helper is named and given the prototype
//!   `void *f(void *cache[, void *name])` (each call's result is typed
//!   `swift.type_metadata *` where it can be, `kuna_swiftmetaresult`), and at every
//!   call whose arguments are `adrp`/`add` (or `adr`) constants the mangled
//!   name is demangled and the cache named after the type it caches
//!   (`hf.Box<hf.Unit>.type_metadata_demangling_cache`), as the image's
//!   own `…MD` symbol would be.
//! - Lazy protocol witness table accessors: `adrp; ldr x0, [cache]; cbz x0;
//!   ret`, then one call for the concrete type's metadata (`mov x1, x0`)
//!   and `swift_getWitnessTable(conformance, type, conditional tables)`.
//!   The conformance descriptor (local, or a `__got` import) names the
//!   protocol, the metadata call names the type (an instantiation helper
//!   call above, a local type's accessor, or an imported `…Ma`), and the
//!   function becomes `<T:P>.lazy_protocol_witness_table_accessor`, its
//!   cache `<T:P>.lazy_protocol_witness_table_cache`, with the prototype
//!   `void *f(void)`.
//!
//! arm64 only. Function names follow the metadata pass's label-gated rename
//! (a real symbol is never replaced); a body that does not decode to the
//! shapes above, or a call whose arguments are not constants, is left as
//! it was.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

use kuna_decomp::dtype::Datatype;
use kuna_decomp::fspec::PrototypePieces;
use object::read::{Object, ObjectSection};

use super::descriptors::{Target, TypeRef};
use super::kuna_swifttypes::Types;
use crate::demangle::kuna_swift::demangle_swift_tree;
use crate::demangle::kuna_swift::demangle_swift_type;
use crate::demangle::kuna_swift::name::print_name;
use crate::demangle::kuna_swift::node::{Kind, Node, NodeRef};
use crate::pass::{AnalysisOutput, FidMatch, SymFact, SymKind, TypedDataFact};


#[derive(Default, Debug)]
pub struct Stats {
    pub helpers: usize,
    pub caches: usize,
    pub lazy_accessors: usize,
}

/// The longest body either helper shape is looked for in, in instructions.
const MAX_BODY: usize = 48;
/// How far back from a call its constant arguments are looked for.
const MAX_BACK: usize = 8;

/// Which argument of an instantiation helper holds the mangled-name record.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Record {
    /// The cache word itself (offset and negated length until filled).
    Cache,
    /// A second argument; the first is a separate cache word.
    Second,
}

#[derive(Clone, Copy, Debug)]
struct Helper {
    record: Record,
    abstract_state: bool,
}

/// Where a register's value at a call comes from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Value {
    Addr(u64),
    /// Loaded from this pointer slot (a `__got` entry).
    Slot(u64),
}

fn adrp_page(pc: u64, w: u32) -> u64 {
    let imm = (((w >> 5) & 0x7_ffff) << 2 | ((w >> 29) & 3)) as i64;
    let imm = (imm << 43) >> 43;
    (pc & !0xfff).wrapping_add((imm << 12) as u64)
}

fn adr_target(pc: u64, w: u32) -> u64 {
    let imm = (((w >> 5) & 0x7_ffff) << 2 | ((w >> 29) & 3)) as i64;
    let imm = (imm << 43) >> 43;
    pc.wrapping_add(imm as u64)
}

fn is_adrp(w: u32) -> bool {
    w & 0x9f00_0000 == 0x9000_0000
}

fn is_adr(w: u32) -> bool {
    w & 0x9f00_0000 == 0x1000_0000
}

/// `add xd, xn, #imm{, lsl #12}` (64-bit, flags untouched).
fn add_imm(w: u32) -> Option<(u32, u32, u64)> {
    if w & 0xff80_0000 != 0x9100_0000 {
        return None;
    }
    let imm = ((w >> 10) & 0xfff) as u64;
    let imm = if w & (1 << 22) != 0 { imm << 12 } else { imm };
    Some((w & 31, (w >> 5) & 31, imm))
}

/// `ldr xt, [xn, #imm]` (64-bit, unsigned offset).
fn ldr_imm(w: u32) -> Option<(u32, u32, u64)> {
    if w & 0xffc0_0000 != 0xf940_0000 {
        return None;
    }
    Some((w & 31, (w >> 5) & 31, (((w >> 10) & 0xfff) as u64) * 8))
}

fn bl_target(pc: u64, w: u32) -> Option<u64> {
    if w & 0xfc00_0000 != 0x9400_0000 {
        return None;
    }
    let imm = ((w & 0x03ff_ffff) as i64) << 38 >> 36;
    Some(pc.wrapping_add(imm as u64))
}

/// `mov xd, xs` (`orr xd, xzr, xs`).
fn mov_reg(w: u32) -> Option<(u32, u32)> {
    (w & 0xffe0_ffe0 == 0xaa00_03e0).then_some((w & 31, (w >> 16) & 31))
}

/// `add xd, xn, wm, sxtw`.
fn add_sxtw(w: u32) -> Option<(u32, u32, u32)> {
    (w & 0xffe0_fc00 == 0x8b20_c000).then_some((w & 31, (w >> 5) & 31, (w >> 16) & 31))
}

/// A branch, call or return (the system instructions sharing the encoding
/// group, `nop` among them, are not).
fn is_branch(w: u32) -> bool {
    w & 0x1c00_0000 == 0x1400_0000 && w & 0xffc0_0000 != 0xd500_0000
}

const RET: u32 = 0xd65f_03c0;

/// `b.cond`, `cbz`/`cbnz` or `tbz`/`tbnz`.
fn is_cond_branch(w: u32) -> bool {
    w & 0xff00_0010 == 0x5400_0000 || w & 0x7e00_0000 == 0x3400_0000 || w & 0x7e00_0000 == 0x3600_0000
}

/// The general register an instruction writes, as far as the backward
/// walks need: `None` for a store, a compare, a branch or a nop, else the
/// destination field (a load pair also writes its second register,
/// reported by [`writes_second`]).
fn writes(w: u32) -> Option<u32> {
    if w == 0xd503_201f || is_branch(w) {
        return None;
    }
    // Loads and stores.
    if w & 0x0a00_0000 == 0x0800_0000 {
        // Load/store pair.
        if w & 0x3a00_0000 == 0x2800_0000 {
            return (w & (1 << 22) != 0).then_some(w & 31);
        }
        // Single register, immediate or register offset: opc in bits 22-23.
        if w & 0x3b00_0000 == 0x3900_0000 || w & 0x3b20_0000 == 0x3800_0000 {
            return (w & (3 << 22) != 0).then_some(w & 31);
        }
        // Exclusive / acquire-release: L in bit 22.
        if w & 0x3f00_0000 == 0x0800_0000 {
            return (w & (1 << 22) != 0).then_some(w & 31);
        }
        return Some(w & 31);
    }
    // Flag-setting compares write xzr.
    let rd = w & 31;
    (rd != 31).then_some(rd)
}

fn writes_second(w: u32) -> Option<u32> {
    (w & 0x3a40_0000 == 0x2840_0000).then_some((w >> 10) & 31)
}

fn writes_reg(w: u32, r: u32) -> bool {
    writes(w) == Some(r) || writes_second(w) == Some(r)
}

/// The code of the image's text sections.
struct Code<'d> {
    sections: Vec<(u64, &'d [u8])>,
}

impl<'d> Code<'d> {
    fn new(file: &'d object::File<'d>) -> Code<'d> {
        let sections = file
            .sections()
            .filter(|s| s.kind() == object::SectionKind::Text)
            .filter_map(|s| Some((s.address(), s.data().ok()?)))
            .collect();
        Code { sections }
    }

    fn word(&self, at: u64) -> Option<u32> {
        let (base, data) = self.sections.iter().find(|(b, d)| at >= *b && at - *b < d.len() as u64)?;
        let off = usize::try_from(at - base).ok()?;
        Some(u32::from_le_bytes(data.get(off..off.checked_add(4)?)?.try_into().ok()?))
    }

    fn words(&self, at: u64, n: usize) -> Vec<u32> {
        (0..n as u64).map_while(|i| self.word(at.wrapping_add(4 * i))).collect()
    }

    /// Every `bl` in the text, as `(call site, target)`.
    fn for_each_call(&self, mut f: impl FnMut(u64, u64)) {
        for (base, data) in &self.sections {
            for (i, c) in data.chunks_exact(4).enumerate() {
                let w = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                let pc = base.wrapping_add(4 * i as u64);
                if let Some(t) = bl_target(pc, w) {
                    f(pc, t);
                }
            }
        }
    }

    /// The constant `reg` holds just before the instruction at `at`, from
    /// the straight-line code above it.
    fn value_before(&self, at: u64, reg: u32) -> Option<Value> {
        let mut pc = at;
        let mut want = reg;
        let mut addend = 0u64;
        let mut seen_add = false;
        for _ in 0..MAX_BACK {
            pc = pc.checked_sub(4)?;
            let w = self.word(pc)?;
            if is_branch(w) || w == RET {
                return None;
            }
            if !writes_reg(w, want) {
                continue;
            }
            if is_adr(w) && w & 31 == want {
                return Some(Value::Addr(adr_target(pc, w).wrapping_add(addend)));
            }
            if is_adrp(w) && w & 31 == want {
                return seen_add.then(|| Value::Addr(adrp_page(pc, w).wrapping_add(addend)));
            }
            if let Some((rd, rn, imm)) = add_imm(w) {
                if rd == want && !seen_add && rn != 31 {
                    want = rn;
                    addend = imm;
                    seen_add = true;
                    continue;
                }
            }
            if let Some((rt, rn, imm)) = ldr_imm(w) {
                if rt == want && !seen_add {
                    return self.page_before(pc, rn).map(|p| Value::Slot(p.wrapping_add(imm)));
                }
            }
            return None;
        }
        None
    }

    /// The `adrp` page `reg` holds before `at`.
    fn page_before(&self, at: u64, reg: u32) -> Option<u64> {
        let mut pc = at;
        for _ in 0..MAX_BACK {
            pc = pc.checked_sub(4)?;
            let w = self.word(pc)?;
            if is_branch(w) || w == RET {
                return None;
            }
            if writes_reg(w, reg) {
                return (is_adrp(w) && w & 31 == reg).then(|| adrp_page(pc, w));
            }
        }
        None
    }
}

pub fn emit(types: &Types, file: &object::File, out: &mut AnalysisOutput) -> Stats {
    let mut stats = Stats::default();
    if file.architecture() != object::Architecture::Aarch64 {
        return stats;
    }
    let code = Code::new(file);
    let accessors = types
        .md
        .contexts
        .values()
        .filter_map(|c| Some((c.accessor?, c.addr)))
        .collect();
    let names = CalleeNames { types, code: &code, accessors, resolved: Default::default() };
    let mut seen: HashSet<u64> = HashSet::new();
    let mut helpers: BTreeMap<u64, Helper> = BTreeMap::new();
    let mut lazy: BTreeMap<u64, Lazy> = BTreeMap::new();
    code.for_each_call(|_, t| {
        if !seen.insert(t) {
            return;
        }
        if let Some(h) = names.helper(t) {
            helpers.insert(t, h);
        } else if let Some(l) = names.lazy(t) {
            lazy.insert(t, l);
        }
    });
    if helpers.is_empty() && lazy.is_empty() {
        return stats;
    }
    let metadata_ptr = types
        .metadata_struct()
        .and_then(|t| types.tf.get_type_pointer(types.ptr_size, t, 1).ok());
    let void_ptr = types.void_ptr();
    for (&addr, h) in &helpers {
        let name = if h.abstract_state {
            "__swift_instantiateConcreteTypeFromMangledNameAbstract"
        } else {
            "__swift_instantiateConcreteTypeFromMangledName"
        };
        rename(out, addr, name.to_string());
        stats.helpers += 1;
        if let Some(p) = void_ptr.clone() {
            let (intypes, innames): (Vec<Rc<Datatype>>, Vec<String>) = match h.record {
                Record::Cache => (vec![Rc::clone(&p)], vec!["cache".into()]),
                Record::Second => (vec![Rc::clone(&p), Rc::clone(&p)], vec!["cache".into(), "name".into()]),
            };
            out.swift_helpers.prototypes_at.push((addr, pieces(name, Some(p), intypes, innames)));
        }
    }
    // The type each instantiation call asks for, by cache word.
    let mut cache_types: BTreeMap<u64, NodeRef> = BTreeMap::new();
    let mut decoded: HashMap<u64, Option<NodeRef>> = HashMap::new();
    code.for_each_call(|site, target| {
        let Some(h) = helpers.get(&target) else { return };
        if let Some((cache, node)) = names.instantiation(site, *h, &mut decoded) {
            cache_types.entry(cache).or_insert(node);
        }
    });
    for (cache, node) in &cache_types {
        let g = Node::with(Kind::Global, vec![Node::with(Kind::TypeMetadataDemanglingCache, vec![Rc::clone(node)])]);
        if let (Some(name), Some(t)) = (print_name(&g), metadata_ptr.clone()) {
            out.swift_helpers.typed_data.push(TypedDataFact { addr: *cache, name, type_: t });
            stats.caches += 1;
        }
    }
    for (addr, l) in lazy {
        if name_lazy(&names, addr, &l, &helpers, &mut decoded, void_ptr.as_ref(), out) {
            stats.lazy_accessors += 1;
        }
    }
    stats
}

/// Name one lazy witness table accessor and its cache after the conformance
/// at the concrete type; `false` when either part cannot be read.
fn name_lazy(
    names: &CalleeNames,
    addr: u64,
    l: &Lazy,
    helpers: &BTreeMap<u64, Helper>,
    decoded: &mut HashMap<u64, Option<NodeRef>>,
    void_ptr: Option<&Rc<Datatype>>,
    out: &mut AnalysisOutput,
) -> bool {
    let Some(ty) = names.metadata_call_type(l.metadata_call, helpers, decoded) else { return false };
    let Some(proto) = names.conformance_protocol(l.conformance) else { return false };
    let conf = Node::with(Kind::ProtocolConformance, vec![Rc::clone(&ty), proto]);
    let accessor = Node::with(
        Kind::Global,
        vec![Node::with(Kind::LazyProtocolWitnessTableAccessor, vec![Rc::clone(&ty), Rc::clone(&conf)])],
    );
    let cache = Node::with(
        Kind::Global,
        vec![Node::with(Kind::LazyProtocolWitnessTableCacheVariable, vec![ty, conf])],
    );
    let Some(name) = print_name(&accessor) else { return false };
    rename(out, addr, name.clone());
    if let Some(p) = void_ptr {
        if let Some(cname) = print_name(&cache) {
            out.swift_helpers.typed_data.push(TypedDataFact { addr: l.cache, name: cname, type_: Rc::clone(p) });
        }
        out.swift_helpers.prototypes_at.push((addr, pieces(&name, Some(Rc::clone(p)), Vec::new(), Vec::new())));
    }
    true
}

fn rename(out: &mut AnalysisOutput, addr: u64, name: String) {
    out.swift_helpers.symbols.push(SymFact { addr, name: name.clone(), kind: SymKind::Function });
    out.swift_helpers.fid_names.push(FidMatch { addr, name, by_signature: false });
}

fn pieces(name: &str, outtype: Option<Rc<Datatype>>, intypes: Vec<Rc<Datatype>>, innames: Vec<String>) -> PrototypePieces {
    PrototypePieces {
        name: name.to_string(),
        outtype,
        intypes,
        innames,
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
        kuna_library_names: None,
        kuna_library_table: false,
    }
}

/// A lazy witness table accessor's parts.
struct Lazy {
    cache: u64,
    /// The call whose result is the conforming type's metadata.
    metadata_call: (u64, u64),
    conformance: Value,
}

struct CalleeNames<'t, 'a, 'd> {
    types: &'t Types<'a>,
    code: &'t Code<'d>,
    /// Local metadata accessor → its type's descriptor.
    accessors: HashMap<u64, u64>,
    /// Symbolic references already resolved, by kind and the address they
    /// name.
    resolved: std::cell::RefCell<HashMap<(u8, u64), Option<NodeRef>>>,
}

impl CalleeNames<'_, '_, '_> {
    /// The import a call target is (a stub or PLT entry), its leading `_`
    /// dropped.
    fn import(&self, target: u64) -> Option<&str> {
        let imports = self.types.imports;
        let name = match imports.get(&target) {
            Some(n) => n.as_str(),
            None => imports.get(&self.stub_slot(target)?)?.as_str(),
        };
        Some(name.strip_prefix('_').unwrap_or(name))
    }

    /// The pointer slot a stub jumps through: `adrp x16; ldr x16, [x16, #o];
    /// br x16`, the arm64e `adrp x17; add x17, x17, #o; ldr x16, [x17];
    /// braa x16, x17`, or a PLT entry `adrp x16; ldr x17, [x16, #o]; add
    /// x16, x16, #o; br x17`.
    fn stub_slot(&self, at: u64) -> Option<u64> {
        let w = self.code.words(at, 4);
        let [a, b, c, d] = w[..] else { return None };
        if !is_adrp(a) {
            return None;
        }
        let page = adrp_page(at, a);
        let r = a & 31;
        if let Some((rt, rn, imm)) = ldr_imm(b) {
            let br = 0xd61f_0000 | (rt << 5);
            if rn == r && (c == br || d == br) {
                return Some(page.wrapping_add(imm));
            }
        }
        if let (Some((rd, rn, imm)), Some((rt, rn2, 0))) = (add_imm(b), ldr_imm(c)) {
            if rd == r && rn == r && rn2 == r && d & 0xffff_fc00 == 0xd71f_0800 && (d >> 5) & 31 == rt {
                return Some(page.wrapping_add(imm));
            }
        }
        None
    }

    /// An instantiation helper's shape: one call, to the runtime's
    /// mangled-name lookup, whose name argument is a relative pointer
    /// resolved off the first or the second argument.
    fn helper(&self, at: u64) -> Option<Helper> {
        // Both forms load the cache word (`ldr xN, [x0]`) among their first
        // instructions.
        let head = self.code.words(at, 8);
        if !head.iter().any(|&x| ldr_imm(x).is_some_and(|(_, rn, off)| rn == 0 && off == 0)) {
            return None;
        }
        let w = self.code.words(at, MAX_BODY);
        let mut arg0: u32 = 1;
        let mut arg1: u32 = 1 << 1;
        let mut record = None;
        let mut call = None;
        let mut saw_test = false;
        for (i, &x) in w.iter().enumerate() {
            let pc = at + 4 * i as u64;
            if let Some(t) = bl_target(pc, x) {
                if call.is_some() {
                    return None;
                }
                call = Some(t);
                continue;
            }
            if x & 0xfc00_0000 == 0x1400_0000 || x & 0xfffffc1f == 0xd63f_0000 || x & 0xfffffc1f == 0xd61f_0000 {
                return None;
            }
            if x == RET {
                // The cache hit may return before the call; anything else
                // that returns first is not this helper.
                if call.is_some() {
                    break;
                }
                if !saw_test {
                    return None;
                }
            }
            if is_cond_branch(x) {
                saw_test = true;
            }
            if call.is_none() {
                if let Some((_, rn, _)) = add_sxtw(x) {
                    if arg1 & (1 << rn) != 0 {
                        record = Some(Record::Second);
                    } else if arg0 & (1 << rn) != 0 {
                        record = Some(Record::Cache);
                    }
                }
                if let Some((rd, rs)) = mov_reg(x) {
                    let bit = 1u32 << rd;
                    arg0 = if arg0 & (1 << rs) != 0 { arg0 | bit } else { arg0 & !bit };
                    arg1 = if arg1 & (1 << rs) != 0 { arg1 | bit } else { arg1 & !bit };
                    continue;
                }
                for r in [writes(x), writes_second(x)].into_iter().flatten() {
                    arg0 &= !(1 << r);
                    arg1 &= !(1 << r);
                }
            }
        }
        let abstract_state = match self.import(call?)? {
            "swift_getTypeByMangledNameInContext2" => false,
            "swift_getTypeByMangledNameInContextInMetadataState2" => true,
            _ => return None,
        };
        Some(Helper { record: record?, abstract_state })
    }

    /// A lazy witness table accessor: the cache test opening it, then the
    /// metadata call and `swift_getWitnessTable`.
    fn lazy(&self, at: u64) -> Option<Lazy> {
        let [a, b, c, d] = self.code.words(at, 4)[..] else { return None };
        let (rt, rn, off) = ldr_imm(b)?;
        if !is_adrp(a) || a & 31 != rn || rt != 0 || c != 0xb400_0040 || d != RET {
            return None;
        }
        let cache = adrp_page(at, a).wrapping_add(off);
        let w = self.code.words(at, MAX_BODY);
        let mut calls = Vec::new();
        for (i, &x) in w.iter().enumerate().skip(4) {
            let pc = at + 4 * i as u64;
            if let Some(t) = bl_target(pc, x) {
                calls.push((pc, t));
                if self.import(t) == Some("swift_getWitnessTable") {
                    break;
                }
            } else if x == RET || is_branch(x) {
                return None;
            }
        }
        let &(wt_site, wt) = calls.last()?;
        if self.import(wt) != Some("swift_getWitnessTable") {
            return None;
        }
        let conformance = self.code.value_before(wt_site, 0)?;
        // x1 = x0 straight after the metadata call.
        let k = calls.len().checked_sub(2)?;
        let metadata_call = calls[k];
        let after = self.code.word(metadata_call.0 + 4)?;
        if mov_reg(after) != Some((1, 0)) {
            return None;
        }
        let mut pc = metadata_call.0 + 8;
        while pc < wt_site {
            if writes_reg(self.code.word(pc)?, 1) {
                return None;
            }
            pc += 4;
        }
        Some(Lazy { cache, metadata_call, conformance })
    }

    /// The cache word and the type of one instantiation call.
    fn instantiation(
        &self,
        site: u64,
        h: Helper,
        decoded: &mut HashMap<u64, Option<NodeRef>>,
    ) -> Option<(u64, NodeRef)> {
        let Value::Addr(cache) = self.code.value_before(site, 0)? else { return None };
        let record = match h.record {
            Record::Cache => cache,
            Record::Second => match self.code.value_before(site, 1)? {
                Value::Addr(r) => r,
                Value::Slot(_) => return None,
            },
        };
        let node = decoded.entry(record).or_insert_with(|| self.mangled_type(record)).clone()?;
        Some((cache, node))
    }

    /// The type a mangled-name record (`int32` offset to the name, then
    /// its length) names.
    fn mangled_type(&self, record: u64) -> Option<NodeRef> {
        let img = self.types.reader.img;
        let rel = i32::from_le_bytes(img.read(record, 4)?.try_into().ok()?);
        let at = record.wrapping_add(rel as i64 as u64);
        let bytes = self.types.reader.mangled_bytes(at)?;
        let resolver = |kind: u8, pos: usize, value: u64| {
            let target = at.wrapping_add(pos as u64).wrapping_add(value as u32 as i32 as i64 as u64);
            self.resolved
                .borrow_mut()
                .entry((kind, target))
                .or_insert_with(|| self.types.resolve(at, kind, pos, value))
                .clone()
        };
        let node = demangle_swift_type(&bytes, &resolver)?;
        (node.kind == Kind::Type).then_some(node)
    }

    /// The type whose metadata the call at `(site, target)` returns.
    fn metadata_call_type(
        &self,
        (site, target): (u64, u64),
        helpers: &BTreeMap<u64, Helper>,
        decoded: &mut HashMap<u64, Option<NodeRef>>,
    ) -> Option<NodeRef> {
        if let Some(h) = helpers.get(&target) {
            return self.instantiation(site, *h, decoded).map(|(_, n)| n);
        }
        if let Some(ctx) = self.accessors.get(&target).and_then(|a| self.types.md.contexts.get(a)) {
            if ctx.flags & 0x80 != 0 {
                return None;
            }
            return self.types.context_node(ctx.addr, 0);
        }
        let tree = demangle_swift_tree(self.import(target)?)
            .or_else(|| demangle_swift_tree(&format!("_{}", self.import(target)?)))?;
        let top = tree.first()?;
        (top.kind == Kind::TypeMetadataAccessFunction).then(|| top.first().cloned()).flatten()
    }

    /// The protocol of the conformance descriptor a lazy accessor passes.
    fn conformance_protocol(&self, v: Value) -> Option<NodeRef> {
        match v {
            Value::Addr(d) => {
                let c = self.types.md.conformances.iter().find(|c| c.addr == d)?;
                if !matches!(c.type_ref, TypeRef::Descriptor(_)) {
                    return None;
                }
                match c.protocol {
                    Target::Direct(p) => self.types.context_node(p, 0),
                    Target::Slot(s) => super::kuna_swifttypes::imported_node(self.types.imports.get(&s)?),
                }
            }
            Value::Slot(s) => {
                let raw = self.types.imports.get(&s)?;
                let tree = demangle_swift_tree(raw).or_else(|| demangle_swift_tree(&format!("_{raw}")))?;
                let top = tree.first()?;
                if top.kind != Kind::ProtocolConformanceDescriptor {
                    return None;
                }
                let conf = top.first()?;
                (conf.kind == Kind::ProtocolConformance).then(|| conf.child(1).cloned()).flatten()
            }
        }
    }
}
