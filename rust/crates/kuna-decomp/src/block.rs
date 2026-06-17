//! Port of `decompiler/cpp/block.{hh,cc}` (W3, item `w3-ir-block`) — the
//! `FlowBlock` control-flow hierarchy, its per-function arena, the edge
//! manipulation primitives, the Cooper/Harvey/Kennedy dominator algorithm,
//! the Tarjan spanning-tree / irreducibility passes, and the block-structure
//! encode/decode.
//!
//! ## ADR 0001 (IR arenas) realization
//!
//! The C++ `FlowBlock` is a heap object cross-referenced by raw pointers:
//! `parent`/`immed_dom`/`copymap` are `FlowBlock *`, each `BlockEdge.point` is
//! a `FlowBlock *`, and a `BlockGraph` *owns* its components through a
//! `vector<FlowBlock *> list` (its destructor `delete`s them).  Per ADR 0001
//! every block lives in a [`BlockArena`] (a `slotmap` generational arena) keyed
//! by the newtype [`BlockId`](crate::seams::BlockId); the pointer members
//! become `Option<BlockId>` and the component list becomes `Vec<BlockId>`.
//! Edge endpoints (`BlockEdge.point`) are likewise `BlockId`s.
//!
//! Because edges cross-reference *other* blocks, every edge-manipulation
//! primitive (the C++ `FlowBlock::addInEdge`/`removeInEdge`/`replaceOutEdge`/…
//! and `BlockGraph::addEdge`/`removeEdge`/`switchEdge`/…) is a method on the
//! owning [`BlockArena`] that takes the participating [`BlockId`]s, exactly as
//! op.rs's intrusive-list helpers take `&mut OpArena`.  This preserves the C++
//! aliasing — `a->addInEdge(b)` mutates *both* `a.intothis` and `b.outofthis` —
//! which Rust cannot express with two simultaneous `&mut`.
//!
//! ## Subtype dispatch (look-ahead note for W7 `blockaction.cc`)
//!
//! The C++ hierarchy is `FlowBlock` (base) → `BlockGraph` → the structured
//! subtypes (`BlockCopy`, `BlockGoto`, `BlockMultiGoto`, `BlockList`,
//! `BlockCondition`, `BlockIf`, `BlockWhileDo`, `BlockDoWhile`, `BlockInfLoop`,
//! `BlockSwitch`) and the leaf `BlockBasic`.  All of them share the same base
//! fields and the same edge/dominator algorithms — the polymorphism `block.cc`
//! actually relies on for *tree manipulation* is a small set of virtuals
//! (`getType`, `subBlock`, `getExitLeaf`, `nextFlowAfter`, `firstOp`, `lastOp`,
//! `getSize`/`getBlock`).  Following op.rs's enum-of-kind convention (and the
//! task's guidance), this port uses **one [`FlowBlock`] struct** carrying every
//! base field plus a [`BlockKind`] enum that holds the per-subtype payload
//! (component list, goto target, condition opcode, switch cases, …).  The
//! `getType()` switch and the per-subtype data are thus a single `match` on
//! `kind`, and a `BlockGraph` *is* a `FlowBlock` whose `kind` is one of the
//! graph variants — exactly the C++ is-a relationship.  W7's `blockaction.cc`
//! collapse/identify passes will dispatch on [`FlowBlock::get_type`] /
//! [`FlowBlock::kind`] the same way `block.cc` dispatches on `getType()`.
//!
//! ## Deferred surfaces (W7 / W8)
//!
//! The structuring *decision* logic and all printing depend on subsystems not
//! yet ported:
//!   - `emit`/`printRaw`/`printHeader`/`printTree` need `PrintLanguage` (W8);
//!     they are seam-noted (`// SEAM(W8)`) and not transcribed here.
//!   - The SSA/data-flow-heavy `BlockBasic` methods (`isComplex`,
//!     `unblockedMulti`, `noInterveningStatement`, `findMultiequal`,
//!     `earliestUse`, …), `BlockWhileDo::findLoopVariable` and friends,
//!     `flipInPlaceTest`/`flipInPlaceExecute`, and `preferComplement` need
//!     `Funcdata`/`Varnode`/`HighVariable` (W7); they are seam-noted
//!     (`// SEAM(W7)`) and left to the funcdata/blockaction wave.
//!   - `getJumptable`/`getSwitchType`/`grabCaseBasic` need `JumpTable` and the
//!     type system (W4/W7); the switch *structure* (`CaseOrder`, edge handling)
//!     is carried, the jump-table-driven label assignment is seam-noted.
//!
//! The deliverable is the **tree**: the FlowBlock data model, the edge
//! primitives, the dominator/spanning-tree/loop algorithms, and encode/decode.
//!
//! ## op.rs seam surfaces fulfilled here
//!
//! op.rs documented (`// SEAM(W3-block)`) that `PcodeOp::nextOp`/`previousOp`/
//! `target`/`compareOrder` and `IopSpace::printRaw`'s branch arm need the block
//! graph.  `compareOrder` needs only [`FlowBlock::find_common_block`] (provided
//! here) plus the two ops' parents; the others need *both* the op intrusive
//! lists and the block graph, i.e. the not-yet-ported `Funcdata`.  This file
//! provides the block-side halves as free functions taking both
//! [`PcodeOpBank`](crate::op::PcodeOpBank) and [`BlockArena`]; the `funcdata_op`
//! wave will call them (or inline them) once it owns both banks.

#![allow(clippy::needless_range_loop)] // C++ indexes by edge slot; preserve it

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    AttributeId, Decoder, ElementId, Encoder, ATTRIB_INDEX, ATTRIB_TYPE, ELEM_TARGET,
};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::{get_opname, OpCode};
use slotmap::SlotMap;

use crate::op::PcodeOp;
use crate::seams::BlockId;

// ---------------------------------------------------------------------------
// Marshaling ids (block-local in C++ `block.cc:22-31`)
// ---------------------------------------------------------------------------

/// Marshaling attribute "altindex" (C++ `ATTRIB_ALTINDEX`, `block.cc:22`).
pub const ATTRIB_ALTINDEX: AttributeId = AttributeId::new("altindex", 75);
/// Marshaling attribute "depth" (C++ `ATTRIB_DEPTH`, `block.cc:23`).
pub const ATTRIB_DEPTH: AttributeId = AttributeId::new("depth", 76);
/// Marshaling attribute "end" (C++ `ATTRIB_END`, `block.cc:24`).
pub const ATTRIB_END: AttributeId = AttributeId::new("end", 77);
/// Marshaling attribute "opcode" (C++ `ATTRIB_OPCODE`, `block.cc:25`).
pub const ATTRIB_OPCODE: AttributeId = AttributeId::new("opcode", 78);
/// Marshaling attribute "rev" (C++ `ATTRIB_REV`, `block.cc:26`).
pub const ATTRIB_REV: AttributeId = AttributeId::new("rev", 79);

/// Marshaling element \<bhead> (C++ `ELEM_BHEAD`, `block.cc:28`).
pub const ELEM_BHEAD: ElementId = ElementId::new("bhead", 102);
/// Marshaling element \<block> (C++ `ELEM_BLOCK`, `block.cc:29`).
pub const ELEM_BLOCK: ElementId = ElementId::new("block", 103);
/// Marshaling element \<blockedge> (C++ `ELEM_BLOCKEDGE`, `block.cc:30`).
pub const ELEM_BLOCKEDGE: ElementId = ElementId::new("blockedge", 104);
/// Marshaling element \<edge> (C++ `ELEM_EDGE`, `block.cc:31`).
pub const ELEM_EDGE: ElementId = ElementId::new("edge", 105);

// ---------------------------------------------------------------------------
// Block / edge flag enums (verbatim from block.hh:77-121)
// ---------------------------------------------------------------------------

/// The possible block types (C++ `FlowBlock::block_type`, `block.hh:77-80`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    /// A plain FlowBlock (`t_plain`)
    Plain,
    /// A basic block (`t_basic`)
    Basic,
    /// A generic BlockGraph (`t_graph`)
    Graph,
    /// A BlockCopy (`t_copy`)
    Copy,
    /// A BlockGoto (`t_goto`)
    Goto,
    /// A BlockMultiGoto (`t_multigoto`)
    MultiGoto,
    /// A BlockList (`t_ls`)
    Ls,
    /// A BlockCondition (`t_condition`)
    Condition,
    /// A BlockIf (`t_if`)
    If,
    /// A BlockWhileDo (`t_whiledo`)
    WhileDo,
    /// A BlockDoWhile (`t_dowhile`)
    DoWhile,
    /// A BlockSwitch (`t_switch`)
    Switch,
    /// A BlockInfLoop (`t_infloop`)
    InfLoop,
}

/// Boolean properties of blocks (C++ `FlowBlock::block_flags`,
/// `block.hh:88-107`).
pub mod block_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;

    /// (Block ends in) non-structured branch
    pub const f_goto_goto: uint4 = 1;
    /// Block ends with a break;
    pub const f_break_goto: uint4 = 2;
    /// Block ends with a continue;
    pub const f_continue_goto: uint4 = 4;
    /// Output is decided by switch
    pub const f_switch_out: uint4 = 0x10;
    /// Block is destination of unstructured goto
    pub const f_unstructured_targ: uint4 = 0x20;
    /// Generic way to mark a block
    pub const f_mark: uint4 = 0x80;
    /// A secondary mark
    pub const f_mark2: uint4 = 0x100;
    /// Official entry point of the function
    pub const f_entry_point: uint4 = 0x200;
    /// The block has an unstructured jump out of interior
    pub const f_interior_gotoout: uint4 = 0x400;
    /// Block is target of unstructured jump to its interior
    pub const f_interior_gotoin: uint4 = 0x800;
    /// Any label printed higher up in hierarchy
    pub const f_label_bumpup: uint4 = 0x1000;
    /// Block does nothing in infinite loop (halt)
    pub const f_donothing_loop: uint4 = 0x2000;
    /// Block is in process of being deleted
    pub const f_dead: uint4 = 0x4000;
    /// Set if the conditional block of a whiledo is too big to print as while(cond) {
    pub const f_whiledo_overflow: uint4 = 0x8000;
    /// If true, out edges have been flipped since last time path was traced
    pub const f_flip_path: uint4 = 0x10000;
    /// Block is a merged form of original basic blocks
    pub const f_joined_block: uint4 = 0x20000;
    /// Block is a duplicated version of an original basic block
    pub const f_duplicate_block: uint4 = 0x40000;
    /// Potential \e do \e nothing block whose removal has been delayed
    pub const f_delayed_donothing: uint4 = 0x80000;
    /// Has the final transform been run
    pub const f_final_transform: uint4 = 0x100000;
}

/// Boolean properties on edges (C++ `FlowBlock::edge_flags`,
/// `block.hh:110-121`).
pub mod edge_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;

    /// Edge is unstructured
    pub const f_goto_edge: uint4 = 1;
    /// Edge completes a loop, removing these edges gives you a DAG
    pub const f_loop_edge: uint4 = 2;
    /// This is default edge from switchblock
    pub const f_defaultswitch_edge: uint4 = 4;
    /// Edge which must be removed to make graph reducible
    pub const f_irreducible: uint4 = 8;
    /// An edge in the spanning tree
    pub const f_tree_edge: uint4 = 0x10;
    /// An edge that jumps forward in the spanning tree
    pub const f_forward_edge: uint4 = 0x20;
    /// An edge that crosses subtrees in the spanning tree
    pub const f_cross_edge: uint4 = 0x40;
    /// Within (reducible) graph, a back edge defining a loop
    pub const f_back_edge: uint4 = 0x80;
    /// Edge exits the body of a loop
    pub const f_loop_exit_edge: uint4 = 0x100;
    /// Copy propagation has happened across the edge
    pub const f_immed_copy: uint4 = 0x200;
}

// ---------------------------------------------------------------------------
// BlockEdge
// ---------------------------------------------------------------------------

/// \brief A control-flow edge between blocks (C++ `BlockEdge`,
/// `block.hh:57-65`).
///
/// The edge is owned by the source block; `point` is the [`BlockId`] at the
/// other end (the C++ `FlowBlock *point`), `reverse_index` is the slot of the
/// mirror edge in the other block's edge list, and `label` holds
/// [`edge_flags`].
#[derive(Debug, Clone, Default)]
pub struct BlockEdge {
    /// Label of the edge (C++ `uint4 label`)
    pub label: uint4,
    /// Other end of the edge (C++ `FlowBlock *point`).  `None` only transiently
    /// during decode before [`BlockEdge::decode`] resolves it.
    pub point: Option<BlockId>,
    /// Index for edge coming the other way (C++ `int4 reverse_index`)
    pub reverse_index: int4,
}

impl BlockEdge {
    /// Construct an edge (C++ `BlockEdge(FlowBlock*,uint4,int4)`,
    /// `block.hh:62`).
    pub fn new(pt: BlockId, lab: uint4, rev: int4) -> BlockEdge {
        BlockEdge { label: lab, point: Some(pt), reverse_index: rev }
    }

    /// Encode \b this edge to a stream (C++ `BlockEdge::encode`, `block.cc:35`).
    ///
    /// The label is intentionally not saved (C++ comment); only the other
    /// endpoint's index and the reverse-index position are written.
    pub fn encode(&self, encoder: &mut dyn Encoder, arena: &BlockArena) {
        encoder.open_element(&ELEM_EDGE);
        // We are not saving label currently
        let point = self.point.expect("BlockEdge::encode: unresolved point");
        encoder.write_signed_integer(&ATTRIB_END, arena[point].get_index() as i64);
        encoder.write_signed_integer(&ATTRIB_REV, self.reverse_index as i64);
        encoder.close_element(&ELEM_EDGE);
    }

    /// Restore \b this edge from a stream (C++ `BlockEdge::decode`,
    /// `block.cc:48`).
    pub fn decode(&mut self, decoder: &mut dyn Decoder, resolver: &BlockMap) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_EDGE)?;
        self.label = 0; // Tag does not currently contain info about label
        let end_index = decoder.read_signed_integer_id(&ATTRIB_END)? as int4;
        self.point = resolver.find_level_block(end_index);
        if self.point.is_none() {
            return Err(KunaError::lowlevel("Bad serialized edge in block graph"));
        }
        self.reverse_index = decoder.read_signed_integer_id(&ATTRIB_REV)? as int4;
        decoder.close_element(elem_id)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// BlockKind — per-subtype payload (the C++ derived-class fields)
// ---------------------------------------------------------------------------

/// \brief A class for annotating and sorting the individual cases of a switch
/// (C++ `BlockSwitch::CaseOrder`, `block.hh:769-779`).
#[derive(Debug, Clone)]
pub struct CaseOrder {
    /// The structured \e case block (C++ `FlowBlock *block`)
    pub block: BlockId,
    /// The first basic-block to execute within the \e case block
    /// (C++ `const FlowBlock *basicblock`)
    pub basicblock: Option<BlockId>,
    /// The \e label for this case, as an untyped constant (C++ `uintb label`)
    pub label: kuna_base::types::uintb,
    /// How deep in a fall-thru chain we are (C++ `int4 depth`)
    pub depth: int4,
    /// Who we immediately chain to (caseblocks index, -1 for none)
    /// (C++ `int4 chain`)
    pub chain: int4,
    /// Index coming out of switch to this case (C++ `int4 outindex`)
    pub outindex: int4,
    /// (If non-zero) What type of unstructured \e case is this? (C++ `uint4 gototype`)
    pub gototype: uint4,
    /// Does this case flow to the \e exit block (C++ `bool isexit`)
    pub isexit: bool,
    /// True if this is formal \e default case for the switch (C++ `bool isdefault`)
    pub isdefault: bool,
}

impl CaseOrder {
    /// Compare two cases (C++ `BlockSwitch::CaseOrder::compare`,
    /// `block.hh:921`).  Cases are compared by their label, then by depth.
    pub fn compare(a: &CaseOrder, b: &CaseOrder) -> std::cmp::Ordering {
        if a.label != b.label {
            return a.label.cmp(&b.label);
        }
        a.depth.cmp(&b.depth)
    }
}

/// The per-subtype payload that distinguishes a [`FlowBlock`] beyond its base
/// fields.  Each variant mirrors the fields the corresponding C++ class adds on
/// top of `FlowBlock`/`BlockGraph`.  The component `list` of a `BlockGraph`
/// lives in [`FlowBlock::list`] (shared by every graph variant), not here.
#[derive(Debug, Clone)]
pub enum BlockKind {
    /// `t_plain` — a bare FlowBlock (no components, no extra fields).
    Plain,
    /// `t_basic` — a [`BlockBasic`].  The op list lives as basic-block intrusive
    /// links on the ops (op.rs); here we keep the `Funcdata` owner is implicit,
    /// the original address `cover`, and the op-list head/tail (SEAM(W7): the
    /// `Funcdata *data` back-pointer is supplied by funcdata).
    Basic(BasicData),
    /// `t_graph` — a generic BlockGraph (components only).
    Graph,
    /// `t_copy` — mirrors a leaf block (C++ `BlockCopy::copy`).
    Copy { copy: Option<BlockId> },
    /// `t_goto` — unstructured branch (C++ `BlockGoto`).
    Goto { gototarget: Option<BlockId>, gototype: uint4 },
    /// `t_multigoto` — multiple-exit with some unstructured edges.
    MultiGoto { gotoedges: Vec<BlockId>, defaultswitch: bool },
    /// `t_ls` — a BlockList (components only).
    Ls,
    /// `t_condition` — two conditions glued by BOOL_AND/BOOL_OR.
    Condition { opc: OpCode },
    /// `t_if` — a formal "if" structure.
    If { gototype: uint4, gototarget: Option<BlockId> },
    /// `t_whiledo` — top-tested loop.  SEAM(W7): the `initializeOp`/`iterateOp`/
    /// `loopDef` (for-loop detection) are filled by the structuring wave.
    WhileDo {
        initialize_op: Option<crate::seams::OpId>,
        iterate_op: Option<crate::seams::OpId>,
        loop_def: Option<crate::seams::OpId>,
    },
    /// `t_dowhile` — bottom-tested loop (components only).
    DoWhile,
    /// `t_infloop` — infinite loop (components only).
    InfLoop,
    /// `t_switch` — structured switch.  `jt_index` is the `Funcdata::jumpvec`
    /// slot of the [`JumpTable`](crate::jumptable::JumpTable) the switch was
    /// recovered from (C++ `BlockSwitch::jump` is the pointer; the rust port
    /// keeps the owning function's slot index so the printer can reach the
    /// table's labels via `Funcdata::get_jump_table`).  The case structure is
    /// carried in `caseblocks`.
    Switch { caseblocks: Vec<CaseOrder>, jt_index: usize },
}

