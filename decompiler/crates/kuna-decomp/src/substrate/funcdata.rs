//! Port of `decompiler/cpp/funcdata.{hh,cc}` and the block-manipulation half
//! `funcdata_block.cc` (W3, item `w3-ir-funcdata`) — the [`Funcdata`] container
//! that owns the per-function IR (the [`VarnodeBank`], the [`PcodeOpBank`], and
//! the two [`BlockGraph`]s) and is the single API through which the graph is
//! mutated (ADR 0001).
//!
//! ## ADR 0001 (IR arenas) realization
//!
//! The C++ `Funcdata` *contains* `vbank`, `obank`, `bblocks`, `sblocks` by value
//! and every mutating helper (`op*`, `new*`, `block*`) routes through it.  Here
//! `Funcdata` owns those same containers (each of which owns its slotmap arena),
//! and **all** cross-arena mutation lives here — most importantly the
//! basic-block op-list manipulation (`opInsert`/`opUninsert`/`BlockBasic::insert`
//! / `removeOp` / `setOrder`), which in C++ is split between `Funcdata` and
//! `BlockBasic` but touches *both* the op arena (`obank`) and the block arena
//! (`bblocks`).  Rust cannot hold two `&mut` arenas through a method on one of
//! them, so the op-in-block primitives are [`Funcdata`] methods that reach into
//! both: [`Funcdata::bb_insert_op`], [`Funcdata::bb_remove_op`],
//! [`Funcdata::bb_set_order`].  The per-op basic-block membership links live on
//! the op (`set_basic_prev`/`set_basic_next`, the third intrusive list of
//! ADR 0001) and the per-block head/tail live in [`BasicData`].
//!
//! ## VarnodeBank callbacks (the seam `varnode.rs` documented)
//!
//! `VarnodeBank::xref`/`set_def`/`set_input`/`create_def` need two callbacks the
//! bank cannot supply itself (they reach the op graph):
//!   - `replace_reads(bank, old, new)` — when `xref` unifies a fresh varnode
//!     with an existing equivalent free varnode, every op reading `old` must be
//!     repointed to `new` (the C++ `Funcdata::totalReplace` driven inline);
//!   - `def_addr_time(op) -> (Address, uintm)` — `VarnodeBank::find` confirms a
//!     candidate's defining op's address/time.
//!
//! `Funcdata` owns both the bank and the op bank, so it constructs these
//! closures over `&mut obank` / `&obank` at each call site
//! ([`Funcdata::replace_reads_thunk`] and [`Funcdata::def_addr_time`]).
//!
//! ## Look-ahead pre-declarations (funcdata_op.cc / funcdata_varnode.cc)
//!
//! The `funcdata_op` (`w3-ir-funcdata-op`) and `funcdata_varnode`
//! (`w3-ir-funcdata-varnode`) porters run **after** this item, in parallel, with
//! NO seam-editing rights.  This module therefore pre-declares every `Funcdata`
//! field and seam surface those files reach, so they only add method `impl`
//! blocks:
//!   - `vbank`/`obank` and their accessors (`vbank()`/`vbank_mut()`/`obank()`/
//!     `obank_mut()`): the varnode/op factories (`newConstant`, `newUnique`,
//!     `newVarnodeOut`, `newOp`, …) create through these;
//!   - [`Funcdata::replace_reads_thunk`] / [`Funcdata::def_addr_time`]: the bank
//!     callbacks `opSetOutput`/`opSetInput`/`setInputVarnode`/`findVarnodeWritten`
//!     need;
//!   - the block op-list primitives ([`Funcdata::bb_insert_op`],
//!     [`Funcdata::bb_remove_op`], [`Funcdata::bb_op_head`],
//!     [`Funcdata::bb_op_tail`], [`Funcdata::bb_set_order`]) that `opInsert*`
//!     build on;
//!   - `glb` ([`ArchHandle`]) for the constant/unique/iop spaces and
//!     `minLanedSize`; `min_laned_size`, the create-index phase fields, and the
//!     `flags` word with `is_high_on()`;
//!   - [`Funcdata::set_varnode_properties`] (a `// STUB(W4)` no-op standing in
//!     for `localmap->queryProperties` + `Cover` calc) that `opSetOutput` and
//!     the `newVarnode*` factories call.
//!
//! ## Deferred surfaces (W4 / W6 / W7 / W8)
//!
//! Most of `funcdata.cc` is W4+ subsystem glue (the `Architecture`/`TypeFactory`
//! / `ScopeLocal` / `FuncProto` / `JumpTable` / `Override` / `Heritage` / `Merge`
//! / union-resolution machinery).  Those are seam-noted ([`crate::context`]'
//! `Architecture`/`Scope`/`FuncProto`, [`crate::dtype`]) and either return an
//! explicit `Err`/`None` or are left out; printing (`printRaw`/`printBlockTree`)
//! is W8.  This module carries the IR-ownership skeleton, the flag/phase state
//! machine, and the block-manipulation methods that are self-contained at the
//! W3 IR level (`structureReset`, `clearBlocks`, the edge-rewiring wrappers).
//!
//! # The Funcdata impl map
//!
//! `Funcdata` is one struct with its impl blocks split by owning phase — the
//! split IS the documentation of which phase mutates what:
//!
//! - `substrate/funcdata.rs`          — construction, arenas, core accessors
//! - `substrate/funcdata_op.rs`       — op creation/mutation primitives
//! - `substrate/funcdata_varnode.rs`  — varnode creation/lookup primitives
//! - `substrate/funcdata_block.rs`    — CFG surgery + the jump-table drivers
//! - `substrate/funcdata_encode.rs`   — marshaling
//! - `substrate/funcdata_printraw.rs` — raw printing
//! - `p2_lift/funcdata_resolveflow.rs`— flow resolution (P2)
//! - `p5_types/funcdata_union.rs`     — union facet resolution (P5)
//! - `p6_variables/funcdata_facing.rs`, `funcdata_merge.rs`,
//!   `funcdata_spacebase.rs`          — variable/merge/stack tiers (P6)
//! - `p9_emit/coreaction_casts.rs`    — cast insertion hooks (P9)

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::{int2, int4, uint4, uint8, uintm, Wrap};

use crate::block::{block_flags, BasicData, BlockGraph, BlockKind, FlowBlock};
use crate::fspec::{FuncCallSpecs, FuncProto, ParamActive};
use crate::op::PcodeOpBank;
use crate::context::{ArchHandle, BlockId, OpId, VarnodeId};
use crate::varnode::{DefOpInfo, VarnodeBank};

/// Boolean properties associated with a [`Funcdata`] (C++ anonymous `enum` in
/// `class Funcdata`, `funcdata.hh:57-74`).
///
/// Verbatim transcription of the C++ flag bits.
pub mod funcdata_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;

    /// Set if Varnodes have HighVariables assigned
    pub const highlevel_on: uint4 = 1;
    /// Set if Basic blocks have been generated
    pub const blocks_generated: uint4 = 2;
    /// Set if at least one basic block is currently unreachable
    pub const blocks_unreachable: uint4 = 4;
    /// Set if processing has started
    pub const processing_started: uint4 = 8;
    /// Set if processing completed
    pub const processing_complete: uint4 = 0x10;
    /// Set if data-type analysis will be performed
    pub const typerecovery_on: uint4 = 0x20;
    /// Set if data-type recovery is started
    pub const typerecovery_start: uint4 = 0x40;
    /// Set if there is no code available for this function
    pub const no_code: uint4 = 0x80;
    /// Set if \b this Funcdata object is dedicated to jump-table recovery
    pub const jumptablerecovery_on: uint4 = 0x100;
    /// Don't try to recover jump-tables, always truncate
    pub const jumptablerecovery_dont: uint4 = 0x200;
    /// Analysis must be restarted (because of new override info)
    pub const restart_pending: uint4 = 0x400;
    /// Set if function contains unimplemented instructions
    pub const unimplemented_present: uint4 = 0x800;
    /// Set if function flowed into bad data
    pub const baddata_present: uint4 = 0x1000;
    /// Set if we are performing double precision recovery
    pub const double_precis_on: uint4 = 0x2000;
    /// Set if data-type propagation passes reached maximum
    pub const typerecovery_exceeded: uint4 = 0x4000;
    /// Set if normalization will be performed
    pub const normalization_on: uint4 = 0x8000;
}

/// \brief Container for data structures associated with a single function
/// (C++ `class Funcdata`, `funcdata.hh:56`).
///
/// Holds the primary data structures for decompiling a function: control-flow
/// ([`bblocks`](Funcdata::bblocks_ref)/[`sblocks`](Funcdata::sblocks_ref)),
/// data-flow ([`vbank`](Funcdata::vbank)/[`obank`](Funcdata::obank)), and the
/// flag/phase state machine.  Most W4+ subsystems (`heritage`, `covermerge`,
/// `activeoutput`, `localoverride`, `qlst`) are seam-noted and omitted until
/// their waves; the `unionMap` (`union_map`) union-field resolution cache is
/// ported (W8, [`crate::funcdata_union`]) and the `lanedMap` ([`laned_map`])
/// laned-register access map is ported (W10, [`ActionLaneDivide`]).
///
/// [`laned_map`]: Funcdata
/// [`ActionLaneDivide`]: crate::coreaction_render::ActionLaneDivide
///
/// Sort key for [`Funcdata::laned_map`], the faithful transcription of the C++
/// `VarnodeData::operator<` (`pcoderaw.hh:67`): space index ascending, offset
/// ascending, then BIG sizes first.  The size component is wrapped in
/// [`std::cmp::Reverse`] so the derived [`Ord`] yields the descending-size
/// ordering of the C++ `return (size > op2.size)`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LanedKey {
    /// `space->getIndex()` (C++ first comparison key).
    space_index: int4,
    /// `offset` (C++ second comparison key).
    offset: u64,
    /// `size`, reversed so larger sizes sort first (C++ `size > op2.size`).
    size_rev: std::cmp::Reverse<u32>,
}

impl LanedKey {
    /// Build the key from a storage `(addr, size)`, mirroring how the C++ fills a
    /// `VarnodeData{space, offset, size}` before inserting into `lanedMap`.
    pub(crate) fn new(addr: &Address, size: int4) -> LanedKey {
        let space_index = addr.get_space().map(|s| s.get_index() as int4).unwrap_or(0);
        LanedKey {
            space_index,
            offset: addr.get_offset(),
            size_rev: std::cmp::Reverse(size as u32),
        }
    }
}

/// (kuna `framelayout`) One recovered stack-frame slot: the name, data type and
/// byte size a `restructure_varnode` pass gave it.
///
/// Recorded per pass into [`Funcdata::record_frame_slots`] so a slot that a later
/// pass's dataflow folded away is still reportable on the `decompile-all --json`
/// `variables` surface.  Carries no IR identity -- it is a description of the
/// frame, not a Varnode.
#[derive(Clone, Debug)]
pub struct FrameSlot {
    /// The symbol name the pass minted (`local_18`, `v3`, ...).
    pub name: String,
    /// The recovered data type.
    pub dtype: std::rc::Rc<crate::dtype::Datatype>,
    /// Byte size of the slot.
    pub size: int4,
}

pub struct Funcdata {
    /// Boolean properties associated with \b this function (C++ `flags`)
    flags: uint4,
    /// Creation index of first Varnode created after start of cleanup
    /// (C++ `clean_up_index`)
    clean_up_index: uint4,
    /// Creation index of first Varnode created after HighVariables are created
    /// (C++ `high_level_index`)
    high_level_index: uint4,
    /// Creation index of first Varnode created after ActionSetCasts
    /// (C++ `cast_phase_index`)
    cast_phase_index: uint4,
    /// Minimum Varnode size to check as LanedRegister (C++ `minLanedSize`)
    min_laned_size: int4,
    /// Current storage locations which may be laned registers (C++
    /// `Funcdata::lanedMap`, a `map<VarnodeData,const LanedRegister *>`).
    ///
    /// Keyed by `(space-index, offset, Reverse(size))` — the faithful transcription
    /// of `VarnodeData::operator<` (space index ascending, offset ascending, BIG
    /// sizes first).  The value is the matching [`LanedRegister`](crate::transform::LanedRegister),
    /// cloned from the architecture's immutable `lanerecords` (the C++ stores the
    /// `const LanedRegister *`).  Populated by [`check_for_laned_register`] when a
    /// laned-register-sized Varnode is created and read/cleared by
    /// `ActionLaneDivide`.
    ///
    /// [`check_for_laned_register`]: crate::funcdata::Funcdata::check_for_laned_register
    ///
    /// The value carries the storage `Address` and byte `size` (the C++
    /// `VarnodeData` the key was built from) alongside the matching record, so
    /// `ActionLaneDivide` can recover the `(addr, sz)` pair to iterate the
    /// Varnodes at that location (the C++ reads them off the `VarnodeData` key).
    laned_map: std::collections::BTreeMap<LanedKey, (Address, int4, crate::transform::LanedRegister)>,
    /// Number of bytes of binary data in function body (C++ `size`)
    size: int4,
    /// Global configuration data (C++ `glb`).  // STUB(W4)
    glb: ArchHandle,
    /// Name of function (C++ `name`)
    name: String,
    /// Name to display in output (C++ `displayName`)
    display_name: String,
    /// Starting code address of binary data (C++ `baseaddr`)
    baseaddr: Address,
    /// Prototype of this function (C++ `funcp`).  The real [`fspec::FuncProto`]
    /// (W10 un-seam): proto-recovery actions read/mutate the recovered model,
    /// lock state, and (via [`Self::get_active_output`]) the return-value trials.
    funcp: FuncProto,
    /// Data for assessing which return values are produced by \b this function
    /// (C++ `activeoutput`); `None` until [`Self::init_active_output`] turns on
    /// the proto-recovery output gathering (`ActionPrototypeTypes`).
    activeoutput: Option<ParamActive>,
    /// Local variables (symbols in the function scope) (C++ `localmap`, a
    /// `ScopeLocal *`).  `None` when filled in by decode.
    ///
    /// In C++ the `ScopeLocal` is a child of `glb->symboltab`; the merged Rust
    /// tree carries the global `Database` on the console `Architecture` (not on
    /// `glb`), so the `ScopeLocal` owns its own self-contained `Database` — see
    /// [`crate::varmap::ScopeLocal`].  The IR-mutating restructure/sync over the
    /// live varnode graph remains a documented seam (LOSS-109).
    localmap: Option<crate::varmap::ScopeLocal>,
    /// (kuna `framelayout`) Every stack-frame slot any `restructure_varnode` pass
    /// ever recovered, keyed by signed stack offset -> `(name, type, size)`.
    ///
    /// `restructure_varnode` re-derives the frame from the LIVE stack Varnodes on
    /// every pass and clears the previous pass's unlocked symbols first, so a slot
    /// whose store/load pair the dataflow later folded into a COPY (and then away)
    /// is present in an early layout and absent from the final one.  The emitted C
    /// is right to drop it -- there is no longer an expression to declare -- but the
    /// *frame* still has it, and the `decompile-all --json` `variables` surface is
    /// meant to report the recovered frame, the way IDA's stack view and Binary
    /// Ninja's variable list do.  This side table accumulates the union so
    /// `extract_variables` can report it; it never feeds back into the IR.
    frame_slots: std::cell::RefCell<std::collections::BTreeMap<i64, FrameSlot>>,
    /// List of jump-tables for this function (C++ `jumpvec`).
    ///
    /// The real `JumpTable` (`jumptable.{hh,cc}`) now lives here: the recovery
    /// chain (`recoverJumpTable`/`stageJumpTable`/`switchOverJumpTables`) populates
    /// it with the recovered address tables.
    jumpvec: Vec<crate::jumptable::JumpTable>,
    /// Container of Varnode objects for \b this function (C++ `vbank`)
    vbank: VarnodeBank,
    /// Container of PcodeOp objects for \b this function (C++ `obank`)
    obank: PcodeOpBank,
    /// Unstructured basic blocks (C++ `bblocks`)
    bblocks: BlockGraph,
    /// Structured block hierarchy on top of basic blocks (C++ `sblocks`)
    sblocks: BlockGraph,
    /// The HighVariable / VariableGroup / VariablePiece arena (W7, STUB(W7)).
    ///
    /// The C++ `HighVariable`s are allocated by `new HighVariable` from
    /// `Funcdata::assignHigh`/`Merge` and reverse-linked from each member
    /// `vn->high`; per ADR 0001 they live in this [`HighVariableBank`] keyed by
    /// [`crate::context::HighVariableId`], the back-link being the `Varnode::high`
    /// field already wired in `varnode.rs`.
    high_bank: crate::variable::HighVariableBank,
    /// SSA-construction manager (C++ `Heritage heritage`, `funcdata.hh:90`).
    ///
    /// Owns the heritage pass state (`pass`, the disjoint cover, the augmented
    /// dominator tree, the per-space info list) across the multiple
    /// `ActionHeritage` invocations in the universalAction loop, exactly as the
    /// C++ `Funcdata` member does.  Driven through [`op_heritage`](Funcdata::op_heritage).
    heritage: crate::heritage::Heritage,
    /// List of calls this function makes (C++ `vector<FuncCallSpecs *> qlst`,
    /// `funcdata.hh:89`).  Populated by `FlowInfo::setupCallSpecs`/
    /// `setupCallindSpecs` during flow analysis (the call op's in0 is an
    /// \e fspec annotation whose offset is the index into this vector), walked by
    /// the call-site recovery actions (`ActionFuncLink`/`ActionActiveParam`/
    /// `ActionActiveReturn`/`ActionDefaultParams`) and the printer's `opCall`.
    ///
    /// In C++ the `qlst` holds raw `FuncCallSpecs *` and the fspec address offset
    /// *is* that pointer; here the entries live inline and the offset is the
    /// vector index (the faithful equivalent — see `newVarnodeCallSpecs`).
    qlst: Vec<FuncCallSpecs>,
    /// HighVariable merging engine (C++ `Merge covermerge`, `funcdata.hh:91`).
    ///
    /// The C++ `Funcdata` owns a single `Merge` whose `copyTrims` accumulator
    /// (the trim COPYs `mergeAddrTied`/`mergeMarker` insert) **persists** across
    /// the merge actions so the later `ActionDominantCopy` (`processCopyTrims`)
    /// can replace them with a single dominant COPY.  The Rust engine takes
    /// `&mut dyn MergeContext` (= `&mut Funcdata`), so the field is move-out /
    /// move-back through [`Self::with_covermerge`] (the same self-mutation idiom
    /// as `op_heritage`); `None` until first use, built lazily by
    /// [`Self::ensure_covermerge`].  `pub(crate)` so the `funcdata_merge` bridge
    /// module can take/replace it.
    pub(crate) covermerge: Option<crate::merge::Merge>,
    /// Overrides of data-flow, prototypes, etc. that are local to \b this function
    /// (C++ `Override localoverride`, `funcdata.hh:99`).  The console
    /// `override flow|prototype|...` commands write here (C++ `dcp->fd->getOverride()`),
    /// and `FlowInfo` reads `hasFlowOverride()`/`getFlowOverride(addr)` from it at
    /// flow time (`flow.cc:43,434`).
    localoverride: crate::overrides::Override,
    /// A map from data-flow edges to the resolved field of a `TypeUnion` being
    /// accessed (C++ `map<ResolveEdge,ResolvedUnion> unionMap`, `funcdata.hh:101`).
    ///
    /// The W8 cast-insertion / union-resolution subsystem
    /// ([`crate::coreaction_cleanup::ActionSetCasts`]) and the per-op
    /// `getInputCast`/`resolveInFlow` surface read and write this through
    /// `getUnionField`/`setUnionField`/`forceFacingType`/`inheritUnionField`
    /// ([`crate::funcdata_union`]).  Keyed by [`crate::unionresolve::ResolveEdge`]
    /// (its [`Ord`] is the verbatim C++ `operator<`), so a `BTreeMap` reproduces
    /// the C++ `std::map` iteration / `emplace` semantics exactly (HashMap is
    /// clippy-banned and would not preserve the ordered `find`).
    pub(crate) union_map: std::collections::BTreeMap<
        crate::unionresolve::ResolveEdge,
        crate::unionresolve::ResolvedUnion,
    >,
    /// Warning/header comments produced during flow analysis (C++
    /// `Funcdata::warning`/`warningHeader` push directly into
    /// `glb->commentdb`; `funcdata.cc:119,135`).
    ///
    /// The merged Rust tree owns the `CommentDatabase` on the console
    /// `Architecture`, not on the W3 `glb` ([`crate::context::ArchHandle`]), so a
    /// `Funcdata` produced during flow follow buffers its analysis comments here
    /// and the decompile drive ([`crate::decompile_drive`]) flushes them into
    /// `arch.commentdb` once it has the `&mut Architecture` back (the same
    /// re-seed precedent as `mapped_symbols`/`pending_prototypes`).  Each entry is
    /// `(comment_type, placement_address, text)`; the function address is
    /// `baseaddr`.
    pending_comments: Vec<(kuna_base::types::uint4, Address, String)>,
    /// (kuna) Why the decompile pipeline aborted for this function, when it did.
    ///
    /// A caught per-function abort (`LOSS-131`) unwinds and discards the
    /// half-built `Funcdata`, so the console keeps rendering the *previous*,
    /// un-decompiled one — which has no structured blocks.  The driver stamps
    /// the recoverable error text here
    /// ([`Self::set_kuna_pipeline_failure`]) so the printer says the pipeline
    /// failed, and why, instead of blaming structuring (`PrintC::emit_function_document`).
    kuna_pipeline_failure: Option<String>,
    /// (kuna) The flow overrides `FlowInfo::process` asked for and
    /// [`Self::override_flow`] refused, as `(instruction, flow type, reason)`.
    ///
    /// A refusal is a REJECTED caller assertion, not a reason to discard the
    /// function: nothing was mutated before the refusal, so the IR is exactly the
    /// one this run would have produced without the directive.  Recording it here
    /// is what lets the front-ends report the rejection (`--assert` ledger, exit
    /// code) while still emitting the C.
    kuna_rejected_flow: Vec<(Address, kuna_base::types::uint4, String)>,
    /// (kuna, ghidra Phase 4) WIRE-ONLY symbols the encode-time link pass
    /// synthesized for named HighVariables the analysis deliberately left
    /// symbol-less — see [`crate::database::WireSymbol`].  They are encoded
    /// into `<localdb>` and referenced by `<high symref>` / `<vardecl symref>`
    /// but never enter the analysis scope, so the wire encode cannot perturb
    /// the emitted C.  Empty outside the ghidra-mode encode.
    pub(crate) kuna_wire_symbols: Vec<crate::database::WireSymbol>,
    /// Index into [`Self::kuna_wire_symbols`] per HighVariable.
    pub(crate) kuna_wire_symbol_for_high:
        std::collections::BTreeMap<crate::context::HighVariableId, usize>,
    /// (kuna `rustabi`) Per direct-call-target evidence about the registers the
    /// callee is *proven* not to write, keyed by `(space index, entry offset)`.
    /// Seeded by the driver after the flow build — the only place the disassembly
    /// engine is still reachable — and read at the call-output seam
    /// ([`crate::kuna_rustabi::classify_call_output_pair`]).  Empty unless
    /// `option rustabi` is live for this function.
    kuna_callee_ret_writes: std::collections::HashMap<
        (int4, kuna_base::types::uintb),
        std::rc::Rc<crate::kuna_rustabi::CalleeReturnWrites>,
    >,
    /// (kuna `calleedeadarg`) Per direct-call-target evidence about the registers
    /// the callee is *proven* never to read before writing, keyed by
    /// `(space index, entry offset)`.  Seeded by the driver after the flow build
    /// — the only place the disassembly engine is still reachable — and read at
    /// the input-trial scoring seam
    /// ([`crate::kuna_calleedeadarg::trial_is_dead_in_callee`]).  Empty unless
    /// `option calleedeadarg` is live for this function.
    kuna_callee_entry_dead: std::collections::HashMap<
        (int4, kuna_base::types::uintb),
        std::rc::Rc<crate::kuna_calleedeadarg::CalleeEntryDead>,
    >,
}

/// Opaque handle for a jump-table (C++ `JumpTable *` slot in `jumpvec`).
///
/// STUB(W4): the `JumpTable` type and all recovery logic (`recoverJumpTable`,
/// `stageJumpTable`, …) are W4; `Funcdata` only needs to track table identity at
/// W3 so `structureReset`'s dead-table sweep and `installSwitchDefaults` can run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JumpTableId(pub u32);

impl Funcdata {
    /// C++ `Funcdata::Funcdata` (`funcdata.cc:34`).
    ///
    /// The C++ pulls `vbank(scope->getArch())`, `minLanedSize` from
    /// `glb->getMinimumLanedRegisterSize()`, and attaches a `ScopeLocal` to the
    /// symbol table.  Here `glb` is the [`ArchHandle`] seam (it carries the
    /// `AddrSpaceManager`); the `VarnodeBank` analysis unique-start is supplied
    /// by the caller (`uniq_start`, the program's `Translate` —
    /// `getUniqueStart(ANALYSIS)`), exactly as `varnode.rs` documents.  The
    /// `ScopeLocal` attachment is `// STUB(W4)`; here `localmap` is created empty
    /// when a name is given (the C++ `nm.size()==0` "filled in by decode" branch
    /// leaves it `None`).
    pub fn new(
        nm: &str,
        disp: &str,
        glb: ArchHandle,
        addr: Address,
        uniq_start: uintm,
        sz: int4,
    ) -> KunaResult<Funcdata> {
        let vbank = VarnodeBank::new(glb.manage(), uniq_start)?;
        let min_laned_size = glb.get_minimum_laned_register_size();
        // bblocks / sblocks each get a root BlockGraph node (the C++ BlockGraph
        // *is* a FlowBlock; its `list` holds the components).
        let mut bblocks = BlockGraph::new();
        let broot = bblocks.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        bblocks.root = Some(broot);
        let mut sblocks = BlockGraph::new();
        let sroot = sblocks.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        sblocks.root = Some(sroot);

        // C++ funcdata.cc:54-71: stackid = glb->getStackSpace(); if nm is empty,
        // localmap = 0 (filled in by decode); else build a ScopeLocal on the
        // stack space and attach it.  The C++ then calls
        // `funcp.setScope(localmap,baseaddr-1)` (which sets the default proto
        // model) and `localmap->resetLocalWindow()`; here the proto model is set
        // by the proto-recovery wave (LOSS-136), so the local window is reset
        // lazily via [`Funcdata::reset_local_window`] once a model exists.  The
        // scope itself (the `addSymbol` target the console `map` commands reach)
        // is built eagerly, closing the `getScopeLocal()->addSymbol` seam.
        let localmap = if nm.is_empty() {
            None
        } else {
            // C++ id: sym ? sym->getId() : (0x57AB12CD << 32 | addr.offset&0xffffffff).
            // No FunctionSymbol is threaded here (the console builds the fd from
            // a name), so use the address-derived id, exactly as C++ does when
            // `sym == 0`.
            let id: uint8 = (0x57AB_12CDu64 << 32) | (addr.get_offset() & 0xffff_ffff);
            match glb.manage().get_stack_space() {
                Some(stackid) => {
                    let num_spaces = glb.manage().num_spaces();
                    Some(crate::varmap::ScopeLocal::new(id, stackid.clone(), nm, num_spaces)?)
                }
                // No stack space in the manager (some hand-built fixtures): the
                // C++ getStackSpace returns the spacebase space; if absent there
                // is no local frame to map (localmap stays absent).
                None => None,
            }
        };

        Ok(Funcdata {
            flags: 0,
            clean_up_index: 0,
            high_level_index: 0,
            cast_phase_index: 0,
            min_laned_size,
            laned_map: std::collections::BTreeMap::new(),
            size: sz,
            glb,
            name: nm.to_string(),
            display_name: disp.to_string(),
            baseaddr: addr,
            funcp: FuncProto::new(),
            activeoutput: None,
            localmap,
            frame_slots: std::cell::RefCell::new(std::collections::BTreeMap::new()),
            jumpvec: Vec::new(),
            vbank,
            obank: PcodeOpBank::new(),
            bblocks,
            sblocks,
            high_bank: crate::variable::HighVariableBank::new(),
            heritage: crate::heritage::Heritage::new(),
            qlst: Vec::new(),
            covermerge: None,
            localoverride: crate::overrides::Override::new(),
            union_map: std::collections::BTreeMap::new(),
            pending_comments: Vec::new(),
            kuna_pipeline_failure: None,
            kuna_rejected_flow: Vec::new(),
            kuna_wire_symbols: Vec::new(),
            kuna_wire_symbol_for_high: std::collections::BTreeMap::new(),
            kuna_callee_ret_writes: std::collections::HashMap::new(),
            kuna_callee_entry_dead: std::collections::HashMap::new(),
        })
    }

