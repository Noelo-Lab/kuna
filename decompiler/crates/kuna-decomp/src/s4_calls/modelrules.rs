//! Port of `decompiler/cpp/modelrules.{cc,hh}` (W6, item `w6-s4-modelrules`):
//! the declarative `<modelrule>` parameter-assignment machinery decoded from a
//! compiler spec (`.cspec`).
//!
//! The pieces:
//!
//!   - [`PrimitiveExtractor`] — recursively collects the formal *primitive*
//!     elements of a composite data-type, laying them out with offsets, and
//!     records boolean properties (unknown / unaligned / holes / invalid).
//!   - [`DatatypeFilter`] — selects a class of data-type ([`SizeRestrictedFilter`],
//!     [`MetaTypeFilter`], [`HomogeneousAggregate`]).
//!   - [`QualifierFilter`] — filters on aspects of a function prototype
//!     ([`AndFilter`], [`VarargsFilter`], [`PositionMatchFilter`],
//!     [`DatatypeMatchFilter`]).
//!   - [`AssignAction`] — an action that assigns an [`Address`] to a parameter
//!     ([`GotoStack`], [`ConvertToPointer`], [`MultiSlotAssign`],
//!     [`MultiMemberAssign`], [`MultiSlotDualAssign`], [`ConsumeAs`],
//!     [`HiddenReturnAssign`], [`ConsumeExtra`], [`ExtraStack`],
//!     [`ConsumeRemaining`]).
//!   - [`ModelRule`] — a rule combining a [`DatatypeFilter`], an optional
//!     [`QualifierFilter`], an [`AssignAction`], and pre-/side-effect actions.
//!
//! ## Seam (`SEAM(w6-modelrules)` in `fspec.rs`)
//!
//! The C++ `AssignAction` holds a `const ParamListStandard *resource`
//! back-pointer, and `ParamListStandard::modelRules` owns a
//! `vector<ModelRule>`.  In the kuna port `fspec.rs`'s `ParamListStandard`
//! carries an **uninhabitable** placeholder `ModelRule` enum (the documented
//! seam), and `fspec.rs` is **not** owned by this item — so the real
//! [`ModelRule`] here cannot be planted into `ParamListStandard::model_rules`
//! without editing `fspec.rs`.  Instead this module is **standalone**: the
//! [`AssignAction`]/[`ModelRule`] methods take the owning
//! [`ParamListStandard`] resource (and the [`AddrSpaceManager`]) as **explicit
//! arguments** rather than holding a back-pointer.  This is a faithful
//! transcription of every C++ method body — only the `resource->` / `this->`
//! field accesses become parameter accesses — and is exercised directly by the
//! tests (decode + assignment over real-shaped `.cspec` constructs).  Wiring
//! these into `ParamListStandard::assignAddress`/`fillinMap` requires the
//! fspec.rs `ModelRule` placeholder to be replaced (a later, fspec-owned step);
//! that integration is recorded as a loss for this item.
//!
//! The `Architecture *glb` reach in the C++ (`tlist.getArch()`,
//! `tlist.getArch()->getDefaultDataSpace()`) is replaced by the
//! [`AddrSpaceManager`] threaded through `assign_address`, matching the
//! `fspec-2` seam (`ProtoModel` has no `glb` back-pointer in the kuna port).
//!
//! Integer model per ADR 0003: `uintb->u64`, `intb->i64`, `int4->i32`,
//! `uint4->u32`; the `uint4` AssignAction response is modeled by the fspec
//! [`AssignActionResponse`] enum.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    AttributeId, Decoder, ElementId, ATTRIB_ALIGN, ATTRIB_INDEX, ATTRIB_NAME, ATTRIB_STACKSPILL,
    ATTRIB_STORAGE,
};
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::{int4, uint4};
use kuna_num::pcoderaw::VarnodeData;

use crate::dtype::{
    metatype2typeclass, string2metatype, string2typeclass, type_class, type_metatype, Datatype,
    TypeFactory,
};
use crate::fspec::{
    parameter_pieces_flags, AssignActionResponse, ParamActive, ParamEntry, ParamListStandard,
    ParamListType, ParameterPieces, PrototypePieces,
};

// =============================================================================
// Marshaling ids (modelrules.cc:21-42, plus the borrowed fspec/architecture ids)
// =============================================================================
//
// These ids are module-private: the W4 decode wiring that would register them
// in the shared `IdRegistry` is not yet ported, so they live here with the
// exact C++ id numbers (so the byte/string protocol matches when a decoder is
// configured with them; tests register them explicitly).

/// Marshaling attribute "sizes" (modelrules.cc:21).
pub const ATTRIB_SIZES: AttributeId = AttributeId::new("sizes", 151);
/// Marshaling attribute "maxprimitives" (modelrules.cc:22).
pub const ATTRIB_MAX_PRIMITIVES: AttributeId = AttributeId::new("maxprimitives", 153);
/// Marshaling attribute "reversesignif" (modelrules.cc:23).
pub const ATTRIB_REVERSESIGNIF: AttributeId = AttributeId::new("reversesignif", 154);
/// Marshaling attribute "matchsize" (modelrules.cc:24).
pub const ATTRIB_MATCHSIZE: AttributeId = AttributeId::new("matchsize", 155);
/// Marshaling attribute "afterbytes" (modelrules.cc:25).
pub const ATTRIB_AFTER_BYTES: AttributeId = AttributeId::new("afterbytes", 156);
/// Marshaling attribute "afterstorage" (modelrules.cc:26).
pub const ATTRIB_AFTER_STORAGE: AttributeId = AttributeId::new("afterstorage", 157);
/// Marshalling attribute "fillalternate" (modelrules.cc:27).
pub const ATTRIB_FILL_ALTERNATE: AttributeId = AttributeId::new("fillalternate", 158);

// Borrowed from fspec.cc / architecture.cc (their kuna ids are not yet defined;
// the modelrules decode references them, so they are carried here with their
// upstream id numbers — fspec.cc:29-38, architecture.cc:48).
/// Marshaling attribute "maxsize" (fspec.cc:29).
pub const ATTRIB_MAXSIZE: AttributeId = AttributeId::new("maxsize", 120);
/// Marshaling attribute "minsize" (fspec.cc:30).
pub const ATTRIB_MINSIZE: AttributeId = AttributeId::new("minsize", 121);
/// Marshaling attribute "strategy" (fspec.cc:36).
pub const ATTRIB_STRATEGY: AttributeId = AttributeId::new("strategy", 127);
/// Marshaling attribute "voidlock" (fspec.cc:38).
pub const ATTRIB_VOIDLOCK: AttributeId = AttributeId::new("voidlock", 129);
/// Marshaling attribute "reversejustify" (architecture.cc:48).
pub const ATTRIB_REVERSEJUSTIFY: AttributeId = AttributeId::new("reversejustify", 111);
/// Marshaling attribute "first" (address.cc:21).
pub const ATTRIB_FIRST: AttributeId = AttributeId::new("first", 27);
/// Marshaling attribute "last" (address.cc:22).
pub const ATTRIB_LAST: AttributeId = AttributeId::new("last", 28);
/// Marshaling attribute "a" (cpool: id 80, shared name).
pub const ATTRIB_A: AttributeId = AttributeId::new("a", 80);
/// Marshaling attribute "b" (cpool: id 81, shared name).
pub const ATTRIB_B: AttributeId = AttributeId::new("b", 81);

/// Marshaling element \<datatype> (modelrules.cc:29).
pub const ELEM_DATATYPE: ElementId = ElementId::new("datatype", 273);
/// Marshaling element \<consume> (modelrules.cc:30).
pub const ELEM_CONSUME: ElementId = ElementId::new("consume", 274);
/// Marshaling element \<consume_extra> (modelrules.cc:31).
pub const ELEM_CONSUME_EXTRA: ElementId = ElementId::new("consume_extra", 275);
/// Marshaling element \<convert_to_ptr> (modelrules.cc:32).
pub const ELEM_CONVERT_TO_PTR: ElementId = ElementId::new("convert_to_ptr", 276);
/// Marshaling element \<goto_stack> (modelrules.cc:33).
pub const ELEM_GOTO_STACK: ElementId = ElementId::new("goto_stack", 277);
/// Marshaling element \<join> (modelrules.cc:34).
pub const ELEM_JOIN: ElementId = ElementId::new("join", 278);
/// Marshaling element \<datatype_at> (modelrules.cc:35).
pub const ELEM_DATATYPE_AT: ElementId = ElementId::new("datatype_at", 279);
/// Marshaling element \<position> (modelrules.cc:36).
pub const ELEM_POSITION: ElementId = ElementId::new("position", 280);
/// Marshaling element \<varargs> (modelrules.cc:37).
pub const ELEM_VARARGS: ElementId = ElementId::new("varargs", 281);
/// Marshaling element \<hidden_return> (modelrules.cc:38).
pub const ELEM_HIDDEN_RETURN: ElementId = ElementId::new("hidden_return", 282);
/// Marshaling element \<join_per_primitive> (modelrules.cc:39).
pub const ELEM_JOIN_PER_PRIMITIVE: ElementId = ElementId::new("join_per_primitive", 283);
/// Marshaling element \<join_dual_class> (modelrules.cc:40).
pub const ELEM_JOIN_DUAL_CLASS: ElementId = ElementId::new("join_dual_class", 285);
/// Marshaling element \<extra_stack> (modelrules.cc:41).
pub const ELEM_EXTRA_STACK: ElementId = ElementId::new("extra_stack", 287);
/// Marshaling element \<consume_remaining> (modelrules.cc:42).
pub const ELEM_CONSUME_REMAINING: ElementId = ElementId::new("consume_remaining", 288);
/// Marshaling element \<rule> (architecture.cc:75).
pub const ELEM_RULE: ElementId = ElementId::new("rule", 153);

/// Register every modelrules `<rule>`/action element id and the attribute ids the
/// `ModelRule`/`AssignAction`/`DatatypeFilter`/`QualifierFilter` decoders consult,
/// so an [`IdRegistry`](kuna_base::marshal::IdRegistry) configured for cspec
/// decoding can drive `ModelRule::decode`.  Registration is keyed by name and is
/// idempotent, matching the C++ static `ElementId`/`AttributeId` table (the kuna
/// registry is the explicit form of the C++ global id space).
pub fn register_ids(reg: &mut kuna_base::marshal::IdRegistry) {
    for e in [
        &ELEM_DATATYPE,
        &ELEM_CONSUME,
        &ELEM_CONSUME_EXTRA,
        &ELEM_CONVERT_TO_PTR,
        &ELEM_GOTO_STACK,
        &ELEM_JOIN,
        &ELEM_DATATYPE_AT,
        &ELEM_POSITION,
        &ELEM_VARARGS,
        &ELEM_HIDDEN_RETURN,
        &ELEM_JOIN_PER_PRIMITIVE,
        &ELEM_JOIN_DUAL_CLASS,
        &ELEM_EXTRA_STACK,
        &ELEM_CONSUME_REMAINING,
        &ELEM_RULE,
    ] {
        reg.register_element(e);
    }
    for a in [
        &ATTRIB_SIZES,
        &ATTRIB_MAX_PRIMITIVES,
        &ATTRIB_REVERSESIGNIF,
        &ATTRIB_MATCHSIZE,
        &ATTRIB_AFTER_BYTES,
        &ATTRIB_AFTER_STORAGE,
        &ATTRIB_FILL_ALTERNATE,
        &ATTRIB_MAXSIZE,
        &ATTRIB_MINSIZE,
        &ATTRIB_STRATEGY,
        &ATTRIB_VOIDLOCK,
        &ATTRIB_REVERSEJUSTIFY,
        &ATTRIB_FIRST,
        &ATTRIB_LAST,
        &ATTRIB_A,
        &ATTRIB_B,
    ] {
        reg.register_attribute(a);
    }
}