/// The BlockBasic-specific data (C++ `BlockBasic` non-edge members).
///
/// SEAM(W7): the op sequence (`list<PcodeOp*> op`) is realized as basic-block
/// intrusive links on the ops themselves (op.rs `ListKind::Basic`), driven
/// through `Funcdata`; this struct keeps the head/tail [`OpId`]s and the
/// original address `cover`.  The `Funcdata *data` back-pointer is the owning
/// function (supplied at construction by funcdata).
#[derive(Debug, Clone, Default)]
pub struct BasicData {
    /// First op in the block (C++ `op.front()`), `None` when empty
    pub op_head: Option<crate::seams::OpId>,
    /// Last op in the block (C++ `op.back()`), `None` when empty
    pub op_tail: Option<crate::seams::OpId>,
    /// Number of ops in the block (C++ `op.size()`)
    pub op_len: usize,
    /// Original range of addresses covered by this basic block (C++ `RangeList cover`)
    pub cover: kuna_base::address::RangeList,
}

// ---------------------------------------------------------------------------
// FlowBlock — the unified node
// ---------------------------------------------------------------------------

/// \brief Description of a control-flow block (C++ `FlowBlock` + `BlockGraph`
/// + the structured subtypes, unified per the dispatch note above).
///
/// Carries every C++ `FlowBlock` base field plus the `BlockGraph` component
/// `list` (empty for leaf blocks) and a [`BlockKind`] discriminating the
/// subtype payload.
#[derive(Debug, Clone)]
pub struct FlowBlock {
    /// Collection of [`block_flags`] (C++ `uint4 flags`)
    flags: uint4,
    /// The parent block to which \b this belongs (C++ `FlowBlock *parent`)
    parent: Option<BlockId>,
    /// Immediate dominating block (C++ `FlowBlock *immed_dom`)
    immed_dom: Option<BlockId>,
    /// Back reference to a BlockCopy of \b this (C++ `FlowBlock *copymap`)
    copymap: Option<BlockId>,
    /// Reference index (reverse post order) (C++ `int4 index`)
    index: int4,
    /// A count of visits of this node for various algorithms (C++ `int4 visitcount`)
    visitcount: int4,
    /// Number of descendants in spanning tree (+1) (C++ `int4 numdesc`)
    numdesc: int4,
    /// Blocks which (can) fall into this block (C++ `vector<BlockEdge> intothis`)
    intothis: Vec<BlockEdge>,
    /// Blocks into which this block (can) fall (C++ `vector<BlockEdge> outofthis`)
    outofthis: Vec<BlockEdge>,
    /// Component blocks of a BlockGraph (C++ `BlockGraph::list`); empty for leaves
    list: Vec<BlockId>,
    /// The subtype payload (discriminates the C++ derived class)
    kind: BlockKind,
}

impl FlowBlock {
    /// Construct a block with no edges (C++ `FlowBlock::FlowBlock`,
    /// `block.cc:61`).
    pub fn new() -> FlowBlock {
        FlowBlock {
            flags: 0,
            parent: None,
            immed_dom: None,
            copymap: None,
            index: 0,
            visitcount: 0,
            numdesc: 0,
            intothis: Vec::new(),
            outofthis: Vec::new(),
            list: Vec::new(),
            kind: BlockKind::Plain,
        }
    }

    /// Construct a block with the given subtype payload.
    pub fn new_kind(kind: BlockKind) -> FlowBlock {
        FlowBlock { kind, ..FlowBlock::new() }
    }

    // --- flag accessors (C++ inline) ---------------------------------------

    /// Set a boolean property (C++ `setFlag`).
    pub fn set_flag(&mut self, fl: uint4) {
        self.flags |= fl;
    }
    /// Clear a boolean property (C++ `clearFlag`).
    pub fn clear_flag(&mut self, fl: uint4) {
        self.flags &= !fl;
    }
    /// Clear all properties (C++ `clearAllFlags`).
    pub fn clear_all_flags(&mut self) {
        self.flags = 0;
    }
    /// Get the block_flags properties (C++ `getFlags`).
    pub fn get_flags(&self) -> uint4 {
        self.flags
    }

    /// Get the index assigned to \b this block (C++ `getIndex`).
    pub fn get_index(&self) -> int4 {
        self.index
    }
    /// Set the reference index (used during decode and tree construction).
    pub fn set_index(&mut self, i: int4) {
        self.index = i;
    }
    /// Get the parent FlowBlock of \b this (C++ `getParent`).
    pub fn get_parent(&self) -> Option<BlockId> {
        self.parent
    }
    /// Get the immediate dominator FlowBlock (C++ `getImmedDom`).
    pub fn get_immed_dom(&self) -> Option<BlockId> {
        self.immed_dom
    }
    /// Get the mapped FlowBlock (C++ `getCopyMap`).
    pub fn get_copy_map(&self) -> Option<BlockId> {
        self.copymap
    }
    /// The block a `BlockCopy` mirrors (C++ `BlockCopy::copy`, the underlying
    /// FlowBlock — a *bblocks* [`BlockId`] when produced by the cross-arena
    /// `build_copy_from`).  `None` for non-Copy nodes.
    pub fn get_copy(&self) -> Option<BlockId> {
        match &self.kind {
            BlockKind::Copy { copy } => *copy,
            _ => None,
        }
    }
    /// The unstructured-branch target of a `BlockIf` (C++ `BlockIf::getGotoTarget`),
    /// or `None` (also for non-If nodes).  When set, the if has only the
    /// condition component and emits a `goto` instead of a braced body.
    pub fn get_if_goto_target(&self) -> Option<BlockId> {
        match &self.kind {
            BlockKind::If { gototarget, .. } => *gototarget,
            _ => None,
        }
    }
    /// The unstructured-branch type of a `BlockIf` (C++ `BlockIf::getGotoType`).
    pub fn get_if_goto_type(&self) -> uint4 {
        match &self.kind {
            BlockKind::If { gototype, .. } => *gototype,
            _ => 0,
        }
    }
    /// The iterate op of a `BlockWhileDo` (C++ `BlockWhileDo::getIterateOp`), or
    /// `None`.  When set, the while-do is emitted as a `for` loop.
    pub fn get_iterate_op(&self) -> Option<crate::seams::OpId> {
        match &self.kind {
            BlockKind::WhileDo { iterate_op, .. } => *iterate_op,
            _ => None,
        }
    }
    /// The initializer op of a `BlockWhileDo` (C++ `BlockWhileDo::getInitializeOp`).
    pub fn get_initialize_op(&self) -> Option<crate::seams::OpId> {
        match &self.kind {
            BlockKind::WhileDo { initialize_op, .. } => *initialize_op,
            _ => None,
        }
    }
    /// The loop-def MULTIEQUAL of a `BlockWhileDo` (C++ `loopDef`).
    pub fn get_loop_def(&self) -> Option<crate::seams::OpId> {
        match &self.kind {
            BlockKind::WhileDo { loop_def, .. } => *loop_def,
            _ => None,
        }
    }
    /// Set the iterate op of a `BlockWhileDo` (C++ `iterateOp = ...`).
    pub fn set_iterate_op(&mut self, op: Option<crate::seams::OpId>) {
        if let BlockKind::WhileDo { iterate_op, .. } = &mut self.kind {
            *iterate_op = op;
        }
    }
    /// Set the initializer op of a `BlockWhileDo` (C++ `initializeOp = ...`).
    pub fn set_initialize_op(&mut self, op: Option<crate::seams::OpId>) {
        if let BlockKind::WhileDo { initialize_op, .. } = &mut self.kind {
            *initialize_op = op;
        }
    }
    /// Set the loop-def MULTIEQUAL of a `BlockWhileDo` (C++ `loopDef = ...`).
    pub fn set_loop_def(&mut self, op: Option<crate::seams::OpId>) {
        if let BlockKind::WhileDo { loop_def, .. } = &mut self.kind {
            *loop_def = op;
        }
    }
    /// The unstructured-branch target of a `BlockGoto` (C++
    /// `BlockGoto::getGotoTarget`), or `None` (also for non-Goto nodes).
    pub fn get_goto_target(&self) -> Option<BlockId> {
        match &self.kind {
            BlockKind::Goto { gototarget, .. } => *gototarget,
            _ => None,
        }
    }
    /// The unstructured-branch type of a `BlockGoto` (C++
    /// `BlockGoto::getGotoType`).
    pub fn get_goto_type(&self) -> uint4 {
        match &self.kind {
            BlockKind::Goto { gototype, .. } => *gototype,
            _ => 0,
        }
    }
    /// Set the number of times this block has been visited (C++ `setVisitCount`).
    pub fn set_visit_count(&mut self, i: int4) {
        self.visitcount = i;
    }
    /// Get the count of visits (C++ `getVisitCount`).
    pub fn get_visit_count(&self) -> int4 {
        self.visitcount
    }

    /// Get the subtype payload.
    pub fn kind(&self) -> &BlockKind {
        &self.kind
    }
    /// Get a mutable handle on the subtype payload.
    pub fn kind_mut(&mut self) -> &mut BlockKind {
        &mut self.kind
    }

    /// Flip a `BlockCondition`'s AND/OR opcode (C++ `BlockCondition::negateCondition`
    /// — `opc = (opc==CPUI_BOOL_AND) ? CPUI_BOOL_OR : CPUI_BOOL_AND`).  No-op on a
    /// non-Condition node.
    pub fn flip_condition_opcode(&mut self) {
        if let BlockKind::Condition { opc } = &mut self.kind {
            *opc = if *opc == OpCode::CPUI_BOOL_AND {
                OpCode::CPUI_BOOL_OR
            } else {
                OpCode::CPUI_BOOL_AND
            };
        }
    }

    /// Get a `BlockCondition`'s gluing opcode (C++ `BlockCondition::getOpcode`,
    /// `block.hh`).  `None` on a non-Condition node.
    pub fn get_condition_opcode(&self) -> Option<OpCode> {
        if let BlockKind::Condition { opc } = &self.kind {
            Some(*opc)
        } else {
            None
        }
    }

    /// Get the FlowBlock type of \b this (C++ `getType`, dispatched per
    /// derived class).
    pub fn get_type(&self) -> BlockType {
        match &self.kind {
            BlockKind::Plain => BlockType::Plain,
            BlockKind::Basic(_) => BlockType::Basic,
            BlockKind::Graph => BlockType::Graph,
            BlockKind::Copy { .. } => BlockType::Copy,
            BlockKind::Goto { .. } => BlockType::Goto,
            BlockKind::MultiGoto { .. } => BlockType::MultiGoto,
            BlockKind::Ls => BlockType::Ls,
            BlockKind::Condition { .. } => BlockType::Condition,
            BlockKind::If { .. } => BlockType::If,
            BlockKind::WhileDo { .. } => BlockType::WhileDo,
            BlockKind::DoWhile => BlockType::DoWhile,
            BlockKind::InfLoop => BlockType::InfLoop,
            BlockKind::Switch { .. } => BlockType::Switch,
        }
    }

    // --- edge query accessors (C++ inline, block.hh) -----------------------

    /// Get the number of out edges (C++ `sizeOut`).
    pub fn size_out(&self) -> int4 {
        self.outofthis.len() as int4
    }
    /// Get the number of in edges (C++ `sizeIn`).
    pub fn size_in(&self) -> int4 {
        self.intothis.len() as int4
    }
    /// Get the i-th output FlowBlock (C++ `getOut`).
    pub fn get_out(&self, i: int4) -> BlockId {
        self.outofthis[i as usize].point.expect("getOut: unresolved edge")
    }
    /// Get the input index of the i-th output FlowBlock (C++ `getOutRevIndex`).
    pub fn get_out_rev_index(&self, i: int4) -> int4 {
        self.outofthis[i as usize].reverse_index
    }
    /// Get the i-th input FlowBlock (C++ `getIn`).
    pub fn get_in(&self, i: int4) -> BlockId {
        self.intothis[i as usize].point.expect("getIn: unresolved edge")
    }
    /// Get the output index of the i-th input FlowBlock (C++ `getInRevIndex`).
    pub fn get_in_rev_index(&self, i: int4) -> int4 {
        self.intothis[i as usize].reverse_index
    }
    /// Get the \b false output FlowBlock (C++ `getFalseOut`).
    pub fn get_false_out(&self) -> BlockId {
        self.get_out(0)
    }
    /// Get the \b true output FlowBlock (C++ `getTrueOut`).
    pub fn get_true_out(&self) -> BlockId {
        self.get_out(1)
    }

    /// Search incoming edges for the given block (C++ `getInIndex`,
    /// `block.cc:579`).  Returns -1 if not found.
    pub fn get_in_index(&self, bl: BlockId) -> int4 {
        for blocknum in 0..self.intothis.len() {
            if self.intothis[blocknum].point == Some(bl) {
                return blocknum as int4;
            }
        }
        -1
    }
    /// Search outgoing edges for the given block (C++ `getOutIndex`,
    /// `block.cc:592`).  Returns -1 if not found.
    pub fn get_out_index(&self, bl: BlockId) -> int4 {
        for blocknum in 0..self.outofthis.len() {
            if self.outofthis[blocknum].point == Some(bl) {
                return blocknum as int4;
            }
        }
        -1
    }

    // --- edge-label predicates (C++ inline, block.hh:303-354) --------------

    /// Has an immediate COPY propagated across the i-th out edge
    /// (C++ `hasImmedCopyEdge`).
    pub fn has_immed_copy_edge(&self, i: int4) -> bool {
        (self.outofthis[i as usize].label & edge_flags::f_immed_copy) != 0
    }
    /// Have out edges been flipped (C++ `getFlipPath`).
    pub fn get_flip_path(&self) -> bool {
        (self.flags & block_flags::f_flip_path) != 0
    }
    /// Is there a looping edge coming into \b this block (C++ `hasLoopIn`,
    /// `block.cc:428`).
    pub fn has_loop_in(&self) -> bool {
        for i in 0..self.intothis.len() {
            if (self.intothis[i].label & edge_flags::f_loop_edge) != 0 {
                return true;
            }
        }
        false
    }
    /// Is there a looping edge going out of \b this block (C++ `hasLoopOut`,
    /// `block.cc:437`).
    pub fn has_loop_out(&self) -> bool {
        for i in 0..self.outofthis.len() {
            if (self.outofthis[i].label & edge_flags::f_loop_edge) != 0 {
                return true;
            }
        }
        false
    }
    /// Is the i-th incoming edge a \e loop edge (C++ `isLoopIn`).
    pub fn is_loop_in(&self, i: int4) -> bool {
        (self.intothis[i as usize].label & edge_flags::f_loop_edge) != 0
    }
    /// Is the i-th outgoing edge a \e loop edge (C++ `isLoopOut`).
    pub fn is_loop_out(&self, i: int4) -> bool {
        (self.outofthis[i as usize].label & edge_flags::f_loop_edge) != 0
    }
    /// Is the i-th out edge the switch default edge (C++ `isDefaultBranch`).
    pub fn is_default_branch(&self, i: int4) -> bool {
        (self.outofthis[i as usize].label & edge_flags::f_defaultswitch_edge) != 0
    }
    /// Can \b this and the i-th output be merged into a BlockIf/BlockList
    /// (C++ `isDecisionOut`).
    pub fn is_decision_out(&self, i: int4) -> bool {
        (self.outofthis[i as usize].label
            & (edge_flags::f_irreducible | edge_flags::f_back_edge | edge_flags::f_goto_edge))
            == 0
    }
    /// Can \b this and the i-th input be merged into a BlockIf/BlockList
    /// (C++ `isDecisionIn`).
    pub fn is_decision_in(&self, i: int4) -> bool {
        (self.intothis[i as usize].label
            & (edge_flags::f_irreducible | edge_flags::f_back_edge | edge_flags::f_goto_edge))
            == 0
    }
    /// Is the i-th outgoing edge part of the DAG sub-graph (C++ `isLoopDAGOut`).
    pub fn is_loop_dag_out(&self, i: int4) -> bool {
        (self.outofthis[i as usize].label
            & (edge_flags::f_irreducible
                | edge_flags::f_back_edge
                | edge_flags::f_loop_exit_edge
                | edge_flags::f_goto_edge))
            == 0
    }
    /// Is the i-th incoming edge part of the DAG sub-graph (C++ `isLoopDAGIn`).
    pub fn is_loop_dag_in(&self, i: int4) -> bool {
        (self.intothis[i as usize].label
            & (edge_flags::f_irreducible
                | edge_flags::f_back_edge
                | edge_flags::f_loop_exit_edge
                | edge_flags::f_goto_edge))
            == 0
    }
    /// Is the i-th incoming edge unstructured (C++ `isGotoIn`).
    pub fn is_goto_in(&self, i: int4) -> bool {
        (self.intothis[i as usize].label & (edge_flags::f_irreducible | edge_flags::f_goto_edge))
            != 0
    }
    /// Is the i-th outgoing edge unstructured (C++ `isGotoOut`).
    pub fn is_goto_out(&self, i: int4) -> bool {
        (self.outofthis[i as usize].label & (edge_flags::f_irreducible | edge_flags::f_goto_edge))
            != 0
    }
    /// Is the i-th incoming edge part of the spanning tree (C++ `isTreeEdgeIn`).
    pub fn is_tree_edge_in(&self, i: int4) -> bool {
        (self.intothis[i as usize].label & edge_flags::f_tree_edge) != 0
    }
    /// Is the i-th incoming edge a \e back edge (C++ `isBackEdgeIn`).
    pub fn is_back_edge_in(&self, i: int4) -> bool {
        (self.intothis[i as usize].label & edge_flags::f_back_edge) != 0
    }
    /// Is the i-th outgoing edge a \e back edge (C++ `isBackEdgeOut`).
    pub fn is_back_edge_out(&self, i: int4) -> bool {
        (self.outofthis[i as usize].label & edge_flags::f_back_edge) != 0
    }
    /// Is the i-th outgoing edge an irreducible edge (C++ `isIrreducibleOut`).
    pub fn is_irreducible_out(&self, i: int4) -> bool {
        (self.outofthis[i as usize].label & edge_flags::f_irreducible) != 0
    }
    /// Is the i-th incoming edge an irreducible edge (C++ `isIrreducibleIn`).
    pub fn is_irreducible_in(&self, i: int4) -> bool {
        (self.intothis[i as usize].label & edge_flags::f_irreducible) != 0
    }

    // --- block flag predicates (C++ inline, block.hh:291-340) --------------

