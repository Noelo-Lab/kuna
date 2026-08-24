//! (kuna `dwarfvariants`) DWARF **discriminated-union** import —
//! `DW_TAG_variant_part` / `DW_AT_discr` / `DW_TAG_variant` /
//! `DW_AT_discr_value`, i.e. the layout a Rust tagged enum keeps there instead of
//! in `DW_TAG_member` children.
//!
//! ## What was missing
//!
//! [`super::kuna_dwarfstructs`] walks `DW_TAG_member` children. A Rust enum has
//! **none** — its entire layout hangs off a `DW_TAG_variant_part` — so it
//! recovered its `DW_AT_byte_size` and zero fields. A field-less aggregate is
//! still an aggregate the ABI classifier acts on, so an 8-byte
//! `fn(u32) -> Result<u32,u32>` came out
//! `core::result::Result<u32, u32> *ret_result(core::result::Result<u32, u32>
//! *rethidden,uint4 x)` — a phantom hidden-return parameter, with the real one
//! shifted a slot right, and the body then reading `x` out of the phantom.
//!
//! ## Why DWARF and not codegen
//!
//! The two questions a decompiler cannot answer from shape are *is this a
//! discriminated union* and *which variant is which*. Two return paths storing
//! different constants at offset 0 is also a `#[repr(C)] struct {kind, val}`, a
//! `(u64,u64)` tuple, a bitmask pair, and a `&'static str` fat pointer (whose
//! "discriminant" would be a `.rodata` address and whose "payload" would be the
//! length). DWARF states the answer: `DW_AT_discr` names the tag member, and each
//! `DW_TAG_variant`'s `DW_AT_discr_value` names the value that selects it. So no
//! name this module writes is an inference — every one of them is the source's.
//! Whether a given name may be WRITTEN AT ALL is a separate question, answered
//! under "What the union model cannot say" below.
//!
//! ## The honest limitation
//!
//! This needs **full debug info** (`-C debuginfo=2`, cargo `debug = true`).
//! Where a binary's DWARF carries no type DIEs the pass is not degraded, it is
//! INERT — it recovers nothing, and no fallback is attempted, because the only
//! available fallback is the shape inference above.
//!
//! ## The recovered type
//!
//! ```text
//!   struct <Enum> {                       // DW_AT_byte_size of the enum
//!       <tagtype> tag;                    // at DW_AT_discr's data_member_location
//!       union <Enum>::payload {           // at the lowest offset any variant uses
//!           struct <Enum>::Some { ... };  // one facet per payload-carrying
//!       } payload;                        // DW_TAG_variant
//!   };
//! ```
//!
//! …and, for the niche shape below, the union alone under the enum's own name.
//!
//! A facet is named after its `DW_TAG_variant` only when no OTHER variant claims
//! a byte it claims — see "What the union model cannot say" below. `Option<u32>`
//! is the shape above; `Result<u32,u32>`, whose two variants overlay each other,
//! gets one facet per variant named `field_0x4` instead of `Ok` / `Err`.
//!
//! Union members all sit at offset 0 (`TypeUnion::setFields`), which IS a variant
//! overlay, so this uses the existing type model rather than adding a
//! `type_metatype`: the enum is matched at ~1,700 sites in this workspace
//! (`grep -ro 'type_metatype::' --include=*.rs decompiler/crates | wc -l` reports
//! 1678 on this branch), mostly non-exhaustively, so a new variant would compile
//! clean and behave wrong; `sub_metatype` is a contiguous `0..23` propagation
//! sort key, and `metatype2string` writes a fixed vocabulary onto the Ghidra
//! wire.
//!
//! Facet fields are **re-based** to the payload overlay: DWARF gives a variant's
//! payload struct the width of the WHOLE enum with its members at their absolute
//! offsets (`Result<u32,u32>::Ok` is 8 bytes with `__0` at 4), which describes an
//! overlay at offset 0 — it cannot be placed beside a `tag` field at the same
//! offset. Subtracting the payload offset says the same thing in a shape the
//! struct model can hold.
//!
//! **Niche encoding.** A `DW_TAG_variant` with no `DW_AT_discr_value` is the
//! DEFAULT variant — "every value the others did not claim" — and is kept as
//! one. When the discriminant's bytes OVERLAP the payload (`Option<&T>` reads
//! the tag out of the reference itself) there is no byte range that is only the
//! tag, so the recovered type is the **overlay alone** — the union, at the
//! variants' own DWARF offsets, with no enclosing struct, because a `tag` field
//! would have to sit at an offset a variant already owns. The geometry still
//! reaches the side table, marked [`VariantLayout::niche`].
//!
//! ## What the union model cannot say, and what is therefore not printed
//!
//! A union member selects itself by OFFSET, not by the tag, so where two
//! variants claim the same bytes it is the union-field scorer — not the
//! discriminant — that decides which facet an access resolves to. For a tagged
//! enum that is not a corner case, it is the definition: every payload variant
//! begins immediately after the tag, so `Ok` and `Err` are at the SAME offset,
//! always, and the facet the scorer picks is not evidence of anything. Left
//! alone this prints actively false code — a `Result<u64,u64>` witness rendered
//! `Ok` on both arms of an if/else and `Err` nowhere in the whole binary, and a
//! consumer's `Err(e) => e + 100` came out `(long)v2.payload.Ok + 100`.
//!
//! So a variant name is installed only where it is FORCED:
//!
//! * [`label_for`] — the union member keeps the variant's name only when no
//!   other variant claims a byte this one claims. `Option<T>` has exactly one
//!   payload-carrying variant, so `Some` is unambiguous and survives; every
//!   `Result<T,E>`, and any enum with two payload variants overlaying each
//!   other, is spelled `field_0x<offset>` instead — the same offset-based
//!   rendering `dwarfvariants off` produces, which is what a reader gets when
//!   the answer is unknown.
//! * [`field_name_survives`] — a field inside a facet keeps its DWARF name only
//!   when every other variant either claims none of its bytes or names exactly
//!   that range the same way. rustc names tuple payloads `__0`/`__1`, so
//!   `Result`'s two `__0`s agree and the name claims nothing about which variant
//!   is live; a struct variant's `b` against a tuple variant's `__0` over the
//!   same word does not, and becomes `field_0x<offset>`.
//!
//! What is LOST is the label, not the layout: offsets, widths, member types and
//! the enum's own size are exactly what DWARF states either way, and every
//! variant's source name and `DW_AT_discr_value` are on [`VariantLayout`].
//! Picking the facet from the tag needs a dominating-guard analysis — that is
//! what the side table is recorded for, and it is not attempted here.
//!
//! ## Name collisions
//!
//! `Datatype::hash_name` makes the registered name the type id, and rustc names
//! variant payload structs BARE. Reproducible from this repo alone: the
//! committed `dwarfvariants_x86_64` fixture already carries **3 structure DIEs
//! named `Some`** at **two different widths** (8 and 16) and 2 each named `Ok`
//! and `Err` at two widths (1 and 8). On a std-linked `rustc 1.90
//! -C debuginfo=2` witness with 152 variant parts the same scan reports 61 DIEs
//! named `Some` across 8 byte sizes (0/8/12/16/24/32/48/64), 61 `None`, and 31
//! each `Ok`/`Err` across 7. So every name minted here is derived from the
//! enclosing enum's own PARENT-QUALIFIED name
//! (`core::result::Result<u32, u32>::Ok`), and a name still held by something of
//! a different shape is stepped over by
//! [`super::kuna_dwarfstructs::resolve_name`], the policy `dwarfstructs`
//! established, or by [`open_facet`] for a suppressed facet.
//!
//! ## Deliberate refusals
//!
//! Every one of these ends at the same place — a named aggregate with the enum's
//! `DW_AT_byte_size` and no fields, which is exactly what
//! [`super::kuna_dwarfstructs`] produces for the same DIE (a variant-part struct
//! has no `DW_TAG_member` children for its member walk to find) — rather than a
//! layout that is partly guessed.
//!
//! Refused BEFORE anything is interned, so the DIE simply falls through to
//! `dwarfstructs`:
//!
//! * no `DW_AT_discr` (a single-variant enum, `core::convert::Infallible`);
//! * a `DW_TAG_variant` with anything other than exactly one named member;
//! * two variants with no `DW_AT_discr_value`, or two with the same one, or two
//!   with the same NAME;
//! * a payload struct that carries its own `DW_TAG_variant_part` (a NESTED
//!   variant part — see the note on [`variant_fields`]);
//! * no variant with any field at all (that is a C-like enum, and rustc emits
//!   `DW_TAG_enumeration_type` for those);
//! * a discriminant whose type is not integer-shaped, or is zero-width, or would
//!   extend past the enum's `DW_AT_byte_size`;
//! * a zero-width or absent `DW_AT_byte_size`, and a `DW_AT_declaration` DIE.
//!
//! Refused AFTER the shell is interned — it has to exist before the members are
//! walked, so a recursive payload has something to point at. These do NOT return
//! `None`: that would leave a ZERO-SIZE incomplete type under the enum's own
//! name, the very defect this arm exists to remove. They funnel into one place
//! and [`seal_shell`] completes the shell at the enum's own width:
//!
//! * any member that would extend past the enum's `DW_AT_byte_size`;
//! * every facet's fields being unbuildable, which would leave a zero-member
//!   union describing nothing;
//! * the overlay union's name, or a facet's, being unmintable, or the factory
//!   refusing any of the three completions.
//!
//! The caller adds one more: the arm runs only when
//! [`super::kuna_dwarfstructs::enabled`] is also true, because it EXTENDS that
//! import and `dwarfstructs off` has to stay the name-only mapping its own
//! catalog row promises.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::types::{int4, uint4};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeField, TypeFactory};
use kuna_decomp::kuna_dwarfvariants::{VariantFacet, VariantLayout};