// `AssignActionResponse` is the fspec `enum` inside `AssignAction`; reuse the
// faithful constants (modelrules.hh:264-270).
use AssignActionResponse::{
    fail as RES_FAIL, hiddenret_ptrparam as RES_HIDDENRET_PTRPARAM,
    hiddenret_specialreg as RES_HIDDENRET_SPECIALREG,
    hiddenret_specialreg_void as RES_HIDDENRET_SPECIALREG_VOID, success as RES_SUCCESS,
};

// =============================================================================
// PrimitiveExtractor (modelrules.hh:54-89, modelrules.cc:44-248)
// =============================================================================

/// Boolean property flags for a [`PrimitiveExtractor`] (C++ anonymous enum,
/// `modelrules.hh:59-65`).
mod prim_flags {
    use kuna_base::types::uint4;
    /// Contains at least one TYPE_UNKNOWN primitive.
    pub const UNKNOWN_ELEMENT: uint4 = 1;
    /// At least one primitive is not properly aligned.
    pub const UNALIGNED: uint4 = 2;
    /// Data-type contains empty space not attributable to alignment padding.
    pub const EXTRA_SPACE: uint4 = 4;
    /// Data-type exceeded maximum or contained illegal elements.
    pub const INVALID: uint4 = 8;
    /// Unions are treated as an illegal element.
    pub const UNION_INVALID: uint4 = 16;
}

/// A primitive data-type and its offset within the containing data-type (C++
/// `PrimitiveExtractor::Primitive`, `modelrules.hh:67-73`).
#[derive(Debug, Clone)]
pub struct Primitive {
    /// Primitive data-type (C++ `dt`).
    pub dt: Rc<Datatype>,
    /// Offset within container (C++ `offset`).
    pub offset: int4,
}

impl Primitive {
    /// Constructor (C++ `Primitive(Datatype *d,int4 off)`).
    pub fn new(dt: Rc<Datatype>, off: int4) -> Primitive {
        Primitive { dt, offset: off }
    }
}

/// Extracts the primitive elements of a (possibly composite) data-type (C++
/// `PrimitiveExtractor`, `modelrules.hh:58-89`).
#[derive(Debug, Clone)]
pub struct PrimitiveExtractor {
    /// List of extracted primitives (C++ `primitives`).
    primitives: Vec<Primitive>,
    /// Boolean properties of the data-type (C++ `flags`).
    flags: uint4,
}

impl PrimitiveExtractor {
    /// Construct from a data-type (C++
    /// `PrimitiveExtractor(Datatype *dt,bool unionIllegal,int offset,int4 max)`,
    /// modelrules.cc:242-248).
    pub fn new(dt: &Rc<Datatype>, union_illegal: bool, offset: int4, max: int4) -> PrimitiveExtractor {
        let mut res = PrimitiveExtractor {
            primitives: Vec::new(),
            flags: if union_illegal { prim_flags::UNION_INVALID } else { 0 },
        };
        if !res.extract(dt, max, offset) {
            res.flags |= prim_flags::INVALID;
        }
        res
    }

    /// Number of primitives extracted (C++ `size`).
    pub fn size(&self) -> int4 {
        self.primitives.len() as int4 // cast: count fits int4 (C++ stores size() as int4)
    }
    /// Get a particular primitive (C++ `get`).
    pub fn get(&self, i: int4) -> &Primitive {
        &self.primitives[i as usize]
    }
    /// True if primitives were successfully extracted (C++ `isValid`).
    pub fn is_valid(&self) -> bool {
        (self.flags & prim_flags::INVALID) == 0
    }
    /// Are there \b unknown elements (C++ `containsUnknown`).
    pub fn contains_unknown(&self) -> bool {
        (self.flags & prim_flags::UNKNOWN_ELEMENT) != 0
    }
    /// Are all elements aligned (C++ `isAligned`).
    pub fn is_aligned(&self) -> bool {
        (self.flags & prim_flags::UNALIGNED) == 0
    }
    /// Is there empty space that is not padding (C++ `containsHoles`).
    pub fn contains_holes(&self) -> bool {
        (self.flags & prim_flags::EXTRA_SPACE) != 0
    }

    /// Check that a big Primitive properly overlaps smaller Primitives (C++
    /// `checkOverlap`, modelrules.cc:56-75).  Returns the index of the next
    /// primitive after the overlap, or `None` (C++ -1) if the overlap is invalid.
    fn check_overlap(
        res: &mut Vec<Primitive>,
        small: &[Primitive],
        mut point: int4,
        big: &Primitive,
    ) -> Option<int4> {
        let end_off = big.offset + big.dt.get_align_size();
        // If big data-type is a float, let smaller primitives override it,
        // otherwise keep the big primitive.
        let use_small = big.dt.get_metatype() == type_metatype::TYPE_FLOAT;
        while point < small.len() as int4 {
            let cur = &small[point as usize];
            let mut cur_off = cur.offset;
            if cur_off >= end_off {
                break;
            }
            cur_off += cur.dt.get_align_size();
            if cur_off > end_off {
                return None; // Improper overlap of the end of big
            }
            if use_small {
                res.push(cur.clone());
            }
            point += 1;
        }
        if !use_small {
            // big data-type was preferred
            res.push(big.clone()); // use big Primitive in the refinement
        }
        Some(point)
    }

    /// Overwrite `first` list with the common refinement of `first` and
    /// `second` (C++ `commonRefinement`, modelrules.cc:88-131).
    fn common_refinement(first: &mut Vec<Primitive>, second: &[Primitive]) -> bool {
        let mut first_point: int4 = 0;
        let mut second_point: int4 = 0;
        let mut common: Vec<Primitive> = Vec::new();
        while first_point < first.len() as int4 && second_point < second.len() as int4 {
            // Clone the active elements so the borrow doesn't pin `first`/`common`.
            let first_element = first[first_point as usize].clone();
            let second_element = second[second_point as usize].clone();
            if first_element.offset < second_element.offset
                && first_element.offset + first_element.dt.get_align_size() <= second_element.offset
            {
                common.push(first_element);
                first_point += 1;
                continue;
            }
            if second_element.offset < first_element.offset
                && second_element.offset + second_element.dt.get_align_size() <= first_element.offset
            {
                common.push(second_element);
                second_point += 1;
                continue;
            }
            if first_element.dt.get_align_size() >= second_element.dt.get_align_size() {
                match Self::check_overlap(&mut common, second, second_point, &first_element) {
                    None => return false,
                    Some(sp) => second_point = sp,
                }
                first_point += 1;
            } else {
                match Self::check_overlap(&mut common, first, first_point, &second_element) {
                    None => return false,
                    Some(fp) => first_point = fp,
                }
                second_point += 1;
            }
        }
        // Add any tail primitives from either list.
        while first_point < first.len() as int4 {
            common.push(first[first_point as usize].clone());
            first_point += 1;
        }
        while second_point < second.len() as int4 {
            common.push(second[second_point as usize].clone());
            second_point += 1;
        }
        *first = common; // Replace first with the refinement (C++ first.swap(common))
        true
    }

    /// Add primitives representing a union data-type (C++ `handleUnion`,
    /// modelrules.cc:139-164).
    fn handle_union(&mut self, dt: &Rc<Datatype>, max: int4, offset: int4) -> bool {
        if (self.flags & prim_flags::UNION_INVALID) != 0 {
            return false;
        }
        let num = dt.num_depend();
        if num == 0 {
            return false;
        }
        let cur_field = match dt.get_field(0) {
            Some(f) => f,
            None => return false,
        };
        let mut common = PrimitiveExtractor::new(
            &cur_field.field_type,
            false,
            offset + cur_field.offset,
            max,
        );
        if !common.is_valid() {
            return false;
        }
        for i in 1..num {
            let cur_field = match dt.get_field(i) {
                Some(f) => f,
                None => return false,
            };
            let next = PrimitiveExtractor::new(
                &cur_field.field_type,
                false,
                offset + cur_field.offset,
                max,
            );
            if !next.is_valid() {
                return false;
            }
            if !Self::common_refinement(&mut common.primitives, &next.primitives) {
                return false;
            }
        }
        if self.primitives.len() as int4 + common.primitives.len() as int4 > max {
            return false;
        }
        for p in common.primitives.iter() {
            self.primitives.push(p.clone());
        }
        true
    }

    /// Extract list of primitives from given data-type (C++ `extract`,
    /// modelrules.cc:179-236).
    fn extract(&mut self, dt: &Rc<Datatype>, max: int4, offset: int4) -> bool {
        match dt.get_metatype() {
            type_metatype::TYPE_UNKNOWN => {
                self.flags |= prim_flags::UNKNOWN_ELEMENT; // contains an unknown primitive
                // fallthru
                if self.primitives.len() as int4 >= max {
                    return false;
                }
                self.primitives.push(Primitive::new(Rc::clone(dt), offset));
                true
            }
            type_metatype::TYPE_INT
            | type_metatype::TYPE_UINT
            | type_metatype::TYPE_BOOL
            | type_metatype::TYPE_CODE
            | type_metatype::TYPE_FLOAT
            | type_metatype::TYPE_PTR
            | type_metatype::TYPE_PTRREL => {
                if self.primitives.len() as int4 >= max {
                    return false;
                }
                self.primitives.push(Primitive::new(Rc::clone(dt), offset));
                true
            }
            type_metatype::TYPE_ARRAY => {
                let num_els = dt.num_elements().unwrap_or(0);
                let base = match dt.get_array_base() {
                    Some(b) => b,
                    None => return false,
                };
                let mut off = offset;
                for _ in 0..num_els {
                    if !self.extract(&base, max, off) {
                        return false;
                    }
                    off += base.get_align_size();
                }
                true
            }
            type_metatype::TYPE_UNION => self.handle_union(dt, max, offset),
            type_metatype::TYPE_STRUCT => self.extract_struct(dt, max, offset),
            _ => false,
        }
    }

    /// The `TYPE_STRUCT` tail of [`Self::extract`] (modelrules.cc:215-235).
    fn extract_struct(&mut self, dt: &Rc<Datatype>, max: int4, offset: int4) -> bool {
        let num = dt.num_depend();
        let mut expected_off = offset;
        for i in 0..num {
            let (comp_dt, field_off) = {
                let field = match dt.get_field(i) {
                    Some(f) => f,
                    None => return false,
                };
                (Rc::clone(&field.field_type), field.offset)
            };
            let cur_off = field_off + offset;
            let align = comp_dt.get_alignment();
            if cur_off % align != 0 {
                self.flags |= prim_flags::UNALIGNED;
            }
            let rem = expected_off % align;
            if rem != 0 {
                expected_off += align - rem;
            }
            if expected_off != cur_off {
                self.flags |= prim_flags::EXTRA_SPACE;
            }
            if !self.extract(&comp_dt, max, cur_off) {
                return false;
            }
            expected_off = cur_off + comp_dt.get_align_size();
        }
        true
    }
}

// =============================================================================
// DatatypeFilter (modelrules.hh:91-166, modelrules.cc:250-445)
// =============================================================================