    /// Return \b true if \b this block has been marked (C++ `isMark`).
    pub fn is_mark(&self) -> bool {
        (self.flags & block_flags::f_mark) != 0
    }
    /// Mark \b this block (C++ `setMark`).
    pub fn set_mark(&mut self) {
        self.flags |= block_flags::f_mark;
    }
    /// Clear any mark on \b this block (C++ `clearMark`).
    pub fn clear_mark(&mut self) {
        self.flags &= !block_flags::f_mark;
    }
    /// Label \b this as a \e do \e nothing loop (C++ `setDonothingLoop`).
    pub fn set_donothing_loop(&mut self) {
        self.flags |= block_flags::f_donothing_loop;
    }
    /// Label \b this as dead (C++ `setDead`).
    pub fn set_dead(&mut self) {
        self.flags |= block_flags::f_dead;
    }
    /// Return \b true if \b this uses a different label (C++ `hasSpecialLabel`).
    pub fn has_special_label(&self) -> bool {
        (self.flags & (block_flags::f_joined_block | block_flags::f_duplicate_block)) != 0
    }
    /// Return \b true if \b this is a \e joined basic block (C++ `isJoined`).
    pub fn is_joined(&self) -> bool {
        (self.flags & block_flags::f_joined_block) != 0
    }
    /// Return \b true if \b this is a \e duplicated block (C++ `isDuplicated`).
    pub fn is_duplicated(&self) -> bool {
        (self.flags & block_flags::f_duplicate_block) != 0
    }
    /// Are labels for \b this printed by the parent (C++ `isLabelBumpUp`).
    pub fn is_label_bump_up(&self) -> bool {
        (self.flags & block_flags::f_label_bumpup) != 0
    }
    /// Is \b this the target of an unstructured goto (C++ `isUnstructuredTarget`).
    pub fn is_unstructured_target(&self) -> bool {
        (self.flags & block_flags::f_unstructured_targ) != 0
    }
    /// Is there an unstructured goto to \b this block's interior
    /// (C++ `isInteriorGotoTarget`).
    pub fn is_interior_goto_target(&self) -> bool {
        (self.flags & block_flags::f_interior_gotoin) != 0
    }
    /// Is there an unstructured goto out of \b this block's interior
    /// (C++ `hasInteriorGoto`).
    pub fn has_interior_goto(&self) -> bool {
        (self.flags & block_flags::f_interior_gotoout) != 0
    }
    /// Is the entry point of the function (C++ `isEntryPoint`).
    pub fn is_entry_point(&self) -> bool {
        (self.flags & block_flags::f_entry_point) != 0
    }
    /// Is \b this a switch block (C++ `isSwitchOut`).
    pub fn is_switch_out(&self) -> bool {
        (self.flags & block_flags::f_switch_out) != 0
    }
    /// Is \b this a \e do \e nothing block (C++ `isDonothingLoop`).
    pub fn is_donothing_loop(&self) -> bool {
        (self.flags & block_flags::f_donothing_loop) != 0
    }
    /// Is \b this block dead (C++ `isDead`).
    pub fn is_dead(&self) -> bool {
        (self.flags & block_flags::f_dead) != 0
    }
    /// Is \b this block marked for delayed do-nothing removal
    /// (C++ `isDelayedDonothing`).
    pub fn is_delayed_donothing(&self) -> bool {
        (self.flags & block_flags::f_delayed_donothing) != 0
    }
    /// Mark as \e do \e nothing block with delayed removal (C++ `setDelayedDonothing`).
    pub fn set_delayed_donothing(&mut self) {
        self.flags |= block_flags::f_delayed_donothing;
    }
    /// Clear mark for delayed removal (C++ `clearDelayedDonothing`).
    pub fn clear_delayed_donothing(&mut self) {
        self.flags &= !block_flags::f_delayed_donothing;
    }

    // --- BlockGraph component accessors (C++ block.hh:389-391) -------------

    /// Get the list of component FlowBlock objects (C++ `getList`).
    pub fn get_list(&self) -> &[BlockId] {
        &self.list
    }
    /// Get the number of components (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.list.len() as int4
    }
    /// Get the i-th component (C++ `getBlock`).
    pub fn get_block(&self, i: int4) -> BlockId {
        self.list[i as usize]
    }

    /// Borrow the switch case descriptions (C++ `BlockSwitch::caseblocks`); empty
    /// for non-switch blocks.
    pub fn switch_caseblocks(&self) -> &[CaseOrder] {
        match &self.kind {
            BlockKind::Switch { caseblocks, .. } => caseblocks,
            _ => &[],
        }
    }

    /// Mutable view of the switch case descriptions (C++ `BlockSwitch::caseblocks`).
    pub fn switch_caseblocks_mut(&mut self) -> Option<&mut Vec<CaseOrder>> {
        match &mut self.kind {
            BlockKind::Switch { caseblocks, .. } => Some(caseblocks),
            _ => None,
        }
    }

    /// The `Funcdata::jumpvec` slot of this switch's [`JumpTable`](crate::jumptable::JumpTable)
    /// (C++ `BlockSwitch::jump`); `None` for non-switch blocks.
    pub fn switch_jt_index(&self) -> Option<usize> {
        match &self.kind {
            BlockKind::Switch { jt_index, .. } => Some(*jt_index),
            _ => None,
        }
    }

    /// Clone the switch case descriptions for mutation outside the arena borrow
    /// (the label-finalize pass needs `&Funcdata` for JumpTable queries while it
    /// updates the cases).
    pub fn switch_caseblocks_mut_snapshot(&self) -> Option<Vec<CaseOrder>> {
        match &self.kind {
            BlockKind::Switch { caseblocks, .. } => Some(caseblocks.clone()),
            _ => None,
        }
    }

    /// Return \b true if finalTransform() has been run (C++ `hasFinalTransform`).
    pub fn has_final_transform(&self) -> bool {
        (self.flags & block_flags::f_final_transform) != 0
    }

    /// Does \b this require overflow syntax (BlockWhileDo) (C++ `hasOverflowSyntax`).
    pub fn has_overflow_syntax(&self) -> bool {
        (self.flags & block_flags::f_whiledo_overflow) != 0
    }
    /// Set that \b this requires overflow syntax (C++ `setOverflowSyntax`).
    pub fn set_overflow_syntax(&mut self) {
        self.flags |= block_flags::f_whiledo_overflow;
    }
}

impl Default for FlowBlock {
    fn default() -> Self {
        FlowBlock::new()
    }
}

// ---------------------------------------------------------------------------
// BlockArena — owns all FlowBlocks (ADR 0001)
// ---------------------------------------------------------------------------

/// The arena owning all [`FlowBlock`]s of a function (ADR 0001).  The C++
/// pointer/ownership web (`BlockGraph::list` owning, `delete` in `clear`)
/// becomes generational [`BlockId`] handles here.  Edge primitives that touch
/// two blocks at once are methods on the arena so both endpoints can be
/// mutated, exactly as the C++ `a->addInEdge(b)` mutates `a` and `b`.
pub type BlockArena = SlotMap<BlockId, FlowBlock>;

/// A graph of [`FlowBlock`]s plus the arena that owns them.  The C++
/// `BlockGraph` both *is-a* `FlowBlock` and *owns* its components; here the
/// owning arena is shared across the whole function (ADR 0001), so a
/// [`BlockGraph`] pairs the arena with the root [`BlockId`] of the graph node.
///
/// All edge-manipulation and the dominator/spanning-tree algorithms are methods
/// on this struct so they can read/write across the arena.
#[derive(Default)]
pub struct BlockGraph {
    /// The arena of all blocks (ADR 0001)
    pub arena: BlockArena,
    /// The root BlockGraph node id (C++ `this` of the top BlockGraph), if built
    pub root: Option<BlockId>,
}

impl BlockGraph {
    /// Construct an empty graph.
    pub fn new() -> BlockGraph {
        BlockGraph { arena: BlockArena::with_key(), root: None }
    }

    /// Borrow a block (panics on stale handle, per ADR 0001).
    pub fn block(&self, id: BlockId) -> &FlowBlock {
        &self.arena[id]
    }
    /// Mutably borrow a block.
    pub fn block_mut(&mut self, id: BlockId) -> &mut FlowBlock {
        &mut self.arena[id]
    }

    // ----------------------------------------------------------------------
    // Half-edge primitives (FlowBlock private, block.cc)
    // ----------------------------------------------------------------------

    /// Add an edge coming into `this_id` from `b` (C++ `FlowBlock::addInEdge`,
    /// `block.cc:73`).  Mutates both `this_id.intothis` and `b.outofthis`.
    fn add_in_edge(&mut self, this_id: BlockId, b: BlockId, lab: uint4) {
        let ourrev = self.arena[b].outofthis.len() as int4;
        let brev = self.arena[this_id].intothis.len() as int4;
        self.arena[this_id].intothis.push(BlockEdge::new(b, lab, ourrev));
        self.arena[b].outofthis.push(BlockEdge::new(this_id, lab, brev));
    }

    /// Delete the \e in half of an edge, correcting indices
    /// (C++ `FlowBlock::halfDeleteInEdge`, `block.cc:100`).
    fn half_delete_in_edge(&mut self, this_id: BlockId, mut slot: int4) {
        while (slot as usize) < self.arena[this_id].intothis.len() - 1 {
            // edge = intothis[slot+1]; slide over
            let edge = self.arena[this_id].intothis[slot as usize + 1].clone();
            let point = edge.point.expect("halfDeleteInEdge: unresolved");
            let rev = edge.reverse_index;
            self.arena[this_id].intothis[slot as usize] = edge;
            // Correct the index coming the other way
            self.arena[point].outofthis[rev as usize].reverse_index -= 1;
            slot += 1;
        }
        self.arena[this_id].intothis.pop();
    }

    /// Delete the \e out half of an edge, correcting indices
    /// (C++ `FlowBlock::halfDeleteOutEdge`, `block.cc:115`).
    fn half_delete_out_edge(&mut self, this_id: BlockId, mut slot: int4) {
        while (slot as usize) < self.arena[this_id].outofthis.len() - 1 {
            let edge = self.arena[this_id].outofthis[slot as usize + 1].clone();
            let point = edge.point.expect("halfDeleteOutEdge: unresolved");
            let rev = edge.reverse_index;
            self.arena[this_id].outofthis[slot as usize] = edge;
            self.arena[point].intothis[rev as usize].reverse_index -= 1;
            slot += 1;
        }
        self.arena[this_id].outofthis.pop();
    }

    /// Remove an incoming edge (C++ `FlowBlock::removeInEdge`, `block.cc:130`).
    fn remove_in_edge(&mut self, this_id: BlockId, slot: int4) {
        let b = self.arena[this_id].intothis[slot as usize].point.expect("removeInEdge");
        let rev = self.arena[this_id].intothis[slot as usize].reverse_index;
        self.half_delete_in_edge(this_id, slot);
        self.half_delete_out_edge(b, rev);
    }

    /// Remove an outgoing edge (C++ `FlowBlock::removeOutEdge`, `block.cc:144`).
    fn remove_out_edge(&mut self, this_id: BlockId, slot: int4) {
        let b = self.arena[this_id].outofthis[slot as usize].point.expect("removeOutEdge");
        let rev = self.arena[this_id].outofthis[slot as usize].reverse_index;
        self.half_delete_out_edge(this_id, slot);
        self.half_delete_in_edge(b, rev);
    }

    /// Make an incoming edge flow from a given block
    /// (C++ `FlowBlock::replaceInEdge`, `block.cc:160`).
    fn replace_in_edge(&mut self, this_id: BlockId, num: int4, b: BlockId) {
        let oldb = self.arena[this_id].intothis[num as usize].point.expect("replaceInEdge");
        let old_rev = self.arena[this_id].intothis[num as usize].reverse_index;
        self.half_delete_out_edge(oldb, old_rev);
        let label = self.arena[this_id].intothis[num as usize].label;
        let new_rev = self.arena[b].outofthis.len() as int4;
        self.arena[this_id].intothis[num as usize].point = Some(b);
        self.arena[this_id].intothis[num as usize].reverse_index = new_rev;
        self.arena[b].outofthis.push(BlockEdge::new(this_id, label, num));
    }

    /// Make an outgoing edge flow to a given block
    /// (C++ `FlowBlock::replaceOutEdge`, `block.cc:178`).
    fn replace_out_edge(&mut self, this_id: BlockId, num: int4, b: BlockId) {
        let oldb = self.arena[this_id].outofthis[num as usize].point.expect("replaceOutEdge");
        let old_rev = self.arena[this_id].outofthis[num as usize].reverse_index;
        self.half_delete_in_edge(oldb, old_rev);
        let label = self.arena[this_id].outofthis[num as usize].label;
        let new_rev = self.arena[b].intothis.len() as int4;
        self.arena[this_id].outofthis[num as usize].point = Some(b);
        self.arena[this_id].outofthis[num as usize].reverse_index = new_rev;
        self.arena[b].intothis.push(BlockEdge::new(this_id, label, num));
    }

    /// Remove \b this from flow between two blocks
    /// (C++ `FlowBlock::replaceEdgesThru`, `block.cc:198`).
    fn replace_edges_thru(&mut self, this_id: BlockId, in_slot: int4, out_slot: int4) {
        let inb = self.arena[this_id].intothis[in_slot as usize].point.expect("replaceEdgesThru");
        let inblock_outslot = self.arena[this_id].intothis[in_slot as usize].reverse_index;
        let outb = self.arena[this_id].outofthis[out_slot as usize].point.expect("replaceEdgesThru");
        let outblock_inslot = self.arena[this_id].outofthis[out_slot as usize].reverse_index;
        self.arena[inb].outofthis[inblock_outslot as usize].point = Some(outb);
        self.arena[inb].outofthis[inblock_outslot as usize].reverse_index = outblock_inslot;
        self.arena[outb].intothis[outblock_inslot as usize].point = Some(inb);
        self.arena[outb].intothis[outblock_inslot as usize].reverse_index = inblock_outslot;
        self.half_delete_in_edge(this_id, in_slot);
        self.half_delete_out_edge(this_id, out_slot);
    }

    /// Swap the first and second \e out edges (C++ `FlowBlock::swapEdges`,
    /// `block.cc:218`).
    pub fn swap_edges(&mut self, this_id: BlockId) {
        self.arena[this_id].outofthis.swap(0, 1);
        let bl0 = self.arena[this_id].outofthis[0].point.expect("swapEdges");
        let rev0 = self.arena[this_id].outofthis[0].reverse_index;
        self.arena[bl0].intothis[rev0 as usize].reverse_index = 0;
        let bl1 = self.arena[this_id].outofthis[1].point.expect("swapEdges");
        let rev1 = self.arena[this_id].outofthis[1].reverse_index;
        self.arena[bl1].intothis[rev1 as usize].reverse_index = 1;
        self.arena[this_id].flags ^= block_flags::f_flip_path;
    }

    /// Apply an \e out edge label (C++ `FlowBlock::setOutEdgeFlag`,
    /// `block.cc:240`).
    pub fn set_out_edge_flag(&mut self, this_id: BlockId, i: int4, lab: uint4) {
        let bbout = self.arena[this_id].outofthis[i as usize].point.expect("setOutEdgeFlag");
        let rev = self.arena[this_id].outofthis[i as usize].reverse_index;
        self.arena[this_id].outofthis[i as usize].label |= lab;
        self.arena[bbout].intothis[rev as usize].label |= lab;
    }

    /// Remove an \e out edge label (C++ `FlowBlock::clearOutEdgeFlag`,
    /// `block.cc:250`).
    pub fn clear_out_edge_flag(&mut self, this_id: BlockId, i: int4, lab: uint4) {
        let bbout = self.arena[this_id].outofthis[i as usize].point.expect("clearOutEdgeFlag");
        let rev = self.arena[this_id].outofthis[i as usize].reverse_index;
        self.arena[this_id].outofthis[i as usize].label &= !lab;
        self.arena[bbout].intothis[rev as usize].label &= !lab;
    }

    /// Label the edge exiting \b this as a loop (C++ `setLoopExit`).
    pub fn set_loop_exit(&mut self, this_id: BlockId, i: int4) {
        self.set_out_edge_flag(this_id, i, edge_flags::f_loop_exit_edge);
    }
    /// Clear the loop exit edge (C++ `clearLoopExit`).
    pub fn clear_loop_exit(&mut self, this_id: BlockId, i: int4) {
        self.clear_out_edge_flag(this_id, i, edge_flags::f_loop_exit_edge);
    }
    /// Label the \e back edge of a loop (C++ `setBackEdge`).
    pub fn set_back_edge(&mut self, this_id: BlockId, i: int4) {
        self.set_out_edge_flag(this_id, i, edge_flags::f_back_edge);
    }
    /// Mark that an immediate COPY has propagated across the edge
    /// (C++ `setImmedCopyEdge`).
    pub fn set_immed_copy_edge(&mut self, this_id: BlockId, i: int4) {
        self.set_out_edge_flag(this_id, i, edge_flags::f_immed_copy);
    }

    /// Mark a \e goto branch (C++ `FlowBlock::setGotoBranch`, `block.cc:305`).
    pub fn set_goto_branch(&mut self, this_id: BlockId, i: int4) -> KunaResult<()> {
        if i >= 0 && i < self.arena[this_id].outofthis.len() as int4 {
            self.set_out_edge_flag(this_id, i, edge_flags::f_goto_edge);
        } else {
            return Err(KunaError::lowlevel("Could not find block edge to mark unstructured"));
        }
        self.arena[this_id].flags |= block_flags::f_interior_gotoout;
        let target = self.arena[this_id].outofthis[i as usize].point.expect("setGotoBranch");
        self.arena[target].flags |= block_flags::f_interior_gotoin;
        Ok(())
    }

    /// Mark an edge as the switch default (C++ `FlowBlock::setDefaultSwitch`,
    /// `block.cc:318`).
    pub fn set_default_switch(&mut self, this_id: BlockId, pos: int4) {
        for i in 0..self.arena[this_id].outofthis.len() as int4 {
            if self.arena[this_id].is_default_branch(i) {
                self.clear_out_edge_flag(this_id, i, edge_flags::f_defaultswitch_edge);
            }
        }
        self.set_out_edge_flag(this_id, pos, edge_flags::f_defaultswitch_edge);
    }

    /// Return \b true if non-fallthru jump flows into \b this
    /// (C++ `FlowBlock::isJumpTarget`, `block.cc:329`).
    pub fn is_jump_target(&self, this_id: BlockId) -> bool {
        let index = self.arena[this_id].index;
        for i in 0..self.arena[this_id].intothis.len() {
            let p = self.arena[this_id].intothis[i].point.expect("isJumpTarget");
            if self.arena[p].index != index - 1 {
                return true;
            }
        }
        false
    }

    // ----------------------------------------------------------------------
    // copy-map remapping (C++ FlowBlock::replaceUsingMap / replaceEdgeMap)
    // ----------------------------------------------------------------------

    /// Update block references in edges with the copy map
    /// (C++ `FlowBlock::replaceEdgeMap` over both lists +
    /// `replaceUsingMap`, `block.cc:268-286`).
    fn replace_using_map(&mut self, this_id: BlockId) {
        let n_in = self.arena[this_id].intothis.len();
        for i in 0..n_in {
            let p = self.arena[this_id].intothis[i].point.expect("replaceEdgeMap in");
            let mapped = self.arena[p].copymap;
            self.arena[this_id].intothis[i].point = mapped;
        }
        let n_out = self.arena[this_id].outofthis.len();
        for i in 0..n_out {
            let p = self.arena[this_id].outofthis[i].point.expect("replaceEdgeMap out");
            let mapped = self.arena[p].copymap;
            self.arena[this_id].outofthis[i].point = mapped;
        }
        if let Some(idom) = self.arena[this_id].immed_dom {
            self.arena[this_id].immed_dom = self.arena[idom].copymap;
        }
    }

