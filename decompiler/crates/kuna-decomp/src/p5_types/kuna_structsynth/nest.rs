//! (kuna `structsynth nest`) A pointer field whose value is loaded and then
//! dereferenced at constant offsets is typed as a pointer to a record.
//!
//! # The gap
//!
//! `param` measures a parameter's own dereferences and stops there, so a field
//! that holds a pointer keeps whatever scalar the loads of it carried, and every
//! read through it is raw offset arithmetic again:
//!
//! ```text
//! struct struct_0 { long *field_0x0; long field_0x8; int field_0x10; ... };
//!   v1 = a0->field_0x0;
//!   return (long)(int)v1[1] + *v1 + (long)*(int *)((long)v1 + 0xc) + ...;
//! ```
//!
//! # The rule
//!
//! Every `LOAD` of a pointer-sized field is a value of the field's type, so the
//! dereferences made through all of them are one base's evidence, merged under
//! the rules `Evidence::record` applies to one base. That evidence is held to
//! exactly the conditions a parameter is: each loaded value already carries
//! `TYPE_PTR` and is not type-locked, none reaches a phi, an index, or an
//! integer operation, the lattice has settled, there are two distinct offsets
//! including 0, and the offsets are not an array's uniform run. What passes is
//! pruned and made dense as a parameter's layout is, and the field is typed as a
//! pointer to the record the ledger answers for it. A field whose loads fail any
//! condition keeps the type it had.
//!
//! # The same record
//!
//! A value loaded from a field of a record is the same kind of record when
//! everything it was seen to hold the enclosing record also holds, at the same
//! offset, width and type, and the bytes it accessed without claiming are laid
//! out alike (`ledger::keeps_unclaimed`). Then the field points at the enclosing
//! record itself: `grep`'s `treenext` reads `{0, 8, 0x10, 0x18}` of a tree node
//! and the same four fields of the node its `llink` points at. The condition is
//! claims, not offsets that merely land inside the record, and it applies after
//! the standalone rules, so one coincident word is never enough: `obstack`'s
//! `_obstack_newchunk` reads the chunk's `prev` at 8 where the obstack itself
//! keeps an integer word.
//!
//! A record that points at itself cannot be completed first, because its own
//! field names it; the ledger mints it around the incomplete shell (see
//! `ledger::lookup_or_mint`). Only the record directly holding the field is
//! compared: a record two levels up would have to be minted together with the
//! one in between.
//!
//! # Bounds
//!
//! [`MAX_NEST_DEPTH`] records below the parameter. Each nested record is looked
//! up before the record holding it, so its pointer names a completed type.

use std::collections::BTreeMap;

use kuna_base::types::{int4, intb};

use super::{accepts_record, answer, fields_for, ledger, note_uses, points_at_named_composite, vn_type, Evidence};
use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use crate::funcdata::Funcdata;

/// How many records below the parameter a chain of pointer fields is followed.
pub(super) const MAX_NEST_DEPTH: u32 = 2;

/// What every level of one parameter's nesting shares.
pub(super) struct Nesting<'a> {
    /// Every base's unpruned evidence.
    pub ev: &'a BTreeMap<VarnodeId, Evidence>,
    /// The program's type factory.
    pub types: &'a dyn TypeFactory,
    /// Pointer width in bytes.
    pub ptr_size: int4,
}

/// Retype every pointer field of one accepted record whose loads are the base of
/// a record in turn, and return the offsets of the fields that point at `rec`
/// itself.
///
/// `fields` is the record's member list as `fields_for` built it from `rec` and
/// `size` its size; a nested field takes a pointer to the completed record the
/// ledger answered, and a self field keeps its type for the ledger to replace.
pub(super) fn nest_fields(
    data: &mut Funcdata,
    cx: &Nesting,
    rec: &Evidence,
    fields: &mut [TypeField],
    size: int4,
    depth: u32,
) -> Vec<int4> {
    let own: Vec<TypeField> = fields.to_vec();
    let mut selfs = Vec::new();
    let census = std::env::var_os("KUNA_SSCENSUS").is_some();
    for i in 0..fields.len() {
        let field = &own[i];
        if field.field_type.get_size() != cx.ptr_size {
            continue;
        }
        if census && depth == 0 {
            census_field(data, cx, rec, field);
        }
        if !is_record_pointer(&field.field_type) {
            continue;
        }
        let Some(inner) = loaded_record(data, cx, rec, intb::from(field.offset)) else { continue };
        if !accepts_record(data, &field.field_type, &inner, false, false) {
            continue;
        }
        let Some((mut inner_fields, inner_size)) = fields_for(cx.types, &inner) else { continue };
        let unclaimed = inner.unclaimed_ranges();
        if is_same_record(&own, size, &inner_fields, inner_size, &unclaimed) {
            selfs.push(field.offset);
            continue;
        }
        if depth >= MAX_NEST_DEPTH {
            continue;
        }
        let inner_selfs = nest_fields(data, cx, &inner, &mut inner_fields, inner_size, depth + 1);
        let Some(st) = answer(data, cx.types, inner_fields, inner_size, &unclaimed, &inner_selfs) else {
            continue;
        };
        let Ok(ptr) = cx.types.get_type_pointer(cx.ptr_size, st, 1) else { continue };
        fields[i].field_type = ptr;
    }
    selfs
}