use super::kuna_dwarfstructs::{aggregate_alignment, aggregate_size, qualified_name, resolve_name};
use super::kuna_typedepth::TypeWalk;
use super::DieSnap;

/// Whether variant-part import is enabled for this process (the `dwarfvariants`
/// env bridge — the mapping happens inside `load file`, upstream of every
/// `option` command). The caller ALSO requires
/// [`super::kuna_dwarfstructs::enabled`]: this arm extends that one, and
/// `dwarfstructs off` has to stay the pre-DIV-86 name-only mapping its own
/// catalog row promises.
pub(super) fn enabled() -> bool {
    kuna_decomp::kuna_dwarfvariants::dwarfvariants_enabled()
}

/// One `DW_TAG_variant`, as read from DWARF and before any kuna type exists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RawVariant {
    /// `DW_AT_discr_value`; `None` marks the DEFAULT (niche) variant.
    pub discr: Option<i64>,
    /// The variant's `DW_TAG_member` name (`Ok`, `Err`, `Some`, `None`, …).
    pub name: String,
    /// `DW_AT_alignment` of the payload struct, when stated.
    pub alignment: Option<u64>,
    /// The payload struct's members as `(name, offset within the ENUM, type DIE)`.
    pub fields: Vec<(String, int4, Option<usize>)>,
}

