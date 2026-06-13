//! Port of `decompiler/cpp/constseq.{cc,hh}` — combining constants written to a
//! contiguous region of memory.
//!
//! When code writes a string into memory one character at a time (a run of COPY
//! ops into a stack array, or a run of STORE ops through a heap pointer), the
//! decompiler can collapse the whole run into a single `memcpy`/`strncpy` user-op
//! taking the assembled string as its source.  The shared machinery for *finding
//! and validating* such a run lives in [`ArraySequence`]; the two concrete
//! drivers — `StringSequence` (COPY-into-array) and `HeapSequence`
//! (STORE-through-pointer) — extend it, and the [`RuleStringCopy`]/`RuleStringStore`
//! rules apply them.
//!
//! # What this port covers
//!
//! The reusable, output-determining **base** is fully ported:
//!
//! * [`ArraySequence::MINIMUM_SEQUENCE_LENGTH`] / `MAXIMUM_SEQUENCE_LENGTH`.
//! * [`WriteNode`] and its block-order comparator (`getSeqNum().getOrder()`),
//!   which is the tie-breaker [`ArraySequence::check_interference`]'s `sort`
//!   relies on.
//! * [`ArraySequence::interfere_between`] — the "no interfering LOAD/STORE/CALL
//!   between two ops" test.
//! * [`ArraySequence::check_interference`] — the maximal-no-interference window
//!   selection around the root op.
//! * [`ArraySequence::form_byte_array`] — the constant-gathering ORDER: how the
//!   per-op constant inputs are laid into a single byte array by offset, the
//!   null-terminator handling, the contiguity/length check, and the moveOps
//!   truncation.  **This is the routine `kuna_memsetsequence` reuses**, so the
//!   base is exposed `pub(crate)` (see *Cross-pack visibility* below).
//!
//! # Faithfulness
//!
//! `int4`→`i32`, `uint8`/`uintb`→`u64`, `uint1`→`u8`.  The byte-array assembly
//! uses the same big/little-endian byte unpacking, the same `used[]` marking
//! (`1` = non-null char, `2` = null terminator), the same single-terminator
//! allowance, and the same `< MINIMUM_SEQUENCE_LENGTH` rejection.  The C++ holds
//! `Funcdata &data` / `PcodeOp *` / `Varnode *`; the port holds `&mut Funcdata`
//! plus `OpId`/`VarnodeId` read through the banks, and snapshots the moveOps
//! vector exactly as the C++ `vector<WriteNode>` is iterated.
//!
//! # Cross-pack visibility (for the kuna-pack porter)
//!
//! Upstream kuna **widened `StringSequence`'s members to `protected`** so
//! `kuna_memsetsequence` could reuse the COPY-gathering machinery
//! (`constseq.hh` `(kuna)` note).  In the Rust port the analogue is: the
//! [`ArraySequence`] struct fields and the base methods
//! (`interfere_between`, `check_interference`, `form_byte_array`,
//! `select_string_copy_function`) are **`pub(crate)`** so the
//! `kuna_memsetsequence` module (same crate) can build on them directly,
//! mirroring the C++ `protected` widening.  Do **not** re-port this base in the
//! memset module — extend [`ArraySequence`].
//!
//! # Deferred halves (cross-wave seams, ledgered as losses)
//!
//! `StringSequence`/`HeapSequence`'s *collection* and *transform* bodies, and the
//! [`RuleStringCopy`]/`RuleStringStore` drivers, reach subsystems later waves
//! own and are intentionally **not** transcribed here:
//!
//! * The address-only `beginLoc(addr)`/`endLoc(addr)` location-set overload
//!   (`StringSequence::collectCopyOps`) — only the size+addr overload is exposed
//!   today (W4 loc-set surface).
//! * `Scope::queryContainer` / `Symbol`/`SymbolEntry` array-component resolution
//!   (`StringSequence` ctor, `RuleStringCopy::applyOp`) — W4 symbol table.
//! * `Funcdata::getInternalString`, `getTypePointer*`, `inheritUnionField*`,
//!   `constructSpacebaseInput`, the PTRSUB/PTRADD typed-pointer builder
//!   (`buildStringCopy`, `constructTypedPointer`) — W4/W6 type-factory + union
//!   resolution wiring (the `Architecture` seam has no `types` field yet).
//! * `selectStringCopyFunction`'s `charType == types->getTypeChar(...)` identity
//!   check — the live `TypeFactory` is not wired into `Architecture`; the ported
//!   [`ArraySequence::select_string_copy_function`] keeps the structure and falls
//!   through to `BUILTIN_MEMCPY` (the C++ default) until the factory lands.
//!
//! These are recorded as losses in the structured output.

