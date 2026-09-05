//! Port of `decompiler/cpp/flow.{hh,cc}` (W3, item `w3-ir-flow`) — [`FlowInfo`],
//! the class that generates the control-flow structure for a single function by
//! following machine-instruction p-code, instruction-by-instruction, from the
//! entry address.
//!
//! ## Two-phase model (verbatim from `flow.hh`)
//!
//! Control-flow is generated in two phases.  [`FlowInfo::generate_ops`] produces
//! every raw p-code op (into the [`PcodeOpBank`](crate::op::PcodeOpBank) \e dead
//! list) for every machine instruction reachable from the entry point, following
//! all flow and trimming at RETURN; CALL/CALLIND are fall-through.
//! [`FlowInfo::generate_blocks`] then organizes the raw ops into
//! `PcodeBlockBasic`s and wires the directed control-flow edges.
//!
//! ## What this wave ports (self-contained at the W3 IR level)
//!
//! The **pure control-flow analysis** over the op graph + the visited-instruction
//! map + the address stack is fully transcribed, statement-for-statement, because
//! it touches only the W3 IR surface (`Funcdata`'s op/varnode/block banks, the
//! `op*` mutators, `Address`/`SeqNum`):
//!
//!   - the [`VisitStat`] map and `target`/`branch_target`/`find_rel_target`/
//!     `fallthru_op`/`new_address`/`update_target` lookups — **THE op-creation
//!     and SeqNum-allocation order of the decompiler** (the `addrlist` stack walk
//!     in `fallthru`/`set_fallthru_bound`, the `xref_control_flow` classification,
//!     and the `maxtime`/`delete_remaining_ops` relative-branch trimming) is
//!     transcribed exactly;
//!   - `artificial_halt`, `reinterpreted`, `handle_out_of_bounds`,
//!     `find_unprocessed`, `dedup_unprocessed`, `fillin_branch_stubs`;
//!   - the block-building trio `collect_edges`/`split_basic`/`connect_basic` and
//!     the `generate_blocks` shell.
//!
//! ## Cross-wave boundary: the [`FlowEnvironment`] trait (W2 wired, W4/W6 deferred)
//!
//! `FlowInfo` in C++ reaches its owning `Architecture` (`glb`) for several
//! subsystems that are **not** part of the W3 IR: the SLEIGH `Translate`
//! (`glb->translate->oneInstruction`, W2 — *available* as the kuna-sleigh
//! [`Translate`](kuna_sleigh::translate::Translate) trait), the W6 op-code →
//! `TypeOp` table (`glb->inst[opc]`, needed so `opSetOpcode(op, OpCode)` resolves
//! the behavioral-class flags `is_branch()`/`is_code_ref()`/`code()` read), the
//! user-op table (`glb->userops`, the CALLOTHER-injected check), the p-code
//! injection library (`glb->pcodeinjectlib`), and the per-function `Override`
//! hooks (`data.getOverride()`).  None of those live on the W3 `Architecture`
//! skeleton ([`crate::context::ArchContext`]).
//!
//! Rather than half-wire them, this module declares a **local boundary trait**
//! [`FlowEnvironment`] carrying exactly the slice `FlowInfo` needs: the
//! `Translate` for `oneInstruction`, an op-code→`TypeOp` resolver (the W6 table),
//! the flow-override lookup (W4 `Override`), the CALLOTHER-injected predicate
//! (W4 user-ops), and the V850/SPARC kuna anchor predicates.  The W3-portable
//! algorithms drive `FlowInfo` against this trait, so they compile and are
//! testable against a real [`Translate`] instance now; W4 supplies the concrete
//! `Architecture`-backed impl by wiring the trait, without touching the ported
//! algorithm bodies.
//!
//! ## Deferred surfaces (`// STUB(W4)` / `// STUB(W6)`) — losses
//!
//! The op-building emitter (C++ `PcodeEmitFd::dump`) needs `newVarnodeOut`
//! (→ `opSetOutput` → the `(vbank,obank)` split-borrow accessor that only the
//! `funcdata` owner can add) and `newCodeRef` (a `funcdata_varnode` factory not
//! yet ported); both are boundary-noted.  The call-site machinery (`FuncCallSpecs`,
//! `setupCallSpecs`/`setupCallindSpecs`/`checkForFlowModification`/`queryCall`),
//! jump-table recovery (`JumpTable`, `recoverJumpTables`/`truncateIndirectJump`/
//! `checkMultistageJumptables`), p-code injection (`injectPcode`/`doInjection`/
//! `injectUserOp`/`inlineSubFunction`/the inline-clone family), `Override` hooks,
//! and the `warning`/`warningHeader`/`removeUnreachableBlocks` reporting are all
//! W4 subsystems — their methods are present as faithful skeletons that drive the
//! W3-portable parts and return a precise [`KunaError`] / `// STUB(W4)` note at
//! the subsystem boundary.  Recorded in the structured `losses`.
//!
//! ## GH-8817 V850 anchor (`flow.cc:278`)
//!
//! The kuna `xref_control_flow` anchor edit (reclassify a V850 `jmp [reg]` CALLIND
//! to BRANCHIND for switch recovery) is transcribed; the `option
//! v850indirectbranch` read is the `// STUB(W4): ArchOption v850indirectbranch`
//! default-false predicate on [`FlowEnvironment`] (and the SPARC struct-return
//! `// STUB` anchor of `flow.cc:326` likewise).

use kuna_base::address::{Address, SeqNum};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4, uintb, uintm};
use kuna_num::opcodes::OpCode;
use kuna_sleigh::translate::{PcodeEmit, Translate};
use std::collections::BTreeMap;

use crate::funcdata::Funcdata;
use crate::jumptable::RecoveryMode;
use crate::op::pcodeop_flags;
use crate::context::{BlockId, OpId, TypeOp};

/// Boolean options for flow following (C++ anonymous `enum` in `class FlowInfo`,
/// `flow.hh:60-74`).  Verbatim transcription of the bit values.
pub mod flow_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;

    /// Ignore/truncate flow into addresses out of the specified range
    pub const ignore_outofbounds: uint4 = 1;
    /// Treat unimplemented instructions as a NOP (no operation)
    pub const ignore_unimplemented: uint4 = 2;
    /// Throw an exception for flow into addresses out of the specified range
    pub const error_outofbounds: uint4 = 4;
    /// Throw an exception for flow into unimplemented instructions
    pub const error_unimplemented: uint4 = 8;
    /// Throw an exception for flow into previously encountered data at a different \e cut
    pub const error_reinterpreted: uint4 = 0x10;
    /// Throw an exception if too many instructions are encountered
    pub const error_toomanyinstructions: uint4 = 0x20;
    /// Indicate we have encountered unimplemented instructions
    pub const unimplemented_present: uint4 = 0x40;
    /// Indicate we have encountered flow into unaccessible data
    pub const baddata_present: uint4 = 0x80;
    /// Indicate we have encountered flow out of the specified range
    pub const outofbounds_present: uint4 = 0x100;
    /// Indicate we have encountered reinterpreted data
    pub const reinterpreted_present: uint4 = 0x200;
    /// Indicate the maximum instruction threshold was reached
    pub const toomanyinstructions_present: uint4 = 0x400;
    /// Indicate a CALL was converted to a BRANCH and some code may be unreachable
    pub const possible_unreachable: uint4 = 0x1000;
    /// Indicate flow is being generated to in-line (a function)
    pub const flow_forinline: uint4 = 0x2000;
    /// Indicate that any jump table recovery should record the table structure
    pub const record_jumploads: uint4 = 0x4000;
}

/// The kuna flow-override modes (C++ `Override::NONE`/... , `override.hh`).
///
/// STUB(W4): the per-function `Override` subsystem (`override.{hh,cc}`) is W4;
/// only the `NONE` sentinel is reached by `process_instruction`'s override branch,
/// so the W3-portable shell carries the `NONE` discriminant and the
/// [`FlowEnvironment::flow_override`] hook reports it.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowOverride {
    /// No override (C++ `Override::NONE`).
    NONE,
}

/// Which kind of injection [`FlowInfo::do_injection`] is weaving in (C++
/// `doInjection` resolves the payload differently for the two callers, but the
/// weave-in tail is shared).
#[derive(Debug, Clone, Copy)]
enum InjectSource {
    /// A CALLOTHER user-op injection — payload resolved via the user-op table
    /// (C++ `FlowInfo::injectUserOp`, `fc == 0`).  Carries the user-op index.
    UserOp(uintb),
    /// A CALL/CALLIND call-fixup injection — payload resolved by the call spec's
    /// parked inject id (C++ `FlowInfo::injectSubFunction`, `fc != 0`).  Carries
    /// the inject id.
    CallFixup(int4),
}

/// \brief A helper describing the number of bytes in a machine instruction and
/// the starting p-code op (C++ `struct FlowInfo::VisitStat`, `flow.hh:77`).
#[derive(Debug, Clone)]
pub struct VisitStat {
    /// Sequence number of first PcodeOp in the instruction (or INVALID if no
    /// p-code) (C++ `seqnum`).
    pub seqnum: SeqNum,
    /// Number of bytes in the instruction (C++ `size`).
    pub size: int4,
}

/// The flow's instruction-boundary map (C++ `FlowInfo::visited`).  A type alias
/// so the jump-table recovery's partial-clone seeding signatures stay readable.
/// (`Address` is logically immutable as a key — its interior is a memoized space
/// lookup, never mutated through a `&` key — so the `clippy::mutable_key_type`
/// lints on the public signatures using it are `#[allow]`ed.)
pub type VisitedMap = BTreeMap<Address, VisitStat>;

/// The reduced "jumptable" action-pipeline runner the recovery chain calls per
/// partial clone (it builds the partial's blocks + runs the action set).  Type
/// alias to keep the recovery signatures from tripping `clippy::type_complexity`.
pub type JtPipelineFn<'a> = dyn FnMut(&mut Funcdata, &VisitedMap) -> KunaResult<()> + 'a;

/// The W4/W6 subsystem slice [`FlowInfo`] reaches through its owning
/// `Architecture` (C++ `glb`) and per-function `Override`.
///
/// This is a **local boundary** (the serial-chain porter's right): it abstracts
/// exactly the non-W3-IR surface the flow algorithms touch, so they compile and
/// are testable against a real [`Translate`] now.  W4 supplies the concrete
/// `Architecture`/`Override`-backed implementor; the algorithm bodies in this
/// module never change.
pub trait FlowEnvironment {
    /// The SLEIGH translator (C++ `glb->translate`).  W2 — *available*.
    ///
    /// `one_instruction(emit, baseaddr)` lifts one machine instruction's raw
    /// p-code, returning the instruction length in bytes.
    fn translate(&self) -> &dyn Translate;

    /// Resolve an [`OpCode`] to its behavioral-class [`TypeOp`]
    /// (C++ `glb->inst[opc]`).  // STUB(W6)
    ///
    /// The returned `TypeOp` must carry the op-code's property-flag word
    /// (`pcodeop_flags::branch`/`call`/`coderef`/`marker`/…) so that, after
    /// `opSetOpcode`, the op's `is_branch()`/`is_call()`/`is_code_ref()`/`code()`
    /// queries the flow classifier reads return the C++ values.  The W3 test impl
    /// builds these from a static table; W6 returns the interned singleton.
    fn resolve_typeop(&self, opc: OpCode) -> TypeOp;

    /// Get the flow-override mode registered for `addr` (C++
    /// `data.getOverride().getFlowOverride(addr)`).  // STUB(W4)
    ///
    /// The W3 shell always reports [`FlowOverride::NONE`].
    fn flow_override(&self, _addr: &Address) -> FlowOverride {
        FlowOverride::NONE
    }

    /// Does the function have any registered flow-override instructions (C++
    /// `data.getOverride().hasFlowOverride()`).  // STUB(W4)
    fn has_flow_override(&self) -> bool {
        false
    }

    /// Is the CALLOTHER `op` a user-op marked as \e injected (C++
    /// `glb->userops.getOp(in0)->getType() == UserPcodeOp::injected`).
    ///
    /// The default reports `false` (the W3 shell has no user-op table); the
    /// engine-backed [`ArchFlowEnv`](crate::decompile_drive) queries
    /// `glb->userops.getOp(index)->getType()`.  When true, the CALLOTHER is
    /// queued onto `injectlist` and replaced by its callother-fixup p-code in
    /// `inject_pcode` (e.g. the MIPS `setISAMode` no-op that, once injected as a
    /// dead COPY, is dead-code-eliminated).
    fn is_injected_userop(&self, _userop_index: uintb) -> bool {
        false
    }

    /// Emit the callother-fixup p-code for the \e injected user-op `userop_index`
    /// into `emit`, resolving its parameters against `context` (C++
    /// `FlowInfo::injectUserOp` → `payload->inject(icontext,emitter)` via the
    /// SLEIGH inject engine).
    ///
    /// The default errs (the W3 shell has no inject library / compiled template);
    /// the engine-backed env looks up the user-op's `InjectedUserOp` payload, its
    /// compiled `ConstructTpl`, and drives `SleighInjectEngine::emit_payload`.
    fn inject_userop(
        &self,
        _userop_index: uintb,
        _context: &mut crate::pcodeinject::InjectContext,
        _emit: &mut dyn kuna_sleigh::translate::PcodeEmit,
    ) -> KunaResult<()> {
        Err(KunaError::lowlevel("inject_userop: no inject engine in this FlowEnvironment"))
    }

    /// Is the \e injected user-op `userop_index`'s callother-fixup payload marked
    /// \e incidental-copy (C++ `payload->isIncidentalCopy()`)?  When true,
    /// `doInjection` marks the injected COPYs incidental so deadcode/expression
    /// folding can dissolve them (the MIPS `setISAMode` fixup sets it).  The W3
    /// shell reports `false`.
    fn is_incidental_copy_userop(&self, _userop_index: uintb) -> bool {
        false
    }

