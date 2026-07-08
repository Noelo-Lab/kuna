//! The `ScoreUnionFields` **driver** — the graph-walking scoring engine
//! (`decompiler/cpp/unionresolve.cc:135-1223`) that the leaf score functions in
//! [`crate::unionresolve`] feed (W10 item `w10-union-scoring`).
//!
//! [`crate::unionresolve`] already lands every *leaf* scoring decision
//! (`scoreLockedType`/`scoreParameter`/`scoreReturnType`/`scoreTruncation`/
//! `scoreConstantFit`/`derefPointer`/`computeBestIndex` plus the per-opcode
//! metatype→score tables `score_trial_down_pure`/`score_trial_up_pure`).  What was
//! missing — the seam this module closes — is the **driver** that walks the live
//! `Funcdata` data-flow graph: `ScoreUnionFields::run`/`runOneLevel`/`newTrials`/
//! `newTrialsDown`/`scoreTrialDown`/`scoreTrialUp` and the three constructors, plus
//! `testSimpleCases`/`testArrayArithmetic`.  Without it the per-function
//! `unionMap` cache ([`crate::funcdata_union`]) never gets populated, so
//! `find_resolve_facing` ([`crate::funcdata_facing`]) always misses and union field
//! accesses never resolve (the cast plane renders `CAST(...)` instead of
//! `ptr->field`).
//!
//! # Faithfulness / the Rust borrow model
//!
//! The C++ `ScoreUnionFields` is a struct holding `Funcdata &data` and
//! `TypeFactory &typegrp`, mutated through the scoring run.  The scoring **never
//! mutates the data-flow graph** — it only reads varnodes/ops (`beginDescend`,
//! `getDef`, `getSlot`, `isTypeLock`, `getIn`/`getOut`, `getOffset`, `getSize`,
//! `code`) and accumulates into local `scores`/`fields`/`trialCurrent`/`trialNext`/
//! `visited`.  The single graph write (`setUnionField`) happens in
//! `resolveInFlow` *after* `run()` returns the `ResolvedUnion`.  So the driver here
//! takes `&Funcdata` immutably; [`crate::funcdata_union::Funcdata::resolve_in_flow`]
//! drives it and then writes the result.
//!
//! Every method is transcribed statement-for-statement from the cited C++ body.
//! `Varnode *` / `PcodeOp *` become [`VarnodeId`]/[`OpId`] read back through the
//! banks; `Datatype *` become `Rc<Datatype>` (a null pointer becomes `None`).
//!
//! # Seams folded in (vs. the leaf-function placeholders)
//!
//! The leaf functions in [`crate::unionresolve`] took resolved seam inputs
//! ([`LockedProtoFacts`]/[`ConstantFitFacts`]) because they had no `Funcdata`.  The
//! driver *does* have the `Funcdata`, so it builds those facts from the live proto
//! (`data.getCallSpecs(op)` / `data.getFuncProto()`) and the live constant
//! (`glb->getDefaultDataSpace()` pointer bounds + `translate->getFloatFormat`).
//! The float-format query is the one surface the merged arch-handle does not yet
//! expose; absent it the `FLOAT` constant arm takes the C++ `format == 0` branch
//! (`score = -1`), recorded as a loss.

use std::rc::Rc;

use kuna_base::address::sign_extend;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, int8, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::context::{OpId, VarnodeId};
use crate::unionresolve::{
    compute_best_index, deref_pointer, score_constant_fit, score_locked_type, score_parameter,
    score_return_type, score_tables, score_truncation, ConstantFitFacts, FloatConstFact,
    LockedProtoFacts, ResolvedUnion, MAX_PASSES, MAX_TRIALS, THRESHOLD,
};

/// Direction a trial pushes (C++ `ScoreUnionFields::Trial::dir_type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirType {
    /// Only push the fit down \e with the data-flow (C++ `fit_down`).
    Down,
    /// Only push the fit up \e against the data-flow (C++ `fit_up`).
    Up,
}

/// A trial data-type fitted to a specific place in the data-flow
/// (C++ `ScoreUnionFields::Trial`, `unionresolve.hh:88-121`).
#[derive(Debug, Clone)]
struct Trial {
    /// The Varnode we are testing for data-type fit (C++ `vn`).
    vn: VarnodeId,
    /// The PcodeOp reading the Varnode, or `None` for an upward trial (C++ `op`).
    op: Option<OpId>,
    /// The slot reading the Varnode (or -1) (C++ `inslot`).
    inslot: int4,
    /// Direction to push fit (C++ `direction`).
    direction: DirType,
    /// Maximum byte offset that can be added to `fit_type` as a pointer
    /// (C++ `maxLength`).
    max_length: int4,
    /// The putative data-type of the Varnode (C++ `fitType`).
    fit_type: Rc<Datatype>,
    /// The original field being scored by this trial (C++ `scoreIndex`).
    score_index: int4,
}

impl Trial {
    /// Construct a downward trial for a Varnode (C++ `Trial(PcodeOp*,int4,...)`,
    /// `unionresolve.hh:110-111`): `vn = o->getIn(slot)`.
    fn new_down(
        data: &Funcdata,
        o: OpId,
        slot: int4,
        ct: Rc<Datatype>,
        index: int4,
        max: int4,
    ) -> Trial {
        let vn = data
            .obank()
            .get(o)
            .and_then(|op| op.get_in(slot))
            .expect("Trial::new_down: stale op / slot");
        Trial {
            vn,
            op: Some(o),
            inslot: slot,
            direction: DirType::Down,
            fit_type: ct,
            score_index: index,
            max_length: max,
        }
    }

    /// Construct an upward trial for a Varnode (C++ `Trial(Varnode*,...)`,
    /// `unionresolve.hh:119-120`).
    fn new_up(v: VarnodeId, ct: Rc<Datatype>, index: int4, max: int4) -> Trial {
        Trial {
            vn: v,
            op: None,
            inslot: -1,
            direction: DirType::Up,
            fit_type: ct,
            score_index: index,
            max_length: max,
        }
    }
}

/// A mark accumulated when a given Varnode is visited with a specific field index
/// (C++ `ScoreUnionFields::VisitMark`, `unionresolve.hh:124-139`).
///
/// The C++ `operator<` orders by `(vn, index)` (Varnode pointer first).  The Rust
/// [`VarnodeId`] is `Copy + Ord`, so `(VarnodeId, int4)` reproduces the set key
/// ordering exactly (the pointer order is not observable as a *score* — only
/// "already inserted?" matters — and a `BTreeSet<(VarnodeId,int4)>` answers that).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct VisitMark(VarnodeId, int4);