// The `ArraySequence` base and its read helpers are consumed by the sibling
// `kuna_memsetsequence` module (its Rust port lands with the kuna-pack porter)
// and by this module's tests; until the memset port and the W4/W6 transform
// halves land, the in-crate (non-test) consumer is absent, so the base reads as
// dead.  This mirrors the upstream `protected` widening for reuse (`constseq.hh`
// `(kuna)` note) — kept whole for the memset porter to extend, NOT re-ported.
#![allow(dead_code)]

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{OpId, VarnodeId};
use crate::userop::{BUILTIN_MEMCPY, BUILTIN_STRNCPY, BUILTIN_WCSNCPY};

/// Helper holding a data-flow edge and optionally a memory offset being COPYed
/// into or from (C++ `ArraySequence::WriteNode`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct WriteNode {
    /// Offset into the memory region.
    pub offset: u64,
    /// PcodeOp moving into/out of the memory region.
    pub op: OpId,
    /// Either input slot (`>= 0`) or output (`-1`).
    pub slot: int4,
    /// Cached block order (`op->getSeqNum().getOrder()`), used by the
    /// `operator<` comparator.  The C++ dereferences the live `PcodeOp *` for the
    /// order at compare time; the Rust port caches it at construction so the
    /// comparator needs no `Funcdata` handle (the order is immutable once the op
    /// is sequenced).
    pub order: u32,
}

impl WriteNode {
    /// C++ `WriteNode(uint8 off,PcodeOp *o,int4 sl)`, with the block order cached.
    pub(crate) fn new(off: u64, op: OpId, slot: int4, order: u32) -> Self {
        WriteNode { offset: off, op, slot, order }
    }
}

/// A sequence of PcodeOps that move data in-to/out-of an array data-type
/// (C++ `class ArraySequence`).
///
/// The fields and base methods are `pub(crate)` so the `kuna_memsetsequence`
/// module can reuse the gathering machinery — see the module-level
/// *Cross-pack visibility* note (the analogue of the upstream `protected`
/// widening).
pub(crate) struct ArraySequence {
    /// Element data-type.
    pub(crate) char_type: Rc<Datatype>,
    /// Number of elements in the final sequence.
    pub(crate) num_elements: int4,
    /// COPY/STORE into the array memory region (block-ordered after
    /// `check_interference`).
    pub(crate) move_ops: Vec<WriteNode>,
    /// Constants collected in a single array.
    pub(crate) byte_array: Vec<u8>,
}

impl ArraySequence {
    /// Minimum number of sequential characters to trigger replacement with
    /// CALLOTHER (C++ `ArraySequence::MINIMUM_SEQUENCE_LENGTH = 4`).
    pub(crate) const MINIMUM_SEQUENCE_LENGTH: int4 = 4;
    /// Maximum number of characters in replacement string
    /// (C++ `ArraySequence::MAXIMUM_SEQUENCE_LENGTH = 0x20000`).
    pub(crate) const MAXIMUM_SEQUENCE_LENGTH: int4 = 0x20000;

    /// Initialize the (empty) sequence with the element data-type
    /// (C++ `ArraySequence::ArraySequence`).  `rootOp`/`block` are tracked by the
    /// concrete driver in the Rust port (the base only needs `char_type`).
    pub(crate) fn new(char_type: Rc<Datatype>) -> Self {
        ArraySequence { char_type, num_elements: 0, move_ops: Vec::new(), byte_array: Vec::new() }
    }

    /// Return `true` if a sequence was found (C++ `ArraySequence::isValid`).
    pub(crate) fn is_valid(&self) -> bool {
        self.num_elements != 0
    }

    /// Check for interfering ops between the two given ops
    /// (C++ static `ArraySequence::interfereBetween`).
    ///
    /// Output Varnodes are read only outside the basic block, so effectively only
    /// LOADs, STOREs, and CALLs can interfere.  Walk from after `start_op` up to
    /// `end_op`; any `special`-eval op that is not one of the benign markers
    /// blocks.  Returns `true` if there is no interference.
    pub(crate) fn interfere_between(data: &Funcdata, start_op: OpId, end_op: OpId) -> bool {
        // startOp = startOp->nextOp();
        let mut cur = match data.op_next_op(start_op) {
            Some(o) => o,
            None => return true, // C++ would loop until end; a null next can't equal end_op here
        };
        while cur != end_op {
            let o = data.obank().get(cur).expect("constseq: stale op");
            if o.get_eval_type() == pcodeop_flags::special {
                let opc = o.code();
                if opc != OpCode::CPUI_INDIRECT
                    && opc != OpCode::CPUI_CALLOTHER
                    && opc != OpCode::CPUI_SEGMENTOP
                    && opc != OpCode::CPUI_CPOOLREF
                    && opc != OpCode::CPUI_NEW
                {
                    return false;
                }
            }
            cur = match data.op_next_op(cur) {
                Some(o2) => o2,
                None => break,
            };
        }
        true
    }

