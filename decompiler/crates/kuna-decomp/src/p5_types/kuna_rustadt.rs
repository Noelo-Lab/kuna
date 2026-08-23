//! (kuna, rustc) Give the two-register value `rustabi` keeps alive a NAME and a
//! SHAPE: a tagged two-variant aggregate, installed on the recovered prototype
//! and resolved to one variant per return site.
//!
//! # The symptom
//!
//! A `Result<u32,u32>` producer written with three explicit returns, `rustc -C
//! opt-level=2`, x86-64. With [`crate::kuna_rustabi`] the payload survives; this
//! is what it looks like before the pass below:
//!
//! ```text
//! undefined16 branchy(uint4 a0,uint4 a1)
//! {
//!   char v1 [16];
//!   if (a0 <= 10)   return ZEXT416((a1 + a0) * 3) << 0x40;   // which variant?
//!   if (0x65 <= a1) return ZEXT416(a1 - 1) << 0x40;
//!   v1[8] = a1 ^ a0;                                         // where is the payload?
//!   v1[0] = 1;                                               // what is byte 0?
//!   v1[0xc] = 0;
//!   return v1;
//! }
//! ```
//!
//! Everything is present and nothing is named. The tag is byte 0, the payload
//! starts at byte 8, and the two `<< 0x40` returns are variant constructions
//! whose tag is a literal zero the shift folded away.
//!
//! # What is synthesized, and why a struct-of-union rather than a new metatype
//!
//! `struct { uint<off> tag; union { Variant0; Variant1 } payload; }`.
//!
//! A union's fields are all forced to offset 0 (`assign_union_field_offsets`),
//! which *is* a variant overlay -- so the existing type model already expresses
//! this exactly, and adding a `type_metatype` would buy nothing and cost a lot:
//! `type_metatype::` is matched at ~1141 sites, mostly non-exhaustively (a new
//! variant would compile clean and behave wrong); `sub_metatype` is contiguous
//! `0..23` with the discriminant used as the propagation sort key; and
//! `metatype2string` writes a fixed vocabulary onto the Ghidra wire, where an
//! unknown string is an immediate decode failure.
//!
//! # The variant names are NEUTRAL, and that is a decision
//!
//! The members are `Variant0`/`Variant1`. They are never `Ok`/`Err` or
//! `Some`/`None`, because **tagged `Result` is `Ok = 0`, `Err = 1` and tagged
//! `Option` is `None = 0`, `Some = 1`** -- opposite polarity, identical code
//! generation, and nothing in the recovered shape separating them. A guessed name
//! is a coin flip that *reads* as a finding and would be wrong on roughly half the
//! population. On the 4.9 MB rustc measurement binary all five functions this pass
//! recovers are `Option`, so `Ok`/`Err` would have been wrong on every member of
//! every one; that binary also carries 4241 function symbols of which none names a
//! concrete `Result`/`Option` return, so the symbol channel does not rescue it.
//! Real names need corroboration from OUTSIDE the shape -- a DWARF
//! `DW_TAG_variant_part`, a demangled `drop_in_place<core::option::Option<…>>`
//! naming the same layout, an adjacent `core::result::unwrap_failed` callee --
//! and no such channel is implemented here.
//!
//! # Every condition is a refusal, and the refusal is all-or-nothing
//!
//! [`analyze`] wants: two or more `RETURN`s over one `join`-space geometry with
//! exactly two pieces; a tag that is a **constant on every path**; and exactly two
//! distinct tag values. A path whose tag is computed has no knowable variant, and
//! a partially classified union hands its remaining edges back to
//! `ScoreUnionFields` -- the thing this pass exists to keep out -- so one
//! unreadable path refuses the whole function.
//!
//! Two documented losses follow. The **branchless** rustc shape (`xor %eax,%eax;
//! setb %al`, one `RETURN`, the tag a flag) is out of scope, and it is the common
//! shape at `-C opt-level=2`: on the measurement binary only five functions of
//! 4194 meet the literal-tag precondition. Three-or-more variants are out of
//! scope. Niche-encoded enums (`Option<&T>` is bit-for-bit a nullable pointer,
//! with no tag at all) are out of scope in principle.
//!
//! # The facet comes from the guard, never from a score
//!
//! [`pin_facets`] is the reason the pass exists at all. `ScoreUnionFields` picks
//! a union member by type similarity, and variant selection is *control flow*:
//! the two members of a two-variant payload are routinely the same width and the
//! same metatype, so the trials tie and the winner is trial order. Scored, a
//! single basic block ends up reading `Variant0` and `Variant1` of the same
//! value. [`variant_regions`] instead marks every block from which the only
//! reachable `RETURN`s carry one tag -- which is exactly a block below the branch
//! that selected that variant -- and pins it there, into both the op-keyed cache
//! the printer's field-path descent reads and the address-keyed one
//! `Funcdata::resolve_in_flow` consults before it would build a scorer.
//!
//! # Where it runs
//!
//! [`ActionRustAdt`] sits between the `cleanup` rule pool and `ActionAssignHigh`.
//! Earlier, the concatenation has not reached its final shape and the payload has
//! no inferred type; later, the merge phase has already built the HighVariable
//! that `ActionOutputPrototype` reads the recovered return type out of.
//! [`ActionRustAdtFacet`] runs separately, immediately before `ActionSetCasts`,
//! because the resolution cache is keyed by `PcodeOp::getTime()` and must be
//! seeded on the final op graph.
//!
//! # The gate
//!
//! `option rustadt off|auto|always`. `auto` acts only when the loader's
//! source-language detection reported rustc, which the XML datatest bootstrap
//! never runs -- so the C corpus is untouched by construction, not by luck.
//! `always` drops the language test and is what the stage testcase uses, since a
//! `<bytechunk>` carries no `.comment`.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{ruleflags, Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use crate::funcdata::Funcdata;
use crate::unionresolve::ResolvedUnion;

