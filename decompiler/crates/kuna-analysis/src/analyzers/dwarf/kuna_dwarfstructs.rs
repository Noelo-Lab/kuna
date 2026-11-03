//! (kuna `dwarfstructs`) DWARF aggregate LAYOUT import — `DW_AT_byte_size` plus
//! the `DW_TAG_member` children of a `DW_TAG_structure_type` /
//! `DW_TAG_union_type` / `DW_TAG_class_type`, installed on the interned type.
//!
//! ## What the mapper did instead
//!
//! [`super::intern_aggregate`] resolved every aggregate DIE to
//! `get_type_struct(name)` — a NAMED, EMPTY, ZERO-SIZE shell. The file's own
//! header stated the scope openly ("Type recovery is name/signature-level"), and
//! for a pointer-to-struct parameter that is enough. It is not enough anywhere
//! the *width* matters, and a size of zero is not a conservative answer — the
//! ABI classifier acts on it:
//!
//! * `int take_struct(P8 p,int k)` rendered `int take_struct(unsigned long,int)`
//!   — an 8-byte by-value struct parameter degraded to a raw integer;
//! * `P8 ret_struct(unsigned x)` rendered
//!   `P8 *ret_struct(P8 *rethidden,unsigned int x)` and then did arithmetic on
//!   `rethidden` — a sizeless return type is classified as a hidden-return-buffer
//!   call, so a *phantom* first parameter appears and the real one shifts;
//! * every field access rendered as a cast-and-offset (`*(int *)((long)n + 4)`).
//!
//! ## Origin (upstream Ghidra)
//!
//! `DWARFDataTypeImporter.makeDataTypeForStruct` / `makeDataTypeForUnion`:
//! create the composite at `DW_AT_byte_size`, iterate the `DW_TAG_member`
//! children, and place each at its `DW_AT_data_member_location`
//! (`DWARFUtil.getStructLayout` / `addStaticStructMember`), recursing on each
//! member's `DW_AT_type` through the same `getDataType` switch.
//!
//! ## Offsets are taken VERBATIM
//!
//! The layout is the compiler's own answer for this target's ABI, so it is
//! installed through [`TypeFactory::set_fields_struct_raw`] rather than
//! `assign_raw_fields_struct` (which re-derives offsets with the C packing
//! rules; with every offset already known it would also size the structure at
//! 0, because its running cursor only advances over the fields it places).
//!
//! ## Bitfields
//!
//! A member with `DW_AT_bit_size` is placed from its bit position in the
//! target's bit order — `DW_AT_data_bit_offset` (DWARF 4/5) or
//! `DW_AT_data_member_location` + `DW_AT_byte_size` + `DW_AT_bit_offset`
//! (DWARF 2/3) — into the SMALLEST byte span that covers it, which is the
//! geometry the compiler's own access agrees with (see [`group_bitfields`]). A
//! member that occupies whole aligned bytes of a natural width is not a bitfield
//! at all and is emitted as a plain field ([`byte_exact_bitfield`]) — exact in
//! either byte order, and the containment for a `BitFieldPullTransform`
//! divergence documented there. A member with `DW_AT_bit_size` that cannot be
//! placed is skipped.
//!
//! ## Two hazards this module exists to contain
//!
//! **Name collision.** The type factory interns by `(name, id = hash(name))` and
//! `find_add` REFUSES a second, different definition of a name it already holds
//! ("Trying to alter definition of type: X"). While every aggregate was a
//! sizeless shell that was invisible — two shells compare equal — but the moment
//! fields are installed it is live for every name that repeats. Rust is the
//! sharp case: `rustc -g` names each enum variant's payload struct BARE
//! (`Some`, `Ok`, `Err`, `None`), and a five-function witness carries four
//! distinct `Some` DIEs of sizes 16, 24, 16 and 12. So aggregates are interned
//! under their **parent-qualified** name (`core::option::Option<usize>::Some`,
//! reusing [`super::kuna_cppproto`]'s ancestry walk) and, if a name is STILL
//! held by an aggregate of a different size, under a size-suffixed variant
//! ([`resolve_name`]). A name held by a non-aggregate (kuna registers a core type
//! called `code`; zlib typedefs a struct to exactly that) is likewise stepped
//! over rather than fought.
//!
//! **Self-reference.** `struct node { struct node *next; }` reaches its own DIE
//! while its fields are being built. The interned shell is registered BEFORE the
//! members are walked, so the inner resolution finds it by name and the pointer
//! is built against it; [`TypeWalk::begin_aggregate`] additionally refuses the
//! re-entrant *population* so the recursion cannot restart. Documented LOSS: the
//! `Rc<Datatype>` the inner pointer captured is the pre-completion shell (kuna's
//! interned types are immutable, so completing one mints a new `Rc` — the same
//! model the wire `decode_composite` path lives with), so `n->next->val` chains
//! one level less than `n->val`. The name still renders.
//!
//! ## Out of scope for THIS module
//!
//! `DW_TAG_variant_part` / `DW_TAG_variant` / `DW_AT_discr` — the Rust
//! tagged-enum encoding — are not read here: with the member walk alone a Rust
//! enum gets its correct `DW_AT_byte_size` and no fields, exactly as any
//! aggregate whose members this pass cannot ground. The sibling
//! [`super::kuna_dwarfvariants`] arm (`option dwarfvariants`) reads them, runs
//! FIRST, and falls through to this module whenever it refuses; four of this
//! module's helpers ([`aggregate_size`], [`qualified_name`], [`resolve_name`],
//! [`aggregate_alignment`]) are shared with it rather than restated.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::types::{int4, uint4};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeBitField, TypeField, TypeFactory};