    // ----------------------------------------------------------------------
    // dedup (C++ FlowBlock::dedup / findDups / eliminate{In,Out}Dups)
    // ----------------------------------------------------------------------

    /// Eliminate duplicate \e in edges from a given block
    /// (C++ `FlowBlock::eliminateInDups`, `block.cc:446`).
    fn eliminate_in_dups(&mut self, this_id: BlockId, bl: BlockId) {
        let mut indval: int4 = -1;
        let mut i: int4 = 0;
        while (i as usize) < self.arena[this_id].intothis.len() {
            if self.arena[this_id].intothis[i as usize].point == Some(bl) {
                if indval == -1 {
                    indval = i;
                    i += 1;
                } else {
                    let lab = self.arena[this_id].intothis[i as usize].label;
                    self.arena[this_id].intothis[indval as usize].label |= lab;
                    let rev = self.arena[this_id].intothis[i as usize].reverse_index;
                    self.half_delete_in_edge(this_id, i);
                    self.half_delete_out_edge(bl, rev);
                }
            } else {
                i += 1;
            }
        }
    }

    /// Eliminate duplicate \e out edges to a given block
    /// (C++ `FlowBlock::eliminateOutDups`, `block.cc:475`).
    fn eliminate_out_dups(&mut self, this_id: BlockId, bl: BlockId) {
        let mut indval: int4 = -1;
        let mut i: int4 = 0;
        while (i as usize) < self.arena[this_id].outofthis.len() {
            if self.arena[this_id].outofthis[i as usize].point == Some(bl) {
                if indval == -1 {
                    indval = i;
                    i += 1;
                } else {
                    let lab = self.arena[this_id].outofthis[i as usize].label;
                    self.arena[this_id].outofthis[indval as usize].label |= lab;
                    let rev = self.arena[this_id].outofthis[i as usize].reverse_index;
                    self.half_delete_out_edge(this_id, i);
                    self.half_delete_in_edge(bl, rev);
                }
            } else {
                i += 1;
            }
        }
    }

    /// Find blocks that are at the end of multiple edges
    /// (C++ `FlowBlock::findDups`, `block.cc:507`).  Edge list is selected by
    /// `incoming`.
    fn find_dups(&mut self, this_id: BlockId, incoming: bool, duplist: &mut Vec<BlockId>) {
        let n = if incoming {
            self.arena[this_id].intothis.len()
        } else {
            self.arena[this_id].outofthis.len()
        };
        for i in 0..n {
            let p = if incoming {
                self.arena[this_id].intothis[i].point.expect("findDups")
            } else {
                self.arena[this_id].outofthis[i].point.expect("findDups")
            };
            if (self.arena[p].flags & block_flags::f_mark2) != 0 {
                continue; // Already marked as a duplicate
            }
            if (self.arena[p].flags & block_flags::f_mark) != 0 {
                duplist.push(p);
                self.arena[p].flags |= block_flags::f_mark2;
            } else {
                self.arena[p].flags |= block_flags::f_mark;
            }
        }
        for i in 0..n {
            let p = if incoming {
                self.arena[this_id].intothis[i].point.expect("findDups")
            } else {
                self.arena[this_id].outofthis[i].point.expect("findDups")
            };
            self.arena[p].flags &= !(block_flags::f_mark | block_flags::f_mark2);
        }
    }

    /// Eliminate duplicate edges (C++ `FlowBlock::dedup`, `block.cc:525`).
    fn dedup(&mut self, this_id: BlockId) {
        let mut duplist: Vec<BlockId> = Vec::new();
        self.find_dups(this_id, true, &mut duplist);
        for &bl in &duplist {
            self.eliminate_in_dups(this_id, bl);
        }
        duplist.clear();
        self.find_dups(this_id, false, &mut duplist);
        for &bl in &duplist {
            self.eliminate_out_dups(this_id, bl);
        }
    }

    // ----------------------------------------------------------------------
    // Leaf / depth / dominance traversal (FlowBlock public)
    // ----------------------------------------------------------------------

    /// Get the first leaf FlowBlock (C++ `FlowBlock::getFrontLeaf`,
    /// `block.cc:340`).  Descends taking the front block until a `t_copy`.
    pub fn get_front_leaf(&self, this_id: BlockId) -> Option<BlockId> {
        let mut bl = this_id;
        while self.arena[bl].get_type() != BlockType::Copy {
            let sub = self.sub_block(bl, 0);
            match sub {
                Some(s) => bl = s,
                None => return None,
            }
        }
        Some(bl)
    }

    /// Within the hierarchy of `this_id`, assuming `bl` falls through at some
    /// point, return the first leaf block (BlockBasic/BlockCopy) that executes
    /// after `bl` completes — or `None` if the next block is not unique.
    ///
    /// Faithful transcription of the virtual `nextFlowAfter` hierarchy:
    ///   * base `FlowBlock` (`block.hh:902`): `None`.
    ///   * `BlockGraph` (`block.cc:1336`): block after `bl` in `list`, front-leaf;
    ///     recurse into `getParent()` past the end of the list.
    ///   * `BlockGoto` (`block.cc:2947`): `getGotoTarget()->getFrontLeaf()`.
    ///   * `BlockMultiGoto`/`BlockCondition`/`BlockDoWhile` (`block.cc:2979`/`3101`/`3499`): `None`.
    ///   * `BlockIf` (`block.cc:3175`): `None` if `bl` is the condition block,
    ///     else recurse into the parent.
    ///   * `BlockWhileDo`/`BlockInfLoop` (`block.cc:3389`/`3527`): front-leaf of
    ///     `getBlock(0)` (the loop body re-executes), `None` for the while-do
    ///     condition block.
    ///   * `BlockSwitch` (`block.cc:3690`): next case block (fall-thru) front-leaf,
    ///     or recurse into the parent for the last case; `None` otherwise.
    pub fn next_flow_after(&self, this_id: BlockId, bl: BlockId) -> Option<BlockId> {
        match self.arena[this_id].get_type() {
            BlockType::Goto => {
                let target = self.arena[this_id].get_goto_target()?;
                self.get_front_leaf(target)
            }
            BlockType::If => {
                // getBlock(0)==bl => None (don't know where flow goes)
                if self.sub_block(this_id, 0) == Some(bl) {
                    return None;
                }
                let parent = self.arena[this_id].get_parent()?;
                self.next_flow_after(parent, this_id)
            }
            BlockType::WhileDo => {
                if self.sub_block(this_id, 0) == Some(bl) {
                    return None; // don't know what executes next
                }
                let b0 = self.sub_block(this_id, 0)?;
                self.get_front_leaf(b0)
            }
            BlockType::InfLoop => {
                let b0 = self.sub_block(this_id, 0)?;
                self.get_front_leaf(b0)
            }
            BlockType::Switch => {
                if self.sub_block(this_id, 0) == Some(bl) {
                    return None;
                }
                // Fall-thru must be a goto block (else there is a break).
                if self.arena[bl].get_type() != BlockType::Goto {
                    return None;
                }
                let cases = self.arena[this_id].switch_caseblocks();
                let mut idx = cases.len();
                for (i, c) in cases.iter().enumerate() {
                    if c.block == bl {
                        idx = i;
                        break;
                    }
                }
                if idx == cases.len() {
                    return None; // didn't find block
                }
                let next = idx + 1;
                if next < cases.len() {
                    let nb = cases[next].block;
                    return self.get_front_leaf(nb);
                }
                // last case: flow is to the switch exit
                let parent = self.arena[this_id].get_parent()?;
                self.next_flow_after(parent, this_id)
            }
            BlockType::Graph | BlockType::Ls => {
                // BlockGraph::nextFlowAfter: block after `bl` in `list`.
                let list = &self.arena[this_id].list;
                let mut pos = None;
                for (i, &b) in list.iter().enumerate() {
                    if b == bl {
                        pos = Some(i);
                        break;
                    }
                }
                match pos {
                    Some(i) if i + 1 < list.len() => {
                        let nextbl = list[i + 1];
                        self.get_front_leaf(nextbl)
                    }
                    _ => {
                        // Off the end of the list (or not found): recurse up.
                        let parent = self.arena[this_id].get_parent()?;
                        self.next_flow_after(parent, this_id)
                    }
                }
            }
            // base FlowBlock + MultiGoto + Condition + DoWhile: None.
            _ => None,
        }
    }

    /// Should a `BlockGoto` emit a formal `goto` statement (C++
    /// `BlockGoto::gotoPrints`, `block.cc:2929`)?  True when the goto target's
    /// front-leaf is not the block that would execute next in normal flow.  For
    /// non-Goto blocks, this is vacuously `false`.
    pub fn goto_prints(&self, this_id: BlockId) -> bool {
        if self.arena[this_id].get_type() != BlockType::Goto {
            return false;
        }
        match self.arena[this_id].get_parent() {
            Some(parent) => {
                let nextbl = self.next_flow_after(parent, this_id);
                let gotobl = self
                    .arena[this_id]
                    .get_goto_target()
                    .and_then(|t| self.get_front_leaf(t));
                gotobl != nextbl
            }
            None => false,
        }
    }