/// How deep the constant/piece walks chase move-only operations.
const MAX_DEPTH: u32 = 16;

/// How many name candidates [`resolve_name`] tries before declining.
const MAX_NAME_ATTEMPTS: u32 = 8;

/// The number of variants this pass recognizes. Three-or-more is out of scope.
const NUM_VARIANTS: usize = 2;

/// The three values of `option rustadt`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RustAdtMode {
    /// Never act (the shipped default).
    Off,
    /// Act only when the loader detected a rustc-produced image.
    Auto,
    /// Act regardless of the detected source language.
    Always,
}

impl RustAdtMode {
    /// The `option rustadt <p1>` token for this mode.
    pub fn as_str(self) -> &'static str {
        match self {
            RustAdtMode::Off => "off",
            RustAdtMode::Auto => "auto",
            RustAdtMode::Always => "always",
        }
    }

    /// The wire encoding carried on `Architecture`/`ArchContext` (0/1/2).
    pub fn as_u8(self) -> u8 {
        match self {
            RustAdtMode::Off => 0,
            RustAdtMode::Auto => 1,
            RustAdtMode::Always => 2,
        }
    }

    /// Decode the wire encoding; anything unrecognized is [`RustAdtMode::Off`].
    pub fn from_u8(v: u8) -> RustAdtMode {
        match v {
            1 => RustAdtMode::Auto,
            2 => RustAdtMode::Always,
            _ => RustAdtMode::Off,
        }
    }
}

/// Parse `option rustadt off|auto|always`, returning the mode and the
/// confirmation message.
pub fn parse_rust_adt_mode(p1: &str) -> KunaResult<(RustAdtMode, String)> {
    let mode = match p1.trim().to_ascii_lowercase().as_str() {
        "off" | "0" | "false" => RustAdtMode::Off,
        "auto" | "on" | "1" | "true" => RustAdtMode::Auto,
        "always" => RustAdtMode::Always,
        _ => return Err(KunaError::parse("Must specify off, auto or always")),
    };
    Ok((mode, format!("Rust tagged-variant return typing set to {} form", mode.as_str())))
}

/// Is the tagged-variant rule live for this function?
pub fn live(data: &Funcdata) -> bool {
    match RustAdtMode::from_u8(data.get_arch().rust_adt) {
        RustAdtMode::Off => false,
        RustAdtMode::Auto => data.get_arch().source_is_rust,
        RustAdtMode::Always => true,
    }
}

// =============================================================================
// The observed shape
// =============================================================================

/// One RETURN of a function whose return value is a tagged two-variant pair.
#[derive(Clone, Debug)]
pub struct ReturnSite {
    /// The RETURN op.
    pub op: OpId,
    /// The value being returned (the `join`-space concatenation).
    pub vn: VarnodeId,
    /// The constant occupying the tag bytes on this path.
    pub tag: u64,
    /// The Varnode holding the payload bytes, when one exactly covers them.
    pub payload: Option<VarnodeId>,
}