    // -----------------------------------------------------------------------
    // Simple accessors (C++ inline getters)
    // -----------------------------------------------------------------------

    /// Get the function's local symbol name (C++ `getName`).
    pub fn get_name(&self) -> &str {
        &self.name
    }
    /// Get the name to display in output (C++ `getDisplayName`).
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
    /// (kuna, Phase 3) Override the display name (the C++ `displayName` set by
    /// `Funcdata::decode` when the host sent a template-simplified label): the
    /// ghidra-mode decompileAt keeps the RAW `name` as the Java-side identity
    /// (the `HighFunction.decode` name echo) while the printed signature uses
    /// this display form.
    pub fn set_display_name(&mut self, disp: &str) {
        self.display_name = disp.to_string();
    }
    /// Get the [`Override`](crate::overrides::Override) object for \b this function
    /// (C++ `getOverride`, `funcdata.hh:214`).
    pub fn get_override(&self) -> &crate::overrides::Override {
        &self.localoverride
    }
    /// Mutably get the [`Override`](crate::overrides::Override) for \b this function
    /// (C++ `getOverride` non-const).  The console override commands write here.
    pub fn get_override_mut(&mut self) -> &mut crate::overrides::Override {
        &mut self.localoverride
    }

    /// Buffer an analysis warning comment indexed at a placement address (C++
    /// `Funcdata::warning`, `funcdata.cc:119`): the emitter places it before the
    /// source expression mapping most closely to `ad`.
    ///
    /// The C++ prefixes the text with "WARNING (jumptable): " when
    /// `jumptablerecovery_on` is set, else "WARNING: ", and pushes the
    /// `Comment::warning` into `glb->commentdb` via `addCommentNoDuplicate`.  The
    /// merged Rust tree owns the comment database on the console `Architecture`
    /// (not `glb`), so the comment is buffered on the `Funcdata`
    /// ([`Self::drain_pending_comments`]) and flushed by the decompile drive.
    pub fn warning(&mut self, txt: &str, ad: &Address) {
        let msg = self.warning_prefix() + txt;
        self.pending_comments.push((
            crate::architecture::comment_type::warning,
            ad.clone(),
            msg,
        ));
    }

    /// Buffer an analysis warning comment for the function header (C++
    /// `Funcdata::warningHeader`, `funcdata.cc:135`): emitted in the block comment
    /// printed right before the prototype, indexed at the function entry address.
    pub fn warning_header(&mut self, txt: &str) {
        let msg = self.warning_prefix() + txt;
        let entry = self.baseaddr.clone();
        self.pending_comments.push((
            crate::architecture::comment_type::warningheader,
            entry,
            msg,
        ));
    }

    /// The "WARNING: " / "WARNING (jumptable): " prefix the C++ `warning`/
    /// `warningHeader` prepend depending on `jumptablerecovery_on`
    /// (`funcdata.cc:123-126`).
    fn warning_prefix(&self) -> String {
        if (self.flags & funcdata_flags::jumptablerecovery_on) != 0 {
            "WARNING (jumptable): ".to_string()
        } else {
            "WARNING: ".to_string()
        }
    }

    /// Drain the buffered analysis comments produced during flow follow (the
    /// `(comment_type, placement_address, text)` triples; the function address is
    /// [`Self::get_address`]).  Called by the decompile drive to flush them into
    /// the console `Architecture`'s `commentdb`.
    pub fn drain_pending_comments(
        &mut self,
    ) -> Vec<(kuna_base::types::uint4, Address, String)> {
        std::mem::take(&mut self.pending_comments)
    }

    /// Read-only view of the buffered analysis comments (the
    /// `(comment_type, placement_address, text)` triples that
    /// [`Self::drain_pending_comments`] will hand to the console `Architecture`'s
    /// comment database).
    ///
    /// Used by [`crate::p8_structure::kuna_condfold`] to decline folding a basic
    /// block that carries an instruction comment into a condition operand: the
    /// printer suppresses `emitCommentGroup` under `comma_separate`, so such a
    /// fold would silently drop the comment.
    pub fn pending_comments_ref(
        &self,
    ) -> &[(kuna_base::types::uint4, Address, String)] {
        &self.pending_comments
    }

    /// Buffer an already-prefixed comment triple (the cross-function carry path:
    /// `FlowInfo::inlineFlow` drains a nested callee flow's buffered comments and
    /// re-buffers them on the top-level function, since both reach the same
    /// `glb->commentdb` in C++).  The text already carries its "WARNING: " prefix.
    pub fn push_raw_comment(
        &mut self,
        tp: kuna_base::types::uint4,
        ad: Address,
        txt: String,
    ) {
        self.pending_comments.push((tp, ad, txt));
    }

    /// (kuna) Record why the decompile pipeline aborted for this function.
    ///
    /// Called by a front-end that catches the drive's recoverable per-function
    /// error and keeps the previous `Funcdata` around to render (the console
    /// `decompile` command).  Newlines are folded and any comment terminator
    /// neutralized so the text is safe to plant in the emitted C.
    pub fn set_kuna_pipeline_failure(&mut self, reason: &str) {
        let flat: String = reason
            .chars()
            .map(|c| if c == '\n' || c == '\r' || c == '\t' { ' ' } else { c })
            .collect();
        self.kuna_pipeline_failure = Some(flat.replace("*/", "* /").trim().to_string());
    }

    /// (kuna) Why the decompile pipeline aborted for this function, if it did.
    pub fn kuna_pipeline_failure(&self) -> Option<&str> {
        self.kuna_pipeline_failure.as_deref()
    }

    /// (kuna) Record a flow override [`Self::override_flow`] refused at `addr`.
    /// Idempotent per `(addr, type_)`, so a restart re-flow that re-attempts the
    /// same override does not report it twice.
    pub fn note_rejected_flow_override(
        &mut self,
        addr: Address,
        type_: kuna_base::types::uint4,
        reason: &str,
    ) {
        if self.kuna_rejected_flow.iter().any(|(a, t, _)| a == &addr && *t == type_) {
            return;
        }
        self.kuna_rejected_flow.push((addr, type_, reason.to_string()));
    }

    /// (kuna) The flow overrides this function's flow follow refused.
    pub fn kuna_rejected_flow_overrides(&self) -> &[(Address, kuna_base::types::uint4, String)] {
        &self.kuna_rejected_flow
    }

    /// (kuna `rustabi`) Record what a probe of `entry`'s body proved about the
    /// registers it writes.
    pub fn kuna_set_callee_ret_writes(
        &mut self,
        entry: &Address,
        writes: std::rc::Rc<crate::kuna_rustabi::CalleeReturnWrites>,
    ) {
        if let Some(sp) = entry.get_space() {
            self.kuna_callee_ret_writes.insert((sp.get_index(), entry.get_offset()), writes);
        }
    }

    /// (kuna `rustabi`) The recorded probe of `entry`'s body, if one was taken.
    pub fn kuna_callee_ret_writes(
        &self,
        entry: &Address,
    ) -> Option<&crate::kuna_rustabi::CalleeReturnWrites> {
        let sp = entry.get_space()?;
        self.kuna_callee_ret_writes.get(&(sp.get_index(), entry.get_offset())).map(|r| r.as_ref())
    }

    /// (kuna `calleedeadarg`) Record what a probe of `entry`'s body proved about
    /// the registers it never reads before writing.
    pub fn kuna_set_callee_entry_dead(
        &mut self,
        entry: &Address,
        dead: std::rc::Rc<crate::kuna_calleedeadarg::CalleeEntryDead>,
    ) {
        if let Some(sp) = entry.get_space() {
            self.kuna_callee_entry_dead.insert((sp.get_index(), entry.get_offset()), dead);
        }
    }

    /// (kuna `calleedeadarg`) The recorded entry-liveness probe of `entry`'s
    /// body, if one was taken.
    pub fn kuna_callee_entry_dead(
        &self,
        entry: &Address,
    ) -> Option<&crate::kuna_calleedeadarg::CalleeEntryDead> {
        let sp = entry.get_space()?;
        self.kuna_callee_entry_dead.get(&(sp.get_index(), entry.get_offset())).map(|r| r.as_ref())
    }

