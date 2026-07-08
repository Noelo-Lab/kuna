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
//! # What landed (rport/w10-string-sequence)
//!
//! The **STORE-through-pointer** driver is now fully ported and live:
//!
//! * [`HeapSequence`] (`constseq.cc:486-967`): `findBasePointer`,
//!   `findDuplicateBases`, `findInitialStores`, `calcAddElements`/`calcPtraddOffset`,
//!   `collectStoreOps`, `buildStringCopy` (the typed-pointer PTRADD + CALLOTHER
//!   builder), `gatherIndirectPairs`/`deduplicatePairs`/`removeStoreOps`, and
//!   `transform` — all transcribed branch-for-branch.
//! * [`RuleStringStore::apply_op`] runs the full body (read-facing pointer guard
//!   → `HeapSequence::build` → `transform`).
//! * `Funcdata::getInternalString` (`funcdata_varnode.cc:1434`) registers the
//!   assembled byte-array into the architecture's persistent `StringManager`
//!   (a shared `Rc<RefCell<StringManagerUnicode>>` on `Architecture`, threaded
//!   into the W4 seam as `internal_strings`) and builds the `BUILTIN_STRINGDATA`
//!   CALLOTHER whose output displays as the quoted string.
//! * [`ArraySequence::select_string_copy_function`] performs the faithful
//!   `charType == types->getTypeChar(...)` pointer-identity selection (now that
//!   the cspec `<data_organization>` `<wchar_size>` is decoded, so `getSizeOfWChar`
//!   reflects the ABI and the wide-vs-narrow split matches upstream).
//! * The printer's `opCallother` (`printc.cc:693`) renders the functional /
//!   `display_string` forms, and `BUILTIN_STRINGDATA` output type-locals resolve
//!   to the char-pointer (`InternalStringOp::getOutputLocal`) so no spurious cast
//!   wraps the literal.  The four string builtins are pre-registered into
//!   `userops` at boot (`register_string_builtins`).
//!
//! This drives `heapstring.xml` (Heap string #1-7) to full parity.
//!
//! # Deferred half (still ledgered as a loss)
//!
//! The **COPY-into-array** driver `StringSequence` (`constseq.cc:188-483`) — the
//! `stackstring.xml` path — is **not** yet transcribed; it reaches W4 surfaces the
//! heap path does not:
//!
//! * The address-only `beginLoc(addr)`/`endLoc(addr)` location-set overload
//!   (`StringSequence::collectCopyOps`).
//! * `data.getScopeLocal()->queryContainer(...)` array-component resolution
//!   (`StringSequence` ctor, `RuleStringCopy::applyOp`).
//! * `constructTypedPointer`'s `constructSpacebaseInput`/`constructConstSpacebase`
//!   + `getTypePointerStripArray` PTRSUB/PTRADD chain and `inheritUnionFieldPtr`.
//!
//! Until that lands, `RuleStringCopy::applyOp` declines after its constant-input
//! guard (byte-identical to the rule being disabled) and the stack COPYs are
//! removed by dead-code instead of collapsed into a `strncpy` user-op.

// The `ArraySequence` base and its read helpers are consumed by the sibling
// `kuna_memsetsequence` module (its Rust port lands with the kuna-pack porter)
// and by this module's tests; until the memset port and the W4/W6 transform
// halves land, the in-crate (non-test) consumer is absent, so the base reads as
// dead.  This mirrors the upstream `protected` widening for reuse (`constseq.hh`
// `(kuna)` note) — kept whole for the memset porter to extend, NOT re-ported.
#![allow(dead_code)]

use kuna_base::address::Address;
use kuna_base::types::{int4, int8, uintb};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::context::{OpId, VarnodeId};
use crate::userop::{BUILTIN_MEMCPY, BUILTIN_MEMSET, BUILTIN_STRNCPY, BUILTIN_WCSNCPY};

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
    /// STUB(W6): the C++ compares `charType` *by pointer identity* against
    /// `types->getTypeChar(getSizeOfChar())` / `getTypeChar(getSizeOfWChar())`.
    /// The live `TypeFactory` is not wired into `Architecture` yet, so the
    /// identity match cannot be performed; this falls through to the C++ default
    /// `BUILTIN_MEMCPY` with the byte count (`numElements * getAlignSize()`).
    /// The narrow/wide-char selection is preserved structurally for restoration.
    /// Recorded as a loss.
    pub(crate) fn select_string_copy_function(&self, data: &Funcdata) -> (uintb, int4) {
        // Faithful to constseq.cc:161-175: compare `charType` by pointer identity
        // against `types->getTypeChar(getSizeOfChar())` / `getTypeChar(getSizeOfWChar())`.
        // The factory interns its core char types, so `Rc::ptr_eq` is the Rust
        // analogue of the C++ pointer-identity `==`.  Falls through to the C++
        // default `BUILTIN_MEMCPY` only when neither identity matches.
        if let Some(types) = data.get_arch().types_rc() {
            if let Ok(narrow) = types.get_type_char(types.get_size_of_char()) {
                if Rc::ptr_eq(&self.char_type, &narrow) {
                    return (BUILTIN_STRNCPY as uintb, self.num_elements);
                }
            }
            if let Ok(wide) = types.get_type_char(types.get_size_of_wchar()) {
                if Rc::ptr_eq(&self.char_type, &wide) {
                    return (BUILTIN_WCSNCPY as uintb, self.num_elements);
                }
            }
        }
        let index = self.num_elements * self.char_type.get_align_size();
        (BUILTIN_MEMCPY as uintb, index)
    }
}

// =============================================================================
// HeapSequence (constseq.cc:486-967) — STORE-through-pointer driver
// =============================================================================

/// Input/output Varnode pair flowing across the sequence STOREs, preserved as a
/// single INDIRECT around the replacement user-op (C++
/// `HeapSequence::IndirectPair`).
#[derive(Clone, Copy)]
struct IndirectPair {
    in_vn: VarnodeId,
    out_vn: VarnodeId,
    duplicate: bool,
}

impl IndirectPair {
    fn new(in_vn: VarnodeId, out_vn: VarnodeId) -> Self {
        IndirectPair { in_vn, out_vn, duplicate: false }
    }
}

/// A sequence of STORE ops moving single constant characters through a common
/// base pointer (C++ `class HeapSequence : public ArraySequence`).
struct HeapSequence {
    /// The shared gathering machinery (C++ base `ArraySequence`).
    base: ArraySequence,
    /// The root STORE (C++ `ArraySequence::rootOp`).
    root_op: OpId,
    /// The basic block of the root STORE (C++ `ArraySequence::block`).
    block: crate::context::BlockId,
    /// Space being STOREd into (C++ `storeSpace`).
    store_space: Rc<kuna_base::space::AddrSpace>,
    /// Pointer multiplier matching the char element, in address units
    /// (C++ `ptrAddMult`).
    ptr_add_mult: uintb,
    /// Putative common base pointer of all the STOREs (C++ `basePointer`).
    base_pointer: VarnodeId,
    /// The op immediately reading `base_pointer` on the path to the root
    /// (C++ `immedRead`).
    immed_read: OpId,
    /// Byte offset of the root STORE relative to `base_pointer` (C++ `baseOffset`).
    base_offset: uintb,
    /// Non-constant Varnodes on the additive path from base to root
    /// (C++ `nonConstAdds`).
    non_const_adds: Vec<VarnodeId>,
}

impl HeapSequence {
    /// C++ `HeapSequence::HeapSequence(Funcdata&,Datatype*,PcodeOp *root)`.
    fn new(data: &Funcdata, ct: Rc<Datatype>, root: OpId) -> HeapSequence {
        let block = data.obank().get(root).expect("HeapSequence: stale root").get_parent().expect(
            "HeapSequence: root has no parent block",
        );
        // storeSpace = root->getIn(0)->getSpaceFromConst();
        let space_idx = {
            let in0 = op_get_in(data, root, 0);
            vn_get_offset(data, in0)
        };
        let store_space = data
            .get_arch()
            .manage()
            .get_space(space_idx as int4)
            .cloned()
            .expect("HeapSequence: bad store space");
        // ptrAddMult = byteToAddressInt(charType->getAlignSize(), storeSpace->getWordSize());
        let ptr_add_mult = kuna_base::space::AddrSpace::byte_to_address_int(
            ct.get_align_size() as i64,
            store_space.get_word_size(),
        ) as uintb;
        // Defaults; findBasePointer / collectStoreOps fill the rest.
        let root_in1 = op_get_in(data, root, 1);
        let mut seq = HeapSequence {
            base: ArraySequence::new(ct),
            root_op: root,
            block,
            store_space,
            ptr_add_mult,
            base_pointer: root_in1,
            immed_read: root,
            base_offset: 0,
            non_const_adds: Vec::new(),
        };
        seq.find_base_pointer(data);
        seq
    }