/// A filter selecting a specific class of data-type (C++ `DatatypeFilter`
/// family, `modelrules.hh:91-166`).  The C++ class hierarchy
/// (`SizeRestrictedFilter` base, `MetaTypeFilter`, `HomogeneousAggregate`) is
/// collapsed into one enum tagged by variant; the per-variant method bodies
/// dispatch on the variant.
#[derive(Debug, Clone)]
pub enum DatatypeFilter {
    /// `SizeRestrictedFilter` — bounds on data-type size only.
    SizeRestricted(SizeRestriction),
    /// `MetaTypeFilter` — a single meta data-type plus size bounds.
    MetaType { meta_type: type_metatype, size: SizeRestriction },
    /// `HomogeneousAggregate` — an aggregate whose primitives are all the same.
    Homogeneous { meta_type: type_metatype, max_primitives: int4, size: SizeRestriction },
}

/// The size-restriction state shared by every [`DatatypeFilter`] (C++
/// `SizeRestrictedFilter`, `modelrules.hh:123-137`).
#[derive(Debug, Clone)]
pub struct SizeRestriction {
    /// Minimum size of the data-type in bytes (C++ `minSize`).
    min_size: int4,
    /// Maximum size of the data-type in bytes (C++ `maxSize`).
    max_size: int4,
    /// An enumerated list of sizes (if not empty) (C++ `sizes`, a `set<int4>`).
    sizes: std::collections::BTreeSet<int4>,
}

impl SizeRestriction {
    /// Constructor for use with decode (C++ `SizeRestrictedFilter()`,
    /// modelrules.hh:130): `minSize=0; maxSize=0`.
    pub fn new_for_decode() -> SizeRestriction {
        SizeRestriction { min_size: 0, max_size: 0, sizes: std::collections::BTreeSet::new() }
    }

    /// Constructor (C++ `SizeRestrictedFilter(int4 min,int4 max)`,
    /// modelrules.cc:303-312).
    pub fn new(min: int4, max: int4) -> SizeRestriction {
        let mut res = SizeRestriction {
            min_size: min,
            max_size: max,
            sizes: std::collections::BTreeSet::new(),
        };
        if res.max_size == 0 && res.min_size >= 0 {
            // No ATTRIB_MAXSIZE given -> no upper bound on size.
            res.max_size = 0x7fffffff;
        }
        res
    }

    /// Initialize filter from enumerated list of sizes (C++ `initFromSizeList`,
    /// modelrules.cc:277-301).
    fn init_from_size_list(&mut self, str: &[u8]) -> KunaResult<()> {
        // C++ `istringstream s(str); for(;;) { val=-1; s>>ws; if (eof) break;
        // if (peek==',') s>>punc>>ws; s>>val; if (val<=0) throw; sizes.insert(val); }`
        let parsed = parse_size_list(str)?;
        for &val in parsed.iter() {
            self.sizes.insert(val);
        }
        if !self.sizes.is_empty() {
            self.min_size = *self.sizes.iter().next().unwrap();
            // C++ `iter = sizes.end(); --iter; maxSize = *iter;`
            self.max_size = *self.sizes.iter().next_back().unwrap();
        }
        Ok(())
    }

    /// Enforce any size bounds on a given data-type (C++ `filterOnSize`,
    /// modelrules.cc:327-335).
    fn filter_on_size(&self, dt: &Datatype) -> bool {
        if self.max_size == 0 {
            return true; // maxSize of 0 means no size filtering
        }
        if !self.sizes.is_empty() {
            return self.sizes.contains(&dt.get_size());
        }
        dt.get_size() >= self.min_size && dt.get_size() <= self.max_size
    }

    /// Decode the size attributes from the stream (C++
    /// `SizeRestrictedFilter::decode`, modelrules.cc:337-364).
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_MINSIZE {
                if !self.sizes.is_empty() {
                    return Err(KunaError::decoder(
                        "Mixing \"sizes\" with \"minsize\" and \"maxsize\"",
                    ));
                }
                self.min_size = decoder.read_unsigned_integer()? as int4; // cast: int4 member
            } else if attrib_id == ATTRIB_MAXSIZE {
                if !self.sizes.is_empty() {
                    return Err(KunaError::decoder(
                        "Mixing \"sizes\" with \"minsize\" and \"maxsize\"",
                    ));
                }
                self.max_size = decoder.read_unsigned_integer()? as int4; // cast: int4 member
            } else if attrib_id == ATTRIB_SIZES {
                if self.min_size != 0 || self.max_size != 0 {
                    return Err(KunaError::decoder(
                        "Mixing \"sizes\" with \"minsize\" and \"maxsize\"",
                    ));
                }
                let size_list = decoder.read_string()?;
                self.init_from_size_list(&size_list)?;
            }
        }
        if self.max_size == 0 && self.min_size >= 0 {
            // No ATTRIB_MAXSIZE given -> no upper bound on size.
            self.max_size = 0x7fffffff;
        }
        Ok(())
    }
}

impl DatatypeFilter {
    /// Test whether the given data-type belongs to this filter's class (C++
    /// `filter`, the per-subclass overrides).
    pub fn filter(&self, dt: &Rc<Datatype>) -> bool {
        match self {
            // SizeRestrictedFilter::filter (modelrules.hh:135).
            DatatypeFilter::SizeRestricted(sz) => sz.filter_on_size(dt),
            // MetaTypeFilter::filter (modelrules.cc:384-389).
            DatatypeFilter::MetaType { meta_type, size } => {
                if dt.get_metatype() != *meta_type {
                    return false;
                }
                size.filter_on_size(dt)
            }
            // HomogeneousAggregate::filter (modelrules.cc:412-430).
            DatatypeFilter::Homogeneous { meta_type, max_primitives, size: _ } => {
                let meta = dt.get_metatype();
                if meta != type_metatype::TYPE_ARRAY && meta != type_metatype::TYPE_STRUCT {
                    return false;
                }
                let primitives = PrimitiveExtractor::new(dt, true, 0, *max_primitives);
                if !primitives.is_valid()
                    || primitives.size() == 0
                    || primitives.contains_unknown()
                    || !primitives.is_aligned()
                    || primitives.contains_holes()
                {
                    return false;
                }
                let base = &primitives.get(0).dt;
                if base.get_metatype() != *meta_type {
                    return false;
                }
                for i in 1..primitives.size() {
                    // C++ compares the Datatype pointers directly.
                    if !Rc::ptr_eq(&primitives.get(i).dt, base) {
                        return false;
                    }
                }
                true
            }
        }
    }

    /// Instantiate a filter from the given stream (C++
    /// `DatatypeFilter::decodeFilter`, modelrules.cc:252-272).
    pub fn decode_filter(decoder: &mut dyn Decoder) -> KunaResult<DatatypeFilter> {
        let elem_id = decoder.open_element_id(&ELEM_DATATYPE)?;
        let nm = decoder.read_string_id(&ATTRIB_NAME)?;
        let mut filter = if nm == b"any" {
            DatatypeFilter::SizeRestricted(SizeRestriction::new_for_decode())
        } else if nm == b"homogeneous-float-aggregate" {
            DatatypeFilter::Homogeneous {
                meta_type: type_metatype::TYPE_FLOAT,
                max_primitives: 4,
                size: SizeRestriction::new(0, 0),
            }
        } else {
            // If no other name matches, assume this is a metatype.
            let s = std::str::from_utf8(&nm)
                .map_err(|_| KunaError::decoder("Non-UTF8 metatype name"))?;
            let meta = string2metatype(s)?;
            DatatypeFilter::MetaType { meta_type: meta, size: SizeRestriction::new_for_decode() }
        };
        filter.decode(decoder)?;
        decoder.close_element(elem_id)?;
        Ok(filter)
    }