use super::kuna_typedepth::TypeWalk;
use super::DieSnap;

/// Whether aggregate-layout import is enabled for this process (the
/// `dwarfstructs` env bridge — the mapping happens inside `load file`, upstream
/// of every `option` command).
pub(super) fn enabled() -> bool {
    kuna_decomp::kuna_dwarfstructs::dwarfstructs_enabled()
}

/// Ancestry-walk bound for [`qualified_name`] (a nesting depth no real program
/// reaches).
const MAX_SCOPE_DEPTH: u32 = 32;

/// How many candidate names [`resolve_name`] will try before giving up and
/// leaving the aggregate opaque.
const MAX_NAME_ATTEMPTS: u32 = 4;

/// Intern the aggregate for `die` and populate it from DWARF.
///
/// Returns the completed type, or the opaque shell when the DIE carries no
/// layout to install (a `DW_AT_declaration` forward reference, or no
/// `DW_AT_byte_size`) — in which case the behavior is the pre-`dwarfstructs`
/// one. `None` only when no name can be interned at all, which the caller turns
/// into `void` exactly as before.
#[allow(clippy::too_many_arguments)]
pub(super) fn intern_aggregate(
    types: &dyn TypeFactory,
    die: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
    alias: Option<&str>,
    fallback: &str,
    walk: &mut TypeWalk,
    word_size: uint4,
    cpp: bool,
    union: bool,
) -> Option<Rc<Datatype>> {
    let want = if union { type_metatype::TYPE_UNION } else { type_metatype::TYPE_STRUCT };
    let size = aggregate_size(die);
    let base = qualified_name(die, dies, alias, fallback, size);
    let name = resolve_name(types, &base, size, fallback, want)?;

    // An aggregate already under this name is either a complete definition from
    // an earlier compilation unit or the shell an enclosing frame just interned
    // for a self-reference. Either way it is the answer; only an INCOMPLETE one
    // that nothing is currently populating may still be filled in below.
    let existing = match types.find_by_name(&name) {
        Ok(Some(t)) if t.get_metatype() == want => Some(t),
        Ok(_) => None,
        Err(_) => return None,
    };
    if let Some(t) = &existing {
        if !t.is_incomplete() {
            return Some(Rc::clone(t));
        }
    }

    let shell = match existing {
        Some(t) => t,
        None => {
            let built = if union { types.get_type_union(&name) } else { types.get_type_struct(&name) };
            built.ok().filter(|t| t.get_metatype() == want)?
        }
    };

    // Nothing to install: a forward declaration, or an aggregate the producer
    // gave no width. Leave the named opaque (the pre-`dwarfstructs` mapping).
    let Some(size) = size else { return Some(shell) };
    if die.declaration {
        return Some(shell);
    }
    if !walk.begin_aggregate(&name) {
        // An enclosing frame is already populating this aggregate; hand back the
        // shell it interned so the self-referential pointer still names it.
        return Some(shell);
    }
    let members = collect_members(die, dies, types, word_size, walk, cpp, union);
    walk.end_aggregate(&name);

    let (fields, bits) = members;
    let align = aggregate_alignment(die, size, &fields, &bits);
    let done = if union {
        types.set_fields_union_raw(&shell, fields, size, align)
    } else {
        let extra = if bits.is_empty() { 0 } else { kuna_decomp::dtype::flags::has_bitfields };
        types.set_fields_struct_raw(&shell, fields, bits, size, align, extra)
    };
    // A refused completion (a name the factory will not re-key) costs the layout,
    // never the type: the opaque shell is still a better answer than `void`.
    Some(done.unwrap_or(shell))
}

