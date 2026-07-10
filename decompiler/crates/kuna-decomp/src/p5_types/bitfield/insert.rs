//! Port of `BitFieldInsertTransform` (`decompiler/cpp/bitfield.cc:118-1017`):
//! the transform that converts bitfield *insertion* (write) expressions —
//! `(orig & ~mask) | ((value << shift) & mask)` style read-modify-write
//! sequences — into explicit `CPUI_INSERT` p-code ops.
//!
//! `doTrace()` traces backward from a root Varnode (the value being STOREd, or
//! the mapped Varnode written) to find points that can be treated as a write to
//! an individual bitfield, building an [`InsertRecord`] at each; `apply()` then
//! materializes an INSERT op for each record.
//!
//! # Faithfulness
//!
//! Every method is a line-for-line transcription of the C++ named in its doc
//! comment.  The op-creation order in `apply` is **semantics**.  Field
//! offsets/widths come from the [`BitFieldNodeState`] worklist (filled from the
//! `TypeStruct` bitfield `TypeField`s by the base `establishFields`); nothing is
//! hardcoded.

use kuna_base::address::{calc_mask, extend_signbit, leastsigbit_set, popcount};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use super::{BitFieldNodeState, BitFieldTransform, BitRange, ALLOWED_FINAL_WRITES};
use crate::dtype::{type_metatype, Datatype};
use crate::expression::{pointer_equality, root_pointer};
use crate::funcdata::Funcdata;
use crate::context::{OpId, VarnodeId};
use std::rc::Rc;

/// `sizeof(uintb)` — the C++ `uintb` is `uint8`, i.e. 8 bytes.
const SIZEOF_UINTB: int4 = 8;

/// Get a mask representing the INSERTed range of bits (C++
/// `InsertExpression::getRangeMask`, expression.cc:496-507).
fn insert_range_mask(data: &Funcdata, insert: OpId) -> uintb {
    let o = data.obank().get(insert).unwrap();
    let least_bit_off = data.vbank().get(o.get_in(2).unwrap()).unwrap().get_offset() as int4;
    let bit_size = data.vbank().get(o.get_in(3).unwrap()).unwrap().get_offset() as int4;
    let mut res: uintb = !0u64;
    if bit_size < 8 * SIZEOF_UINTB {
        res = !(res << bit_size);
    }
    res << least_bit_off
}

/// Info about a Varnode that can be treated as a write to a single bitfield (C++
/// `BitFieldInsertTransform::InsertRecord`, bitfield.hh:69-80).
#[derive(Debug, Clone)]
struct InsertRecord {
    /// Value being inserted, or `None` (C++ `vn`).
    vn: Option<VarnodeId>,
    /// Constant value being inserted (C++ `constVal`).
    const_val: uintb,
    /// Data-type associated with value, or `None` (C++ `dt`).
    dt: Option<Rc<Datatype>>,
    /// Position being inserted to (C++ `pos`).
    pos: int4,
    /// Number of bits being inserted (C++ `numBits`).
    num_bits: int4,
    /// Amount that value needs to be right shifted (C++ `shiftAmount`).
    shift_amount: int4,
}

impl InsertRecord {
    /// Constructor for a Varnode value (C++ `InsertRecord(Varnode*,Datatype*,int4,
    /// int4,int4)`, bitfield.hh:78).
    fn from_vn(vn: VarnodeId, dt: Option<Rc<Datatype>>, p: int4, sz: int4, sa: int4) -> InsertRecord {
        InsertRecord { vn: Some(vn), const_val: 0, dt, pos: p, num_bits: sz, shift_amount: sa }
    }

    /// Constructor for a constant value (C++ `InsertRecord(uintb,Datatype*,int4,
    /// int4)`, bitfield.hh:79).
    fn from_const(val: uintb, dt: Option<Rc<Datatype>>, p: int4, sz: int4) -> InsertRecord {
        InsertRecord { vn: None, const_val: val, dt, pos: p, num_bits: sz, shift_amount: 0 }
    }
}

/// Class that converts bitfield insertion expressions into explicit INSERT
/// operations (C++ `BitFieldInsertTransform`, bitfield.hh:67-117).
pub struct BitFieldInsertTransform {
    base: BitFieldTransform,
    /// STORE to bitfields or op outputting to bitfields (C++ `finalWriteOp`).
    final_write_op: OpId,
    /// Value prior to insertion, or `None` (C++ `originalValue`).
    original_value: Option<VarnodeId>,
    /// Bitfield container written to, or `None` (C++ `mappedVn`).
    mapped_vn: Option<VarnodeId>,
    /// Insertion actions (C++ `insertList`).
    insert_list: Vec<InsertRecord>,
}

impl BitFieldInsertTransform {
    /// Construct from a terminating op (C++
    /// `BitFieldInsertTransform::BitFieldInsertTransform`, bitfield.cc:791-821).
    pub fn new(
        data: &Funcdata,
        op: OpId,
        dt: &Rc<Datatype>,
        off: int4,
        big_endian: bool,
    ) -> Option<BitFieldInsertTransform> {
        let mut base = BitFieldTransform::new(dt, off, big_endian);
        if base.initial_offset == -1 {
            return None;
        }
        let mut final_write_op = op;
        let mut mapped_vn: Option<VarnodeId> = None;
        let outvn: VarnodeId;
        let code = data.obank().get(final_write_op)?.code();
        if code == OpCode::CPUI_STORE {
            outvn = data.obank().get(final_write_op)?.get_in(2)?;
        } else if code == OpCode::CPUI_INDIRECT {
            let mv = data.obank().get(final_write_op)?.get_out()?;
            mapped_vn = Some(mv);
            let ov = data.obank().get(op)?.get_in(0)?;
            if !data.vbank().get(ov)?.is_written() {
                return None;
            }
            final_write_op = data.vbank().get(ov)?.get_def()?;
            outvn = ov;
            if !data.vbank().get(mv)?.is_addr_tied() {
                return None;
            }
            // binary_search(allowedFinalWrites, finalWriteOp->code())
            let fwc = data.obank().get(final_write_op)?.code();
            if !ALLOWED_FINAL_WRITES.contains(&fwc) {
                return None;
            }
        } else {
            outvn = data.obank().get(final_write_op)?.get_out()?;
            mapped_vn = Some(outvn);
            if !data.vbank().get(outvn)?.is_addr_tied() {
                return None;
            }
        }
        let container_size = data.vbank().get(outvn)?.get_size();
        base.container_size = container_size;
        let mut t = BitFieldInsertTransform {
            base,
            final_write_op,
            original_value: None,
            mapped_vn,
            insert_list: Vec::new(),
        };
        // establishFields(outvn,true);
        t.base.establish_fields(outvn, container_size, true);
        Some(t)
    }

