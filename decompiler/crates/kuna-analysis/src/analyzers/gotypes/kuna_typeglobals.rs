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
//! The symbol is spelled as the linker spells it ([`linker_name`]): a named
//! type's qualifier is the full import path from `uncommontype.pkgPath`
//! (`type:internal/poll.FD`, where the name record holds `poll.FD`), and a
//! pointer, slice, array or map spelling is rebuilt from its elements' so the
//! qualifier reaches it. The located records (typelinks, or the go1.27+
//! descriptor region) are the roots of a walk over the type graph
//! ([`reachable`]): the types a listed type is built from — a struct's
//! field types, a slice's element, a func's parameters — have descriptors
//! of their own that allocation and reflection sites name just as often.
//!
//! Name vocabulary: the harvest's `name_ok` is for INTERNED shells and
//! demands a dotted package path; a global name only has to print safely, so
//! unnamed composites (`[]uint8`, `map[string]int`) are kept — their spelled
//! form is exactly what an analyst searches for — while anything with a
//! space (anonymous `struct { ... }` literals, `func(int) bool`, `chan int`)
//! or non-graphic byte is dropped: a symbol with interior whitespace breaks
//! the C document it is printed into.

use std::collections::{BTreeMap, VecDeque};

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

/// The most descriptors one image's graph walk visits. The walk reaches every
/// descriptor the typelinks' types refer to, several times the typelink count
/// on a large image; the ceiling only bounds a corrupt graph.
const WALK_MAX: usize = 200_000;

/// The descriptor at `vma`, when it lies inside the type blob.
fn descriptor_at(img: &GoImage, meta: &TypeMeta, vma: u64) -> Option<GoType> {
    let in_blob =
        vma >= meta.types && vma.checked_add(img.type_base()).is_some_and(|end| end <= meta.etypes);
    in_blob.then(|| read_type(img, meta.types, meta.etypes, (vma - meta.types) as u32)).flatten()
}

/// The descriptor a 32-bit `typeOff` at `at` names (`0` is "none").
fn descriptor_off(img: &GoImage, meta: &TypeMeta, at: u64) -> Option<u64> {
    let off = i32::from_le_bytes(img.read(at, 4)?.try_into().ok()?);
    (off > 0).then(|| meta.types.checked_add(off as u64)).flatten()
}

/// The descriptors `t` refers to: `ptrToThis`, then per kind the element and
/// key (array, chan, map, pointer, slice), the parameters and results (func),
/// the method types (interface) and the field types (struct).
fn edges(img: &GoImage, meta: &TypeMeta, t: &GoType) -> Vec<u64> {
    let (w, base) = (img.word_size, t.vma + img.type_base());
    let mut next: Vec<u64> = descriptor_off(img, meta, base - 4).into_iter().collect();
    match t.kind {
        KIND_ARRAY | KIND_MAP => next.extend([word(img, base), word(img, base + w)].into_iter().flatten()),
        KIND_CHAN | KIND_POINTER | KIND_SLICE => next.extend(word(img, base)),
        KIND_FUNC => {
            if let Some(b) = img.read(base, 4) {
                let n = u64::from(u16::from_le_bytes([b[0], b[1]]))
                    + u64::from(u16::from_le_bytes([b[2], b[3]]) & 0x7fff);
                let params = base + w + if t.tflag & TFLAG_UNCOMMON != 0 { UNCOMMON_SIZE } else { 0 };
                next.extend((0..n.min(512)).filter_map(|k| word(img, params + k * w)));
            }
        }
        KIND_INTERFACE => {
            if let Some((ptr, len)) = plausible_slice(img, base + w, 4096) {
                next.extend((0..len).filter_map(|k| descriptor_off(img, meta, ptr + k * 8 + 4)));
            }
        }
        KIND_STRUCT => {
            if let Some((ptr, len)) = plausible_slice(img, base + w, 4096) {
                next.extend((0..len).filter_map(|k| word(img, ptr + k * 3 * w + w)));
            }
        }
        _ => {}
    }
    next
}