    /// Configure details of the data-type class from the stream (C++ `decode`,
    /// per-subclass).
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        match self {
            DatatypeFilter::SizeRestricted(sz) => sz.decode(decoder),
            DatatypeFilter::MetaType { size, .. } => size.decode(decoder),
            DatatypeFilter::Homogeneous { max_primitives, size, .. } => {
                // HomogeneousAggregate::decode (modelrules.cc:432-445).
                size.decode(decoder)?;
                decoder.rewind_attributes();
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    }
                    if attrib_id == ATTRIB_MAX_PRIMITIVES {
                        let xml_max_prim = decoder.read_unsigned_integer()? as uint4;
                        if xml_max_prim > 0 {
                            *max_primitives = xml_max_prim as int4; // cast: int4 member
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

// =============================================================================
// QualifierFilter (modelrules.hh:168-256, modelrules.cc:447-577)
// =============================================================================

/// A filter on some aspect of a specific function prototype (C++
/// `QualifierFilter` family, `modelrules.hh:168-256`).  The C++ hierarchy
/// (`AndFilter`, `VarargsFilter`, `PositionMatchFilter`, `DatatypeMatchFilter`)
/// is collapsed into this enum.
#[derive(Debug, Clone)]
pub enum QualifierFilter {
    /// `AndFilter` — all sub-filters must pass.
    And(Vec<QualifierFilter>),
    /// `VarargsFilter` — a range of optional (varargs) parameters.
    Varargs { first_pos: int4, last_pos: int4 },
    /// `PositionMatchFilter` — a specific parameter position.
    Position(int4),
    /// `DatatypeMatchFilter` — a data-type at a fixed position.
    DatatypeMatch { position: int4, type_filter: Box<DatatypeFilter> },
}

impl QualifierFilter {
    /// Test whether the prototype meets this filter's criteria (C++ `filter`,
    /// per-subclass overrides).
    pub fn filter(&self, proto: &PrototypePieces, pos: int4) -> bool {
        match self {
            // AndFilter::filter (modelrules.cc:492-500).
            QualifierFilter::And(subs) => {
                for sub in subs.iter() {
                    if !sub.filter(proto, pos) {
                        return false;
                    }
                }
                true
            }
            // VarargsFilter::filter (modelrules.cc:502-508).
            QualifierFilter::Varargs { first_pos, last_pos } => {
                if proto.first_var_arg_slot < 0 {
                    return false;
                }
                let pos = pos - proto.first_var_arg_slot;
                pos >= *first_pos && pos <= *last_pos
            }
            // PositionMatchFilter::filter (modelrules.cc:525-529).
            QualifierFilter::Position(position) => pos == *position,
            // DatatypeMatchFilter::filter (modelrules.cc:555-568).
            QualifierFilter::DatatypeMatch { position, type_filter } => {
                // The position of the current parameter being assigned, pos, is NOT used.
                let dt = if *position < 0 {
                    match &proto.outtype {
                        Some(t) => Rc::clone(t),
                        None => return false, // C++ would call filter(null) — guard
                    }
                } else {
                    if *position >= proto.intypes.len() as int4 {
                        return false;
                    }
                    Rc::clone(&proto.intypes[*position as usize])
                };
                type_filter.filter(&dt)
            }
        }
    }

    /// Try to instantiate a qualifier filter from the stream (C++
    /// `QualifierFilter::decodeFilter`, modelrules.cc:451-466).  Returns `None`
    /// (C++ null) if the next element is not a qualifier filter.
    pub fn decode_filter(decoder: &mut dyn Decoder) -> KunaResult<Option<QualifierFilter>> {
        let elem_id = decoder.peek_element()?;
        let mut filter = if elem_id == ELEM_VARARGS.get_id() {
            // VarargsFilter() — firstPos = 0x80000000; lastPos = 0x7fffffff.
            QualifierFilter::Varargs { first_pos: 0x80000000u32 as i32, last_pos: 0x7fffffff }
        } else if elem_id == ELEM_POSITION.get_id() {
            QualifierFilter::Position(-1)
        } else if elem_id == ELEM_DATATYPE_AT.get_id() {
            // DatatypeMatchFilter() — position = -1; typeFilter = null.
            QualifierFilter::DatatypeMatch {
                position: -1,
                type_filter: Box::new(DatatypeFilter::SizeRestricted(
                    SizeRestriction::new_for_decode(),
                )),
            }
        } else {
            return Ok(None);
        };
        filter.decode(decoder)?;
        Ok(Some(filter))
    }

    /// Configure details of the criteria from the stream (C++ `decode`,
    /// per-subclass).
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        match self {
            // AndFilter::decode is empty (modelrules.hh:206).
            QualifierFilter::And(_) => Ok(()),
            // VarargsFilter::decode (modelrules.cc:510-523).
            QualifierFilter::Varargs { first_pos, last_pos } => {
                let elem_id = decoder.open_element_id(&ELEM_VARARGS)?;
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    }
                    if attrib_id == ATTRIB_FIRST {
                        *first_pos = decoder.read_signed_integer()? as int4; // cast: int4 member
                    } else if attrib_id == ATTRIB_LAST {
                        *last_pos = decoder.read_signed_integer()? as int4; // cast: int4 member
                    }
                }
                decoder.close_element(elem_id)?;
                Ok(())
            }
            // PositionMatchFilter::decode (modelrules.cc:531-537).
            QualifierFilter::Position(position) => {
                let elem_id = decoder.open_element_id(&ELEM_POSITION)?;
                *position = decoder.read_signed_integer_id(&ATTRIB_INDEX)? as int4; // cast: int4 member
                decoder.close_element(elem_id)?;
                Ok(())
            }
            // DatatypeMatchFilter::decode (modelrules.cc:570-577).
            QualifierFilter::DatatypeMatch { position, type_filter } => {
                let elem_id = decoder.open_element_id(&ELEM_DATATYPE_AT)?;
                *position = decoder.read_signed_integer_id(&ATTRIB_INDEX)? as int4; // cast: int4 member
                *type_filter = Box::new(DatatypeFilter::decode_filter(decoder)?);
                decoder.close_element(elem_id)?;
                Ok(())
            }
        }
    }
}

// =============================================================================
// AssignAction (modelrules.hh:257-529, modelrules.cc:579-1588)
// =============================================================================

/// An action that assigns an Address to a function prototype parameter (C++
/// `AssignAction` family, `modelrules.hh:257-529`).  The C++ hierarchy is
/// collapsed into this enum; the per-variant method bodies dispatch on the
/// variant.
///
/// Cached `ParamEntry` lookups (the C++ `initializeEntries`/`initializeEntry`
/// that store `const ParamEntry *` pointers) are modeled as **indices** into
/// the owning `ParamListStandard::entry` vector and resolved against the
/// `resource` passed to [`AssignAction::initialize`].
#[derive(Debug, Clone)]
pub enum AssignAction {
    /// `GotoStack` — assign from the next available stack location.
    GotoStack {
        /// Index of the stack `ParamEntry` (C++ `stackEntry`); `None` until
        /// [`AssignAction::initialize`].
        stack_entry: Option<usize>,
    },
    /// `ConvertToPointer` — convert the data-type to a pointer.
    ConvertToPointer {
        /// Address space used for pointer size (C++ `space`), or `None` for the
        /// default data space.
        space: Option<Rc<AddrSpace>>,
    },
    /// `MultiSlotAssign` — consume multiple registers, joining them.
    MultiSlotAssign {
        resource_type: type_class,
        is_big_endian: bool,
        consume_from_stack: bool,
        consume_most_sig: bool,
        enforce_alignment: bool,
        justify_right: bool,
        /// Indices of joinable register entries (C++ `tiles`).
        tiles: Vec<usize>,
        /// Index of the stack entry (C++ `stackEntry`).
        stack_entry: Option<usize>,
    },
    /// `MultiMemberAssign` — a register per primitive member.
    MultiMemberAssign { resource_type: type_class, consume_from_stack: bool, consume_most_sig: bool },
    /// `MultiSlotDualAssign` — consume from two storage classes.
    MultiSlotDualAssign {
        base_type: type_class,
        alt_type: type_class,
        is_big_endian: bool,
        consume_from_stack: bool,
        consume_most_sig: bool,
        justify_right: bool,
        fill_alternate: bool,
        tile_size: int4,
        base_tiles: Vec<usize>,
        alt_tiles: Vec<usize>,
        stack_entry: Option<usize>,
    },
    /// `ConsumeAs` — consume from a specific resource list.
    ConsumeAs { resource_type: type_class },
    /// `HiddenReturnAssign` — allocate the return value as an input parameter.
    HiddenReturnAssign { ret_code: AssignActionResponse },
    /// `ConsumeExtra` — side-effect consuming extra registers.
    ConsumeExtra { resource_type: type_class, match_size: bool, tiles: Vec<usize> },
    /// `ExtraStack` — side-effect consuming stack resources.
    ExtraStack { after_bytes: int4, after_storage: type_class, stack_entry: Option<usize> },
    /// `ConsumeRemaining` — side-effect consuming all remaining registers.
    ConsumeRemaining { resource_type: type_class, tiles: Vec<usize> },
}

impl AssignAction {
    /// Return `true` if `fillinOutputMap` is active for this action (C++
    /// `AssignAction::canAffectFillinOutput` reading `fillinOutputActive`).
    ///
    /// In the C++ this is a per-instance bool set in each constructor; the value
    /// is `true` for exactly the actions whose constructors set
    /// `fillinOutputActive = true` (`GotoStack`, `MultiSlotAssign`,
    /// `MultiMemberAssign`, `MultiSlotDualAssign`, `ConsumeAs`).
    pub fn can_affect_fillin_output(&self) -> bool {
        matches!(
            self,
            AssignAction::GotoStack { .. }
                | AssignAction::MultiSlotAssign { .. }
                | AssignAction::MultiMemberAssign { .. }
                | AssignAction::MultiSlotDualAssign { .. }
                | AssignAction::ConsumeAs { .. }
        )
    }

    /// Resolve the cached `ParamEntry` indices against `resource` (C++
    /// `initializeEntries`/`initializeEntry`, called from the constructors and
    /// the `decode` tails).  Separated out because the kuna actions hold no
    /// `resource` back-pointer.
    pub fn initialize(&mut self, resource: &ParamListStandard) -> KunaResult<()> {
        match self {
            // GotoStack::initializeEntry (modelrules.cc:696-702).
            AssignAction::GotoStack { stack_entry } => {
                *stack_entry = resource.get_stack_entry();
                if stack_entry.is_none() {
                    return Err(KunaError::lowlevel(
                        "Cannot find matching <pentry> for action: goto_stack",
                    ));
                }
                Ok(())
            }
            // MultiSlotAssign::initializeEntries (modelrules.cc:787-796).
            AssignAction::MultiSlotAssign {
                resource_type,
                consume_from_stack,
                tiles,
                stack_entry,
                ..
            } => {
                tiles.clear();
                resource.extract_tiles(tiles, *resource_type);
                *stack_entry = resource.get_stack_entry();
                if tiles.is_empty() {
                    return Err(KunaError::lowlevel(
                        "Could not find matching resources for action: join",
                    ));
                }
                if *consume_from_stack && stack_entry.is_none() {
                    return Err(KunaError::lowlevel(
                        "Cannot find matching <pentry> for action: join",
                    ));
                }
                Ok(())
            }
            // MultiSlotDualAssign::initializeEntries (modelrules.cc:1064-1078).
            AssignAction::MultiSlotDualAssign {
                base_type,
                alt_type,
                consume_from_stack,
                tile_size,
                base_tiles,
                alt_tiles,
                stack_entry,
                ..
            } => {
                base_tiles.clear();
                alt_tiles.clear();
                resource.extract_tiles(base_tiles, *base_type);
                resource.extract_tiles(alt_tiles, *alt_type);
                *stack_entry = resource.get_stack_entry();
                if base_tiles.is_empty() || alt_tiles.is_empty() {
                    return Err(KunaError::lowlevel(
                        "Could not find matching resources for action: join_dual_class",
                    ));
                }
                let entries = resource.get_entry();
                *tile_size = entries[base_tiles[0]].get_size();
                if *tile_size != entries[alt_tiles[0]].get_size() {
                    return Err(KunaError::lowlevel(
                        "Storage class register sizes do not match for action: join_dual_class",
                    ));
                }
                if *consume_from_stack && stack_entry.is_none() {
                    return Err(KunaError::lowlevel(
                        "Cannot find matching stack resource for action: join_dual_class",
                    ));
                }
                Ok(())
            }
            // ConsumeExtra::initializeEntries (modelrules.cc:1411-1417).
            AssignAction::ConsumeExtra { resource_type, tiles, .. } => {
                tiles.clear();
                resource.extract_tiles(tiles, *resource_type);
                if tiles.is_empty() {
                    return Err(KunaError::lowlevel(
                        "Could not find matching resources for action: consume_extra",
                    ));
                }
                Ok(())
            }
            // ConsumeRemaining::initializeEntries (modelrules.cc:1472-1478).
            AssignAction::ConsumeRemaining { resource_type, tiles } => {
                tiles.clear();
                resource.extract_tiles(tiles, *resource_type);
                if tiles.is_empty() {
                    return Err(KunaError::lowlevel(
                        "Could not find matching resources for action: consume_remaining",
                    ));
                }
                Ok(())
            }
            // ExtraStack::initializeEntry (modelrules.cc:1516-1522).
            AssignAction::ExtraStack { stack_entry, .. } => {
                *stack_entry = resource.get_stack_entry();
                if stack_entry.is_none() {
                    return Err(KunaError::lowlevel(
                        "Cannot find matching <pentry> for action: extra_stack",
                    ));
                }
                Ok(())
            }
            // ConvertToPointer / ConsumeAs / HiddenReturnAssign / MultiMemberAssign
            // cache no entries.
            _ => Ok(()),
        }
    }

    /// Assign an address and other meta-data for a parameter or return storage
    /// (C++ `assignAddress`, per-subclass overrides).
    #[allow(clippy::too_many_arguments)]
    pub fn assign_address(
        &self,
        dt: &Rc<Datatype>,
        proto: &PrototypePieces,
        pos: int4,
        tlist: &dyn TypeFactory,
        status: &mut [int4],
        res: &mut ParameterPieces,
        resource: &ParamListStandard,
        manager: &AddrSpaceManager,
    ) -> KunaResult<AssignActionResponse> {
        match self {
            AssignAction::GotoStack { stack_entry } => {
                assign_goto_stack(*stack_entry, dt, status, res, resource, manager)
            }
            AssignAction::ConvertToPointer { space } => assign_convert_to_pointer(
                space, dt, proto, pos, tlist, status, res, resource, manager,
            ),
            AssignAction::MultiSlotAssign { .. } => {
                self.assign_multi_slot(dt, status, res, resource, manager)
            }
            AssignAction::MultiMemberAssign { .. } => {
                self.assign_multi_member(dt, status, res, resource, manager)
            }
            AssignAction::MultiSlotDualAssign { .. } => {
                self.assign_multi_slot_dual(dt, status, res, resource, manager)
            }
            AssignAction::ConsumeAs { resource_type } => {
                // ConsumeAs::assignAddress (modelrules.cc:1339-1343).
                resource.assign_address_fallback(*resource_type, dt, true, status, res, manager)
            }
            AssignAction::HiddenReturnAssign { ret_code } => {
                // HiddenReturnAssign::assignAddress (modelrules.cc:1380-1384).
                Ok(*ret_code) // Signal to assignMap to use TYPECLASS_HIDDENRET
            }
            AssignAction::ConsumeExtra { .. } => {
                self.assign_consume_extra(dt, status, resource)
            }
            AssignAction::ExtraStack { .. } => {
                self.assign_extra_stack(dt, status, res, resource, manager)
            }
            AssignAction::ConsumeRemaining { .. } => self.assign_consume_remaining(status, resource),
        }
    }

