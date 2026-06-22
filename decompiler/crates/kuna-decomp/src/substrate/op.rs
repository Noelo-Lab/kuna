//! Port of `decompiler/cpp/op.{hh,cc}` (W3, item `w3-ir-op`) — the `PcodeOp`
//! and `PcodeOpBank` classes: the lowest-level operation of the p-code IR and
//! its per-function container.
//!
//! ## ADR 0001 (IR arenas) realization
//!
//! The C++ `PcodeOp` is a heap object cross-referenced by raw pointers
//! (`output`/`inrefs` are `Varnode *`, `parent` is `BlockBasic *`, `opcode` is
//! `TypeOp *`) and it sits on **three** intrusive `std::list`s at once, each
//! tracked by a cached `list::iterator`:
//!
//!   - `basiciter`  — its position within the parent basic block,
//!   - `insertiter` — its position in the global alive/dead list,
//!   - `codeiter`   — its position in the per-op-code dispatch list.
//!
//! Per ADR 0001 the Rust port stores `PcodeOp`s in a `Funcdata`-owned slotmap
//! arena keyed by [`OpId`]; here [`PcodeOpBank`] owns that arena directly (the
//! bank *is* the op store, exactly as `VarnodeBank` is the varnode store).  The
//! three `list::iterator`s become prev/next **`OpId`** links stored on the node
//! ([`IntrusiveLinks`]) and managed by a small [`IntrusiveList`] helper that
//! reproduces `std::list` `insert`/`erase`/`splice` order and the
//! iterator-stability the cached-iterator code relies on.  `output` becomes
//! `Option<VarnodeId>`, `inrefs` a `Vec<Option<VarnodeId>>` (slots can be
//! `null`, exactly as the C++ `inrefs[slot] = (Varnode *)0`), and `opcode` an
//! `Option<TypeOp>` (the W6 seam, [`crate::seams::TypeOp`]).
//!
//! ## ADR 0002 (ordered containers) realization
//!
//! `optree` is a `BTreeMap<SeqNum, OpId>` keyed by the op's [`SeqNum`] (the C++
//! `PcodeOpTree = map<SeqNum,PcodeOp *>`); `SeqNum`'s `Ord` (ported in
//! `kuna-base`) is the comparator.  `begin(addr)`/`end(addr)`/`target` become
//! `BTreeMap::range` queries over constructed `SeqNum` bound keys mirroring the
//! C++ `lower_bound`/`upper_bound` calls exactly.  `uniqid` is the one-up
//! allocation counter feeding each new op's `SeqNum::uniq`; allocation order is
//! semantics (it drives `optree` ordering, hence iteration, hence rule
//! application), so the create paths are transcribed step for step.
//!
//! ## Cross-wave seams
//!
//! - `TypeOp` (W6): [`crate::seams::TypeOp`] carries the `opcode`/`flags`/`name`
//!   slice `op.cc` touches.  The emulation+type-local methods (`collapse`,
//!   `executeSimple`, `outputTypeLocal`, `inputTypeLocal`) need the W6
//!   `OpBehavior`/`TypeFactory`; they are seam-noted and take the behavior as an
//!   explicit argument or are deferred.
//! - `Varnode` (W3, already ported): the methods that read input/output varnode
//!   contents (`getSlot`, `getCseHash`, `getNZMaskLocal`, ...) take a
//!   `&VarnodeBank` and read through it — the Funcdata-mediated access of
//!   ADR 0001, with the bank standing in for the not-yet-ported `Funcdata`.
//! - `BlockBasic`/`FlowBlock` (W3 `block`, sibling, not yet ported): the
//!   control-flow methods (`nextOp`, `previousOp`, `target`, `compareOrder`,
//!   `isMoveable`, `setCopyImmed`, `hasCopyImmed`) and `IopSpace::printRaw`
//!   need the block graph; they are seam-noted (`// SEAM(W3-block)`) and live
//!   in `funcdata`/`funcdata_op` rather than here, or take a block accessor.

use std::collections::BTreeMap;
use std::ops::Bound;

use kuna_base::address::{
    calc_mask, coveringmask, leastsigbit_set, mostsigbit_set, pcode_left, pcode_right, popcount,
    sign_extend_sized, Address, SeqNum,
};
use kuna_base::types::{int4, uint4, uintb, uintm, Wrap};
use kuna_num::opcodes::OpCode;

use crate::seams::{OpId, TypeOp, VarnodeId};
use crate::varnode::VarnodeBank;

/// Boolean attributes (flags) that can be placed on a [`PcodeOp`].
///
/// Verbatim transcription of the C++ first anonymous `enum` in
/// `class PcodeOp` (`op.hh:72-107`).  These are stored in the op's `flags`
/// word; many are read publicly via the `is_*`/`get_*` query methods.
pub mod pcodeop_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;

    /// This instruction starts a basic block
    pub const startbasic: uint4 = 1;
    /// This instruction is a branch
    pub const branch: uint4 = 2;
    /// This instruction calls a subroutine
    pub const call: uint4 = 4;
    /// This instruction returns to caller
    pub const returns: uint4 = 0x8;
    /// This op cannot be collapsed further
    pub const nocollapse: uint4 = 0x10;
    /// This operation is dead
    pub const dead: uint4 = 0x20;
    /// special placeholder op (multiequal or indirect) or CPUI_COPY between
    /// different copies of same variable
    pub const marker: uint4 = 0x40;
    /// Boolean operation
    pub const booloutput: uint4 = 0x80;
    /// Set if condition must be false to take branch
    pub const boolean_flip: uint4 = 0x100;
    /// Set if fallthru happens on true condition
    pub const fallthru_true: uint4 = 0x200;
    /// Op is source of (one or more) CPUI_INDIRECTs
    pub const indirect_source: uint4 = 0x400;
    /// The first parameter to this op is a coderef
    pub const coderef: uint4 = 0x800;
    /// This op is the first in its instruction
    pub const startmark: uint4 = 0x1000;
    /// Used by many algorithms that need to detect loops or avoid repeats
    pub const mark: uint4 = 0x2000;
    /// Order of input parameters does not matter
    pub const commutative: uint4 = 0x4000;
    /// Evaluate as unary expression
    pub const unary: uint4 = 0x8000;
    /// Evaluate as binary expression
    pub const binary: uint4 = 0x10000;
    /// Cannot be evaluated (without special processing)
    pub const special: uint4 = 0x20000;
    /// Evaluate as ternary operator (or higher)
    pub const ternary: uint4 = 0x40000;
    /// Special form of COPY op for holding global values to (past) the end of
    /// the function
    pub const return_copy: uint4 = 0x80000;
    /// Op should not be directly printed as source
    pub const nonprinting: uint4 = 0x100000;
    /// instruction causes processor or process to halt
    pub const halt: uint4 = 0x200000;
    /// placeholder for bad instruction data
    pub const badinstruction: uint4 = 0x400000;
    /// placeholder for unimplemented instruction
    pub const unimplemented: uint4 = 0x800000;
    /// placeholder for previous call that doesn't exit
    pub const noreturn: uint4 = 0x1000000;
    /// ops at this address were not generated
    pub const missing: uint4 = 0x2000000;
    /// Loads or stores from a dynamic pointer into a spacebase
    pub const spacebase_ptr: uint4 = 0x4000000;
    /// Output varnode is created by indirect effect
    pub const indirect_creation: uint4 = 0x8000000;
    /// Output has been determined to be a 1-bit boolean value
    pub const calculated_bool: uint4 = 0x10000000;
    /// Op has a call specification associated with it
    pub const has_callspec: uint4 = 0x20000000;
    /// Op consumes or produces a ptr
    pub const ptrflow: uint4 = 0x40000000;
    /// CPUI_INDIRECT is caused by CPUI_STORE
    pub const indirect_store: uint4 = 0x80000000;
}

/// Additional boolean attributes (`addlflags`) of a [`PcodeOp`].
///
/// Verbatim transcription of the C++ second anonymous `enum` in
/// `class PcodeOp` (`op.hh:108-122`).  The kuna addition `canonical_lessequal`
/// lives at `0x1000`, the start of the kuna `addlflags` range (CLAUDE.md).
pub mod pcodeop_addlflags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;

    /// Does some special form of datatype propagation
    pub const special_prop: uint4 = 1;
    /// Op is marked for special printing
    pub const special_print: uint4 = 2;
    /// This op has been modified by the current action
    pub const modified: uint4 = 4;
    /// Warning has been generated for this op
    pub const warning: uint4 = 8;
    /// Treat this as \e incidental for parameter recovery algorithms
    pub const incidental_copy: uint4 = 0x10;
    /// Have we checked for cpool transforms
    pub const is_cpool_transformed: uint4 = 0x20;
    /// Stop data-type propagation into output from descendants
    pub const stop_type_propagation: uint4 = 0x40;
    /// Output varnode (of call) should not be removed if it is unread
    pub const hold_output: uint4 = 0x80;
    /// Output of \b this is root of a CONCAT tree
    pub const concat_root: uint4 = 0x100;
    /// Do not collapse \b this INDIRECT (via RuleIndirectCollapse)
    pub const no_indirect_collapse: uint4 = 0x200;
    /// If STORE collapses to a stack Varnode, force it to be unmapped
    pub const store_unmapped: uint4 = 0x400;
    /// Copy has propagated into input of \b this op
    pub const immed_copy: uint4 = 0x800;
    /// (kuna) INT_LESS/INT_SLESS produced by canonicalizing a LESSEQUAL
    /// (RuleIntLessEqual)
    pub const canonical_lessequal: uint4 = 0x1000;
}

// ---------------------------------------------------------------------------
// IntrusiveList — std::list<PcodeOp*> realized as prev/next OpId links
// ---------------------------------------------------------------------------

/// One op's membership links in the three intrusive lists it can belong to
/// (ADR 0001).  Each pair is the `std::list::iterator` the C++ caches, encoded
/// as the neighbouring [`OpId`]s; `None` marks a list end (the sentinel the C++
/// `std::list` provides).  An op that is not on a given list has both links
/// `None` *and* the bank's head/tail for that list does not reference it.
#[derive(Debug, Clone, Default)]
struct IntrusiveLinks {
    /// Previous op in the basic-block list (C++ `basiciter` predecessor)
    basic_prev: Option<OpId>,
    /// Next op in the basic-block list (C++ `basiciter` successor)
    basic_next: Option<OpId>,
    /// Previous op in the alive/dead list (C++ `insertiter` predecessor)
    insert_prev: Option<OpId>,
    /// Next op in the alive/dead list (C++ `insertiter` successor)
    insert_next: Option<OpId>,
    /// Previous op in the per-op-code list (C++ `codeiter` predecessor)
    code_prev: Option<OpId>,
    /// Next op in the per-op-code list (C++ `codeiter` successor)
    code_next: Option<OpId>,
}

/// Which of the three intrusive lists a link operation addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListKind {
    /// Basic-block position list (`basiciter`).  SEAM(W3-block): the per-block
    /// list head/tail belongs to `block`/`funcdata_block`; the membership links
    /// live on the op here and are driven through
    /// [`PcodeOp::set_basic_prev`]/[`PcodeOp::set_basic_next`].  Unused in the
    /// op crate alone (its only consumer is the not-yet-ported block wave + the
    /// in-module test), hence the allow.
    #[allow(dead_code)]
    Basic,
    /// Global alive/dead list (`insertiter`)
    Insert,
    /// Per-op-code dispatch list (`codeiter`)
    Code,
}

/// A doubly-linked intrusive list over the op arena, reproducing the
/// `std::list<PcodeOp *>` whose iterators the C++ caches on each op.
///
/// The list owns only its `head`/`tail` [`OpId`] sentinels; the prev/next links
/// live on each op's [`IntrusiveLinks`] (selected by [`ListKind`]).  This gives
/// O(1) `push_back`/`insert_after`/`erase`/`splice` with the exact ordering and
/// iterator-stability of `std::list`, which the bank's cached-iterator code
/// (`markAlive`/`markDead`/`insertAfterDead`/`moveSequenceDead`) relies on.
#[derive(Debug, Clone, Default)]
struct IntrusiveList {
    /// First op (C++ `list::begin()`), or `None` when empty
    head: Option<OpId>,
    /// Last op (C++ `--list::end()`), or `None` when empty
    tail: Option<OpId>,
    /// Element count (C++ `list::size()`)
    len: usize,
}

impl IntrusiveLinks {
    /// Borrow the (prev, next) link pair for the given list kind.
    fn get(&self, kind: ListKind) -> (Option<OpId>, Option<OpId>) {
        match kind {
            ListKind::Basic => (self.basic_prev, self.basic_next),
            ListKind::Insert => (self.insert_prev, self.insert_next),
            ListKind::Code => (self.code_prev, self.code_next),
        }
    }
    /// Set the previous-link for the given list kind.
    fn set_prev(&mut self, kind: ListKind, v: Option<OpId>) {
        match kind {
            ListKind::Basic => self.basic_prev = v,
            ListKind::Insert => self.insert_prev = v,
            ListKind::Code => self.code_prev = v,
        }
    }
    /// Set the next-link for the given list kind.
    fn set_next(&mut self, kind: ListKind, v: Option<OpId>) {
        match kind {
            ListKind::Basic => self.basic_next = v,
            ListKind::Insert => self.insert_next = v,
            ListKind::Code => self.code_next = v,
        }
    }
}