    /// Get the entry point address (C++ `getAddress`).
    pub fn get_address(&self) -> &Address {
        &self.baseaddr
    }
    /// Get the function body size in bytes (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.size
    }
    /// Get the program/architecture owning \b this function (C++ `getArch`).
    pub fn get_arch(&self) -> &ArchHandle {
        &self.glb
    }

    /// Build a fresh empty Funcdata sharing this function's arch / entry /
    /// unique-base (a placeholder for the move-out-build-blocks-move-in dance in
    /// `build_partial_blocks`).
    pub fn new_placeholder_like(src: &Funcdata) -> KunaResult<Funcdata> {
        Funcdata::new(
            "@@placeholder",
            "@@placeholder",
            src.glb.clone(),
            src.baseaddr.clone(),
            src.vbank.get_uniqbase(),
            0,
        )
    }
    /// Get the function's prototype object (C++ `getFuncProto`).
    pub fn get_func_proto(&self) -> &FuncProto {
        &self.funcp
    }
    /// Mutably borrow the function's prototype object (C++ non-const
    /// `getFuncProto`).  Proto-recovery actions (`ActionPrototypeTypes`,
    /// `ActionReturnRecovery`, ...) set the model and derive the output map.
    pub fn get_func_proto_mut(&mut self) -> &mut FuncProto {
        &mut self.funcp
    }

    /// Apply a parsed-and-locked input/output prototype (from the console
    /// `parse line extern <decl>`) to this function's `funcp` (C++
    /// `Architecture::setPrototype` on a queried `Funcdata`).
    ///
    /// Reaches the type factory / address manager / default model through the
    /// `glb` [`ArchHandle`] and runs [`FuncProto::seed_locked_from_pieces`].  A
    /// no-op (returns `Ok`) if the architecture has no default model (no model
    /// to lock to); the function then falls back to the unlocked recovery path.
    ///
    /// If storage assignment for the declared parameters reaches an un-ported
    /// seam (e.g. `assignParameterStorage`'s hidden-return-pointer path for a
    /// struct-returning function — a W4 surface), the partially-mutated `funcp`
    /// is reset to the clean empty prototype and the prototype is left
    /// **unapplied** (returning `Ok`).  The function then decompiles exactly as
    /// it did before this seed wired in (the prior unrecovered behavior), so a
    /// not-yet-supported declaration degrades gracefully rather than aborting the
    /// whole decompile.
    ///
    /// (kuna `cppsig`) A `pieces` with no `outtype` locks the INPUT half only —
    /// see [`FuncProto::seed_locked_from_pieces`], which owns that contract so the
    /// caller-side rebuild honors it identically.
    pub fn apply_locked_prototype(
        &mut self,
        pieces: &crate::fspec::PrototypePieces,
    ) -> KunaResult<()> {
        self.apply_locked_prototype_with_model(pieces, None)
    }

    /// [`Self::apply_locked_prototype`] with an explicit prototype MODEL (the
    /// ghidra-mode path: the host's `<prototype model=…>` names the convention
    /// its committed parameter storage was assigned under, so re-deriving
    /// storage from kuna's default model would disagree with the database and
    /// force Java's `checkFullCommit` to rewrite the user's signature).
    /// `None` keeps the architecture default.
    pub fn apply_locked_prototype_with_model(
        &mut self,
        pieces: &crate::fspec::PrototypePieces,
        model: Option<Rc<crate::fspec::ProtoModel>>,
    ) -> KunaResult<()> {
        let defaultfp = match model.or_else(|| self.glb.default_fp().cloned()) {
            Some(m) => m,
            None => return Ok(()),
        };
        let void_type =
            Rc::new(crate::dtype::Datatype::new(0, crate::dtype::type_metatype::TYPE_VOID));
        // The type factory + manager live on the architecture, shared into `glb`.
        // Clone the `Rc<ArchContext>` (cheap refcount bump) so the factory/manager
        // borrows come from the clone, leaving `self.funcp` freely mutable.
        let glb = self.glb.clone();
        let types = glb.types().ok_or_else(|| {
            kuna_base::error::KunaError::lowlevel("apply_locked_prototype: no type factory on glb")
        })?;
        let manager = glb.manage();
        if let Err(e) =
            self.funcp.seed_locked_from_pieces(pieces, defaultfp, void_type, types, manager)
        {
            // Storage assignment reached an un-ported seam (W4); discard the
            // half-applied prototype and decompile as the unrecovered function.
            self.funcp = FuncProto::new();
            let _ = e;
        }
        Ok(())
    }

    /// Re-apply a console `map param <i> <addr> <typedecl>` storage lock to the
    /// (rebuilt) prototype.
    ///
    /// The C++ console (`IfcMapParam::execute`, ifacedecomp.cc:613) writes the
    /// locked input straight onto the queried Funcdata's live `FuncProto` via
    /// `setParam` (`store->setInput`).  In kuna the `decompile` command rebuilds
    /// the Funcdata from scratch, discarding that console-set store, so the lock
    /// must be re-seeded here on the fresh prototype — the same re-seed model as
    /// [`Self::apply_locked_prototype`] / [`Self::seed_mapped_symbols`].  Each
    /// `(i, name, piece)` carries the parsed `ParameterPieces` (typelock|namelock
    /// already set by the directive), so the rebuilt proto becomes input-locked
    /// and `ActionPrototypeTypes` forces the typed input Varnode.
    pub fn apply_mapped_params(
        &mut self,
        params: &[(int4, String, crate::fspec::ParameterPieces)],
    ) {
        if params.is_empty() {
            return;
        }
        let void_type =
            Rc::new(crate::dtype::Datatype::new(0, crate::dtype::type_metatype::TYPE_VOID));
        // C++ `getFuncProto().setParam` relies on the store the Funcdata's
        // `setScope` attached at construction; the rebuilt proto may have no store
        // yet, so attach the internal store first (as `IfcMapParam` does).
        self.funcp.attach_internal_store(void_type);
        for (i, name, piece) in params {
            self.funcp.set_param(*i, name, piece);
        }
    }

    /// The active return-value recovery state, or `None` if output recovery is
    /// not in progress (C++ `Funcdata::getActiveOutput`).
    ///
    /// `ActionPrototypeTypes::apply` calls [`Self::init_active_output`] (the C++
    /// `initActiveOutput`) before heritage when the output is not locked, so
    /// `Heritage::guardReturns` and `ActionReturnRecovery` see a live
    /// [`ParamActive`].  `ActionDeadCode::gatherConsumedReturn` also reads it to
    /// decide whether the return is fully consumed.
    pub fn get_active_output(&self) -> Option<&ParamActive> {
        self.activeoutput.as_ref()
    }

    /// Mutably borrow the active return-value recovery state (C++ non-const
    /// `getActiveOutput`).
    pub fn get_active_output_mut(&mut self) -> Option<&mut ParamActive> {
        self.activeoutput.as_mut()
    }

    /// Initialize \e return prototype recovery analysis (C++
    /// `Funcdata::initActiveOutput`, `funcdata_varnode.cc:603`).
    ///
    /// Allocates a fresh [`ParamActive`] for the output trials and sets its
    /// max-pass from the prototype model's maximum output heritage delay
    /// (capped at 3, the C++ `if (maxdelay>0) maxdelay = 3`).
    pub fn init_active_output(&mut self) {
        let mut active = ParamActive::new(false);
        // C++ `funcp.getMaxOutputDelay()` reads the model; the C++ FuncProto
        // always has a model by this point (the ctor's setScope/setInternal).
        // Guard the unrecovered (model-less) case so this never panics.
        let mut maxdelay =
            if self.funcp.has_model() { self.funcp.get_max_output_delay() } else { 0 };
        if maxdelay > 0 {
            maxdelay = 3;
        }
        active.set_max_pass(maxdelay);
        self.activeoutput = Some(active);
    }

    /// Stop tracking \e return prototype recovery (C++
    /// `Funcdata::clearActiveOutput`, `funcdata.hh:429`).
    pub fn clear_active_output(&mut self) {
        self.activeoutput = None;
    }

    /// Move the active-output [`ParamActive`] out of `self` (leaving `None`), so
    /// `ActionReturnRecovery` can drive `ancestor_op_use` (which needs `&mut
    /// self`) while owning the trial container.  Pair with
    /// [`Self::restore_active_output`].  The C++ holds `activeoutput` as a member
    /// pointer and mutates it and the IR concurrently; the Rust borrow checker
    /// requires the temporary move-out.
    pub fn take_active_output(&mut self) -> Option<ParamActive> {
        self.activeoutput.take()
    }

    /// Restore an active-output container previously taken with
    /// [`Self::take_active_output`].
    pub fn restore_active_output(&mut self, active: ParamActive) {
        self.activeoutput = Some(active);
    }

    /// Number of sub-function call specifications (C++ `Funcdata::numCalls`,
    /// `funcdata.hh:281`).  The `qlst` is populated by `FlowInfo::setupCallSpecs`
    /// during flow analysis.
    pub fn num_calls(&self) -> int4 {
        self.qlst.len() as int4
    }

    /// Get the i-th call specification (C++ `Funcdata::getCallSpecs(int4)`,
    /// `funcdata.hh:282`).
    pub fn get_call_specs(&self, i: int4) -> &FuncCallSpecs {
        &self.qlst[i as usize]
    }

    /// Get the i-th call specification mutably (the recovery actions need to
    /// mutate the `ParamActive` trials in place).
    pub fn get_call_specs_mut(&mut self, i: int4) -> &mut FuncCallSpecs {
        &mut self.qlst[i as usize]
    }

    /// Get the call specification associated with a CALL op (C++
    /// `Funcdata::getCallSpecs(const PcodeOp *)`, `funcdata.cc:481`).
    ///
    /// In C++ this first checks whether `op->getIn(0)` is an \e fspec annotation
    /// (recovering the `FuncCallSpecs *` from the offset directly); since the
    /// offset is the `qlst` index here, both arms reduce to the same vector entry,
    /// so the index lookup is the faithful equivalent.  Returns the matching
    /// `qlst` index, or `None`.
    pub fn get_call_specs_index(&self, op: OpId) -> Option<int4> {
        self.qlst.iter().position(|fc| fc.get_op() == op).map(|i| i as int4)
    }

    /// Append a newly-built call specification to the `qlst` (C++
    /// `qlst.push_back(res)` in `FlowInfo::setupCallSpecs`).  Returns its index
    /// (the \e fspec handle).
    pub fn push_call_specs(&mut self, fc: FuncCallSpecs) -> int4 {
        self.qlst.push(fc);
        (self.qlst.len() - 1) as int4
    }

    /// Remove all call specifications (C++ `Funcdata::clearCallSpecs`,
    /// `funcdata.cc:462`).
    pub fn clear_call_specs(&mut self) {
        self.qlst.clear();
    }

    /// Move the `qlst` out of `self` (leaving it empty), so the recovery actions
    /// can iterate the call specs while still borrowing `&mut Funcdata` for the
    /// per-call IR rewrites.  Mirror of [`Self::take_active_output`] — the C++
    /// holds a `FuncCallSpecs *` and mutates `data` through it; the borrow checker
    /// forces the take/restore dance here.
    pub fn take_call_specs(&mut self) -> Vec<FuncCallSpecs> {
        std::mem::take(&mut self.qlst)
    }

    /// Restore the `qlst` taken by [`Self::take_call_specs`].
    pub fn restore_call_specs(&mut self, qlst: Vec<FuncCallSpecs>) {
        self.qlst = qlst;
    }

    /// Splice a single call spec out of `qlst` (CORRECTION-7 finalize tail).
    ///
    /// The input-trial *check* path keeps every spec on `qlst` so the cross-call
    /// `getCallSpecs` lookup resolves; the *finalize* tail
    /// (`final_input_check`/`build_input_from_trials`) instead needs a single
    /// `&mut FuncCallSpecs` held alongside `&mut Funcdata`.  Removing the entry
    /// shifts the higher indices down by one — paired with
    /// [`Self::restore_call_specs_at`] (`insert(idx, ..)`) the index ordering is
    /// re-established before the next outer-loop iteration, and the spec just
    /// finalized is never looked up cross-call (its trials are resolved).
    pub fn replace_call_specs(&mut self, index: int4) -> FuncCallSpecs {
        self.qlst.remove(index as usize)
    }

    /// Re-insert a call spec spliced out by [`Self::replace_call_specs`] at the
    /// same position, restoring index stability for the remaining iterations.
    pub fn restore_call_specs_at(&mut self, index: int4, fc: FuncCallSpecs) {
        self.qlst.insert(index as usize, fc);
    }

    /// Remove the `qlst` entry at `index` (C++ `FlowInfo::deleteCallSpec`,
    /// `flow.cc:1308`): the call spec whose CALL op has just been in-lined /
    /// injected away is dropped.  Because the \e fspec handle is the call op's own
    /// identity (not the vector position; see [`Self::get_call_specs_index`]),
    /// erasing the entry does not invalidate the remaining annotation Varnodes.
    pub fn delete_call_spec(&mut self, index: int4) {
        self.qlst.remove(index as usize);
    }

    /// Remove the call spec associated with the given CALL op (C++
    /// `Funcdata::deleteCallSpecs`, `funcdata.cc:521`): scan `qlst` for the entry
    /// whose op matches and erase it.  Used by `blockRemoveInternal` when an
    /// unreachable block containing a CALL is deleted.  Because the \e fspec handle
    /// is the call op's own identity (not the vector position; see
    /// [`Self::get_call_specs_index`]), erasing the entry does not invalidate the
    /// remaining annotation Varnodes.  A no-op if the op has no registered spec.
    pub fn delete_call_specs(&mut self, op: OpId) {
        if let Some(idx) = self.qlst.iter().position(|fc| fc.get_op() == op) {
            self.qlst.remove(idx);
        }
    }

    /// Put the calls in dominance order so earlier calls get evaluated first
    /// (C++ `Funcdata::sortCallSpecs`, `funcdata.cc:514`; comparator
    /// `compareCallspecs`, `funcdata.cc:501`: by parent-block index, then by the
    /// call op's `SeqNum` order).  Order affects parameter analysis.
    ///
    /// Because the \e fspec handle is the call op's own identity (not the vector
    /// position), reordering `qlst` does not invalidate the annotation Varnodes
    /// (see [`Self::get_call_specs_index`]).
    pub fn sort_call_specs(&mut self) {
        // Pre-compute (block index, seqnum order) for each call op so the sort key
        // does not re-borrow the op bank inside the comparator.
        let mut keyed: Vec<(int4, u32, FuncCallSpecs)> = self
            .qlst
            .drain(..)
            .map(|fc| {
                let op = fc.get_op();
                let o = self.obank.get(op);
                let ind = o
                    .and_then(|o| o.get_parent())
                    .map(|b| self.bblocks.block(b).get_index())
                    .unwrap_or(0);
                let order = self
                    .obank
                    .get(op)
                    .map(|o| o.get_seq_num().get_order())
                    .unwrap_or(0);
                (ind, order, fc)
            })
            .collect();
        keyed.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
        self.qlst = keyed.into_iter().map(|(_, _, fc)| fc).collect();
    }

    /// Find the jump table associated with a BRANCHIND op, or `None` (C++
    /// `Funcdata::findJumpTable`, `funcdata_block.cc:462`): look up the table whose
    /// `getOpAddress()` matches the op's address.
    pub fn find_jump_table(&self, op: OpId) -> Option<&crate::jumptable::JumpTable> {
        let addr = self.obank().get(op)?.get_addr();
        self.jumpvec.iter().find(|jt| jt.get_op_address() == addr)
    }

    /// Index of the jump table associated with a BRANCHIND op (companion to
    /// [`find_jump_table`](Self::find_jump_table) for the `collectEdges` consult,
    /// which needs the index to read the out-edge address table).
    pub fn find_jump_table_index(&self, op: OpId) -> Option<usize> {
        let addr = self.obank().get(op)?.get_addr().clone();
        self.jumpvec.iter().position(|jt| *jt.get_op_address() == addr)
    }

    /// Perform an entire heritage pass linking Varnode reads to writes (C++
    /// `Funcdata::opHeritage`, `funcdata.hh:471` — `heritage.heritage()`).
    ///
    /// Drives the owned [`Heritage`](crate::heritage::Heritage) engine against
    /// the live IR, mutating it into SSA form (free reads linked to their
    /// reaching writes/inputs, MULTIEQUAL phi-nodes placed at the dominance
    /// frontier of each write).  The engine is temporarily moved out of `self`
    /// so it can take `&mut self` (the C++ `heritage` member holds a `fd`
    /// back-pointer; Rust expresses the same self-mutation with a move-out /
    /// move-back).  `build_info_list` is idempotent and ensures the per-space
    /// info list exists — the merged-tree substitute for the
    /// `startProcessing` → `heritage.buildInfoList()` call (a W4 seam there).
    pub fn op_heritage(&mut self) {
        self.op_heritage_with_deadline(None)
    }

    /// [`op_heritage`](Self::op_heritage) with the (kuna) `decompile-all`
    /// per-function watchdog deadline threaded through to
    /// [`Heritage::heritage`](crate::heritage::Heritage::heritage)'s
    /// per-address-space loop (`ActionHeritage` passes its
    /// [`ActionContext::deadline`](crate::action::ActionContext); every other
    /// caller passes `None` and is byte-identical to before).
    pub fn op_heritage_with_deadline(&mut self, deadline: Option<std::time::Instant>) {
        let mut heritage = std::mem::take(&mut self.heritage);
        heritage.build_info_list(self);
        // C++ `Funcdata::startProcessing` runs `localoverride.applyDeadCodeDelay`
        // right after `buildInfoList` (funcdata.cc:167): re-apply any persisted
        // per-space deadcode delays (installed by `Heritage::bumpDeadcodeDelay`
        // on the restart) to the freshly-built per-space `HeritageInfo`.  Without
        // this, a deadcode-delay bump installed on the Override before a restart
        // re-flow would be lost when `build_info_list` re-seeds the info to the
        // space defaults, so the stack-alias store would still be dead-eliminated
        // one pass before the aliasing LOAD resolves.  The C++ does this in
        // `startProcessing` (which re-runs each restart); the merged tree drives
        // heritage lazily here, so the apply lands at the same point in the order.
        for (space_index, delay) in self.localoverride.deadcode_delays() {
            if let Some(spc) = self.glb.manage().get_space(space_index) {
                let spc = std::rc::Rc::clone(spc);
                // C++ `setDeadCodeDelay` throws if `delay < info.delay`; the
                // bump only ever installs `deadcodeDelay+1 >= delay`, so this is
                // well-formed.  Swallow the error defensively (a malformed
                // console-supplied `override deadcodedelay` degrades to no-op).
                let _ = heritage.set_dead_code_delay(&spc, delay);
            }
        }
        heritage.heritage(self, deadline);
        self.heritage = heritage;
    }

    /// Get the heritage pass when the given address was last heritaged, or -1
    /// (C++ `Funcdata::isHeritaged` reads `heritage.heritagePass`).
    pub fn heritage_pass(&self, addr: &Address) -> int4 {
        self.heritage.heritage_pass(addr)
    }

    /// Number of heritage passes that have been performed for the given space
    /// (C++ `Funcdata::numHeritagePasses`, `funcdata.hh:245` —
    /// `heritage.numHeritagePasses(spc)`).
    ///
    /// The C++ throws `LowlevelError` if the space has never been heritaged; the
    /// owned [`Heritage`](crate::heritage::Heritage) engine surfaces the same as
    /// an `Err`, which the caller forwards as the C++ would.
    pub fn num_heritage_passes(
        &self,
        spc: &std::rc::Rc<kuna_base::space::AddrSpace>,
    ) -> kuna_base::error::KunaResult<int4> {
        self.heritage.num_heritage_passes(spc)
    }

    /// Overall count of heritage passes (C++ `Funcdata::getHeritagePass`,
    /// `funcdata.hh:239` — `heritage.getPass()`).
    pub fn get_heritage_pass(&self) -> int4 {
        self.heritage.get_pass()
    }

    /// Get the list of guarded LOADs (C++ `Funcdata::getLoadGuards`,
    /// `funcdata.hh:276` — `heritage.getLoadGuards()`).
    pub fn get_load_guards(&self) -> &[crate::heritage::LoadGuard] {
        self.heritage.get_load_guards()
    }

    /// Get the list of guarded STOREs (C++ `Funcdata::getStoreGuards`,
    /// `funcdata.hh:277` — `heritage.getStoreGuards()`).
    pub fn get_store_guards(&self) -> &[crate::heritage::LoadGuard] {
        self.heritage.get_store_guards()
    }

    /// Get the LoadGuard associated with a STORE op, if any (C++
    /// `Funcdata::getStoreGuard`, `funcdata.hh:278` — `heritage.getStoreGuard(op)`).
    pub fn get_store_guard(&self, op: crate::context::OpId) -> Option<&crate::heritage::LoadGuard> {
        self.heritage.get_store_guard(op)
    }

    /// Force the heritage engine to regenerate its block structures on the next
    /// pass (C++ `Funcdata::structureReset` -> `heritage.forceRestructure()`).
    ///
    /// Called from `structure_reset` after the CFG changed, so the cached
    /// augmented dominator tree (holding stale block handles) is not reused — see
    /// [`Heritage::force_restructure`](crate::heritage::Heritage::force_restructure).
    pub fn heritage_force_restructure(&mut self) {
        self.heritage.force_restructure();
    }

    /// Is it safe to remove dead code in a space? (C++
    /// `Funcdata::deadRemovalAllowed`, `funcdata.hh:262` —
    /// `heritage.deadRemovalAllowed(spc)`).
    pub fn dead_removal_allowed(&self, spc: &std::rc::Rc<kuna_base::space::AddrSpace>) -> bool {
        self.heritage.dead_removal_allowed(spc)
    }

    /// Record that dead code has been seen in a space (C++
    /// `Funcdata::seenDeadcode`, `funcdata.hh:250` — `heritage.seenDeadCode(spc)`).
    pub fn seen_deadcode(&mut self, spc: &std::rc::Rc<kuna_base::space::AddrSpace>) {
        self.heritage.seen_dead_code(spc);
    }

    /// Is dead-code removal safe for a space, and if so mark that it happened
    /// (C++ `Funcdata::deadRemovalAllowedSeen`, `funcdata.hh:268` —
    /// `heritage.deadRemovalAllowedSeen(spc)`).
    pub fn dead_removal_allowed_seen(
        &mut self,
        spc: &std::rc::Rc<kuna_base::space::AddrSpace>,
    ) -> bool {
        self.heritage.dead_removal_allowed_seen(spc)
    }

    /// Delete any dead PcodeOps (C++ `Funcdata::clearDeadOps`, `funcdata.hh:437`
    /// — `obank.destroyDead()`).
    pub fn clear_dead_ops(&mut self) {
        self.obank_mut().destroy_dead();
    }

    /// Ensure the per-space heritage info list exists (C++
    /// `Heritage::buildInfoList`, called by `startProcessing` before the action
    /// pipeline runs).  Idempotent.
    ///
    /// `deadRemovalAllowed`/`seenDeadcode` index this list by space, so any
    /// action that reads them (e.g. `ActionDeadCode`) needs it populated; the
    /// C++ invariant is `startProcessing` builds it before any action runs, but
    /// the merged tree's `ActionStart` is a seam, so the actions ensure it.
    pub fn ensure_heritage_info_list(&mut self) {
        let mut heritage = std::mem::take(&mut self.heritage);
        heritage.build_info_list(self);
        self.heritage = heritage;
    }

    /// Get the local function scope (C++ `getScopeLocal`).
    pub fn get_scope_local(&self) -> Option<&crate::varmap::ScopeLocal> {
        self.localmap.as_ref()
    }

    /// (kuna `framelayout`) Fold this pass's recovered stack-frame layout into the
    /// running union.  Called at the tail of every `restructure_varnode`.
    ///
    /// A slot already recorded is kept: the FIRST pass to see it had the most
    /// dataflow still standing, so its type hint is the better-informed one.
    pub fn record_frame_slots(&self, slots: impl IntoIterator<Item = (i64, FrameSlot)>) {
        let mut map = self.frame_slots.borrow_mut();
        for (off, slot) in slots {
            map.entry(off).or_insert(slot);
        }
    }

    /// (kuna `framelayout`) The union of every stack-frame slot any pass recovered.
    pub fn frame_slots(&self) -> Vec<(i64, FrameSlot)> {
        self.frame_slots.borrow().iter().map(|(k, v)| (*k, v.clone())).collect()
    }
    /// Mutably borrow the local function scope (C++ non-const `getScopeLocal`).
    /// The console `map` commands and `ActionRestructureVarnode` reach the
    /// `ScopeLocal` through this to add/restructure symbols.
    pub fn get_scope_local_mut(&mut self) -> Option<&mut crate::varmap::ScopeLocal> {
        self.localmap.as_mut()
    }
    /// The console-mapped Symbol specs in this function's local scope (the
    /// `map addr` symbols).  Empty when there is no local scope.  Used to carry
    /// the symbols across the kuna console's IR rebuild on `decompile`.
    pub fn mapped_symbol_specs(
        &self,
    ) -> Vec<(String, std::rc::Rc<crate::dtype::Datatype>, Address, uint4)> {
        self.localmap.as_ref().map(|lm| lm.mapped_symbol_specs()).unwrap_or_default()
    }

    /// The usepoint-scoped console Symbol specs in this function's local scope (the
    /// register-storage `type varnode %REG(pc) <type> <name>` symbols), each paired
    /// with its use address.  Empty when there is no local scope.  Carried across
    /// the kuna console's IR rebuild on `decompile` (the register `tmp` retstruct
    /// return Symbol) — the usepoint-aware counterpart of
    /// [`mapped_symbol_specs`](Self::mapped_symbol_specs).
    #[allow(clippy::type_complexity)]
    pub fn usepoint_symbol_specs(
        &self,
    ) -> Vec<(String, std::rc::Rc<crate::dtype::Datatype>, Address, uint4, Address, bool)> {
        self.localmap.as_ref().map(|lm| lm.usepoint_symbol_specs()).unwrap_or_default()
    }

    /// Snapshot the console-added dynamic (`map hash` / `map convert`) symbols of
    /// this function's local scope as re-seed specs — the dynamic counterpart of
    /// [`mapped_symbol_specs`](Self::mapped_symbol_specs).  Each
    /// [`DynamicSymbolSpec`](crate::database::DynamicSymbolSpec) carries the
    /// symbol's category + display format so an `EquateSymbol` survives the
    /// rebuild intact (see `seed_dynamic_symbols`).
    pub fn dynamic_symbol_specs(&self) -> Vec<crate::database::DynamicSymbolSpec> {
        self.localmap
            .as_ref()
            .map(|lm| lm.database().scope_dynamic_symbol_specs(lm.scope_id()))
            .unwrap_or_default()
    }

    /// Re-create the given console-added dynamic (`map hash`) Symbols in this
    /// function's local scope (the dynamic counterpart of
    /// [`seed_mapped_symbols`](Self::seed_mapped_symbols)).  The console set
    /// `namelock|typelock` on each; re-applied here so `ActionDynamicSymbols` sees
    /// the same dynamic-entry list the C++ `getScopeLocal()->beginDynamic()` does.
    pub fn seed_dynamic_symbols(&mut self, specs: &[crate::database::DynamicSymbolSpec]) {
        use crate::database::symbol_category;
        use crate::varnode::varnode_flags;
        if let Some(lm) = self.localmap.as_mut() {
            for spec in specs {
                // An EquateSymbol (`map convert`) must be re-created via
                // addEquateSymbol so its category stays `equate` and its forced
                // display format (`dispflags`) + value are preserved — otherwise
                // ActionDynamicMapping's equate arm never fires and the constant
                // renders in the default format instead of the forced one.  The
                // C++ console never rebuilds the IR, so this re-seed is the kuna
                // stand-in for that persistence (the equate's type IS the
                // `getBase(1,TYPE_UNKNOWN)` base1 the ctor uses).
                if spec.category == symbol_category::EQUATE {
                    if let Some(value) = spec.equate_value {
                        let _ = lm.add_equate_symbol(
                            &spec.name,
                            spec.dispflags,
                            value,
                            &spec.addr,
                            spec.hash,
                            std::rc::Rc::clone(&spec.dtype),
                        );
                    }
                    continue;
                }
                // A UnionFacetSymbol (`map unionfacet`) must be re-created via
                // addUnionFacetSymbol so its category stays `union_facet` AND its
                // forced field number survives the rebuild — otherwise
                // ActionDynamicSymbols' `applyUnionFacet` arm (which reads
                // `getFieldNumber()`) never fires and the union store/load renders
                // through the default `resolveInFlow` field instead of the user's.
                // The console form is always non-addr-based (`addr_based == false`).
                if spec.category == symbol_category::UNION_FACET {
                    if let Some((field_num, _addr_based)) = spec.union_facet {
                        if let Ok(sym) = lm.add_union_facet_symbol(
                            &spec.name,
                            std::rc::Rc::clone(&spec.dtype),
                            field_num,
                            &spec.addr,
                            spec.hash,
                        ) {
                            lm.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
                        }
                    }
                    continue;
                }
                // A plain `map hash` dynamic symbol: namelock|typelock (the locks
                // the console set on it).
                if let Ok(sym) =
                    lm.add_dynamic_symbol(&spec.name, std::rc::Rc::clone(&spec.dtype), &spec.addr, spec.hash)
                {
                    lm.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
                }
            }
        }
    }

    /// Re-create the given console-mapped Symbols in this function's local scope
    /// and re-apply the `namelock|typelock` attributes (`IfcMapaddress`'s fd-local
    /// form).  The kuna console rebuilds the `Funcdata` on `decompile` (C++ reuses
    /// the same `fd`), so the `map addr` symbols are carried across here.
    pub fn seed_mapped_symbols(
        &mut self,
        specs: &[(String, std::rc::Rc<crate::dtype::Datatype>, Address, uint4)],
    ) {
        use crate::varnode::varnode_flags;
        let invalid = Address::new_invalid();
        if let Some(lm) = self.localmap.as_mut() {
            for (name, ct, addr, flags) in specs {
                if let Ok(sym) = lm.add_symbol(name, std::rc::Rc::clone(ct), addr, &invalid) {
                    // Re-apply the locks the console set (namelock|typelock and any
                    // inherited global property bits carried in `flags`).
                    let lock = flags & (varnode_flags::namelock | varnode_flags::typelock);
                    lm.set_attribute(sym, lock);
                }
            }
        }
    }

    /// Re-create the given usepoint-scoped console Symbols (the register-storage
    /// `type varnode %REG(pc)` symbols) in this function's local scope, mapping each
    /// at its recorded **use address** so its `SymbolEntry::uselimit` is restored.
    /// The usepoint-aware counterpart of
    /// [`seed_mapped_symbols`](Self::seed_mapped_symbols): the kuna console rebuilds
    /// the `Funcdata` on `decompile` (C++ reuses the same `fd`), and unlike the
    /// addr-tied `map addr` symbols these would be lost unless re-mapped with their
    /// usepoint (an invalid usepoint would make `inUse` false at every read).
    pub fn seed_usepoint_symbols(
        &mut self,
        specs: &[(String, std::rc::Rc<crate::dtype::Datatype>, Address, uint4, Address, bool)],
    ) {
        use crate::varnode::varnode_flags;
        if let Some(lm) = self.localmap.as_mut() {
            for (name, ct, addr, flags, usepoint, isolated) in specs {
                if let Ok(sym) = lm.add_symbol(name, std::rc::Rc::clone(ct), addr, usepoint) {
                    let lock = flags & (varnode_flags::namelock | varnode_flags::typelock);
                    lm.set_attribute(sym, lock);
                    // (kuna L4) Restore `Symbol::setIsolated(true)` for the
                    // `type varnode` register temp so `mergeAdjacent`'s isolated
                    // arm refuses to over-merge it into the param it shares storage
                    // with (Return Structure #1/#2/#4).
                    if *isolated {
                        lm.set_symbol_isolated(sym, true);
                    }
                }
            }
        }
    }

    /// (kuna, ghidra Phase 4) The local-scope Symbol **id** a HighVariable's
    /// name resolves to — the naming pass's recorded bind, an analysis-time
    /// dynamic/equate symbol, or a symbol the encode-time link pass
    /// materialized.  `None` when the high is deliberately symbol-less.
    ///
    /// This is what the markup's `<vardecl symref>` must carry: Java resolves
    /// it through `LocalSymbolMap`, whose ids are exactly the ones
    /// `<localdb>` encodes.  (Before Phase 4 the markup wrote a varnode
    /// create index here — a placeholder that can never resolve, which left
    /// rename/retype dead on declaration-line tokens.)
    pub fn kuna_high_symbol_wire_id(
        &self,
        high: crate::context::HighVariableId,
    ) -> Option<uint8> {
        // Every branch below returns only an id the `<localdb>` encode ACTUALLY
        // WRITES: `encode_scope`'s defensive per-symbol skips (and the wire
        // symbols' `is_encodable`) would otherwise leave the declaration
        // pointing at a symbol that is not in the document — "Invalid symbol
        // reference" in Java's log and a dead rename on that line, exactly what
        // this attribute exists to prevent.  The caller falls back to the
        // varnode create index, which is no worse.
        //
        // A wire-only symbol first (the encode-time link pass synthesized it
        // for a high the analysis left symbol-less), then the real scope
        // symbol the naming pass / analysis bound.
        if let Some(idx) = self.kuna_wire_symbol_for_high.get(&high) {
            if let Some(w) = self.kuna_wire_symbols.get(*idx) {
                if w.is_encodable() {
                    return Some(w.id);
                }
            }
        }
        let h = self.high_bank.get(high)?;
        // `kuna_ref_symbol` last of the recorded binds: it is the Symbol a
        // `&symbol` REFERENCE high points at (C++ `setSymbolReference`), which a
        // high that also owns storage would already have answered above.  It is
        // the only bind a stack aggregate reached solely through `&sym` has —
        // its entire high is the constant PTRSUB operand, so the addr-tied
        // re-derivation below cannot see it.
        if let Some(sid) = h
            .kuna_link_symbol()
            .or_else(|| h.kuna_dynamic_symbol())
            .or_else(|| h.kuna_equate_symbol())
            .or_else(|| h.kuna_ref_symbol())
        {
            let lm = self.localmap.as_ref()?;
            if lm.symbol_is_encodable(sid) {
                return Some(lm.symbol_id_and_category(sid).0);
            }
            return None;
        }
        // A declaration can be keyed on a GROUP MEMBER high (the printer
        // collapses the several highs of one mapped Symbol into a single
        // declaration) whose own symbol link lives on the sibling that carried
        // the name.  Resolve the covering local Symbol through the high's
        // addr-tied instances — the very query the declaration's name and type
        // came from (`kuna_mapped_symbol_entry`).  A conflict-separated high
        // never reaches here: the link pass already gave it a wire symbol of
        // its own, which the branch above returns.
        let lm = self.localmap.as_ref()?;
        let invalid = Address::new_invalid();
        let n = h.num_instances();
        for i in 0..n {
            let vn = h.get_instance(i);
            let Some(v) = self.vbank.get(vn) else { continue };
            if v.is_free() || !v.is_addr_tied() {
                continue;
            }
            if let Some((sid, _, _, _)) =
                lm.container_symbol_link(v.get_addr(), &invalid)
            {
                if !lm.symbol_is_encodable(sid) {
                    continue;
                }
                return Some(lm.symbol_id_and_category(sid).0);
            }
        }
        None
    }

    /// Seed name recommendations into the local scope (the ghidra-mode
    /// carrier for C++ `ScopeLocal::collectNameRecs` results: the host
    /// `<localdb>`'s namelocked-but-not-typelocked locals, i.e. GUI renames of
    /// untyped variables).  `(name, storage addr, usepoint, size)`; an invalid
    /// usepoint = address-tied.  Applied by the `ActionNameVars` port
    /// (`recoverNameRecommendationsForSymbols`).
    pub fn seed_name_recommendations(&mut self, specs: &[(String, Address, Address, int4)]) {
        if let Some(lm) = self.localmap.as_mut() {
            for (name, addr, usepoint, size) in specs {
                lm.add_recommend_name(addr.clone(), usepoint.clone(), *size, name);
            }
        }
    }

    /// Seed DYNAMIC name recommendations (the hash-storage half of the
    /// ghidra-mode carrier — C++ `ScopeLocal::dynRecommend`).  `(name,
    /// first-use address, hash)`: the Ghidra GUI writes hash storage for any
    /// variable that `requiresDynamicStorage` (unique-space representatives,
    /// `splitOutMergeGroup` products), so this is the channel a rename of such
    /// a variable comes back through.  Applied by
    /// [`Funcdata::kuna_apply_dynamic_recommendations`].
    pub fn seed_dynamic_recommendations(&mut self, specs: &[(String, Address, u64)]) {
        if let Some(lm) = self.localmap.as_mut() {
            for (name, addr, hash) in specs {
                lm.add_recommend_dynamic(addr.clone(), *hash, name);
            }
        }
    }

    /// Apply the dynamic name recommendations (C++
    /// `ScopeLocal::recoverNameRecommendationsForSymbols`'s `dynRecommend`
    /// loop, varmap.cc:1557-1573): resolve each recorded hash back to its
    /// Varnode with `DynamicHash::findVarnode`, and — when that Varnode's
    /// HighVariable is still unnamed — bind the recommended name AND a real
    /// dynamic Symbol carrying the SAME hash, so the wire `<localdb>` echoes
    /// a `<mapsym type="dynamic"><hash>` Java resolves to the very variable
    /// the user renamed.
    ///
    /// Runs at the top of the naming pass (upstream runs it at the top of
    /// `ActionNameVars::apply`); a no-op with no recommendations, so the
    /// standalone pipeline is structurally unaffected.
    ///
    /// PLACEMENT (a real divergence, guarded rather than reordered): upstream
    /// runs this loop AFTER `linkSymbols`, so `vn->getHigh()->getSymbol()` is
    /// already populated and its guards — `sym == 0`, `sym->getScope() != this`,
    /// `!sym->isNameUndefined()` — are live, and it only ever RENAMES an
    /// existing Symbol.  kuna's naming pass fuses linkSymbols with the `vN`
    /// default assignment into one location-ordered walk, so there is no point
    /// "after linking, before defaults" to run at; the loop therefore runs
    /// first and CREATES a dynamic Symbol.  The per-high guard below is then
    /// vacuous (no high is named yet), so the equivalent guard is applied
    /// against the SCOPE instead: a hash that lands on storage the naming walk
    /// would bind to a real Symbol — a cat-0 parameter, or any Symbol that
    /// already has a defined name — is skipped, which is what upstream's
    /// `sym != 0 && !isNameUndefined` pair achieves.  Without it a stale or
    /// shape-shifted host hash could take a parameter's variable, and that
    /// high's `<high symref>` would stop pointing at the parameter.
    pub fn kuna_apply_dynamic_recommendations(&mut self) {
        let recs: Vec<(String, Address, u64)> = match self.localmap.as_ref() {
            Some(lm) => lm
                .dynamic_recommendations()
                .iter()
                .map(|r| (r.name.clone(), r.use_point.clone(), r.hash))
                .collect(),
            None => return,
        };
        if recs.is_empty() {
            return;
        }
        for (name, addr, hash) in recs {
            // Java computes the same hash with a hardcoded budget of 8
            // (`DynamicHash.java:440`); use it so both sides agree.
            let mut dh = crate::dynamic::DynamicHash::new();
            let Some(vn) = dh.find_varnode(self, &addr, hash) else {
                continue;
            };

            let Some(high) = self.vbank().get(vn).and_then(|v| v.get_high()) else {
                continue;
            };
            if self.vbank().get(vn).map(|v| v.is_annotation()).unwrap_or(true) {
                continue;
            }
            // C++ `!sym->isNameUndefined()` — never paint over a resolved name.
            let already = self
                .high_bank()
                .get(high)
                .map(|h| {
                    h.kuna_name().is_some()
                        || h.kuna_dynamic_symbol().is_some()
                        || h.kuna_equate_symbol().is_some()
                        || h.kuna_link_symbol().is_some()
                })
                .unwrap_or(true);
            if already {
                continue;
            }
            // The scope-side stand-in for upstream's `sym == 0` /
            // `!sym->isNameUndefined()` pair (see the placement note above):
            // never take a Varnode whose storage the naming walk is going to
            // bind to a real Symbol.
            {
                let usepoint = self.vn_use_point(vn);
                let Some(v_addr) = self.vbank().get(vn).map(|v| v.get_addr().clone()) else {
                    continue;
                };
                let blocked = self
                    .localmap
                    .as_ref()
                    .and_then(|lm| lm.query_container_for_link(&v_addr, &usepoint))
                    .map(|info| {
                        info.category == crate::database::symbol_category::FUNCTION_PARAMETER
                            || !info.is_name_undefined
                    })
                    .unwrap_or(false);
                if blocked {
                    continue;
                }
            }
            let dtype = self.with_high_split(|hb, ctx| {
                hb.get_mut(high).expect("dyn rec: stale high").get_type(ctx, None)
            });
            let unique = self
                .localmap
                .as_ref()
                .map(|lm| lm.make_local_name_unique(&name))
                .unwrap_or_else(|| name.clone());
            let sid = self
                .localmap
                .as_mut()
                .and_then(|lm| lm.add_dynamic_symbol(&unique, dtype, &addr, hash).ok());
            if let Some(h) = self.high_bank_mut().get_mut(high) {
                h.set_kuna_name(unique);
                if let Some(sid) = sid {
                    h.set_kuna_link_symbol(sid);
                }
            }
        }
    }

    /// C++ `localmap->resetLocalWindow()` — reset the local-variable discovery
    /// window from the function prototype's stack ranges.  Faithful to the C++
    /// `Funcdata` constructor / `clear()` call cadence, but deferred until a
    /// proto model exists (the merged tree sets the model in the proto-recovery
    /// wave); a no-op when there is no local scope or no proto model yet.
    pub fn reset_local_window(&mut self) {
        if self.localmap.is_none() || !self.funcp.has_model() {
            return;
        }
        let local = self.funcp.get_local_range().clone();
        let param = self.funcp.get_param_range().clone();
        let grows_neg = self.funcp.is_stack_grows_negative();
        if let Some(sl) = self.localmap.as_mut() {
            sl.reset_local_window(&local, &param, grows_neg);
        }
    }
    /// Get the minimum laned-register size threshold (C++ `minLanedSize`).
    pub fn get_min_laned_size(&self) -> int4 {
        self.min_laned_size
    }
    /// Mark that laned registers have been collected (C++ `setLanedRegGenerated`).
    pub fn set_laned_reg_generated(&mut self) {
        self.min_laned_size = 1000000;
    }

    /// Record a laned-register storage location (C++
    /// `Funcdata::checkForLanedRegister`'s `lanedMap[storage] = lanedRegister`).
    /// The key carries the C++ `VarnodeData` ordering; the value keeps the
    /// `(addr, size)` so the access map can be replayed.
    pub(crate) fn laned_map_insert(
        &mut self,
        key: LanedKey,
        addr: Address,
        sz: int4,
        lr: crate::transform::LanedRegister,
    ) {
        self.laned_map.insert(key, (addr, sz, lr));
    }

    /// Snapshot the laned-register access map in C++ `lanedMap` iteration order
    /// (`beginLaneAccess()..endLaneAccess()`), as `(addr, size, record)` tuples.
    ///
    /// `ActionLaneDivide::apply` iterates this; the map itself is not mutated
    /// during the apply (only the Varnode bank is), so a snapshot reproduces the
    /// C++ `std::map` iteration exactly while sidestepping the borrow conflict
    /// with the mutable `processVarnode` calls inside the loop.
    pub fn lane_access_snapshot(&self) -> Vec<(Address, int4, crate::transform::LanedRegister)> {
        self.laned_map.values().cloned().collect()
    }

    /// Clear records from the laned-access list (C++
    /// `Funcdata::clearLanedAccessMap`, `funcdata.hh:408`).
    pub fn clear_laned_access_map(&mut self) {
        self.laned_map.clear();
    }

    // -----------------------------------------------------------------------
    // Flag query / toggle methods (C++ inline `is*`/`has*`/`set*`)
    // -----------------------------------------------------------------------

    /// Are high-level variables assigned to Varnodes (C++ `isHighOn`).
    pub fn is_high_on(&self) -> bool {
        (self.flags & funcdata_flags::highlevel_on) != 0
    }
    /// The raw function-state flag word (C++ `flags`).  Read by the jump-table
    /// recovery chain for the `jumptablerecovery_dont` short-circuit.
    pub fn flags(&self) -> uint4 {
        self.flags
    }
    /// Has processing of the function started (C++ `isProcStarted`).
    pub fn is_proc_started(&self) -> bool {
        (self.flags & funcdata_flags::processing_started) != 0
    }
    /// Is processing of the function complete (C++ `isProcComplete`).
    pub fn is_proc_complete(&self) -> bool {
        (self.flags & funcdata_flags::processing_complete) != 0
    }
    /// Did this function exhibit unreachable code (C++ `hasUnreachableBlocks`).
    pub fn has_unreachable_blocks(&self) -> bool {
        (self.flags & funcdata_flags::blocks_unreachable) != 0
    }
    /// Will data-type analysis be performed (C++ `isTypeRecoveryOn`).
    pub fn is_type_recovery_on(&self) -> bool {
        (self.flags & funcdata_flags::typerecovery_on) != 0
    }
    /// Has data-type recovery processes started (C++ `hasTypeRecoveryStarted`).
    pub fn has_type_recovery_started(&self) -> bool {
        (self.flags & funcdata_flags::typerecovery_start) != 0
    }
    /// Has maximum propagation passes been reached (C++ `isTypeRecoveryExceeded`).
    pub fn is_type_recovery_exceeded(&self) -> bool {
        (self.flags & funcdata_flags::typerecovery_exceeded) != 0
    }
    /// Will normalization be performed (C++ `isNormalizationOn`).
    pub fn is_normalization_on(&self) -> bool {
        (self.flags & funcdata_flags::normalization_on) != 0
    }
    /// Return \b true if \b this function has no code body (C++ `hasNoCode`).
    pub fn has_no_code(&self) -> bool {
        (self.flags & funcdata_flags::no_code) != 0
    }
    /// Toggle whether \b this has a body (C++ `setNoCode`).
    pub fn set_no_code(&mut self, val: bool) {
        if val {
            self.flags |= funcdata_flags::no_code;
        } else {
            self.flags &= !funcdata_flags::no_code;
        }
    }
    /// Toggle whether \b this is being used for jump-table recovery
    /// (C++ `setJumptableRecovery`).
    pub fn set_jumptable_recovery(&mut self, val: bool) {
        if val {
            self.flags &= !funcdata_flags::jumptablerecovery_dont;
        } else {
            self.flags |= funcdata_flags::jumptablerecovery_dont;
        }
    }
    /// Is \b this used for jump-table recovery (C++ `isJumptableRecoveryOn`).
    pub fn is_jumptable_recovery_on(&self) -> bool {
        (self.flags & funcdata_flags::jumptablerecovery_on) != 0
    }
    /// Toggle whether double precision analysis is used
    /// (C++ `setDoublePrecisRecovery`).
    pub fn set_double_precis_recovery(&mut self, val: bool) {
        if val {
            self.flags |= funcdata_flags::double_precis_on;
        } else {
            self.flags &= !funcdata_flags::double_precis_on;
        }
    }
    /// Is double precision analysis enabled (C++ `isDoublePrecisOn`).
    pub fn is_double_precis_on(&self) -> bool {
        (self.flags & funcdata_flags::double_precis_on) != 0
    }
    /// Return \b true if no block structuring was performed
    /// (C++ `hasNoStructBlocks`).
    pub fn has_no_struct_blocks(&self) -> bool {
        self.sblocks_get_size() == 0
    }
    /// Mark that data-type analysis has started (C++ `startTypeRecovery`).
    pub fn start_type_recovery(&mut self) -> bool {
        if (self.flags & funcdata_flags::typerecovery_start) != 0 {
            return false; // Already started
        }
        self.flags |= funcdata_flags::typerecovery_start;
        true
    }
    /// Toggle whether data-type recovery will be performed (C++ `setTypeRecovery`).
    pub fn set_type_recovery(&mut self, val: bool) {
        self.flags = if val {
            self.flags | funcdata_flags::typerecovery_on
        } else {
            self.flags & !funcdata_flags::typerecovery_on
        };
    }
    /// Mark propagation passes have reached maximum (C++ `setTypeRecoveryExceeded`).
    pub fn set_type_recovery_exceeded(&mut self) {
        self.flags |= funcdata_flags::typerecovery_exceeded;
    }
    /// Toggle whether normalization transforms will be performed
    /// (C++ `setNormalization`).
    pub fn set_normalization(&mut self, val: bool) {
        self.flags = if val {
            self.flags | funcdata_flags::normalization_on
        } else {
            self.flags & !funcdata_flags::normalization_on
        };
    }
    /// Toggle whether analysis needs to be restarted (C++ `setRestartPending`).
    pub fn set_restart_pending(&mut self, val: bool) {
        self.flags = if val {
            self.flags | funcdata_flags::restart_pending
        } else {
            self.flags & !funcdata_flags::restart_pending
        };
    }
    /// Does \b this function need to restart its analysis (C++ `hasRestartPending`).
    pub fn has_restart_pending(&self) -> bool {
        (self.flags & funcdata_flags::restart_pending) != 0
    }
    /// Does \b this function have unimplemented instructions (C++ `hasUnimplemented`).
    pub fn has_unimplemented(&self) -> bool {
        (self.flags & funcdata_flags::unimplemented_present) != 0
    }
    /// Does \b this function flow into bad data (C++ `hasBadData`).
    pub fn has_bad_data(&self) -> bool {
        (self.flags & funcdata_flags::baddata_present) != 0
    }

    // -----------------------------------------------------------------------
    // Creation-index phase machinery (C++ inline, driven by vbank create index)
    // -----------------------------------------------------------------------

    /// Start the \b cast insertion phase (C++ `startCastPhase`).
    pub fn start_cast_phase(&mut self) {
        self.cast_phase_index = self.vbank.get_create_index();
    }
    /// Get creation index at the start of \b cast insertion (C++ `getCastPhaseIndex`).
    pub fn get_cast_phase_index(&self) -> uint4 {
        self.cast_phase_index
    }
    /// Get creation index at the start of HighVariable creation
    /// (C++ `getHighLevelIndex`).
    pub fn get_high_level_index(&self) -> uint4 {
        self.high_level_index
    }
    /// Start \e clean-up phase (C++ `startCleanUp`).
    pub fn start_clean_up(&mut self) {
        self.clean_up_index = self.vbank.get_create_index();
    }
    /// Get creation index at the start of \b clean-up phase (C++ `getCleanUpIndex`).
    pub fn get_clean_up_index(&self) -> uint4 {
        self.clean_up_index
    }

    // -----------------------------------------------------------------------
    // IR container access (the seam funcdata_op/funcdata_varnode build on)
    // -----------------------------------------------------------------------

    /// Borrow the Varnode container (C++ `vbank`).
    pub fn vbank(&self) -> &VarnodeBank {
        &self.vbank
    }
    /// Mutably borrow the Varnode container.
    pub fn vbank_mut(&mut self) -> &mut VarnodeBank {
        &mut self.vbank
    }
    /// Borrow the PcodeOp container (C++ `obank`).
    pub fn obank(&self) -> &PcodeOpBank {
        &self.obank
    }
    /// Mutably borrow the PcodeOp container.
    pub fn obank_mut(&mut self) -> &mut PcodeOpBank {
        &mut self.obank
    }

    /// Split-borrow the Varnode and PcodeOp containers **simultaneously**
    /// (the accessor `funcdata_op.cc`/`funcdata_varnode.cc` documented they need).
    ///
    /// In C++ the two banks are plain members of `Funcdata` and every method
    /// aliases them freely; the read-repointing `xref` callback (`replace_reads`)
    /// runs *inside* a `vbank` mutation yet reaches `obank`.  Rust forbids holding
    /// two `&mut` through separate `&mut self` accessors, so the
    /// `vbank.setInput`/`setDef`/`createDef` callers (`opSetOutput`,
    /// `setInputVarnode`, `newVarnodeOut`/`newUniqueOut`) split-borrow here and
    /// build [`replace_reads_thunk`](Funcdata::replace_reads_thunk) over the `obank`
    /// half while mutating the `vbank` half.
    ///
    /// `pub(crate)` so only the funcdata_op/funcdata_varnode ports reach it.
    pub(crate) fn banks_mut(&mut self) -> (&mut VarnodeBank, &mut PcodeOpBank) {
        // Disjoint borrows of two distinct fields: the borrow checker accepts
        // this single split, where two separate `&mut self` accessors would not.
        (&mut self.vbank, &mut self.obank)
    }
    /// Get the total number of Varnodes (C++ `numVarnodes`).
    pub fn num_varnodes(&self) -> int4 {
        self.vbank.num_varnodes()
    }

    /// Get the basic blocks container (C++ `getBasicBlocks`).
    pub fn bblocks_ref(&self) -> &BlockGraph {
        &self.bblocks
    }
    /// Mutably borrow the basic blocks container.
    pub fn bblocks_mut(&mut self) -> &mut BlockGraph {
        &mut self.bblocks
    }
    /// Get the control-flow structuring hierarchy (C++ `getStructure`).
    pub fn sblocks_ref(&self) -> &BlockGraph {
        &self.sblocks
    }
    /// Mutably borrow the structuring hierarchy.
    pub fn sblocks_mut(&mut self) -> &mut BlockGraph {
        &mut self.sblocks
    }

    /// The root graph node of `bblocks` (the C++ `bblocks` *is* this graph; its
    /// `list` holds the basic blocks).
    fn bblocks_root(&self) -> BlockId {
        self.bblocks.root.expect("Funcdata: bblocks root not constructed (internal invariant)")
    }
    /// Number of basic blocks (C++ `bblocks.getSize()`).
    pub fn bblocks_get_size(&self) -> int4 {
        let root = self.bblocks_root();
        self.bblocks.block(root).get_size()
    }
    /// The i-th basic block (C++ `bblocks.getBlock(i)`).
    pub fn bblocks_get_block(&self, i: int4) -> BlockId {
        let root = self.bblocks_root();
        self.bblocks.block(root).get_block(i)
    }
    /// The starting code address of a basic block (C++ `FlowBlock::getStart`).
    /// Used to place a forced-input extension op at the function entry block.
    pub fn bblocks_block_start(&self, bl: BlockId) -> Address {
        crate::block::block_get_start(&self.bblocks.arena, bl)
    }
    /// The root graph node of `sblocks`.
    pub(crate) fn sblocks_root(&self) -> BlockId {
        self.sblocks.root.expect("Funcdata: sblocks root not constructed (internal invariant)")
    }
    /// Number of structured blocks (C++ `sblocks.getSize()`).
    pub fn sblocks_get_size(&self) -> int4 {
        let root = self.sblocks_root();
        self.sblocks.block(root).get_size()
    }

    /// (kuna) Tally the `quality` goto-count structure-quality metric over the
    /// structured tree (C++ `IfcKunaQuality`).  Pub wrapper over the private
    /// `sblocks_root` so the console command can read the metric.
    pub fn kuna_quality_counts(&self) -> crate::block::KunaQualityCounts {
        let mut counts = crate::block::KunaQualityCounts::default();
        let root = self.sblocks_root();
        self.sblocks.kuna_count_quality(root, &mut counts);
        counts
    }

    /// Seed `sblocks` with a `BlockCopy` mirror of every `bblocks` basic block
    /// (the first half of C++ `ActionBlockStructure::apply`, blockaction.cc:2170 —
    /// `graph.buildCopy(data.getBasicBlocks())`).  Borrows `sblocks` mutably and
    /// `bblocks` immutably at once (distinct fields) so the cross-arena
    /// [`BlockGraph::build_copy_from`] can mirror the topology.  The
    /// [`CollapseStructure`](crate::blockaction::CollapseStructure) engine then
    /// runs over the seeded `sblocks` (driven by `ActionBlockStructure`).
    pub(crate) fn seed_sblocks_copy(&mut self) {
        let sroot = self.sblocks.root.expect("sblocks root");
        let broot = self.bblocks.root.expect("bblocks root");
        // C++ `buildCopy` writes `(*iter)->copymap = copyblock` back into each
        // SOURCE basic block (block.cc:1938).  The cross-arena port returns the
        // src(bblocks)->dst(sblocks BlockCopy) map; apply it here.
        let copymap = self.sblocks.build_copy_from(sroot, &self.bblocks, broot);
        for (src, dst) in copymap {
            self.bblocks.set_copy_map(src, Some(dst));
        }
    }

    /// (kuna) Discard a partially-structured `sblocks` graph and rebuild a fresh
    /// `BlockCopy` mirror of `bblocks`.  Used only by the region-based structurer
    /// fallback (`option regionstructure`): when the region structurer cannot
    /// collapse the graph to a single root it leaves `sblocks` half-structured, so
    /// this resets it to a clean seed before `CollapseStructure` runs.  Replaces
    /// `sblocks` with a fresh empty `BlockGraph` (the half-structured arena is
    /// dropped) and re-runs [`seed_sblocks_copy`](Self::seed_sblocks_copy).
    pub(crate) fn reseed_sblocks_copy(&mut self) {
        let mut sb = BlockGraph::new();
        let sroot = sb.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        sb.root = Some(sroot);
        self.sblocks = sb;
        self.seed_sblocks_copy();
    }

    // -----------------------------------------------------------------------
    // Jump-table identity (W4 contents seamed out)
    // -----------------------------------------------------------------------

    /// Get the number of jump-tables for \b this function (C++ `numJumpTables`).
    pub fn num_jump_tables(&self) -> int4 {
        self.jumpvec.len() as int4
    }
    /// Get the i-th jump-table (C++ `getJumpTable`).
    pub fn get_jump_table(&self, i: int4) -> &crate::jumptable::JumpTable {
        &self.jumpvec[i as usize]
    }
    /// Mutable access to the i-th jump-table.
    pub fn get_jump_table_mut(&mut self, i: int4) -> &mut crate::jumptable::JumpTable {
        &mut self.jumpvec[i as usize]
    }
    /// Mutable access to the jump-table vector (for the recovery chain and the
    /// `clear_jump_tables`/`structure_reset` sweeps).
    pub(crate) fn jumpvec_mut(&mut self) -> &mut Vec<crate::jumptable::JumpTable> {
        &mut self.jumpvec
    }
    /// Immutable slice of the jump-table vector (recovery-chain read accessor).
    pub(crate) fn jumpvec_slice(&self) -> &[crate::jumptable::JumpTable] {
        &self.jumpvec
    }

    // -----------------------------------------------------------------------
    // VarnodeBank callbacks (the seam varnode.rs documented)
    // -----------------------------------------------------------------------

    /// Build the `replace_reads` callback `VarnodeBank::xref` invokes when it
    /// unifies a fresh varnode with an existing equivalent free varnode.
    ///
    /// In the C++ this is the inline read-repointing inside `xref` (a
    /// `totalReplace` of `oldvn` by `newvn`): for every op reading `oldvn`,
    /// repoint that input slot to `newvn` and add the op to `newvn`'s descend
    /// list.  Because it runs *inside* a `&mut VarnodeBank` borrow it cannot also
    /// borrow `self.obank`; the closure therefore captures `&mut self.obank`
    /// only and is handed the bank as its first argument, exactly as
    /// [`crate::varnode::ReplaceReads`] is typed.
    ///
    /// STUB(W3-op): the op-side read iteration/repointing is the funcdata_op
    /// wave's; this method establishes the closure shape and where it captures.
    /// Until funcdata_op ports `opSetInput`, the bodies that *call* `xref`
    /// (`setInputVarnode`, `opSetOutput`) live in funcdata_op; this thunk is the
    /// surface they use, declared here so they need no seam edit.
    pub fn replace_reads_thunk(obank: &mut PcodeOpBank) -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> + '_ {
        move |bank: &mut VarnodeBank, oldvn: VarnodeId, newvn: VarnodeId| -> KunaResult<()> {
            // C++ `VarnodeBank::replace` (varnode.cc:1351).
            // C++ walks oldvn's descend list (one entry per op-read of oldvn) and,
            // for each non-skipped entry, severs *that one* link, repoints the
            // single slot getSlot finds, and adds the op to newvn's descend.
            //
            // Iterate a snapshot in descend (push_back) order since we mutate the
            // list; mirror the `iter++` cursor by erasing exactly the visited link
            // (not a blanket destroy) so the self-def skip leaves oldvn's link to
            // that op intact, just as C++ does.
            let readers: Vec<OpId> = bank
                .get(oldvn)
                .map(|vn| vn.descend_iter().collect())
                .unwrap_or_default();
            for op in readers {
                // An op cannot be an input to its own definition; leave its slot
                // reading oldvn and leave oldvn's descend link to it untouched.
                if obank.get(op).and_then(|o| o.get_out()) == Some(newvn) {
                    continue;
                }
                // The first slot reading oldvn; this descend entry corresponds to
                // exactly that read.  (-1 only if a prior entry for the same op
                // already consumed the read, leaving none — then there is no slot
                // to repoint and no link to sever.)
                let i = obank.get(op).map(|o| o.get_slot(oldvn)).unwrap_or(-1);
                if i < 0 {
                    continue;
                }
                // Sever just this one link.
                bank.erase_descend(oldvn, op);
                bank.add_descend(newvn, op)?;
                if let Some(o) = obank.get_mut(op) {
                    o.set_input(Some(newvn), i);
                }
            }
            Ok(())
        }
    }

    /// Map an `OpId` to its `(getAddr, getTime)` for `VarnodeBank::find`
    /// (the def-op address/time confirmation, C++ inline in `find`).
    pub fn def_addr_time(&self, op: OpId) -> (Address, uintm) {
        let o = self.obank.get(op).expect("def_addr_time: stale op (internal invariant)");
        (o.get_addr().clone(), o.get_time())
    }

    /// The point at which a Varnode first comes into scope (C++
    /// `Varnode::getUsePoint`, `varnode.cc:715`): the def-op's address if written,
    /// else `fd.getAddress() + -1`.  Used as the `usepoint` of a
    /// `queryProperties` look-up.
    ///
    /// Consumed by [`Funcdata::set_varnode_properties`] as the `usepoint` of the
    /// global `queryProperties` look-up, and by `linkSymbol`'s
    /// `query_container_for_link` (the local-scope usepoint-scoped Symbol query).
    pub(crate) fn vn_use_point(&self, vn: VarnodeId) -> Address {
        let v = self.vbank().get(vn).expect("vn_use_point: stale vn");
        if v.is_written() {
            if let Some(op) = v.get_def() {
                return self.obank().get(op).expect("vn_use_point: stale def").get_addr().clone();
            }
        }
        // fd.getAddress()+-1
        &self.get_address().clone() + -1
    }

    /// Identity of the smallest containing local SymbolEntry for a Varnode — the
    /// kuna analog of C++ `Varnode::getSymbolEntry()` (the `mapentry` pointer).
    ///
    /// Re-derives the entry with the same containment query `linkSymbol` uses
    /// (`localmap->findContainer(addr, 1, vn->getUsePoint())`) and keys it by
    /// `(SymbolId, entry-base-offset, entry-size)`.  Returns `None` when the
    /// Varnode is not mapped into the local scope (no containing entry).  Used by
    /// `RulePieceStructure`/`PieceNode::isLeaf` to compare two Varnodes' entries.
    pub(crate) fn vn_container_entry_key(
        &self,
        vn: VarnodeId,
    ) -> Option<(crate::database::SymbolId, kuna_base::types::uintb, int4)> {
        let usepoint = self.vn_use_point(vn);
        let addr = self.vbank().get(vn)?.get_addr().clone();
        self.get_scope_local()?.container_entry_key(&addr, &usepoint)
    }

    /// Look-up boolean properties and data-type information for a Varnode
    /// (C++ `Funcdata::setVarnodeProperties`, `funcdata_varnode.cc:25`).
    ///
    /// where `localmap->queryProperties` reaches the global scope, so a
    /// global-mapped Varnode would pick up `mapped | addrtied | persist` at every
    /// Varnode-creation site (`newVarnode`/`newVarnodeOut`/`setInput`).
    ///
    /// DEFERRED (the persist/addrtied marking is a no-op here, as in the W3 base):
    /// the global-store *survival* this item targets is delivered instead by the
    /// heritage path — `Heritage::guard` queries `query_global_properties` for the
    /// same `mapped | addrtied | persist` directly and `guard_returns` inserts the
    /// `addrforce` RETURN-COPY that keeps the store's def-chain alive through
    /// `ActionDeadCode`.  That path is sufficient for every global-store datatest
    /// (displayformat, condconst, varcross), so this early marking is redundant
    /// for the target.
    ///
    /// Marking persist/addrtied *here* (at IR construction, on every global READ as
    /// well) was measured to regress `varcross.xml::global_cross` ("Global cross
    /// #2", a positive-content assertion): the early `addrtied` flag perturbs the
    /// HighVariable merge so the recovered global-flow register (`v1`) renders as a
    /// raw register instead of its name — the downstream HighVariable-naming /
    /// global-store render seams (`merge.rs`/`variable.rs`/`printc.rs`, owned by the
    /// naming/render waves) are not yet landed.  Activating it gains **zero** passing
    /// assertions over the heritage path while regressing `global_cross`, so it is
    /// held until the naming seam lands (matrix in
    /// `docs/rust-port/reviews/w10-global-persist.md`).  When that seam lands the
    /// body above folds back in unchanged.
    pub fn set_varnode_properties(&mut self, vn: VarnodeId) {
        // An already-mapped Varnode keeps its flags.
        let already_mapped = match self.vbank().get(vn) {
            Some(v) => v.is_mapped(),
            None => return,
        };
        if !already_mapped {
            // Both halves of the C++ walk are now wired: the LOCAL symbol-map half
            // (recovered stack locals, via `ScopeLocal::query_properties`) and the
            // GLOBAL half — the parent-scope reach that paints a global-mapped RAM
            // store `mapped | addrtied | persist` (+ `readonly`/`volatile`), the
            // `GlobalQuery` snapshot wired onto `glb`.  For a register / unique
            // address both queries return 0, so such Varnodes are untouched, exactly
            // as the C++ local-then-global walk leaves them with `vflags == 0`.
            //
            // Marking `addrtied`/`persist` here is the keystone the merge + naming
            // seams need: `Merge::mergeTestSpeculative` (merge.cc:226-233) refuses to
            // speculatively merge `persist`/`addrtied` highs, so two distinct global
            // stores of the same data-type (`globalfree`/`globaloct`) stay in
            // separate HighVariables instead of collapsing into one; and the global
            // store's HighVariable reports `addrtied`/`persist` so it carries its
            // global Symbol name in `ActionNameVars` rather than a `dat_`/`Unique`.
            let usepoint = self.vn_use_point(vn);
            let (addr, size) = match self.vbank().get(vn) {
                Some(v) => (v.get_addr().clone(), v.get_size()),
                None => return,
            };
            use crate::varnode::varnode_flags;
            // The LOCAL-scope half of `setVarnodeProperties` (funcdata_varnode.cc:30).
            // A `map addr`/restructure-mapped stack range returns `mapped|addrtied`
            // here, so a Varnode freshly created at a mapped stack address (e.g. a
            // per-byte COPY output split out of a wide stack COPY by `RuleSplitCopy`
            // in the cleanup pool, just before `RuleStringCopy`) is marked
            // address-tied — the keystone `RuleStringCopy::applyOp`'s
            // `outvn->isAddrTied()` guard reads.  OR'd with the global half below.
            let mut vflags = self.query_local_properties(&addr, size, &usepoint);
            vflags |= self.get_arch().query_global_properties(&addr, size, &usepoint);
            if vflags != 0 {
                // typelock is set by `updateType`, never here.  These flags
                // (`mapped|addrtied|persist`, plus any `readonly`/`volatile`) are
                // what survival + merge + naming read.
                if let Some(v) = self.vbank_mut().get_mut(vn) {
                    v.set_flags_pub(vflags & !varnode_flags::typelock);
                }
            }
            // C++ `Varnode::setSymbolProperties` (varnode.cc:429) -> `entry->updateType`
            // (database.cc:136): a type-locked covering Symbol forces its
            // `getSizedType` onto the Varnode via `updateType(dt, lock=true,
            // override=true)`.  This is the type-force half of the global query that
            // the prior flag-only stand-in deferred: it seeds `ActionInferTypes` from
            // the mapped global's data-type (e.g. `octint4` for `globaloct`), so the
            // forced display format propagates through the store's COPY to the stored
            // constant (`globaloct = 05555`).  The local `ScopeLocal` half (recovered
            // stack locals) remains the naming wave's seam.
            if let Some((symtype, off)) =
                self.get_arch().sized_type_for_global_varnode(&addr, size, &usepoint)
            {
                // The exact type piece against the shared TypeFactory; null (no
                // exact piece) leaves the Varnode untyped, as C++ `getSizedType`
                // returning NULL skips the `updateType`.
                let dt = self
                    .get_arch()
                    .types()
                    .and_then(|t| t.get_exact_piece(symtype, off, size).ok().flatten());
                if let Some(dt) = dt {
                    if let Some(v) = self.vbank_mut().get_mut(vn) {
                        v.update_type_locked(dt, true, true);
                    }
                }
            }
        }
        // C++ `if (vn->cover == 0) { if (isHighOn()) vn->calcCover(); }`
        // (funcdata_varnode.cc:42).  This ALLOCATES the Varnode's Cover object (and
        // sets `coverdirty`) the first time `setVarnodeProperties` runs on a
        // covered, high-enabled Varnode — for example the fresh `newUnique` output
        // of a `Merge::allocateCopyTrim` COPY, which `opSetOutput` routes through
        // here right after `setDef` marks it written.  Without this, the trim
        // COPY's `cover` field stays `None`; the lazy `Varnode::updateCover`
        // (`update_varnode_cover`) only rebuilds a cover it can clone out (i.e. a
        // `Some(_)`), so a `None` cover never gets rebuilt and the Varnode
        // contributes an EMPTY cover to its HighVariable.  That fragments a
        // register's merged high (the loop-counter phi-temps lose their back-edge
        // span), letting an unrelated same-typed value speculatively merge into it
        // — the gh1276 / gh9218 cover-fragmentation bug.  (kuna fix)
        let needs_calc = match self.vbank().get(vn) {
            Some(v) => v.cover().is_none(),
            None => return,
        };
        if needs_calc && self.is_high_on() {
            if let Some(v) = self.vbank_mut().get_mut(vn) {
                v.calc_cover();
            }
        }
    }

    // -----------------------------------------------------------------------
    // Basic-block op-list manipulation (cross-arena; the seam opInsert* needs)
    // -----------------------------------------------------------------------
    //
    // These reproduce `BlockBasic::insert`/`removeOp`/`setOrder` (block.cc) but
    // live on `Funcdata` because they touch both the op arena (the per-op basic
    // links + order) and the block arena (the BasicData head/tail/len).  The op
    // membership links are the third intrusive list of ADR 0001.

    /// First op in basic block `bl` (C++ `BlockBasic::beginOp()` front /
    /// `firstOp`), `None` when empty.
    pub fn bb_op_head(&self, bl: BlockId) -> Option<OpId> {
        match self.bblocks.block(bl).kind() {
            BlockKind::Basic(b) => b.op_head,
            _ => None,
        }
    }
    /// Last op in basic block `bl` (C++ `BlockBasic::lastOp`), `None` when empty.
    pub fn bb_op_tail(&self, bl: BlockId) -> Option<OpId> {
        match self.bblocks.block(bl).kind() {
            BlockKind::Basic(b) => b.op_tail,
            _ => None,
        }
    }
    /// The op following `op` in its basic block's intrusive op list (C++
    /// `++iter` over `bb->beginOp()`), `None` at the end of the block.  The
    /// printer's `emitBlockBasic` walks the block ops with this.
    pub fn bb_op_next(&self, op: OpId) -> Option<OpId> {
        self.obank.get(op).and_then(|o| o.basic_neighbours().1)
    }
    /// Number of ops in basic block `bl` (C++ `op.size()`).
    pub fn bb_op_len(&self, bl: BlockId) -> usize {
        match self.bblocks.block(bl).kind() {
            BlockKind::Basic(b) => b.op_len,
            _ => 0,
        }
    }
    /// Return \b true if basic block `bl` contains no operations (C++ `emptyOp`).
    pub fn bb_empty_op(&self, bl: BlockId) -> bool {
        self.bb_op_len(bl) == 0
    }

    /// Mutable access to a block's [`BasicData`] (panics if `bl` is not basic).
    fn basic_data_mut(&mut self, bl: BlockId) -> &mut BasicData {
        match self.bblocks.block_mut(bl).kind_mut() {
            BlockKind::Basic(b) => b,
            _ => panic!("Funcdata: expected BlockBasic (internal invariant)"),
        }
    }

    /// Insert `op` into basic block `bl` immediately before `before` (or at the
    /// end if `before` is `None`), assigning the SeqNum order index.
    ///
    /// C++ `BlockBasic::insert` (`block.cc:2262`): set the
    /// op's parent, splice it onto the per-op intrusive basic-block list before
    /// `before`, then compute `ordbefore`/`ordafter` from neighbours and either
    /// recompute the whole order ([`bb_set_order`](Funcdata::bb_set_order)) or
    /// set the midpoint (overflow-aware).  The BRANCHIND `f_switch_out` mark is
    /// applied via the block flags.
    pub fn bb_insert_op(&mut self, op: OpId, bl: BlockId, before: Option<OpId>) {
        self.obank.get_mut(op).expect("bb_insert_op: stale op").set_parent(Some(bl));

        // Determine the predecessor `prev` of the insertion point.  Inserting
        // before `before` means `prev = before->basic_prev`; before == None
        // (end) means `prev = tail`.
        let prev: Option<OpId> = match before {
            Some(b) => self.obank.get(b).expect("bb_insert_op: stale before").basic_neighbours().0,
            None => self.bb_op_tail(bl),
        };

        // Splice `op` between `prev` and `before`.
        self.op_set_basic_prev(op, prev);
        self.op_set_basic_next(op, before);
        match prev {
            Some(p) => self.op_set_basic_next(p, Some(op)),
            None => self.basic_data_mut(bl).op_head = Some(op),
        }
        match before {
            Some(b) => self.op_set_basic_prev(b, Some(op)),
            None => self.basic_data_mut(bl).op_tail = Some(op),
        }
        self.basic_data_mut(bl).op_len += 1;

        // ordbefore: if newiter == op.begin() => 2 (minimum possible) else the
        // order of the preceding op.
        let ordbefore: uintm = match prev {
            None => 2,
            Some(p) => self.obank.get(p).expect("bb_insert_op").get_seq_num().get_order(),
        };
        // ordafter: if iter == op.end() => ordbefore + 0x1000000 (clamped to ~0
        // on overflow) else the order of the op we inserted before.
        let ordafter: uintm = match before {
            None => {
                let oa = ordbefore.wadd(0x1000000);
                if oa <= ordbefore {
                    uintm::MAX
                } else {
                    oa
                }
            }
            Some(b) => self.obank.get(b).expect("bb_insert_op").get_seq_num().get_order(),
        };
        if ordafter.wsub(ordbefore) <= 1 {
            self.bb_set_order(bl);
        } else {
            // beware overflow
            let mid = (ordafter / 2).wadd(ordbefore / 2);
            self.obank.get_mut(op).expect("bb_insert_op").set_order(mid);
        }

        if self.obank.get(op).expect("bb_insert_op").is_branch()
            && self.obank.get(op).expect("bb_insert_op").code()
                == kuna_num::opcodes::OpCode::CPUI_BRANCHIND
        {
            self.bblocks.block_mut(bl).set_flag(block_flags::f_switch_out);
        }
    }

    /// Remove `op` from its basic block `bl` (C++ `BlockBasic::removeOp`,
    /// `block.cc:2296`).  `op` \e must be in `bl`.  Clears the op's parent and
    /// splices it out of the per-op intrusive list, fixing head/tail/len.
    pub fn bb_remove_op(&mut self, bl: BlockId, op: OpId) {
        self.obank.get_mut(op).expect("bb_remove_op: stale op").set_parent(None);
        let (prev, next) = self.obank.get(op).expect("bb_remove_op").basic_neighbours();
        match prev {
            Some(p) => self.op_set_basic_next(p, next),
            None => self.basic_data_mut(bl).op_head = next,
        }
        match next {
            Some(n) => self.op_set_basic_prev(n, prev),
            None => self.basic_data_mut(bl).op_tail = prev,
        }
        // Detach the removed op's own links.
        self.op_set_basic_prev(op, None);
        self.op_set_basic_next(op, None);
        let len = self.bb_op_len(bl);
        self.basic_data_mut(bl).op_len = len - 1;
    }

    /// Recompute the SeqNum order field for every op in basic block `bl`
    /// (C++ `BlockBasic::setOrder`, `block.cc:2686`).
    ///
    /// `step = (~0 / op.size()) - 1`; each op gets `count += step`.
    pub fn bb_set_order(&mut self, bl: BlockId) {
        let n = self.bb_op_len(bl);
        if n == 0 {
            return;
        }
        let step = (uintm::MAX / n as uintm).wsub(1);
        let mut count: uintm = 0;
        let mut cur = self.bb_op_head(bl);
        while let Some(op) = cur {
            count = count.wadd(step);
            self.obank.get_mut(op).expect("bb_set_order").set_order(count);
            cur = self.obank.get(op).expect("bb_set_order").basic_neighbours().1;
        }
    }

    /// Iterate the ops of basic block `bl` in list order (head..tail).
    pub fn bb_ops(&self, bl: BlockId) -> Vec<OpId> {
        let mut out = Vec::with_capacity(self.bb_op_len(bl));
        let mut cur = self.bb_op_head(bl);
        while let Some(op) = cur {
            out.push(op);
            cur = self.obank.get(op).expect("bb_ops").basic_neighbours().1;
        }
        out
    }

    /// C++ `BlockBasic::noInterveningStatement` (`block.cc`): \b true if the block
    /// contains no statement that would have to be emitted before a switch is
    /// reached — i.e. every op is a marker/branch, a side-effect-free COPY/SUBPIECE,
    /// or its output is used only within this block (so folding the guard branch
    /// directly into the switch does not strand a visible statement).
    pub fn block_no_intervening_statement(&self, bl: BlockId) -> bool {
        use crate::op::pcodeop_flags;
        for bop in self.bb_ops(bl) {
            let o = match self.obank().get(bop) {
                Some(o) => o,
                None => continue,
            };
            if o.is_marker() {
                continue;
            }
            if o.is_branch() {
                continue;
            }
            if o.get_eval_type() == pcodeop_flags::special {
                if o.is_call() {
                    return false;
                }
                let opc = o.code();
                if opc == OpCode::CPUI_STORE || opc == OpCode::CPUI_NEW {
                    return false;
                }
            } else {
                let opc = o.code();
                if opc == OpCode::CPUI_COPY || opc == OpCode::CPUI_SUBPIECE {
                    continue;
                }
            }
            let outvn = match o.get_out() {
                Some(v) => v,
                None => continue,
            };
            if self.vbank().get(outvn).map(|v| v.is_addr_tied()).unwrap_or(false) {
                return false;
            }
            // Every use of the output must be inside this same block.
            let descend: Vec<OpId> =
                self.vbank().get(outvn).map(|v| v.descend_iter().collect()).unwrap_or_default();
            for dop in descend {
                if self.obank().get(dop).and_then(|d| d.get_parent()) != Some(bl) {
                    return false;
                }
            }
        }
        true
    }

    // Thin op-link setters so bb_* helpers don't repeatedly unwrap.
    fn op_set_basic_prev(&mut self, op: OpId, v: Option<OpId>) {
        self.obank.get_mut(op).expect("op_set_basic_prev: stale op").set_basic_prev(v);
    }
    fn op_set_basic_next(&mut self, op: OpId, v: Option<OpId>) {
        self.obank.get_mut(op).expect("op_set_basic_next: stale op").set_basic_next(v);
    }

    // -----------------------------------------------------------------------
    // clear / printRaw (W3-portable / seam-noted)
    // -----------------------------------------------------------------------

    /// Clear everything associated with decompilation analysis
    /// (C++ `Funcdata::clear`, `funcdata.cc:84`).
    ///
    /// The W4+ subsystem clears (`localmap->clearUnlocked`, `funcp`,
    /// `clearActiveOutput`, `unionMap`, `clearCallSpecs`, `clearJumpTables`,
    /// `heritage.clear`, `covermerge.clear`) are seam-noted; the W3 IR clears
    /// (`clearBlocks`, `obank.clear`, `vbank.clear`) and the flag/index reset are
    /// faithful.
    pub fn clear(&mut self) {
        // Clear the analysis-derived flags (the exact mask from funcdata.cc:88).
        self.flags &= !(funcdata_flags::highlevel_on
            | funcdata_flags::blocks_generated
            | funcdata_flags::processing_started
            | funcdata_flags::typerecovery_start
            | funcdata_flags::typerecovery_on
            | funcdata_flags::double_precis_on
            | funcdata_flags::restart_pending
            | funcdata_flags::normalization_on);
        self.clean_up_index = 0;
        self.high_level_index = 0;
        self.cast_phase_index = 0;
        self.min_laned_size = self.glb.get_minimum_laned_register_size();

        // localmap->clearUnlocked(); localmap->resetLocalWindow();  -- STUB(W4)
        // clearActiveOutput() (funcdata.cc): drop the output-trial state.
        self.clear_active_output();
        // funcp.clearUnlockedOutput();                               -- STUB(W4)
        // unionMap.clear() (funcdata.cc:90): drop the union-field resolution cache.
        self.union_map.clear();
        self.clear_blocks();
        self.obank.clear();
        self.vbank.clear();
        // clearCallSpecs() (funcdata.cc:104): drop the call-spec list so a restart
        // (which re-follows flow and rebuilds qlst) does not keep stale ops.
        self.clear_call_specs();
        self.clear_jump_tables();
        // heritage.clear() (funcdata.cc:107): reset the SSA-construction state.
        self.heritage.clear();
        // covermerge.clear() tears down the HighVariable arena (the
        // `new HighVariable`s are freed); the W7 high bank is cleared here to
        // mirror that lifecycle.
        self.high_bank.clear();
        // (kuna, ghidra Phase 4) The wire symbols are keyed by HighVariableId,
        // so they MUST die with the arena that issued those ids — a restart
        // would otherwise let a rebuilt high inherit another variable's symbol
        // id and hand the GUI the wrong rename target.
        self.kuna_wire_symbols.clear();
        self.kuna_wire_symbol_for_high.clear();
    }

    /// Set a delay/flag bit directly (test/seam helper; not a C++ method).
    /// Used by the funcdata_op/funcdata_varnode waves to set flags whose toggle
    /// is not a public setter (e.g. `blocks_generated`).
    pub fn set_flag_raw(&mut self, fl: uint4) {
        self.flags |= fl;
    }
    /// Clear a raw flag bit (companion to [`set_flag_raw`](Funcdata::set_flag_raw)).
    pub fn clear_flag_raw(&mut self, fl: uint4) {
        self.flags &= !fl;
    }
    /// Read the raw flags word (test/seam helper).
    pub fn flags_raw(&self) -> uint4 {
        self.flags
    }
}

