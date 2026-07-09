//! The ObjC 2.0 `class_t` / `class_ro_t` readers (64-bit Mach-O).
//!
//! Port of Ghidra's `Objc2Class` + `Objc2ClassRW`/`Objc2ClassRO` byte models. A
//! `class_t` is five pointer-sized words (`isa`, `superclass`, `cache`, `vtable`,
//! `data`); the `data` word's low bits are flags (`FAST_DATA_MASK` masks them off
//! to reach the `class_ro_t`). The `class_ro_t` carries the class `name` and the
//! `baseMethods` `method_list_t*` this pass needs.
//!
//! Only the fields the name/method walk reads are modeled; the rest (cache,
//! protocols, ivars, properties) are skipped by offset, faithful to the minimal
//! `ObjcTypeMetadataAnalyzer` walk (§3.2 of the design).

use super::sections::MachoImage;

/// The 64-bit `class_t.data` low-bit mask Ghidra/`objc4` calls `FAST_DATA_MASK`:
/// the `data` word packs `RW`/realized flags in the low 3 bits, so `data & ~0x7`
/// is the `class_ro_t*`. (32-bit uses `~0x3`; this pass is the 64-bit Mach-O path,
/// so `~0x7`.)
pub const FAST_DATA_MASK_64: u64 = !0x7u64;

/// `class_t` field offsets (64-bit: five pointer-sized words).
mod class_t {
    /// `isa` — points at the metaclass `class_t` (its `baseMethods` are the `+`
    /// class methods).
    pub const ISA: u64 = 0x00;
    /// `data` — `class_ro_t*` once `FAST_DATA_MASK` is applied.
    pub const DATA: u64 = 0x20;
}

/// `class_ro_t` field offsets (64-bit). `flags`/`instanceStart`/`instanceSize`/
/// `reserved` are four `u32`s, then pointer-sized `ivarLayout`, `name`,
/// `baseMethodList`, ….
mod class_ro_t {
    /// `name` — the NUL-terminated class name (`Greeter`).
    pub const NAME: u64 = 0x18;
    /// `baseMethodList` — the instance `method_list_t*` (NULL when the class
    /// defines no methods).
    pub const BASE_METHODS: u64 = 0x20;
    /// `ivars` — the `ivar_list_t*` (NULL when the class declares no instance
    /// variables). After `name`(0x18), `baseMethodList`(0x20), `baseProtocols`
    /// (0x28) comes `ivars`(0x30).
    pub const IVARS: u64 = 0x30;
}

/// `ivar_list_t` header offsets (the same `entsizeAndFlags`/`count` shape as
/// `method_list_t`, with no small/relative form) + the `ivar_t` record layout.
mod ivar_list_t {
    /// `entsizeAndFlags` (`u32`): low 16 bits = per-entry size (an `ivar_t` is 0x20
    /// on a 64-bit Mach-O).
    pub const ENTSIZE: u64 = 0x00;
    /// `count` (`u32`): number of `ivar_t` records.
    pub const COUNT: u64 = 0x04;
    /// First `ivar_t` record.
    pub const FIRST: u64 = 0x08;
}

/// `ivar_t` field offsets (64-bit): `*offset`(ptr @0), `name`(ptr @8),
/// `type`(ptr @0x10), `alignment`(u32 @0x18), `size`(u32 @0x1c).
mod ivar_t {
    /// `*offset` — pointer to the ivar's *offset variable* (the
    /// `_OBJC_IVAR_$_Class.ivar` slot we label).
    pub const OFFSET_PTR: u64 = 0x00;
    /// `name` — the NUL-terminated ivar name.
    pub const NAME: u64 = 0x08;
    /// `type` — the NUL-terminated ObjC type encoding of the ivar's element.
    pub const TYPE: u64 = 0x10;
}

/// A resolved `class_t`: the addresses the method walk needs.
#[derive(Clone, Copy, Debug)]
pub struct ClassT {
    /// VMA of the `class_t` itself (the `classlist` entry).
    pub addr: u64,
    /// VMA of the metaclass `class_t` (`isa`) — its `baseMethods` are the `+`
    /// (class) methods. `0` when absent.
    pub isa: u64,
    /// VMA of the resolved `class_ro_t` (`data & FAST_DATA_MASK`). `0` when the
    /// `data` word is unreadable.
    pub ro: u64,
}