/// `DW_AT_byte_size` as a kuna type size. `None` for a DIE with no width (a
/// forward declaration, or a producer that omitted it); a zero or absurd width is
/// refused rather than installed.
pub(super) fn aggregate_size(die: &DieSnap) -> Option<int4> {
    let bs = die.byte_size?;
    if bs == 0 || bs > int4::MAX as u64 {
        return None;
    }
    Some(bs as int4)
}

/// The base interned name: the DIE's own `DW_AT_name` qualified by its
/// namespace/class/struct ancestry, else the typedef it was reached through, else
/// a `fallback` made distinctive by the shape it describes.
///
/// The qualification is what keeps Rust's bare variant names (`Some`, `Ok`)
/// apart; the shaped fallback is what keeps unrelated ANONYMOUS aggregates apart,
/// which mattered far less when every one of them was an empty shell.
pub(super) fn qualified_name(
    die: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
    alias: Option<&str>,
    fallback: &str,
    size: Option<int4>,
) -> String {
    if die.name.is_empty() {
        if let Some(a) = alias {
            return a.to_string();
        }
        let nmem = die
            .children
            .iter()
            .filter_map(|c| dies.get(c))
            .filter(|c| c.tag == gimli::DW_TAG_member)
            .count();
        return match size {
            Some(s) => format!("{fallback}_{s}_{nmem}"),
            None => fallback.to_string(),
        };
    }
    let mut path: Vec<&str> = Vec::new();
    let mut cur = die.parent;
    // The DIE tree is finite and acyclic, but bound the walk anyway: a corrupt
    // parent link must not hang the loader.
    for _ in 0..MAX_SCOPE_DEPTH {
        let Some(off) = cur else { break };
        let Some(p) = dies.get(&off) else { break };
        if matches!(
            p.tag,
            gimli::DW_TAG_namespace
                | gimli::DW_TAG_class_type
                | gimli::DW_TAG_structure_type
                | gimli::DW_TAG_union_type
        ) && !p.name.is_empty()
        {
            path.push(&p.name);
        }
        cur = p.parent;
    }
    if path.is_empty() {
        return die.name.clone();
    }
    path.reverse();
    path.push(&die.name);
    path.join("::")
}

/// Pick the name to intern under, stepping over a name the factory already holds
/// for something this aggregate is not.
///
/// `base` is tried first. If it is held by a non-aggregate, or by an aggregate of
/// a DIFFERENT size, the size-suffixed `base_<size>` is tried next (and then
/// numbered), so two same-named, different-sized aggregates in one binary both
/// survive instead of the second silently inheriting the first's layout. A DIE
/// with no width has no suffix to mint, so it gets one retry under the anonymous
/// `fallback` — the pre-`dwarfstructs` behavior for a contested borrowed name.
/// `None` when every candidate is taken; the caller then leaves the type unbuilt,
/// which is likewise the pre-`dwarfstructs` outcome.
pub(super) fn resolve_name(
    types: &dyn TypeFactory,
    base: &str,
    size: Option<int4>,
    fallback: &str,
    want: type_metatype,
) -> Option<String> {
    for attempt in 0..MAX_NAME_ATTEMPTS {
        let name = match (attempt, size) {
            (0, _) => base.to_string(),
            (1, None) if base != fallback => fallback.to_string(),
            (_, None) => return None,
            (1, Some(s)) => format!("{base}_{s}"),
            (n, Some(s)) => format!("{base}_{s}_{n}"),
        };
        match types.find_by_name(&name) {
            Ok(None) => return Some(name),
            Ok(Some(t)) => {
                if t.get_metatype() != want {
                    continue; // the name belongs to something else entirely
                }
                if t.is_incomplete() {
                    return Some(name); // an opaque shell we may complete or reuse
                }
                match size {
                    Some(s) if t.get_size() != s => continue,
                    _ => return Some(name),
                }
            }
            Err(_) => return None,
        }
    }
    None
}