impl ReturnSite {
    /// The payload this site's function actually WROTE, if any.
    ///
    /// A payload half with no definition anywhere is a register the function left
    /// alone -- the unit variant (`None`, `Ok(())`) -- and naming the leftover it
    /// happens to contain would present the caller's garbage as a returned value.
    pub fn written_payload(&self, data: &Funcdata) -> Option<VarnodeId> {
        let p = self.payload?;
        let v = data.vbank().get(p)?;
        if v.is_constant() || v.get_def().is_some() {
            Some(p)
        } else {
            None
        }
    }
}

/// What [`analyze`] found: a two-variant tagged return and every site that
/// builds one.
#[derive(Clone, Debug)]
pub struct AdtShape {
    /// Byte width of the whole returned value.
    pub total: int4,
    /// Byte offset of the payload half (== the least-significant piece's size).
    pub payload_off: int4,
    /// Byte width of the payload half.
    pub payload_size: int4,
    /// The two distinct tag constants, ascending.
    pub tags: [u64; NUM_VARIANTS],
    /// Every RETURN, in op order.
    pub sites: Vec<ReturnSite>,
}

impl AdtShape {
    /// The variant index (0 or 1) for a tag constant.
    pub fn variant_of(&self, tag: u64) -> Option<int4> {
        self.tags.iter().position(|&t| t == tag).map(|i| i as int4)
    }
}

/// Recognize the two-variant tagged return of `data`, or decline.
///
/// Every condition is a refusal, and they are the whole list: a `join`-space
/// return value with exactly two pieces, a tag that is a CONSTANT on **every**
/// path (a path whose tag is computed leaves the variant unknowable, and a
/// partially classified union is worse than none -- the unclassified sites fall
/// back to `ScoreUnionFields`, which is what this pass exists to keep out), and
/// exactly two distinct tag values across those paths.
pub fn analyze(data: &Funcdata) -> Option<AdtShape> {
    let rets: Vec<OpId> = data.obank().iter_code(OpCode::CPUI_RETURN).collect();
    if rets.len() < 2 {
        return None;
    }
    let mut geom: Option<(int4, int4, int4)> = None;
    let mut sites: Vec<ReturnSite> = Vec::new();
    for op in rets {
        let o = data.obank().get(op)?;
        if o.is_dead() || o.num_input() < 2 {
            return None;
        }
        let vn = o.get_in(1)?;
        let v = data.vbank().get(vn)?;
        if !v.get_addr().is_join() {
            return None;
        }
        let joff = v.get_addr().get_offset();
        let rec = data.get_arch().manage.find_join(joff).ok()?;
        if rec.num_pieces() != NUM_VARIANTS as i32 {
            return None;
        }
        // JoinRecord pieces run most-significant first: [0] is the payload half,
        // [1] the tag half that the little-endian value starts with.
        let hi_size = rec.get_piece(0).size as int4;
        let lo_size = rec.get_piece(1).size as int4;
        let total = v.get_size();
        if lo_size + hi_size != total || lo_size <= 0 || hi_size <= 0 || lo_size > 8 {
            return None;
        }
        match geom {
            None => geom = Some((total, lo_size, hi_size)),
            Some(g) if g == (total, lo_size, hi_size) => {}
            Some(_) => return None,
        }
        let tag = const_low(data, vn, lo_size, 0)?;
        let payload = piece_at(data, vn, lo_size, hi_size, 0)
            .or_else(|| widest_subpiece(data, vn, lo_size, hi_size));
        sites.push(ReturnSite { op, vn, tag, payload });
    }
    let (total, payload_off, payload_size) = geom?;
    let mut distinct: Vec<u64> = sites.iter().map(|s| s.tag).collect();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != NUM_VARIANTS {
        return None;
    }
    Some(AdtShape {
        total,
        payload_off,
        payload_size,
        tags: [distinct[0], distinct[1]],
        sites,
    })
}

