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
//!   - [`Funcdata::set_varnode_properties`] (a `// SEAM(W4)` no-op standing in
//!     for `localmap->queryProperties` + `Cover` calc) that `opSetOutput` and
//!     the `newVarnode*` factories call.
//!
//! ## Deferred surfaces (W4 / W6 / W7 / W8)
//!
//! Most of `funcdata.cc` is W4+ subsystem glue (the `Architecture`/`TypeFactory`
//! / `ScopeLocal` / `FuncProto` / `JumpTable` / `Override` / `Heritage` / `Merge`
//! / union-resolution machinery).  Those are seam-noted ([`crate::seams`]'
//! `Architecture`/`Scope`/`FuncProto`, [`crate::dtype`]) and either return an
//! explicit `Err`/`None` or are left out; printing (`printRaw`/`printBlockTree`)
//! is W8.  This module carries the IR-ownership skeleton, the flag/phase state
//! machine, and the block-manipulation methods that are self-contained at the
//! W3 IR level (`structureReset`, `clearBlocks`, the edge-rewiring wrappers).

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::{int2, int4, uint4, uint8, uintm, Wrap};

use crate::block::{block_flags, BasicData, BlockGraph, BlockKind, FlowBlock};
use crate::fspec::{FuncCallSpecs, FuncProto, ParamActive};
use crate::op::PcodeOpBank;
use crate::seams::{ArchHandle, BlockId, OpId, VarnodeId};
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
/// flag/phase state machine.  The W4+ subsystems (`heritage`, `covermerge`,
/// `activeoutput`, `localoverride`, `lanedMap`, `unionMap`, `qlst`) are
/// seam-noted and omitted until their waves.
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
    /// Number of bytes of binary data in function body (C++ `size`)
    size: int4,
    /// Global configuration data (C++ `glb`).  // SEAM(W4)
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
    /// List of jump-tables for this function (C++ `jumpvec`).
    ///
    /// SEAM(W4): the real `JumpTable` (`jumptable.{hh,cc}`) is W4; the slots are
    /// carried as opaque [`JumpTableId`] handles so `numJumpTables`/`getJumpTable`
    /// and the dead-table sweep in `structureReset` keep their identity, but the
    /// table *contents* live in the W4 wave.
    jumpvec: Vec<JumpTableId>,
    /// Container of Varnode objects for \b this function (C++ `vbank`)
    vbank: VarnodeBank,
    /// Container of PcodeOp objects for \b this function (C++ `obank`)
    obank: PcodeOpBank,
    /// Unstructured basic blocks (C++ `bblocks`)
    bblocks: BlockGraph,
    /// Structured block hierarchy on top of basic blocks (C++ `sblocks`)
    sblocks: BlockGraph,
    /// The HighVariable / VariableGroup / VariablePiece arena (W7, SEAM(W7)).
    ///
    /// The C++ `HighVariable`s are allocated by `new HighVariable` from
    /// `Funcdata::assignHigh`/`Merge` and reverse-linked from each member
    /// `vn->high`; per ADR 0001 they live in this [`HighVariableBank`] keyed by
    /// [`crate::seams::HighVariableId`], the back-link being the `Varnode::high`
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
}

/// Opaque handle for a jump-table (C++ `JumpTable *` slot in `jumpvec`).
///
/// SEAM(W4): the `JumpTable` type and all recovery logic (`recoverJumpTable`,
/// `stageJumpTable`, …) are W4; `Funcdata` only needs to track table identity at
/// W3 so `structureReset`'s dead-table sweep and `installSwitchDefaults` can run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JumpTableId(pub u32);

