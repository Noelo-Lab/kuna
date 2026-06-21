//! The end-to-end decompilation orchestrator (item `w9x-arch-engine-glue`).
//!
//! Wires the merged subsystems — the [`Architecture`] god object, the
//! [`FlowInfo`] flow engine, the universalAction pipeline
//! ([`crate::universalaction`]), and the [`PrintC`] printer — into a single
//! function-decompilation path, mirroring `decompiler/cpp/ifacedecomp.cc`'s
//! `IfcDecompile` + `IfcPrintC`:
//!
//! ```text
//! IfcDecompile::execute (ifacedecomp.cc:889)
//!   fd->followFlow(...)                         -> generate_ops + generate_blocks
//!   allacts.getCurrent()->reset(*fd)
//!   res = allacts.getCurrent()->perform(*fd)    -> the restart loop
//! IfcPrintC::execute (ifacedecomp.cc:925)
//!   print->docFunction(fd)                      -> PrintC::doc_function shell
//! ```
//!
//! ## What runs end-to-end today (and what seams)
//!
//! * **Flow following is real.**  [`FlowInfo::generate_ops`] (C++
//!   `Funcdata::followFlow` -> `generateOps`) lifts and links every
//!   straight-line instruction's p-code into the `Funcdata`; CALL / jump-table
//!   sites hit the documented W4 `FlowInfo` seams (FuncCallSpecs / JumpTable),
//!   which are no-ops here (faithful partial flow), so `generate_ops` returns
//!   the IR built up to those boundaries rather than erroring.
//! * **The universalAction perform loop is real.**  The 252-pass `decompile`
//!   root is installed and run; the *boot* passes (`ActionStart` -> the C++
//!   `Funcdata::startProcessing`) are W3/W4 seam no-ops in the merged tree
//!   (which is why the flow follow is driven explicitly here, outside the
//!   pipeline, exactly as the C++ `followFlow` runs before `perform`), so the
//!   pass scheduler/status state-machine executes without rebuilding the IR.
//! * **The printer body is the W9-emit seam.**  [`PrintC::doc_function`] emits a
//!   structurally-complete C function *shell* (real signature + matched braces)
//!   driving the real [`Emit`](crate::prettyprint::Emit) primitives; the
//!   per-statement RPN expression body is the `// SEAM(W9-emit)` driver absent
//!   from the merged tree (see `printc.rs`).
//!
//! This proves the full path RUNS and emits plausible C — not byte-parity (the
//! W10 grind), which the e2e gate (`tests/decompile_e2e.rs`) asserts.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::action::ActionContext;
use crate::architecture::Architecture;
use crate::flow::{FlowEnvironment, FlowInfo};
use crate::funcdata::{funcdata_flags, Funcdata};
use crate::seams::TypeOp;

use kuna_sleigh::translate::Translate;

/// A [`FlowEnvironment`] backed by a borrowed [`Architecture`] — the real
/// engine-backed shape the C++ `FlowInfo` uses (`glb->translate` for the
/// decoder, `glb->inst[opc]` for the op-property resolution).
///
/// The override / user-op tables default to "none" (the W4 surfaces the
/// `FlowInfo` trait already defaults); the architecture-owned `inst` table
/// drives `resolve_typeop` so the built ops carry the correct
/// branch/call/coderef/marker property flags.
struct ArchFlowEnv {
    /// Raw pointer to the architecture (read-only use: `translate` / `resolve_
    /// typeop` / `query_call`).  A raw pointer (rather than `&Architecture`) lets
    /// the jump-table recovery hold `&mut Architecture` for the action sub-
    /// pipeline (`allacts`) concurrently: the env's reads (`translate`/`inst`/
    /// `symboltab`) never alias the `allacts` mutation, so the access is sound.
    arch: *const Architecture,
}

impl ArchFlowEnv {
    #[inline]
    fn arch(&self) -> &Architecture {
        // SAFETY: the pointer is created from a live `&mut Architecture` in
        // `build_and_follow_flow` and used only for non-aliasing read methods
        // (`translate`/`resolve_typeop`/`query_call`) for the duration of the
        // flow-follow; the architecture outlives the env.
        unsafe { &*self.arch }
    }
}

