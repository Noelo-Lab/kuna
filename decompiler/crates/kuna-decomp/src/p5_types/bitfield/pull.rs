//! Port of `BitFieldPullTransform` (`decompiler/cpp/bitfield.cc:1023-1669`):
//! the transform that converts bitfield *pull* (extraction/read) expressions —
//! `(value >> #shift) & #mask` style mask-and-shift sequences — into explicit
//! `CPUI_ZPULL` / `CPUI_SPULL` p-code ops.
//!
//! `doTrace()` traces forward from a root Varnode that carries a bitfield
//! data-type, building a [`PullRecord`] at each point where an individual
//! bitfield has been fully isolated; `apply()` then materializes a ZPULL or
//! SPULL op for each record, splicing it into the live data-flow.
//!
//! # Faithfulness
//!
//! Every method is a line-for-line transcription of the C++ named in its doc
//! comment.  The op-creation order in `apply`/`applyRecord` is **semantics**:
//! `newOp`/`newConstant`/`newUnique*` each bump the function create-index, so
//! the exact sequence here determines the resulting IR's varnode ordering and is
//! observable in `print C` text.  Field offsets/widths come from the
//! [`BitFieldNodeState`] worklist (which the base `establishFields` filled from
//! the `TypeStruct` bitfield `TypeField`s); nothing is hardcoded.

use kuna_base::address::{extend_signbit, leastsigbit_set, popcount};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use super::{BitFieldNodeState, BitFieldTransform, BitRange};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::context::{OpId, VarnodeId};
use std::rc::Rc;

/// `sizeof(uintb)` — the C++ `uintb` is `uint8`, i.e. 8 bytes.
const SIZEOF_UINTB: int4 = 8;

/// Type of [`PullRecord`] (C++ anonymous enum in `BitFieldPullTransform::
/// PullRecord`, bitfield.hh:134-138).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PullKind {
    /// A single field pull (C++ `normal = 0`).
    Normal,
    /// Pull for INT_EQUAL or INT_NOTEQUAL (C++ `equal = 1`).
    Equal,
    /// The pull for the entire PcodeOp should be aborted (C++ `aborted = 2`).
    Aborted,
}

/// Info about a single read by a PcodeOp that can be treated as a \e pull of one
/// or more bitfields (C++ `BitFieldPullTransform::PullRecord`, bitfield.hh:133-153).
#[derive(Debug, Clone)]
struct PullRecord {
    /// Varnode holding pulled value (C++ `readVn`).
    read_vn: Option<VarnodeId>,
    /// Op reading the pulled value, or `None` if `read_vn` itself is redefined
    /// (C++ `readOp`).
    read_op: Option<OpId>,
    /// Data-type associated with the pulled value (C++ `dt`).
    dt: Option<Rc<Datatype>>,
    /// Type of pull (C++ `type`).
    kind: PullKind,
    /// Bit position of field being pulled (C++ `pos`).
    pos: int4,
    /// Number of bits in field being pulled (C++ `numBits`).
    num_bits: int4,
    /// Amount final field is left shifted (C++ `leftShift`).
    left_shift: int4,
    /// Mask representing the bitfield within the Varnode (C++ `mask`).
    mask: uintb,
    /// The `read_op`'s sequence number captured for the stable `operator<`
    /// ordering: `(pc.space.index, pc.offset, uniq)`, exactly the tuple C++
    /// `SeqNum::operator<` compares (`pc` then `uniq`; address.hh:154-158).
    read_op_order: Option<(int4, uintb, u32)>,
}

impl PullRecord {
    /// Construct pull record for a specific PcodeOp read (C++
    /// `PullRecord(const BitFieldNodeState&,PcodeOp*)`, bitfield.cc:1023-1034).
    fn normal(state: &BitFieldNodeState, op: Option<OpId>) -> PullRecord {
        let field = state.field().expect("normal PullRecord requires a field");
        PullRecord {
            read_vn: Some(state.node),
            read_op: op,
            dt: Some(Rc::clone(&field.field_type)),
            kind: PullKind::Normal,
            pos: state.orig_least_sig_bit,
            num_bits: field.num_bits,
            left_shift: state.bits_field.least_sig_bit,
            mask: 0,
            read_op_order: None,
        }
    }

    /// Construct record for a pull into an INT_EQUAL or INT_NOTEQUAL (C++
    /// `PullRecord(const BitFieldNodeState&,PcodeOp*,uintb)`, bitfield.cc:1039-1050).
    fn equal(state: &BitFieldNodeState, op: OpId, val: uintb) -> PullRecord {
        let field = state.field().expect("equal PullRecord requires a field");
        PullRecord {
            read_vn: Some(state.node),
            read_op: Some(op),
            dt: Some(Rc::clone(&field.field_type)),
            kind: PullKind::Equal,
            pos: state.orig_least_sig_bit,
            num_bits: field.num_bits,
            left_shift: state.bits_field.least_sig_bit,
            mask: val,
            read_op_order: None,
        }
    }