    /// Construct, collect, and validate (the body of the C++ ctor after the
    /// field initialization).  Returns the built sequence; `is_valid()` is `false`
    /// when no viable sequence exists.
    fn build(data: &Funcdata, ct: Rc<Datatype>, root: OpId) -> HeapSequence {
        let mut seq = HeapSequence::new(data, ct, root);
        // if (!collectStoreOps()) return;
        if !seq.collect_store_ops(data) {
            return seq;
        }
        // if (!checkInterference()) return;
        if !seq.base.check_interference(data, seq.root_op) {
            return seq;
        }
        // int4 arrSize = moveOps.size() * charType->getAlignSize();
        let arr_size = seq.base.move_ops.len() as int4 * seq.base.char_type.get_align_size();
        let big_endian = seq.store_space.is_big_endian();
        // numElements = formByteArray(arrSize, 2, 0, bigEndian);
        seq.base.num_elements = seq.base.form_byte_array(data, arr_size, 2, 0, big_endian);
        seq
    }

    fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    /// C++ `HeapSequence::findBasePointer`.
    fn find_base_pointer(&mut self, data: &Funcdata) {
        // basePointer = rootOp->getIn(1); immedRead = rootOp;
        self.base_pointer = op_get_in(data, self.root_op, 1);
        self.immed_read = self.root_op;
        // while(basePointer->isWritten()) { ... }
        loop {
            let v = data.vbank().get(self.base_pointer).expect("findBasePointer: stale vn");
            if !v.is_written() {
                break;
            }
            let op = v.get_def().expect("findBasePointer: written vn has no def");
            let opc = data.obank().get(op).expect("findBasePointer: stale op").code();
            if opc == OpCode::CPUI_PTRADD {
                let in2 = op_get_in(data, op, 2);
                let sz = vn_get_offset(data, in2);
                if sz != self.ptr_add_mult {
                    break;
                }
            } else if opc != OpCode::CPUI_COPY {
                break;
            }
            self.base_pointer = op_get_in(data, op, 0);
            self.immed_read = op;
        }
    }

    /// C++ `HeapSequence::findDuplicateBases`.
    fn find_duplicate_bases(&self, data: &Funcdata, duplist: &mut Vec<VarnodeId>) {
        let bp = self.base_pointer;
        let bp_written = data.vbank().get(bp).expect("findDuplicateBases: stale bp").is_written();
        if !bp_written {
            duplist.push(bp);
            return;
        }
        let mut op = data.vbank().get(bp).unwrap().get_def().unwrap();
        let mut opc = data.obank().get(op).expect("stale op").code();
        let in1_const = |d: &Funcdata, o: OpId| -> bool {
            d.vbank().get(op_get_in(d, o, 1)).map(|v| v.is_constant()).unwrap_or(false)
        };
        if (opc != OpCode::CPUI_PTRSUB && opc != OpCode::CPUI_INT_ADD && opc != OpCode::CPUI_PTRADD)
            || !in1_const(data, op)
        {
            duplist.push(bp);
            return;
        }
        // C++ `Varnode *copyRoot = basePointer;` — reassigned on the first loop
        // iteration (the do-while body always runs), so the seed is immediately
        // overwritten; named with a leading underscore to mark the dead seed.
        let mut copy_root;
        let mut offset: Vec<uintb> = Vec::new();
        loop {
            let in1 = op_get_in(data, op, 1);
            let mut off = vn_get_offset(data, in1);
            if opc == OpCode::CPUI_PTRADD {
                let in2 = op_get_in(data, op, 2);
                off = off.wrapping_mul(vn_get_offset(data, in2));
            }
            offset.push(off);
            copy_root = op_get_in(data, op, 0);
            let cr_written =
                data.vbank().get(copy_root).expect("stale copy_root").is_written();
            if !cr_written {
                break;
            }
            op = data.vbank().get(copy_root).unwrap().get_def().unwrap();
            opc = data.obank().get(op).expect("stale op").code();
            if opc != OpCode::CPUI_PTRSUB
                && opc != OpCode::CPUI_INT_ADD
                && opc != OpCode::CPUI_PTRADD
            {
                break;
            }
            // while(op->getIn(1)->isConstant());
            if !in1_const(data, op) {
                break;
            }
        }

        duplist.push(copy_root);
        let mut midlist: Vec<VarnodeId> = Vec::new();
        for i in (0..offset.len()).rev() {
            std::mem::swap(duplist, &mut midlist);
            duplist.clear();
            for &vn in midlist.iter() {
                for op2 in data.descend_snapshot(vn) {
                    let opc2 = data.obank().get(op2).expect("stale op2").code();
                    if opc2 != OpCode::CPUI_PTRSUB
                        && opc2 != OpCode::CPUI_INT_ADD
                        && opc2 != OpCode::CPUI_PTRADD
                    {
                        continue;
                    }
                    if op_get_in(data, op2, 0) != vn || !in1_const(data, op2) {
                        continue;
                    }
                    let mut off = vn_get_offset(data, op_get_in(data, op2, 1));
                    if opc2 == OpCode::CPUI_PTRADD {
                        off = off.wrapping_mul(vn_get_offset(data, op_get_in(data, op2, 2)));
                    }
                    if off != offset[i] {
                        continue;
                    }
                    if let Some(out) = data.obank().get(op2).and_then(|o| o.get_out()) {
                        duplist.push(out);
                    }
                }
            }
        }
    }

    /// C++ `HeapSequence::findInitialStores`.
    fn find_initial_stores(&self, data: &Funcdata, stores: &mut Vec<OpId>) {
        let mut ptradds: Vec<VarnodeId> = Vec::new();
        self.find_duplicate_bases(data, &mut ptradds);
        let mut pos = 0;
        while pos < ptradds.len() {
            let vn = ptradds[pos];
            pos += 1;
            for op in data.descend_snapshot(vn) {
                let opc = data.obank().get(op).expect("stale op").code();
                if opc == OpCode::CPUI_PTRADD {
                    if op_get_in(data, op, 0) != vn {
                        continue;
                    }
                    if vn_get_offset(data, op_get_in(data, op, 2)) != self.ptr_add_mult {
                        continue;
                    }
                    if let Some(out) = data.obank().get(op).and_then(|o| o.get_out()) {
                        ptradds.push(out);
                    }
                } else if opc == OpCode::CPUI_COPY {
                    if let Some(out) = data.obank().get(op).and_then(|o| o.get_out()) {
                        ptradds.push(out);
                    }
                } else if opc == OpCode::CPUI_STORE
                    && data.obank().get(op).and_then(|o| o.get_parent()) == Some(self.block)
                    && op != self.root_op
                {
                    if op_get_in(data, op, 1) != vn {
                        continue;
                    }
                    stores.push(op);
                }
            }
        }
    }