    /// Find the maximal set of ops containing the root with no interfering ops in
    /// between (C++ `ArraySequence::checkInterference`).
    ///
    /// Sorts `move_ops` by block order, locates the root op, walks backward then
    /// forward stopping at the first interfering gap, and truncates `move_ops` to
    /// that window.  `root_op` is supplied by the caller (the concrete driver
    /// holds it).  Returns `false` if the window is shorter than
    /// `MINIMUM_SEQUENCE_LENGTH` or the root op is not present.
    pub(crate) fn check_interference(&mut self, data: &Funcdata, root_op: OpId) -> bool {
        // sort(moveOps) by block order
        self.move_ops.sort_by(|a, b| a.order.cmp(&b.order));
        // for(pos=0;pos<size;++pos) if (moveOps[pos].op == rootOp) break;
        let mut pos = self.move_ops.len();
        for (i, node) in self.move_ops.iter().enumerate() {
            if node.op == root_op {
                pos = i;
                break;
            }
        }
        if pos == self.move_ops.len() {
            return false;
        }
        // Walk backward from pos-1
        let mut cur_op = self.move_ops[pos].op;
        let mut starting_pos: i64 = pos as i64 - 1;
        while starting_pos >= 0 {
            let prev_op = self.move_ops[starting_pos as usize].op;
            if !Self::interfere_between(data, prev_op, cur_op) {
                break;
            }
            cur_op = prev_op;
            starting_pos -= 1;
        }
        starting_pos += 1;
        // Walk forward from pos+1
        cur_op = self.move_ops[pos].op;
        let mut ending_pos = pos + 1;
        while ending_pos < self.move_ops.len() {
            let next_op = self.move_ops[ending_pos].op;
            if !Self::interfere_between(data, cur_op, next_op) {
                break;
            }
            cur_op = next_op;
            ending_pos += 1;
        }
        let starting_pos = starting_pos as usize;
        if (ending_pos as int4) - (starting_pos as int4) < Self::MINIMUM_SEQUENCE_LENGTH {
            return false;
        }
        if starting_pos > 0 {
            for i in starting_pos..ending_pos {
                self.move_ops[i - starting_pos] = self.move_ops[i];
            }
        }
        self.move_ops.truncate(ending_pos - starting_pos);
        true
    }

    /// Put constant values from COPYs into a single byte array
    /// (C++ `ArraySequence::formByteArray`).
    ///
    /// Lay each op's constant input (at `slot`) into `byte_array` by its offset
    /// relative to `root_off`, big- or little-endian.  Mark `used[]` per element
    /// (`1` non-null, `2` null terminator).  Count the leading contiguous run
    /// (allowing one terminator), reject if shorter than the minimum, and if the
    /// run is shorter than the collected ops, drop the trailing ops.  Returns the
    /// number of characters in the contiguous region (0 = too small).
    ///
    /// This is the constant-sequence gathering ORDER reused by
    /// `kuna_memsetsequence`.
    pub(crate) fn form_byte_array(
        &mut self,
        data: &Funcdata,
        sz: int4,
        slot: int4,
        root_off: u64,
        big_endian: bool,
    ) -> int4 {
        let sz_us = sz.max(0) as usize;
        self.byte_array = vec![0u8; sz_us];
        let mut used: Vec<u8> = vec![0u8; sz_us];
        let el_size = self.char_type.get_size();
        for i in 0..self.move_ops.len() {
            // bytePos = moveOps[i].offset - rootOff;   (int4, can be negative)
            let byte_pos: int4 = (self.move_ops[i].offset.wrapping_sub(root_off)) as int4;
            if byte_pos < 0 || byte_pos + el_size > sz {
                continue;
            }
            let mut val = vn_get_offset(data, op_get_in(data, self.move_ops[i].op, slot));
            // Mark byte as used; a 2 indicates a null terminator
            used[byte_pos as usize] = if val == 0 { 2 } else { 1 };
            if big_endian {
                for j in 0..el_size {
                    let b = ((val >> ((el_size - 1 - j) * 8)) & 0xff) as u8;
                    self.byte_array[(byte_pos + j) as usize] = b;
                }
            } else {
                for j in 0..el_size {
                    self.byte_array[(byte_pos + j) as usize] = val as u8;
                    val >>= 8;
                }
            }
        }
        let big_el_size = self.char_type.get_align_size();
        let max_el = (used.len() as int4) / big_el_size;
        let mut count: int4 = 0;
        while count < max_el {
            let val = used[(count * big_el_size) as usize];
            if val != 1 {
                // Count number of characters not including null terminator
                if val == 2 {
                    count += 1; // Allow a single null terminator
                }
                break;
            }
            count += 1;
        }
        if count < Self::MINIMUM_SEQUENCE_LENGTH {
            return 0;
        }
        if count != self.move_ops.len() as int4 {
            let max_off = root_off.wrapping_add((count * big_el_size) as u64);
            let mut final_ops: Vec<WriteNode> = Vec::new();
            for i in 0..self.move_ops.len() {
                if self.move_ops[i].offset < max_off {
                    final_ops.push(self.move_ops[i]);
                }
            }
            self.move_ops = final_ops;
        }
        count
    }