impl FlowEnvironment for ArchFlowEnv {
    fn translate(&self) -> &dyn Translate {
        self.arch().translate()
    }
    fn resolve_typeop(&self, opc: OpCode) -> TypeOp {
        self.arch().resolve_typeop(opc)
    }
    fn query_call(&self, entry: &Address) -> Option<String> {
        let arch = self.arch();
        // C++ FlowInfo::queryCall -> getScopeLocal()->getParent()->queryFunction(entry):
        // resolve the callee's display name from the global symbol table (populated
        // by readLoaderSymbols at load).
        let scope = arch.symboltab.get_global_scope()?;
        let sid = arch.symboltab.find_function(scope, entry)?;
        let name = arch.symboltab.symbol(sid).get_display_name();
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }
    fn query_call_no_return(&self, entry: &Address) -> bool {
        // C++ `queryCall` copies the callee proto's `isNoReturn()` flow effect;
        // the flag is set by `option noreturn <name>` (OptionNoReturn) on the
        // resolved FunctionSymbol.
        let arch = self.arch();
        match arch.symboltab.get_global_scope() {
            Some(scope) => arch.symboltab.function_is_no_return(scope, entry),
            None => false,
        }
    }
    fn query_call_inline(&self, entry: &Address) -> bool {
        // C++ `queryCall` copies the callee proto's `isInline()` flow effect; the
        // flag is set by `option inline <name>` (OptionInline) on the resolved
        // FunctionSymbol.
        let arch = self.arch();
        match arch.symboltab.get_global_scope() {
            Some(scope) => arch.symboltab.function_is_inline(scope, entry),
            None => false,
        }
    }
    fn query_call_inject_id(&self, entry: &Address) -> int4 {
        // The callee's parked inject id (IfcFixupApply); -1 for none.
        let arch = self.arch();
        match arch.symboltab.get_global_scope() {
            Some(scope) => arch.symboltab.function_inject_id(scope, entry),
            None => -1,
        }
    }
    fn build_inline_funcdata(&self, entry: &Address) -> KunaResult<Option<Funcdata>> {
        // C++ `Funcdata::inlineFlow` builds a fresh FlowInfo over the queried
        // callee Funcdata (after clearAnalysis).  Resolve the callee symbol's
        // name (the C++ `queryFunction(entry)` -> Funcdata), then build a fresh
        // Funcdata at that entry through the engine's `new_funcdata` factory.  No
        // callee symbol -> no inline (the C++ `fd == 0` short-circuit).
        let arch = self.arch();
        let scope = match arch.symboltab.get_global_scope() {
            Some(s) => s,
            None => return Ok(None),
        };
        let sid = match arch.symboltab.find_function(scope, entry) {
            Some(s) => s,
            None => return Ok(None),
        };
        let name = arch.symboltab.symbol(sid).get_display_name().to_string();
        // size 0: unbounded natural extent — inlineFlow sets its own flow range
        // (the full entry-space) before generating ops.
        let fd = arch.new_funcdata(&name, entry.clone(), 0)?;
        Ok(Some(fd))
    }

    fn is_injected_userop(&self, userop_index: kuna_base::types::uintb) -> bool {
        // C++ `glb->userops.getOp(in0)->getType() == UserPcodeOp::injected`.
        let arch = self.arch();
        match arch.userops.get_op(userop_index as kuna_base::types::uint4) {
            Some(op) => op.get_type() == crate::userop::userop_type::injected,
            None => false,
        }
    }

    fn is_incidental_copy_userop(&self, userop_index: kuna_base::types::uintb) -> bool {
        // C++ `payload->isIncidentalCopy()` for the user-op's injection payload.
        let arch = self.arch();
        let injectid = match arch
            .userops
            .get_op(userop_index as kuna_base::types::uint4)
            .and_then(|op| op.get_inject_id())
        {
            Some(id) => id as int4,
            None => return false,
        };
        arch.pcodeinjectlib.get_payload(injectid).core().is_incidental_copy()
    }

    fn inject_userop(
        &self,
        userop_index: kuna_base::types::uintb,
        context: &mut crate::pcodeinject::InjectContext,
        emit: &mut dyn kuna_sleigh::translate::PcodeEmit,
    ) -> KunaResult<()> {
        // C++ `FlowInfo::injectUserOp`'s emit step:
        //   InjectedUserOp *userop = (InjectedUserOp *)glb->userops.getOp(idx);
        //   InjectPayload *payload = glb->pcodeinjectlib->getPayload(userop->getInjectId());
        //   payload->inject(icontext, emitter);
        let arch = self.arch();
        let injectid = arch
            .userops
            .get_op(userop_index as kuna_base::types::uint4)
            .and_then(|op| op.get_inject_id())
            .ok_or_else(|| KunaError::lowlevel("inject_userop: user-op is not injected"))?
            as int4;
        let payload = arch.pcodeinjectlib.get_payload(injectid);
        let tpl = arch.pcodeinjectlib.get_tpl(injectid).ok_or_else(|| {
            // The callother-fixup body never compiled (parse_inject_all was not
            // driven, or the SLEIGH compile failed): cannot inject.
            KunaError::lowlevel("inject_userop: callother-fixup template not compiled")
        })?;
        // SleighInjectEngine over the arch's const/unique/default-code spaces.
        let engine = crate::inject_sleigh::SleighInjectEngine::new(
            Rc::clone(arch.manage().get_constant_space().expect("no constant space")),
            Rc::clone(arch.manage().get_unique_space().expect("no unique space")),
            Rc::clone(arch.manage().get_default_code_space().expect("no default code space")),
        );
        engine.emit_payload(payload, tpl, context, emit)
    }

