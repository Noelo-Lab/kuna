//! (kuna `variantguard`) Pick a Rust enum's variant from the DWARF discriminant
//! and the control flow that fixed it, instead of from a union-field score.
//!
//! This is the **dominating-guard analysis** the `dwarfvariants` importer
//! (DIV-87) records its side table for, and it is the second half of that
//! feature rather than a new claim about codegen.
//!
//! ## The gap it closes
//!
//! `dwarfvariants` installs a Rust tagged enum as `struct { tag; union payload; }`
//! and records the exact geometry of the variant part on the `TypeFactory`
//! ([`crate::kuna_dwarfvariants::VariantLayout`]): the discriminant's offset and
//! width, and per variant its source name, its `DW_AT_discr_value` and its
//! payload struct.  A union member, though, **selects itself by offset** — the
//! discriminant is never consulted — and in a tagged enum every payload variant
//! begins immediately after the tag, so `Ok.__0` and `Err.__0` sit at the same
//! offset with the same width and `ScoreUnionFields` ties between them.  Which
//! one wins is trial order, not evidence.  So `dwarfvariants` refuses to write a
//! variant name wherever two variants overlay one byte range and installs an
//! offset-derived `field_0x…` label instead ([`VariantFacet::label`]) — correct,
//! and mute.
//!
//! Control flow is the missing evidence, and the compiler already stated the
//! other half of it.  A block below `if (r.tag == 1)` is a block where the
//! discriminant is 1, and DWARF says which variant the value 1 selects.  Neither
//! step guesses.
//!
//! ## What is proved, in three steps
//!
//! **1. Seeds.**  Two kinds, both anchored on a Varnode whose *recovered type* is
//! the enum (or a pointer to it) — so nothing here can fire on a C struct, a
//! tuple or a fat pointer, whatever their codegen shape:
//!
//! * a **consumer** guard: a `CBRANCH` whose condition is an equality (possibly
//!   masked, truncated, zero-extended or negated) against the bytes at
//!   `tag_offset`.  The predicate is evaluated over the enum's *discriminant
//!   domain* — every `DW_AT_discr_value`, plus the DEFAULT variant, which is
//!   possible on an edge exactly when some value **outside** the claimed set
//!   satisfies that edge's predicate (counted exactly, not assumed).  Each of the
//!   branch's two out-edges gets the variant set its side of the predicate
//!   allows.
//! * a **producer** store: a constant written over the tag bytes of the object,
//!   which selects the facet [`VariantLayout::facet_for_discr`] names — the niche
//!   variant included, since that is precisely the "every value the others did
//!   not claim" case.
//!
//! The two are evidence about **different things**, and are deliberately not
//! mixed.  A guard is evidence about the value an execution is *carrying*, so it
//! colours every op its edge reaches, reads included.  A store is evidence about
//! what the object *becomes*, so it colours **writes only** — see
//! [`producer_writes`], which is where an earlier revision of this pass got it
//! badly wrong and is worth reading before changing anything here.
//!
//! **2. The kill discipline, which both halves share.**  Every defect this pass
//! shipped was one thing: *a fact about an object's discriminant applied at a
//! program point where it no longer held*.  A guard proves what the object was
//! AT THE GUARD; a store proves what it becomes.  Neither survives an **event**
//! ([`object_events`]): a store over the tag bytes, a call (a callee handed the
//! pointer may store through it), or any store this pass cannot attribute to the
//! object at all (it may alias it).  A **value** object has no events, because an
//! SSA value cannot be clobbered — which is why a `match` on a returned enum
//! keeps its names while the same shape behind a pointer loses them across a
//! call.
//!
//! **3. Regions.**  A forward may-analysis over the basic-block graph propagates
//! the guard edge constraints: a block is reachable only under the union of what
//! its in-edges allow, and the fact LEAVING a block that holds any event is the
//! whole set.  Within a block the kill is positional ([`Regions::guard_at`]), so
//! a read above a clobber keeps the guard and one below it does not.  A guard's
//! own `CBRANCH` is its block's last op, so a re-test after a clobber still
//! constrains the edge.  Only a **singleton** set is a fact.
//!
//! The two facts are **intersected** at the pin and a disagreement REFUSES.
//! Neither outranks the other: a revision that gave a singleton guard region
//! precedence discarded a correctly-computed producer fact and named a write
//! that builds `Err`, inside an `Ok`-guarded block, `Ok` — one line below its own
//! `tag = 1`.
//!
//! **4. Uses.**  A block region alone is not enough, because a compiler hoists.
//! In `match r { Ok(v) => v, Err(e) => e + 100 }` at `-C opt-level=1` the `e +
//! 100` is computed *before* the branch and selected by a `cmov`, so the read of
//! the payload that means `Err` lives in a block that can still reach both
//! variants.  A backward walk over def-use closes that: an op **every** one of
//! whose uses is in a variant-`k` context is a variant-`k` op, where a
//! `MULTIEQUAL` input slot's context is its own **in-edge**'s variant set rather
//! than the phi block's.  Def-use in SSA is acyclic once phi inputs are read as
//! edges, so the walk is exact rather than a fixpoint guess.
//!
//! A single use whose own context is still ambiguous BLOCKS the conclusion,
//! rather than merely failing to narrow it.  That distinction is the difference
//! between a proof and a majority vote: a value read once and consumed both by
//! the discriminant test itself and inside one arm would otherwise be attributed
//! to that arm, and the label would then be a claim about the bytes that the
//! other arm's execution contradicts.  The cost is measured and stated below.
//!
//! ## What it will not do, measured
//!
//! * A **branchless producer** names nothing.  rustc at `-C opt-level=1`
//!   computes a `Result`'s discriminant as `(x < 0xb)` rather than storing a
//!   literal, so the committed `put_res` / `r16` witnesses stay on the offset
//!   spelling: there is no constant for [`facet_for_discr`] to look up.
//! * A **switch / jump-table dispatch** on the tag is not a seed.  Only
//!   equality-shaped conditions are read.
//! * A **producer store names no read at all**, not even one below it.  Only a
//!   guard names a read, and only until the first event.  A producer store also
//!   does not reach out of its own block, so a construction split across blocks
//!   recovers nothing.
//! * **Any call kills a memory object's guard**, with no attempt to prove the
//!   callee cannot reach it.  That is the largest cost measured: on a std-linked
//!   `-g` witness it takes the recovery from 5 functions / 17 labels to 4 / 11,
//!   all of the loss being reads below a call in drop glue and in a recursive
//!   tree walk.  Refining it needs an escape analysis for the object's address,
//!   which is a real analysis and not a predicate.
//! * **Any store this pass cannot attribute to the object kills too**, because it
//!   may alias.  `*p = Ok(1); *q = Err(2); (*p).payload = x;` is a committed
//!   witness: `q` may be `p`.
//! * A read consumed by the discriminant test AND by one arm is refused, per the
//!   blocking rule above.  On a NICHE encoding the two are literally the same
//!   bytes -- `enum Tree { Leaf(i64), Node(Box<Tree>, Box<Tree>) }` stores the
//!   tag in `Node`'s first box -- so its drop glue names the arm's read and
//!   leaves the guard's read on the offset spelling.  Closing that needs a
//!   per-ACCESS width rule (an 8-byte read at offset 0 of that `Tree` can only
//!   be `Node`, because `Leaf` claims no byte there), which is a layout question
//!   and not a guard question, and is not attempted here.
//! * The facet's own interned TYPE name is never substituted, so a cast still
//!   prints `Result<u64, u64>::field_0x8`.  The type is global to the program;
//!   the proof is per-access.
//! * The DEPTH at which a member renders is not this pass's to choose.  The
//!   implied-field seam emits one member token (`v2.payload.Err`) and the
//!   `PTRSUB` seam emits the whole field path (`(dst->payload).Err.__0`); that
//!   difference is `dwarfvariants`' own rendering and is present verbatim with
//!   this option OFF (`v2.payload.field_0x8` against
//!   `(dst->payload).field_0x8.__0`).  Only the member NAME is substituted.
//! * The FIELD inside a named facet keeps `dwarfvariants`'s own per-field
//!   suppression (`Multi`'s `P.field_0x8`), which is a separate rule.
//!
//! ## What it will not do
//!
//! Only a layout with a **suppressed** payload facet is considered at all
//! ([`suppressed`]): an enum `dwarfvariants` already names — every `Option<T>`,
//! `enum List { Cons(..), Nil }`, `enum Three { A(u32)@4, B(u64)@8, C }` — is
//! skipped outright, so this pass can never overwrite a label that is already
//! forced by the layout, and can never change output where `dwarfvariants` had an
//! answer.  Every access is attributed to an **object root** (the Varnode the
//! bytes were extracted from, or the pointer they were loaded through) and is
//! pinned only against seeds on that same root, so a second `Result` in the same
//! function is not steered by the first one's guard.  A region that is ambiguous,
//! empty (a contradiction) or unreachable is left alone, and the access then
//! renders exactly as `variantguard off` renders it.
//!
//! ## Two caches and one proof channel
//!
//! The pin is written to the op-keyed union-resolution cache the printer's
//! field-path descent reads (`Funcdata::get_union_resolution`) *and* to the
//! slot-free address-keyed cache `resolve_in_flow` consults before it would build
//! a scorer, so an op created later still gets the guard's answer.  Both are
//! locked, which is what stops `ScoreUnionFields` from overwriting the field.
//!
//! The *proof* is recorded separately ([`Funcdata::kuna_record_variant_proof`]),
//! because the printer must distinguish "the discriminant says `Err`" from "the
//! scorer picked field 1", and `ResolvedUnion`'s lock does not survive either of
//! the two paths that rebuild a resolution onto a **new** op —
//! `Funcdata::resolve_in_flow`'s address-based materialization and
//! `ActionSetCasts::resolveUnion`'s spliced `PTRSUB` — both of which construct a
//! fresh unlocked resolution, and both of which create ops the printer descends
//! with.  The proof channel is therefore keyed by op time *and* by instruction
//! address, and an address at which two different members were proved is dropped.
//!
//! ## Where it runs
//!
//! P5 by taxonomy — it decides which member of a union type an access refers to,
//! which is the same question `TypeUnion::resolveInFlow` answers and lives beside
//! it in this folder.  It is *scheduled* immediately before `ActionSetCasts`
//! because the resolution cache is keyed by `PcodeOp::getTime()`: earlier, the
//! ops it pins are still being created and destroyed by the main loop; later, the
//! cast plane has already filled the cache from the scorer and a `set_union_field`
//! on an occupied entry cannot install a lock (`ResolvedUnion::update` copies the
//! field and the data-type, never the lock).

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, int8, uint8, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::kuna_dwarfvariants::VariantLayout;
use crate::options::on_or_off;
use crate::unionresolve::ResolvedUnion;

