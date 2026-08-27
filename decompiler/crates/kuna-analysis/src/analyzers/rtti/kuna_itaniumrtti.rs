//! (kuna, NOVEL) S1 Itanium (GCC/Clang) RTTI + vtable recovery — the capability
//! **Ghidra does not have at all**.
//!
//! Ghidra's `RttiAnalyzer` is a *Microsoft*-PE analyzer; on a GCC/Clang C++ binary
//! nothing in its auto-analysis reads the Itanium type-information graph, and its
//! GCC class recovery is script-tier (`RecoverClassesFromRTTIScript` /
//! `RTTIGccClassRecoverer`), never run by default. Measured on a stripped
//! `g++ -O0` probe, Ghidra resolves the `Shape` vtable to `DAT_00107a80`. IDA Pro
//! *does* reconstruct `_ZTV6Widget` / `_ZTI6Widget` / `_ZTS8Drawable` from
//! `.rodata` + `.data.rel.ro` alone, and this pass is the kuna analog of that
//! behaviour, built against the Itanium C++ ABI rather than a port of either tool.
//!
//! # The graph (Itanium C++ ABI §2.9.5, §2.5.2)
//!
//! A polymorphic class `C` compiled by GCC/Clang emits three linked objects:
//!
//! * `_ZTS<C>` — the **type-name string**, the bare `<mangled-name>` component
//!   (`6Widget`, `N7leveldb3EnvE`), NUL-terminated, in `.rodata`.
//! * `_ZTI<C>` — the **typeinfo object** in `.data.rel.ro`:
//!   `[vptr][name ptr]` for a base-less class, plus `[base typeinfo ptr]` for
//!   single inheritance, or `[flags u32][base_count u32][(base ptr, offset_flags)…]`
//!   for the multiple/virtual-inheritance case. The `vptr` names the flavour: it
//!   points 2 words into `__cxxabiv1::__class_type_info`'s /
//!   `__si_class_type_info`'s / `__vmi_class_type_info`'s vtable.
//! * `_ZTV<C>` — the **vtable object**: one *sub-vtable* per polymorphic base
//!   subobject, each `[offset-to-top (signed)][typeinfo ptr][slot…]`. The value an
//!   object's vptr holds is the address of the first *slot*, i.e. two words in.
//!
//! # Why this is exact rather than heuristic — the relocation anchor
//!
//! The three `__cxxabiv1` typeinfo vtables live in **libstdc++**, so on any
//! dynamically linked C++ image every `_ZTI…` object's leading `vptr` word is an
//! `UND` dynamic relocation naming one of them, with addend `2 * ptr_size`. That
//! relocation's `r_offset` **is** the typeinfo object's address and its symbol
//! **is** the flavour — both survive `strip --strip-all` untouched, because
//! `.rela.dyn` is a loader input, not debug information. So discovery here is a
//! table lookup, not a byte-pattern scan: [`ItaniumRttiPass`] never guesses which
//! bytes are a typeinfo, which is the structural difference from the MSVC sibling
//! pass ([`super::RttiPass`], which byte-searches `.?A` strings and `ref − 12`).
//! Where a `.symtab`/`.dynsym` survives, defined `_ZTI…` symbols are taken as a
//! second discovery source and their flavour is sniffed from the object's shape.
//!
//! # From typeinfo to vtable
//!
//! Every sub-vtable's second header word points at its most-derived class's
//! typeinfo, so one scan for pointer slots holding a discovered typeinfo address
//! yields every sub-vtable in the image: the slot at `s` means a sub-vtable header
//! at `s − ptr` whose first virtual slot is `s + ptr`. Two exact ABI constraints
//! reject the coincidental hits (chiefly the *base-class* pointers inside other
//! typeinfo objects, which also hold a typeinfo address): **offset-to-top is always
//! `<= 0`** (0 for the primary sub-vtable, negative for a secondary), and a real
//! sub-vtable has **at least one slot pointing into an executable section**.
//!
//! A slot whose file word is 0 but which carries a dynamic relocation is an
//! *imported* virtual method (`__cxa_pure_virtual`, a base method defined in
//! another image): the walk steps over it instead of terminating, so an abstract
//! interface's vtable keeps its true extent.
//!
//! # What it emits
//!
//! Per recovered class: `<C>::typeinfo`, `<C>::typeinfo_name`, `<C>::vtable`, and
//! for a secondary sub-vtable — named from the inheritance displacement the vmi/si
//! base list carries — `<C>::vtable_for_<Base>`; per virtual slot a **function**
//! symbol `<C>::vtable_<i>`, and the slot array marked read-only. An inherited slot
//! claimed by several classes' vtables is attributed to the **defining base**, not
//! to whichever vtable the scan reached first, using the recovered inheritance
//! graph — so `Shape::perimeter`, present in `Shape`'s, `Circle`'s and `Square`'s
//! tables, is named once, for `Shape`.
//!
//! # Gating + parity safety
//!
//! Default-**OFF** (`--option itaniumrtti on`), and registered only on an ELF image
//! ([`crate::passes::passes_for`]). The XML datatest path never loads an ELF object
//! and never runs this pass, so the datatest / stage-model parity oracles are
//! structurally untouched. Defensive by construction (the [`AnalysisPass`]
//! never-fail contract): a non-ELF input, a C binary, a `-fno-rtti` build, or any
//! malformed structure yields an **empty** output — never a panic.