/// Every descriptor reachable from the located records through [`edges`],
/// keyed by VMA. The typelinks (and the go1.27+ descriptor region) list the
/// types interface conversion reaches; the types those are built from — a
/// struct's field types, a slice's element, a function's parameters — have
/// descriptors of their own that allocation and reflection sites name just as
/// often, and the walk is what reaches them.
pub(super) fn reachable(img: &GoImage, meta: &TypeMeta) -> BTreeMap<u64, GoType> {
    let mut records: BTreeMap<u64, GoType> = BTreeMap::new();
    let mut queue: VecDeque<u64> = meta.offsets.iter().map(|&o| meta.types + u64::from(o)).collect();
    while let Some(vma) = queue.pop_front() {
        if records.len() >= WALK_MAX || records.contains_key(&vma) {
            continue;
        }
        let Some(t) = descriptor_at(img, meta, vma) else { continue };
        queue.extend(edges(img, meta, &t));
        records.insert(vma, t);
    }
    records
}

/// The `uncommontype.pkgPath` of a named type: the import path of the package
/// that declares it. The uncommon record follows the kind's own record; map
/// descriptors, whose record changed size with the swiss-table maps, are not
/// read.
fn pkg_path(img: &GoImage, meta: &TypeMeta, t: &GoType) -> Option<String> {
    if t.tflag & TFLAG_UNCOMMON == 0 || t.tflag & TFLAG_NAMED == 0 {
        return None;
    }
    let w = img.word_size;
    let record = match t.kind {
        KIND_ARRAY => 3 * w,
        KIND_CHAN => 2 * w,
        KIND_FUNC | KIND_POINTER | KIND_SLICE => w,
        KIND_INTERFACE | KIND_STRUCT => 4 * w,
        KIND_MAP => return None,
        _ => 0,
    };
    let at = descriptor_off(img, meta, t.vma + img.type_base() + record)?;
    read_go_name(img, at)
}

/// `t`'s name as the linker spells it: the package qualifier is the full
/// import path (`internal/poll.FD`, where the name record holds `poll.FD`),
/// taken from `pkgPath`; pointer, slice, array and map names are rebuilt from
/// their elements' so the qualifier reaches them too. Any other spelling
/// (a func or chan type, an anonymous struct) is the record's own.
pub(super) fn linker_name(img: &GoImage, meta: &TypeMeta, t: &GoType) -> String {
    linker_name_depth(img, meta, t, 8)
}

fn linker_name_depth(img: &GoImage, meta: &TypeMeta, t: &GoType, depth: u32) -> String {
    let base = t.vma + img.type_base();
    let child = |at: u64| {
        word(img, at)
            .filter(|_| depth > 0)
            .and_then(|e| descriptor_at(img, meta, e))
            .map(|e| linker_name_depth(img, meta, &e, depth - 1))
    };
    if t.tflag & TFLAG_NAMED == 0 {
        let composed = match t.kind {
            KIND_POINTER => child(base).map(|e| format!("*{e}")),
            KIND_SLICE => child(base).map(|e| format!("[]{e}")),
            KIND_ARRAY => word(img, base + 2 * img.word_size).zip(child(base)).map(|(n, e)| format!("[{n}]{e}")),
            KIND_MAP => child(base).zip(child(base + img.word_size)).map(|(k, e)| format!("map[{k}]{e}")),
            _ => None,
        };
        return composed.unwrap_or_else(|| t.name.clone());
    }
    let Some(path) = pkg_path(img, meta, t) else { return t.name.clone() };
    let short = path.rsplit('/').next().unwrap_or(&path);
    match t.name.strip_prefix(short).and_then(|r| r.strip_prefix('.')) {
        Some(rest) if path != short => format!("{path}.{rest}"),
        _ => t.name.clone(),
    }
}

/// Walk the located metadata once more and produce one [`TypedDataFact`] per
/// reachable, printable descriptor: the record's VMA, the linker-style
/// `type:<name>` symbol ([`linker_name`]), and the `go.typedesc` shell.
/// Pointer records keep their decoded `*` prefix (their name genuinely starts
/// with it), and a pointer's pointee joins even without its own typelink —
/// the pair `type:main.Point` / `type:*main.Point` is how the blob stores
/// them. Deduplicated by VMA, emitted in address order.
pub(super) fn collect(tf: &dyn TypeFactory, img: &GoImage, meta: &TypeMeta) -> Vec<TypedDataFact> {
    let Some(desc) = typedesc_shell(tf, img.word_size as int4) else {
        return Vec::new();
    };
    reachable(img, meta)
        .into_values()
        .map(|t| (t.vma, linker_name(img, meta, &t)))
        .filter(|(_, name)| global_name_ok(name))
        .map(|(addr, name)| TypedDataFact {
            addr,
            name: format!("{NAME_PREFIX}{name}"),
            type_: std::rc::Rc::clone(&desc),
        })
        .collect()
}