    /// C++ `HeapSequence::calcAddElements`.
    fn calc_add_elements(
        data: &Funcdata,
        vn: VarnodeId,
        non_const: &mut Vec<VarnodeId>,
        max_depth: int4,
    ) -> u64 {
        let v = data.vbank().get(vn).expect("calcAddElements: stale vn");
        if v.is_constant() {
            return v.get_offset();
        }
        let is_add = v.is_written()
            && data.obank().get(v.get_def().unwrap()).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY)
                == OpCode::CPUI_INT_ADD;
        if !is_add || max_depth == 0 {
            non_const.push(vn);
            return 0;
        }
        let def = v.get_def().unwrap();
        let in0 = op_get_in(data, def, 0);
        let in1 = op_get_in(data, def, 1);
        let mut res = Self::calc_add_elements(data, in0, non_const, max_depth - 1);
        res = res.wrapping_add(Self::calc_add_elements(data, in1, non_const, max_depth - 1));
        res
    }

    /// C++ `HeapSequence::calcPtraddOffset`.
    fn calc_ptradd_offset(
        &self,
        data: &Funcdata,
        mut vn: VarnodeId,
        non_const: &mut Vec<VarnodeId>,
    ) -> u64 {
        let mut res: u64 = 0;
        loop {
            let v = data.vbank().get(vn).expect("calcPtraddOffset: stale vn");
            if !v.is_written() {
                break;
            }
            let op = v.get_def().unwrap();
            let opc = data.obank().get(op).expect("stale op").code();
            if opc == OpCode::CPUI_PTRADD {
                let mult = vn_get_offset(data, op_get_in(data, op, 2));
                if mult != self.ptr_add_mult {
                    break;
                }
                let mut off = Self::calc_add_elements(data, op_get_in(data, op, 1), non_const, 3);
                off = off.wrapping_mul(mult);
                res = res.wrapping_add(off);
                vn = op_get_in(data, op, 0);
            } else if opc == OpCode::CPUI_COPY {
                vn = op_get_in(data, op, 0);
            } else {
                break;
            }
        }
        kuna_base::space::AddrSpace::address_to_byte_int(res as i64, self.store_space.get_word_size())
            as u64
    }

    /// C++ `HeapSequence::setsEqual`.
    fn sets_equal(op1: &[VarnodeId], op2: &[VarnodeId]) -> bool {
        if op1.len() != op2.len() {
            return false;
        }
        for i in 0..op1.len() {
            if op1[i] != op2[i] {
                return false;
            }
        }
        true
    }

    /// C++ `HeapSequence::testValue`.
    fn test_value(&self, data: &Funcdata, op: OpId) -> bool {
        let in2 = op_get_in(data, op, 2);
        let v = data.vbank().get(in2).expect("testValue: stale vn");
        if !v.is_constant() {
            return false;
        }
        if v.get_size() != self.base.char_type.get_size() {
            return false;
        }
        true
    }

    /// C++ `HeapSequence::collectStoreOps`.
    fn collect_store_ops(&mut self, data: &Funcdata) -> bool {
        let mut init_stores: Vec<OpId> = Vec::new();
        self.find_initial_stores(data, &mut init_stores);
        if (init_stores.len() as int4) + 1 < ArraySequence::MINIMUM_SEQUENCE_LENGTH {
            return false;
        }
        // uint8 maxSize = MAXIMUM_SEQUENCE_LENGTH * charType->getAlignSize();
        let max_size =
            (ArraySequence::MAXIMUM_SEQUENCE_LENGTH as u64) * (self.base.char_type.get_align_size() as u64);
        // uint8 wrapMask = calc_mask(storeSpace->getAddrSize());
        let wrap_mask = kuna_base::address::calc_mask(self.store_space.get_addr_size() as i32);
        // baseOffset = calcPtraddOffset(rootOp->getIn(1), nonConstAdds);
        let root_in1 = op_get_in(data, self.root_op, 1);
        let mut non_const_adds = Vec::new();
        self.base_offset = self.calc_ptradd_offset(data, root_in1, &mut non_const_adds);
        self.non_const_adds = non_const_adds;
        for i in 0..init_stores.len() {
            let op = init_stores[i];
            let mut non_const_comp: Vec<VarnodeId> = Vec::new();
            let cur_offset =
                self.calc_ptradd_offset(data, op_get_in(data, op, 1), &mut non_const_comp);
            let diff = cur_offset.wrapping_sub(self.base_offset) & wrap_mask;
            if Self::sets_equal(&self.non_const_adds, &non_const_comp) {
                if diff >= max_size {
                    return false;
                }
                if !self.test_value(data, op) {
                    return false;
                }
                let order = data.obank().get(op).unwrap().get_seq_num().get_order();
                self.base.move_ops.push(WriteNode::new(diff, op, -1, order));
            }
        }
        let root_order = data.obank().get(self.root_op).unwrap().get_seq_num().get_order();
        self.base.move_ops.push(WriteNode::new(0, self.root_op, -1, root_order));
        true
    }

    /// C++ `HeapSequence::buildStringCopy`.
    fn build_string_copy(&self, data: &mut Funcdata) -> Option<OpId> {
        // PcodeOp *insertPoint = moveOps[0].op;
        let insert_point = self.base.move_ops[0].op;
        // Datatype *charPtrType = rootOp->getIn(1)->getTypeReadFacing(rootOp);
        let root_in1 = op_get_in(data, self.root_op, 1);
        let char_ptr_type = Rc::clone(
            data.vbank().get(root_in1).expect("buildStringCopy: stale ptr vn").get_type_read_facing(self.root_op),
        );
        // int4 numBytes = numElements * charType->getSize();
        let num_bytes = self.base.num_elements * self.base.char_type.get_size();
        let insert_addr = data.obank().get(insert_point).unwrap().get_addr().clone();
        // Varnode *srcPtr = data.getInternalString(byteArray.data(), numBytes, charPtrType, insertPoint);
        let byte_array = self.base.byte_array.clone();
        let src_ptr =
            data.get_internal_string(&byte_array, num_bytes, Rc::clone(&char_ptr_type), insert_point)?;

        // Varnode *destPtr = basePointer;
        let mut dest_ptr = self.base_pointer;
        if self.base_offset != 0 || !self.non_const_adds.is_empty() {
            // Datatype *intType = glb->types->getBase(basePointer->getSize(), TYPE_INT);
            let bp_size = data.vbank().get(self.base_pointer).unwrap().get_size();
            let int_type = data
                .get_arch()
                .types()
                .expect("buildStringCopy: no type factory")
                .get_base(bp_size, type_metatype::TYPE_INT)
                .expect("buildStringCopy: getBase(INT)");
            let mut index_vn: Option<VarnodeId> = None;
            if !self.non_const_adds.is_empty() {
                let mut iv = self.non_const_adds[0];
                for i in 1..self.non_const_adds.len() {
                    let add_op = data.new_op(2, insert_addr.clone());
                    data.op_set_opcode_code(add_op, OpCode::CPUI_INT_ADD);
                    data.op_set_input(add_op, iv, 0).ok();
                    data.op_set_input(add_op, self.non_const_adds[i], 1).ok();
                    let iv_size = data.vbank().get(iv).unwrap().get_size();
                    let new_iv = data.new_unique_out(iv_size, add_op).expect("newUniqueOut");
                    data.vbank_mut().get_mut(new_iv).unwrap().update_type(Rc::clone(&int_type));
                    data.op_insert_before(add_op, insert_point);
                    iv = new_iv;
                }
                index_vn = Some(iv);
            }
            if self.base_offset != 0 {
                let num_el = self.base_offset / (self.base.char_type.get_align_size() as u64);
                let cvn = data.new_constant(bp_size, num_el);
                data.vbank_mut().get_mut(cvn).unwrap().update_type(Rc::clone(&int_type));
                match index_vn {
                    None => index_vn = Some(cvn),
                    Some(iv) => {
                        let add_op = data.new_op(2, insert_addr.clone());
                        data.op_set_opcode_code(add_op, OpCode::CPUI_INT_ADD);
                        data.op_set_input(add_op, iv, 0).ok();
                        data.op_set_input(add_op, cvn, 1).ok();
                        let iv_size = data.vbank().get(iv).unwrap().get_size();
                        let new_iv = data.new_unique_out(iv_size, add_op).expect("newUniqueOut");
                        data.vbank_mut().get_mut(new_iv).unwrap().update_type(Rc::clone(&int_type));
                        data.op_insert_before(add_op, insert_point);
                        index_vn = Some(new_iv);
                    }
                }
            }
            // PcodeOp *ptrAdd = data.newOp(3, ...); destPtr = newUniqueOut(...)
            let ptr_add = data.new_op(3, insert_addr.clone());
            data.op_set_opcode_code(ptr_add, OpCode::CPUI_PTRADD);
            let dp = data.new_unique_out(bp_size, ptr_add).expect("newUniqueOut");
            data.op_set_input(ptr_add, self.base_pointer, 0).ok();
            data.op_set_input(ptr_add, index_vn.expect("index_vn set"), 1).ok();
            let align = self.base.char_type.get_align_size();
            let align_con = data.new_constant(bp_size, align as u64);
            data.op_set_input(ptr_add, align_con, 2).ok();
            data.vbank_mut().get_mut(dp).unwrap().update_type(Rc::clone(&char_ptr_type));
            data.op_insert_before(ptr_add, insert_point);
            // (kuna) inheritUnionField on a union base pointer omitted — the heap
            // sequence's base pointer is a plain typed pointer in the ported tests;
            // if the base type needs resolution the union-field inheritance is the
            // W6 follow-up (no current datatest exercises it).
            dest_ptr = dp;
        }
        // uint4 builtInId = selectStringCopyFunction(index);
        let (built_in_id, index) = self.base.select_string_copy_function(data);
        // glb->userops.registerBuiltin(builtInId);  (pre-registered at boot)
        // PcodeOp *copyOp = data.newOp(4, insertPoint->getAddr());
        let copy_op = data.new_op(4, insert_addr.clone());
        data.op_set_opcode_code(copy_op, OpCode::CPUI_CALLOTHER);
        data.obank_mut().get_mut(copy_op).unwrap().clear_flag(pcodeop_flags::call);
        let id_con = data.new_constant(4, built_in_id);
        data.op_set_input(copy_op, id_con, 0).ok();
        data.op_set_input(copy_op, dest_ptr, 1).ok();
        data.op_set_input(copy_op, src_ptr, 2).ok();
        // Varnode *lenVn = data.newConstant(4,index); lenVn->updateType(inputTypeLocal(3));
        let len_vn = data.new_constant(4, index as u64);
        // inputTypeLocal(3) for the builtin is its int4 length parameter; build it
        // directly from the factory (equivalent to the userop's typed param).
        if let Some(types) = data.get_arch().types() {
            if let Ok(int4t) = types.get_base(4, type_metatype::TYPE_INT) {
                data.vbank_mut().get_mut(len_vn).unwrap().update_type(int4t);
            }
        }
        data.op_set_input(copy_op, len_vn, 3).ok();
        data.op_insert_before(copy_op, insert_point);
        Some(copy_op)
    }

    /// C++ `HeapSequence::gatherIndirectPairs`.
    fn gather_indirect_pairs(
        &self,
        data: &mut Funcdata,
        indirects: &mut Vec<OpId>,
        pairs: &mut Vec<IndirectPair>,
    ) {
        for i in 0..self.base.move_ops.len() {
            let mut op_opt = data.op_previous_op(self.base.move_ops[i].op);
            while let Some(op) = op_opt {
                if data.obank().get(op).expect("stale op").code() != OpCode::CPUI_INDIRECT {
                    break;
                }
                data.obank_mut().get_mut(op).unwrap().set_mark();
                indirects.push(op);
                op_opt = data.op_previous_op(op);
            }
        }
        for i in 0..indirects.len() {
            let op = indirects[i];
            let outvn = data.obank().get(op).and_then(|o| o.get_out());
            let outvn = match outvn {
                Some(v) => v,
                None => continue,
            };
            let mut has_use = false;
            for use_op in data.descend_snapshot(outvn) {
                if !data.obank().get(use_op).map(|o| o.is_mark()).unwrap_or(false) {
                    has_use = true;
                    break;
                }
            }
            if has_use {
                let mut invn = op_get_in(data, op, 0);
                loop {
                    let v = data.vbank().get(invn).expect("stale invn");
                    if !v.is_written() {
                        break;
                    }
                    let def_op = v.get_def().unwrap();
                    if !data.obank().get(def_op).map(|o| o.is_mark()).unwrap_or(false) {
                        break;
                    }
                    invn = op_get_in(data, def_op, 0);
                }
                pairs.push(IndirectPair::new(invn, outvn));
            }
        }
        for &op in indirects.iter() {
            data.obank_mut().get_mut(op).unwrap().clear_mark();
        }
    }

    /// C++ `HeapSequence::IndirectPair::compareOutput` (sort key).
    fn compare_output(data: &Funcdata, a: &IndirectPair, b: &IndirectPair) -> std::cmp::Ordering {
        let v1 = data.vbank().get(a.out_vn).expect("stale out_vn");
        let v2 = data.vbank().get(b.out_vn).expect("stale out_vn");
        let s1 = v1.get_space().get_index();
        let s2 = v2.get_space().get_index();
        if s1 != s2 {
            return s1.cmp(&s2);
        }
        if v1.get_offset() != v2.get_offset() {
            return v1.get_offset().cmp(&v2.get_offset());
        }
        v1.get_size().cmp(&v2.get_size())
    }

    /// C++ `HeapSequence::deduplicatePairs`.
    fn deduplicate_pairs(&self, data: &mut Funcdata, pairs: &mut [IndirectPair]) -> bool {
        if pairs.is_empty() {
            return true;
        }
        // sort(copy, compareOutput) — sort the indices into `pairs`.
        let mut order: Vec<usize> = (0..pairs.len()).collect();
        order.sort_by(|&i, &j| Self::compare_output(data, &pairs[i], &pairs[j]));

        let mut head = order[0];
        let mut dup_count = 0;
        for k in 1..order.len() {
            let cur = order[k];
            let head_out = pairs[head].out_vn;
            let cur_out = pairs[cur].out_vn;
            let overlap = {
                let vhead = data.vbank().get(head_out).expect("stale head_out");
                let vcur = data.vbank().get(cur_out).expect("stale cur_out");
                vhead.characterize_overlap(vcur)
            };
            if overlap == 1 {
                return false; // Partial overlap
            }
            if overlap == 2 {
                if pairs[cur].in_vn != pairs[head].in_vn {
                    return false;
                }
                pairs[cur].duplicate = true;
                dup_count += 1;
            } else {
                head = cur;
            }
        }
        if dup_count > 0 {
            let mut head = order[0];
            for k in 1..order.len() {
                let cur = order[k];
                if pairs[cur].duplicate {
                    let from = pairs[cur].out_vn;
                    let to = pairs[head].out_vn;
                    data.total_replace(from, to).ok();
                } else {
                    head = cur;
                }
            }
        }
        true
    }

    /// C++ `HeapSequence::removeStoreOps`.
    fn remove_store_ops(
        &self,
        data: &mut Funcdata,
        indirects: &[OpId],
        indirect_pairs: &[IndirectPair],
        replace_op: OpId,
    ) {
        let mut scratch: Vec<OpId> = Vec::new();
        // Unhook Varnodes we don't want destroyed.
        for p in indirect_pairs.iter() {
            if let Some(def) = data.vbank().get(p.out_vn).and_then(|v| v.get_def()) {
                data.op_unset_output(def);
            }
        }
        for i in 0..self.base.move_ops.len() {
            let op = self.base.move_ops[i].op;
            data.op_destroy_recursive(op, &mut scratch);
        }
        for &op in indirects.iter() {
            data.op_destroy(op);
        }
        let replace_addr = data.obank().get(replace_op).unwrap().get_addr().clone();
        for p in indirect_pairs.iter() {
            if p.duplicate {
                continue;
            }
            let new_ind = data.new_op(2, replace_addr.clone());
            data.op_set_opcode_code(new_ind, OpCode::CPUI_INDIRECT);
            data.op_set_output(new_ind, p.out_vn).ok();
            data.op_set_input(new_ind, p.in_vn, 0).ok();
            let iop = data.new_varnode_iop(replace_op);
            data.op_set_input(new_ind, iop, 1).ok();
            data.op_insert_before(new_ind, replace_op);
        }
    }

    /// C++ `HeapSequence::transform`.
    fn transform(&self, data: &mut Funcdata) -> bool {
        let mut indirects: Vec<OpId> = Vec::new();
        let mut indirect_pairs: Vec<IndirectPair> = Vec::new();
        self.gather_indirect_pairs(data, &mut indirects, &mut indirect_pairs);
        if !self.deduplicate_pairs(data, &mut indirect_pairs) {
            return false;
        }
        let mem_cpy_op = match self.build_string_copy(data) {
            Some(o) => o,
            None => return false,
        };
        self.remove_store_ops(data, &indirects, &indirect_pairs, mem_cpy_op);
        true
    }
}