use std::collections::{HashMap, HashSet};

use object::read::{Object, ObjectSymbol, ObjectSymbolTable};
use object::{BinaryFormat, RelocationTarget};

use crate::demangle::demangle_typeinfo_name;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, SymFact, SymKind};

use super::models::ImageBytes;
use super::refkind::{End, RefKind};
use super::vftable::TextRanges;

/// The `__cxxabiv1` typeinfo-vtable symbol names, and the class-typeinfo flavour
/// each one identifies. A typeinfo object whose `vptr` relocation names one of
/// these IS that flavour — the ABI leaves no ambiguity.
const ABI_TYPEINFO_VTABLES: [(&str, Flavor); 3] = [
    ("_ZTVN10__cxxabiv117__class_type_infoE", Flavor::Class),
    ("_ZTVN10__cxxabiv120__si_class_type_infoE", Flavor::SiClass),
    ("_ZTVN10__cxxabiv121__vmi_class_type_infoE", Flavor::VmiClass),
];

/// The Itanium typeinfo-name symbol prefix (`_ZTI<mangled-name>`), the second
/// discovery source: a defined `_ZTI…` OBJECT symbol names a typeinfo object.
const ZTI_PREFIX: &str = "_ZTI";

/// Upper bound on virtual-method slots walked from one sub-vtable base — far beyond
/// any real class's vtable, a guard against a malformed header with no terminator.
const MAX_SLOTS: usize = 4096;

/// Upper bound on the base count a `__vmi_class_type_info` may declare before the
/// object is rejected as malformed (the ABI permits more; no real class has this
/// many direct bases, and the bound keeps a garbage `u32` from driving the walk).
const MAX_BASES: u32 = 64;

/// The largest plausible `offset-to-top`, in bytes: a sub-object displacement is a
/// class-layout offset, never an image-sized number. Bounds the sanity gate.
const MAX_SUBOBJECT_OFFSET: i64 = 1 << 24;

/// Which `__cxxabiv1` type-information class a typeinfo object is an instance of.
/// Fixes the object's byte layout past the `[vptr][name ptr]` prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flavor {
    /// `__class_type_info` — no base classes; the object is exactly two words.
    Class,
    /// `__si_class_type_info` — exactly one public, non-virtual, offset-0 base,
    /// whose typeinfo pointer follows the name pointer.
    SiClass,
    /// `__vmi_class_type_info` — everything else (multiple / virtual / non-public
    /// inheritance): `[flags u32][base_count u32][(base ptr, offset_flags)…]`.
    VmiClass,
}

impl Flavor {
    /// The flavour a `__cxxabiv1` typeinfo-vtable symbol name identifies, or `None`
    /// if `name` is not one of the three class flavours (a `__fundamental_`,
    /// `__pointer_` or `__enum_type_info` names no class and is skipped).
    fn from_abi_vtable(name: &str) -> Option<Flavor> {
        // A versioned dynamic symbol can carry an `@VER` suffix in some readers.
        let bare = name.split('@').next().unwrap_or(name);
        ABI_TYPEINFO_VTABLES.iter().find(|(s, _)| *s == bare).map(|(_, f)| *f)
    }
}

/// A decoded Itanium typeinfo object.
#[derive(Clone, Debug)]
struct TypeInfo {
    /// VMA of the typeinfo object (`_ZTI<C>`).
    addr: u64,
    /// VMA of its NUL-terminated type-name string (`_ZTS<C>`).
    name_addr: u64,
    /// The demangled qualified class name (`leveldb::Env`, `Widget`).
    class: String,
    /// Direct base classes as `(base typeinfo VMA, byte displacement of the base
    /// subobject within this class)`. Empty for [`Flavor::Class`]. The displacement
    /// is the Itanium `offset_flags >> 8` for a vmi base and 0 for an si base —
    /// the datum the MSVC sibling pass discards with its `pmd` fields.
    bases: Vec<(u64, i64)>,
    /// Byte extent of the object, used to reject a pointer *inside* a typeinfo from
    /// being read as a vtable header.
    size: u64,
}

/// A recovered sub-vtable: one polymorphic base subobject's slice of a `_ZTV<C>`
/// object.
#[derive(Clone, Debug)]
struct SubVtable {
    /// VMA of the sub-vtable's `offset-to-top` word — what `_ZTV<C>` names for the
    /// primary, and what a secondary base subobject's vptr is derived from.
    header: u64,
    /// VMA of the first virtual slot (the value an object's vptr holds).
    first_slot: u64,
    /// The `offset-to-top` displacement: 0 for the primary sub-vtable, negative for
    /// a secondary (the byte distance back to the most-derived object's start).
    offset_to_top: i64,
    /// The typeinfo (hence the most-derived class) this sub-vtable belongs to.
    typeinfo: u64,
    /// The virtual-method target VAs, in slot order. A slot filled by the dynamic
    /// loader (an imported method) is recorded as `None` so slot INDICES stay
    /// faithful to the ABI while carrying no name.
    slots: Vec<Option<u64>>,
}

/// The Itanium (GCC/Clang) RTTI + vtable recovery pass (gate id `itaniumrtti`).
/// Default-OFF, ELF path only.
#[derive(Default)]
pub struct ItaniumRttiPass;