/// A `DW_TAG_variant_part` as read from DWARF: the discriminant's placement plus
/// the variants. Independent of the type factory, so it is directly testable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RawVariantPart {
    /// The discriminant member's `DW_AT_data_member_location`.
    pub tag_offset: int4,
    /// The discriminant member's `DW_AT_type`.
    pub tag_type: Option<usize>,
    /// The variants, in DIE order.
    pub variants: Vec<RawVariant>,
}

/// The single `DW_TAG_variant_part` child of `die`, or `None` when there is not
/// exactly one (none ⇒ an ordinary aggregate; several ⇒ a shape no producer
/// emits, refused rather than guessed at).
pub(super) fn variant_part<'a>(
    die: &DieSnap,
    dies: &'a BTreeMap<usize, DieSnap>,
) -> Option<&'a DieSnap> {
    let mut found = None;
    for &coff in &die.children {
        let Some(c) = dies.get(&coff) else { continue };
        if c.tag != gimli::DW_TAG_variant_part {
            continue;
        }
        if found.is_some() {
            return None;
        }
        found = Some(c);
    }
    found
}

/// Read one variant's payload members: `(name, offset within the ENUM, type DIE)`.
///
/// `None` REFUSES the variant (and with it the whole enum). The refusals are the
/// payload not being a `DW_TAG_structure_type`, and the payload carrying its own
/// `DW_TAG_variant_part`. The second is a NESTED variant part: DWARF 5 admits it
/// and it would mean the payload's own fields are discriminant-selected, which
/// this module's single-level overlay cannot describe. It is a precaution rather
/// than an observed shape — rustc 1.90 emitted **0 nested variant parts across
/// the 75** in the two witnesses scanned for this change (the 10 in the
/// committed `#![no_std]` fixture and the 65 in a std-linked binary, which
/// includes an enum whose variant payload is another enum — that nests the TYPE
/// but not the variant part).
///
/// A member with no name, no buildable type, or a negative offset is SKIPPED, the
/// same policy `dwarfstructs`'s member walk uses: one exotic member costs its own
/// field, not the layout.
fn variant_fields(
    payload: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
) -> Option<Vec<(String, int4, Option<usize>)>> {
    if payload.tag != gimli::DW_TAG_structure_type {
        return None;
    }
    let mut out = Vec::new();
    for &coff in &payload.children {
        let Some(m) = dies.get(&coff) else { continue };
        if m.tag == gimli::DW_TAG_variant_part {
            return None;
        }
        if m.tag != gimli::DW_TAG_member || m.name.is_empty() {
            continue;
        }
        let off = m.data_member_location.unwrap_or(0);
        if off < 0 || off > int4::MAX as i64 {
            continue;
        }
        out.push((m.name.clone(), off as int4, m.type_ref));
    }
    Some(out)
}

