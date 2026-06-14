//! `AddTreeState` — the pointer-arithmetic add-tree analysis driving
//! `RulePtrArith` (C++ `ruleaction.cc:5996-6560`, `ruleaction.hh:AddTreeState`).
//!
//! A string of `INT_ADD`s rooted at a Varnode of known pointer type is split into
//! a *multiple* component (terms that are multiples of the pointed-to data-type
//! size, which become a `PTRADD`), an *offset* into a sub data-type (which becomes
//! a `PTRSUB`), and a *non-multiple* remainder (re-added with `INT_ADD`).  The
//! whole `buildTree` rewrite preserves the original output Varnode and destroys
//! the root `INT_ADD`.
//!
//! ## Faithfulness
//!
//! This is a statement-for-statement transcription of the C++ `AddTreeState`
//! methods (`clear`/`initAlternateForm`/ctor/`hasMatchingSubType`/`checkMultTerm`/
//! `checkTerm`/`spanAddTree`/`calcSubtype`/`assignPropagatedType`/`buildMultiples`/
//! `buildExtra`/`buildDegenerate`/`apply`/`buildTree`).  `getTypeReadFacing(op)`
//! reduces to `Varnode::getType()` for the non-union corpus (union mid-flow
//! resolution is the deferred W8 surface); the `needsResolution` union branches in
//! `buildTree` (`forceFacingType`/`inheritUnionField`) are guarded by
//! `needs_resolution()`, which is `false` for plain pointers — faithful no-ops
//! there.  The `warningHeader`/`warning` emit a comment (a print-markup surface,
//! not part of the transform); those are SEAM-noted no-ops, only reachable on a
//! distribution failure that does not occur on the targeted corpus.

use std::rc::Rc;

use kuna_base::address::{calc_mask, sign_extend, uintb_negate};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, int8, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};

/// The mutable analysis state for one `RulePtrArith` application
/// (C++ `class AddTreeState`).
pub struct AddTreeState<'a> {
    data: &'a mut Funcdata,
    /// The root ADD op (C++ `baseOp`).
    base_op: OpId,
    /// The slot of the pointer in the root op (C++ `baseSlot`).  Read by the C++
    /// `buildTree` union path (`inheritUnionField(..., baseOp, baseSlot)`), which
    /// is a no-op for plain pointers here (SEAM(W8 union)); kept for faithful
    /// restoration when union mid-flow resolution lands.
    #[allow(dead_code)]
    base_slot: int4,
    /// The pointer Varnode (C++ `ptr`).
    ptr: VarnodeId,
    /// The pointer's data-type (C++ `ct`, a TypePointer).
    ct: Rc<Datatype>,
    /// The base data-type pointed to (C++ `baseType`).
    base_type: Rc<Datatype>,
    /// Non-null when `ct` is a formal relative pointer (C++ `pRelType`).
    p_rel_type: Option<Rc<Datatype>>,
    /// Size of the pointer Varnode in bytes (C++ `ptrsize`).
    ptrsize: int4,
    /// Mask covering the pointer's bytes (C++ `ptrmask`).
    ptrmask: uintb,
    /// Size of the base data-type in address units (C++ `size`).
    size: int8,
    /// Sum of multiple constants (C++ `multsum`).
    multsum: uintb,
    /// Sum of non-multiple constants (C++ `nonmultsum`).
    nonmultsum: uintb,
    /// Biggest non-multiple coefficient seen (C++ `biggestNonMultCoeff`).
    biggest_non_mult_coeff: uintb,
    /// Non-constant multiple terms (C++ `multiple`).
    multiple: Vec<VarnodeId>,
    /// Their coefficients (C++ `coeff`).
    coeff: Vec<int8>,
    /// Non-multiple terms (C++ `nonmult`).
    nonmult: Vec<VarnodeId>,
    /// Op where a distribution may be needed (C++ `distributeOp`).
    distribute_op: Option<OpId>,
    /// Computed offset into a sub data-type (C++ `offset`).
    offset: uintb,
    /// Correction for double-counted non-multiple constants (C++ `correct`).
    correct: uintb,
    /// The base type's element is unit-sized (C++ `isDegenerate`).
    is_degenerate: bool,
    /// A subtype offset (`PTRSUB`) is being calculated (C++ `isSubtype`).
    is_subtype: bool,
    /// The analysis is still consistent (C++ `valid`).
    valid: bool,
    /// A distribute transform was used by the analysis (C++ `isDistributeUsed`).
    is_distribute_used: bool,
    /// Suppress distribution on this pass (C++ `preventDistribution`).
    prevent_distribution: bool,
}