/// The constant value of the low `nbytes` bytes of `vn`, when it is provably
/// constant.
///
/// This is the tag reader. It evaluates only the shapes rustc's two-register
/// return actually produces -- a `PIECE` concatenation, a zero-extension, and
/// the `ZEXT(payload) << 8*off` form the rule pool rewrites a zero tag into --
/// and answers `None` for everything else rather than guessing.
fn const_low(data: &Funcdata, vn: VarnodeId, nbytes: int4, depth: u32) -> Option<u64> {
    if depth >= MAX_DEPTH || nbytes <= 0 || nbytes > 8 {
        return None;
    }
    let v = data.vbank().get(vn)?;
    let bits = (nbytes as u32) * 8;
    let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
    if v.is_constant() {
        return Some(v.get_offset() & mask);
    }
    if nbytes > v.get_size() {
        return None;
    }
    let def = v.get_def()?;
    let op = data.obank().get(def)?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => const_low(data, op.get_in(0)?, nbytes, depth + 1),
        OpCode::CPUI_INT_ZEXT => {
            let in0 = op.get_in(0)?;
            let insz = data.vbank().get(in0)?.get_size();
            if nbytes <= insz {
                const_low(data, in0, nbytes, depth + 1)
            } else {
                // The low `insz` bytes come from the operand, the rest are zero.
                const_low(data, in0, insz, depth + 1)
            }
        }
        OpCode::CPUI_PIECE if op.num_input() == 2 => {
            let lo = op.get_in(1)?;
            let losz = data.vbank().get(lo)?.get_size();
            if nbytes <= losz {
                const_low(data, lo, nbytes, depth + 1)
            } else {
                let l = const_low(data, lo, losz, depth + 1)?;
                let h = const_low(data, op.get_in(0)?, nbytes - losz, depth + 1)?;
                Some(l | (h << ((losz as u32) * 8)))
            }
        }
        OpCode::CPUI_SUBPIECE => {
            let off = data.vbank().get(op.get_in(1)?)?;
            if off.is_constant() && off.get_offset() == 0 {
                const_low(data, op.get_in(0)?, nbytes, depth + 1)
            } else {
                None
            }
        }
        OpCode::CPUI_INT_LEFT => {
            let sh = data.vbank().get(op.get_in(1)?)?;
            if sh.is_constant() && sh.get_offset() >= bits as u64 {
                Some(0)
            } else {
                None
            }
        }
        OpCode::CPUI_MULTIEQUAL => {
            let mut answer: Option<u64> = None;
            for i in 0..op.num_input() {
                let c = const_low(data, op.get_in(i)?, nbytes, depth + 1)?;
                match answer {
                    None => answer = Some(c),
                    Some(a) if a == c => {}
                    Some(_) => return None,
                }
            }
            answer
        }
        _ => None,
    }
}

/// The Varnode whose value occupies exactly bytes `[off, off+sz)` of `vn`, when
/// one exists.
fn piece_at(data: &Funcdata, vn: VarnodeId, off: int4, sz: int4, depth: u32) -> Option<VarnodeId> {
    if depth >= MAX_DEPTH || sz <= 0 || off < 0 {
        return None;
    }
    let v = data.vbank().get(vn)?;
    // An exact cover IS the value: stop here rather than tracing through a COPY
    // to its source, so the answer names the storage the return site wrote. The
    // one thing looked through is a zero-extension, which carries no information
    // -- rustc widens a narrow payload to fill the register, and the payload's
    // logical width is the operand's.
    if off == 0 && sz == v.get_size() {
        if let Some(inner) = zext_operand(data, vn) {
            return piece_at(data, inner, 0, data.vbank().get(inner)?.get_size(), depth + 1)
                .or(Some(vn));
        }
        return Some(vn);
    }
    let Some(def) = v.get_def() else { return None };
    let op = data.obank().get(def)?;
    match op.code() {
        OpCode::CPUI_COPY => piece_at(data, op.get_in(0)?, off, sz, depth + 1),
        OpCode::CPUI_PIECE if op.num_input() == 2 => {
            let lo = op.get_in(1)?;
            let losz = data.vbank().get(lo)?.get_size();
            if off + sz <= losz {
                piece_at(data, lo, off, sz, depth + 1)
            } else if off >= losz {
                piece_at(data, op.get_in(0)?, off - losz, sz, depth + 1)
            } else {
                None
            }
        }
        OpCode::CPUI_INT_ZEXT => {
            let in0 = op.get_in(0)?;
            let insz = data.vbank().get(in0)?.get_size();
            if off + sz <= insz {
                piece_at(data, in0, off, sz, depth + 1)
            } else {
                None
            }
        }
        OpCode::CPUI_INT_LEFT => {
            let sh = data.vbank().get(op.get_in(1)?)?;
            if !sh.is_constant() || sh.get_offset() % 8 != 0 {
                return None;
            }
            let shb = (sh.get_offset() / 8) as int4;
            if off < shb {
                return None;
            }
            piece_at(data, op.get_in(0)?, off - shb, sz, depth + 1)
        }
        _ => None,
    }
}