// The funcdata_block.cc method ports live in the sibling module and add to the
// same `impl Funcdata`.  Re-export nothing here; `funcdata_block.rs` is wired by
// `lib.rs` and references `Funcdata` directly.

/// Convenience newtype the funcdata_op wave uses for the defining-op carrier the
/// VarnodeBank `set_def`/`create_def` paths take (re-exported so the parallel
/// wave needs no extra import path).
pub type DefOp = DefOpInfo;

// =============================================================================
// W7 HighVariable / Cover lifecycle wiring (STUB(W7))
// =============================================================================
//
// The C++ `Varnode::cover` rebuild (`Cover::rebuild`) and the `HighVariable`
// re-derivation walk the op/block/varnode graphs; with the ADR 0001 arenas those
// reads cross from `Funcdata`'s `vbank` into its `obank`/`bblocks`.  The
// `cover::CoverContext` / `variable::HighContext` adapters below let those ported
// algorithms reach the graph through `Funcdata`, exactly where the C++ reads it.

use crate::cover::{Cover, CoverContext, CoverPoint};
use crate::dtype::Datatype;
use crate::variable::{CompareNameView, HighContext, VarnodeView, VarnodeViewLoc};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

impl Funcdata {
    /// Convert a constant pointer into a `PTRSUB(spacebase, off)` (+ extra/zext/
    /// subpiece adjusters) anchored on the symbol the constant points to (C++
    /// `Funcdata::spacebaseConstant`, `funcdata.cc:358-460`).
    ///
    /// `op` is the PcodeOp referencing the constant pointer in slot `slot`;
    /// `entry` is the global Symbol being pointed (in)to (its data-type and entry
    /// address, the result of `ActionConstantPtr::isPointer`'s `queryContainer`);
    /// `rampoint` is the constant interpreted as an Address; `origval`/`origsize`
    /// are the original constant value and Varnode size.
    ///
    /// The LOAD-BEARING lines (funcdata.cc:413/417): `ptrentrytype =
    /// getTypePointerStripArray(sz, sym->getType(), wordsize)` — the STRIPPED-array
    /// pointer type — is forced onto the PTRSUB output, so `RulePtrArith` (which
    /// keys on a `TYPE_PTR` input) selects it and builds the already-correct
    /// `AddTreeState` for the 2D/3D global array.
    ///
    /// `uintb` is `u64` with wrapping ops.
    pub fn spacebase_constant(
        &mut self,
        op: OpId,
        slot: int4,
        entry: &crate::context::GlobalContainer,
        rampoint: &Address,
        origval: u64,
        origsize: int4,
    ) -> KunaResult<()> {
        use crate::dtype::TypeFactory;
        use crate::varnode::varnode_flags;
        use kuna_base::space::AddrSpace;

        let sz = rampoint.get_addr_size();
        let spaceid = rampoint.get_space().expect("spacebaseConstant: rampoint has no space").clone();
        let wordsize = spaceid.get_word_size();

        let types =
            self.get_arch().types_rc().ok_or_else(|| {
                kuna_base::error::KunaError::lowlevel("spacebaseConstant: no type factory")
            })?;
        let invalid = Address::new_invalid();
        let sb_spacebase = types.get_type_spacebase(Rc::clone(&spaceid), &invalid)?;
        let sb_type = types.get_type_pointer(sz, sb_spacebase, wordsize)?;
        // The covering Symbol's declared type.
        let entrytype = entry
            .symbol_type
            .clone()
            .ok_or_else(|| kuna_base::error::KunaError::lowlevel("spacebaseConstant: entry has no type"))?;
        let ptrentrytype =
            types.get_type_pointer_strip_array(sz, Rc::clone(&entrytype), wordsize)?;

        let extra_bytes = rampoint.get_offset().wrapping_sub(entry.entry_addr.get_offset());
        let extra = AddrSpace::byte_to_address(extra_bytes, wordsize);

        // COPY-replacement bookkeeping (funcdata.cc:370-388).  For the INT_ADD/
        // STORE/CALL/comparison cases isCopy stays false and addOp/extraOp/... are
        // freshly created below.
        let opc = self.obank().get(op).expect("spacebaseConstant: stale op").code();
        let mut add_op: Option<OpId> = None;
        let mut extra_op: Option<OpId> = None;
        let mut zext_op: Option<OpId> = None;
        let mut sub_op: Option<OpId> = None;
        let is_copy = opc == OpCode::CPUI_COPY;
        if is_copy {
            if sz < origsize {
                zext_op = Some(op);
            } else {
                // PTRSUB/ADD/SUBPIECE all take 2 parameters.
                self.obank_mut().get_mut(op).expect("spacebaseConstant: stale op").insert_input(1);
                if origsize < sz {
                    sub_op = Some(op);
                } else if extra != 0 {
                    extra_op = Some(op);
                } else {
                    add_op = Some(op);
                }
            }
        }

        let spacebase_vn = self.new_constant(sz, 0);
        {
            let v = self.vbank_mut().get_mut(spacebase_vn).expect("spacebaseConstant: spacebase vn");
            v.update_type_locked(sb_type, true, true);
            v.set_flags_pub(varnode_flags::spacebase);
        }

        let op_addr = self.obank().get(op).expect("spacebaseConstant: stale op").get_addr().clone();
        // addOp: reuse the COPY (if addOp==op) else create a fresh 2-input op.
        let add_op = match add_op {
            None => {
                let new = self.new_op(2, op_addr.clone());
                self.op_set_opcode_code(new, OpCode::CPUI_PTRSUB);
                self.new_unique_out(sz, new)?;
                self.op_insert_before(new, op);
                new
            }
            Some(existing) => {
                self.op_set_opcode_code(existing, OpCode::CPUI_PTRSUB);
                existing
            }
        };
        let mut outvn =
            self.obank().get(add_op).expect("spacebaseConstant: addOp").get_out().expect("addOp out");

        // origval - extra, all already in address units.
        let newconstoff = origval.wrapping_sub(extra);
        let newconst = self.new_constant(sz, newconstoff);
        self.vbank_mut().get_mut(newconst).expect("spacebaseConstant: newconst").set_ptr_check();
        if spaceid.is_truncated() {
            self.obank_mut().get_mut(add_op).expect("spacebaseConstant: addOp").set_ptr_flow();
        }
        self.op_set_input(add_op, spacebase_vn, 0)?;
        self.op_set_input(add_op, newconst, 1)?;

        // THE load-bearing line (updateType with the entry pointer type).
        let mut typelock = entry.is_type_locked();
        if typelock && entrytype.get_metatype() == crate::dtype::type_metatype::TYPE_UNKNOWN {
            typelock = false;
        }
        self.vbank_mut()
            .get_mut(outvn)
            .expect("spacebaseConstant: outvn")
            .update_type_locked(ptrentrytype, typelock, false);

        if extra != 0 {
            let extra_op = match extra_op {
                None => {
                    let new = self.new_op(2, op_addr.clone());
                    self.op_set_opcode_code(new, OpCode::CPUI_INT_ADD);
                    self.new_unique_out(sz, new)?;
                    self.op_insert_before(new, op);
                    new
                }
                Some(existing) => {
                    self.op_set_opcode_code(existing, OpCode::CPUI_INT_ADD);
                    existing
                }
            };
            let extconst = self.new_constant(sz, extra);
            self.vbank_mut().get_mut(extconst).expect("spacebaseConstant: extconst").set_ptr_check();
            self.op_set_input(extra_op, outvn, 0)?;
            self.op_set_input(extra_op, extconst, 1)?;
            outvn = self.obank().get(extra_op).expect("spacebaseConstant: extraOp").get_out().expect("extraOp out");
        }

        if sz < origsize {
            // Extend the smaller new constant back up to the original Varnode size.
            let zext_op = match zext_op {
                None => {
                    let new = self.new_op(1, op_addr.clone());
                    self.op_set_opcode_code(new, OpCode::CPUI_INT_ZEXT);
                    self.new_unique_out(origsize, new)?;
                    self.op_insert_before(new, op);
                    new
                }
                Some(existing) => {
                    self.op_set_opcode_code(existing, OpCode::CPUI_INT_ZEXT);
                    existing
                }
            };
            self.op_set_input(zext_op, outvn, 0)?;
            outvn = self.obank().get(zext_op).expect("spacebaseConstant: zextOp").get_out().expect("zextOp out");
        } else if origsize < sz {
            // Truncate the bigger new constant down to the original Varnode size.
            let sub_op = match sub_op {
                None => {
                    let new = self.new_op(2, op_addr.clone());
                    self.op_set_opcode_code(new, OpCode::CPUI_SUBPIECE);
                    self.new_unique_out(origsize, new)?;
                    self.op_insert_before(new, op);
                    new
                }
                Some(existing) => {
                    self.op_set_opcode_code(existing, OpCode::CPUI_SUBPIECE);
                    existing
                }
            };
            self.op_set_input(sub_op, outvn, 0)?;
            let lsb = self.new_constant(4, 0); // Take least significant piece
            self.op_set_input(sub_op, lsb, 1)?;
            outvn = self.obank().get(sub_op).expect("spacebaseConstant: subOp").get_out().expect("subOp out");
        }

        if !is_copy {
            self.op_set_input(op, outvn, slot)?;
        }
        Ok(())
    }