impl<'a> AddTreeState<'a> {
    /// C++ `AddTreeState::AddTreeState(Funcdata&, PcodeOp*, int4)`.
    pub fn new(data: &'a mut Funcdata, op: OpId, slot: int4) -> Option<AddTreeState<'a>> {
        let base_op = op;
        let base_slot = slot;
        let ptr = data.obank().get(op)?.get_in(slot)?;
        // ct = (const TypePointer *)ptr->getTypeReadFacing(op);
        let ct = data.vbank().get(ptr)?.get_type_read_facing(op).clone();
        let ptrsize = data.vbank().get(ptr)?.get_size();
        let ptrmask = calc_mask(ptrsize);
        let mut base_type = ct.get_ptr_to()?;
        let mut nonmultsum: uintb = 0;
        let mut p_rel_type: Option<Rc<Datatype>> = None;
        // if (ct->isFormalPointerRel()) { pRelType = ct; baseType = parent; nonmultsum = addrOff; }
        if ct.is_formal_pointer_rel() {
            p_rel_type = Some(Rc::clone(&ct));
            base_type = ct.get_rel_parent()?;
            nonmultsum = (ct.get_address_offset()? as uintb) & ptrmask;
        }
        let word_size = ct.get_word_size().unwrap_or(1);
        // size = baseType->isVariableLength() ? 0 : byteToAddressInt(getAlignSize(),wordSize)
        let size: int8 = if base_type.is_variable_length() {
            0
        } else {
            AddrSpace::byte_to_address_int(base_type.get_align_size() as i64, word_size) as int8
        };
        // unitsize = addressToByteInt(1, wordSize);
        let unitsize = AddrSpace::address_to_byte_int(1, word_size);
        let align = base_type.get_align_size();
        let is_degenerate = (align as i64) <= unitsize && align > 0;
        Some(AddTreeState {
            data,
            base_op,
            base_slot,
            ptr,
            ct,
            base_type,
            p_rel_type,
            ptrsize,
            ptrmask,
            size,
            multsum: 0,
            nonmultsum,
            biggest_non_mult_coeff: 0,
            multiple: Vec::new(),
            coeff: Vec::new(),
            nonmult: Vec::new(),
            distribute_op: None,
            offset: 0,
            correct: 0,
            is_degenerate,
            is_subtype: false,
            valid: true,
            is_distribute_used: false,
            prevent_distribution: false,
        })
    }

    /// C++ `AddTreeState::clear()`.
    fn clear(&mut self) {
        self.multsum = 0;
        self.nonmultsum = 0;
        self.biggest_non_mult_coeff = 0;
        if let Some(rel) = &self.p_rel_type {
            // nonmultsum = ((TypePointerRel*)ct)->getAddressOffset() & ptrmask;
            self.nonmultsum =
                (rel.get_address_offset().unwrap_or(0) as uintb) & self.ptrmask;
        }
        self.multiple.clear();
        self.coeff.clear();
        self.nonmult.clear();
        self.correct = 0;
        self.offset = 0;
        self.valid = true;
        self.is_distribute_used = false;
        self.is_subtype = false;
        self.distribute_op = None;
    }

    /// C++ `AddTreeState::initAlternateForm()`.
    pub fn init_alternate_form(&mut self) -> bool {
        if self.p_rel_type.is_none() {
            return false;
        }
        self.p_rel_type = None;
        // baseType = ct->getPtrTo();
        self.base_type = match self.ct.get_ptr_to() {
            Some(b) => b,
            None => return false,
        };
        let word_size = self.ct.get_word_size().unwrap_or(1);
        self.size = if self.base_type.is_variable_length() {
            0
        } else {
            AddrSpace::byte_to_address_int(self.base_type.get_align_size() as i64, word_size) as int8
        };
        let unitsize = AddrSpace::address_to_byte_int(1, word_size);
        let align = self.base_type.get_align_size();
        self.is_degenerate = (align as i64) <= unitsize && align > 0;
        self.prevent_distribution = false;
        self.clear();
        true
    }

    // --- small IR accessors -------------------------------------------------

    fn vn_is_constant(&self, vn: VarnodeId) -> bool {
        self.data.vbank().get(vn).map(|v| v.is_constant()).unwrap_or(false)
    }
    fn vn_offset(&self, vn: VarnodeId) -> uintb {
        self.data.vbank().get(vn).map(|v| v.get_offset()).unwrap_or(0)
    }
    fn vn_size(&self, vn: VarnodeId) -> int4 {
        self.data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(1)
    }
    fn vn_is_written(&self, vn: VarnodeId) -> bool {
        self.data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false)
    }
    fn vn_is_free(&self, vn: VarnodeId) -> bool {
        self.data.vbank().get(vn).map(|v| v.is_free()).unwrap_or(true)
    }
    fn vn_def(&self, vn: VarnodeId) -> Option<OpId> {
        self.data.vbank().get(vn).and_then(|v| v.get_def())
    }
    fn op_code(&self, op: OpId) -> OpCode {
        self.data.obank().get(op).map(|o| o.code()).unwrap_or(OpCode::CPUI_MAX)
    }
    fn op_in(&self, op: OpId, slot: int4) -> VarnodeId {
        self.data
            .obank()
            .get(op)
            .and_then(|o| o.get_in(slot))
            .expect("addtreestate: null op input (C++ UB)")
    }