/// Read the `DW_TAG_variant_part` of `die` into [`RawVariantPart`], or `None`
/// when any of the module header's refusals applies.
///
/// Pure over the DIE map — no type factory, no interning — so every refusal is
/// directly testable.
pub(super) fn read_variant_part(
    die: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
) -> Option<RawVariantPart> {
    let vp = variant_part(die, dies)?;
    // `DW_AT_discr` is the whole point: without it DWARF has not said which
    // member is the tag, and nothing else in the DIE says either.
    let tag = dies.get(&vp.discr_ref?)?;
    if tag.tag != gimli::DW_TAG_member {
        return None;
    }
    let tag_offset = tag.data_member_location.unwrap_or(0);
    if !(0..=int4::MAX as i64).contains(&tag_offset) {
        return None;
    }

    let mut variants: Vec<RawVariant> = Vec::new();
    for &coff in &vp.children {
        let Some(v) = dies.get(&coff) else { continue };
        if v.tag != gimli::DW_TAG_variant {
            continue;
        }
        // Exactly one named member: the variant's name and its payload struct.
        let mut member: Option<&DieSnap> = None;
        for &moff in &v.children {
            let Some(m) = dies.get(&moff) else { continue };
            if m.tag != gimli::DW_TAG_member {
                continue;
            }
            if member.is_some() || m.name.is_empty() {
                return None;
            }
            member = Some(m);
        }
        let member = member?;
        let payload = dies.get(&member.type_ref?)?;
        variants.push(RawVariant {
            discr: v.discr_value,
            name: member.name.clone(),
            alignment: payload.alignment,
            fields: variant_fields(payload, dies)?,
        });
    }
    if variants.is_empty() {
        return None;
    }
    // At most one DEFAULT variant, and no repeated value or name: each is a
    // contradiction in the layout, not a case to pick a winner in.
    if variants.iter().filter(|v| v.discr.is_none()).count() > 1 {
        return None;
    }
    for i in 1..variants.len() {
        let (head, tail) = variants.split_at(i);
        let v = &tail[0];
        if head.iter().any(|p| p.name == v.name) {
            return None;
        }
        if v.discr.is_some() && head.iter().any(|p| p.discr == v.discr) {
            return None;
        }
    }
    Some(RawVariantPart { tag_offset: tag_offset as int4, tag_type: tag.type_ref, variants })
}