    /// Get the last leaf FlowBlock reachable as an exit (C++
    /// `FlowBlock::getExitLeaf`, virtual per subtype, `block.cc`/`block.hh`):
    ///   * `t_basic` / `t_copy`: returns itself.
    ///   * `t_goto` / `t_multigoto`: `getBlock(0)->getExitLeaf()`.
    ///   * `t_ls` (BlockList): `getBlock(size-1)->getExitLeaf()`.
    ///   * `t_if`: only an `ifgoto` (size 1) has an exit leaf — `getBlock(0)->...`.
    ///   * everything else (base default): no exit leaf (`None`).
    pub fn get_exit_leaf(&self, this_id: BlockId) -> Option<BlockId> {
        match self.arena[this_id].get_type() {
            BlockType::Basic | BlockType::Copy => Some(this_id),
            BlockType::Goto | BlockType::MultiGoto => {
                let b0 = self.sub_block(this_id, 0)?;
                self.get_exit_leaf(b0)
            }
            BlockType::Ls => {
                let n = self.arena[this_id].get_size();
                if n == 0 {
                    return None;
                }
                let last = self.sub_block(this_id, n - 1)?;
                self.get_exit_leaf(last)
            }
            BlockType::If => {
                if self.arena[this_id].get_size() == 1 {
                    let b0 = self.sub_block(this_id, 0)?;
                    self.get_exit_leaf(b0)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// The last PcodeOp of a structured FlowBlock (C++ `FlowBlock::lastOp`,
    /// virtual per subtype, `block.cc`).  Resolves down the structure to the
    /// basic block carrying the block's final op:
    ///   * `t_basic` (`block.cc:2348`): `op.back()` (the basic block's tail op).
    ///   * `t_ls` (BlockList, `block.cc:3008`): `getBlock(size-1)->lastOp()`.
    ///   * `t_condition` (`block.cc:3064`): `getBlock(1)->lastOp()`.
    ///   * `t_if` (`block.cc:3167`): only an `ifgoto` (size 1) has a last op —
    ///     `getBlock(0)->lastOp()`.
    ///   * everything else (base default, `block.hh`): `None`.
    pub fn struct_last_op(&self, this_id: BlockId) -> Option<crate::seams::OpId> {
        match &self.arena[this_id].kind {
            BlockKind::Basic(bd) => bd.op_tail,
            BlockKind::Ls => {
                let n = self.arena[this_id].get_size();
                if n == 0 {
                    return None;
                }
                let last = self.sub_block(this_id, n - 1)?;
                self.struct_last_op(last)
            }
            BlockKind::Condition { .. } => {
                let b1 = self.sub_block(this_id, 1)?;
                self.struct_last_op(b1)
            }
            BlockKind::If { .. } => {
                if self.arena[this_id].get_size() == 1 {
                    let b0 = self.sub_block(this_id, 0)?;
                    self.struct_last_op(b0)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get the i-th component block (C++ `subBlock`, dispatched per type).
    ///
    /// `BlockGraph` subtypes return `list[i]`; `BlockCopy` returns its `copy`
    /// for any `i`; leaves return `None`.
    pub fn sub_block(&self, this_id: BlockId, i: int4) -> Option<BlockId> {
        match &self.arena[this_id].kind {
            BlockKind::Copy { copy } => *copy,
            BlockKind::Plain | BlockKind::Basic(_) => None,
            _ => {
                // BlockGraph and structured subtypes: list[i]
                self.arena[this_id].list.get(i as usize).copied()
            }
        }
    }

    /// How many getParent() calls from the leaf to \b this
    /// (C++ `FlowBlock::calcDepth`, `block.cc:368`).  Returns -1 if not an
    /// ancestor.
    pub fn calc_depth(&self, this_id: BlockId, leaf: Option<BlockId>) -> int4 {
        let mut depth = 0;
        let mut leaf = leaf;
        while leaf != Some(this_id) {
            match leaf {
                None => return -1,
                Some(l) => {
                    leaf = self.arena[l].parent;
                    depth += 1;
                }
            }
        }
        depth
    }

    /// Is a boolean condition restricted to a single edge into \b this block
    /// (C++ `FlowBlock::restrictedByConditional`, `block.cc:405`).
    ///
    /// Returns \b true if every path into `this_id` either has a unique direct
    /// edge from `cond` or never passes through `cond` at all — i.e. a constant
    /// known down one out-edge of `cond` provably holds throughout `this_id`.
    pub fn restricted_by_conditional(&self, this_id: BlockId, cond: BlockId) -> bool {
        // if (sizeIn() == 1) return true;
        if self.arena[this_id].size_in() == 1 {
            return true;
        }
        // if (getImmedDom() != cond) return false;
        if self.arena[this_id].get_immed_dom() != Some(cond) {
            return false;
        }
        let mut seen_cond = false;
        let n = self.arena[this_id].size_in();
        for i in 0..n {
            let in_block = self.arena[this_id].get_in(i);
            if in_block == cond {
                if seen_cond {
                    return false; // Coming in from cond block on multiple direct edges
                }
                seen_cond = true;
                continue;
            }
            // while(inBlock != this) { if (inBlock == cond) return false; inBlock = inBlock->getImmedDom(); }
            let mut walk = Some(in_block);
            while walk != Some(this_id) {
                match walk {
                    Some(b) if b == cond => return false, // Must have come through sibling
                    Some(b) => walk = self.arena[b].get_immed_dom(),
                    None => break,
                }
            }
        }
        true
    }

    /// Does \b this block dominate the given block (C++ `FlowBlock::dominates`,
    /// `block.cc:386`).  Assumes reverse-post-order indices.
    pub fn dominates(&self, this_id: BlockId, sub_block: Option<BlockId>) -> bool {
        let index = self.arena[this_id].index;
        let mut sub_block = sub_block;
        while let Some(sb) = sub_block {
            if index > self.arena[sb].index {
                break;
            }
            if sb == this_id {
                return true;
            }
            sub_block = self.arena[sb].immed_dom;
        }
        false
    }

    // ----------------------------------------------------------------------
    // Common-dominator helpers (FlowBlock static)
    // ----------------------------------------------------------------------

    /// Find the earliest common ancestor of two FlowBlocks within the dominator
    /// tree (C++ `FlowBlock::findCommonBlock(bl1,bl2)`, `block.cc:736`).
    pub fn find_common_block(&mut self, bl1: BlockId, bl2: BlockId) -> Option<BlockId> {
        let mut common: Option<BlockId> = None;
        let mut b1: Option<BlockId> = Some(bl1);
        let mut b2: Option<BlockId> = Some(bl2);
        loop {
            if b2.is_none() {
                while let Some(b) = b1 {
                    if self.arena[b].is_mark() {
                        common = Some(b);
                        break;
                    }
                    b1 = self.arena[b].immed_dom;
                }
                break;
            }
            if b1.is_none() {
                while let Some(b) = b2 {
                    if self.arena[b].is_mark() {
                        common = Some(b);
                        break;
                    }
                    b2 = self.arena[b].immed_dom;
                }
                break;
            }
            let i1 = b1.unwrap();
            if self.arena[i1].is_mark() {
                common = Some(i1);
                break;
            }
            self.arena[i1].set_mark();
            let i2 = b2.unwrap();
            if self.arena[i2].is_mark() {
                common = Some(i2);
                break;
            }
            self.arena[i2].set_mark();
            b1 = self.arena[i1].immed_dom;
            b2 = self.arena[i2].immed_dom;
        }
        // Clear our marks
        let mut c1: Option<BlockId> = Some(bl1);
        while let Some(b) = c1 {
            if !self.arena[b].is_mark() {
                break;
            }
            self.arena[b].clear_mark();
            c1 = self.arena[b].immed_dom;
        }
        let mut c2: Option<BlockId> = Some(bl2);
        while let Some(b) = c2 {
            if !self.arena[b].is_mark() {
                break;
            }
            self.arena[b].clear_mark();
            c2 = self.arena[b].immed_dom;
        }
        common
    }

    /// Find the most immediate dominating FlowBlock of all blocks in the given
    /// set (C++ `FlowBlock::findCommonBlock(blockSet)`, `block.cc:796`).
    pub fn find_common_block_set(&mut self, block_set: &[BlockId]) -> BlockId {
        let mut marked_set: Vec<BlockId> = Vec::new();
        let mut res = block_set[0];
        let mut best_index = self.arena[res].index;
        let mut bl = Some(res);
        // do { ... } while(bl != null)
        loop {
            let b = bl.expect("findCommonBlock: null in do-while head");
            self.arena[b].set_mark();
            marked_set.push(b);
            bl = self.arena[b].immed_dom;
            if bl.is_none() {
                break;
            }
        }
        for i in 1..block_set.len() {
            if best_index == 0 {
                break;
            }
            let mut cur = block_set[i];
            while !self.arena[cur].is_mark() {
                self.arena[cur].set_mark();
                marked_set.push(cur);
                cur = self.arena[cur].immed_dom.expect("findCommonBlock: walked off root");
            }
            if self.arena[cur].index < best_index {
                res = cur;
                best_index = self.arena[res].index;
            }
        }
        for &b in &marked_set {
            self.arena[b].clear_mark();
        }
        res
    }

    // ----------------------------------------------------------------------
    // BlockGraph component management (block.cc)
    // ----------------------------------------------------------------------

    /// Add a component FlowBlock and make `graph_id` the parent
    /// (C++ `BlockGraph::addBlock`, `block.cc:862`).  Updates the graph index to
    /// the minimum over all components.
    fn add_block(&mut self, graph_id: BlockId, bl: BlockId) {
        let min = self.arena[bl].index;
        // C++: if (list.empty()) index=min; else if (min<index) index=min;
        // both arms assign min, so the combined guard is faithful.
        if self.arena[graph_id].list.is_empty() || min < self.arena[graph_id].index {
            self.arena[graph_id].index = min;
        }
        self.arena[bl].parent = Some(graph_id);
        self.arena[graph_id].list.push(bl);
    }

    /// Force `graph_id` to have the indicated number of outputs, creating
    /// self-loop edges if necessary (C++ `BlockGraph::forceOutputNum`,
    /// `block.cc:880`).
    fn force_output_num(&mut self, graph_id: BlockId, i: int4) {
        while self.arena[graph_id].size_out() < i {
            self.add_in_edge(
                graph_id,
                graph_id,
                edge_flags::f_loop_edge | edge_flags::f_back_edge,
            );
        }
    }

    /// Inherit our edges from the edges of our components
    /// (C++ `BlockGraph::selfIdentify`, `block.cc:895`).
    fn self_identify(&mut self, graph_id: BlockId) {
        if self.arena[graph_id].list.is_empty() {
            return;
        }
        let comps: Vec<BlockId> = self.arena[graph_id].list.clone();
        for mybl in comps {
            // walk intothis
            let mut i = 0;
            while i < self.arena[mybl].intothis.len() {
                let otherbl = self.arena[mybl].intothis[i].point.expect("selfIdentify in");
                if self.arena[otherbl].parent == Some(graph_id) {
                    i += 1;
                } else {
                    for j in 0..self.arena[otherbl].outofthis.len() as int4 {
                        if self.arena[otherbl].outofthis[j as usize].point == Some(mybl) {
                            self.replace_out_edge(otherbl, j, graph_id);
                        }
                    }
                    // Dont increment i
                }
            }
            // walk outofthis
            let mut i = 0;
            while i < self.arena[mybl].outofthis.len() {
                let otherbl = self.arena[mybl].outofthis[i].point.expect("selfIdentify out");
                if self.arena[otherbl].parent == Some(graph_id) {
                    i += 1;
                } else {
                    for j in 0..self.arena[otherbl].intothis.len() as int4 {
                        if self.arena[otherbl].intothis[j as usize].point == Some(mybl) {
                            self.replace_in_edge(otherbl, j, graph_id);
                        }
                    }
                    if self.arena[mybl].is_switch_out() {
                        self.arena[graph_id].set_flag(block_flags::f_switch_out);
                    }
                }
            }
        }
        self.dedup(graph_id);
    }

    /// Move component nodes from `graph_id` into a new BlockGraph node `ident`
    /// (C++ `BlockGraph::identifyInternal`, `block.cc:940`).
    fn identify_internal(&mut self, graph_id: BlockId, ident: BlockId, nodes: &[BlockId]) {
        for &n in nodes {
            self.arena[n].set_mark();
            self.add_block(ident, n); // Maintain order of blocks
            let inherit = self.arena[n].flags
                & (block_flags::f_interior_gotoout | block_flags::f_interior_gotoin);
            self.arena[ident].flags |= inherit;
        }
        // Remove -nodes- from graph's list
        let oldlist: Vec<BlockId> = self.arena[graph_id].list.clone();
        let mut newlist: Vec<BlockId> = Vec::new();
        for it in oldlist {
            if !self.arena[it].is_mark() {
                newlist.push(it);
            } else {
                self.arena[it].clear_mark();
            }
        }
        self.arena[graph_id].list = newlist;
        self.self_identify(ident);
    }

    /// Clear a set of properties from all edges in the graph
    /// (C++ `BlockGraph::clearEdgeFlags`, `block.cc:966`).
    fn clear_edge_flags(&mut self, graph_id: BlockId, fl: uint4) {
        let fl = !fl;
        let list: Vec<BlockId> = self.arena[graph_id].list.clone();
        for bl in list {
            for i in 0..self.arena[bl].intothis.len() {
                self.arena[bl].intothis[i].label &= fl;
            }
            for i in 0..self.arena[bl].outofthis.len() {
                self.arena[bl].outofthis[i].label &= fl;
            }
        }
    }

    /// Create a single artificial root block with no \e in edges and one \e out
    /// edge to each real entry point (C++ `BlockGraph::createVirtualRoot`,
    /// `block.cc:988`).  The root is added to the arena but not to any graph's
    /// component list (the C++ root is unowned).
    fn create_virtual_root(&mut self, rootlist: &[BlockId]) -> BlockId {
        let newroot = self.arena.insert(FlowBlock::new());
        for &r in rootlist {
            self.add_in_edge(r, newroot, 0);
        }
        newroot
    }

    /// Force the \e false out edge to go to the given FlowBlock
    /// (C++ `BlockGraph::forceFalseEdge`, `block.cc:1204`).
    fn force_false_edge(&mut self, graph_id: BlockId, out0: BlockId) -> KunaResult<()> {
        if self.arena[graph_id].size_out() != 2 {
            return Err(KunaError::lowlevel("Can only preserve binary condition"));
        }
        let mut out0 = out0;
        if self.arena[out0].parent == Some(graph_id) {
            out0 = graph_id; // Allow for loops to self
        }
        if self.arena[graph_id].outofthis[0].point != Some(out0) {
            self.swap_edges(graph_id);
        }
        if self.arena[graph_id].outofthis[0].point != Some(out0) {
            return Err(KunaError::lowlevel("Unable to preserve condition"));
        }
        Ok(())
    }

    /// Swap the positions of two component FlowBlocks
    /// (C++ `BlockGraph::swapBlocks`, `block.cc:1221`).
    pub fn swap_blocks(&mut self, graph_id: BlockId, i: int4, j: int4) {
        self.arena[graph_id].list.swap(i as usize, j as usize);
    }

    /// Set properties on the first leaf FlowBlock
    /// (C++ `BlockGraph::markCopyBlock`, `block.cc:1233`).
    ///
    /// SEAM(W7): consumed only by the structuring `markUnstructured` overrides
    /// (BlockGoto/BlockIf/BlockSwitch), which the W7 `blockaction` wave ports;
    /// unused by the W3 tree deliverable alone, hence the allow.
    #[allow(dead_code)]
    fn mark_copy_block(&mut self, bl: BlockId, fl: uint4) {
        if let Some(leaf) = self.get_front_leaf(bl) {
            self.arena[leaf].flags |= fl;
        }
    }

    // ----------------------------------------------------------------------
    // Public BlockGraph edge API (block.cc)
    // ----------------------------------------------------------------------

    /// Add a directed edge between component FlowBlocks
    /// (C++ `BlockGraph::addEdge`, `block.cc:1442`).
    pub fn add_edge(&mut self, begin: BlockId, end: BlockId) {
        self.add_in_edge(end, begin, 0);
    }

    /// Mark a given edge as a \e loop edge (C++ `BlockGraph::addLoopEdge`,
    /// `block.cc:1454`).
    pub fn add_loop_edge(&mut self, begin: BlockId, outindex: int4) {
        self.set_out_edge_flag(begin, outindex, edge_flags::f_loop_edge);
    }

    /// Remove an edge between component FlowBlocks (C++ `BlockGraph::removeEdge`,
    /// `block.cc:1472`).  The edge must already exist.
    pub fn remove_edge(&mut self, begin: BlockId, end: BlockId) {
        let mut i = 0;
        while i < self.arena[end].intothis.len() {
            if self.arena[end].intothis[i].point == Some(begin) {
                break;
            }
            i += 1;
        }
        self.remove_in_edge(end, i as int4);
    }

    /// Move an edge from one out FlowBlock to another (C++ `BlockGraph::switchEdge`,
    /// `block.cc:1492`).
    pub fn switch_edge(&mut self, in_id: BlockId, outbefore: BlockId, outafter: BlockId) {
        for i in 0..self.arena[in_id].outofthis.len() as int4 {
            if self.arena[in_id].outofthis[i as usize].point == Some(outbefore) {
                self.replace_out_edge(in_id, i, outafter);
            }
        }
    }

    /// Move the indicated \e out edge to a new input FlowBlock
    /// (C++ `BlockGraph::moveOutEdge`, `block.cc:1505`).
    pub fn move_out_edge(&mut self, blold: BlockId, slot: int4, blnew: BlockId) {
        let outbl = self.arena[blold].get_out(slot);
        let i = self.arena[blold].get_out_rev_index(slot);
        self.replace_in_edge(outbl, i, blnew);
    }

    /// Remove a FlowBlock from \b this BlockGraph and delete it
    /// (C++ `BlockGraph::removeBlock`, `block.cc:1520`).
    pub fn remove_block(&mut self, graph_id: BlockId, bl: BlockId) {
        while self.arena[bl].size_in() > 0 {
            let in0 = self.arena[bl].get_in(0);
            self.remove_edge(in0, bl);
        }
        while self.arena[bl].size_out() > 0 {
            let out0 = self.arena[bl].get_out(0);
            self.remove_edge(bl, out0);
        }
        if let Some(pos) = self.arena[graph_id].list.iter().position(|&x| x == bl) {
            self.arena[graph_id].list.remove(pos);
        }
        self.arena.remove(bl); // Free up memory
    }

    /// Remove a FlowBlock preserving flow (C++ `BlockGraph::removeFromFlow`,
    /// `block.cc:1548`).  Does not remove `bl` from the graph.
    pub fn remove_from_flow(&mut self, bl: BlockId) {
        while self.arena[bl].size_out() > 0 {
            let last = self.arena[bl].size_out() - 1;
            let bbout = self.arena[bl].get_out(last);
            self.remove_out_edge(bl, last);
            while self.arena[bl].size_in() > 0 {
                let bbin = self.arena[bl].get_in(0);
                let rev = self.arena[bl].intothis[0].reverse_index;
                self.replace_out_edge(bbin, rev, bbout);
            }
        }
    }

    /// Remove FlowBlock splitting flow between input and output edges
    /// (C++ `BlockGraph::removeFromFlowSplit`, `block.cc:1578`).
    pub fn remove_from_flow_split(&mut self, bl: BlockId, flipflow: bool) {
        if flipflow {
            self.replace_edges_thru(bl, 0, 1); // 0 -> 1
        } else {
            self.replace_edges_thru(bl, 1, 1); // 1 -> 1
        }
        self.replace_edges_thru(bl, 0, 0); // Replace remaining edge
    }

    /// Splice the given FlowBlock together with its single output
    /// (C++ `BlockGraph::spliceBlock`, `block.cc:1600`).
    pub fn splice_block(&mut self, graph_id: BlockId, bl: BlockId) -> KunaResult<()> {
        let mut outbl: Option<BlockId> = None;
        if self.arena[bl].size_out() == 1 {
            let o = self.arena[bl].get_out(0);
            if self.arena[o].size_in() == 1 {
                outbl = Some(o);
            }
        }
        let outbl = match outbl {
            Some(o) => o,
            None => {
                return Err(KunaError::lowlevel(
                    "Can only splice a block with 1 output to a block with 1 input",
                ))
            }
        };
        let fl1 = self.arena[bl].flags
            & (block_flags::f_unstructured_targ | block_flags::f_entry_point);
        let fl2 = self.arena[outbl].flags & block_flags::f_switch_out;
        self.remove_out_edge(bl, 0);
        let szout = self.arena[outbl].size_out();
        for _ in 0..szout {
            self.move_out_edge(outbl, 0, bl);
        }
        self.remove_block(graph_id, outbl);
        self.arena[bl].flags = fl1 | fl2;
        Ok(())
    }

    /// Set the entry point FlowBlock for the graph (C++ `BlockGraph::setStartBlock`,
    /// `block.cc:1628`).
    pub fn set_start_block(&mut self, graph_id: BlockId, bl: BlockId) {
        let first = self.arena[graph_id].list[0];
        if (self.arena[first].flags & block_flags::f_entry_point) != 0 {
            if bl == first {
                return; // Already set as start block
            }
            self.arena[first].flags &= !block_flags::f_entry_point;
        }
        let mut i = 0;
        while i < self.arena[graph_id].list.len() {
            if self.arena[graph_id].list[i] == bl {
                break;
            }
            i += 1;
        }
        // Slide everybody down
        let mut j = i;
        while j > 0 {
            self.arena[graph_id].list[j] = self.arena[graph_id].list[j - 1];
            j -= 1;
        }
        self.arena[graph_id].list[0] = bl;
        self.arena[bl].flags |= block_flags::f_entry_point;
    }

    /// Get the entry point FlowBlock (C++ `BlockGraph::getStartBlock`,
    /// `block.cc:1652`).
    pub fn get_start_block(&self, graph_id: BlockId) -> KunaResult<BlockId> {
        let list = &self.arena[graph_id].list;
        if list.is_empty() || (self.arena[list[0]].flags & block_flags::f_entry_point) == 0 {
            return Err(KunaError::lowlevel("No start block registered"));
        }
        Ok(list[0])
    }

    // --- factory functions (block.cc) --------------------------------------

    /// Build a new plain FlowBlock and add it to the graph
    /// (C++ `BlockGraph::newBlock`, `block.cc:1662`).
    pub fn new_block(&mut self, graph_id: BlockId) -> BlockId {
        let ret = self.arena.insert(FlowBlock::new());
        self.add_block(graph_id, ret);
        ret
    }

    /// Build a new BlockBasic and add it to the graph
    /// (C++ `BlockGraph::newBlockBasic`, `block.cc:1673`).
    ///
    /// SEAM(W7): the C++ takes `Funcdata *fd`; here the funcdata back-pointer is
    /// implicit (the function owns the arena), so we only build the basic data.
    pub fn new_block_basic(&mut self, graph_id: BlockId) -> BlockId {
        let ret =
            self.arena.insert(FlowBlock::new_kind(BlockKind::Basic(BasicData::default())));
        self.add_block(graph_id, ret);
        ret
    }

    /// Build a new BlockCopy and add it to the graph
    /// (C++ `BlockGraph::newBlockCopy`, `block.cc:1684`).  Copies edges,
    /// dominator, index, numdesc, and flags from the underlying block.
    pub fn new_block_copy(&mut self, graph_id: BlockId, bl: BlockId) -> BlockId {
        let intothis = self.arena[bl].intothis.clone();
        let outofthis = self.arena[bl].outofthis.clone();
        let immed_dom = self.arena[bl].immed_dom;
        let index = self.arena[bl].index;
        let numdesc = self.arena[bl].numdesc;
        let blflags = self.arena[bl].flags;
        let mut ret = FlowBlock::new_kind(BlockKind::Copy { copy: Some(bl) });
        ret.intothis = intothis;
        ret.outofthis = outofthis;
        ret.immed_dom = immed_dom;
        ret.index = index;
        // visitcount stays 0 (FlowBlock ctor), per C++ comment
        ret.numdesc = numdesc;
        ret.flags |= blflags;
        if ret.outofthis.len() > 2 {
            ret.flags |= block_flags::f_switch_out;
        }
        let ret_id = self.arena.insert(ret);
        self.add_block(graph_id, ret_id);
        ret_id
    }

    /// Build a new BlockGoto incorporating the given FlowBlock
    /// (C++ `BlockGraph::newBlockGoto`, `block.cc:1705`).
    pub fn new_block_goto(&mut self, graph_id: BlockId, bl: BlockId) -> BlockId {
        let target = self.arena[bl].get_out(0);
        let ret = self
            .arena
            .insert(FlowBlock::new_kind(BlockKind::Goto {
                gototarget: Some(target),
                gototype: block_flags::f_goto_goto,
            }));
        self.identify_internal(graph_id, ret, &[bl]);
        self.add_block(graph_id, ret);
        self.force_output_num(ret, 1);
        let out0 = self.arena[ret].get_out(0);
        self.remove_edge(ret, out0); // Treat out edge as if it didn't exist
        ret
    }

    /// Build (or extend) a BlockMultiGoto for the given goto edge
    /// (C++ `BlockGraph::newBlockMultiGoto`, `block.cc:1723`).
    pub fn new_block_multi_goto(
        &mut self,
        graph_id: BlockId,
        bl: BlockId,
        outedge: int4,
    ) -> BlockId {
        let targetbl = self.arena[bl].get_out(outedge);
        let isdefaultedge = self.arena[bl].is_default_branch(outedge);
        if self.arena[bl].get_type() == BlockType::MultiGoto {
            let ret = bl;
            if let BlockKind::MultiGoto { gotoedges, .. } = &mut self.arena[ret].kind {
                gotoedges.push(targetbl);
            }
            self.remove_edge(ret, targetbl);
            if isdefaultedge {
                if let BlockKind::MultiGoto { defaultswitch, .. } = &mut self.arena[ret].kind {
                    *defaultswitch = true;
                }
            }
            ret
        } else {
            let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::MultiGoto {
                gotoedges: Vec::new(),
                defaultswitch: false,
            }));
            let orig_size_out = self.arena[bl].size_out();
            self.identify_internal(graph_id, ret, &[bl]);
            self.add_block(graph_id, ret);
            if let BlockKind::MultiGoto { gotoedges, .. } = &mut self.arena[ret].kind {
                gotoedges.push(targetbl);
            }
            if targetbl != bl {
                if self.arena[ret].size_out() != orig_size_out {
                    let n = self.arena[ret].size_out() + 1;
                    self.force_output_num(ret, n);
                }
                self.remove_edge(ret, targetbl);
            }
            if isdefaultedge {
                if let BlockKind::MultiGoto { defaultswitch, .. } = &mut self.arena[ret].kind {
                    *defaultswitch = true;
                }
            }
            ret
        }
    }

    /// Build a new BlockList collapsing the given components
    /// (C++ `BlockGraph::newBlockList`, `block.cc:1761`).
    pub fn new_block_list(&mut self, graph_id: BlockId, nodes: &[BlockId]) -> KunaResult<BlockId> {
        let back = *nodes.last().expect("newBlockList: empty nodes");
        let outforce = self.arena[back].size_out();
        let mut out0: Option<BlockId> = None;
        if outforce == 2 {
            out0 = Some(self.arena[back].get_out(0));
        }
        let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::Ls));
        self.identify_internal(graph_id, ret, nodes);
        self.add_block(graph_id, ret);
        self.force_output_num(ret, outforce);
        if self.arena[ret].size_out() == 2 {
            self.force_false_edge(ret, out0.expect("newBlockList: out0"))?;
        }
        Ok(ret)
    }

    /// Build a new BlockCondition collapsing its two pieces
    /// (C++ `BlockGraph::newBlockCondition`, `block.cc:1783`).
    pub fn new_block_condition(
        &mut self,
        graph_id: BlockId,
        b1: BlockId,
        b2: BlockId,
    ) -> KunaResult<BlockId> {
        let out0 = self.arena[b2].get_out(0);
        let opc = if self.arena[b1].get_false_out() == b2 {
            OpCode::CPUI_INT_OR
        } else {
            OpCode::CPUI_INT_AND
        };
        let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::Condition { opc }));
        self.identify_internal(graph_id, ret, &[b1, b2]);
        self.add_block(graph_id, ret);
        self.force_output_num(ret, 2); // All conditions must have two outputs
        self.force_false_edge(ret, out0)?;
        Ok(ret)
    }

    /// Build a new BlockIfGoto collapsing the given condition
    /// (C++ `BlockGraph::newBlockIfGoto`, `block.cc:1802`).
    pub fn new_block_if_goto(&mut self, graph_id: BlockId, cond: BlockId) -> KunaResult<BlockId> {
        if !self.arena[cond].is_goto_out(1) {
            return Err(KunaError::lowlevel(
                "Building ifgoto where true branch is not the goto",
            ));
        }
        let out0 = self.arena[cond].get_out(0);
        let goto_t = self.arena[cond].get_out(1);
        let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::If {
            gototype: block_flags::f_goto_goto,
            gototarget: Some(goto_t),
        }));
        self.identify_internal(graph_id, ret, &[cond]);
        self.add_block(graph_id, ret);
        self.force_output_num(ret, 2);
        self.force_false_edge(ret, out0)?;
        let true_out = self.arena[ret].get_true_out();
        self.remove_edge(ret, true_out); // Treat the true edge as if it didn't exist
        Ok(ret)
    }