    /// C++ `AddTreeState::hasMatchingSubType(off, arrayHint, &newoff)`.
    fn has_matching_sub_type(&self, off: int8, array_hint: uintb) -> (bool, int8) {
        if array_hint == 0 {
            return match self.base_type.get_sub_type(off) {
                Ok((Some(_), newoff)) => (true, newoff),
                _ => (false, 0),
            };
        }
        // type_before = nearestArrayedComponentBackward(off,128,&offBefore,&elSizeBefore)
        let (type_before, off_before, el_size_before) = self
            .base_type
            .nearest_arrayed_component_backward(off, 128)
            .unwrap_or((-1, 0, 0));
        let (type_after, off_after, el_size_after) = self
            .base_type
            .nearest_arrayed_component_forward(off, 128)
            .unwrap_or((-1, 0, 0));
        if type_before < 0 && type_after < 0 {
            return match self.base_type.get_sub_type(off) {
                Ok((Some(_), newoff)) => (true, newoff),
                _ => (false, 0),
            };
        }
        if type_before < 0 {
            return (true, off_after); // Only array is after
        }
        if type_after < 0 {
            return (true, off_before); // Only array is before
        }
        if off_after == off_before {
            return (true, off_after);
        }
        // Array before and after the offset point.
        if array_hint != 1 && el_size_before != el_size_after {
            if el_size_before == array_hint as int8 {
                return (true, off_before);
            }
            if el_size_after == array_hint as int8 {
                return (true, off_after);
            }
        }
        if let Ok((Some(_), newoff)) = self.base_type.get_sub_type(off) {
            if newoff == off_before || newoff == off_after {
                return (true, newoff); // Offset is contained in an arrayed component
            }
        }
        let dist_before = if off_before < 0 { (-off_before) as uintb } else { off_before as uintb };
        let dist_after = if off_after < 0 { (-off_after) as uintb } else { off_after as uintb };
        let newoff = if dist_after < dist_before { off_after } else { off_before };
        (true, newoff)
    }