    /// Emit a \e call-fixup injection payload (resolved by raw inject id) into
    /// `emit` (C++ `FlowInfo::injectSubFunction` → `payload->inject(icontext,
    /// emitter)` via the SLEIGH inject engine).  Unlike [`inject_userop`] the
    /// payload is named by `inject_id` directly (`fc->getInjectId()`, parked by
    /// `IfcFixupApply`), not via the user-op table.  The default errs (no inject
    /// engine in the W3 shell); the engine-backed env drives
    /// `SleighInjectEngine::emit_payload` over the library payload + compiled tpl.
    fn inject_call_fixup_payload(
        &self,
        _inject_id: int4,
        _context: &mut crate::pcodeinject::InjectContext,
        _emit: &mut dyn kuna_sleigh::translate::PcodeEmit,
    ) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "inject_call_fixup_payload: no inject engine in this FlowEnvironment",
        ))
    }

    /// Is the call-fixup payload `inject_id`'s body marked \e incidental-copy
    /// (C++ `payload->isIncidentalCopy()`)?  The W3 shell reports `false`.
    fn is_incidental_copy_payload(&self, _inject_id: int4) -> bool {
        false
    }

    /// The \e param-shift of the call-fixup payload `inject_id` (C++
    /// `payload->getParamShift()`): the number of parameters shifted in the
    /// original call, propagated to the injected call's callspec.  The W3 shell
    /// reports `0`.
    fn call_fixup_param_shift(&self, _inject_id: int4) -> int4 {
        0
    }

    /// The display name of the call-fixup `inject_id` (C++
    /// `glb->pcodeinjectlib->getCallFixupName(injectid)`), used in the
    /// `warningHeader` after a sub-function injection.  The W3 shell reports an
    /// empty name.
    fn call_fixup_name(&self, _inject_id: int4) -> String {
        String::new()
    }

    /// Build a locked override [`FuncProto`](crate::fspec::FuncProto) from the
    /// parsed [`PrototypePieces`](crate::fspec::PrototypePieces) of an `override
    /// prototype <addr> <decl>` (C++ `IfcProtooverride`'s `new FuncProto;
    /// setInternal(pieces.model, void); setPieces(pieces)`).  Used by
    /// `build_call_specs` to apply the override onto the matching call spec
    /// (`Override::applyPrototype` → `fspecs.copy(*proto)`).  The W3 shell has no
    /// type factory / model registry and reports `None`.
    fn build_override_proto(
        &self,
        _pieces: &crate::fspec::PrototypePieces,
    ) -> KunaResult<Option<crate::fspec::FuncProto>> {
        Ok(None)
    }

    /// (kuna) GH-8817: is `op` a V850 `jmp [reg]` CALLIND to reclassify as
    /// BRANCHIND? (C++ `kunaIsV850IndirectJmp`).
    ///
    /// // STUB(W4): ArchOption v850indirectbranch — gated on `option
    /// v850indirectbranch on|off`, default \b false (upstream byte-identical).
    fn is_v850_indirect_jmp(&self, _fd: &Funcdata, _op: OpId) -> bool {
        false
    }

    /// (kuna) GH-6882: is `op` a SPARC struct-return `unimp`-after-call trap
    /// BRANCHIND to drop as a fall-through no-op? (C++ `kunaIsSparcStructRetTrap`).
    /// // STUB(W4).
    fn is_sparc_struct_ret_trap(&self, _fd: &Funcdata, _op: OpId) -> bool {
        false
    }

    /// (kuna `fastfailnoreturn`) Is `op` the CALLIND half of a Windows `int 0x29`
    /// (`__fastfail`), which never returns?  See
    /// [`kuna_fastfailnoreturn`](crate::kuna_fastfailnoreturn).  The default shell
    /// reports `false` (upstream behavior: the interrupt is an ordinary modelled
    /// call, so the cspec's `extrapop` raises the stack pointer by 8 at every site).
    fn is_fastfail_callind(&self, _fd: &Funcdata, _op: OpId) -> bool {
        false
    }

    /// (kuna) tee-O2 tail-jump: is the direct `CPUI_BRANCH` `op`, whose target is
    /// `dest`, an `-O2` tail call to another known function's entry (and so should
    /// be recovered as a `CALL` + `RETURN` rather than flow-followed into the
    /// callee)?  See [`kuna_tailcalljump`](crate::kuna_tailcalljump).
    ///
    /// // STUB(W4): ArchOption tailcalljump — gated on `option tailcalljump
    /// on|off`, default \b false (default-pipeline byte-identical).
    fn is_tail_call_branch(&self, _fd: &Funcdata, _op: OpId, _dest: &Address) -> bool {
        false
    }

    /// (kuna `tailcallframe`) Same question as
    /// [`is_tail_call_branch`](FlowEnvironment::is_tail_call_branch) for a `dest`
    /// the symbol table does NOT know: does the run of instructions ending at
    /// `op` tear down exactly the frame the entry block built, so the jump is a
    /// tail call into a callee no discovery oracle reached?  See
    /// [`kuna_tailcallframe`](crate::kuna_tailcallframe).  The default shell
    /// reports `false`.
    fn is_frame_teardown_tail_call(&self, _fd: &Funcdata, _op: OpId, _dest: &Address) -> bool {
        false
    }

    /// Resolve a direct-call entry address to its callee symbol (C++
    /// `FlowInfo::queryCall` → `Scope::queryFunction(entryaddr)`,
    /// `flow.cc:674`): return the callee's display name (so the call renders
    /// `name(args)` not the address) when the symbol table knows a function at
    /// that address.  `None` for an unknown callee (the generic `func_<addr>` /
    /// `sub_<addr>` name applies at print time).
    ///
    /// The W4 `Scope::queryFunction` returns a `Funcdata *` carrying the callee's
    /// own `FuncProto`; the proto is filled in at `ActionDefaultParams` time
    /// (`queryCall`'s `copyFlowEffects` postpone), so only the name is needed
    /// here.  The default returns no name (no symbol table in the W3 shell).
    fn query_call(&self, _entry: &Address) -> Option<String> {
        None
    }

    /// Does the function at the direct-call `entry` address have its prototype
    /// marked \e noreturn? (C++ `FlowInfo::queryCall` →
    /// `fspecs.copyFlowEffects(otherfunc->getFuncProto())` →
    /// `FuncCallSpecs::isNoReturn()`).
    ///
    /// `option noreturn <name>` (`OptionNoReturn`) sets this flag on the named
    /// FunctionSymbol's prototype; `checkForFlowModification` plants an
    /// `artificialHalt(noreturn)` after the call op when it is true, so flow does
    /// not run off the end of the function into unmapped bytes.  The W3 shell has
    /// no symbol table and reports `false`.
    fn query_call_no_return(&self, _entry: &Address) -> bool {
        false
    }

    /// (kuna `funcboundflow`) Is the fall-through bound at a known function entry
    /// enabled (`option funcboundflow`, `Architecture::funcbound_flow`)?  When on,
    /// `process_instruction` truncates a fall-through that reaches the entry of
    /// another known function (via [`query_call`](FlowEnvironment::query_call))
    /// instead of decoding the next function's body into this one.  The default
    /// shell reports `false` (no bounding, upstream behavior).  See
    /// [`kuna_funcboundflow`](crate::kuna_funcboundflow).
    fn funcbound_flow_enabled(&self) -> bool {
        false
    }

    /// (kuna `overlapbranch`) Is the overlapping-branch truncation enabled
    /// (`option overlapbranch`, `Architecture::overlap_branch`)?  When on,
    /// `process_instruction` truncates a conditional branch's fall-through
    /// instruction when the branch's own target lies strictly inside that
    /// instruction's encoding (the anti-disassembly junk-byte idiom), instead of
    /// letting the fall-through decode swallow the target and desynchronise the
    /// stream.  The default shell reports `false` (upstream behavior).  See
    /// [`kuna_overlapbranch`](crate::kuna_overlapbranch).
    fn overlap_branch_enabled(&self) -> bool {
        false
    }

    /// Is the function at the direct-call `entry` address marked \e inline? (C++
    /// `FlowInfo::queryCall` → `fspecs.copyFlowEffects(otherfunc->getFuncProto())`
    /// → `FuncProto::isInline()`).
    ///
    /// `option inline <name>` (`OptionInline`) sets this flag on the named
    /// FunctionSymbol's prototype; `checkForFlowModification` pushes the CALL op
    /// onto `injectlist` when it is true, so `injectPcode` clones the callee's
    /// body into this flow at the call site.  The W3 shell reports `false`.
    fn query_call_inline(&self, _entry: &Address) -> bool {
        false
    }

    /// The \e injection id registered for the function at the direct-call `entry`
    /// address, or `-1` for none (C++ `FuncProto::getInjectId()` after
    /// `copyFlowEffects`).
    ///
    /// `IfcFixupApply` parks an inject id on the symbol; a non-negative id routes
    /// `injectPcode` through `injectSubFunction` (payload injection) rather than
    /// `inlineSubFunction` (whole-body clone).  The W3 shell reports `-1`.
    fn query_call_inject_id(&self, _entry: &Address) -> int4 {
        -1
    }

    /// Build a fresh [`Funcdata`] for the inline-callee at `entry`, ready for a
    /// nested flow follow (C++ `Funcdata::inlineFlow` constructs
    /// `FlowInfo inlineflow(*inlinefd,...)` over the queried callee `Funcdata`
    /// after `clearAnalysis(inlinefd)`).
    ///
    /// Returns `Ok(None)` when there is no callee symbol at `entry` (the C++
    /// `fc->getFuncdata() == 0` short-circuit in `inlineSubFunction`).  The W3
    /// shell has no symbol table / architecture and reports `None`.
    fn build_inline_funcdata(&self, _entry: &Address) -> KunaResult<Option<Funcdata>> {
        Ok(None)
    }
}

/// Allocate a process-unique \e fspec handle (the offset of the \e fspec
/// annotation address).  In C++ the offset is the raw `FuncCallSpecs *`, a unique
/// process pointer; here a monotonic counter plays the same role, registered in
/// the fspec-space side table ([`FuncCallSpecs::register_in_fspec_space`]).  The
/// handle is never used as a `qlst` index (call specs are looked up by op), so a
/// simple ever-increasing id is sufficient and stable across `sortCallSpecs`.
pub(crate) fn next_fspec_handle() -> kuna_base::types::uintb {
    use std::sync::atomic::{AtomicU64, Ordering};
    // Start above 0 so the handle is never confused with a null offset; the high
    // bit is left clear so the value is a valid (small) address offset.
    static NEXT: AtomicU64 = AtomicU64::new(0x10000);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// \brief A class for generating the control-flow structure for a single
/// function (C++ `class FlowInfo`, `flow.hh:58`).
///
/// The C++ holds `Funcdata &data` plus references to its internal containers
/// (`obank`/`bblocks`/`qlst`) and an owned `PcodeEmitFd emitter`.  Here the
/// single owned [`Funcdata`] *is* those containers (ADR 0001), and the emitter is
/// the local [`FlowEmit`] built per-instruction; the architecture/override slice
/// is the borrowed [`FlowEnvironment`] (`E`).
///
/// The C++ `qlst` (the discovered `FuncCallSpecs` list) is W4; here it is carried
/// as a placeholder count so the `hasInject`/call-site cadence is preserved
/// without the W4 type.
pub struct FlowInfo<'a, E: FlowEnvironment> {
    /// The function being flow-followed (C++ `Funcdata &data`; owns `obank`,
    /// `bblocks`, `qlst`).
    pub data: Funcdata,
    /// The architecture/override boundary slice (C++ `glb` + `data.getOverride()`).
    env: &'a E,
    /// Addresses which are permanently unprocessed (C++ `unprocessed`).
    unprocessed: Vec<Address>,
    /// (kuna) The subset of [`Self::unprocessed`] that flow reached by leaving the
    /// declared extent, i.e. every address [`Self::handle_out_of_bounds`] warned
    /// about.  [`Self::fillin_branch_stubs`] registers only these in
    /// [`Self::visited`], so a branch to one resolves to its stub instead of
    /// aborting the function; an unprocessed address that is IN the extent keeps
    /// upstream's throw, because there the missing op is a real defect and
    /// clipping it would silently truncate a body.
    outofbounds: std::collections::BTreeSet<Address>,
    /// Addresses to which there is flow — the work stack (C++ `addrlist`).
    addrlist: Vec<Address>,
    /// List of BRANCHIND ops (preparing for jump table recovery) (C++ `tablelist`).
    tablelist: Vec<OpId>,
    /// List of p-code ops that need injection (C++ `injectlist`).
    injectlist: Vec<OpId>,
    /// Map of machine instructions visited so far (C++ `visited`).
    visited: BTreeMap<Address, VisitStat>,
    /// Source p-code op of basic-block edges (C++ `block_edge1`).
    block_edge1: Vec<OpId>,
    /// Destination p-code op of basic-block edges (C++ `block_edge2`).
    block_edge2: Vec<OpId>,
    /// Number of instructions flowed through (C++ `insn_count`).
    insn_count: uint4,
    /// Maximum number of instructions (C++ `insn_max`).
    insn_max: uint4,
    /// Start of range in which we are allowed to flow (C++ `baddr`).
    baddr: Address,
    /// End of range in which we are allowed to flow (C++ `eaddr`).
    eaddr: Address,
    /// Start of actual function range (C++ `minaddr`).
    minaddr: Address,
    /// End of actual function range (C++ `maxaddr`).
    maxaddr: Address,
    /// Does the function have registered flow override instructions
    /// (C++ `flowoverride_present`).
    flowoverride_present: bool,
    /// Boolean options for flow following (C++ `flags`).
    flags: uint4,
    /// Number of discovered call sites (C++ `qlst.size()`).
    ///
    /// STUB(W4): the `FuncCallSpecs` objects themselves are W4; the count tracks
    /// the call-site cadence so the injection/`paramshift` bookkeeping order is
    /// preserved.
    qlst_count: usize,
    /// Entry address of the first function in the in-lining chain (C++
    /// `Funcdata *inline_head`, `flow.hh:102`).
    ///
    /// The C++ stores the head `Funcdata *`; here only its entry address is needed
    /// (the head is reached solely to emit `warning`/`warningHeader`, which the
    /// merged tree buffers on the per-function `Funcdata`).  `None` until the top
    /// level of inlining sets it.
    inline_head: Option<Address>,
    /// Active set of entry addresses for functions currently being in-lined (C++
    /// `set<Address> *inline_recursion`, pointing at `inline_base`; `flow.hh:103`).
    ///
    /// The C++ uses a pointer into `inline_base` (or a parent flow's set, copied
    /// by `forwardRecursion`); the merged tree owns the set inline.  An address in
    /// the set means that function is on the current in-lining stack — a CALL to it
    /// cannot be inlined again (the cycle break that prints "Could not inline
    /// here").
    inline_recursion: std::collections::BTreeSet<Address>,
    /// (kuna `overlapbranch`) The `(fall-through address, branch target)` pair
    /// recorded when the instruction just processed was a conditional branch with
    /// a forward target.  Consumed by the very next
    /// [`process_instruction`](FlowInfo::process_instruction), which truncates the
    /// fall-through when the branch target turns out to lie strictly inside it.
    /// See [`kuna_overlapbranch`](crate::kuna_overlapbranch).
    overlap_watch: Option<(Address, Address)>,
    /// (kuna `overlapbranch`) Direct target of the CBRANCH seen while
    /// xref-classifying the current instruction, staged for `overlap_watch`.
    overlap_cbranch_target: Option<Address>,
    /// Entry address of the call-fixup whose body is currently being woven in
    /// (C++ `setupCallSpecs(op, fc)`'s non-NULL `fc` argument — the call site
    /// being injected).  While a [`do_injection`](Self::do_injection) for a
    /// call-fixup is xref-classifying its emitted ops, a nested CALL/CALLIND to
    /// the SAME entry must `cancel_inject_id` so the injection does not recurse.
    /// `None` outside a call-fixup injection (the user-op path passes NULL `fc`).
    injecting_entry: Option<Address>,
}

impl<'a, E: FlowEnvironment> FlowInfo<'a, E> {
    /// Prepare for tracing flow for a new function (C++ `FlowInfo::FlowInfo`,
    /// `flow.cc:28`).
    ///
    /// The C++ initializes `baddr`/`eaddr` to the full range of the entry-point
    /// space, `minaddr`/`maxaddr` to the entry address, `insn_max` to `~0`, and
    /// reads `flowoverride_present` from the override.
    pub fn new(data: Funcdata, env: &'a E) -> FlowInfo<'a, E> {
        let entry = data.get_address().clone();
        let space = entry.get_space().expect("FlowInfo: entry has no space").clone();
        let baddr = Address::new(space.clone(), 0);
        let eaddr = Address::new(space, !0u64);
        // The per-function Override lives on the Funcdata (`localoverride`); the
        // env-routed `has_flow_override` is the W2-era boundary default (always false)
        // and is OR'd in only for back-compat with envs that still carry it.
        let flowoverride_present = data.get_override().has_flow_override() || env.has_flow_override();
        FlowInfo {
            data,
            env,
            unprocessed: Vec::new(),
            outofbounds: std::collections::BTreeSet::new(),
            addrlist: Vec::new(),
            tablelist: Vec::new(),
            injectlist: Vec::new(),
            visited: BTreeMap::new(),
            block_edge1: Vec::new(),
            block_edge2: Vec::new(),
            insn_count: 0,
            insn_max: !0u32,
            baddr,
            eaddr,
            minaddr: entry.clone(),
            maxaddr: entry,
            flowoverride_present,
            flags: 0,
            qlst_count: 0,
            inline_head: None,
            inline_recursion: std::collections::BTreeSet::new(),
            overlap_watch: None,
            overlap_cbranch_target: None,
            injecting_entry: None,
        }
    }

    /// Pull in-lining recursion information from a parent flow (C++
    /// `FlowInfo::forwardRecursion`, `flow.cc:1045`): when preparing p-code for an
    /// in-lined function, the nested generation needs to know which functions are
    /// already on the in-lining stack so it does not re-inline a cycle.
    fn forward_recursion(&mut self, op2: &FlowInfo<'a, E>) {
        self.inline_recursion = op2.inline_recursion.clone();
        self.inline_head = op2.inline_head.clone();
    }

    // -----------------------------------------------------------------------
    // option setters (C++ inline)
    // -----------------------------------------------------------------------

    /// Construct a FlowInfo over a partial-clone Funcdata, seeding the cloned
    /// flow's `visited` map (C++ `FlowInfo::FlowInfo(...,const FlowInfo *op2)`,
    /// `flow.cc:54`): the partial reuses the source flow's `visited` so its
    /// `generateBlocks` (fillinBranchStubs / fallthru-edge resolution) sees the
    /// same instruction boundaries.
    #[allow(clippy::mutable_key_type)]
    pub fn new_seeded(
        data: Funcdata,
        env: &'a E,
        visited: VisitedMap,
    ) -> FlowInfo<'a, E> {
        let mut flow = FlowInfo::new(data, env);
        flow.visited = visited;
        flow
    }

    /// Establish the flow bounds (C++ `setRange`, `flow.hh:144`).
    pub fn set_range(&mut self, b: Address, e: Address) {
        self.baddr = b;
        self.eaddr = e;
    }
    /// Set the maximum number of instructions (C++ `setMaximumInstructions`).
    pub fn set_maximum_instructions(&mut self, max: uint4) {
        self.insn_max = max;
    }
    /// Enable a specific option (C++ `setFlags`).
    pub fn set_flags(&mut self, val: uint4) {
        self.flags |= val;
    }
    /// Disable a specific option (C++ `clearFlags`).
    pub fn clear_flags(&mut self, val: uint4) {
        self.flags &= !val;
    }

    // -----------------------------------------------------------------------
    // flag queries (C++ inline)
    // -----------------------------------------------------------------------