    fn inject_call_fixup_payload(
        &self,
        inject_id: int4,
        context: &mut crate::pcodeinject::InjectContext,
        emit: &mut dyn kuna_sleigh::translate::PcodeEmit,
    ) -> KunaResult<()> {
        // C++ `FlowInfo::injectSubFunction`'s emit step:
        //   InjectPayload *payload = glb->pcodeinjectlib->getPayload(fc->getInjectId());
        //   payload->inject(icontext, emitter);
        let arch = self.arch();
        let payload = arch.pcodeinjectlib.get_payload(inject_id);
        let tpl = arch.pcodeinjectlib.get_tpl(inject_id).ok_or_else(|| {
            // The call-fixup body never compiled (parse_inject_all was not driven,
            // or the SLEIGH compile failed): cannot inject.
            KunaError::lowlevel("inject_call_fixup_payload: call-fixup template not compiled")
        })?;
        let engine = crate::inject_sleigh::SleighInjectEngine::new(
            Rc::clone(arch.manage().get_constant_space().expect("no constant space")),
            Rc::clone(arch.manage().get_unique_space().expect("no unique space")),
            Rc::clone(arch.manage().get_default_code_space().expect("no default code space")),
        );
        engine.emit_payload(payload, tpl, context, emit)
    }

    fn is_incidental_copy_payload(&self, inject_id: int4) -> bool {
        let arch = self.arch();
        arch.pcodeinjectlib.get_payload(inject_id).core().is_incidental_copy()
    }

    fn call_fixup_param_shift(&self, inject_id: int4) -> int4 {
        let arch = self.arch();
        arch.pcodeinjectlib.get_payload(inject_id).core().get_param_shift()
    }

    fn call_fixup_name(&self, inject_id: int4) -> String {
        let arch = self.arch();
        String::from_utf8_lossy(&arch.pcodeinjectlib.base.get_call_fixup_name(inject_id)).into_owned()
    }

    fn build_override_proto(
        &self,
        pieces: &crate::fspec::PrototypePieces,
    ) -> KunaResult<Option<crate::fspec::FuncProto>> {
        // C++ `IfcProtooverride`: `new FuncProto; setInternal(pieces.model,
        // getTypeVoid()); setPieces(pieces)`.  The pieces carry no model
        // back-pointer in the merged tree (// SEAM(w6-fspec-2)), so use the default
        // model (the console parses `override prototype` against the default model
        // unless a `__model` was named — not exercised by the corpus).
        let arch = self.arch();
        let model = match arch.default_fp() {
            Some(m) => Rc::clone(m),
            None => return Ok(None),
        };
        let void = arch.types().get_type_void()?;
        let mut proto = crate::fspec::FuncProto::new();
        proto.set_internal(model.clone(), void);
        proto.set_pieces(pieces, Some(model), arch.types(), arch.manage())?;
        // insertProtoOverride marked this an override (locked); `set_pieces` already
        // sets the input/output/model locks.  Flag the override bit so the call
        // spec's `isOverride()` short-circuit (deindirect proto-merge) sees it.
        proto.set_override(true);
        Ok(Some(proto))
    }
}

/// Build a [`Funcdata`] for the function `name` at `entry` and follow its flow,
/// returning the IR-populated `Funcdata` (C++ `Funcdata` construction +
/// `Funcdata::followFlow`).
///
/// The Funcdata is built through [`Architecture::new_funcdata`] (the W3 boot
/// seam: it carries the IR-boundary address-space slice + the analysis
/// unique-start).  Flow following runs the real [`FlowInfo`] against an
/// [`ArchFlowEnv`]; on success the `processing_started` flag is set so the
/// printer's `isProcStarted` gate (and the pipeline's resume bookkeeping) see a
/// started function.
#[allow(clippy::mutable_key_type)]
pub fn build_and_follow_flow(
    arch: &mut Architecture,
    name: &str,
    entry: Address,
    size: int4,
) -> KunaResult<Funcdata> {
    build_and_follow_flow_with_override(arch, name, entry, size, &[])
}

/// Like [`build_and_follow_flow`], but seeds the fresh `Funcdata`'s per-function
/// flow-`Override` (C++ `data.getOverride()`) before flow follows — so the console
/// `override flow <addr> <type>` command (which sets the override on the function
/// before `decompile` rebuilds the IR) takes effect at flow time
/// (`FlowInfo::process` reads `hasFlowOverride`/`getFlowOverride` then
/// `Funcdata::overrideFlow`).
///
/// `flow_overrides` are the `(address, flow_type)` facts the console stashed; they
/// are re-inserted into the fresh Funcdata's `localoverride` (the C++ override is
/// kept on the reused Funcdata, but the kuna console rebuilds the IR — see the
/// `pending_prototypes`/`mapped_symbols` re-seed precedent).
///
/// Takes `&mut Architecture` because the jump-table recovery pipeline (below)
/// holds a `*mut Architecture` to drive the `allacts` sub-pipeline concurrently
/// with the env's read-only `*const Architecture` (see [`ArchFlowEnv`]).
#[allow(clippy::mutable_key_type)]
pub fn build_and_follow_flow_with_override(
    arch: &mut Architecture,
    name: &str,
    entry: Address,
    size: int4,
    flow_overrides: &[(Address, kuna_base::types::uint4)],
) -> KunaResult<Funcdata> {
    build_and_follow_flow_with_override_and_protos(arch, name, entry, size, flow_overrides, &[])
}