    /// Pick either strncpy, wcsncpy, or memcpy used to copy the string
    /// (C++ `ArraySequence::selectStringCopyFunction`).  Returns
    /// `(builtin_id, index)` where `index` is the count passed back.
    ///
    /// SEAM(W6): the C++ compares `charType` *by pointer identity* against
    /// `types->getTypeChar(getSizeOfChar())` / `getTypeChar(getSizeOfWChar())`.
    /// The live `TypeFactory` is not wired into `Architecture` yet, so the
    /// identity match cannot be performed; this falls through to the C++ default
    /// `BUILTIN_MEMCPY` with the byte count (`numElements * getAlignSize()`).
    /// The narrow/wide-char selection is preserved structurally for restoration.
    /// Recorded as a loss.
    pub(crate) fn select_string_copy_function(&self, _data: &Funcdata) -> (uintb, int4) {
        // SEAM(W6): TypeFactory not reachable from Architecture.
        // if (charType == types->getTypeChar(getSizeOfChar()))  -> (BUILTIN_STRNCPY, numElements)
        // if (charType == types->getTypeChar(getSizeOfWChar())) -> (BUILTIN_WCSNCPY, numElements)
        let _narrow = BUILTIN_STRNCPY; // kept live for the restored identity check
        let _wide = BUILTIN_WCSNCPY;
        let index = self.num_elements * self.char_type.get_align_size();
        (BUILTIN_MEMCPY as uintb, index)
    }
}

// =============================================================================
// RuleStringCopy / RuleStringStore (constseq.cc:969-1029)
// =============================================================================
//
// The two drivers' early guards reachable on the merged tree are ported here;
// the `StringSequence`/`HeapSequence` construction + `transform()` that the
// guards gate reach W4 (symbol table `queryContainer`, the `getScopeLocal`
// `ScopeLocal`) and W4/W6 (the union-aware `getTypeDefFacing`/`getTypeReadFacing`
// type-facing, `getInternalString`, `getTypePointer*`, `constructTypedPointer`)
// — exactly the *Deferred halves* the module docs ledger.  Until those land the
// `applyOp` bodies decline after the reachable guards, byte-identical to the
// rule being disabled (the `kuna_memsetsequence::RuleMemsetCopy` convention).
// The guards that *are* reachable (the constant-input test) run faithfully so
// the seam is as narrow as the C++ permits.