impl AnalysisPass for ItaniumRttiPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "itaniumrtti"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();

        // ELF gate (belt-and-suspenders on top of the passes_for registration gate):
        // the MSVC ABI owns PE, and Mach-O's chained-fixup relocation model does not
        // expose the anchor this pass reads.
        if ctx.file.format() != BinaryFormat::Elf {
            return out;
        }

        let rk = RefKind::new(
            ctx.file.is_64(),
            0,
            if ctx.file.is_little_endian() { End::Little } else { End::Big },
        );
        // Discovery first, and bail before building anything else: a plain-C binary
        // and a `-fno-rtti` C++ build both have no typeinfo, and that is the common
        // case this pass must stay nearly free on.
        let candidates = discover_typeinfos(ctx.file, rk.ptr_size() as u64);
        if candidates.is_empty() {
            return out;
        }
        let img = ImageBytes::new(ctx.file);
        let typeinfos = decode_typeinfos(&img, &rk, &candidates);
        if typeinfos.is_empty() {
            return out;
        }

        let text = TextRanges::new(ctx.file);
        let relocated = relocated_slots(ctx.file);
        let subs = recover_subvtables(&img, &rk, &text, &relocated, &typeinfos);
        emit(&typeinfos, &subs, &rk, &mut out);
        out
    }
}

/// The address of every slot a dynamic relocation writes. A vtable slot whose file
/// word is 0 is an **imported** virtual method when it appears here (the loader
/// fills it), so the walk steps over it rather than treating the 0 as the table's
/// terminator.
fn relocated_slots(file: &object::File) -> HashSet<u64> {
    let mut out = HashSet::new();
    if let Some(relocs) = file.dynamic_relocations() {
        for (offset, _) in relocs {
            out.insert(offset);
        }
    }
    out
}

/// Discover candidate typeinfo objects as `(addr, known flavour)`.
///
/// Anchor 1 — the exact one, and the only one that survives `strip` on a shared
/// object: a dynamic relocation whose target symbol is a `__cxxabiv1` typeinfo
/// vtable and whose addend is `2 * ptr` (the typeinfo's `vptr` points past the
/// vtable's two header words). The relocation's offset IS the typeinfo address and
/// its symbol IS the flavour.
///
/// Anchor 2 — a defined `_ZTI…` OBJECT symbol, for the unstripped / statically
/// linked case where anchor 1 has nothing to name. Flavour unknown at this point;
/// [`decode_typeinfos`] sniffs it from the object's shape.
fn discover_typeinfos(file: &object::File, ptr: u64) -> Vec<(u64, Option<Flavor>)> {
    let mut found: HashMap<u64, Option<Flavor>> = HashMap::new();

    if let (Some(relocs), Some(dynsyms)) = (file.dynamic_relocations(), file.dynamic_symbol_table())
    {
        for (offset, rel) in relocs {
            let RelocationTarget::Symbol(idx) = rel.target() else {
                continue;
            };
            let Ok(sym) = dynsyms.symbol_by_index(idx) else {
                continue;
            };
            let Ok(name) = sym.name() else {
                continue;
            };
            let Some(flavor) = Flavor::from_abi_vtable(name) else {
                continue;
            };
            // A REL (implicit-addend) target stores its addend in the section bytes;
            // accept it unchecked rather than mis-reading the explicit field as 0.
            if !rel.has_implicit_addend() && rel.addend() != 2 * ptr as i64 {
                continue;
            }
            found.insert(offset, Some(flavor));
        }
    }

    for sym in file.symbols().chain(file.dynamic_symbols()) {
        let Ok(name) = sym.name() else {
            continue;
        };
        // `_ZTIN10__cxxabiv1…` is the ABI's own typeinfo, not a user class.
        if !name.starts_with(ZTI_PREFIX) || name.contains("__cxxabiv1") {
            continue;
        }
        let addr = sym.address();
        if addr != 0 {
            found.entry(addr).or_insert(None);
        }
    }

    let mut out: Vec<(u64, Option<Flavor>)> = found.into_iter().collect();
    out.sort_by_key(|&(a, _)| a);
    out
}

/// Decode each candidate into a [`TypeInfo`], keyed by address.
///
/// Two sweeps: the first accepts a candidate whose name pointer resolves to a
/// mangled type name that demangles (the gate that rejects a coincidental address),
/// the second reads the base list — which needs the accepted address set, both to
/// sniff an unknown flavour and to validate each base pointer.
fn decode_typeinfos(
    img: &ImageBytes,
    rk: &RefKind,
    candidates: &[(u64, Option<Flavor>)],
) -> Vec<TypeInfo> {
    /// Longest type-name string accepted (a deeply templated name is long).
    const MAX_NAME: usize = 4096;

    let ptr = rk.ptr_size() as u64;
    let mut named: Vec<(u64, Option<Flavor>, u64, String)> = Vec::new();
    for &(addr, flavor) in candidates {
        let Some(name_addr) = read_ptr(img, rk, addr + ptr) else {
            continue;
        };
        let Some(mangled) = img.read_cstr(name_addr, MAX_NAME) else {
            continue;
        };
        let Some(class) = demangle_typeinfo_name(&mangled).map(|c| sanitize_class_name(&c))
        else {
            continue;
        };
        if class.is_empty() {
            continue;
        }
        named.push((addr, flavor, name_addr, class));
    }
    let accepted: HashSet<u64> = named.iter().map(|&(a, ..)| a).collect();

    let mut out = Vec::new();
    for (addr, flavor, name_addr, class) in named {
        let flavor = flavor.unwrap_or_else(|| sniff_flavor(img, rk, addr, &accepted));
        let (bases, size) = read_bases(img, rk, addr, flavor, &accepted);
        out.push(TypeInfo { addr, name_addr, class, bases, size });
    }
    out
}

