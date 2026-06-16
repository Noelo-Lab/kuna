//! Port of `decompiler/cpp/unionresolve.{cc,hh}` — union-field resolution
//! (item `w6-s5-unionresolve`).
//!
//! A Varnode whose data-type is a union, a pointer to union, or a part of a
//! union can be accessed many ways; each read/write may touch a specific field
//! or the union as a whole.  [`ScoreUnionFields`] scores every candidate field
//! against the surrounding data-flow and reports the best-fitting field as a
//! [`ResolvedUnion`].
//!
//! # What this module ports (real)
//!
//!   * [`ResolvedUnion`] — the resolved-data-type record (all three C++
//!     constructors, [`ResolvedUnion::update`], the accessors).  This is a pure
//!     value type over `Rc<Datatype>` plus the [`TypeFactory`] handle, and is
//!     ported faithfully.
//!   * [`ResolveEdge`] — the data-flow-edge key, **including its load-bearing
//!     total order** (`operator<`, `unionresolve.hh:212-220`): order by
//!     `typeId`, then `encoding`, then `opTime`.  The ordering keys the C++
//!     `unionMap` (`map<ResolveEdge,ResolvedUnion>`); a wrong tie-break corrupts
//!     resolution lookup, so it is transcribed field-by-field as [`Ord`].
//!   * The scoring constants and the **facet scoring matrices** — the per-opcode
//!     metatype→score tables from `scoreTrialDown`/`scoreTrialUp`
//!     (`unionresolve.cc:355-921`), plus [`score_locked_type`],
//!     [`score_truncation`], [`score_constant_fit`], [`deref_pointer`],
//!     [`score_parameter`]/[`score_return_type`], and [`compute_best_index`].
//!     These encode the iteration order / tie-breakers that determine which
//!     field wins, transcribed verbatim.
//!
//! # What is seamed (SEAM(W4)/SEAM(W6)/SEAM(W7)) — see module `losses`
//!
//! The *driver* `ScoreUnionFields::run`/`runOneLevel`/`newTrials`/`newTrialsDown`
//! and the three `ScoreUnionFields` constructors walk the live `Funcdata`
//! data-flow graph (`Varnode::beginDescend`/`getDef`/`getTypeReadFacing`/
//! `getTypeDefFacing`/`isTypeLock`, `PcodeOp::getSlot`, `Funcdata::getCallSpecs`/
//! `getFuncProto`).  Several of those surfaces are unported seams:
//!
//!   * `Funcdata::getCallSpecs` is a `// SEAM(W4)` (no `FuncCallSpecs`/`FuncProto`
//!     yet — `seams::FuncProto` is a placeholder), so the locked-parameter and
//!     locked-return branches of [`score_parameter`]/[`score_return_type`] take
//!     a resolved [`LockedProtoFacts`] input instead of querying a live proto.
//!   * `Varnode::getTypeReadFacing`/`getTypeDefFacing` are `// SEAM(W6)` (the
//!     facing-type machinery is not ported), so the locked-Varnode short-circuit
//!     of `newTrials`/`newTrialsDown` is not wired here.
//!   * `Funcdata::unionMap` / `getUnionResolution` / `setUnionField` /
//!     `Varnode::updateType` are `// SEAM(W6)`/`// SEAM(W7)`; [`ResolveCache`]
//!     (which is *entirely* those surfaces) is therefore left as documented
//!     stubs that return the unresolved data-type (the C++ "no entry" behavior),
//!     and the `Funcdata` `unionMap` wiring is reported partial.
//!   * `TypeOpSubpiece::computeByteOffsetForComposite` (the SUBPIECE byte offset)
//!     is a `typeop.cc` surface not yet ported; the SUBPIECE arm of
//!     [`score_trial_down`] takes the offset as a parameter.
//!   * `glb->translate->getFloatFormat` and `glb->getDefaultDataSpace` (the
//!     constant-fit float class + pointer-bounds check) are `// SEAM(W4)`, so
//!     [`score_constant_fit`] takes a resolved [`ConstantFitFacts`] input.
//!
//! The scoring *decisions* (every branch of every opcode arm) are ported in full
//! and exercised by the synthetic-union scoring-matrix tests; W4/W6/W7 fill the
//! graph-walk driver in place against the frozen scoring functions here.

use std::rc::Rc;

use kuna_base::address::{Address, bit_transitions};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, int8, uint8, uintb, uintm};
use kuna_num::opcodes::OpCode;

use crate::dtype::{Datatype, TypeFactory, type_metatype};

// =============================================================================
// ResolvedUnion (unionresolve.hh:39-56, unionresolve.cc:25-98)
// =============================================================================

/// A data-type \e resolved from an associated `TypeUnion` or `TypeStruct`
/// (C++ `ResolvedUnion`, `unionresolve.hh:39`).
///
/// A \b parent refers to either (1) a union, (2) a structure that is an
/// effective union, or (3) a pointer to a union/structure.  The resolution can
/// be a specific field of the parent, a pointer to a specific field, or the
/// parent itself; [`field_num`](ResolvedUnion::get_field_num) of `-1` means the
/// parent itself is the resolution.
#[derive(Debug, Clone)]
pub struct ResolvedUnion {
    /// The resolved data-type (C++ `resolve`).
    resolve: Rc<Datatype>,
    /// Union or Structure being resolved (pointers and partials stripped)
    /// (C++ `baseType`).
    base_type: Rc<Datatype>,
    /// Index of field referenced by `resolve` (C++ `fieldNum`).
    field_num: int4,
    /// If `true`, resolution cannot be overridden (C++ `lock`).
    lock: bool,
}

impl ResolvedUnion {
    /// Construct a data-type that resolves to itself (C++
    /// `ResolvedUnion::ResolvedUnion(Datatype*)`, `unionresolve.cc:25-36`).
    ///
    /// The original data-type must be a union, a partial union, a structure with
    /// a single field, an array with a single element, or a pointer to one of
    /// these.  The object is set up to resolve to the parent.
    pub fn new(unres_type: Rc<Datatype>) -> ResolvedUnion {
        // baseType = unresType;
        // if (baseType->getMetatype() == TYPE_PTR) baseType = ((TypePointer*)baseType)->getPtrTo();
        // else if (baseType->getMetatype() == TYPE_PARTIALUNION)
        //   baseType = ((TypePartialUnion*)baseType)->getParentUnion();
        let base_type = match unres_type.get_metatype() {
            type_metatype::TYPE_PTR => unres_type
                .get_ptr_to()
                .unwrap_or_else(|| Rc::clone(&unres_type)),
            type_metatype::TYPE_PARTIALUNION => unres_type
                .get_partial_base()
                .unwrap_or_else(|| Rc::clone(&unres_type)),
            _ => Rc::clone(&unres_type),
        };
        ResolvedUnion { resolve: Rc::clone(&unres_type), base_type, field_num: -1, lock: false }
    }

    /// Construct a resolution with a specific field number (C++
    /// `ResolvedUnion::ResolvedUnion(Datatype*,int4,TypeFactory&)`,
    /// `unionresolve.cc:41-75`).
    ///
    /// \param unres_type is the data-type that needs resolution
    /// \param fld_num is the index of the field to resolve to (or -1 to resolve
    /// to `unres_type`)
    /// \param typegrp constructs the resolved data-type of the field
    pub fn new_field(
        unres_type: Rc<Datatype>,
        fld_num: int4,
        typegrp: &dyn TypeFactory,
    ) -> KunaResult<ResolvedUnion> {
        let mut base_type = Rc::clone(&unres_type);
        let field_num = fld_num;
        let lock = false;
        let resolve: Rc<Datatype>;
        match unres_type.get_metatype() {
            type_metatype::TYPE_PARTIALUNION => {
                // TypePartialUnion *partial = (TypePartialUnion *)unresType;
                // baseType = partial->getParentUnion();
                base_type = unres_type.get_partial_base().ok_or_else(|| {
                    KunaError::lowlevel("ResolvedUnion: partial union missing parent")
                })?;
                if fld_num < 0 {
                    // resolve = unresType->getStripped();
                    resolve = unres_type.get_stripped().ok_or_else(|| {
                        KunaError::lowlevel("ResolvedUnion: partial union missing stripped")
                    })?;
                } else {
                    // const TypeField *field = partial->getParentUnion()->getField(fldNum);
                    let field = base_type.get_field(fld_num).ok_or_else(|| {
                        KunaError::lowlevel("ResolvedUnion: parent union field out of range")
                    })?;
                    let field_type = Rc::clone(&field.field_type);
                    let offset = unres_type.get_partial_offset().ok_or_else(|| {
                        KunaError::lowlevel("ResolvedUnion: partial union missing offset")
                    })?;
                    let size = unres_type.get_size();
                    // resolve = typegrp.getExactPiece(field->type, partial->getOffset(), partial->getSize());
                    // if (resolve == 0) resolve = partial->getStripped();
                    match typegrp.get_exact_piece(field_type, offset, size)? {
                        Some(piece) => resolve = piece,
                        None => {
                            resolve = unres_type.get_stripped().ok_or_else(|| {
                                KunaError::lowlevel(
                                    "ResolvedUnion: partial union missing stripped",
                                )
                            })?;
                        }
                    }
                }
            }
            type_metatype::TYPE_PTR => {
                // TypePointer *pointer = (TypePointer *)unresType;
                // baseType = pointer->getPtrTo();
                base_type = unres_type.get_ptr_to().ok_or_else(|| {
                    KunaError::lowlevel("ResolvedUnion: pointer missing ptrto")
                })?;
                if fld_num < 0 {
                    resolve = Rc::clone(&unres_type);
                } else {
                    // Datatype *field = pointer->getPtrTo()->getDepend(fldNum);
                    let field = base_type.get_depend(fld_num).ok_or_else(|| {
                        KunaError::lowlevel("ResolvedUnion: pointer ptrto field out of range")
                    })?;
                    // resolve = typegrp.getTypePointerStripArray(unresType->getSize(),field,pointer->getWordSize());
                    let wordsize = unres_type.get_word_size().ok_or_else(|| {
                        KunaError::lowlevel("ResolvedUnion: pointer missing wordsize")
                    })?;
                    resolve =
                        typegrp.get_type_pointer_strip_array(unres_type.get_size(), field, wordsize)?;
                }
            }
            _ => {
                if fld_num < 0 {
                    resolve = Rc::clone(&unres_type);
                } else {
                    // resolve = unresType->getDepend(fldNum);
                    resolve = unres_type.get_depend(fld_num).ok_or_else(|| {
                        KunaError::lowlevel("ResolvedUnion: field out of range")
                    })?;
                }
            }
        }
        Ok(ResolvedUnion { resolve, base_type, field_num, lock })
    }

