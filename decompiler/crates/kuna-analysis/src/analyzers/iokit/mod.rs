//! (kuna `iokitclasses`) IOKit C++ class recovery for XNU kernels and kexts.
//!
//! Every libkern class registers itself from a static initializer: the
//! `__mod_init_func` code calls `OSMetaClass::OSMetaClass(&X::gMetaClass,
//! "X", &Super::gMetaClass, sizeof(X))` and then stores the `X::MetaClass`
//! vtable into `X::gMetaClass`. Tracing those initializers with a small
//! AArch64 register tracker ([`a64`]) gives each class's name, metaclass
//! object, superclass and instance size with no symbols. The constructor is
//! the callee most of those call sites share (with a class-name string in
//! `x1`), so it is found by vote rather than by name.
//!
//! A class's own vtable is found through `X::getMetaClass() const`, a leaf
//! that returns `&X::gMetaClass` within five instructions: a run of code
//! pointers in a data section, behind the two zero header words, holding such
//! a function in its first twelve slots is `X`'s vtable. Slots are then named
//! by inheritance: a slot that overrides a named slot of the superclass's
//! vtable takes the superclass method's name under `X::`.
//!
//! Emits data symbols (`X::gMetaClass`, `vtable for X`, `vtable for
//! X::MetaClass`), placeholder-only function renames, and every vtable slot as
//! a function entry (methods reachable only through a vtable are otherwise
//! never discovered).

mod a64;
pub mod osclassinfo;
mod tables;

pub use osclassinfo::OsClassInfoPass;
pub use tables::XnuTablesPass;

use std::collections::{BTreeMap, HashMap};

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{Architecture, SectionFlags, SectionKind};

use crate::loader::format::macho::resolve_chained_fixups;
use crate::objc::sections::MachoImage;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, DataObjectFact, FidMatch, Phase, SymFact, SymKind};
use a64::Event;

const MH_KEXT_BUNDLE: u32 = 0xb;
const S_MOD_INIT_FUNC_POINTERS: u32 = 0x9;
const S_INIT_FUNC_OFFSETS: u32 = 0x16;
const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x400;
const S_ATTR_PURE_INSTRUCTIONS: u32 = 0x8000_0000;
const INIT_BUDGET: usize = 0x4000;
const GET_META_CLASS_LIMIT: usize = 5;
const GET_META_CLASS_SLOTS: usize = 12;

#[derive(Default)]
pub struct IokitClassesPass;

impl AnalysisPass for IokitClassesPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "iokitclasses"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if ctx.file.format() != object::BinaryFormat::MachO
            || ctx.file.architecture() != Architecture::Aarch64
            || !is_xnu_image(ctx.file, ctx.bytes)
        {
            return out;
        }
        let img = MachoImage::with_fixups(ctx.file, resolve_chained_fixups(ctx.file, ctx.bytes));
        let classes = recover(ctx.file, &img);
        classes.emit(&mut out);
        out
    }
}

/// A kext (`MH_KEXT_BUNDLE`) or an XNU kernel (it has a `__KLD` segment).
fn is_xnu_image(file: &object::File, bytes: &[u8]) -> bool {
    let filetype = bytes.get(12..16).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
    filetype == Some(MH_KEXT_BUNDLE) || file.sections().any(|s| s.segment_name().ok().flatten() == Some("__KLD"))
}

fn macho_flags(sec: &object::Section) -> u32 {
    match sec.flags() {
        SectionFlags::MachO { flags } => flags,
        _ => 0,
    }
}

/// One recovered class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IokitClass {
    pub name: String,
    pub meta: u64,
    pub super_meta: Option<u64>,
    pub size: u64,
    /// Slot 0 of `X::MetaClass`'s vtable, as the initializer stores it.
    pub meta_vtable: Option<u64>,
    /// Slot 0 of `X`'s own vtable and its slots.
    pub vtable: Option<(u64, Vec<u64>)>,
}