/// Walk the `DW_TAG_member` children into kuna fields and bitfields.
///
/// A member is SKIPPED (never fatal) when it has no name, no buildable type, a
/// zero-width or `void` type, an unusable location or bit position, or is a
/// static member. A member with no location starts the container. LOSS: an
/// ANONYMOUS member is skipped rather than flattened into the parent. Struct
/// fields are sorted and deduplicated by offset; union fields keep declaration
/// order. Bitfields are sorted in memory order, which `find_matching_bit_field`
/// binary-searches.
fn collect_members(
    die: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
    types: &dyn TypeFactory,
    word_size: uint4,
    walk: &mut TypeWalk,
    cpp: bool,
    union: bool,
) -> (Vec<TypeField>, Vec<TypeBitField>) {
    let mut fields: Vec<TypeField> = Vec::new();
    let mut raw_bits: Vec<(i64, int4, String, Rc<Datatype>)> = Vec::new();
    let big_endian = types.is_big_endian();
    let end_bit = die.byte_size.and_then(|s| i64::try_from(s).ok()).and_then(|s| s.checked_mul(8));
    for &coff in &die.children {
        let Some(m) = dies.get(&coff) else { continue };
        if m.tag != gimli::DW_TAG_member || m.name.is_empty() || is_static_member(m) {
            continue;
        }
        let Some(ty) = super::build_datatype(m.type_ref, dies, types, word_size, walk, cpp) else {
            continue;
        };
        if ty.get_metatype() == type_metatype::TYPE_VOID || ty.get_size() <= 0 {
            continue;
        }
        if m.bit_size.is_some() {
            let inside = |&(start, num_bits): &(i64, int4)| {
                end_bit.map_or(true, |end| start + i64::from(num_bits) <= end)
            };
            let placed = bitfield_bits(m, ty.get_size(), big_endian).filter(inside);
            if let Some((start, num_bits)) = placed {
                raw_bits.push((start, num_bits, m.name.clone(), ty));
            }
            continue;
        }
        if union {
            fields.push(TypeField::new(fields.len() as int4, 0, m.name.clone(), ty));
            continue;
        }
        let Some(off) = member_offset(m) else { continue };
        if off < 0 || off > int4::MAX as i64 {
            continue;
        }
        fields.push(TypeField::new(off as int4, off as int4, m.name.clone(), ty));
    }
    // A union has no bitfield list (`TypeUnion::setFields` takes fields only), so
    // its true bitfields are a documented LOSS; byte-exact ones land at offset 0.
    let mut true_bits: Vec<(i64, int4, String, Rc<Datatype>)> = Vec::new();
    for (start, num_bits, name, ty) in raw_bits {
        match byte_exact_bitfield(start, num_bits, types, &ty) {
            Some((off, plain)) => {
                let off = if union { 0 } else { off };
                fields.push(TypeField::new(off, off, name, plain));
            }
            None if union => {}
            None => true_bits.push((start, num_bits, name, ty)),
        }
    }
    if !union {
        fields.sort_by_key(|f| (f.offset, f.field_type.get_size()));
        fields.dedup_by_key(|f| f.offset);
    }
    true_bits.sort_by_key(|b| b.0);
    (fields, group_bitfields(true_bits, big_endian))
}

/// A C++ `static` data member is a global, not part of the layout. DWARF 2-4
/// declares it as a `DW_TAG_member` marked external (upstream skips those) or
/// declaration-only; DWARF 5 uses `DW_TAG_variable` instead.
pub(super) fn is_static_member(m: &DieSnap) -> bool {
    m.external == Some(true) || m.declaration
}