    /// Copy constructor with updated resolve data-type (C++
    /// `ResolvedUnion::ResolvedUnion(const ResolvedUnion&,Datatype*)`,
    /// `unionresolve.cc:77-84`).
    pub fn new_with_resolve(op: &ResolvedUnion, res: Rc<Datatype>) -> ResolvedUnion {
        ResolvedUnion {
            base_type: Rc::clone(&op.base_type),
            field_num: op.field_num,
            lock: op.lock,
            resolve: res,
        }
    }

    /// Get the resolved data-type (C++ `getDatatype`).
    #[inline]
    pub fn get_datatype(&self) -> &Rc<Datatype> {
        &self.resolve
    }

    /// Get the union or structure being referenced (C++ `getBase`).
    #[inline]
    pub fn get_base(&self) -> &Rc<Datatype> {
        &self.base_type
    }

    /// Get the index of the resolved field or -1 (C++ `getFieldNum`).
    #[inline]
    pub fn get_field_num(&self) -> int4 {
        self.field_num
    }

    /// Return `true` if the field resolution is locked (C++ `isLocked`).
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.lock
    }

    /// Update a resolution with a new data-type (C++ `ResolvedUnion::update`,
    /// `unionresolve.cc:88-98`).
    ///
    /// \return `true` if any change is made.
    pub fn update(&mut self, op: &ResolvedUnion) -> bool {
        // if (lock && fieldNum != op.fieldNum) return false;
        if self.lock && self.field_num != op.field_num {
            return false; // Lock protects the fieldNum resolution not the resolved data-type
        }
        // if (fieldNum == op.fieldNum && resolve == op.resolve) return false;
        if self.field_num == op.field_num && Rc::ptr_eq(&self.resolve, &op.resolve) {
            return false;
        }
        self.field_num = op.field_num;
        self.resolve = Rc::clone(&op.resolve);
        true
    }

    /// Update the resolution data-type without changing field resolution (C++
    /// `setResolve`).
    #[inline]
    pub fn set_resolve(&mut self, res: Rc<Datatype>) {
        self.resolve = res;
    }

    /// Set the resolved field index (C++ `result.fieldNum = ...`, written
    /// directly by the friend `ScoreUnionFields::computeBestIndex`,
    /// `unionresolve.cc:1061`).  The driver in [`crate::unionresolve_run`] cannot
    /// reach the private field, so this is the faithful equivalent of the C++
    /// friend write.
    #[inline]
    pub fn set_field_num(&mut self, field_num: int4) {
        self.field_num = field_num;
    }

    /// Set whether this resolution is locked against overrides (C++ `setLock`).
    #[inline]
    pub fn set_lock(&mut self, val: bool) {
        self.lock = val;
    }
}

// =============================================================================
// ResolveEdge (unionresolve.hh:63-71,212-220, unionresolve.cc:103-129)
// =============================================================================

/// A data-flow edge to which a resolved data-type can be assigned (C++
/// `ResolveEdge`, `unionresolve.hh:63`).
///
/// The edge is associated with the union (or pointer to union) being resolved.
/// The edge collapses different kinds of pointers to the same base union.  The
/// ordering ([`Ord`]) keys the C++ `unionMap`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolveEdge {
    /// Id of base data-type being resolved (C++ `typeId`).
    type_id: uint8,
    /// Id of PcodeOp edge (C++ `opTime`).
    op_time: uintm,
    /// Encoding of the slot and pointer-ness (C++ `encoding`).
    encoding: int4,
}

impl ResolveEdge {
    /// Construct from a PcodeOp edge (C++
    /// `ResolveEdge::ResolveEdge(const Datatype*,const PcodeOp*,int4)`,
    /// `unionresolve.cc:103-116`).
    ///
    /// \param unres_type is the data-type that needs resolution
    /// \param op_time is `op->getTime()` (the PcodeOp's unique id)
    /// \param slot is the slot (>=0 for input, -1 for output)
    pub fn new_op(unres_type: &Datatype, op_time: uintm, slot: int4) -> ResolveEdge {
        // opTime = op->getTime();
        // encoding = slot;
        let mut encoding = slot;
        let type_id = match unres_type.get_metatype() {
            type_metatype::TYPE_PTR => {
                // typeId = ((TypePointer*)unresType)->getPtrTo()->getId();  // Strip pointer
                // encoding += 0x1000;  // Encode the fact that a pointer is getting accessed
                encoding += 0x1000;
                unres_type
                    .get_ptr_to()
                    .map(|p| p.get_id())
                    .unwrap_or_else(|| unres_type.get_id())
            }
            type_metatype::TYPE_PARTIALUNION => unres_type
                .get_partial_base()
                .map(|p| p.get_id())
                .unwrap_or_else(|| unres_type.get_id()),
            _ => unres_type.get_id(),
        };
        ResolveEdge { type_id, op_time, encoding }
    }

    /// Construct an address-based resolve (C++
    /// `ResolveEdge::ResolveEdge(const Datatype*,const Address&,int4)`,
    /// `unionresolve.cc:118-129`).
    ///
    /// The C++ ignores the `slot` parameter for the address form and always
    /// encodes `0x2000`.
    pub fn new_addr(unres_type: &Datatype, addr: &Address, _slot: int4) -> ResolveEdge {
        // opTime = addr.getOffset();
        // encoding = 0x2000;
        // (uintm) cast: the C++ stores addr.getOffset() (uintb/u64) into the
        // uintm (u32) `opTime` member, truncating to 32 bits.
        let op_time = addr.get_offset() as uintm;
        let encoding = 0x2000;
        let type_id = match unres_type.get_metatype() {
            type_metatype::TYPE_PTR => unres_type
                .get_ptr_to()
                .map(|p| p.get_id())
                .unwrap_or_else(|| unres_type.get_id()),
            type_metatype::TYPE_PARTIALUNION => unres_type
                .get_partial_base()
                .map(|p| p.get_id())
                .unwrap_or_else(|| unres_type.get_id()),
            _ => unres_type.get_id(),
        };
        ResolveEdge { type_id, op_time, encoding }
    }
}

impl Ord for ResolveEdge {
    /// Compare based on the data-type, the slot, and the PcodeOp's unique id
    /// (C++ `ResolveEdge::operator<`, `unionresolve.hh:212-220`): order by
    /// `typeId`, then `encoding`, then `opTime`.
    fn cmp(&self, op2: &ResolveEdge) -> std::cmp::Ordering {
        // if (typeId != op2.typeId) return (typeId < op2.typeId);
        if self.type_id != op2.type_id {
            return self.type_id.cmp(&op2.type_id);
        }
        // if (encoding != op2.encoding) return (encoding < op2.encoding);
        if self.encoding != op2.encoding {
            return self.encoding.cmp(&op2.encoding);
        }
        // return (opTime < op2.opTime);
        self.op_time.cmp(&op2.op_time)
    }
}

impl PartialOrd for ResolveEdge {
    fn partial_cmp(&self, op2: &ResolveEdge) -> Option<std::cmp::Ordering> {
        Some(self.cmp(op2))
    }
}

// =============================================================================
// ScoreUnionFields constants (unionresolve.cc:131-133)
// =============================================================================

/// Threshold of trials over which to cancel additional passes (C++
/// `ScoreUnionFields::threshold`).
pub const THRESHOLD: int4 = 256;
/// Maximum number of levels to score through (C++ `ScoreUnionFields::maxPasses`).
pub const MAX_PASSES: int4 = 6;
/// Maximum number of trials to evaluate (C++ `ScoreUnionFields::maxTrials`).
pub const MAX_TRIALS: int4 = 1024;

// =============================================================================
// Scoring matrices (the facet scoring walk) — unionresolve.cc:190-1031
// =============================================================================