/// Intern the discriminated union for `die` and populate it from DWARF.
///
/// `None` means REFUSED — the DIE is left to [`super::kuna_dwarfstructs`] and the
/// pre-`dwarfvariants` behaviour, which is a named aggregate with its byte size
/// and no fields. `Some` is the completed type — a struct of tag plus overlay
/// union, or the overlay union alone for a niche — with the geometry recorded on
/// the factory's side table.
#[allow(clippy::too_many_arguments)]
pub(super) fn intern_variant_aggregate(
    types: &dyn TypeFactory,
    die: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
    alias: Option<&str>,
    fallback: &str,
    walk: &mut TypeWalk,
    word_size: uint4,
    cpp: bool,
) -> Option<Rc<Datatype>> {
    if die.declaration {
        return None;
    }
    let size = aggregate_size(die)?;
    let raw = read_variant_part(die, dies)?;

    // The discriminant's type decides the shape below, and it is a base type or
    // an enum -- never the enclosing aggregate -- so it is resolved before
    // anything is interned.
    let tag_type = super::build_datatype(raw.tag_type, dies, types, word_size, walk, cpp)?;
    let tag_size = tag_type.get_size();
    if tag_size <= 0 || !is_discriminant_shaped(&tag_type) || raw.tag_offset + tag_size > size {
        return None;
    }

    // The payload overlay starts at the lowest offset ANY variant places a field
    // at. No field anywhere means this is a C-like enum wearing a variant part,
    // which rustc does not emit and this module will not synthesize an overlay
    // for.
    let lowest = raw.variants.iter().flat_map(|v| v.fields.iter().map(|f| f.1)).min()?;

    // NICHE: the discriminant's bytes and the payload's overlap, because Rust
    // read the tag out of the payload itself (`Option<&T>` uses the null
    // pointer). There is then no byte range that is only the tag, so the
    // recovered type is the overlay ALONE -- a union of the variants at their
    // DWARF offsets, which is exactly what the payload structs already describe.
    // A struct wrapping it would have to place a `tag` field at an offset a
    // variant already owns.
    let niche = raw.tag_offset.max(lowest) < (raw.tag_offset + tag_size).min(size);
    let payload_offset = if niche { 0 } else { lowest };
    let payload_size = size - payload_offset;
    if payload_size <= 0 {
        return None;
    }

    let want = if niche { type_metatype::TYPE_UNION } else { type_metatype::TYPE_STRUCT };
    let name = resolve_name(
        types,
        &qualified_name(die, dies, alias, fallback, Some(size)),
        Some(size),
        fallback,
        want,
    )?;
    let (shell, complete) = open_shell(types, &name, niche)?;
    if complete {
        return Some(shell);
    }
    if !walk.begin_aggregate(&name) {
        // An enclosing frame is already populating this enum (a recursive
        // payload, `enum List { Cons(u32, *const List), Nil }`); the shell it
        // interned is what the inner pointer must name.
        return Some(shell);
    }
    let built =
        build_facets(types, dies, &raw, &name, size, payload_offset, tag_size, walk, word_size, cpp);
    walk.end_aggregate(&name);

    // From here on the name is ALREADY interned, so a refusal must not return
    // `None`: that would leave a zero-size incomplete type under the enum's own
    // name -- the exact defect this arm exists to remove, and the way the sibling
    // `dwarfstructs` import silently degraded a type to `void`. Every remaining
    // refusal funnels into one `None` and is SEALED at the enum's
    // `DW_AT_byte_size` with no fields, which is precisely the answer
    // `dwarfstructs` gives the same DIE.
    let done = built.and_then(|(union_fields, facets)| {
        install_layout(
            types,
            die,
            &shell,
            &raw,
            &name,
            size,
            payload_offset,
            tag_size,
            tag_type,
            niche,
            union_fields,
            facets,
        )
    });
    Some(done.unwrap_or_else(|| seal_shell(types, die, shell, size, niche)))
}

/// Install the recovered layout on the already-interned `shell` and record its
/// geometry on the factory's side table.
///
/// Split out of [`intern_variant_aggregate`] so that every refusal reachable
/// after the shell exists lands on a single `None`, which the caller turns into
/// [`seal_shell`] rather than a zero-size type under the enum's own name.
#[allow(clippy::too_many_arguments)]
fn install_layout(
    types: &dyn TypeFactory,
    die: &DieSnap,
    shell: &Rc<Datatype>,
    raw: &RawVariantPart,
    name: &str,
    size: int4,
    payload_offset: int4,
    tag_size: int4,
    tag_type: Rc<Datatype>,
    niche: bool,
    union_fields: Vec<TypeField>,
    facets: Vec<VariantFacet>,
) -> Option<Rc<Datatype>> {
    let payload_size = size - payload_offset;
    let mut layout = VariantLayout {
        type_name: name.to_string(),
        size,
        tag_offset: raw.tag_offset,
        tag_size,
        payload_offset,
        niche,
        union_type: name.to_string(),
        variants: facets,
    };

    if niche {
        let align = facet_alignment(die.alignment, size, &union_fields);
        let done = types.set_fields_union_raw(shell, union_fields, size, align).ok()?;
        types.kuna_record_variant_layout(layout);
        return Some(done);
    }

    // Not a niche: a struct of the discriminant plus the overlay union.
    let uname = resolve_name(
        types,
        &format!("{name}::payload"),
        Some(payload_size),
        name,
        type_metatype::TYPE_UNION,
    )?;
    let (ushell, ucomplete) = open_shell(types, &uname, true)?;
    let union = if ucomplete {
        ushell
    } else {
        let ualign = facet_alignment(None, payload_size, &union_fields);
        types.set_fields_union_raw(&ushell, union_fields, payload_size, ualign).ok()?
    };
    layout.union_type = uname;

    let mut fields = vec![
        TypeField::new(payload_offset, payload_offset, "payload".to_string(), union),
        TypeField::new(raw.tag_offset, raw.tag_offset, "tag".to_string(), tag_type),
    ];
    fields.sort_by_key(|f| f.offset);
    let align = aggregate_alignment(die, size, &fields, &[]);
    let done = types.set_fields_struct_raw(shell, fields, Vec::new(), size, align, 0).ok()?;
    types.kuna_record_variant_layout(layout);
    Some(done)
}