/// Like [`build_and_follow_flow_with_override`], but also re-seeds the
/// `override prototype <addr> <decl>` facts onto the fresh `Funcdata`'s
/// `localoverride` (the proto-override store) before flow follows, so
/// `FlowInfo::build_call_specs` applies them (`Override::applyPrototype`).  The
/// override survives the deindirect-driven `clear()` + re-flow, so a prototype
/// forced onto an injected CALLIND (the `injectoverride` corpus case) takes
/// effect on the restart re-flow.
#[allow(clippy::mutable_key_type)]
pub fn build_and_follow_flow_with_override_and_protos(
    arch: &mut Architecture,
    name: &str,
    entry: Address,
    size: int4,
    flow_overrides: &[(Address, kuna_base::types::uint4)],
    proto_overrides: &[(Address, crate::fspec::PrototypePieces)],
) -> KunaResult<Funcdata> {
    let mut fd = arch.new_funcdata(name, entry, size)?;
    for (addr, ty) in flow_overrides {
        fd.get_override_mut().insert_flow_override(addr.clone(), *ty);
    }
    for (callpoint, pieces) in proto_overrides {
        let ov: Box<dyn crate::overrides::FuncProtoOverride> =
            Box::new(crate::overrides::PiecesProtoOverride { pieces: pieces.clone() });
        fd.get_override_mut().insert_proto_override(callpoint.clone(), ov);
    }
    follow_flow_on_fd(arch, fd)
}

/// Follow flow for an (already-constructed) `fd`, returning the IR-populated
/// `Funcdata`.  Shared by the fresh build ([`build_and_follow_flow_with_override`])
/// and the restart re-flow ([`refollow_flow`], C++ `ActionStart::startProcessing`
/// → `followFlow` after `clearAnalysis`): both run the same
/// `generateOps`/`generateBlocks`/`startProcessing` sequence; the only difference
/// is whether `fd` is brand-new or just `clear()`ed (its per-function `Override`
/// — including the indirect override a prior `deindirect` installed — survives the
/// clear, so the re-flow rebuilds the CALLIND straight as a direct CALL).
#[allow(clippy::mutable_key_type)]
fn follow_flow_on_fd(arch: &mut Architecture, fd: Funcdata) -> KunaResult<Funcdata> {
    let env = ArchFlowEnv { arch: arch as *const Architecture };
    let mut flow = FlowInfo::new(fd, &env);
    // C++ followFlow: generateOps() then generateBlocks().  The jump-table
    // recovery loop runs inside generateOps (via the action sub-pipeline).
    {
        let arch_ptr: *mut Architecture = arch;
        let mut run_jt_pipeline = |partial: &mut Funcdata,
                                   visited: &crate::flow::VisitedMap|
         -> KunaResult<()> {
            // SAFETY: `arch_ptr` aliases the live `&mut Architecture`; the env's
            // reads (`translate`/`inst`/`symboltab`) are disjoint from the
            // `allacts` mutation here, and the `flow` borrow of `env`/`arch` does
            // not overlap this closure's run (it is only active between calls).
            let arch_mut: &mut Architecture = unsafe { &mut *arch_ptr };
            run_jumptable_pipeline(arch_mut, partial, visited)
        };
        flow.generate_ops_with_jumptables(&mut run_jt_pipeline)?;
    }
    flow.generate_blocks()?;
    // C++ followFlow: switchOverJumpTables(flow) — map each recovered table's
    // addresses to the basic-block out-edges (the `target` surface is
    // `FlowInfo::target`).  Drive it before the FlowInfo is consumed.
    let target_snapshot = flow.target_index_snapshot();
    let mut data = flow.data;
    // Flush the analysis comments buffered during flow follow (C++
    // `Funcdata::warning`/`warningHeader` write straight to `glb->commentdb`; the
    // merged Rust tree buffers them on the `Funcdata` because the console owns the
    // comment database, so re-deposit them now that `&mut Architecture` is in
    // hand — the same re-seed model as `mapped_symbols`/`pending_prototypes`).
    let func_addr = data.get_address().clone();
    for (tp, ad, txt) in data.drain_pending_comments() {
        arch.commentdb.add_comment_no_duplicate(tp, &func_addr, &ad, &txt);
    }
    data.switch_over_jump_tables(|fd, addr| {
        crate::flow::target_in(fd, &target_snapshot, addr)
    })?;
    // C++ `Funcdata::startProcessing` (funcdata.cc:150) runs after `followFlow`:
    // it calls `structureReset()` — which builds the basic-block reverse-post
    // ordering AND the forward dominator tree (`bblocks.calcForwardDominator`).
    // `ActionHeritage::buildADT` *requires* that dominator tree, so the reset is
    // part of the heritage-application prerequisite (the merged tree's
    // `ActionStart` is a seam, so it is driven here, exactly as the C++
    // `followFlow`→`startProcessing` order runs it before the action pipeline).
    data.structure_reset();
    // C++ startProcessing also calls sortCallSpecs() (dominance order for the
    // call-spec list); now that qlst exists, sort it.
    data.sort_call_specs();
    // startProcessing then sets the processing_started flag (so isProcStarted()
    // is true; the rest of startProcessing — sortCallSpecs / buildInfoList /
    // applyDeadCodeDelay — is W4 seam or handled lazily in op_heritage).
    data.set_flag_raw(funcdata_flags::processing_started);
    Ok(data)
}