/// Score a trial data-type against a locked data-type (C++
/// `ScoreUnionFields::scoreLockedType`, `unionresolve.cc:190-222`).
///
/// A trial that encounters a locked data-type does not propagate through it but
/// scores the trial data-type against the locked data-type.  Pure over the two
/// data-types.
pub fn score_locked_type(ct: &Rc<Datatype>, lock_type: &Rc<Datatype>) -> int4 {
    use type_metatype::*;
    let mut score: int4 = 0;

    // if (lockType == ct) score += 5;  // Perfect match (pointer identity)
    if Rc::ptr_eq(lock_type, ct) {
        score += 5;
    }

    // while(ct->getMetatype() == TYPE_PTR) { if (lockType->getMetatype() != TYPE_PTR) break; ... }
    let mut ct: Rc<Datatype> = Rc::clone(ct);
    let mut lock_type: Rc<Datatype> = Rc::clone(lock_type);
    while ct.get_metatype() == TYPE_PTR {
        if lock_type.get_metatype() != TYPE_PTR {
            break;
        }
        score += 5;
        // ct = ((TypePointer*)ct)->getPtrTo(); lockType = ((TypePointer*)lockType)->getPtrTo();
        let next_ct = match ct.get_ptr_to() {
            Some(p) => p,
            None => break,
        };
        let next_lock = match lock_type.get_ptr_to() {
            Some(p) => p,
            None => break,
        };
        ct = next_ct;
        lock_type = next_lock;
    }

    let ct_meta = ct.get_metatype();
    let vn_meta = lock_type.get_metatype();
    if ct_meta == vn_meta {
        if ct_meta == TYPE_STRUCT
            || ct_meta == TYPE_UNION
            || ct_meta == TYPE_ARRAY
            || ct_meta == TYPE_CODE
        {
            score += 10;
        } else {
            score += 3;
        }
    } else {
        if (ct_meta == TYPE_INT && vn_meta == TYPE_UINT)
            || (ct_meta == TYPE_UINT && vn_meta == TYPE_INT)
        {
            score -= 1;
        } else {
            score -= 5;
        }
        if ct.get_size() != lock_type.get_size() {
            score -= 2;
        }
    }
    score
}

/// Resolved facts about a (possibly locked) function prototype, standing in for
/// `Funcdata::getCallSpecs(op)` / `Funcdata::getFuncProto()` (C++ `FuncProto`).
///
/// SEAM(W4): `Funcdata::getCallSpecs` and the `FuncProto`/`FuncCallSpecs` query
/// surface (`isInputLocked`/`isOutputLocked`/`numParams`/`getParam`/
/// `getOutputType`) are not yet ported (`seams::FuncProto` is a placeholder).
/// The locked branch of [`score_parameter`]/[`score_return_type`] consumes the
/// already-resolved query results here; W4 produces them from a live proto.
#[derive(Debug, Clone)]
pub struct LockedProtoFacts {
    /// `proto->isInputLocked()` (or `false` if the proto is null). // SEAM(W4)
    pub input_locked: bool,
    /// `proto->isOutputLocked()` (or `false` if the proto is null). // SEAM(W4)
    pub output_locked: bool,
    /// `proto->numParams()`. // SEAM(W4)
    pub num_params: int4,
    /// `proto->getParam(i)->getType()` for `i` in `0..num_params`. // SEAM(W4)
    pub param_types: Vec<Rc<Datatype>>,
    /// `proto->getOutputType()`. // SEAM(W4)
    pub output_type: Option<Rc<Datatype>>,
}

/// Score a trial data-type against a parameter (C++
/// `ScoreUnionFields::scoreParameter`, `unionresolve.cc:230-240`).
///
/// SEAM(W4): the C++ queries `Funcdata::getCallSpecs(op)`; here the proto facts
/// arrive as [`LockedProtoFacts`] (`None` transcribes a null proto).
pub fn score_parameter(
    ct: &Rc<Datatype>,
    proto: Option<&LockedProtoFacts>,
    param_slot: int4,
) -> int4 {
    use type_metatype::*;
    // if (proto != 0 && proto->isInputLocked() && proto->numParams() > paramSlot)
    //   return scoreLockedType(ct,proto->getParam(paramSlot)->getType());
    if let Some(p) = proto {
        if p.input_locked && p.num_params > param_slot {
            // cast: paramSlot is a valid parameter index (the > check guards it).
            return score_locked_type(ct, &p.param_types[param_slot as usize]);
        }
    }
    let meta = ct.get_metatype();
    if meta == TYPE_ARRAY || meta == TYPE_STRUCT || meta == TYPE_UNION || meta == TYPE_CODE {
        return -1; // Vaguely unlikely thing to pass as a param
    }
    0
}

/// Score a trial data-type against the return data-type of a function (C++
/// `ScoreUnionFields::scoreReturnType`, `unionresolve.cc:246-256`).
///
/// SEAM(W4): proto facts arrive as [`LockedProtoFacts`].
pub fn score_return_type(ct: &Rc<Datatype>, proto: Option<&LockedProtoFacts>) -> int4 {
    use type_metatype::*;
    // if (proto != 0 && proto->isOutputLocked()) return scoreLockedType(ct,proto->getOutputType());
    if let Some(p) = proto {
        if p.output_locked {
            if let Some(out) = &p.output_type {
                return score_locked_type(ct, out);
            }
        }
    }
    let meta = ct.get_metatype();
    if meta == TYPE_ARRAY || meta == TYPE_STRUCT || meta == TYPE_UNION || meta == TYPE_CODE {
        return -1; // Vaguely unlikely thing to return from a function
    }
    0
}

/// The result of [`deref_pointer`]: a score, plus the value data-type to recurse
/// into (or `None`).
#[derive(Debug, Clone)]
pub struct DerefResult {
    /// The score passed back (C++ `score` out-param).
    pub score: int4,
    /// The data-type of the value being loaded/stored, or `None` (C++ return).
    pub res_type: Option<Rc<Datatype>>,
}

/// Test if `fit_type` is a pointer whose pointed-to data-type is compatible with
/// the size of the value being loaded or stored (C++
/// `ScoreUnionFields::derefPointer`, `unionresolve.cc:266-290`).
///
/// \param fit_type is the trial data-type (C++ `trial.fitType`)
/// \param max_length is the trial's `maxLength`
/// \param vn_size is `vn->getSize()` of the value Varnode
/// \param typegrp builds the array fallback data-type
pub fn deref_pointer(
    fit_type: &Rc<Datatype>,
    max_length: int4,
    vn_size: int4,
    typegrp: &dyn TypeFactory,
) -> KunaResult<DerefResult> {
    use type_metatype::*;
    // score = 0;
    if fit_type.get_metatype() == TYPE_PTR {
        // Datatype *ptrto = ((TypePointer*)trial.fitType)->getPtrTo();
        let ptrto = fit_type
            .get_ptr_to()
            .ok_or_else(|| KunaError::lowlevel("derefPointer: pointer missing ptrto"))?;
        // Datatype *subType = ptrto;
        // while(subType != 0 && subType->getSize() > vn->getSize()) { int8 newoff; subType = subType->getSubType(0,&newoff); }
        let mut sub_type: Option<Rc<Datatype>> = Some(Rc::clone(&ptrto));
        while let Some(st) = &sub_type {
            if st.get_size() <= vn_size {
                break;
            }
            let (next, _newoff) = st.get_sub_type(0)?;
            sub_type = next;
        }
        // if (subType != 0 && subType->getSize() == vn->getSize()) { score = 10; return subType; }
        if let Some(st) = &sub_type {
            if st.get_size() == vn_size {
                return Ok(DerefResult { score: 10, res_type: Some(Rc::clone(st)) });
            }
        }
        // if (trial.maxLength != 0 && (vn->getSize() % ptrto->getSize()) == 0) {
        //   score = -4; return typegrp.getTypeArray(vn->getSize()/ptrto->getSize(), ptrto);
        // }
        if max_length != 0 && ptrto.get_size() != 0 && (vn_size % ptrto.get_size()) == 0 {
            let arr = typegrp.get_type_array(vn_size / ptrto.get_size(), ptrto)?;
            return Ok(DerefResult { score: -4, res_type: Some(arr) });
        }
        // score = -5;  // Score to something but it doesn't fit
        Ok(DerefResult { score: -5, res_type: None })
    } else {
        // score = -10;  // Dereferencing something that isn't even a pointer
        Ok(DerefResult { score: -10, res_type: None })
    }
}