    /// Borrow the HighVariable arena (the W7 high-variable map).
    pub fn high_bank(&self) -> &crate::variable::HighVariableBank {
        &self.high_bank
    }
    /// Mutably borrow the HighVariable arena.
    pub fn high_bank_mut(&mut self) -> &mut crate::variable::HighVariableBank {
        &mut self.high_bank
    }

    /// C++ `Varnode::updateType(Datatype*)` (`varnode.cc:475-483`) in full,
    /// including the `high->typeDirty()` the Varnode-local
    /// [`Varnode::update_type`](crate::varnode::Varnode::update_type) cannot reach
    /// (the high lives in this arena, not on the Varnode).  Returns whether the
    /// Datatype changed.
    pub fn vn_update_type(&mut self, vn: VarnodeId, ct: std::rc::Rc<crate::dtype::Datatype>) -> bool {
        let high = self.vbank().get(vn).and_then(|v| v.get_high());
        let changed = self
            .vbank_mut()
            .get_mut(vn)
            .map(|v| v.update_type(ct))
            .unwrap_or(false);
        if changed {
            if let Some(h) = high {
                if let Some(hh) = self.high_bank_mut().get_mut(h) {
                    hh.type_dirty();
                }
            }
        }
        changed
    }

    /// Map a `BlockId` to the block's own `getIndex()`.
    fn block_index(&self, bl: BlockId) -> int4 {
        self.bblocks.block(bl).get_index()
    }

    /// `CoverBlock::getUIndex(op)` for a real op (`cover.cc:29-49`): the SeqNum
    /// order, with the MULTIEQUAL/INDIRECT special-casing.  Returns the
    /// `(uindex, code)` pair the [`CoverPoint::Op`] caches.
    fn op_uindex_code(&self, op: OpId) -> (uintm, OpCode) {
        let o = self.obank.get(op).expect("op_uindex_code: stale op");
        let code = o.code();
        if o.is_marker() {
            if code == OpCode::CPUI_MULTIEQUAL {
                // MULTIEQUALs are considered at the very beginning (order 0).
                return (0, code);
            } else if code == OpCode::CPUI_INDIRECT {
                // INDIRECTs are at the location of the op they are indirect for.
                if let Some(in1) = o.get_in(1) {
                    if let Some(vn) = self.vbank.get(in1) {
                        let addr = vn.get_addr();
                        // getOpFromConst: the iop offset is the op's slotmap ffi key
                        let target = OpId::from(slotmap::KeyData::from_ffi(addr.get_offset()));
                        if let Some(t) = self.obank.get(target) {
                            return (t.get_seq_num().get_order(), code);
                        }
                    }
                }
                // Fall through to the default order if the iop target is gone.
            }
        }
        (o.get_seq_num().get_order(), code)
    }

    /// Build the [`CoverPoint`] for a real op (the `(block_index, point)` the
    /// Cover stores for a def/ref).
    fn op_cover_point(&self, op: OpId) -> CoverPoint {
        let (uindex, code) = self.op_uindex_code(op);
        CoverPoint::Op { id: op, uindex, code }
    }

    // -----------------------------------------------------------------------
    // Helpers the `funcdata_merge` MergeContext bridge delegates to (the C++
    // `Merge`/`Cover`/`Varnode` reads that cross the arena boundary).
    // -----------------------------------------------------------------------

    /// `bl->getIndex()` (the bridge's `op_parent_index`/`varnode_def_point`).
    pub(crate) fn block_index_pub(&self, bl: BlockId) -> int4 {
        self.bblocks.block(bl).get_index()
    }

    /// `(block_index, CoverPoint)` of `op` for the merge cover tests.
    pub(crate) fn op_cover_point_pub(&self, op: OpId) -> CoverPoint {
        self.op_cover_point(op)
    }

    /// `((BlockBasic*)bl)->getStop()` (the MULTIEQUAL trim insert point).
    pub(crate) fn block_stop_addr(&self, bl: BlockId) -> Address {
        crate::block::block_get_stop(&self.bblocks.arena, bl)
    }

