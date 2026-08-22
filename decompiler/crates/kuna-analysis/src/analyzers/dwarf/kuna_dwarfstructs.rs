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
//! ## Out of scope, deliberately
//!
//! `DW_TAG_variant_part` / `DW_TAG_variant` / `DW_AT_discr` — the Rust
//! tagged-enum encoding — are NOT read. A Rust enum therefore gets its correct
//! `DW_AT_byte_size` (which is most of the ABI benefit) and no fields, exactly as
//! any aggregate whose members this pass cannot ground.

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
    let name = resolve_name(types, &base, size, want)?;

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
fn aggregate_size(die: &DieSnap) -> Option<int4> {
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
fn qualified_name(
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
/// survive instead of the second silently inheriting the first's layout. `None`
/// when every candidate is taken — the caller then leaves the type unbuilt, which
/// is the pre-`dwarfstructs` outcome for a contested name.
fn resolve_name(
    types: &dyn TypeFactory,
    base: &str,
    size: Option<int4>,
    want: type_metatype,
) -> Option<String> {
    for attempt in 0..MAX_NAME_ATTEMPTS {
        let name = match (attempt, size) {
            (0, _) => base.to_string(),
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

/// Walk the `DW_TAG_member` children into kuna fields.
///
/// A member is SKIPPED (never fatal) when it has no name, no buildable type, a
/// zero-width or `void` type, or an offset that would put it outside the
/// aggregate — one exotic member costs its own field, not the whole layout.
/// Bitfields go to the second list; `DW_TAG_variant_part`/`DW_TAG_variant`
/// children are not members and are not read.
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
    let mut bits: Vec<TypeBitField> = Vec::new();
    let big_endian = types.is_big_endian();
    for &coff in &die.children {
        let Some(m) = dies.get(&coff) else { continue };
        if m.tag != gimli::DW_TAG_member || m.name.is_empty() {
            continue;
        }
        // A C++ `static` data member is a global, not part of the layout: it has
        // no `DW_AT_data_member_location` (upstream skips it the same way).
        let Some(ty) = super::build_datatype(m.type_ref, dies, types, word_size, walk, cpp) else {
            continue;
        };
        if ty.get_metatype() == type_metatype::TYPE_VOID || ty.get_size() <= 0 {
            continue;
        }
        if let Some(bf) = bitfield_geometry(m, big_endian) {
            let mut b = TypeBitField::new(bf.byte_offset, bf.num_bits, big_endian, m.name.clone(), ty);
            b.byte_offset = bf.byte_offset;
            b.byte_size = bf.byte_size;
            b.least_sig_bit = bf.least_sig_bit;
            bits.push(b);
            continue;
        }
        let off = if union { 0 } else { m.data_member_location.unwrap_or(0) };
        if off < 0 || off > int4::MAX as i64 {
            continue;
        }
        fields.push(TypeField::new(off as int4, off as int4, m.name.clone(), ty));
    }
    // `collect_bit_fields` / `has_bit_fields_in_range` binary-search both lists,
    // and the wire encoder interleaves them by offset: both must be sorted.
    fields.sort_by_key(|f| (f.offset, f.field_type.get_size()));
    fields.dedup_by_key(|f| f.offset);
    bits.sort_by_key(|b| (b.byte_offset + b.byte_size, b.least_sig_bit));
    (fields, bits)
}

/// The bit geometry of one bitfield member, in kuna's container frame.
struct BitGeom {
    byte_offset: int4,
    byte_size: int4,
    least_sig_bit: int4,
    num_bits: int4,
}

/// Read a `DW_TAG_member`'s bitfield geometry, or `None` for a plain member.
///
/// Two DWARF spellings are handled. DWARF 4/5 (what gcc and clang emit today)
/// gives `DW_AT_data_bit_offset` — the bit distance from the START of the
/// aggregate — and the container is taken as exactly the bytes the field spans.
/// DWARF 2/3 gives `DW_AT_data_member_location` + `DW_AT_byte_size` (the
/// container) + `DW_AT_bit_offset` (bits from the container's MOST significant
/// bit), which converts to a least-significant-bit position by subtraction.
///
/// LOSS: for a BIG-endian target only the DWARF 2/3 spelling is grounded; a
/// big-endian producer emitting `DW_AT_data_bit_offset` yields `None` and the
/// member is skipped rather than laid out on a guess.
fn bitfield_geometry(m: &DieSnap, big_endian: bool) -> Option<BitGeom> {
    let num_bits = m.bit_size.filter(|&b| b > 0 && b <= 64)? as int4;
    if let Some(dbo) = m.data_bit_offset {
        if big_endian {
            return None;
        }
        let byte_offset = (dbo / 8) as int4;
        let least_sig_bit = (dbo % 8) as int4;
        let byte_size = (least_sig_bit + num_bits + 7) / 8;
        return Some(BitGeom { byte_offset, byte_size, least_sig_bit, num_bits });
    }
    let byte_size = m.byte_size.filter(|&b| b > 0 && b <= 16)? as int4;
    let bit_offset = m.bit_offset? as int4;
    let byte_offset = m.data_member_location.filter(|&o| o >= 0)? as int4;
    let least_sig_bit = byte_size * 8 - bit_offset - num_bits;
    if least_sig_bit < 0 {
        return None;
    }
    Some(BitGeom { byte_offset, byte_size, least_sig_bit, num_bits })
}

/// The aggregate's alignment: `DW_AT_alignment` when the producer states it,
/// else the widest member alignment, clamped so it never exceeds the aggregate
/// itself (`calc_align_size` would otherwise pad a recovered type past the width
/// DWARF just gave us).
fn aggregate_alignment(
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