impl Funcdata {
    /// Constructor (C++ `Funcdata::Funcdata`, `funcdata.cc:34`).
    ///
    /// The C++ pulls `vbank(scope->getArch())`, `minLanedSize` from
    /// `glb->getMinimumLanedRegisterSize()`, and attaches a `ScopeLocal` to the
    /// symbol table.  Here `glb` is the [`ArchHandle`] seam (it carries the
    /// `AddrSpaceManager`); the `VarnodeBank` analysis unique-start is supplied
    /// by the caller (`uniq_start`, the program's `Translate` —
    /// `getUniqueStart(ANALYSIS)`), exactly as `varnode.rs` documents.  The
    /// `ScopeLocal` attachment is `// SEAM(W4)`; here `localmap` is created empty
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
            size: sz,
            glb,
            name: nm.to_string(),
            display_name: disp.to_string(),
            baseaddr: addr,
            funcp: FuncProto::new(),
            activeoutput: None,
            localmap,
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
    pub fn apply_locked_prototype(
        &mut self,
        pieces: &crate::fspec::PrototypePieces,
    ) -> KunaResult<()> {
        let defaultfp = match self.glb.default_fp() {
            Some(m) => Rc::clone(m),
            None => return Ok(()),
        };
        let void_type =
            Rc::new(crate::dtype::Datatype::new(0, crate::dtype::type_metatype::TYPE_VOID));
        // The type factory + manager live on the architecture, shared into `glb`.
        // Clone the `Rc<ArchSeam>` (cheap refcount bump) so the factory/manager
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
    /// `Funcdata::findJumpTable`).
    ///
    /// SEAM(W7): `ActionDeadCode` uses this only for the BRANCHIND switch-var
    /// consume mask; with no jump-table recovery (`JumpTable` is a W7 seam) this
    /// is always `None`, so the BRANCHIND input is treated as fully consumed (the
    /// conservative default the C++ takes when `jt == 0`).
    pub fn find_jump_table(&self, _op: OpId) -> Option<()> {
        None
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
        let mut heritage = std::mem::take(&mut self.heritage);
        heritage.build_info_list(self);
        heritage.heritage(self);
        self.heritage = heritage;
    }

    /// Get the heritage pass when the given address was last heritaged, or -1
    /// (C++ `Funcdata::isHeritaged` reads `heritage.heritagePass`).
    pub fn heritage_pass(&self, addr: &Address) -> int4 {
        self.heritage.heritage_pass(addr)
    }

    /// Overall count of heritage passes (C++ `Funcdata::getHeritagePass`,
    /// `funcdata.hh:239` — `heritage.getPass()`).
    pub fn get_heritage_pass(&self) -> int4 {
        self.heritage.get_pass()
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

    // -----------------------------------------------------------------------
    // Flag query / toggle methods (C++ inline `is*`/`has*`/`set*`)
    // -----------------------------------------------------------------------

    /// Are high-level variables assigned to Varnodes (C++ `isHighOn`).
    pub fn is_high_on(&self) -> bool {
        (self.flags & funcdata_flags::highlevel_on) != 0
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
    /// half while mutating the `vbank` half:
    ///
    /// ```text
    ///   let (vbank, obank) = self.banks_mut();
    ///   let mut replace = Funcdata::replace_reads_thunk(obank);
    ///   let vn = vbank.set_def(vn, def, &mut replace)?;
    /// ```
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
        self.sblocks.build_copy_from(sroot, &self.bblocks, broot);
    }

    // -----------------------------------------------------------------------
    // Jump-table identity (W4 contents seamed out)
    // -----------------------------------------------------------------------

    /// Get the number of jump-tables for \b this function (C++ `numJumpTables`).
    pub fn num_jump_tables(&self) -> int4 {
        self.jumpvec.len() as int4
    }
    /// Get the i-th jump-table handle (C++ `getJumpTable`).  // SEAM(W4)
    pub fn get_jump_table(&self, i: int4) -> JumpTableId {
        self.jumpvec[i as usize]
    }
    /// Mutable access to the jump-table handle vector (for the W4 jump-table
    /// wave and the `clear_jump_tables`/`structure_reset` sweeps).  // SEAM(W4)
    pub(crate) fn jumpvec_mut(&mut self) -> &mut Vec<JumpTableId> {
        &mut self.jumpvec
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
    /// SEAM(W3-op): the op-side read iteration/repointing is the funcdata_op
    /// wave's; this method establishes the closure shape and where it captures.
    /// Until funcdata_op ports `opSetInput`, the bodies that *call* `xref`
    /// (`setInputVarnode`, `opSetOutput`) live in funcdata_op; this thunk is the
    /// surface they use, declared here so they need no seam edit.
    pub fn replace_reads_thunk(obank: &mut PcodeOpBank) -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> + '_ {
        move |bank: &mut VarnodeBank, oldvn: VarnodeId, newvn: VarnodeId| -> KunaResult<()> {
            // Faithful transcription of `VarnodeBank::replace` (varnode.cc:1351).
            // C++ walks oldvn's descend list (one entry per op-read of oldvn) and,
            // for each non-skipped entry, severs *that one* link, repoints the
            // single slot getSlot finds, and adds the op to newvn's descend:
            //
            //   while(iter!=oldvn->descend.end()) {
            //     op = *iter; tmpiter = iter++;
            //     if (op->output == newvn) continue;   // self-def: not an input
            //     i = op->getSlot(oldvn);
            //     oldvn->descend.erase(tmpiter);
            //     op->clearInput(i); newvn->addDescend(op); op->setInput(newvn,i);
            //   }
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
                // `if (op->output == newvn) continue;` — an op cannot be an input
                // to its own definition; leave its slot reading oldvn and leave
                // oldvn's descend link to it untouched.
                if obank.get(op).and_then(|o| o.get_out()) == Some(newvn) {
                    continue;
                }
                // `i = op->getSlot(oldvn);` — the first slot reading oldvn; this
                // descend entry corresponds to exactly that read.  (-1 only if a
                // prior entry for the same op already consumed the read, leaving
                // none — then there is no slot to repoint and no link to sever.)
                let i = obank.get(op).map(|o| o.get_slot(oldvn)).unwrap_or(-1);
                if i < 0 {
                    continue;
                }
                // `oldvn->descend.erase(tmpiter);` — sever just this one link.
                bank.erase_descend(oldvn, op);
                // `op->clearInput(i); newvn->addDescend(op); op->setInput(newvn,i);`
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

    /// Look-up boolean properties and data-type information for a Varnode
    /// (C++ `Funcdata::setVarnodeProperties`).
    ///
    /// SEAM(W4): the real body queries `localmap->queryProperties` for the
    /// symbol entry and, if `isHighOn()`, calls `vn->calcCover()` (W7).  The W3
    /// IR has no symbol scope and no HighVariable, so this is a no-op that the
    /// op/varnode factories can call unconditionally; W4 fills the body and the
    /// callers stay unchanged.
    pub fn set_varnode_properties(&mut self, _vn: VarnodeId) {
        // localmap->queryProperties(...) ; if (isHighOn()) vn->calcCover();
        //   -- SEAM(W4)/SEAM(W7): no scope, no HighVariable yet.
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
    /// Faithful transcription of `BlockBasic::insert` (`block.cc:2262`): set the
    /// op's parent, splice it onto the per-op intrusive basic-block list before
    /// `before`, then compute `ordbefore`/`ordafter` from neighbours and either
    /// recompute the whole order ([`bb_set_order`](Funcdata::bb_set_order)) or
    /// set the midpoint (overflow-aware).  The BRANCHIND `f_switch_out` mark is
    /// applied via the block flags.
    pub fn bb_insert_op(&mut self, op: OpId, bl: BlockId, before: Option<OpId>) {
        // inst->setParent(this);
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
            // inst->setOrder(ordafter/2 + ordbefore/2);  // beware overflow
            let mid = (ordafter / 2).wadd(ordbefore / 2);
            self.obank.get_mut(op).expect("bb_insert_op").set_order(mid);
        }

        // if (inst->isBranch()) { if (code()==BRANCHIND) setFlag(f_switch_out); }
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
        // inst->setParent(0);
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

        // localmap->clearUnlocked(); localmap->resetLocalWindow();  -- SEAM(W4)
        // clearActiveOutput() (funcdata.cc): drop the output-trial state.
        self.clear_active_output();
        // funcp.clearUnlockedOutput();                               -- SEAM(W4)
        // unionMap.clear();                                          -- SEAM(W6)
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
// W7 HighVariable / Cover lifecycle wiring (SEAM(W7))
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
    /// Borrow the HighVariable arena (the W7 high-variable map).
    pub fn high_bank(&self) -> &crate::variable::HighVariableBank {
        &self.high_bank
    }
    /// Mutably borrow the HighVariable arena.
    pub fn high_bank_mut(&mut self) -> &mut crate::variable::HighVariableBank {
        &mut self.high_bank
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
                // MULTIEQUALs are considered very beginning -> 0
                return (0, code);
            } else if code == OpCode::CPUI_INDIRECT {
                // INDIRECTs are at the location of the op they are indirect for:
                // getOpFromConst(getIn(1)->getAddr())->getSeqNum().getOrder()
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
        Ok(copy_op)
    }

    /// C++ `Merge::trimOpOutput` (`merge.cc:656`): bump the op's output forward
    /// through a new COPY so its Cover shrinks to a single point.
    pub(crate) fn do_trim_op_output(&mut self, op: OpId) -> KunaResult<()> {
        let code = self.obank.get(op).expect("do_trim_op_output: stale op").code();
        let afterop = if code == OpCode::CPUI_INDIRECT {
            // getOpFromConst(op->getIn(1)->getAddr())
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

    /// `op->outputTypeLocal()` — the local-from-op output type (W6 type-op
    /// table).  Conservative unknown of the op's output size; reached only by
    /// `markInternalCopies` (not on the `mergeMarker` path).
    pub(crate) fn op_output_type_local_pub(&self, op: OpId) -> Rc<Datatype> {
        let sz = self
            .obank
            .get(op)
            .and_then(|o| o.get_out())
            .and_then(|out| self.vbank.get(out))
            .map(|v| v.get_size())
            .unwrap_or(1);
        Rc::new(Datatype::new(sz, crate::dtype::type_metatype::TYPE_UNKNOWN))
    }

    /// `op->inputTypeLocal(slot)` — see [`op_output_type_local_pub`].
    pub(crate) fn op_input_type_local_pub(&self, op: OpId, slot: int4) -> Rc<Datatype> {
        let sz = self
            .obank
            .get(op)
            .and_then(|o| o.get_in(slot))
            .and_then(|inv| self.vbank.get(inv))
            .map(|v| v.get_size())
            .unwrap_or(1);
        Rc::new(Datatype::new(sz, crate::dtype::type_metatype::TYPE_UNKNOWN))
    }

    /// C++ `Cover single; single.addDefPoint(vn); single.addRefPoint(op,vn)`
    /// (`merge.cc:503-505`) — the cover of a single read.  Reached by
    /// `eliminateIntersect` (not on the `mergeMarker` path, which is what the
    /// boolless vertical slice drives); the full `addRefPoint` walk is the
    /// [`Cover::rebuild`] machinery, surfaced when `processCopyTrims`/
    /// `eliminateIntersect` are scheduled.
    pub(crate) fn build_single_read_cover(&self, vn: VarnodeId, _op: OpId) -> Cover {
        let mut single = Cover::new();
        let ctx = FuncdataCoverCtx { fd: self };
        let (def, is_input) = ctx.def_point(vn);
        single.add_def_point(def, is_input);
        single
    }

    /// C++ `Merge::checkCopyPair` cover range (`merge.cc:1120-1121`).  Reached by
    /// `processCopyTrims` (not on the `mergeMarker` path) — see
    /// [`build_single_read_cover`].
    pub(crate) fn build_copy_pair_range(&self, dom_op: OpId, _sub_op: OpId) -> Cover {
        let mut range = Cover::new();
        let ctx = FuncdataCoverCtx { fd: self };
        if let Some(dom_out) = self.obank.get(dom_op).and_then(|o| o.get_out()) {
            let (def, is_input) = ctx.def_point(dom_out);
            range.add_def_point(def, is_input);
        }
        range
    }

    /// The `getTiedVarnode`/`getInputVarnode` read on a HighVariable, across the
    /// `high_bank` <-> `vbank`/`obank` field split (the bridge cannot destructure
    /// private fields from another module).  `which` selects tied (`false`) vs
    /// input (`true`).
    pub(crate) fn high_tied_or_input_varnode(
        &self,
        high: crate::seams::HighVariableId,
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
        high1: crate::seams::HighVariableId,
        high2: crate::seams::HighVariableId,
        isspeculative: bool,
        cache: &mut crate::variable::HighIntersectTest,
        set_high_log: &mut Vec<(VarnodeId, crate::seams::HighVariableId, int2)>,
        mark_set: &std::cell::RefCell<std::collections::BTreeSet<crate::seams::HighVariableId>>,
    ) -> KunaResult<()> {
        let Funcdata { high_bank, vbank, obank, .. } = self;
        let ctx = HighReadView::new(vbank, obank);
        let mut set_high = |vn: VarnodeId, id: crate::seams::HighVariableId, mg: int2| {
            set_high_log.push((vn, id, mg));
        };
        let mut set_mark = |id: crate::seams::HighVariableId| {
            mark_set.borrow_mut().insert(id);
        };
        let mut clear_mark = |id: crate::seams::HighVariableId| {
            mark_set.borrow_mut().remove(&id);
        };
        let is_mark = |id: crate::seams::HighVariableId| mark_set.borrow().contains(&id);
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
            let bl = self.bblocks_get_block(0);
            (bl, self.block_stop_addr(bl), None)
        } else {
            let def = v.get_def().expect("snip_reads_insert_point: non-input has no def");
            let bl = self.obank.get(def).and_then(|o| o.get_parent()).expect("snip: def no parent");
            let pc = self.obank.get(def).map(|o| o.get_addr().clone()).unwrap_or_else(Address::new_invalid);
            (bl, pc, Some(def))
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
        high: crate::seams::HighVariableId,
        copy: &[OpId],
        pos: int4,
        size: int4,
    ) -> KunaResult<()> {
        // blockSet = { copy[pos+i]->getParent() }; domBl = findCommonBlock(blockSet)
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
            // domCopy = data.newOp(1, domBl->getStop()); SetOpcode(COPY)
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
                // bCover.merge(*vn->getCover()): the rebuilt member cover.
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
                // op->clearMark() (the marked-set was local; nothing to clear)
                continue;
            }
            let out_vn = self.obank.get(op).and_then(|o| o.get_out()).expect("build_dominant_copy: copy out");
            if out_vn != dom_vn {
                // outVn->getHigh()->remove(outVn)
                if let Some(out_high) = self.vbank.get(out_vn).and_then(|v| v.get_high()) {
                    self.high_remove_member(out_high, out_vn);
                }
                self.total_replace(out_vn, dom_vn)?;
                self.op_destroy(op);
            }
        }

        if count > 0 && dom_copy_is_new {
            // high->merge(domVn->getHigh(), 0, true)
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
    fn high_remove_member(&mut self, high: crate::seams::HighVariableId, vn: VarnodeId) {
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
        high1: crate::seams::HighVariableId,
        high2: crate::seams::HighVariableId,
        isspeculative: bool,
    ) -> KunaResult<()> {
        let opset = crate::cover::PcodeOpSet::new(Box::new(Vec::new), Box::new(|_, _| false));
        let mut cache = crate::variable::HighIntersectTest::new(opset);
        let mut set_high_log: Vec<(VarnodeId, crate::seams::HighVariableId, int2)> = Vec::new();
        let mark_set: std::cell::RefCell<std::collections::BTreeSet<crate::seams::HighVariableId>> =
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
    /// Merge driver after data-flow changes.  This is the `// SEAM(W7)` cover
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
struct FuncdataCoverCtx<'a> {
    fd: &'a Funcdata,
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
            // for j in 0..numInput: if getIn(j)==vn -> addRefRecurse(bl->getIn(j))
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
    /// SEAM(W4): the `hasWarning`/`issueDatatypeWarning` datatype-warning step
    /// (`glb->types`) is a W4 surface and is omitted.  The `calcCover` + `new
    /// HighVariable(vn)` + `vn->setHigh(id,0)` lifecycle is wired here.
    pub fn assign_high_var(&mut self, vn: VarnodeId) -> Option<crate::seams::HighVariableId> {
        if !self.is_high_on() {
            return None;
        }
        let v = self.vbank.get(vn).expect("assign_high_var: stale vn");
        if v.has_cover() {
            self.vbank_mut().get_mut(vn).unwrap().calc_cover();
        }
        // if (!vn->isAnnotation()) return new HighVariable(vn);
        if self.vbank.get(vn).unwrap().is_annotation() {
            return None;
        }
        let id = self.high_bank.new_high(vn);
        // vn->setHigh(this, numMergeClasses-1) == setHigh(id, 0)
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
    /// partial-union case (SEAM(W4): `None` until the Varnode-Symbol link lands).
    pub fn high_get_type(&mut self, vn: VarnodeId) -> Option<std::rc::Rc<crate::dtype::Datatype>> {
        let id = self.vbank.get(vn)?.get_high()?;
        Some(self.with_high_split(|hb, ctx| hb.get_mut(id).unwrap().get_type(ctx, None)))
    }

    /// Drive a HighVariable's external cover update (the C++
    /// `HighVariable::updateCover`, called by Merge).  Convenience over
    /// [`with_high_split`] for the bank's `update_cover`.
    pub fn high_update_cover(&mut self, id: crate::seams::HighVariableId) {
        self.with_high_split(|hb, ctx| hb.update_cover(id, ctx));
    }

    /// The HighVariable's name representative Varnode (C++
    /// `HighVariable::getNameRepresentative`), across the bank field-split.
    /// `None` if the high is gone.
    pub fn high_name_representative(&mut self, id: crate::seams::HighVariableId) -> Option<VarnodeId> {
        self.high_bank.get(id)?;
        Some(self.with_high_split(|hb, ctx| hb.get_mut(id).unwrap().get_name_representative(ctx)))
    }

    /// Whether a HighVariable can carry a name (C++ `HighVariable::hasName`),
    /// across the bank field-split.  `false` if the high is gone or its
    /// coverability check errors (the C++ `LowlevelError` -> conservative `false`).
    pub fn high_has_name(&mut self, id: crate::seams::HighVariableId) -> bool {
        if self.high_bank.get(id).is_none() {
            return false;
        }
        self.with_high_split(|hb, ctx| hb.get_mut(id).unwrap().has_name(ctx).unwrap_or(false))
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
    use crate::seams::{Architecture, TypeOp};

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
        let glb = Rc::new(Architecture::new(manage));
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