    /// C++ `AddTreeState::checkMultTerm(vn, op, treeCoeff)`.
    fn check_mult_term(&mut self, vn: VarnodeId, op: OpId, tree_coeff: uintb) -> bool {
        let vnconst = self.op_in(op, 1);
        let vnterm = self.op_in(op, 0);
        if self.vn_is_free(vnterm) {
            self.valid = false;
            return false;
        }
        if self.vn_is_constant(vnconst) {
            // val = (vnconst->getOffset() * treeCoeff) & ptrmask;
            let val = self.vn_offset(vnconst).wrapping_mul(tree_coeff) & self.ptrmask;
            let sval = sign_extend(val as i64, self.vn_size(vn) * 8 - 1);
            let rem = if self.size == 0 { sval } else { sval % self.size };
            if rem != 0 {
                if (val >= self.size as uintb) && self.size != 0 {
                    self.valid = false; // Size too big: pointer type must be wrong
                    return false;
                }
                if !self.prevent_distribution
                    && self.vn_is_written(vnterm)
                    && self.vn_def(vnterm).map(|d| self.op_code(d)) == Some(OpCode::CPUI_INT_ADD)
                {
                    if self.distribute_op.is_none() {
                        self.distribute_op = Some(op);
                    }
                    let def = self.vn_def(vnterm).expect("check_mult_term: vnterm def");
                    return self.span_add_tree(def, val);
                }
                let vncoeff = if sval < 0 { (-sval) as uintb } else { sval as uintb };
                if vncoeff > self.biggest_non_mult_coeff {
                    self.biggest_non_mult_coeff = vncoeff;
                }
                return true;
            } else {
                if tree_coeff != 1 {
                    self.is_distribute_used = true;
                }
                self.multiple.push(vnterm);
                self.coeff.push(sval);
                return false;
            }
        }
        if tree_coeff > self.biggest_non_mult_coeff {
            self.biggest_non_mult_coeff = tree_coeff;
        }
        true
    }

    /// C++ `AddTreeState::checkTerm(vn, treeCoeff)`.
    fn check_term(&mut self, vn: VarnodeId, tree_coeff: uintb) -> bool {
        if vn == self.ptr {
            return false;
        }
        if self.vn_is_constant(vn) {
            // val = vn->getOffset() * treeCoeff;  (NOT masked here in C++)
            let val = self.vn_offset(vn).wrapping_mul(tree_coeff);
            let sval = sign_extend(val as i64, self.vn_size(vn) * 8 - 1);
            let rem = if self.size == 0 { sval } else { sval % self.size };
            if rem != 0 {
                // constant is not a multiple of size
                if tree_coeff != 1 {
                    let meta = self.base_type.get_metatype();
                    if meta == type_metatype::TYPE_ARRAY || meta == type_metatype::TYPE_STRUCT {
                        self.is_distribute_used = true;
                    }
                }
                self.nonmultsum = self.nonmultsum.wrapping_add(val) & self.ptrmask;
                return true;
            }
            if tree_coeff != 1 {
                self.is_distribute_used = true;
            }
            self.multsum = self.multsum.wrapping_add(val) & self.ptrmask;
            return false;
        }
        if self.vn_is_written(vn) {
            let def = self.vn_def(vn).expect("check_term: written vn def");
            let code = self.op_code(def);
            if code == OpCode::CPUI_INT_ADD {
                return self.span_add_tree(def, tree_coeff);
            }
            if code == OpCode::CPUI_COPY {
                // Not finished reducing yet
                self.valid = false;
                return false;
            }
            if code == OpCode::CPUI_INT_MULT {
                return self.check_mult_term(vn, def, tree_coeff);
            }
        } else if self.vn_is_free(vn) {
            self.valid = false;
            return false;
        }
        if tree_coeff > self.biggest_non_mult_coeff {
            self.biggest_non_mult_coeff = tree_coeff;
        }
        true
    }

    /// C++ `AddTreeState::spanAddTree(op, treeCoeff)`.
    fn span_add_tree(&mut self, op: OpId, tree_coeff: uintb) -> bool {
        let in0 = self.op_in(op, 0);
        let in1 = self.op_in(op, 1);
        let one_is_non = self.check_term(in0, tree_coeff);
        if !self.valid {
            return false;
        }
        let two_is_non = self.check_term(in1, tree_coeff);
        if !self.valid {
            return false;
        }
        if self.p_rel_type.is_some()
            && (self.multsum != 0
                || self.nonmultsum >= self.size as uintb
                || !self.multiple.is_empty())
        {
            self.valid = false;
            return false;
        }
        if one_is_non && two_is_non {
            return true;
        }
        if one_is_non {
            self.nonmult.push(in0);
        }
        if two_is_non {
            self.nonmult.push(in1);
        }
        false // At least one side contains multiples
    }