    /// C++ `Varnode::copyShadow` (`varnode.cc:996`): `a` and `b` are the same
    /// value through a COPY chain.
    pub(crate) fn varnode_copy_shadow(&self, a: VarnodeId, b: VarnodeId) -> bool {
        if a == b {
            return true;
        }
        // One step up a COPY chain: `vn`'s COPY-input, or `None` at the source.
        let copy_pred = |vn: VarnodeId| -> Option<VarnodeId> {
            let v = self.vbank.get(vn)?;
            if !v.is_written() {
                return None;
            }
            let def = v.get_def()?;
            if self.obank.get(def).map(|o| o.code())? != OpCode::CPUI_COPY {
                return None;
            }
            self.obank.get(def).and_then(|o| o.get_in(0))
        };
        // Trace `a` to the source of its COPY chain; hit `b` -> shadow.
        let mut vn = a;
        while let Some(pred) = copy_pred(vn) {
            vn = pred;
            if vn == b {
                return true;
            }
        }
        // Trace `b` to the source; the two sources matching -> shadow.
        let mut ob = b;
        while let Some(pred) = copy_pred(ob) {
            ob = pred;
            if vn == ob {
                return true;
            }
        }
        false
    }

    /// One step up `vn`'s COPY chain (`vn->getDef()->getIn(0)` when `vn` is a
    /// written COPY), or `None` at the chain source.  Shared by the shadow
    /// helpers below (C++ inlines the same `while(isWritten && COPY)` walk).
    fn copy_chain_pred(&self, vn: VarnodeId) -> Option<VarnodeId> {
        let v = self.vbank.get(vn)?;
        if !v.is_written() {
            return None;
        }
        let def = v.get_def()?;
        if self.obank.get(def).map(|o| o.code())? != OpCode::CPUI_COPY {
            return None;
        }
        self.obank.get(def).and_then(|o| o.get_in(0))
    }

    /// Walk `vn` to the end of its COPY chain (`while(isWritten && COPY) vn = in0`).
    fn skip_copy_chain(&self, mut vn: VarnodeId) -> VarnodeId {
        while let Some(pred) = self.copy_chain_pred(vn) {
            vn = pred;
        }
        vn
    }