/// Hops the location walkers follow before giving up (a pointer chain longer
/// than this is not an enum field path).
const MAX_WALK: u32 = 24;

/// Widest variant count the bitset region analysis describes.  rustc enums in
/// the wild are far below this; a wider one is refused rather than truncated.
const MAX_VARIANTS: usize = 64;

/// Cap on `(layout, object root)` pairs analysed in one function, so a
/// pathological function cannot make the pass superlinear.
const MAX_OBJECTS: usize = 32;

/// Rounds of the backward use-context walk.  Def-use is acyclic once `MULTIEQUAL`
/// inputs are read as edges, so this converges; the cap only bounds a
/// pathologically deep chain.
const MAX_USE_ROUNDS: u32 = 12;

// =============================================================================
// The option
// =============================================================================

/// (kuna) The `variantguard on|off` ArchOption.
pub struct OptionVariantGuard;

impl OptionVariantGuard {
    /// The option name.
    pub const NAME: &'static str = "variantguard";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("DWARF discriminant-guarded variant selection turned {prop}")))
    }
}

// =============================================================================
// The proof channel (read by the P9 printer)
// =============================================================================

impl Funcdata {
    /// (kuna `variantguard`) Record that member `field` of the overlay union
    /// `union_type` is the one the DWARF discriminant selects at this edge.
    ///
    /// Both keys are written: the op-time edge (exact, for the ops that exist
    /// now) and the instruction address (for the cast/`PTRSUB` ops spliced into
    /// this edge afterwards).  An address that ends up carrying two different
    /// members is a contradiction at instruction granularity and is dropped, so
    /// the coarse key never resolves a conflict by luck.
    pub fn kuna_record_variant_proof(
        &mut self,
        union_type: &Datatype,
        op: OpId,
        slot: int4,
        field: int4,
    ) {
        let id = union_type.get_id();
        let Some(o) = self.obank().get(op) else { return };
        let (time, addr) = (o.get_time(), o.get_addr().clone());
        self.kuna_variant_proof_op.insert((id, time, slot), field);
        self.record_addr_proof(id, addr, slot, field);
    }

    /// (kuna `variantguard`) Record the slot `-1` alias of an already-recorded
    /// POINTER pin at the same address.
    ///
    /// `ActionSetCasts::resolveUnion` splices a zero-`PTRSUB` into a
    /// pointer-to-union edge and reads its resolution back at slot -1. That is
    /// the only slot rewrite the cast plane performs, so it is recorded
    /// explicitly rather than by making the address key slot-blind.
    pub fn kuna_record_variant_proof_alias(
        &mut self,
        union_type: &Datatype,
        op: OpId,
        field: int4,
    ) {
        let id = union_type.get_id();
        let Some(o) = self.obank().get(op) else { return };
        let addr = o.get_addr().clone();
        self.record_addr_proof(id, addr, -1, field);
    }

    /// (kuna `variantguard`) Poison the address-keyed proof for this edge.
    ///
    /// Called when the address-based resolution cache already holds a LOCKED
    /// entry naming a different member, so the coarse key can no longer be
    /// trusted to answer for an op created later at this address.
    pub fn kuna_record_variant_proof_addr_conflict(
        &mut self,
        union_type: &Datatype,
        op: OpId,
        slot: int4,
    ) {
        let id = union_type.get_id();
        let Some(o) = self.obank().get(op) else { return };
        let addr = o.get_addr().clone();
        self.kuna_variant_proof_addr.insert((id, addr, slot), None);
    }

    /// Insert one address-keyed proof, TOMBSTONING the key permanently on a
    /// contradiction. Removing the entry instead would let a later record
    /// resurrect a key two earlier records had already disagreed about.
    fn record_addr_proof(
        &mut self,
        id: uint8,
        addr: kuna_base::address::Address,
        slot: int4,
        field: int4,
    ) {
        match self.kuna_variant_proof_addr.get(&(id, addr.clone(), slot)) {
            Some(Some(prev)) if *prev != field => {
                self.kuna_variant_proof_addr.insert((id, addr, slot), None);
            }
            Some(_) => {}
            None => {
                self.kuna_variant_proof_addr.insert((id, addr, slot), Some(field));
            }
        }
    }