/// Complete an interned-but-unpopulated shell at the enum's `DW_AT_byte_size`
/// with no fields — the answer [`super::kuna_dwarfstructs`] gives the same DIE
/// (a variant-part struct has no `DW_TAG_member` children for its member walk to
/// find). Never `None`: if the factory refuses the completion, the opaque shell
/// is still returned, because an opaque named aggregate beats `void`.
fn seal_shell(
    types: &dyn TypeFactory,
    die: &DieSnap,
    shell: Rc<Datatype>,
    size: int4,
    niche: bool,
) -> Rc<Datatype> {
    let align = aggregate_alignment(die, size, &[], &[]);
    let sealed = if niche {
        types.set_fields_union_raw(&shell, Vec::new(), size, align)
    } else {
        types.set_fields_struct_raw(&shell, Vec::new(), Vec::new(), size, align, 0)
    };
    sealed.unwrap_or(shell)
}

/// One variant's payload after its member types are built, before any facet type
/// is interned. `claims` is the variant's named byte ranges as
/// `(source name, offset within the ENUM, width)`, which is what the naming rule
/// in [`label_for`] and [`field_name_survives`] is decided on.
struct BuiltFacet {
    name: String,
    discr: Option<i64>,
    alignment: Option<u64>,
    fields: Vec<TypeField>,
    claims: Vec<(String, int4, int4)>,
}

/// Whether byte ranges `[ao, ao+asz)` and `[bo, bo+bsz)` share a byte.
fn ranges_overlap(ao: int4, asz: int4, bo: int4, bsz: int4) -> bool {
    ao < bo + bsz && bo < ao + asz
}

/// The union-member spelling for facet `i`: the variant's own name only when NO
/// other variant claims a byte this one also claims, and an offset-derived
/// `field_0x…` otherwise.
///
/// This is the whole naming rule. A union member selects itself by OFFSET, and
/// the discriminant is not consulted, so a variant name printed on an access
/// into a byte range two variants both claim is a coin flip — see the module
/// header. Where exactly one variant claims the range there is nothing to flip:
/// no other variant has anything meaningful there.
fn label_for(i: usize, built: &[BuiltFacet]) -> String {
    let me = &built[i];
    let contested = built.iter().enumerate().any(|(j, other)| {
        j != i
            && me.claims.iter().any(|(_, ao, asz)| {
                other.claims.iter().any(|(_, bo, bsz)| ranges_overlap(*ao, *asz, *bo, *bsz))
            })
    });
    if contested {
        format!("field_0x{:x}", me.claims.iter().map(|c| c.1).min().unwrap_or(0))
    } else {
        me.name.clone()
    }
}

/// Whether facet `i`'s field at `[off, off+width)` may keep its DWARF name: only
/// when every other variant either claims none of those bytes or names exactly
/// that range with exactly that spelling.
///
/// Two variants of the same tagged enum usually agree (rustc names every tuple
/// payload `__0`, `__1`, …), and where they agree the name claims nothing about
/// WHICH variant is live, so it is kept. Where they disagree — a struct variant's
/// `b` against a tuple variant's `__0` over the same word — the offset spelling
/// is used instead.
fn field_name_survives(i: usize, off: int4, width: int4, name: &str, built: &[BuiltFacet]) -> bool {
    built.iter().enumerate().all(|(j, other)| {
        j == i
            || other.claims.iter().all(|(on, oo, ow)| {
                !ranges_overlap(off, width, *oo, *ow)
                    || (*oo == off && *ow == width && on == name)
            })
    })
}

/// Whether an already-complete interned struct has exactly `fields`
/// (offset, name, width), i.e. is the same facet rather than a name collision.
fn same_layout(t: &Rc<Datatype>, fields: &[TypeField]) -> bool {
    if t.num_depend() != fields.len() as int4 {
        return false;
    }
    fields.iter().enumerate().all(|(i, f)| match t.get_field(i as int4) {
        Some(g) => {
            g.offset == f.offset
                && g.name == f.name
                && g.field_type.get_size() == f.field_type.get_size()
        }
        None => false,
    })
}