/// Score a truncation in the data-flow (C++
/// `ScoreUnionFields::scoreTruncation`, `unionresolve.cc:931-977`).
///
/// The truncation may be an explicit `CPUI_SUBPIECE` or implied.  Returns the
/// score and the data-type to recurse into (or `None`).
///
/// \param ct is the data-type to truncate
/// \param vn_size is `vn->getSize()` of the Varnode the truncation fits into
/// \param offset is the number of bytes truncated off the start
/// \param result_base is `result.getBase()` (for the union self-match bonus), or
/// `None`
pub fn score_truncation(
    ct: &Rc<Datatype>,
    vn_size: int4,
    offset: int4,
    result_base: Option<&Rc<Datatype>>,
) -> KunaResult<(int4, Option<Rc<Datatype>>)> {
    use type_metatype::*;
    let score: int4;
    let mut ct_out: Option<Rc<Datatype>> = Some(Rc::clone(ct));
    if ct.get_metatype() == TYPE_UNION {
        // TypeUnion *unionDt = (TypeUnion *)ct;
        // ct = 0;  // Don't recurse a data-type from truncation of a union
        ct_out = None;
        // score = -10;  // Negative score if the union has no field matching the size
        let mut score_local = -10;
        let num = ct.num_depend();
        for i in 0..num {
            let field = ct
                .get_field(i)
                .ok_or_else(|| KunaError::lowlevel("scoreTruncation: union field out of range"))?;
            if field.offset == offset && field.field_type.get_size() == vn_size {
                score_local = 10;
                // if (result.getBase() == unionDt) score += 5;
                if let Some(base) = result_base {
                    if Rc::ptr_eq(base, ct) {
                        score_local += 5;
                    }
                }
                break;
            }
        }
        score = score_local;
    } else {
        // score = 10;  // If we can find a size match for the truncation
        let mut score_local = 10;
        // int8 curOff = offset;
        let mut cur_off: int8 = offset as int8;
        // while(ct != 0 && (curOff != 0 || ct->getSize() != vn->getSize())) { ... }
        // The `ct != 0` half is the `while` guard; the `(curOff != 0 || size
        // mismatch)` half is the loop-continuation check, inverted to a `break`.
        // (Cannot use `while let Some(c) = &ct_out` — the body reassigns `ct_out`.)
        while ct_out.is_some() {
            let cur = Rc::clone(ct_out.as_ref().unwrap());
            if cur_off == 0 && cur.get_size() == vn_size {
                break;
            }
            let meta = cur.get_metatype();
            if meta == TYPE_INT || meta == TYPE_UINT {
                // if (ct->getSize() >= vn->getSize() + curOff) { score = 1; break; }
                if cur.get_size() as int8 >= vn_size as int8 + cur_off {
                    score_local = 1; // Size doesn't match, but still possibly a reasonable operation
                    break;
                }
            } else if meta == TYPE_ARRAY {
                // Datatype *elType = ct->getDepend(0);
                let el_type = cur.get_depend(0).ok_or_else(|| {
                    KunaError::lowlevel("scoreTruncation: array missing element type")
                })?;
                let align = el_type.get_align_size();
                if align < vn_size {
                    // if ((curOff + vn->getSize()) % elType->getAlignSize() != 0) score = -5; else score = 1;
                    if align != 0 && (cur_off + vn_size as int8) % align as int8 != 0 {
                        score_local = -5; // Varnode is unaligned with array elements
                    } else {
                        score_local = 1; // Varnode covers array elements
                    }
                    break;
                }
            }
            // ct = ct->getSubType(curOff,&curOff);
            let (next, new_off) = cur.get_sub_type(cur_off)?;
            cur_off = new_off;
            ct_out = next;
        }
        // if (ct == 0) score = -10;
        if ct_out.is_none() {
            score_local = -10;
        }
        score = score_local;
    }
    Ok((score, ct_out))
}

/// Resolved facts for [`score_constant_fit`] standing in for the constant's
/// arch-derived classification.
///
/// SEAM(W4): the C++ reaches `glb->translate->getFloatFormat(size)` (the float
/// class + exponent) and `glb->getDefaultDataSpace()` (the pointer-bounds
/// check).  Those `Architecture`/`Translate` surfaces are not ported, so the
/// classifications arrive resolved here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantFitFacts {
    /// For a [`type_metatype::TYPE_FLOAT`] trial: the float classification of
    /// the constant.  `None` transcribes `getFloatFormat(size) == 0` (no format
    /// for the size).  // SEAM(W4)
    pub float_fact: Option<FloatConstFact>,
    /// For an int/uint/ptr trial with nonzero `val`: whether the value looks
    /// like a pointer.  This is the C++
    /// `val >= spc->getPointerLowerBound() && val <= spc->getPointerUpperBound()
    /// && bit_transitions(val,size) >= 3`.  // SEAM(W4)
    pub looks_like_pointer: bool,
}

/// The float classification facts for [`ConstantFitFacts`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatConstFact {
    /// `format->getClass(val) == FloatFormat::zero`.
    Zero,
    /// `format->getClass(val) == FloatFormat::normalized` with the exponent
    /// already extracted (`format->getExponent(val)`).
    Normalized {
        /// `format->getExponent(val)`.
        exponent: int4,
    },
    /// Any other float class (infinity/nan/denormalized/normalized-out-of-range
    /// is handled by the caller's exponent test).
    Other,
}

/// Score a trial data-type against a constant (C++
/// `ScoreUnionFields::scoreConstantFit`, `unionresolve.cc:982-1031`).
///
/// Assumes the constant has no data-type of its own; evaluates whether it looks
/// like an integer/pointer/float/etc.  Returns the score (the C++ adds it to
/// `scores[trial.scoreIndex]`).
///
/// \param size is `trial.vn->getSize()`
/// \param val is `trial.vn->getOffset()`
/// \param fit_meta is `trial.fitType->getMetatype()`
/// \param facts are the arch-derived classifications (SEAM(W4))
pub fn score_constant_fit(
    size: int4,
    val: uintb,
    fit_meta: type_metatype,
    facts: &ConstantFitFacts,
) -> int4 {
    use type_metatype::*;
    let meta = fit_meta;
    if meta == TYPE_BOOL {
        // score = (size == 1 && val < 2) ? 2 : -2;
        if size == 1 && val < 2 { 2 } else { -2 }
    } else if meta == TYPE_FLOAT {
        // score = -1;
        // const FloatFormat *format = typegrp.getArch()->translate->getFloatFormat(size);
        // if (format != 0) { fClass = format->getClass(val); ... }
        match facts.float_fact {
            None => -1,
            Some(FloatConstFact::Zero) => 2,
            Some(FloatConstFact::Normalized { exponent }) => {
                // if (exp < 7 && exp > -4) score = 2;  else stays -1
                if exponent < 7 && exponent > -4 { 2 } else { -1 }
            }
            Some(FloatConstFact::Other) => -1,
        }
    } else if meta == TYPE_INT || meta == TYPE_UINT || meta == TYPE_PTR {
        if val == 0 {
            2 // Zero is equally valid as pointer or integer
        } else {
            let looks_like_pointer = facts.looks_like_pointer;
            if meta == TYPE_PTR {
                if looks_like_pointer { 2 } else { -2 }
            } else if looks_like_pointer {
                1
            } else {
                2
            }
        }
    } else if meta == TYPE_ARRAY {
        0 // Could be multiple values from an array
    } else {
        -2
    }
}

/// Compute whether a nonzero constant looks like a pointer, transcribing the
/// inner test of `scoreConstantFit` (`unionresolve.cc:1011-1017`).
///
/// `val >= spc->getPointerLowerBound() && val <= spc->getPointerUpperBound()`
/// then `bit_transitions(val,size) >= 3`.  The pointer bounds come from the
/// default data space (SEAM(W4)); the [`bit_transitions`] half is ported in full
/// and shared so the resolved fact can be produced once the bounds are wired.
pub fn looks_like_pointer(val: uintb, size: int4, ptr_lower: uintb, ptr_upper: uintb) -> bool {
    if val >= ptr_lower && val <= ptr_upper {
        return bit_transitions(val, size) >= 3;
    }
    false
}

/// Compute the best index from a complete score vector (C++
/// `ScoreUnionFields::computeBestIndex`, `unionresolve.cc:1050-1063`).
///
/// Returns `(field_num, best_index)` where `field_num = best_index - 1`
/// (renormalized to a field index, `-1` for the whole union).  Ties keep the
/// earliest (lowest) index — the C++ uses strict `>`.
pub fn compute_best_index(scores: &[int4]) -> (int4, int4) {
    // int4 bestScore = scores[0]; int4 bestIndex = 0;
    let mut best_score = scores[0];
    let mut best_index: int4 = 0;
    // for(int4 i=1;i<scores.size();++i) if (scores[i] > bestScore) { ... }
    for (i, &s) in scores.iter().enumerate().skip(1) {
        if s > best_score {
            best_score = s;
            // cast: i < scores.len(); a score vector that large is impossible.
            best_index = i as int4;
        }
    }
    // result.fieldNum = bestIndex - 1;  // Renormalize score index to field index
    (best_index - 1, best_index)
}

/// The outcome of scoring a single trial: the score to add to the field, plus
/// the data-type (and max-length) to recurse into (or `None`/`0`).
///
/// The C++ scoring methods write `scores[trial.scoreIndex] += score` then
/// recurse on a non-null `resType`; this carries both back so the seamed driver
/// (W4/W6/W7) can apply them.
#[derive(Debug, Clone)]
pub struct TrialScore {
    /// The score to add to `scores[trial.scoreIndex]`.
    pub score: int4,
    /// The data-type to recurse into, or `None` (C++ `resType`).
    pub res_type: Option<Rc<Datatype>>,
    /// The max-length to carry into the recursion (C++ `maxLength`).
    pub max_length: int4,
}