/// Run the reduced "jumptable" universalAction on a partial-clone Funcdata (the
/// `partial.truncatedFlow` block-build + `allacts.setCurrent("jumptable")` +
/// reset + perform of C++ `Funcdata::stageJumpTable`, funcdata_block.cc:512).
///
/// The partial already has its ops + jump-tables cloned; this builds its basic
/// blocks (seeded with the source flow's `visited`), runs `structureReset` +
/// `sortCallSpecs` (the `startProcessing` prerequisites), then drives the
/// "jumptable" action set to simplify it so the BRANCHIND's switch calculation
/// becomes a straight-line index expression the recovery can emulate.
#[allow(clippy::mutable_key_type)]
fn run_jumptable_pipeline(
    arch: &mut Architecture,
    partial: &mut Funcdata,
    visited: &crate::flow::VisitedMap,
) -> KunaResult<()> {
    // Build the partial's basic blocks (partialflow.generateBlocks).
    let env = ArchFlowEnv { arch: arch as *const Architecture };
    crate::flow::build_partial_blocks(partial, &env, visited)?;
    // startProcessing prerequisites for heritage (forward dominators + RPO).
    partial.structure_reset();
    partial.sort_call_specs();
    partial.set_flag_raw(funcdata_flags::processing_started);
    // Run the reduced "jumptable" universalAction root over the partial.
    let saved = arch.allacts.get_current_name().to_string();
    arch.allacts.set_current("jumptable")?;
    let mut ctx = ActionContext::new();
    let result = {
        let root = arch
            .allacts
            .get_current_mut()
            .ok_or_else(|| KunaError::lowlevel("no current jumptable action"))?;
        root.reset(partial);
        // catch_unwind so an un-ported pass seam in the sub-pipeline degrades to a
        // recoverable error (the recovery falls back to truncating the BRANCHIND),
        // never an abort — same policy as the main `decompile_func_full` drive.
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            root.perform(partial, &mut ctx)
        }))
    };
    // Restore the previous action set regardless of outcome.
    let _ = arch.allacts.set_current(&saved);
    match result {
        Ok(r) => {
            if r < 0 {
                return Err(KunaError::lowlevel("jumptable pipeline hit a breakpoint"));
            }
            Ok(())
        }
        Err(payload) => Err(KunaError::lowlevel(format!(
            "jumptable pipeline reached an un-ported seam: {}",
            panic_message(&payload)
        ))),
    }
}

/// Run the `decompile` universalAction root to completion against `fd` (C++
/// `IfcDecompile::execute`, ifacedecomp.cc:889 — `getCurrent()->reset(fd)` then
/// `getCurrent()->perform(fd)`).  Returns the perform result (`>=0` change
/// count, `<0` on a breakpoint).
///
/// The action root owns the restart loop (C++ non-virtual `Action::perform`,
/// already ported in `action.rs`); this drives it through the engine-side
/// [`ActionContext`] so the warning-emission points are observable.
fn run_pipeline(arch: &mut Architecture, fd: &mut Funcdata) -> KunaResult<int4> {
    // C++ allacts.setCurrent("decompile") derives the filtered decompile root
    // (idempotent if already current).
    arch.allacts.set_current("decompile")?;
    // The action loop's in-loop restart cannot re-follow flow (no SLEIGH
    // translator inside `allacts`); it hands a pending restart up here via
    // `ctx.reflow_requested`.  This outer loop owns `&mut Architecture`, so it can
    // `clear()` + re-`followFlow` the IR (C++ `clearAnalysis` + `ActionStart` →
    // `startProcessing`) and re-perform the root — exactly the C++ restart
    // semantics, just relocated out of the borrow-restricted action loop.  Bound
    // the iterations defensively (the action layer's own `maxrestarts` already
    // caps in-loop restarts; this caps cross-flow restarts).
    const MAX_REFLOW: int4 = 8;
    let mut total: int4 = 0;
    for _ in 0..=MAX_REFLOW {
        let res = {
            let root = arch
                .allacts
                .get_current_mut()
                .ok_or_else(|| kuna_base::error::KunaError::lowlevel("no current action"))?;
            root.reset(fd);
            let mut ctx = ActionContext::new();
            let r = root.perform(fd, &mut ctx);
            (r, ctx.reflow_requested)
        };
        let (r, reflow_requested) = res;
        if r < 0 {
            return Ok(r); // breakpoint — propagate verbatim (no re-flow)
        }
        total += r;
        if !(reflow_requested && fd.has_restart_pending()) {
            return Ok(total);
        }
        // Re-follow flow against the same Funcdata (its Override — incl. the
        // deindirect's indirect override — survives `clear()`), then loop to
        // re-perform.  `clear()` resets the restart flag, so a fresh request must
        // come from the re-flowed pass to loop again.
        refollow_flow(arch, fd)?;
    }
    // Exceeded the cross-flow restart budget; keep the last analyzed IR.
    fd.set_restart_pending(false);
    Ok(total)
}