    /// Construct record representing an abort (C++ `PullRecord(PcodeOp*)`,
    /// bitfield.cc:1053-1064).
    fn aborted(op: OpId) -> PullRecord {
        PullRecord {
            read_vn: None,
            read_op: Some(op),
            dt: None,
            kind: PullKind::Aborted,
            pos: 0,
            num_bits: 0,
            left_shift: 0,
            mask: 0,
            read_op_order: None,
        }
    }
}

/// Class that converts bitfield pull expressions into explicit ZPULL and SPULL
/// operations (C++ `BitFieldPullTransform`, bitfield.hh:124-178).
pub struct BitFieldPullTransform {
    /// The shared base (parentStruct/initialOffset/containerSize/isBigEndian/
    /// workList).
    base: BitFieldTransform,
    /// Value being pulled from (C++ `root`).
    root: VarnodeId,
    /// LOAD op producing root, if non-null (C++ `loadOp`).
    load_op: Option<OpId>,
    /// Pull actions (C++ `pullList`).
    pull_list: Vec<PullRecord>,
}

/// State maintained across all `applyRecord` transforms (C++
/// `BitFieldPullTransform::TransformState`, bitfield.hh:125-131).
struct TransformState {
    /// Partial data-type of the root container (C++ `partialType`).
    partial_type: Rc<Datatype>,
    /// Number of PullRecords processed (C++ `count`).
    count: int4,
    /// Scratch space for opDestroyRecursive (C++ `deadScratch`).
    dead_scratch: Vec<OpId>,
}

impl BitFieldPullTransform {
    /// Construct from a Varnode containing bitfields (C++
    /// `BitFieldPullTransform::BitFieldPullTransform`, bitfield.cc:1610-1622).
    ///
    /// `big_endian` is `func->getArch()->getDefaultDataSpace()->isBigEndian()`;
    /// the caller (the rule body) reads it off the architecture.
    pub fn new(
        data: &Funcdata,
        r: VarnodeId,
        dt: &Rc<Datatype>,
        off: int4,
        big_endian: bool,
    ) -> Option<BitFieldPullTransform> {
        let mut base = BitFieldTransform::new(dt, off, big_endian);
        if base.initial_offset == -1 {
            return None;
        }
        let root = r;
        let container_size = data.vbank().get(root)?.get_size();
        base.container_size = container_size;
        // if (root->isWritten() && root->getDef()->code() == CPUI_LOAD) loadOp = ...
        let rv = data.vbank().get(root)?;
        let load_op = if rv.is_written() {
            rv.get_def().filter(|&d| {
                data.obank().get(d).map(|o| o.code()) == Some(OpCode::CPUI_LOAD)
            })
        } else {
            None
        };
        let mut t = BitFieldPullTransform {
            base,
            root,
            load_op,
            pull_list: Vec::new(),
        };
        // establishFields(root,false);  // Don't follow holes
        t.base.establish_fields(root, container_size, false);
        Some(t)
    }

    /// Test if all consumed bits are in the given bitfield (C++
    /// `BitFieldPullTransform::testConsumed`, bitfield.cc:1086-1093).
    fn test_consumed(data: &Funcdata, vn: VarnodeId, bit_field: &BitRange) -> bool {
        if bit_field.byte_size > SIZEOF_UINTB {
            return false;
        }
        let mask = bit_field.get_mask();
        let consume = match data.vbank().get(vn) {
            Some(v) => v.get_consume(),
            None => return false,
        };
        let intersect = mask & consume;
        intersect == consume
    }

    // --- forward handlers (bitfield.cc:1099-1341) -----------------------------