    /// (kuna `variantguard`) Whether member `field` of `union_type` at this edge
    /// is the DWARF discriminant's answer rather than the union scorer's.
    ///
    /// The printer spells a `DW_TAG_variant` name only where this is true.
    pub fn kuna_variant_proved(
        &self,
        union_type: &Datatype,
        op: OpId,
        slot: int4,
        field: int4,
    ) -> bool {
        if field < 0 {
            return false;
        }
        let id = union_type.get_id();
        let Some(o) = self.obank().get(op) else { return false };
        if self.kuna_variant_proof_op.get(&(id, o.get_time(), slot)) == Some(&field) {
            return true;
        }
        // The coarse key answers only for an op created AFTER the analysis ran --
        // a cast or a zero-`PTRSUB` the cast plane spliced into a pinned edge,
        // which carries that edge's address.  An op the analysis saw and left
        // alone is not proved by a neighbour that happens to share its address.
        if o.get_time() <= self.kuna_variant_proof_horizon {
            return false;
        }
        self.kuna_variant_proof_addr.get(&(id, o.get_addr().clone(), slot)) == Some(&Some(field))
    }

    /// (kuna `variantguard`) Close the proof set: every op alive now is one the
    /// analysis saw, so only later ops may consult the address-keyed map.
    pub fn kuna_seal_variant_proofs(&mut self) {
        let horizon = self
            .obank()
            .iter_alive()
            .filter_map(|op| self.obank().get(op).map(|o| o.get_time()))
            .max()
            .unwrap_or(0);
        self.kuna_variant_proof_horizon = self.kuna_variant_proof_horizon.max(horizon);
    }
}

// =============================================================================
// Locating the bytes an access touches, and the object they belong to
// =============================================================================

/// Whether a location names bytes of a value in flight or bytes in memory behind
/// a pointer.  Two objects with the same root Varnode but different kinds are
/// different objects.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum ObjKind {
    /// The bytes of `root` itself (a register pair, a returned join, a constant).
    Value,
    /// The bytes at `root + off` in memory, `root` being the base pointer.
    Mem,
}

/// The byte range one Varnode (or one store) names, relative to an object root.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Loc {
    /// Value bytes or memory bytes.
    pub kind: ObjKind,
    /// The Varnode the bytes hang off: the aggregate itself, or the base pointer.
    pub root: VarnodeId,
    /// Byte offset of the range within the object.
    pub off: int8,
    /// Width of the range in bytes.
    pub size: int4,
}

/// The constant a Varnode carries, if it is one.
fn const_of(data: &Funcdata, vn: VarnodeId) -> Option<uintb> {
    let v = data.vbank().get(vn)?;
    if v.is_constant() {
        Some(v.get_offset())
    } else {
        None
    }
}

/// A constant read as a signed byte displacement of its own width.
fn signed_const(data: &Funcdata, vn: VarnodeId) -> Option<int8> {
    let v = data.vbank().get(vn)?;
    if !v.is_constant() {
        return None;
    }
    let bits = (v.get_size() as u32).min(8) * 8;
    let raw = v.get_offset();
    if bits >= 64 {
        return Some(raw as int8);
    }
    let sign = 1u64 << (bits - 1);
    let masked = raw & ((1u64 << bits) - 1);
    Some(if masked & sign != 0 { (masked | !((1u64 << bits) - 1)) as int8 } else { masked as int8 })
}

/// Follow a pointer expression back to `(base pointer Varnode, byte offset)`.
///
/// `PTRSUB`/`PTRADD`/`INT_ADD` with a constant displacement, plus the transparent
/// `COPY`/`CAST`, are exactly the shapes a recovered field path is built from.
pub(crate) fn locate_ptr(data: &Funcdata, vn: VarnodeId) -> (VarnodeId, int8) {
    let mut cur = vn;
    let mut off: int8 = 0;
    for _ in 0..MAX_WALK {
        let Some(def) = data.vbank().get(cur).and_then(|v| v.get_def()) else { break };
        let Some(o) = data.obank().get(def) else { break };
        let (next, step) = match o.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => (o.get_in(0), Some(0)),
            OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD => {
                match (o.get_in(0), o.get_in(1)) {
                    (Some(a), Some(b)) => (Some(a), signed_const(data, b)),
                    _ => (None, None),
                }
            }
            OpCode::CPUI_PTRADD => match (o.get_in(0), o.get_in(1), o.get_in(2)) {
                (Some(a), Some(b), Some(c)) => {
                    let idx = signed_const(data, b);
                    let scale = signed_const(data, c);
                    (Some(a), idx.zip(scale).and_then(|(i, s)| i.checked_mul(s)))
                }
                _ => (None, None),
            },
            _ => (None, None),
        };
        match (next, step) {
            (Some(n), Some(s)) => {
                off += s;
                cur = n;
            }
            _ => break,
        }
    }
    (cur, off)
}

/// Locate the bytes a value-carrying Varnode names.
///
/// `SUBPIECE` is where a register pair becomes "offset 8 of the returned enum";
/// `LOAD` is where the object becomes a memory object behind a pointer.
pub(crate) fn locate_value(data: &Funcdata, vn: VarnodeId, big_endian: bool) -> Loc {
    locate_value_depth(data, vn, big_endian, MAX_WALK)
}

fn locate_value_depth(data: &Funcdata, vn: VarnodeId, big_endian: bool, depth: u32) -> Loc {
    let size = data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
    let here = Loc { kind: ObjKind::Value, root: vn, off: 0, size };
    if depth == 0 {
        return here;
    }
    let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else { return here };
    let Some(o) = data.obank().get(def) else { return here };
    match o.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => match o.get_in(0) {
            Some(a) => {
                let mut l = locate_value_depth(data, a, big_endian, depth - 1);
                l.size = size;
                l
            }
            None => here,
        },
        // A zero-extension widens the container, not the meaningful bytes.
        OpCode::CPUI_INT_ZEXT => match o.get_in(0) {
            Some(a) => locate_value_depth(data, a, big_endian, depth - 1),
            None => here,
        },
        OpCode::CPUI_SUBPIECE => {
            let (Some(a), Some(b)) = (o.get_in(0), o.get_in(1)) else { return here };
            let Some(c) = const_of(data, b) else { return here };
            let insize = data.vbank().get(a).map(|v| v.get_size()).unwrap_or(0);
            if c > int4::MAX as uintb {
                return here;
            }
            // SUBPIECE drops `c` low-order BYTES; on a little-endian layout those
            // are the low addresses, on a big-endian one the high ones.
            let shift = if big_endian { (insize - c as int4 - size) as int8 } else { c as int8 };
            let base = locate_value_depth(data, a, big_endian, depth - 1);
            Loc { kind: base.kind, root: base.root, off: base.off + shift, size }
        }
        OpCode::CPUI_LOAD => match o.get_in(1) {
            Some(p) => {
                let (root, off) = locate_ptr(data, p);
                Loc { kind: ObjKind::Mem, root, off, size }
            }
            None => here,
        },
        _ => here,
    }
}

// =============================================================================
// The recorded layouts this pass may act on
// =============================================================================

/// Whether `layout` has a payload facet whose union-member label is NOT the
/// variant's source name, i.e. one `dwarfvariants` suppressed.
///
/// This is the whole scope of the pass.  An enum whose every payload variant
/// already keeps its `DW_TAG_variant` name — `Option<T>`, `enum List { Cons, Nil }`,
/// an enum whose payloads occupy disjoint byte ranges — is skipped before any
/// analysis runs, so no label the layout already forces can be disturbed.
pub fn suppressed(layout: &VariantLayout) -> bool {
    layout.variants.iter().any(|v| !v.label.is_empty() && v.label != v.name)
}

/// One enum type this function actually mentions, with its resolved types.
struct Subject {
    layout: Rc<VariantLayout>,
    /// The recovered enum type (the struct of tag + payload, or the overlay union
    /// itself for a niche).
    enum_ty: Rc<Datatype>,
    /// The overlay union whose member the printer resolves.
    union_ty: Rc<Datatype>,
}