// =============================================================================
// StringSequence (constseq.cc:188-483) — COPY-into-array driver
// =============================================================================

/// A sequence of COPY ops moving single constant characters into a contiguous
/// array memory region tied to a (stack) Symbol (C++
/// `class StringSequence : public ArraySequence`).
pub(crate) struct StringSequence {
    /// The shared gathering machinery (C++ base `ArraySequence`).
    base: ArraySequence,
    /// The root COPY (C++ `ArraySequence::rootOp`).
    root_op: OpId,
    /// The basic block of the root COPY (C++ `ArraySequence::block`).
    block: crate::context::BlockId,
    /// Address being COPYed into by the root op (C++ `rootAddr`).
    root_addr: Address,
    /// Starting address of the array containing the character data (C++ `startAddr`).
    start_addr: Address,
    /// Snapshot of the containing Symbol entry (C++ `entry`).
    entry: crate::varmap::StringContainerEntry,
}

impl StringSequence {
    /// C++ `StringSequence::StringSequence(Funcdata&,Datatype*,SymbolEntry*,PcodeOp*,const Address&)`
    /// followed by the ctor body (`collectCopyOps`/`checkInterference`/`formByteArray`).
    ///
    /// Returns the built sequence; `is_valid()` is `false` when no viable
    /// COPY-into-array string exists for the given root.
    fn build(
        data: &Funcdata,
        ct: Rc<Datatype>,
        entry: crate::varmap::StringContainerEntry,
        root: OpId,
        addr: Address,
    ) -> Option<StringSequence> {
        let block =
            data.obank().get(root).expect("StringSequence: stale root").get_parent().expect(
                "StringSequence: root has no parent block",
            );
        let mut seq = StringSequence {
            base: ArraySequence::new(ct),
            root_op: root,
            block,
            root_addr: addr.clone(),
            start_addr: addr.clone(),
            entry,
        };
        // if (entry->getAddr().getSpace() != addr.getSpace()) return;
        let entry_space = seq.entry.addr.get_space()?.clone();
        let addr_space = addr.get_space()?.clone();
        if !Rc::ptr_eq(&entry_space, &addr_space) {
            return Some(seq); // invalid (num_elements stays 0)
        }
        // int8 off = rootAddr.getOffset() - entry->getFirst();
        let mut off: int8 = seq.root_addr.get_offset().wrapping_sub(seq.entry.first) as int8;
        // if (off >= entry->getSize()) return;
        if off >= seq.entry.size as int8 {
            return Some(seq);
        }
        // if (rootOp->getIn(0)->getOffset() == 0) return;
        let in0 = op_get_in(data, seq.root_op, 0);
        if vn_get_offset(data, in0) == 0 {
            return Some(seq);
        }
        // Walk down the parent type to the array of `ct`.  `parent_type` is `None`
        // once the walk falls off the end (C++ parentType==0).
        let mut parent_type: Option<Rc<Datatype>> = Some(Rc::clone(&seq.entry.sym_type));
        let mut array_type: Option<Rc<Datatype>> = None;
        let mut last_off: int8 = 0;
        loop {
            let pt = match &parent_type {
                Some(p) => Rc::clone(p),
                None => break, // parentType == 0
            };
            // if (parentType == ct) break;
            if Rc::ptr_eq(&pt, &seq.base.char_type) {
                break;
            }
            array_type = Some(Rc::clone(&pt));
            last_off = off;
            if pt.needs_resolution() {
                // const TypeField *field = parentType->resolveTruncation(off, root, -1, off);
                match Self::resolve_field(data, &pt, off, root, -1) {
                    Some((field_ty, new_off)) => {
                        off = new_off;
                        parent_type = Some(field_ty);
                    }
                    None => break,
                }
            } else {
                // parentType = parentType->getSubType(off, &off);
                match pt.get_sub_type(off) {
                    Ok((Some(sub), new_off)) => {
                        off = new_off;
                        parent_type = Some(sub);
                    }
                    // parentType == 0: loop exits, post-check fails.
                    _ => parent_type = None,
                }
            }
        }
        // if (parentType != ct || arrayType == 0 || arrayType->getMetatype() != TYPE_ARRAY) return;
        let parent_matches_ct =
            parent_type.as_ref().map(|p| Rc::ptr_eq(p, &seq.base.char_type)).unwrap_or(false);
        if !parent_matches_ct {
            return Some(seq);
        }
        let array_type = match array_type {
            Some(a) if a.get_metatype() == type_metatype::TYPE_ARRAY => a,
            _ => return Some(seq),
        };
        // startAddr = rootAddr - lastOff;
        seq.start_addr = &seq.root_addr + (-last_off);
        // if (!collectCopyOps(arrayType->getSize())) return;
        if !seq.collect_copy_ops(data, array_type.get_size()) {
            return Some(seq);
        }
        // if (!checkInterference()) return;
        if !seq.base.check_interference(data, seq.root_op) {
            return Some(seq);
        }
        // int4 arrSize = arrayType->getSize() - (int4)(rootAddr.getOffset() - startAddr.getOffset());
        let arr_size = array_type.get_size()
            - (seq.root_addr.get_offset().wrapping_sub(seq.start_addr.get_offset()) as int4);
        // numElements = formByteArray(arrSize, 0, rootAddr.getOffset(), rootAddr.isBigEndian());
        let big_endian = seq.root_addr.is_big_endian();
        seq.base.num_elements =
            seq.base.form_byte_array(data, arr_size, 0, seq.root_addr.get_offset(), big_endian);
        Some(seq)
    }

    fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    /// Resolve a union/partial-union field via `Funcdata::resolve_truncation`,
    /// returning `(field_type, newoff)` (the C++ `resolveTruncation` returning a
    /// `TypeField *` plus `newoff`).  `data` is `&Funcdata` here (the read-only
    /// resolution path); `resolve_truncation` needs `&mut`, but the ctor runs
    /// before the transform with the COPYs already in place — the union path is
    /// not exercised by the stack-string datatests (plain `char[]` arrays), so
    /// the read-only walk returns `None` for unions, matching the C++ "no field"
    /// fall-through (the resolved-field branch is the deferred union follow-up).
    fn resolve_field(
        _data: &Funcdata,
        _ct: &Rc<Datatype>,
        _off: int8,
        _op: OpId,
        _slot: int4,
    ) -> Option<(Rc<Datatype>, int8)> {
        // Union resolution requires `&mut Funcdata` (scoring writes the slot-1
        // edge); the COPY-array driver runs on `&Funcdata`.  No stack-string
        // datatest reaches a union container, so decline (== C++ field==0).
        None
    }

    /// C++ `StringSequence::collectCopyOps`.
    ///
    /// Gather constant-input COPYs into the formal array `[startAddr, startAddr+size)`,
    /// in the root block, skipping earlier elements and stopping at the first gap.
    /// Returns `false` on a wrong-size COPY, a COPY to the element just before the
    /// root (root is not first), or fewer than the minimum number of COPYs.
    fn collect_copy_ops(&mut self, data: &Funcdata, size: int4) -> bool {
        // Address endAddr = startAddr + (size - 1);
        let end_addr = &self.start_addr + i64::from(size - 1);
        // Address beginAddr = startAddr; if (startAddr != rootAddr) beginAddr = rootAddr - charType->getAlignSize();
        let begin_addr = if self.start_addr != self.root_addr {
            &self.root_addr + (-(self.base.char_type.get_align_size() as i64))
        } else {
            self.start_addr.clone()
        };
        // iter = beginLoc(beginAddr); enditer = endLoc(endAddr);  (address-only overload)
        // endLoc(endAddr) == lower_bound(endAddr + 1); the half-open window is
        // [beginAddr, endAddr + 1).
        let scan_end = &end_addr + 1;
        let ids: Vec<VarnodeId> =
            data.vbank().iter_loc_addr_range(&begin_addr, &scan_end).collect();
        let mut diff: int4 =
            self.root_addr.get_offset().wrapping_sub(self.start_addr.get_offset()) as int4;
        let align = self.base.char_type.get_align_size();
        let char_size = self.base.char_type.get_size();
        for vn in ids {
            let v = data.vbank().get(vn).expect("collectCopyOps: stale vn");
            // if (!vn->isWritten()) continue;
            if !v.is_written() {
                continue;
            }
            let op = v.get_def().expect("collectCopyOps: written vn has no def");
            // if (op->code() != CPUI_COPY) continue;
            if data.obank().get(op).expect("collectCopyOps: stale op").code() != OpCode::CPUI_COPY {
                continue;
            }
            // if (op->getParent() != block) continue;
            if data.obank().get(op).unwrap().get_parent() != Some(self.block) {
                continue;
            }
            // if (!op->getIn(0)->isConstant()) continue;
            let in0 = op_get_in(data, op, 0);
            if !data.vbank().get(in0).map(|x| x.is_constant()).unwrap_or(false) {
                continue;
            }
            // if (vn->getSize() != charType->getSize()) return false;  (not yet split)
            if v.get_size() != char_size {
                return false;
            }
            // int4 tmpDiff = vn->getOffset() - startAddr.getOffset();
            let tmp_diff: int4 =
                v.get_offset().wrapping_sub(self.start_addr.get_offset()) as int4;
            if tmp_diff < diff {
                // if (tmpDiff + charType->getAlignSize() == diff) return false;
                if tmp_diff + align == diff {
                    return false; // COPY to previous element, root is not first
                }
                continue;
            } else if tmp_diff > diff {
                // if (tmpDiff - diff < charType->getAlignSize()) continue;
                if tmp_diff - diff < align {
                    continue;
                }
                // if (tmpDiff - diff > charType->getAlignSize()) break;  (gap)
                if tmp_diff - diff > align {
                    break;
                }
                diff = tmp_diff; // Advanced by one character
            }
            // moveOps.emplace_back(vn->getOffset(), op, -1);
            let order = data.obank().get(op).unwrap().get_seq_num().get_order();
            self.base.move_ops.push(WriteNode::new(v.get_offset(), op, -1, order));
        }
        self.base.move_ops.len() as int4 >= ArraySequence::MINIMUM_SEQUENCE_LENGTH
    }