/// Re-follow flow on an existing, already-analyzed `fd` after a restart request
/// (C++ `ActionRestartGroup` → `clearAnalysis` + `ActionStart::startProcessing` →
/// `followFlow`).  Clears the analysis-derived IR (`Funcdata::clear`) while
/// preserving the per-function `Override`, then re-runs the flow follow in place.
#[allow(clippy::mutable_key_type)]
fn refollow_flow(arch: &mut Architecture, fd: &mut Funcdata) -> KunaResult<()> {
    // C++ `glb->clearAnalysis(&data)` == fd->clear() (the comment-DB clear is the
    // W4 seam) — drops ops/vbank/blocks/qlst/heritage but keeps `localoverride`.
    fd.clear();
    // Move the cleared shell out so `follow_flow_on_fd` can re-flow it (it takes
    // `fd` by value, mirroring the fresh-build path), then put the re-flowed IR
    // back into the caller's slot.  The throwaway placeholder is a minimal valid
    // Funcdata at the same entry (so `new_funcdata`'s space lookup succeeds); it is
    // immediately overwritten by the re-flowed result.
    let entry = fd.get_address().clone();
    let placeholder = arch.new_funcdata("", entry, 0)?;
    let shell = std::mem::replace(fd, placeholder);
    *fd = follow_flow_on_fd(arch, shell)?;
    Ok(())
}

/// Decompile the function `name` at `funcaddr` to a ready-to-print
/// [`Funcdata`] (C++ `IfcDecompile`): build the IR (follow flow), install the
/// universalAction `decompile` root, and run the pass pipeline to completion.
///
/// `size` bounds the flow follow (0 = unbounded, the function's natural extent).
/// On success the returned `Funcdata` has its IR built and the pipeline run; it
/// is ready for [`print_c`].  The universalAction must already be installed on
/// the architecture (via [`Architecture::build_action`] / `init_post_engine`).
pub fn decompile_func(
    arch: &mut Architecture,
    name: &str,
    funcaddr: Address,
    size: int4,
) -> KunaResult<Funcdata> {
    decompile_func_with_symbols(arch, name, funcaddr, size, &[])
}

/// Like [`decompile_func`], but seeds the freshly-built `Funcdata`'s local scope
/// with console-mapped Symbol specs (`map addr`).  The kuna console rebuilds the
/// IR on `decompile` (C++ reuses the same `fd`); this carries the `map addr`
/// symbols across that rebuild so stack-variable promotion can name them.
pub fn decompile_func_with_symbols(
    arch: &mut Architecture,
    name: &str,
    funcaddr: Address,
    size: int4,
    mapped_symbols: &[(String, std::rc::Rc<crate::dtype::Datatype>, Address, kuna_base::types::uint4)],
) -> KunaResult<Funcdata> {
    decompile_func_full(arch, name, funcaddr, size, mapped_symbols, None)
}

/// The full decompile drive: like [`decompile_func_with_symbols`] but also
/// applies a parsed-and-locked input/output prototype (`parse line extern
/// <decl>`) to the fresh `Funcdata` before the pipeline runs (C++
/// `Architecture::setPrototype` on the queried `Funcdata`).
///
/// The console captures the [`PrototypePieces`](crate::fspec::PrototypePieces)
/// at `parse line` and stashes them by name; the decompile rebuilds the IR, so
/// the lock must be re-applied to the fresh `funcp` here — the seed that lets
/// `ActionPrototypeTypes` force the typed input/output Varnodes and the type
/// plane (`ActionInferTypes`) flow from them.
pub fn decompile_func_full(
    arch: &mut Architecture,
    name: &str,
    funcaddr: Address,
    size: int4,
    mapped_symbols: &[(String, std::rc::Rc<crate::dtype::Datatype>, Address, kuna_base::types::uint4)],
    pending_proto: Option<&crate::fspec::PrototypePieces>,
) -> KunaResult<Funcdata> {
    decompile_func_full_with_override(arch, name, funcaddr, size, mapped_symbols, pending_proto, &[])
}