    /// Build a new BlockIf collapsing the condition and body
    /// (C++ `BlockGraph::newBlockIf`, `block.cc:1825`).
    pub fn new_block_if(&mut self, graph_id: BlockId, cond: BlockId, tc: BlockId) -> BlockId {
        let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::If {
            gototype: block_flags::f_goto_goto,
            gototarget: None,
        }));
        self.identify_internal(graph_id, ret, &[cond, tc]);
        self.add_block(graph_id, ret);
        self.force_output_num(ret, 1);
        ret
    }

    /// Build a new BlockIfElse collapsing the condition, true and false clauses
    /// (C++ `BlockGraph::newBlockIfElse`, `block.cc:1843`).
    pub fn new_block_if_else(
        &mut self,
        graph_id: BlockId,
        cond: BlockId,
        tc: BlockId,
        fc: BlockId,
    ) -> BlockId {
        let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::If {
            gototype: block_flags::f_goto_goto,
            gototarget: None,
        }));
        self.identify_internal(graph_id, ret, &[cond, tc, fc]);
        self.add_block(graph_id, ret);
        self.force_output_num(ret, 1);
        ret
    }

    /// Build a new BlockWhileDo collapsing the condition and clause
    /// (C++ `BlockGraph::newBlockWhileDo`, `block.cc:1861`).
    pub fn new_block_while_do(&mut self, graph_id: BlockId, cond: BlockId, cl: BlockId) -> BlockId {
        let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::WhileDo {
            initialize_op: None,
            iterate_op: None,
            loop_def: None,
        }));
        self.identify_internal(graph_id, ret, &[cond, cl]);
        self.add_block(graph_id, ret);
        self.force_output_num(ret, 1);
        ret
    }

    /// Build a new BlockDoWhile collapsing the condition clause
    /// (C++ `BlockGraph::newBlockDoWhile`, `block.cc:1877`).
    pub fn new_block_do_while(&mut self, graph_id: BlockId, condcl: BlockId) -> BlockId {
        let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::DoWhile));
        self.identify_internal(graph_id, ret, &[condcl]);
        self.add_block(graph_id, ret);
        self.force_output_num(ret, 1);
        ret
    }

    /// Build a new BlockInfLoop collapsing the body
    /// (C++ `BlockGraph::newBlockInfLoop`, `block.cc:1892`).
    pub fn new_block_inf_loop(&mut self, graph_id: BlockId, body: BlockId) -> BlockId {
        let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::InfLoop));
        self.identify_internal(graph_id, ret, &[body]);
        self.add_block(graph_id, ret);
        ret
    }

    /// Build a new BlockSwitch collapsing the switch root + its case components
    /// (C++ `BlockGraph::newBlockSwitch`, `block.cc:1907`).
    ///
    /// `cs[0]` is the switch component (the multi-out block); `cs[1..]` are the
    /// case components.  `jt_index` is the `Funcdata::jumpvec` slot of the
    /// recovered [`JumpTable`](crate::jumptable::JumpTable) (C++ resolves this via
    /// `rootbl->getExitLeaf()`'s BRANCHIND → `findJumpTable`; the Rust port passes
    /// the precomputed slot since the structuring graph has no Funcdata pointer).
    pub fn new_block_switch(
        &mut self,
        graph_id: BlockId,
        cs: &[BlockId],
        has_exit: bool,
        jt_index: usize,
        switch_case_edges: &std::collections::BTreeMap<(BlockId, BlockId), (int4, bool)>,
    ) -> KunaResult<BlockId> {
        let rootbl = cs[0];
        // const FlowBlock *leafbl = rootbl->getExitLeaf();
        let leafbl = self
            .get_exit_leaf(rootbl)
            .ok_or_else(|| KunaError::lowlevel("Could not get switch leaf"))?;
        if self.arena[leafbl].get_type() != BlockType::Copy {
            return Err(KunaError::lowlevel("Could not get switch leaf"));
        }
        let ret = self.arena.insert(FlowBlock::new_kind(BlockKind::Switch {
            caseblocks: Vec::new(),
            jt_index,
        }));
        // uret->grabCaseBasic(leafbl->subBlock(0), cs);  -- the switch underlying
        // basic block is `leafbl->subBlock(0)` (the bblocks BlockBasic the copy
        // mirrors).  The case-topology (in/out indices) is computed against the
        // switch ROOT `rootbl` (its sblocks out-edges mirror the bblocks block's,
        // edge labels and all), so we pass `rootbl` as the topology anchor.
        let caseblocks = self.grab_case_basic(ret, rootbl, cs, switch_case_edges);
        if let BlockKind::Switch { caseblocks: cb, .. } = &mut self.arena[ret].kind {
            *cb = caseblocks;
        }
        // identifyInternal(uret.get(), cs);
        self.identify_internal(graph_id, ret, cs);
        // addBlock(ret);
        self.add_block(graph_id, ret);
        if has_exit {
            self.force_output_num(ret, 1); // exactly 1 out edge if there is an exit
        }
        self.arena[ret].clear_flag(block_flags::f_switch_out); // not a switch "out"
        Ok(ret)
    }

    /// Build the annotated [`CaseOrder`] descriptions for a switch (C++
    /// `BlockSwitch::grabCaseBasic`, `block.cc:3575`).
    ///
    /// `switch_root` is the switch component (`cs[0]`); its sblocks out-edges
    /// mirror the underlying bblocks switch block's (including the
    /// `f_defaultswitch_edge` label, copied by `build_copy_from`), so the
    /// in/out-index bookkeeping is computed here in sblocks-land.  `basicblock` is
    /// recorded as the case front-leaf's `copy` (the bblocks BlockBasic), which
    /// the printer's `finalize` step feeds to `JumpTable::numIndicesByBlock`.
    fn grab_case_basic(
        &mut self,
        switch_id: BlockId,
        switch_root: BlockId,
        cs: &[BlockId],
        switch_case_edges: &std::collections::BTreeMap<(BlockId, BlockId), (int4, bool)>,
    ) -> Vec<CaseOrder> {
        let mut caseblocks: Vec<CaseOrder> = Vec::new();
        // casemap from outindex to caseblocks position (sizeOut of the switch).
        let sizeout = self.arena[switch_root].size_out();
        let mut casemap: Vec<int4> = vec![-1; sizeout as usize];
        for &casebl in cs.iter().skip(1) {
            let curcase = self.make_case(switch_root, casebl, 0, switch_case_edges);
            let outindex = curcase.outindex;
            caseblocks.push(curcase);
            if (outindex as usize) < casemap.len() {
                casemap[outindex as usize] = caseblocks.len() as int4 - 1;
            }
        }
        // Fillin fall-thru chaining: a t_goto case targeting another case chains.
        for i in 0..caseblocks.len() {
            let casebl = caseblocks[i].block;
            if self.arena[casebl].get_type() == BlockType::Goto {
                if let Some(targetbl) = self.arena[casebl].get_goto_target() {
                    if let Some(front) = self.get_front_leaf(targetbl) {
                        if let Some(basicbl) = self.sub_block(front, 0) {
                            // inindex of the switch into the target's front leaf.
                            let inindex = self.arena[front].get_in_index(switch_root);
                            if inindex != -1 {
                                let oidx = self.arena[front].get_in_rev_index(inindex);
                                if (oidx as usize) < casemap.len() {
                                    caseblocks[i].chain = casemap[oidx as usize];
                                }
                            }
                            let _ = basicbl;
                        }
                    }
                }
            }
        }
        // cs[0] t_multigoto: marked switch edges become f_goto_goto cases.
        if self.arena[switch_root].get_type() == BlockType::MultiGoto {
            let gotos: Vec<BlockId> = match &self.arena[switch_root].kind {
                BlockKind::MultiGoto { gotoedges, .. } => gotoedges.clone(),
                _ => Vec::new(),
            };
            for g in gotos {
                let c = self.make_case(switch_root, g, block_flags::f_goto_goto, switch_case_edges);
                caseblocks.push(c);
            }
        }
        let _ = switch_id;
        caseblocks
    }

    /// Build a single [`CaseOrder`] for `bl` (C++ `BlockSwitch::addCase`,
    /// `block.cc:3546`).  `gt` is the unstructured branch type (0 if structured).
    fn make_case(
        &self,
        switch_root: BlockId,
        bl: BlockId,
        gt: uint4,
        switch_case_edges: &std::collections::BTreeMap<(BlockId, BlockId), (int4, bool)>,
    ) -> CaseOrder {
        // const FlowBlock *basicbl = bl->getFrontLeaf()->subBlock(0);
        let front = self.get_front_leaf(bl);
        let basicblock = front.and_then(|f| self.sub_block(f, 0));
        // int4 inindex = basicbl->getInIndex(switchbl);
        // curcase.outindex = basicbl->getInRevIndex(inindex);
        // curcase.isdefault = switchbl->isDefaultBranch(curcase.outindex);
        //
        // C++ resolves this against the live `BlockBasic` (`bl->getFrontLeaf()->
        // subBlock(0)`), whose in/out edges to/from the underlying switch BlockBasic
        // are never severed by structuring.  In the dual-arena Rust port a case's
        // *sblocks* front-leaf can lose its in-edge once wrapped in a BlockGoto, so
        // resolve `(outindex, isdefault)` from the precomputed **bblocks** edge map
        // (keyed by the switch's and this case's underlying bblocks blocks).  The
        // switch root's bblocks block is its exit-leaf copy's `copy` pointer.
        let switch_bb = self.get_exit_leaf(switch_root).and_then(|leaf| self.arena[leaf].get_copy());
        let resolved = match (switch_bb, basicblock) {
            (Some(sbb), Some(cbb)) => switch_case_edges.get(&(sbb, cbb)).copied(),
            _ => None,
        };
        let (outindex, isdefault) = match resolved {
            Some((oi, isdef)) => (oi, isdef),
            None => {
                // Fall back to the sblocks topology (exact for un-wrapped leaf cases
                // and for hand-built unit-test graphs that have no bblocks map).
                match front {
                    Some(f) => {
                        let inindex = self.arena[f].get_in_index(switch_root);
                        if inindex == -1 {
                            (0, false)
                        } else {
                            let oi = self.arena[f].get_in_rev_index(inindex);
                            let isdef = self.arena[switch_root].is_default_branch(oi);
                            (oi, isdef)
                        }
                    }
                    None => (0, false),
                }
            }
        };
        let isexit = if gt != 0 { false } else { self.arena[bl].size_out() == 1 };
        CaseOrder {
            block: bl,
            basicblock,
            label: 0,
            depth: 0,
            chain: -1,
            outindex,
            gototype: gt,
            isexit,
            isdefault,
        }
    }

    /// Build a copy of a BlockGraph as BlockCopy nodes (C++ `BlockGraph::buildCopy`,
    /// `block.cc:1929`).  `graph_src` is the source BlockGraph node whose
    /// component list is mirrored into `graph_dst`.
    pub fn build_copy(&mut self, graph_dst: BlockId, graph_src: BlockId) {
        let startsize = self.arena[graph_dst].list.len();
        let srclist: Vec<BlockId> = self.arena[graph_src].list.clone();
        for it in srclist {
            let copyblock = self.new_block_copy(graph_dst, it);
            self.arena[it].copymap = Some(copyblock); // Store map basic->copy
        }
        let dstlist: Vec<BlockId> = self.arena[graph_dst].list[startsize..].to_vec();
        for it in dstlist {
            self.replace_using_map(it);
        }
    }

    /// Cross-arena variant of [`build_copy`](BlockGraph::build_copy) (C++
    /// `BlockGraph::buildCopy`, `block.cc:1929`).
    ///
    /// In C++ `bblocks` and `sblocks` are two `BlockGraph` objects living in the
    /// same address space, so a `BlockCopy` node created in `sblocks` simply
    /// stores a `FlowBlock *` back into a `bblocks` node (`copymap`), and the
    /// edge-`point` remap walks both graphs through raw pointers.  The Rust port
    /// keeps each `BlockGraph` in its **own** slotmap arena (ADR 0001), so a
    /// `bblocks` [`BlockId`] is not a key in the `sblocks` arena.  This method
    /// therefore takes the source graph by reference and threads the src→dst
    /// identity map explicitly:
    ///
    ///   * each source component (a `BlockBasic`) becomes a fresh `BlockCopy` in
    ///     `self` (the dst, i.e. `sblocks`) whose `copy` field holds the **source**
    ///     [`BlockId`] (the `bblocks` block — the printer resolves it against
    ///     `bblocks` to walk the op list), with the source block's
    ///     edge-topology / `index` / `numdesc` / `flags` / `immed_dom` copied in
    ///     verbatim (still keyed by *source* ids);
    ///   * `replace_using_map` then rewrites every edge `point` and `immed_dom`
    ///     from the source id to the corresponding dst `BlockCopy` id using the
    ///     map — exactly the second pass of C++ `buildCopy`.
    ///
    /// `graph_dst` is `self`'s root graph node; `src` is the source [`BlockGraph`]
    /// (e.g. `bblocks`) and `graph_src` its root graph node.
    ///
    /// Returns the source-`bblocks`-id -> destination-`sblocks`-`BlockCopy`-id map
    /// so the caller can write each source block's `copymap` field (the C++
    /// `(*iter)->copymap = copyblock;` write-back at `block.cc:1938`, which a
    /// cross-arena port cannot do directly since `src` is borrowed immutably).
    pub fn build_copy_from(
        &mut self,
        graph_dst: BlockId,
        src: &BlockGraph,
        graph_src: BlockId,
    ) -> std::collections::BTreeMap<BlockId, BlockId> {
        let startsize = self.arena[graph_dst].list.len();
        let srclist: Vec<BlockId> = src.arena[graph_src].list.clone();
        // src block id -> dst BlockCopy id (the C++ `bl->copymap`).
        let mut copymap: std::collections::BTreeMap<BlockId, BlockId> =
            std::collections::BTreeMap::new();
        for it in &srclist {
            let copyblock = self.new_block_copy_from(graph_dst, src, *it);
            copymap.insert(*it, copyblock);
        }
        // Second pass: remap each new node's edge points + idom through the map
        // (cross-arena `replace_using_map`).
        let dstlist: Vec<BlockId> = self.arena[graph_dst].list[startsize..].to_vec();
        for it in dstlist {
            let n_in = self.arena[it].intothis.len();
            for i in 0..n_in {
                let p = self.arena[it].intothis[i].point.expect("build_copy_from in");
                self.arena[it].intothis[i].point = copymap.get(&p).copied();
            }
            let n_out = self.arena[it].outofthis.len();
            for i in 0..n_out {
                let p = self.arena[it].outofthis[i].point.expect("build_copy_from out");
                self.arena[it].outofthis[i].point = copymap.get(&p).copied();
            }
            if let Some(idom) = self.arena[it].immed_dom {
                self.arena[it].immed_dom = copymap.get(&idom).copied();
            }
        }
        copymap
    }

    /// Set the `BlockCopy` a block maps to (C++ `FlowBlock::copymap`), used by the
    /// cross-arena `buildCopy` write-back (`block.cc:1938`).
    pub fn set_copy_map(&mut self, this_id: BlockId, copyblock: Option<BlockId>) {
        self.arena[this_id].copymap = copyblock;
    }

    /// Cross-arena variant of [`new_block_copy`](BlockGraph::new_block_copy) (C++
    /// `BlockGraph::newBlockCopy`, `block.cc:1684`).  Reads the source block's
    /// base fields out of `src` (a different arena) and builds a `BlockCopy` in
    /// `self` whose `copy` field is the **source** [`BlockId`].  The copied edge
    /// points still reference *source* ids until `build_copy_from`'s second pass
    /// remaps them.
    fn new_block_copy_from(&mut self, graph_id: BlockId, src: &BlockGraph, bl: BlockId) -> BlockId {
        let intothis = src.arena[bl].intothis.clone();
        let outofthis = src.arena[bl].outofthis.clone();
        let immed_dom = src.arena[bl].immed_dom;
        let index = src.arena[bl].index;
        let numdesc = src.arena[bl].numdesc;
        let blflags = src.arena[bl].flags;
        let mut ret = FlowBlock::new_kind(BlockKind::Copy { copy: Some(bl) });
        ret.intothis = intothis;
        ret.outofthis = outofthis;
        ret.immed_dom = immed_dom;
        ret.index = index;
        // visitcount stays 0 (FlowBlock ctor), per C++ comment
        ret.numdesc = numdesc;
        ret.flags |= blflags;
        if ret.outofthis.len() > 2 {
            ret.flags |= block_flags::f_switch_out;
        }
        let ret_id = self.arena.insert(ret);
        self.add_block(graph_id, ret_id);
        ret_id
    }

    /// Clear the visit count in all node FlowBlocks
    /// (C++ `BlockGraph::clearVisitCount`, `block.cc:1944`).
    pub fn clear_visit_count(&mut self, graph_id: BlockId) {
        let list: Vec<BlockId> = self.arena[graph_id].list.clone();
        for bl in list {
            self.arena[bl].visitcount = 0;
        }
    }

    // ----------------------------------------------------------------------
    // Spanning tree / irreducibility (Tarjan), block.cc
    // ----------------------------------------------------------------------

    /// Find a spanning tree skipping irreducible edges (C++
    /// `BlockGraph::findSpanningTree`, `block.cc:1009`).  Labels pre/reverse-post
    /// orderings, tree/forward/cross/back edges, descendant counts, and reorders
    /// `list` into reverse-post order.
    pub fn find_spanning_tree(
        &mut self,
        graph_id: BlockId,
        preorder: &mut Vec<BlockId>,
        rootlist: &mut Vec<BlockId>,
    ) {
        let listlen = self.arena[graph_id].list.len();
        if listlen == 0 {
            return;
        }
        let mut rpostorder: Vec<Option<BlockId>> = vec![None; listlen];
        let mut state: Vec<BlockId> = Vec::new();
        let mut istate: Vec<int4> = Vec::new();

        let list: Vec<BlockId> = self.arena[graph_id].list.clone();
        for &tmpbl in &list {
            self.arena[tmpbl].index = -1;
            self.arena[tmpbl].visitcount = -1;
            self.arena[tmpbl].copymap = Some(tmpbl);
            if self.arena[tmpbl].size_in() == 0 {
                rootlist.push(tmpbl);
            }
        }
        if rootlist.len() > 1 {
            // Make sure orighead is visited last (so it is first in reverse post order)
            let lastpos = rootlist.len() - 1;
            rootlist.swap(lastpos, 0);
        } else if rootlist.is_empty() {
            rootlist.push(list[0]);
        }
        let origrootpos = rootlist.len() - 1;

        let mut repeat = 0;
        loop {
            let mut extraroots = false;
            let mut rpostcount = listlen as int4;
            let mut rootindex = 0usize;
            self.clear_edge_flags(
                graph_id,
                edge_flags::f_irreducible
                    | edge_flags::f_tree_edge
                    | edge_flags::f_forward_edge
                    | edge_flags::f_cross_edge
                    | edge_flags::f_back_edge
                    | edge_flags::f_loop_edge
                    | edge_flags::f_loop_exit_edge,
            );
            while preorder.len() < listlen {
                let mut startbl: Option<BlockId> = None;
                while rootindex < rootlist.len() {
                    let cand = rootlist[rootindex];
                    rootindex += 1;
                    if self.arena[cand].visitcount == -1 {
                        startbl = Some(cand);
                        break;
                    }
                    // cand isn't really a root (from previous pass); remove it
                    for i in rootindex..rootlist.len() {
                        rootlist[i - 1] = rootlist[i];
                    }
                    rootlist.pop();
                    rootindex -= 1;
                    startbl = None;
                }
                let startbl = match startbl {
                    Some(b) => b,
                    None => {
                        extraroots = true;
                        let mut found = list[0];
                        for &cand in &list {
                            if self.arena[cand].visitcount == -1 {
                                found = cand;
                                break;
                            }
                        }
                        rootlist.push(found);
                        rootindex += 1;
                        found
                    }
                };

                state.push(startbl);
                istate.push(0);
                self.arena[startbl].visitcount = preorder.len() as int4;
                preorder.push(startbl);
                self.arena[startbl].numdesc = 1;

                while let Some(&curbl) = state.last() {
                    let cur_sizeout = self.arena[curbl].size_out();
                    if cur_sizeout <= *istate.last().unwrap() {
                        // Visited all children
                        state.pop();
                        istate.pop();
                        rpostcount -= 1;
                        self.arena[curbl].index = rpostcount;
                        rpostorder[rpostcount as usize] = Some(curbl);
                        if let Some(&back) = state.last() {
                            let nd = self.arena[curbl].numdesc;
                            self.arena[back].numdesc += nd;
                        }
                    } else {
                        let edgenum = *istate.last().unwrap();
                        *istate.last_mut().unwrap() += 1;
                        if self.arena[curbl].is_irreducible_out(edgenum) {
                            continue; // Pretend irreducible edges don't exist
                        }
                        let childbl = self.arena[curbl].get_out(edgenum);
                        if self.arena[childbl].visitcount == -1 {
                            self.set_out_edge_flag(curbl, edgenum, edge_flags::f_tree_edge);
                            state.push(childbl);
                            istate.push(0);
                            self.arena[childbl].visitcount = preorder.len() as int4;
                            preorder.push(childbl);
                            self.arena[childbl].numdesc = 1;
                        } else if self.arena[childbl].index == -1 {
                            // childbl is already on stack
                            self.set_out_edge_flag(
                                curbl,
                                edgenum,
                                edge_flags::f_back_edge | edge_flags::f_loop_edge,
                            );
                        } else if self.arena[curbl].visitcount < self.arena[childbl].visitcount {
                            self.set_out_edge_flag(curbl, edgenum, edge_flags::f_forward_edge);
                        } else {
                            self.set_out_edge_flag(curbl, edgenum, edge_flags::f_cross_edge);
                        }
                    }
                }
            }
            if !extraroots {
                break;
            }
            if repeat == 1 {
                // Mirrors the C++ throw; surfaces as a panic on internal-invariant
                // violation (ADR 0004) since findSpanningTree has no Result.
                panic!("Could not generate spanning tree");
            }

            // Regenerate post order so entry block comes first
            let lastpos = rootlist.len() - 1;
            rootlist.swap(lastpos, origrootpos);

            for &tmpbl in &list {
                self.arena[tmpbl].index = -1;
                self.arena[tmpbl].visitcount = -1;
                self.arena[tmpbl].copymap = Some(tmpbl);
            }
            preorder.clear();
            state.clear();
            istate.clear();
            repeat += 1;
        }

        if rootlist.len() > 1 {
            let lastpos = rootlist.len() - 1;
            rootlist.swap(lastpos, 0);
        }

        let rpost: Vec<BlockId> =
            rpostorder.into_iter().map(|x| x.expect("rpostorder hole")).collect();
        self.arena[graph_id].list = rpost;
    }

    /// Identify irreducible edges (C++ `BlockGraph::findIrreducible`,
    /// `block.cc:1147`).  Returns `true` if the spanning tree needs rebuilding.
    pub fn find_irreducible(
        &mut self,
        preorder: &[BlockId],
        irreduciblecount: &mut int4,
    ) -> bool {
        let mut reachunder: Vec<BlockId> = Vec::new();
        let mut needrebuild = false;
        let mut xi = preorder.len() as int4 - 1;
        while xi >= 0 {
            let x = preorder[xi as usize];
            xi -= 1;
            let sizein = self.arena[x].size_in();
            for i in 0..sizein {
                if !self.arena[x].is_back_edge_in(i) {
                    continue;
                }
                let y = self.arena[x].get_in(i);
                if y == x {
                    continue; // Reachunder set does not include the loop head
                }
                let ycopy = self.arena[y].copymap.expect("findIrreducible: FIND(y)");
                reachunder.push(ycopy);
                self.arena[ycopy].set_mark();
            }
            let mut q = 0;
            while q < reachunder.len() {
                let t = reachunder[q];
                q += 1;
                let sizein_t = self.arena[t].size_in();
                for i in 0..sizein_t {
                    if self.arena[t].is_irreducible_in(i) {
                        continue;
                    }
                    let y = self.arena[t].get_in(i);
                    let yprime = self.arena[y].copymap.expect("findIrreducible: y'");
                    let x_vc = self.arena[x].visitcount;
                    let x_nd = self.arena[x].numdesc;
                    let yp_vc = self.arena[yprime].visitcount;
                    if (x_vc > yp_vc) || (x_vc + x_nd <= yp_vc) {
                        *irreduciblecount += 1;
                        let edgeout = self.arena[t].get_in_rev_index(i);
                        self.set_out_edge_flag(y, edgeout, edge_flags::f_irreducible);
                        if self.arena[t].is_tree_edge_in(i) {
                            needrebuild = true;
                        } else {
                            self.clear_out_edge_flag(
                                y,
                                edgeout,
                                edge_flags::f_cross_edge | edge_flags::f_forward_edge,
                            );
                        }
                    } else if !self.arena[yprime].is_mark() && yprime != x {
                        reachunder.push(yprime);
                        self.arena[yprime].set_mark();
                    }
                }
            }
            // Collapse reachunder into x
            for &s in &reachunder {
                self.arena[s].clear_mark();
                self.arena[s].copymap = Some(x);
            }
            reachunder.clear();
        }
        needrebuild
    }

    /// Look for directed cycles, marking loop edges (C++ `BlockGraph::calcLoop`,
    /// `block.cc:2108`).  Only used as a failsafe for irreducible graphs.
    pub fn calc_loop(&mut self, graph_id: BlockId) {
        if self.arena[graph_id].list.is_empty() {
            return;
        }
        let front = self.arena[graph_id].list[0];
        let mut path: Vec<BlockId> = Vec::new();
        let mut state: Vec<int4> = Vec::new();
        path.push(front);
        state.push(0);
        self.arena[front].set_flag(block_flags::f_mark | block_flags::f_mark2);
        while let Some(&bl) = path.last() {
            let i = *state.last().unwrap();
            if i >= self.arena[bl].size_out() {
                self.arena[bl].clear_flag(block_flags::f_mark2);
                path.pop();
                state.pop();
            } else {
                *state.last_mut().unwrap() += 1;
                if self.arena[bl].is_loop_out(i) {
                    continue;
                }
                let nextbl = self.arena[bl].get_out(i);
                if (self.arena[nextbl].flags & block_flags::f_mark2) != 0 {
                    // We found a cycle!
                    self.add_loop_edge(bl, i);
                } else if (self.arena[nextbl].flags & block_flags::f_mark) == 0 {
                    self.arena[nextbl].set_flag(block_flags::f_mark | block_flags::f_mark2);
                    path.push(nextbl);
                    state.push(0);
                }
            }
        }
        let list: Vec<BlockId> = self.arena[graph_id].list.clone();
        for bl in list {
            self.arena[bl].clear_flag(block_flags::f_mark | block_flags::f_mark2);
        }
    }

    /// Collect reachable / unreachable FlowBlocks (C++ `BlockGraph::collectReachable`,
    /// `block.cc:2158`).
    pub fn collect_reachable(
        &mut self,
        graph_id: BlockId,
        res: &mut Vec<BlockId>,
        bl: BlockId,
        un: bool,
    ) {
        self.arena[bl].set_mark();
        res.push(bl);
        let mut total = 0;
        while total < res.len() {
            let blk = res[total];
            total += 1;
            for j in 0..self.arena[blk].size_out() {
                let blk2 = self.arena[blk].get_out(j);
                if self.arena[blk2].is_mark() {
                    continue;
                }
                self.arena[blk2].set_mark();
                res.push(blk2);
            }
        }
        if un {
            res.clear();
            let list: Vec<BlockId> = self.arena[graph_id].list.clone();
            for blk in list {
                if self.arena[blk].is_mark() {
                    self.arena[blk].clear_mark();
                } else {
                    res.push(blk);
                }
            }
        } else {
            for &b in res.iter() {
                self.arena[b].clear_mark();
            }
        }
    }

    /// Label loop edges: find irreducible edges + spanning tree + reverse-post
    /// indices + edge labels (C++ `BlockGraph::structureLoops`, `block.cc:2198`).
    pub fn structure_loops(&mut self, graph_id: BlockId, rootlist: &mut Vec<BlockId>) {
        let mut preorder: Vec<BlockId> = Vec::new();
        let mut irreduciblecount = 0;
        loop {
            self.find_spanning_tree(graph_id, &mut preorder, rootlist);
            let needrebuild = self.find_irreducible(&preorder, &mut irreduciblecount);
            if needrebuild {
                self.clear_edge_flags(
                    graph_id,
                    edge_flags::f_tree_edge
                        | edge_flags::f_forward_edge
                        | edge_flags::f_cross_edge
                        | edge_flags::f_back_edge
                        | edge_flags::f_loop_edge,
                );
                preorder.clear();
                rootlist.clear();
            }
            if !needrebuild {
                break;
            }
        }
        if irreduciblecount > 0 {
            self.calc_loop(graph_id);
        }
    }

    // ----------------------------------------------------------------------
    // Dominator algorithm (Cooper/Harvey/Kennedy), block.cc
    // ----------------------------------------------------------------------

    /// Calculate the immediate dominator for each FlowBlock node for forward
    /// control-flow (C++ `BlockGraph::calcForwardDominator`, `block.cc:1958`).
    /// Assumes the blocks are in reverse post-order (reflected in `index`).
    pub fn calc_forward_dominator(&mut self, graph_id: BlockId, rootlist: &[BlockId]) {
        let listlen = self.arena[graph_id].list.len();
        if listlen == 0 {
            return;
        }
        let numnodes = listlen as int4 - 1;
        // postorder[numnodes - i] = list[i]; postorder has length listlen, plus
        // possibly a pushed-back virtualroot.
        let mut postorder: Vec<BlockId> = vec![BlockId::default(); listlen];
        let list: Vec<BlockId> = self.arena[graph_id].list.clone();
        for i in 0..listlen {
            self.arena[list[i]].immed_dom = None;
            postorder[(numnodes - i as int4) as usize] = list[i];
        }
        let mut virtualroot: Option<BlockId> = None;
        if rootlist.len() > 1 {
            let vr = self.create_virtual_root(rootlist);
            virtualroot = Some(vr);
            postorder.push(vr);
        }

        let mut b = *postorder.last().unwrap(); // The official start node
        if self.arena[b].size_in() != 0 {
            if rootlist.len() != 1 || rootlist[0] != b {
                panic!("Problems finding root node of graph");
            }
            let vr = self.create_virtual_root(rootlist);
            virtualroot = Some(vr);
            postorder.push(vr);
            b = vr;
        }
        self.arena[b].immed_dom = Some(b);
        for i in 0..self.arena[b].size_out() {
            let o = self.arena[b].get_out(i);
            self.arena[o].immed_dom = Some(b);
        }
        let mut changed = true;
        while changed {
            changed = false;
            // for(i=postorder.size()-2; i>=0; --i)
            let mut i = postorder.len() as int4 - 2;
            while i >= 0 {
                let bb = postorder[i as usize];
                let last = *postorder.last().unwrap();
                if self.arena[bb].immed_dom != Some(last) {
                    // Find first processed in-edge
                    let mut new_idom: Option<BlockId> = None;
                    let mut j = 0;
                    while j < self.arena[bb].size_in() {
                        let cand = self.arena[bb].get_in(j);
                        if self.arena[cand].immed_dom.is_some() {
                            new_idom = Some(cand);
                            break;
                        }
                        j += 1;
                    }
                    j += 1;
                    let mut new_idom = new_idom.expect("calcForwardDominator: no processed in-edge");
                    while j < self.arena[bb].size_in() {
                        let rho = self.arena[bb].get_in(j);
                        if self.arena[rho].immed_dom.is_some() {
                            // intersection routine
                            let mut finger1 = numnodes - self.arena[rho].index;
                            let mut finger2 = numnodes - self.arena[new_idom].index;
                            while finger1 != finger2 {
                                while finger1 < finger2 {
                                    let idom = self.arena[postorder[finger1 as usize]]
                                        .immed_dom
                                        .expect("intersect: idom1");
                                    finger1 = numnodes - self.arena[idom].index;
                                }
                                while finger2 < finger1 {
                                    let idom = self.arena[postorder[finger2 as usize]]
                                        .immed_dom
                                        .expect("intersect: idom2");
                                    finger2 = numnodes - self.arena[idom].index;
                                }
                            }
                            new_idom = postorder[finger1 as usize];
                        }
                        j += 1;
                    }
                    if self.arena[bb].immed_dom != Some(new_idom) {
                        self.arena[bb].immed_dom = Some(new_idom);
                        changed = true;
                    }
                }
                i -= 1;
            }
        }
        if let Some(vr) = virtualroot {
            for i in 0..listlen {
                if self.arena[postorder[i]].immed_dom == Some(vr) {
                    self.arena[postorder[i]].immed_dom = None;
                }
            }
            while self.arena[vr].size_out() > 0 {
                let last = self.arena[vr].size_out() - 1;
                self.remove_out_edge(vr, last);
            }
            self.arena.remove(vr);
        } else {
            let last = *postorder.last().unwrap();
            self.arena[last].immed_dom = None;
        }
    }

    /// Build the dominator tree as a child-list indexed by FlowBlock index
    /// (C++ `BlockGraph::buildDomTree`, `block.cc:2040`).
    pub fn build_dom_tree(&self, graph_id: BlockId) -> Vec<Vec<BlockId>> {
        let listlen = self.arena[graph_id].list.len();
        let mut child: Vec<Vec<BlockId>> = vec![Vec::new(); listlen + 1];
        for i in 0..listlen {
            let bl = self.arena[graph_id].list[i];
            match self.arena[bl].immed_dom {
                Some(idom) => {
                    let idx = self.arena[idom].index as usize;
                    child[idx].push(bl);
                }
                None => child[listlen].push(bl),
            }
        }
        child
    }

    /// Calculate dominator depths, returning the maximum
    /// (C++ `BlockGraph::buildDomDepth`, `block.cc:2060`).
    pub fn build_dom_depth(&self, graph_id: BlockId) -> (Vec<int4>, int4) {
        let listlen = self.arena[graph_id].list.len();
        let mut depth: Vec<int4> = vec![0; listlen + 1];
        let mut max = 0;
        for i in 0..listlen {
            let bl = self.arena[graph_id].list[i];
            match self.arena[bl].immed_dom {
                Some(idom) => {
                    depth[i] = depth[self.arena[idom].get_index() as usize] + 1;
                }
                None => depth[i] = 1,
            }
            if max < depth[i] {
                max = depth[i];
            }
        }
        depth[listlen] = 0;
        (depth, max)
    }

    /// Collect nodes from a dominator sub-tree (C++ `BlockGraph::buildDomSubTree`,
    /// `block.cc:2084`).  Assumes blocks are in reverse post order.
    pub fn build_dom_sub_tree(&self, graph_id: BlockId, root: BlockId) -> Vec<BlockId> {
        let mut res: Vec<BlockId> = Vec::new();
        let rootindex = self.arena[root].get_index();
        res.push(root);
        let listlen = self.arena[graph_id].list.len() as int4;
        let mut i = rootindex + 1;
        while i < listlen {
            let bl = self.arena[graph_id].list[i as usize];
            match self.arena[bl].get_immed_dom() {
                None => break,
                Some(dombl) => {
                    if self.arena[dombl].get_index() > rootindex {
                        break;
                    }
                    res.push(bl);
                }
            }
            i += 1;
        }
        res
    }

    // ----------------------------------------------------------------------
    // encode / decode (FlowBlock + BlockGraph), block.cc
    // ----------------------------------------------------------------------

    /// Encode basic info as attributes (C++ `FlowBlock::encodeHeader` +
    /// the BlockCopy/BlockCondition overrides, `block.cc:2474`/`2896`/`3107`).
    fn encode_header(&self, this_id: BlockId, encoder: &mut dyn Encoder) {
        encoder.write_signed_integer(&ATTRIB_INDEX, self.arena[this_id].index as i64);
        match &self.arena[this_id].kind {
            BlockKind::Copy { copy } => {
                // BlockCopy::encodeHeader: altindex = copy->getIndex()
                let altindex = copy.map(|c| self.arena[c].get_index()).unwrap_or(0);
                encoder.write_signed_integer(&ATTRIB_ALTINDEX, altindex as i64);
            }
            BlockKind::Condition { opc } => {
                // BlockCondition::encodeHeader: opcode name
                encoder.write_string(&ATTRIB_OPCODE, get_opname(*opc).as_bytes());
            }
            _ => {}
        }
    }

    /// Decode basic info from element attributes (C++ `FlowBlock::decodeHeader`,
    /// `block.cc:2481`).
    fn decode_header(&mut self, this_id: BlockId, decoder: &mut dyn Decoder) -> KunaResult<()> {
        self.arena[this_id].index = decoder.read_signed_integer_id(&ATTRIB_INDEX)? as int4;
        Ok(())
    }

    /// Encode the \<bhead> list + recursive bodies of a BlockGraph
    /// (C++ `BlockGraph::encodeBody`, `block.cc:1377`).  Leaf blocks have empty
    /// bodies except BlockBasic, which writes its `cover`.
    fn encode_body(&self, this_id: BlockId, encoder: &mut dyn Encoder) {
        let bt = self.arena[this_id].get_type();
        if let BlockKind::Basic(bd) = &self.arena[this_id].kind {
            // BlockBasic::encodeBody: cover.encode(encoder)
            bd.cover.encode(encoder);
            return;
        }
        if !is_graph_subtype(bt) {
            // FlowBlock::encodeBody (base) is empty (Plain/Copy)
            return;
        }
        // BlockGraph and subtypes: bhead headers then recursive encode
        let list = self.arena[this_id].list.clone();
        for &bl in &list {
            encoder.open_element(&ELEM_BHEAD);
            encoder.write_signed_integer(&ATTRIB_INDEX, self.arena[bl].get_index() as i64);
            let nm = self.body_type_name(bl);
            encoder.write_string(&ATTRIB_TYPE, nm.as_bytes());
            encoder.close_element(&ELEM_BHEAD);
        }
        for &bl in &list {
            self.encode(bl, encoder);
        }
        // Subtype-specific trailing <target> elements
        self.encode_body_extra(this_id, encoder);
    }

    /// The `<bhead>` type name for a component (C++ `BlockGraph::encodeBody`'s
    /// inner naming, with the BlockIf `ifgoto`/`properif`/`ifelse` distinction).
    fn body_type_name(&self, bl: BlockId) -> String {
        let bt = self.arena[bl].get_type();
        if bt == BlockType::If {
            let sz = self.arena[bl].get_size();
            if sz == 1 {
                "ifgoto".to_string()
            } else if sz == 2 {
                "properif".to_string()
            } else {
                "ifelse".to_string()
            }
        } else {
            type_to_name(bt).to_string()
        }
    }

    /// Subtype-specific trailing elements in `encodeBody` (the `<target>`
    /// elements of BlockGoto/BlockMultiGoto/BlockIf-goto), block.cc:2953/2986/3185.
    fn encode_body_extra(&self, this_id: BlockId, encoder: &mut dyn Encoder) {
        match &self.arena[this_id].kind {
            BlockKind::Goto { gototarget, gototype } => {
                let gt = gototarget.expect("BlockGoto::encodeBody: no target");
                let leaf = self.get_front_leaf(gt).expect("BlockGoto leaf");
                let depth = self.calc_depth(gt, Some(leaf));
                encoder.open_element(&ELEM_TARGET);
                encoder.write_signed_integer(&ATTRIB_INDEX, self.arena[leaf].get_index() as i64);
                encoder.write_signed_integer(&ATTRIB_DEPTH, depth as i64);
                encoder.write_unsigned_integer(&ATTRIB_TYPE, *gototype as u64);
                encoder.close_element(&ELEM_TARGET);
            }
            BlockKind::MultiGoto { gotoedges, .. } => {
                let edges = gotoedges.clone();
                for gt in edges {
                    let leaf = self.get_front_leaf(gt).expect("BlockMultiGoto leaf");
                    let depth = self.calc_depth(gt, Some(leaf));
                    encoder.open_element(&ELEM_TARGET);
                    encoder
                        .write_signed_integer(&ATTRIB_INDEX, self.arena[leaf].get_index() as i64);
                    encoder.write_signed_integer(&ATTRIB_DEPTH, depth as i64);
                    encoder.close_element(&ELEM_TARGET);
                }
            }
            BlockKind::If { gototype, gototarget } => {
                // BlockIf::encodeBody: only the ifgoto case (getSize()==1)
                if self.arena[this_id].get_size() == 1 {
                    let gt = gototarget.expect("BlockIf::encodeBody: no target");
                    let leaf = self.get_front_leaf(gt).expect("BlockIf leaf");
                    let depth = self.calc_depth(gt, Some(leaf));
                    encoder.open_element(&ELEM_TARGET);
                    encoder
                        .write_signed_integer(&ATTRIB_INDEX, self.arena[leaf].get_index() as i64);
                    encoder.write_signed_integer(&ATTRIB_DEPTH, depth as i64);
                    encoder.write_unsigned_integer(&ATTRIB_TYPE, *gototype as u64);
                    encoder.close_element(&ELEM_TARGET);
                }
            }
            _ => {}
        }
    }

    /// Encode edge information (C++ `FlowBlock::encodeEdges`, `block.cc:2489`).
    fn encode_edges(&self, this_id: BlockId, encoder: &mut dyn Encoder) {
        for i in 0..self.arena[this_id].intothis.len() {
            self.arena[this_id].intothis[i].encode(encoder, &self.arena);
        }
    }

    /// Encode \b this and all sub-components as a \<block> element
    /// (C++ `FlowBlock::encode`, `block.cc:2514`).
    pub fn encode(&self, this_id: BlockId, encoder: &mut dyn Encoder) {
        encoder.open_element(&ELEM_BLOCK);
        self.encode_header(this_id, encoder);
        self.encode_body(this_id, encoder);
        self.encode_edges(this_id, encoder);
        encoder.close_element(&ELEM_BLOCK);
    }

    /// Decode the next \<edge> element into this block's in-edge list
    /// (C++ `FlowBlock::decodeNextInEdge`, `block.cc:85`).
    fn decode_next_in_edge(
        &mut self,
        this_id: BlockId,
        decoder: &mut dyn Decoder,
        resolver: &BlockMap,
    ) -> KunaResult<()> {
        let mut inedge = BlockEdge::default();
        inedge.decode(decoder, resolver)?;
        let point = inedge.point.expect("decodeNextInEdge: resolved");
        let rev = inedge.reverse_index;
        // Ensure the other block's outofthis is long enough
        while (self.arena[point].outofthis.len() as int4) <= rev {
            self.arena[point].outofthis.push(BlockEdge::default());
        }
        self.arena[this_id].intothis.push(inedge);
        let new_rev = self.arena[this_id].intothis.len() as int4 - 1;
        let outedge = &mut self.arena[point].outofthis[rev as usize];
        outedge.label = 0;
        outedge.point = Some(this_id);
        outedge.reverse_index = new_rev;
        Ok(())
    }

    /// Restore edges from an encoded stream (C++ `FlowBlock::decodeEdges`,
    /// `block.cc:2501`).
    fn decode_edges(
        &mut self,
        this_id: BlockId,
        decoder: &mut dyn Decoder,
        resolver: &BlockMap,
    ) -> KunaResult<()> {
        loop {
            let sub_id = decoder.peek_element()?;
            if sub_id != ELEM_EDGE.get_id() {
                break;
            }
            self.decode_next_in_edge(this_id, decoder, resolver)?;
        }
        Ok(())
    }

    /// Decode the body of a block from a stream (C++ `FlowBlock::decodeBody`
    /// default / `BlockBasic::decodeBody` / `BlockGraph::decodeBody`,
    /// `block.cc:1405`/`2707`).
    fn decode_body(&mut self, this_id: BlockId, decoder: &mut dyn Decoder) -> KunaResult<()> {
        match &mut self.arena[this_id].kind {
            BlockKind::Basic(_) => {
                let mut cover = kuna_base::address::RangeList::new();
                cover.decode(decoder)?;
                if let BlockKind::Basic(bd) = &mut self.arena[this_id].kind {
                    bd.cover = cover;
                }
                Ok(())
            }
            BlockKind::Graph => self.decode_graph_body(this_id, decoder),
            // Other (structured) subtypes are not produced by the decode factory
            // (BlockMap::resolveBlock only builds plain/copy/graph); their bodies
            // are never decoded.  Plain/Copy have empty bodies.
            _ => Ok(()),
        }
    }

    /// Decode a BlockGraph body: the \<bhead> list then the recursive component
    /// blocks (C++ `BlockGraph::decodeBody`, `block.cc:1405`).
    fn decode_graph_body(&mut self, this_id: BlockId, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let mut newresolver = BlockMap::new();
        let mut tmplist: Vec<BlockId> = Vec::new();
        loop {
            let sub_id = decoder.peek_element()?;
            if sub_id != ELEM_BHEAD.get_id() {
                break;
            }
            decoder.open_element()?;
            let newindex = decoder.read_signed_integer_id(&ATTRIB_INDEX)? as int4;
            let name = decoder.read_string_id(&ATTRIB_TYPE)?;
            let bl = newresolver.create_block(&mut self.arena, &String::from_utf8_lossy(&name));
            self.arena[bl].index = newindex; // Need to set index here for sort
            tmplist.push(bl);
            decoder.close_element(sub_id)?;
        }
        newresolver.sort_list(&self.arena);

        for &bl in &tmplist {
            self.decode(bl, decoder, &newresolver)?;
            self.add_block(this_id, bl);
        }
        Ok(())
    }

    /// Recover \b this and all sub-components from a \<block> element
    /// (C++ `FlowBlock::decode`, `block.cc:2529`).
    pub fn decode(
        &mut self,
        this_id: BlockId,
        decoder: &mut dyn Decoder,
        resolver: &BlockMap,
    ) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_BLOCK)?;
        self.decode_header(this_id, decoder)?;
        self.decode_body(this_id, decoder)?;
        self.decode_edges(this_id, decoder, resolver)?;
        decoder.close_element(elem_id)?;
        Ok(())
    }

    /// Decode a top-level BlockGraph (C++ `BlockGraph::decode`, `block.cc:1432`).
    /// Creates the resolver and decodes the root graph node.
    pub fn decode_top(&mut self, this_id: BlockId, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let resolver = BlockMap::new();
        self.decode(this_id, decoder, &resolver)?;
        Ok(())
    }
}