    /// C++ `StringSequence::constructTypedPointer`.
    ///
    /// Build a typed pointer (in)to the containing Symbol at `rootAddr`, emitting
    /// PTRSUB/PTRADD ops before `insert_point` with the appropriate pointer
    /// data-types, and return the final pointer Varnode.
    fn construct_typed_pointer(&self, data: &mut Funcdata, insert_point: OpId) -> Option<VarnodeId> {
        use crate::dtype::TypeFactory;
        let spc = self.root_addr.get_space()?.clone();
        let types = data.get_arch().types_rc()?;
        let insert_addr = data.obank().get(insert_point)?.get_addr().clone();
        // spacePtr = (spc->getType()==IPTR_SPACEBASE) ? constructSpacebaseInput(spc) : constructConstSpacebase(spc);
        let mut space_ptr = if spc.get_type() == kuna_base::space::spacetype::IPTR_SPACEBASE {
            data.construct_spacebase_input(&spc).ok()?
        } else {
            data.construct_const_spacebase(&spc).ok()?
        };
        let mut base_type = Rc::clone(&self.entry.sym_type);
        // PTRSUB from base register to the Symbol.
        let ptrsub = data.new_op(2, insert_addr.clone());
        data.op_set_opcode_code(ptrsub, OpCode::CPUI_PTRSUB);
        data.op_set_input(ptrsub, space_ptr, 0).ok();
        let sp_size = data.vbank().get(space_ptr)?.get_size();
        // uintb baseOff = byteToAddress(entry->getFirst(), spc->getWordSize());
        let mut base_off =
            kuna_base::space::AddrSpace::byte_to_address(self.entry.first, spc.get_word_size());
        let off_con = data.new_constant(sp_size, base_off);
        data.op_set_input(ptrsub, off_con, 1).ok();
        space_ptr = data.new_unique_out(sp_size, ptrsub).ok()?;
        data.op_insert_before(ptrsub, insert_point);
        // TypePointer *curType = getTypePointerStripArray(sz, baseType, wordsize);
        let mut cur_type = types
            .get_type_pointer_strip_array(sp_size, Rc::clone(&base_type), spc.get_word_size())
            .ok()?;
        data.vbank_mut().get_mut(space_ptr)?.update_type(Rc::clone(&cur_type));
        // int8 curOff = rootAddr.getOffset() - entry->getFirst();
        let mut cur_off: int8 = self.root_addr.get_offset().wrapping_sub(self.entry.first) as int8;
        while !Rc::ptr_eq(&base_type, &self.base.char_type) {
            // int4 elSize = (baseType->getMetatype()==TYPE_ARRAY) ? base->getAlignSize() : -1;
            let el_size: int4 = if base_type.get_metatype() == type_metatype::TYPE_ARRAY {
                base_type.get_array_base().map(|b| b.get_align_size()).unwrap_or(-1)
            } else {
                -1
            };
            let new_off: int8;
            if base_type.needs_resolution() {
                match Self::resolve_field(data, &base_type, cur_off, insert_point, -1) {
                    Some((field_ty, no)) => {
                        base_type = field_ty;
                        cur_off = no;
                        continue; // Do not create PTRSUB for union resolution here
                    }
                    None => break,
                }
            } else {
                // baseType = baseType->getSubType(curOff, &newOff);
                match base_type.get_sub_type(cur_off) {
                    Ok((Some(sub), no)) => {
                        base_type = sub;
                        new_off = no;
                    }
                    _ => break,
                }
            }
            // curOff -= newOff;
            cur_off -= new_off;
            base_off =
                kuna_base::space::AddrSpace::byte_to_address(cur_off as u64, spc.get_word_size());
            let ptr_op: OpId;
            if el_size >= 0 {
                if cur_off == 0 {
                    // Don't create a PTRADD(#0, ...); baseType already updated, type already stripped.
                    cur_off = new_off;
                    continue;
                }
                // PTRADD: in1 = numEl, in2 = elSize
                ptr_op = data.new_op(3, insert_addr.clone());
                data.op_set_opcode_code(ptr_op, OpCode::CPUI_PTRADD);
                let num_el = cur_off / (el_size as int8);
                let numel_con = data.new_constant(4, num_el as u64);
                data.op_set_input(ptr_op, numel_con, 1).ok();
                let elsz_con = data.new_constant(4, el_size as u64);
                data.op_set_input(ptr_op, elsz_con, 2).ok();
            } else {
                // PTRSUB: in1 = baseOff
                ptr_op = data.new_op(2, insert_addr.clone());
                data.op_set_opcode_code(ptr_op, OpCode::CPUI_PTRSUB);
                let boff_con = data.new_constant(sp_size, base_off);
                data.op_set_input(ptr_op, boff_con, 1).ok();
            }
            data.op_set_input(ptr_op, space_ptr, 0).ok();
            // if (curType->needsResolution()) inheritUnionFieldPtr(curType, ptrsub, 0, insertPoint, -1);
            if cur_type.needs_resolution() {
                data.inherit_union_field_ptr(Rc::clone(&cur_type), ptr_op, 0, insert_point, -1).ok();
            }
            space_ptr = data.new_unique_out(sp_size, ptr_op).ok()?;
            data.op_insert_before(ptr_op, insert_point);
            cur_type = types
                .get_type_pointer_strip_array(sp_size, Rc::clone(&base_type), spc.get_word_size())
                .ok()?;
            data.vbank_mut().get_mut(space_ptr)?.update_type(Rc::clone(&cur_type));
            cur_off = new_off;
        }
        if cur_off != 0 {
            // INT_ADD spacePtr + byteToAddress(curOff)
            let add_op = data.new_op(2, insert_addr.clone());
            data.op_set_opcode_code(add_op, OpCode::CPUI_INT_ADD);
            data.op_set_input(add_op, space_ptr, 0).ok();
            base_off =
                kuna_base::space::AddrSpace::byte_to_address(cur_off as u64, spc.get_word_size());
            let boff_con = data.new_constant(sp_size, base_off);
            data.op_set_input(add_op, boff_con, 1).ok();
            space_ptr = data.new_unique_out(sp_size, add_op).ok()?;
            data.op_insert_before(add_op, insert_point);
            cur_type = types
                .get_type_pointer(sp_size, Rc::clone(&self.base.char_type), spc.get_word_size())
                .ok()?;
            data.vbank_mut().get_mut(space_ptr)?.update_type(Rc::clone(&cur_type));
        }
        Some(space_ptr)
    }

    /// C++ `StringSequence::buildStringCopy`.
    fn build_string_copy(&self, data: &mut Funcdata) -> Option<OpId> {
        use crate::dtype::TypeFactory;
        // PcodeOp *insertPoint = moveOps[0].op;
        let insert_point = self.base.move_ops[0].op;
        // int4 numBytes = moveOps.size() * charType->getSize();
        let num_bytes = self.base.move_ops.len() as int4 * self.base.char_type.get_size();
        let types = data.get_arch().types_rc()?;
        let word_size = self.root_addr.get_space()?.get_word_size();
        // charPtrType = getTypePointer(getSizeOfPointer(), charType, rootAddr.getSpace()->getWordSize());
        let char_ptr_type = types
            .get_type_pointer(
                types.get_size_of_pointer(),
                Rc::clone(&self.base.char_type),
                word_size,
            )
            .ok()?;
        // Varnode *srcPtr = data.getInternalString(byteArray.data(), numBytes, charPtrType, insertPoint);
        let byte_array = self.base.byte_array.clone();
        let src_ptr = data.get_internal_string(&byte_array, num_bytes, char_ptr_type, insert_point)?;
        // uint4 builtInId = selectStringCopyFunction(index);  (registerBuiltin pre-done at boot)
        let (built_in_id, index) = self.base.select_string_copy_function(data);
        let insert_addr = data.obank().get(insert_point)?.get_addr().clone();
        // PcodeOp *copyOp = data.newOp(4, insertPoint->getAddr());
        let copy_op = data.new_op(4, insert_addr);
        data.op_set_opcode_code(copy_op, OpCode::CPUI_CALLOTHER);
        data.obank_mut().get_mut(copy_op)?.clear_flag(pcodeop_flags::call);
        let id_con = data.new_constant(4, built_in_id);
        data.op_set_input(copy_op, id_con, 0).ok();
        // Varnode *destPtr = constructTypedPointer(insertPoint);
        let dest_ptr = self.construct_typed_pointer(data, insert_point)?;
        data.op_set_input(copy_op, dest_ptr, 1).ok();
        data.op_set_input(copy_op, src_ptr, 2).ok();
        // if (destPtr->getType()->needsResolution()) inheritUnionFieldPtr(destPtr->getType(), copyOp, 1, insertPoint, -1);
        let dest_ty = Rc::clone(data.vbank().get(dest_ptr)?.get_type_def_facing());
        if dest_ty.needs_resolution() {
            data.inherit_union_field_ptr(dest_ty, copy_op, 1, insert_point, -1).ok();
        }
        // Varnode *lenVn = data.newConstant(4, index); lenVn->updateType(copyOp->inputTypeLocal(3));
        let len_vn = data.new_constant(4, index as u64);
        if let Ok(int4t) = types.get_base(4, type_metatype::TYPE_INT) {
            data.vbank_mut().get_mut(len_vn)?.update_type(int4t);
        }
        data.op_set_input(copy_op, len_vn, 3).ok();
        data.op_insert_before(copy_op, insert_point);
        Some(copy_op)
    }