/// TEMPORARY census: why one pointer-width field of an accepted record is not
/// given a record of its own, and what two relaxations would do with it.
fn census_field(data: &mut Funcdata, cx: &Nesting, rec: &Evidence, field: &TypeField) {
    let fa = data.get_address().get_offset();
    let tag = |s: String| eprintln!("SSCENSUS nest {fa:x} {:x} {s}", field.offset);
    let off = intb::from(field.offset);
    let Some(loads) = rec.loads.get(&off) else {
        return tag("noload".into());
    };
    let mut values: Vec<VarnodeId> = Vec::new();
    for (w, v) in loads {
        if *w == cx.ptr_size && !values.contains(v) {
            values.push(*v);
        }
    }
    if values.is_empty() {
        return tag("narrowload".into());
    }
    let mut merged = Evidence::default();
    let mut valptr = true;
    for v in values {
        let Some(vn) = data.vbank().get(v) else { return tag("novn".into()) };
        if vn.is_type_lock() || vn.is_persist() || vn.is_spacebase() {
            return tag("valuelocked".into());
        }
        let Some(ct) = vn_type(data, v) else { return tag("novaltype".into()) };
        if !is_record_pointer(&ct) {
            valptr = false;
        }
        match cx.ev.get(&v) {
            Some(e) => merged.absorb(e),
            None => {
                let mut uses = Evidence::default();
                note_uses(data, v, &mut uses);
                merged.absorb(&uses);
            }
        }
    }
    if merged.slots.is_empty() {
        let mut vt = String::new();
        let mut uses = String::new();
        for (w, v) in loads {
            if *w != cx.ptr_size {
                continue;
            }
            if let Some(ct) = vn_type(data, *v) {
                let named = points_at_named_composite(&ct);
                let pm = ct.get_ptr_to().map(|p| format!("{:?}", p.get_metatype())).unwrap_or_else(|| format!("{:?}", ct.get_metatype()));
                vt = format!("{}{}", if named { "NAMED-" } else { "" }, pm.trim_start_matches("TYPE_").to_lowercase());
            }
            if let Some(vn) = data.vbank().get(*v) {
                for u in vn.descend_iter() {
                    if let Some(op) = data.obank().get(u) {
                        let c = format!("{:?}", op.code());
                        let c = c.trim_start_matches("CPUI_").to_lowercase();
                        if !uses.contains(&c) {
                            uses.push_str(&c);
                            uses.push('+');
                        }
                    }
                }
            }
        }
        let ft = field.field_type.get_ptr_to().map(|p| format!("{:?}", p.get_metatype())).unwrap_or_else(|| format!("{:?}", field.field_type.get_metatype()));
        return tag(format!("noderef/{}/{}/{}", ft.trim_start_matches("TYPE_").to_lowercase(), vt, uses));
    }
    let inner = merged.pruned();
    let fieldptr = is_record_pointer(&field.field_type);
    // The two relaxations, reported independently of each other.
    let neg = inner.dynamic_offset || inner.integer_use || inner.phi_reached;
    let zero = inner.slots.contains_key(&0);
    let two = inner.slots.len() >= 2;
    let arr = {
        let slots = &inner.slots;
        super::is_array_shaped(slots, cx.ptr_size)
    };
    let why = if neg {
        "negative"
    } else if !two {
        "onefield"
    } else if arr {
        "arrayrun"
    } else if !zero {
        "nozero"
    } else {
        "clean"
    };
    tag(format!("{}/{}/{}", if fieldptr { "fptr" } else { "fint" }, if valptr { "vptr" } else { "vint" }, why));
}