    /// C++ `AddTreeState::calcSubtype()`.
    fn calc_subtype(&mut self) {
        let tmpoff = self.multsum.wrapping_add(self.nonmultsum) & self.ptrmask;
        if self.size == 0 || tmpoff < self.size as uintb {
            self.offset = tmpoff;
        } else {
            let stmpoff = sign_extend(tmpoff as i64, self.ptrsize * 8 - 1);
            let stmpoff = stmpoff % self.size;
            if stmpoff >= 0 {
                self.offset = stmpoff as uintb;
            } else if self.base_type.get_metatype() == type_metatype::TYPE_STRUCT
                && self.biggest_non_mult_coeff != 0
                && self.multsum == 0
            {
                self.offset = tmpoff;
            } else {
                self.offset = (stmpoff + self.size) as uintb;
            }
        }
        self.correct = self.nonmultsum;
        self.multsum = tmpoff.wrapping_sub(self.offset) & self.ptrmask;
        let word_size = self.ct.get_word_size().unwrap_or(1);
        if self.nonmult.is_empty() {
            if self.multsum == 0 && self.multiple.is_empty() {
                self.valid = false;
                return;
            }
            self.is_subtype = false;
        } else if self.base_type.get_metatype() == type_metatype::TYPE_SPACEBASE {
            // offsetbytes = addressToByteInt(offset, wordSize)
            let offsetbytes = AddrSpace::address_to_byte_int(self.offset as i64, word_size);
            let (matched, extra) = self.has_matching_sub_type(offsetbytes, self.biggest_non_mult_coeff);
            if !matched {
                self.valid = false;
                return;
            }
            let extra = AddrSpace::byte_to_address(extra as u64, word_size);
            self.offset = self.offset.wrapping_sub(extra) & self.ptrmask;
            self.correct = self.correct.wrapping_sub(extra) & self.ptrmask;
            self.is_subtype = true;
        } else if self.base_type.get_metatype() == type_metatype::TYPE_STRUCT {
            let soffset = sign_extend(self.offset as i64, self.ptrsize * 8 - 1);
            let offsetbytes = AddrSpace::address_to_byte_int(soffset, word_size);
            let (matched, mut extra) = self.has_matching_sub_type(offsetbytes, self.biggest_non_mult_coeff);
            if !matched {
                // Compare as bytes, not address units.
                if offsetbytes < 0 || offsetbytes >= self.base_type.get_size() as int8 {
                    self.valid = false;
                    return;
                }
                extra = 0; // No field, pretend there is something there
            }
            let extra = AddrSpace::byte_to_address_int(extra, word_size);
            self.offset = self.offset.wrapping_sub(extra as uintb) & self.ptrmask;
            self.correct = self.correct.wrapping_sub(extra as uintb) & self.ptrmask;
            if let Some(rel) = &self.p_rel_type {
                if self.offset == rel.get_address_offset().unwrap_or(0) as uintb {
                    // Offset falls within basic ptrto.
                    if rel.evaluate_thru_parent(0) != Some(true) {
                        self.valid = false;
                        return;
                    }
                }
            }
            self.is_subtype = true;
        } else if self.base_type.get_metatype() == type_metatype::TYPE_ARRAY {
            self.is_subtype = true;
            self.correct = self.correct.wrapping_sub(self.offset) & self.ptrmask;
            self.offset = 0;
        } else {
            // No struct or array, but nonmult is non-empty
            self.valid = false;
        }
        if let Some(_rel) = &self.p_rel_type {
            // int4 ptrOff = ((TypePointerRel*)ct)->getAddressOffset();
            let ptr_off = self.ct.get_address_offset().unwrap_or(0) as uintb;
            self.offset = self.offset.wrapping_sub(ptr_off) & self.ptrmask;
            self.correct = self.correct.wrapping_sub(ptr_off) & self.ptrmask;
        }
    }

    /// C++ `AddTreeState::assignPropagatedType(op)`.  Propagate the input pointer's
    /// data-type to the output of the newly created PTRADD/PTRSUB.
    fn assign_propagated_type(&mut self, op: OpId) {
        let vn = self.op_in(op, 0);
        let in_type = self.data.vbank().get(vn).map(|v| v.get_type_read_facing(op).clone());
        let out = match self.data.obank().get(op).and_then(|o| o.get_out()) {
            Some(o) => o,
            None => return,
        };
        if let Some(in_type) = in_type {
            // newType = op->getOpcode()->propagateType(inType, op, vn, out, 0, -1);
            let new_type = crate::coreaction_infertypes::propagate_type_pub(
                self.data, in_type, op, vn, out, 0, -1,
            );
            if let Some(nt) = new_type {
                if let Some(v) = self.data.vbank_mut().get_mut(out) {
                    v.update_type(nt);
                }
            }
        }
    }