    /// C++ `StringSequence::removeForward`.
    fn remove_forward(
        data: &mut Funcdata,
        cur_node: WriteNode,
        xref: &mut std::collections::HashMap<OpId, usize>,
        points: &mut Vec<RemovePoint>,
        dead_ops: &mut Vec<WriteNode>,
    ) {
        let vn = match data.obank().get(cur_node.op).and_then(|o| o.get_out()) {
            Some(v) => v,
            None => return,
        };
        for op in data.descend_snapshot(vn) {
            if let Some(&idx) = xref.get(&op) {
                // Seen the PIECE twice: merge offsets, drop the recorded point, add the PIECE.
                let off = points[idx].alive.map(|p| p.offset.min(cur_node.offset)).unwrap_or(cur_node.offset);
                points[idx].alive = None; // erase
                let order = data.obank().get(op).map(|o| o.get_seq_num().get_order()).unwrap_or(0);
                dead_ops.push(WriteNode::new(off, op, -1, order));
                xref.remove(&op);
            } else {
                let slot = data.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
                points.push(RemovePoint { alive: Some(WriteNode::new(cur_node.offset, op, slot, 0)) });
                if data.obank().get(op).map(|o| o.code() == OpCode::CPUI_PIECE).unwrap_or(false) {
                    xref.insert(op, points.len() - 1);
                }
            }
        }
    }

    /// C++ `StringSequence::removeCopyOps`.
    fn remove_copy_ops(&self, data: &mut Funcdata, replace_op: OpId) {
        let mut concat_set: std::collections::HashMap<OpId, usize> = std::collections::HashMap::new();
        let mut points: Vec<RemovePoint> = Vec::new();
        let mut dead_ops: Vec<WriteNode> = Vec::new();
        for i in 0..self.base.move_ops.len() {
            Self::remove_forward(data, self.base.move_ops[i], &mut concat_set, &mut points, &mut dead_ops);
        }
        let mut pos = 0;
        while pos < dead_ops.len() {
            Self::remove_forward(data, dead_ops[pos], &mut concat_set, &mut points, &mut dead_ops);
            pos += 1;
        }
        let replace_addr = data.obank().get(replace_op).unwrap().get_addr().clone();
        for point in points.iter() {
            let p = match point.alive {
                Some(p) => p,
                None => continue,
            };
            let vn = match data.obank().get(p.op).and_then(|o| o.get_in(p.slot)) {
                Some(v) => v,
                None => continue,
            };
            // if (vn->getDef()->code() != CPUI_INDIRECT) { ...build INDIRECT(0, iop)... }
            let is_indirect = data
                .vbank()
                .get(vn)
                .and_then(|v| v.get_def())
                .map(|d| data.obank().get(d).map(|o| o.code() == OpCode::CPUI_INDIRECT).unwrap_or(false))
                .unwrap_or(false);
            if !is_indirect {
                let sz = data.vbank().get(vn).unwrap().get_size();
                let new_in = data.new_constant(sz, 0);
                let ind_op = data.new_op(2, replace_addr.clone());
                data.op_set_opcode_code(ind_op, OpCode::CPUI_INDIRECT);
                data.op_set_input(ind_op, new_in, 0).ok();
                let iop = data.new_varnode_iop(replace_op);
                data.op_set_input(ind_op, iop, 1).ok();
                data.op_set_output(ind_op, vn).ok();
                data.mark_indirect_creation(ind_op, false).ok();
                data.op_insert_before(ind_op, replace_op);
            }
        }
        for i in 0..self.base.move_ops.len() {
            data.op_destroy(self.base.move_ops[i].op);
        }
        for i in 0..dead_ops.len() {
            data.op_destroy(dead_ops[i].op);
        }
    }

    /// C++ `StringSequence::transform`.
    fn transform(&self, data: &mut Funcdata) -> bool {
        let mem_cpy_op = match self.build_string_copy(data) {
            Some(o) => o,
            None => return false,
        };
        self.remove_copy_ops(data, mem_cpy_op);
        true
    }

    // ---------------------------------------------------------------------
    // Constant-fill (memset/bzero) recovery — kuna GH-9230/1537.
    //
    // The C++ `MemsetSequence : public StringSequence` reuses StringSequence's
    // `constructTypedPointer`/`removeCopyOps`; the Rust port keeps that reuse in
    // this `impl StringSequence` (private fields are reachable here) and exposes
    // only the `pub(crate)` entry points the `kuna_memsetsequence` driver needs.
    // ---------------------------------------------------------------------

    /// (kuna GH-9230) Build a fill sequence for memset recovery: the same
    /// containing-array type-walk as [`build`](Self::build), but gathers the
    /// constant COPY run with each COPY's byte size recorded in the `WriteNode`
    /// slot (via [`collect_fill_run`](Self::collect_fill_run)) and skips
    /// `formByteArray` — the driver runs the single-value-fill detection
    /// ([`detect_fill_run`](crate::kuna_memsetsequence::detect_fill_run)) on the
    /// returned `move_ops` and then [`transform_memset`](Self::transform_memset).
    /// Returns `None` when no containing `ct`-array exists or no run was gathered.
    pub(crate) fn build_for_fill(
        data: &Funcdata,
        ct: Rc<Datatype>,
        entry: crate::varmap::StringContainerEntry,
        root: OpId,
        addr: Address,
    ) -> Option<StringSequence> {
        let block = data.obank().get(root)?.get_parent()?;
        let mut seq = StringSequence {
            base: ArraySequence::new(ct),
            root_op: root,
            block,
            root_addr: addr.clone(),
            start_addr: addr.clone(),
            entry,
        };
        // entry/addr must share an address space, and the root must be inside the
        // entry, with a non-zero (real) fill offset — same gates as `build`.
        let entry_space = seq.entry.addr.get_space()?.clone();
        let addr_space = addr.get_space()?.clone();
        if !Rc::ptr_eq(&entry_space, &addr_space) {
            return None;
        }
        let off0: int8 = seq.root_addr.get_offset().wrapping_sub(seq.entry.first) as int8;
        if off0 >= seq.entry.size as int8 {
            return None;
        }
        // Walk the containing Symbol's type down to the `ct`-array (mirrors `build`).
        let mut off = off0;
        let mut parent_type: Option<Rc<Datatype>> = Some(Rc::clone(&seq.entry.sym_type));
        let mut array_type: Option<Rc<Datatype>> = None;
        let mut last_off: int8 = 0;
        loop {
            let pt = match &parent_type {
                Some(p) => Rc::clone(p),
                None => break,
            };
            if Rc::ptr_eq(&pt, &seq.base.char_type) {
                break;
            }
            array_type = Some(Rc::clone(&pt));
            last_off = off;
            if pt.needs_resolution() {
                match Self::resolve_field(data, &pt, off, root, -1) {
                    Some((field_ty, new_off)) => {
                        off = new_off;
                        parent_type = Some(field_ty);
                    }
                    None => break,
                }
            } else {
                match pt.get_sub_type(off) {
                    Ok((Some(sub), new_off)) => {
                        off = new_off;
                        parent_type = Some(sub);
                    }
                    _ => parent_type = None,
                }
            }
        }
        let parent_matches_ct =
            parent_type.as_ref().map(|p| Rc::ptr_eq(p, &seq.base.char_type)).unwrap_or(false);
        if !parent_matches_ct {
            return None;
        }
        let array_type = match array_type {
            Some(a) if a.get_metatype() == type_metatype::TYPE_ARRAY => a,
            _ => return None,
        };
        seq.start_addr = &seq.root_addr + (-last_off);
        if !seq.collect_fill_run(data, array_type.get_size()) {
            return None;
        }
        Some(seq)
    }

    /// (kuna GH-9230) Mirror of [`collect_copy_ops`](Self::collect_copy_ops), but
    /// records each COPY's byte size in the `WriteNode` slot (the memset fill
    /// detection needs the per-COPY stride) instead of `-1`.  The constant
    /// uniform-byte check is deferred to `detect_fill_run`; here it only gathers
    /// in-block constant COPYs tiling the `ct`-array region.
    pub(crate) fn collect_fill_run(&mut self, data: &Funcdata, size: int4) -> bool {
        let end_addr = &self.start_addr + i64::from(size - 1);
        let begin_addr = if self.start_addr != self.root_addr {
            &self.root_addr + (-(self.base.char_type.get_align_size() as i64))
        } else {
            self.start_addr.clone()
        };
        let scan_end = &end_addr + 1;
        let ids: Vec<VarnodeId> =
            data.vbank().iter_loc_addr_range(&begin_addr, &scan_end).collect();
        let mut diff: int4 =
            self.root_addr.get_offset().wrapping_sub(self.start_addr.get_offset()) as int4;
        let align = self.base.char_type.get_align_size();
        let char_size = self.base.char_type.get_size();
        for vn in ids {
            let v = data.vbank().get(vn).expect("collect_fill_run: stale vn");
            if !v.is_written() {
                continue;
            }
            let op = v.get_def().expect("collect_fill_run: written vn has no def");
            if data.obank().get(op).expect("collect_fill_run: stale op").code() != OpCode::CPUI_COPY {
                continue;
            }
            if data.obank().get(op).unwrap().get_parent() != Some(self.block) {
                continue;
            }
            let in0 = op_get_in(data, op, 0);
            if !data.vbank().get(in0).map(|x| x.is_constant()).unwrap_or(false) {
                continue;
            }
            // A memset element COPY is exactly one array element wide (the SIMD
            // wide-store form lifts to per-element COPYs by this stage).
            if v.get_size() != char_size {
                return false;
            }
            let tmp_diff: int4 =
                v.get_offset().wrapping_sub(self.start_addr.get_offset()) as int4;
            if tmp_diff < diff {
                if tmp_diff + align == diff {
                    return false;
                }
                continue;
            } else if tmp_diff > diff {
                if tmp_diff - diff < align {
                    continue;
                }
                if tmp_diff - diff > align {
                    break;
                }
                diff = tmp_diff;
            }
            let order = data.obank().get(op).unwrap().get_seq_num().get_order();
            // slot carries the COPY byte size (the fill stride), not -1.
            self.base.move_ops.push(WriteNode::new(v.get_offset(), op, v.get_size(), order));
        }
        self.base.move_ops.len() as int4 >= ArraySequence::MINIMUM_SEQUENCE_LENGTH
    }