/// A predicate on the discriminant: `((tag & mask) == cmp) == eq`.
#[derive(Clone, Copy, Debug)]
struct TagPred {
    mask: uintb,
    cmp: uintb,
    eq: bool,
}

impl TagPred {
    /// Whether the concrete discriminant `v` satisfies the predicate.
    fn holds(&self, v: uintb) -> bool {
        ((v & self.mask) == self.cmp) == self.eq
    }

    /// How many of the `2^(8*tag_size)` discriminant values satisfy it.
    fn population(&self, tag_size: int4) -> u128 {
        let bits = (tag_size as u32).min(8) * 8;
        let total: u128 = 1u128 << bits;
        let full: uintb = if bits >= 64 { uintb::MAX } else { (1u64 << bits) - 1 };
        let mask = self.mask & full;
        let cmp = self.cmp & full;
        // `(v & mask) == cmp` has no solution unless cmp only sets bits mask keeps.
        let matching: u128 =
            if cmp & !mask != 0 { 0 } else { 1u128 << (bits - mask.count_ones()) };
        if self.eq {
            matching
        } else {
            total - matching
        }
    }
}

/// The variant set a predicate admits, as a bitmask over `layout.variants`.
///
/// The DEFAULT variant (no `DW_AT_discr_value`) is admitted exactly when some
/// value the other variants did NOT claim satisfies the predicate — counted, not
/// assumed, so `if (tag == 0)` on an `Option<&T>` whose `None` is 0 and whose
/// `Some` is the default correctly excludes `Some` on the true edge.
fn facets_admitted(layout: &VariantLayout, pred: &TagPred) -> u64 {
    let mut set = 0u64;
    let mut explicit_hits: u128 = 0;
    let mut default_idx: Option<usize> = None;
    for (i, v) in layout.variants.iter().enumerate() {
        match v.discr {
            Some(d) => {
                if pred.holds(d) {
                    set |= 1u64 << i;
                    explicit_hits += 1;
                }
            }
            None => default_idx = Some(i),
        }
    }
    if let Some(i) = default_idx {
        if pred.population(layout.tag_size) > explicit_hits {
            set |= 1u64 << i;
        }
    }
    set
}

// =============================================================================
// Seeds
// =============================================================================

/// A branch that fixes the discriminant of one object.
struct GuardSeed {
    block: BlockId,
    /// Variants possible on the edge taken when the predicate holds.
    on_true: u64,
    /// Variants possible on the other edge.
    on_false: u64,
    /// The out-edge index the predicate-true side takes.
    true_index: int4,
}

/// Read a Varnode as `(tag & mask)` of the tag bytes of the object `root`.
///
/// Returns the mask; the caller already knows the object.  Only the transparent
/// widening/narrowing shapes rustc emits around a discriminant read are followed.
fn tag_mask_of(
    data: &Funcdata,
    subject: &Subject,
    root: VarnodeId,
    kind: ObjKind,
    vn: VarnodeId,
    big_endian: bool,
    depth: u32,
) -> Option<uintb> {
    if depth == 0 {
        return None;
    }
    if let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) {
        if let Some(o) = data.obank().get(def) {
            match o.code() {
                OpCode::CPUI_INT_AND => {
                    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
                    if let Some(k) = const_of(data, b) {
                        let m =
                            tag_mask_of(data, subject, root, kind, a, big_endian, depth - 1)?;
                        return Some(m & k);
                    }
                    if let Some(k) = const_of(data, a) {
                        let m =
                            tag_mask_of(data, subject, root, kind, b, big_endian, depth - 1)?;
                        return Some(m & k);
                    }
                    return None;
                }
                OpCode::CPUI_INT_ZEXT => {
                    let a = o.get_in(0)?;
                    return tag_mask_of(data, subject, root, kind, a, big_endian, depth - 1);
                }
                _ => {}
            }
        }
    }
    // A plain read of the tag bytes: it must start exactly at the discriminant
    // and stay inside it, so the value read is the tag truncated to its width.
    let loc = locate_value(data, vn, big_endian);
    if loc.root != root || loc.kind != kind {
        return None;
    }
    let l = subject.layout.as_ref();
    if loc.off != l.tag_offset as int8 || loc.size <= 0 || loc.size > l.tag_size {
        return None;
    }
    let bits = (loc.size as u32).min(8) * 8;
    Some(if bits >= 64 { uintb::MAX } else { (1u64 << bits) - 1 })
}

/// Read a CBRANCH condition as a predicate on one object's discriminant.
fn branch_predicate(
    data: &Funcdata,
    subject: &Subject,
    root: VarnodeId,
    kind: ObjKind,
    cond: VarnodeId,
    big_endian: bool,
    depth: u32,
) -> Option<TagPred> {
    if depth == 0 {
        return None;
    }
    if let Some(def) = data.vbank().get(cond).and_then(|v| v.get_def()) {
        if let Some(o) = data.obank().get(def) {
            match o.code() {
                OpCode::CPUI_BOOL_NEGATE => {
                    let a = o.get_in(0)?;
                    let mut p =
                        branch_predicate(data, subject, root, kind, a, big_endian, depth - 1)?;
                    p.eq = !p.eq;
                    return Some(p);
                }
                OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
                    let eq = o.code() == OpCode::CPUI_INT_EQUAL;
                    for (val, konst) in [(a, b), (b, a)] {
                        let Some(k) = const_of(data, konst) else { continue };
                        if let Some(mask) =
                            tag_mask_of(data, subject, root, kind, val, big_endian, depth - 1)
                        {
                            return Some(TagPred { mask, cmp: k, eq });
                        }
                    }
                    return None;
                }
                _ => {}
            }
        }
    }
    // `if (tag)` -- the discriminant itself used as the condition.
    let mask = tag_mask_of(data, subject, root, kind, cond, big_endian, depth)?;
    Some(TagPred { mask, cmp: 0, eq: false })
}

// =============================================================================
// The region analysis
// =============================================================================

/// Per-block variant sets for one `(enum type, object root)` pair.
struct Regions {
    /// `BlockId` -> index into the per-block vectors below.
    index: BTreeMap<BlockId, usize>,
    /// The variant set proved at each block's ENTRY, or 0 when nothing is.
    region: Vec<u64>,
    /// The variant set each in-edge of each block carries: `edge[b][i]`.
    edge: Vec<Vec<u64>>,
    /// Position of the first event ([`object_events`]) in each block, if any.
    /// A guard fact does not survive it.
    first_event: Vec<Option<usize>>,
    /// Every op's `(block index, position in `bb_ops`)`, so the kill can be
    /// applied POSITIONALLY rather than to a whole block.
    op_pos: BTreeMap<OpId, (usize, usize)>,
    /// The set every guard-less point falls back to.
    all: u64,
}

impl Regions {
    /// The variant set a GUARD proves at `op` — the fact at its block's entry,
    /// unless an event has already invalidated it earlier in the same block.
    ///
    /// This is the whole kill discipline on the guard side: a guard proves what
    /// the object was AT THE GUARD, and a store over the tag bytes, a call, or
    /// any store that might alias the object ends that.
    fn guard_at(&self, op: OpId) -> u64 {
        let Some(&(bi, pos)) = self.op_pos.get(&op) else { return self.all };
        if let Some(ev) = self.first_event[bi] {
            if pos > ev {
                return self.all;
            }
        }
        self.region[bi]
    }
}