    /// C++ `Varnode::findSubpieceShadow` (`varnode.cc:1025-1072`): does `vn`
    /// equal `whole` truncated by `least_byte` least-significant bytes, allowing
    /// COPY/MULTIEQUAL in the flow path (bounded recursion depth 1).
    fn find_subpiece_shadow(&self, vn: VarnodeId, least_byte: int4, whole: VarnodeId, recurse: int4) -> bool {
        let vn = self.skip_copy_chain(vn);
        let v = match self.vbank.get(vn) {
            Some(v) => v,
            None => return false,
        };
        if !v.is_written() {
            if v.get_addr().is_constant() {
                let whole = self.skip_copy_chain(whole);
                let wv = match self.vbank.get(whole) {
                    Some(w) => w,
                    None => return false,
                };
                if !wv.get_addr().is_constant() {
                    return false;
                }
                let off = (wv.get_offset() >> (least_byte * 8))
                    & kuna_base::address::calc_mask(v.get_size());
                return off == v.get_offset();
            }
            return false;
        }
        let def = match v.get_def() {
            Some(d) => d,
            None => return false,
        };
        let opc = self.obank.get(def).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY);
        if opc == OpCode::CPUI_SUBPIECE {
            let o = self.obank.get(def).unwrap();
            let tmpvn = match o.get_in(0) {
                Some(t) => t,
                None => return false,
            };
            let off = o
                .get_in(1)
                .and_then(|c| self.vbank.get(c))
                .map(|c| c.get_offset() as int4)
                .unwrap_or(0);
            let tmp_size = self.vbank.get(tmpvn).map(|t| t.get_size()).unwrap_or(0);
            let whole_size = self.vbank.get(whole).map(|w| w.get_size()).unwrap_or(0);
            if off != least_byte || tmp_size != whole_size {
                return false;
            }
            if tmpvn == whole {
                return true;
            }
            let mut tmpvn = tmpvn;
            while let Some(pred) = self.copy_chain_pred(tmpvn) {
                tmpvn = pred;
                if tmpvn == whole {
                    return true;
                }
            }
        } else if opc == OpCode::CPUI_MULTIEQUAL {
            let recurse = recurse + 1;
            if recurse > 1 {
                return false; // Truncate the recursion at maximum depth
            }
            let whole = self.skip_copy_chain(whole);
            let wv = match self.vbank.get(whole) {
                Some(w) => w,
                None => return false,
            };
            if !wv.is_written() {
                return false;
            }
            let big_op = match wv.get_def() {
                Some(d) => d,
                None => return false,
            };
            if self.obank.get(big_op).map(|o| o.code()) != Some(OpCode::CPUI_MULTIEQUAL) {
                return false;
            }
            let small_op = def;
            let big_parent = self.obank.get(big_op).and_then(|o| o.get_parent());
            let small_parent = self.obank.get(small_op).and_then(|o| o.get_parent());
            if big_parent != small_parent {
                return false;
            }
            let ni = self.obank.get(small_op).map(|o| o.num_input()).unwrap_or(0);
            for i in 0..ni {
                let small_in = self.obank.get(small_op).and_then(|o| o.get_in(i));
                let big_in = self.obank.get(big_op).and_then(|o| o.get_in(i));
                match (small_in, big_in) {
                    (Some(si), Some(bi)) => {
                        if !self.find_subpiece_shadow(si, least_byte, bi, recurse) {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
            return true; // All branches were copy shadows
        }
        false
    }

    /// C++ `Varnode::findPieceShadow` (`varnode.cc:1081-1110`): is `vn` formed out
    /// of `piece` via PIECE (backtracking COPY chains and nested PIECEs), with
    /// `least_byte` least-significant bytes truncated from `vn` to reach `piece`.
    fn find_piece_shadow(&self, vn: VarnodeId, mut least_byte: int4, piece: VarnodeId) -> bool {
        let vn = self.skip_copy_chain(vn);
        let v = match self.vbank.get(vn) {
            Some(v) => v,
            None => return false,
        };
        if !v.is_written() {
            return false;
        }
        let def = match v.get_def() {
            Some(d) => d,
            None => return false,
        };
        if self.obank.get(def).map(|o| o.code()) != Some(OpCode::CPUI_PIECE) {
            return false;
        }
        // tmpvn = getIn(1) (least significant part)
        let o = self.obank.get(def).unwrap();
        let mut tmpvn = match o.get_in(1) {
            Some(t) => t,
            None => return false,
        };
        let tmp_size = self.vbank.get(tmpvn).map(|t| t.get_size()).unwrap_or(0);
        let piece_size = self.vbank.get(piece).map(|p| p.get_size()).unwrap_or(0);
        if least_byte >= tmp_size {
            least_byte -= tmp_size;
            tmpvn = match o.get_in(0) {
                Some(t) => t,
                None => return false,
            };
        } else if piece_size + least_byte > tmp_size {
            return false;
        }
        let tmp_size = self.vbank.get(tmpvn).map(|t| t.get_size()).unwrap_or(0);
        if least_byte == 0 && tmp_size == piece_size {
            if tmpvn == piece {
                return true;
            }
            let mut tmpvn = tmpvn;
            while let Some(pred) = self.copy_chain_pred(tmpvn) {
                tmpvn = pred;
                if tmpvn == piece {
                    return true;
                }
            }
            return false;
        }
        // CPUI_PIECE input is too big, recursively search for another CPUI_PIECE
        self.find_piece_shadow(tmpvn, least_byte, piece)
    }

    /// C++ `Varnode::partialCopyShadow` (`varnode.cc:1121-1147`): does one of `a`
    /// / `b` contain the other (as a value) via a SUBPIECE or CONCAT chain at the
    /// given relative byte offset.  Used by `Merge::inflateTest` /
    /// `HighIntersectTest::blockIntersection` to allow partial-shadow overlaps.
    pub(crate) fn varnode_partial_copy_shadow(&self, a: VarnodeId, b: VarnodeId, rel_off: int4) -> bool {
        let sa = match self.vbank.get(a) {
            Some(v) => v.get_size(),
            None => return false,
        };
        let sb = match self.vbank.get(b) {
            Some(v) => v.get_size(),
            None => return false,
        };
        // Pick the smaller as `vn`, the larger as `op2` (swap rel_off when swapped).
        let (vn, op2, rel_off) = if sa < sb {
            (a, b, rel_off)
        } else if sa > sb {
            (b, a, -rel_off)
        } else {
            return false;
        };
        if rel_off < 0 {
            return false; // Not proper containment
        }
        let vn_size = self.vbank.get(vn).map(|v| v.get_size()).unwrap_or(0);
        let op2_size = self.vbank.get(op2).map(|v| v.get_size()).unwrap_or(0);
        if rel_off + vn_size > op2_size {
            return false; // Not proper containment
        }
        // C++ `bigEndian = getSpace()->isBigEndian()` reads the space of `this`
        // (the original caller `b`, BEFORE the size-driven `vn`/`op2` swap), not
        // of `vn`.  Read it from `b` to match (endianness is architecture-uniform
        // per space, so this is equivalent on a single-arch corpus, but the
        // faithful source is the caller varnode).
        let big_endian = self
            .vbank
            .get(b)
            .map(|v| v.get_space().is_big_endian())
            .unwrap_or(false);
        let least_byte = if big_endian { (op2_size - vn_size) - rel_off } else { rel_off };
        if self.find_subpiece_shadow(vn, least_byte, op2, 0) {
            return true;
        }
        if self.find_piece_shadow(op2, least_byte, vn) {
            return true;
        }
        false
    }

    /// C++ `Varnode::characterizeOverlap` (`varnode.cc:155`): 0 = no overlap,
    /// 1 = partial, 2 = identical storage range.
    pub(crate) fn varnode_characterize_overlap(&self, a: VarnodeId, b: VarnodeId) -> int4 {
        let (va, vb) = match (self.vbank.get(a), self.vbank.get(b)) {
            (Some(va), Some(vb)) => (va, vb),
            _ => return 0,
        };
        let (sa, sb) = (va.get_addr().get_space(), vb.get_addr().get_space());
        if sa.map(|s| s.get_index()) != sb.map(|s| s.get_index()) {
            return 0;
        }
        let (oa, ob) = (va.get_addr().get_offset(), vb.get_addr().get_offset());
        let (za, zb) = (va.get_size() as u64, vb.get_size() as u64);
        if oa == ob {
            if za == zb {
                2
            } else {
                1
            }
        } else if oa < ob {
            let thisright = oa + (za - 1);
            if thisright < ob {
                0
            } else {
                1
            }
        } else {
            let opright = ob + (zb - 1);
            if opright < oa {
                0
            } else {
                1
            }
        }
    }

    /// C++ `Merge::allocateCopyTrim` (`merge.cc:411`): build a COPY of `in_vn`
    /// into a fresh unique, returning the new (unattached) COPY op.  The union
    /// `needsResolution` arm is the conservative default (no union types in the
    /// merged tree).
    pub(crate) fn build_copy_trim_op(
        &mut self,
        in_vn: VarnodeId,
        addr: Address,
        _trim_op: OpId,
    ) -> KunaResult<OpId> {
        let copy_op = self.new_op(1, addr);
        self.op_set_opcode(copy_op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        let (ct, size) = {
            let v = self.vbank.get(in_vn).expect("build_copy_trim_op: stale in_vn");
            (Rc::clone(v.get_type()), v.get_size())
        };
        let out_vn = self.new_unique(size, Some(ct));
        self.op_set_output(copy_op, out_vn)?;
        self.op_set_input(copy_op, in_vn, 0)?;
        // (kuna LOSS-229) Preserve the dynamic-symbol / `mapped` binding across a
        // cover-trim re-insertion.  In upstream the firstuse COPY that carries a
        // dynamic-hash symbol is never destroyed, so its `mapped` output survives to
        // `ActionMarkExplicit` (coreaction.cc:3148, isMapped arm) and is rendered as
        // an explicit named local.  In the kuna pipeline `RulePropagateCopy` collapses
        // that COPY during the fullloop and `Merge::mergeAddrTied` cover-separation
        // re-materialises it here; the fresh unique output must inherit the `mapped`
        // bit from the cover-forced input so the SAME markexplicit arm fires (the
        // post-merge late `ActionDynamicSymbols` then re-attaches the name).  Guarded
        // on the input actually being mapped, so non-dynamic cover trims are unchanged.
        if self.vbank.get(in_vn).map(|v| v.is_mapped()).unwrap_or(false) {
            if let Some(v) = self.vbank.get_mut(out_vn) {
                v.set_flags_pub(crate::varnode::varnode_flags::mapped);
            }
        }
        Ok(copy_op)
    }

    /// C++ `Merge::trimOpOutput` (`merge.cc:656`): bump the op's output forward
    /// through a new COPY so its Cover shrinks to a single point.
    pub(crate) fn do_trim_op_output(&mut self, op: OpId) -> KunaResult<()> {
        let code = self.obank.get(op).expect("do_trim_op_output: stale op").code();
        let afterop = if code == OpCode::CPUI_INDIRECT {
            let addr = self
                .obank
                .get(op)
                .and_then(|o| o.get_in(1))
                .and_then(|in1| self.vbank.get(in1))
                .map(|v| v.get_addr().get_offset())
                .unwrap_or(0);
            OpId::from(slotmap::KeyData::from_ffi(addr))
        } else {
            op
        };
        let (vn, ct, size, op_addr) = {
            let o = self.obank.get(op).expect("do_trim_op_output: stale op");
            let vn = o.get_out().expect("do_trim_op_output: op has no output");
            let op_addr = o.get_addr().clone();
            let v = self.vbank.get(vn).expect("do_trim_op_output: stale out");
            (vn, Rc::clone(v.get_type()), v.get_size(), op_addr)
        };
        let copyop = self.new_op(1, op_addr);
        self.op_set_opcode(copyop, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        let uniq = self.new_unique(size, Some(ct));
        self.op_set_output(op, uniq)?; // op output is now the stubby uniq
        self.op_set_output(copyop, vn)?; // original output bumped onto the copy
        self.op_set_input(copyop, uniq, 0)?;
        self.op_insert_after(copyop, afterop);
        Ok(())
    }

    /// C++ `data.opMarkNonPrinting` (the merge copymarker suppression).  The
    /// non-printing bit is consumed by the printer; wired through the addl-flag.
    pub(crate) fn op_mark_non_printing_pub(&mut self, op: OpId) {
        if let Some(o) = self.obank_mut().get_mut(op) {
            o.set_flag(crate::op::pcodeop_flags::nonprinting);
        }
    }

    /// C++ `Funcdata::opMarkSpecialPrint` (`funcdata_op.cc`): set the
    /// `special_print` additional flag.  The bitfield transforms mark the
    /// INSERT (and its terminating STORE) so the printer renders them with the
    /// dedicated `pushBitfield` path rather than the raw operator.
    pub(crate) fn op_mark_special_print(&mut self, op: OpId) {
        if let Some(o) = self.obank_mut().get_mut(op) {
            o.set_additional_flag(crate::op::pcodeop_addlflags::special_print);
        }
    }

    /// `op->outputTypeLocal()` — the local-from-op output type (C++
    /// `TypeOp::getOutputLocal`, typeop.cc:262).
    ///
    /// (kuna L3) Routes through the per-op-code [`type_op_info`] dispatch on the
    /// shared (INTERNED) [`TypeFactory`] (`glb->types`), so two ops whose local
    /// type is the same size+metatype return the SAME `Rc<Datatype>`.  This is
    /// load-bearing for `Merge::mergeAdjacent`'s pointer-identity same-type test
    /// (merge.cc:990 `ct != op->inputTypeLocal(i)`): a fresh `Rc` per call would
    /// never compare equal, so `mergeAdjacent` could never tie an op's input to
    /// its output.  Falls back to a fresh unknown only when no TypeFactory is
    /// attached (hand-built fixtures) or the op-code's dispatch errors.
    pub(crate) fn op_output_type_local_pub(&self, op: OpId) -> Rc<Datatype> {
        let (sz, opc) = match self.obank.get(op) {
            Some(o) => {
                let sz = o
                    .get_out()
                    .and_then(|out| self.vbank.get(out))
                    .map(|v| v.get_size())
                    .unwrap_or(1);
                (sz, o.code())
            }
            None => (1, OpCode::CPUI_COPY),
        };
        if let Some(tlst) = self.get_arch().types() {
            if let Ok(ct) = crate::typeop::type_op_info(opc).get_output_local(tlst, sz) {
                return ct;
            }
        }
        Rc::new(Datatype::new(sz, crate::dtype::type_metatype::TYPE_UNKNOWN))
    }

    /// `op->inputTypeLocal(slot)` — see [`op_output_type_local_pub`].
    pub(crate) fn op_input_type_local_pub(&self, op: OpId, slot: int4) -> Rc<Datatype> {
        let (sz, opc) = match self.obank.get(op) {
            Some(o) => {
                let sz = o
                    .get_in(slot)
                    .and_then(|inv| self.vbank.get(inv))
                    .map(|v| v.get_size())
                    .unwrap_or(1);
                (sz, o.code())
            }
            None => (1, OpCode::CPUI_COPY),
        };
        if let Some(tlst) = self.get_arch().types() {
            if let Ok(ct) = crate::typeop::type_op_info(opc).get_input_local(tlst, slot, sz) {
                return ct;
            }
        }
        Rc::new(Datatype::new(sz, crate::dtype::type_metatype::TYPE_UNKNOWN))
    }

    /// C++ `Cover single; single.addDefPoint(vn); single.addRefPoint(op,vn)`
    /// (`merge.cc:503-505`) — the cover of a single read, used by
    /// `Merge::eliminateIntersect` to decide whether the read crosses an
    /// intervening write at the same storage address.
    pub(crate) fn build_single_read_cover(&self, vn: VarnodeId, op: OpId) -> Cover {
        // The `addRefPoint` extends the cover from `vn`'s def to the reading op so
        // intervening writes at the same storage address fall inside it; omitting it
        // collapses the cover to the def point and no intersection is ever found
        // (LOSS-229: the dynamic-hash firstuse COPY was never cover-trimmed).
        let mut single = Cover::new();
        let ctx = FuncdataCoverCtx { fd: self };
        let (def, is_input) = ctx.def_point(vn);
        single.add_def_point(def, is_input);
        single.add_ref_point_for(&ctx, op, vn);
        single
    }

    /// C++ `Merge::checkCopyPair` cover range (`merge.cc:1120-1121`):
    /// `range.addDefPoint(domOp->getOut()); range.addRefPoint(subOp,subOp->getIn(0))`.
    ///
    /// Both points are required.  With only the def point the range collapses to
    /// the dominant COPY's write and no intervening write to the HighVariable is
    /// ever found inside it, so `checkCopyPair` reports every dominated COPY
    /// redundant and `markRedundantCopies` deletes a load-bearing restore from
    /// the emitted C (the same omission `build_single_read_cover` records as
    /// LOSS-229).
    pub(crate) fn build_copy_pair_range(&self, dom_op: OpId, sub_op: OpId) -> Cover {
        let mut range = Cover::new();
        let ctx = FuncdataCoverCtx { fd: self };
        if let Some(dom_out) = self.obank.get(dom_op).and_then(|o| o.get_out()) {
            let (def, is_input) = ctx.def_point(dom_out);
            range.add_def_point(def, is_input);
        }
        if let Some(sub_in) = self.obank.get(sub_op).and_then(|o| o.get_in(0)) {
            range.add_ref_point_for(&ctx, sub_op, sub_in);
        }
        range
    }

    /// The `getTiedVarnode`/`getInputVarnode` read on a HighVariable, across the
    /// `high_bank` <-> `vbank`/`obank` field split (the bridge cannot destructure
    /// private fields from another module).  `which` selects tied (`false`) vs
    /// input (`true`).
    pub(crate) fn high_tied_or_input_varnode(
        &self,
        high: crate::context::HighVariableId,
        input: bool,
    ) -> Option<VarnodeId> {
        let Funcdata { high_bank, vbank, obank, .. } = self;
        let ctx = HighReadView::new(vbank, obank);
        let h = high_bank.get(high)?;
        if input {
            h.get_input_varnode(&ctx).ok()
        } else {
            h.get_tied_varnode(&ctx).ok()
        }
    }

    /// Drive the bank-level `HighVariable::merge` across the field split,
    /// returning the deferred `vn->setHigh` writes for the caller to replay once
    /// the read-view borrow is released (the merge never reads `vn->high`).
    pub(crate) fn bank_merge_with_log(
        &mut self,
        high1: crate::context::HighVariableId,
        high2: crate::context::HighVariableId,
        isspeculative: bool,
        cache: &mut crate::variable::HighIntersectTest,
        set_high_log: &mut Vec<(VarnodeId, crate::context::HighVariableId, int2)>,
        mark_set: &std::cell::RefCell<std::collections::BTreeSet<crate::context::HighVariableId>>,
    ) -> KunaResult<()> {
        let Funcdata { high_bank, vbank, obank, .. } = self;
        let ctx = HighReadView::new(vbank, obank);
        let mut set_high = |vn: VarnodeId, id: crate::context::HighVariableId, mg: int2| {
            set_high_log.push((vn, id, mg));
        };
        let mut set_mark = |id: crate::context::HighVariableId| {
            mark_set.borrow_mut().insert(id);
        };
        let mut clear_mark = |id: crate::context::HighVariableId| {
            mark_set.borrow_mut().remove(&id);
        };
        let is_mark = |id: crate::context::HighVariableId| mark_set.borrow().contains(&id);
        high_bank.merge(
            high1,
            high2,
            isspeculative,
            &ctx,
            &mut set_high,
            Some(cache),
            &mut set_mark,
            &mut clear_mark,
            &is_mark,
        )
    }

    /// C++ `Merge::snipReads` insert-point (`merge.cc:454-466`).  Reached by
    /// `snipReads`/`eliminateIntersect` (not on the `mergeMarker` path).
    pub(crate) fn do_snip_reads_insert_point(&self, vn: VarnodeId) -> (BlockId, Address, Option<OpId>) {
        let v = self.vbank.get(vn).expect("snip_reads_insert_point: stale vn");
        if v.is_input() {
            // C++ `Merge::snipReads` (merge.cc:454): for an input Varnode the trim
            // COPY is placed at the entry block's START (`bl->getStart()`), not its
            // stop — so the firstuse-address dynamic-hash COPY lands at the function
            // entry where `DynamicHash::findVarnode` (dynamic.cc:571) expects it.
            let bl = self.bblocks_get_block(0);
            (bl, self.bblocks_block_start(bl), None)
        } else {
            let def = v.get_def().expect("snip_reads_insert_point: non-input has no def");
            let o = self.obank.get(def).expect("snip: stale def");
            let bl = o.get_parent().expect("snip: def no parent");
            let pc = o.get_addr().clone();
            // C++ merge.cc:461-464: an INDIRECT def places the COPY after the op
            // CAUSING the effect (in(1)'s iop encoding), not the INDIRECT itself.
            let afterop = if o.code() == OpCode::CPUI_INDIRECT {
                let off = o
                    .get_in(1)
                    .and_then(|in1| self.vbank.get(in1))
                    .map(|iv| iv.get_addr().get_offset())
                    .expect("snip: INDIRECT without iop input");
                crate::funcdata_varnode::op_iop_decode(off)
            } else {
                def
            };
            (bl, pc, Some(afterop))
        }
    }

    /// Replace a set of COPYs from the same Varnode with a single dominant COPY
    /// (C++ `Merge::buildDominantCopy`, `merge.cc:1151-1238`).
    ///
    /// This is the IR-surgery body of `buildDominantCopy`: the cover math
    /// (`bCover`/`aCover`/`intersect`) decides which COPY outputs can be redirected
    /// to one dominating Varnode without introducing a Cover intersection, then the
    /// non-intersecting ones are `totalReplace`d and destroyed.  Faithful to the
    /// C++; the `needsResolution` union arm is the conservative default (no union
    /// types in the merged tree).
    pub(crate) fn build_dominant_copy_impl(
        &mut self,
        high: crate::context::HighVariableId,
        copy: &[OpId],
        pos: int4,
        size: int4,
    ) -> KunaResult<()> {
        let mut block_set: Vec<BlockId> = Vec::with_capacity(size as usize);
        for i in 0..size {
            let op = copy[(pos + i) as usize];
            let parent = self.obank.get(op).and_then(|o| o.get_parent());
            block_set.push(parent.expect("build_dominant_copy: copy op has no parent"));
        }
        let dom_bl = self.bblocks.find_common_block_set(&block_set);

        let mut dom_copy = copy[pos as usize];
        let root_vn = self.obank.get(dom_copy).and_then(|o| o.get_in(0)).expect("build_dominant_copy: domCopy in0");
        let mut dom_vn = self.obank.get(dom_copy).and_then(|o| o.get_out()).expect("build_dominant_copy: domCopy out");
        let dom_copy_parent = self.obank.get(dom_copy).and_then(|o| o.get_parent());
        let dom_copy_is_new = dom_copy_parent != Some(dom_bl);
        if dom_copy_is_new {
            // (the needsResolution union-facing arm is the conservative default —
            //  no `needsResolution` types in the merged tree.)
            let stop_addr = self.block_stop_addr(dom_bl);
            let new_copy = self.new_op(1, stop_addr);
            self.op_set_opcode(new_copy, crate::typeop::type_op_for(OpCode::CPUI_COPY));
            let (ct, size_root) = {
                let v = self.vbank.get(root_vn).expect("build_dominant_copy: stale rootVn");
                (Rc::clone(v.get_type()), v.get_size())
            };
            let new_vn = self.new_unique(size_root, Some(ct));
            self.op_set_output(new_copy, new_vn)?;
            self.op_set_input(new_copy, root_vn, 0)?;
            self.op_insert_end(new_copy, dom_bl);
            dom_copy = new_copy;
            dom_vn = new_vn;
        }

        // bCover: cover formed by removing all COPYs from rootVn (skip COPY
        // instances whose in0 copyShadows rootVn).
        let mut b_cover = Cover::new();
        {
            let n = self.high_bank.get(high).map(|h| h.num_instances()).unwrap_or(0);
            for i in 0..n {
                let vn = self.high_bank.get(high).expect("build_dominant_copy: stale high").get_instance(i);
                let mut skip = false;
                if self.vbank.get(vn).map(|v| v.is_written()).unwrap_or(false) {
                    if let Some(op) = self.vbank.get(vn).and_then(|v| v.get_def()) {
                        if self.obank.get(op).map(|o| o.code()) == Some(OpCode::CPUI_COPY) {
                            let in0 = self.obank.get(op).and_then(|o| o.get_in(0));
                            if let Some(in0) = in0 {
                                if self.varnode_copy_shadow(in0, root_vn) {
                                    skip = true;
                                }
                            }
                        }
                    }
                }
                if skip {
                    continue;
                }
                // The rebuilt member cover.
                let vc = self.full_varnode_cover(vn);
                b_cover.merge(&vc);
            }
        }

        // For each non-dominant COPY, build the hypothetical aCover (def at domVn,
        // refs at outVn's reads); if it intersects bCover by >1 the redirect would
        // create a Cover intersection, so leave that COPY in place (mark it).
        let mut marked: Vec<bool> = vec![false; size as usize];
        let mut count = size;
        for i in 0..size {
            let op = copy[(pos + i) as usize];
            if op == dom_copy {
                continue; // No intersections from domVn already proven
            }
            let out_vn = self.obank.get(op).and_then(|o| o.get_out()).expect("build_dominant_copy: copy out");
            let mut a_cover = Cover::new();
            {
                let ctx = FuncdataCoverCtx { fd: self };
                let (def, is_input) = ctx.def_point(dom_vn);
                a_cover.add_def_point(def, is_input);
                let descend: Vec<OpId> =
                    self.vbank.get(out_vn).map(|v| v.descend_iter().collect()).unwrap_or_default();
                for refop in descend {
                    a_cover.add_ref_point_for(&ctx, refop, out_vn);
                }
            }
            if b_cover.intersect(&a_cover) > 1 {
                count -= 1;
                marked[i as usize] = true;
            }
        }

        if count <= 1 {
            // Don't bother if we only replace one COPY with another.
            for m in marked.iter_mut() {
                *m = true;
            }
            count = 0;
            if dom_copy_is_new {
                self.op_destroy(dom_copy);
            }
        }

        // Replace all non-intersecting COPYs with a read of the dominating Varnode.
        for i in 0..size {
            let op = copy[(pos + i) as usize];
            if marked[i as usize] {
                // The marked-set was local; nothing to clear.
                continue;
            }
            let out_vn = self.obank.get(op).and_then(|o| o.get_out()).expect("build_dominant_copy: copy out");
            if out_vn != dom_vn {
                if let Some(out_high) = self.vbank.get(out_vn).and_then(|v| v.get_high()) {
                    self.high_remove_member(out_high, out_vn);
                }
                self.total_replace(out_vn, dom_vn)?;
                self.op_destroy(op);
            }
        }

        if count > 0 && dom_copy_is_new {
            if let Some(dom_high) = self.vbank.get(dom_vn).and_then(|v| v.get_high()) {
                if dom_high != high {
                    self.merge_two_highs(high, dom_high, true)?;
                }
            }
        }
        Ok(())
    }

    /// `vn->getCover()` as a freshly rebuilt [`Cover`] (the C++ `bCover.merge`
    /// reads each member's rebuilt cover).  Builds the full def/use cover off the
    /// live graph rather than relying on the cached (possibly dirty) one.
    fn full_varnode_cover(&self, vn: VarnodeId) -> Cover {
        let mut cover = Cover::new();
        let ctx = FuncdataCoverCtx { fd: self };
        cover.rebuild(&ctx, vn);
        cover
    }

    /// `outVn->getHigh()->remove(outVn)` across the bank field split (the high
    /// loses one member; its cover is marked dirty).
    pub(crate) fn high_remove_member(&mut self, high: crate::context::HighVariableId, vn: VarnodeId) {
        let has_symbol_entry = false; // no symbol entries in the merged tree
        let Funcdata { high_bank, vbank, obank, .. } = self;
        let ctx = HighReadView::new(vbank, obank);
        high_bank.remove_member(high, vn, has_symbol_entry, &ctx);
    }

    /// `high1->merge(high2, &testCache, isspeculative)` for the dominant-copy
    /// path, replaying the deferred `vn->setHigh` writes (see [`bank_merge_with_log`]).
    /// The intersection cache is local here (the new dominating high has no cached
    /// edges yet), matching the C++ pass of `data.getMerge()`'s `testCache`.
    fn merge_two_highs(
        &mut self,
        high1: crate::context::HighVariableId,
        high2: crate::context::HighVariableId,
        isspeculative: bool,
    ) -> KunaResult<()> {
        let opset = crate::cover::PcodeOpSet::new(Box::new(Vec::new), Box::new(|_, _| false));
        let mut cache = crate::variable::HighIntersectTest::new(opset);
        let mut set_high_log: Vec<(VarnodeId, crate::context::HighVariableId, int2)> = Vec::new();
        let mark_set: std::cell::RefCell<std::collections::BTreeSet<crate::context::HighVariableId>> =
            std::cell::RefCell::new(std::collections::BTreeSet::new());
        let res = self.bank_merge_with_log(high1, high2, isspeculative, &mut cache, &mut set_high_log, &mark_set);
        for (vn, id, mg) in set_high_log {
            if let Some(v) = self.vbank_mut().get_mut(vn) {
                v.set_high(id, mg);
            }
        }
        res
    }

    /// Rebuild a Varnode's Cover, driving `Varnode::updateCover` across the arena
    /// boundary (the C++ `vn->updateCover()` / `Cover::rebuild`).  Called by the
    /// Merge driver after data-flow changes.  This is the `// STUB(W7)` cover
    /// rebuild that `funcdata_block`/merge will invoke.
    pub fn update_varnode_cover(&mut self, vn: VarnodeId) {
        // C++ `Varnode::updateCover`: if coverdirty, and hasCover & cover!=0,
        // rebuild; then clear coverdirty.  We clone the Cover out, rebuild it
        // against a read-only graph view, and write it back (the borrow split).
        let v = self.vbank.get(vn).expect("update_varnode_cover: stale vn");
        if !v.is_cover_dirty_flag() {
            return; // not dirty: nothing to do (C++ early-out)
        }
        let cover0 = if v.has_cover() { v.cover().cloned() } else { None };
        if let Some(mut cover) = cover0 {
            {
                let ctx = FuncdataCoverCtx { fd: self };
                cover.rebuild(&ctx, vn);
            }
            self.vbank_mut()
                .get_mut(vn)
                .expect("update_varnode_cover: stale vn")
                .set_cover(cover);
        }
        self.vbank_mut()
            .get_mut(vn)
            .expect("update_varnode_cover: stale vn")
            .clear_cover_dirty();
    }
}

/// Read-only graph view for the [`Cover`] def/use walk (the cross-arena reads
/// `Cover::rebuild` makes off the held `Varnode *`/`PcodeOp *`/`FlowBlock *`).
pub(crate) struct FuncdataCoverCtx<'a> {
    // (kuna) `pub(crate)` so `kuna_paramcopyhoist` can build the hypothetical
    // hoisted Cover off the same view `buildDominantCopy` uses.
    pub(crate) fd: &'a Funcdata,
}

impl<'a> FuncdataCoverCtx<'a> {
    /// Resolve a block *index* to its `BlockId` (the inverse of `getIndex()`).
    fn block_id_of_index(&self, index: int4) -> BlockId {
        let n = self.fd.bblocks_get_size();
        for i in 0..n {
            let bid = self.fd.bblocks_get_block(i);
            if self.fd.bblocks.block(bid).get_index() == index {
                return bid;
            }
        }
        panic!("FuncdataCoverCtx: no block with index {index}");
    }
}

impl<'a> CoverContext for FuncdataCoverCtx<'a> {
    fn size_in(&self, bl: int4) -> int4 {
        let bid = self.block_id_of_index(bl);
        self.fd.bblocks.block(bid).size_in()
    }
    fn get_in(&self, bl: int4, j: int4) -> int4 {
        let bid = self.block_id_of_index(bl);
        let pred = self.fd.bblocks.block(bid).get_in(j);
        self.fd.bblocks.block(pred).get_index()
    }
    fn def_point(&self, vn: VarnodeId) -> (Option<(int4, CoverPoint)>, bool) {
        let v = self.fd.vbank.get(vn).expect("def_point: stale vn");
        match v.get_def() {
            Some(op) => {
                let parent = self.fd.obank.get(op).and_then(|o| o.get_parent());
                let blk = parent.map(|p| self.fd.block_index(p)).unwrap_or(0);
                (Some((blk, self.fd.op_cover_point(op))), false)
            }
            None => (None, v.is_input()),
        }
    }
    fn descend(&self, vn: VarnodeId) -> Vec<OpId> {
        self.fd.vbank.get(vn).map(|v| v.descend_iter().collect()).unwrap_or_default()
    }
    fn ref_point(&self, op: OpId, vn: VarnodeId) -> (int4, CoverPoint, bool, Vec<int4>) {
        let o = self.fd.obank.get(op).expect("ref_point: stale op");
        let parent = o.get_parent().expect("ref_point: op has no parent");
        let bl = self.fd.block_index(parent);
        let point = self.fd.op_cover_point(op);
        let is_multiequal = o.code() == OpCode::CPUI_MULTIEQUAL;
        let mut preds = Vec::new();
        if is_multiequal {
            let n = o.num_input();
            for j in 0..n {
                if o.get_in(j) == Some(vn) {
                    let pred = self.fd.bblocks.block(parent).get_in(j);
                    preds.push(self.fd.bblocks.block(pred).get_index());
                }
            }
        }
        (bl, point, is_multiequal, preds)
    }
    fn out_implied(&self, op: OpId) -> Option<VarnodeId> {
        let o = self.fd.obank.get(op)?;
        let out = o.get_out()?;
        let ov = self.fd.vbank.get(out)?;
        if ov.is_implied() {
            Some(out)
        } else {
            None
        }
    }
}

impl Funcdata {
    /// Split the `high_bank` field off from the rest of `Funcdata` so a
    /// `&mut HighVariableBank` and a read-only [`HighContext`] over the remaining
    /// fields can coexist (the high arena is a distinct field from `vbank`/`obank`).
    ///
    /// Returns a re-borrowing closure runner: the caller's `f` gets the mutable
    /// high bank plus a `HighContext` view of the other fields.
    pub(crate) fn with_high_split<R>(
        &mut self,
        f: impl FnOnce(&mut crate::variable::HighVariableBank, &dyn HighContext) -> R,
    ) -> R {
        // Field-split borrow: `high_bank` mutable, the rest immutable through a
        // dedicated read view that borrows only vbank/obank.
        let Funcdata { high_bank, vbank, obank, .. } = self;
        let ctx = HighReadView { vbank, obank };
        f(high_bank, &ctx)
    }

    /// If HighVariables are enabled, ensure the given Varnode has one assigned
    /// (C++ `Funcdata::assignHigh`, `funcdata_varnode.cc:48-61`).
    ///
    /// STUB(W4): the `hasWarning`/`issueDatatypeWarning` datatype-warning step
    /// (`glb->types`) is a W4 surface and is omitted.  The `calcCover` + `new
    /// HighVariable(vn)` + `vn->setHigh(id,0)` lifecycle is wired here.
    pub fn assign_high_var(&mut self, vn: VarnodeId) -> Option<crate::context::HighVariableId> {
        if !self.is_high_on() {
            return None;
        }
        let v = self.vbank.get(vn).expect("assign_high_var: stale vn");
        if v.has_cover() {
            self.vbank_mut().get_mut(vn).unwrap().calc_cover();
        }
        if self.vbank.get(vn).unwrap().is_annotation() {
            return None;
        }
        let id = self.high_bank.new_high(vn);
        self.vbank_mut().get_mut(vn).unwrap().set_high(id, 0);
        Some(id)
    }

    /// Turn on HighVariable objects for all Varnodes (C++
    /// `Funcdata::setHighLevel`, `funcdata_varnode.cc:613-623`).
    pub fn set_high_level(&mut self) {
        if self.is_high_on() {
            return;
        }
        self.flags |= funcdata_flags::highlevel_on;
        self.high_level_index = self.vbank.get_create_index();
        let all: Vec<VarnodeId> = self.vbank.iter_loc().collect();
        for vn in all {
            self.assign_high_var(vn);
        }
    }

    /// Get the (re-derived) data-type of a Varnode's HighVariable (the C++
    /// `vn->getHigh()->getType()` the type-read paths use — M1).  Returns `None`
    /// if the Varnode has no HighVariable yet.
    ///
    /// `symbol_submeta` is the backing-symbol metatype for the `stripType`
    /// partial-union case (STUB(W4): `None` until the Varnode-Symbol link lands).
    pub fn high_get_type(&mut self, vn: VarnodeId) -> Option<std::rc::Rc<crate::dtype::Datatype>> {
        let id = self.vbank.get(vn)?.get_high()?;
        Some(self.with_high_split(|hb, ctx| hb.get_mut(id).unwrap().get_type(ctx, None)))
    }

    /// Drive a HighVariable's external cover update (the C++
    /// `HighVariable::updateCover`, called by Merge).  Convenience over
    /// [`with_high_split`] for the bank's `update_cover`.
    pub fn high_update_cover(&mut self, id: crate::context::HighVariableId) {
        self.with_high_split(|hb, ctx| hb.update_cover(id, ctx));
    }

    /// The HighVariable's name representative Varnode (C++
    /// `HighVariable::getNameRepresentative`), across the bank field-split.
    /// `None` if the high is gone.
    pub fn high_name_representative(&mut self, id: crate::context::HighVariableId) -> Option<VarnodeId> {
        self.high_bank.get(id)?;
        Some(self.with_high_split(|hb, ctx| hb.get_mut(id).unwrap().get_name_representative(ctx)))
    }

    /// Whether a HighVariable can carry a name (C++ `HighVariable::hasName`),
    /// across the bank field-split.  `false` if the high is gone or its
    /// coverability check errors (the C++ `LowlevelError` -> conservative `false`).
    pub fn high_has_name(&mut self, id: crate::context::HighVariableId) -> bool {
        if self.high_bank.get(id).is_none() {
            return false;
        }
        self.with_high_split(|hb, ctx| hb.get_mut(id).unwrap().has_name(ctx).unwrap_or(false))
    }

    /// Build a \e dynamic Symbol associated with the given (constant) Varnode (C++
    /// `Funcdata::buildDynamicSymbol`, `funcdata_varnode.cc:1304-1326`).
    ///
    /// If a Symbol is already attached, no change is made.  Otherwise a special
    /// \e dynamic Symbol is created, associated with the Varnode via a hash of its
    /// local data-flow (rather than its storage address), and attached to the
    /// Varnode's HighVariable.
    ///
    /// Faithful to the C++ except for two merged-tree seams threaded in as
    /// parameters (the same convention [`crate::dynamic::DynamicHash::unique_hash`]
    /// uses): `maxduplicates` is the `glb->dynamic_hash_maxdup_high` collision
    /// budget and `base1_unknown` is the EquateSymbol's `getBase(1,TYPE_UNKNOWN)`
    /// type — both resolved from the `Architecture` at the call site.  Only the
    /// constant arm is reached by the `force varnode` console command; the
    /// non-constant `addDynamicSymbol` arm errs as a documented seam (the merged
    /// tree has no Varnode→SymbolEntry retype link).
    ///
    /// On success the EquateSymbol id is parked on `high->kuna_equate_symbol`,
    /// which is the merged-tree stand-in for the C++ `vn->setSymbolEntry(...)`
    /// effect `high->getSymbol() == sym` (read by `PrintC::push_integer`).
    pub fn build_dynamic_symbol(
        &mut self,
        vn: VarnodeId,
        maxduplicates: uint4,
        base1_unknown: std::rc::Rc<crate::dtype::Datatype>,
    ) -> KunaResult<()> {
        use kuna_base::error::KunaError;
        let v = self
            .vbank
            .get(vn)
            .ok_or_else(|| KunaError::lowlevel("build_dynamic_symbol: stale varnode"))?;
        if v.is_type_lock() || v.is_name_lock() {
            return Err(KunaError::lowlevel(
                "Trying to build dynamic symbol on locked varnode",
            ));
        }
        if !self.is_high_on() {
            return Err(KunaError::lowlevel(
                "Cannot create dynamic symbols until decompile has completed",
            ));
        }
        let is_constant = v.is_constant();
        let value = v.get_offset();
        let high = self
            .vbank
            .get(vn)
            .and_then(|v| v.get_high())
            .ok_or_else(|| KunaError::lowlevel("build_dynamic_symbol: varnode has no high"))?;
        // Symbol already exists.
        if self
            .high_bank
            .get(high)
            .and_then(|h| h.kuna_equate_symbol())
            .is_some()
        {
            return Ok(());
        }
        let (hash, addr) =
            crate::dynamic::dynamic_unique_hash(vn, maxduplicates, self)?;
        if hash == 0 {
            return Err(KunaError::lowlevel("Unable to find unique hash for varnode"));
        }
        // The non-constant arm (addDynamicSymbol over high->getType()) needs the
        // merged-tree Varnode→SymbolEntry retype link, which is a W4 seam; the
        // `force varnode` command only ever reaches the constant arm.
        if !is_constant {
            return Err(KunaError::lowlevel(
                "kuna rust port: build_dynamic_symbol non-constant arm needs the W4 Varnode-SymbolEntry link",
            ));
        }
        let localmap = self
            .localmap
            .as_mut()
            .ok_or_else(|| KunaError::lowlevel("build_dynamic_symbol: no local scope"))?;
        let sym = localmap.add_equate_symbol(
            "",
            crate::database::symbol_dispflags::FORCE_HEX,
            value,
            &addr,
            hash,
            base1_unknown,
        )?;
        if let Some(h) = self.high_bank.get_mut(high) {
            h.set_kuna_equate_symbol(sym);
        }
        Ok(())
    }

    /// The equate-Symbol bound to `vn`'s HighVariable by
    /// [`build_dynamic_symbol`](Self::build_dynamic_symbol) (the merged-tree
    /// `vn->getHigh()->getSymbol()` stand-in for the constant-format path).
    /// `None` if the Varnode has no high or no bound equate symbol.
    pub fn vn_high_equate_symbol(&self, vn: VarnodeId) -> Option<crate::database::SymbolId> {
        // Prefer the HighVariable mirror (set when a High existed at mapping time),
        // then fall back to the Varnode-level binding (C++ `vn->getSymbolEntry()`):
        // the early `ActionDynamicMapping` may bind the constant before its High is
        // built, so the render must still see the equate.  The fallback only applies
        // when the bound Symbol is an equate (the render reads its display format).
        if let Some(high) = self.vbank.get(vn).and_then(|v| v.get_high()) {
            if let Some(sym) = self.high_bank.get(high).and_then(|h| h.kuna_equate_symbol()) {
                return Some(sym);
            }
        }
        let sym = self.vbank.get(vn)?.kuna_symbol_entry()?;
        let localmap = self.localmap.as_ref()?;
        if localmap.database().symbol(sym).get_category() == crate::database::symbol_category::EQUATE
        {
            Some(sym)
        } else {
            None
        }
    }

    /// The merged-tree stand-in for the C++ `vn->getSymbolEntry() != 0`
    /// idempotency guard in `attemptDynamicMapping[Late]`
    /// (`funcdata_varnode.cc:1347,1378`): a dynamic SymbolEntry has already been
    /// bound to `vn` when its HighVariable carries either the attached name (the
    /// non-equate arm's `set_kuna_name`) OR the attached equate Symbol (the
    /// equate arm's `set_kuna_equate_symbol`).  Both arms set
    /// `vn->setSymbolEntry(entry)` in the C++, so a faithful stand-in must treat
    /// either binding as "already labeled" — otherwise a re-run of the (early)
    /// equate arm would re-bind an already-equated Varnode (the C++ returns
    /// `false` there).
    fn vn_high_has_dynamic_binding(&self, vn: VarnodeId) -> bool {
        // C++ `vn->getSymbolEntry() != 0` (funcdata_varnode.cc:1348): the binding
        // lives on the Varnode itself, so it survives heritage rebuilding the High.
        if self.vbank.get(vn).and_then(|v| v.kuna_symbol_entry()).is_some() {
            return true;
        }
        // Fall back to the HighVariable mirror (set by the late name/equate arms when
        // a High already exists) so a name bound there is likewise idempotent.
        let high = match self.vbank.get(vn).and_then(|v| v.get_high()) {
            Some(h) => h,
            None => return false,
        };
        match self.high_bank.get(high) {
            Some(h) => h.kuna_name().is_some() || h.kuna_equate_symbol().is_some(),
            None => false,
        }
    }

    /// C++ `Funcdata::attemptDynamicMapping` (`funcdata_varnode.cc:1335`): the
    /// EARLY dynamic mapping, run mid-pipeline by `ActionDynamicMapping`.  Finds
    /// the Varnode the dynamic SymbolEntry maps to and binds the Symbol's
    /// properties (size/type-lock) to it — the C++ `setSymbolProperties`.
    ///
    /// The behavioural point of the early mapping (vs. the late, name-only one)
    /// is to PIN the matched Varnode before the merge/copy-elimination passes:
    /// binding the symbol marks the Varnode `mapped`, so it survives as an
    /// explicit storage location (the dynamic-hash COPY the late hash later
    /// targets) instead of being copy-propagated away.  The kuna stand-in is the
    /// same as the late path (`kuna_name` + the `mapped` flag); the `updateType`/
    /// type-lock retype is the documented W4 loss.  Returns `true` on a match.
    pub fn attempt_dynamic_mapping(
        &mut self,
        entry: &crate::database::SymbolEntry,
    ) -> KunaResult<bool> {
        use crate::database::symbol_category;
        let sym_id = entry.symbol;
        let (category, sym_name) = {
            let localmap = match self.localmap.as_ref() {
                Some(l) => l,
                None => return Ok(false),
            };
            let sym = localmap.database().symbol(sym_id);
            (sym.get_category(), sym.get_name().to_string())
        };
        if category == symbol_category::UNION_FACET {
            return self.apply_union_facet(entry);
        }
        let first_use = entry.get_first_use_address();
        let hash = entry.get_hash();
        let mut dhash = crate::dynamic::DynamicHash::new();
        let vn = match dhash.find_varnode(self, &first_use, hash) {
            Some(v) => v,
            None => return Ok(false),
        };
        // Idempotent: the Varnode is already bound to a dynamic SymbolEntry (C++
        // checks `vn->getSymbolEntry()`, `funcdata_varnode.cc:1348`).  The binding is
        // parked on the Varnode (not the HighVariable, which may not exist yet at
        // this early mapping or be rebuilt each heritage pass), so the re-run of this
        // `rule_repeatapply` action does not re-report a change and loop forever.
        if self.vn_high_has_dynamic_binding(vn) {
            return Ok(false);
        }
        if category == symbol_category::EQUATE {
            // C++ `vn->setSymbolEntry(entry)` (varnode.cc:448) marks the matched
            // Varnode `Varnode::mapped`.  That `mapped` bit is load-bearing for the
            // EARLY mapping: it pins the dynamic-hash constant as explicit storage so
            // it survives the merge/copy-propagation passes that run before the LATE
            // mapping + render (the C++ class comment, coreaction.cc, calls this the
            // whole point of the early pass).  Without it the COPY carrying the
            // equated constant is propagated away, the late `findVarnode` finds
            // nothing, and the forced display format is never applied.  The binding
            // (symbol id) goes on the Varnode so this stays idempotent across passes;
            // the HighVariable mirror (read by the printer) is refreshed when present.
            if let Some(v) = self.vbank.get_mut(vn) {
                v.set_kuna_symbol_entry(sym_id);
            }
            if let Some(high) = self.vbank.get(vn).and_then(|v| v.get_high()) {
                if let Some(h) = self.high_bank.get_mut(high) {
                    h.set_kuna_equate_symbol(sym_id);
                }
            }
            return Ok(true);
        }
        // C++ `Varnode::setSymbolProperties` (varnode.cc:429) updates the Varnode's
        // type and (only when the Symbol is type-locked) binds `mapentry`; in either
        // case the matched Varnode picks up the entry's flags (incl. `Varnode::mapped`
        // via `getAllFlags`), pinning it explicit so it survives to the LATE pass.
        // The kuna stand-in marks `mapped` here but does NOT bind the Varnode's
        // symbol-entry: the NAME (and its struct type/offset) is attached to the
        // HighVariable by the LATE `attemptDynamicMappingLate`, which must still run
        // once a High exists (the early High is frequently absent).  When a High is
        // already present the name is mirrored here too (idempotency key for this
        // `rule_repeatapply` early pass — see `vn_high_has_dynamic_binding`).
        use crate::varnode::varnode_flags;
        let vn_size = self.vbank.get(vn).map(|v| v.get_size()).unwrap_or(0);
        if entry.get_size() == vn_size {
            if let Some(v) = self.vbank.get_mut(vn) {
                v.set_flags_pub(varnode_flags::mapped);
            }
            if let Some(high) = self.vbank.get(vn).and_then(|v| v.get_high()) {
                if let Some(h) = self.high_bank.get_mut(high) {
                    h.set_kuna_name(sym_name);
                    // (kuna LOSS-229) Bind the Symbol id on the high too (C++
                    // `setSymbolProperties`/`high->setSymbol`) so the merge passes'
                    // symbol guard (mergeTestRequired) keeps the dynamic temp distinct
                    // from the field it copies — see ActionMergeRequired re-mapping.
                    h.set_kuna_dynamic_symbol(sym_id);
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    /// C++ `Funcdata::attemptDynamicMappingLate` (`funcdata_varnode.cc:1368`):
    /// find the Varnode a dynamic SymbolEntry maps to (via [`DynamicHash`]) and
    /// attach the Symbol's NAME to it.  Returns `true` if a Varnode was adjusted.
    ///
    /// STUB(W4): the merged tree has no Varnode→SymbolEntry retype link, so the
    /// `vn->setSymbolEntry(entry)` effect is expressed as the kuna stand-in: the
    /// matched HighVariable's `kuna_name` is set to the Symbol's name (read by the
    /// printer as `getSymbol()->getDisplayName()`), and the matched Varnode is
    /// marked `mapped` so the next cleanup-loop `ActionMarkExplicit::baseExplicit`
    /// forces it explicit (C++ `isMapped()` arm, `coreaction.cc:3148`) — exactly
    /// the C++ effect on the dynamic-hash temp.  The `union_facet`/`applyUnionFacet`
    /// arm (no union facets in the merged-tree slices) and the `retypeSymbol`
    /// type-propagation are documented losses; the equate arm reuses the existing
    /// `kuna_equate_symbol` binding.
    pub fn attempt_dynamic_mapping_late(
        &mut self,
        entry: &crate::database::SymbolEntry,
    ) -> KunaResult<bool> {
        use crate::database::symbol_category;
        // Read the symbol's identity, category, name (for the warning), and size up
        // front (snapshot so the `&mut self` DynamicHash search below does not alias
        // the scope borrow).
        let sym_id = entry.symbol;
        let (category, sym_name, sym_name_undefined, sym_type_locked, sym_type) = {
            let localmap = match self.localmap.as_ref() {
                Some(l) => l,
                None => return Ok(false), // no local scope: nothing dynamic to map
            };
            let sym = localmap.database().symbol(sym_id);
            (
                sym.get_category(),
                sym.get_name().to_string(),
                sym.is_name_undefined(),
                sym.is_type_locked(),
                sym.dtype.clone(),
            )
        };
        if category == symbol_category::UNION_FACET {
            return self.apply_union_facet(entry);
        }
        let first_use = entry.get_first_use_address();
        let hash = entry.get_hash();
        let mut dhash = crate::dynamic::DynamicHash::new();
        let vn = match dhash.find_varnode(self, &first_use, hash) {
            Some(v) => v,
            None => return Ok(false),
        };
        // Symbol already applied.  Stand-in: the matched high already carries a name
        // OR an equate binding (idempotent re-run; see vn_high_has_dynamic_binding).
        if self.vn_high_has_dynamic_binding(vn) {
            return Ok(false);
        }
        if category == symbol_category::EQUATE {
            // C++ `setSymbolEntry` marks the Varnode `Varnode::mapped` (varnode.cc:448)
            // and binds the entry; mirror both here as in the early arm
            // (`attempt_dynamic_mapping`) so the re-run stays idempotent.
            if let Some(v) = self.vbank.get_mut(vn) {
                v.set_kuna_symbol_entry(sym_id);
            }
            if let Some(high) = self.vbank.get(vn).and_then(|v| v.get_high()) {
                if let Some(h) = self.high_bank.get_mut(high) {
                    h.set_kuna_equate_symbol(sym_id);
                }
            }
            return Ok(true);
        }
        let vn_size = self.vbank.get(vn).map(|v| v.get_size()).unwrap_or(0);
        if vn_size != entry.get_size() {
            let mut s = String::from("Unable to use symbol ");
            if !sym_name_undefined {
                s.push_str(&sym_name);
                s.push(' ');
            }
            s.push_str(": Size does not match variable it labels");
            self.warning_header(&s);
            return Ok(false);
        }
        // When vn is implied, use the explicit varnode on the other side of a CAST.
        let mut vn = vn;
        if self.vbank.get(vn).map(|v| v.is_implied()).unwrap_or(false) {
            let mut newvn: Option<VarnodeId> = None;
            let v = self.vbank.get(vn);
            let written = v.map(|v| v.is_written()).unwrap_or(false);
            let def = v.and_then(|v| v.get_def());
            if written && def.and_then(|d| self.obank.get(d)).map(|o| o.code()) == Some(OpCode::CPUI_CAST) {
                newvn = def.and_then(|d| self.obank.get(d)).and_then(|o| o.get_in(0));
            } else {
                let mut it = self.vbank.get(vn).map(|v| v.descend_iter().collect::<Vec<_>>()).unwrap_or_default();
                if it.len() == 1 {
                    let castop = it.pop().unwrap();
                    if self.obank.get(castop).map(|o| o.code()) == Some(OpCode::CPUI_CAST) {
                        newvn = self.obank.get(castop).and_then(|o| o.get_out());
                    }
                }
            }
            if let Some(nv) = newvn {
                if self.vbank.get(nv).map(|v| v.is_explicit()).unwrap_or(false) {
                    vn = nv;
                }
            }
        }

        // Bind the Symbol name to the matched high, and mark the Varnode `mapped` so
        // ActionMarkExplicit forces it explicit.  The binding goes on the Varnode too
        // (C++ `setSymbolEntry`) for idempotency.
        if let Some(v) = self.vbank.get_mut(vn) {
            v.set_kuna_symbol_entry(sym_id);
        }
        // The late mapping attaches only the NAME (C++: "the data-type and possibly
        // other properties are not put on the Varnode"); the high keeps its own
        // propagated type for rendering.
        let _ = sym_type;
        if let Some(high) = self.vbank.get(vn).and_then(|v| v.get_high()) {
            if let Some(h) = self.high_bank.get_mut(high) {
                h.set_kuna_name(sym_name);
                // (kuna LOSS-229) mirror the early-arm symbol binding.
                h.set_kuna_dynamic_symbol(sym_id);
            }
        }
        // C++ retypes the Symbol from the Varnode's propagated type
        // (`localmap->retypeSymbol`); the merged-tree Symbol type is not read back
        // by the printer (the high renders through `kuna_symbol_type`), so the
        // retype is a no-op stand-in here.  The type-lock-mismatch warning arm is
        // likewise not reachable in the merged-tree slices (no type-locked dynamic
        // symbols), recorded as a documented loss.
        let _ = sym_type_locked;
        Ok(true)
    }

    /// The forced integer display format of the equate-Symbol bound to `vn`'s
    /// HighVariable (`vn->getHigh()->getSymbol()->getDisplayFormat()`, the value
    /// `PrintC::push_integer` reads at printc.cc:1376).  `0` (no override) when
    /// there is no bound equate Symbol — including the no-local-scope case.
    pub fn vn_high_display_format(&self, vn: VarnodeId) -> uint4 {
        let sym = match self.vn_high_equate_symbol(vn) {
            Some(s) => s,
            None => return 0,
        };
        match self.localmap.as_ref() {
            Some(lm) => lm.database().symbol(sym).get_display_format(),
            None => 0,
        }
    }
}

/// A field-split read view used by [`Funcdata::with_high_split`]: implements
/// [`HighContext`] borrowing only `vbank`/`obank`, so the sibling `high_bank`
/// field stays mutably borrowable.
pub(crate) struct HighReadView<'a> {
    vbank: &'a VarnodeBank,
    obank: &'a PcodeOpBank,
}

impl<'a> HighReadView<'a> {
    /// Build the read view from the two banks (the `funcdata_merge` bridge uses
    /// this for the bank-merge field-split, mirroring [`Funcdata::with_high_split`]).
    pub(crate) fn new(vbank: &'a VarnodeBank, obank: &'a PcodeOpBank) -> HighReadView<'a> {
        HighReadView { vbank, obank }
    }
}

impl<'a> HighContext for HighReadView<'a> {
    fn vn_view(&self, vn: VarnodeId) -> VarnodeView {
        let v = self.vbank.get(vn).expect("vn_view: stale vn");
        let space_internal = v
            .get_addr()
            .get_space()
            .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_INTERNAL)
            .unwrap_or(false);
        let def_time = v
            .get_def()
            .and_then(|op| self.obank.get(op))
            .map(|o| o.get_time())
            .unwrap_or(0);
        VarnodeView {
            flags: v.get_flags(),
            size: v.get_size(),
            type_: std::rc::Rc::clone(v.get_type()),
            type_lock: v.is_type_lock(),
            merge_group: v.get_merge_group(),
            written: v.is_written(),
            def_time,
            space_internal,
            create_index: v.get_create_index(),
        }
    }
    fn vn_cover(&self, vn: VarnodeId) -> Option<Cover> {
        self.vbank.get(vn).and_then(|v| v.cover().cloned())
    }
    fn vn_has_cover(&self, vn: VarnodeId) -> bool {
        self.vbank.get(vn).map(|v| v.has_cover()).unwrap_or(false)
    }
    fn vn_name_view(&self, vn: VarnodeId) -> CompareNameView {
        let v = self.vbank.get(vn).expect("vn_name_view: stale vn");
        let space_internal = v
            .get_addr()
            .get_space()
            .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_INTERNAL)
            .unwrap_or(false);
        let def_time = v
            .get_def()
            .and_then(|op| self.obank.get(op))
            .map(|o| o.get_time())
            .unwrap_or(0);
        CompareNameView {
            name_lock: v.is_name_lock(),
            unaffected: v.is_unaffected(),
            persist: v.is_persist(),
            input: v.is_input(),
            addr_tied: v.is_addr_tied(),
            proto_partial: v.is_proto_partial(),
            space_internal,
            written: v.is_written(),
            def_time,
        }
    }
    fn vn_loc_view(&self, vn: VarnodeId) -> VarnodeViewLoc {
        let v = self.vbank.get(vn).expect("vn_loc_view: stale vn");
        VarnodeViewLoc { addr: v.get_addr().clone() }
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
    use kuna_num::opcodes::OpCode;

    use crate::dtype::{type_metatype, Datatype};
    use crate::context::{ArchContext, TypeOp};

    /// Build an AddrSpaceManager with constant/unique/ram spaces, mirroring the
    /// op.rs/block.rs test harness.
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

    fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.glb.manage().get_space_by_name("ram").unwrap())
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    fn unk_type() -> Rc<Datatype> {
        Rc::new(Datatype::new(4, type_metatype::TYPE_UNKNOWN))
    }

    #[test]
    fn construction_sets_up_containers() {
        let fd = build_fd();
        assert_eq!(fd.get_name(), "func");
        assert_eq!(fd.get_size(), 0x40);
        assert_eq!(fd.num_varnodes(), 0);
        assert!(fd.obank().empty());
        // Two graph roots exist.
        assert_eq!(fd.bblocks_get_size(), 0);
        assert_eq!(fd.sblocks_get_size(), 0);
        // Default flags clear.
        assert!(!fd.is_proc_started());
        assert!(!fd.is_high_on());
        assert!(fd.has_no_struct_blocks());
    }

    #[test]
    fn flag_toggles_match_cpp_masks() {
        let mut fd = build_fd();
        fd.set_no_code(true);
        assert!(fd.has_no_code());
        fd.set_no_code(false);
        assert!(!fd.has_no_code());

        // jumptable recovery toggle clears/sets the *dont* bit (inverse sense).
        fd.set_jumptable_recovery(true);
        assert_eq!(fd.flags & funcdata_flags::jumptablerecovery_dont, 0);
        fd.set_jumptable_recovery(false);
        assert_ne!(fd.flags & funcdata_flags::jumptablerecovery_dont, 0);

        assert!(fd.start_type_recovery()); // first call -> true
        assert!(!fd.start_type_recovery()); // already started -> false
        assert!(fd.has_type_recovery_started());
    }

    #[test]
    fn create_index_phases_track_vbank() {
        let mut fd = build_fd();
        let ram = ram_space(&fd);
        let ct = unk_type();
        // Create a few free varnodes to advance the create index.
        let _ = fd.vbank.create(4, Address::new(Rc::clone(&ram), 0x40), Rc::clone(&ct));
        let _ = fd.vbank.create(4, Address::new(Rc::clone(&ram), 0x44), Rc::clone(&ct));
        let ci = fd.vbank.get_create_index();
        assert_eq!(ci, 2);
        fd.start_clean_up();
        assert_eq!(fd.get_clean_up_index(), 2);
        fd.start_cast_phase();
        assert_eq!(fd.get_cast_phase_index(), 2);
    }

    /// Build a basic block holding `n` ops, returning (block id, op ids in order).
    fn make_block_with_ops(fd: &mut Funcdata, n: int4) -> (BlockId, Vec<OpId>) {
        let root = fd.bblocks_root();
        let bl = fd.bblocks.new_block_basic(root);
        let ram = ram_space(fd);
        let mut ops = Vec::new();
        for i in 0..n {
            let pc = Address::new(Rc::clone(&ram), 0x1000 + i as u64 * 4);
            let op = fd.obank.create_at(2, pc);
            // Give the op an opcode so code() works (COPY = harmless).
            fd.obank.change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
            fd.bb_insert_op(op, bl, None); // append at end
            ops.push(op);
        }
        (bl, ops)
    }

    #[test]
    fn bb_insert_append_order_and_links() {
        let mut fd = build_fd();
        let (bl, ops) = make_block_with_ops(&mut fd, 3);
        assert_eq!(fd.bb_op_len(bl), 3);
        assert_eq!(fd.bb_op_head(bl), Some(ops[0]));
        assert_eq!(fd.bb_op_tail(bl), Some(ops[2]));
        // List order matches insertion order.
        assert_eq!(fd.bb_ops(bl), ops);
        // Orders are strictly increasing (insert assigns midpoints / setOrder).
        let orders: Vec<uintm> = ops
            .iter()
            .map(|&o| fd.obank.get(o).unwrap().get_seq_num().get_order())
            .collect();
        assert!(orders[0] < orders[1] && orders[1] < orders[2]);
    }

    #[test]
    fn bb_insert_before_middle() {
        let mut fd = build_fd();
        let (bl, ops) = make_block_with_ops(&mut fd, 3);
        // Insert a new op before ops[1].
        let ram = ram_space(&fd);
        let pc = Address::new(ram, 0x2000);
        let newop = fd.obank.create_at(1, pc);
        fd.obank.change_opcode(newop, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
        fd.bb_insert_op(newop, bl, Some(ops[1]));
        assert_eq!(fd.bb_op_len(bl), 4);
        assert_eq!(fd.bb_ops(bl), vec![ops[0], newop, ops[1], ops[2]]);
        // The inserted op is ordered between ops[0] and ops[1].
        let o0 = fd.obank.get(ops[0]).unwrap().get_seq_num().get_order();
        let on = fd.obank.get(newop).unwrap().get_seq_num().get_order();
        let o1 = fd.obank.get(ops[1]).unwrap().get_seq_num().get_order();
        assert!(o0 < on && on < o1);
    }

    #[test]
    fn bb_remove_op_fixes_links() {
        let mut fd = build_fd();
        let (bl, ops) = make_block_with_ops(&mut fd, 3);
        // Remove the middle op.
        fd.bb_remove_op(bl, ops[1]);
        assert_eq!(fd.bb_op_len(bl), 2);
        assert_eq!(fd.bb_ops(bl), vec![ops[0], ops[2]]);
        assert_eq!(fd.bb_op_head(bl), Some(ops[0]));
        assert_eq!(fd.bb_op_tail(bl), Some(ops[2]));
        // Removed op has no parent.
        assert_eq!(fd.obank.get(ops[1]).unwrap().get_parent(), None);

        // Remove head.
        fd.bb_remove_op(bl, ops[0]);
        assert_eq!(fd.bb_op_head(bl), Some(ops[2]));
        assert_eq!(fd.bb_op_tail(bl), Some(ops[2]));
        // Remove last remaining.
        fd.bb_remove_op(bl, ops[2]);
        assert!(fd.bb_empty_op(bl));
        assert_eq!(fd.bb_op_head(bl), None);
        assert_eq!(fd.bb_op_tail(bl), None);
    }

    #[test]
    fn branchind_marks_switch_out() {
        use crate::op::pcodeop_flags;
        let mut fd = build_fd();
        let root = fd.bblocks_root();
        let bl = fd.bblocks.new_block_basic(root);
        let ram = ram_space(&fd);
        let op = fd.obank.create_at(1, Address::new(ram, 0x1000));
        // The W6 TypeOp for BRANCHIND carries the `branch` flag; replicate it so
        // is_branch()/code() drive the f_switch_out mark in bb_insert_op.
        fd.obank.change_opcode(
            op,
            TypeOp::new(OpCode::CPUI_BRANCHIND, pcodeop_flags::branch, "BRANCHIND"),
        );
        fd.bb_insert_op(op, bl, None);
        assert!(fd.bblocks.block(bl).is_switch_out());
    }

    #[test]
    fn clear_resets_ir_and_flags() {
        let mut fd = build_fd();
        let (_bl, _ops) = make_block_with_ops(&mut fd, 2);
        fd.set_flag_raw(funcdata_flags::processing_started | funcdata_flags::highlevel_on);
        assert!(fd.is_proc_started());
        fd.clear();
        assert!(!fd.is_proc_started());
        assert!(!fd.is_high_on());
        assert!(fd.obank().empty());
        assert_eq!(fd.num_varnodes(), 0);
        // bblocks reset to a fresh empty graph.
        assert_eq!(fd.bblocks_get_size(), 0);
    }

    // ---- W7 HighVariable / Cover wiring ----------------------------------

    /// Create a coverable (insert-flagged) register varnode at the given offset,
    /// with the given datatype, returning its id.  Mirrors the post-heritage
    /// "real" varnode the merge phase sees.
    fn make_insert_vn(fd: &mut Funcdata, off: u64, ct: Rc<Datatype>) -> VarnodeId {
        let ram = ram_space(fd);
        let id = fd.vbank.create(ct.get_size(), Address::new(ram, off), ct);
        // Mark inserted (output of an op / input) so hasCover() is true.
        fd.vbank.get_mut(id).unwrap().set_insert_for_test();
        id
    }

    #[test]
    fn set_high_level_assigns_a_high_to_each_varnode() {
        let mut fd = build_fd();
        let v1 = make_insert_vn(&mut fd, 0x40, unk_type());
        let v2 = make_insert_vn(&mut fd, 0x48, unk_type());
        assert!(!fd.is_high_on());
        fd.set_high_level();
        assert!(fd.is_high_on());
        // Each non-annotation varnode got a HighVariable.
        assert!(fd.vbank.get(v1).unwrap().get_high().is_some());
        assert!(fd.vbank.get(v2).unwrap().get_high().is_some());
        assert_eq!(fd.high_bank().num_highs(), 2);
        // Distinct highs, in creation order (HighVariableId order).
        assert_ne!(
            fd.vbank.get(v1).unwrap().get_high(),
            fd.vbank.get(v2).unwrap().get_high()
        );
    }

    #[test]
    fn build_dynamic_symbol_guards_and_equate_format_accessors() {
        // The DynamicHash/equate-Symbol creation path needs a local scope + a
        // live decompiled IR (exercised end-to-end by the `force varnode` console
        // command); here we cover the C++ guard arms (`!isHighOn()` /
        // `isTypeLock()`) and the merged-tree `vn->getHigh()->getSymbol()`
        // display-format stand-in accessors (vn_high_equate_symbol /
        // vn_high_display_format), which return the no-equate sentinel until
        // build_dynamic_symbol binds one.
        let mut fd = build_fd();
        let c = fd.new_constant(4, 0xaa);
        let unk = unk_type();

        // !isHighOn() -> "Cannot create dynamic symbols until decompile has completed".
        assert!(!fd.is_high_on());
        let err = fd
            .build_dynamic_symbol(c, 8, Rc::clone(&unk))
            .expect_err("must reject before high-level");
        assert!(err.explain().contains("decompile has completed"));

        fd.set_high_level();
        // No equate symbol bound yet: the stand-in accessors report "none"/0.
        assert!(fd.vn_high_equate_symbol(c).is_none());
        assert_eq!(fd.vn_high_display_format(c), 0);

        // isTypeLock() -> "Trying to build dynamic symbol on locked varnode".
        fd.vbank.get_mut(c).unwrap().set_flags_pub(crate::varnode::varnode_flags::typelock);
        let err = fd
            .build_dynamic_symbol(c, 8, unk)
            .expect_err("must reject a type-locked varnode");
        assert!(err.explain().contains("locked varnode"));
    }

    /// (ghidra Phase 4, review round C2) The DYNAMIC name-recommendation
    /// channel — upstream `ScopeLocal::dynRecommend` +
    /// `recoverNameRecommendationsForSymbols`'s hash loop (varmap.cc:1557).
    ///
    /// This is the mechanism a GUI rename of a variable with hash storage
    /// (anything `requiresDynamicStorage`: a unique-space representative, a
    /// `splitOutMergeGroup` product) comes back through.  Seed a
    /// recommendation keyed on the SAME hash `DynamicHash` computes for a
    /// varnode, apply it, and the varnode's HighVariable must take the
    /// recommended name AND acquire a dynamic Symbol carrying that hash — so
    /// the re-encoded `<localdb>` hands Java a `<mapsym type="dynamic">` it
    /// resolves to the very variable the user renamed.
    #[test]
    fn dynamic_name_recommendation_renames_the_hashed_variable() {
        // A manager WITH a stack space so the Funcdata gets a real ScopeLocal
        // (the recommendation lists live on it).
        let mut m = build_manager();
        let regspc = Rc::new(kuna_base::space::AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            false,
            8,
            1,
            5,
            kuna_base::space::addrspace_flags::hasphysical,
            1,
            1,
        ));
        m.insert_space(Rc::clone(&regspc)).unwrap();
        m.insert_space(Rc::new(kuna_base::space::SpacebaseSpace::new(
            "stack", 6, 8, &regspc, 1, true, false,
        )))
        .unwrap();
        let glb = Rc::new(ArchContext::new(m));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let entry = Address::new(ram, 0x1000);
        let mut fd = Funcdata::new("func", "func", glb, entry, 0x10000000, 0x40).unwrap();
        assert!(fd.get_scope_local().is_some(), "fixture must have a local scope");
        let rs = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().set_start_block(root, bl);
        // out = in + in, so `out` is a written varnode the hasher can key on.
        let a = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x100), None);
        let a = fd.set_input_varnode(a).unwrap();
        let op = fd.new_op(2, Address::new(Rc::clone(&rs), 0x1000));
        fd.op_set_opcode(op, crate::context::TypeOp::new(OpCode::CPUI_INT_ADD, 0, "ADD"));
        let out = fd.new_varnode_out(4, &Address::new(Rc::clone(&rs), 0x200), op).unwrap();
        fd.op_set_input(op, a, 0).unwrap();
        fd.op_set_input(op, a, 1).unwrap();
        fd.op_insert(op, bl, None);
        fd.structure_reset();
        fd.set_high_level();

        // The hash Java would compute for this variable (same algorithm, same
        // hardcoded budget of 8 — DynamicHash.java:440).
        let (hash, hash_addr) =
            crate::dynamic::dynamic_unique_hash(out, 8, &mut fd).expect("hash");
        assert_ne!(hash, 0, "the fixture varnode must hash uniquely");

        fd.seed_dynamic_recommendations(&[("user_renamed".to_string(), hash_addr, hash)]);
        fd.kuna_apply_dynamic_recommendations();

        let high = fd.vbank().get(out).and_then(|v| v.get_high()).expect("high");
        let h = fd.high_bank().get(high).expect("high present");
        assert_eq!(
            h.kuna_name(),
            Some("user_renamed"),
            "the dynamic recommendation did not name the hashed variable"
        );
        assert!(
            h.kuna_link_symbol().is_some(),
            "the recommendation must also bind a Symbol so the wire carries an id"
        );
        // A second apply is idempotent (the name is already resolved).
        fd.kuna_apply_dynamic_recommendations();
        assert_eq!(
            fd.high_bank().get(high).unwrap().kuna_name(),
            Some("user_renamed")
        );
    }

    /// Adversarial (Convert B2): binding a dynamic SymbolEntry to a Varnode marks
    /// it `Varnode::mapped` (the C++ `setSymbolEntry`/`varnode.cc:448` effect that
    /// pins it explicit so the equated COPY survives copy-elimination), and the
    /// binding is the idempotency key — `vn_high_has_dynamic_binding` reports it even
    /// when the Varnode has no HighVariable yet (the early `ActionDynamicMapping`
    /// runs before highs are built; without the Varnode-level key the repeat-apply
    /// pass loops forever).  Generic over any SymbolId — no convert-specific value.
    #[test]
    fn varnode_symbol_entry_binding_marks_mapped_and_is_idempotent() {
        use crate::database::SymbolId;
        let mut fd = build_fd();
        let c = fd.new_constant(4, 0x100);
        // Unbound, no high: not mapped, no dynamic binding.
        assert!(!fd.vbank.get(c).unwrap().is_mapped());
        assert!(fd.vbank.get(c).unwrap().get_high().is_none());
        assert!(!fd.vn_high_has_dynamic_binding(c));
        // Bind an arbitrary symbol id (slotmap default key stands in for any equate).
        let sym: SymbolId = Default::default();
        fd.vbank.get_mut(c).unwrap().set_kuna_symbol_entry(sym);
        // setSymbolEntry marks `mapped` (the B2 copy-elim survival pin) ...
        assert!(fd.vbank.get(c).unwrap().is_mapped());
        // ... and is the idempotency key even with no HighVariable present.
        assert!(fd.vbank.get(c).unwrap().get_high().is_none());
        assert!(fd.vn_high_has_dynamic_binding(c));
        assert_eq!(fd.vbank.get(c).unwrap().kuna_symbol_entry(), Some(sym));
    }

    #[test]
    fn high_get_type_reads_member_datatype() {
        let mut fd = build_fd();
        let int4_ty = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let v = make_insert_vn(&mut fd, 0x40, Rc::clone(&int4_ty));
        fd.set_high_level();
        // The high's type derives from its single member's type (INT).
        let ty = fd.high_get_type(v).expect("high present");
        assert_eq!(ty.get_metatype(), type_metatype::TYPE_INT);
    }

    #[test]
    fn update_varnode_cover_rebuilds_input_def_point() {
        // An input varnode with a single-block cover: its def-point is the input
        // marker in block 0.  Drive the cross-arena cover rebuild.
        let mut fd = build_fd();
        let root = fd.bblocks_root();
        let bl = fd.bblocks.new_block_basic(root);
        // Give the block index 0 (new_block_basic assigns indices in order).
        let _ = bl;
        let ram = ram_space(&fd);
        let v = fd.vbank.create(4, Address::new(ram, 0x40), unk_type());
        // Make it an input + inserted so it has a cover and is non-free.
        fd.vbank.get_mut(v).unwrap().set_insert_for_test();
        fd.vbank.get_mut(v).unwrap().set_input_for_test();
        fd.vbank.get_mut(v).unwrap().calc_cover();
        assert!(fd.vbank.get(v).unwrap().is_cover_dirty_flag());
        fd.update_varnode_cover(v);
        // No longer dirty; the cover now marks the input point in block 0.
        assert!(!fd.vbank.get(v).unwrap().is_cover_dirty_flag());
        let cover = fd.vbank.get(v).unwrap().cover().expect("cover built");
        assert!(!cover.get_cover_block(0).empty());
    }

    #[test]
    fn covermerge_persists_across_with_covermerge_calls() {
        // The persistent `covermerge` (C++ `Funcdata::covermerge`) must survive the
        // move-out / move-back of `with_covermerge`, so the `copyTrims` accumulated
        // by an earlier merge action (`ActionMergeRequired`) reach the later
        // `ActionDominantCopy` (`processCopyTrims`).  Pin that the engine instance
        // and its accumulator persist (this is the architectural fix that lets the
        // dominant-copy hoist see the trim COPYs at all).
        let mut fd = build_fd();
        assert!(fd.covermerge.is_none());
        // First use builds it lazily and reads an empty accumulator.
        let first = fd.with_covermerge(|merge, _data| merge.copy_trims_len());
        assert_eq!(first, 0);
        assert!(fd.covermerge.is_some(), "covermerge built lazily on first use");
        // Push a (fake) trim into the persistent engine, then re-enter: the push
        // must still be visible (the engine was moved back, not re-created).
        let fake = OpId::from(slotmap::KeyData::from_ffi(7));
        fd.covermerge.as_mut().unwrap().push_copy_trim_for_test(fake);
        let second = fd.with_covermerge(|merge, _data| merge.copy_trims_len());
        assert_eq!(second, 1, "copyTrims accumulator survives with_covermerge");
        // clear_covermerge empties it (C++ Merge::clear in Funcdata::clear).
        fd.clear_covermerge();
        let third = fd.with_covermerge(|merge, _data| merge.copy_trims_len());
        assert_eq!(third, 0);
    }
}