/// Everything the pass learned about one image.
#[derive(Default)]
pub struct Recovered {
    own_code: Vec<(u64, u64)>,
    pub ctor: Option<u64>,
    pub classes: Vec<IokitClass>,
    pub meta_vtables: BTreeMap<u64, Vec<u64>>,
    pub getters: BTreeMap<u64, String>,
    names: HashMap<u64, String>,
}

fn code_ranges(file: &object::File) -> Vec<(u64, u64)> {
    let mut out: Vec<(u64, u64)> = file
        .sections()
        .filter(|s| {
            s.kind() == SectionKind::Text
                || macho_flags(s) & (S_ATTR_SOME_INSTRUCTIONS | S_ATTR_PURE_INSTRUCTIONS) != 0
        })
        .map(|s| (s.address(), s.address().saturating_add(s.size())))
        .collect();
    out.sort_unstable();
    out
}

fn in_ranges(ranges: &[(u64, u64)], va: u64) -> bool {
    let i = ranges.partition_point(|&(lo, _)| lo <= va);
    i > 0 && va < ranges[i - 1].1
}

fn init_functions(file: &object::File, img: &MachoImage) -> Vec<u64> {
    let base = file.sections().find(|s| s.segment_name().ok().flatten() == Some("__TEXT")).map(|s| s.address());
    let mut out = Vec::new();
    for sec in file.sections() {
        let (addr, size) = (sec.address(), sec.size());
        match macho_flags(&sec) & 0xff {
            S_MOD_INIT_FUNC_POINTERS => {
                out.extend((0..size / 8).filter_map(|k| img.read_ptr(addr + 8 * k)).filter(|&p| p != 0));
            }
            S_INIT_FUNC_OFFSETS => {
                let Some(base) = base else { continue };
                out.extend((0..size / 4).filter_map(|k| img.read_u32(addr + 4 * k)).map(|o| base + o as u64));
            }
            _ => {}
        }
    }
    out
}

/// A class name as the constructor receives it: a short C identifier.
fn class_name(img: &MachoImage, va: u64) -> Option<String> {
    let s = img.read_cstr(va)?;
    let ok = (2..=0x60).contains(&s.len())
        && s.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    ok.then_some(s)
}

/// Demangled names of the image's own defined symbols, by address.
fn symbol_names(file: &object::File) -> HashMap<u64, String> {
    let mut out = HashMap::new();
    for sym in file.symbols() {
        if sym.is_undefined() || sym.address() == 0 {
            continue;
        }
        let Ok(raw) = sym.name() else { continue };
        let name = crate::demangle::demangle_name(raw)
            .unwrap_or_else(|| raw.strip_prefix('_').unwrap_or(raw).to_string());
        out.entry(sym.address()).or_insert(name);
    }
    out
}

/// Recover the classes of one image.
pub fn recover(file: &object::File, img: &MachoImage) -> Recovered {
    recover_with(file, img, symbol_names(file))
}

