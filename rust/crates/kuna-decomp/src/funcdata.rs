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
use kuna_base::types::{int4, uint4, uintm, Wrap};

use crate::block::{block_flags, BasicData, BlockGraph, BlockKind, FlowBlock};
use crate::op::PcodeOpBank;
use crate::seams::{ArchHandle, BlockId, FuncProto, OpId, Scope, VarnodeId};
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
    /// Prototype of this function (C++ `funcp`).  // SEAM(W4)
    funcp: FuncProto,
    /// Local variables (symbols in the function scope) (C++ `localmap`).
    /// `None` when filled in by decode.  // SEAM(W4)
    localmap: Option<Scope>,
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

        // if (nm.size()==0) localmap = 0; else { ScopeLocal ... }  -- SEAM(W4)
        let localmap = if nm.is_empty() { None } else { Some(Scope) };

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
            funcp: FuncProto,
            localmap,
            jumpvec: Vec::new(),
            vbank,
            obank: PcodeOpBank::new(),
            bblocks,
            sblocks,
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
    /// Get the function's prototype object (C++ `getFuncProto`).  // SEAM(W4)
    pub fn get_func_proto(&self) -> &FuncProto {
        &self.funcp
    }
    /// Get the local function scope (C++ `getScopeLocal`).  // SEAM(W4)
    pub fn get_scope_local(&self) -> Option<&Scope> {
        self.localmap.as_ref()
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
    /// The root graph node of `sblocks`.
    fn sblocks_root(&self) -> BlockId {
        self.sblocks.root.expect("Funcdata: sblocks root not constructed (internal invariant)")
    }
    /// Number of structured blocks (C++ `sblocks.getSize()`).
    pub fn sblocks_get_size(&self) -> int4 {
        let root = self.sblocks_root();
        self.sblocks.block(root).get_size()
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
            // for each op reading oldvn: repoint its slot to newvn, splice it
            // onto newvn's descend list.  Iterate a snapshot since we mutate.
            let readers: Vec<OpId> = bank
                .get(oldvn)
                .map(|vn| vn.descend_iter().collect())
                .unwrap_or_default();
            for op in readers {
                // Repoint every slot of `op` that holds oldvn.
                let ninput = obank.get(op).map(|o| o.num_input()).unwrap_or(0);
                for slot in 0..ninput {
                    if obank.get(op).and_then(|o| o.get_in(slot)) == Some(oldvn) {
                        if let Some(o) = obank.get_mut(op) {
                            o.set_input(Some(newvn), slot);
                        }
                        bank.add_descend(newvn, op)?;
                    }
                }
            }
            // oldvn no longer has descendants (it is about to be deleted).
            bank.destroy_descend(oldvn);
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
        // clearActiveOutput(); funcp.clearUnlockedOutput();         -- SEAM(W4)
        // unionMap.clear();                                          -- SEAM(W6)
        self.clear_blocks();
        self.obank.clear();
        self.vbank.clear();
        // clearCallSpecs();                                          -- SEAM(W4)
        self.clear_jump_tables();
        // heritage.clear(); covermerge.clear();                      -- SEAM(W7)
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
}