/// The strictly narrower operand of a zero-extension defining `vn`, if that is
/// what defines it.
fn zext_operand(data: &Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
    let v = data.vbank().get(vn)?;
    let op = data.obank().get(v.get_def()?)?;
    if op.code() != OpCode::CPUI_INT_ZEXT {
        return None;
    }
    let in0 = op.get_in(0)?;
    if data.vbank().get(in0)?.get_size() < v.get_size() {
        Some(in0)
    } else {
        None
    }
}

/// The widest natural-width Varnode covering the START of `[off, off+cap)`.
///
/// rustc puts a `ScalarPair` payload at the bottom of its register, so a payload
/// narrower than the register (the common `Result<u32,u32>` in `RDX`) is found by
/// stepping the width down rather than by widening the search.
fn widest_subpiece(data: &Funcdata, vn: VarnodeId, off: int4, cap: int4) -> Option<VarnodeId> {
    for w in [8, 4, 2, 1] {
        if w > cap {
            continue;
        }
        if let Some(p) = piece_at(data, vn, off, w, 0) {
            return Some(p);
        }
    }
    None
}

// =============================================================================
// Synthesis
// =============================================================================

/// Build (or reuse) the `struct { tag; union { Variant0; Variant1 } }` for
/// `shape`, returning the outer struct.
pub fn synthesize(data: &Funcdata, shape: &AdtShape) -> Option<Rc<Datatype>> {
    let types = data.get_arch().types()?;
    let tagty = types.get_base(shape.payload_off, type_metatype::TYPE_UINT).ok()?;
    let mut vartys: Vec<Rc<Datatype>> = Vec::new();
    let mut varnames: Vec<String> = Vec::new();
    for k in 0..NUM_VARIANTS {
        let fieldty = variant_field_type(data, shape, k as int4)?;
        vartys.push(fieldty);
        varnames.push(format!("Variant{k}"));
    }
    let base = format!(
        "RustEnum2_{}_{}_{}_{}_{}_{}",
        shape.total,
        shape.payload_off,
        shape.tags[0],
        shape.tags[1],
        type_tag(&vartys[0]),
        type_tag(&vartys[1])
    );

    let mut variants: Vec<Rc<Datatype>> = Vec::new();
    for k in 0..NUM_VARIANTS {
        let n = resolve_name(types, &format!("{base}_{}", varnames[k]), shape.payload_size, type_metatype::TYPE_STRUCT)?;
        let existing = types.find_by_name(&n).ok()?;
        let ty = match existing {
            Some(t) if !t.is_incomplete() => t,
            _ => {
                let shell = types.get_type_struct(&n).ok()?;
                let w = vartys[k].get_size();
                let mut fd = vec![TypeField::new(0, 0, "f0", Rc::clone(&vartys[k]))];
                // rustc leaves the rest of the payload register alone, and the
                // producer's zeroing of it is real code; a named field renders it
                // as `_pad` rather than an artificial `_4_4_` byte range.
                if w < shape.payload_size {
                    if let Some(pad) = fallback_field(types, shape.payload_size - w) {
                        fd.push(TypeField::new(1, w, "_pad", pad));
                    }
                }
                types
                    .set_fields_struct_raw(&shell, fd, Vec::new(), shape.payload_size, 1, 0)
                    .unwrap_or(shell)
            }
        };
        variants.push(ty);
    }

    let un = resolve_name(types, &format!("{base}_Payload"), shape.payload_size, type_metatype::TYPE_UNION)?;
    let union_ty = match types.find_by_name(&un).ok()? {
        Some(t) if !t.is_incomplete() => t,
        _ => {
            let shell = types.get_type_union(&un).ok()?;
            let fd: Vec<TypeField> = (0..NUM_VARIANTS)
                .map(|k| TypeField::new(k as int4, 0, &varnames[k], Rc::clone(&variants[k])))
                .collect();
            types.set_fields_union_raw(&shell, fd, shape.payload_size, 1).unwrap_or(shell)
        }
    };

    let sn = resolve_name(types, &base, shape.total, type_metatype::TYPE_STRUCT)?;
    let outer = match types.find_by_name(&sn).ok()? {
        Some(t) if !t.is_incomplete() => t,
        _ => {
            let shell = types.get_type_struct(&sn).ok()?;
            let fd = vec![
                TypeField::new(0, 0, "tag", tagty),
                TypeField::new(1, shape.payload_off, "payload", Rc::clone(&union_ty)),
            ];
            types
                .set_fields_struct_raw(&shell, fd, Vec::new(), shape.total, 1, 0)
                .unwrap_or(shell)
        }
    };
    if outer.get_size() != shape.total {
        return None;
    }
    Some(outer)
}