fn recover_with(file: &object::File, img: &MachoImage, names: HashMap<u64, String>) -> Recovered {
    let mut rec = Recovered { names, own_code: code_ranges(file), ..Recovered::default() };
    let inits = init_functions(file, img);
    let traces: Vec<Vec<Event>> = inits.iter().map(|&f| a64::trace(img, f, INIT_BUDGET, None)).collect();

    let mut votes: BTreeMap<u64, usize> = BTreeMap::new();
    for ev in traces.iter().flatten() {
        if let Event::Call(c) = ev {
            if let [Some(_), Some(name), _, Some(size)] = c.args {
                if (1..=0x10_0000).contains(&size) && class_name(img, name).is_some() {
                    *votes.entry(c.target).or_default() += 1;
                }
            }
        }
    }
    let Some((&ctor, _)) = votes.iter().max_by_key(|&(&t, &n)| (n, std::cmp::Reverse(t))) else {
        return rec;
    };
    rec.ctor = Some(ctor);
    let traces: Vec<Vec<Event>> = inits.iter().map(|&f| a64::trace(img, f, INIT_BUDGET, Some(ctor))).collect();

    let mut seen = std::collections::HashSet::new();
    for events in &traces {
        for (i, ev) in events.iter().enumerate() {
            let Event::Call(c) = ev else { continue };
            let (Some(meta), Some(name_ptr), Some(size)) = (c.args[0], c.args[1], c.args[3]) else { continue };
            if c.target != ctor || !seen.insert(meta) {
                continue;
            }
            let Some(name) = class_name(img, name_ptr) else { continue };
            let meta_vtable = events[i + 1..]
                .iter()
                .take_while(|e| !matches!(e, Event::Call(n) if n.target == ctor))
                .find_map(|e| match e {
                    Event::Store(s) if s.addr == meta => Some(s.value),
                    _ => None,
                });
            rec.classes.push(IokitClass {
                name,
                meta,
                super_meta: c.args[2].filter(|&s| s != 0),
                size: size & 0xffff_ffff,
                meta_vtable,
                vtable: None,
            });
        }
    }
    rec.find_vtables(file, img);
    rec
}

impl Recovered {
    fn class_by_meta(&self) -> HashMap<u64, usize> {
        self.classes.iter().enumerate().map(|(i, c)| (c.meta, i)).collect()
    }

    fn find_vtables(&mut self, file: &object::File, img: &MachoImage) {
        let code = Code::new(file);
        let by_meta = self.class_by_meta();
        let mut returned: HashMap<u64, Option<u64>> = HashMap::new();
        for c in &self.classes {
            let Some(slot0) = c.meta_vtable else { continue };
            let slots = pointer_run(img, &code, slot0);
            if !slots.is_empty() {
                self.meta_vtables.insert(slot0, slots);
            }
        }
        for sec in file.sections() {
            if sec.kind() == SectionKind::Text || macho_flags(&sec) & 0xff != 0 || sec.size() < 24 {
                continue;
            }
            if !matches!(sec.kind(), SectionKind::Data | SectionKind::ReadOnlyData | SectionKind::Unknown) {
                continue;
            }
            let (lo, hi) = (sec.address(), sec.address() + sec.size());
            let mut va = lo + (8 - lo % 8) % 8;
            while va + 8 <= hi {
                if !(img.read_ptr(va) == Some(0)
                    && img.read_ptr(va + 8) == Some(0)
                    && img.read_ptr(va + 16).is_some_and(|p| code.slot(p)))
                {
                    va += 8;
                    continue;
                }
                let slot0 = va + 16;
                let slots = pointer_run(img, &code, slot0);
                va = slot0 + 8 * slots.len() as u64;
                if self.meta_vtables.contains_key(&slot0) {
                    continue;
                }
                let owner = slots.iter().take(GET_META_CLASS_SLOTS).find_map(|&f| {
                    let meta = *returned.entry(f).or_insert_with(|| a64::returned_constant(img, f, GET_META_CLASS_LIMIT));
                    meta.and_then(|m| by_meta.get(&m).map(|&i| (i, f)))
                });
                if let Some((i, getter)) = owner {
                    if self.classes[i].vtable.is_none() {
                        self.getters.insert(getter, self.classes[i].name.clone());
                        self.classes[i].vtable = Some((slot0, slots));
                    }
                }
            }
        }
    }