    /// `MultiSlotAssign::assignAddress` (modelrules.cc:833-900).
    fn assign_multi_slot(
        &self,
        dt: &Rc<Datatype>,
        status: &mut [int4],
        res: &mut ParameterPieces,
        resource: &ParamListStandard,
        manager: &AddrSpaceManager,
    ) -> KunaResult<AssignActionResponse> {
        let (resource_type, is_big_endian, consume_from_stack, consume_most_sig, enforce_alignment, justify_right, tiles, stack_entry) =
            match self {
                AssignAction::MultiSlotAssign {
                    resource_type,
                    is_big_endian,
                    consume_from_stack,
                    consume_most_sig,
                    enforce_alignment,
                    justify_right,
                    tiles,
                    stack_entry,
                } => (
                    *resource_type,
                    *is_big_endian,
                    *consume_from_stack,
                    *consume_most_sig,
                    *enforce_alignment,
                    *justify_right,
                    tiles,
                    *stack_entry,
                ),
                _ => unreachable!(),
            };
        let entries = resource.get_entry();
        let mut tmp_status: Vec<int4> = status.to_vec();
        let mut pieces: Vec<VarnodeData> = Vec::new();
        let mut size_left = dt.get_size();
        let mut align = dt.get_alignment();
        let mut iter = 0usize;
        if enforce_alignment {
            let mut resources_consumed = 0;
            while iter != tiles.len() {
                let entry = &entries[tiles[iter]];
                if tmp_status[entry.get_group() as usize] == 0 {
                    // Not consumed
                    let reg_size = entry.get_size();
                    if align <= reg_size || (resources_consumed % align) == 0 {
                        break;
                    }
                    tmp_status[entry.get_group() as usize] = -1; // Consume unaligned register
                }
                resources_consumed += entry.get_size();
                iter += 1;
            }
        }
        while size_left > 0 && iter != tiles.len() {
            let entry = &entries[tiles[iter]];
            iter += 1;
            if tmp_status[entry.get_group() as usize] != 0 {
                continue; // Already consumed
            }
            let trial_size = entry.get_size();
            let grp = entry.get_group() as usize;
            let addr = entry.get_addr_by_slot(&mut tmp_status[grp], trial_size, align, manager)?;
            tmp_status[grp] = -1; // Consume the register
            pieces.push(VarnodeData {
                space: addr.get_space().cloned(),
                offset: addr.get_offset(),
                size: trial_size as u32,
            });
            size_left -= trial_size;
            align = 1; // Treat remaining partial pieces as having no alignment requirement
        }
        if size_left > 0 {
            // Have to use stack to get enough bytes
            if !consume_from_stack {
                return Ok(RES_FAIL);
            }
            let se = stack_entry.expect("MultiSlotAssign stack entry");
            let grp = entries[se].get_group() as usize;
            let addr = entries[se].get_addr_by_slot_justify(
                &mut tmp_status[grp],
                size_left,
                align,
                justify_right,
                manager,
            )?; // Consume all the space we need
            if addr.is_invalid() {
                return Ok(RES_FAIL);
            }
            pieces.push(VarnodeData {
                space: addr.get_space().cloned(),
                offset: addr.get_offset(),
                size: size_left as u32,
            });
        } else if size_left < 0 {
            // Have odd data-type size
            if resource_type == type_class::TYPECLASS_FLOAT && pieces.len() == 1 {
                let tmp = &mut pieces[0];
                let addr = manager.construct_float_extension_address(
                    &tmp.get_addr(),
                    tmp.size as i32,
                    dt.get_size(),
                )?;
                tmp.space = addr.get_space().cloned();
                tmp.offset = addr.get_offset();
                tmp.size = dt.get_size() as u32;
            } else {
                justify_pieces(&mut pieces, -size_left, is_big_endian, consume_most_sig, justify_right);
            }
        }
        status.copy_from_slice(&tmp_status); // Commit resource usage for all the pieces
        res.flags = 0;
        res.type_ = Some(Rc::clone(dt));
        res.assign_address_from_pieces(&mut pieces, consume_most_sig, manager)?;
        Ok(RES_SUCCESS)
    }

    /// `MultiMemberAssign::assignAddress` (modelrules.cc:992-1017).
    fn assign_multi_member(
        &self,
        dt: &Rc<Datatype>,
        status: &mut [int4],
        res: &mut ParameterPieces,
        resource: &ParamListStandard,
        manager: &AddrSpaceManager,
    ) -> KunaResult<AssignActionResponse> {
        let (resource_type, consume_from_stack, consume_most_sig) = match self {
            AssignAction::MultiMemberAssign { resource_type, consume_from_stack, consume_most_sig } => {
                (*resource_type, *consume_from_stack, *consume_most_sig)
            }
            _ => unreachable!(),
        };
        let mut tmp_status: Vec<int4> = status.to_vec();
        let mut pieces: Vec<VarnodeData> = Vec::new();
        let primitives = PrimitiveExtractor::new(dt, false, 0, 16);
        if !primitives.is_valid()
            || primitives.size() == 0
            || primitives.contains_unknown()
            || !primitives.is_aligned()
            || primitives.contains_holes()
        {
            return Ok(RES_FAIL);
        }
        let mut param = ParameterPieces::default();
        for i in 0..primitives.size() {
            let cur_type = Rc::clone(&primitives.get(i).dt);
            if resource.assign_address_fallback(
                resource_type,
                &cur_type,
                !consume_from_stack,
                &mut tmp_status,
                &mut param,
                manager,
            )? == RES_FAIL
            {
                return Ok(RES_FAIL);
            }
            pieces.push(VarnodeData {
                space: param.addr.get_space().cloned(),
                offset: param.addr.get_offset(),
                size: cur_type.get_size() as u32,
            });
        }
        status.copy_from_slice(&tmp_status); // Commit resource usage for all the pieces
        res.flags = 0;
        res.type_ = Some(Rc::clone(dt));
        res.assign_address_from_pieces(&mut pieces, consume_most_sig, manager)?;
        Ok(RES_SUCCESS)
    }

    /// `MultiSlotDualAssign::assignAddress` (modelrules.cc:1174-1240).
    fn assign_multi_slot_dual(
        &self,
        dt: &Rc<Datatype>,
        status: &mut [int4],
        res: &mut ParameterPieces,
        resource: &ParamListStandard,
        manager: &AddrSpaceManager,
    ) -> KunaResult<AssignActionResponse> {
        let (is_big_endian, consume_from_stack, consume_most_sig, justify_right, base_tiles, alt_tiles, stack_entry) =
            match self {
                AssignAction::MultiSlotDualAssign {
                    is_big_endian,
                    consume_from_stack,
                    consume_most_sig,
                    justify_right,
                    base_tiles,
                    alt_tiles,
                    stack_entry,
                    ..
                } => (
                    *is_big_endian,
                    *consume_from_stack,
                    *consume_most_sig,
                    *justify_right,
                    base_tiles,
                    alt_tiles,
                    *stack_entry,
                ),
                _ => unreachable!(),
            };
        let entries = resource.get_entry();
        let primitives = PrimitiveExtractor::new(dt, false, 0, 1024);
        if !primitives.is_valid() || primitives.size() == 0 || primitives.contains_holes() {
            return Ok(RES_FAIL);
        }
        let mut primitive_index = 0;
        let mut tmp_status: Vec<int4> = status.to_vec();
        let mut pieces: Vec<VarnodeData> = Vec::new();
        let type_size = dt.get_size();
        let align = dt.get_alignment();
        let mut size_left = type_size;
        let mut iter_base = 0usize;
        let mut iter_alt = 0usize;
        while size_left > 0 {
            let iter_type = self.get_tile_class(&primitives, type_size - size_left, &mut primitive_index);
            if iter_type < 0 {
                return Ok(RES_FAIL);
            }
            let entry_idx;
            if iter_type == 0 {
                iter_base = get_first_unused(iter_base, base_tiles, &tmp_status, entries);
                if iter_base == base_tiles.len() {
                    if !consume_from_stack {
                        return Ok(RES_FAIL); // Out of general purpose registers
                    }
                    break;
                }
                entry_idx = base_tiles[iter_base];
            } else {
                iter_alt = get_first_unused(iter_alt, alt_tiles, &tmp_status, entries);
                if iter_alt == alt_tiles.len() {
                    if !consume_from_stack {
                        return Ok(RES_FAIL); // Out of alternate registers
                    }
                    break;
                }
                entry_idx = alt_tiles[iter_alt];
            }
            let entry = &entries[entry_idx];
            let trial_size = entry.get_size();
            let grp = entry.get_group() as usize;
            let addr = entry.get_addr_by_slot(&mut tmp_status[grp], trial_size, 1, manager)?;
            tmp_status[grp] = -1; // Consume the register
            pieces.push(VarnodeData {
                space: addr.get_space().cloned(),
                offset: addr.get_offset(),
                size: trial_size as u32,
            });
            size_left -= trial_size;
        }
        if size_left > 0 {
            if !consume_from_stack {
                return Ok(RES_FAIL);
            }
            let se = stack_entry.expect("MultiSlotDualAssign stack entry");
            let grp = entries[se].get_group() as usize;
            let addr = entries[se].get_addr_by_slot_justify(
                &mut tmp_status[grp],
                size_left,
                align,
                justify_right,
                manager,
            )?; // Consume all the space we need
            if addr.is_invalid() {
                return Ok(RES_FAIL);
            }
            pieces.push(VarnodeData {
                space: addr.get_space().cloned(),
                offset: addr.get_offset(),
                size: size_left as u32,
            });
        }
        if size_left < 0 {
            // Have odd data-type size
            justify_pieces(&mut pieces, -size_left, is_big_endian, consume_most_sig, justify_right);
        }
        status.copy_from_slice(&tmp_status); // Commit resource usage for all the pieces
        res.flags = 0;
        res.type_ = Some(Rc::clone(dt));
        res.assign_address_from_pieces(&mut pieces, consume_most_sig, manager)?;
        Ok(RES_SUCCESS)
    }

    /// `MultiSlotDualAssign::getTileClass` (modelrules.cc:1108-1135).
    fn get_tile_class(
        &self,
        primitives: &PrimitiveExtractor,
        off: int4,
        index: &mut int4,
    ) -> int4 {
        let (alt_type, fill_alternate, tile_size) = match self {
            AssignAction::MultiSlotDualAssign { alt_type, fill_alternate, tile_size, .. } => {
                (*alt_type, *fill_alternate, *tile_size)
            }
            _ => unreachable!(),
        };
        let mut res = 1;
        let mut count = 0;
        let end_boundary = off + tile_size;
        if *index >= primitives.size() {
            return -1;
        }
        let first_primitive_size = primitives.get(*index).dt.get_size();
        while *index < primitives.size() {
            let element = primitives.get(*index);
            if element.offset < off {
                return -1;
            }
            if element.offset >= end_boundary {
                break;
            }
            if element.offset + element.dt.get_size() > end_boundary {
                return -1;
            }
            count += 1;
            *index += 1;
            let storage = metatype2typeclass(element.dt.get_metatype());
            if storage != alt_type {
                res = 0;
            }
        }
        if count == 0 {
            return -1; // Must be at least one primitive in section
        }
        if fill_alternate {
            // Only use altType if the tile contains one primitive of exactly the tile size.
            if count > 1 {
                res = 0;
            }
            if first_primitive_size != tile_size {
                res = 0;
            }
        }
        res
    }