/// Decide the flavour of a typeinfo object discovered without a relocation to name
/// it, from the object's own shape: a word past the name pointer that resolves to
/// another accepted typeinfo means single inheritance; an in-range `base_count`
/// whose whole base array resolves means the vmi form; otherwise the two-word form.
fn sniff_flavor(img: &ImageBytes, rk: &RefKind, addr: u64, accepted: &HashSet<u64>) -> Flavor {
    let ptr = rk.ptr_size() as u64;
    if let Some(base) = read_ptr(img, rk, addr + 2 * ptr) {
        if accepted.contains(&base) {
            return Flavor::SiClass;
        }
    }
    if let Some(count) = read_u32(img, rk, addr + 2 * ptr + 4) {
        if (1..=MAX_BASES).contains(&count) {
            let all_resolve = (0..count as u64).all(|i| {
                read_ptr(img, rk, addr + 2 * ptr + 8 + i * 2 * ptr)
                    .is_some_and(|b| accepted.contains(&b))
            });
            if all_resolve {
                return Flavor::VmiClass;
            }
        }
    }
    Flavor::Class
}

/// Read a typeinfo's direct base list and the object's byte extent, per flavour.
/// A base pointer that does not resolve to an accepted typeinfo is dropped rather
/// than trusted.
fn read_bases(
    img: &ImageBytes,
    rk: &RefKind,
    addr: u64,
    flavor: Flavor,
    accepted: &HashSet<u64>,
) -> (Vec<(u64, i64)>, u64) {
    let ptr = rk.ptr_size() as u64;
    match flavor {
        Flavor::Class => (Vec::new(), 2 * ptr),
        Flavor::SiClass => {
            let base = read_ptr(img, rk, addr + 2 * ptr).filter(|b| accepted.contains(b));
            // An si base is by definition at displacement 0.
            (base.map(|b| vec![(b, 0)]).unwrap_or_default(), 3 * ptr)
        }
        Flavor::VmiClass => {
            let Some(count) = read_u32(img, rk, addr + 2 * ptr + 4) else {
                return (Vec::new(), 2 * ptr);
            };
            let count = count.min(MAX_BASES) as u64;
            let mut bases = Vec::new();
            for i in 0..count {
                let entry = addr + 2 * ptr + 8 + i * 2 * ptr;
                let Some(bti) = read_ptr(img, rk, entry) else {
                    break;
                };
                let Some(flags) = read_ptr(img, rk, entry + ptr) else {
                    break;
                };
                if !accepted.contains(&bti) {
                    continue;
                }
                // Itanium `__base_class_type_info::__offset_flags`: the low 8 bits
                // are access/virtual flags, the rest a SIGNED byte displacement.
                bases.push((bti, (flags as i64) >> 8));
            }
            (bases, 2 * ptr + 8 + count * 2 * ptr)
        }
    }
}

/// Recover every sub-vtable in the image: scan each loaded section once for a
/// pointer-width slot holding a discovered typeinfo address (a sub-vtable's second
/// header word), then validate and walk each candidate.
///
/// A single scan rather than one [`ImageBytes::find_ptrs`] per class: the per-class
/// form is `O(classes × image)`, which on a real C++ library with hundreds of
/// polymorphic classes is the difference between milliseconds and seconds.
fn recover_subvtables(
    img: &ImageBytes,
    rk: &RefKind,
    text: &TextRanges,
    relocated: &HashSet<u64>,
    typeinfos: &[TypeInfo],
) -> Vec<SubVtable> {
    let ptr = rk.ptr_size() as u64;
    let by_addr: HashMap<u64, &TypeInfo> = typeinfos.iter().map(|t| (t.addr, t)).collect();
    let mut out = Vec::new();

    for slot in img.scan_ptr_slots(rk, &by_addr.keys().copied().collect()) {
        let (ti_slot, ti_addr) = slot;
        // The header word before the typeinfo pointer is `offset-to-top`.
        let Some(header) = ti_slot.checked_sub(ptr) else {
            continue;
        };
        // Reject a pointer that lies INSIDE a typeinfo object: a base-class pointer
        // in an si/vmi typeinfo also holds a typeinfo address, but is not a vtable.
        if typeinfos.iter().any(|t| header >= t.addr && header < t.addr + t.size) {
            continue;
        }
        let Some(raw_top) = read_ptr(img, rk, header) else {
            continue;
        };
        let offset_to_top = sign_extend(raw_top, rk);
        // The ABI's exact constraint: offset-to-top is 0 for the primary sub-vtable
        // and NEGATIVE for a secondary — never positive, never image-sized.
        if offset_to_top > 0 || offset_to_top < -MAX_SUBOBJECT_OFFSET {
            continue;
        }
        let first_slot = ti_slot + ptr;
        let slots = walk_slots(img, rk, text, relocated, first_slot);
        if !slots.iter().any(|s| s.is_some()) {
            continue;
        }
        out.push(SubVtable { header, first_slot, offset_to_top, typeinfo: ti_addr, slots });
    }
    out.sort_by_key(|s| s.header);
    out
}