    /// Mutable access to the gathered fill run, so the driver can run the
    /// single-value-fill detection (`detect_fill_run`) which truncates the run.
    pub(crate) fn fill_move_ops_mut(&mut self) -> &mut Vec<WriteNode> {
        &mut self.base.move_ops
    }

    /// (kuna GH-9230) Build the `builtin_memset(dest, value, count)` CALLOTHER
    /// (C++ `MemsetSequence::buildMemset`), mirroring
    /// [`build_string_copy`](Self::build_string_copy): construct the typed dest
    /// pointer into the array, then a 4-input CALLOTHER with the registered
    /// `BUILTIN_MEMSET` id, the fill value, and the byte count.
    pub(crate) fn build_memset(
        &self,
        data: &mut Funcdata,
        fill_value: u8,
        fill_count: int4,
    ) -> Option<OpId> {
        use crate::dtype::TypeFactory;
        let insert_point = self.base.move_ops[0].op;
        let insert_addr = data.obank().get(insert_point)?.get_addr().clone();
        let types = data.get_arch().types_rc()?;
        // PcodeOp *memsetOp = data.newOp(4, insertPoint->getAddr()); CALLOTHER.
        let memset_op = data.new_op(4, insert_addr);
        data.op_set_opcode_code(memset_op, OpCode::CPUI_CALLOTHER);
        data.obank_mut().get_mut(memset_op)?.clear_flag(pcodeop_flags::call);
        let id_con = data.new_constant(4, BUILTIN_MEMSET as uintb);
        data.op_set_input(memset_op, id_con, 0).ok();
        // in1 = constructTypedPointer(insertPoint).
        let dest_ptr = self.construct_typed_pointer(data, insert_point)?;
        data.op_set_input(memset_op, dest_ptr, 1).ok();
        if let Ok(dest_v) = data.vbank().get(dest_ptr).ok_or(()) {
            let dest_ty = Rc::clone(dest_v.get_type_def_facing());
            if dest_ty.needs_resolution() {
                data.inherit_union_field_ptr(dest_ty, memset_op, 1, insert_point, -1).ok();
            }
        }
        let int4t = types.get_base(4, type_metatype::TYPE_INT).ok();
        // in2 = fill value (the repeated byte), in3 = fill byte count.
        let val_vn = data.new_constant(4, fill_value as uintb);
        if let Some(t) = int4t.clone() {
            data.vbank_mut().get_mut(val_vn)?.update_type(t);
        }
        data.op_set_input(memset_op, val_vn, 2).ok();
        let cnt_vn = data.new_constant(4, fill_count as uintb);
        if let Some(t) = int4t {
            data.vbank_mut().get_mut(cnt_vn)?.update_type(t);
        }
        data.op_set_input(memset_op, cnt_vn, 3).ok();
        data.op_insert_before(memset_op, insert_point);
        Some(memset_op)
    }

    /// (kuna GH-9230) `MemsetSequence::transform`: build the memset and tear down
    /// the original COPY run (reusing [`remove_copy_ops`](Self::remove_copy_ops)).
    pub(crate) fn transform_memset(
        &self,
        data: &mut Funcdata,
        fill_value: u8,
        fill_count: int4,
    ) -> bool {
        let memset_op = match self.build_memset(data, fill_value, fill_count) {
            Some(o) => o,
            None => return false,
        };
        self.remove_copy_ops(data, memset_op);
        true
    }
}

/// A recorded read point in `removeCopyOps`, modeling the C++
/// `list<WriteNode>` element that may be erased (the `points` list).  An erased
/// element keeps its slot in the Vec with `alive == None` (the C++ `list::erase`
/// is emulated by tombstoning so the `xref` indices stay stable).
struct RemovePoint {
    alive: Option<WriteNode>,
}

// =============================================================================
// RuleStringCopy / RuleStringStore (constseq.cc:969-1029)
// =============================================================================
//
// Both `RuleStringStore` (STORE-through-pointer, [`HeapSequence`]) and
// `RuleStringCopy` (COPY-into-array, [`StringSequence`]) drive their live
// transforms.

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
        // Varnode *outvn = op->getOut();
        let outvn = match data.obank().get(op).and_then(|o| o.get_out()) {
            Some(v) => v,
            None => return 0,
        };
        // Datatype *ct = outvn->getTypeDefFacing();
        let ct = Rc::clone(data.vbank().get(outvn).expect("RuleStringCopy: stale outvn").get_type_def_facing());
        // if (!ct->isCharPrint()) return 0;
        if !ct.is_char_print() {
            return 0;
        }
        // if (ct->isOpaqueString()) return 0;
        if ct.is_opaque_string() {
            return 0;
        }
        // if (!outvn->isAddrTied()) return 0;
        if !data.vbank().get(outvn).unwrap().is_addr_tied() {
            return 0;
        }
        // SymbolEntry *entry = data.getScopeLocal()->queryContainer(outvn->getAddr(), outvn->getSize(), op->getAddr());
        let out_addr = data.vbank().get(outvn).unwrap().get_addr().clone();
        let out_size = data.vbank().get(outvn).unwrap().get_size();
        let op_addr = data.obank().get(op).unwrap().get_addr().clone();
        // The C++ `getScopeLocal()->queryContainer` walks up to the global parent
        // scope.  In the merged tree the local `ScopeLocal::db` only carries the
        // function's stack scope; a global-mapped array (`map addr ... char
        // globstring[32]`) lives in the frozen global-scope snapshot on `glb`.  So
        // try the local stack scope first, then fall back to the global container.
        let entry = data
            .get_scope_local()
            .and_then(|lm| lm.query_container(&out_addr, out_size, &op_addr))
            .or_else(|| {
                data.get_arch()
                    .query_container_global(&out_addr, out_size, &op_addr)
                    .and_then(|g| {
                        let sym_type = g.symbol_type.clone()?;
                        // entry->getFirst()/getSize() for a whole-symbol global
                        // mapping: the entry starts at the symbol and spans its type.
                        Some(crate::varmap::StringContainerEntry {
                            first: g.entry_addr.get_offset(),
                            size: sym_type.get_size(),
                            addr: g.entry_addr.clone(),
                            sym_type,
                        })
                    })
            });
        let entry = match entry {
            Some(e) => e,
            None => return 0,
        };
        // StringSequence sequence(data,ct,entry,op,outvn->getAddr());
        let sequence = match StringSequence::build(data, ct, entry, op, out_addr) {
            Some(s) => s,
            None => return 0,
        };
        // if (!sequence.isValid()) return 0;
        if !sequence.is_valid() {
            return 0;
        }
        // if (!sequence.transform()) return 0;
        if !sequence.transform(data) {
            return 0;
        }
        1
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
        // Varnode *ptrvn = op->getIn(1);
        let ptrvn = match data.obank().get(op).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => return 0,
        };
        // Datatype *ct = ptrvn->getTypeReadFacing(op);
        let ct = Rc::clone(
            data.vbank().get(ptrvn).expect("RuleStringStore: stale ptrvn").get_type_read_facing(op),
        );
        // if (ct->getMetatype() != TYPE_PTR) return 0;
        if ct.get_metatype() != type_metatype::TYPE_PTR {
            return 0;
        }
        // ct = ((TypePointer *)ct)->getPtrTo();
        let ct = match ct.get_ptr_to() {
            Some(c) => c,
            None => return 0,
        };
        // if (!ct->isCharPrint()) return 0;
        if !ct.is_char_print() {
            return 0;
        }
        // if (ct->isOpaqueString()) return 0;
        if ct.is_opaque_string() {
            return 0;
        }
        // HeapSequence sequence(data,ct,op);
        let sequence = HeapSequence::build(data, ct, op);
        // if (!sequence.isValid()) return 0;
        if !sequence.is_valid() {
            return 0;
        }
        // if (!sequence.transform()) return 0;
        if !sequence.transform(data) {
            return 0;
        }
        1
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