    /// `ConsumeExtra::assignAddress` (modelrules.cc:1434-1450).
    fn assign_consume_extra(
        &self,
        dt: &Rc<Datatype>,
        status: &mut [int4],
        resource: &ParamListStandard,
    ) -> KunaResult<AssignActionResponse> {
        let (match_size, tiles) = match self {
            AssignAction::ConsumeExtra { match_size, tiles, .. } => (*match_size, tiles),
            _ => unreachable!(),
        };
        let entries = resource.get_entry();
        let mut iter = 0usize;
        let mut size_left = dt.get_size();
        while size_left > 0 && iter != tiles.len() {
            let entry = &entries[tiles[iter]];
            iter += 1;
            if status[entry.get_group() as usize] != 0 {
                continue; // Already consumed
            }
            status[entry.get_group() as usize] = -1; // Consume the slot/register
            size_left -= entry.get_size();
            if !match_size {
                break; // Only consume a single register
            }
        }
        Ok(RES_SUCCESS)
    }

    /// `ConsumeRemaining::assignAddress` (modelrules.cc:1493-1505).
    fn assign_consume_remaining(
        &self,
        status: &mut [int4],
        resource: &ParamListStandard,
    ) -> KunaResult<AssignActionResponse> {
        let tiles = match self {
            AssignAction::ConsumeRemaining { tiles, .. } => tiles,
            _ => unreachable!(),
        };
        let entries = resource.get_entry();
        let mut iter = 0usize;
        while iter != tiles.len() {
            let entry = &entries[tiles[iter]];
            iter += 1;
            if status[entry.get_group() as usize] != 0 {
                continue; // Already consumed
            }
            status[entry.get_group() as usize] = -1; // Consume the slot/register
        }
        Ok(RES_SUCCESS)
    }

    /// `ExtraStack::assignAddress` (modelrules.cc:1542-1572).
    fn assign_extra_stack(
        &self,
        dt: &Rc<Datatype>,
        status: &mut [int4],
        res: &mut ParameterPieces,
        resource: &ParamListStandard,
        manager: &AddrSpaceManager,
    ) -> KunaResult<AssignActionResponse> {
        let (after_bytes, after_storage, stack_entry) = match self {
            AssignAction::ExtraStack { after_bytes, after_storage, stack_entry } => {
                (*after_bytes, *after_storage, *stack_entry)
            }
            _ => unreachable!(),
        };
        let entries = resource.get_entry();
        let se = stack_entry.expect("ExtraStack stack entry");
        let stack_space = entries[se].get_space();
        if res.addr.get_space().map(|s| Rc::ptr_eq(s, stack_space)).unwrap_or(false) {
            return Ok(RES_SUCCESS); // Parameter was already assigned to the stack
        }
        let grp = entries[se].get_group();
        // Check whether we have consumed enough storage to need to adjust the stack yet.
        if after_bytes > 0 {
            let mut bytes_consumed = 0;
            for entry in entries.iter() {
                if entry.get_group() == grp || entry.get_type() != after_storage {
                    continue;
                }
                if status[entry.get_group() as usize] != 0 {
                    bytes_consumed += entry.get_size();
                }
            }
            if bytes_consumed < after_bytes {
                return Ok(RES_SUCCESS);
            }
        }
        // We assign the stack address (but ignore the actual address) updating the status for
        // the stack, which consumes the stack resources.
        let mut slot = status[grp as usize];
        let _ = entries[se].get_addr_by_slot(&mut slot, dt.get_size(), dt.get_alignment(), manager)?;
        status[grp as usize] = slot;
        Ok(RES_SUCCESS)
    }

    /// Test if this action could produce return value storage matching the
    /// given set of trials (C++ `fillinOutputMap`, per-subclass overrides).
    /// The default (`AssignAction::fillinOutputMap`, modelrules.cc:579-583)
    /// returns `false`.
    pub fn fillin_output_map(&self, active: &mut ParamActive, resource: &ParamListStandard) -> bool {
        let entries = resource.get_entry();
        match self {
            // GotoStack::fillinOutputMap (modelrules.cc:731-746).
            AssignAction::GotoStack { stack_entry } => {
                let se = match stack_entry {
                    Some(s) => *s,
                    None => return false,
                };
                let mut count = 0;
                for i in 0..active.get_num_trials() {
                    let trial = active.get_trial(i);
                    let entry = match trial.get_entry() {
                        Some(e) => e,
                        None => break,
                    };
                    if entry != se {
                        return false;
                    }
                    count += 1;
                    if count > 1 {
                        return false;
                    }
                }
                count == 1
            }
            // MultiSlotAssign::fillinOutputMap (modelrules.cc:902-952).
            AssignAction::MultiSlotAssign {
                resource_type, consume_most_sig, justify_right, ..
            } => fillin_multi_slot(
                active,
                entries,
                *resource_type,
                *consume_most_sig,
                *justify_right,
            ),
            // MultiMemberAssign::fillinOutputMap (modelrules.cc:1019-1047).
            AssignAction::MultiMemberAssign { resource_type, consume_most_sig, .. } => {
                let mut count = 0;
                let mut cur_group = -1;
                for i in 0..active.get_num_trials() {
                    let trial = active.get_trial(i);
                    let entry_idx = match trial.get_entry() {
                        Some(e) => e,
                        None => break,
                    };
                    let entry = &entries[entry_idx];
                    if entry.get_type() != *resource_type {
                        return false; // Trials must come from action's type_class
                    }
                    if count == 0 {
                        if !entry.is_first_in_class() {
                            return false;
                        }
                    } else if entry.get_group() != cur_group + 1 {
                        return false; // Trials must be consecutive
                    }
                    cur_group = entry.get_group();
                    if trial.get_offset() != 0 {
                        return false; // Entry must be justified
                    }
                    count += 1;
                }
                if count == 0 {
                    return false;
                }
                if *consume_most_sig {
                    active.set_join_reverse();
                }
                true
            }
            // MultiSlotDualAssign::fillinOutputMap (modelrules.cc:1242-1298).
            AssignAction::MultiSlotDualAssign {
                base_type, alt_type, consume_most_sig, justify_right, ..
            } => fillin_multi_slot_dual(
                active,
                entries,
                *base_type,
                *alt_type,
                *consume_most_sig,
                *justify_right,
            ),
            // ConsumeAs::fillinOutputMap (modelrules.cc:1345-1364).
            AssignAction::ConsumeAs { resource_type } => {
                let mut count = 0;
                for i in 0..active.get_num_trials() {
                    let trial = active.get_trial(i);
                    let entry_idx = match trial.get_entry() {
                        Some(e) => e,
                        None => break,
                    };
                    let entry = &entries[entry_idx];
                    if entry.get_type() != *resource_type {
                        return false; // Trials must come from action's type_class
                    }
                    if !entry.is_first_in_class() {
                        return false;
                    }
                    count += 1;
                    if count > 1 {
                        return false;
                    }
                    if trial.get_offset() != 0 {
                        return false; // Entry must be justified
                    }
                }
                count > 0
            }
            // All other actions use the default (inactive) implementation.
            _ => false,
        }
    }

    // -- decode (per-subclass) ----------------------------------------------

    /// Read the next model rule action element from the stream (C++
    /// `AssignAction::decodeAction`, modelrules.cc:592-621).
    pub fn decode_action(
        decoder: &mut dyn Decoder,
        resource: &ParamListStandard,
    ) -> KunaResult<AssignAction> {
        let elem_id = decoder.peek_element()?;
        let mut action = if elem_id == ELEM_GOTO_STACK.get_id() {
            new_goto_stack_decode()
        } else if elem_id == ELEM_JOIN.get_id() {
            new_multi_slot_assign_default(resource)
        } else if elem_id == ELEM_CONSUME.get_id() {
            AssignAction::ConsumeAs { resource_type: type_class::TYPECLASS_GENERAL }
        } else if elem_id == ELEM_CONVERT_TO_PTR.get_id() {
            AssignAction::ConvertToPointer { space: resource.get_spacebase().cloned() }
        } else if elem_id == ELEM_HIDDEN_RETURN.get_id() {
            AssignAction::HiddenReturnAssign { ret_code: RES_HIDDENRET_SPECIALREG }
        } else if elem_id == ELEM_JOIN_PER_PRIMITIVE.get_id() {
            // MultiMemberAssign(TYPECLASS_GENERAL,false,res->isBigEndian(),res).
            AssignAction::MultiMemberAssign {
                resource_type: type_class::TYPECLASS_GENERAL,
                consume_from_stack: false,
                consume_most_sig: resource.is_big_endian(),
            }
        } else if elem_id == ELEM_JOIN_DUAL_CLASS.get_id() {
            new_multi_slot_dual_default(resource)
        } else {
            return Err(KunaError::decoder("Expecting model rule action"));
        };
        action.decode(decoder, resource)?;
        Ok(action)
    }

    /// Read the next model rule precondition element (C++
    /// `AssignAction::decodePrecondition`, modelrules.cc:630-644).  Returns
    /// `None` if the next element is not a precondition.
    pub fn decode_precondition(
        decoder: &mut dyn Decoder,
        resource: &ParamListStandard,
    ) -> KunaResult<Option<AssignAction>> {
        let elem_id = decoder.peek_element()?;
        let mut action = if elem_id == ELEM_CONSUME_EXTRA.get_id() {
            new_consume_extra_decode()
        } else {
            return Ok(None);
        };
        action.decode(decoder, resource)?;
        Ok(Some(action))
    }

    /// Read the next model rule sideeffect element (C++
    /// `AssignAction::decodeSideeffect`, modelrules.cc:653-672).
    pub fn decode_sideeffect(
        decoder: &mut dyn Decoder,
        resource: &ParamListStandard,
    ) -> KunaResult<AssignAction> {
        let elem_id = decoder.peek_element()?;
        let mut action = if elem_id == ELEM_CONSUME_EXTRA.get_id() {
            new_consume_extra_decode()
        } else if elem_id == ELEM_EXTRA_STACK.get_id() {
            new_extra_stack_decode()
        } else if elem_id == ELEM_CONSUME_REMAINING.get_id() {
            new_consume_remaining_decode()
        } else {
            return Err(KunaError::decoder("Expecting model rule sideeffect"));
        };
        action.decode(decoder, resource)?;
        Ok(action)
    }