/// A member's byte offset, as upstream `parseDataMemberOffset` reads it: an
/// absent `DW_AT_data_member_location` places the member at the start of its
/// container (DWARF permits omitting it there), and a location that is present
/// but not a constant displacement leaves the member unplaced.
pub(super) fn member_offset(m: &DieSnap) -> Option<i64> {
    match m.data_member_location {
        Some(offset) => Some(offset),
        None if m.member_location_unknown => None,
        None => Some(0),
    }
}

/// A bitfield's first bit and width, or `None` when it cannot be placed. The bit
/// counts from the start of the aggregate in the target's bit order, as
/// `DW_AT_data_bit_offset` does: from the least significant bit of byte 0 on a
/// little-endian target, from its most significant bit on a big-endian one.
/// DWARF 2/3 `DW_AT_bit_offset` counts from the most significant bit of a
/// storage unit of `DW_AT_byte_size` bytes (the member type's size when absent);
/// without either bit offset the member starts at its byte offset.
fn bitfield_bits(m: &DieSnap, type_size: int4, big_endian: bool) -> Option<(i64, int4)> {
    let num_bits = m.bit_size.filter(|&b| b > 0 && b <= 64)? as i64;
    let base = || member_offset(m).filter(|&o| o >= 0)?.checked_mul(8);
    let start = match (m.data_bit_offset, m.bit_offset) {
        (Some(dbo), _) => i64::try_from(dbo).ok()?,
        (None, None) => base()?,
        (None, Some(bit_offset)) => {
            let unit = match m.byte_size {
                Some(b) => i64::try_from(b).ok().filter(|&b| b > 0 && b <= 16)?,
                None => i64::from(type_size),
            };
            let within = match big_endian {
                true => bit_offset,
                false => (unit * 8 - num_bits).checked_sub(bit_offset)?,
            };
            base()?.checked_add(within)?
        }
    };
    let end = start.checked_add(num_bits)?;
    (start >= 0 && end / 8 <= i64::from(int4::MAX)).then_some((start, num_bits as int4))
}

/// A bitfield covering whole aligned bytes of a natural width, as a plain field:
/// `(struct byte offset, resized member type)`, exact in either byte order.
/// `None` for a real bitfield. Such fields stay out of the bitfield list because
/// `BitFieldPullTransform` (`p5_types/bitfield/pull.rs`) asserts in
/// `op_destroy_recursive` when one is read through a load of exactly its bytes.
fn byte_exact_bitfield(
    start: i64,
    num_bits: int4,
    types: &dyn TypeFactory,
    ty: &Rc<Datatype>,
) -> Option<(int4, Rc<Datatype>)> {
    if start % 8 != 0 || num_bits % 8 != 0 {
        return None;
    }
    let bytes = num_bits / 8;
    if !matches!(bytes, 1 | 2 | 4 | 8) {
        return None;
    }
    let off = int4::try_from(start / 8).ok()?;
    let plain = if ty.get_size() == bytes {
        Rc::clone(ty)
    } else {
        types.resize_integer(Rc::clone(ty), bytes).ok()?
    };
    (plain.get_size() == bytes).then_some((off, plain))
}

/// Fold the bitfields into kuna's container frame: each gets the SMALLEST byte
/// span that covers it, the span the compiler's own access and the printer's
/// `.`-versus-`->` test (`checkBitFieldMember`) agree on, and its least
/// significant bit within that span read in the target's byte order.
fn group_bitfields(
    raw: Vec<(i64, int4, String, Rc<Datatype>)>,
    big_endian: bool,
) -> Vec<TypeBitField> {
    let mut out: Vec<TypeBitField> = Vec::new();
    for (start, num_bits, name, ty) in raw {
        let byte_offset = (start / 8) as int4;
        let least_sig_bit = if big_endian {
            ((8 - (start + i64::from(num_bits)) % 8) % 8) as int4
        } else {
            (start % 8) as int4
        };
        let byte_size = (least_sig_bit + num_bits + 7) / 8;
        let mut b = TypeBitField::new(byte_offset, num_bits, big_endian, name, ty);
        b.byte_offset = byte_offset;
        b.byte_size = byte_size;
        b.least_sig_bit = least_sig_bit;
        out.push(b);
    }
    out
}

