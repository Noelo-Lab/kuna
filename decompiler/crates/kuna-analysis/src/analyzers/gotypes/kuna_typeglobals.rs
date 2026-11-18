//! (kuna `gotypeglobals`) The first `gotypes` CONSUMER — the walked `_type`
//! descriptors become NAMED, TYPED data globals.
//!
//! A Go binary's runtime metadata types exactly one family of data objects
//! outright: its own type descriptors. Every `_type` record the harvest
//! decodes is a global of layout `internal/abi.Type`, and Go's linker names
//! it `type:<go name>` (`type:main.Point`, `type:*main.Point`,
//! `type:[]uint8`) — names a stripped image loses with the symtab while the
//! records themselves stay reachable through the moduledata. Reinstating
//! them is what makes the harvested names APPEAR in emitted C: every
//! `runtime.newobject(&type:main.Point)`, interface conversion and
//! reflection call site renders the type it manipulates instead of a bare
//! `dat_<addr>` or constant.
//!
//! Each fact carries the `go.typedesc` shell (the 48/32-byte `abi.Type` base
//! record, stable since go1.18, same field positions `read_type` decodes), so
//! a runtime function walking a descriptor renders named field reads
//! (`t->Kind_`, `t->Size_`). The shell spans only the base record — a
//! struct/map/func descriptor's kind-specific tail belongs to the next
//! consumer — so the mapped extent never overlaps the following record.
//!
//! Name vocabulary: the harvest's `name_ok` is for INTERNED shells and
//! demands a dotted package path; a global name only has to print safely, so
//! unnamed composites (`[]uint8`, `map[string]int`) are kept — their spelled
//! form is exactly what an analyst searches for — while anything with a
//! space (anonymous `struct { ... }` literals, `func(int) bool`, `chan int`)
//! or non-graphic byte is dropped: a symbol with interior whitespace breaks
//! the C document it is printed into.

use std::collections::BTreeMap;

use super::*;
use crate::pass::TypedDataFact;

/// The linker-style symbol prefix (go1.21+ spelling; earlier links used
/// `type.`, but the colon form is what current tooling and goroutine dumps
/// show, and it cannot collide with a package path).
const NAME_PREFIX: &str = "type:";

/// `true` when a decoded descriptor name can serve as a data-symbol name:
/// non-empty, bounded, every byte ASCII-graphic (NO spaces — unlike
/// [`name_ok`](super::name_ok), which admits them for anonymous layouts).
pub(super) fn global_name_ok(name: &str) -> bool {
    !name.is_empty() && name.len() <= 256 && name.bytes().all(|b| b.is_ascii_graphic())
}

/// Get-or-create the `go.typedesc` shell: the `internal/abi.Type` base record
/// at the harvest's own decode offsets (`word_size`-scaled, 48 bytes on
/// 64-bit, 32 on 32-bit). An already-completed struct of that name is reused.
pub(super) fn typedesc_shell(
    tf: &dyn TypeFactory,
    width: int4,
) -> Option<std::rc::Rc<Datatype>> {
    let ct = shell(tf, "go.typedesc").ok()?;
    if ct.get_size() != 0 {
        return Some(ct);
    }
    let void = tf.get_type_void().ok()?;
    let vptr = tf.get_type_pointer(width, void, 1).ok()?;
    let uptr = tf.get_base(width, type_metatype::TYPE_UINT).ok()?;
    let u4 = tf.get_base(4, type_metatype::TYPE_UINT).ok()?;
    let u1 = tf.get_base(1, type_metatype::TYPE_UINT).ok()?;
    let i4 = tf.get_base(4, type_metatype::TYPE_INT).ok()?;
    let w = width;
    let rows: [(&str, std::rc::Rc<Datatype>, int4); 11] = [
        ("Size_", std::rc::Rc::clone(&uptr), 0),
        ("PtrBytes", uptr, w),
        ("Hash", u4, 2 * w),
        ("TFlag", std::rc::Rc::clone(&u1), 2 * w + 4),
        ("Align_", std::rc::Rc::clone(&u1), 2 * w + 5),
        ("FieldAlign_", std::rc::Rc::clone(&u1), 2 * w + 6),
        ("Kind_", u1, 2 * w + 7),
        ("Equal", std::rc::Rc::clone(&vptr), 2 * w + 8),
        ("GCData", vptr, 3 * w + 8),
        ("Str", std::rc::Rc::clone(&i4), 4 * w + 8),
        ("PtrToThis", i4, 4 * w + 12),
    ];
    let fields: Vec<TypeField> = rows
        .into_iter()
        .enumerate()
        .map(|(i, (name, ty, offset))| TypeField {
            ident: i as int4,
            offset,
            name: name.to_string(),
            field_type: ty,
        })
        .collect();
    tf.set_fields_struct_raw(&ct, fields, Vec::new(), 4 * w + 16, w, 0).ok()
}

/// Walk the located metadata once more and produce one [`TypedDataFact`] per
/// decodable, printable descriptor: the record's VMA, the linker-style
/// `type:<name>` symbol, and the `go.typedesc` shell. Pointer records keep
/// their decoded `*` prefix (their name genuinely starts with it), and a
/// pointer's pointee joins even without its own typelink — the pair
/// `type:main.Point` / `type:*main.Point` is how the blob stores them.
/// Deduplicated by VMA, emitted in address order.
pub(super) fn collect(tf: &dyn TypeFactory, img: &GoImage, meta: &TypeMeta) -> Vec<TypedDataFact> {
    let Some(desc) = typedesc_shell(tf, img.word_size as int4) else {
        return Vec::new();
    };
    let mut records: BTreeMap<u64, GoType> = BTreeMap::new();
    for &off in &meta.offsets {
        let Some(t) = read_type(img, meta.types, meta.etypes, off) else { continue };
        if t.kind == KIND_POINTER {
            if let Some(elem) = word(img, t.vma + img.type_base()) {
                if elem >= meta.types
                    && elem.checked_add(img.type_base()).is_some_and(|end| end <= meta.etypes)
                {
                    if let Some(p) = read_type(img, meta.types, meta.etypes, (elem - meta.types) as u32)
                    {
                        records.entry(p.vma).or_insert(p);
                    }
                }
            }
        }
        records.entry(t.vma).or_insert(t);
    }
    records
        .into_values()
        .filter(|t| global_name_ok(&t.name))
        .map(|t| TypedDataFact {
            addr: t.vma,
            name: format!("{NAME_PREFIX}{}", t.name),
            type_: std::rc::Rc::clone(&desc),
        })
        .collect()
}