    /// Configure any details of how this action should behave from the stream
    /// (C++ `decode`, per-subclass).  Runs the `initializeEntries` tail where
    /// the C++ does.
    fn decode(&mut self, decoder: &mut dyn Decoder, resource: &ParamListStandard) -> KunaResult<()> {
        match self {
            // GotoStack::decode (modelrules.cc:748-754).
            AssignAction::GotoStack { .. } => {
                let elem_id = decoder.open_element_id(&ELEM_GOTO_STACK)?;
                decoder.close_element(elem_id)?;
                self.initialize(resource)
            }
            // ConvertToPointer::decode (modelrules.cc:778-783).
            AssignAction::ConvertToPointer { .. } => {
                let elem_id = decoder.open_element_id(&ELEM_CONVERT_TO_PTR)?;
                decoder.close_element(elem_id)?;
                Ok(())
            }
            // MultiSlotAssign::decode (modelrules.cc:954-981).
            AssignAction::MultiSlotAssign {
                resource_type,
                consume_most_sig,
                enforce_alignment,
                justify_right,
                consume_from_stack,
                ..
            } => {
                let elem_id = decoder.open_element_id(&ELEM_JOIN)?;
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    }
                    if attrib_id == ATTRIB_REVERSEJUSTIFY {
                        if decoder.read_bool()? {
                            *justify_right = !*justify_right;
                        }
                    } else if attrib_id == ATTRIB_REVERSESIGNIF {
                        if decoder.read_bool()? {
                            *consume_most_sig = !*consume_most_sig;
                        }
                    } else if attrib_id == ATTRIB_STORAGE {
                        *resource_type = read_typeclass(decoder)?;
                    } else if attrib_id == ATTRIB_ALIGN {
                        *enforce_alignment = decoder.read_bool()?;
                    } else if attrib_id == ATTRIB_STACKSPILL {
                        *consume_from_stack = decoder.read_bool()?;
                    }
                }
                decoder.close_element(elem_id)?;
                self.initialize(resource) // Need new firstIter
            }
            // MultiMemberAssign::decode (modelrules.cc:1049-1061).
            AssignAction::MultiMemberAssign { resource_type, .. } => {
                let elem_id = decoder.open_element_id(&ELEM_JOIN_PER_PRIMITIVE)?;
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    }
                    if attrib_id == ATTRIB_STORAGE {
                        *resource_type = read_typeclass(decoder)?;
                    }
                }
                decoder.close_element(elem_id)?;
                Ok(())
            }
            // MultiSlotDualAssign::decode (modelrules.cc:1300-1330).
            AssignAction::MultiSlotDualAssign {
                base_type,
                alt_type,
                consume_most_sig,
                justify_right,
                consume_from_stack,
                fill_alternate,
                ..
            } => {
                let elem_id = decoder.open_element_id(&ELEM_JOIN_DUAL_CLASS)?;
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    }
                    if attrib_id == ATTRIB_REVERSEJUSTIFY {
                        if decoder.read_bool()? {
                            *justify_right = !*justify_right;
                        }
                    } else if attrib_id == ATTRIB_REVERSESIGNIF {
                        if decoder.read_bool()? {
                            *consume_most_sig = !*consume_most_sig;
                        }
                    } else if attrib_id == ATTRIB_STORAGE || attrib_id == ATTRIB_A {
                        *base_type = read_typeclass(decoder)?;
                    } else if attrib_id == ATTRIB_B {
                        *alt_type = read_typeclass(decoder)?;
                    } else if attrib_id == ATTRIB_STACKSPILL {
                        *consume_from_stack = decoder.read_bool()?;
                    } else if attrib_id == ATTRIB_FILL_ALTERNATE {
                        *fill_alternate = decoder.read_bool()?;
                    }
                }
                decoder.close_element(elem_id)?;
                self.initialize(resource) // Need new firstIter
            }
            // ConsumeAs::decode (modelrules.cc:1366-1372).
            AssignAction::ConsumeAs { resource_type } => {
                let elem_id = decoder.open_element_id(&ELEM_CONSUME)?;
                *resource_type = read_typeclass_id(decoder, &ATTRIB_STORAGE)?;
                decoder.close_element(elem_id)?;
                Ok(())
            }
            // HiddenReturnAssign::decode (modelrules.cc:1386-1408).
            AssignAction::HiddenReturnAssign { ret_code } => {
                *ret_code = RES_HIDDENRET_SPECIALREG;
                let elem_id = decoder.open_element_id(&ELEM_HIDDEN_RETURN)?;
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == ATTRIB_VOIDLOCK {
                        *ret_code = RES_HIDDENRET_SPECIALREG_VOID;
                    } else if attrib_id == ATTRIB_STRATEGY {
                        let strategy_string = decoder.read_string()?;
                        if strategy_string == b"normalparam" {
                            *ret_code = RES_HIDDENRET_PTRPARAM;
                        } else if strategy_string == b"special" {
                            *ret_code = RES_HIDDENRET_SPECIALREG;
                        } else {
                            let s = String::from_utf8_lossy(&strategy_string);
                            return Err(KunaError::decoder(format!(
                                "Bad <hidden_return> strategy: {s}"
                            )));
                        }
                    } else {
                        break;
                    }
                }
                decoder.close_element(elem_id)?;
                Ok(())
            }
            // ConsumeExtra::decode (modelrules.cc:1452-1468).
            AssignAction::ConsumeExtra { resource_type, match_size, .. } => {
                let elem_id = decoder.open_element_id(&ELEM_CONSUME_EXTRA)?;
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    } else if attrib_id == ATTRIB_STORAGE {
                        *resource_type = read_typeclass(decoder)?;
                    } else if attrib_id == ATTRIB_MATCHSIZE {
                        *match_size = decoder.read_bool()?;
                    }
                }
                decoder.close_element(elem_id)?;
                self.initialize(resource)
            }
            // ConsumeRemaining::decode (modelrules.cc:1507-1514).
            AssignAction::ConsumeRemaining { resource_type, .. } => {
                let elem_id = decoder.open_element_id(&ELEM_CONSUME_REMAINING)?;
                *resource_type = read_typeclass_id(decoder, &ATTRIB_STORAGE)?;
                decoder.close_element(elem_id)?;
                self.initialize(resource)
            }
            // ExtraStack::decode (modelrules.cc:1574-1588).
            AssignAction::ExtraStack { after_bytes, after_storage, .. } => {
                let elem_id = decoder.open_element_id(&ELEM_EXTRA_STACK)?;
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    } else if attrib_id == ATTRIB_AFTER_BYTES {
                        *after_bytes = decoder.read_unsigned_integer()? as int4; // cast: int4 member
                    } else if attrib_id == ATTRIB_AFTER_STORAGE {
                        *after_storage = read_typeclass(decoder)?;
                    }
                }
                decoder.close_element(elem_id)?;
                self.initialize(resource)
            }
        }
    }
}

// -- AssignAction default constructors ---------------------------------------

/// `GotoStack(res,0)` decode constructor (modelrules.cc:706-711): `stackEntry`
/// null; `fillinOutputActive = true`.
fn new_goto_stack_decode() -> AssignAction {
    AssignAction::GotoStack { stack_entry: None }
}

/// `MultiSlotAssign(res)` decode constructor (modelrules.cc:800-817).
fn new_multi_slot_assign_default(resource: &ParamListStandard) -> AssignAction {
    let is_big_endian = resource.is_big_endian();
    let list_type = resource.get_type();
    // Consume from stack on input parameters by default.
    let consume_from_stack =
        list_type != ParamListType::RegisterOut && list_type != ParamListType::StandardOut;
    let mut consume_most_sig = false;
    let mut justify_right = false;
    if is_big_endian {
        consume_most_sig = true;
        justify_right = true;
    }
    AssignAction::MultiSlotAssign {
        resource_type: type_class::TYPECLASS_GENERAL, // Join general purpose registers
        is_big_endian,
        consume_from_stack,
        consume_most_sig,
        enforce_alignment: false,
        justify_right,
        tiles: Vec::new(),
        stack_entry: None,
    }
}

/// `MultiSlotDualAssign(res)` decode constructor (modelrules.cc:1139-1156).
fn new_multi_slot_dual_default(resource: &ParamListStandard) -> AssignAction {
    let is_big_endian = resource.is_big_endian();
    let mut consume_most_sig = false;
    let mut justify_right = false;
    if is_big_endian {
        consume_most_sig = true;
        justify_right = true;
    }
    AssignAction::MultiSlotDualAssign {
        base_type: type_class::TYPECLASS_GENERAL, // Tile from general purpose registers
        alt_type: type_class::TYPECLASS_FLOAT, // Specialized registers for floating-point
        is_big_endian,
        consume_from_stack: false,
        consume_most_sig,
        justify_right,
        fill_alternate: false,
        tile_size: 0,
        base_tiles: Vec::new(),
        alt_tiles: Vec::new(),
        stack_entry: None,
    }
}

/// `ConsumeExtra(res)` decode constructor (modelrules.cc:1419-1424):
/// `resourceType = GENERAL; matchSize = true`.
fn new_consume_extra_decode() -> AssignAction {
    AssignAction::ConsumeExtra {
        resource_type: type_class::TYPECLASS_GENERAL,
        match_size: true,
        tiles: Vec::new(),
    }
}

/// `ConsumeRemaining(res)` decode constructor (modelrules.cc:1480-1484):
/// `resourceType = GENERAL`.
fn new_consume_remaining_decode() -> AssignAction {
    AssignAction::ConsumeRemaining {
        resource_type: type_class::TYPECLASS_GENERAL,
        tiles: Vec::new(),
    }
}

/// `ExtraStack(res)` decode constructor (modelrules.cc:1525-1531):
/// `afterBytes = -1; afterStorage = GENERAL`.
fn new_extra_stack_decode() -> AssignAction {
    AssignAction::ExtraStack {
        after_bytes: -1,
        after_storage: type_class::TYPECLASS_GENERAL,
        stack_entry: None,
    }
}

// -- AssignAction free helpers -----------------------------------------------

/// `GotoStack::assignAddress` (modelrules.cc:721-729).
fn assign_goto_stack(
    stack_entry: Option<usize>,
    dt: &Rc<Datatype>,
    status: &mut [int4],
    res: &mut ParameterPieces,
    resource: &ParamListStandard,
    manager: &AddrSpaceManager,
) -> KunaResult<AssignActionResponse> {
    let se = stack_entry.expect("GotoStack stack entry");
    let entry = &resource.get_entry()[se];
    let grp = entry.get_group() as usize;
    res.type_ = Some(Rc::clone(dt));
    let mut slot = status[grp];
    res.addr = entry.get_addr_by_slot(&mut slot, dt.get_size(), dt.get_alignment(), manager)?;
    status[grp] = slot;
    res.flags = 0;
    Ok(RES_SUCCESS)
}

/// `ConvertToPointer::assignAddress` (modelrules.cc:762-776).
#[allow(clippy::too_many_arguments)]
fn assign_convert_to_pointer(
    space: &Option<Rc<AddrSpace>>,
    dt: &Rc<Datatype>,
    proto: &PrototypePieces,
    pos: int4,
    tlist: &dyn TypeFactory,
    status: &mut [int4],
    res: &mut ParameterPieces,
    resource: &ParamListStandard,
    manager: &AddrSpaceManager,
) -> KunaResult<AssignActionResponse> {
    let _ = (proto, pos);
    let spc = match space {
        Some(s) => Rc::clone(s),
        None => manager
            .get_default_data_space()
            .ok_or_else(|| KunaError::lowlevel("No default data space"))?
            .clone(),
    };
    let pointersize = spc.get_addr_size() as int4;
    let wordsize = spc.get_word_size();
    // Convert the data-type to a pointer.
    let pointertp = tlist.get_type_pointer(pointersize, Rc::clone(dt), wordsize)?;
    // (Recursively) assign storage.
    let response_code =
        resource.assign_address(&pointertp, proto, pos, tlist, status, res, manager)?;
    res.flags = parameter_pieces_flags::INDIRECTSTORAGE;
    Ok(response_code)
}

/// `MultiSlotDualAssign::getFirstUnused` (modelrules.cc:1086-1095).
fn get_first_unused(
    mut iter: usize,
    tiles: &[usize],
    status: &[int4],
    entries: &[ParamEntry],
) -> usize {
    while iter != tiles.len() {
        let entry = &entries[tiles[iter]];
        if status[entry.get_group() as usize] != 0 {
            iter += 1;
            continue; // Already consumed
        }
        return iter;
    }
    tiles.len()
}