/// The aggregate's alignment: `DW_AT_alignment` when the producer states it,
/// else the widest member alignment, clamped so it never exceeds the aggregate
/// itself (`calc_align_size` would otherwise pad a recovered type past the width
/// DWARF just gave us).
pub(super) fn aggregate_alignment(
    die: &DieSnap,
    size: int4,
    fields: &[TypeField],
    bits: &[TypeBitField],
) -> int4 {
    let stated = die.alignment.filter(|&a| a > 0 && a <= 64).map(|a| a as int4);
    let derived = fields
        .iter()
        .map(|f| f.field_type.get_alignment())
        .chain(bits.iter().map(|b| b.field_type.get_alignment()))
        .max()
        .unwrap_or(1)
        .max(1);
    let align = stated.unwrap_or(derived);
    // A power-of-two alignment no wider than the aggregate: `calc_align_size`
    // rounds the size UP to a multiple of it, and DWARF already told us the size.
    let mut a = 1;
    while a * 2 <= align && a * 2 <= size {
        a *= 2;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_decomp::dtype::TypeFactoryImpl;

    fn die(tag: gimli::DwTag, name: &str) -> DieSnap {
        let mut d = DieSnap::new(tag, 1);
        d.name = name.into();
        d
    }

    #[test]
    fn absent_locations_start_the_container_and_unknown_or_static_members_are_skipped() {
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(8), 8, 4);
        let _ = types.cache_core_types();
        let mut dies = BTreeMap::new();
        let mut int = die(gimli::DW_TAG_base_type, "int");
        int.byte_size = Some(4);
        int.encoding = Some(gimli::DW_ATE_signed);
        dies.insert(1, int);
        let mut record = die(gimli::DW_TAG_structure_type, "Layout");
        record.byte_size = Some(8);
        record.children = vec![2, 3, 4, 5, 6];
        dies.insert(10, record);
        let member = |name: &str| {
            let mut m = die(gimli::DW_TAG_member, name);
            m.type_ref = Some(1);
            m
        };
        dies.insert(2, member("first"));
        let mut second = member("second");
        second.data_member_location = Some(4);
        dies.insert(3, second);
        let mut virtual_base = member("virtual_base");
        virtual_base.member_location_unknown = true;
        dies.insert(4, virtual_base);
        let mut shared = member("shared");
        shared.external = Some(true);
        dies.insert(5, shared);
        let mut declared = member("declared");
        declared.declaration = true;
        dies.insert(6, declared);
        let ty = intern_aggregate(&types, &dies[&10], &dies, None, "Layout",
            &mut TypeWalk::with_gate(false), 8, false, false).unwrap();
        let fields: Vec<_> = (0..ty.num_depend()).filter_map(|i| ty.get_field(i))
            .map(|f| (f.offset, f.name.clone())).collect();
        assert_eq!(fields, vec![(0, "first".to_string()), (4, "second".to_string())]);
    }

    type Layout = (Vec<(int4, int4, String)>, Vec<(String, int4, int4, int4)>);

    /// An `unsigned int` bitfield whose first bit, in the target's bit order, is
    /// `start`: spelled `DW_AT_data_bit_offset` (`dwarf5`), or as a 4-byte unit
    /// with `DW_AT_bit_offset` and a location.
    fn bitfield(name: &str, start: u64, width: u64, big_endian: bool, dwarf5: bool) -> DieSnap {
        let mut m = die(gimli::DW_TAG_member, name);
        m.type_ref = Some(1);
        m.bit_size = Some(width);
        if dwarf5 {
            m.data_bit_offset = Some(start);
        } else {
            let within = (start % 32) as i64;
            m.byte_size = Some(4);
            m.bit_offset = Some(if big_endian { within } else { 32 - within - width as i64 });
            m.data_member_location = Some((start / 32 * 4) as i64);
        }
        m
    }

    /// The `(offset, size, name)` fields and `(name, byte offset, byte size, least
    /// significant bit)` bitfields of a 12-byte struct of `members`.
    fn layout(big_endian: bool, members: Vec<DieSnap>) -> Layout {
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(8), 8, 4);
        types.set_truncate_big_endian(big_endian);
        let _ = types.cache_core_types();
        let mut dies = BTreeMap::new();
        let mut uint = die(gimli::DW_TAG_base_type, "unsigned int");
        uint.byte_size = Some(4);
        uint.encoding = Some(gimli::DW_ATE_unsigned);
        dies.insert(1, uint);
        let mut record = die(gimli::DW_TAG_structure_type, "Bits");
        record.byte_size = Some(12);
        record.children = (2..2 + members.len()).collect();
        for (i, m) in members.into_iter().enumerate() {
            dies.insert(2 + i, m);
        }
        let (fields, bits) = collect_members(&record, &dies, &types, 8,
            &mut TypeWalk::with_gate(false), false, false);
        (fields.iter().map(|f| (f.offset, f.field_type.get_size(), f.name.clone())).collect(),
         bits.iter()
            .map(|b| (b.name.clone(), b.byte_offset, b.byte_size, b.least_sig_bit))
            .collect())
    }

    #[test]
    fn bitfields_follow_the_target_bit_order_in_either_spelling() {
        // struct { unsigned lo:3, hi:5, wide:16, top:8; unsigned a:4, b:12, c:16; }
        let declared = [("lo", 0, 3), ("hi", 3, 5), ("wide", 8, 16), ("top", 24, 8),
            ("a", 32, 4), ("b", 36, 12), ("c", 48, 16)];
        let fields = vec![(1, 2, "wide".to_string()), (3, 1, "top".into()), (6, 2, "c".into())];
        for (big_endian, bits) in [
            (true, [("lo", 0, 1, 5), ("hi", 0, 1, 0), ("a", 4, 1, 4), ("b", 4, 2, 0)]),
            (false, [("lo", 0, 1, 0), ("hi", 0, 1, 3), ("a", 4, 1, 0), ("b", 4, 2, 4)]),
        ] {
            let bits: Vec<_> = bits.iter().map(|&(n, o, s, l)| (n.to_string(), o, s, l)).collect();
            for dwarf5 in [false, true] {
                let members = declared.iter()
                    .map(|&(n, s, w)| bitfield(n, s, w, big_endian, dwarf5)).collect();
                assert_eq!(layout(big_endian, members), (fields.clone(), bits.clone()),
                    "big_endian={big_endian} dwarf5={dwarf5}");
            }
        }
    }

    #[test]
    fn bitfields_are_listed_in_memory_order() {
        // struct { unsigned a:6, c:4, d:6; }: c and d end in the same byte.
        for (big_endian, expected) in [
            (true, [("a", 0, 1, 2), ("c", 0, 2, 6), ("d", 1, 1, 0)]),
            (false, [("a", 0, 1, 0), ("c", 0, 2, 6), ("d", 1, 1, 2)]),
        ] {
            let members = [("a", 0, 6), ("c", 6, 4), ("d", 10, 6)].iter()
                .map(|&(n, s, w)| bitfield(n, s, w, big_endian, true)).collect();
            let expected: Vec<_> =
                expected.iter().map(|&(n, o, s, l)| (n.to_string(), o, s, l)).collect();
            assert_eq!(layout(big_endian, members).1, expected, "big_endian={big_endian}");
        }
    }

    #[test]
    fn bitfields_without_a_bit_offset_or_with_a_negative_one_are_placed() {
        let mut packed = bitfield("packed", 0, 30, false, false);
        packed.bit_offset = Some(-6);
        let mut bare = die(gimli::DW_TAG_member, "bare");
        bare.type_ref = Some(1);
        bare.bit_size = Some(4);
        bare.data_member_location = Some(8);
        let mut too_wide = bitfield("too_wide", 64, 65, false, true);
        too_wide.bit_size = Some(65);
        let past_the_end = bitfield("past_the_end", 92, 8, false, true);
        let (fields, bits) = layout(false, vec![packed, bare.clone(), too_wide, past_the_end]);
        assert!(fields.is_empty(), "{fields:?}");
        assert_eq!(bits, vec![("packed".to_string(), 1, 4, 0), ("bare".into(), 8, 1, 0)]);
        assert_eq!(layout(true, vec![bare]).1, vec![("bare".to_string(), 8, 1, 4)]);
    }
}