    fn final_code(&self, data: &Funcdata) -> OpCode {
        data.obank().get(self.final_write_op).unwrap().code()
    }

    /// Is given state a partial field that is overwritten later (C++
    /// `isOverwrittenPartial`, bitfield.cc:131-144).
    fn is_overwritten_partial(&self, data: &Funcdata, state: &BitFieldNodeState) -> bool {
        if state.field().is_some() {
            return false; // Field is not partial
        }
        if state.bits_field.byte_size > SIZEOF_UINTB {
            return false;
        }
        if self.final_code(data) != OpCode::CPUI_STORE {
            let mapped = self.mapped_vn.unwrap();
            let mapped_size = data.vbank().get(mapped).unwrap().get_size();
            let cur_range = BitRange::new(
                self.base.initial_offset,
                mapped_size,
                state.orig_least_sig_bit,
                state.bits_field.num_bits,
                self.base.is_big_endian,
            );
            let bl = data.obank().get(self.final_write_op).unwrap().get_parent().unwrap();
            return Self::find_overwrite(data, mapped, bl, &cur_range);
        }
        false
    }

    /// Is this an original value defined by ZPULL or SPULL (C++
    /// `checkPulledOriginalValue`, bitfield.cc:146-158).
    fn check_pulled_original_value(&mut self, data: &Funcdata, state: &BitFieldNodeState) -> bool {
        if !data.vbank().get(state.node).map(|v| v.is_written()).unwrap_or(false) {
            return false;
        }
        let op = data.vbank().get(state.node).unwrap().get_def().unwrap();
        let opc = data.obank().get(op).unwrap().code();
        if opc != OpCode::CPUI_ZPULL && opc != OpCode::CPUI_SPULL {
            return false;
        }
        let pos = data.vbank().get(data.obank().get(op).unwrap().get_in(1).unwrap()).unwrap().get_offset() as int4;
        let numbits = data.vbank().get(data.obank().get(op).unwrap().get_in(2).unwrap()).unwrap().get_offset() as int4;
        if pos != state.bits_field.least_sig_bit {
            return false;
        }
        if numbits != state.bits_field.num_bits {
            return false;
        }
        let in0 = data.obank().get(op).unwrap().get_in(0).unwrap();
        self.check_original_base(data, in0)
    }