/// Like [`decompile_func_full`], but also seeds the per-function flow `Override`
/// (`override flow <addr> <type>`) before flow follows — see
/// [`build_and_follow_flow_with_override`].
#[allow(clippy::too_many_arguments)]
pub fn decompile_func_full_with_override(
    arch: &mut Architecture,
    name: &str,
    funcaddr: Address,
    size: int4,
    mapped_symbols: &[(String, std::rc::Rc<crate::dtype::Datatype>, Address, kuna_base::types::uint4)],
    pending_proto: Option<&crate::fspec::PrototypePieces>,
    flow_overrides: &[(Address, kuna_base::types::uint4)],
) -> KunaResult<Funcdata> {
    decompile_func_full_with_override_dyn(
        arch,
        name,
        funcaddr,
        size,
        mapped_symbols,
        &[],
        &[],
        pending_proto,
        flow_overrides,
        &[],
        &[],
    )
}

/// Like [`decompile_func_full_with_override`], but also re-seeds the console-added
/// dynamic (`map hash`) symbols (`(name, type, hashAddr, hash)`) into the rebuilt
/// local scope, so `ActionDynamicSymbols` can name the matched temporaries.
#[allow(clippy::too_many_arguments)]
pub fn decompile_func_full_with_override_dyn(
    arch: &mut Architecture,
    name: &str,
    funcaddr: Address,
    size: int4,
    mapped_symbols: &[(String, std::rc::Rc<crate::dtype::Datatype>, Address, kuna_base::types::uint4)],
    usepoint_symbols: &[(String, std::rc::Rc<crate::dtype::Datatype>, Address, kuna_base::types::uint4, Address, bool)],
    dynamic_symbols: &[crate::database::DynamicSymbolSpec],
    pending_proto: Option<&crate::fspec::PrototypePieces>,
    flow_overrides: &[(Address, kuna_base::types::uint4)],
    proto_overrides: &[(Address, crate::fspec::PrototypePieces)],
    mapped_params: &[(int4, String, crate::fspec::ParameterPieces)],
) -> KunaResult<Funcdata> {
    let mut fd =
        build_and_follow_flow_with_override_and_protos(arch, name, funcaddr, size, flow_overrides, proto_overrides)?;
    // Apply any parsed-and-locked prototype to the fresh funcp (the input-param
    // recovery SEED): after this the inputs/output are type-locked, so
    // ActionPrototypeTypes forces the typed Varnodes.
    if let Some(pieces) = pending_proto {
        fd.apply_locked_prototype(pieces)?;
    }
    // Re-seed any console `map param <i> <addr> <typedecl>` storage locks (lost
    // when the IR is rebuilt, like `pending_proto`/`mapped_symbols`).  This makes
    // the rebuilt proto input-locked so `ActionPrototypeTypes` forces the typed
    // input Varnode (C++ `IfcMapParam` writes straight onto the live FuncProto).
    fd.apply_mapped_params(mapped_params);
    // Re-seed the console-mapped symbols (lost when the IR is rebuilt).
    fd.seed_mapped_symbols(mapped_symbols);
    // Re-seed the usepoint-scoped console symbols (the register-storage
    // `type varnode %REG(pc)` symbols, e.g. retstruct's `tmp`) WITH their use
    // address so `linkSymbol`'s usepoint query binds them at the scoped read.
    fd.seed_usepoint_symbols(usepoint_symbols);
    // Re-seed the console-added dynamic (`map hash`) symbols (likewise lost).
    fd.seed_dynamic_symbols(dynamic_symbols);
    // With the single-manager unification (LOSS-132) the universalAction passes
    // now reach the *real* lifted varnodes, so the pipeline genuinely executes
    // heritage / simplification / merge / … on live IR.  Some pass BODIES are
    // still un-ported seams (LOSS-131, the M3 grind): a hand-built fixture never
    // reached them, but a real corpus function can hit, e.g.,
    // `Heritage::normalizeWriteSize`'s PIECE-concat path.  Those seams abort via
    // `unimplemented_seam` (a deliberate `#[cold] panic!`).  Convert such a
    // seam-abort into a recoverable `Err` at this orchestration boundary so the
    // end-to-end harnesses degrade to the documented "honest partial parity"
    // (the pipeline ran; a body declined at a seam) instead of taking down the
    // whole run — exactly the graceful-degradation the LOSS-130/131 measurement
    // assumes.  `fd`/`arch` are discarded on the unwind, so no half-mutated
    // state escapes (`AssertUnwindSafe` is sound here for that reason).
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run_pipeline(arch, &mut fd)));
    match res {
        Ok(r) => {
            r?;
            Ok(fd)
        }
        Err(payload) => {
            let msg = panic_message(&payload);
            Err(kuna_base::error::KunaError::lowlevel(format!(
                "decompile pipeline reached an un-ported seam (LOSS-131): {msg}"
            )))
        }
    }
}