/// The graph-walking union-field scoring engine (C++ `ScoreUnionFields`,
/// `unionresolve.hh:86-172`).
///
/// Holds the scoring state and an immutable `&Funcdata` for graph queries plus a
/// `&dyn TypeFactory` for type construction.  Build with one of the three
/// constructors, then read [`ScoreUnionFields::get_result`].
pub struct ScoreUnionFields<'a> {
    /// Function containing data-flow being scored (C++ `data`).
    data: &'a Funcdata,
    /// The factory containing data-types (C++ `typegrp`).
    typegrp: &'a crate::dtype::TypeFactoryImpl,
    /// Score for each field, indexed by `fieldNum + 1` (whole union is index 0)
    /// (C++ `scores`).
    scores: Vec<int4>,
    /// Field corresponding to each score (C++ `fields`).
    fields: Vec<Rc<Datatype>>,
    /// Places that have already been visited (C++ `visited`).
    visited: std::collections::BTreeSet<VisitMark>,
    /// Current trials being pushed (C++ `trialCurrent`).
    trial_current: Vec<Trial>,
    /// Next set of trials (C++ `trialNext`).
    trial_next: Vec<Trial>,
    /// The best result (C++ `result`).
    result: ResolvedUnion,
    /// Number of trials evaluated so far (C++ `trialCount`).
    trial_count: int4,
}

impl<'a> ScoreUnionFields<'a> {
    /// Get the resulting best field resolution (C++ `getResult`).
    pub fn get_result(&self) -> &ResolvedUnion {
        &self.result
    }

    /// Take ownership of the resulting best field resolution (so the caller can
    /// store it into the cache after the driver borrow ends).
    pub fn into_result(self) -> ResolvedUnion {
        self.result
    }

    // -- testArrayArithmetic / testSimpleCases (unionresolve.cc:140-183) -------