/// Compute the per-block and per-in-edge variant sets from the CONSUMER guards.
///
/// A guard is evidence about the value an execution is carrying, so it colours
/// every op the guarded edge reaches.  Producer stores are deliberately NOT part
/// of this: a store is evidence about what the object BECOMES, and folding it in
/// backwards would colour an op that ran BEFORE it -- see [`producer_writes`].
#[allow(clippy::needless_range_loop)]
fn compute_regions(
    data: &Funcdata,
    all: u64,
    guards: &[GuardSeed],
    events: &[Vec<TagEvent>],
) -> Regions {
    let n = data.bblocks_get_size().max(0) as usize;
    let mut blocks = Vec::with_capacity(n);
    let mut index = BTreeMap::new();
    for i in 0..n {
        let b = data.bblocks_get_block(i as int4);
        index.insert(b, i);
        blocks.push(b);
    }

    // Out-edge constraints from the guards, defaulting to "anything".
    let mut out_constraint: Vec<Vec<u64>> = blocks
        .iter()
        .map(|&b| vec![all; data.bblocks_ref().block(b).size_out().max(0) as usize])
        .collect();
    for g in guards {
        let Some(&i) = index.get(&g.block) else { continue };
        let outs = out_constraint[i].len() as int4;
        if outs != 2 || g.true_index < 0 || g.true_index >= outs {
            continue;
        }
        let t = g.true_index as usize;
        let f = 1 - t;
        // Several guards on one block intersect: each is independently true.
        out_constraint[i][t] &= g.on_true;
        out_constraint[i][f] &= g.on_false;
    }

    // Forward may-analysis: what the in-edges allow.  Every block with no
    // predecessor is a root and is unconstrained -- the function entry, and any
    // block a split left without one.  Keying that on index 0 alone would leave a
    // re-indexed entry at the empty set and silently disable the whole analysis.
    let mut fwd = vec![0u64; blocks.len()];
    for (i, &b) in blocks.iter().enumerate() {
        if data.bblocks_ref().block(b).size_in() == 0 {
            fwd[i] = all;
        }
    }
    let mut edge: Vec<Vec<u64>> =
        blocks.iter().map(|&b| vec![0u64; data.bblocks_ref().block(b).size_in().max(0) as usize]).collect();
    for _ in 0..=blocks.len() {
        let mut changed = false;
        for i in 0..blocks.len() {
            let b = blocks[i];
            let nin = data.bblocks_ref().block(b).size_in();
            if nin == 0 {
                continue;
            }
            let mut acc = 0u64;
            for k in 0..nin {
                let pred = data.bblocks_ref().block(b).get_in(k);
                let rev = data.bblocks_ref().block(b).get_in_rev_index(k);
                let allowed = match index.get(&pred) {
                    Some(&pi) => {
                        let c = out_constraint[pi]
                            .get(rev.max(0) as usize)
                            .copied()
                            .unwrap_or(all);
                        // The fact LEAVING a predecessor is TOP when anything in
                        // it may have changed the discriminant.  The guard's own
                        // CBRANCH is the block's last op, so a re-test after a
                        // kill still constrains the edge -- which is exactly how
                        // `if (tag == 0)` below a clobber stays usable.
                        let out_p = if events[pi].is_empty() { fwd[pi] } else { all };
                        out_p & c
                    }
                    None => all,
                };
                if edge[i][k as usize] != allowed {
                    edge[i][k as usize] = allowed;
                    changed = true;
                }
                acc |= allowed;
            }
            if fwd[i] != acc {
                fwd[i] = acc;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let first_event: Vec<Option<usize>> =
        events.iter().map(|e| e.iter().map(|x| x.pos).min()).collect();
    let mut op_pos: BTreeMap<OpId, (usize, usize)> = BTreeMap::new();
    for (bi, &b) in blocks.iter().enumerate() {
        for (pos, op) in data.bb_ops(b).into_iter().enumerate() {
            op_pos.insert(op, (bi, pos));
        }
    }
    Regions { index, region: fwd, edge, first_event, op_pos, all }
}

// =============================================================================
// The producer side: what a constant tag store proves, and about which ops
// =============================================================================

/// One event in a block that changes, or may change, what the object holds.
///
/// Positioned in `bb_ops` order, i.e. PROGRAM order within the block.
struct TagEvent {
    /// Position of the op in `bb_ops` order.
    pos: usize,
    /// The variant a readable constant discriminant store selects.  `None` is an
    /// opaque event: a tag store this pass cannot read as a constant, a call, or
    /// a store it cannot attribute to this object at all.
    variant: Option<usize>,
}

/// Every event in block `bl` that changes, or may change, the discriminant of
/// the object `(kind, root)`, in program order.
///
/// This is the single kill discipline both halves of the pass are built on, and
/// it exists because every defect this feature shipped was one thing: **a fact
/// about an object's discriminant applied at a program point where it no longer
/// held.**  A guard proves what the object was AT THE GUARD; a store proves what
/// it becomes.  Neither survives an event.
///
/// Three kinds, and the third is the one aliasing forces:
///
/// * a **STORE over the tag bytes** of this object — the discriminant changed.
///   A store to the object's payload bytes ALONE is not an event, because it
///   cannot change which variant is live.  (Under a niche encoding the tag bytes
///   *are* payload bytes, and the range test then reports the overlap, so a
///   niche payload store correctly does kill.)
/// * a **call** — a callee handed the pointer may store anything through it.
/// * **any store this pass cannot attribute to this object**, because it may
///   alias it.  `*p = Ok(1); *q = Err(2); (*p).payload = x;` is a real
///   counterexample: `q` may be `p`, so the `Ok` is not proved.  Refusing every
///   unattributable store is coarse, and deliberately so — a wrong variant name
///   is worse than no name.
///
/// A **value** object (`ObjKind::Value`) has NO events: it is an SSA value, and
/// nothing can clobber it.  That is not a concession, it is what SSA means, and
/// it is why a `match` on a returned enum keeps its names while the same shape
/// behind a pointer does not.
fn object_events(
    data: &Funcdata,
    subject: &Subject,
    root: VarnodeId,
    kind: ObjKind,
    big_endian: bool,
    bl: BlockId,
) -> Vec<TagEvent> {
    let mut events: Vec<TagEvent> = Vec::new();
    if kind != ObjKind::Mem {
        return events;
    }
    let layout = subject.layout.as_ref();
    let t0 = layout.tag_offset as int8;
    let t1 = t0 + layout.tag_size as int8;
    for (pos, &op) in data.bb_ops(bl).iter().enumerate() {
        let Some(o) = data.obank().get(op) else { continue };
        if o.is_call() {
            events.push(TagEvent { pos, variant: None });
            continue;
        }
        if o.code() != OpCode::CPUI_STORE {
            continue;
        }
        let (Some(p), Some(val)) = (o.get_in(1), o.get_in(2)) else {
            events.push(TagEvent { pos, variant: None });
            continue;
        };
        let (r, off) = locate_ptr(data, p);
        if r != root {
            // Not provably this object: it may alias it.
            events.push(TagEvent { pos, variant: None });
            continue;
        }
        let width = data.vbank().get(val).map(|v| v.get_size()).unwrap_or(0);
        if off + (width as int8) <= t0 || off >= t1 {
            continue; // this object's payload only, so the discriminant stands
        }
        let variant = const_of(data, val)
            .and_then(|k| tag_slice(layout, off, width, k, big_endian))
            .and_then(|v| variant_index_for_discr(layout, v));
        events.push(TagEvent { pos, variant });
    }
    events
}

/// The variant each WRITE into the object is building, from the constant tag
/// store nearest it in its own block.
///
/// ## Why this is not the block-level backward analysis it replaces
///
/// A producer store is evidence about what the object BECOMES. It says nothing
/// about a value that was read BEFORE it, and an earlier revision of this pass
/// coloured the whole block (and every block that reached it), which made
///
/// ```ignore
/// let v = match *dst { Ok(a) => a, Err(b) => b };   // reads the CALLER's value
/// *dst = Err(7);
/// v
/// ```
///
/// render the load as `(*dst).payload.Err.__0`. The control is decisive: the same
/// function clobbering with `Ok(9)` named the SAME instruction `Ok`. Two
/// identical reads of the same source expression got opposite variant names,
/// decided by a store that happens after them; at most one could be right, and
/// by construction neither was. That is a name asserted from a shape rather than
/// proved about the read, which is precisely what DIV-87's suppression rule
/// exists to prevent.
///
/// This is not a corner case. Read-then-overwrite of an enum in place is what
/// `mem::replace`, `Option::take` and any state machine that consumes its old
/// state and writes the new one compile to, and it is the shape drop glue and
/// `?`-desugaring produce -- so it ships as the default rendering for very
/// ordinary `-g` Rust. `variantguard_clobber_x86_64` is the committed fixture.
///
/// So the producer fact is confined to what it can actually support:
///
/// * it applies **only to writes** -- a store into the object, and the pointer
///   arithmetic that feeds one and nothing else ([`writes_only`]);
/// * it is **intra-block and positional**, read off the nearest constant tag
///   store above or below the write in `bb_ops` (program) order;
/// * it is **killed** by any intervening store over the tag bytes this pass
///   cannot read as a constant, and by any call, which may store through the
///   pointer itself;
/// * a write with a constant tag store on BOTH sides that disagree is refused.
///
/// Both directions are sound for a write and both are needed: `payload = v;
/// tag = 0;` (a constructor -- the bytes become `Ok`'s payload) and `tag = 1;
/// payload = v;` (the object is already `Err` when the payload is written).
fn producer_writes(
    data: &Funcdata,
    root: VarnodeId,
    kind: ObjKind,
    events: &[Vec<TagEvent>],
) -> BTreeMap<OpId, usize> {
    let mut out: BTreeMap<OpId, usize> = BTreeMap::new();
    if kind != ObjKind::Mem {
        return out;
    }
    let n = data.bblocks_get_size().max(0);
    for bi in 0..n {
        let writes = &events[bi as usize];
        if writes.is_empty() {
            continue;
        }
        let bl = data.bblocks_get_block(bi);
        for (pos, &op) in data.bb_ops(bl).iter().enumerate() {
            // The op is itself an event: it writes the discriminant (or is a
            // call, or an unattributable store), so there is nothing to name.
            if writes.iter().any(|w| w.pos == pos) {
                continue;
            }
            if !writes_only(data, root, op) {
                continue;
            }
            // The nearest event above and below this write.  An OPAQUE event on
            // either side -- a call, an unreadable tag store, a store that may
            // alias -- is a kill and refuses that direction.
            let before = writes.iter().filter(|w| w.pos < pos).next_back().and_then(|w| w.variant);
            let after = writes.iter().find(|w| w.pos > pos).and_then(|w| w.variant);
            let v = match (before, after) {
                (Some(a), Some(b)) if a != b => continue, // the two sides disagree
                (Some(a), _) => a,
                (None, Some(b)) => b,
                (None, None) => continue,
            };
            out.insert(op, v);
        }
    }
    out
}

/// Whether every memory access `op` feeds into the object is a STORE.
///
/// A `STORE` through a pointer into the object qualifies directly; so does the
/// pointer arithmetic that computes one, provided EVERY use of its result ends
/// at a store's pointer input.  A `LOAD`, a call argument, or any other use
/// disqualifies it -- that is the whole point, because a producer store must
/// never colour a read.
fn writes_only(data: &Funcdata, root: VarnodeId, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    if o.code() == OpCode::CPUI_STORE {
        let Some(p) = o.get_in(1) else { return false };
        return locate_ptr(data, p).0 == root;
    }
    let Some(out) = o.get_out() else { return false };
    // The output must be a pointer that lands in the object.
    let is_ptr = data
        .vbank()
        .get(out)
        .map(|v| v.get_type().get_metatype() == type_metatype::TYPE_PTR)
        .unwrap_or(false);
    if !is_ptr || locate_ptr(data, out).0 != root {
        return false;
    }
    let mut frontier: Vec<VarnodeId> = vec![out];
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut any = false;
    for _ in 0..MAX_WALK {
        let Some(vn) = frontier.pop() else { break };
        if !seen.insert(vn) {
            continue;
        }
        let Some(v) = data.vbank().get(vn) else { return false };
        let uses: Vec<OpId> = v.descend_iter().collect();
        if uses.is_empty() {
            return false;
        }
        for u in uses {
            let Some(uo) = data.obank().get(u) else { return false };
            match uo.code() {
                OpCode::CPUI_STORE if uo.get_in(1) == Some(vn) => any = true,
                OpCode::CPUI_PTRSUB
                | OpCode::CPUI_PTRADD
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_COPY
                | OpCode::CPUI_CAST => match uo.get_out() {
                    Some(next) => frontier.push(next),
                    None => return false,
                },
                _ => return false,
            }
        }
    }
    any && frontier.is_empty()
}

// =============================================================================
// The backward use-context walk
// =============================================================================

/// The variant set each op's *result* is consumed under.
///
/// Starts at the op's block region and narrows: an op whose output is read only
/// in variant-`k` contexts is itself a variant-`k` op, which is what reaches the
/// hoisted `e + 100` of `match r { Ok(v) => v, Err(e) => e + 100 }` — computed in
/// the block above the branch, but used only on the `Err` edge.
fn use_contexts(data: &Funcdata, regions: &Regions, all: u64) -> BTreeMap<OpId, u64> {
    let mut ctx: BTreeMap<OpId, u64> = BTreeMap::new();
    let ops: Vec<OpId> = data.obank().iter_alive().collect();
    for &op in &ops {
        // `guard_at` applies the positional kill, so a value computed after the
        // object was overwritten starts at TOP rather than at its block's guard.
        ctx.insert(op, regions.guard_at(op));
    }
    for _ in 0..MAX_USE_ROUNDS {
        let mut changed = false;
        for &op in &ops {
            let Some(o) = data.obank().get(op) else { continue };
            let Some(out) = o.get_out() else { continue };
            let Some(v) = data.vbank().get(out) else { continue };
            let mut acc: Option<u64> = None;
            let mut blocked = false;
            for u in v.descend_iter() {
                let Some(uo) = data.obank().get(u) else { continue };
                let here = if uo.code() == OpCode::CPUI_MULTIEQUAL {
                    // A phi input's context is its own IN-EDGE, not the phi block:
                    // reading it as the block would lose exactly the arm
                    // information the guard established.  This is also what makes
                    // the def-use walk acyclic.
                    //
                    // The SAME Varnode can occupy several slots of one phi (two
                    // predecessors carrying the same value), and it is then live
                    // on every one of those edges -- so intersect them all rather
                    // than taking `get_slot`'s first match, which would attribute
                    // the value to whichever edge happened to be lowest-numbered.
                    let parent = uo.get_parent();
                    let mut acc_phi: Option<u64> = None;
                    for i in 0..uo.num_input() {
                        if uo.get_in(i) != Some(out) {
                            continue;
                        }
                        let e = match parent {
                            Some(pb) => regions
                                .index
                                .get(&pb)
                                .and_then(|&bi| regions.edge[bi].get(i as usize).copied())
                                .unwrap_or(all),
                            None => all,
                        };
                        acc_phi = Some(acc_phi.map_or(e, |a| a & e));
                    }
                    acc_phi.unwrap_or(all)
                } else {
                    ctx.get(&u).copied().unwrap_or(all)
                };
                // STRICT: a use whose own context is not yet a single variant
                // proves nothing, and an ambiguous use must BLOCK rather than
                // merely fail to constrain -- otherwise a value read once and
                // consumed both by the discriminant test and inside one arm
                // would be attributed to that arm, which is a claim about the
                // bytes that the other arm's execution contradicts.
                if here.count_ones() != 1 {
                    blocked = true;
                    break;
                }
                acc = Some(acc.map_or(here, |a| a & here));
            }
            if blocked {
                continue;
            }
            let Some(from_uses) = acc else { continue };
            let base = ctx.get(&op).copied().unwrap_or(all);
            let next = base & from_uses;
            if next != base {
                ctx.insert(op, next);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    ctx
}

// =============================================================================
// The pass
// =============================================================================

/// The recovered enum type a Varnode's type names, if any.
fn subject_of_type(subjects: &[Subject], ty: &Rc<Datatype>) -> Option<(usize, ObjKind)> {
    for (i, s) in subjects.iter().enumerate() {
        if Rc::ptr_eq(ty, &s.enum_ty) {
            return Some((i, ObjKind::Value));
        }
        if ty.get_metatype() == type_metatype::TYPE_PTR {
            if let Some(p) = ty.get_ptr_to() {
                if Rc::ptr_eq(&p, &s.enum_ty) {
                    return Some((i, ObjKind::Mem));
                }
            }
        }
    }
    None
}

/// Run the analysis over one function.  Returns the number of edges pinned.
fn run(data: &mut Funcdata) -> int4 {
    let Some(typegrp) = data.get_arch().types_rc() else { return 0 };
    let layouts: Vec<Rc<VariantLayout>> = typegrp
        .kuna_variant_layouts()
        .into_iter()
        .filter(|l| suppressed(l) && l.variants.len() <= MAX_VARIANTS)
        .collect();
    if layouts.is_empty() {
        return 0;
    }

    // Resolve each candidate layout's recovered types once.
    let mut subjects: Vec<Subject> = Vec::new();
    for l in layouts {
        let Ok(Some(enum_ty)) = typegrp.find_by_name(&l.type_name) else { continue };
        let Ok(Some(union_ty)) = typegrp.find_by_name(&l.union_type) else { continue };
        if union_ty.get_metatype() != type_metatype::TYPE_UNION {
            continue;
        }
        subjects.push(Subject { layout: l, enum_ty, union_ty });
    }
    if subjects.is_empty() {
        return 0;
    }

    // Which objects of those types this function actually holds.  One pass over
    // the op graph; a function that mentions none costs exactly this.
    let ops: Vec<OpId> = data.obank().iter_alive().collect();
    let mut objects: BTreeSet<(usize, ObjKind, VarnodeId)> = BTreeSet::new();
    for &op in &ops {
        let Some(o) = data.obank().get(op) else { continue };
        let n = o.num_input();
        for slot in -1..n {
            let vn = if slot < 0 { o.get_out() } else { o.get_in(slot) };
            let Some(vn) = vn else { continue };
            let Some(v) = data.vbank().get(vn) else { continue };
            let Some((si, kind)) = subject_of_type(&subjects, v.get_type()) else { continue };
            objects.insert((si, kind, vn));
            if objects.len() > MAX_OBJECTS * 4 {
                break;
            }
        }
    }
    if objects.is_empty() {
        return 0;
    }

    let big_endian = data.get_address().is_big_endian();

    // A Varnode typed as the enum is only a ROOT when nothing upstream of it is
    // the same object; `locate_*` canonicalizes, so collapse to the canonical
    // root and drop the rest.
    let mut roots: Vec<(usize, ObjKind, VarnodeId)> = Vec::new();
    for (si, kind, vn) in objects {
        let root = match kind {
            ObjKind::Value => locate_value(data, vn, big_endian).root,
            ObjKind::Mem => vn,
        };
        if !roots.contains(&(si, kind, root)) {
            roots.push((si, kind, root));
        }
    }
    roots.truncate(MAX_OBJECTS);

    let mut pinned = 0;
    for (si, kind, root) in roots {
        pinned += run_one(data, &subjects[si], kind, root, big_endian, &ops);
    }
    pinned
}

/// The analysis for one `(enum type, object root)` pair.
fn run_one(
    data: &mut Funcdata,
    subject: &Subject,
    kind: ObjKind,
    root: VarnodeId,
    big_endian: bool,
    ops: &[OpId],
) -> int4 {
    let layout = Rc::clone(&subject.layout);
    let nvar = layout.variants.len();
    let all: u64 = if nvar >= 64 { u64::MAX } else { (1u64 << nvar) - 1 };

    // --- seeds ------------------------------------------------------------
    let mut guards: Vec<GuardSeed> = Vec::new();
    for &op in ops.iter() {
        let Some(o) = data.obank().get(op) else { continue };
        let Some(block) = o.get_parent() else { continue };
        match o.code() {
            OpCode::CPUI_CBRANCH => {
                let Some(cond) = o.get_in(1) else { continue };
                let flip = o.is_boolean_flip();
                if data.bblocks_ref().block(block).size_out() != 2 {
                    continue;
                }
                let Some(pred) =
                    branch_predicate(data, subject, root, kind, cond, big_endian, MAX_WALK)
                else {
                    continue;
                };
                let on_true = facets_admitted(&layout, &pred);
                let mut inv = pred;
                inv.eq = !inv.eq;
                let on_false = facets_admitted(&layout, &inv);
                // `goto trueOut if (cond != 0)`, or `if (cond == 0)` when the
                // branch carries a boolean flip (funcdata_printraw::render_cbranch).
                let true_index = if flip { 0 } else { 1 };
                guards.push(GuardSeed { block, on_true, on_false, true_index });
            }
            _ => {}
        }
    }

    // --- the shared kill discipline, then both halves ----------------------
    // One scan of every block for the events that end a discriminant fact; the
    // guard side and the producer side are both built on it, so neither can
    // outlive a clobber the other would have seen.
    let nblocks = data.bblocks_get_size().max(0);
    let events: Vec<Vec<TagEvent>> = (0..nblocks)
        .map(|bi| {
            object_events(data, subject, root, kind, big_endian, data.bblocks_get_block(bi))
        })
        .collect();
    let regions = compute_regions(data, all, &guards, &events);
    let produced = producer_writes(data, root, kind, &events);
    if produced.is_empty() && regions.region.iter().all(|&r| r.count_ones() != 1) {
        return 0;
    }
    let ctx = use_contexts(data, &regions, all);

    // --- pin --------------------------------------------------------------
    let Some(typegrp) = data.get_arch().types_rc() else { return 0 };
    let mut count = 0;
    let mut work: Vec<(OpId, int4, int4, Option<Rc<Datatype>>, Address)> = Vec::new();
    for &op in ops {
        let Some(o) = data.obank().get(op) else { continue };
        let is_phi = o.code() == OpCode::CPUI_MULTIEQUAL;
        let block_ctx = ctx.get(&op).copied().unwrap_or(all);
        // A producer fact is a fact about the WRITE, not about the block, so it
        // is intersected with whatever the guards allow rather than replacing it.
        let producer = produced.get(&op).copied();
        let addr = o.get_addr().clone();
        let n = o.num_input();
        for slot in -1..n {
            let vn = if slot < 0 { o.get_out() } else { o.get_in(slot) };
            let Some(vn) = vn else { continue };
            // A MULTIEQUAL input slot's context is its own IN-EDGE, not the phi
            // block's -- the same rule `use_contexts` applies, which one context
            // shared across every slot would contradict.
            let here = if is_phi && slot >= 0 {
                edge_context(data, &regions, op, slot, all)
            } else {
                block_ctx
            };
            // INTERSECT the two facts; a disagreement REFUSES.  Precedence was
            // the round-3 defect: a singleton guard region beat a producer fact
            // the analysis had computed correctly, so a write that built `Err`
            // inside an `Ok`-guarded block was named `Ok` one line below its own
            // `tag = 1`.  Neither fact outranks the other -- they either agree or
            // there is no answer.
            let combined = match producer {
                Some(v) => here & (1u64 << v),
                None => here,
            };
            if combined.count_ones() != 1 {
                continue;
            }
            let variant = combined.trailing_zeros() as usize;
            let Some(field) = layout.union_field_for_variant(variant) else { continue };
            let (value_hit, ptr_ty) = attribution(data, subject, root, kind, vn, big_endian);
            if !value_hit && ptr_ty.is_none() {
                continue;
            }
            work.push((op, slot, field, ptr_ty, addr.clone()));
        }
    }

    for (op, slot, field, ptr_ty, addr) in work {
        // `set_union_field` returns false when a previously LOCKED association
        // already holds this edge -- an operator `map unionfacet`, or an earlier
        // root's pin. That is the resolution the printer reads at this edge, so
        // recording a proof for it when the write did not take would authorize a
        // name read off someone else's answer.
        //
        // `set_address_based_union_field` is a HINT for ops that do not exist
        // yet, and its bool cannot be read the same way: `ResolveEdge::new_addr`
        // discards the slot (unionresolve.cc:118-129 encodes a fixed 0x2000), so
        // this pass's own two pins on one op -- the output edge and an input edge
        // -- collide there by construction and the second always reports false.
        // It is therefore best-effort, and a genuine conflict (a LOCKED entry
        // naming a DIFFERENT member) tombstones the address proof instead.
        let ty = ptr_ty.clone().unwrap_or_else(|| Rc::clone(&subject.union_ty));
        let is_ptr = ptr_ty.is_some();
        let mut installed = false;
        let mut addr_conflict = false;
        if let Ok(mut r) = ResolvedUnion::new_field(Rc::clone(&ty), field, typegrp.as_ref()) {
            r.set_lock(true);
            installed = data.set_union_field(&ty, op, slot, r.clone());
            match data.get_address_based_union_field(&ty, &addr, slot) {
                Some(prev) if prev.is_locked() && prev.get_field_num() != field => {
                    addr_conflict = true;
                }
                _ => {
                    data.set_address_based_union_field(&ty, &addr, slot, r);
                }
            }
        }
        if addr_conflict {
            // Poison the coarse key rather than let a later op read either answer.
            data.kuna_record_variant_proof_addr_conflict(&subject.union_ty, op, slot);
        }
        if installed {
            data.kuna_record_variant_proof(&subject.union_ty, op, slot, field);
            // `ActionSetCasts::resolveUnion` moves a POINTER-to-union resolution
            // onto a zero-`PTRSUB` it splices into this same edge, at this same
            // address, and reads it back at slot -1. That one alias is the only
            // slot rewrite the cast plane performs, so it is recorded explicitly
            // instead of leaving the address key slot-blind.
            if is_ptr && slot != -1 {
                data.kuna_record_variant_proof_alias(&subject.union_ty, op, field);
            }
            count += 1;
        }
    }
    count
}

/// The variant set carried by in-edge `slot` of the block holding `op`.
fn edge_context(
    data: &Funcdata,
    regions: &Regions,
    op: OpId,
    slot: int4,
    all: u64,
) -> u64 {
    data.obank()
        .get(op)
        .and_then(|o| o.get_parent())
        .and_then(|b| regions.index.get(&b))
        .and_then(|&bi| regions.edge[bi].get(slot.max(0) as usize).copied())
        .unwrap_or(all)
}

/// How `vn` relates to the object `(kind, root)`: `(it names bytes OF the
/// object, it is a pointer INTO the object)`.
///
/// The two are exclusive and the distinction is load-bearing.  A Varnode LOADED
/// out of the object holds bytes of it (so the read resolves to a facet), but
/// the POINTER it happens to carry addresses a different object entirely — a
/// recursive `enum Tree { Leaf(i64), Node(Box<Tree>, Box<Tree>) }` passes exactly
/// such a value to itself.  Pinning that as a pointer into the parent makes the
/// cast plane splice a facet `PTRSUB` onto the callee's argument, which is a
/// structural change and not a rename.
fn attribution(
    data: &Funcdata,
    subject: &Subject,
    root: VarnodeId,
    kind: ObjKind,
    vn: VarnodeId,
    big_endian: bool,
) -> (bool, Option<Rc<Datatype>>) {
    let ty = data.vbank().get(vn).map(|v| Rc::clone(v.get_type()));
    let points_at_union = ty
        .as_ref()
        .map(|t| {
            t.get_metatype() == type_metatype::TYPE_PTR
                && t.get_ptr_to().map(|p| Rc::ptr_eq(&p, &subject.union_ty)).unwrap_or(false)
        })
        .unwrap_or(false);
    if points_at_union && kind == ObjKind::Mem {
        // The field path is spelled as pointer arithmetic on the object's own
        // base, so only a chain that reaches THIS root counts.
        let (r, off) = locate_ptr(data, vn);
        if r == root && off >= 0 && off < subject.layout.size as int8 {
            return (false, ty);
        }
    }
    let l = locate_value(data, vn, big_endian);
    (l.kind == kind && l.root == root, None)
}

/// The discriminant value a constant store of `width` bytes at `off` writes over
/// the tag, or `None` when the store does not cover the tag.
fn tag_slice(
    layout: &VariantLayout,
    off: int8,
    width: int4,
    value: uintb,
    big_endian: bool,
) -> Option<uintb> {
    let t0 = layout.tag_offset as int8;
    let t1 = t0 + layout.tag_size as int8;
    if off > t0 || off + (width as int8) < t1 {
        return None;
    }
    let shift_bytes = if big_endian { off + (width as int8) - t1 } else { t0 - off };
    let bits = (shift_bytes as u32) * 8;
    if bits >= 64 {
        return None;
    }
    let raw = value >> bits;
    let keep = (layout.tag_size as u32).min(8) * 8;
    Some(if keep >= 64 { raw } else { raw & ((1u64 << keep) - 1) })
}

/// The index into `layout.variants` a discriminant value selects.
fn variant_index_for_discr(layout: &VariantLayout, value: uintb) -> Option<usize> {
    if let Some(i) = layout.variants.iter().position(|v| v.discr == Some(value)) {
        return Some(i);
    }
    layout.variants.iter().position(|v| v.discr.is_none())
}

/// \brief Resolve a recovered Rust enum's union facet from the DWARF discriminant
/// and the guard that fixed it, instead of from `ScoreUnionFields`
/// (kuna `variantguard`; C++ has no analogue).
pub struct ActionVariantGuard {
    base: ActionBase,
}

impl ActionVariantGuard {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionVariantGuard { base: ActionBase::new(0, "variantguard", g) })
    }
}

impl Action for ActionVariantGuard {
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
        Some(Box::new(ActionVariantGuard { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option variantguard on|off`.  Off => no pin and no proof,
        // so the printer keeps the offset spelling `dwarfvariants` installs.
        if !data.get_arch().variantguard {
            return 0;
        }
        let n = run(data);
        data.kuna_seal_variant_proofs();
        self.base.count += n;
        0
    }
}

#[cfg(test)]
#[path = "kuna_variantguard/tests.rs"]
mod tests;