    /// Check if the given Varnode is the original LOAD or mapped value (C++
    /// `checkOriginalBase`, bitfield.cc:164-182).
    fn check_original_base(&mut self, data: &Funcdata, vn: VarnodeId) -> bool {
        if self.final_code(data) == OpCode::CPUI_STORE {
            if !data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false) {
                return false;
            }
            let load_op = data.vbank().get(vn).unwrap().get_def().unwrap();
            if data.obank().get(load_op).unwrap().code() != OpCode::CPUI_LOAD {
                return false;
            }
            let l1 = data.obank().get(load_op).unwrap().get_in(1).unwrap();
            let f1 = data.obank().get(self.final_write_op).unwrap().get_in(1).unwrap();
            if !pointer_equality(l1, f1, data.vbank(), data.obank()) {
                return false;
            }
            if data.obank().get(load_op).unwrap().get_parent()
                != data.obank().get(self.final_write_op).unwrap().get_parent()
            {
                return false;
            }
        } else {
            let mapped = self.mapped_vn.unwrap();
            if mapped == vn {
                return false;
            }
            let mv = data.vbank().get(mapped).unwrap();
            let vv = data.vbank().get(vn).unwrap();
            if mv.get_addr() != vv.get_addr() || mv.get_size() != vv.get_size() {
                return false;
            }
            if !vv.is_addr_tied() {
                return false;
            }
        }
        self.original_value = Some(vn);
        true
    }

    /// Is the given Varnode a (partial) copy of the original value being INSERTed
    /// into (C++ `isOriginalValue`, bitfield.cc:186-194).
    fn is_original_value(&mut self, data: &Funcdata, state: &BitFieldNodeState) -> bool {
        if state.bits_field.least_sig_bit != state.orig_least_sig_bit {
            return false;
        }
        if Some(state.node) == self.original_value {
            return true;
        }
        if self.check_pulled_original_value(data, state) {
            return true;
        }
        self.check_original_base(data, state.node)
    }

    /// Create InsertRecord writing a constant into the field (C++
    /// `addConstantWrite`, bitfield.cc:199-216).
    fn add_constant_write(&mut self, data: &Funcdata, state: &mut BitFieldNodeState) -> bool {
        let mut value = data.vbank().get(state.node).unwrap().get_offset();
        state.node = VarnodeId::default();
        let field = match state.field() {
            Some(f) => f.clone(),
            None => return false,
        };
        if state.bits_field.byte_size > SIZEOF_UINTB {
            return false;
        }
        let mask = state.bits_field.get_mask();
        value &= mask;
        value >>= state.bits_field.least_sig_bit;
        if field.field_type.get_metatype() == type_metatype::TYPE_INT {
            value = extend_signbit(value, state.bits_field.num_bits, state.bits_field.byte_size);
        }
        self.insert_list.push(InsertRecord::from_const(
            value,
            Some(Rc::clone(&field.field_type)),
            state.orig_least_sig_bit,
            field.num_bits,
        ));
        true
    }

    /// Create InsertRecord writing 0 into the field (C++ `addZeroOut`,
    /// bitfield.cc:222-231).
    fn add_zero_out(&mut self, state: &mut BitFieldNodeState) -> bool {
        state.node = VarnodeId::default();
        let field = match state.field() {
            Some(f) => f.clone(),
            None => return false,
        };
        self.insert_list.push(InsertRecord::from_const(
            0,
            Some(Rc::clone(&field.field_type)),
            state.orig_least_sig_bit,
            field.num_bits,
        ));
        true
    }

    /// Create InsertRecord writing Varnode into the field (C++ `addFieldWrite`,
    /// bitfield.cc:234-242).
    fn add_field_write(&mut self, data: &Funcdata, state: &mut BitFieldNodeState) {
        let field = state.field().unwrap().clone();
        let mut dt: Option<Rc<Datatype>> = Some(Rc::clone(&field.field_type));
        let node_size = data.vbank().get(state.node).unwrap().get_size();
        if field.field_type.get_size() != node_size {
            dt = None;
        }
        self.insert_list.push(InsertRecord::from_vn(
            state.node,
            dt,
            state.orig_least_sig_bit,
            field.num_bits,
            state.bits_field.least_sig_bit,
        ));
        state.node = VarnodeId::default();
    }

    // --- backward handlers (bitfield.cc:251-438) ------------------------------

    /// Follow bitfield back through INT_AND with a mask (C++ `handleAndBack`,
    /// bitfield.cc:251-268).
    fn handle_and_back(&mut self, data: &Funcdata, state: &mut BitFieldNodeState, op: OpId) -> bool {
        let cvn = data.obank().get(op).unwrap().get_in(1).unwrap();
        if !data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
            return false;
        }
        if state.bits_field.byte_size > SIZEOF_UINTB {
            return false;
        }
        let val = state.bits_field.get_mask();
        let cval = data.vbank().get(cvn).unwrap().get_offset();
        let res = val & cval;
        if res == val {
            state.node = data.obank().get(op).unwrap().get_in(0).unwrap();
            state.bits_used.intersect_mask(cval);
            return true;
        }
        if res == 0 {
            return self.add_zero_out(state);
        }
        false
    }

    /// Follow bitfield back through one branch of INT_OR (C++ `handleOrBack`,
    /// bitfield.cc:275-296).
    fn handle_or_back(&mut self, data: &Funcdata, state: &mut BitFieldNodeState, op: OpId) -> bool {
        if state.bits_field.byte_size > SIZEOF_UINTB {
            return false;
        }
        let mask = state.bits_field.get_mask();
        let vn0 = data.obank().get(op).unwrap().get_in(0).unwrap();
        let vn1 = data.obank().get(op).unwrap().get_in(1).unwrap();
        let nz0 = data.vbank().get(vn0).unwrap().get_nz_mask();
        let nz1 = data.vbank().get(vn1).unwrap().get_nz_mask();
        let is_masked0 = (nz0 & mask) == 0;
        let is_masked1 = (nz1 & mask) == 0;
        if is_masked0 == is_masked1 {
            if data.vbank().get(vn1).map(|v| v.is_constant()).unwrap_or(false)
                && (nz1 & mask) == mask
            {
                state.node = vn1;
                return true;
            }
            return false;
        }
        state.node = if is_masked0 { vn1 } else { vn0 };
        true
    }

    /// Follow bitfield back through INT_ADD (C++ `handleAddBack`,
    /// bitfield.cc:298-315).
    fn handle_add_back(&mut self, data: &Funcdata, state: &mut BitFieldNodeState, op: OpId) -> bool {
        if state.bits_field.byte_size > SIZEOF_UINTB {
            return false;
        }
        let vn0 = data.obank().get(op).unwrap().get_in(0).unwrap();
        let vn1 = data.obank().get(op).unwrap().get_in(1).unwrap();
        let mask0 = data.vbank().get(vn0).unwrap().get_nz_mask();
        let mask1 = data.vbank().get(vn1).unwrap().get_nz_mask();
        if (mask0 & mask1) != 0 {
            return false;
        }
        let mask = state.bits_field.get_mask();
        let is_masked0 = (mask0 & mask) == 0;
        let is_masked1 = (mask1 & mask) == 0;
        if is_masked0 == is_masked1 {
            return false;
        }
        state.node = if is_masked0 { vn1 } else { vn0 };
        true
    }

    /// Follow bitfield back through INT_LEFT by a constant (C++ `handleLeftBack`,
    /// bitfield.cc:323-343).
    fn handle_left_back(&mut self, data: &Funcdata, state: &mut BitFieldNodeState, op: OpId) -> bool {
        let cvn = data.obank().get(op).unwrap().get_in(1).unwrap();
        if !data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
            return false;
        }
        let sa = data.vbank().get(cvn).unwrap().get_offset() as int4;
        if !(0..SIZEOF_UINTB * 8).contains(&sa) {
            return false;
        }
        let mut new_range = state.bits_field;
        new_range.shift(-sa);
        if state.bits_field.num_bits == new_range.num_bits {
            state.bits_field = new_range;
            state.bits_used.shift(-sa);
            state.node = data.obank().get(op).unwrap().get_in(0).unwrap();
            return true;
        } else if new_range.num_bits == 0 {
            return self.add_zero_out(state);
        }
        false
    }

    /// Follow bitfield back through INT_SRIGHT by a constant (C++
    /// `handleRightBack`, bitfield.cc:349-365).
    fn handle_right_back(&mut self, data: &Funcdata, state: &mut BitFieldNodeState, op: OpId) -> bool {
        let cvn = data.obank().get(op).unwrap().get_in(1).unwrap();
        if !data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
            return false;
        }
        let sa = data.vbank().get(cvn).unwrap().get_offset() as int4;
        if !(0..SIZEOF_UINTB * 8).contains(&sa) {
            return false;
        }
        let mut new_range = state.bits_field;
        new_range.shift(sa);
        if state.bits_field.num_bits == new_range.num_bits {
            state.bits_field = new_range;
            state.bits_used.shift(sa);
            state.node = data.obank().get(op).unwrap().get_in(0).unwrap();
            return true;
        }
        false
    }

    /// Follow bitfield back through INT_ZEXT (C++ `handleZextBack`,
    /// bitfield.cc:372-389).
    fn handle_zext_back(&mut self, data: &Funcdata, state: &mut BitFieldNodeState, op: OpId) -> bool {
        let vn = data.obank().get(op).unwrap().get_in(0).unwrap();
        let out = data.obank().get(op).unwrap().get_out().unwrap();
        let trunc_amount =
            data.vbank().get(out).unwrap().get_size() - data.vbank().get(vn).unwrap().get_size();
        let mut new_range = state.bits_field;
        new_range.truncate_most_sig_bytes(trunc_amount);
        if state.bits_field.num_bits == new_range.num_bits {
            state.bits_field = new_range;
            state.bits_used.truncate_most_sig_bytes(trunc_amount);
            state.node = vn;
        } else if state.bits_field.num_bits == 0 {
            return self.add_zero_out(state);
        } else {
            return false;
        }
        true
    }

    /// Treat INT_MULT by a power of 2 like INT_LEFT (C++ `handleMultBack`,
    /// bitfield.cc:395-415).
    fn handle_mult_back(&mut self, data: &Funcdata, state: &mut BitFieldNodeState, op: OpId) -> bool {
        let vn1 = data.obank().get(op).unwrap().get_in(1).unwrap();
        if !data.vbank().get(vn1).map(|v| v.is_constant()).unwrap_or(false) {
            return false;
        }
        let val = data.vbank().get(vn1).unwrap().get_offset();
        if popcount(val) != 1 {
            return false;
        }
        let sa = leastsigbit_set(val);
        let mut new_range = state.bits_field;
        new_range.shift(-sa);
        if state.bits_field.num_bits == new_range.num_bits {
            state.bits_field = new_range;
            state.bits_used.shift(-sa);
            state.node = data.obank().get(op).unwrap().get_in(0).unwrap();
            return true;
        } else if state.bits_field.num_bits == 0 {
            return self.add_zero_out(state);
        }
        false
    }

    /// Follow the field into the input of the SUBPIECE (C++ `handleSubpieceBack`,
    /// bitfield.cc:421-438).
    fn handle_subpiece_back(&mut self, data: &Funcdata, state: &mut BitFieldNodeState, op: OpId) -> bool {
        let in_vn = data.obank().get(op).unwrap().get_in(0).unwrap();
        let extend_amount =
            data.vbank().get(in_vn).unwrap().get_size() - data.vbank().get(state.node).unwrap().get_size();
        let sa = data.vbank().get(data.obank().get(op).unwrap().get_in(1).unwrap()).unwrap().get_offset() as int4 * 8;
        let mut new_range = state.bits_field;
        new_range.extend_bytes(extend_amount);
        new_range.shift(-sa);
        if state.bits_field.num_bits == new_range.num_bits {
            state.bits_field = new_range;
            state.bits_used.extend_bytes(extend_amount);
            state.bits_used.shift(-sa);
            state.node = data.obank().get(op).unwrap().get_in(0).unwrap();
            return true;
        }
        false
    }

    /// Test if a call is producing the \e original \e value (C++
    /// `testCallOriginal`, bitfield.cc:445-469).
    fn test_call_original(&mut self, data: &mut Funcdata, state: &BitFieldNodeState, op: OpId) -> bool {
        if !data.obank().get(op).map(|o| o.is_call()).unwrap_or(false) {
            return false;
        }
        if self.final_code(data) == OpCode::CPUI_STORE {
            return false;
        }
        if state.bits_field.least_sig_bit != state.orig_least_sig_bit {
            return false;
        }
        let mapped = self.mapped_vn.unwrap();
        if data.vbank().get(mapped).unwrap().is_addr_tied() {
            return false;
        }
        if self.original_value.is_some() {
            return false;
        }
        let out = data.obank().get(op).unwrap().get_out().unwrap();
        let mut dt = data.vn_type_def_facing(out);
        let off;
        let meta = dt.get_metatype();
        if meta == type_metatype::TYPE_STRUCT {
            off = 0;
        } else if meta == type_metatype::TYPE_PARTIALSTRUCT {
            off = dt.get_partial_offset().unwrap_or(0);
            dt = match dt.get_partial_base() {
                Some(p) => p,
                None => return false,
            };
        } else {
            return false;
        }
        let parent = self.base.parent_struct.as_ref().unwrap();
        if !Rc::ptr_eq(&dt, parent) {
            return false;
        }
        if off != self.base.initial_offset {
            return false;
        }
        self.original_value = Some(out);
        true
    }

    /// Follow field back, creating an InsertRecord if possible (C++
    /// `processBackward`, bitfield.cc:473-553).
    fn process_backward(&mut self, data: &mut Funcdata, state: &mut BitFieldNodeState) -> bool {
        while state.node != VarnodeId::default() {
            if data.vbank().get(state.node).map(|v| v.is_constant()).unwrap_or(false) {
                return self.add_constant_write(data, state);
            }
            if self.is_original_value(data, state) {
                state.node = VarnodeId::default();
                return true;
            }
            if state.field().is_some() && state.is_field_aligned() {
                self.add_field_write(data, state);
                return true;
            }
            if !data.vbank().get(state.node).map(|v| v.is_written()).unwrap_or(false) {
                return false;
            }
            let op = data.vbank().get(state.node).unwrap().get_def().unwrap();
            let code = data.obank().get(op).unwrap().code();
            let lift_res = match code {
                OpCode::CPUI_COPY => {
                    state.node = data.obank().get(op).unwrap().get_in(0).unwrap();
                    true
                }
                OpCode::CPUI_INT_ADD => self.handle_add_back(data, state, op),
                OpCode::CPUI_INT_AND => self.handle_and_back(data, state, op),
                OpCode::CPUI_INT_LEFT => self.handle_left_back(data, state, op),
                OpCode::CPUI_INT_ZEXT => self.handle_zext_back(data, state, op),
                OpCode::CPUI_INT_OR => self.handle_or_back(data, state, op),
                OpCode::CPUI_INT_MULT => self.handle_mult_back(data, state, op),
                OpCode::CPUI_SUBPIECE => self.handle_subpiece_back(data, state, op),
                OpCode::CPUI_INT_SRIGHT => self.handle_right_back(data, state, op),
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => {
                    let r = self.test_call_original(data, state, op);
                    if r {
                        state.node = VarnodeId::default();
                        return true;
                    }
                    false
                }
                _ => false,
            };
            if !lift_res {
                if state.field().is_none() {
                    return false;
                }
                if state.bits_field.byte_size > SIZEOF_UINTB {
                    return false;
                }
                let nz = data.vbank().get(state.node).unwrap().get_nz_mask();
                let mut non_zero_bits = state.bits_field;
                non_zero_bits.intersect_mask(nz);
                if non_zero_bits.num_bits == 0 {
                    return self.add_zero_out(state);
                }
                state.bits_used.intersect_mask(nz);
                if non_zero_bits.num_bits == state.bits_used.num_bits {
                    self.add_field_write(data, state);
                    return true;
                }
                return false;
            }
        }
        true
    }

    /// Return true if specified bits in a Varnode are overwritten in the same
    /// basic block (C++ `BitFieldTransform::findOverwrite`, bitfield.cc:571-650).
    fn find_overwrite(data: &Funcdata, vn: VarnodeId, bl: crate::context::BlockId, range: &BitRange) -> bool {
        let mut min_range = *range;
        min_range.minimize_container();
        let base_addr = data.vbank().get(vn).unwrap().get_addr().clone();
        let addr = &base_addr + ((min_range.byte_offset - range.byte_offset) as i64);
        let descend: Vec<OpId> = data.vbank().get(vn).unwrap().descend_iter().collect();
        for first_op in descend {
            let mut cur_vn = vn;
            let mut op = Some(first_op);
            let mut cur_range = *range;
            while let Some(cop) = op {
                if data.obank().get(cop).unwrap().get_parent() != Some(bl) {
                    if cur_range.num_bits != 0 {
                        return false;
                    }
                    break;
                }
                let mut clear_op = false;
                match data.obank().get(cop).unwrap().code() {
                    OpCode::CPUI_PIECE => {
                        let in0 = data.obank().get(cop).unwrap().get_in(0).unwrap();
                        if in0 == cur_vn {
                            let sz = data.vbank().get(data.obank().get(cop).unwrap().get_in(1).unwrap()).unwrap().get_size();
                            cur_range.extend_bytes(sz);
                            cur_range.shift(sz * 8);
                        } else {
                            let sz0 = data.vbank().get(in0).unwrap().get_size();
                            cur_range.extend_bytes(sz0);
                        }
                    }
                    OpCode::CPUI_INT_LEFT => {
                        let cvn = data.obank().get(cop).unwrap().get_in(1).unwrap();
                        if data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
                            cur_range.shift(data.vbank().get(cvn).unwrap().get_offset() as int4);
                        } else {
                            return false;
                        }
                    }
                    OpCode::CPUI_INT_RIGHT => {
                        let cvn = data.obank().get(cop).unwrap().get_in(1).unwrap();
                        if data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
                            cur_range.shift(-(data.vbank().get(cvn).unwrap().get_offset() as int4));
                        } else {
                            return false;
                        }
                    }
                    OpCode::CPUI_COPY
                    | OpCode::CPUI_INT_OR
                    | OpCode::CPUI_INT_XOR
                    | OpCode::CPUI_INT_NEGATE => {}
                    OpCode::CPUI_INT_AND => {
                        let cvn = data.obank().get(cop).unwrap().get_in(1).unwrap();
                        if data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
                            cur_range.intersect_mask(data.vbank().get(cvn).unwrap().get_offset());
                        }
                    }
                    OpCode::CPUI_INSERT => {
                        cur_range.intersect_mask(!insert_range_mask(data, cop));
                    }
                    OpCode::CPUI_INDIRECT => {
                        cur_vn = data.obank().get(cop).unwrap().get_out().unwrap();
                        let cvn_addr = data.vbank().get(cur_vn).unwrap().get_addr().clone();
                        let cvn_size = data.vbank().get(cur_vn).unwrap().get_size();
                        if addr.contained_by(min_range.byte_size, &cvn_addr, cvn_size) {
                            return cur_range.num_bits == 0;
                        }
                        return false;
                    }
                    _ => {
                        if cur_range.num_bits != 0 {
                            return false;
                        }
                        clear_op = true;
                    }
                }
                if clear_op {
                    break;
                }
                cur_vn = data.obank().get(cop).unwrap().get_out().unwrap();
                let cvn_addr = data.vbank().get(cur_vn).unwrap().get_addr().clone();
                let cvn_size = data.vbank().get(cur_vn).unwrap().get_size();
                if addr.contained_by(min_range.byte_size, &cvn_addr, cvn_size) && cur_range.num_bits == 0 {
                    return true;
                }
                if data.vbank().get(cur_vn).unwrap().has_no_descend() {
                    break;
                }
                op = data.lone_descend(cur_vn);
            }
        }
        false
    }

    /// Fill-in INSERT inputs based on given InsertRecord (C++ `setInsertInputs`,
    /// bitfield.cc:657-684).  `op` is a preexisting op to reconfigure, or `None`.
    fn set_insert_inputs(&self, data: &mut Funcdata, op: Option<OpId>, rec: &InsertRecord) -> OpId {
        let op = match op {
            None => {
                let addr = data.obank().get(self.final_write_op).unwrap().get_addr().clone();
                data.new_op(4, addr)
            }
            Some(o) => {
                while data.obank().get(o).unwrap().num_input() < 4 {
                    let slot = data.obank().get(o).unwrap().num_input();
                    data.op_insert_input(o, VarnodeId::default(), slot).ok();
                }
                o
            }
        };
        data.op_set_opcode_code(op, OpCode::CPUI_INSERT);
        data.op_set_input(op, self.original_value.unwrap(), 0).ok();
        let val_vn = match rec.vn {
            Some(v) => v,
            None => {
                if let Some(dt) = &rec.dt {
                    let v = data.new_constant(dt.get_size(), rec.const_val);
                    data.vbank_mut().get_mut(v).map(|x| x.update_type(Rc::clone(dt)));
                    v
                } else {
                    data.new_constant(self.base.container_size, rec.const_val)
                }
            }
        };
        data.op_set_input(op, val_vn, 1).ok();
        let c_pos = data.new_constant(4, rec.pos as uintb);
        data.op_set_input(op, c_pos, 2).ok();
        let c_num = data.new_constant(4, rec.num_bits as uintb);
        data.op_set_input(op, c_num, 3).ok();
        data.op_mark_special_print(op);
        op
    }

    /// Create any shift p-code op specified by given InsertRecord (C++
    /// `addFieldShift`, bitfield.cc:689-701).
    fn add_field_shift(&self, data: &mut Funcdata, insert_op: OpId, rec: &InsertRecord) {
        if rec.shift_amount == 0 {
            return;
        }
        let val_vn = data.obank().get(insert_op).unwrap().get_in(1).unwrap();
        let val_size = data.vbank().get(val_vn).unwrap().get_size();
        let addr = data.obank().get(insert_op).unwrap().get_addr().clone();
        let shift_op = data.new_op(2, addr);
        data.op_set_opcode_code(shift_op, OpCode::CPUI_INT_RIGHT);
        let new_out = data.new_unique_out(val_size, shift_op).unwrap();
        data.op_set_input(insert_op, new_out, 1).ok();
        data.op_set_input(shift_op, val_vn, 0).ok();
        let c_sa = data.new_constant(4, rec.shift_amount as uintb);
        data.op_set_input(shift_op, c_sa, 1).ok();
        data.op_insert_before(shift_op, insert_op);
    }

    /// Try to mark LOAD as part of INSERT (C++ `foldLoad`, bitfield.cc:707-721).
    fn fold_load(&self, data: &mut Funcdata, load_op: OpId) -> bool {
        let outvn = data.obank().get(load_op).unwrap().get_out().unwrap();
        let descend: Vec<OpId> = data.vbank().get(outvn).unwrap().descend_iter().collect();
        for op in descend {
            if op == self.final_write_op {
                continue;
            }
            let opc = data.obank().get(op).unwrap().code();
            if opc != OpCode::CPUI_INSERT && opc != OpCode::CPUI_ZPULL && opc != OpCode::CPUI_SPULL {
                return false;
            }
        }
        data.op_mark_non_printing_pub(load_op);
        true
    }

    /// Try to mark PTRSUB as part of INSERT (C++ `foldPtrsub`,
    /// bitfield.cc:726-741).
    fn fold_ptrsub(&self, data: &mut Funcdata, load_op: OpId) {
        let vn = data.obank().get(load_op).unwrap().get_in(1).unwrap();
        if !data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false) {
            return;
        }
        let ptrsub = data.vbank().get(vn).unwrap().get_def().unwrap();
        if data.obank().get(ptrsub).unwrap().code() != OpCode::CPUI_PTRSUB {
            return;
        }
        let descend: Vec<OpId> = data.vbank().get(vn).unwrap().descend_iter().collect();
        for op in descend {
            let o = data.obank().get(op).unwrap();
            let c = o.code();
            if c == OpCode::CPUI_STORE && o.does_special_printing() {
                continue;
            }
            if c == OpCode::CPUI_LOAD && o.not_printed() {
                continue;
            }
            return;
        }
        data.op_mark_non_printing_pub(ptrsub);
    }

    /// Look for redundant INSERTs and delete the second (C++ `checkRedundancy`,
    /// bitfield.cc:746-785).
    fn check_redundancy(&self, data: &mut Funcdata, rec: &InsertRecord) {
        let rec_vn = match rec.vn {
            Some(v) => v,
            None => return,
        };
        let mut immed_op: Option<OpId> = None;
        let descend: Vec<OpId> = data.vbank().get(rec_vn).unwrap().descend_iter().collect();
        for raw_op in descend {
            let mut op = raw_op;
            if data.obank().get(op).unwrap().code() != OpCode::CPUI_INSERT {
                if data.obank().get(op).unwrap().code() != OpCode::CPUI_INT_RIGHT {
                    continue;
                }
                let out = data.obank().get(op).unwrap().get_out().unwrap();
                op = match data.lone_descend(out) {
                    Some(o) if data.obank().get(o).unwrap().code() == OpCode::CPUI_INSERT => o,
                    _ => continue,
                };
            }
            let immed = match immed_op {
                None => {
                    immed_op = Some(op);
                    continue;
                }
                Some(i) => i,
            };
            let op2 = data.vbank().get(data.obank().get(op).unwrap().get_in(2).unwrap()).unwrap().get_offset();
            let im2 = data.vbank().get(data.obank().get(immed).unwrap().get_in(2).unwrap()).unwrap().get_offset();
            if op2 != im2 {
                continue;
            }
            let op3 = data.vbank().get(data.obank().get(op).unwrap().get_in(3).unwrap()).unwrap().get_offset();
            let im3 = data.vbank().get(data.obank().get(immed).unwrap().get_in(3).unwrap()).unwrap().get_offset();
            if op3 != im3 {
                continue;
            }
            if self.final_code(data) == OpCode::CPUI_STORE {
                let store1 = match data.lone_descend(data.obank().get(op).unwrap().get_out().unwrap()) {
                    Some(s) if data.obank().get(s).unwrap().code() == OpCode::CPUI_STORE => s,
                    _ => continue,
                };
                let store2 = match data.lone_descend(data.obank().get(immed).unwrap().get_out().unwrap()) {
                    Some(s) if data.obank().get(s).unwrap().code() == OpCode::CPUI_STORE => s,
                    _ => continue,
                };
                if data.obank().get(store1).unwrap().get_parent() != data.obank().get(store2).unwrap().get_parent() {
                    continue;
                }
                let s1p = data.obank().get(store1).unwrap().get_in(1).unwrap();
                let s2p = data.obank().get(store2).unwrap().get_in(1).unwrap();
                if !pointer_equality(s1p, s2p, data.vbank(), data.obank()) {
                    continue;
                }
                let mut scratch: Vec<OpId> = Vec::new();
                let ord1 = data.obank().get(store1).unwrap().get_seq_num().get_order();
                let ord2 = data.obank().get(store2).unwrap().get_seq_num().get_order();
                if ord1 < ord2 {
                    data.op_destroy_recursive(store2, &mut scratch);
                } else {
                    data.op_destroy_recursive(store1, &mut scratch);
                }
            }
            return;
        }
    }

    // --- verify (bitfield.cc:827-917) -----------------------------------------

    /// Verify that no interfering STORE affects original-value bits (C++
    /// `verifyLoadStoreOriginalValue`, bitfield.cc:827-856).
    fn verify_load_store_original_value(&self, data: &Funcdata, mask: uintb) -> bool {
        let original = self.original_value.unwrap();
        let load_op = data.vbank().get(original).unwrap().get_def().unwrap();
        let bl = data.obank().get(self.final_write_op).unwrap().get_parent().unwrap();
        let ops = data.bb_ops(bl);
        // Find index of finalWriteOp, walk backward to block start.
        let final_idx = match ops.iter().position(|&o| o == self.final_write_op) {
            Some(i) => i,
            None => return true,
        };
        let f1 = data.obank().get(self.final_write_op).unwrap().get_in(1).unwrap();
        let mut off: uintb = 0;
        let base_ptr = root_pointer(f1, &mut off, data.vbank(), data.obank());
        for idx in (0..final_idx).rev() {
            let op = ops[idx];
            if op == load_op {
                return true;
            }
            if data.obank().get(op).unwrap().is_call() {
                return false;
            }
            if data.obank().get(op).unwrap().code() != OpCode::CPUI_STORE {
                continue;
            }
            let lspace = data.vbank().get(data.obank().get(load_op).unwrap().get_in(0).unwrap()).unwrap().get_offset();
            let sspace = data.vbank().get(data.obank().get(op).unwrap().get_in(0).unwrap()).unwrap().get_offset();
            if sspace != lspace {
                continue;
            }
            let sp = data.obank().get(op).unwrap().get_in(1).unwrap();
            let mut other_off: uintb = 0;
            if base_ptr != root_pointer(sp, &mut other_off, data.vbank(), data.obank()) {
                return false;
            }
            if other_off != off {
                continue;
            }
            let vn = data.obank().get(op).unwrap().get_in(2).unwrap();
            if !data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false) {
                return false;
            }
            let insert_op = data.vbank().get(vn).unwrap().get_def().unwrap();
            if data.obank().get(insert_op).unwrap().code() != OpCode::CPUI_INSERT {
                return false;
            }
            let insert_mask = insert_range_mask(data, insert_op);
            if (insert_mask & mask) != 0 {
                return false;
            }
        }
        true
    }

    /// Verify mapped original value (C++ `verifyMappedOriginalValue`,
    /// bitfield.cc:862-883).
    fn verify_mapped_original_value(&self, data: &Funcdata, mask: uintb) -> bool {
        let original = self.original_value.unwrap();
        let bl = data.obank().get(self.final_write_op).unwrap().get_parent().unwrap();
        let ops = data.bb_ops(bl);
        let final_idx = match ops.iter().position(|&o| o == self.final_write_op) {
            Some(i) => i,
            None => return true,
        };
        let orig_addr = data.vbank().get(original).unwrap().get_addr().clone();
        let orig_size = data.vbank().get(original).unwrap().get_size();
        for idx in (0..final_idx).rev() {
            let op = ops[idx];
            let vn = match data.obank().get(op).unwrap().get_out() {
                Some(v) => v,
                None => continue,
            };
            if vn == original {
                return true;
            }
            if data.obank().get(op).unwrap().is_call() {
                return false;
            }
            if data.vbank().get(vn).unwrap().get_addr() != &orig_addr {
                continue;
            }
            if data.vbank().get(vn).unwrap().get_size() != orig_size {
                continue;
            }
            if !data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false) {
                return false;
            }
            let insert_op = data.vbank().get(vn).unwrap().get_def().unwrap();
            if data.obank().get(insert_op).unwrap().code() != OpCode::CPUI_INSERT {
                return false;
            }
            let insert_mask = insert_range_mask(data, insert_op);
            if (insert_mask & mask) != 0 {
                return false;
            }
        }
        true
    }

    /// Calculate mask where 1 bits represent all the bits being preserved (C++
    /// `constructOriginalValueMask`, bitfield.cc:888-905).
    fn construct_original_value_mask(&self, data: &Funcdata) -> uintb {
        let mut mask: uintb = 0;
        for rec in &self.insert_list {
            let mut val: uintb = 0;
            if rec.num_bits < 8 * SIZEOF_UINTB {
                val = 1;
                val <<= rec.num_bits;
            }
            val = val.wrapping_sub(1);
            val <<= rec.pos;
            mask |= val;
        }
        let orig_size = data.vbank().get(self.original_value.unwrap()).unwrap().get_size();
        !mask & calc_mask(orig_size)
    }

    /// Do final check that unINSERTed bits come from the original value (C++
    /// `verifyOriginalValueBits`, bitfield.cc:908-917).
    fn verify_original_value_bits(&self, data: &Funcdata) -> bool {
        if self.original_value.is_none() {
            return true;
        }
        let mask = self.construct_original_value_mask(data);
        if mask == 0 {
            return true;
        }
        if self.final_code(data) == OpCode::CPUI_STORE {
            self.verify_load_store_original_value(data, mask)
        } else {
            self.verify_mapped_original_value(data, mask)
        }
    }

    /// Trace bitfields backward from the terminating op (C++ `doTrace`,
    /// bitfield.cc:921-934).
    pub fn do_trace(&mut self, data: &mut Funcdata) -> bool {
        if self.base.work_list.is_empty() {
            return false;
        }
        while !self.base.work_list.is_empty() {
            let mut node = self.base.work_list.remove(0);
            if !self.process_backward(data, &mut node) && !self.is_overwritten_partial(data, &node) {
                return false;
            }
        }
        if self.insert_list.is_empty() {
            return false;
        }
        self.verify_original_value_bits(data)
    }

    /// Build the (partial) data-type associated with the root bitfield container
    /// (C++ `buildPartialType`, bitfield.cc:556-562).
    fn build_partial_type(&self, data: &Funcdata) -> Rc<Datatype> {
        let parent = self.base.parent_struct.as_ref().unwrap();
        if self.base.container_size == parent.get_size() {
            return Rc::clone(parent);
        }
        let arch = Rc::clone(data.get_arch());
        if let Some(types) = arch.types() {
            if let Ok(t) = types.get_type_partial_struct(
                Rc::clone(parent),
                self.base.initial_offset,
                self.base.container_size,
            ) {
                return t;
            }
        }
        Rc::clone(parent)
    }

    /// Transform recovered expressions into INSERT operations (C++ `apply`,
    /// bitfield.cc:936-1017).
    pub fn apply(&mut self, data: &mut Funcdata) {
        let partial_type = self.build_partial_type(data);
        let records: Vec<InsertRecord> = self.insert_list.clone();
        if self.final_code(data) == OpCode::CPUI_STORE {
            self.apply_store(data, &partial_type, &records);
        } else {
            self.apply_mapped(data, &partial_type, &records);
        }
        for rec in &records {
            self.check_redundancy(data, rec);
        }
    }

    /// STORE-terminated apply (C++ `apply`, bitfield.cc:941-987).
    fn apply_store(&mut self, data: &mut Funcdata, partial_type: &Rc<Datatype>, records: &[InsertRecord]) {
        let dead_point = data.obank().get(self.final_write_op).unwrap().get_in(2).unwrap();
        let mut current_store = Some(self.final_write_op);
        let mut load_model: Option<OpId> = None;
        let mut load_type: Option<Rc<Datatype>> = None;
        if self.original_value.is_none() {
            self.original_value = Some(data.new_constant(self.base.container_size, 0));
        } else {
            let ov = self.original_value.unwrap();
            load_model = data.vbank().get(ov).unwrap().get_def();
            load_type = Some(data.vn_type_def_facing(ov));
        }
        for rec in records {
            if current_store.is_none() {
                let fwc = data.obank().get(self.final_write_op).unwrap();
                let addr = fwc.get_addr().clone();
                let f0 = fwc.get_in(0).unwrap();
                let f1 = fwc.get_in(1).unwrap();
                let cs = data.new_op(3, addr);
                data.op_set_opcode_code(cs, OpCode::CPUI_STORE);
                data.op_set_input(cs, f0, 0).ok();
                data.op_set_input(cs, f1, 1).ok();
                data.op_insert_after(cs, self.final_write_op);
                if let Some(lm) = load_model {
                    let laddr = data.obank().get(lm).unwrap().get_addr().clone();
                    let l0 = data.obank().get(lm).unwrap().get_in(0).unwrap();
                    let l1 = data.obank().get(lm).unwrap().get_in(1).unwrap();
                    let load_op = data.new_op(2, laddr);
                    data.op_set_opcode_code(load_op, OpCode::CPUI_LOAD);
                    data.op_set_input(load_op, l0, 0).ok();
                    data.op_set_input(load_op, l1, 1).ok();
                    let ov = data.new_unique_out(self.base.container_size, load_op).unwrap();
                    if let Some(lt) = &load_type {
                        data.vbank_mut().get_mut(ov).map(|v| v.update_type(Rc::clone(lt)));
                    }
                    self.original_value = Some(ov);
                    data.op_insert_before(load_op, cs);
                    data.op_mark_non_printing_pub(load_op);
                }
                current_store = Some(cs);
            }
            let cs = current_store.unwrap();
            let insert_op = self.set_insert_inputs(data, None, rec);
            let new_out = data.new_unique_out(self.base.container_size, insert_op).unwrap();
            data.vbank_mut().get_mut(new_out).map(|v| v.update_type(Rc::clone(partial_type)));
            let insert_out = data.obank().get(insert_op).unwrap().get_out().unwrap();
            data.op_set_input(cs, insert_out, 2).ok();
            data.op_insert_before(insert_op, cs);
            data.op_mark_special_print(cs);
            self.add_field_shift(data, insert_op, rec);
            current_store = None;
        }
        data.destroy_varnode_recursive(dead_point).ok();
        if let Some(lm) = load_model {
            if data.obank().get(lm).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY) == OpCode::CPUI_LOAD
                && self.fold_load(data, lm)
            {
                self.fold_ptrsub(data, lm);
            }
        }
    }

    /// Mapped-variable apply (C++ `apply`, bitfield.cc:988-1013).
    fn apply_mapped(&mut self, data: &mut Funcdata, partial_type: &Rc<Datatype>, records: &[InsertRecord]) {
        let num_in = data.obank().get(self.final_write_op).unwrap().num_input();
        let mut dead_points: Vec<VarnodeId> = Vec::new();
        for i in 0..num_in {
            if let Some(v) = data.obank().get(self.final_write_op).unwrap().get_in(i) {
                dead_points.push(v);
            }
        }
        if self.original_value.is_none() {
            self.original_value = Some(data.new_constant(self.base.container_size, 0));
        }
        let mapped = self.mapped_vn.unwrap();
        let mapped_addr = data.vbank().get(mapped).unwrap().get_addr().clone();
        let mut iter = records.iter();
        let first = iter.next().unwrap();
        let mut insert_op = self.set_insert_inputs(data, Some(self.final_write_op), first);
        let insert_out = data.obank().get(insert_op).unwrap().get_out().unwrap();
        data.vbank_mut().get_mut(insert_out).map(|v| v.update_type(Rc::clone(partial_type)));
        self.add_field_shift(data, insert_op, first);
        for rec in iter {
            let last_op = insert_op;
            data.op_unset_input(last_op, 0);
            insert_op = self.set_insert_inputs(data, None, rec);
            let new_out = data.new_varnode_out(self.base.container_size, &mapped_addr, insert_op).unwrap();
            data.vbank_mut().get_mut(new_out).map(|v| v.update_type(Rc::clone(partial_type)));
            data.op_set_input(last_op, new_out, 0).ok();
            data.op_insert_before(insert_op, last_op);
            self.add_field_shift(data, insert_op, rec);
        }
        for vn in dead_points {
            data.destroy_varnode_recursive(vn).ok();
        }
    }
}