/// Is the given block type a structured BlockGraph subtype (has a component
/// list)?  Used by `encode_body` to decide if there's a graph body to emit.
fn is_graph_subtype(bt: BlockType) -> bool {
    matches!(
        bt,
        BlockType::Graph
            | BlockType::Goto
            | BlockType::MultiGoto
            | BlockType::Ls
            | BlockType::Condition
            | BlockType::If
            | BlockType::WhileDo
            | BlockType::DoWhile
            | BlockType::InfLoop
            | BlockType::Switch
    )
}

// ---------------------------------------------------------------------------
// FlowBlock type-name <-> block_type (FlowBlock static, block.cc)
// ---------------------------------------------------------------------------

/// Get the block_type associated with a name string (C++ `FlowBlock::nameToType`,
/// `block.cc:657`).  Currently only used by decode (only graph/copy resolve to
/// non-plain).
pub fn name_to_type(nm: &str) -> BlockType {
    if nm == "graph" {
        BlockType::Graph
    } else if nm == "copy" {
        BlockType::Copy
    } else {
        BlockType::Plain
    }
}

/// Get the name string associated with a block_type (C++ `FlowBlock::typeToName`,
/// `block.cc:671`).
pub fn type_to_name(bt: BlockType) -> &'static str {
    match bt {
        BlockType::Plain => "plain",
        BlockType::Basic => "basic",
        BlockType::Graph => "graph",
        BlockType::Copy => "copy",
        BlockType::Goto => "goto",
        BlockType::MultiGoto => "multigoto",
        BlockType::Ls => "list",
        BlockType::Condition => "condition",
        BlockType::If => "properif",
        BlockType::WhileDo => "whiledo",
        BlockType::DoWhile => "dowhile",
        BlockType::Switch => "switch",
        BlockType::InfLoop => "infloop",
    }
}