    /// Follow bitfield forward through INT_LEFT (C++ `handleLeftForward`,
    /// bitfield.cc:1099-1118).
    fn handle_left_forward(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        let o = data.obank().get(op).unwrap();
        if o.get_in(0) != Some(state.node) {
            return;
        }
        let cvn = match o.get_in(1) {
            Some(v) => v,
            None => return,
        };
        if !data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
            return;
        }
        let sa = data.vbank().get(cvn).unwrap().get_offset() as int4;
        let mut new_range = state.bits_field;
        new_range.shift(sa);
        if new_range.num_bits == 0 {
            return;
        }
        let out = o.get_out();
        if state.bits_field.num_bits == new_range.num_bits {
            let new_sign_ext = state.is_sign_extended || new_range.is_most_significant();
            let mut ns = BitFieldNodeState::with_new_field(state, &new_range, out.unwrap(), new_sign_ext);
            ns.bits_used.shift(sa);
            self.base.work_list.push(ns);
        } else if Self::test_consumed(data, out.unwrap(), &new_range) {
            self.pull_list.push(PullRecord::normal(state, Some(op)));
        }
    }

    /// Follow bitfield forward through INT_RIGHT or INT_SRIGHT (C++
    /// `handleRightForward`, bitfield.cc:1124-1146).
    fn handle_right_forward(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        let o = data.obank().get(op).unwrap();
        if o.get_in(0) != Some(state.node) {
            return;
        }
        let cvn = match o.get_in(1) {
            Some(v) => v,
            None => return,
        };
        if !data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
            return;
        }
        let sa = data.vbank().get(cvn).unwrap().get_offset() as int4;
        let mut new_range = state.bits_field;
        new_range.shift(-sa);
        if new_range.num_bits == 0 {
            return;
        }
        let code = o.code();
        let out = o.get_out();
        if state.bits_field.num_bits == new_range.num_bits {
            let new_sign_ext = if code == OpCode::CPUI_INT_SRIGHT {
                state.is_sign_extended
            } else {
                false
            };
            let mut ns = BitFieldNodeState::with_new_field(state, &new_range, out.unwrap(), new_sign_ext);
            ns.bits_used.shift(-sa);
            if code == OpCode::CPUI_INT_SRIGHT && !state.is_sign_extended {
                ns.bits_used.expand_to_most();
            }
            self.base.work_list.push(ns);
        } else if Self::test_consumed(data, out.unwrap(), &new_range) {
            self.pull_list.push(PullRecord::normal(state, Some(op)));
        }
    }

    /// Follow bitfield forward through INT_AND (C++ `handleAndForward`,
    /// bitfield.cc:1154-1173).
    fn handle_and_forward(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        let o = data.obank().get(op).unwrap();
        if o.get_in(0) != Some(state.node) {
            return;
        }
        if state.bits_field.byte_size > SIZEOF_UINTB {
            return;
        }
        let cvn = match o.get_in(1) {
            Some(v) => v,
            None => return,
        };
        if !data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
            return;
        }
        let and_val = data.vbank().get(cvn).unwrap().get_offset();
        let mask = state.bits_field.get_mask();
        let intersect = and_val & mask;
        if intersect == 0 {
            return; // Field is completely masked away
        }
        let out = o.get_out();
        if intersect == mask {
            // Nothing is masked away, follow the whole field
            let new_sign_ext = state.bits_field.is_most_significant();
            let mut ns = BitFieldNodeState::with_new_field(
                state,
                &state.bits_field,
                out.unwrap(),
                new_sign_ext,
            );
            ns.bits_used.intersect_mask(and_val);
            self.base.work_list.push(ns);
        } else if Self::test_consumed(data, out.unwrap(), &state.bits_field) {
            self.pull_list.push(PullRecord::normal(state, Some(op)));
        }
    }

    /// Follow bitfield forward through INT_ZEXT or INT_SEXT (C++
    /// `handleExtForward`, bitfield.cc:1178-1190).
    fn handle_ext_forward(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        let o = data.obank().get(op).unwrap();
        let outvn = o.get_out().unwrap();
        let out_size = data.vbank().get(outvn).unwrap().get_size();
        let node_size = data.vbank().get(state.node).unwrap().get_size();
        let diff = out_size - node_size;
        let code = o.code();
        let new_sign_ext = if code == OpCode::CPUI_INT_SEXT {
            state.is_sign_extended
        } else {
            false
        };
        let mut ns =
            BitFieldNodeState::with_new_field(state, &state.bits_field, outvn, new_sign_ext);
        ns.bits_field.extend_bytes(diff);
        ns.bits_used.extend_bytes(diff);
        if code == OpCode::CPUI_INT_SEXT && !state.is_sign_extended {
            ns.bits_used.expand_to_most();
        }
        self.base.work_list.push(ns);
    }

    /// Follow bitfield forward through INT_MULT (C++ `handleMultForward`,
    /// bitfield.cc:1197-1218).
    fn handle_mult_forward(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        let o = data.obank().get(op).unwrap();
        if o.get_in(0) != Some(state.node) {
            return;
        }
        let vn1 = match o.get_in(1) {
            Some(v) => v,
            None => return,
        };
        if !data.vbank().get(vn1).map(|v| v.is_constant()).unwrap_or(false) {
            return;
        }
        let val = data.vbank().get(vn1).unwrap().get_offset();
        if popcount(val) != 1 {
            self.handle_least_sig_op(data, state, op);
            return;
        }
        let sa = leastsigbit_set(val);
        let mut new_range = state.bits_field;
        new_range.shift(sa);
        if new_range.num_bits == 0 {
            return;
        }
        let out = o.get_out();
        if state.bits_field.num_bits == new_range.num_bits {
            let new_sign_ext = state.is_sign_extended || new_range.is_most_significant();
            let mut ns =
                BitFieldNodeState::with_new_field(state, &new_range, out.unwrap(), new_sign_ext);
            ns.bits_used.shift(sa);
            self.base.work_list.push(ns);
        }
    }

    /// Follow bitfield forward through SUBPIECE (C++ `handleSubpieceForward`,
    /// bitfield.cc:1224-1245).
    fn handle_subpiece_forward(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        let o = data.obank().get(op).unwrap();
        if o.get_in(0) != Some(state.node) {
            return;
        }
        let least_trunc = data.vbank().get(o.get_in(1).unwrap()).unwrap().get_offset() as int4;
        let out_size = data.vbank().get(o.get_out().unwrap()).unwrap().get_size();
        let most_trunc = (state.bits_field.byte_size - least_trunc) - out_size;
        let mut new_range = state.bits_field;
        new_range.truncate_least_sig_bytes(least_trunc);
        new_range.truncate_most_sig_bytes(most_trunc);
        if new_range.num_bits == 0 {
            return;
        }
        let out = o.get_out();
        if state.bits_field.num_bits == new_range.num_bits {
            // Any sign extension is preserved, since we only truncate and whole field present.
            let new_sign_ext = state.is_sign_extended;
            let mut ns =
                BitFieldNodeState::with_new_field(state, &new_range, out.unwrap(), new_sign_ext);
            ns.bits_used.truncate_least_sig_bytes(least_trunc);
            ns.bits_used.truncate_most_sig_bytes(most_trunc);
            self.base.work_list.push(ns);
        } else if Self::test_consumed(data, out.unwrap(), &new_range) {
            self.pull_list.push(PullRecord::normal(state, Some(op)));
        }
    }

    /// Follow bitfield forward into INSERT (C++ `handleInsertForward`,
    /// bitfield.cc:1251-1260).
    fn handle_insert_forward(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        let o = data.obank().get(op).unwrap();
        if o.get_in(1) != Some(state.node) {
            return; // Handle only if pull is value being inserted
        }
        if state.bits_field.least_sig_bit != 0 {
            return; // Test if least sig bits of inserted value are in the bitfield
        }
        let sz = data.vbank().get(o.get_in(3).unwrap()).unwrap().get_offset() as int4;
        if sz > state.bits_field.num_bits {
            return; // Test if more bits are getting INSERTed than are in bitfield
        }
        self.pull_list.push(PullRecord::normal(state, Some(op)));
    }

    /// Follow bitfield forward through INT_LESS, INT_SLESS, INT_LESSEQUAL,
    /// INT_SLESSEQUAL (C++ `handleLessForward`, bitfield.cc:1267-1311).
    fn handle_less_forward(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        if !state.bits_field.is_most_significant() {
            return;
        }
        let o = data.obank().get(op).unwrap();
        let slot = o.get_slot(state.node);
        let cvn = o.get_in(1 - slot).unwrap();
        if !data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
            return;
        }
        let val = data.vbank().get(cvn).unwrap().get_offset();
        let least_sig_zero_bits = (val & 1) == 0;
        let mut num_extremal_bits = if least_sig_zero_bits {
            leastsigbit_set(val)
        } else {
            leastsigbit_set(!val)
        };
        if num_extremal_bits < 0 {
            num_extremal_bits = SIZEOF_UINTB * 8;
        }
        let mut need_mask_check = false;
        let opc = o.code();
        if opc == OpCode::CPUI_INT_SLESS || opc == OpCode::CPUI_INT_LESS {
            if least_sig_zero_bits && slot != 0 {
                return;
            }
            if !least_sig_zero_bits && slot == 0 {
                need_mask_check = true;
            }
        } else if opc == OpCode::CPUI_INT_SLESSEQUAL || opc == OpCode::CPUI_INT_LESSEQUAL {
            if least_sig_zero_bits && slot != 1 {
                return;
            }
            if !least_sig_zero_bits && slot == 1 {
                need_mask_check = true;
            }
        }
        if need_mask_check {
            let mut mask: uintb = if num_extremal_bits >= 8 * SIZEOF_UINTB {
                0
            } else {
                (1u64) << num_extremal_bits
            };
            mask = mask.wrapping_sub(1);
            let nz = data.vbank().get(state.node).unwrap().get_nz_mask();
            if (mask & nz) == mask {
                return; // Must be at least one 0 bit
            }
        }
        if state.bits_field.least_sig_bit <= num_extremal_bits {
            // The comparison is only affected by field bits.
            self.pull_list.push(PullRecord::normal(state, Some(op)));
        }
    }

    /// Follow bitfield into INT_ADD, INT_MULT, INT_OR, INT_XOR etc. (C++
    /// `handleLeastSigOp`, bitfield.cc:1317-1324).
    fn handle_least_sig_op(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        if state.bits_field.least_sig_bit != 0 {
            return; // Field must be in least significant bits
        }
        let out = data.obank().get(op).unwrap().get_out().unwrap();
        if Self::test_consumed(data, out, &state.bits_field) {
            self.pull_list.push(PullRecord::normal(state, Some(op)));
        }
    }

    /// Follow bitfield into INT_EQUAL or INT_NOTEQUAL (C++ `handleEqualForward`,
    /// bitfield.cc:1328-1341).
    fn handle_equal_forward(&mut self, data: &Funcdata, state: &BitFieldNodeState, op: OpId) {
        let o = data.obank().get(op).unwrap();
        let cvn = match o.get_in(1) {
            Some(v) => v,
            None => return,
        };
        if state.bits_field.byte_size > SIZEOF_UINTB {
            return;
        }
        if !data.vbank().get(cvn).map(|v| v.is_constant()).unwrap_or(false) {
            return;
        }
        match state.field() {
            Some(f) if f.num_bits == state.bits_field.num_bits => {
                let val = state.bits_field.get_mask();
                self.pull_list.push(PullRecord::equal(state, op, val));
            }
            _ => {
                self.pull_list.push(PullRecord::aborted(op)); // Abort any pulls into this op
            }
        }
    }

    /// Follow a bitfield forward one level through all its descendants (C++
    /// `processForward`, bitfield.cc:1344-1399).
    fn process_forward(&mut self, data: &Funcdata, state: BitFieldNodeState) {
        if state.is_field_aligned() && state.does_sign_extension_match() {
            self.pull_list.push(PullRecord::normal(&state, None));
            return;
        }
        let descend: Vec<OpId> = match data.vbank().get(state.node) {
            Some(v) => v.descend_iter().collect(),
            None => return,
        };
        for op in descend {
            let code = match data.obank().get(op) {
                Some(o) => o.code(),
                None => continue,
            };
            match code {
                OpCode::CPUI_INT_LEFT => self.handle_left_forward(data, &state, op),
                OpCode::CPUI_INT_MULT => self.handle_mult_forward(data, &state, op),
                OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                    self.handle_right_forward(data, &state, op)
                }
                OpCode::CPUI_INT_AND => self.handle_and_forward(data, &state, op),
                OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
                    self.handle_ext_forward(data, &state, op)
                }
                OpCode::CPUI_INT_LESS
                | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL => self.handle_less_forward(data, &state, op),
                OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                    self.handle_equal_forward(data, &state, op)
                }
                OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_2COMP
                | OpCode::CPUI_INT_NEGATE => self.handle_least_sig_op(data, &state, op),
                OpCode::CPUI_SUBPIECE => self.handle_subpiece_forward(data, &state, op),
                OpCode::CPUI_INSERT => self.handle_insert_forward(data, &state, op),
                _ => {}
            }
        }
    }

    /// Compare two PullRecords (C++ `PullRecord::operator<`, bitfield.cc:1069-1081),
    /// keyed off the captured `read_op_order` (the defining op's sequence number).
    fn record_less(a: &PullRecord, b: &PullRecord) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (a.read_op, b.read_op) {
            (Some(_), Some(_)) => {
                if a.read_op != b.read_op {
                    // readOp->getSeqNum() < op2.readOp->getSeqNum()
                    a.read_op_order.cmp(&b.read_op_order)
                } else {
                    Ordering::Equal
                }
            }
            (None, _) => Ordering::Less,    // readOp == 0 -> true (this before)
            (Some(_), None) => Ordering::Greater, // op2.readOp == 0 -> false
        }
    }

    /// Capture the sequence-number tuple of each record's `read_op` so the sort
    /// is a faithful, stable transcription of `getSeqNum()` ordering.
    fn snapshot_orders(&mut self, data: &Funcdata) {
        for rec in self.pull_list.iter_mut() {
            rec.read_op_order = rec.read_op.and_then(|op| {
                data.obank().get(op).map(|o| {
                    let sq = o.get_seq_num();
                    let addr = sq.get_addr();
                    let space_idx = addr.get_space().map(|s| s.get_index()).unwrap_or(-1);
                    (space_idx, addr.get_offset(), sq.get_time())
                })
            });
        }
    }

    /// Trace bitfields from root to points where they are pulled (C++
    /// `BitFieldPullTransform::doTrace`, bitfield.cc:1626-1644).
    pub fn do_trace(&mut self, data: &Funcdata) -> bool {
        while let Some(front) = pop_front_worklist(&mut self.base.work_list) {
            self.process_forward(data, front);
        }
        if self.pull_list.is_empty() {
            return false;
        }
        self.snapshot_orders(data);
        self.pull_list.sort_by(Self::record_less);
        // Run testCompareGroup over non-normal record groups.
        let mut idx = 0;
        while idx < self.pull_list.len() {
            if self.pull_list[idx].kind != PullKind::Normal {
                idx = self.test_compare_group(data, idx);
            } else {
                idx += 1;
            }
        }
        !self.pull_list.is_empty()
    }

    /// Determine if pulls at a specific INT_EQUAL or INT_NOTEQUAL are consistent
    /// as a whole (C++ `testCompareGroup`, bitfield.cc:1409-1430).  Returns the
    /// index just past all PullRecords for the op.
    fn test_compare_group(&mut self, data: &Funcdata, start: usize) -> usize {
        let op = self.pull_list[start].read_op;
        let vn = self.pull_list[start].read_vn.unwrap();
        let val = {
            let o = data.obank().get(op.unwrap()).unwrap();
            data.vbank().get(o.get_in(1).unwrap()).unwrap().get_offset()
        };
        let mut cur = start;
        let mut is_aborted = false;
        let mut collect_mask: uintb = 0;
        while cur < self.pull_list.len() {
            let rec = &self.pull_list[cur];
            if rec.read_op != op {
                break;
            }
            cur += 1;
            if rec.kind == PullKind::Aborted {
                is_aborted = true;
            }
            collect_mask |= rec.mask;
        }
        let nz = data.vbank().get(vn).unwrap().get_nz_mask();
        if is_aborted || (!collect_mask & val) != 0 || (!collect_mask & nz) != 0 {
            self.pull_list.drain(start..cur);
            return start; // erase returns iterator at `start`
        }
        cur
    }

    // --- apply (bitfield.cc:1435-1669) ----------------------------------------

    /// Transform recovered expressions into ZPULL or SPULL operations (C++
    /// `BitFieldPullTransform::apply`, bitfield.cc:1649-1669).
    pub fn apply(&mut self, data: &mut Funcdata) {
        let partial_type = self.build_partial_type(data);
        let mut state = TransformState {
            partial_type,
            count: 0,
            dead_scratch: Vec::new(),
        };
        // Process the pull list front-to-back; equal records are first converted,
        // then normal records are materialized and popped.
        while !self.pull_list.is_empty() {
            if self.pull_list[0].kind == PullKind::Equal {
                self.apply_compare_record(data);
            } else {
                let rec = self.pull_list.remove(0);
                self.apply_record(data, rec, &mut state);
            }
        }
        if let Some(load) = self.load_op {
            if self.fold_load(data, load) {
                self.fold_ptrsub(data, load);
            }
        }
    }

    /// Build the (partial) data-type associated with the root bitfield container
    /// (C++ `BitFieldTransform::buildPartialType`, bitfield.cc:556-562).
    fn build_partial_type(&self, data: &Funcdata) -> Rc<Datatype> {
        let parent = self.base.parent_struct.as_ref().expect("parentStruct set");
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

    /// Perform transform corresponding to the given PullRecord (C++
    /// `BitFieldPullTransform::applyRecord`, bitfield.cc:1435-1499).
    fn apply_record(&mut self, data: &mut Funcdata, mut rec: PullRecord, state: &mut TransformState) {
        let mod_op: OpId;
        if rec.read_op.is_none() {
            // null readOp -> readVn contains a complete pull.
            let read_vn = rec.read_vn.unwrap();
            mod_op = data.vbank().get(read_vn).unwrap().get_def().unwrap();
            data.op_unset_output(mod_op);
        } else {
            let read_op = rec.read_op.unwrap();
            let read_vn = rec.read_vn.unwrap();
            if read_vn != self.root {
                mod_op = data.vbank().get(read_vn).unwrap().get_def().unwrap();
            } else {
                mod_op = read_op;
            }
            let slot = data.obank().get(read_op).unwrap().get_slot(read_vn);
            let sz = data.vbank().get(read_vn).unwrap().get_size();
            let new_vn = data.new_unique(sz, None);
            rec.read_vn = Some(new_vn);
            data.op_set_input(read_op, new_vn, slot).ok();
        }
        let mut in_vn = self.root;
        if let Some(load) = self.load_op {
            if state.count > 0 {
                let load_addr = data.obank().get(load).unwrap().get_addr().clone();
                let new_load = data.new_op(2, load_addr);
                data.op_set_opcode_code(new_load, OpCode::CPUI_LOAD);
                let lin0 = data.obank().get(load).unwrap().get_in(0).unwrap();
                let lin1 = data.obank().get(load).unwrap().get_in(1).unwrap();
                data.op_set_input(new_load, lin0, 0).ok();
                data.op_set_input(new_load, lin1, 1).ok();
                in_vn = data.new_unique_out(self.base.container_size, new_load).unwrap();
                data.op_insert_after(new_load, load);
                data.op_mark_non_printing_pub(new_load);
            }
        }
        data
            .vbank_mut()
            .get_mut(in_vn)
            .map(|v| v.update_type(Rc::clone(&state.partial_type)));
        let mod_addr = data.obank().get(mod_op).unwrap().get_addr().clone();
        let pull_op = data.new_op(3, mod_addr.clone());
        let pull_code = if rec.dt.as_ref().unwrap().get_metatype() == type_metatype::TYPE_INT {
            OpCode::CPUI_SPULL
        } else {
            OpCode::CPUI_ZPULL
        };
        data.op_set_opcode_code(pull_op, pull_code);
        data.op_set_input(pull_op, in_vn, 0).ok();
        let c_pos = data.new_constant(4, rec.pos as uintb);
        data.op_set_input(pull_op, c_pos, 1).ok();
        let c_num = data.new_constant(4, rec.num_bits as uintb);
        data.op_set_input(pull_op, c_num, 2).ok();
        if Some(mod_op) != rec.read_op {
            data.op_insert_after(pull_op, mod_op);
        } else {
            data.op_insert_before(pull_op, mod_op);
        }
        if rec.left_shift != 0 {
            let shift_vn = data.new_unique_out(self.base.container_size, pull_op).unwrap();
            let shift_op = data.new_op(2, mod_addr);
            data.op_set_opcode_code(shift_op, OpCode::CPUI_INT_LEFT);
            data.op_set_input(shift_op, shift_vn, 0).ok();
            let c_shift = data.new_constant(4, rec.left_shift as uintb);
            data.op_set_input(shift_op, c_shift, 1).ok();
            data.op_insert_after(shift_op, pull_op);
            data.op_set_output(shift_op, rec.read_vn.unwrap()).ok();
        } else {
            data.op_set_output(pull_op, rec.read_vn.unwrap()).ok();
        }

        let pull_out = data.obank().get(pull_op).unwrap().get_out().unwrap();
        let pull_out_meta = data.vbank().get(pull_out).unwrap().get_type().get_metatype();
        let pull_out_size = data.vbank().get(pull_out).unwrap().get_size();
        if pull_out_meta == type_metatype::TYPE_UNKNOWN {
            let arch = Rc::clone(data.get_arch());
            if let Some(types) = arch.types() {
                if let Ok(dt) = types.resize_integer(Rc::clone(rec.dt.as_ref().unwrap()), pull_out_size) {
                    data.vbank_mut().get_mut(pull_out).map(|v| v.update_type(dt));
                }
            }
        } else if rec.dt.as_ref().unwrap().get_metatype() == type_metatype::TYPE_BOOL
            && pull_out_size == 1
            && rec.num_bits == 1
        {
            let dt = Rc::clone(rec.dt.as_ref().unwrap());
            data.vbank_mut().get_mut(pull_out).map(|v| v.update_type(dt));
        }
        if Some(mod_op) != rec.read_op {
            let outvn = data.obank().get(mod_op).unwrap().get_out();
            let dead = match outvn {
                None => true,
                Some(v) => data.vbank().get(v).map(|x| x.has_no_descend()).unwrap_or(true),
            };
            if dead {
                data.op_destroy_recursive(mod_op, &mut state.dead_scratch);
            }
        }
        state.count += 1;
    }

    /// Perform transform on an INT_EQUAL or INT_NOTEQUAL group (C++
    /// `BitFieldPullTransform::applyCompareRecord`, bitfield.cc:1507-1567).
    fn apply_compare_record(&mut self, data: &mut Funcdata) {
        let first_read_op = self.pull_list[0].read_op.unwrap();
        let orig_val = {
            let o = data.obank().get(first_read_op).unwrap();
            data.vbank().get(o.get_in(1).unwrap()).unwrap().get_offset()
        };
        // Gather count of records for the same compare op.
        let mut num = 0;
        while num < self.pull_list.len() && self.pull_list[num].read_op == Some(first_read_op) {
            num += 1;
        }
        if num > 1 {
            let opc = data.obank().get(first_read_op).unwrap().code();
            let combine_code = if opc == OpCode::CPUI_INT_EQUAL {
                OpCode::CPUI_BOOL_AND
            } else {
                OpCode::CPUI_BOOL_OR
            };
            let vn = data.obank().get(first_read_op).unwrap().get_in(0).unwrap();
            let mut cur_combine = first_read_op;
            data.op_set_opcode_code(cur_combine, combine_code);
            for i in 0..num {
                let cc_addr = data.obank().get(cur_combine).unwrap().get_addr().clone();
                let op = data.new_op(2, cc_addr);
                data.op_set_opcode_code(op, opc);
                let bool_vn = data.new_unique_out(1, op).unwrap();
                data.op_set_input(op, vn, 0).ok();
                data.op_insert_before(op, cur_combine);
                if i == 0 {
                    data.op_set_input(cur_combine, bool_vn, 0).ok();
                } else if i < num - 1 {
                    let cc2_addr = data.obank().get(cur_combine).unwrap().get_addr().clone();
                    let combine_op = data.new_op(2, cc2_addr);
                    data.op_set_opcode_code(combine_op, combine_code);
                    let bool2_vn = data.new_unique_out(1, combine_op).unwrap();
                    data.op_set_input(cur_combine, bool2_vn, 1).ok();
                    data.op_set_input(combine_op, bool_vn, 0).ok();
                    data.op_insert_before(combine_op, cur_combine);
                    cur_combine = combine_op;
                } else {
                    data.op_set_input(cur_combine, bool_vn, 1).ok();
                }
                self.pull_list[i].read_op = Some(op);
            }
        }
        // Adjust compare value for each record and convert to normal.
        for i in 0..num {
            let (read_op, read_vn, dt, mask, left_shift, num_bits) = {
                let r = &self.pull_list[i];
                (
                    r.read_op.unwrap(),
                    r.read_vn.unwrap(),
                    Rc::clone(r.dt.as_ref().unwrap()),
                    r.mask,
                    r.left_shift,
                    r.num_bits,
                )
            };
            let mut val = orig_val & mask;
            val >>= left_shift;
            let read_size = data.vbank().get(read_vn).unwrap().get_size();
            if dt.get_metatype() == type_metatype::TYPE_INT {
                val = extend_signbit(val, num_bits, read_size);
            }
            let cvn = data.new_constant(read_size, val);
            let arch = Rc::clone(data.get_arch());
            if let Some(types) = arch.types() {
                if let Ok(ndt) = types.resize_integer(Rc::clone(&dt), read_size) {
                    data.vbank_mut().get_mut(cvn).map(|v| v.update_type(ndt));
                }
            }
            data.op_set_input(read_op, cvn, 1).ok();
            self.pull_list[i].kind = PullKind::Normal;
            self.pull_list[i].left_shift = 0;
        }
    }

    /// Try to mark LOAD as part of ZPULL or SPULL (C++
    /// `BitFieldPullTransform::foldLoad`, bitfield.cc:1573-1585).
    fn fold_load(&self, data: &mut Funcdata, load_op: OpId) -> bool {
        let outvn = match data.obank().get(load_op).and_then(|o| o.get_out()) {
            Some(v) => v,
            None => return false,
        };
        let descend: Vec<OpId> = data.vbank().get(outvn).unwrap().descend_iter().collect();
        for op in descend {
            let opc = data.obank().get(op).unwrap().code();
            if opc != OpCode::CPUI_ZPULL && opc != OpCode::CPUI_SPULL && opc != OpCode::CPUI_INSERT {
                return false;
            }
        }
        data.op_mark_non_printing_pub(load_op);
        true
    }

    /// Try to mark PTRSUB as part of ZPULL or SPULL (C++
    /// `BitFieldPullTransform::foldPtrsub`, bitfield.cc:1590-1604).
    fn fold_ptrsub(&self, data: &mut Funcdata, load_op: OpId) {
        let vn = match data.obank().get(load_op).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => return,
        };
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
            if o.code() != OpCode::CPUI_LOAD {
                return;
            }
            if !o.not_printed() {
                return; // Check if LOAD has been absorbed
            }
        }
        data.op_mark_non_printing_pub(ptrsub);
    }
}

/// Pop the front of the worklist (the C++ `workList` is a `std::list` consumed
/// front-to-back via `pop_front`).  A `Vec::remove(0)` is O(n) but the worklist
/// is short and this preserves the exact processing order.
fn pop_front_worklist(work_list: &mut Vec<BitFieldNodeState>) -> Option<BitFieldNodeState> {
    if work_list.is_empty() {
        None
    } else {
        Some(work_list.remove(0))
    }
}