/// The per-opcode metatype→score table for [`score_trial_down`]/
/// [`score_trial_up`] arms that depend **only** on `(OpCode, type_metatype,
/// slot)` and not on the live data-flow graph.
///
/// This is the bulk of the facet scoring walk: the long `switch` bodies of
/// `scoreTrialDown` (`unionresolve.cc:364-713`) and `scoreTrialUp`
/// (`unionresolve.cc:735-916`) whose arms set a plain `score` with no
/// `resType`/`newTrials` recursion.  The arms that *do* recurse or read live
/// varnodes (COPY/LOAD/STORE/ADD-with-constant/SUBPIECE/CALL*/RETURN) are kept in
/// the driver (SEAM(W4)/SEAM(W6)) and call the dedicated helpers above; this
/// table is the pure remainder, exercised directly by the scoring-matrix tests.
pub mod score_tables {
    use super::*;

    /// Direction the trial pushes (C++ `ScoreUnionFields::Trial::dir_type`).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Direction {
        /// Only push the fit down \e with the data-flow (C++ `fit_down`).
        Down,
        /// Only push the fit up \e against the data-flow (C++ `fit_up`).
        Up,
    }

    /// The pure-score arms of `scoreTrialDown` (`unionresolve.cc:364-713`).
    ///
    /// Returns `Some(score)` for an opcode arm that is a pure metatype→score
    /// table (no `resType` recursion and no live-varnode read), or `None` for
    /// the arms the driver must handle itself (COPY/MULTIEQUAL/INDIRECT, LOAD,
    /// STORE, CALL*/CALLIND/RETURN, INT_ADD/INT_SUB/PTRSUB pointer-arithmetic,
    /// SUBPIECE).
    ///
    /// \param opc is `trial.op->code()`
    /// \param meta is `trial.fitType->getMetatype()`
    /// \param inslot is `trial.inslot`
    pub fn score_trial_down_pure(opc: OpCode, meta: type_metatype, inslot: int4) -> Option<int4> {
        use OpCode::*;
        use type_metatype::*;
        let score = match opc {
            CPUI_CBRANCH => {
                if meta == TYPE_BOOL {
                    10
                } else {
                    -10
                }
            }
            CPUI_BRANCHIND => {
                if meta == TYPE_PTR
                    || meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else {
                    1
                }
            }
            CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -1
                } else {
                    1
                }
            }
            CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL | CPUI_INT_SCARRY | CPUI_INT_SBORROW => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else if meta == TYPE_PTR
                    || meta == TYPE_UNKNOWN
                    || meta == TYPE_UINT
                    || meta == TYPE_BOOL
                {
                    -1
                } else {
                    5
                }
            }
            CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_CARRY => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else if meta == TYPE_PTR || meta == TYPE_UNKNOWN || meta == TYPE_UINT {
                    5
                } else if meta == TYPE_INT {
                    -5
                } else {
                    0
                }
            }
            CPUI_INT_ZEXT => {
                if meta == TYPE_UINT {
                    2
                } else if meta == TYPE_INT || meta == TYPE_BOOL {
                    1
                } else if meta == TYPE_UNKNOWN {
                    0
                } else {
                    -5 // struct,union,ptr,array,code,float
                }
            }
            CPUI_INT_SEXT => {
                if meta == TYPE_INT {
                    2
                } else if meta == TYPE_UINT || meta == TYPE_BOOL {
                    1
                } else if meta == TYPE_UNKNOWN {
                    0
                } else {
                    -5 // struct,union,ptr,array,code,float
                }
            }
            CPUI_INT_2COMP => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else if meta == TYPE_PTR || meta == TYPE_UNKNOWN || meta == TYPE_BOOL {
                    -1
                } else if meta == TYPE_INT {
                    5
                } else {
                    0
                }
            }
            CPUI_INT_NEGATE | CPUI_INT_XOR | CPUI_INT_AND | CPUI_INT_OR | CPUI_POPCOUNT
            | CPUI_LZCOUNT => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                    -1
                } else if meta == TYPE_UINT || meta == TYPE_UNKNOWN {
                    2
                } else {
                    0
                }
            }
            CPUI_INT_LEFT | CPUI_INT_RIGHT => {
                if inslot == 0 {
                    if meta == TYPE_ARRAY
                        || meta == TYPE_STRUCT
                        || meta == TYPE_UNION
                        || meta == TYPE_CODE
                        || meta == TYPE_FLOAT
                    {
                        -5
                    } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                        -1
                    } else if meta == TYPE_UINT || meta == TYPE_UNKNOWN {
                        2
                    } else {
                        0
                    }
                } else if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                    || meta == TYPE_PTR
                {
                    -5
                } else {
                    1
                }
            }
            CPUI_INT_SRIGHT => {
                if inslot == 0 {
                    if meta == TYPE_ARRAY
                        || meta == TYPE_STRUCT
                        || meta == TYPE_UNION
                        || meta == TYPE_CODE
                        || meta == TYPE_FLOAT
                    {
                        -5
                    } else if meta == TYPE_PTR
                        || meta == TYPE_BOOL
                        || meta == TYPE_UINT
                        || meta == TYPE_UNKNOWN
                    {
                        -1
                    } else {
                        2
                    }
                } else if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                    || meta == TYPE_PTR
                {
                    -5
                } else {
                    1
                }
            }
            CPUI_INT_MULT => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -10
                } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                    -2
                } else {
                    5
                }
            }
            CPUI_INT_DIV | CPUI_INT_REM => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -10
                } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                    -2
                } else if meta == TYPE_UINT || meta == TYPE_UNKNOWN {
                    5
                } else {
                    0
                }
            }
            CPUI_INT_SDIV | CPUI_INT_SREM => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -10
                } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                    -2
                } else if meta == TYPE_INT {
                    5
                } else {
                    0
                }
            }
            CPUI_BOOL_NEGATE | CPUI_BOOL_AND | CPUI_BOOL_XOR | CPUI_BOOL_OR => {
                if meta == TYPE_BOOL {
                    10
                } else if meta == TYPE_INT || meta == TYPE_UINT || meta == TYPE_UNKNOWN {
                    -1
                } else {
                    -10
                }
            }
            CPUI_FLOAT_EQUAL | CPUI_FLOAT_NOTEQUAL | CPUI_FLOAT_LESS | CPUI_FLOAT_LESSEQUAL
            | CPUI_FLOAT_NAN | CPUI_FLOAT_ADD | CPUI_FLOAT_DIV | CPUI_FLOAT_MULT
            | CPUI_FLOAT_SUB | CPUI_FLOAT_NEG | CPUI_FLOAT_ABS | CPUI_FLOAT_SQRT
            | CPUI_FLOAT_FLOAT2FLOAT | CPUI_FLOAT_TRUNC | CPUI_FLOAT_CEIL | CPUI_FLOAT_FLOOR
            | CPUI_FLOAT_ROUND => {
                if meta == TYPE_FLOAT {
                    10
                } else {
                    -10
                }
            }
            CPUI_FLOAT_INT2FLOAT => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -10
                } else if meta == TYPE_PTR {
                    -5
                } else if meta == TYPE_INT {
                    5
                } else {
                    0
                }
            }
            CPUI_PIECE => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else {
                    0
                }
            }
            CPUI_SEGMENTOP => {
                if inslot == 2 {
                    if meta == TYPE_PTR {
                        5
                    } else if meta == TYPE_ARRAY
                        || meta == TYPE_STRUCT
                        || meta == TYPE_UNION
                        || meta == TYPE_CODE
                        || meta == TYPE_FLOAT
                    {
                        -5
                    } else {
                        -1
                    }
                } else if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                    || meta == TYPE_PTR
                {
                    -2
                } else {
                    0
                }
            }
            // Arms the driver handles itself (recursion / live varnode):
            CPUI_COPY | CPUI_MULTIEQUAL | CPUI_INDIRECT | CPUI_LOAD | CPUI_STORE | CPUI_CALL
            | CPUI_CALLOTHER | CPUI_CALLIND | CPUI_RETURN | CPUI_INT_ADD | CPUI_INT_SUB
            | CPUI_PTRSUB | CPUI_SUBPIECE | CPUI_PTRADD => return None,
            _ => -10, // default: Doesn't fit
        };
        Some(score)
    }

    /// The pure-score arms of `scoreTrialUp` (`unionresolve.cc:735-916`).
    ///
    /// Returns `Some(score)` for an opcode arm that is a pure metatype→score
    /// table (no propagation and no live-varnode read), or `None` for the arms
    /// the driver must handle itself (COPY/MULTIEQUAL/INDIRECT, LOAD,
    /// CALL*/CALLIND, the INT_ADD/INT_SUB/PTRSUB const-bonus, the PTRADD
    /// align-match bonus).
    ///
    /// \param def_opc is `def->code()` (the defining op)
    /// \param meta is `trial.fitType->getMetatype()`
    /// \param fit_size is `trial.fitType->getSize()` (only read by the
    /// comparison arm)
    pub fn score_trial_up_pure(
        def_opc: OpCode,
        meta: type_metatype,
        fit_size: int4,
    ) -> Option<int4> {
        use OpCode::*;
        use type_metatype::*;
        let score = match def_opc {
            CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL
            | CPUI_INT_SCARRY | CPUI_INT_SBORROW | CPUI_INT_LESS | CPUI_INT_LESSEQUAL
            | CPUI_INT_CARRY | CPUI_BOOL_NEGATE | CPUI_BOOL_AND | CPUI_BOOL_XOR | CPUI_BOOL_OR
            | CPUI_FLOAT_EQUAL | CPUI_FLOAT_NOTEQUAL | CPUI_FLOAT_LESS | CPUI_FLOAT_LESSEQUAL
            | CPUI_FLOAT_NAN => {
                if meta == TYPE_BOOL {
                    10
                } else if fit_size == 1 {
                    1
                } else {
                    -10
                }
            }
            CPUI_INT_2COMP => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else if meta == TYPE_PTR || meta == TYPE_UNKNOWN || meta == TYPE_BOOL {
                    -1
                } else if meta == TYPE_INT {
                    5
                } else {
                    0
                }
            }
            CPUI_INT_NEGATE | CPUI_INT_XOR | CPUI_INT_AND | CPUI_INT_OR | CPUI_POPCOUNT
            | CPUI_LZCOUNT => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                    -1
                } else if meta == TYPE_UINT || meta == TYPE_UNKNOWN {
                    2
                } else {
                    0
                }
            }
            CPUI_INT_LEFT | CPUI_INT_RIGHT => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                    -1
                } else if meta == TYPE_UINT || meta == TYPE_UNKNOWN {
                    2
                } else {
                    0
                }
            }
            CPUI_INT_SRIGHT => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -5
                } else if meta == TYPE_PTR
                    || meta == TYPE_BOOL
                    || meta == TYPE_UINT
                    || meta == TYPE_UNKNOWN
                {
                    -1
                } else {
                    2
                }
            }
            CPUI_INT_MULT => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -10
                } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                    -2
                } else {
                    5
                }
            }
            CPUI_INT_DIV | CPUI_INT_REM => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -10
                } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                    -2
                } else if meta == TYPE_UINT || meta == TYPE_UNKNOWN {
                    5
                } else {
                    0
                }
            }
            CPUI_INT_SDIV | CPUI_INT_SREM => {
                if meta == TYPE_ARRAY
                    || meta == TYPE_STRUCT
                    || meta == TYPE_UNION
                    || meta == TYPE_CODE
                    || meta == TYPE_FLOAT
                {
                    -10
                } else if meta == TYPE_PTR || meta == TYPE_BOOL {
                    -2
                } else if meta == TYPE_INT {
                    5
                } else {
                    0
                }
            }
            CPUI_FLOAT_ADD | CPUI_FLOAT_DIV | CPUI_FLOAT_MULT | CPUI_FLOAT_SUB | CPUI_FLOAT_NEG
            | CPUI_FLOAT_ABS | CPUI_FLOAT_SQRT | CPUI_FLOAT_FLOAT2FLOAT | CPUI_FLOAT_CEIL
            | CPUI_FLOAT_FLOOR | CPUI_FLOAT_ROUND | CPUI_FLOAT_INT2FLOAT => {
                if meta == TYPE_FLOAT {
                    10
                } else {
                    -10
                }
            }
            CPUI_FLOAT_TRUNC => {
                if meta == TYPE_INT || meta == TYPE_UINT {
                    2
                } else {
                    -2
                }
            }
            CPUI_PIECE => {
                if meta == TYPE_FLOAT || meta == TYPE_BOOL {
                    -5
                } else if meta == TYPE_CODE || meta == TYPE_PTR {
                    -2
                } else {
                    0
                }
            }
            // Arms the driver handles itself (propagation / live varnode):
            CPUI_COPY | CPUI_MULTIEQUAL | CPUI_INDIRECT | CPUI_LOAD | CPUI_CALL | CPUI_CALLOTHER
            | CPUI_CALLIND | CPUI_INT_ADD | CPUI_INT_SUB | CPUI_PTRSUB | CPUI_SUBPIECE
            | CPUI_PTRADD => return None,
            _ => -10, // default: Datatype doesn't fit
        };
        Some(score)
    }
}