/// (constseq) Replace a sequence of COPY ops moving single characters with a
/// CALLOTHER copying a whole string (C++ `class RuleStringCopy`).
///
/// Given a root COPY of a constant character, search for other COPYs in the same
/// basic block that form a sequence interpretable as a single string, and
/// replace the sequence with a single `memcpy`/`wcsncpy` user-op.
pub struct RuleStringCopy {
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleStringCopy {
    /// Construct in group `g` (C++ `RuleStringCopy(const string &g)`:
    /// `Rule(g, 0, "stringcopy")`).
    pub fn new(g: impl Into<String>) -> RuleStringCopy {
        RuleStringCopy { group: g.into() }
    }
}

impl Rule for RuleStringCopy {
    /// C++ `RuleStringCopy::getOpList`: `oplist.push_back(CPUI_COPY);`
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_COPY]
    }

    /// C++ `RuleStringCopy::clone`:
    /// `if (!grouplist.contains(getGroup())) return 0;`
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleStringCopy { group: self.group.clone() }))
    }

    /// C++ `RuleStringCopy::applyOp` (`constseq.cc:981`).
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->getIn(0)->isConstant()) return 0;  // Constant
        let in0 = match data.obank().get(op).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return 0,
        };
        if !data.vbank().get(in0).map(|v| v.is_constant()).unwrap_or(false) {
            return 0;
        }
        // SEAM(W4/W6): the remaining guards and the StringSequence build/transform
        // reach the W6 type-facing factory and the W4 symbol table (see the
        // section note above).  Until they land the body declines, which is
        // byte-identical to the rule being disabled.
        //
        //   Varnode *outvn = op->getOut();
        //   Datatype *ct = outvn->getTypeDefFacing();
        //   if (!ct->isCharPrint()) return 0;
        //   if (ct->isOpaqueString()) return 0;
        //   if (!outvn->isAddrTied()) return 0;
        //   SymbolEntry *entry = data.getScopeLocal()->queryContainer(
        //       outvn->getAddr(), outvn->getSize(), op->getAddr());
        //   if (entry == 0) return 0;
        //   StringSequence sequence(data,ct,entry,op,outvn->getAddr());
        //   if (!sequence.isValid()) return 0;
        //   if (!sequence.transform()) return 0;
        //   return 1;
        let _ = data;
        0
    }
}

/// (constseq) Replace a sequence of STORE ops moving single characters with a
/// CALLOTHER copying a whole string (C++ `class RuleStringStore`).
///
/// Given a root STORE of a constant character, search for other STOREs in the
/// same basic block off the same base pointer that form a sequence
/// interpretable as a single string, and replace the STOREs with a single
/// `strncpy`/`wcsncpy` user-op.
pub struct RuleStringStore {
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleStringStore {
    /// Construct in group `g` (C++ `RuleStringStore(const string &g)`:
    /// `Rule(g, 0, "stringstore")`).
    pub fn new(g: impl Into<String>) -> RuleStringStore {
        RuleStringStore { group: g.into() }
    }
}

impl Rule for RuleStringStore {
    /// C++ `RuleStringStore::getOpList`: `oplist.push_back(CPUI_STORE);`
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_STORE]
    }

    /// C++ `RuleStringStore::clone`:
    /// `if (!grouplist.contains(getGroup())) return 0;`
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleStringStore { group: self.group.clone() }))
    }

    /// C++ `RuleStringStore::applyOp` (`constseq.cc:1013`).
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->getIn(2)->isConstant()) return 0;  // Constant
        let in2 = match data.obank().get(op).and_then(|o| o.get_in(2)) {
            Some(v) => v,
            None => return 0,
        };
        if !data.vbank().get(in2).map(|v| v.is_constant()).unwrap_or(false) {
            return 0;
        }
        // SEAM(W4/W6): the type-facing pointer guard and the HeapSequence
        // build/transform reach the W6 type-facing factory (`getTypeReadFacing`,
        // `getPtrTo`) and the heap-STORE machinery (see the section note above).
        // Until they land the body declines, byte-identical to disabled.
        //
        //   Varnode *ptrvn = op->getIn(1);
        //   Datatype *ct = ptrvn->getTypeReadFacing(op);
        //   if (ct->getMetatype() != TYPE_PTR) return 0;
        //   ct = ((TypePointer *)ct)->getPtrTo();
        //   if (!ct->isCharPrint()) return 0;
        //   if (ct->isOpaqueString()) return 0;
        //   HeapSequence sequence(data,ct,op);
        //   if (!sequence.isValid()) return 0;
        //   if (!sequence.transform()) return 0;
        //   return 1;
        let _ = (data, type_metatype::TYPE_PTR);
        0
    }
}

/// Per-file registration rows in C++ definition order
/// (`RuleStringCopy` then `RuleStringStore`).
///
/// Both ship in the `"constsequence"` group (the `universalAction` cleanup-pool
/// slot, C++ `coreaction.cc:5987-5988`); the schedule re-registers them under
/// that slot group.  The per-file placeholder is `"analysis"`.
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "analysis", ctor: || Box::new(RuleStringCopy::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleStringStore::new("analysis")) },
    ]
}

// =============================================================================
// Small IR read helpers (mirrors the double.rs/prefersplit.rs idiom)
// =============================================================================

#[inline]
fn vn_get_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("constseq: stale vn").get_offset()
}
#[inline]
fn op_get_in(data: &Funcdata, op: OpId, slot: int4) -> VarnodeId {
    data.obank()
        .get(op)
        .expect("constseq: stale op")
        .get_in(slot)
        .expect("constseq: missing input")
}

#[cfg(test)]
mod tests;