/// Read a `class_t` at `addr`: follow `data & FAST_DATA_MASK` to the `class_ro_t`
/// and capture `isa` (the metaclass). `None` only when the `class_t.data` word is
/// unreadable (an unmapped / truncated class).
pub fn read_class_t(img: &MachoImage, addr: u64) -> Option<ClassT> {
    let isa = img.read_ptr(addr.wrapping_add(class_t::ISA)).unwrap_or(0);
    let data = img.read_ptr(addr.wrapping_add(class_t::DATA))?;
    let ro = data & FAST_DATA_MASK_64;
    Some(ClassT { addr, isa, ro })
}

/// The class name at `class_ro_t.name`, NUL-terminated ASCII (`Greeter`). `None`
/// when the `name` pointer / string is unreadable or non-ASCII.
pub fn class_name(img: &MachoImage, ro: u64) -> Option<String> {
    let name_ptr = img.read_ptr(ro.wrapping_add(class_ro_t::NAME))?;
    if name_ptr == 0 {
        return None;
    }
    img.read_cstr(name_ptr)
}

/// The `baseMethodList` (`method_list_t*`) at `class_ro_t.baseMethods`. `None` /
/// `Some(0)` when the class defines no methods.
pub fn base_methods(img: &MachoImage, ro: u64) -> Option<u64> {
    let p = img.read_ptr(ro.wrapping_add(class_ro_t::BASE_METHODS))?;
    if p == 0 {
        None
    } else {
        Some(p)
    }
}

/// The metaclass `class_ro_t` reached through `class_t.isa.data`: its
/// `baseMethods` are the `+` (class) methods. `None` when `isa` is unset or its
/// `data` word is unreadable.
pub fn metaclass_ro(img: &MachoImage, isa: u64) -> Option<u64> {
    if isa == 0 {
        return None;
    }
    let data = img.read_ptr(isa.wrapping_add(class_t::DATA))?;
    Some(data & FAST_DATA_MASK_64)
}

/// The `ivars` (`ivar_list_t*`) at `class_ro_t.ivars`. `None` when the class
/// declares no instance variables (the common case for the root-class fixture).
pub fn base_ivars(img: &MachoImage, ro: u64) -> Option<u64> {
    let p = img.read_ptr(ro.wrapping_add(class_ro_t::IVARS))?;
    if p == 0 {
        None
    } else {
        Some(p)
    }
}

/// One resolved instance variable: its offset-slot VMA, name, and type encoding.
#[derive(Clone, Debug)]
pub struct Ivar {
    /// VMA of the ivar's *offset variable* (the `_OBJC_IVAR_$_Class.ivar` slot we
    /// label `<Class>::<ivar>`).
    pub offset_va: u64,
    /// The ivar name (`_count`), NUL-terminated ASCII.
    pub name: String,
    /// The ivar's ObjC type encoding (`q`, `@`, …), or `None` when unreadable.
    pub types: Option<String>,
}

/// Walk an `ivar_list_t` at `list_va`, returning each resolved [`Ivar`]. The
/// `ivar_list_t` shares `method_list_t`'s `entsizeAndFlags`/`count` header but has
/// no small/relative form (an `ivar_t` is always the fixed 0x20-byte record). A
/// record whose name/offset is unreadable is skipped; empty for a NULL/unreadable
/// list. The count is capped so a corrupt header can never spin.
pub fn walk_ivar_list(img: &MachoImage, list_va: u64) -> Vec<Ivar> {
    let mut out = Vec::new();
    let Some(ent) = img.read_u32(list_va + ivar_list_t::ENTSIZE) else { return out };
    let Some(count) = img.read_u32(list_va + ivar_list_t::COUNT) else { return out };
    let entsize = (ent & 0xffff) as u64;
    if entsize == 0 {
        return out;
    }
    let count = count.min(4096);
    for i in 0..count as u64 {
        let rec = list_va
            .wrapping_add(ivar_list_t::FIRST)
            .wrapping_add(i.wrapping_mul(entsize));
        // `*offset` points at the offset variable; the slot VMA is what we label.
        let Some(offset_va) = img.read_ptr(rec.wrapping_add(ivar_t::OFFSET_PTR)) else {
            continue;
        };
        let Some(name_ptr) = img.read_ptr(rec.wrapping_add(ivar_t::NAME)) else { continue };
        let Some(name) = img.read_cstr(name_ptr) else { continue };
        let types = img
            .read_ptr(rec.wrapping_add(ivar_t::TYPE))
            .filter(|&p| p != 0)
            .and_then(|p| img.read_cstr(p));
        if offset_va == 0 || name.is_empty() {
            continue;
        }
        out.push(Ivar { offset_va, name, types });
    }
    out
}