// =============================================================================
// ResolveCache (unionresolve.hh:180-207, unionresolve.cc:1230-1291) — SEAM(W6)
// =============================================================================

/// A collection of specific union data-types and how they resolve in a given
/// context (C++ `ResolveCache`, `unionresolve.hh:180`).
///
/// SEAM(W6)/SEAM(W7): every method of `ResolveCache` calls
/// `Funcdata::getUnionResolution` / `setUnionField` / `Varnode::updateType`
/// (`funcdata.rs`'s `unionMap` is a `// SEAM(W6)` placeholder — those surfaces do
/// not exist yet), so the cache is left as documented stubs.  `add_resolution`
/// is a no-op (nothing to read out of a missing `unionMap`); [`ResolveCache::resolve`]
/// returns the unresolved data-type (the C++ "no entry" fallthrough at
/// `unionresolve.cc:1256`); the two `inherit_resolution` forms are absent
/// pending the Funcdata wiring.  Reported as a loss; W6/W7 fills the bodies.
#[derive(Debug, Default)]
pub struct ResolveCache {
    records: Vec<ResolveRecord>,
}

/// A specific resolution of a union data-type (C++ `ResolveCache::Record`,
/// `unionresolve.hh:182-189`).
#[derive(Debug, Clone)]
struct ResolveRecord {
    /// The union or structure needing resolution (pointers/partials stripped)
    /// (C++ `baseType`).
    base_type: Rc<Datatype>,
    /// Index of the specific resolution (C++ `fieldNum`).
    field_num: int4,
    /// Context key (C++ `key`).
    key: int4,
}

impl ResolveRecord {
    /// Does the given key/data-type match this record (C++ `Record::match`).
    fn matches(&self, k: int4, dt: &Rc<Datatype>) -> bool {
        self.key == k && Rc::ptr_eq(&self.base_type, dt)
    }
}

impl ResolveCache {
    /// Construct an empty cache (C++ `ResolveCache(Funcdata&)`).
    pub fn new() -> ResolveCache {
        ResolveCache { records: Vec::new() }
    }