/// Intern the payload struct for one facet under `base`, stepping over a
/// same-named complete type whose layout is a DIFFERENT one.
///
/// The step-over matters because a suppressed facet's name is derived from its
/// offset, so two variants of the same enum can want the same one
/// (`Tree::field_0x8` for both `Leaf` and `Node`). Where the layouts match, the
/// single interned struct is shared, which is correct — the two facets describe
/// the same bytes. Where they differ they must not be merged: the ABI classifier
/// common-refines the union's members, so collapsing two shapes into one would
/// change how the enum is passed.
fn open_facet(
    types: &dyn TypeFactory,
    base: &str,
    fallback: &str,
    fields: Vec<TypeField>,
    size: int4,
    align: int4,
) -> Option<(Rc<Datatype>, String)> {
    for attempt in 0..8 {
        let cand = if attempt == 0 { base.to_string() } else { format!("{base}_{attempt}") };
        let fname =
            resolve_name(types, &cand, Some(size), fallback, type_metatype::TYPE_STRUCT)?;
        let (fshell, fcomplete) = open_shell(types, &fname, false)?;
        if fcomplete {
            if same_layout(&fshell, &fields) {
                return Some((fshell, fname));
            }
            continue;
        }
        let done = types.set_fields_struct_raw(&fshell, fields, Vec::new(), size, align, 0).ok()?;
        return Some((done, fname));
    }
    None
}

/// Build one payload struct per variant and the union members that name them.
///
/// Split out of [`intern_variant_aggregate`] so the `begin_aggregate` claim is
/// released on every path, including the refusals inside here. `payload_offset`
/// is what each facet's fields are RE-BASED by: DWARF gives a variant's payload
/// struct the width of the whole enum with its members at their absolute offsets
/// (`Result<u32,u32>::Ok` is 8 bytes with `__0` at 4), which describes an overlay
/// at offset 0 and cannot be placed beside a `tag` field there. In the niche
/// shape the overlay IS the whole type, so `payload_offset` is 0 and the offsets
/// go in verbatim.
///
/// The member types are built for EVERY variant first, because the naming rule
/// ([`label_for`], [`field_name_survives`]) needs the widths of all of them
/// before any name can be decided.
#[allow(clippy::too_many_arguments)]
fn build_facets(
    types: &dyn TypeFactory,
    dies: &BTreeMap<usize, DieSnap>,
    raw: &RawVariantPart,
    name: &str,
    size: int4,
    payload_offset: int4,
    tag_size: int4,
    walk: &mut TypeWalk,
    word_size: uint4,
    cpp: bool,
) -> Option<(Vec<TypeField>, Vec<VariantFacet>)> {
    let payload_size = size - payload_offset;
    // `DW_AT_discr_value` is masked to the discriminant's width, matching how the
    // printer looks a constant up: the Varnode carries the truncated value.
    let tag_mask: u64 = if tag_size >= 8 { u64::MAX } else { (1u64 << (tag_size * 8)) - 1 };

    let mut built: Vec<BuiltFacet> = Vec::new();
    for v in &raw.variants {
        let mut fields: Vec<TypeField> = Vec::new();
        let mut claims: Vec<(String, int4, int4)> = Vec::new();
        for (fname, foff, ftype) in &v.fields {
            let Some(fty) = super::build_datatype(*ftype, dies, types, word_size, walk, cpp)
            else {
                continue;
            };
            if fty.get_metatype() == type_metatype::TYPE_VOID || fty.get_size() <= 0 {
                continue;
            }
            if *foff + fty.get_size() > size || *foff < payload_offset {
                return None;
            }
            let width = fty.get_size();
            let rebased = foff - payload_offset;
            fields.push(TypeField::new(rebased, rebased, fname.clone(), fty));
            claims.push((fname.clone(), *foff, width));
        }
        fields.sort_by_key(|f| (f.offset, f.field_type.get_size()));
        fields.dedup_by_key(|f| f.offset);
        claims.sort_by_key(|c| (c.1, c.2));
        claims.dedup_by_key(|c| c.1);
        built.push(BuiltFacet {
            name: v.name.clone(),
            discr: v.discr,
            alignment: v.alignment,
            fields,
            claims,
        });
    }

    let mut union_fields: Vec<TypeField> = Vec::new();
    let mut facets: Vec<VariantFacet> = Vec::new();
    for i in 0..built.len() {
        // A FIELDLESS variant (`None`, `Nil`, a unit variant) overlays nothing,
        // so it gets no union facet: an empty struct of the overlay's width is
        // indistinguishable to `ScoreUnionFields` from the facet that does carry
        // the payload, and it wins the tie by declaration order. Ablating this
        // exclusion, a std-linked `rustc -g` witness writes an `Option<i64>`
        // payload as `v13.payload.None = ...` and reads drop glue as
        // `(*a0).dropfn.drop.None` (2 functions of 612, measured). The variant is
        // not lost: its name and its discriminant value are recorded on the side
        // table, which is where a `match` renderer reads them, and there is no
        // payload for a field path to reach anyway.
        let (payload_type, label) = if built[i].fields.is_empty() {
            (String::new(), String::new())
        } else {
            let label = label_for(i, &built);
            let renamed: Vec<TypeField> = built[i]
                .fields
                .iter()
                .map(|f| {
                    let abs = f.offset + payload_offset;
                    let width = f.field_type.get_size();
                    let fname = if field_name_survives(i, abs, width, &f.name, &built) {
                        f.name.clone()
                    } else {
                        format!("field_0x{abs:x}")
                    };
                    TypeField::new(f.offset, f.ident, fname, Rc::clone(&f.field_type))
                })
                .collect();
            let falign = facet_alignment(built[i].alignment, payload_size, &renamed);
            let (facet, fname) = open_facet(
                types,
                &format!("{name}::{label}"),
                name,
                renamed,
                payload_size,
                falign,
            )?;
            union_fields.push(TypeField::new(
                union_fields.len() as int4,
                0,
                label.clone(),
                facet,
            ));
            (fname, label)
        };
        facets.push(VariantFacet {
            name: built[i].name.clone(),
            discr: built[i].discr.map(|d| (d as u64) & tag_mask),
            label,
            payload_type,
            fields: built[i].claims.iter().map(|(n, o, _)| (n.clone(), *o)).collect(),
        });
    }

    // Every facet's fields were skipped (each member `void`, zero-width, or
    // unbuildable): there is no overlay left to install, and a zero-member union
    // describes nothing.
    if union_fields.is_empty() {
        return None;
    }
    Some((union_fields, facets))
}