    /// Proposed function names: each address with exactly one proposal wins.
    fn function_names(&self) -> BTreeMap<u64, String> {
        let mut proposals: BTreeMap<u64, Vec<String>> = BTreeMap::new();
        let mut propose = |addr: u64, name: String| proposals.entry(addr).or_default().push(name);
        if let Some(ctor) = self.ctor {
            propose(ctor, "OSMetaClass::OSMetaClass".to_string());
        }
        for (&f, class) in &self.getters {
            propose(f, format!("{class}::getMetaClass"));
        }
        let alloc_slot = self.meta_alloc_slot();
        for c in &self.classes {
            let Some(slots) = c.meta_vtable.and_then(|v| self.meta_vtables.get(&v)) else { continue };
            for (j, &f) in slots.iter().enumerate() {
                if j < 2 {
                    propose(f, format!("{}::MetaClass::~MetaClass", c.name));
                } else if Some(j) == alloc_slot {
                    propose(f, format!("{}::MetaClass::alloc", c.name));
                }
            }
        }

        for (f, name) in self.vtable_method_names() {
            propose(f, name);
        }
        proposals
            .into_iter()
            .filter(|(addr, _)| in_ranges(&self.own_code, *addr) && !self.names.contains_key(addr))
            .filter_map(|(addr, mut names)| {
                names.sort();
                names.dedup();
                (names.len() == 1).then(|| (addr, names.pop().unwrap()))
            })
            .collect()
    }

    /// `i` and its superclasses that are classes of this image, nearest first.
    fn ancestors(&self, i: usize, by_meta: &HashMap<u64, usize>) -> Vec<usize> {
        let mut chain = vec![i];
        while let Some(&p) = self.classes[*chain.last().unwrap()].super_meta.and_then(|s| by_meta.get(&s)) {
            if chain.contains(&p) {
                break;
            }
            chain.push(p);
        }
        chain
    }

    /// A name for every unnamed function of this image's code that sits in a
    /// class vtable. The owner is the nearest class that is an ancestor of every
    /// class whose vtable holds the function; the method is the symbol name of
    /// that slot in the owner's nearest ancestor that has one, else
    /// `vfn_<offset>`. A function seen at two different slots (the pure-virtual
    /// trap, folded stubs) is left alone.
    fn vtable_method_names(&self) -> Vec<(u64, String)> {
        let by_meta = self.class_by_meta();
        let mut users: BTreeMap<u64, Vec<(usize, usize)>> = BTreeMap::new();
        for (i, c) in self.classes.iter().enumerate() {
            for (j, &f) in c.vtable.iter().flat_map(|(_, s)| s.iter()).enumerate() {
                users.entry(f).or_default().push((i, j));
            }
        }
        let chains: Vec<Vec<usize>> = (0..self.classes.len()).map(|i| self.ancestors(i, &by_meta)).collect();
        let mut out = Vec::new();
        for (f, uses) in users {
            if self.names.contains_key(&f) || !in_ranges(&self.own_code, f) || self.getters.contains_key(&f) {
                continue;
            }
            let j = uses[0].1;
            if uses.iter().any(|&(_, k)| k != j) {
                continue;
            }
            let Some(&owner) = chains[uses[0].0].iter().find(|a| uses.iter().all(|&(i, _)| chains[i].contains(a))) else {
                continue;
            };
            let class = &self.classes[owner].name;
            let name = if j < 2 {
                format!("{class}::~{class}")
            } else {
                let method = chains[owner].iter().find_map(|&a| {
                    let g = *self.classes[a].vtable.as_ref()?.1.get(j)?;
                    let n = self.names.get(&g)?;
                    n.rsplit("::").next().filter(|m| !m.starts_with('~')).map(str::to_string)
                });
                match method {
                    Some(m) => format!("{class}::{m}"),
                    None => format!("{class}::vfn_{:#x}", 8 * j),
                }
            };
            out.push((f, name));
        }
        out
    }

    /// The `MetaClass::alloc` slot: the one past the destructors whose
    /// function differs between the most metaclass vtables.
    fn meta_alloc_slot(&self) -> Option<usize> {
        let longest = self.meta_vtables.values().map(Vec::len).max()?;
        (2..longest)
            .map(|j| {
                let mut fs: Vec<u64> = self.meta_vtables.values().filter_map(|v| v.get(j).copied()).collect();
                fs.sort_unstable();
                fs.dedup();
                (fs.len(), std::cmp::Reverse(j))
            })
            .max()
            .filter(|&(n, _)| n > 1 || self.meta_vtables.len() == 1)
            .map(|(_, std::cmp::Reverse(j))| j)
    }