    /// C++ `AddTreeState::buildMultiples()`.  Build the sub-expression summing to a
    /// multiple of the base data-type size (the PTRADD index).  Returns the result
    /// Varnode or `None` if there are no multiples.
    fn build_multiples(&mut self) -> Option<VarnodeId> {
        let smultsum = sign_extend(self.multsum as i64, self.ptrsize * 8 - 1);
        let const_coeff: uintb = if self.size == 0 {
            0
        } else {
            ((smultsum / self.size) as uintb) & self.ptrmask
        };
        let mut res_node: Option<VarnodeId> =
            if const_coeff == 0 { None } else { Some(self.data.new_constant(self.ptrsize, const_coeff)) };
        for i in 0..self.multiple.len() {
            let final_coeff: uintb = if self.size == 0 {
                0
            } else {
                ((self.coeff[i] / self.size) as uintb) & self.ptrmask
            };
            let mut vn = self.multiple[i];
            if final_coeff != 1 {
                let c = self.data.new_constant(self.ptrsize, final_coeff);
                let op = self.data.new_op_before(self.base_op, OpCode::CPUI_INT_MULT, vn, c, None);
                vn = self.data.obank().get(op).and_then(|o| o.get_out()).expect("build_multiples: mult out");
            }
            res_node = match res_node {
                None => Some(vn),
                Some(rn) => {
                    let op = self.data.new_op_before(self.base_op, OpCode::CPUI_INT_ADD, vn, rn, None);
                    Some(self.data.obank().get(op).and_then(|o| o.get_out()).expect("build_multiples: add out"))
                }
            };
        }
        res_node
    }

    /// C++ `AddTreeState::buildExtra()`.  Build the sum of non-multiple terms,
    /// correcting for double-counted constants.
    fn build_extra(&mut self) -> Option<VarnodeId> {
        let mut res_node: Option<VarnodeId> = None;
        for i in 0..self.nonmult.len() {
            let vn = self.nonmult[i];
            if self.vn_is_constant(vn) {
                self.correct = self.correct.wrapping_sub(self.vn_offset(vn));
                continue;
            }
            res_node = match res_node {
                None => Some(vn),
                Some(rn) => {
                    let op = self.data.new_op_before(self.base_op, OpCode::CPUI_INT_ADD, vn, rn, None);
                    Some(self.data.obank().get(op).and_then(|o| o.get_out()).expect("build_extra: add out"))
                }
            };
        }
        self.correct &= self.ptrmask;
        if self.correct != 0 {
            let vn = self
                .data
                .new_constant(self.ptrsize, uintb_negate(self.correct.wrapping_sub(1), self.ptrsize));
            res_node = match res_node {
                None => Some(vn),
                Some(rn) => {
                    let op = self.data.new_op_before(self.base_op, OpCode::CPUI_INT_ADD, vn, rn, None);
                    Some(self.data.obank().get(op).and_then(|o| o.get_out()).expect("build_extra: corr add out"))
                }
            };
        }
        res_node
    }

    /// C++ `AddTreeState::buildDegenerate()`.
    fn build_degenerate(&mut self) -> bool {
        let word_size = self.ct.get_word_size().unwrap_or(1) as int4;
        if self.base_type.get_align_size() < word_size {
            return false; // Probably padding; don't transform
        }
        // if (baseOp->getOut()->getTypeDefFacing()->getMetatype() != TYPE_PTR) return false;
        let out = match self.data.obank().get(self.base_op).and_then(|o| o.get_out()) {
            Some(o) => o,
            None => return false,
        };
        let out_meta = self
            .data
            .vbank()
            .get(out)
            .map(|v| v.get_type_def_facing().get_metatype())
            .unwrap_or(type_metatype::TYPE_UNKNOWN);
        if out_meta != type_metatype::TYPE_PTR {
            return false;
        }
        // newparams = [ ptr, baseOp->getIn(1-slot), newConstant(ct->getSize(),1) ]
        let slot = self.data.obank().get(self.base_op).map(|o| o.get_slot(self.ptr)).unwrap_or(0);
        let other = self.op_in(self.base_op, 1 - slot);
        let one = self.data.new_constant(self.ct.get_size(), 1);
        let newparams = [self.ptr, other, one];
        let _ = self.data.op_set_all_input(self.base_op, &newparams);
        self.data.op_set_opcode_code(self.base_op, OpCode::CPUI_PTRADD);
        true
    }