/// Compare FlowBlock by index (C++ `FlowBlock::compareBlockIndex`,
/// `block.hh:911`).  `true` if the first comes before the second.
pub fn compare_block_index(arena: &BlockArena, bl1: BlockId, bl2: BlockId) -> bool {
    arena[bl1].get_index() < arena[bl2].get_index()
}

// ---------------------------------------------------------------------------
// BlockMap — resolver for deserialization (block.cc)
// ---------------------------------------------------------------------------

/// \brief Helper class for resolving cross-references while deserializing
/// BlockGraph objects (C++ `BlockMap`, `block.hh:823-836`).
///
/// Maintains a list of deserialized [`BlockId`]s sorted by index and looks up
/// the block matching a given index as edges specify them.
#[derive(Default)]
pub struct BlockMap {
    /// The list of deserialized FlowBlock objects (C++ `vector<FlowBlock*> sortlist`)
    sortlist: Vec<BlockId>,
    /// (id, index) cache captured at [`sort_list`](BlockMap::sort_list) time so
    /// [`find_level_block`](BlockMap::find_level_block) (called by
    /// [`BlockEdge::decode`], which holds only the resolver) can resolve indices
    /// without the arena.  Mirrors the C++ where `findLevelBlock` reads the
    /// `index` directly off the `FlowBlock *` in the sorted list.
    index_cache: Vec<(BlockId, int4)>,
}

impl BlockMap {
    /// Construct an empty resolver.
    pub fn new() -> BlockMap {
        BlockMap { sortlist: Vec::new(), index_cache: Vec::new() }
    }

    /// Construct a FlowBlock of the given type (C++ `BlockMap::resolveBlock`,
    /// `block.cc:3716`).  Only plain/copy/graph are ever produced by the decode
    /// factory.
    fn resolve_block(arena: &mut BlockArena, bt: BlockType) -> Option<BlockId> {
        match bt {
            BlockType::Plain => Some(arena.insert(FlowBlock::new())),
            BlockType::Copy => {
                Some(arena.insert(FlowBlock::new_kind(BlockKind::Copy { copy: None })))
            }
            BlockType::Graph => Some(arena.insert(FlowBlock::new_kind(BlockKind::Graph))),
            _ => None,
        }
    }

    /// Locate a FlowBlock with a given index via binary search over the
    /// (id, index) cache (C++ `BlockMap::findBlock`, `block.cc:3736`).  The cache
    /// is sorted by index at [`sort_list`](BlockMap::sort_list) time, exactly as
    /// the C++ `sortlist` is sorted before any `findBlock`.
    fn find_block(&self, ind: int4) -> Option<BlockId> {
        let mut min: int4 = 0;
        let mut max: int4 = self.index_cache.len() as int4 - 1;
        while min <= max {
            let mid = (min + max) / 2;
            let (block, bidx) = self.index_cache[mid as usize];
            if bidx == ind {
                return Some(block);
            }
            if bidx < ind {
                min = mid + 1;
            } else {
                max = mid - 1;
            }
        }
        None
    }

    /// Find the FlowBlock matching the given index (C++ `BlockMap::findLevelBlock`,
    /// `block.hh:834`).
    ///
    /// The C++ reads `getIndex()` directly off the `FlowBlock *`s in the sorted
    /// `sortlist`; here the same lookup runs over the arena-less (id, index)
    /// cache so [`BlockEdge::decode`] can stay C++-faithful (it holds only the
    /// `resolver`, not the arena).
    pub fn find_level_block(&self, index: int4) -> Option<BlockId> {
        self.find_block(index)
    }

    /// Sort the list of FlowBlock objects by index (C++ `BlockMap::sortList`,
    /// `block.cc:3755`).  Also captures the (id, index) cache used by
    /// [`find_level_block`](BlockMap::find_level_block).
    fn sort_list(&mut self, arena: &BlockArena) {
        self.sortlist.sort_by(|&a, &b| {
            if compare_block_index(arena, a, b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        self.index_cache =
            self.sortlist.iter().map(|&id| (id, arena[id].get_index())).collect();
    }

    /// Build a FlowBlock from a `<bhead>` name and register it
    /// (C++ `BlockMap::createBlock`, `block.cc:3765`).
    fn create_block(&mut self, arena: &mut BlockArena, name: &str) -> BlockId {
        let bt = name_to_type(name);
        let bl = BlockMap::resolve_block(arena, bt).expect("createBlock: unresolvable type");
        self.sortlist.push(bl);
        bl
    }
}

// ---------------------------------------------------------------------------
// op.rs seam surfaces (SEAM(W3-block) fulfilled)
// ---------------------------------------------------------------------------

/// Yield the `(shortcut_char, target_start_addr)` of a branch op's non-fallthru
/// target's start block, for [`crate::op::iop_space_print_raw`]'s `block_info`
/// closure (op.rs `// SEAM(W3-block)`, `op.cc:41-59`).
///
/// The C++ `IopSpace::printRaw` branch arm reads `bs = op->getParent();` then
/// `bl = bs->getOut(0); if (bl->getStart()==op->getAddr()) bl = bs->getOut(1);`
/// and prints `code_<shortcut><start-addr>`.  `parent_of` maps an op to its
/// parent block id; `arena`/`shortcut_of` provide the block graph + the
/// space-shortcut char.
///
/// SEAM(W3-block): the full wiring (op→parent, addr-of-op) lives in
/// `funcdata_op`, which owns both banks; this provides the block-side decode.
pub fn iop_block_info(
    arena: &BlockArena,
    parent: BlockId,
    op_addr: &Address,
    shortcut_of: &dyn Fn(BlockId) -> char,
) -> (char, Address) {
    // bl = bs->getOut(0); if (bl->getStart() == op->getAddr()) bl = bs->getOut(1);
    let mut bl = arena[parent].get_out(0);
    if block_get_start(arena, bl) == *op_addr {
        bl = arena[parent].get_out(1);
    }
    let shortcut = shortcut_of(bl);
    (shortcut, block_get_start(arena, bl))
}

/// Get the starting address of code in a block (C++ `FlowBlock::getStart`
/// virtual: invalid for non-basic, first cover range for basic, `block.cc:2323`).
pub fn block_get_start(arena: &BlockArena, this_id: BlockId) -> Address {
    if let BlockKind::Basic(bd) = &arena[this_id].kind {
        if let Some(range) = bd.cover.get_first_range() {
            return range.get_first_addr();
        }
    }
    Address::new_invalid()
}

/// Get the ending address of code in a block (C++ `FlowBlock::getStop`
/// virtual: invalid for non-basic, last cover range for basic, `block.cc:2332`).
pub fn block_get_stop(arena: &BlockArena, this_id: BlockId) -> Address {
    if let BlockKind::Basic(bd) = &arena[this_id].kind {
        if let Some(range) = bd.cover.get_last_range() {
            return range.get_last_addr();
        }
    }
    Address::new_invalid()
}

/// Compare the execution order of two ops (C++ `PcodeOp::compareOrder`,
/// `op.cc:808`).  Returns -1 if `op_a` executes earlier (dominates), 1 if `op_b`
/// does, 0 if there is no absolute order.
///
/// This is the block-graph-dependent half op.rs deferred (`// SEAM(W3-block)`).
/// `op_a`/`op_b` are the two ops (the caller resolves them from the op bank);
/// their parent block ids and instruction orders are passed alongside so the
/// op crate need not be borrowed mutably here.
pub fn pcodeop_compare_order(
    graph: &mut BlockGraph,
    op_a: &PcodeOp,
    parent_a: BlockId,
    op_b: &PcodeOp,
    parent_b: BlockId,
) -> int4 {
    if parent_a == parent_b {
        return if op_a.get_seq_num().get_order() < op_b.get_seq_num().get_order() {
            -1
        } else {
            1
        };
    }
    let common = graph.find_common_block(parent_a, parent_b);
    if common == Some(parent_a) {
        return -1;
    }
    if common == Some(parent_b) {
        return 1;
    }
    0
}

#[cfg(test)]
mod tests;