/// (kuna) Build the IR for `name` and run a **named reduced pipeline** variant
/// over it as a sub-query (C++ `IfcKunaPipeline::execute`).
///
/// Mirrors [`decompile_func_full_with_override_dyn`] but installs the named
/// action group (`normalize`/`paramid`/`register`/`firstpass`/`jumptable`)
/// instead of `decompile`, runs it once (no cross-flow restart loop — a reduced
/// variant has no restart group), then restores `decompile` as the current root
/// (the C++ save/switch/perform/restore around `allacts.setCurrent`).  The
/// resulting [`Funcdata`] holds whatever IR the reduced pipeline produced (for
/// `normalize`, no `sblocks` — `quality` then hits its `hasNoStructBlocks`
/// guard).  Seam aborts degrade to a recoverable `Err`, like the full drive.
pub fn run_named_pipeline_variant(
    arch: &mut Architecture,
    name: &str,
    funcaddr: Address,
    size: int4,
    variant: &str,
) -> KunaResult<Funcdata> {
    let mut fd = build_and_follow_flow(arch, name, funcaddr, size)?;
    let saved = arch.allacts.get_current_name().to_string();
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        arch.allacts.set_current(variant)?;
        let mut ctx = ActionContext::new();
        let root = arch
            .allacts
            .get_current_mut()
            .ok_or_else(|| KunaError::lowlevel(format!("no current {variant} action")))?;
        root.reset(&mut fd);
        let r = root.perform(&mut fd, &mut ctx);
        if r < 0 {
            return Err(KunaError::lowlevel(format!(
                "{variant} pipeline hit a breakpoint"
            )));
        }
        Ok(())
    }));
    // Restore the root action regardless of outcome (C++ restores setCurrent).
    let _ = arch.allacts.set_current(&saved);
    match res {
        Ok(Ok(())) => Ok(fd),
        Ok(Err(e)) => Err(e),
        Err(payload) => Err(KunaError::lowlevel(format!(
            "{variant} pipeline reached an un-ported seam: {}",
            panic_message(&payload)
        ))),
    }
}

/// Best-effort extraction of a panic payload's message (the `panic!` string),
/// for surfacing an un-ported-seam abort as a recoverable [`KunaError`].
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "panic with non-string payload".to_string()
    }
}

/// Render `fd` to C text (C++ `IfcPrintC::execute` -> `print->docFunction(fd)`).
///
/// Drives [`PrintC::doc_function_full`] over the analyzed [`Funcdata`]: the
/// signature shell (recovered return type) plus the **structured-block body**
/// (the if/else hierarchy + per-statement RPN expressions) when `sblocks` is
/// present.
///
/// ## The W10 (`w10-structure-printbody`) closure
///
/// `ActionBlockStructure` now seeds `sblocks` (the cross-arena `build_copy` +
/// `CollapseStructure`), `ActionMarkExplicit`/`ActionMarkImplied` classify the
/// Varnodes, and the IR-coupled body driver
/// ([`PrintC::emit_function_body`](crate::printc::PrintC::emit_function_body))
/// walks the structured tree emitting real statements (`if (cond) { … }`,
/// assignments, `return`) through the ported RPN engine.  The remaining gap to
/// full byte-parity is the **next analysis layer**, not the printer:
///
///   * the recovered local **names** (`v1`) need Merge/HighVariable + the naming
///     pass (a Varnode with no bound Symbol falls back to its register / global
///     `dat_<addr>` name here — faithful `pushVnExplicit`);
///   * the **comparison/branch direction** (`dat_52 <= 10` vs the un-joined
///     `10 < dat_52`) needs `ActionNodeJoin`/`ConditionalJoin` + the
///     present-compare canonicalization to collapse the two-compare boolean
///     pattern into one `INT_LESSEQUAL`;
///   * the return-type / local-decl text needs the proto store + symbol scope.
///
/// The structure of the body — the if/else hierarchy, the statement sequence,
/// the operator expressions — is fully driven here and generalizes across the
/// corpus (real `if` statements now emit for boolless / ccmp / condconst /
/// condexesub / skipnext2 / promotecompare).
pub fn print_c(arch: &mut Architecture, fd: &Funcdata) -> String {
    // Drive the IR-coupled body emitter (C++ `IfcPrintC::execute` ->
    // `print->docFunction(fd)`): the real signature (recovered return type) plus
    // the structured-block body (the if/else hierarchy + per-statement RPN
    // expressions) when `sblocks` is present.  `doc_function_full` needs both the
    // printer (`arch.print_mut()`) and the architecture (for register-name
    // resolution); split the borrows by moving the printer out, driving it, and
    // moving it back (the printer is owned by `arch`).
    let mut printer = arch.take_print();
    let out = printer.doc_function_full(fd, arch);
    arch.put_print(printer);
    out
}