impl IntrusiveList {
    /// Number of elements (C++ `list::size`).
    fn len(&self) -> usize {
        self.len
    }
    /// `true` if the list is empty (C++ `list::empty`).
    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert `op` at the end (C++ `list.insert(list.end(), op)` /
    /// `push_back`).  `op` must not already be on this list.
    fn push_back(&mut self, arena: &mut OpArena, kind: ListKind, op: OpId) {
        let prev = self.tail;
        arena[op].links.set_prev(kind, prev);
        arena[op].links.set_next(kind, None);
        match prev {
            Some(p) => arena[p].links.set_next(kind, Some(op)),
            None => self.head = Some(op),
        }
        self.tail = Some(op);
        self.len += 1;
    }

    /// Insert `op` immediately after the node `after` (C++ idiom
    /// `iter = after->iter; ++iter; list.insert(iter, op)`).  `after` must be on
    /// the list; `op` must not.
    fn insert_after(&mut self, arena: &mut OpArena, kind: ListKind, after: OpId, op: OpId) {
        let next = arena[after].links.get(kind).1;
        arena[op].links.set_prev(kind, Some(after));
        arena[op].links.set_next(kind, next);
        arena[after].links.set_next(kind, Some(op));
        match next {
            Some(n) => arena[n].links.set_prev(kind, Some(op)),
            None => self.tail = Some(op),
        }
        self.len += 1;
    }

    /// Unlink `op` from the list (C++ `list.erase(op->iter)`).  `op` must be on
    /// the list; its links are cleared.
    fn erase(&mut self, arena: &mut OpArena, kind: ListKind, op: OpId) {
        let (prev, next) = arena[op].links.get(kind);
        match prev {
            Some(p) => arena[p].links.set_next(kind, next),
            None => self.head = next,
        }
        match next {
            Some(n) => arena[n].links.set_prev(kind, prev),
            None => self.tail = prev,
        }
        arena[op].links.set_prev(kind, None);
        arena[op].links.set_next(kind, None);
        self.len -= 1;
    }

    /// Splice the inclusive range `[first, last]` out of `self` and reinsert it
    /// immediately before `pos` (C++ `list.splice(pos, list, first, last)` where
    /// the C++ `last` is one past `lastop`).  All nodes stay on the same list
    /// (this models the single-list `moveSequenceDead` splice); the element
    /// count is unchanged.
    ///
    /// Precondition (matching the C++ caller's degenerate-move guard): `pos` is
    /// not already the node right after `last`, i.e. this is a real move.
    fn splice_range(
        &mut self,
        arena: &mut OpArena,
        kind: ListKind,
        first: OpId,
        last: OpId,
        pos: Option<OpId>,
    ) {
        // Detach [first, last] from its current position.
        let before = arena[first].links.get(kind).0;
        let after = arena[last].links.get(kind).1;
        match before {
            Some(b) => arena[b].links.set_next(kind, after),
            None => self.head = after,
        }
        match after {
            Some(a) => arena[a].links.set_prev(kind, before),
            None => self.tail = before,
        }
        // Reinsert [first, last] before `pos`.
        let new_prev = match pos {
            Some(p) => arena[p].links.get(kind).0,
            None => self.tail,
        };
        arena[first].links.set_prev(kind, new_prev);
        match new_prev {
            Some(np) => arena[np].links.set_next(kind, Some(first)),
            None => self.head = Some(first),
        }
        arena[last].links.set_next(kind, pos);
        match pos {
            Some(p) => arena[p].links.set_prev(kind, Some(last)),
            None => self.tail = Some(last),
        }
    }

    /// Clear the list head/tail/len (C++ `list::clear`).
    fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.len = 0;
    }

    /// Iterate the op ids in list order (C++ `for(iter=begin;iter!=end;++iter)`).
    fn iter<'a>(&self, arena: &'a OpArena, kind: ListKind) -> IntrusiveIter<'a> {
        IntrusiveIter { arena, kind, cur: self.head }
    }
}

/// Forward iterator over an [`IntrusiveList`] (C++ `list::const_iterator`).
struct IntrusiveIter<'a> {
    arena: &'a OpArena,
    kind: ListKind,
    cur: Option<OpId>,
}

impl Iterator for IntrusiveIter<'_> {
    type Item = OpId;
    fn next(&mut self) -> Option<OpId> {
        let cur = self.cur?;
        self.cur = self.arena[cur].links.get(self.kind).1;
        Some(cur)
    }
}

/// The op arena alias (ADR 0001): a slotmap keyed by [`OpId`].
type OpArena = slotmap::SlotMap<OpId, PcodeOp>;

// ---------------------------------------------------------------------------
// PcodeOp
// ---------------------------------------------------------------------------

/// \brief Lowest level operation of the \b p-code language
///
/// Port of the C++ `class PcodeOp` (`op.hh:63-260`).  See the module docs for
/// the arena/intrusive-list realization of the pointer/iterator members.
#[derive(Debug, Clone)]
pub struct PcodeOp {
    /// Behavioral class of the operation (C++ `TypeOp *opcode`; `None` == null).
    /// SEAM(W6): [`TypeOp`] is the minimal W6 skeleton.
    opcode: Option<TypeOp>,
    /// Collection of boolean attributes (C++ `mutable uint4 flags`)
    flags: uint4,
    /// Additional boolean attributes (C++ `mutable uint4 addlflags`)
    addlflags: uint4,
    /// What instruction address this is attached to (C++ `SeqNum start`)
    start: SeqNum,
    /// Basic block in which this op is contained (C++ `BlockBasic *parent`).
    /// SEAM(W3-block): [`crate::seams::BlockId`]; `None` == null.
    parent: Option<crate::seams::BlockId>,
    /// The three intrusive-list memberships (C++ `basiciter`/`insertiter`/
    /// `codeiter`)
    links: IntrusiveLinks,
    /// The one possible output Varnode of this op (C++ `Varnode *output`)
    output: Option<VarnodeId>,
    /// The ordered list of input Varnodes (C++ `vector<Varnode *> inrefs`).
    /// A slot can be `None` (the C++ `inrefs[slot] = (Varnode *)0`).
    inrefs: Vec<Option<VarnodeId>>,
}

impl PcodeOp {
    /// Construct a completely unattached PcodeOp (C++ `PcodeOp(int4,const
    /// SeqNum&)`, `op.cc:71`).  Space is reserved for `s` input slots, all
    /// initially `null`; output, opcode, and parent are null; flags start at 0
    /// (the op starts life dead).
    pub fn new(s: int4, sq: SeqNum) -> PcodeOp {
        PcodeOp {
            opcode: None,
            flags: 0,
            addlflags: 0,
            start: sq,
            parent: None,
            links: IntrusiveLinks::default(),
            output: None,
            // inrefs(s): s null slots
            inrefs: vec![None; s.max(0) as usize],
        }
    }

    // --- Mutators used only by the bank / Funcdata (C++ private) ------------

    /// Set the opcode for this op, caching the op-code's property flags
    /// (C++ `PcodeOp::setOpcode`, `op.cc:299`).
    fn set_opcode(&mut self, t_op: TypeOp) {
        // Clear the flag bits that are derived from the opcode, then OR in the
        // new opcode's flags.
        self.flags &= !(pcodeop_flags::branch
            | pcodeop_flags::call
            | pcodeop_flags::coderef
            | pcodeop_flags::commutative
            | pcodeop_flags::returns
            | pcodeop_flags::nocollapse
            | pcodeop_flags::marker
            | pcodeop_flags::booloutput
            | pcodeop_flags::unary
            | pcodeop_flags::binary
            | pcodeop_flags::ternary
            | pcodeop_flags::special
            | pcodeop_flags::has_callspec
            | pcodeop_flags::return_copy);
        self.flags |= t_op.get_flags();
        self.opcode = Some(t_op);
    }

    /// Set the output Varnode (C++ `setOutput`).
    pub fn set_output(&mut self, vn: Option<VarnodeId>) {
        self.output = vn;
    }
    /// Clear a specific input Varnode to \e null (C++ `clearInput`).
    pub fn clear_input(&mut self, slot: int4) {
        self.inrefs[slot as usize] = None;
    }
    /// Set a specific input Varnode (C++ `setInput`).
    pub fn set_input(&mut self, vn: Option<VarnodeId>, slot: int4) {
        self.inrefs[slot as usize] = vn;
    }
    /// Set specific boolean attribute(s) (C++ `setFlag`).
    pub fn set_flag(&mut self, fl: uint4) {
        self.flags |= fl;
    }
    /// Clear specific boolean attribute(s) (C++ `clearFlag`).
    pub fn clear_flag(&mut self, fl: uint4) {
        self.flags &= !fl;
    }
    /// Set a specific additional boolean attribute (C++ `setAdditionalFlag`).
    pub fn set_additional_flag(&mut self, fl: uint4) {
        self.addlflags |= fl;
    }
    /// Clear a specific additional boolean attribute (C++ `clearAdditionalFlag`).
    pub fn clear_additional_flag(&mut self, fl: uint4) {
        self.addlflags &= !fl;
    }
    /// Flip the setting of specific boolean attribute(s) (C++ `flipFlag`).
    pub fn flip_flag(&mut self, fl: uint4) {
        self.flags ^= fl;
    }

    /// Make sure this op has exactly \b num inputs, all set to \e null
    /// (C++ `PcodeOp::setNumInputs`, `op.cc:313`).
    pub fn set_num_inputs(&mut self, num: int4) {
        self.inrefs.resize(num.max(0) as usize, None);
        for slot in self.inrefs.iter_mut() {
            *slot = None;
        }
    }

    /// Eliminate the input Varnode in `slot`; all later inputs renumber down
    /// (C++ `PcodeOp::removeInput`, `op.cc:324`).
    pub fn remove_input(&mut self, slot: int4) {
        let slot = slot as usize;
        // for(i=slot+1;i<size;++i) inrefs[i-1]=inrefs[i]; pop_back();
        for i in (slot + 1)..self.inrefs.len() {
            self.inrefs[i - 1] = self.inrefs[i];
        }
        self.inrefs.pop();
    }

    /// Insert space for a new \e null input before `slot`
    /// (C++ `PcodeOp::insertInput`, `op.cc:334`).
    pub fn insert_input(&mut self, slot: int4) {
        let slot = slot as usize;
        // push_back(null); for(i=size-1;i>slot;--i) inrefs[i]=inrefs[i-1]; inrefs[slot]=null;
        self.inrefs.push(None);
        let mut i = self.inrefs.len() - 1;
        while i > slot {
            self.inrefs[i] = self.inrefs[i - 1];
            i -= 1;
        }
        self.inrefs[slot] = None;
    }

    /// Order this op within the ops for a single instruction (C++ `setOrder`).
    pub fn set_order(&mut self, ord: uintm) {
        self.start.set_order(ord);
    }
    /// Set the parent basic block of this op (C++ `setParent`).
    pub fn set_parent(&mut self, p: Option<crate::seams::BlockId>) {
        self.parent = p;
    }

    // --- Public accessors (C++ inline getters) ------------------------------

    /// Get the number of inputs to this op (C++ `numInput`).
    pub fn num_input(&self) -> int4 {
        self.inrefs.len() as int4
    }
    /// Get the output Varnode of this op or `None` (C++ `getOut`).
    pub fn get_out(&self) -> Option<VarnodeId> {
        self.output
    }
    /// Get a specific input Varnode to this op (C++ `getIn`).  The slot may hold
    /// `None` (a null input); an out-of-range slot is also `None` (matching the
    /// `Option` contract — callers that peek `getIn(1)` on an op before confirming
    /// it has two inputs, e.g. `RuleExpandLoad`, must not panic).
    pub fn get_in(&self, slot: int4) -> Option<VarnodeId> {
        self.inrefs.get(slot as usize).copied().flatten()
    }
    /// Get the parent basic block (C++ `getParent`).  SEAM(W3-block).
    pub fn get_parent(&self) -> Option<crate::seams::BlockId> {
        self.parent
    }
    /// Get the instruction address associated with this op (C++ `getAddr`).
    pub fn get_addr(&self) -> &Address {
        self.start.get_addr()
    }
    /// Get the time index indicating when this op was created (C++ `getTime`).
    pub fn get_time(&self) -> uintm {
        self.start.get_time()
    }
    /// Get the sequence number associated with this op (C++ `getSeqNum`).
    pub fn get_seq_num(&self) -> &SeqNum {
        &self.start
    }
    /// Get all boolean attributes (no direct C++ getter; mirrors `flags`).
    pub fn get_flags(&self) -> uint4 {
        self.flags
    }
    /// Get all additional boolean attributes (mirrors `addlflags`).
    pub fn get_addlflags(&self) -> uint4 {
        self.addlflags
    }