/// The payload field type for variant `k`: the recovered type of the Varnode the
/// return sites carrying that tag put in the payload half, or an `undefined` of
/// the payload width when no single Varnode covers it.
fn variant_field_type(data: &Funcdata, shape: &AdtShape, k: int4) -> Option<Rc<Datatype>> {
    let types = data.get_arch().types()?;
    let tag = shape.tags[k as usize];
    let mut chosen: Option<Rc<Datatype>> = None;
    for s in shape.sites.iter().filter(|s| s.tag == tag) {
        let ty = match s.payload {
            Some(p) => Rc::clone(data.vbank().get(p)?.get_type()),
            None => return fallback_field(types, shape.payload_size),
        };
        match &chosen {
            None => chosen = Some(ty),
            Some(c) if c.get_size() == ty.get_size() => {}
            Some(_) => return fallback_field(types, shape.payload_size),
        }
    }
    match chosen {
        Some(c) if c.get_size() > 0 && c.get_size() <= shape.payload_size => Some(c),
        _ => fallback_field(types, shape.payload_size),
    }
}

/// The `undefined<n>` stand-in for a payload no single Varnode covers.
fn fallback_field(types: &dyn TypeFactory, size: int4) -> Option<Rc<Datatype>> {
    for w in [size, 8, 4, 2, 1] {
        if w <= 0 || w > size {
            continue;
        }
        if let Ok(t) = types.get_base(w, type_metatype::TYPE_UNKNOWN) {
            return Some(t);
        }
    }
    None
}

/// A short, stable spelling of a payload field type for the synthesized name.
///
/// Structural rather than nominal: an inferred payload type is frequently
/// anonymous (`get_name()` is empty for a base type), and the name has to be
/// deterministic because [`Datatype::hash_name`] turns it into the type id.
fn type_tag(ty: &Rc<Datatype>) -> String {
    let n: String = ty.get_name().chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
    if !n.is_empty() {
        return n;
    }
    let m = match ty.get_metatype() {
        type_metatype::TYPE_UINT => "u",
        type_metatype::TYPE_INT => "i",
        type_metatype::TYPE_BOOL => "b",
        type_metatype::TYPE_FLOAT => "f",
        type_metatype::TYPE_PTR => "p",
        type_metatype::TYPE_ARRAY => "a",
        type_metatype::TYPE_STRUCT => "s",
        type_metatype::TYPE_UNION => "n",
        type_metatype::TYPE_CODE => "c",
        _ => "x",
    };
    format!("{m}{}", ty.get_size())
}

/// A free, stable name of metatype `want` and size `size`.
///
/// [`Datatype::hash_name`] makes the registered NAME determine the type id, and
/// the factory refuses a second, different definition of a name it holds, so a
/// synthesized name that collides costs the whole type. Same policy as the DWARF
/// aggregate importer: take the name when it is free or already holds exactly
/// this shape, otherwise step to a suffixed one.
fn resolve_name(
    types: &dyn TypeFactory,
    base: &str,
    size: int4,
    want: type_metatype,
) -> Option<String> {
    for attempt in 0..MAX_NAME_ATTEMPTS {
        let name = if attempt == 0 { base.to_string() } else { format!("{base}_{attempt}") };
        match types.find_by_name(&name) {
            Ok(None) => return Some(name),
            Ok(Some(t)) => {
                if t.get_metatype() != want {
                    continue;
                }
                if t.is_incomplete() || t.get_size() == size {
                    return Some(name);
                }
            }
            Err(_) => return None,
        }
    }
    None
}

