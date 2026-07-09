//! The ObjC 2.0 `method_list_t` / `method_t` walk (both list forms).
//!
//! Port of Ghidra's `Objc2MethodList` + `Objc2Method` (`ObjcMethod.applyTo` /
//! `ObjcUtils.createMethods`). A `method_list_t` is a header (`entsizeAndFlags`,
//! `count`) followed by `count` `method_t` records. There are two record forms,
//! selected by the high bit of `entsizeAndFlags`:
//!
//! - **Large / absolute** (`entsizeAndFlags & 0x80000000 == 0`, entsize 24): three
//!   pointer-sized absolute VMAs — `name` (selector string ptr), `types` (encoding
//!   string ptr), `imp` (the function VA). The classic 64-bit Mach-O form the
//!   design's §3.2 MVP targets.
//! - **Small / relative** (`entsizeAndFlags & 0x80000000 != 0`, entsize 12): three
//!   self-relative `i32` displacements. `name` points at a **selref** (a pointer to
//!   the selector string, not the string itself), `types`/`imp` are direct
//!   self-relative. The form a modern clang emits (`__TEXT,__objc_methlist`).
//!
//! Both are handled here: the small form is what real clang produces for the
//! self-contained fixture, so handling it is what makes the end-to-end naming work
//! on a bare-toolchain build. The IMP `imp` VA is the function the pass renames
//! `-[Class sel]` / `+[Class sel]`.

use super::sections::MachoImage;

/// `method_list_t.entsizeAndFlags` high bit: the list holds **small / relative**
/// `method_t` records (12-byte, `i32` self-relative fields) rather than the
/// large absolute-pointer form.
pub const METHOD_LIST_SMALL: u32 = 0x8000_0000;

/// The low bits of `entsizeAndFlags` are the per-entry size in bytes.
const ENTSIZE_MASK: u32 = 0x0000_ffff;

/// `method_list_t` header offsets.
mod method_list {
    /// `entsizeAndFlags` (`u32`): low 16 bits = per-entry size, high bit = small.
    pub const ENTSIZE_AND_FLAGS: u64 = 0x00;
    /// `count` (`u32`): number of `method_t` records.
    pub const COUNT: u64 = 0x04;
    /// First `method_t` record.
    pub const FIRST: u64 = 0x08;
}

/// One resolved method: the selector + type encoding + the IMP function VA.
#[derive(Clone, Debug)]
pub struct Method {
    /// The selector (`greet:`), NUL-terminated ASCII.
    pub selector: String,
    /// VMA of the selector string itself (the `__objc_methname` byte), for a
    /// selector Data symbol. `0` when not separately known.
    pub selector_va: u64,
    /// The ObjC type encoding (`i20@0:8i16`), or `None` when unreadable.
    pub types: Option<String>,
    /// The IMP function virtual address (the rename / function-definition target).
    pub imp: u64,
}

/// Walk a `method_list_t` at `list_va`, returning each resolved [`Method`].
///
/// Caps the count at a sane bound so a corrupt header can never spin. A record
/// whose selector/imp is unreadable is skipped (additive, never fails). Empty for
/// a NULL/unreadable list.
pub fn walk_method_list(img: &MachoImage, list_va: u64) -> Vec<Method> {
    let mut out = Vec::new();
    let Some(ent) = img.read_u32(list_va + method_list::ENTSIZE_AND_FLAGS) else {
        return out;
    };
    let Some(count) = img.read_u32(list_va + method_list::COUNT) else {
        return out;
    };
    let small = ent & METHOD_LIST_SMALL != 0;
    let entsize = (ent & ENTSIZE_MASK) as u64;
    if entsize == 0 {
        return out;
    }
    // A defensive cap: no real class has this many methods, and it bounds a
    // corrupt `count` (faithful to the inert/no-op analysis-pass contract).
    let count = count.min(4096);

    for i in 0..count as u64 {
        // Wrapping (entsize ≤ 0xffff and count ≤ 4096, so this cannot wrap for any
        // real `list_va`; wrapping just keeps a hostile header inert rather than
        // panicking in debug — a wrapped `m` misses the reads in read_small/large).
        let m = list_va
            .wrapping_add(method_list::FIRST)
            .wrapping_add(i.wrapping_mul(entsize));
        let method = if small { read_small(img, m) } else { read_large(img, m) };
        if let Some(method) = method {
            out.push(method);
        }
    }
    out
}

/// A **large / absolute** `method_t` at `m`: `name`, `types`, `imp` are three
/// pointer-sized absolute VMAs. `name`/`types` point at the strings directly; `imp`
/// is the function VA.
fn read_large(img: &MachoImage, m: u64) -> Option<Method> {
    let ps = img.ptr_size();
    let name_ptr = img.read_ptr(m)?;
    let types_ptr = img.read_ptr(m.wrapping_add(ps))?;
    let imp = img.read_ptr(m.wrapping_add(2 * ps))?;
    if imp == 0 {
        return None;
    }
    let selector = img.read_cstr(name_ptr)?;
    let types = img.read_cstr(types_ptr);
    Some(Method { selector, selector_va: name_ptr, types, imp: strip_thumb(imp) })
}

/// A **small / relative** `method_t` at `m`: `name`, `types`, `imp` are three
/// self-relative `i32` displacements. The `name` field points at a **selref**
/// (`*selref` is the selector-string VMA); `types`/`imp` are direct.
fn read_small(img: &MachoImage, m: u64) -> Option<Method> {
    let n_rel = img.read_i32(m)?;
    let t_rel = img.read_i32(m.wrapping_add(4))?;
    let i_rel = img.read_i32(m.wrapping_add(8))?;

    // Each field is self-relative to ITS OWN address (the `name` field at `m`, the
    // `types` field at `m+4`, the `imp` field at `m+8`). `rel_to` adds a signed
    // displacement to a base VMA with wrapping (well-defined for any VMA, never
    // panics in debug); a wrapped/garbage result just misses the section reads
    // below — the inert, never-wrong path.

    // name: self-relative to a selref slot; deref the selref to the string VMA.
    let selref_va = rel_to(m, n_rel);
    let sel_ptr = img.read_ptr(selref_va)?;
    let selector = img.read_cstr(sel_ptr)?;

    // types: self-relative (field at m+4) to the encoding string.
    let types_va = rel_to(m.wrapping_add(4), t_rel);
    let types = img.read_cstr(types_va);

    // imp: self-relative (field at m+8) to the function VA.
    let imp = rel_to(m.wrapping_add(8), i_rel);
    if imp == 0 {
        return None;
    }
    Some(Method { selector, selector_va: sel_ptr, types, imp: strip_thumb(imp) })
}

/// Add a signed self-relative `i32` displacement to a base VMA with wrapping
/// (sign-extended). Well-defined for any base (no overflow panic in debug, no
/// surprising behavior at high VMAs); a wrapped result simply misses the later
/// section reads, which is the inert path.
fn rel_to(base: u64, rel: i32) -> u64 {
    base.wrapping_add(rel as i64 as u64)
}

/// Strip the 32-bit ARM Thumb-mode low bit from an IMP address. On AArch64 / x86
/// this is a no-op (those IMPs are even-aligned); on 32-bit ARM the IMP's LSB
/// flags Thumb and is not part of the address. (The x86-64 MVP never sets it; this
/// is the forward-compatible normalization Ghidra's `ObjcMethod` applies.)
fn strip_thumb(imp: u64) -> u64 {
    imp & !1u64
}