    /// \brief Get the slot number of the indicated input varnode (C++ `getSlot`).
    ///
    /// Returns the first slot equal to `vn`, or `numInput()` if absent (the C++
    /// returns the loop index `i==n` on no-match — note: *not* -1).
    pub fn get_slot(&self, vn: VarnodeId) -> int4 {
        let n = self.inrefs.len();
        let mut i = 0;
        while i < n {
            if self.inrefs[i] == Some(vn) {
                break;
            }
            i += 1;
        }
        i as int4
    }

    /// Get the evaluation type of this op (C++ `getEvalType`).
    pub fn get_eval_type(&self) -> uint4 {
        self.flags
            & (pcodeop_flags::unary
                | pcodeop_flags::binary
                | pcodeop_flags::special
                | pcodeop_flags::ternary)
    }

    /// Get type which indicates unusual halt in control-flow (C++ `getHaltType`).
    pub fn get_halt_type(&self) -> uint4 {
        self.flags
            & (pcodeop_flags::halt
                | pcodeop_flags::badinstruction
                | pcodeop_flags::unimplemented
                | pcodeop_flags::noreturn
                | pcodeop_flags::missing)
    }

    /// Return `true` if this op is dead (C++ `isDead`).
    pub fn is_dead(&self) -> bool {
        (self.flags & pcodeop_flags::dead) != 0
    }
    /// Return `true` if this op has an output (C++ `isAssignment`).
    pub fn is_assignment(&self) -> bool {
        self.output.is_some()
    }
    /// Return `true` if this op indicates call semantics (C++ `isCall`).
    pub fn is_call(&self) -> bool {
        (self.flags & pcodeop_flags::call) != 0
    }
    /// Return `true` if this acts as a call but has no full specification
    /// (C++ `isCallWithoutSpec`).
    pub fn is_call_without_spec(&self) -> bool {
        (self.flags & (pcodeop_flags::call | pcodeop_flags::has_callspec)) == pcodeop_flags::call
    }
    /// Return `true` if a special SSA form op (C++ `isMarker`).
    pub fn is_marker(&self) -> bool {
        (self.flags & pcodeop_flags::marker) != 0
    }
    /// Return `true` if op creates a varnode indirectly (C++ `isIndirectCreation`).
    pub fn is_indirect_creation(&self) -> bool {
        (self.flags & pcodeop_flags::indirect_creation) != 0
    }
    /// Return `true` if this INDIRECT is caused by STORE (C++ `isIndirectStore`).
    pub fn is_indirect_store(&self) -> bool {
        (self.flags & pcodeop_flags::indirect_store) != 0
    }
    /// Return `true` if this op is not directly represented in C output
    /// (C++ `notPrinted`).
    pub fn not_printed(&self) -> bool {
        (self.flags
            & (pcodeop_flags::marker | pcodeop_flags::nonprinting | pcodeop_flags::noreturn))
            != 0
    }
    /// Return `true` if this op produces a boolean output (C++ `isBoolOutput`).
    pub fn is_bool_output(&self) -> bool {
        (self.flags & pcodeop_flags::booloutput) != 0
    }
    /// Return `true` if this op is a branch (C++ `isBranch`).
    pub fn is_branch(&self) -> bool {
        (self.flags & pcodeop_flags::branch) != 0
    }
    /// Return `true` if this op is a call or branch (C++ `isCallOrBranch`).
    pub fn is_call_or_branch(&self) -> bool {
        (self.flags & (pcodeop_flags::branch | pcodeop_flags::call)) != 0
    }
    /// Return `true` if this op breaks fall-thru flow (C++ `isFlowBreak`).
    pub fn is_flow_break(&self) -> bool {
        (self.flags & (pcodeop_flags::branch | pcodeop_flags::returns)) != 0
    }
    /// Return `true` if this op flips the true/false meaning of its branching
    /// (C++ `isBooleanFlip`).
    pub fn is_boolean_flip(&self) -> bool {
        (self.flags & pcodeop_flags::boolean_flip) != 0
    }
    /// Return `true` if the fall-thru branch is taken when the boolean input is
    /// true (C++ `isFallthruTrue`).
    pub fn is_fallthru_true(&self) -> bool {
        (self.flags & pcodeop_flags::fallthru_true) != 0
    }
    /// Return `true` if the first input is a code reference (C++ `isCodeRef`).
    pub fn is_code_ref(&self) -> bool {
        (self.flags & pcodeop_flags::coderef) != 0
    }
    /// Return `true` if this starts an instruction (C++ `isInstructionStart`).
    pub fn is_instruction_start(&self) -> bool {
        (self.flags & pcodeop_flags::startmark) != 0
    }
    /// Return `true` if this starts a basic block (C++ `isBlockStart`).
    pub fn is_block_start(&self) -> bool {
        (self.flags & pcodeop_flags::startbasic) != 0
    }
    /// Return `true` if modified by the current action (C++ `isModified`).
    pub fn is_modified(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::modified) != 0
    }
    /// Return `true` if this op has been marked (C++ `isMark`).
    pub fn is_mark(&self) -> bool {
        (self.flags & pcodeop_flags::mark) != 0
    }
    /// Set the mark on this op (C++ `setMark`).
    pub fn set_mark(&mut self) {
        self.flags |= pcodeop_flags::mark;
    }
    /// Return `true` if a warning has been generated for this op (C++ `isWarning`).
    pub fn is_warning(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::warning) != 0
    }
    /// Clear any mark on this op (C++ `clearMark`).
    pub fn clear_mark(&mut self) {
        self.flags &= !pcodeop_flags::mark;
    }
    /// Return `true` if this causes an INDIRECT (C++ `isIndirectSource`).
    pub fn is_indirect_source(&self) -> bool {
        (self.flags & pcodeop_flags::indirect_source) != 0
    }
    /// Mark this op as source of INDIRECT (C++ `setIndirectSource`).
    pub fn set_indirect_source(&mut self) {
        self.flags |= pcodeop_flags::indirect_source;
    }
    /// Clear INDIRECT source flag (C++ `clearIndirectSource`).
    pub fn clear_indirect_source(&mut self) {
        self.flags &= !pcodeop_flags::indirect_source;
    }
    /// Return `true` if this produces/consumes ptrs (C++ `isPtrFlow`).
    pub fn is_ptr_flow(&self) -> bool {
        (self.flags & pcodeop_flags::ptrflow) != 0
    }
    /// Mark this op as consuming/producing ptrs (C++ `setPtrFlow`).
    pub fn set_ptr_flow(&mut self) {
        self.flags |= pcodeop_flags::ptrflow;
    }
    /// Return `true` if this does datatype propagation (C++ `doesSpecialPropagation`).
    pub fn does_special_propagation(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::special_prop) != 0
    }
    /// Return `true` if this needs special printing (C++ `doesSpecialPrinting`).
    pub fn does_special_printing(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::special_print) != 0
    }
    /// Return `true` if this COPY is \e incidental (C++ `isIncidentalCopy`).
    pub fn is_incidental_copy(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::incidental_copy) != 0
    }
    /// Return `true` if output is 1-bit boolean (C++ `isCalculatedBool`).
    pub fn is_calculated_bool(&self) -> bool {
        (self.flags & (pcodeop_flags::calculated_bool | pcodeop_flags::booloutput)) != 0
    }
    /// Return `true` if we have already examined this cpool (C++ `isCpoolTransformed`).
    pub fn is_cpool_transformed(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::is_cpool_transformed) != 0
    }
    /// Is data-type propagation from below stopped (C++ `stopsTypePropagation`).
    pub fn stops_type_propagation(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::stop_type_propagation) != 0
    }
    /// Stop data-type propagation from below (C++ `setStopTypePropagation`).
    pub fn set_stop_type_propagation(&mut self) {
        self.addlflags |= pcodeop_addlflags::stop_type_propagation;
    }
    /// Allow data-type propagation from below (C++ `clearStopTypePropagation`).
    pub fn clear_stop_type_propagation(&mut self) {
        self.addlflags &= !pcodeop_addlflags::stop_type_propagation;
    }
    /// (kuna) Was this comparison canonicalized from a LESSEQUAL
    /// (C++ `isCanonicalLessequal`).
    pub fn is_canonical_lessequal(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::canonical_lessequal) != 0
    }
    /// (kuna) Mark this comparison as canonicalized from a LESSEQUAL
    /// (C++ `setCanonicalLessequal`).
    pub fn set_canonical_lessequal(&mut self) {
        self.addlflags |= pcodeop_addlflags::canonical_lessequal;
    }
    /// (kuna) Clear the canonicalized-LESSEQUAL mark (C++ `clearCanonicalLessequal`).
    pub fn clear_canonical_lessequal(&mut self) {
        self.addlflags &= !pcodeop_addlflags::canonical_lessequal;
    }
    /// If `true`, do not remove output as dead code (C++ `holdOutput`).
    pub fn hold_output(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::hold_output) != 0
    }
    /// Prevent output from being removed as dead code (C++ `setHoldOutput`).
    pub fn set_hold_output(&mut self) {
        self.addlflags |= pcodeop_addlflags::hold_output;
    }
    /// Output is root of CONCAT tree (C++ `isPartialRoot`).
    pub fn is_partial_root(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::concat_root) != 0
    }
    /// Mark this as root of CONCAT tree (C++ `setPartialRoot`).
    pub fn set_partial_root(&mut self) {
        self.addlflags |= pcodeop_addlflags::concat_root;
    }
    /// Is this a \e return form COPY (C++ `isReturnCopy`).
    pub fn is_return_copy(&self) -> bool {
        (self.flags & pcodeop_flags::return_copy) != 0
    }
    /// Check if INDIRECT collapse is possible (C++ `noIndirectCollapse`).
    pub fn no_indirect_collapse(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::no_indirect_collapse) != 0
    }
    /// Prevent collapse of INDIRECT (C++ `setNoIndirectCollapse`).
    pub fn set_no_indirect_collapse(&mut self) {
        self.addlflags |= pcodeop_addlflags::no_indirect_collapse;
    }
    /// Is STORE location supposed to be unmapped (C++ `isStoreUnmapped`).
    pub fn is_store_unmapped(&self) -> bool {
        (self.addlflags & pcodeop_addlflags::store_unmapped) != 0
    }
    /// Mark that STORE location should be unmapped (C++ `setStoreUnmapped`).
    pub fn set_store_unmapped(&mut self) {
        self.addlflags |= pcodeop_addlflags::store_unmapped;
    }
    /// Return `true` if this LOADs or STOREs from a dynamic spacebase pointer
    /// (C++ `usesSpacebasePtr`).
    pub fn uses_spacebase_ptr(&self) -> bool {
        (self.flags & pcodeop_flags::spacebase_ptr) != 0
    }
    /// Return `true` if inputs commute (C++ `isCommutative`).
    pub fn is_commutative(&self) -> bool {
        (self.flags & pcodeop_flags::commutative) != 0
    }

    /// Get the behavioral class for this op, or `None` (C++ `getOpcode`).
    /// SEAM(W6): [`TypeOp`].
    pub fn get_opcode(&self) -> Option<&TypeOp> {
        self.opcode.as_ref()
    }

    /// Get the op-code id (enum) for this op (C++ `code` = `opcode->getOpcode()`).
    ///
    /// Panics if the opcode is unset (the C++ would dereference a null `opcode`
    /// — an internal-invariant violation, ADR 0004): `code()` is only ever
    /// called on ops that have been through `PcodeOpBank::change_opcode`.
    pub fn code(&self) -> OpCode {
        self.opcode
            .as_ref()
            .expect("PcodeOp::code() on op with no opcode set (C++ UB)")
            .get_opcode()
    }

    /// Return the display name of this op's op-code (C++ `getOpName` =
    /// `opcode->getName()`).  Panics if the opcode is unset (C++ UB).
    pub fn get_op_name(&self) -> &str {
        self.opcode
            .as_ref()
            .expect("PcodeOp::getOpName() on op with no opcode set (C++ UB)")
            .get_name()
    }

    /// Get the (prev, next) neighbours in the parent basic-block list
    /// (the dereferenced C++ `basiciter`).
    ///
    /// SEAM(W3-block): a [`BlockBasic`](crate::seams::BlockId) keeps its own
    /// `std::list<PcodeOp *>` (`op`/`deadlist` in `block.hh`) and caches each
    /// op's position as `basiciter`.  That per-block list head/tail belongs to
    /// the `block`/`funcdata_block` wave; the *membership links* live here on
    /// the op (the third of the three intrusive memberships, ADR 0001).  The
    /// block wave reads these via `basic_neighbours` and rewires them via
    /// [`set_basic_prev`](PcodeOp::set_basic_prev)/[`set_basic_next`](PcodeOp::set_basic_next)
    /// exactly as the C++ stores/updates `basiciter`.
    ///
    /// SEAM(W3-block): only the not-yet-ported block wave (and the in-module
    /// test) consume these, so they are unused in the op crate alone.
    #[allow(dead_code)]
    pub(crate) fn basic_neighbours(&self) -> (Option<OpId>, Option<OpId>) {
        self.links.get(ListKind::Basic)
    }
    /// Set the predecessor link in the parent basic-block list (SEAM(W3-block)).
    #[allow(dead_code)]
    pub(crate) fn set_basic_prev(&mut self, v: Option<OpId>) {
        self.links.set_prev(ListKind::Basic, v);
    }
    /// Set the successor link in the parent basic-block list (SEAM(W3-block)).
    #[allow(dead_code)]
    pub(crate) fn set_basic_next(&mut self, v: Option<OpId>) {
        self.links.set_next(ListKind::Basic, v);
    }

    /// \brief Find the slot for a Varnode that may occupy multiple input slots
    /// (C++ `PcodeOp::getRepeatSlot`, `op.cc:93`).
    ///
    /// `descend_prefix` is the prefix of `vn`'s descend list up to (exclusive
    /// of) the specific descendant iterator producing `this`; the C++ walks
    /// `vn->beginDescend()` up to `iter` counting hits of `this`.  Returns the
    /// slot, or `-1` if not found (the C++ sentinel).
    ///
    /// SEAM(W3): the C++ takes a live `vn->beginDescend()..iter` range; the
    /// Funcdata-mediated caller (`funcdata_op`) supplies that prefix as a slice
    /// of `OpId`s read from the varnode bank.
    pub fn get_repeat_slot(
        &self,
        vn: VarnodeId,
        first_slot: int4,
        descend_prefix: &[OpId],
        self_id: OpId,
    ) -> int4 {
        // count = 1 + (#occurrences of `this` in the prefix [begin, iter))
        let mut count = 1;
        for &oiter in descend_prefix {
            if oiter == self_id {
                count += 1;
            }
        }
        if count == 1 {
            return first_slot;
        }
        let mut recount = 1;
        let mut i = first_slot + 1;
        while (i as usize) < self.inrefs.len() {
            if self.inrefs[i as usize] == Some(vn) {
                recount += 1;
                if recount == count {
                    return i;
                }
            }
            i += 1;
        }
        -1
    }

    /// Print raw info about this op to a string (C++ `printRaw`,
    /// `opcode->printRaw(s,this)`).
    ///
    /// SEAM(W6): TypeOp printing depends on the W6 `TypeOp` print dispatch and
    /// per-op formatting; until W6 supplies it this returns the seam-deferred
    /// error rather than guessing a rendering.
    pub fn print_raw(&self, _s: &mut String) -> kuna_base::error::KunaResult<()> {
        Err(kuna_base::error::KunaError::lowlevel(
            "kuna rust port: PcodeOp::printRaw requires TypeOp printing (W6)",
        ))
    }
}