    /// Look up the in-context resolution for the given data-type (C++
    /// `ResolveCache::resolve`, `unionresolve.cc:1245-1257`).
    ///
    /// If a matching record exists and its `fieldNum >= 0`, return the
    /// `fieldNum`-th component; otherwise return the original data-type.  (The
    /// `add_resolution` population path is SEAM(W6), so in practice the list is
    /// empty and this returns `dt`.)
    pub fn resolve(&self, key: int4, dt: &Rc<Datatype>) -> Rc<Datatype> {
        for rec in &self.records {
            if rec.matches(key, dt) {
                if rec.field_num < 0 {
                    return Rc::clone(dt);
                }
                // return dt->getDepend((*iter).fieldNum);
                if let Some(dep) = dt.get_depend(rec.field_num) {
                    return dep;
                }
                return Rc::clone(dt);
            }
        }
        Rc::clone(dt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::{DatatypeKind, TypeField, sub_metatype};
    use kuna_base::address::sign_extend;

    // ---- synthetic-type builders --------------------------------------------

    fn base_type(size: int4, m: type_metatype) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, m))
    }

    fn with_id(mut dt: Datatype, id: uint8) -> Rc<Datatype> {
        dt.id = id;
        Rc::new(dt)
    }

    fn pointer_to(ptrto: Rc<Datatype>, size: int4, wordsize: u32) -> Rc<Datatype> {
        let mut p = Datatype::new(size, type_metatype::TYPE_PTR);
        p.submeta = sub_metatype::SUB_PTR;
        p.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize };
        Rc::new(p)
    }

    /// Build a union with the given (offset, type) fields.
    fn make_union(size: int4, fields: &[(int4, Rc<Datatype>)]) -> Rc<Datatype> {
        let mut u = Datatype::new(size, type_metatype::TYPE_UNION);
        u.submeta = sub_metatype::SUB_UNION;
        let field: Vec<TypeField> = fields
            .iter()
            .enumerate()
            .map(|(i, (off, ct))| {
                TypeField::new(i as int4, *off, format!("f{i}"), Rc::clone(ct))
            })
            .collect();
        u.kind = DatatypeKind::Union { field };
        Rc::new(u)
    }

    // ---- ResolveEdge total order (the unionMap key) -------------------------

    #[test]
    fn resolve_edge_order_typeid_then_encoding_then_optime() {
        // typeId dominates; then encoding; then opTime.
        let int_t = base_type(4, type_metatype::TYPE_INT);
        let mut a = ResolveEdge::new_op(&int_t, 100, 0);
        // Make three edges sharing typeId but differing in encoding/opTime.
        a.type_id = 5;
        a.encoding = 2;
        a.op_time = 100;
        let mut b = a;
        b.encoding = 3; // bigger encoding -> bigger
        assert!(a < b);
        let mut c = a;
        c.type_id = 6; // typeId dominates everything
        c.encoding = 0;
        c.op_time = 0;
        assert!(a < c);
        let mut d = a;
        d.op_time = 200; // same typeId+encoding, bigger opTime
        assert!(a < d);
        // equality
        let e = a;
        assert_eq!(a.cmp(&e), std::cmp::Ordering::Equal);
    }

    #[test]
    fn resolve_edge_op_pointer_strips_and_flags() {
        // For a pointer, typeId = ptrTo->getId() and encoding = slot + 0x1000.
        let union_t = with_id(
            (*make_union(4, &[(0, base_type(4, type_metatype::TYPE_INT))])).clone(),
            42,
        );
        let ptr = pointer_to(Rc::clone(&union_t), 8, 1);
        let edge = ResolveEdge::new_op(&ptr, 7, 3);
        assert_eq!(edge.type_id, 42); // stripped to the union's id
        assert_eq!(edge.encoding, 3 + 0x1000); // pointer flag
        assert_eq!(edge.op_time, 7);

        // Non-pointer: typeId = unresType->getId(), no flag.
        let edge2 = ResolveEdge::new_op(&union_t, 9, 1);
        assert_eq!(edge2.type_id, 42);
        assert_eq!(edge2.encoding, 1);
    }

    #[test]
    fn resolve_edge_addr_form_encodes_0x2000() {
        let union_t = with_id(
            (*make_union(4, &[(0, base_type(4, type_metatype::TYPE_INT))])).clone(),
            17,
        );
        let addr = Address::new_invalid();
        // invalid address offset is 0; use a real space-less test of the encoding.
        let edge = ResolveEdge::new_addr(&union_t, &addr, 99);
        assert_eq!(edge.encoding, 0x2000); // slot ignored
        assert_eq!(edge.type_id, 17);
        assert_eq!(edge.op_time, addr.get_offset() as uintm);
    }

    // ---- ResolvedUnion ------------------------------------------------------

    #[test]
    fn resolved_union_self_resolution_strips_base() {
        // Pointer parent: baseType strips to ptrTo.
        let union_t = make_union(4, &[(0, base_type(4, type_metatype::TYPE_INT))]);
        let ptr = pointer_to(Rc::clone(&union_t), 8, 1);
        let res = ResolvedUnion::new(Rc::clone(&ptr));
        assert!(Rc::ptr_eq(res.get_datatype(), &ptr)); // resolves to the parent
        assert!(Rc::ptr_eq(res.get_base(), &union_t)); // base is the stripped union
        assert_eq!(res.get_field_num(), -1);
        assert!(!res.is_locked());

        // Non-pointer parent: baseType is the parent.
        let res2 = ResolvedUnion::new(Rc::clone(&union_t));
        assert!(Rc::ptr_eq(res2.get_base(), &union_t));
        assert_eq!(res2.get_field_num(), -1);
    }

    #[test]
    fn resolved_union_update_lock_protects_fieldnum() {
        let union_t = make_union(4, &[(0, base_type(4, type_metatype::TYPE_INT))]);
        let int_t = base_type(4, type_metatype::TYPE_INT);
        let mut res = ResolvedUnion::new(Rc::clone(&union_t));
        res.set_lock(true);
        // Same fieldNum, same resolve -> no change.
        let same = ResolvedUnion::new(Rc::clone(&union_t));
        assert!(!res.clone().update(&same));
        // Locked, but a different fieldNum -> rejected.
        let mut other = ResolvedUnion::new(Rc::clone(&union_t));
        other.field_num = 0;
        other.resolve = Rc::clone(&int_t);
        assert!(!res.update(&other));
        // Same fieldNum but new resolve -> change accepted even when locked.
        let mut newres = ResolvedUnion::new(Rc::clone(&union_t));
        newres.field_num = -1;
        newres.resolve = Rc::clone(&int_t);
        assert!(res.update(&newres));
        assert!(Rc::ptr_eq(res.get_datatype(), &int_t));
    }

    // ---- score_locked_type matrix -------------------------------------------

    #[test]
    fn score_locked_type_matrix() {
        let int4_t = base_type(4, type_metatype::TYPE_INT);
        let uint4_t = base_type(4, type_metatype::TYPE_UINT);
        let int8_t = base_type(8, type_metatype::TYPE_INT);
        let struct_t = base_type(4, type_metatype::TYPE_STRUCT);

        // Perfect pointer identity + same metatype (plain int, not composite): 5 + 3.
        assert_eq!(score_locked_type(&int4_t, &int4_t), 5 + 3);
        // Different objects, same metatype int (size match): +3.
        let int4_b = base_type(4, type_metatype::TYPE_INT);
        assert_eq!(score_locked_type(&int4_t, &int4_b), 3);
        // int vs uint, same size: -1 mismatch, no size penalty.
        assert_eq!(score_locked_type(&int4_t, &uint4_t), -1);
        // int vs int, different size: same metatype +3 (size only penalizes on mismatch metatype).
        assert_eq!(score_locked_type(&int4_t, &int8_t), 3);
        // int vs uint, different size: -1 (sign mismatch) + (-2 size): -3.
        let uint8_t = base_type(8, type_metatype::TYPE_UINT);
        assert_eq!(score_locked_type(&int4_t, &uint8_t), -1 - 2);
        // struct vs struct, same metatype composite: +3? No — composite gets +10.
        let struct_b = base_type(4, type_metatype::TYPE_STRUCT);
        assert_eq!(score_locked_type(&struct_t, &struct_b), 10);
        // int vs struct (general mismatch), same size: -5.
        assert_eq!(score_locked_type(&int4_t, &struct_t), -5);

        // Pointer chain: ptr->ptr->int vs ptr->ptr->int, identity at top: 5 + 5 + 5 + 3.
        let pp_a = pointer_to(pointer_to(Rc::clone(&int4_t), 8, 1), 8, 1);
        assert_eq!(score_locked_type(&pp_a, &pp_a), 5 + 5 + 5 + 3);
    }

    // ---- score_parameter / score_return_type --------------------------------

    #[test]
    fn score_parameter_unlocked_defaults() {
        let int_t = base_type(4, type_metatype::TYPE_INT);
        let struct_t = base_type(4, type_metatype::TYPE_STRUCT);
        // No proto: composite -> -1, scalar -> 0.
        assert_eq!(score_parameter(&struct_t, None, 0), -1);
        assert_eq!(score_parameter(&int_t, None, 0), 0);
        assert_eq!(score_return_type(&struct_t, None), -1);
        assert_eq!(score_return_type(&int_t, None), 0);
    }

    #[test]
    fn score_parameter_locked_uses_score_locked_type() {
        let int_t = base_type(4, type_metatype::TYPE_INT);
        let facts = LockedProtoFacts {
            input_locked: true,
            output_locked: true,
            num_params: 1,
            param_types: vec![Rc::clone(&int_t)],
            output_type: Some(Rc::clone(&int_t)),
        };
        // Locked, slot 0 in range -> scoreLockedType(int,int identity) = 5+3.
        assert_eq!(score_parameter(&int_t, Some(&facts), 0), 8);
        // Locked but slot out of range -> falls through to default scalar 0.
        assert_eq!(score_parameter(&int_t, Some(&facts), 5), 0);
        // Locked output -> scoreLockedType identity 8.
        assert_eq!(score_return_type(&int_t, Some(&facts)), 8);
    }

    // ---- score_constant_fit -------------------------------------------------

    #[test]
    fn score_constant_fit_matrix() {
        use type_metatype::*;
        let no_facts = ConstantFitFacts { float_fact: None, looks_like_pointer: false };
        // bool: size 1, val<2 -> 2; else -2.
        assert_eq!(score_constant_fit(1, 1, TYPE_BOOL, &no_facts), 2);
        assert_eq!(score_constant_fit(1, 5, TYPE_BOOL, &no_facts), -2);
        assert_eq!(score_constant_fit(2, 1, TYPE_BOOL, &no_facts), -2);
        // float, no format -> -1.
        assert_eq!(score_constant_fit(4, 0, TYPE_FLOAT, &no_facts), -1);
        // float, zero class -> 2.
        let zero_facts =
            ConstantFitFacts { float_fact: Some(FloatConstFact::Zero), looks_like_pointer: false };
        assert_eq!(score_constant_fit(4, 0, TYPE_FLOAT, &zero_facts), 2);
        // float, normalized in range -> 2, out of range -> -1.
        let norm_in = ConstantFitFacts {
            float_fact: Some(FloatConstFact::Normalized { exponent: 3 }),
            looks_like_pointer: false,
        };
        assert_eq!(score_constant_fit(4, 1, TYPE_FLOAT, &norm_in), 2);
        let norm_out = ConstantFitFacts {
            float_fact: Some(FloatConstFact::Normalized { exponent: 20 }),
            looks_like_pointer: false,
        };
        assert_eq!(score_constant_fit(4, 1, TYPE_FLOAT, &norm_out), -1);
        // int/uint/ptr, val==0 -> 2.
        assert_eq!(score_constant_fit(4, 0, TYPE_INT, &no_facts), 2);
        assert_eq!(score_constant_fit(4, 0, TYPE_PTR, &no_facts), 2);
        // nonzero ptr, looks like pointer -> 2, else -2.
        let ptrish = ConstantFitFacts { float_fact: None, looks_like_pointer: true };
        assert_eq!(score_constant_fit(4, 0x401000, TYPE_PTR, &ptrish), 2);
        assert_eq!(score_constant_fit(4, 0x401000, TYPE_PTR, &no_facts), -2);
        // nonzero int, looks like pointer -> 1, else -> 2.
        assert_eq!(score_constant_fit(4, 0x401000, TYPE_INT, &ptrish), 1);
        assert_eq!(score_constant_fit(4, 0x401000, TYPE_INT, &no_facts), 2);
        // array -> 0; other -> -2.
        assert_eq!(score_constant_fit(4, 7, TYPE_ARRAY, &no_facts), 0);
        assert_eq!(score_constant_fit(4, 7, TYPE_CODE, &no_facts), -2);
    }

    #[test]
    fn looks_like_pointer_bounds_and_transitions() {
        // In bounds with enough bit transitions.
        assert!(looks_like_pointer(0x401000, 4, 0x1000, 0x7fffffff));
        // Out of (above) upper bound.
        assert!(!looks_like_pointer(0x80000000, 4, 0x1000, 0x7fffffff));
        // In bounds but too few bit transitions (e.g. a single run).
        assert!(!looks_like_pointer(0x1, 4, 0x0, 0x7fffffff));
    }

    // ---- compute_best_index -------------------------------------------------

    #[test]
    fn compute_best_index_ties_keep_lowest() {
        // index 0 is the whole-union; field_num = bestIndex - 1.
        let (fnum, idx) = compute_best_index(&[0, 5, 5, 3]);
        assert_eq!(idx, 1); // strict > keeps the first max
        assert_eq!(fnum, 0);
        // whole union wins.
        let (fnum2, idx2) = compute_best_index(&[10, 5, 3]);
        assert_eq!(idx2, 0);
        assert_eq!(fnum2, -1);
        // last field wins.
        let (fnum3, idx3) = compute_best_index(&[0, 1, 2, 9]);
        assert_eq!(idx3, 3);
        assert_eq!(fnum3, 2);
    }

    // ---- score_truncation ---------------------------------------------------

    #[test]
    fn score_truncation_union_field_match() {
        use type_metatype::*;
        // Union with an int4 field at offset 0 and a uint2 field at offset 0.
        let int4_t = base_type(4, TYPE_INT);
        let uint2_t = base_type(2, TYPE_UINT);
        let union_t = make_union(4, &[(0, Rc::clone(&int4_t)), (0, Rc::clone(&uint2_t))]);
        // Truncation of size 2 at offset 0 matches the uint2 field -> 10, no recurse.
        let (score, recurse) = score_truncation(&union_t, 2, 0, None).unwrap();
        assert_eq!(score, 10);
        assert!(recurse.is_none()); // unions never recurse from truncation
        // With result.getBase() == this union -> +5 bonus.
        let (score_b, _) = score_truncation(&union_t, 2, 0, Some(&union_t)).unwrap();
        assert_eq!(score_b, 15);
        // No field matching size 3 -> -10.
        let (score_c, recurse_c) = score_truncation(&union_t, 3, 0, None).unwrap();
        assert_eq!(score_c, -10);
        assert!(recurse_c.is_none());
    }

    #[test]
    fn score_truncation_scalar_exact_match() {
        use type_metatype::*;
        // Exact size match at offset 0: score 10, recurse the type itself.
        let int4_t = base_type(4, TYPE_INT);
        let (score, recurse) = score_truncation(&int4_t, 4, 0, None).unwrap();
        assert_eq!(score, 10);
        assert!(recurse.is_some());
        assert!(Rc::ptr_eq(&recurse.unwrap(), &int4_t));
        // int wider than the varnode, offset 0: int can hold it -> score 1.
        let int8_t = base_type(8, TYPE_INT);
        let (score2, _) = score_truncation(&int8_t, 4, 0, None).unwrap();
        assert_eq!(score2, 1);
    }

    // ---- score table matrices (the down/up walk) ----------------------------

    #[test]
    fn score_trial_down_pure_samples() {
        use OpCode::*;
        use score_tables::score_trial_down_pure as down;
        use type_metatype::*;
        // CBRANCH: bool -> 10, else -10.
        assert_eq!(down(CPUI_CBRANCH, TYPE_BOOL, -1), Some(10));
        assert_eq!(down(CPUI_CBRANCH, TYPE_INT, -1), Some(-10));
        // INT_SLESS: signed int -> 5; uint -> -1; struct -> -5.
        assert_eq!(down(CPUI_INT_SLESS, TYPE_INT, 0), Some(5));
        assert_eq!(down(CPUI_INT_SLESS, TYPE_UINT, 0), Some(-1));
        assert_eq!(down(CPUI_INT_SLESS, TYPE_STRUCT, 0), Some(-5));
        // INT_LESS: uint/ptr/unknown -> 5; int -> -5; bool -> 0 (falls to else).
        assert_eq!(down(CPUI_INT_LESS, TYPE_UINT, 0), Some(5));
        assert_eq!(down(CPUI_INT_LESS, TYPE_INT, 0), Some(-5));
        assert_eq!(down(CPUI_INT_LESS, TYPE_BOOL, 0), Some(0));
        // INT_LEFT slot 0 vs slot 1 differ.
        assert_eq!(down(CPUI_INT_LEFT, TYPE_UINT, 0), Some(2));
        assert_eq!(down(CPUI_INT_LEFT, TYPE_UINT, 1), Some(1));
        assert_eq!(down(CPUI_INT_LEFT, TYPE_PTR, 1), Some(-5));
        // SEGMENTOP slot 2 vs other.
        assert_eq!(down(CPUI_SEGMENTOP, TYPE_PTR, 2), Some(5));
        assert_eq!(down(CPUI_SEGMENTOP, TYPE_PTR, 0), Some(-2));
        // FLOAT_ADD: float -> 10 else -10.
        assert_eq!(down(CPUI_FLOAT_ADD, TYPE_FLOAT, 0), Some(10));
        assert_eq!(down(CPUI_FLOAT_ADD, TYPE_INT, 0), Some(-10));
        // Driver-handled arms return None.
        assert_eq!(down(CPUI_COPY, TYPE_INT, 0), None);
        assert_eq!(down(CPUI_LOAD, TYPE_PTR, -1), None);
        assert_eq!(down(CPUI_INT_ADD, TYPE_PTR, 0), None);
        assert_eq!(down(CPUI_SUBPIECE, TYPE_INT, 0), None);
        // Unknown opcode default -10.
        assert_eq!(down(CPUI_NEW, TYPE_INT, 0), Some(-10));
    }

    #[test]
    fn score_trial_up_pure_samples() {
        use OpCode::*;
        use score_tables::score_trial_up_pure as up;
        use type_metatype::*;
        // Comparison: bool -> 10; size-1 -> 1; else -10.
        assert_eq!(up(CPUI_INT_EQUAL, TYPE_BOOL, 1), Some(10));
        assert_eq!(up(CPUI_INT_EQUAL, TYPE_INT, 1), Some(1));
        assert_eq!(up(CPUI_INT_EQUAL, TYPE_INT, 4), Some(-10));
        // INT_MULT: composite -> -10; ptr/bool -> -2; else 5.
        assert_eq!(up(CPUI_INT_MULT, TYPE_STRUCT, 4), Some(-10));
        assert_eq!(up(CPUI_INT_MULT, TYPE_PTR, 8), Some(-2));
        assert_eq!(up(CPUI_INT_MULT, TYPE_INT, 4), Some(5));
        // FLOAT_TRUNC: int/uint -> 2 else -2.
        assert_eq!(up(CPUI_FLOAT_TRUNC, TYPE_INT, 4), Some(2));
        assert_eq!(up(CPUI_FLOAT_TRUNC, TYPE_FLOAT, 4), Some(-2));
        // PIECE: float/bool -> -5; code/ptr -> -2; else 0.
        assert_eq!(up(CPUI_PIECE, TYPE_FLOAT, 4), Some(-5));
        assert_eq!(up(CPUI_PIECE, TYPE_CODE, 4), Some(-2));
        assert_eq!(up(CPUI_PIECE, TYPE_INT, 4), Some(0));
        // Driver-handled arms return None.
        assert_eq!(up(CPUI_COPY, TYPE_INT, 4), None);
        assert_eq!(up(CPUI_PTRADD, TYPE_PTR, 8), None);
        assert_eq!(up(CPUI_INT_ADD, TYPE_PTR, 8), None);
        // Unknown default -10.
        assert_eq!(up(CPUI_NEW, TYPE_INT, 4), Some(-10));
    }

    // ---- ResolveCache stub --------------------------------------------------

    #[test]
    fn resolve_cache_empty_returns_unresolved() {
        let cache = ResolveCache::new();
        let int_t = base_type(4, type_metatype::TYPE_INT);
        // No records (population is SEAM(W6)) -> returns the original data-type.
        assert!(Rc::ptr_eq(&cache.resolve(0, &int_t), &int_t));
    }

    // Touch the otherwise-driver-only imports so a future driver wiring change
    // surfaces here rather than as dead-code warnings.
    #[test]
    fn constants_and_helpers_present() {
        assert_eq!(THRESHOLD, 256);
        assert_eq!(MAX_PASSES, 6);
        assert_eq!(MAX_TRIALS, 1024);
        // sign_extend/int8/uint8/uintb are used by the seamed driver helpers;
        // exercise sign_extend here to keep the import live and pin its behavior
        // for the INT_ADD constant-offset arm (C++ `sign_extend(off,size*8-1)`).
        let off: int8 = sign_extend(0xff, 8 * 1 - 1);
        assert_eq!(off, -1);
        let _ = (0u64 as uint8, 0u64 as uintb);
    }

    fn _use_deref(typegrp: &dyn TypeFactory) {
        // Reference deref_pointer / direction so a driver rewiring keeps them
        // discoverable (not invoked: needs a live TypeFactory).
        let int_t = base_type(4, type_metatype::TYPE_INT);
        let _ = deref_pointer(&int_t, 0, 4, typegrp);
        let _ = score_tables::Direction::Down;
        let _ = score_tables::Direction::Up;
    }
}