    /// Are there possible unreachable ops (C++ `hasPossibleUnreachable`).
    fn has_possible_unreachable(&self) -> bool {
        (self.flags & flow_flags::possible_unreachable) != 0
    }
    /// Mark that there may be unreachable ops (C++ `setPossibleUnreachable`).
    ///
    /// STUB(W4): the sole W3-portable caller is [`inline_sub_function`](Self::inline_sub_function),
    /// itself a W4 skeleton (no W3 caller yet); kept as the faithful private helper
    /// the W4 inline wave drives.
    #[allow(dead_code)]
    fn set_possible_unreachable(&mut self) {
        self.flags |= flow_flags::possible_unreachable;
    }
    /// Has the given instruction (address) been seen in flow (C++ `seenInstruction`).
    fn seen_instruction(&self, addr: &Address) -> bool {
        self.visited.contains_key(addr)
    }
    /// Does \b this flow have injections (C++ `hasInject`).
    pub fn has_inject(&self) -> bool {
        !self.injectlist.is_empty()
    }
    /// Does \b this flow have unimplemented instructions (C++ `hasUnimplemented`).
    pub fn has_unimplemented(&self) -> bool {
        (self.flags & flow_flags::unimplemented_present) != 0
    }
    /// Does \b this flow reach inaccessible data (C++ `hasBadData`).
    pub fn has_bad_data(&self) -> bool {
        (self.flags & flow_flags::baddata_present) != 0
    }
    /// Does \b this flow out of bound (C++ `hasOutOfBounds`).
    pub fn has_out_of_bounds(&self) -> bool {
        (self.flags & flow_flags::outofbounds_present) != 0
    }
    /// Does \b this flow reinterpret bytes (C++ `hasReinterpreted`).
    pub fn has_reinterpreted(&self) -> bool {
        (self.flags & flow_flags::reinterpreted_present) != 0
    }
    /// Does \b this flow have too many instructions (C++ `hasTooManyInstructions`).
    pub fn has_too_many_instructions(&self) -> bool {
        (self.flags & flow_flags::toomanyinstructions_present) != 0
    }
    /// Is \b this flow to be in-lined (C++ `isFlowForInline`).
    pub fn is_flow_for_inline(&self) -> bool {
        (self.flags & flow_flags::flow_forinline) != 0
    }
    /// Should jump table structure be recorded (C++ `doesJumpRecord`).
    pub fn does_jump_record(&self) -> bool {
        (self.flags & flow_flags::record_jumploads) != 0
    }
    /// Get the number of bytes covered by the flow (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        // (int4)(maxaddr.getOffset() - minaddr.getOffset())
        self.maxaddr.get_offset().wrapping_sub(self.minaddr.get_offset()) as int4
    }

    /// Clear any discovered flow properties (C++ `clearProperties`, `flow.cc:80`).
    fn clear_properties(&mut self) {
        self.flags &= !(flow_flags::unimplemented_present
            | flow_flags::baddata_present
            | flow_flags::outofbounds_present);
        self.insn_count = 0;
    }

    // -----------------------------------------------------------------------
    // target / branch-target lookups (pure W3-IR; THE flow walk)
    // -----------------------------------------------------------------------

    /// Find the fall-thru p-code op for the given op (C++ `fallthruOp`,
    /// `flow.cc:90`).
    ///
    /// For efficiency this assumes `op` can actually fall-thru.  Returns `None`
    /// if there is no possible p-code op.
    fn fallthru_op(&self, op: OpId) -> Option<OpId> {
        if let Some(retop) = self.dead_next(op) {
            // same instruction
            if !self.data.obank().get(retop).expect("fallthru_op: stale op").is_instruction_start() {
                return Some(retop);
            }
        }
        // Find address of instruction containing this op.
        let opaddr = self.data.obank().get(op).expect("fallthru_op: stale op").get_addr().clone();
        let (first, size) = self.visited_last_le_owned(&opaddr)?;
        let endaddr = &first + size as i64;
        if endaddr <= opaddr {
            return None;
        }
        self.target(&endaddr).ok()
    }

    /// Return the first p-code op for the instruction at the given address (C++
    /// `target`, `flow.cc:117`).
    ///
    /// If the instruction generated no p-code, fall-thru to the next instruction.
    /// If no op is ultimately found, returns an error (the C++ `LowlevelError`).
    pub fn target(&self, addr: &Address) -> KunaResult<OpId> {
        let mut cur = addr.clone();
        while let Some(stat) = self.visited.get(&cur) {
            if !stat.seqnum.get_addr().is_invalid() {
                if let Some(retop) = self.data.obank().find_op(&stat.seqnum) {
                    return Ok(retop);
                }
                break;
            }
            // Visit fall thru address in case of no-op:
            cur = &cur + stat.size as i64;
        }
        Err(KunaError::lowlevel(Self::target_error_message(addr)))
    }

    /// Snapshot the `visited` map for a post-FlowInfo `target` lookup
    /// (`switchOverJumpTables` runs after the FlowInfo's `data` is moved out).
    #[allow(clippy::mutable_key_type)]
    pub fn target_index_snapshot(&self) -> VisitedMap {
        self.visited.clone()
    }

    fn target_error_message(addr: &Address) -> String {
        let mut msg = String::from("Could not find op at target address: (");
        if let Some(sp) = addr.get_space() {
            msg.push_str(sp.get_name());
        }
        msg.push(',');
        let mut raw = String::new();
        let _ = addr.print_raw(&mut raw);
        msg.push_str(&raw);
        msg.push(')');
        msg
    }

    /// Generate the target PcodeOp for a relative branch (C++ `findRelTarget`,
    /// `flow.cc:151`).
    ///
    /// Returns `Ok(Some(op))` for a properly-internal branch, `Ok(None)` with the
    /// fall-thru address passed back in `res` when the relative branch is really to
    /// the next instruction, and `Err` otherwise (C++ `LowlevelError`).
    fn find_rel_target(&self, op: OpId, res: &mut Address) -> KunaResult<Option<OpId>> {
        let o = self.data.obank().get(op).expect("find_rel_target: stale op");
        let in0 = o.get_in(0).expect("findRelTarget: BRANCH null in0 (C++ UB)");
        let addr =
            self.data.vbank().get(in0).expect("find_rel_target: stale vn").get_addr().clone();
        let id: uintm = o.get_time().wrapping_add(addr.get_offset() as uintm);
        let opaddr = o.get_addr().clone();
        let seqnum = SeqNum::new(opaddr.clone(), id);
        // internal branch
        if let Some(retop) = self.data.obank().find_op(&seqnum) {
            return Ok(Some(retop));
        }
        let seqnum1 = SeqNum::new(opaddr.clone(), id.wrapping_sub(1));
        if let Some(retop) = self.data.obank().find_op(&seqnum1) {
            // branch was indeed to next instruction
            let retaddr = self
                .data
                .obank()
                .get(retop)
                .expect("find_rel_target: stale retop")
                .get_addr()
                .clone();
            if let Some((first, size)) = self.visited_last_le_owned(&retaddr) {
                *res = &first + size as i64;
                // res has the fallthru addr
                if &opaddr < res {
                    return Ok(None);
                }
            }
        }
        let mut msg = String::from("Bad relative branch at instruction : (");
        if let Some(sp) = opaddr.get_space() {
            msg.push_str(sp.get_name());
        }
        msg.push(',');
        let mut raw = String::new();
        let _ = opaddr.print_raw(&mut raw);
        msg.push_str(&raw);
        msg.push(')');
        Err(KunaError::lowlevel(msg))
    }

    /// Return the p-code op a BRANCH/CBRANCH refers to (C++ `branchTarget`,
    /// `flow.cc:189`).
    pub fn branch_target(&self, op: OpId) -> KunaResult<OpId> {
        let in0 = self
            .data
            .obank()
            .get(op)
            .expect("branch_target: stale op")
            .get_in(0)
            .expect("branchTarget: null in0 (C++ UB)");
        let addr =
            self.data.vbank().get(in0).expect("branch_target: stale vn").get_addr().clone();
        if addr.is_constant() {
            // relative sequence number.  `Address res;` — default/invalid; the C++
            // only reads it back on the `Ok(None)` path, which always writes it
            // first (inside the `op->getAddr() < res` guard).
            let mut res = Address::default();
            if let Some(retop) = self.find_rel_target(op, &mut res)? {
                return Ok(retop);
            }
            return self.target(&res);
        }
        self.target(&addr) // normal address target
    }

    /// Replace any reference to the op being inlined with the first op of the
    /// inlined sequence (C++ `updateTarget`, `flow.cc:206`).
    ///
    /// Public in C++ (`flow.hh:150`): used by `doInjection` (W4) to repoint the
    /// target map after an inlined op replaces a call.
    pub fn update_target(&mut self, old_op: OpId, new_op: OpId) {
        let oldaddr =
            self.data.obank().get(old_op).expect("update_target: stale old").get_addr().clone();
        let oldseq =
            self.data.obank().get(old_op).expect("update_target").get_seq_num().clone();
        let newseq =
            self.data.obank().get(new_op).expect("update_target: stale new").get_seq_num().clone();
        if let Some(stat) = self.visited.get_mut(&oldaddr) {
            if stat.seqnum == oldseq {
                stat.seqnum = newseq;
            }
        }
    }

    /// Register a new (non fall-thru) flow target (C++ `newAddress`, `flow.cc:221`).
    fn new_address(&mut self, from: OpId, to: &Address) -> KunaResult<()> {
        if to < &self.baddr || &self.eaddr < to {
            let fromaddr =
                self.data.obank().get(from).expect("new_address: stale from").get_addr().clone();
            self.handle_out_of_bounds(&fromaddr, to)?;
            self.unprocessed.push(to.clone());
            self.outofbounds.insert(to.clone());
            return Ok(());
        }
        if self.seen_instruction(to) {
            let op = self.target(to)?;
            self.op_mark_start_basic(op);
            return Ok(());
        }
        self.addrlist.push(to.clone());
        Ok(())
    }

    // -----------------------------------------------------------------------
    // op flag marks (C++ Funcdata::opMarkStart* — trivial setFlag, ported here)
    // -----------------------------------------------------------------------

    /// Mark the given op as the start of a basic block (C++
    /// `Funcdata::opMarkStartBasic` → `op->setFlag(PcodeOp::startbasic)`).
    fn op_mark_start_basic(&mut self, op: OpId) {
        self.data
            .obank_mut()
            .get_mut(op)
            .expect("op_mark_start_basic: stale op")
            .set_flag(pcodeop_flags::startbasic);
    }

    /// Mark the given op as the first op of its instruction (C++
    /// `Funcdata::opMarkStartInstruction` → `op->setFlag(PcodeOp::startmark)`).
    fn op_mark_start_instruction(&mut self, op: OpId) {
        self.data
            .obank_mut()
            .get_mut(op)
            .expect("op_mark_start_instruction: stale op")
            .set_flag(pcodeop_flags::startmark);
    }

    // -----------------------------------------------------------------------
    // visited-map helpers (the std::map<Address,VisitStat> idioms)
    // -----------------------------------------------------------------------

    /// `visited.upper_bound(addr); if (!=begin) --iter; return *iter` — the last
    /// visited entry with key `<= addr`.  Returns an owned `(key, size)` to avoid
    /// holding a borrow across the surrounding `&self` reads.
    fn visited_last_le_owned(&self, addr: &Address) -> Option<(Address, int4)> {
        self.visited.range(..=addr.clone()).next_back().map(|(k, v)| (k.clone(), v.size))
    }

    /// The successor of `op` in the global \e dead list (C++ `++op->getInsertIter()`
    /// while the op is dead), or `None` at the end.
    fn dead_next(&self, op: OpId) -> Option<OpId> {
        self.data.obank().dead_next(op)
    }

    // -----------------------------------------------------------------------
    // out-of-bounds / reinterpreted reporting
    // -----------------------------------------------------------------------

    /// Generate warning or throw for out-of-bounds flow (C++ `handleOutOfBounds`,
    /// `flow.cc:537`).
    ///
    /// STUB(W4): the `data.warning`/`warningHeader` reporting is the W4 comment
    /// store; the W3 shell sets the discovery flag and returns the error in the
    /// `error_outofbounds` case (faithful), suppressing only the warning text.
    fn handle_out_of_bounds(&mut self, fromaddr: &Address, toaddr: &Address) -> KunaResult<()> {
        if (self.flags & flow_flags::ignore_outofbounds) == 0 {
            let msg = Self::out_of_bounds_message(fromaddr, toaddr);
            if (self.flags & flow_flags::error_outofbounds) == 0 {
                // (kuna) The C++ `data.warning`/`data.warningHeader` calls, no
                // longer stubbed: the range is the whole entry-point space unless
                // a caller declares a function extent, so this fires only under a
                // declared boundary — and a declared end that cuts real flow must
                // say so rather than silently truncating the body.
                self.data.warning(&msg, toaddr);
                if !self.has_out_of_bounds() {
                    self.flags |= flow_flags::outofbounds_present;
                    self.data.warning_header("Function flows out of bounds");
                }
            } else {
                return Err(KunaError::lowlevel(msg));
            }
        }
        Ok(())
    }

    /// Build the C++ out-of-bounds error/warning string (`flow.cc:541-547`).
    fn out_of_bounds_message(fromaddr: &Address, toaddr: &Address) -> String {
        let mut msg = String::from("Function flow out of bounds: ");
        msg.push(fromaddr.get_shortcut());
        let mut raw = String::new();
        let _ = fromaddr.print_raw(&mut raw);
        msg.push_str(&raw);
        msg.push_str(" flows to ");
        msg.push(toaddr.get_shortcut());
        let mut raw2 = String::new();
        let _ = toaddr.print_raw(&mut raw2);
        msg.push_str(&raw2);
        msg
    }

    /// Generate warning/exception for a \e reinterpreted address (C++
    /// `reinterpreted`, `flow.cc:624`).
    ///
    /// STUB(W4): warning store; the `error_reinterpreted` exception is faithful.
    fn reinterpreted(&mut self, addr: &Address) -> KunaResult<()> {
        let addr2 = match self.visited_last_le_owned(addr) {
            Some((k, _)) => k,
            None => return Ok(()), // Should never happen
        };
        let mut s = String::from("Instruction at (");
        if let Some(sp) = addr.get_space() {
            s.push_str(sp.get_name());
        }
        s.push(',');
        let mut raw = String::new();
        let _ = addr.print_raw(&mut raw);
        s.push_str(&raw);
        s.push_str(") overlaps instruction at (");
        if let Some(sp) = addr2.get_space() {
            s.push_str(sp.get_name());
        }
        s.push(',');
        let mut raw2 = String::new();
        let _ = addr2.print_raw(&mut raw2);
        s.push_str(&raw2);
        s.push(')');
        s.push('\n');
        if (self.flags & flow_flags::error_reinterpreted) != 0 {
            return Err(KunaError::lowlevel(s));
        }
        if (self.flags & flow_flags::reinterpreted_present) == 0 {
            self.flags |= flow_flags::reinterpreted_present;
            // data.warningHeader(s);  -- STUB(W4)
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // artificial halt
    // -----------------------------------------------------------------------

    /// Create an artificial halt p-code op (C++ `artificialHalt`, `flow.cc:610`).
    ///
    /// A special form of RETURN op annotated with the desired \e type.
    fn artificial_halt(&mut self, addr: &Address, flag: uint4) -> KunaResult<OpId> {
        let haltop = self.data.new_op(1, addr.clone());
        let retop = self.env.resolve_typeop(OpCode::CPUI_RETURN);
        self.data.op_set_opcode(haltop, retop);
        let c = self.data.new_constant(4, 1);
        self.data.op_set_input(haltop, c, 0)?;
        if flag != 0 {
            self.data.op_mark_halt(haltop, flag)?;
        }
        Ok(haltop)
    }

    // -----------------------------------------------------------------------
    // per-instruction processing (THE op-creation order)
    // -----------------------------------------------------------------------

    /// Delete any remaining ops at the end of the instruction (because they have
    /// been predetermined to be dead) (C++ `deleteRemainingOps`, `flow.cc:242`).
    ///
    /// The C++ walks the raw dead list from `oiter` to the end, `opDestroyRaw`-ing
    /// each.  `start` here is the first op (in dead-list order) to delete; the
    /// dead-list slice from `start` to the end is the C++ `[oiter, endDead())`.
    ///
    /// STUB(W3-funcdata): `opDestroyRaw` needs `destroyVarnode` (funcdata_varnode);
    /// the `funcdata_op` wave's `op_destroy_raw` returns the boundary error.  The set
    /// of ops to delete is computed faithfully; the destruction is deferred and
    /// recorded as a loss.  Returns the list of ops the C++ would have destroyed.
    fn delete_remaining_ops(&mut self, start: Option<OpId>) -> KunaResult<()> {
        let start = match start {
            Some(s) => s,
            None => return Ok(()), // oiter == endDead(): nothing to delete
        };
        // Collect [start .. endDead()) by walking the dead list forward from
        // `start`.  A `start` that is no longer on the list deletes nothing (the
        // bank's dead-list navigation reports non-membership as `None`).
        let mut victims: Vec<OpId> = Vec::new();
        let mut cur =
            if self.data.obank().on_dead_list(start) { Some(start) } else { None };
        while let Some(op) = cur {
            victims.push(op);
            cur = self.data.obank().dead_next(op);
        }
        for op in victims {
            //   -- STUB(W3-funcdata): op_destroy_raw needs destroyVarnode; the
            //      funcdata_op wave defers it.  Propagate the boundary error so the
            //      missing surface is explicit (faithful: the C++ destroys here).
            self.data.op_destroy_raw(op)?;
        }
        Ok(())
    }

    /// Analyze control-flow within p-code for a single instruction (C++
    /// `xrefControlFlow`, `flow.cc:266`).
    ///
    /// Walks the raw p-code from `oiter` (the first op of the instruction, given by
    /// `start`) to the end of the dead list, looking for control-flow ops and
    /// adding annotations (startbasic, new addresses, table/inject lists).
    /// `startbasic` persists across calls; `isfallthru` is passed back.  Returns
    /// the last processed op (or `None` if there were no ops).
    ///
    /// The op-creation/classification order here is **observable** (it drives the
    /// SeqNum allocation and the basic-block start marks); transcribed exactly.
    ///
    /// STUB(W4): `setupCallSpecs`/`setupCallindSpecs` (the CALL/CALLIND cases) need
    /// `FuncCallSpecs` (W4); those branches call the W4 skeletons, which return the
    /// boundary note.  The CALLOTHER-injected check uses the [`FlowEnvironment`]
    /// predicate.  The `fc` injection-context parameter is the W4 `FuncCallSpecs *`.
    fn xref_control_flow(
        &mut self,
        start: Option<OpId>,
        startbasic: &mut bool,
        isfallthru: &mut bool,
    ) -> KunaResult<Option<OpId>> {
        let mut op: Option<OpId> = None;
        *isfallthru = false;
        let mut maxtime: uintm = 0; // Deepest internal relative branch
        // Build the op walk: the dead-list slice [start, endDead()).  The C++
        // mutates the list (deleteRemainingOps splices the tail) while iterating;
        // we re-derive the live cursor by tracking which ops are still dead.
        let mut cursor = start;
        while let Some(curop) = cursor {
            op = Some(curop);
            // Advance the cursor before the body may delete the tail.
            cursor = self.dead_next(curop);
            if *startbasic {
                self.op_mark_start_basic(curop);
                *startbasic = false;
            }
            // (kuna) GH-8817: reclassify V850 jmp [reg] CALLIND -> BRANCHIND.
            if self.env.is_v850_indirect_jmp(&self.data, curop) {
                let branchind = self.env.resolve_typeop(OpCode::CPUI_BRANCHIND);
                self.data.op_set_opcode(curop, branchind);
            }
            match self.data.obank().get(curop).expect("xref_control_flow: stale op").code() {
                OpCode::CPUI_CBRANCH => {
                    let destaddr = self.branch_in0_addr(curop);
                    if destaddr.is_constant() {
                        let mut fall_thru = Address::default(); // unused in CBRANCH/BRANCH
                        if let Some(destop) = self.find_rel_target(curop, &mut fall_thru)? {
                            self.op_mark_start_basic(destop); // target starts a block
                            let newtime =
                                self.data.obank().get(destop).expect("xref: stale destop").get_time();
                            if newtime > maxtime {
                                maxtime = newtime;
                            }
                        } else {
                            *isfallthru = true; // relative branch is to end of instruction
                        }
                    } else {
                        // (kuna overlapbranch) stage the machine-level conditional
                        // target for the fall-through overlap check.
                        self.overlap_cbranch_target = Some(destaddr.clone());
                        self.new_address(curop, &destaddr)?; // generate branch address
                    }
                    *startbasic = true;
                }
                OpCode::CPUI_BRANCH => {
                    let destaddr = self.branch_in0_addr(curop);
                    if destaddr.is_constant() {
                        let mut fall_thru = Address::default(); // unused in CBRANCH/BRANCH
                        if let Some(destop) = self.find_rel_target(curop, &mut fall_thru)? {
                            self.op_mark_start_basic(destop);
                            let newtime =
                                self.data.obank().get(destop).expect("xref: stale destop").get_time();
                            if newtime > maxtime {
                                maxtime = newtime;
                            }
                        } else {
                            *isfallthru = true;
                        }
                        if self.data.obank().get(curop).expect("xref").get_time() >= maxtime {
                            self.delete_remaining_ops(cursor)?;
                            cursor = None;
                        }
                        *startbasic = true;
                    } else if let Some(tailkind) = self.tail_call_kind(curop, &destaddr) {
                        // (kuna) tee-O2 tail-jump: a direct `jmp` to another known
                        // function's entry is a tail call.  Rewrite BRANCH -> CALL
                        // (so the callee resolves by name and its return value
                        // flows out) + an artificial RETURN, instead of
                        // new_address()-following the jump INTO the callee (which
                        // would inline a PLT thunk and mis-render it as a
                        // `(*dat_...)(...)` indirect call).  The rewrite + halt-
                        // insert + cursor re-derive mirror the CPUI_CALL arm and
                        // `truncate_indirect_jump` / `setup_callind_specs`.
                        self.data.op_set_opcode_code(curop, OpCode::CPUI_CALL);
                        // (kuna) logging contract: this option-gated pass is
                        // output-changing — it introduces a *new* CALL where the
                        // jump used to flow into the callee.  Emit an observable
                        // warning at the branch site so the recovered tail call is
                        // attributable in the output (`WARNING:` comment) and in the
                        // restart/warning log.
                        let site = self
                            .data
                            .obank()
                            .get(curop)
                            .expect("tail-call: stale call op")
                            .get_addr()
                            .clone();
                        let mut destbuf = String::new();
                        let _ = destaddr.print_raw(&mut destbuf);
                        self.data.warning(
                            &format!(
                                "{tailkind}: recovered tail call -> introduced call to {destbuf}"
                            ),
                            &site,
                        );
                        let no_return = self.setup_call_specs(curop)?;
                        if !no_return {
                            // Terminate flow with a normal return after the tail
                            // call (a noreturn callee already had its halt planted
                            // by check_for_flow_modification).
                            let addr = self
                                .data
                                .obank()
                                .get(curop)
                                .expect("tail-call: stale call op")
                                .get_addr()
                                .clone();
                            let truncop = self.artificial_halt(&addr, 0)?;
                            self.data.op_dead_insert_after(truncop, curop);
                        }
                        // Re-derive the successor AFTER the halt insertion so the
                        // next iteration picks up the planted RETURN (the CPUI_CALL
                        // arm's `cursor = dead_next(curop)` idiom).
                        cursor = self.dead_next(curop);
                        *startbasic = true;
                    } else {
                        self.new_address(curop, &destaddr)?;
                        if self.data.obank().get(curop).expect("xref").get_time() >= maxtime {
                            self.delete_remaining_ops(cursor)?;
                            cursor = None;
                        }
                        *startbasic = true;
                    }
                }
                OpCode::CPUI_BRANCHIND => {
                    // (kuna) GH-6882: SPARC struct-return `unimp` after a call.
                    if self.env.is_sparc_struct_ret_trap(&self.data, curop) {
                        //   -- STUB(W3-funcdata): op_destroy_raw deferred (loss).
                        self.data.op_destroy_raw(curop)?;
                        op = None;
                        *isfallthru = true;
                        break;
                    }
                    self.tablelist.push(curop); // Put off trying to recover the table
                    if self.data.obank().get(curop).expect("xref").get_time() >= maxtime {
                        self.delete_remaining_ops(cursor)?;
                        cursor = None;
                    }
                    *startbasic = true;
                }
                OpCode::CPUI_RETURN => {
                    if self.data.obank().get(curop).expect("xref").get_time() >= maxtime {
                        self.delete_remaining_ops(cursor)?;
                        cursor = None;
                    }
                    *startbasic = true;
                }
                OpCode::CPUI_CALL => {
                    if self.setup_call_specs(curop)? {
                        // The C++ `--oiter` backs up to the op *after* the call —
                        // the noreturn halt `checkForFlowModification` just inserted
                        // via `opDeadInsertAfter` — so the next iteration picks up
                        // the halt (NOT re-reads the call).  Re-derive the successor
                        // of `curop` AFTER the insertion.
                        cursor = self.dead_next(curop);
                    }
                }
                OpCode::CPUI_CALLIND => {
                    if self.setup_callind_specs(curop)? {
                        cursor = self.dead_next(curop);
                    }
                }
                OpCode::CPUI_CALLOTHER => {
                    let in0 = self
                        .data
                        .obank()
                        .get(curop)
                        .expect("xref")
                        .get_in(0)
                        .expect("CALLOTHER null in0 (C++ UB)");
                    let index = self.data.vbank().get(in0).expect("xref: stale vn").get_offset();
                    if self.env.is_injected_userop(index) {
                        self.injectlist.push(curop);
                    }
                }
                _ => {}
            }
        }
        if *isfallthru {
            // Explicit relative branch to end of instruction: next starts a block.
            *startbasic = true;
        } else {
            // Calculate fallthru by looking at the last op.
            match op {
                None => *isfallthru = true, // No ops at all means a fallthru
                Some(lastop) => {
                    match self.data.obank().get(lastop).expect("xref: stale lastop").code() {
                        OpCode::CPUI_BRANCH
                        | OpCode::CPUI_BRANCHIND
                        | OpCode::CPUI_RETURN => {} // branch: no fallthru
                        _ => *isfallthru = true,    // otherwise a fallthru
                    }
                }
            }
        }
        Ok(op)
    }

    /// The Address held by a branch op's first input (its code reference).
    fn branch_in0_addr(&self, op: OpId) -> Address {
        let in0 = self
            .data
            .obank()
            .get(op)
            .expect("branch_in0_addr: stale op")
            .get_in(0)
            .expect("branch in0 is null (C++ UB)");
        self.data.vbank().get(in0).expect("branch_in0_addr: stale vn").get_addr().clone()
    }

    /// (kuna) Which tail-jump rule, if either, claims this direct `CPUI_BRANCH`?
    ///
    /// [`kuna_tailcalljump`](crate::kuna_tailcalljump) is asked first, so a
    /// branch to a known function entry keeps the existing decision and the
    /// existing `tailcalljump:` warning text; [`kuna_tailcallframe`](
    /// crate::kuna_tailcallframe) then gets the targets the symbol table does not
    /// know.  The returned name is the option that fired, so the introduced call
    /// is attributable to the rule that introduced it.
    fn tail_call_kind(&self, op: OpId, dest: &Address) -> Option<&'static str> {
        if self.env.is_tail_call_branch(&self.data, op, dest) {
            return Some("tailcalljump");
        }
        if self.env.is_frame_teardown_tail_call(&self.data, op, dest) {
            return Some("tailcallframe");
        }
        None
    }

    /// Generate p-code for a single machine instruction and process discovered
    /// flow information (C++ `processInstruction`, `flow.cc:401`).
    ///
    /// Returns `true` if the processed instruction has a fall-thru flow.
    ///
    /// STUB(W4)/STUB(W3-funcdata): the actual op-building emitter
    /// ([`FlowEmit`]) defers `newVarnodeOut`/`newCodeRef`; when a decoded
    /// instruction needs either, this method surfaces the precise missing-surface
    /// error (the op shells are still created).  The `oneInstruction` decode, the
    /// `visited`/`minaddr`/`maxaddr` bookkeeping, the `opMarkStartInstruction`
    /// mark, and the `xrefControlFlow` classification are faithful.  The decode
    /// error policies (unimplemented/baddata/too-many) are transcribed.
    pub fn process_instruction(
        &mut self,
        curaddr: &Address,
        startbasic: &mut bool,
    ) -> KunaResult<bool> {
        let mut isfallthru = true;
        let mut step: int4;
        // (kuna overlapbranch) the pair staged by the previous instruction; valid
        // for exactly this call and cleared whether or not it fires.
        let overlap_watch = self.overlap_watch.take();

        if self.insn_count >= self.insn_max {
            if (self.flags & flow_flags::error_toomanyinstructions) != 0 {
                return Err(KunaError::lowlevel("Flow exceeded maximum allowable instructions"));
            }
            // (kuna) Truncate and STOP.  The C++ plants the halt and then decodes
            // `curaddr` anyway, so the next address is queued and the next call
            // truncates again: the "truncation" still walks every reachable
            // instruction, which on an obfuscated function of millions of them is
            // an out-of-memory abort rather than a truncated body.  Halting here
            // caps the decode at `insn_max` instructions plus one halt per address
            // still queued.  The halt is registered in `visited` as the
            // instruction at `curaddr` so a branch arriving here later resolves to
            // it (`target`) instead of raising "Could not find op at target
            // address", and starts a basic block so an incoming edge has one to
            // land on (the `funcboundflow` truncation learned the same lesson).
            let halt = self.artificial_halt(curaddr, pcodeop_flags::badinstruction)?;
            self.op_mark_start_basic(halt);
            self.op_mark_start_instruction(halt);
            let seq =
                self.data.obank().get(halt).expect("truncate: stale halt").get_seq_num().clone();
            self.visited.insert(curaddr.clone(), VisitStat { seqnum: seq, size: 1 });
            // Once per function, not once per cut: every address still queued
            // truncates here too, and on a function this size that is thousands of
            // them.  The header names the knob, because an agent reading a
            // truncated body has to be able to ask for more of it without going
            // looking for the option.
            if !self.has_too_many_instructions() {
                self.flags |= flow_flags::toomanyinstructions_present;
                self.data.warning_header(&format!(
                    "Exceeded the {} instruction budget: some flow is truncated \
(raise it with `--option maxinstruction N`, or make the overrun fatal again with \
`--option errortoomanyinstructions on`)",
                    self.insn_max
                ));
                self.data.warning("Too many instructions -- truncating flow here", curaddr);
            }
            return Ok(false);
        }
        self.insn_count += 1;

        // Track whether the dead list was empty, and the marker op before decode
        // (the C++ caches `--endDead()` so it can find the first new op afterward).
        let emptyflag = self.data.obank().empty();
        let marker: Option<OpId> = if emptyflag { None } else { self.dead_tail() };

        // (the per-function Override lives on the Funcdata; `localoverride`).
        let flowoverride: uint4 = if self.flowoverride_present {
            self.data.get_override().get_flow_override(curaddr)
        } else {
            crate::overrides::flow_type::NONE
        };

        let mut emit = FlowEmit::new(&mut self.data, self.env);
        match self.env.translate().one_instruction(&mut emit, curaddr) {
            Ok(s) => {
                let emit_err = emit.error;
                step = s;
                if let Some(err) = emit_err {
                    // A `new*` factory threw mid-`dump` (the C++ exception unwinds
                    // out of `oneInstruction`).  The `PcodeEmit::dump` trait method
                    // is infallible, so the error was captured; re-raise it here, at
                    // the same point the C++ exception would propagate.
                    return Err(err);
                }
            }
            Err(err) => {
                step = self.handle_decode_error(curaddr, err)?;
            }
        }

        // (kuna overlapbranch) The previous instruction was a conditional branch
        // whose own target lands strictly inside the instruction just decoded --
        // the fall-through decode has swallowed the branch target (the x86
        // anti-disassembly junk-lead-byte idiom).  The explicitly encoded target
        // wins: drop the ops this decode emitted and end the fall-through edge
        // with a halt at its own address, so the target is decoded on its own
        // boundary by the pending addrlist entry.  Nothing already committed to
        // is moved -- the loser is the instruction currently being decoded.
        let overlap_offsets = overlap_watch.as_ref().and_then(|(fallthru, target)| {
            let same_space = target.get_space().map(|sp| sp.get_index())
                == curaddr.get_space().map(|sp| sp.get_index());
            if fallthru == curaddr && same_space {
                Some((fallthru.get_offset(), target.get_offset()))
            } else {
                None
            }
        });
        if crate::kuna_overlapbranch::kuna_overlaps_pending_branch(
            self.env.overlap_branch_enabled(),
            curaddr.get_offset(),
            step,
            overlap_offsets,
        ) && !self.kuna_overlap_reconverges(curaddr, step, &overlap_watch)
        {
            let bogus = if emptyflag {
                self.dead_head()
            } else {
                marker.and_then(|m| self.dead_next(m))
            };
            self.delete_remaining_ops(bogus)?;
            self.artificial_halt(curaddr, pcodeop_flags::badinstruction)?;
            let mut target = String::new();
            if let Some((_, t)) = overlap_watch.as_ref() {
                let _ = t.print_raw(&mut target);
            }
            self.data.warning(
                &format!(
                    "overlapbranch: this instruction overlaps the branch target at {target}; \
truncating the fall-through here"
                ),
                curaddr,
            );
            step = 1;
        }

        // (Insert with an INVALID seqnum; the first-op seqnum is filled in below.)
        self.visited.insert(
            curaddr.clone(),
            VisitStat { seqnum: invalid_seqnum(), size: step },
        );

        // Update min/max address.
        if curaddr < &self.minaddr {
            self.minaddr = curaddr.clone();
        }
        let endaddr = curaddr + step as i64;
        if self.maxaddr < endaddr {
            self.maxaddr = endaddr.clone();
        }

        // oiter points at first new op: beginDead() if was empty, else ++marker.
        let first_new: Option<OpId> = if emptyflag {
            self.dead_head()
        } else {
            marker.and_then(|m| self.dead_next(m))
        };

        self.overlap_cbranch_target = None; // (kuna overlapbranch) per-instruction
        if let Some(firstop) = first_new {
            let seq = self.data.obank().get(firstop).expect("process: stale firstop").get_seq_num().clone();
            if let Some(stat) = self.visited.get_mut(curaddr) {
                stat.seqnum = seq;
            }
            self.op_mark_start_instruction(firstop);
            if flowoverride != crate::overrides::flow_type::NONE {
                self.data.override_flow(curaddr, flowoverride)?;
            }
            self.xref_control_flow(Some(firstop), startbasic, &mut isfallthru)?;
        }

        if isfallthru {
            let next = curaddr + step as i64;
            // (kuna overlapbranch) arm the overlap check for the fall-through this
            // instruction is about to push: only a forward conditional target can
            // be swallowed by the fall-through's own encoding.
            self.overlap_watch = match self.overlap_cbranch_target.take() {
                Some(t) if t > next => Some((next.clone(), t)),
                _ => None,
            };
            if self.is_funcbound_fallthru(&next) {
                // (kuna funcboundflow) Fall-through has reached the entry of the
                // next function -- the callee just executed (an unnamed static
                // `exit`/`abort`/`die()` wrapper kuna could not prove no-return)
                // must be no-return.  Plant a no-return artificial RETURN and
                // stop, instead of decoding the next function's body into this
                // one (the cross-function merge bug).  Mirrors the noreturn-call
                // halt in `check_for_flow_modification`; the halt is appended
                // after the current instruction's ops by `artificial_halt`.
                // It starts a basic block because `collect_edges` emits a
                // fall-through edge for a CBRANCH regardless, and without a
                // boundary that edge is a self-loop on the branch's own block.  It
                // is NOT marked an instruction start: `fallthru_op` can only reach
                // it as a same-instruction op, the next address being undecoded.
                let halt = self.artificial_halt(curaddr, pcodeop_flags::noreturn)?;
                self.op_mark_start_basic(halt);
                self.data.warning(
                    "funcboundflow: fall-through reached the next function entry; truncating flow here",
                    curaddr,
                );
                isfallthru = false;
            } else {
                self.addrlist.push(next);
            }
        }
        Ok(isfallthru)
    }

    /// (kuna `overlapbranch`) Do the fall-through decode at `curaddr` and the
    /// branch target's own decode end at the same address?  True means they are
    /// the same instruction with leading bytes skipped — glibc's conditional-`LOCK`
    /// idiom, where both streams are real — and the truncation must not fire.
    /// See [`kuna_overlapbranch::kuna_streams_reconverge`](crate::kuna_overlapbranch).
    ///
    /// Runs only once the cheap strict-interior test has already matched, which it
    /// never does in ordinary code, so the extra decode costs nothing in practice.
    fn kuna_overlap_reconverges(
        &self,
        curaddr: &Address,
        step: int4,
        watch: &Option<(Address, Address)>,
    ) -> bool {
        let target = match watch {
            Some((_, t)) => t,
            None => return true,
        };
        let len = self.env.translate().instruction_length(target).ok();
        crate::kuna_overlapbranch::kuna_streams_reconverge(
            curaddr.get_offset(),
            step,
            target.get_offset(),
            len,
        )
    }

    /// (kuna `funcboundflow`) Should a fall-through to `next` be truncated because
    /// it is the entry of another known function (so the walk has run off the end
    /// of the current function)?  Gated by `option funcboundflow`
    /// (`funcbound_flow_enabled`); when on, resolves `next` against the symbol
    /// table via [`query_call`](FlowEnvironment::query_call) and excludes the
    /// current function's own entry.  See [`kuna_funcboundflow`].
    fn is_funcbound_fallthru(&self, next: &Address) -> bool {
        // Fast-path the default-off gate: no symbol-table lookup per instruction.
        if !self.env.funcbound_flow_enabled() {
            return false;
        }
        crate::kuna_funcboundflow::kuna_should_bound_at_entry(
            true,
            self.env.query_call(next).is_some(),
            next == self.data.get_address(),
        )
    }

    /// Apply the decode-error policy for an `oneInstruction` failure (C++
    /// `processInstruction`'s `catch` ladder, `flow.cc:441-475`).
    ///
    /// Returns the instruction `step` to record.  The unimplemented/baddata
    /// branches and the ignore/error/truncate policy bits are transcribed.
    fn handle_decode_error(&mut self, curaddr: &Address, err: KunaError) -> KunaResult<int4> {
        match &err {
            KunaError::Unimpl { instruction_length, .. } => {
                if (self.flags & flow_flags::ignore_unimplemented) != 0 {
                    let step = *instruction_length;
                    if !self.has_unimplemented() {
                        self.flags |= flow_flags::unimplemented_present;
                        // data.warningHeader("Control flow ignored unimplemented instructions");  -- STUB(W4)
                    }
                    Ok(step)
                } else if (self.flags & flow_flags::error_unimplemented) != 0 {
                    Err(err) // rethrow
                } else {
                    // Add infinite loop instruction (pretend size 1).
                    self.artificial_halt(curaddr, pcodeop_flags::unimplemented)?;
                    // data.warning("Unimplemented instruction - Truncating control flow here", curaddr);  -- STUB(W4)
                    if !self.has_unimplemented() {
                        self.flags |= flow_flags::unimplemented_present;
                        // data.warningHeader("Control flow encountered unimplemented instructions");  -- STUB(W4)
                    }
                    Ok(1)
                }
            }
            KunaError::BadData { .. } => {
                if (self.flags & flow_flags::error_unimplemented) != 0 {
                    Err(err) // rethrow
                } else {
                    self.artificial_halt(curaddr, pcodeop_flags::badinstruction)?;
                    // data.warning("Bad instruction - Truncating control flow here", curaddr);  -- STUB(W4)
                    if !self.has_bad_data() {
                        self.flags |= flow_flags::baddata_present;
                        // data.warningHeader("Control flow encountered bad instruction data");  -- STUB(W4)
                    }
                    Ok(1)
                }
            }
            // The C++ only catches UnimplError/BadDataError here; any other error
            // (LowlevelError, ...) propagates out of processInstruction.
            _ => Err(err),
        }
    }

    /// First / last op of the global dead list (helpers for the marker idiom).
    fn dead_head(&self) -> Option<OpId> {
        self.data.obank().dead_front()
    }
    fn dead_tail(&self) -> Option<OpId> {
        self.data.obank().dead_back()
    }

    // -----------------------------------------------------------------------
    // fall-thru walk (the addrlist stack discipline)
    // -----------------------------------------------------------------------

    /// Figure out how far we can follow fall-thru instructions from the top of the
    /// `addrlist` stack before hitting something already seen (C++
    /// `setFallthruBound`, `flow.cc:507`).
    ///
    /// Passes back the first already-seen address in `bound`; returns `false` if
    /// the top address has already been visited (it is popped).
    fn set_fallthru_bound(&mut self, bound: &mut Address) -> KunaResult<bool> {
        let addr = self.addrlist.last().expect("set_fallthru_bound: empty addrlist").clone();
        // We compute the predecessor (last <= addr) and its successor.
        let le = self.visited_last_le_owned(&addr);
        if let Some((first, size)) = le {
            if addr == first {
                // Already visited this address: make sure it starts a basic block.
                let op = self.target(&addr)?;
                self.op_mark_start_basic(op);
                self.addrlist.pop();
                return Ok(false);
            }
            if addr < &first + size as i64 {
                self.reinterpreted(&addr)?;
            }
        }
        // ++iter: the first visited key strictly greater than addr.
        let next_key = self.visited.range((std::ops::Bound::Excluded(addr.clone()), std::ops::Bound::Unbounded)).next().map(|(k, _)| k.clone());
        match next_key {
            Some(k) => *bound = k,
            None => *bound = self.eaddr.clone(),
        }
        Ok(true)
    }

    /// Process the next sequence of instructions in fall-thru order (C++
    /// `fallthru`, `flow.cc:563`).
    fn fallthru(&mut self) -> KunaResult<()> {
        let mut bound = self.eaddr.clone();
        if !self.set_fallthru_bound(&mut bound)? {
            return Ok(());
        }
        let mut startbasic = true;
        loop {
            let curaddr = self.addrlist.pop().expect("fallthru: empty addrlist");
            let fallthruflag = self.process_instruction(&curaddr, &mut startbasic)?;
            if !fallthruflag {
                break;
            }
            if self.addrlist.is_empty() {
                break;
            }
            let backaddr = self.addrlist.last().expect("fallthru: addrlist back").clone();
            if bound <= backaddr {
                if bound == self.eaddr {
                    if backaddr <= self.eaddr {
                        // (kuna) `eaddr` is the last IN-BODY byte, so an
                        // instruction that STARTS on it is inside the range --
                        // `new_address` admits a branch to the very same address
                        // (`eaddr < to`).  Upstream can compare equal here only at
                        // the top of memory, because its `eaddr` is the space's
                        // highest address; under a declared extent it is every
                        // function's last instruction, and calling that one out of
                        // bounds dropped the closing `ret` of a CORRECTLY declared
                        // body.  Decode it — the fall-through past it is caught on
                        // the next lap, where `backaddr` really is above `eaddr`.
                        continue;
                    }
                    self.handle_out_of_bounds(&self.eaddr.clone(), &backaddr)?;
                    self.outofbounds.insert(backaddr.clone());
                    self.unprocessed.push(backaddr);
                    self.addrlist.pop();
                    return Ok(());
                }
                if bound == backaddr {
                    // Hit the bound exactly.
                    if startbasic {
                        let op = self.target(&backaddr)?;
                        self.op_mark_start_basic(op);
                    }
                    self.addrlist.pop();
                    break;
                }
                if !self.set_fallthru_bound(&mut bound)? {
                    return Ok(()); // Reset bound
                }
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // unprocessed-address cleanup (branch stubs)
    // -----------------------------------------------------------------------

    /// Add any remaining un-followed addresses to the `unprocessed` list (C++
    /// `findUnprocessed`, `flow.cc:852`).
    fn find_unprocessed(&mut self) -> KunaResult<()> {
        let addrs: Vec<Address> = self.addrlist.clone();
        for addr in addrs {
            if self.seen_instruction(&addr) {
                let op = self.target(&addr)?;
                self.op_mark_start_basic(op);
            } else {
                self.unprocessed.push(addr);
            }
        }
        Ok(())
    }

    /// Sort and deduplicate the `unprocessed` list (C++ `dedupUnprocessed`,
    /// `flow.cc:868`).
    fn dedup_unprocessed(&mut self) {
        if self.unprocessed.is_empty() {
            return;
        }
        self.unprocessed.sort();
        self.unprocessed.dedup();
    }

    /// Fill-in artificial HALT p-code for `unprocessed` addresses (C++
    /// `fillinBranchStubs`, `flow.cc:891`).
    fn fillin_branch_stubs(&mut self) -> KunaResult<()> {
        self.find_unprocessed()?;
        self.dedup_unprocessed();
        let addrs: Vec<Address> = self.unprocessed.clone();
        for addr in addrs {
            let op = self.artificial_halt(&addr, pcodeop_flags::missing)?;
            self.op_mark_start_basic(op);
            self.op_mark_start_instruction(op);
            // (kuna) A caller-declared extent (`--define-function START-END`) is the
            // only thing that narrows the flow range, and a branch that leaves it
            // leaves an address the walk deliberately never decoded.  `collect_edges`
            // then asks `target` for the edge head and upstream throws "Could not
            // find op at target address", so declaring an extent that cuts any real
            // edge produced no C at all.  Register the stub as the instruction at
            // that address and the edge lands on it -- the clipped body plus the
            // `Function flows out of bounds` header the warning path already emits.
            // Restricted to the out-of-extent set on purpose: an unprocessed address
            // INSIDE the extent means an op that should exist does not, and
            // resolving that to a halt would truncate a function instead of
            // reporting the defect.
            if self.outofbounds.contains(&addr) {
                let seq = self
                    .data
                    .obank()
                    .get(op)
                    .expect("fillin_branch_stubs: stale stub")
                    .get_seq_num()
                    .clone();
                self.visited.insert(addr.clone(), VisitStat { seqnum: seq, size: 1 });
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // block building (collectEdges / splitBasic / connectBasic)
    // -----------------------------------------------------------------------

    /// Collect edges between basic blocks as PcodeOp→PcodeOp pairs (C++
    /// `collectEdges`, `flow.cc:908`).
    ///
    /// Edges are generated for fall-thru to a block-start op or for an explicit
    /// branch.  The `mark` discipline for de-duplicating BRANCHIND table edges is
    /// transcribed (the C++ `setMark`/`clearMark` over `block_edge` tails).
    ///
    /// STUB(W4): `data.findJumpTable(op)` (the BRANCHIND case) needs the W4
    /// `JumpTable`; the W3 shell treats every BRANCHIND as having no table (the
    /// "partial flow analysis: assume no branches out" C++ path), which is exactly
    /// the C++ behavior when `findJumpTable` returns null.
    fn collect_edges(&mut self) -> KunaResult<()> {
        if self.data.bblocks_get_size() != 0 {
            return Err(KunaError::lowlevel("Basic blocks already calculated\n"));
        }
        let order: Vec<OpId> = self.data.obank().iter_dead().collect();
        for (idx, &op) in order.iter().enumerate() {
            let nextstart = match order.get(idx + 1) {
                None => true,
                Some(&next) => {
                    self.data.obank().get(next).expect("collect_edges: stale next").is_block_start()
                }
            };
            match self.data.obank().get(op).expect("collect_edges: stale op").code() {
                OpCode::CPUI_BRANCH => {
                    let targ = self.branch_target(op)?;
                    self.block_edge1.push(op);
                    self.block_edge2.push(targ);
                }
                OpCode::CPUI_BRANCHIND => {
                    //   no JumpTable -> assume no out-branches (partial flow).
                    if let Some(jt_idx) = self.data.find_jump_table_index(op) {
                        let num = self.data.get_jump_table(jt_idx as int4).num_entries();
                        // (kuna, angr `test_decompiling_optimized_memcpy`,
                        // `option unrolledguard`) Inside a jump-table-recovery
                        // partial clone, a case target may belong to an
                        // already-recovered SIBLING table whose case body was
                        // never decoded into this partial's `visited` (it is
                        // decoded into the parent flow only after the recovery
                        // pass returns).  Upstream never reaches this because it
                        // builds one shared partial and runs collectEdges once
                        // while sibling tables are still empty; kuna rebuilds a
                        // fresh partial per table, re-cloning recovered siblings.
                        // When the gate is on, skip an unresolvable case-target
                        // edge instead of throwing -- the same "assume no branches
                        // out" shape the `findJumpTable==0` partial path uses.
                        let tolerate_missing = self.data.get_arch().unrolled_guard
                            && (self.data.flags()
                                & crate::funcdata::funcdata_flags::jumptablerecovery_on)
                                != 0;
                        // Edge per recovered case target, deduped by the C++
                        // `setMark`/`clearMark` discipline over the target ops.
                        let edge_start = self.block_edge1.len();
                        let mut skipped = 0u32;
                        for i in 0..num {
                            let addr =
                                self.data.get_jump_table(jt_idx as int4).get_address_by_index(i);
                            let targ = if tolerate_missing {
                                match self.target(&addr) {
                                    Ok(t) => t,
                                    Err(_) => {
                                        // (kuna) Sibling table body not decoded in
                                        // this partial -- skip the edge.
                                        skipped += 1;
                                        continue;
                                    }
                                }
                            } else {
                                self.target(&addr)?
                            };
                            if self.data.obank().get(targ).expect("collect_edges: targ").is_mark() {
                                continue; // Already a link between these blocks
                            }
                            self.data.obank_mut().get_mut(targ).unwrap().set_mark();
                            self.block_edge1.push(op);
                            self.block_edge2.push(targ);
                        }
                        // Clean up our marks (C++ walks back over the edges just
                        // pushed for this op).
                        for k in edge_start..self.block_edge2.len() {
                            let targ = self.block_edge2[k];
                            self.data.obank_mut().get_mut(targ).unwrap().clear_mark();
                        }
                        if skipped != 0 {
                            // (kuna, `option unrolledguard`) Log the recovery
                            // (standing requirement: "log when it recovers the
                            // table").  Only reached with the gate on, so it never
                            // pollutes the default datatest run.
                            let oaddr = self
                                .data
                                .obank()
                                .get(op)
                                .map(|o| o.get_addr().get_offset())
                                .unwrap_or(0);
                            eprintln!(
                                "[kuna unrolledguard] interleaved jump table at 0x{:x}: \
                                 skipped {} undecoded sibling-table case-target edge(s) in \
                                 the partial-flow rebuild (table recovers instead of \
                                 truncating to a computed call)",
                                oaddr, skipped,
                            );
                        }
                    }
                }
                OpCode::CPUI_RETURN => {}
                OpCode::CPUI_CBRANCH => {
                    // fallthru edge then branch-target edge.
                    if let Some(targ) = self.fallthru_op(op) {
                        self.block_edge1.push(op);
                        self.block_edge2.push(targ);
                    } else {
                        // The C++ pushes whatever fallthruOp returns (it asserts
                        // non-null for a CBRANCH that can fall thru); a missing
                        // fallthru is a malformed graph — surface it.
                        return Err(KunaError::lowlevel(
                            "collectEdges: CBRANCH has no fall-thru op",
                        ));
                    }
                    let targ = self.branch_target(op)?;
                    self.block_edge1.push(op);
                    self.block_edge2.push(targ);
                }
                _ => {
                    if nextstart {
                        // Put in fallthru edge if new basic block.
                        if let Some(targ) = self.fallthru_op(op) {
                            self.block_edge1.push(op);
                            self.block_edge2.push(targ);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Split raw p-code ops up into basic blocks (C++ `splitBasic`, `flow.cc:985`).
    ///
    /// PcodeOps are moved out of the dead list into their assigned
    /// `PcodeBlockBasic` (created at each `startbasic`-marked op), and the
    /// instruction address ranges are recorded.
    fn split_basic(&mut self) -> KunaResult<()> {
        let order: Vec<OpId> = self.data.obank().iter_dead().collect();
        let mut it = order.into_iter();
        let first = match it.next() {
            Some(o) => o,
            None => return Ok(()),
        };
        if !self.data.obank().get(first).expect("split_basic: stale first").is_block_start() {
            return Err(KunaError::lowlevel("First op not marked as entry point"));
        }
        let root = self.bblocks_root();
        let mut cur = self.data.bblocks_mut().new_block_basic(root);
        self.data.op_insert(first, cur, None); // insert at end of cur
        self.data.bblocks_mut().set_start_block(root, cur);
        let mut start = self.data.obank().get(first).expect("split_basic").get_addr().clone();
        let mut stop = start.clone();
        for op in it {
            if self.data.obank().get(op).expect("split_basic: stale op").is_block_start() {
                self.data.set_basic_block_range(cur, &start, &stop);
                cur = self.data.bblocks_mut().new_block_basic(root); // next block
                start = self.data.obank().get(op).expect("split_basic").get_seq_num().get_addr().clone();
                stop = start.clone();
            } else {
                let next_addr =
                    self.data.obank().get(op).expect("split_basic").get_addr().clone();
                if stop < next_addr {
                    stop = next_addr;
                }
            }
            self.data.op_insert(op, cur, None);
        }
        self.data.set_basic_block_range(cur, &start, &stop);
        Ok(())
    }

    /// Generate edges between basic blocks (C++ `connectBasic`, `flow.cc:1023`).
    fn connect_basic(&mut self) {
        let n = self.block_edge1.len();
        for i in 0..n {
            let op = self.block_edge1[i];
            let targ_op = self.block_edge2[i];
            let bs = self.data.obank().get(op).expect("connect_basic: stale op").get_parent().expect("connect_basic: op has no parent");
            let targ_bs = self.data.obank().get(targ_op).expect("connect_basic: stale targ").get_parent().expect("connect_basic: targ has no parent");
            self.data.bblocks_mut().add_edge(bs, targ_bs);
        }
    }

    /// The root graph node of `bblocks`.
    fn bblocks_root(&self) -> BlockId {
        self.data.bblocks_ref().root.expect("FlowInfo: bblocks root not constructed")
    }

    // -----------------------------------------------------------------------
    // top-level driving (generateOps / generateBlocks)
    // -----------------------------------------------------------------------

    /// Generate raw control-flow from the function's base address (C++
    /// `generateOps`, `flow.cc:789`).
    ///
    /// The fall-thru-recovery loop runs fully; the jump-table recovery loop
    /// (`recoverJumpTables` + `newAddress` re-fill) is driven via
    /// [`generate_ops_with_jumptables`](Self::generate_ops_with_jumptables) when a
    /// jump-table pipeline runner is available (the `build_and_follow_flow`
    /// caller, which owns `&mut Architecture` for the "jumptable" action set).
    /// `injectPcode`/`checkContainedCall`/`checkMultistageJumptables` remain
    /// `// STUB(W4)`.
    pub fn generate_ops(&mut self) -> KunaResult<()> {
        self.clear_properties();
        self.addrlist.push(self.data.get_address().clone());
        while !self.addrlist.is_empty() {
            // Recovering as much as possible except jumptables.
            self.fallthru()?;
        }
        if self.has_inject() {
            self.inject_pcode()?;
        }
        // The jump-table recovery loop is driven from
        // `generate_ops_with_jumptables` (needs the action-pipeline runner).  Any
        // BRANCHIND left in `tablelist` is handled there; without a runner the
        // BRANCHIND has no out-branches (faithful partial flow), matching the C++
        // `findJumpTable==0` path in `collectEdges`.
        Ok(())
    }

    /// Drive the jump-table recovery loop after the fall-thru phase (C++
    /// `generateOps`'s `do { while(!tablelist.empty()) recoverJumpTables ... }`
    /// loop, `flow.cc:799-823`).
    ///
    /// `run_pipeline` runs the "jumptable" action set on a partial Funcdata (the
    /// caller binds it to the real [`Architecture`]'s action root).  For each
    /// recovered table, `newAddress` re-fills as much flow as possible.
    ///
    /// STUB(W4): `checkContainedCall` / `checkMultistageJumptables` (the PIC /
    /// multistage outer loop) need the override table + FuncCallSpecs; the single
    /// recovery pass over the current `tablelist` is the load-bearing part for the
    /// corpus and runs fully.
    ///
    /// The outer `do/while` drains `injectlist` on every round (`flow.cc:819`), so a
    /// registered `<callotherfixup>` is applied to table-discovered flow exactly as it
    /// is to fall-thru flow; a round that injects may itself queue new indirect
    /// branches, hence the loop.
    pub fn generate_ops_with_jumptables(
        &mut self,
        run_pipeline: &mut JtPipelineFn<'_>,
    ) -> KunaResult<()> {
        self.generate_ops()?;
        loop {
            while !self.tablelist.is_empty() {
                let new_tables = self.recover_jump_tables(run_pipeline)?;
                self.tablelist.clear();
                for jt_idx in new_tables {
                    let indop = match self.data.get_jump_table(jt_idx as int4).get_indirect_op() {
                        Some(op) => op,
                        None => continue,
                    };
                    let num = self.data.get_jump_table(jt_idx as int4).num_entries();
                    for i in 0..num {
                        let addr = self.data.get_jump_table(jt_idx as int4).get_address_by_index(i);
                        self.new_address(indop, &addr)?;
                    }
                    while !self.addrlist.is_empty() {
                        self.fallthru()?;
                    }
                }
            }
            // STUB(W4): checkContainedCall + checkMultistageJumptables (outer
            // do/while) — the multistage/inline restart that could re-populate
            // tablelist is the W4 override table; the single pass suffices for the
            // corpus switches.
            if self.has_inject() {
                self.inject_pcode()?;
            }
            if self.tablelist.is_empty() {
                break;
            }
        }
        Ok(())
    }

    /// Generate basic blocks from the raw control-flow (C++ `generateBlocks`,
    /// `flow.cc:826`).
    ///
    /// STUB(W4): `removeUnreachableBlocks` is the W4 structuring sweep; the new
    /// entry-block insertion when the start block has incoming edges, and the
    /// fillin/collect/split/connect pipeline, are faithful.
    pub fn generate_blocks(&mut self) -> KunaResult<()> {
        self.fillin_branch_stubs()?;
        self.collect_edges()?;
        self.split_basic()?; // Split ops up into basic blocks
        self.connect_basic(); // Generate edges between basic blocks
        if self.data.bblocks_get_size() != 0 {
            let startblock = self.data.bblocks_get_block(0);
            if self.data.bblocks_ref().block(startblock).size_in() != 0 {
                let root = self.bblocks_root();
                let newfront = self.data.bblocks_mut().new_block_basic(root);
                self.data.bblocks_mut().add_edge(newfront, startblock);
                self.data.bblocks_mut().set_start_block(root, newfront);
                let entry = self.data.get_address().clone();
                self.data.set_basic_block_range(newfront, &entry, &entry);
            }
        }
        if self.has_possible_unreachable() {
            // data.removeUnreachableBlocks(false, true);  -- STUB(W4): structuring.
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // W4 subsystem skeletons (FuncCallSpecs / JumpTable / injection)
    //
    // Each is a faithful skeleton: it carries the W3-portable bookkeeping and
    // returns a precise stub error / no-op at the W4 boundary.  Recorded as
    // losses.  The exact C++ bodies live in flow.cc and are reproduced in the
    // method docs so the W4 wave can fill them in against this surface.
    // -----------------------------------------------------------------------

    /// Set up the FuncCallSpecs for a new CALL site (C++ `setupCallSpecs`,
    /// `flow.cc:698`).  Returns `true` if the sub-function never returns.
    ///
    /// Builds a [`FuncCallSpecs`](crate::fspec::FuncCallSpecs) bound to the CALL op
    /// and its resolved entry address, pushes it onto the function's `qlst`,
    /// replaces the call op's in0 with an \e fspec annotation Varnode (so the
    /// printer recovers the callee name and the body machinery treats the rest of
    /// the inputs as parameters), and resolves the callee symbol via
    /// [`queryCall`](Self::query_call).
    ///
    /// STUB(W4): `Override::applyPrototype` and the no-return /
    /// `checkForFlowModification` flow-modification family are W4; the proto query
    /// returns only the callee name (the full proto copy lands at
    /// `ActionDefaultParams` time, exactly as the C++ `queryCall` postpone notes).
    fn setup_call_specs(&mut self, op: OpId) -> KunaResult<bool> {
        // C++ FuncCallSpecs(op) reads the entry address off op->getIn(0) for a
        // direct CALL (fspec.cc:4938-4945).
        let mut entry = self
            .data
            .obank()
            .get(op)
            .and_then(|o| o.get_in(0))
            .and_then(|vn| self.data.vbank().get(vn))
            .map(|v| v.get_addr().clone())
            .unwrap_or_default();
        // op->getIn(0) was already converted to an fspec pointer.  This happens
        // when we are cloning an op for inlining (inlineClone -> xrefInlinedBranch
        // -> setupCallSpecs): the cloned CALL still carries the *callee's* fspec
        // annotation in slot 0, not a raw code-ref.  C++ recovers the original
        // entry via `getFspecFromConst(entryaddress)->entryaddress`; here the fspec
        // offset is a process-unique handle into the side table, so unwrap it the
        // same way (fspec.cc:4940-4945).
        if entry
            .get_space()
            .map(|sp| sp.get_type() == kuna_base::space::spacetype::IPTR_FSPEC)
            .unwrap_or(false)
        {
            if let Some(info) = kuna_base::space::fspec_lookup(entry.get_offset()) {
                entry = info.entry;
            }
        }
        self.build_call_specs(op, entry.clone(), false)?;
        self.qlst_count += 1;
        // While weaving a call-fixup body, a nested direct CALL to the SAME
        // fixup entry must NOT re-inject (cycle break; C++ cancelInjectId).
        if self.injecting_entry.as_ref() == Some(&entry) {
            if let Some(idx) = self.data.get_call_specs_index(op) {
                self.data.get_call_specs_mut(idx).proto_mut().cancel_inject_id();
            }
        }
        self.check_for_flow_modification(op, &entry)
    }

    /// Examine and apply the call site's prototype/control-flow modifications
    /// (C++ `FlowInfo::checkForFlowModification`, `flow.cc:654`).
    ///
    /// If the call spec is marked \e inline, the CALL op is queued on `injectlist`
    /// (so `injectPcode` clones the callee body at the site).  If it is \e
    /// noreturn, an artificial halt (`PcodeOp::noreturn`) is planted in the dead
    /// list immediately after the call op — the per-instruction processor backs up
    /// one op and picks up the halt instead of following flow off the end — and (if
    /// not inline) a `"Subroutine does not return"` warning is buffered.  Returns
    /// `true` when the call never returns (the caller backs up one op).
    ///
    /// `op` is the CALL/CALLIND op and `entry` its resolved direct entry address
    /// (invalid for an indirect call); the inline/noreturn flow effects were
    /// already copied onto the fspec by [`build_call_specs`].
    fn check_for_flow_modification(&mut self, op: OpId, entry: &Address) -> KunaResult<bool> {
        // The flow effects live on the just-built fspec; recover its flags.
        let (is_inline, is_no_return) = match self.data.get_call_specs_index(op) {
            Some(idx) => {
                let fc = self.data.get_call_specs(idx);
                (fc.proto().is_inline(), fc.proto().is_no_return())
            }
            None => (false, false),
        };
        if is_inline {
            self.injectlist.push(op);
        }
        // The noreturn flag is also reported straight from the symbol table for
        // envs that carry it only there (the W4-era `query_call_no_return` hook).
        let no_return = is_no_return || (!entry.is_invalid() && self.env.query_call_no_return(entry));
        if no_return {
            let addr = self
                .data
                .obank()
                .get(op)
                .expect("checkForFlowModification: stale call op")
                .get_addr()
                .clone();
            let haltop = self.artificial_halt(&addr, pcodeop_flags::noreturn)?;
            self.data.op_dead_insert_after(haltop, op);
            if !is_inline {
                self.data.warning("Subroutine does not return", &addr);
            }
            return Ok(true);
        }
        Ok(false)
    }

    /// Set up the FuncCallSpecs for a new CALLIND site (C++ `setupCallindSpecs`,
    /// `flow.cc:722`).  Returns `true` if the sub-function never returns.
    ///
    /// An indirect call leaves the entry address invalid (no direct callee); the
    /// fspec annotation is NOT installed (the C++ only swaps in the fspec input
    /// when an override turns the indirect call into a direct one).  The call op's
    /// in0 stays the indirect target Varnode, which the printer renders as the
    /// `(*funcptr)(...)` callee.
    fn setup_callind_specs(&mut self, op: OpId) -> KunaResult<bool> {
        // C++ `data.getOverride().applyIndirect(data,*res)` (flow.cc:729): an
        // indirect-call destination override (planted by `FuncCallSpecs::deindirect`
        // on the previous decompilation pass) redirects this CALLIND to a known
        // direct target.  Look it up by the op's address.
        let op_addr = self
            .data
            .obank()
            .get(op)
            .map(|o| o.get_addr().clone())
            .unwrap_or_default();
        let mut direct = self.data.get_override().find_indirect_override(&op_addr).cloned();

        // C++ flow.cc:730-731: `if (fc != 0 && fc->getEntryAddress() ==
        // res->getEntryAddress()) res->setAddress(Address());` — while weaving a
        // call-fixup body, cancel an indirect override that would redirect this
        // CALLIND back to the SAME fixup entry (cycle break).
        if direct.is_some() && direct.as_ref() == self.injecting_entry.as_ref() {
            direct = None;
        }

        if let Some(entry) = direct {
            // res->getEntryAddress() is now valid -> change the indirect pcode call
            // into a normal pcode call and run the full direct call-spec build
            // (queryCall name + callee-proto copy + fspec annotation + CPUI_CALL),
            // exactly as flow.cc:735-739 does after `applyIndirect`/`applyPrototype`.
            self.data.op_set_opcode_code(op, OpCode::CPUI_CALL);
            self.build_call_specs(op, entry.clone(), false)?;
            self.qlst_count += 1;
            return self.check_for_flow_modification(op, &entry);
        }

        self.build_call_specs(op, Address::default(), true)?;
        self.qlst_count += 1;
        // (kuna `fastfailnoreturn`) A Windows `int 0x29` lifts to `intloc =
        // swi(0x29); call [intloc]` — a call with no matching push, which the
        // cspec's `extrapop` then hands 8 bytes back for.  `__fastfail` never
        // returns, so mark the spec and plant the halt here; downstream that is
        // indistinguishable from a named no-return callee.  The warning the
        // no-return path buffers is deliberately not emitted: the divergence is
        // definitional, not a surprise, and a function can hold a dozen sites.
        if self.env.is_fastfail_callind(&self.data, op) {
            if let Some(idx) = self.data.get_call_specs_index(op) {
                self.data.get_call_specs_mut(idx).proto_mut().set_no_return(true);
            }
            let addr = self
                .data
                .obank()
                .get(op)
                .expect("fastfailnoreturn: stale call op")
                .get_addr()
                .clone();
            let haltop = self.artificial_halt(&addr, pcodeop_flags::noreturn)?;
            self.data.op_dead_insert_after(haltop, op);
            return Ok(true);
        }
        // C++ `return checkForFlowModification(*res)` (flow.cc:740).  An indirect
        // call has an invalid entry, so the inline/noreturn flow effects are only
        // present if an override turned it direct (handled above).
        self.check_for_flow_modification(op, &Address::default())
    }

    /// Shared body of [`setup_call_specs`]/[`setup_callind_specs`]: create the
    /// `FuncCallSpecs`, push it onto `qlst`, and (for a direct CALL) install the
    /// \e fspec annotation in0 + register the printed name.
    fn build_call_specs(&mut self, op: OpId, entry: Address, indirect: bool) -> KunaResult<()> {
        use crate::fspec::FuncCallSpecs;
        let mut fc = FuncCallSpecs::new(op, entry.clone());
        // C++ `data.getOverride().applyPrototype(data, *res)` (flow.cc:706 direct /
        // flow.cc:732 indirect, BEFORE `queryCall`): an `override prototype <addr>
        // <decl>` keyed by the call op's address copies its parsed prototype onto
        // this call spec (`fspecs.copy(*proto)`).  Applies to BOTH direct and
        // indirect calls.  Runs BEFORE the inject-id query so the queryCall-equiv
        // `copyFlowEffects` (inline/inject id, below) is NOT clobbered by the copy —
        // matching the C++ `applyPrototype` → `queryCall` order, which is what lets
        // an injected CALLIND carry BOTH the override args and the fixup injection.
        {
            let op_addr =
                self.data.obank().get(op).map(|o| o.get_addr().clone()).unwrap_or_default();
            let pieces = self
                .data
                .get_override()
                .find_proto_override(&op_addr)
                .and_then(|ov| ov.pieces())
                .cloned();
            if let Some(pieces) = pieces {
                if let Some(proto) = self.env.build_override_proto(&pieces)? {
                    fc.proto_mut().copy(&proto);
                }
            }
        }
        // queryCall: resolve the direct callee symbol's name (W4 proto copy
        // postponed to ActionDefaultParams).
        if !indirect && !entry.is_invalid() {
            if let Some(name) = self.env.query_call(&entry) {
                let _ = fc.set_funcdata(entry.clone(), &name);
            }
            // C++ queryCall: fspecs.copyFlowEffects(otherfunc->getFuncProto()) —
            // copies the IS_INLINE / NO_RETURN flow effects from the callee proto.
            // Carry the inline flag and inject id so `checkForFlowModification`
            // (`fspecs.isInline()`) pushes the op to `injectlist` and `injectPcode`
            // can route to inject- vs inline-subfunction.  `option inline <name>`
            // (no inject id) sets is_inline only; `IfcFixupApply` sets the inject
            // id (which also implies inline, fspec.cc `setInjectId`).
            let inject_id = self.env.query_call_inject_id(&entry);
            let mut is_inline_call = false;
            if inject_id >= 0 {
                fc.proto_mut().set_inject_id(inject_id);
                is_inline_call = true;
            } else if self.env.query_call_inline(&entry) {
                fc.proto_mut().set_inline(true);
                is_inline_call = true;
            }
            // C++ `ActionDefaultParams::apply` (coreaction.cc:2379-2391) performs
            // the full callee-proto copy (only when `fc` has no model).
            // RELOCATION STUB(W4 callee-Funcdata copy): the C++ does this copy at
            // ActionDefaultParams time; flow.cc:683-686 deliberately POSTPONES the
            // full copy from queryCall to ActionDefaultParams "so last-second
            // prototype changes can come in".  kuna performs the copy HERE instead
            // — `coreaction_protos.rs` (which hosts `ActionDefaultParams::apply`)
            // is a reserved file, and a new schedule node would break the B0
            // byte-identical `list action` listing.  On the kuna analysis path the
            // declared callee signature is locked by `parse line extern` BEFORE
            // flow runs, so no last-second change occurs and the observable result
            // (the call spec carrying the callee's recovered FuncProto) is
            // identical to the postponed C++ copy.  When `query_callee_proto`
            // returns None (an unknown callee — the common datatest case),
            // ActionDefaultParams still applies its default-model internal proto
            // (the pristine `set_internal(evalfp, void)` arm, unchanged): we only
            // pre-seat the model when a declared callee proto exists, which is the
            // `fc->copy(otherfunc->getFuncProto())` arm.
            //
            // INLINE EXCLUSION (faithful to the postpone): an \e inline call is
            // injected DURING flow (`checkForFlowModification` queues it on
            // `injectlist`; `injectPcode` clones the callee body and removes the
            // call op), so by C++ ActionDefaultParams time its FuncCallSpecs is
            // already gone — the copy arm never runs for it.  Pre-seating a model
            // here would instead corrupt the inline injection path (the inline
            // datatest renders the cloned body, not a modeled call).  So we
            // restrict the pre-seat to NON-inline calls, exactly mirroring which
            // calls the C++ ActionDefaultParams copy arm actually reaches.
            if !is_inline_call && !fc.proto().has_model() {
                let callee_proto = if entry.is_invalid() {
                    None
                } else {
                    self.data.get_arch().query_callee_proto(&entry)
                };
                if let Some(callee_proto) = callee_proto {
                    fc.proto_mut().copy(&callee_proto);
                    let evalfp = self
                        .data
                        .get_arch()
                        .eval_fp_called()
                        .or_else(|| self.data.get_arch().default_fp())
                        .cloned();
                    if let Some(evalfp) = evalfp {
                        if !fc.proto().is_model_locked() && !fc.proto().has_matching_model(&evalfp)
                        {
                            fc.proto_mut().set_model(Some(evalfp));
                        }
                    }
                }
            }
        }
        // The index is the call spec's identity (looked up by
        // op at use sites — see Funcdata::get_call_specs_index).
        let _idx = self.data.push_call_specs(fc);

        if !indirect {
            // Replace the
            // call-target Varnode with the fspec annotation.  The handle is a
            // process-unique counter; the printed name + entry are registered in
            // the fspec-space side table (the C++ pointer-cast equivalent).
            let handle = next_fspec_handle();
            let angr = self.data.get_arch().kuna_name_style();
            // Register the call spec's printed name under the handle.
            self.data
                .get_call_specs(self.data.num_calls() - 1)
                .register_in_fspec_space(handle, angr);
            let fspecvn = self.data.new_varnode_call_specs(handle);
            self.data.op_set_input(op, fspecvn, 0)?;
        }
        Ok(())
    }

    /// In-line the sub-function at the given call site (C++ `inlineSubFunction`,
    /// `flow.cc:1244`).  Returns `true` if the in-lining is successful.
    ///
    /// `op` is the CALL op carrying the inline-marked fspec; `entry` is the
    /// callee's resolved entry address (the fspec's `getEntryAddress()`).  P-code
    /// is generated for the sub-function ([`inline_flow`](Self::inline_flow)) and
    /// then woven into \b this flow at the call site.  The `inline_recursion`
    /// cycle tracking prevents a function from being inlined into itself.
    fn inline_sub_function(&mut self, op: OpId, entry: &Address) -> KunaResult<bool> {
        // The callee Funcdata is built fresh from the entry address (queryCall
        // associated the symbol; here we build it on demand).
        let mut inlinefd = match self.env.build_inline_funcdata(entry)? {
            Some(fd) => fd,
            None => return Ok(false),
        };

        if self.inline_head.is_none() {
            self.inline_head = Some(self.data.get_address().clone());
        }
        // Insert current function
        self.inline_recursion.insert(self.data.get_address().clone());
        if self.inline_recursion.contains(inlinefd.get_address()) {
            // This function has already been included with current inlining.
            let opaddr =
                self.data.obank().get(op).expect("inlineSubFunction: stale call op").get_addr().clone();
            self.data.warning("Could not inline here", &opaddr);
            return Ok(false);
        }

        let res = self.inline_flow(&mut inlinefd, op)?;
        if res < 0 {
            return Ok(false);
        } else if res == 0 {
            // easy model: remove inlined function from list so it can be inlined
            // again, even if it also inlines.
            self.inline_recursion.remove(inlinefd.get_address());
        } else {
            // hard model: add inlined function to recursion list (even if it
            // contains no inlined calls) to prevent parent from inlining it twice.
            self.inline_recursion.insert(inlinefd.get_address().clone());
        }

        // Changing CALL to JUMP may make some original code unreachable.
        self.set_possible_unreachable();
        Ok(true)
    }

    /// In-line the p-code from another function into \b this function (C++
    /// `Funcdata::inlineFlow`, `funcdata_op.cc:853`).
    ///
    /// Raw PcodeOps for the in-line function are generated into `inlinefd`
    /// (a nested [`FlowInfo`] over the same [`FlowEnvironment`], with
    /// `flow_forinline` set and the recursion state forwarded) and then cloned
    /// into \b this function via [`inline_ez_clone`](Self::inline_ez_clone) (a
    /// straight-line leaf, the \e easy model) or
    /// [`inline_clone`](Self::inline_clone) (the \e hard model, preserving
    /// addresses and replacing RETURN with a BRANCH).
    ///
    /// Returns 0 for the easy model, 1 for the hard model, -1 if inlining was not
    /// successful (`callop` is the site of the injection).
    fn inline_flow(&mut self, inlinefd: &mut Funcdata, callop: OpId) -> KunaResult<int4> {
        // inlinefd->getArch()->clearAnalysis(inlinefd): the callee Funcdata is
        // built fresh (no prior analysis to clear).
        inlinefd.obank_mut().set_uniq_id(self.data.obank().get_uniq_id());

        // Build a nested FlowInfo over the callee Funcdata using a placeholder
        // swap (the same move-out / move-back idiom as `build_partial_blocks`).
        let placeholder = Funcdata::new_placeholder_like(inlinefd)?;
        let inline_data = std::mem::replace(inlinefd, placeholder);
        let mut inlineflow = FlowInfo::new(inline_data, self.env);

        let space = self
            .data
            .get_address()
            .get_space()
            .expect("inlineFlow: caller entry has no space")
            .clone();
        let baddr = Address::new(space.clone(), 0);
        let eaddr = Address::new(space, !0u64);
        inlineflow.set_range(baddr, eaddr);
        inlineflow.set_flags(
            flow_flags::error_outofbounds
                | flow_flags::error_unimplemented
                | flow_flags::error_reinterpreted
                | flow_flags::flow_forinline,
        );
        inlineflow.forward_recursion(self);
        inlineflow.generate_ops()?;
        // Carry any warnings the nested flow buffered back to the top-level
        // function's comment store (the C++ `inline_head`/`data` reach the same
        // `glb->commentdb`).  This includes nested "Could not inline here".
        let nested_comments = inlineflow.data.drain_pending_comments();
        for (tp, ad, txt) in nested_comments {
            self.data.push_raw_comment(tp, ad, txt);
        }

        let res: int4;
        if inlineflow.check_ez_model() {
            res = 0;
            // With an EZ clone there are no jumptables to clone.
            let marker = self.dead_tail(); // last op before the clone (there is at least one)
            let calladdr = self
                .data
                .obank()
                .get(callop)
                .expect("inlineFlow: stale callop")
                .get_addr()
                .clone();
            self.inline_ez_clone(&inlineflow, &calladdr)?;
            let firstop = match marker {
                Some(m) => self.dead_next(m),
                None => self.dead_head(),
            };
            if let Some(firstop) = firstop {
                let lastop = self.dead_tail().expect("inlineFlow EZ: dead list non-empty");
                self.data.obank_mut().move_sequence_dead(firstop, lastop, callop);
                let callop_block_start = self
                    .data
                    .obank()
                    .get(callop)
                    .expect("inlineFlow: stale callop")
                    .is_block_start();
                if callop_block_start {
                    self.data
                        .obank_mut()
                        .get_mut(firstop)
                        .expect("inlineFlow: stale firstop")
                        .set_flag(pcodeop_flags::startbasic);
                    self.update_target(callop, firstop);
                } else {
                    self.data
                        .obank_mut()
                        .get_mut(firstop)
                        .expect("inlineFlow: stale firstop")
                        .clear_flag(pcodeop_flags::startbasic);
                }
            }
            self.data.op_destroy_raw(callop)?;
        } else {
            // Hard model.
            let mut retaddr = Address::default();
            if !self.test_hard_inline_restrictions(&inlineflow, callop, &mut retaddr)? {
                // Restore the callee data before bailing.
                *inlinefd = inlineflow.data;
                return Ok(-1);
            }
            res = 1;
            // Clone any jumptables from the inline piece.
            //   -- STUB(W4): inlinefd->jumpvec is the recovered-table list; an
            //      inline callee with a jumptable (none in the corpus) would clone
            //      them here.  No jumptable in the inline corpus, so this is empty.
            self.inline_clone(&inlineflow, &retaddr)?;

            // Convert the CALL op to a jump.
            loop {
                let n = self.data.obank().get(callop).expect("inlineFlow: stale callop").num_input();
                if n <= 1 {
                    break;
                }
                self.data.op_remove_input(callop, n - 1);
            }
            self.data.op_set_opcode_code(callop, OpCode::CPUI_BRANCH);
            let inline_entry = inlineflow.data.get_address().clone();
            let inlineaddr = self.data.new_code_ref(&inline_entry);
            self.data.op_set_input(callop, inlineaddr, 0)?;
        }

        let inline_uniq = inlineflow.data.obank().get_uniq_id();
        self.data.obank_mut().set_uniq_id(inline_uniq);
        // Move the (now spent) callee data back out.
        *inlinefd = inlineflow.data;
        Ok(res)
    }

    /// A function is in the EZ model if it is a straight-line leaf function (C++
    /// `FlowInfo::checkEZModel`, `flow.cc:1159`): \b true if this flow contains no
    /// CALL or BRANCH ops.
    fn check_ez_model(&self) -> bool {
        for op in self.data.obank().iter_dead() {
            if self.data.obank().get(op).expect("checkEZModel: stale op").is_call_or_branch() {
                return false;
            }
        }
        true
    }

    /// For in-lining using the \e hard model, make sure some restrictions are met
    /// (C++ `FlowInfo::testHardInlineRestrictions`, `flow.cc:1135`):
    ///
    ///   - Can only in-line the function once.
    ///   - There must be a p-code op to return to.
    ///   - There must be a distinct return address (so RETURN -> BRANCH).
    ///
    /// `inlineflow` is the (already-generated) callee flow; `op` is the CALL at
    /// the in-line site; `retaddr` is passed back with the distinct return address
    /// (unless the in-lined function doesn't return).  Returns `true` when all the
    /// hard-model restrictions are met.
    fn test_hard_inline_restrictions(
        &mut self,
        inlineflow: &FlowInfo<'a, E>,
        op: OpId,
        retaddr: &mut Address,
    ) -> KunaResult<bool> {
        if !inlineflow.data.get_func_proto().is_no_return() {
            let nextop = match self.dead_next(op) {
                Some(n) => n,
                None => {
                    let opaddr =
                        self.data.obank().get(op).expect("testHardInline: stale op").get_addr().clone();
                    self.data.warning("No fallthrough prevents inlining here", &opaddr);
                    return Ok(false);
                }
            };
            *retaddr = self.data.obank().get(nextop).expect("testHardInline: stale nextop").get_addr().clone();
            let opaddr = self.data.obank().get(op).expect("testHardInline: stale op").get_addr().clone();
            if &opaddr == retaddr {
                self.data.warning("Return address prevents inlining here", &opaddr);
                return Ok(false);
            }
            // If the inlining "jumps back" this starts a new basic block.
            self.op_mark_start_basic(nextop);
        }
        Ok(true)
    }

    /// If the given injected op is a CALL, CALLIND, or BRANCHIND, add references to
    /// it in other flow tables (C++ `FlowInfo::xrefInlinedBranch`, `flow.cc:1055`).
    fn xref_inlined_branch(&mut self, op: OpId) -> KunaResult<()> {
        match self.data.obank().get(op).expect("xrefInlinedBranch: stale op").code() {
            OpCode::CPUI_CALL => {
                self.setup_call_specs(op)?;
            }
            OpCode::CPUI_CALLIND => {
                self.setup_callind_specs(op)?;
            }
            OpCode::CPUI_BRANCHIND => {
                let recovered = match self.data.link_jump_table(op) {
                    Some(jt_idx) => self.data.get_jump_table(jt_idx as int4).num_entries() != 0,
                    None => false,
                };
                if !recovered {
                    self.tablelist.push(op);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Clone the given in-line flow into \b this flow using the \e hard model (C++
    /// `FlowInfo::inlineClone`, `flow.cc:1076`).
    ///
    /// Individual PcodeOps from the in-lined Funcdata are cloned into \b this flow,
    /// preserving their original address.  Any RETURN op is replaced with a jump to
    /// the first address following the call site (`retaddr`).
    fn inline_clone(&mut self, inlineflow: &FlowInfo<'a, E>, retaddr: &Address) -> KunaResult<()> {
        let src = &inlineflow.data;
        let src_ops: Vec<OpId> = src.obank().iter_dead().collect();
        for op in src_ops {
            let opc = src.obank().get(op).expect("inlineClone: stale src op").code();
            let seq = src.obank().get(op).expect("inlineClone: stale src op").get_seq_num().clone();
            let cloneop = if opc == OpCode::CPUI_RETURN && !retaddr.is_invalid() {
                let cloneop = self.data.new_op_seq(1, seq);
                self.data.op_set_opcode_code(cloneop, OpCode::CPUI_BRANCH);
                let vn = self.data.new_code_ref(retaddr);
                self.data.op_set_input(cloneop, vn, 0)?;
                cloneop
            } else {
                self.data.clone_op_from(src, op, seq)?
            };
            if self.data.obank().get(cloneop).expect("inlineClone: stale clone").is_call_or_branch() {
                self.xref_inlined_branch(cloneop)?;
            }
        }
        // Copy in the cross-referencing.
        self.unprocessed.extend(inlineflow.unprocessed.iter().cloned());
        self.outofbounds.extend(inlineflow.outofbounds.iter().cloned());
        self.addrlist.extend(inlineflow.addrlist.iter().cloned());
        for (addr, stat) in inlineflow.visited.iter() {
            // std::map::insert does NOT overwrite an existing key — match it.
            self.visited.entry(addr.clone()).or_insert_with(|| stat.clone());
        }
        // We don't copy inline_recursion or inline_head here.
        Ok(())
    }

    /// Clone the given in-line flow into \b this flow using the EZ model (C++
    /// `FlowInfo::inlineEZClone`, `flow.cc:1110`).
    ///
    /// Individual PcodeOps from the in-lined Funcdata are cloned into \b this flow
    /// but reassigned a new fixed address (`calladdr`), and the RETURN op is
    /// eliminated.
    fn inline_ez_clone(&mut self, inlineflow: &FlowInfo<'a, E>, calladdr: &Address) -> KunaResult<()> {
        let src = &inlineflow.data;
        let src_ops: Vec<OpId> = src.obank().iter_dead().collect();
        for op in src_ops {
            if src.obank().get(op).expect("inlineEZClone: stale src op").code() == OpCode::CPUI_RETURN {
                break;
            }
            let time = src.obank().get(op).expect("inlineEZClone: stale src op").get_seq_num().get_time();
            let myseq = SeqNum::new(calladdr.clone(), time);
            self.data.clone_op_from(src, op, myseq)?;
        }
        // We don't touch unprocessed, addrlist, or visited (straight-line, one addr).
        Ok(())
    }

    /// Perform substitution on any op that requires injection (C++
    /// `FlowInfo::injectPcode`, `flow.cc:1329`).
    ///
    /// Types of substitution: sub-function in-lining (the whole-body clone),
    /// sub-function injection (a registered payload), and user-defined op
    /// injection (CALLOTHER).  Recursion is truncated so a sub-function is not
    /// in-lined more than once.
    ///
    /// STUB(W4): the payload-injection arms (`injectUserOp` / `injectSubFunction`,
    /// which need the `PcodeInjectLibrary` payload + `doInjection`) are deferred to
    /// the injection wave; the in-lining arm (`inlineSubFunction`) — the load-
    /// bearing path for `inline.xml` — is fully wired.
    fn inject_pcode(&mut self) -> KunaResult<()> {
        // Index walk: inlineSubFunction
        // (via inline_clone -> xref_inlined_branch -> setup_call_specs) can push
        // MORE ops onto injectlist (nested inlines), so re-read the length.
        let mut i = 0usize;
        while i < self.injectlist.len() {
            let op = self.injectlist[i];
            i += 1;
            // Nullify so we don't
            // inject more than once.  A destroyed op id is filtered by liveness.
            let code = match self.data.obank().get(op) {
                Some(o) => o.code(),
                None => continue, // op was destroyed by a prior inline (nullified)
            };
            if code == OpCode::CPUI_CALLOTHER {
                // injectUserOp(op): a CALLOTHER user-op marked \e injected.  Weave
                // its callother-fixup p-code in and destroy the original CALLOTHER
                // (e.g. the MIPS `setISAMode` no-op COPY, which deadcode then
                // removes — eliminating the spurious ISA-mode-switch CALLOTHER).
                self.inject_user_op(op)?;
                continue;
            }
            // CPUI_CALL or CPUI_CALLIND.
            let idx = match self.data.get_call_specs_index(op) {
                Some(idx) => idx,
                None => continue,
            };
            let fc = self.data.get_call_specs(idx);
            if !fc.proto().is_inline() {
                continue;
            }
            let inject_id = fc.proto().get_inject_id();
            let entry = fc.get_entry_address().clone();
            let name = fc.get_name().to_string();
            if inject_id >= 0 {
                // injectSubFunction(fc): weave a registered call-fixup payload in
                // at the call site, replacing the original CALL/CALLIND.
                if self.inject_sub_function(idx, op)? {
                    let fixup = self.env.call_fixup_name(inject_id);
                    self.data.warning_header(&format!(
                        "Function: {name} replaced with injection: {fixup}"
                    ));
                    // Remove the now-injected call from qlst.
                    self.data.delete_call_spec(idx);
                }
            } else if self.inline_sub_function(op, &entry)? {
                self.data.warning_header(&format!("Inlined function: {name}"));
                // Remove the now-inlined call from qlst.
                self.data.delete_call_spec(idx);
            }
        }
        self.injectlist.clear();
        Ok(())
    }

    /// Replace an \e injected user-op (CALLOTHER) with its callother-fixup p-code
    /// (C++ `FlowInfo::injectUserOp`, `flow.cc:1214`).
    ///
    /// Builds the [`InjectContext`](crate::pcodeinject::InjectContext) from the
    /// CALLOTHER's operands (skipping in0, the inject-id annotation) and output,
    /// then drives [`do_injection`](Self::do_injection) to weave the fixup body in
    /// and destroy the original op.
    fn inject_user_op(&mut self, op: OpId) -> KunaResult<()> {
        let in0 = self
            .data
            .obank()
            .get(op)
            .expect("injectUserOp: stale op")
            .get_in(0)
            .expect("injectUserOp: CALLOTHER has no in0 (C++ UB)");
        let userop_index =
            self.data.vbank().get(in0).expect("injectUserOp: stale in0").get_offset() as uintb;

        // Build the InjectContext from the op's operands (C++ icontext setup).
        let mut icontext = crate::pcodeinject::InjectContext::default();
        let baseaddr = self.data.obank().get(op).expect("injectUserOp: stale op").get_addr().clone();
        icontext.nextaddr = Some(baseaddr.clone());
        icontext.baseaddr = Some(baseaddr);

        // Skip the inject-id annotation in slot 0.
        let n = self.data.obank().get(op).expect("injectUserOp: stale op").num_input();
        for i in 1..n {
            let vn = match self.data.obank().get(op).expect("injectUserOp: stale op").get_in(i) {
                Some(v) => v,
                None => continue,
            };
            let v = self.data.vbank().get(vn).expect("injectUserOp: stale input vn");
            let addr = v.get_addr();
            icontext.inputlist.push(kuna_num::pcoderaw::VarnodeData {
                space: addr.get_space().cloned(),
                offset: addr.get_offset(),
                size: v.get_size() as u32,
            });
        }
        // output (rare for a CALLOTHER fixup; setISAMode has none).
        if let Some(outvn) = self.data.obank().get(op).expect("injectUserOp: stale op").get_out() {
            let v = self.data.vbank().get(outvn).expect("injectUserOp: stale out vn");
            let addr = v.get_addr();
            icontext.output.push(kuna_num::pcoderaw::VarnodeData {
                space: addr.get_space().cloned(),
                offset: addr.get_offset(),
                size: v.get_size() as u32,
            });
        }

        self.do_injection(InjectSource::UserOp(userop_index), &mut icontext, op)
    }

    /// Inject a registered \e call-fixup payload at a call site and weave it into
    /// the flow, replacing the original CALL/CALLIND (C++
    /// `FlowInfo::injectSubFunction`, `flow.cc:1286`).
    ///
    /// Unlike [`inject_user_op`], the payload is named by the call spec's parked
    /// inject id (`fc->getInjectId()`, set by `IfcFixupApply`); the injection
    /// context carries the call's `baseaddr`/`nextaddr` plus the resolved callee
    /// `calladdr` (so address-relative templates resolve against the fixup
    /// target), but NO operand inputs.  After the weave, the payload's
    /// param-shift is propagated to the injected call's call spec (the last one
    /// pushed onto `qlst` during the woven body's xref).  Returns `true` (the
    /// injection happened; the caller deletes the original call spec).
    fn inject_sub_function(&mut self, idx: int4, op: OpId) -> KunaResult<bool> {
        let fc = self.data.get_call_specs(idx);
        let inject_id = fc.proto().get_inject_id();
        let entry = fc.get_entry_address().clone();
        let baseaddr = self.data.obank().get(op).expect("injectSubFunction: stale op").get_addr().clone();

        // No inputlist/output (unlike the user-op path).
        let mut icontext = crate::pcodeinject::InjectContext::default();
        icontext.baseaddr = Some(baseaddr.clone());
        icontext.nextaddr = Some(baseaddr);
        icontext.calladdr = Some(entry.clone());

        // While the body's emitted ops are xref-classified, a nested CALL/CALLIND
        // to the SAME fixup entry must not re-inject (C++ passes `fc` to
        // `xrefControlFlow` -> `setupCallSpecs(op, fc)`).
        let saved_injecting = self.injecting_entry.take();
        self.injecting_entry = Some(entry);
        let result = self.do_injection(InjectSource::CallFixup(inject_id), &mut icontext, op);
        self.injecting_entry = saved_injecting;
        result?;

        // The injected call's spec is the last one appended to qlst during the
        // woven body's xref (flow.cc:1299-1302).
        let param_shift = self.env.call_fixup_param_shift(inject_id);
        if param_shift != 0 {
            let n = self.data.num_calls();
            if n > 0 {
                self.data.get_call_specs_mut(n - 1).set_paramshift(param_shift);
            }
        }
        Ok(true)
    }

    /// Weave an injection into the flow and destroy the original op (C++
    /// `FlowInfo::doInjection`, `flow.cc:1179`).
    ///
    /// Emits the payload's p-code onto the tail of the dead list, cross-references
    /// its control flow, marks it \e incidental (per the payload), moves it to sit
    /// right after `op`, repoints the target map, and `opDestroyRaw`s `op`.  Only
    /// the user-op path is wired here (the `fc` call-fixup variant is a separate
    /// hook); the no-fallthru / incidental-copy handling is transcribed.
    fn do_injection(
        &mut self,
        source: InjectSource,
        icontext: &mut crate::pcodeinject::InjectContext,
        op: OpId,
    ) -> KunaResult<()> {
        // (the marker op just before the injection lands).
        let marker = self.dead_tail(); // there must be at least one op (`op` itself)

        // Emit the fixup p-code.  The C++
        // resolves the payload differently for a user-op (CALLOTHER) vs a
        // call-fixup (CALL/CALLIND), but the weave-in tail is identical.
        {
            let mut emit = FlowEmit::new(&mut self.data, self.env);
            match source {
                InjectSource::UserOp(userop_index) => {
                    self.env.inject_userop(userop_index, icontext, &mut emit)?;
                }
                InjectSource::CallFixup(inject_id) => {
                    self.env.inject_call_fixup_payload(inject_id, icontext, &mut emit)?;
                }
            }
            if let Some(err) = emit.error.take() {
                return Err(err);
            }
        }

        let mut startbasic =
            self.data.obank().get(op).expect("doInjection: stale op").is_block_start();
        // First op in the injection.
        let firstop = match marker {
            Some(m) => self.dead_next(m),
            None => self.dead_head(),
        };
        let firstop = match firstop {
            Some(f) => f,
            None => {
                // Empty injection: C++ throws LowlevelError("Empty injection: ...").
                // The setISAMode fixup is a non-empty `v0 = v0;` body, so an empty
                // emit means the payload mis-compiled.
                return Err(KunaError::lowlevel("Empty injection"));
            }
        };
        let mut isfallthru = true;
        let lastop = self.xref_control_flow(Some(firstop), &mut startbasic, &mut isfallthru)?;
        let lastop = lastop.unwrap_or(firstop);
        let _ = isfallthru; // (the injection's fallthru is implied by the next op)

        if startbasic {
            if let Some(nextop) = self.dead_next(op) {
                self.op_mark_start_basic(nextop);
            }
        }

        let incidental = match source {
            InjectSource::UserOp(userop_index) => self.env.is_incidental_copy_userop(userop_index),
            InjectSource::CallFixup(inject_id) => self.env.is_incidental_copy_payload(inject_id),
        };
        if incidental {
            self.data.obank_mut().mark_incidental_copy(firstop, lastop);
        }
        // Move the injection after op.
        self.data.obank_mut().move_sequence_dead(firstop, lastop, op);
        // Repoint the target map.
        self.update_target(op, firstop);
        // Drop the original CALLOTHER.
        self.data.op_destroy_raw(op)?;
        Ok(())
    }

    /// Recover jump-tables for the current `tablelist` (C++
    /// `FlowInfo::recoverJumpTables`, `flow.cc:1429`).
    ///
    /// Returns the indices (into `data.jumpvec`) of the recovered tables.  A
    /// BRANCHIND that cannot be recovered is truncated to a call/return via
    /// [`truncate_indirect_jump`](Self::truncate_indirect_jump) (unless this flow
    /// is being inlined).  `run_pipeline` runs the "jumptable" action set on the
    /// partial Funcdata.
    ///
    /// STUB(W4): the `notreached` re-queue (a BRANCHIND not yet reachable through
    /// partial flow) needs the multistage outer loop; here every BRANCHIND in the
    /// list is attempted once (the corpus switches are reachable in one pass).
    #[allow(clippy::mutable_key_type)]
    fn recover_jump_tables(
        &mut self,
        run_pipeline: &mut JtPipelineFn<'_>,
    ) -> KunaResult<Vec<usize>> {
        let mut new_tables: Vec<usize> = Vec::new();
        let table_ops: Vec<OpId> = self.tablelist.clone();
        let record = self.does_jump_record();
        let for_inline = self.is_flow_for_inline();
        // Snapshot the original flow's `visited` for the partial clone.
        let visited_snapshot = self.visited.clone();
        // The partial sub-decompilation shared by every table in this batch (C++
        // builds one `partial` per `recoverJumpTables` call and `stageJumpTable`
        // guards the clone behind `if (!partial.isJumptableRecoveryOn())`).  Left
        // `None` and rebuilt per table when `option jtsharepartial` is off.
        let mut shared_partial: Option<Funcdata> = None;
        for op in table_ops {
            let mode = self.data.recover_jump_table_flow(
                op,
                record,
                &visited_snapshot,
                run_pipeline,
                &mut shared_partial,
            );
            match mode {
                Ok(Some(jt_idx)) => {
                    // jt->isPartial(): if incomplete and there is more flow, the
                    // C++ re-queues into `notreached`.  Single-pass: mark complete.
                    if self.data.get_jump_table(jt_idx as int4).is_partial() {
                        self.data.get_jump_table_mut(jt_idx as int4).mark_complete();
                    }
                    new_tables.push(jt_idx);
                }
                Ok(None) => {
                    // Could not recover the jumptable.
                    if !for_inline {
                        self.truncate_indirect_jump(op, RecoveryMode::FailNormal)?;
                    }
                }
                Err(mode) => {
                    // A specific failure mode (thunk / return / callother).
                    if !for_inline {
                        self.truncate_indirect_jump(op, mode)?;
                    }
                }
            }
        }
        Ok(new_tables)
    }

    /// Convert a BRANCHIND that could not be recovered into a call or return
    /// (C++ `FlowInfo::truncateIndirectJump`, `flow.cc:745`).
    fn truncate_indirect_jump(&mut self, op: OpId, mode: RecoveryMode) -> KunaResult<()> {
        if mode == RecoveryMode::FailReturn {
            self.data.op_set_opcode_code(op, OpCode::CPUI_RETURN);
                let addr = self.data.obank().get(op).unwrap().get_addr().clone();
            self.data.warning("Treating indirect jump as return", &addr);
            return Ok(());
        }
        self.data.op_set_opcode_code(op, OpCode::CPUI_CALLIND);
        self.setup_callind_specs(op)?;
        // The no-return / thunk fc bookkeeping (fc->setBadJumpTable / setNoReturn /
        // setInternal) is the W4 FuncCallSpecs surface -- STUB(W4); the op-code
        // change to CALLIND is the load-bearing CFG effect.
        // Create an artificial return after the call.
        let return_type = if mode == RecoveryMode::FailCallother {
            pcodeop_flags::noreturn
        } else {
            0
        };
        let addr = self.data.obank().get(op).unwrap().get_addr().clone();
        // C++ flow.cc:766/773 emits a warning in the callother/normal failure modes.
        match mode {
            RecoveryMode::FailCallother => {
                // fc->setNoReturn(true) is the STUB(W4) FuncCallSpecs surface.
                self.data.warning("Does not return", &addr);
            }
            RecoveryMode::FailThunk => {}
            _ => {
                // fc->setBadJumpTable(true) is the STUB(W4) FuncCallSpecs surface.
                self.data.warning("Treating indirect jump as call", &addr);
            }
        }
        let truncop = self.artificial_halt(&addr, return_type)?;
        self.data.op_dead_insert_after(truncop, op);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // verification / W4-driver support surface
    //
    // The C++ block-build trio (collectEdges/splitBasic/connectBasic) and the
    // per-op classifier (xrefControlFlow) are private members the W4 jump-table
    // and inline waves drive; the inline-clone family copies `visited`/`addrlist`
    // wholesale.  Exposed here as a thin `pub` surface (named after the C++) so
    // the independent verifier can exercise the W3-portable algorithms over
    // hand-built op graphs and the W4 driver can call them without a boundary edit.
    // -----------------------------------------------------------------------

    /// Seed a `visited` entry (C++ `visited[addr] = {seqnum,size}`; the inline
    /// clone family copies the whole map).  Verification / W4-driver support.
    pub fn seed_visited(&mut self, addr: &Address, seqnum: SeqNum, size: int4) {
        self.visited.insert(addr.clone(), VisitStat { seqnum, size });
    }

    /// Has the given instruction address been visited (C++ `seenInstruction`).
    pub fn visited_contains(&self, addr: &Address) -> bool {
        self.seen_instruction(addr)
    }

    /// The recorded instruction byte-length (`VisitStat::size`) for a visited
    /// address (the `Translate::oneInstruction` fall-through step
    /// `process_instruction` stored), or `None` if `addr` was not visited.
    /// Verification / W4-driver support.
    pub fn visited_size(&self, addr: &Address) -> Option<int4> {
        self.visited.get(addr).map(|s| s.size)
    }

    /// Find the fall-thru op for `op` (C++ `fallthruOp`).  Verification support.
    pub fn fallthru_op_for_test(&self, op: OpId) -> Option<OpId> {
        self.fallthru_op(op)
    }

    /// Run the per-op control-flow classifier over the instruction starting at
    /// `start` (C++ `xrefControlFlow`).  Verification / W4-driver support.
    pub fn xref_for_test(
        &mut self,
        start: OpId,
        startbasic: &mut bool,
        isfallthru: &mut bool,
    ) -> KunaResult<Option<OpId>> {
        self.xref_control_flow(Some(start), startbasic, isfallthru)
    }

    /// Build an artificial halt (C++ `artificialHalt`).  Verification support.
    pub fn artificial_halt_for_test(&mut self, addr: &Address, flag: uint4) -> KunaResult<OpId> {
        self.artificial_halt(addr, flag)
    }

    /// Collect basic-block edges (C++ `collectEdges`).  W4-driver / verification.
    pub fn collect_edges_for_test(&mut self) -> KunaResult<()> {
        self.collect_edges()
    }

    /// Split the dead-list ops into basic blocks (C++ `splitBasic`).
    pub fn split_basic_for_test(&mut self) -> KunaResult<()> {
        self.split_basic()
    }

    /// Connect basic blocks by the collected edges (C++ `connectBasic`).
    pub fn connect_basic_for_test(&mut self) {
        self.connect_basic()
    }

    /// Sort + dedup the unprocessed list (C++ `dedupUnprocessed`).
    pub fn dedup_unprocessed_for_test(&mut self) {
        self.dedup_unprocessed()
    }

    /// Plant the artificial halts for every unprocessed address (C++
    /// `fillinBranchStubs`).  Verification support: the out-of-extent subset is
    /// also registered in `visited`, which is what a caller-declared boundary
    /// depends on and what an in-extent gap must NOT get.
    pub fn fillin_branch_stubs_for_test(&mut self) -> KunaResult<()> {
        self.fillin_branch_stubs()
    }

    /// Does the work-stack (`addrlist`) hold an address at the given offset?
    pub fn addrlist_contains(&self, off: u64) -> bool {
        self.addrlist.iter().any(|a| a.get_offset() == off)
    }

    /// Push an address onto the unprocessed list (verification support).
    pub fn push_unprocessed(&mut self, addr: Address) {
        self.unprocessed.push(addr);
    }

    /// The offsets currently in the unprocessed list, in order.
    pub fn unprocessed_offsets(&self) -> Vec<u64> {
        self.unprocessed.iter().map(|a| a.get_offset()).collect()
    }
}

/// An \e invalid `SeqNum` — the C++ default-constructed `SeqNum` whose address is
/// the null `Address` (`SeqNum::getAddr().isInvalid()` is true).  `visited[addr]`
/// seeds its `seqnum` with this until the first op's real SeqNum is recorded.
fn invalid_seqnum() -> SeqNum {
    SeqNum::new(Address::default(), 0)
}

/// A p-code emitter for building PcodeOp objects into a [`Funcdata`] (C++
/// `class PcodeEmitFd`, `funcdata.hh:636`).
///
/// Built per-instruction (the C++ `emitter` member is reused; here a fresh
/// [`FlowEmit`] borrowing `&mut Funcdata` + the [`FlowEnvironment`] is handed to
/// `Translate::one_instruction`).  Each `dump` converts template `VarnodeData`
/// into a real op + its varnodes in the dead list.
///
/// This is now the **real** op-building emitter — output creation
/// (`newVarnodeOut` → `opSetOutput` via the [`Funcdata::banks_mut`] split-borrow),
/// code-reference inputs (`newCodeRef`), and ordinary inputs (`newVarnode`) all
/// build real Varnodes through the ported `funcdata_op`/`funcdata_varnode`
/// factories.  The `PcodeEmit::dump` trait method cannot return a `Result` (it
/// mirrors the C++ `void dump(...)` that throws), so the first factory error is
/// captured in [`FlowEmit::error`] and surfaced by
/// [`process_instruction`](FlowInfo::process_instruction) after the decode call,
/// exactly where the C++ exception would unwind out of `oneInstruction`.
pub struct FlowEmit<'f, 'e, E: FlowEnvironment> {
    /// The Funcdata container to emit into (C++ `PcodeEmitFd::fd`).
    fd: &'f mut Funcdata,
    /// The op-code → TypeOp resolver + anchor predicates (C++ `glb`).
    env: &'e E,
    /// Ops emitted by this instruction, in dump order (the C++ ops land on the
    /// dead list; tracked here so `process_instruction` can find the first).
    emitted: Vec<OpId>,
    /// The first factory error a `dump` hit (the C++ exception out of a `new*`
    /// factory), captured because the `PcodeEmit::dump` signature is infallible.
    error: Option<KunaError>,
}

impl<'f, 'e, E: FlowEnvironment> FlowEmit<'f, 'e, E> {
    fn new(fd: &'f mut Funcdata, env: &'e E) -> FlowEmit<'f, 'e, E> {
        FlowEmit { fd, env, emitted: Vec::new(), error: None }
    }
}

impl<E: FlowEnvironment> PcodeEmit for FlowEmit<'_, '_, E> {
    fn dump(
        &mut self,
        addr: &Address,
        opc: OpCode,
        outvar: Option<&kuna_num::pcoderaw::VarnodeData>,
        vars: &[kuna_num::pcoderaw::VarnodeData],
    ) {
        // Once a factory has errored, stop building (the C++ exception would have
        // unwound; subsequent dumps for the same instruction never run).
        if self.error.is_some() {
            return;
        }
        // Convert template data into a real PcodeOp (C++ PcodeEmitFd::dump).
        let isize = vars.len() as int4;
        let op = if let Some(out) = outvar {
            let op = self.fd.new_op(isize, addr.clone());
            let ospace =
                out.space.as_ref().expect("FlowEmit::dump: output varnode has no space").clone();
            let oaddr = Address::new(ospace, out.offset);
            if let Err(e) = self.fd.new_varnode_out(out.size as int4, &oaddr, op) {
                self.error = Some(e);
                self.emitted.push(op);
                return;
            }
            op
        } else {
            self.fd.new_op(isize, addr.clone())
        };
        let t_op = self.env.resolve_typeop(opc);
        self.fd.op_set_opcode(op, t_op);
        let mut i = 0usize;
        let is_code_ref = self.fd.obank().get(op).expect("FlowEmit::dump: stale op").is_code_ref();
        if is_code_ref {
            let cspace =
                vars[0].space.as_ref().expect("FlowEmit::dump: code-ref varnode has no space").clone();
            let addrcode = Address::new(cspace, vars[0].offset);
            let cref = self.fd.new_code_ref(&addrcode);
            // opSetInput is fallible only on the constant-share guard; an
            // annotation code-ref is not a constant, so it never errors.
            if let Err(e) = self.fd.op_set_input(op, cref, 0) {
                self.error = Some(e);
                self.emitted.push(op);
                return;
            }
            i += 1;
        }
        // The C++ uses the (size, AddrSpace*, offset) `newVarnode` overload, which
        // builds a free Varnode at that storage.  For a LOAD/STORE the slot-0
        // operand is a spaceid constant in the \e constant space whose offset is
        // the encoded AddrSpace (the Rust runtime stores the space INDEX rather
        // than the heap pointer — LOSS-015); `dump` does not special-case it (the
        // C++ does not either) — it is just a constant-space Varnode at that
        // offset, built through the same factory.
        while i < vars.len() {
            let vd = &vars[i];
            let space = vd.space.as_ref().expect("FlowEmit::dump: varnode has no space").clone();
            let vn = self.fd.new_varnode_space_off(vd.size as int4, space, vd.offset);
            // opSetInput is fallible only on the constant-share guard (single
            // descendant); a freshly created varnode has none, so it never errors.
            if let Err(e) = self.fd.op_set_input(op, vn, i as int4) {
                self.error = Some(e);
                self.emitted.push(op);
                return;
            }
            i += 1;
        }
        self.emitted.push(op);
    }
}

/// Free-function `FlowInfo::target` over an explicit `(Funcdata, visited)` pair
/// (C++ `FlowInfo::target`, `flow.cc:117`): map a flow address to the op
/// starting its instruction's basic block, following no-p-code fall-thru.  Used
/// by `switchOverJumpTables` after the FlowInfo's `data` has been moved out.
#[allow(clippy::mutable_key_type)]
pub fn target_in(
    fd: &Funcdata,
    visited: &VisitedMap,
    addr: &Address,
) -> KunaResult<OpId> {
    let mut cur = addr.clone();
    while let Some(stat) = visited.get(&cur) {
        if !stat.seqnum.get_addr().is_invalid() {
            if let Some(retop) = fd.obank().find_op(&stat.seqnum) {
                return Ok(retop);
            }
            break;
        }
        cur = &cur + stat.size as i64;
    }
    Err(KunaError::lowlevel(format!(
        "Could not find op at target address: {addr:?}"
    )))
}

/// Build the basic blocks of a jump-table-recovery partial clone (the
/// `partialflow.generateBlocks()` half of C++ `Funcdata::truncatedFlow`).
///
/// The partial's ops were cloned by `Funcdata::truncated_flow_clone`; this builds
/// a [`FlowInfo`] over it seeded with the source flow's `visited` and runs
/// `generate_blocks` (fillinBranchStubs / collectEdges / splitBasic /
/// connectBasic).  `env` is the same [`FlowEnvironment`] the source flow used
/// (only the `translate`/`resolve_typeop` surface is touched, never to lift —
/// `generate_blocks` reads no env method).
#[allow(clippy::mutable_key_type)]
pub fn build_partial_blocks<E: FlowEnvironment>(
    partial: &mut Funcdata,
    env: &E,
    visited: &VisitedMap,
) -> KunaResult<()> {
    // Move the partial's data into a FlowInfo, build blocks, move it back.
    let placeholder = Funcdata::new_placeholder_like(partial)?;
    let data = std::mem::replace(partial, placeholder);
    let mut flow = FlowInfo::new_seeded(data, env, visited.clone());
    // partialflow.clearFlags(~possible_unreachable) — only possible_unreachable
    // survives; the partial flow has no error flags to clear here.
    let res = flow.generate_blocks();
    *partial = flow.data;
    res?;
    partial.set_flag_raw(crate::funcdata::funcdata_flags::blocks_generated);
    Ok(())
}