// ---------------------------------------------------------------------------
// Varnode-mediated PcodeOp helpers (ADR 0001: Funcdata-mediated access)
// ---------------------------------------------------------------------------
//
// These C++ `PcodeOp` methods read input/output *Varnode* contents.  Under
// ADR 0001 that access funnels through the owning store; here it is the
// `VarnodeBank` (the not-yet-ported `Funcdata` will route through the same
// bank).  Each is a free function taking `&PcodeOp` + `&VarnodeBank`, exactly
// mirroring the C++ member body.

/// Can this op be collapsed to a COPY of a constant? (C++ `PcodeOp::isCollapsible`,
/// `op.cc:115`).  All inputs must be constants and the op must be an assignment.
pub fn is_collapsible(op: &PcodeOp, vbank: &VarnodeBank) -> bool {
    if (op.flags & pcodeop_flags::nocollapse) != 0 {
        return false;
    }
    if !op.is_assignment() {
        return false;
    }
    if op.inrefs.is_empty() {
        return false;
    }
    for i in 0..op.inrefs.len() {
        let vn = vbank
            .get(op.get_in(i as int4).expect("isCollapsible: null input (C++ UB)"))
            .expect("isCollapsible: stale input id");
        if !vn.is_constant() {
            return false;
        }
    }
    let out = vbank
        .get(op.output.expect("isCollapsible: null output (C++ UB)"))
        .expect("isCollapsible: stale output id");
    // if (getOut()->getSize() > sizeof(uintb)) return false;  (int4 size, sizeof==8)
    if out.get_size() as i64 as u64 > 8 {
        return false;
    }
    true
}

/// Produce a CSE hash of the op (C++ `PcodeOp::getCseHash`, `op.cc:153`).
/// Returns 0 if the op is not cse-hashable.
pub fn get_cse_hash(op: &PcodeOp, vbank: &VarnodeBank) -> uintm {
    // if ((getEvalType()&(unary|binary))==0) return 0;
    if (op.get_eval_type() & (pcodeop_flags::unary | pcodeop_flags::binary)) == 0 {
        return 0;
    }
    if op.code() == OpCode::CPUI_COPY {
        return 0;
    }
    let out = vbank
        .get(op.output.expect("getCseHash: null output (C++ UB)"))
        .expect("getCseHash: stale output id");
    // hash = (output->getSize()<<8) | (uintm)code();
    let mut hash: uintm = ((out.get_size() as uintm) << 8) | (op.code() as i32 as uintm);
    for i in 0..op.inrefs.len() {
        let vn = vbank
            .get(op.get_in(i as int4).expect("getCseHash: null input (C++ UB)"))
            .expect("getCseHash: stale input id");
        // hash = (hash<<8) | (hash>>(sizeof(uintm)*8-8));  rotate-left by 8
        hash = (hash << 8) | (hash >> (std::mem::size_of::<uintm>() * 8 - 8));
        if vn.is_constant() {
            hash ^= vn.get_offset() as uintm;
        } else {
            hash ^= vn.get_create_index(); // uintm == uint4
        }
    }
    hash
}

/// Do two ops represent a common subexpression? (C++ `PcodeOp::isCseMatch`,
/// `op.cc:176`).  Full test corresponding to [`get_cse_hash`].
pub fn is_cse_match(op: &PcodeOp, other: &PcodeOp, vbank: &VarnodeBank) -> bool {
    if (op.get_eval_type() & (pcodeop_flags::unary | pcodeop_flags::binary)) == 0 {
        return false;
    }
    if (other.get_eval_type() & (pcodeop_flags::unary | pcodeop_flags::binary)) == 0 {
        return false;
    }
    let out1 = vbank
        .get(op.output.expect("isCseMatch: null output (C++ UB)"))
        .expect("isCseMatch: stale output id");
    let out2 = vbank
        .get(other.output.expect("isCseMatch: null output (C++ UB)"))
        .expect("isCseMatch: stale output id");
    if out1.get_size() != out2.get_size() {
        return false;
    }
    if op.code() != other.code() {
        return false;
    }
    if op.code() == OpCode::CPUI_COPY {
        return false;
    }
    if op.inrefs.len() != other.inrefs.len() {
        return false;
    }
    for i in 0..op.inrefs.len() {
        let id1 = op.get_in(i as int4);
        let id2 = other.get_in(i as int4);
        if id1 == id2 {
            continue;
        }
        let vn1 = vbank.get(id1.expect("isCseMatch: null input (C++ UB)")).expect("stale id");
        let vn2 = vbank.get(id2.expect("isCseMatch: null input (C++ UB)")).expect("stale id");
        if vn1.is_constant() && vn2.is_constant() && (vn1.get_offset() == vn2.get_offset()) {
            continue;
        }
        return false;
    }
    true
}

/// Compute the local non-zero mask for the op's output (C++
/// `PcodeOp::getNZMaskLocal`, `op.cc:571`).  Assumes the op has an output.
///
/// `cliploop` skips inputs on known looping edges (MULTIEQUAL); the loop-edge
/// test (`parent->isLoopIn(i)`) needs the block graph, so the caller passes a
/// closure `is_loop_in(slot) -> bool` for it (SEAM(W3-block); for non-MULTIEQUAL
/// ops it is never consulted).
pub fn get_nz_mask_local(
    op: &PcodeOp,
    vbank: &VarnodeBank,
    cliploop: bool,
    is_loop_in: &dyn Fn(int4) -> bool,
) -> uintb {
    // Local helpers reading the i-th input varnode through the bank.
    let in_vn = |i: int4| {
        vbank
            .get(op.get_in(i).expect("getNZMaskLocal: null input (C++ UB)"))
            .expect("getNZMaskLocal: stale input id")
    };
    let in_nz = |i: int4| -> uintb { in_vn(i).get_nz_mask() };
    let in_size = |i: int4| -> int4 { in_vn(i).get_size() };
    let in_is_const = |i: int4| -> bool { in_vn(i).is_constant() };
    let in_offset = |i: int4| -> uintb { in_vn(i).get_offset() };

    let size = vbank
        .get(op.output.expect("getNZMaskLocal: null output (C++ UB)"))
        .expect("getNZMaskLocal: stale output id")
        .get_size();
    let fullmask = calc_mask(size);
    let usize_uintb: i32 = std::mem::size_of::<uintb>() as i32; // sizeof(uintb)==8

    let resmask: uintb = match op.code() {
        OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_CARRY
        | OpCode::CPUI_INT_SCARRY
        | OpCode::CPUI_INT_SBORROW
        | OpCode::CPUI_BOOL_NEGATE
        | OpCode::CPUI_BOOL_XOR
        | OpCode::CPUI_BOOL_AND
        | OpCode::CPUI_BOOL_OR
        | OpCode::CPUI_FLOAT_EQUAL
        | OpCode::CPUI_FLOAT_NOTEQUAL
        | OpCode::CPUI_FLOAT_LESS
        | OpCode::CPUI_FLOAT_LESSEQUAL
        | OpCode::CPUI_FLOAT_NAN => 1, // Only 1 bit not guaranteed to be 0
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => in_nz(0),
        OpCode::CPUI_INT_SEXT => sign_extend_sized(in_nz(0), in_size(0), size),
        OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_OR => {
            let mut resmask = in_nz(0);
            if resmask != fullmask {
                resmask |= in_nz(1);
            }
            resmask
        }
        OpCode::CPUI_INT_AND => {
            let mut resmask = in_nz(0);
            if resmask != 0 {
                resmask &= in_nz(1);
            }
            resmask
        }
        OpCode::CPUI_INT_LEFT => {
            if !in_is_const(1) {
                fullmask
            } else {
                let sa = in_offset(1) as int4; // Get shift amount (uintb -> int4)
                let resmask = in_nz(0);
                pcode_left(resmask, sa) & fullmask
            }
        }
        OpCode::CPUI_INT_RIGHT => {
            if !in_is_const(1) {
                fullmask
            } else {
                let sz1 = in_size(0);
                let sa = in_offset(1) as int4; // Get shift amount
                let mut resmask = pcode_right(in_nz(0), sa);
                if sz1 > usize_uintb {
                    // resmask did not hold most sig bits of mask
                    if sa >= 8 * sz1 {
                        resmask = 0;
                    } else if sa >= 8 * usize_uintb {
                        // Full mask shifted over 8*sizeof(uintb)
                        resmask = calc_mask(sz1 - usize_uintb);
                        // C++ `resmask >>= (sa-8*sizeof(uintb))` relies on x86
                        // shift-count masking; sa can exceed 64 for >16-byte inputs.
                        resmask = resmask.wshr((sa - 8 * usize_uintb) as u32);
                    } else {
                        // Fill in one bits from part of mask not originally calculated
                        let mut tmp: uintb = 0;
                        tmp = tmp.wsub(1);
                        tmp = tmp.wshl((8 * usize_uintb - sa) as u32);
                        resmask |= tmp;
                    }
                }
                resmask
            }
        }
        OpCode::CPUI_INT_SRIGHT => {
            if (!in_is_const(1)) || (size > usize_uintb) {
                fullmask
            } else {
                let sa = in_offset(1) as int4; // Get shift amount
                let mut resmask = in_nz(0);
                if (resmask & (fullmask ^ (fullmask >> 1))) == 0 {
                    // If we know sign bit is zero
                    resmask = pcode_right(resmask, sa); // Same as CPUI_INT_RIGHT
                } else {
                    resmask = pcode_right(resmask, sa);
                    // C++ `(fullmask >> sa)` relies on x86 shift-count masking;
                    // a constant-folded sa can be >= 64 for a <=8-byte input.
                    resmask |= fullmask.wshr(sa as u32) ^ fullmask; // Don't know new high bits
                }
                resmask
            }
        }
        OpCode::CPUI_INT_DIV => {
            let val = in_nz(0);
            let mut resmask = coveringmask(val);
            if in_is_const(1) {
                // Dividing by power of 2 is equiv to right shift
                let sa = mostsigbit_set(in_nz(1));
                if sa != -1 {
                    resmask >>= sa; // Add sa additional zerobits
                }
            }
            resmask
        }
        OpCode::CPUI_INT_REM => {
            let val = in_nz(1).wsub(1); // Result is less than modulus
            coveringmask(val)
        }
        OpCode::CPUI_POPCOUNT => {
            let sz1 = popcount(in_nz(0));
            let mut resmask = coveringmask(sz1 as i64 as uintb); // (uintb)sz1
            resmask &= fullmask;
            resmask
        }
        OpCode::CPUI_LZCOUNT => {
            let mut resmask = coveringmask((in_size(0) * 8) as i64 as uintb);
            resmask &= fullmask;
            resmask
        }
        OpCode::CPUI_SUBPIECE => {
            let mut resmask = in_nz(0);
            let sz1 = in_offset(1) as int4;
            if in_size(0) <= usize_uintb {
                if sz1 < usize_uintb {
                    resmask >>= 8 * sz1;
                } else {
                    resmask = 0;
                }
            } else {
                // Extended precision
                if sz1 < usize_uintb {
                    resmask >>= 8 * sz1;
                    if sz1 > 0 {
                        resmask |= fullmask << (8 * (usize_uintb - sz1));
                    }
                } else {
                    resmask = fullmask;
                }
            }
            resmask &= fullmask;
            resmask
        }
        OpCode::CPUI_PIECE => {
            let sa = in_size(1);
            let mut resmask = in_nz(0);
            resmask = if sa < usize_uintb { resmask << (8 * sa) } else { 0 };
            resmask |= in_nz(1);
            resmask
        }
        OpCode::CPUI_INT_MULT => {
            let val = in_nz(0);
            let mut resmask = in_nz(1);
            if size > usize_uintb {
                resmask = fullmask;
            } else {
                let sz1 = mostsigbit_set(val);
                let sz2 = mostsigbit_set(resmask);
                if sz1 == -1 || sz2 == -1 {
                    resmask = 0;
                } else {
                    let l1 = leastsigbit_set(val);
                    let l2 = leastsigbit_set(resmask);
                    let sa = l1 + l2;
                    if sa >= 8 * size {
                        resmask = 0;
                    } else {
                        let sz1b = sz1 - l1 + 1;
                        let sz2b = sz2 - l2 + 1;
                        let mut total = sz1b + sz2b;
                        if sz1b == 1 || sz2b == 1 {
                            total -= 1;
                        }
                        resmask = fullmask;
                        if total < 8 * size {
                            resmask >>= 8 * size - total;
                        }
                        resmask = resmask.wshl(sa as u32) & fullmask;
                    }
                }
            }
            resmask
        }
        OpCode::CPUI_INT_ADD => {
            let mut resmask = in_nz(0);
            if resmask != fullmask {
                let othermask = in_nz(1);
                if (othermask & resmask) == 0 {
                    resmask |= othermask;
                } else {
                    resmask |= othermask;
                    resmask |= resmask << 1; // Account for possible carries
                }
                resmask &= fullmask;
            }
            resmask
        }
        OpCode::CPUI_MULTIEQUAL => {
            if op.inrefs.is_empty() {
                fullmask
            } else {
                let mut resmask: uintb = 0;
                if cliploop {
                    for i in 0..op.inrefs.len() {
                        if is_loop_in(i as int4) {
                            continue;
                        }
                        resmask |= in_nz(i as int4);
                    }
                } else {
                    for i in 0..op.inrefs.len() {
                        resmask |= in_nz(i as int4);
                    }
                }
                resmask
            }
        }
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CPOOLREF => {
            if op.is_calculated_bool() {
                1 // In certain cases we know the output is strictly boolean
            } else {
                fullmask
            }
        }
        _ => fullmask,
    };
    resmask
}