    /// C++ `ScoreUnionFields::testArrayArithmetic` (unionresolve.cc:140-164).
    fn test_array_arithmetic(&self, op: OpId, inslot: int4) -> bool {
        let o = match self.data.obank().get(op) {
            Some(o) => o,
            None => return false,
        };
        let base_size = self.result.get_base().get_size();
        if o.code() == OpCode::CPUI_INT_ADD {
            // Varnode *vn = op->getIn(1 - inslot);
            let vn = match o.get_in(1 - inslot) {
                Some(v) => v,
                None => return false,
            };
            let v = match self.data.vbank().get(vn) {
                Some(v) => v,
                None => return false,
            };
            if v.is_constant() {
                // if (vn->getOffset() >= result.baseType->getSize()) return true;
                if v.get_offset() >= base_size as uintb {
                    return true;
                }
            } else if v.is_written() {
                // PcodeOp *multOp = vn->getDef();
                let mult_op = match v.get_def() {
                    Some(m) => m,
                    None => return false,
                };
                let mo = match self.data.obank().get(mult_op) {
                    Some(m) => m,
                    None => return false,
                };
                if mo.code() == OpCode::CPUI_INT_MULT {
                    // Varnode *vn2 = multOp->getIn(1);
                    if let Some(vn2) = mo.get_in(1) {
                        if let Some(v2) = self.data.vbank().get(vn2) {
                            if v2.is_constant() && v2.get_offset() >= base_size as uintb {
                                return true;
                            }
                        }
                    }
                }
            }
        } else if o.code() == OpCode::CPUI_PTRADD {
            // Varnode *vn = op->getIn(2);
            if let Some(vn) = o.get_in(2) {
                if let Some(v) = self.data.vbank().get(vn) {
                    if v.get_offset() >= base_size as uintb {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// C++ `ScoreUnionFields::testSimpleCases` (unionresolve.cc:171-183).
    fn test_simple_cases(&self, op: OpId, inslot: int4, parent: &Rc<Datatype>) -> bool {
        let is_marker = self.data.obank().get(op).map(|o| o.is_marker()).unwrap_or(false);
        // if (op->isMarker() && inslot < 0) return true;
        if is_marker && inslot < 0 {
            return true; // Propagate raw union across MULTIEQUAL and INDIRECT
        }
        if parent.get_metatype() == type_metatype::TYPE_PTR {
            // if (inslot < 0) return true;
            if inslot < 0 {
                return true; // Don't resolve pointers "up"
            }
            if self.test_array_arithmetic(op, inslot) {
                return true;
            }
        }
        false
    }

    // -- proto / constant fact builders (the folded-in W4 seams) --------------

    /// Build the locked-proto facts for a CALL/CALLOTHER/CALLIND op
    /// (C++ `data.getCallSpecs(op)`), or `None` if there is no call-spec for it.
    fn callspecs_facts(&self, op: OpId) -> Option<LockedProtoFacts> {
        let idx = self.data.get_call_specs_index(op)?;
        let fc = self.data.get_call_specs(idx);
        Some(Self::proto_facts(fc.proto()))
    }

    /// Build the locked-proto facts for the function's own prototype
    /// (C++ `&data.getFuncProto()`).
    fn funcproto_facts(&self) -> LockedProtoFacts {
        Self::proto_facts(self.data.get_func_proto())
    }

    /// Resolve a [`crate::fspec::FuncProto`] into the seam-free [`LockedProtoFacts`]
    /// the leaf [`score_parameter`]/[`score_return_type`] consume.
    fn proto_facts(proto: &crate::fspec::FuncProto) -> LockedProtoFacts {
        let input_locked = proto.is_input_locked();
        let output_locked = proto.is_output_locked();
        let num_params = proto.num_params();
        // Snapshot every param type so the leaf indexes `param_types[paramSlot]`.
        let mut param_types: Vec<Rc<Datatype>> = Vec::with_capacity(num_params.max(0) as usize);
        for i in 0..num_params {
            // proto->getParam(i)->getType()
            let ty = proto
                .get_param(i)
                .and_then(|p| p.get_type().cloned())
                .unwrap_or_else(|| Rc::new(Datatype::new(1, type_metatype::TYPE_UNKNOWN)));
            param_types.push(ty);
        }
        let output_type = proto.get_output_type().cloned();
        LockedProtoFacts { input_locked, output_locked, num_params, param_types, output_type }
    }

    /// Build the constant-fit facts for a constant Varnode (the float class +
    /// pointer-bounds classifications that the leaf [`score_constant_fit`]
    /// consumes, folding in the C++ `getFloatFormat`/`getDefaultDataSpace` seams).
    fn constant_facts(&self, size: int4, val: uintb, fit_meta: type_metatype) -> ConstantFitFacts {
        use type_metatype::*;
        // FLOAT: const FloatFormat *format = translate->getFloatFormat(size);
        let float_fact = if fit_meta == TYPE_FLOAT {
            // STUB(W4 residual): the merged arch-handle does not expose
            // getFloatFormat; absent a format the C++ leaves score = -1, which
            // `float_fact: None` reproduces exactly (loss recorded).
            self.float_format_fact(size, val)
        } else {
            None
        };
        // INT/UINT/PTR nonzero: spc->getPointerLowerBound()/UpperBound() check.
        let looks_like_pointer = if (fit_meta == TYPE_INT
            || fit_meta == TYPE_UINT
            || fit_meta == TYPE_PTR)
            && val != 0
        {
            match self.data.get_arch().manage().get_default_data_space() {
                Some(spc) => crate::unionresolve::looks_like_pointer(
                    val,
                    size,
                    spc.get_pointer_lower_bound(),
                    spc.get_pointer_upper_bound(),
                ),
                None => false,
            }
        } else {
            false
        };
        ConstantFitFacts { float_fact, looks_like_pointer }
    }

    /// The float-format classification (`getClass`/`getExponent`) for the constant,
    /// or `None` when the arch exposes no format for the size (C++ `format == 0`).
    ///
    /// STUB(W4 residual) / LOSS: the merged arch-handle
    /// ([`crate::context::ArchContext`]) does not carry a `FloatFormatProvider`
    /// (the float formats live only on the concrete
    /// [`crate::architecture::Architecture`] behind the lift path, not on the
    /// `glb` seam handle the `Funcdata` holds).  Absent the format, the C++
    /// `scoreConstantFit` FLOAT arm leaves `score = -1` (the `format == 0`
    /// branch, `unionresolve.cc:982-994`), which `None` reproduces exactly.  This
    /// only changes the *constant-fit* disambiguation between two float-sized
    /// union fields; the corpus float fields are disambiguated by data flow
    /// (FLOAT_* opcodes scoring +10 in the pure tables), not constant exponent,
    /// so the live `union_datatype.xml` renders are unaffected.  Wiring a
    /// `FloatFormatProvider` onto the seam flips this on with no further change.
    fn float_format_fact(&self, size: int4, val: uintb) -> Option<FloatConstFact> {
        let _ = (size, val);
        None
    }

    // -- newTrialsDown / newTrials (unionresolve.cc:297-346) -------------------

    /// C++ `ScoreUnionFields::newTrialsDown` (unionresolve.cc:297-315).
    fn new_trials_down(&mut self, vn: VarnodeId, ct: Rc<Datatype>, score_index: int4, max: int4) {
        // if (!visited.insert(mark).second) return;
        if !self.visited.insert(VisitMark(vn, score_index)) {
            return; // Already visited this Varnode
        }
        // if (vn->isTypeLock()) { ... scoreLockedType(ct,vn->getTypeDefFacing()) ... }
        let is_lock = self.data.vbank().get(vn).map(|v| v.is_type_lock()).unwrap_or(false);
        if is_lock {
            let lock_type = self.data.vn_type_def_facing(vn);
            if !lock_type.needs_resolution() {
                self.scores[score_index as usize] += score_locked_type(&ct, &lock_type);
                return; // Don't propagate through locked Varnode
            }
        }
        // for(piter = vn->beginDescend(); ...) trialNext.emplace_back(op,op->getSlot(vn),...)
        let descends: Vec<OpId> = self
            .data
            .vbank()
            .get(vn)
            .map(|v| v.descend_iter().collect())
            .unwrap_or_default();
        for op in descends {
            let slot = self.data.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
            self.trial_next.push(Trial::new_down(self.data, op, slot, Rc::clone(&ct), score_index, max));
        }
    }

    /// C++ `ScoreUnionFields::newTrials` (unionresolve.cc:323-346).
    fn new_trials(&mut self, op: OpId, slot: int4, ct: Rc<Datatype>, score_index: int4, max: int4) {
        // Varnode *vn = op->getIn(slot);
        let vn = match self.data.obank().get(op).and_then(|o| o.get_in(slot)) {
            Some(v) => v,
            None => return,
        };
        // if (!visited.insert(mark).second) return;
        if !self.visited.insert(VisitMark(vn, score_index)) {
            return; // Already visited this Varnode
        }
        let is_lock = self.data.vbank().get(vn).map(|v| v.is_type_lock()).unwrap_or(false);
        if is_lock {
            // Datatype *lockType = vn->getTypeReadFacing(op);
            let lock_type = self.data.vn_type_read_facing(vn, op);
            if !lock_type.needs_resolution() {
                self.scores[score_index as usize] += score_locked_type(&ct, &lock_type);
                return; // Don't propagate through locked Varnode
            }
        }
        // trialNext.emplace_back(vn,ct,scoreIndex,max);  // Try to fit up
        self.trial_next.push(Trial::new_up(vn, Rc::clone(&ct), score_index, max));
        // for(iter=vn->beginDescend(); ...) { if (readOp==op && inslot==slot) continue; ... }
        let descends: Vec<OpId> = self
            .data
            .vbank()
            .get(vn)
            .map(|v| v.descend_iter().collect())
            .unwrap_or_default();
        for read_op in descends {
            let inslot = self.data.obank().get(read_op).map(|o| o.get_slot(vn)).unwrap_or(-1);
            if read_op == op && inslot == slot {
                continue; // Don't go down PcodeOp we came from
            }
            self.trial_next.push(Trial::new_down(
                self.data,
                read_op,
                inslot,
                Rc::clone(&ct),
                score_index,
                max,
            ));
        }
    }

    // -- scoreTrialDown / scoreTrialUp (unionresolve.cc:355-921) ---------------

    /// C++ `ScoreUnionFields::scoreTrialDown` (unionresolve.cc:355-717).
    fn score_trial_down(&mut self, trial: &Trial, last_level: bool) -> KunaResult<()> {
        if trial.direction == DirType::Up {
            return Ok(()); // Trial doesn't push in this direction
        }
        let mut res_type: Option<Rc<Datatype>> = None;
        let mut max_length: int4 = 0;
        let meta = trial.fit_type.get_metatype();
        let mut score: int4 = 0;
        let op = trial.op.expect("scoreTrialDown: down trial has an op");
        let opc = self.data.obank().get(op).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY);

        // Pure metatype→score arms: the leaf table answers these directly.
        if let Some(s) = score_tables::score_trial_down_pure(opc, meta, trial.inslot) {
            score = s;
        } else {
            // Driver-handled arms (recursion / live varnode read).
            match opc {
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => {
                    res_type = Some(Rc::clone(&trial.fit_type)); // No score, but propagate
                    max_length = trial.max_length;
                }
                OpCode::CPUI_LOAD => {
                    let out = self.data.obank().get(op).and_then(|o| o.get_out());
                    if let Some(outvn) = out {
                        let vn_size = self.data.vbank().get(outvn).map(|v| v.get_size()).unwrap_or(0);
                        let dr = deref_pointer(&trial.fit_type, trial.max_length, vn_size, self.typegrp)?;
                        score = dr.score;
                        res_type = dr.res_type;
                    }
                }
                OpCode::CPUI_STORE => {
                    if trial.inslot == 1 {
                        // Datatype *ptrto = derefPointer(trial,op->getIn(2),score);
                        let in2 = self.data.obank().get(op).and_then(|o| o.get_in(2));
                        if let Some(in2vn) = in2 {
                            let vn_size =
                                self.data.vbank().get(in2vn).map(|v| v.get_size()).unwrap_or(0);
                            let dr = deref_pointer(
                                &trial.fit_type,
                                trial.max_length,
                                vn_size,
                                self.typegrp,
                            )?;
                            score = dr.score;
                            if let Some(ptrto) = dr.res_type {
                                if !last_level {
                                    self.new_trials(op, 2, ptrto, trial.score_index, 0);
                                }
                            }
                        }
                    } else if trial.inslot == 2 {
                        if meta == type_metatype::TYPE_CODE {
                            score = -5;
                        } else {
                            // Datatype *storePtr = op->getIn(1)->getTypeReadFacing(op);
                            let in1 = self.data.obank().get(op).and_then(|o| o.get_in(1));
                            if let Some(in1vn) = in1 {
                                let store_ptr = self.data.vn_type_read_facing(in1vn, op);
                                if !last_level && store_ptr.get_metatype() == type_metatype::TYPE_PTR
                                {
                                    let ws = store_ptr.get_word_size().unwrap_or(1);
                                    let ptr = self.typegrp.get_type_pointer_strip_array(
                                        store_ptr.get_size(),
                                        Rc::clone(&trial.fit_type),
                                        ws,
                                    )?;
                                    self.new_trials(
                                        op,
                                        1,
                                        ptr,
                                        trial.score_index,
                                        trial.fit_type.get_size(),
                                    );
                                }
                            }
                            score = 1;
                        }
                    }
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLOTHER => {
                    if trial.inslot > 0 {
                        let facts = self.callspecs_facts(op);
                        score =
                            score_parameter(&trial.fit_type, facts.as_ref(), trial.inslot - 1);
                    }
                }
                OpCode::CPUI_CALLIND => {
                    if trial.inslot == 0 {
                        if meta == type_metatype::TYPE_PTR {
                            let ptrto = trial.fit_type.get_ptr_to();
                            match ptrto {
                                Some(p) if p.get_metatype() == type_metatype::TYPE_CODE => {
                                    score = 10;
                                }
                                _ => {
                                    score = -10;
                                }
                            }
                        }
                    } else {
                        let facts = self.callspecs_facts(op);
                        score =
                            score_parameter(&trial.fit_type, facts.as_ref(), trial.inslot - 1);
                    }
                }
                OpCode::CPUI_RETURN => {
                    let facts = self.funcproto_facts();
                    score = score_return_type(&trial.fit_type, Some(&facts));
                }
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRSUB => {
                    let (s, rt, ml) = self.score_add_down(trial, op, meta)?;
                    score = s;
                    res_type = rt;
                    max_length = ml;
                }
                OpCode::CPUI_SUBPIECE => {
                    // int4 offset = TypeOpSubpiece::computeByteOffsetForComposite(op);
                    let offset = self.subpiece_composite_byte_offset(op);
                    let out = self.data.obank().get(op).and_then(|o| o.get_out());
                    let vn_size = out
                        .and_then(|v| self.data.vbank().get(v))
                        .map(|v| v.get_size())
                        .unwrap_or(0);
                    let (s, rt) = score_truncation(
                        &trial.fit_type,
                        vn_size,
                        offset,
                        Some(self.result.get_base()),
                    )?;
                    score = s;
                    res_type = rt;
                }
                OpCode::CPUI_PTRADD => {
                    let (s, rt) = self.score_ptradd_down(trial, op, meta)?;
                    score = s;
                    res_type = rt;
                }
                _ => {
                    score = -10; // Doesn't fit (the leaf table default)
                }
            }
        }
        // scores[trial.scoreIndex] += score;
        self.scores[trial.score_index as usize] += score;
        // if (resType != 0 && !lastLevel) newTrialsDown(op->getOut(), resType, ...)
        if let Some(rt) = res_type {
            if !last_level {
                if let Some(outvn) = self.data.obank().get(op).and_then(|o| o.get_out()) {
                    self.new_trials_down(outvn, rt, trial.score_index, max_length);
                }
            }
        }
        Ok(())
    }

    /// The INT_ADD/INT_SUB/PTRSUB pointer-arithmetic arm of `scoreTrialDown`
    /// (unionresolve.cc:480-543).  Returns `(score, resType, maxLength)`.
    fn score_add_down(
        &self,
        trial: &Trial,
        op: OpId,
        meta: type_metatype,
    ) -> KunaResult<(int4, Option<Rc<Datatype>>, int4)> {
        use type_metatype::*;
        let mut score: int4 = 0;
        let mut res_type: Option<Rc<Datatype>> = None;
        let mut max_length: int4 = 0;
        if meta == TYPE_PTR {
            if trial.inslot >= 0 {
                // Varnode *vn = op->getIn(1-inslot);
                let vn = self.data.obank().get(op).and_then(|o| o.get_in(1 - trial.inslot));
                let vn = match vn {
                    Some(v) => v,
                    None => return Ok((0, None, 0)),
                };
                let vref = self.data.vbank().get(vn);
                let is_const = vref.map(|v| v.is_constant()).unwrap_or(false);
                if is_const {
                    let vsize = vref.map(|v| v.get_size()).unwrap_or(0);
                    let voff = vref.map(|v| v.get_offset()).unwrap_or(0);
                    // TypePointer *baseType = (TypePointer *)trial.fitType;
                    let mut base_type: Option<Rc<Datatype>> = Some(Rc::clone(&trial.fit_type));
                    // int8 off = sign_extend(vn->getOffset(),vn->getSize()*8-1);
                    // cast: the C++ `sign_extend(uintb,int4)` takes the raw bit
                    // pattern; `voff as i64` reinterprets the same bits (the value
                    // is a constant Varnode offset, sign-extended next).
                    let mut off: int8 = sign_extend(voff as i64, vsize * 8 - 1);
                    if trial.max_length != 0 {
                        if off < 0 || off >= trial.max_length as int8 {
                            base_type = None;
                            score = -1; // Offset doesn't land in allowed range
                        } else {
                            max_length = trial.max_length;
                        }
                    }
                    // while(off != 0 && baseType != 0) baseType = baseType->downChain(...)
                    while off != 0 {
                        let bt = match &base_type {
                            Some(b) => Rc::clone(b),
                            None => break,
                        };
                        // downChain(off,par,parOff,true,typegrp) — allow_array_wrap = true.
                        let (next, newoff, _par, _paroff) =
                            self.typegrp.down_chain(&bt, off, true)?;
                        off = newoff;
                        base_type = next;
                    }
                    if let Some(bt) = &base_type {
                        // resType = baseType;
                        // maxLength = baseType->getPtrTo()->getSize();
                        // score = 5;
                        let ptr_sz = bt.get_ptr_to().map(|p| p.get_size()).unwrap_or(0);
                        res_type = Some(Rc::clone(bt));
                        max_length = ptr_sz;
                        score = 5;
                    }
                } else {
                    // Non-constant index.
                    if trial.max_length != 0 {
                        score = 1;
                        let mut el_size: int4 = 1;
                        if vref.map(|v| v.is_written()).unwrap_or(false) {
                            let mult_op = vref.and_then(|v| v.get_def());
                            if let Some(mo) = mult_op {
                                if self.data.obank().get(mo).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY)
                                    == OpCode::CPUI_INT_MULT
                                {
                                    let mult_vn = self.data.obank().get(mo).and_then(|o| o.get_in(1));
                                    if let Some(mvn) = mult_vn {
                                        if let Some(mv) = self.data.vbank().get(mvn) {
                                            if mv.is_constant() {
                                                el_size = mv.get_offset() as int4;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // if (baseType->getPtrTo()->getAlignSize() == elSize) { ... }
                        let align =
                            trial.fit_type.get_ptr_to().map(|p| p.get_align_size()).unwrap_or(-1);
                        if align == el_size {
                            if el_size == trial.max_length {
                                score = 2; // Only one element available, variable index
                            } else {
                                score = 5;
                            }
                            res_type = Some(Rc::clone(&trial.fit_type));
                            max_length = trial.max_length;
                        }
                    } else {
                        score = 5; // Indexing into something that is not an array
                    }
                }
            }
        } else if meta == TYPE_INT || meta == TYPE_UINT || meta == TYPE_UNKNOWN {
            score = 1;
            res_type = Some(Rc::clone(&trial.fit_type));
        } else if meta == TYPE_BOOL {
            score = -3;
        } else {
            score = -5; // ARRAY STRUCT UNION CODE FLOAT
        }
        Ok((score, res_type, max_length))
    }

    /// The PTRADD arm of `scoreTrialDown` (unionresolve.cc:677-694).
    fn score_ptradd_down(
        &self,
        trial: &Trial,
        op: OpId,
        meta: type_metatype,
    ) -> KunaResult<(int4, Option<Rc<Datatype>>)> {
        use type_metatype::*;
        let mut score: int4 = 0;
        let mut res_type: Option<Rc<Datatype>> = None;
        if meta == TYPE_PTR {
            if trial.inslot == 0 {
                // Datatype *ptrto = ((TypePointer *)trial.fitType)->getPtrTo();
                // if (ptrto->getAlignSize() == op->getIn(2)->getOffset()) ...
                let align = trial.fit_type.get_ptr_to().map(|p| p.get_align_size()).unwrap_or(-1);
                let in2off = self
                    .data
                    .obank()
                    .get(op)
                    .and_then(|o| o.get_in(2))
                    .and_then(|v| self.data.vbank().get(v))
                    .map(|v| v.get_offset())
                    .unwrap_or(0);
                if align as int8 == in2off as int8 {
                    score = 10;
                    res_type = Some(Rc::clone(&trial.fit_type));
                }
            } else {
                score = -10;
            }
        } else if meta == TYPE_ARRAY
            || meta == TYPE_STRUCT
            || meta == TYPE_UNION
            || meta == TYPE_CODE
            || meta == TYPE_FLOAT
        {
            score = -5;
        } else {
            score = 1;
        }
        Ok((score, res_type))
    }

    /// C++ `ScoreUnionFields::scoreTrialUp` (unionresolve.cc:719-921).
    fn score_trial_up(&mut self, trial: &Trial, last_level: bool) -> KunaResult<()> {
        if trial.direction == DirType::Down {
            return Ok(()); // Trial doesn't push in this direction
        }
        let is_written = self.data.vbank().get(trial.vn).map(|v| v.is_written()).unwrap_or(false);
        if !is_written {
            // if (trial.vn->isConstant()) scoreConstantFit(trial);
            if self.data.vbank().get(trial.vn).map(|v| v.is_constant()).unwrap_or(false) {
                self.score_constant_fit(trial);
            }
            return Ok(()); // Nothing to propagate up through
        }
        let mut res_type: Option<Rc<Datatype>> = None;
        let mut max_length: int4 = 0;
        let mut newslot: int4 = 0;
        let mut score: int4 = 0;
        let meta = trial.fit_type.get_metatype();
        let def = self.data.vbank().get(trial.vn).and_then(|v| v.get_def());
        let def = match def {
            Some(d) => d,
            None => return Ok(()),
        };
        let def_opc = self.data.obank().get(def).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY);

        if let Some(s) = score_tables::score_trial_up_pure(def_opc, meta, trial.fit_type.get_size())
        {
            score = s;
        } else {
            match def_opc {
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => {
                    res_type = Some(Rc::clone(&trial.fit_type)); // propagate
                    max_length = trial.max_length;
                    newslot = 0;
                }
                OpCode::CPUI_LOAD => {
                    // resType = getTypePointerStripArray(def->getIn(1)->getSize(),fitType,1);
                    let in1sz = self
                        .data
                        .obank()
                        .get(def)
                        .and_then(|o| o.get_in(1))
                        .and_then(|v| self.data.vbank().get(v))
                        .map(|v| v.get_size())
                        .unwrap_or(0);
                    res_type = Some(self.typegrp.get_type_pointer_strip_array(
                        in1sz,
                        Rc::clone(&trial.fit_type),
                        1,
                    )?);
                    max_length = trial.fit_type.get_size();
                    newslot = 1;
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLOTHER | OpCode::CPUI_CALLIND => {
                    let facts = self.callspecs_facts(def);
                    score = score_return_type(&trial.fit_type, facts.as_ref());
                }
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRSUB => {
                    score = self.score_add_up(trial, def, meta);
                }
                OpCode::CPUI_SUBPIECE => {
                    // unionresolve.cc:890-898 — live read of def->getIn(1)->getOffset().
                    use type_metatype::*;
                    if meta == TYPE_INT || meta == TYPE_UINT || meta == TYPE_BOOL {
                        let in1off = self
                            .data
                            .obank()
                            .get(def)
                            .and_then(|o| o.get_in(1))
                            .and_then(|v| self.data.vbank().get(v))
                            .map(|v| v.get_offset())
                            .unwrap_or(0);
                        if in1off == 0 {
                            score = 3; // Likely truncation
                        } else {
                            score = 1;
                        }
                    } else {
                        score = -5;
                    }
                }
                OpCode::CPUI_PTRADD => {
                    // unionresolve.cc:900-912 — live read of def->getIn(2)->getOffset().
                    use type_metatype::*;
                    if meta == TYPE_PTR {
                        // Datatype *ptrto = ((TypePointer *)trial.fitType)->getPtrTo();
                        // if (ptrto->getAlignSize() == def->getIn(2)->getOffset())
                        let align =
                            trial.fit_type.get_ptr_to().map(|p| p.get_align_size()).unwrap_or(-1);
                        let in2off = self
                            .data
                            .obank()
                            .get(def)
                            .and_then(|o| o.get_in(2))
                            .and_then(|v| self.data.vbank().get(v))
                            .map(|v| v.get_offset())
                            .unwrap_or(0);
                        // C++ compares `int4 alignSize == uintb offset` under
                        // -Wno-sign-compare (the int4 promotes to uintb); mirror the
                        // down-PTRADD arm (score_ptradd_down) which casts both to int8.
                        if align as int8 == in2off as int8 {
                            score = 10;
                        } else {
                            score = 2;
                        }
                    } else if meta == TYPE_ARRAY
                        || meta == TYPE_STRUCT
                        || meta == TYPE_UNION
                        || meta == TYPE_CODE
                        || meta == TYPE_FLOAT
                    {
                        score = -5;
                    } else {
                        score = 1;
                    }
                }
                _ => {
                    score = -10; // Datatype doesn't fit (leaf table default)
                }
            }
        }
        self.scores[trial.score_index as usize] += score;
        if let Some(rt) = res_type {
            if !last_level {
                self.new_trials(def, newslot, rt, trial.score_index, max_length);
            }
        }
        Ok(())
    }

    /// The INT_ADD/INT_SUB/PTRSUB arm of `scoreTrialUp` (unionresolve.cc:778-796).
    fn score_add_up(&self, trial: &Trial, def: OpId, meta: type_metatype) -> int4 {
        use type_metatype::*;
        if meta == TYPE_PTR {
            let mut score = 5;
            // if (trial.maxLength == 0 && def->getIn(1)->isConstant()) { ... }
            if trial.max_length == 0 {
                let in1 = self.data.obank().get(def).and_then(|o| o.get_in(1));
                if let Some(in1vn) = in1 {
                    if let Some(v) = self.data.vbank().get(in1vn) {
                        if v.is_constant() {
                            // Datatype *ptrto = ((TypePointer *)fitType)->getPtrTo();
                            // if (ptrto->getAlignSize() == (int4)def->getIn(1)->getOffset()) score += 1;
                            let align =
                                trial.fit_type.get_ptr_to().map(|p| p.get_align_size()).unwrap_or(-1);
                            if align == v.get_offset() as int4 {
                                score += 1;
                            }
                        }
                    }
                }
            }
            score
        } else if meta == TYPE_INT || meta == TYPE_UINT {
            5 // Don't try to back up further
        } else if meta == TYPE_ARRAY
            || meta == TYPE_STRUCT
            || meta == TYPE_UNION
            || meta == TYPE_CODE
            || meta == TYPE_FLOAT
        {
            -5
        } else {
            1 // BOOL UNKNOWN
        }
    }

    /// C++ `ScoreUnionFields::scoreConstantFit` (unionresolve.cc:982-1031).
    fn score_constant_fit(&mut self, trial: &Trial) {
        let size = self.data.vbank().get(trial.vn).map(|v| v.get_size()).unwrap_or(0);
        let val = self.data.vbank().get(trial.vn).map(|v| v.get_offset()).unwrap_or(0);
        let meta = trial.fit_type.get_metatype();
        let facts = self.constant_facts(size, val, meta);
        let score = score_constant_fit(size, val, meta, &facts);
        self.scores[trial.score_index as usize] += score;
    }

    /// C++ `ScoreUnionFields::scoreTruncation` (unionresolve.cc:931-977): score
    /// the implied/explicit truncation and accumulate into `scores`.  Returns the
    /// data-type to recurse (or `None`).  Used by constructors 2 and 3.
    fn score_truncation_driver(
        &mut self,
        ct: &Rc<Datatype>,
        vn: VarnodeId,
        offset: int4,
        score_index: int4,
    ) -> KunaResult<Option<Rc<Datatype>>> {
        let vn_size = self.data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
        let (score, res) = score_truncation(ct, vn_size, offset, Some(self.result.get_base()))?;
        self.scores[score_index as usize] += score;
        Ok(res)
    }

    /// C++ `TypeOpSubpiece::computeByteOffsetForComposite(op)`
    /// (typeop.cc:2197-2209): the byte offset of a SUBPIECE truncation into an
    /// assumed composite, by endianness.  Transcribed here (rather than reaching
    /// into the Casts stage) so this module stays self-contained.
    fn subpiece_composite_byte_offset(&self, op: OpId) -> int4 {
        let o = match self.data.obank().get(op) {
            Some(o) => o,
            None => return 0,
        };
        // int4 outSize = op->getOut()->getSize();
        let out_size = o
            .get_out()
            .and_then(|v| self.data.vbank().get(v))
            .map(|v| v.get_size())
            .unwrap_or(0);
        // int4 lsb = (int4)op->getIn(1)->getOffset();
        let lsb =
            o.get_in(1).and_then(|v| self.data.vbank().get(v)).map(|v| v.get_offset()).unwrap_or(0)
                as int4;
        // const Varnode *vn = op->getIn(0);
        let in0 = match o.get_in(0) {
            Some(v) => v,
            None => return lsb,
        };
        let in0v = match self.data.vbank().get(in0) {
            Some(v) => v,
            None => return lsb,
        };
        if in0v.get_space().is_big_endian() {
            // byteOff = vn->getSize() - outSize - lsb;
            in0v.get_size() - out_size - lsb
        } else {
            lsb
        }
    }

    // -- runOneLevel / run / computeBestIndex (unionresolve.cc:1036-1085) ------

    /// C++ `ScoreUnionFields::runOneLevel` (unionresolve.cc:1036-1048).
    fn run_one_level(&mut self, last_pass: bool) -> KunaResult<()> {
        // The C++ iterates `trialCurrent` by const-iterator; scoring appends to
        // `trialNext` (never `trialCurrent`), so a snapshot of indices is faithful.
        let current = std::mem::take(&mut self.trial_current);
        for trial in &current {
            self.trial_count += 1;
            if self.trial_count > MAX_TRIALS {
                self.trial_current = current; // restore (run() may not swap)
                return Ok(()); // Absolute number of trials reached
            }
            self.score_trial_down(trial, last_pass)?;
            self.score_trial_up(trial, last_pass)?;
        }
        self.trial_current = current;
        Ok(())
    }

    /// C++ `ScoreUnionFields::computeBestIndex` (unionresolve.cc:1050-1063).
    fn compute_best_index(&mut self) {
        let (field_num, best_index) = compute_best_index(&self.scores);
        // result.fieldNum = bestIndex - 1; result.resolve = fields[bestIndex];
        self.result.set_field_num(field_num);
        self.result.set_resolve(Rc::clone(&self.fields[best_index as usize]));
    }

    /// C++ `ScoreUnionFields::run` (unionresolve.cc:1068-1085).
    fn run(&mut self) -> KunaResult<()> {
        self.trial_count = 0;
        for pass in 0..MAX_PASSES {
            if self.trial_current.is_empty() {
                break;
            }
            if self.trial_count > THRESHOLD {
                break; // Threshold reached
            }
            if pass + 1 == MAX_PASSES {
                self.run_one_level(true)?;
            } else {
                self.run_one_level(false)?;
                // trialCurrent.swap(trialNext); trialNext.clear();
                std::mem::swap(&mut self.trial_current, &mut self.trial_next);
                self.trial_next.clear();
            }
        }
        Ok(())
    }

    // -- Constructors (unionresolve.cc:1095-1223) -----------------------------

    /// C++ constructor 1: `ScoreUnionFields(Funcdata&,Datatype*,PcodeOp*,int4)`
    /// (unionresolve.cc:1095-1148).  This is the data-flow-edge form driving the
    /// pointer-to-union and union read/write resolutions in `union_datatype.xml`.
    pub fn new_edge(
        data: &'a Funcdata,
        typegrp: &'a crate::dtype::TypeFactoryImpl,
        parent_type: Rc<Datatype>,
        op: OpId,
        slot: int4,
    ) -> KunaResult<ScoreUnionFields<'a>> {
        let result = ResolvedUnion::new(Rc::clone(&parent_type));
        let mut s = ScoreUnionFields {
            data,
            typegrp,
            scores: Vec::new(),
            fields: Vec::new(),
            visited: std::collections::BTreeSet::new(),
            trial_current: Vec::new(),
            trial_next: Vec::new(),
            result,
            trial_count: 0,
        };
        // if (testSimpleCases(op, slot, parentType)) return;
        if s.test_simple_cases(op, slot, &parent_type) {
            return Ok(s);
        }
        // int4 wordSize = (parentType->getMetatype()==TYPE_PTR) ? getWordSize() : 0;
        let word_size: int4 = if parent_type.get_metatype() == type_metatype::TYPE_PTR {
            parent_type.get_word_size().unwrap_or(0) as int4
        } else {
            0
        };
        let num_fields = s.result.get_base().num_depend();
        s.scores = vec![0; (num_fields + 1) as usize];
        s.fields = vec![Rc::clone(&parent_type); (num_fields + 1) as usize];
        let parent_size = parent_type.get_size();
        // Varnode *vn = (slot<0) ? op->getOut() : op->getIn(slot);
        let vn = if slot < 0 {
            s.data.obank().get(op).and_then(|o| o.get_out())
        } else {
            s.data.obank().get(op).and_then(|o| o.get_in(slot))
        };
        let vn = match vn {
            Some(v) => v,
            None => return Ok(s),
        };
        let vn_size = s.data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
        if slot < 0 {
            if vn_size != parent_size {
                s.scores[0] -= 10;
            } else {
                s.trial_current.push(Trial::new_up(vn, Rc::clone(&parent_type), 0, 0));
            }
        } else if vn_size != parent_size {
            s.scores[0] -= 10;
        } else {
            s.trial_current.push(Trial::new_down(s.data, op, slot, Rc::clone(&parent_type), 0, 0));
        }
        s.fields[0] = Rc::clone(&parent_type);
        let opc = s.data.obank().get(op).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY);
        // if (wordSize == 0 || opc in {INT_ADD,PTRADD,PTRSUB,STORE,LOAD}) scores[0] -= 1;
        if word_size == 0
            || opc == OpCode::CPUI_INT_ADD
            || opc == OpCode::CPUI_PTRADD
            || opc == OpCode::CPUI_PTRSUB
            || opc == OpCode::CPUI_STORE
            || opc == OpCode::CPUI_LOAD
        {
            s.scores[0] -= 1; // Prefer a resolution from a specific field
        }
        s.visited.insert(VisitMark(vn, 0));
        for i in 0..num_fields {
            // Datatype *fieldType = result.baseType->getDepend(i);
            let mut field_type = match s.result.get_base().get_depend(i) {
                Some(f) => f,
                None => continue,
            };
            let mut max_length: int4 = 0;
            if word_size != 0 {
                // If this is a pointer
                if field_type.get_metatype() == type_metatype::TYPE_ARRAY {
                    max_length = field_type.get_size(); // Keep track of original array size
                } else if field_type.get_size() < s.result.get_base().get_size() {
                    max_length = field_type.get_size();
                }
                // fieldType = getTypePointerStripArray(parentType->getSize(),fieldType,wordSize);
                field_type = s.typegrp.get_type_pointer_strip_array(
                    parent_size,
                    field_type,
                    word_size as u32,
                )?;
            }
            if vn_size != field_type.get_size() {
                s.scores[(i + 1) as usize] -= 10; // size mismatch, don't create trial
            } else if slot < 0 {
                s.trial_current.push(Trial::new_up(vn, Rc::clone(&field_type), i + 1, max_length));
            } else {
                s.trial_current.push(Trial::new_down(
                    s.data,
                    op,
                    slot,
                    Rc::clone(&field_type),
                    i + 1,
                    max_length,
                ));
            }
            s.fields[(i + 1) as usize] = field_type;
            s.visited.insert(VisitMark(vn, i + 1));
        }
        s.run()?;
        s.compute_best_index();
        Ok(s)
    }

    /// C++ constructor 2: `ScoreUnionFields(Funcdata&,TypeUnion*,int4,PcodeOp*)`
    /// (unionresolve.cc:1161-1184).  The SUBPIECE form.
    pub fn new_subpiece(
        data: &'a Funcdata,
        typegrp: &'a crate::dtype::TypeFactoryImpl,
        union_type: Rc<Datatype>,
        offset: int4,
        op: OpId,
    ) -> KunaResult<ScoreUnionFields<'a>> {
        let result = ResolvedUnion::new(Rc::clone(&union_type));
        let mut s = ScoreUnionFields {
            data,
            typegrp,
            scores: Vec::new(),
            fields: Vec::new(),
            visited: std::collections::BTreeSet::new(),
            trial_current: Vec::new(),
            trial_next: Vec::new(),
            result,
            trial_count: 0,
        };
        let vn = match s.data.obank().get(op).and_then(|o| o.get_out()) {
            Some(v) => v,
            None => return Ok(s),
        };
        let vn_size = s.data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
        let num_fields = union_type.num_depend();
        s.scores = vec![0; (num_fields + 1) as usize];
        s.fields = vec![Rc::clone(&union_type); (num_fields + 1) as usize];
        s.fields[0] = Rc::clone(&union_type);
        s.scores[0] = -10;
        for i in 0..num_fields {
            let union_field = match union_type.get_field(i) {
                Some(f) => f,
                None => continue,
            };
            let field_type = Rc::clone(&union_field.field_type);
            let field_offset = union_field.offset;
            s.fields[(i + 1) as usize] = Rc::clone(&field_type);
            // if (field->type->getSize() != vn->getSize() || field->offset != offset) {score=-10; continue;}
            if field_type.get_size() != vn_size || field_offset != offset {
                s.scores[(i + 1) as usize] = -10;
                continue;
            }
            s.new_trials_down(vn, field_type, i + 1, 0);
        }
        // trialCurrent.swap(trialNext);
        std::mem::swap(&mut s.trial_current, &mut s.trial_next);
        s.trial_next.clear();
        if s.trial_current.len() > 1 {
            s.run()?;
        }
        s.compute_best_index();
        // result.resolve = typegrp.getExactPiece(result.resolve, offset, vn->getSize());
        Self::finalize_exact_piece(&mut s, offset, vn_size)?;
        Ok(s)
    }

    /// C++ constructor 3:
    /// `ScoreUnionFields(Funcdata&,TypeUnion*,int4,PcodeOp*,int4)`
    /// (unionresolve.cc:1195-1223).  The implied-truncation form.
    pub fn new_truncation(
        data: &'a Funcdata,
        typegrp: &'a crate::dtype::TypeFactoryImpl,
        union_type: Rc<Datatype>,
        offset: int4,
        op: OpId,
        slot: int4,
    ) -> KunaResult<ScoreUnionFields<'a>> {
        let result = ResolvedUnion::new(Rc::clone(&union_type));
        let mut s = ScoreUnionFields {
            data,
            typegrp,
            scores: Vec::new(),
            fields: Vec::new(),
            visited: std::collections::BTreeSet::new(),
            trial_current: Vec::new(),
            trial_next: Vec::new(),
            result,
            trial_count: 0,
        };
        // if (testSimpleCases(op, slot, unionType)) return;
        if s.test_simple_cases(op, slot, &union_type) {
            return Ok(s);
        }
        let vn = if slot < 0 {
            s.data.obank().get(op).and_then(|o| o.get_out())
        } else {
            s.data.obank().get(op).and_then(|o| o.get_in(slot))
        };
        let vn = match vn {
            Some(v) => v,
            None => return Ok(s),
        };
        let vn_size = s.data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
        let num_fields = union_type.num_depend();
        s.scores = vec![0; (num_fields + 1) as usize];
        s.fields = vec![Rc::clone(&union_type); (num_fields + 1) as usize];
        s.fields[0] = Rc::clone(&union_type);
        s.scores[0] = -10; // entire union is not a good fit
        for i in 0..num_fields {
            let union_field = match union_type.get_field(i) {
                Some(f) => f,
                None => continue,
            };
            let field_type = Rc::clone(&union_field.field_type);
            let field_offset = union_field.offset;
            s.fields[(i + 1) as usize] = Rc::clone(&field_type);
            // Datatype *ct = scoreTruncation(field->type,vn,offset-field->offset,i+1);
            let ct = s.score_truncation_driver(&field_type, vn, offset - field_offset, i + 1)?;
            if let Some(ct) = ct {
                if slot < 0 {
                    s.trial_current.push(Trial::new_up(vn, ct, i + 1, 0)); // flow backward
                } else {
                    s.trial_current.push(Trial::new_down(s.data, op, slot, ct, i + 1, 0)); // downward
                }
                s.visited.insert(VisitMark(vn, i + 1));
            }
        }
        if s.trial_current.len() > 1 {
            s.run()?;
        }
        s.compute_best_index();
        Self::finalize_exact_piece(&mut s, offset, vn_size)?;
        Ok(s)
    }

    /// `result.resolve = typegrp.getExactPiece(result.resolve, offset, vn->getSize());`
    /// (the tail of constructors 2 and 3 — unionresolve.cc:1183, 1222).
    ///
    /// LOSS (faithful divergence, cited): C++ assigns the `getExactPiece` result
    /// *unconditionally*, so when `getExactPiece` returns null (type.cc:4579-4609,
    /// e.g. no sub-piece of the requested size) C++ leaves `result.resolve ==
    /// nullptr` and caches that.  The landed `ResolvedUnion.resolve` field is a
    /// non-nullable `Rc<Datatype>` (unionresolve.rs:84) — making it `Option` would
    /// ripple through every `get_datatype()` consumer (funcdata_facing /
    /// coreaction_casts).  So on the `None` arm we keep the prior `resolve` rather
    /// than store null.  Unobservable on the live paths: constructor 2 (SUBPIECE)
    /// always hits `getExactPiece`'s perfect-size-match early return (the winning
    /// field already has `size == vn->getSize()`), and the only consumer of an
    /// implied-truncation edge, `resolveTruncation`, returns `getField(fieldNum)`
    /// and ignores `getDatatype()`.  Divergence surfaces only if a later
    /// `getUnionField(...)->getDatatype()` reads this exact cached edge.
    fn finalize_exact_piece(
        s: &mut ScoreUnionFields<'a>,
        offset: int4,
        vn_size: int4,
    ) -> KunaResult<()> {
        let cur = Rc::clone(s.result.get_datatype());
        if let Some(piece) = s.typegrp.get_exact_piece(cur, offset, vn_size)? {
            s.result.set_resolve(piece);
        }
        Ok(())
    }
}