/// Walk a sub-vtable's slots from `first_slot`, bounded at the first word that is
/// neither a code pointer nor a loader-filled import slot.
///
/// A slot reading 0 with a dynamic relocation covering it is an imported virtual
/// method — the file simply does not carry its value. Recording it as `None` keeps
/// the slot INDEX faithful (so `<C>::vtable_<i>` matches the ABI's numbering) and
/// lets the walk continue to the locally-defined methods after it, which is what
/// gives an abstract interface's vtable its true extent.
fn walk_slots(
    img: &ImageBytes,
    rk: &RefKind,
    text: &TextRanges,
    relocated: &HashSet<u64>,
    first_slot: u64,
) -> Vec<Option<u64>> {
    let ptr = rk.ptr_size() as u64;
    let mut slots: Vec<Option<u64>> = Vec::new();
    let mut at = first_slot;
    for _ in 0..MAX_SLOTS {
        let Some(word) = read_ptr(img, rk, at) else {
            break;
        };
        if text.contains(word) {
            slots.push(Some(word));
        } else if word == 0 && relocated.contains(&at) {
            slots.push(None);
        } else {
            break;
        }
        at += ptr;
    }
    // Trailing import slots carry no information and may be the next object's
    // relocated header; drop them so the read-only range stops at real content.
    while matches!(slots.last(), Some(None)) {
        slots.pop();
    }
    slots
}

/// Emit the recovered facts: the typeinfo/name/vtable data labels, one function
/// symbol per virtual slot, and the slot arrays as read-only ranges.
fn emit(typeinfos: &[TypeInfo], subs: &[SubVtable], rk: &RefKind, out: &mut AnalysisOutput) {
    let ptr = rk.ptr_size() as u64;
    let by_addr: HashMap<u64, &TypeInfo> = typeinfos.iter().map(|t| (t.addr, t)).collect();

    for ti in typeinfos {
        out.symbols.push(SymFact {
            addr: ti.addr,
            name: data_label(&ti.class, "typeinfo"),
            kind: SymKind::Data,
        });
        out.symbols.push(SymFact {
            addr: ti.name_addr,
            name: data_label(&ti.class, "typeinfo_name"),
            kind: SymKind::Data,
        });
    }

    let mut labelled: Vec<(&SubVtable, &str, String)> = Vec::new();
    for sub in subs {
        let Some(ti) = by_addr.get(&sub.typeinfo) else {
            continue;
        };
        let suffix = subvtable_suffix(ti, sub, &by_addr);
        out.symbols.push(SymFact {
            addr: sub.header,
            name: data_label(&ti.class, &suffix),
            kind: SymKind::Data,
        });
        // The value an object's vptr actually HOLDS is the first slot's address, two
        // words past the header a `_ZTV<C>` symbol names — that is the constant a
        // constructor stores (`*this = 0x7b98`), so it gets its own label.
        out.symbols.push(SymFact {
            addr: sub.first_slot,
            name: data_label(&ti.class, &suffix.replacen("vtable", "vptr", 1)),
            kind: SymKind::Data,
        });
        let end = sub.first_slot + sub.slots.len() as u64 * ptr;
        out.readonly.push((sub.header, end));
        labelled.push((sub, ti.class.as_str(), suffix));
    }

    for (addr, name) in attribute_slots(typeinfos, &labelled) {
        out.symbols.push(SymFact { addr, name, kind: SymKind::Function });
    }
}

/// Compose a **data** symbol name: the class qualification is kept (so a namespaced
/// class still nests under its namespace scope) but the kind suffix is joined with
/// `_`, not `::`.
///
/// kuna's scope splitter splits a symbol name on every `::` and the C printer emits
/// a global by its LEAF name, so `Shape::vptr` and `Circle::vptr` would both render
/// as a bare `vptr` at the vptr store that is this recovery's most visible use site.
/// Joining with `_` puts the class in the leaf — `Shape_vptr`, `leveldb::Env_vtable`
/// → `Env_vtable` — which is unambiguous. Function symbols keep the `::` form: a
/// callee's qualified name IS rendered at the call site (`cppcallnames`, DIV-59).
fn data_label(class: &str, suffix: &str) -> String {
    format!("{class}_{suffix}")
}

/// The kind suffix for one sub-vtable: `vtable` for the primary, and for a secondary
/// the base subobject its `offset-to-top` identifies — `vtable_for_Drawable`, read
/// straight off the recovered inheritance displacements (the datum the MSVC sibling
/// pass discards with its `pmd` fields). A displacement no declared base claims
/// falls back to the raw offset so the label stays unique.
fn subvtable_suffix(
    ti: &TypeInfo,
    sub: &SubVtable,
    by_addr: &HashMap<u64, &TypeInfo>,
) -> String {
    if sub.offset_to_top == 0 {
        return "vtable".to_string();
    }
    let want = -sub.offset_to_top;
    if let Some(base) = ti
        .bases
        .iter()
        .find(|&&(_, off)| off == want)
        .and_then(|&(b, _)| by_addr.get(&b))
    {
        return format!("vtable_for_{}", base_tag(ti, base, by_addr));
    }
    format!("vtable_at_{want:x}")
}