/// A name a discriminant type may have: an integer, a boolean, or an enum.
/// Anything else (a pointer, a float, an aggregate) is not a discriminant this
/// module will read a `DW_AT_discr_value` against.
fn is_discriminant_shaped(t: &Rc<Datatype>) -> bool {
    matches!(
        t.get_metatype(),
        type_metatype::TYPE_UINT
            | type_metatype::TYPE_INT
            | type_metatype::TYPE_BOOL
            | type_metatype::TYPE_ENUM_UINT
            | type_metatype::TYPE_ENUM_INT
            | type_metatype::TYPE_UNKNOWN
    )
}

/// Intern `name` as a struct/union shell. `(type, already complete)` — a complete
/// one is reused verbatim, an incomplete one is the shell to populate. `None`
/// when the name is held by something that is not the wanted aggregate, or the
/// factory refuses it.
fn open_shell(
    types: &dyn TypeFactory,
    name: &str,
    is_union: bool,
) -> Option<(Rc<Datatype>, bool)> {
    let want = if is_union { type_metatype::TYPE_UNION } else { type_metatype::TYPE_STRUCT };
    match types.find_by_name(name) {
        Ok(Some(t)) if t.get_metatype() == want => {
            let complete = !t.is_incomplete();
            Some((t, complete))
        }
        Ok(Some(_)) => None,
        Ok(None) => {
            let built =
                if is_union { types.get_type_union(name) } else { types.get_type_struct(name) };
            built.ok().filter(|t| t.get_metatype() == want).map(|t| (t, false))
        }
        Err(_) => None,
    }
}

/// The alignment for a synthesized facet or overlay: the producer's
/// `DW_AT_alignment` when it stated one, else the widest member's, clamped to a
/// power of two no wider than the piece itself (`calc_align_size` would otherwise
/// pad it past the width DWARF gave).
fn facet_alignment(stated: Option<u64>, size: int4, fields: &[TypeField]) -> int4 {
    let derived = fields
        .iter()
        .map(|f| f.field_type.get_alignment())
        .max()
        .unwrap_or(1)
        .max(1);
    let align = stated.filter(|&a| a > 0 && a <= 64).map(|a| a as int4).unwrap_or(derived);
    let mut a = 1;
    while a * 2 <= align && a * 2 <= size {
        a *= 2;
    }
    a
}

#[cfg(test)]
mod tests;