    /// The size of one `OSMetaClass` object: the smallest gap between two of
    /// this image's metaclass objects (the compiler lays them out back to back).
    fn meta_size(&self) -> u32 {
        let mut metas: Vec<u64> = self.classes.iter().map(|c| c.meta).collect();
        metas.sort_unstable();
        metas.windows(2).map(|w| w[1] - w[0]).filter(|&d| d > 0).min().map_or(8, |d| d.min(0x100) as u32)
    }

    /// Every name this image's symbols or the recovery give an address.
    pub(super) fn known_names(&self) -> HashMap<u64, String> {
        let mut out = self.names.clone();
        for (addr, name) in self.function_names() {
            out.entry(addr).or_insert(name);
        }
        for c in &self.classes {
            out.entry(c.meta).or_insert_with(|| format!("{}::gMetaClass", c.name));
        }
        out
    }

    fn emit(&self, out: &mut AnalysisOutput) {
        let meta_size = self.meta_size();
        let mut data = |addr: u64, name: String, size: u32| {
            if !self.names.contains_key(&addr) {
                out.data_objects.push(DataObjectFact { addr, name, size });
            }
        };
        for c in &self.classes {
            data(c.meta, format!("{}::gMetaClass", c.name), meta_size);
            if let Some((slot0, slots)) = &c.vtable {
                data(slot0 - 16, format!("vtable for {}", c.name), 16 + 8 * slots.len() as u32);
            }
            if let Some(slots) = c.meta_vtable.and_then(|v| self.meta_vtables.get(&v)) {
                data(c.meta_vtable.unwrap() - 16, format!("vtable for {}::MetaClass", c.name), 16 + 8 * slots.len() as u32);
            }
        }
        let names = self.function_names();
        let mut entries: Vec<u64> = self
            .classes
            .iter()
            .flat_map(|c| c.vtable.iter().flat_map(|(_, s)| s.iter().copied()))
            .chain(self.meta_vtables.values().flatten().copied())
            .chain(names.keys().copied())
            .collect();
        entries.retain(|&f| in_ranges(&self.own_code, f));
        entries.sort_unstable();
        entries.dedup();
        out.entries.extend(entries);
        out.symbols.extend(names.iter().map(|(&addr, name)| SymFact { addr, name: name.clone(), kind: SymKind::Function }));
        out.fid_names.extend(names.into_iter().map(|(addr, name)| FidMatch { addr, name, by_signature: false }));
    }
}

/// Where a vtable slot may point: this image's code, or (for a kext cut out of
/// a collection) an aligned address outside every section of the image but in
/// the same 4 GiB region, where the kernel and the other kexts live.
struct Code {
    own: Vec<(u64, u64)>,
    mapped: Vec<(u64, u64)>,
    region: Option<u64>,
}

impl Code {
    fn new(file: &object::File) -> Self {
        let own = code_ranges(file);
        let mut mapped: Vec<(u64, u64)> =
            file.sections().map(|s| (s.address(), s.address().saturating_add(s.size()))).collect();
        mapped.sort_unstable();
        let region = own.first().map(|&(lo, _)| lo >> 32);
        Code { own, mapped, region }
    }

    fn own(&self, p: u64) -> bool {
        in_ranges(&self.own, p)
    }

    fn slot(&self, p: u64) -> bool {
        self.own(p) || (p & 3 == 0 && Some(p >> 32) == self.region && !in_ranges(&self.mapped, p))
    }
}

/// The run of code pointers starting at `slot0`.
fn pointer_run(img: &MachoImage, code: &Code, slot0: u64) -> Vec<u64> {
    let mut slots = Vec::new();
    let mut va = slot0;
    while let Some(p) = img.read_ptr(va).filter(|&p| code.slot(p)) {
        slots.push(p);
        va += 8;
        if slots.len() > 0x1000 {
            break;
        }
    }
    slots
}

#[cfg(test)]
mod tests;