// =============================================================================
// Installation
// =============================================================================

/// Is every return site already carrying an installed variant type?
///
/// The pass sits in the repeating main loop rather than running once, because
/// the shape it reads only becomes visible after the rule pool has simplified
/// the concatenation the return-recovery built; this is what makes the retry
/// idempotent.
fn already_typed(data: &Funcdata, shape: &AdtShape) -> bool {
    shape.sites.iter().all(|s| {
        data.vbank()
            .get(s.vn)
            .map(|v| v.is_type_lock() && v.get_type().get_metatype() == type_metatype::TYPE_STRUCT)
            .unwrap_or(false)
    })
}

/// Install `ty` on every return site of `shape` and pin each site's union facet.
///
/// Returns the number of sites changed.
pub fn install(data: &mut Funcdata, shape: &AdtShape, ty: &Rc<Datatype>) -> int4 {
    let mut count = 0;
    for s in &shape.sites {
        if let Some(v) = data.vbank_mut().get_mut(s.vn) {
            if v.update_type_locked(Rc::clone(ty), true, true) {
                count += 1;
            }
        }
        // The variant constructor the printer renders at this site. Recorded for
        // every site; the printer applies it only where the returned value is a
        // single expression (see `PrintC::op_return_variant_ctor_ir`).
        if let Some(k) = shape.variant_of(s.tag) {
            data.kuna_set_rustadt_ctor(s.op, format!("Variant{k}"), s.written_payload(data));
        }
    }
    count
}

// =============================================================================
// Facet resolution: the variant comes from the guard, never from a score
// =============================================================================

/// The union member of an installed variant struct, if `ty` is one.
pub fn payload_union(ty: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    if ty.get_metatype() != type_metatype::TYPE_STRUCT {
        return None;
    }
    let f = ty.get_field(1)?;
    if f.field_type.get_metatype() != type_metatype::TYPE_UNION {
        return None;
    }
    if f.field_type.num_depend() != NUM_VARIANTS as int4 {
        return None;
    }
    Some(Rc::clone(&f.field_type))
}

/// Pin the union facet of every op that can only be reached with variant `k`
/// selected, at every return site of `shape`.
///
/// **This is the whole reason the pass exists.** `ScoreUnionFields` picks a union
/// member by type similarity, and variant selection is not a similarity question
/// -- it is control flow. Scored, the two members of a `Result` payload are
/// indistinguishable (identical width, frequently identical metatype), so the
/// scorer interleaves them and a single basic block ends up reading `Variant0`
/// and `Variant1` of the same value. The guard that fixed the tag on this path is
/// the answer, and it is exact.
///
/// Both caches are seeded. The op-keyed one is what the printer's field-path
/// descent reads directly (`Funcdata::get_union_resolution`); the address-keyed
/// one is slot-free and is what `Funcdata::resolve_in_flow` consults before it
/// would run the scorer, so an op created after this pass -- a cast plane
/// `PTRSUB`, say -- still gets the guard's answer rather than a score.
pub fn pin_facets(data: &mut Funcdata, shape: &AdtShape, ty: &Rc<Datatype>) -> int4 {
    let Some(union_ty) = payload_union(ty) else { return 0 };
    let Some(typegrp) = data.get_arch().types_rc() else { return 0 };
    let regions = variant_regions(data, shape);
    let mut count = 0;
    for (bl, k) in regions {
        let ops = data.bb_ops(bl);
        let mut addrs: Vec<kuna_base::address::Address> = Vec::new();
        for op in ops {
            let Some(o) = data.obank().get(op) else { continue };
            let n = o.num_input();
            let a = o.get_addr().clone();
            if !addrs.contains(&a) {
                addrs.push(a);
            }
            for slot in -1..n {
                let Ok(mut r) = ResolvedUnion::new_field(Rc::clone(&union_ty), k, typegrp.as_ref())
                else {
                    continue;
                };
                r.set_lock(true);
                data.set_union_field(&union_ty, op, slot, r);
                count += 1;
            }
        }
        for a in addrs {
            let Ok(mut r) = ResolvedUnion::new_field(Rc::clone(&union_ty), k, typegrp.as_ref())
            else {
                continue;
            };
            r.set_lock(true);
            data.set_address_based_union_field(&union_ty, &a, 0, r);
        }
    }
    count
}