/// The identifier a secondary sub-vtable label uses for its base subobject: the
/// base's innermost class name (`shapes::Drawable` → `Drawable`), which reads far
/// better inside a label than the flattened qualified form — falling back to the
/// flattened qualified name when two of `ti`'s bases share an innermost name, so
/// the label stays unique.
fn base_tag(ti: &TypeInfo, base: &TypeInfo, by_addr: &HashMap<u64, &TypeInfo>) -> String {
    let leaf = |c: &str| c.rsplit("::").next().unwrap_or(c).to_string();
    let tag = leaf(&base.class);
    let clashes = ti
        .bases
        .iter()
        .filter_map(|&(b, _)| by_addr.get(&b))
        .filter(|other| other.addr != base.addr && leaf(&other.class) == tag)
        .count();
    if clashes == 0 {
        tag
    } else {
        base.class.replace("::", "_")
    }
}

/// Name every virtual-method slot `<sub-vtable label>_<slot index>`, attributing an
/// inherited slot to the class that **defines** it rather than to whichever vtable
/// the scan reached first.
///
/// A derived class's vtable repeats its bases' slots verbatim, so one function
/// address is claimed by every class in its inheritance chain. Attributing by scan
/// order would call `Shape::perimeter` — inherited unchanged by `Circle` and
/// `Square` — `Square::vtable_3`. The recovered base graph resolves it: among the
/// claimants, the one whose class is an ancestor of all the others wins; a tie
/// (unrelated classes sharing a thunk) falls back to the lowest typeinfo address so
/// the result is deterministic.
///
/// The label rather than the class name carries the prefix because a
/// multiple-inheritance class has SEVERAL sub-vtables, whose slot indices both start
/// at 0: `Widget::vtable_0` and `Widget::vtable_for_Drawable_0` are different
/// functions (the second is the `this`-adjusting thunk).
fn attribute_slots(
    typeinfos: &[TypeInfo],
    subs: &[(&SubVtable, &str, String)],
) -> Vec<(u64, String)> {
    let by_addr: HashMap<u64, &TypeInfo> = typeinfos.iter().map(|t| (t.addr, t)).collect();
    let ancestors: HashMap<u64, HashSet<u64>> =
        typeinfos.iter().map(|t| (t.addr, ancestors_of(t.addr, &by_addr))).collect();

    // fn addr -> the claims on it, as (typeinfo addr, class, suffix, slot index).
    let mut claims: HashMap<u64, Vec<(u64, &str, &str, usize)>> = HashMap::new();
    for (sub, class, suffix) in subs {
        for (i, slot) in sub.slots.iter().enumerate() {
            if let Some(target) = slot {
                claims
                    .entry(*target)
                    .or_default()
                    .push((sub.typeinfo, class, suffix.as_str(), i));
            }
        }
    }

    let mut out: Vec<(u64, String)> = Vec::new();
    for (target, mut cs) in claims {
        cs.sort();
        cs.dedup();
        // The most-base claimant: its class is an ancestor of every other claimant's.
        let best = cs
            .iter()
            .find(|&&(ti, ..)| {
                cs.iter().all(|&(other, ..)| {
                    other == ti || ancestors.get(&other).is_some_and(|a| a.contains(&ti))
                })
            })
            .copied()
            .or_else(|| cs.first().copied());
        if let Some((_, class, suffix, idx)) = best {
            out.push((target, format!("{class}::{suffix}_{idx}")));
        }
    }
    out.sort();
    out
}

/// Every transitive base of `addr` in the recovered inheritance graph. Cycle-safe
/// (a malformed image could point a base back at its derived class).
fn ancestors_of(addr: u64, by_addr: &HashMap<u64, &TypeInfo>) -> HashSet<u64> {
    let mut seen = HashSet::new();
    let mut stack = vec![addr];
    while let Some(cur) = stack.pop() {
        let Some(t) = by_addr.get(&cur) else {
            continue;
        };
        for &(base, _) in &t.bases {
            if seen.insert(base) {
                stack.push(base);
            }
        }
    }
    seen
}

/// Turn a demangled class name into a `::`-qualified chain of C identifiers, keeping
/// **distinct classes distinct**.
///
/// kuna's scope splitter splits a symbol name on every `::` and each component must
/// be a legal identifier, so the raw demangled text cannot be used as-is. The obvious
/// reduction — the module-wide [`strip_bracket_groups`](crate::demangle) that
/// `demangle_name` applies — is wrong *here*: `Vec<int>` and `Vec<double>` are two
/// classes with two vtables, and collapsing both to `Vec` makes the second class's
/// methods lose the idempotent name race and stay `sub_<addr>`. Measured on a probe
/// with both instantiations: 4 of 14 virtual methods silently unrecovered.
///
/// So the template arguments are *folded into* the identifier rather than deleted.
/// The `::` split is depth-aware, so a separator inside a template argument list is
/// not mistaken for a scope boundary; each component then has every non-identifier
/// character collapsed to a single `_`.
///
/// ```text
/// Vec<int>                          -> Vec_int
/// Vec<double>                       -> Vec_double
/// a::b::c::Deep                     -> a::b::c::Deep
/// (anonymous namespace)::Hidden     -> anonymous_namespace::Hidden
/// std::vector<std::pair<int, int> > -> std::vector_std_pair_int_int
/// ```
///
/// The implementation is shared with the `symbolnamechars` option's `ident` mode
/// ([`kuna_decomp::kuna_symbolnamechars::sanitize_ident_chain`]), which is the
/// same reduction asked of a loader symbol name. It is applied here
/// UNCONDITIONALLY, gate or no gate: this recovery invents the class string
/// itself out of a `_ZTS` type-descriptor, and the string it invents has to be a
/// legal name for a type and a scope before anything downstream sees it.
fn sanitize_class_name(class: &str) -> String {
    kuna_decomp::kuna_symbolnamechars::sanitize_ident_chain(class)
}