/// Is this field type a pointer a record may be put behind?  A pointer to a
/// named composite already names its record.
fn is_record_pointer(ct: &Datatype) -> bool {
    ct.get_metatype() == type_metatype::TYPE_PTR && !points_at_named_composite(ct)
}

/// The pruned evidence of every value loaded from `off` of `rec`, merged, or
/// `None` when no load there is dereferenced or one of them fails a condition
/// that is about the value rather than its accesses.
fn loaded_record(data: &mut Funcdata, cx: &Nesting, rec: &Evidence, off: intb) -> Option<Evidence> {
    let mut values: Vec<VarnodeId> = Vec::new();
    for (width, v) in rec.loads.get(&off)? {
        if *width == cx.ptr_size && !values.contains(v) {
            values.push(*v);
        }
    }
    let mut merged = Evidence::default();
    for v in values {
        let vn = data.vbank().get(v)?;
        if vn.is_type_lock() || vn.is_persist() || vn.is_spacebase() {
            return None;
        }
        let ct = vn_type(data, v)?;
        if !is_record_pointer(&ct) {
            return None;
        }
        match cx.ev.get(&v) {
            Some(e) => merged.absorb(e),
            None => {
                let mut uses = Evidence::default();
                note_uses(data, v, &mut uses);
                merged.absorb(&uses);
            }
        }
    }
    if merged.slots.is_empty() {
        return None;
    }
    Some(merged.pruned())
}

/// Does the enclosing record hold everything the loaded one was seen to hold?
fn is_same_record(
    outer: &[TypeField],
    outer_size: int4,
    inner: &[TypeField],
    inner_size: int4,
    unclaimed: &[(int4, int4)],
) -> bool {
    let held = ledger::layout_of_fields(outer, outer_size, &[]);
    let measured = ledger::layout_of_fields(inner, inner_size, &[]);
    if measured.fields.is_empty() || !held.subsumes(&measured) {
        return false;
    }
    let held_members: Vec<ledger::Member> = outer.iter().map(ledger::Member::of).collect();
    let own_members: Vec<ledger::Member> = inner.iter().map(ledger::Member::of).collect();
    ledger::keeps_unclaimed(&held_members, &own_members, unclaimed)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::dtype::TypeFactoryImpl;

    fn factory() -> TypeFactoryImpl {
        let f = TypeFactoryImpl::new();
        f.set_default_alignment_map();
        f.set_max_basetype_size(8);
        f
    }

    fn field(off: int4, ct: &Rc<Datatype>) -> TypeField {
        TypeField::new(off, off, format!("field_0x{off:x}"), Rc::clone(ct))
    }

    /// grep's `treenext` reads the four fields of a node through its `llink`
    /// that it reads through the node itself; `_obstack_newchunk` reads the
    /// chunk's `prev` pointer at 8 and its `limit` at 0, where the obstack
    /// keeps an integer word and a pointer to that chunk.
    #[test]
    fn a_loaded_record_is_the_same_only_when_the_enclosing_one_holds_all_it_read() {
        let f = factory();
        let ptr = f.get_type_pointer(8, f.get_base(8, type_metatype::TYPE_UINT).unwrap(), 1).unwrap();
        let long = f.get_base(8, type_metatype::TYPE_INT).unwrap();
        let byte = f.get_base(1, type_metatype::TYPE_UINT).unwrap();
        let node = vec![field(0, &ptr), field(8, &ptr), field(0x10, &long), field(0x18, &byte)];
        assert!(is_same_record(&node, 0x20, &node, 0x20, &[]));
        let part = vec![field(0, &ptr), field(0x10, &long)];
        assert!(is_same_record(&node, 0x20, &part, 0x18, &[]));
        assert!(!is_same_record(&part, 0x18, &node, 0x20, &[]), "the loaded record reads more");

        let obstack = vec![field(0, &long), field(8, &ptr), field(0x10, &ptr)];
        let chunk = vec![field(0, &ptr), field(8, &ptr)];
        assert!(!is_same_record(&obstack, 0x18, &chunk, 0x10, &[]), "a pointer where the holder keeps an integer");

        // Bytes the loaded record touched without claiming must be laid out alike.
        let ints = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let split = vec![field(0, &ptr), field(8, &ints), field(0xc, &ints), field(0x10, &long)];
        let whole = vec![field(0, &ptr), field(0x10, &long)];
        assert!(is_same_record(&split, 0x18, &whole, 0x18, &[]));
        assert!(!is_same_record(&split, 0x18, &whole, 0x18, &[(8, 8)]));
    }
}