    /// C++ `AddTreeState::apply()`.
    pub fn apply(&mut self) -> bool {
        if self.is_degenerate {
            return self.build_degenerate();
        }
        self.span_add_tree(self.base_op, 1);
        if !self.valid {
            return false;
        }
        if self.distribute_op.is_some() && !self.is_distribute_used {
            self.clear();
            self.prevent_distribution = true;
            self.span_add_tree(self.base_op, 1);
        }
        self.calc_subtype();
        if !self.valid {
            return false;
        }
        while self.valid && self.distribute_op.is_some() {
            let dop = self.distribute_op.expect("apply: distribute_op");
            if !self.data.distribute_int_mult_add(dop).unwrap_or(false) {
                self.valid = false;
                break;
            }
            let dop_in0 = self.op_in(dop, 0);
            let dop_in1 = self.op_in(dop, 1);
            self.data.collapse_int_mult_mult(dop_in0);
            self.data.collapse_int_mult_mult(dop_in1);
            self.clear();
            self.span_add_tree(self.base_op, 1);
            if self.distribute_op.is_some() && !self.is_distribute_used {
                self.clear();
                self.prevent_distribution = true;
                self.span_add_tree(self.base_op, 1);
            }
            self.calc_subtype();
        }
        if !self.valid {
            // SEAM(print-markup): warningHeader emits a comment ("Problems
            // distributing in pointer arithmetic at <addr>"); the comment surface
            // is not part of the transform.  Only reachable on a distribution
            // failure (not exercised by the pointer/array/struct corpus).
            return true;
        }
        self.build_tree();
        true
    }

    /// C++ `AddTreeState::buildTree()`.  Rewrite the ADD tree as a pointer
    /// expression, preserving the original output Varnode.
    fn build_tree(&mut self) {
        let mult_node = self.build_multiples();
        let extra_node = self.build_extra();
        let mut newop: Option<OpId> = None;

        // PTRADD portion.
        let mut mult_node = if let Some(mn) = mult_node {
            let size_const = self.data.new_constant(self.ptrsize, self.size as uintb);
            let op = self.data.new_op_before(
                self.base_op,
                OpCode::CPUI_PTRADD,
                self.ptr,
                mn,
                Some(size_const),
            );
            // needsResolution union branch (faithful no-op for plain pointers).
            // SEAM(W8 union): forceFacingType / inheritUnionField.
            if self.data.is_type_recovery_exceeded() {
                self.assign_propagated_type(op);
            }
            newop = Some(op);
            self.data.obank().get(op).and_then(|o| o.get_out()).expect("build_tree: ptradd out")
        } else {
            self.ptr // Zero multiple terms
        };

        // PTRSUB portion.
        if self.is_subtype {
            let off_const = self.data.new_constant(self.ptrsize, self.offset);
            let op = self.data.new_op_before(
                self.base_op,
                OpCode::CPUI_PTRSUB,
                mult_node,
                off_const,
                None,
            );
            // SEAM(W8 union): inheritUnionField (no-op for plain pointers).
            if self.data.is_type_recovery_exceeded() {
                self.assign_propagated_type(op);
            }
            if self.size != 0 {
                self.data.op_set_stop_type_propagation(op);
            }
            newop = Some(op);
            mult_node = self.data.obank().get(op).and_then(|o| o.get_out()).expect("build_tree: ptrsub out");
        }

        // Add back any remaining terms.
        if let Some(en) = extra_node {
            let op = self.data.new_op_before(self.base_op, OpCode::CPUI_INT_ADD, mult_node, en, None);
            newop = Some(op);
        }

        let newop = match newop {
            Some(o) => o,
            None => {
                // SEAM(print-markup): data.warning("ptrarith problems", addr); comment
                // emission only — never reached when a multiple/subtype/extra exists.
                return;
            }
        };
        let base_out = self.data.obank().get(self.base_op).and_then(|o| o.get_out())
            .expect("build_tree: base_op has output");
        let _ = self.data.op_set_output(newop, base_out);
        self.data.op_destroy(self.base_op);
    }
}