/// Read a pointer-width word at `vma`, or `None` if it is not in a loaded section.
fn read_ptr(img: &ImageBytes, rk: &RefKind, vma: u64) -> Option<u64> {
    img.read(vma, rk.ptr_size()).map(|b| rk.read_ptr(b))
}

/// Read a 4-byte word at `vma` in the image's endianness.
fn read_u32(img: &ImageBytes, rk: &RefKind, vma: u64) -> Option<u32> {
    img.read(vma, 4).map(|b| rk.read_u32(b))
}

/// Interpret a pointer-width word as a signed displacement (`offset-to-top`).
fn sign_extend(raw: u64, rk: &RefKind) -> i64 {
    if rk.ptr_size() == 8 {
        raw as i64
    } else {
        raw as u32 as i32 as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_identity() {
        let p = ItaniumRttiPass;
        assert_eq!(p.id(), "itaniumrtti");
        assert_eq!(p.phase(), Phase::P1);
    }

    #[test]
    fn abi_vtable_names_map_to_flavours() {
        assert_eq!(
            Flavor::from_abi_vtable("_ZTVN10__cxxabiv117__class_type_infoE"),
            Some(Flavor::Class)
        );
        assert_eq!(
            Flavor::from_abi_vtable("_ZTVN10__cxxabiv120__si_class_type_infoE@CXXABI_1.3"),
            Some(Flavor::SiClass)
        );
        assert_eq!(
            Flavor::from_abi_vtable("_ZTVN10__cxxabiv121__vmi_class_type_infoE"),
            Some(Flavor::VmiClass)
        );
        // A non-class type-information flavour names no class.
        assert_eq!(Flavor::from_abi_vtable("_ZTVN10__cxxabiv123__fundamental_type_infoE"), None);
        assert_eq!(Flavor::from_abi_vtable("_ZTV5Shape"), None);
    }

    /// A synthetic x86-64 image reproducing the probe's `Shape`/`Circle` shape:
    /// two typeinfos (`5Shape` base-less, `6Circle` si-derived from it) and their
    /// two vtables, each repeating `Shape::perimeter` at slot 1.
    fn synth() -> (Vec<u8>, Vec<u8>, RefKind) {
        let rk = RefKind::new(true, 0, End::Little);
        // .rodata @ 0x5000: the two type-name strings.
        let mut rodata = vec![0u8; 0x40];
        rodata[0x00..0x07].copy_from_slice(b"5Shape\0");
        rodata[0x10..0x18].copy_from_slice(b"6Circle\0");
        // .data.rel.ro @ 0x7000.
        let mut rel = vec![0u8; 0x100];
        let put = |v: &mut [u8], off: usize, x: u64| {
            v[off..off + 8].copy_from_slice(&x.to_le_bytes())
        };
        // Shape vtable object @0x7000: top=0, ti=0x7080, slots 0x1000, 0x1010.
        put(&mut rel, 0x00, 0);
        put(&mut rel, 0x08, 0x7080);
        put(&mut rel, 0x10, 0x1000);
        put(&mut rel, 0x18, 0x1010);
        // Circle vtable object @0x7030: top=0, ti=0x70a0, slots 0x1020, 0x1010.
        put(&mut rel, 0x30, 0);
        put(&mut rel, 0x38, 0x70a0);
        put(&mut rel, 0x40, 0x1020);
        put(&mut rel, 0x48, 0x1010);
        // _ZTI5Shape @0x7080 (__class_type_info): [vptr][name].
        put(&mut rel, 0x80, 0xdead);
        put(&mut rel, 0x88, 0x5000);
        // _ZTI6Circle @0x70a0 (__si_class_type_info): [vptr][name][base].
        put(&mut rel, 0xa0, 0xbeef);
        put(&mut rel, 0xa8, 0x5010);
        put(&mut rel, 0xb0, 0x7080);
        (rodata, rel, rk)
    }

    fn decode_synth() -> (Vec<TypeInfo>, Vec<SubVtable>, RefKind) {
        let (rodata, rel, rk) = synth();
        let img = ImageBytes::from_spans(vec![(0x5000, &rodata), (0x7000, &rel)]);
        let cands =
            vec![(0x7080u64, Some(Flavor::Class)), (0x70a0u64, Some(Flavor::SiClass))];
        let tis = decode_typeinfos(&img, &rk, &cands);
        let text = TextRanges::for_test(vec![(0x1000, 0x2000)]);
        let subs = recover_subvtables(&img, &rk, &text, &HashSet::new(), &tis);
        (tis, subs, rk)
    }

    #[test]
    fn decodes_class_names_and_single_inheritance() {
        let (tis, _, _) = decode_synth();
        let names: Vec<&str> = tis.iter().map(|t| t.class.as_str()).collect();
        assert_eq!(names, vec!["Shape", "Circle"]);
        assert!(tis[0].bases.is_empty());
        assert_eq!(tis[1].bases, vec![(0x7080, 0)]);
    }

    #[test]
    fn recovers_both_vtables_and_rejects_the_base_pointer() {
        let (_, subs, _) = decode_synth();
        // 0x70b0 (Circle's base-class pointer) also holds 0x7080, but it lies inside
        // the typeinfo object and has no code slot — it must not become a vtable.
        let headers: Vec<u64> = subs.iter().map(|s| s.header).collect();
        assert_eq!(headers, vec![0x7000, 0x7030]);
        assert_eq!(subs[0].slots, vec![Some(0x1000), Some(0x1010)]);
        assert_eq!(subs[1].slots, vec![Some(0x1020), Some(0x1010)]);
    }

    #[test]
    fn inherited_slot_is_attributed_to_the_defining_base() {
        let (tis, subs, _) = decode_synth();
        let by_addr: HashMap<u64, &TypeInfo> = tis.iter().map(|t| (t.addr, t)).collect();
        let labelled: Vec<(&SubVtable, &str, String)> = subs
            .iter()
            .map(|s| {
                let ti = by_addr[&s.typeinfo];
                (s, ti.class.as_str(), subvtable_suffix(ti, s, &by_addr))
            })
            .collect();
        let named: HashMap<u64, String> =
            attribute_slots(&tis, &labelled).into_iter().collect();
        assert_eq!(named[&0x1000], "Shape::vtable_0");
        assert_eq!(named[&0x1020], "Circle::vtable_0");
        // 0x1010 is slot 1 of BOTH vtables; Shape defines it, Circle inherits it.
        assert_eq!(named[&0x1010], "Shape::vtable_1");
    }

    /// A secondary sub-vtable takes the NAME of the base subobject its
    /// `offset-to-top` displacement identifies — the multiple-inheritance case the
    /// probe's `Widget : Loggable, Drawable` produces (`Drawable` at +16, so its
    /// sub-vtable's offset-to-top is -16).
    #[test]
    fn secondary_subvtable_is_named_for_its_base() {
        let shape = TypeInfo {
            addr: 0x7080,
            name_addr: 0x5000,
            class: "Drawable".into(),
            bases: vec![],
            size: 16,
        };
        let widget = TypeInfo {
            addr: 0x70c0,
            name_addr: 0x5020,
            class: "Widget".into(),
            bases: vec![(0x7080, 16)],
            size: 56,
        };
        let by_addr: HashMap<u64, &TypeInfo> =
            [(shape.addr, &shape), (widget.addr, &widget)].into_iter().collect();
        let sub = SubVtable {
            header: 0x7000,
            first_slot: 0x7010,
            offset_to_top: 0,
            typeinfo: widget.addr,
            slots: vec![],
        };
        assert_eq!(subvtable_suffix(&widget, &sub, &by_addr), "vtable");
        let secondary = SubVtable { offset_to_top: -16, ..sub.clone() };
        assert_eq!(
            subvtable_suffix(&widget, &secondary, &by_addr),
            "vtable_for_Drawable"
        );
        // A displacement no declared base claims keeps a unique, raw-offset label.
        let orphan = SubVtable { offset_to_top: -32, ..sub };
        assert_eq!(subvtable_suffix(&widget, &orphan, &by_addr), "vtable_at_20");
        // A DATA label puts the class in the leaf (the printer emits a global by its
        // leaf name), while a namespace qualification is preserved as scope.
        assert_eq!(data_label("Widget", "vtable"), "Widget_vtable");
        assert_eq!(data_label("leveldb::Env", "typeinfo"), "leveldb::Env_typeinfo");
    }

    /// Distinct classes must stay distinct after sanitization — the defect an
    /// adversarial two-instantiation probe found (both `Vec<int>` and `Vec<double>`
    /// reducing to `Vec` cost 4 of 14 virtual methods).
    #[test]
    fn class_names_sanitize_to_distinct_identifier_chains() {
        assert_eq!(sanitize_class_name("Shape"), "Shape");
        assert_eq!(sanitize_class_name("a::b::c::Deep"), "a::b::c::Deep");
        assert_eq!(sanitize_class_name("leveldb::Env"), "leveldb::Env");
        // Template arguments are FOLDED IN, never deleted.
        assert_eq!(sanitize_class_name("Vec<int>"), "Vec_int");
        assert_eq!(sanitize_class_name("Vec<double>"), "Vec_double");
        assert_ne!(sanitize_class_name("Vec<int>"), sanitize_class_name("Vec<double>"));
        // An anonymous namespace is not an identifier; it becomes one.
        assert_eq!(
            sanitize_class_name("(anonymous namespace)::Hidden"),
            "anonymous_namespace::Hidden"
        );
        // A `::` INSIDE a template argument list is not a scope boundary.
        assert_eq!(
            sanitize_class_name("std::vector<std::pair<int, int> >"),
            "std::vector_std_pair_int_int"
        );
        assert_eq!(sanitize_class_name(""), "");
    }

    #[test]
    fn offset_to_top_sign_extends_per_width() {
        let rk64 = RefKind::new(true, 0, End::Little);
        assert_eq!(sign_extend(u64::MAX - 15, &rk64), -16);
        let rk32 = RefKind::new(false, 0, End::Little);
        assert_eq!(sign_extend(0xffff_fff0, &rk32), -16);
    }
}