/// `MultiSlotAssign::fillinOutputMap` body (modelrules.cc:902-952).
fn fillin_multi_slot(
    active: &mut ParamActive,
    entries: &[ParamEntry],
    resource_type: type_class,
    consume_most_sig: bool,
    justify_right: bool,
) -> bool {
    let mut count = 0;
    let mut cur_group = -1;
    let mut partial: int4 = -1;
    for i in 0..active.get_num_trials() {
        let trial = active.get_trial(i);
        let entry_idx = match trial.get_entry() {
            Some(e) => e,
            None => break,
        };
        let entry = &entries[entry_idx];
        if entry.get_type() != resource_type {
            return false; // Trials must come from action's type_class
        }
        if count == 0 {
            if !entry.is_first_in_class() {
                return false; // Trials must start on first entry of the type_class
            }
        } else if entry.get_group() != cur_group + 1 {
            return false; // Trials must be consecutive
        }
        cur_group = entry.get_group();
        if trial.get_size() != entry.get_size() {
            if partial != -1 {
                return false; // At most, one trial can be partial size
            }
            partial = i;
        }
        count += 1;
    }
    if partial != -1 {
        if justify_right {
            if partial != 0 {
                return false;
            }
        } else if partial != count - 1 {
            return false;
        }
        let trial = active.get_trial(partial);
        let entry = &entries[trial.get_entry().expect("partial trial entry")];
        if justify_right == consume_most_sig {
            if trial.get_offset() != 0 {
                return false; // Partial entry must be least sig bytes
            }
        } else if trial.get_offset() + trial.get_size() != entry.get_size() {
            return false; // Partial entry must be most sig bytes
        }
    }
    if count == 0 {
        return false;
    }
    if consume_most_sig {
        active.set_join_reverse();
    }
    true
}

/// `MultiSlotDualAssign::fillinOutputMap` body (modelrules.cc:1242-1298).
fn fillin_multi_slot_dual(
    active: &mut ParamActive,
    entries: &[ParamEntry],
    base_type: type_class,
    alt_type: type_class,
    consume_most_sig: bool,
    justify_right: bool,
) -> bool {
    let mut count = 0;
    let mut cur_group = -1;
    let mut partial: int4 = -1;
    let mut resource_type = type_class::TYPECLASS_GENERAL;
    for i in 0..active.get_num_trials() {
        let trial = active.get_trial(i);
        let entry_idx = match trial.get_entry() {
            Some(e) => e,
            None => break,
        };
        let entry = &entries[entry_idx];
        if count == 0 {
            resource_type = entry.get_type();
            if resource_type != base_type && resource_type != alt_type {
                return false;
            }
        } else if entry.get_type() != resource_type {
            return false; // Trials must come from action's type_class
        }
        if count == 0 {
            if !entry.is_first_in_class() {
                return false; // Trials must start on first entry of the type_class
            }
        } else if entry.get_group() != cur_group + 1 {
            return false; // Trials must be consecutive
        }
        cur_group = entry.get_group();
        if trial.get_size() != entry.get_size() {
            if partial != -1 {
                return false; // At most, one trial can be partial size
            }
            partial = i;
        }
        count += 1;
    }
    if partial != -1 {
        if justify_right {
            if partial != 0 {
                return false;
            }
        } else if partial != count - 1 {
            return false;
        }
        let trial = active.get_trial(partial);
        let entry = &entries[trial.get_entry().expect("partial trial entry")];
        if justify_right == consume_most_sig {
            if trial.get_offset() != 0 {
                return false; // Partial entry must be least sig bytes
            }
        } else if trial.get_offset() + trial.get_size() != entry.get_size() {
            return false; // Partial entry must be most sig bytes
        }
    }
    if count == 0 {
        return false;
    }
    if consume_most_sig {
        active.set_join_reverse();
    }
    true
}

/// Truncate a tiling by a given number of bytes (C++ `AssignAction::justifyPieces`,
/// modelrules.cc:683-694).
fn justify_pieces(
    pieces: &mut [VarnodeData],
    offset: int4,
    is_big_endian: bool,
    consume_most_sig: bool,
    justify_right: bool,
) {
    let add_offset = is_big_endian ^ consume_most_sig ^ justify_right;
    let pos = if justify_right { 0 } else { pieces.len() - 1 };
    let vndata = &mut pieces[pos];
    if add_offset {
        vndata.offset += offset as u64; // C++ `vndata.offset += offset` (uintb += int4)
    }
    vndata.size -= offset as u32; // C++ `vndata.size -= offset` (uint4 -= int4)
}

/// Read a `type_class` from the current attribute (C++
/// `string2typeclass(decoder.readString())`).
fn read_typeclass(decoder: &mut dyn Decoder) -> KunaResult<type_class> {
    let s = decoder.read_string()?;
    let s = std::str::from_utf8(&s).map_err(|_| KunaError::decoder("Non-UTF8 storage class"))?;
    string2typeclass(s)
}

/// Read a `type_class` from a named attribute (C++
/// `string2typeclass(decoder.readString(ATTRIB_STORAGE))`).
fn read_typeclass_id(decoder: &mut dyn Decoder, attrib: &AttributeId) -> KunaResult<type_class> {
    let s = decoder.read_string_id(attrib)?;
    let s = std::str::from_utf8(&s).map_err(|_| KunaError::decoder("Non-UTF8 storage class"))?;
    string2typeclass(s)
}

/// Parse a comma/space separated list of positive decimal integers (C++
/// `SizeRestrictedFilter::initFromSizeList`'s `istringstream` loop,
/// modelrules.cc:280-294).  Errors on a non-positive value ("Bad filter size").
fn parse_size_list(str: &[u8]) -> KunaResult<Vec<int4>> {
    let mut out = Vec::new();
    let mut i = 0usize;
    let n = str.len();
    loop {
        // s >> ws
        while i < n && (str[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= n {
            break; // s.eof()
        }
        // if (s.peek() == ',') { s >> punc >> ws; }
        if str[i] == b',' {
            i += 1;
            while i < n && (str[i] as char).is_whitespace() {
                i += 1;
            }
        }
        // s >> val  (C++ extracts a decimal integer; val stays -1 on failure)
        let start = i;
        if i < n && (str[i] == b'+' || str[i] == b'-') {
            i += 1;
        }
        let digit_start = i;
        while i < n && str[i].is_ascii_digit() {
            i += 1;
        }
        let val: int4 = if i > digit_start {
            std::str::from_utf8(&str[start..i])
                .ok()
                .and_then(|t| t.parse::<i64>().ok())
                .map(|v| v as int4)
                .unwrap_or(-1)
        } else {
            -1 // failed extraction leaves val == -1
        };
        if val <= 0 {
            return Err(KunaError::decoder("Bad filter size"));
        }
        out.push(val);
        if i == start {
            // No progress (e.g. stray non-digit punctuation) — avoid an infinite
            // loop; the C++ stream would fail the extraction and set val=-1,
            // which we already handled above, but guard regardless.
            break;
        }
    }
    Ok(out)
}

// =============================================================================
// ModelRule (modelrules.hh:531-570, modelrules.cc:1590-1709)
// =============================================================================

/// A rule controlling how parameters are assigned addresses (C++ `ModelRule`,
/// `modelrules.hh:531-570`).
#[derive(Debug, Clone)]
pub struct ModelRule {
    /// Which data-types this rule applies to (C++ `filter`).
    filter: DatatypeFilter,
    /// Additional qualifiers for when the rule should apply (C++ `qualifier`).
    qualifier: Option<QualifierFilter>,
    /// How the Address should be assigned (C++ `assign`).
    assign: AssignAction,
    /// Extra actions that happen before assignment, discarded on failure (C++
    /// `preconditions`).
    preconditions: Vec<AssignAction>,
    /// Extra actions that happen on success (C++ `sideeffects`).
    sideeffects: Vec<AssignAction>,
}

impl ModelRule {
    /// Construct from components (C++
    /// `ModelRule(const DatatypeFilter&,const AssignAction&,const ParamListStandard*)`,
    /// modelrules.cc:1615-1621).  The C++ clones the filter/action; the kuna
    /// port takes them by value (the caller's copy is moved in).
    pub fn from_components(type_filter: DatatypeFilter, action: AssignAction) -> ModelRule {
        ModelRule {
            filter: type_filter,
            qualifier: None,
            assign: action,
            preconditions: Vec::new(),
            sideeffects: Vec::new(),
        }
    }

    /// Test and mark the trial(s) that can be a valid return value (C++
    /// `ModelRule::fillinOutputMap`, the inline at modelrules.hh:559-563).
    pub fn fillin_output_map(&self, active: &mut ParamActive, resource: &ParamListStandard) -> bool {
        self.assign.fillin_output_map(active, resource)
    }

    /// Return `true` if `fillinOutputMap` is active for this rule (C++
    /// `ModelRule::canAffectFillinOutput`, the inline at modelrules.hh:566-570).
    pub fn can_affect_fillin_output(&self) -> bool {
        self.assign.can_affect_fillin_output()
    }

    /// Assign an address and other details for a parameter or return storage
    /// (C++ `ModelRule::assignAddress`, modelrules.cc:1651-1672).
    #[allow(clippy::too_many_arguments)]
    pub fn assign_address(
        &self,
        dt: &Rc<Datatype>,
        proto: &PrototypePieces,
        pos: int4,
        tlist: &dyn TypeFactory,
        status: &mut [int4],
        res: &mut ParameterPieces,
        resource: &ParamListStandard,
        manager: &AddrSpaceManager,
    ) -> KunaResult<AssignActionResponse> {
        if !self.filter.filter(dt) {
            return Ok(RES_FAIL);
        }
        if let Some(qual) = &self.qualifier {
            if !qual.filter(proto, pos) {
                return Ok(RES_FAIL);
            }
        }
        let mut tmp_status: Vec<int4> = status.to_vec();
        for precond in self.preconditions.iter() {
            precond.assign_address(dt, proto, pos, tlist, &mut tmp_status, res, resource, manager)?;
        }
        let response =
            self.assign.assign_address(dt, proto, pos, tlist, &mut tmp_status, res, resource, manager)?;
        if response != RES_FAIL {
            status.copy_from_slice(&tmp_status);
            for sideeffect in self.sideeffects.iter() {
                sideeffect.assign_address(dt, proto, pos, tlist, status, res, resource, manager)?;
            }
        }
        Ok(response)
    }

    /// Decode this rule from the stream (C++ `ModelRule::decode`,
    /// modelrules.cc:1676-1709).
    pub fn decode(decoder: &mut dyn Decoder, resource: &ParamListStandard) -> KunaResult<ModelRule> {
        let mut qualifiers: Vec<QualifierFilter> = Vec::new();
        let elem_id = decoder.open_element_id(&ELEM_RULE)?;
        let filter = DatatypeFilter::decode_filter(decoder)?;
        loop {
            match QualifierFilter::decode_filter(decoder)? {
                None => break,
                Some(qual) => qualifiers.push(qual),
            }
        }
        let qualifier = if qualifiers.is_empty() {
            None
        } else if qualifiers.len() == 1 {
            Some(qualifiers.remove(0))
        } else {
            Some(QualifierFilter::And(qualifiers))
        };
        let mut preconditions: Vec<AssignAction> = Vec::new();
        loop {
            match AssignAction::decode_precondition(decoder, resource)? {
                None => break,
                Some(precond) => preconditions.push(precond),
            }
        }
        let assign = AssignAction::decode_action(decoder, resource)?;
        let mut sideeffects: Vec<AssignAction> = Vec::new();
        while decoder.peek_element()? != 0 {
            sideeffects.push(AssignAction::decode_sideeffect(decoder, resource)?);
        }
        decoder.close_element(elem_id)?;
        Ok(ModelRule { filter, qualifier, assign, preconditions, sideeffects })
    }
}

#[cfg(test)]
mod tests;