// ---------------------------------------------------------------------------
// IopSpace::printRaw — restored under LOSS-012 (op.cc:41-59)
// ---------------------------------------------------------------------------

/// Print info about the op an \b iop-space address refers to
/// (C++ `IopSpace::printRaw`, `op.cc:41-59`), restored from LOSS-012.
///
/// The `offset` of an iop-space address encodes a `PcodeOp *`.  In the arena
/// model that is an [`OpId`]; the kuna-base `IopSpace::printRaw` arm decodes the
/// offset to an id and routes here once the op arena is reachable.  Beyond
/// PcodeOp identity (`getSeqNum`/`isBranch`) the branch arm needs the block
/// graph (`getParent`, `bs->sizeOut`, `bs->getOut`, `bl->getStart`), supplied
/// by `block_info` (SEAM(W3-block)).
///
/// `op` is the op the address refers to; `block_info`, when the op is a branch,
/// yields the `(shortcut_char, target_start_addr)` of the branch target's start
/// block — exactly the `code_<shortcut><addr>` the C++ prints.
pub fn iop_space_print_raw(
    op: &PcodeOp,
    s: &mut String,
    block_info: &dyn Fn(&PcodeOp) -> (char, Address),
) -> kuna_base::error::KunaResult<()> {
    if !op.is_branch() {
        // op parameter for CPUI_INDIRECT
        return op.get_seq_num().print_raw(s);
    }
    // bs = op->getParent(); choose non-fallthru target; print code_<shortcut><addr>
    let (shortcut, target_start) = block_info(op);
    s.push_str("code_");
    s.push(shortcut);
    target_start.print_raw(s)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// PieceNode — a node in a tree of CPUI_PIECE operations (op.hh:269, op.cc:822)
// ---------------------------------------------------------------------------

/// \brief A node in a tree structure of CPUI_PIECE operations.
///
/// Port of the C++ `class PieceNode` (`op.hh:269-284`).  When a group of
/// Varnodes are concatenated into a larger structure, this object gathers the
/// PcodeOps (and Varnodes) in the data-flow and views them as a unit.  The
/// `pieceOp` holds the [`OpId`] (C++ stores the `PcodeOp *`); the input Varnode
/// is `pieceOp->getIn(slot)`.
#[derive(Debug, Clone, Copy)]
pub struct PieceNode {
    /// CPUI_PIECE operation combining this particular Varnode piece.
    piece_op: OpId,
    /// The particular slot of this Varnode within CPUI_PIECE.
    slot: int4,
    /// Byte offset into structure/array.
    type_offset: int4,
    /// `true` if this is a leaf of the tree structure.
    leaf: bool,
}

impl PieceNode {
    /// Constructor (C++ `PieceNode(PcodeOp *op,int4 sl,int4 off,bool l)`).
    pub fn new(op: OpId, sl: int4, off: int4, l: bool) -> PieceNode {
        PieceNode { piece_op: op, slot: sl, type_offset: off, leaf: l }
    }
    /// Return `true` if this node is a leaf of the tree structure (C++ `isLeaf`).
    pub fn is_leaf(&self) -> bool {
        self.leaf
    }
    /// Get the byte offset of this node into the data-type (C++ `getTypeOffset`).
    pub fn get_type_offset(&self) -> int4 {
        self.type_offset
    }
    /// Get the input slot associated with this node (C++ `getSlot`).
    pub fn get_slot(&self) -> int4 {
        self.slot
    }
    /// Get the PcodeOp reading this piece (C++ `getOp`).
    pub fn get_op(&self) -> OpId {
        self.piece_op
    }
    /// Get the Varnode representing this piece (C++ `getVarnode`).
    pub fn get_varnode(&self, obank: &PcodeOpBank) -> VarnodeId {
        obank
            .get(self.piece_op)
            .expect("PieceNode: stale op")
            .get_in(self.slot)
            .expect("PieceNode: null PIECE input")
    }
}

/// `vn->loneDescend()` at the bank level (C++ `Varnode::loneDescend`).
fn bank_lone_descend(obank: &PcodeOpBank, vbank: &VarnodeBank, vn: VarnodeId) -> Option<OpId> {
    let v = vbank.get(vn)?;
    if v.num_descend() != 1 {
        return None;
    }
    let op = v.descend_iter().next()?;
    // C++ loneDescend skips ops whose output is the same as the read (it does
    // not — loneDescend just returns the single descendant); kept faithful.
    let _ = obank;
    Some(op)
}

/// Determine if a Varnode is a leaf within the CONCAT tree rooted at `root_vn`
/// (C++ `PieceNode::isLeaf`, `op.cc:831`).
///
/// `rootVn->getSymbolEntry() != vn->getSymbolEntry()` (the mapped-symbol arm) is
/// a W4 seam: the merged Varnode carries no `mapentry` link, so a *mapped*
/// Varnode is treated as a leaf (the conservative C++ result when the two symbol
/// entries differ).  An unmapped Varnode never hits this arm. // SEAM(W4)
pub fn piece_is_leaf(
    obank: &PcodeOpBank,
    vbank: &VarnodeBank,
    root_vn: VarnodeId,
    vn: VarnodeId,
    rel_offset: int4,
) -> bool {
    piece_is_leaf_inner(obank, vbank, root_vn, vn, rel_offset, None)
}

/// Container-entry-aware identity for a Varnode (kuna analog of
/// `Varnode::getSymbolEntry()`); see [`piece_is_leaf_inner`].
pub type SymbolEntryKey = (crate::database::SymbolId, uintb, int4);

/// Core of [`piece_is_leaf`] with an optional Symbol-entry resolver.
///
/// When `entry_of` is supplied, the mapped arm faithfully transcribes C++
/// `if (vn->isMapped() && rootVn->getSymbolEntry() != vn->getSymbolEntry())
/// return true;` — a mapped Varnode is a leaf only when it resolves to a
/// *different* containing SymbolEntry than the root.  When `entry_of` is `None`
/// (the proto-partial-tree path that has no scope handle), the conservative W4
/// seam is preserved: any mapped non-root Varnode is a leaf.
pub fn piece_is_leaf_inner(
    obank: &PcodeOpBank,
    vbank: &VarnodeBank,
    root_vn: VarnodeId,
    vn: VarnodeId,
    rel_offset: int4,
    entry_of: Option<&dyn Fn(VarnodeId) -> Option<SymbolEntryKey>>,
) -> bool {
    let v = vbank.get(vn).expect("piece_is_leaf: stale vn");
    // if (vn->isMapped() && rootVn->getSymbolEntry() != vn->getSymbolEntry()) return true;
    if v.is_mapped() {
        match entry_of {
            Some(resolve) => {
                // C++ compares SymbolEntry pointers: leaf iff the entries differ.
                if resolve(vn) != resolve(root_vn) {
                    return true;
                }
                // Same containing entry as the root -> fall through (not a leaf
                // on this account); the structural tests below still apply.
            }
            None => {
                // SEAM(W4): without a resolver we cannot compare entries; a mapped
                // non-root Varnode is conservatively a leaf.
                return true;
            }
        }
    }
    // if (!vn->isWritten()) return true;
    if !v.is_written() {
        return true;
    }
    let def = match v.get_def() {
        Some(d) => d,
        None => return true,
    };
    // if (def->code() != CPUI_PIECE) return true;
    if obank.get(def).expect("piece_is_leaf: stale def").code() != OpCode::CPUI_PIECE {
        return true;
    }
    // PcodeOp *op = vn->loneDescend(); if (op == 0) return true;
    if bank_lone_descend(obank, vbank, vn).is_none() {
        return true;
    }
    // if (vn->isAddrTied()) { Address addr = rootVn->getAddr() + relOffset; if (vn->getAddr() != addr) return true; }
    if v.is_addr_tied() {
        let root_addr = vbank.get(root_vn).expect("piece_is_leaf: stale root").get_addr().clone();
        let addr = &root_addr + rel_offset as i64;
        if v.get_addr() != &addr {
            return true;
        }
    }
    false
}

/// Build the CONCAT tree rooted at `root_vn` (C++ `PieceNode::gatherPieces`,
/// `op.cc:895`).  Recursively walks backward through CPUI_PIECE ops, stopping at
/// leaves, recording each node's leaf-ness and offset within the root data-type.
pub fn gather_pieces(
    stack: &mut Vec<PieceNode>,
    obank: &PcodeOpBank,
    vbank: &VarnodeBank,
    root_vn: VarnodeId,
    op: OpId,
    base_offset: int4,
    root_offset: int4,
) {
    gather_pieces_inner(stack, obank, vbank, root_vn, op, base_offset, root_offset, None)
}

/// [`gather_pieces`] with an optional Symbol-entry resolver threaded to
/// [`piece_is_leaf_inner`] (C++ `RulePieceStructure` supplies the real
/// `getSymbolEntry()` comparison; the proto-partial-tree caller passes `None`).
#[allow(clippy::too_many_arguments)]
pub fn gather_pieces_inner(
    stack: &mut Vec<PieceNode>,
    obank: &PcodeOpBank,
    vbank: &VarnodeBank,
    root_vn: VarnodeId,
    op: OpId,
    base_offset: int4,
    root_offset: int4,
    entry_of: Option<&dyn Fn(VarnodeId) -> Option<SymbolEntryKey>>,
) {
    let root_big_endian = vbank
        .get(root_vn)
        .expect("gather_pieces: stale root")
        .get_addr()
        .get_space()
        .map(|s| s.is_big_endian())
        .unwrap_or(false);
    for i in 0..2 {
        let opref = obank.get(op).expect("gather_pieces: stale op");
        let vn = opref.get_in(i).expect("gather_pieces: null PIECE input");
        // int4 offset = (rootVn->getSpace()->isBigEndian() == (i==1)) ? baseOffset + op->getIn(1-i)->getSize() : baseOffset;
        let offset = if root_big_endian == (i == 1) {
            let other = opref.get_in(1 - i).expect("gather_pieces: null PIECE input");
            base_offset + vbank.get(other).expect("gather_pieces: stale other").get_size()
        } else {
            base_offset
        };
        let res = piece_is_leaf_inner(obank, vbank, root_vn, vn, offset - root_offset, entry_of);
        stack.push(PieceNode::new(op, i, offset, res));
        if !res {
            let def = vbank.get(vn).expect("gather_pieces: stale vn").get_def().expect("gather_pieces: non-leaf vn has no def");
            gather_pieces_inner(stack, obank, vbank, root_vn, def, offset, root_offset, entry_of);
        }
    }
}

// ---------------------------------------------------------------------------
// PcodeOpBank — container of all PcodeOps for a function
// ---------------------------------------------------------------------------

/// \brief Container class for PcodeOps associated with a single function
///
/// Port of the C++ `class PcodeOpBank` (`op.hh:296-359`).  The op arena lives
/// here (ADR 0001); the C++ multiple sorts become a `BTreeMap` (`optree`) plus
/// six [`IntrusiveList`]s sharing the per-op [`IntrusiveLinks`].  The
/// `deadandgone` retired list keeps ids alive (the C++ keeps the heap objects
/// until container teardown so dangling pointers stay valid); here the arena
/// keeps the slot until [`clear`](PcodeOpBank::clear).
pub struct PcodeOpBank {
    /// The op arena: the bank owns the store (ADR 0001)
    arena: OpArena,
    /// The main sequence-number sort (C++ `PcodeOpTree optree`)
    optree: BTreeMap<SeqNum, OpId>,
    /// List of \e dead PcodeOps (C++ `deadlist`)
    deadlist: IntrusiveList,
    /// List of \e alive PcodeOps (C++ `alivelist`)
    alivelist: IntrusiveList,
    /// List of STORE PcodeOps (C++ `storelist`)
    storelist: IntrusiveList,
    /// List of LOAD PcodeOps (C++ `loadlist`)
    loadlist: IntrusiveList,
    /// List of RETURN PcodeOps (C++ `returnlist`)
    returnlist: IntrusiveList,
    /// List of user-defined PcodeOps (C++ `useroplist`)
    useroplist: IntrusiveList,
    /// List of retired PcodeOps (C++ `deadandgone`)
    deadandgone: Vec<OpId>,
    /// Counter for producing unique ids for each op (C++ `uintm uniqid`)
    uniqid: uintm,
}

impl Default for PcodeOpBank {
    fn default() -> Self {
        Self::new()
    }
}

impl PcodeOpBank {
    /// Construct an empty op bank (C++ `PcodeOpBank() { uniqid = 0; }`).
    pub fn new() -> PcodeOpBank {
        PcodeOpBank {
            arena: OpArena::with_key(),
            optree: BTreeMap::new(),
            deadlist: IntrusiveList::default(),
            alivelist: IntrusiveList::default(),
            storelist: IntrusiveList::default(),
            loadlist: IntrusiveList::default(),
            returnlist: IntrusiveList::default(),
            useroplist: IntrusiveList::default(),
            deadandgone: Vec::new(),
            uniqid: 0,
        }
    }

    /// Borrow a PcodeOp by id.
    pub fn get(&self, id: OpId) -> Option<&PcodeOp> {
        self.arena.get(id)
    }
    /// Mutably borrow a PcodeOp by id.
    pub fn get_mut(&mut self, id: OpId) -> Option<&mut PcodeOp> {
        self.arena.get_mut(id)
    }

    /// Set the unique-id counter (C++ `setUniqId`).
    pub fn set_uniq_id(&mut self, val: uintm) {
        self.uniqid = val;
    }
    /// Get the unique-id counter (C++ `getUniqId`).
    pub fn get_uniq_id(&self) -> uintm {
        self.uniqid
    }

    /// Return `true` if there are no PcodeOps (C++ `empty`, tests `optree`).
    pub fn empty(&self) -> bool {
        self.optree.is_empty()
    }

    /// Add `op` to the dedicated op-code list for its op-code, if any
    /// (C++ `PcodeOpBank::addToCodeList`, `op.cc:911`).
    fn add_to_code_list(&mut self, op: OpId) {
        match self.arena[op].code() {
            OpCode::CPUI_STORE => self.storelist.push_back(&mut self.arena, ListKind::Code, op),
            OpCode::CPUI_LOAD => self.loadlist.push_back(&mut self.arena, ListKind::Code, op),
            OpCode::CPUI_RETURN => self.returnlist.push_back(&mut self.arena, ListKind::Code, op),
            OpCode::CPUI_CALLOTHER => {
                self.useroplist.push_back(&mut self.arena, ListKind::Code, op)
            }
            _ => {}
        }
    }

    /// Remove `op` from its dedicated op-code list, if any
    /// (C++ `PcodeOpBank::removeFromCodeList`, `op.cc:935`).
    fn remove_from_code_list(&mut self, op: OpId) {
        // An op with no opcode was never `add_to_code_list`'d (that runs only
        // after `set_opcode`), so it cannot be in any code list — guard the
        // `code()` read, which panics on a null opcode.  In C++ every reachable
        // op carries an opcode (`newOp` sets it immediately), so `op->code()` is
        // always safe; in the merged tree a pass can `new_op`/`create_at` an op
        // (opcode `None`, dead) and abandon it before assigning the opcode, and
        // the deadcode `destroyDead` sweep then reaches it here.
        if self.arena[op].opcode.is_none() {
            return;
        }
        match self.arena[op].code() {
            OpCode::CPUI_STORE => self.storelist.erase(&mut self.arena, ListKind::Code, op),
            OpCode::CPUI_LOAD => self.loadlist.erase(&mut self.arena, ListKind::Code, op),
            OpCode::CPUI_RETURN => self.returnlist.erase(&mut self.arena, ListKind::Code, op),
            OpCode::CPUI_CALLOTHER => self.useroplist.erase(&mut self.arena, ListKind::Code, op),
            _ => {}
        }
    }

    /// Clear all op-code specific lists (C++ `PcodeOpBank::clearCodeLists`).
    fn clear_code_lists(&mut self) {
        self.storelist.clear();
        self.loadlist.clear();
        self.returnlist.clear();
        self.useroplist.clear();
    }

    /// Create a PcodeOp with `inputs` empty input slots at the given Address;
    /// a fresh sequence number is assigned and the op is appended to the dead
    /// list (C++ `PcodeOpBank::create(int4,const Address&)`, `op.cc:971`).
    pub fn create_at(&mut self, inputs: int4, pc: Address) -> OpId {
        let sq = SeqNum::new(pc, self.uniqid);
        self.uniqid = self.uniqid.wadd(1); // uniqid++
        let op = PcodeOp::new(inputs, sq.clone());
        let id = self.arena.insert(op);
        self.optree.insert(sq, id);
        self.arena[id].set_flag(pcodeop_flags::dead); // Start out life as dead
        self.deadlist.push_back(&mut self.arena, ListKind::Insert, id);
        id
    }

    /// Create a PcodeOp with `inputs` slots and a specific sequence number,
    /// suitable for cloning/restoring (C++ `PcodeOpBank::create(int4,const
    /// SeqNum&)`, `op.cc:987`).  The op is appended to the dead list.
    pub fn create_seq(&mut self, inputs: int4, sq: SeqNum) -> OpId {
        let op = PcodeOp::new(inputs, sq.clone());
        // if (sq.getTime() >= uniqid) uniqid = sq.getTime() + 1;
        if sq.get_time() >= self.uniqid {
            self.uniqid = sq.get_time().wadd(1);
        }
        let id = self.arena.insert(op);
        self.optree.insert(sq, id);
        self.arena[id].set_flag(pcodeop_flags::dead);
        self.deadlist.push_back(&mut self.arena, ListKind::Insert, id);
        id
    }

    /// Destroy/retire every PcodeOp in the dead list (C++ `destroyDead`,
    /// `op.cc:1001`).  Iterates with the cursor advanced *before* destroy, as
    /// the C++ `op = *iter++`; we snapshot the order first since `destroy`
    /// unlinks each op.
    pub fn destroy_dead(&mut self) {
        let order: Vec<OpId> = self.deadlist.iter(&self.arena, ListKind::Insert).collect();
        for op in order {
            self.destroy(op);
        }
    }

    /// Destroy/retire the given PcodeOp (C++ `destroy`, `op.cc:1019`).
    ///
    /// The op must be dead.  It is removed from `optree`, the dead list, and its
    /// op-code list, then pushed onto the retired `deadandgone` list (the arena
    /// slot is kept — the C++ keeps the heap object until container teardown so
    /// dangling pointers stay marked dead).
    ///
    /// Panics if the op is not dead (C++ `throw LowlevelError`).
    pub fn destroy(&mut self, op: OpId) {
        if !self.arena[op].is_dead() {
            panic!("Deleting integrated op"); // C++ LowlevelError (internal invariant)
        }
        let key = self.arena[op].get_seq_num().clone();
        self.optree.remove(&key);
        self.deadlist.erase(&mut self.arena, ListKind::Insert, op);
        self.remove_from_code_list(op);
        self.deadandgone.push(op);
    }

    /// Change the op-code of `op`, moving it between op-code lists as needed
    /// (C++ `PcodeOpBank::changeOpcode`, `op.cc:1035`).
    ///
    /// SEAM(W6): `newopc` is the W6 [`TypeOp`] skeleton.
    pub fn change_opcode(&mut self, op: OpId, newopc: TypeOp) {
        // if (op->opcode != null) removeFromCodeList(op);
        if self.arena[op].opcode.is_some() {
            self.remove_from_code_list(op);
        }
        self.arena[op].set_opcode(newopc);
        self.add_to_code_list(op);
    }

    /// Mark `op` alive: move it from the dead list to the alive list
    /// (C++ `markAlive`, `op.cc:1047`).
    pub fn mark_alive(&mut self, op: OpId) {
        self.deadlist.erase(&mut self.arena, ListKind::Insert, op);
        self.arena[op].clear_flag(pcodeop_flags::dead);
        self.alivelist.push_back(&mut self.arena, ListKind::Insert, op);
    }

    /// Mark `op` dead: move it from the alive list to the dead list
    /// (C++ `markDead`, `op.cc:1058`).
    pub fn mark_dead(&mut self, op: OpId) {
        self.alivelist.erase(&mut self.arena, ListKind::Insert, op);
        self.arena[op].set_flag(pcodeop_flags::dead);
        self.deadlist.push_back(&mut self.arena, ListKind::Insert, op);
    }

    /// Move `op` to right after `prev` in the dead list (C++ `insertAfterDead`,
    /// `op.cc:1069`).  Both must be dead.  Panics otherwise (C++ LowlevelError).
    pub fn insert_after_dead(&mut self, op: OpId, prev: OpId) {
        if (!self.arena[op].is_dead()) || (!self.arena[prev].is_dead()) {
            panic!("Dead move called on ops which aren't dead");
        }
        self.deadlist.erase(&mut self.arena, ListKind::Insert, op);
        // iter = prev->insertiter; ++iter; deadlist.insert(iter, op)  ==
        // insert immediately after `prev`.
        self.deadlist.insert_after(&mut self.arena, ListKind::Insert, prev, op);
    }

    /// Move the inclusive sequence `[firstop, lastop]` to right after `prev` in
    /// the dead list (C++ `moveSequenceDead`, `op.cc:1086`).
    ///
    /// The C++ guards a degenerate move: it splices only when `prev`'s successor
    /// is not already `firstop`.
    pub fn move_sequence_dead(&mut self, firstop: OpId, lastop: OpId, prev: OpId) {
        // previter = prev->insertiter; ++previter;
        let previter = self.arena[prev].links.get(ListKind::Insert).1;
        // if (previter != firstop->insertiter)  -- degenerate-move guard
        if previter != Some(firstop) {
            // splice [first, last] to position `previter` (insert before it)
            self.deadlist
                .splice_range(&mut self.arena, ListKind::Insert, firstop, lastop, previter);
        }
    }

    /// Mark any COPY ops in `[firstop, lastop]` as \e incidental
    /// (C++ `markIncidentalCopy`, `op.cc:1101`).  Walks the dead-list segment
    /// with the cursor advanced before any mutation.
    pub fn mark_incidental_copy(&mut self, firstop: OpId, lastop: OpId) {
        // iter = firstop->insertiter; enditer = lastop->insertiter; ++enditer;
        let mut iter = Some(firstop);
        let enditer = self.arena[lastop].links.get(ListKind::Insert).1;
        while iter != enditer {
            let op = iter.expect("markIncidentalCopy: walked past list end (C++ UB)");
            iter = self.arena[op].links.get(ListKind::Insert).1; // ++iter (before mutate)
            if self.arena[op].code() == OpCode::CPUI_COPY {
                self.arena[op].set_additional_flag(pcodeop_addlflags::incidental_copy);
            }
        }
    }

    /// Find a PcodeOp by sequence number (C++ `findOp`, `op.cc:1129`).
    pub fn find_op(&self, num: &SeqNum) -> Option<OpId> {
        self.optree.get(num).copied()
    }

    // --- optree range queries (C++ lower_bound/upper_bound) -----------------

    /// Iterate all ops in sequence-number order (C++ `beginAll`..`endAll`).
    pub fn iter_all(&self) -> impl Iterator<Item = (&SeqNum, OpId)> + '_ {
        self.optree.iter().map(|(k, &v)| (k, v))
    }

    /// Iterate all ops at one Address (C++ `begin(addr)`..`end(addr)`,
    /// `op.cc:1176`).  The C++ range is `lower_bound(SeqNum(addr,0))` to
    /// `upper_bound(SeqNum(addr,~0))`; here `[Included(SeqNum(addr,0)),
    /// Included(SeqNum(addr,~0))]` — `SeqNum` orders by `(addr, uniq)`, and the
    /// maximal-uniq upper bound covers every op at this addr without spilling
    /// into a strictly-greater addr.
    pub fn iter_at(&self, addr: &Address) -> impl Iterator<Item = (&SeqNum, OpId)> + '_ {
        let lo = SeqNum::new(addr.clone(), 0);
        let hi = SeqNum::new(addr.clone(), uintm::MAX); // ~((uintm)0)
        self.optree
            .range((Bound::Included(lo), Bound::Included(hi)))
            .map(|(k, &v)| (k, v))
    }

    /// The op id whose SeqNum is at or after `addr` (`optree.lower_bound(
    /// SeqNum(addr,0))`), or `None` past the end.  The raw `beginOp(addr)` the
    /// `CommentSorter` uses (no `target()` backward scan).
    pub fn first_op_at_or_after(&self, addr: &Address) -> Option<OpId> {
        let lo = SeqNum::new(addr.clone(), 0);
        self.optree
            .range((Bound::Included(lo), Bound::Unbounded))
            .next()
            .map(|(_, &v)| v)
    }

    /// The first optree entry strictly greater than `sq` (C++ `++op_state`).
    /// O(log n) `BTreeMap::range` — the ActionPool op-cursor advance, which the
    /// C++ does in O(1) by incrementing a `PcodeOpTree::const_iterator`.
    pub fn first_after_seq(&self, sq: &SeqNum) -> Option<(&SeqNum, OpId)> {
        // Borrow `sq` into the range bound (the `(Bound<&T>, Bound<&T>)`
        // RangeBounds impl) so the per-op cursor advance does not clone the key.
        self.optree
            .range((Bound::Excluded(sq), Bound::Unbounded))
            .next()
            .map(|(k, &v)| (k, v))
    }

    /// The op id immediately before `op` in SeqNum order (C++ `--opiter`), or
    /// `None` if `op` is already the first op (`opiter == beginOpAll()`).
    pub fn op_before(&self, op: OpId) -> Option<OpId> {
        let sq = self.arena.get(op)?.get_seq_num().clone();
        self.optree
            .range((Bound::Unbounded, Bound::Excluded(sq)))
            .next_back()
            .map(|(_, &v)| v)
    }

    /// The last op id in SeqNum order (C++ `--endOpAll()` on a non-empty tree),
    /// or `None` if the bank is empty.
    pub fn last_op_all(&self) -> Option<OpId> {
        self.optree.iter().next_back().map(|(_, &v)| v)
    }

    /// Find the first executing PcodeOp at or after `addr`, pre-block-layout
    /// (C++ `PcodeOpBank::target`, `op.cc:1119`).
    ///
    /// The C++ takes `lower_bound(SeqNum(addr,0))` then calls `op->target()`,
    /// which scans backward (over the dead-list / basic-block walk) to the
    /// instruction-start op.  That backward scan is block/dead-list-mediated
    /// (SEAM(W3)); here we return the lower-bound op id and the caller
    /// (`funcdata_op`) runs `target()` on it.  Returns `None` when no op is at
    /// or after `addr`.
    pub fn target_lower_bound(&self, addr: &Address) -> Option<OpId> {
        let lo = SeqNum::new(addr.clone(), 0);
        self.optree
            .range((Bound::Included(lo), Bound::Unbounded))
            .next()
            .map(|(_, &v)| v)
    }

    /// Iterate the op ids of a dedicated op-code list in list order
    /// (C++ `begin(opc)`..`end(opc)`, `op.cc:1188`).  Op-codes without a
    /// dedicated list yield an empty iteration (the C++ returns
    /// `alivelist.end()..alivelist.end()`).
    pub fn iter_code(&self, opc: OpCode) -> Box<dyn Iterator<Item = OpId> + '_> {
        match opc {
            OpCode::CPUI_STORE => Box::new(self.storelist.iter(&self.arena, ListKind::Code)),
            OpCode::CPUI_LOAD => Box::new(self.loadlist.iter(&self.arena, ListKind::Code)),
            OpCode::CPUI_RETURN => Box::new(self.returnlist.iter(&self.arena, ListKind::Code)),
            OpCode::CPUI_CALLOTHER => {
                Box::new(self.useroplist.iter(&self.arena, ListKind::Code))
            }
            _ => Box::new(std::iter::empty()),
        }
    }

    /// Iterate the alive list in order (C++ `beginAlive`..`endAlive`).
    pub fn iter_alive(&self) -> impl Iterator<Item = OpId> + '_ {
        self.alivelist.iter(&self.arena, ListKind::Insert)
    }

    /// Iterate the dead list in order (C++ `beginDead`..`endDead`).
    pub fn iter_dead(&self) -> impl Iterator<Item = OpId> + '_ {
        self.deadlist.iter(&self.arena, ListKind::Insert)
    }

    /// Number of ops in the alive list.
    pub fn num_alive(&self) -> usize {
        self.alivelist.len()
    }
    /// Number of ops in the dead list.
    pub fn num_dead(&self) -> usize {
        self.deadlist.len()
    }

    /// Clear all PcodeOps from the bank (C++ `PcodeOpBank::clear`, `op.cc:1224`).
    /// Resets `uniqid` to 0.
    pub fn clear(&mut self) {
        // The C++ deletes the alive/dead/deadandgone heap objects; here we drop
        // the whole arena.  Clear all lists and the optree, reset uniqid.
        self.arena.clear();
        self.optree.clear();
        self.alivelist.clear();
        self.deadlist.clear();
        self.clear_code_lists();
        self.deadandgone.clear();
        self.uniqid = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use kuna_base::types::int4;

    use crate::dtype::{type_metatype, Datatype};
    use crate::varnode::VarnodeBank;

    fn dt(size: int4) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
    }

    /// const(0), unique(1), ram(2, 8-byte addressable).
    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn ram(m: &AddrSpaceManager, off: u64) -> Address {
        let spc = Rc::clone(m.get_space(2).unwrap());
        Address::new(spc, off)
    }

    /// Minimal TypeOp seam values for the op-codes the bank tests touch.
    fn typeop(opc: OpCode) -> TypeOp {
        // Flags don't matter for the list-bucketing tests; name is the symbol.
        TypeOp::new(opc, 0, format!("{opc:?}"))
    }

    // --- allocation-order monotonicity ------------------------------------

    #[test]
    fn uniqid_monotonic_and_seqnum_time() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let a = bank.create_at(2, ram(&m, 0x100));
        let b = bank.create_at(1, ram(&m, 0x100));
        let c = bank.create_at(0, ram(&m, 0x200));
        // uniqid feeds SeqNum::uniq one-up from 0.
        assert_eq!(bank.get(a).unwrap().get_time(), 0);
        assert_eq!(bank.get(b).unwrap().get_time(), 1);
        assert_eq!(bank.get(c).unwrap().get_time(), 2);
        assert_eq!(bank.get_uniq_id(), 3);
        // input slots reserved, all null
        assert_eq!(bank.get(a).unwrap().num_input(), 2);
        assert_eq!(bank.get(a).unwrap().get_in(0), None);
    }

    #[test]
    fn create_seq_advances_uniqid() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        // A specific high seqnum bumps uniqid to time+1.
        let sq = SeqNum::new(ram(&m, 0x10), 41);
        let _id = bank.create_seq(1, sq);
        assert_eq!(bank.get_uniq_id(), 42);
        // A later create_at uses the bumped uniqid.
        let id2 = bank.create_at(0, ram(&m, 0x20));
        assert_eq!(bank.get(id2).unwrap().get_time(), 42);
        assert_eq!(bank.get_uniq_id(), 43);
    }

    // --- optree iteration order -------------------------------------------

    #[test]
    fn optree_iteration_is_seqnum_order() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        // Create out of address order; optree must sort by (addr, uniq).
        let _hi = bank.create_at(0, ram(&m, 0x200)); // uniq 0
        let _lo = bank.create_at(0, ram(&m, 0x100)); // uniq 1
        let _mid = bank.create_at(0, ram(&m, 0x100)); // uniq 2 (same addr as lo)
        let times: Vec<u32> = bank.iter_all().map(|(k, _)| k.get_time()).collect();
        // addr 0x100 first: lo(uniq1) then mid(uniq2); then addr 0x200: hi(uniq0)
        assert_eq!(times, vec![1, 2, 0]);
    }

    #[test]
    fn iter_at_address_bounds() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let _a = bank.create_at(0, ram(&m, 0x100)); // uniq 0
        let _b = bank.create_at(0, ram(&m, 0x200)); // uniq 1
        let _c = bank.create_at(0, ram(&m, 0x100)); // uniq 2
        // only the two ops at 0x100, in uniq order
        let at: Vec<u32> = bank.iter_at(&ram(&m, 0x100)).map(|(k, _)| k.get_time()).collect();
        assert_eq!(at, vec![0, 2]);
        let at2: Vec<u32> = bank.iter_at(&ram(&m, 0x200)).map(|(k, _)| k.get_time()).collect();
        assert_eq!(at2, vec![1]);
        // target_lower_bound at 0x150 -> first op at-or-after == the 0x200 op
        let tgt = bank.target_lower_bound(&ram(&m, 0x150)).unwrap();
        assert_eq!(bank.get(tgt).unwrap().get_addr().get_offset(), 0x200);
    }

    // --- alive/dead list splice semantics vs a Vec model ------------------

    #[test]
    fn mark_alive_dead_roundtrip() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let a = bank.create_at(0, ram(&m, 0x10));
        let b = bank.create_at(0, ram(&m, 0x20));
        let c = bank.create_at(0, ram(&m, 0x30));
        // all start dead, in creation order
        assert_eq!(bank.iter_dead().collect::<Vec<_>>(), vec![a, b, c]);
        assert_eq!(bank.num_dead(), 3);
        // mark b alive: dead -> [a, c], alive -> [b]
        bank.mark_alive(b);
        assert_eq!(bank.iter_dead().collect::<Vec<_>>(), vec![a, c]);
        assert_eq!(bank.iter_alive().collect::<Vec<_>>(), vec![b]);
        assert!(!bank.get(b).unwrap().is_dead());
        // mark a alive too: alive append-order -> [b, a]
        bank.mark_alive(a);
        assert_eq!(bank.iter_alive().collect::<Vec<_>>(), vec![b, a]);
        assert_eq!(bank.iter_dead().collect::<Vec<_>>(), vec![c]);
        // mark b dead again: appended to dead tail -> [c, b]
        bank.mark_dead(b);
        assert_eq!(bank.iter_dead().collect::<Vec<_>>(), vec![c, b]);
        assert_eq!(bank.iter_alive().collect::<Vec<_>>(), vec![a]);
    }

    #[test]
    fn insert_after_dead_places_correctly() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let a = bank.create_at(0, ram(&m, 0x10));
        let b = bank.create_at(0, ram(&m, 0x20));
        let c = bank.create_at(0, ram(&m, 0x30));
        // dead == [a, b, c]; move c after a -> [a, c, b]
        bank.insert_after_dead(c, a);
        assert_eq!(bank.iter_dead().collect::<Vec<_>>(), vec![a, c, b]);
    }

    /// moveSequenceDead must match a Vec splice reference for the inclusive
    /// range, including the C++ degenerate-move guard.
    #[test]
    fn move_sequence_dead_vs_vec_model() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let ops: Vec<OpId> = (0..6).map(|i| bank.create_at(0, ram(&m, 0x10 * i))).collect();
        // dead == [0,1,2,3,4,5]; move [1,2,3] to after 4 -> [0,4,1,2,3,5]
        bank.move_sequence_dead(ops[1], ops[3], ops[4]);
        let want = vec![ops[0], ops[4], ops[1], ops[2], ops[3], ops[5]];
        assert_eq!(bank.iter_dead().collect::<Vec<_>>(), want);
        // degenerate move: firstop already immediately follows prev -> unchanged
        let before = bank.iter_dead().collect::<Vec<_>>();
        // 1 currently follows 4; "after 4" is the no-op case for firstop=1
        bank.move_sequence_dead(ops[1], ops[1], ops[4]);
        assert_eq!(bank.iter_dead().collect::<Vec<_>>(), before);
    }

    // --- per-opcode code lists in insertion order --------------------------

    #[test]
    fn code_lists_bucket_in_insertion_order() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let s1 = bank.create_at(3, ram(&m, 0x10));
        let l1 = bank.create_at(2, ram(&m, 0x20));
        let s2 = bank.create_at(3, ram(&m, 0x30));
        let r1 = bank.create_at(1, ram(&m, 0x40));
        bank.change_opcode(s1, typeop(OpCode::CPUI_STORE));
        bank.change_opcode(l1, typeop(OpCode::CPUI_LOAD));
        bank.change_opcode(s2, typeop(OpCode::CPUI_STORE));
        bank.change_opcode(r1, typeop(OpCode::CPUI_RETURN));
        // store list in insertion order
        assert_eq!(bank.iter_code(OpCode::CPUI_STORE).collect::<Vec<_>>(), vec![s1, s2]);
        assert_eq!(bank.iter_code(OpCode::CPUI_LOAD).collect::<Vec<_>>(), vec![l1]);
        assert_eq!(bank.iter_code(OpCode::CPUI_RETURN).collect::<Vec<_>>(), vec![r1]);
        // a non-bucketed opcode yields empty
        assert_eq!(bank.iter_code(OpCode::CPUI_INT_ADD).count(), 0);
    }

    #[test]
    fn change_opcode_moves_between_code_lists() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let a = bank.create_at(2, ram(&m, 0x10));
        bank.change_opcode(a, typeop(OpCode::CPUI_STORE));
        assert_eq!(bank.iter_code(OpCode::CPUI_STORE).collect::<Vec<_>>(), vec![a]);
        // re-assign to LOAD: must leave store list, join load list
        bank.change_opcode(a, typeop(OpCode::CPUI_LOAD));
        assert_eq!(bank.iter_code(OpCode::CPUI_STORE).count(), 0);
        assert_eq!(bank.iter_code(OpCode::CPUI_LOAD).collect::<Vec<_>>(), vec![a]);
        assert_eq!(bank.get(a).unwrap().code(), OpCode::CPUI_LOAD);
    }

    // --- destroy / destroyDead / clear ------------------------------------

    #[test]
    fn destroy_dead_op_removes_from_optree_and_lists() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let a = bank.create_at(2, ram(&m, 0x10));
        bank.change_opcode(a, typeop(OpCode::CPUI_STORE));
        let b = bank.create_at(0, ram(&m, 0x20));
        assert_eq!(bank.optree.len(), 2);
        bank.destroy(a); // a is dead
        assert_eq!(bank.optree.len(), 1);
        assert_eq!(bank.iter_dead().collect::<Vec<_>>(), vec![b]);
        assert_eq!(bank.iter_code(OpCode::CPUI_STORE).count(), 0);
        assert!(bank.find_op(&SeqNum::new(ram(&m, 0x10), 0)).is_none());
    }

    #[test]
    #[should_panic(expected = "Deleting integrated op")]
    fn destroy_alive_op_panics() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let a = bank.create_at(0, ram(&m, 0x10));
        bank.mark_alive(a);
        bank.destroy(a); // not dead -> panic
    }

    #[test]
    fn destroy_dead_clears_dead_list() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        // Ops carry an opcode by the time they are destroyed (the real flow
        // runs changeOpcode right after create); removeFromCodeList reads it.
        let a = bank.create_at(0, ram(&m, 0x10));
        let b = bank.create_at(0, ram(&m, 0x20));
        bank.change_opcode(a, typeop(OpCode::CPUI_INT_ADD));
        bank.change_opcode(b, typeop(OpCode::CPUI_STORE));
        assert_eq!(bank.num_dead(), 2);
        bank.destroy_dead();
        assert_eq!(bank.num_dead(), 0);
        assert!(bank.empty());
        // the STORE op left its dedicated code list too
        assert_eq!(bank.iter_code(OpCode::CPUI_STORE).count(), 0);
    }

    #[test]
    fn clear_resets_uniqid_and_everything() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let a = bank.create_at(2, ram(&m, 0x10));
        bank.change_opcode(a, typeop(OpCode::CPUI_LOAD));
        bank.mark_alive(a);
        bank.clear();
        assert!(bank.empty());
        assert_eq!(bank.get_uniq_id(), 0);
        assert_eq!(bank.num_alive(), 0);
        assert_eq!(bank.num_dead(), 0);
        assert_eq!(bank.iter_code(OpCode::CPUI_LOAD).count(), 0);
    }

    // --- markIncidentalCopy (cursor-while-mutating) ------------------------

    #[test]
    fn mark_incidental_copy_marks_only_copies_in_range() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let a = bank.create_at(1, ram(&m, 0x10));
        let b = bank.create_at(1, ram(&m, 0x20));
        let c = bank.create_at(1, ram(&m, 0x30));
        let d = bank.create_at(1, ram(&m, 0x40));
        bank.change_opcode(a, typeop(OpCode::CPUI_COPY));
        bank.change_opcode(b, typeop(OpCode::CPUI_INT_ADD));
        bank.change_opcode(c, typeop(OpCode::CPUI_COPY));
        bank.change_opcode(d, typeop(OpCode::CPUI_COPY));
        // mark [a, c]: a and c are COPY (marked), b is ADD (skipped), d is out of range
        bank.mark_incidental_copy(a, c);
        assert!(bank.get(a).unwrap().is_incidental_copy());
        assert!(!bank.get(b).unwrap().is_incidental_copy());
        assert!(bank.get(c).unwrap().is_incidental_copy());
        assert!(!bank.get(d).unwrap().is_incidental_copy());
    }

    // --- PcodeOp slot / input mutation -------------------------------------

    #[test]
    fn get_slot_and_input_mutation() {
        let m = build_manager();
        let mut vbank = VarnodeBank::new(&m, 0).unwrap();
        let v0 = vbank.create(4, ram(&m, 0x10), dt(4));
        let v1 = vbank.create(4, ram(&m, 0x20), dt(4));
        let mut op = PcodeOp::new(2, SeqNum::new(ram(&m, 0x100), 0));
        op.set_input(Some(v0), 0);
        op.set_input(Some(v1), 1);
        assert_eq!(op.get_slot(v0), 0);
        assert_eq!(op.get_slot(v1), 1);
        // absent varnode -> numInput (C++ returns i==n, not -1)
        let vx = vbank.create(4, ram(&m, 0x30), dt(4));
        assert_eq!(op.get_slot(vx), 2);
        // removeInput renumbers
        op.remove_input(0);
        assert_eq!(op.num_input(), 1);
        assert_eq!(op.get_in(0), Some(v1));
        // insertInput shifts
        op.insert_input(0);
        assert_eq!(op.num_input(), 2);
        assert_eq!(op.get_in(0), None);
        assert_eq!(op.get_in(1), Some(v1));
    }

    /// The basic-block membership (third intrusive list) is driven by the
    /// block wave via the per-op `set_basic_prev`/`set_basic_next` links the
    /// C++ caches as `basiciter`; verify they round-trip.
    #[test]
    fn basic_block_membership_links_roundtrip() {
        let m = build_manager();
        let mut bank = PcodeOpBank::new();
        let a = bank.create_at(0, ram(&m, 0x10));
        let b = bank.create_at(0, ram(&m, 0x20));
        let c = bank.create_at(0, ram(&m, 0x30));
        // model a per-block list [a, b, c] by wiring the basic links directly
        bank.get_mut(a).unwrap().set_basic_next(Some(b));
        bank.get_mut(b).unwrap().set_basic_prev(Some(a));
        bank.get_mut(b).unwrap().set_basic_next(Some(c));
        bank.get_mut(c).unwrap().set_basic_prev(Some(b));
        assert_eq!(bank.get(a).unwrap().basic_neighbours(), (None, Some(b)));
        assert_eq!(bank.get(b).unwrap().basic_neighbours(), (Some(a), Some(c)));
        assert_eq!(bank.get(c).unwrap().basic_neighbours(), (Some(b), None));
        // the basic membership is independent of the alive/dead (insert) list
        assert_eq!(bank.iter_dead().collect::<Vec<_>>(), vec![a, b, c]);
    }

    #[test]
    fn set_opcode_caches_flags() {
        let m = build_manager();
        let mut op = PcodeOp::new(2, SeqNum::new(ram(&m, 0x100), 0));
        // pretend INT_ADD is binary|commutative
        let flags = pcodeop_flags::binary | pcodeop_flags::commutative;
        op.set_opcode(TypeOp::new(OpCode::CPUI_INT_ADD, flags, "+"));
        assert!(op.is_commutative());
        assert_eq!(op.get_eval_type(), pcodeop_flags::binary);
        assert_eq!(op.code(), OpCode::CPUI_INT_ADD);
        // reassigning to a non-commutative op-code clears the opcode-derived bits
        op.set_opcode(TypeOp::new(OpCode::CPUI_COPY, pcodeop_flags::unary, "copy"));
        assert!(!op.is_commutative());
        assert_eq!(op.get_eval_type(), pcodeop_flags::unary);
    }

    // --- get_repeat_slot ---------------------------------------------------

    #[test]
    fn get_repeat_slot_handles_duplicate_inputs() {
        let m = build_manager();
        let mut vbank = VarnodeBank::new(&m, 0).unwrap();
        let v = vbank.create(4, ram(&m, 0x10), dt(4));
        let w = vbank.create(4, ram(&m, 0x20), dt(4));
        // op with v in slots 0 and 2, w in slot 1
        let mut op = PcodeOp::new(3, SeqNum::new(ram(&m, 0x100), 0));
        op.set_input(Some(v), 0);
        op.set_input(Some(w), 1);
        op.set_input(Some(v), 2);
        let self_id: OpId = slotmap::SlotMap::<OpId, ()>::with_key().insert(());
        // first occurrence (empty prefix -> count 1) returns first_slot
        assert_eq!(op.get_repeat_slot(v, 0, &[], self_id), 0);
        // second occurrence: prefix contains one earlier hit of self -> count 2
        assert_eq!(op.get_repeat_slot(v, 0, &[self_id], self_id), 2);
    }

    // --- PieceNode / gather_pieces ----------------------------------------

    /// A single-level CONCAT tree `PIECE(hi@4, lo@0)` over two free (leaf)
    /// Varnodes is gathered into two leaf nodes.  In a little-endian space, slot 0
    /// (the most-significant input) gets type-offset == in1->getSize(), slot 1
    /// (least-significant) gets type-offset 0 (C++ `PieceNode::gatherPieces` byte
    /// math, `op.cc:900`).
    #[test]
    fn gather_pieces_single_level_two_leaves() {
        let m = build_manager();
        let mut vbank = VarnodeBank::new(&m, 0).unwrap();
        let mut obank = PcodeOpBank::new();

        // out @ ram:0 (8 bytes), built from hi @ ram:4 (4b) and lo @ ram:0 (4b).
        let lo = vbank.create(4, ram(&m, 0x0), dt(4));
        let hi = vbank.create(4, ram(&m, 0x4), dt(4));
        let out = vbank.create(8, ram(&m, 0x0), dt(8));

        let pieceop = obank.create_at(2, ram(&m, 0x100));
        {
            let o = obank.get_mut(pieceop).unwrap();
            o.set_opcode(typeop(OpCode::CPUI_PIECE));
            o.set_input(Some(hi), 0); // most-significant
            o.set_input(Some(lo), 1); // least-significant
            o.set_output(Some(out));
        }
        // Link the output's def + the inputs' descend so the bank reads are
        // consistent (gather_pieces only reads code()/get_in/get_size here).
        vbank.add_descend(hi, pieceop).unwrap();
        vbank.add_descend(lo, pieceop).unwrap();

        let mut stack: Vec<PieceNode> = Vec::new();
        gather_pieces(&mut stack, &obank, &vbank, out, pieceop, 0, 0);

        // Both inputs are free (no def) -> leaves; the recursion stops at them.
        assert_eq!(stack.len(), 2);
        // slot 0 (hi) -> offset = in1(lo)->getSize() = 4; slot 1 (lo) -> offset 0.
        assert_eq!(stack[0].get_slot(), 0);
        assert_eq!(stack[0].get_type_offset(), 4);
        assert!(stack[0].is_leaf());
        assert_eq!(stack[0].get_varnode(&obank), hi);
        assert_eq!(stack[1].get_slot(), 1);
        assert_eq!(stack[1].get_type_offset(), 0);
        assert!(stack[1].is_leaf());
        assert_eq!(stack[1].get_varnode(&obank), lo);
    }

    /// `piece_is_leaf` returns true for a free (unwritten) Varnode: a free read
    /// is always a leaf of the CONCAT tree (C++ `PieceNode::isLeaf` `!isWritten()`
    /// arm, `op.cc:837`).
    #[test]
    fn piece_is_leaf_free_is_leaf() {
        let m = build_manager();
        let mut vbank = VarnodeBank::new(&m, 0).unwrap();
        let obank = PcodeOpBank::new();
        let root = vbank.create(8, ram(&m, 0x0), dt(8));
        let free = vbank.create(4, ram(&m, 0x0), dt(4));
        assert!(piece_is_leaf(&obank, &vbank, root, free, 0));
    }
}