/// A block whose reachable-return set has not been established.
const VARIANT_UNKNOWN: u8 = 0x80;

/// The basic blocks from which EVERY reachable `RETURN` carries the same tag,
/// paired with that tag's variant index.
///
/// This is the dominating guard, read off the CFG rather than off a dominator
/// query: a block all of whose forward paths end at variant `k` is exactly a
/// block below the branch that selected `k`. A block that can still reach both
/// variants, or whose successors this walk cannot see (the block graph is
/// hierarchical by the time the facet pass runs, so a successor may be a group
/// rather than a basic block), is left OUT -- an unpinned edge falls back to
/// today's behavior, whereas a wrongly pinned one is a confident lie.
fn variant_regions(data: &Funcdata, shape: &AdtShape) -> Vec<(crate::context::BlockId, int4)> {
    let mut blocks: Vec<crate::context::BlockId> = Vec::new();
    let mut index: std::collections::BTreeMap<crate::context::BlockId, usize> =
        std::collections::BTreeMap::new();
    for op in data.obank().iter_alive() {
        if let Some(bl) = data.obank().get(op).and_then(|o| o.get_parent()) {
            index.entry(bl).or_insert_with(|| {
                blocks.push(bl);
                blocks.len() - 1
            });
        }
    }
    let mut mask: Vec<u8> = vec![0; blocks.len()];
    for s in &shape.sites {
        let Some(k) = shape.variant_of(s.tag) else { continue };
        let Some(bl) = data.obank().get(s.op).and_then(|o| o.get_parent()) else { continue };
        if let Some(&i) = index.get(&bl) {
            mask[i] |= 1u8 << k;
        }
    }
    for _ in 0..=blocks.len() {
        let mut changed = false;
        for (i, &b) in blocks.iter().enumerate() {
            let nout = data.bblocks_ref().block(b).size_out();
            let mut add = 0u8;
            if nout == 0 && mask[i] == 0 {
                add |= VARIANT_UNKNOWN;
            }
            for j in 0..nout {
                let out = data.bblocks_ref().block(b).get_out(j);
                match index.get(&out) {
                    Some(&oi) if mask[oi] != 0 => add |= mask[oi],
                    _ => add |= VARIANT_UNKNOWN,
                }
            }
            if mask[i] | add != mask[i] {
                mask[i] |= add;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| match mask[i] {
            1 => Some((b, 0)),
            2 => Some((b, 1)),
            _ => None,
        })
        .collect()
}

// =============================================================================
// The action
// =============================================================================

/// Type the recovered two-register return as a tagged two-variant aggregate
/// (kuna `rustadt`; C++ has no analogue).
pub struct ActionRustAdt {
    base: ActionBase,
}

impl ActionRustAdt {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionRustAdt {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "rustadt", g),
        })
    }
}

impl Action for ActionRustAdt {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionRustAdt { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !live(data) {
            return 0;
        }
        let Some(shape) = analyze(data) else { return 0 };
        if already_typed(data, &shape) {
            return 0;
        }
        let Some(ty) = synthesize(data, &shape) else { return 0 };
        let n = install(data, &shape, &ty);
        self.base.count += n;
        n
    }
}

/// Pin the union facet of the installed variant type from the dominating guard
/// (kuna `rustadt`, the second half).
///
/// A separate action from [`ActionRustAdt`] because it must run on the FINAL op
/// graph: the resolution cache is keyed by `PcodeOp::getTime()`, and the pass
/// that installs the type sits in the repeating main loop, where the ops it would
/// pin are still being created and destroyed.
pub struct ActionRustAdtFacet {
    base: ActionBase,
}

impl ActionRustAdtFacet {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionRustAdtFacet {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "rustadtfacet", g),
        })
    }
}

impl Action for ActionRustAdtFacet {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionRustAdtFacet { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !live(data) {
            return 0;
        }
        let Some(shape) = analyze(data) else { return 0 };
        let ty = match data.vbank().get(shape.sites[0].vn) {
            Some(v) if v.is_type_lock() => Rc::clone(v.get_type()),
            _ => return 0,
        };
        if pin_facets(data, &shape, &ty) > 0 {
            self.base.count += 1;
        }
        0
    }
}

#[cfg(test)]
#[path = "kuna_rustadt/tests.rs"]
mod tests;
