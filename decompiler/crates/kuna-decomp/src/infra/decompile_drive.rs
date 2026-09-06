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
//! ## What runs end-to-end today (and what stubs out)
//!
//! * **Flow following is real.**  [`FlowInfo::generate_ops`] (C++
//!   `Funcdata::followFlow` -> `generateOps`) lifts and links every
//!   straight-line instruction's p-code into the `Funcdata`; CALL / jump-table
//!   sites hit the documented W4 `FlowInfo` stubs (FuncCallSpecs / JumpTable),
//!   which are no-ops here (faithful partial flow), so `generate_ops` returns
//!   the IR built up to those boundaries rather than erroring.
//! * **The universalAction perform loop is real.**  The 252-pass `decompile`
//!   root is installed and run; the *boot* passes (`ActionStart` -> the C++
//!   `Funcdata::startProcessing`) are W3/W4 stub no-ops in the merged tree
//!   (which is why the flow follow is driven explicitly here, outside the
//!   pipeline, exactly as the C++ `followFlow` runs before `perform`), so the
//!   pass scheduler/status state-machine executes without rebuilding the IR.
//! * **The printer body is the W9-emit stub.**  [`PrintC::doc_function`] emits a
//!   structurally-complete C function *shell* (real signature + matched braces)
//!   driving the real [`Emit`](crate::prettyprint::Emit) primitives; the
//!   per-statement RPN expression body is the `// STUB(W9-emit)` driver absent
//!   from the merged tree (see `printc.rs`).
//!
//! This proves the full path RUNS and emits plausible C — not byte-parity (the
//! W10 grind), which the e2e gate (`tests/decompile_e2e.rs`) asserts.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::action::ActionContext;
use crate::architecture::Architecture;
use crate::flow::{FlowEnvironment, FlowInfo};
use crate::funcdata::{funcdata_flags, Funcdata};
use crate::context::{HighVariableId, TypeOp};

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
        // (kuna, Phase 3) ghidra-mode: the callee name comes from the host's
        // getMappedSymbols answer through the lazy RemoteScope (query-through);
        // the live symboltab is empty in that mode.  Standalone: no provider
        // installed, fall through to the symbol table unchanged.
        if let Some(remote) = &arch.remote_scope {
            if let Some(facts) = remote.function_at(entry) {
                // The DISPLAY form: callee tokens print the label when the host
                // sent one (the raw name is the Java-side identity only).
                return Some(facts.display_name);
            }
        }
        // C++ FlowInfo::queryCall -> getScopeLocal()->getParent()->queryFunction(entry):
        // resolve the callee's display name from the symbol table (populated by
        // readLoaderSymbols + the analysis passes at load).  Resolved across scopes
        // so a namespaced (demangled) callee like `foo::Bar::baz` renders its
        // qualified name instead of `sub_<addr>` (C++ queryFunction spans the
        // scope tree; kuna's per-scope maptable otherwise hides it).
        arch.symboltab.function_display_name_across_scopes(entry)
    }
    fn query_call_no_return(&self, entry: &Address) -> bool {
        // C++ `queryCall` copies the callee proto's `isNoReturn()` flow effect;
        // the flag is set by `option noreturn <name>` (OptionNoReturn) or the
        // no-return analysis pass on the resolved FunctionSymbol.
        let arch = self.arch();
        // (kuna, Phase 3) ghidra-mode: `Function.hasNoReturn()` rides the
        // `<function noreturn>` attribute of the getMappedSymbols answer —
        // upstream carries it on the queried callee's FuncProto the same way.
        if let Some(remote) = &arch.remote_scope {
            if let Some(facts) = remote.function_at(entry) {
                if facts.no_return {
                    return true;
                }
            }
        }
        if arch.symboltab.function_is_no_return_across_scopes(entry) {
            return true;
        }
        // (kuna, angr `test_decompiling_incorrect_duplication_chcon_main`) When
        // `option noreturn_externmatch on` (DIV-13 default-on), also report no-return
        // for a callee whose *name* matches the vendored ELF known-no-return list —
        // closing the ET_REL `.o` gap where the address-keyed `noreturn_known` scan
        // emitted no fact for an undefined extern (`__stack_chk_fail`), so flow stops
        // at the call instead of decoding the trailing alignment padding. A no-op on a
        // normal ELF (the proto flag above is already set). See
        // `kuna_noreturn_externmatch`.
        if arch.noreturn_extern_match {
            if let Some(name) = arch.symboltab.function_display_name_across_scopes(entry) {
                if crate::kuna_noreturn_externmatch::is_known_noreturn_name(&name) {
                    return true;
                }
            }
        }
        // (kuna `securitycheck`, DIV-82) The seven rustc security-check helpers are
        // all declared `-> !` in libcore, so a call to one diverges by definition.
        // kuna's address-keyed no-return discovery only proves that for the helper
        // bodies it can walk, and the ones it misses leave a *returning* panic block
        // that `ActionRemoveSecurityCheck`'s divergence guard then (correctly)
        // declines to remove.  Asserting the fact from the name here is the same
        // seam and the same shape as `noreturn_externmatch` above, on a list that
        // exists nowhere but Rust — so it is inert on a C binary.
        if arch.strip_security_check {
            if let Some(name) = arch.symboltab.function_display_name_across_scopes(entry) {
                if crate::kuna_securitycheck::is_security_check_name(&name) {
                    return true;
                }
            }
        }
        // (kuna) noreturn_extern: when the address-keyed flag is unset, fall back
        // to a name match against the known ELF no-return list.  This catches an
        // **undefined external** no-return (`__stack_chk_fail` in an ET_REL `.o`)
        // that the analysis-tier `noreturn_known` pass — which keys on a *defined*
        // FUNC symbol's address — never marks, so flow would otherwise run off the
        // function's end into the next one.  Default off (`option noreturn_extern`).
        if arch.noreturn_extern_calls {
            if let Some(name) = self.query_call(entry) {
                return crate::kuna_noreturnextern::matches_noreturn_extern_name(&name);
            }
        }
        false
    }
    fn funcbound_flow_enabled(&self) -> bool {
        // (kuna funcboundflow) the Architecture-owned gate (`option funcboundflow`).
        // When on, `flow.rs` truncates a fall-through that reaches a known function
        // entry (via `query_call`) so the next function is not decoded into this one.
        self.arch().funcbound_flow
    }
    fn overlap_branch_enabled(&self) -> bool {
        // (kuna overlapbranch) the Architecture-owned gate (`option overlapbranch`).
        // When on, `flow.rs` truncates a conditional branch's fall-through whose
        // encoding swallows the branch's own target.
        self.arch().overlap_branch
    }
    fn query_call_inline(&self, entry: &Address) -> bool {
        // C++ `queryCall` copies the callee proto's `isInline()` flow effect; the
        // flag is set by `option inline <name>` (OptionInline) on the resolved
        // FunctionSymbol.
        self.arch().symboltab.function_is_inline_across_scopes(entry)
    }
    fn query_call_inject_id(&self, entry: &Address) -> int4 {
        // The callee's parked inject id (IfcFixupApply); -1 for none.
        self.arch().symboltab.function_inject_id_across_scopes(entry)
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
        //
        // (kuna `cortexmpriv`) The synthesized `isCurrentModePrivileged` fixup is
        // registered at architecture bootstrap, before any `option` line is
        // applied, so this — the only live per-CALLOTHER predicate that sees the
        // applied options — is where the flag gates it. Off, the CALLOTHER is left
        // alone and prints through the ordinary user-op path.
        let arch = self.arch();
        let op = match arch.userops.get_op(userop_index as kuna_base::types::uint4) {
            Some(op) => op,
            None => return false,
        };
        if op.get_type() != crate::userop::userop_type::injected {
            return false;
        }
        if !arch.cortexmpriv {
            if let Some(id) = op.get_inject_id() {
                if crate::kuna_cortexmpriv::is_our_fixup(arch.cortexmpriv_inject, id as int4) {
                    return false;
                }
            }
        }
        true
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
        // C++ `FlowInfo::injectUserOp`'s emit step.
        let arch = self.arch();
        let injectid = arch
            .userops
            .get_op(userop_index as kuna_base::types::uint4)
            .and_then(|op| op.get_inject_id())
            .ok_or_else(|| KunaError::lowlevel("inject_userop: user-op is not injected"))?
            as int4;
        emit_inject(arch, injectid, context, emit)
    }

    fn inject_call_fixup_payload(
        &self,
        inject_id: int4,
        context: &mut crate::pcodeinject::InjectContext,
        emit: &mut dyn kuna_sleigh::translate::PcodeEmit,
    ) -> KunaResult<()> {
        // C++ `FlowInfo::injectSubFunction`'s emit step.
        let arch = self.arch();
        emit_inject(arch, inject_id, context, emit)
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
        // back-pointer in the merged tree (// STUB(w6-fspec-2)), so use the default
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

    fn is_v850_indirect_jmp(&self, fd: &Funcdata, op: crate::context::OpId) -> bool {
        // (kuna) GH-8817: wire the ported `kunaIsV850IndirectJmp` predicate.  The
        // gate is the architecture-owned `v850_indirect_branch` flag (`option
        // v850indirectbranch on|off`, default off / upstream byte-identical); the
        // register name is `glb->translate->getRegisterName(spc, off, size)` of
        // op's input-0 varnode (None == the C++ empty string, "not a named
        // register").
        let arch = self.arch();
        if !arch.v850_indirect_branch {
            // Fast-path the default-off gate without touching the IR (matches the
            // predicate's leading `if (!gate) return false`).
            return false;
        }
        // Resolve the input-0 register name for the predicate (the predicate
        // re-checks CALLIND / processor-space / null-input, so only the name
        // resolution lives here).
        let regname: Option<String> = (|| {
            let opref = fd.obank().get(op)?;
            let vn = opref.get_in(0)?;
            let vnref = fd.vbank().get(vn)?;
            let space = vnref.get_space();
            let off = vnref.get_offset();
            let size = vnref.get_size();
            let raw = arch
                .translate()
                .as_sleigh()
                .expect("v850 indirect-branch predicate: standalone Sleigh engine")
                .base()
                .get_register_name(space, off, size);
            if raw.is_empty() {
                None
            } else {
                Some(String::from_utf8_lossy(&raw).into_owned())
            }
        })();
        crate::kuna_v850indbranch::kuna_is_v850_indirect_jmp(
            fd,
            op,
            arch.v850_indirect_branch,
            regname.as_deref(),
        )
    }

    fn is_sparc_struct_ret_trap(&self, fd: &Funcdata, op: crate::context::OpId) -> bool {
        // (kuna) GH-6882: wire the ported `kunaIsSparcStructRetTrap` predicate.
        // The gate is the architecture-owned `sparc_struct_return` flag (`option
        // sparcstructret on|off`, default off / upstream byte-identical); the
        // user-op name resolution is `glb->userops.getOp(id)->getName()` (None ==
        // the C++ null `UserPcodeOp *`).
        let arch = self.arch();
        if !arch.sparc_struct_return {
            // Fast-path the default-off gate without touching the IR (matches the
            // predicate's leading `if (!gate) return false`).
            return false;
        }
        crate::kuna_sparcstructret::kuna_is_sparc_struct_ret_trap(
            fd,
            op,
            arch.sparc_struct_return,
            |id| {
                arch.userops
                    .get_op(id)
                    .map(|uo| String::from_utf8_lossy(uo.get_name()).into_owned())
            },
        )
    }

    fn is_fastfail_callind(&self, fd: &Funcdata, op: crate::context::OpId) -> bool {
        // (kuna `fastfailnoreturn`) wire the `int 0x29` predicate.  The gate is the
        // architecture-owned `fastfail_noreturn` flag (`option fastfailnoreturn
        // on|off`, DIV default-on) plus the compiler-spec component of the resolved
        // language id — `int 0x29` is `__fastfail` by Windows convention alone.  The
        // user-op name resolution is `glb->userops.getOp(id)->getName()`.
        let arch = self.arch();
        if !arch.fastfail_noreturn
            || !crate::kuna_fastfailnoreturn::archid_is_windows(arch.get_description())
        {
            // Fast-path both gates without touching the IR.
            return false;
        }
        crate::kuna_fastfailnoreturn::is_fastfail_callind(fd, op, |id| {
            arch.userops.get_op(id).map(|uo| String::from_utf8_lossy(uo.get_name()).into_owned())
        })
    }

    fn is_tail_call_branch(&self, fd: &Funcdata, op: crate::context::OpId, dest: &Address) -> bool {
        // (kuna) tee-O2 tail-jump: wire the ported `kuna_is_tail_call_branch`
        // predicate.  The gate is the architecture-owned `tail_call_jumps` flag
        // (`option tailcalljump on|off`, default-off opt-in / default-pipeline
        // byte-identical — default-on regresses 2 datatests, Long double #1/#2).
        // The callee resolution is `query_call(dest).is_some()` (is `dest` a known
        // function entry, incl. a PLT thunk?) and the self-entry check is
        // `dest == fd.getAddress()`.
        let arch = self.arch();
        if !arch.tail_call_jumps {
            // Fast-path the default-off gate without touching the IR.
            return false;
        }
        let dest_is_known_function = self.query_call(dest).is_some();
        let dest_is_self = dest == fd.get_address();
        crate::kuna_tailcalljump::kuna_is_tail_call_branch(
            fd,
            op,
            arch.tail_call_jumps,
            dest_is_known_function,
            dest_is_self,
        )
    }

    fn is_frame_teardown_tail_call(
        &self,
        fd: &Funcdata,
        op: crate::context::OpId,
        dest: &Address,
    ) -> bool {
        // (kuna `tailcallframe`) The gate is the architecture-owned
        // `tail_call_frame` flag; the predicate needs the stack-pointer register
        // location, which is the stack space's own base register
        // (`getStackSpace()->getSpacebaseFull(0)`).  A compiler spec with no
        // `<stackpointer>` has no stack space, and the rule declines.
        let arch = self.arch();
        if !arch.tail_call_frame {
            // Fast-path the gate without touching the IR.
            return false;
        }
        let sp = arch
            .manage()
            .get_stack_space()
            .and_then(|spc| spc.get_spacebase_full(0).ok());
        crate::kuna_tailcallframe::kuna_is_frame_teardown_tail_call(
            fd,
            op,
            arch.tail_call_frame,
            fd.get_address(),
            dest,
            sp.as_ref(),
        )
    }
}

/// Build a [`Funcdata`] for the function `name` at `entry` and follow its flow,
/// returning the IR-populated `Funcdata` (C++ `Funcdata` construction +
/// `Funcdata::followFlow`).
///
/// The Funcdata is built through [`Architecture::new_funcdata`] (the W3 boot
/// boundary: it carries the IR-boundary address-space slice + the analysis
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
    // C++ Funcdata::followFlow(baddr, eaddr): a function carrying a declared byte
    // extent restricts flow to it; size 0 keeps the unbounded default the whole
    // engine has used until now (`kuna_console::engine::UNBOUNDED_SIZE`), so this
    // is inert for every caller that does not declare one. `eaddr` is INCLUSIVE
    // (`FlowInfo::new_address` rejects `eaddr < to`), so the last in-body byte is
    // `entry + size - 1`.
    let range = (fd.get_size() > 0).then(|| fd.get_address().clone()).and_then(|start| {
        let last = start.get_offset().checked_add(fd.get_size() as u64 - 1)?;
        let space = Rc::clone(start.get_space()?);
        Some((start.clone(), Address::new(space, last)))
    });
    let env = ArchFlowEnv { arch: arch as *const Architecture };
    let mut flow = FlowInfo::new(fd, &env);
    if let Some((baddr, eaddr)) = range {
        flow.set_range(baddr, eaddr);
    }
    // C++ Funcdata::followFlow (decompiler/cpp/funcdata_op.cc:765): after the
    // FlowInfo is constructed (and its range set), followFlow applies the global
    // flow options and the instruction bound.
    // `flowoptions` defaults to `error_toomanyinstructions` and
    // `max_instructions` to 100000 (`resetDefaultsInternal`); the console
    // options `maxinstruction` / `errortoomanyinstructions` / `unimplemented` /
    // `jumpload` mutate them (options.cc ports in `p0_knowledge/options.rs`).
    flow.set_flags(arch.flowoptions);
    flow.set_maximum_instructions(arch.max_instructions);
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
    // `ActionStart` is a stub, so it is driven here, exactly as the C++
    // `followFlow`→`startProcessing` order runs it before the action pipeline).
    data.structure_reset();
    // C++ startProcessing also calls sortCallSpecs() (dominance order for the
    // call-spec list); now that qlst exists, sort it.
    data.sort_call_specs();
    // startProcessing then sets the processing_started flag (so isProcStarted()
    // is true; the rest of startProcessing — sortCallSpecs / buildInfoList /
    // applyDeadCodeDelay — is a W4 stub or handled lazily in op_heritage).
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
    // (kuna decompile-all watchdog) The jumptable sub-pipeline runs inside the
    // budgeted per-function drive (flow-follow), so it shares the same deadline;
    // `None` on every unbudgeted path (console/parity — structurally unchanged).
    ctx.deadline = arch.kuna_fn_deadline;
    let result = {
        let root = arch
            .allacts
            .get_current_mut()
            .ok_or_else(|| KunaError::lowlevel("no current jumptable action"))?;
        root.reset(partial);
        // catch_unwind so an un-ported pass stub in the sub-pipeline degrades to a
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
            panic_message(payload)
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
            let deadline = arch.kuna_fn_deadline;
            let root = arch
                .allacts
                .get_current_mut()
                .ok_or_else(|| kuna_base::error::KunaError::lowlevel("no current action"))?;
            root.reset(fd);
            let mut ctx = ActionContext::new();
            // (kuna decompile-all watchdog) thread the per-function deadline
            // into the action loop (`None` = no budget, the parity default).
            ctx.deadline = deadline;
            let r = root.perform(fd, &mut ctx);
            (r, ctx.reflow_requested)
        };
        let (r, reflow_requested) = res;
        // (kuna decompile-all watchdog) A budgeted drive that ran past its
        // deadline unwound cooperatively (the action/heritage loops stop
        // scheduling work); surface it as the per-function error the batch
        // driver records, instead of printing a half-analyzed function.
        if let Some(deadline) = arch.kuna_fn_deadline {
            if std::time::Instant::now() >= deadline {
                let secs = arch.kuna_fn_budget.map(|b| b.as_secs()).unwrap_or(0);
                return Err(kuna_base::error::KunaError::lowlevel(format!(
                    "per-function decompile budget exceeded ({secs} s)"
                )));
            }
        }
        if r < 0 {
            return Ok(r); // breakpoint — propagate verbatim (no re-flow)
        }
        total += r;
        if !(reflow_requested && fd.has_restart_pending()) {
            drain_pipeline_comments(arch, fd);
            return Ok(total);
        }
        // Re-follow flow against the same Funcdata (its Override — incl. the
        // deindirect's indirect override — survives `clear()`), then loop to
        // re-perform.  `clear()` resets the restart flag, so a fresh request must
        // come from the re-flowed pass to loop again.
        refollow_flow(arch, fd)?;
        // (kuna `rustabi`) The re-flow hands back a FRESH Funcdata, so the
        // callee-body evidence has to be taken again or the call-output seam
        // silently loses its veto on every restarted function.  Cached, so this
        // is a map lookup per call.
        crate::kuna_rustabi::seed_callee_return_writes(arch, fd);
        // (kuna `calleedeadarg`) Same story for the entry-liveness probe the
        // input-trial scoring seam consults.
        crate::p4_calls::kuna_calleedeadarg::seed_callee_entry_dead(arch, fd);
        // (kuna `calleepreserves`) And for the call-guard seam's view of the
        // callee's writes; shares rustabi's cache, so this is a map lookup.
        crate::p4_calls::kuna_calleepreserves::seed_callee_preserves(arch, fd);
    }
    // Exceeded the cross-flow restart budget; keep the last analyzed IR.
    fd.set_restart_pending(false);
    drain_pipeline_comments(arch, fd);
    Ok(total)
}

/// Flush any analysis comments the **action pipeline** buffered on the `Funcdata`
/// (e.g. the `branchflip:` warning that `ActionBranchFlip` records at S8) into the
/// comment database for emit.  The flow-time `Funcdata::warning`s are drained in
/// `follow_flow_on_fd` before the pipeline runs; warnings raised *inside* the
/// pipeline land here.  No-op when nothing was buffered.
fn drain_pipeline_comments(arch: &mut Architecture, fd: &mut Funcdata) {
    let func_addr = fd.get_address().clone();
    for (tp, ad, txt) in fd.drain_pending_comments() {
        arch.commentdb.add_comment_no_duplicate(tp, &func_addr, &ad, &txt);
    }
}

/// Re-follow flow on an existing, already-analyzed `fd` after a restart request
/// (C++ `ActionRestartGroup` → `clearAnalysis` + `ActionStart::startProcessing` →
/// `followFlow`).  Clears the analysis-derived IR (`Funcdata::clear`) while
/// preserving the per-function `Override`, then re-runs the flow follow in place.
#[allow(clippy::mutable_key_type)]
fn refollow_flow(arch: &mut Architecture, fd: &mut Funcdata) -> KunaResult<()> {
    // C++ `glb->clearAnalysis(&data)` == fd->clear() (the comment-DB clear is the
    // W4 stub) — drops ops/vbank/blocks/qlst/heritage but keeps `localoverride`.
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
    decompile_func_full_with_override_dyn_prefollowed(
        arch,
        name,
        funcaddr,
        size,
        mapped_symbols,
        usepoint_symbols,
        dynamic_symbols,
        pending_proto,
        flow_overrides,
        proto_overrides,
        mapped_params,
        None,
    )
}

/// [`decompile_func_full_with_override_dyn`], but able to adopt a `Funcdata`
/// whose flow has **already been followed** instead of following it again.
///
/// The console runs `load function <name>` and then `decompile`, and each of
/// those followed the flow of the same function from scratch -- so a `kuna
/// decompile` paid the whole lift, including the per-jump-table
/// sub-decompilation, twice.  C++ has no such duplication: `IfcFuncload`
/// follows flow once and `IfcDecompile` re-runs the actions on *that*
/// `Funcdata` after `Architecture::clearAnalysis` (ifacedecomp.cc:889).
///
/// `prefollowed` is the caller's already-followed IR.  It is adopted verbatim,
/// so the caller is responsible for having followed flow with the SAME name,
/// entry, size, flow overrides and prototype overrides this call would have used
/// -- see `kuna_console::ifacedecomp` (`PristineFlow`), which only offers one
/// when every seed below is empty and nothing has run in between.  `None`
/// restores the build-and-follow behaviour every other caller has.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::mutable_key_type)]
pub fn decompile_func_full_with_override_dyn_prefollowed(
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
    prefollowed: Option<Funcdata>,
) -> KunaResult<Funcdata> {
    // (kuna decompile-all watchdog) Arm the per-function deadline from the
    // driver-set budget (`kuna decompile-all --max-fn-seconds N` sets
    // `kuna_fn_budget`; every other path leaves it `None`, so this is a `None`
    // assignment and the pipeline below is structurally unchanged).  The budget
    // covers the whole drive — flow-follow (incl. the jumptable sub-pipeline)
    // and the action pipeline — and is consulted cooperatively at the action /
    // rule-pool / heritage loop boundaries.
    arch.kuna_fn_deadline = arch.kuna_fn_budget.map(|b| std::time::Instant::now() + b);
    // (ghidra-mode, Phase 4) Take the staged name recommendations UP FRONT so
    // an early flow failure can never leak them into a later drive.
    let staged_name_recs = std::mem::take(&mut arch.kuna_pending_name_recs);
    let staged_dyn_recs = std::mem::take(&mut arch.kuna_pending_dyn_recs);
    let staged_proto_model = arch.kuna_pending_proto_model.take();
    let result = (|| {
        // Kept for the parked-prototype lookup below (the flow build consumes the
        // address).
        let entry_addr = funcaddr.clone();
        let mut fd = match prefollowed {
            Some(fd) => fd,
            None => build_and_follow_flow_with_override_and_protos(
                arch,
                name,
                funcaddr,
                size,
                flow_overrides,
                proto_overrides,
            )?,
        };
        // The prototype the function is decompiled *against*. Two sources, in
        // precedence order:
        //
        //  1. `pending_proto` — a prototype the operator declared for this run
        //     (`parse line extern ...`). An explicit declaration always wins.
        //  2. The prototype parked on this function's own global FunctionSymbol.
        //     That is where the DWARF pass's recovered `DW_TAG_subprogram`
        //     signature lands (`set_function_prototype_pieces`), and where the
        //     library-prototype table lands for a named libc function.
        //
        // Source 2 used to be read *only* by a CALLER — `ActionDefaultParams`
        // copies a callee's parked prototype into the call site, so `fmt(FILE*,
        // char const*)` typed its arguments correctly at every call — while the
        // function's own decompile ignored it and re-derived everything from data
        // flow. So `main`, fully described by `.debug_info` as
        // `int main(int argc, char **argv)`, rendered `undefined16 main(uint4
        // a0, void *a1)`: the recovered signature existed and was thrown away.
        //
        // Applying it locks the inputs and the output, which is also what
        // collapses the bogus wide return. With no locked output,
        // `ActionPrototypeTypes` turns on return recovery, `Heritage::guard_returns`
        // registers a trial per output register the model characterizes (x86-64
        // gcc: RAX *and* RDX), and the cspec's `join_dual_class` output rule
        // accepts the consecutive pair as one 16-byte return — materialized as
        // `PIECE(RDX,RAX)` and typed `undefined16`, with the never-written RDX half
        // picking up an uninitialized stack slot. A known `int` return skips that
        // machinery entirely.
        //
        // (kuna `cppproto`) Resolved across ALL scopes, not just the global one: a
        // demangled C++ function is filed under its namespace/class scope, so a
        // global-only lookup never finds the prototype the DWARF C++ arm parked on
        // `Account::deposit`. Inert for every prototype parked by NAME (that path
        // resolves through the global scope, so it can only reach global symbols).
        let recovered_proto = if pending_proto.is_none() {
            arch.symboltab.function_proto_pieces_across_scopes(&entry_addr).cloned()
        } else {
            None
        };
        // Apply the resolved prototype to the fresh funcp (the input-param
        // recovery SEED): after this the inputs/output are type-locked, so
        // ActionPrototypeTypes forces the typed Varnodes.
        if let Some(pieces) = pending_proto.or(recovered_proto.as_ref()) {
            // (ghidra-mode, Phase 4) A host-declared model rides with the
            // pieces so parameter storage is assigned under the SAME
            // convention the database committed (see
            // `Architecture::kuna_pending_proto_model`); `None` everywhere
            // else keeps the architecture default.
            fd.apply_locked_prototype_with_model(pieces, staged_proto_model.clone())?;
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
        // (ghidra-mode, Phase 4) Seed the staged name recommendations (the
        // host `<localdb>`'s rename-only locals — C++ `nameRecommend`
        // entries); empty everywhere outside ghidra mode.
        if !staged_name_recs.is_empty() {
            fd.seed_name_recommendations(&staged_name_recs);
        }
        if !staged_dyn_recs.is_empty() {
            fd.seed_dynamic_recommendations(&staged_dyn_recs);
        }
        // (kuna `rustabi`) Take the callee-body probe the call-output seam needs.
        // The per-function handle the pipeline runs against carries no
        // translator, so this is the last point the callee's instructions can be
        // read at all; inert unless `option rustabi` is live for this image.
        crate::kuna_rustabi::seed_callee_return_writes(arch, &mut fd);
        // (kuna `calleedeadarg`) Take the callee entry-liveness probe the
        // input-trial scoring seam consults, for the same reason and at the same
        // point; inert unless `option calleedeadarg` is live.
        crate::p4_calls::kuna_calleedeadarg::seed_callee_entry_dead(arch, &mut fd);
        // (kuna `calleepreserves`) The call-guard seam's view of the same
        // decode: the registers the callee is proven NOT to write; inert unless
        // `option calleepreserves` is live.
        crate::p4_calls::kuna_calleepreserves::seed_callee_preserves(arch, &mut fd);
        // With the single-manager unification (LOSS-132) the universalAction passes
        // now reach the *real* lifted varnodes, so the pipeline genuinely executes
        // heritage / simplification / merge / … on live IR.  Some pass BODIES are
        // still un-ported stubs (LOSS-131, the M3 grind): a hand-built fixture never
        // reached them, but a real corpus function can hit, e.g.,
        // `Heritage::normalizeWriteSize`'s PIECE-concat path.  Those stubs abort via
        // `unimplemented_stub` (a deliberate `#[cold] panic!`).  Convert such a
        // stub-abort into a recoverable `Err` at this orchestration boundary so the
        // end-to-end harnesses degrade to the documented "honest partial parity"
        // (the pipeline ran; a body declined at a stub) instead of taking down the
        // whole run — exactly the graceful-degradation the LOSS-130/131 measurement
        // assumes.  `fd`/`arch` are discarded on the unwind, so no half-mutated
        // state escapes (`AssertUnwindSafe` is sound here for that reason).
        let res =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run_pipeline(arch, &mut fd)));
        match res {
            Ok(r) => {
                r?;
                Ok(fd)
            }
            Err(payload) => {
                let msg = panic_message(payload);
                Err(kuna_base::error::KunaError::lowlevel(format!(
                    "decompile pipeline reached an un-ported seam (LOSS-131): {msg}"
                )))
            }
        }
    })();
    // (kuna decompile-all watchdog) Disarm the deadline once the drive is over so
    // no later, non-drive pipeline run (console sub-queries) consults a stale one.
    arch.kuna_fn_deadline = None;
    result
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
/// guard).  Stub aborts degrade to a recoverable `Err`, like the full drive.
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
            panic_message(payload)
        ))),
    }
}

/// Best-effort extraction of a panic payload's message (the `panic!` string),
/// for surfacing an un-ported-stub abort as a recoverable [`KunaError`].
///
/// Takes the `catch_unwind` payload **by value**: a `&Box<dyn Any + Send>`
/// unsize-coerces to the *box itself* as the `Any`, so every `downcast` against
/// the payload types silently fails and the message is lost (the bug this
/// signature makes unwritable).
fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<&'static str>() {
        Ok(s) => (*s).to_string(),
        Err(payload) => match payload.downcast::<String>() {
            Ok(s) => *s,
            Err(_) => "panic with non-string payload".to_string(),
        },
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

/// Render the ordinary C text and independently collect the markup references
/// needed to map its lines back to machine instructions.
pub fn print_c_with_provenance(
    arch: &mut Architecture,
    fd: &Funcdata,
) -> (String, CodeProvenance) {
    let mut printer = arch.take_print();
    let out = printer.doc_function_full(fd, arch);
    let markup = printer.doc_function_provenance(fd, arch);
    arch.put_print(printer);
    (out, resolve_markup_provenance(fd, &markup))
}

/// (kuna) Render every user-defined data-type in the architecture's type
/// factory as C definitions (the `PrintC::docTypeDefinitions` port,
/// [`crate::printc::PrintC::doc_type_definitions`]) — the type-definition
/// block of the `kuna decompile-project` `.h` artifact.  Same
/// take/put split-borrow dance as [`print_c`] (the printer is owned by `arch`).
pub fn print_c_types(arch: &mut Architecture) -> String {
    let mut printer = arch.take_print();
    let out = printer.doc_type_definitions(arch);
    arch.put_print(printer);
    out
}

/// (kuna) Render ONLY the decompiled function's prototype declaration —
/// `<ret> <name>(<params>);` — via
/// [`crate::printc::PrintC::doc_prototype`]: the `.h`-prototype half of the
/// `kuna decompile-project` prototype == definition-line contract (the emitted
/// text minus the trailing `;` appears char-for-char inside [`print_c`]'s
/// render of the same `Funcdata`).  A function whose prototype was never
/// recovered (no proto store) renders as `void <name>(void);`.
pub fn print_c_prototype(arch: &mut Architecture, fd: &Funcdata) -> String {
    let mut printer = arch.take_print();
    let out = printer.doc_prototype(fd, arch);
    arch.put_print(printer);
    out
}

/// (kuna) The generated recompile prelude for a `kuna decompile-project` `.h`:
/// C typedefs for the architecture's interned **core** scalar types (`uint4`,
/// `int8`, `float8`, …, spelled per standard C — 8-byte integers always use
/// `long long` so the text is data-model independent) plus the fixed Ghidra
/// `undefined` family block (`undefined`, `undefined1..8` — 3/5/6/7 mapped to
/// the next larger unsigned integer with a sizeof note — and `undefined16/32`
/// as byte-array structs).  `bool` is covered by `#include <stdbool.h>`;
/// `char`/`void` are real C and emit nothing.
pub fn print_c_recompile_prelude(arch: &Architecture) -> String {
    use crate::dtype::type_metatype::*;
    let mut out = String::new();
    out.push_str("/* kuna recompile prelude (generated): core scalar typedefs */\n");
    out.push_str("#include <stdbool.h>\n\n");

    // Generated typedefs for the interned core types (dependent_order filtered
    // on is_core_type; sorted by name for a stable, readable block).
    let mut lines: Vec<String> = Vec::new();
    for ct in arch.types_impl().dependent_order() {
        if !ct.is_core_type() {
            continue;
        }
        let name = ct.get_name();
        // `void`/`char` are real C; `bool` comes from <stdbool.h> above.
        if name.is_empty() || matches!(name, "void" | "char" | "bool") {
            continue;
        }
        let size = ct.get_size();
        let (spelling, note): (&str, &str) = match ct.get_metatype() {
            TYPE_INT => match size {
                1 => ("signed char", ""),
                2 => ("short", ""),
                4 => ("int", ""),
                8 => ("long long", ""),
                _ => continue,
            },
            TYPE_UINT | TYPE_UNKNOWN => match size {
                1 => ("unsigned char", ""),
                2 => ("unsigned short", ""),
                4 => ("unsigned int", ""),
                8 => ("unsigned long long", ""),
                _ => continue,
            },
            TYPE_FLOAT => match size {
                4 => ("float", ""),
                8 => ("double", ""),
                10 | 16 => ("long double", " /* sizeof may differ */"),
                _ => continue,
            },
            // The pseudo "function body" type: only ever used as `code *`;
            // a 1-byte scalar keeps `sizeof(code)` meaningful too.
            TYPE_CODE => ("unsigned char", " /* pseudo (function body) type */"),
            _ => continue,
        };
        lines.push(format!("typedef {spelling} {name};{note}\n"));
    }
    lines.sort();
    lines.dedup();
    for l in &lines {
        out.push_str(l);
    }

    // The fixed Ghidra/kuna `undefined` family (the anonymous-unknown
    // rendering `undefined<N>`, printc.rs `declarator_parts`).
    out.push_str("\n/* the Ghidra/kuna `undefined` family */\n");
    out.push_str("typedef unsigned char undefined;\n");
    out.push_str("typedef unsigned char undefined1;\n");
    out.push_str("typedef unsigned short undefined2;\n");
    out.push_str("typedef unsigned int undefined3; /* 3 bytes in the decompiler; sizeof differs */\n");
    out.push_str("typedef unsigned int undefined4;\n");
    out.push_str("typedef unsigned long long undefined5; /* 5 bytes in the decompiler; sizeof differs */\n");
    out.push_str("typedef unsigned long long undefined6; /* 6 bytes in the decompiler; sizeof differs */\n");
    out.push_str("typedef unsigned long long undefined7; /* 7 bytes in the decompiler; sizeof differs */\n");
    out.push_str("typedef unsigned long long undefined8;\n");
    out.push_str("typedef struct { unsigned char b[16]; } undefined16;\n");
    out.push_str("typedef struct { unsigned char b[32]; } undefined32;\n");
    out
}

/// A recovered variable surfaced for the machine-readable batch output
/// (`kuna decompile-all --json`) — the fields decbench's `type_match` metric
/// consumes from a decompiler's per-function variable list.
#[derive(Debug, Clone)]
pub struct VarInfo {
    /// Variable name as it appears in the decompiled C (`param_1`, `local_18`, a
    /// DWARF name on a `-g` binary, …).
    pub name: String,
    /// The C type string, rendered exactly as `print_c` would spell it
    /// (`int`, `char *`, `undefined8`, …) via [`crate::printc::type_to_c_string`].
    pub type_name: String,
    /// Signed frame-relative stack offset for a stack-resident variable; `None`
    /// for a register-resident parameter (ghidra `getStorage().getStackOffset()`).
    pub stack_offset: Option<i64>,
    /// Size in bytes (the variable's type size).
    pub size: i64,
    /// `true` for a formal parameter (`kind="arg"`), `false` for a local
    /// (`kind="stack"`).
    pub is_param: bool,
    /// ABI parameter index (dense, source order) for parameters; `None` for locals.
    pub arg_index: Option<usize>,
    /// 1-based pseudocode lines containing markup-backed references to this variable.
    pub line_numbers: Vec<usize>,
    /// Machine instruction addresses associated with those pseudocode lines.
    pub addresses: Vec<u64>,
}

/// One 1-based pseudocode line and its associated machine instruction addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineMapping {
    pub line_number: usize,
    pub addresses: Vec<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct VariableUseEvidence {
    line_numbers: Vec<usize>,
    addresses: Vec<u64>,
}

/// Provenance resolved from printer markup against a decompiled function's IR.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CodeProvenance {
    pub line_mappings: Vec<LineMapping>,
    variable_uses: BTreeMap<u64, VariableUseEvidence>,
}

impl CodeProvenance {
    /// Attach markup-backed use evidence to the corresponding reported variables.
    pub fn apply_to_variables(&self, fd: &Funcdata, variables: &mut [VarInfo]) {
        for variable in variables {
            let storage_refs = variable_storage_varrefs(fd, variable);
            let mut evidence = self.evidence_for_refs(&storage_refs);
            if evidence.line_numbers.is_empty() {
                let named_refs = named_high_varrefs(fd, variable);
                evidence = self.evidence_for_refs(&named_refs);
            }
            variable.line_numbers = evidence.line_numbers;
            variable.addresses = evidence.addresses;
        }
    }

    fn evidence_for_refs(&self, varrefs: &BTreeSet<u64>) -> VariableUseEvidence {
        let mut line_numbers = BTreeSet::new();
        let mut addresses = BTreeSet::new();
        for evidence in varrefs.iter().filter_map(|varref| self.variable_uses.get(varref)) {
            line_numbers.extend(evidence.line_numbers.iter().copied());
            addresses.extend(evidence.addresses.iter().copied());
        }
        VariableUseEvidence {
            line_numbers: line_numbers.into_iter().collect(),
            addresses: addresses.into_iter().collect(),
        }
    }
}

fn extend_high_varrefs(fd: &Funcdata, high_id: HighVariableId, varrefs: &mut BTreeSet<u64>) {
    let Some(high) = fd.high_bank().get(high_id) else { return };
    for index in 0..high.num_instances() {
        let Some(varnode) = fd.vbank().get(high.get_instance(index)) else { continue };
        varrefs.insert(varnode.get_create_index() as u64);
    }
}

fn select_storage_varrefs(
    fd: &Funcdata,
    matches: &[(u64, Option<HighVariableId>)],
    variable_name: &str,
) -> BTreeSet<u64> {
    let highs: BTreeSet<HighVariableId> = matches.iter().filter_map(|(_, high)| *high).collect();
    let selected_high = if highs.len() == 1 {
        highs.iter().next().copied()
    } else {
        let named: Vec<HighVariableId> = highs
            .iter()
            .copied()
            .filter(|high_id| {
                fd.high_bank()
                    .get(*high_id)
                    .and_then(|high| high.kuna_name())
                    == Some(variable_name)
            })
            .collect();
        (named.len() == 1).then(|| named[0])
    };

    let mut varrefs = BTreeSet::new();
    if let Some(high_id) = selected_high {
        extend_high_varrefs(fd, high_id, &mut varrefs);
    } else if highs.is_empty() {
        varrefs.extend(matches.iter().map(|(varref, _)| *varref));
    }
    varrefs
}

fn parameter_storage(fd: &Funcdata, arg_index: usize) -> Option<(Address, i64)> {
    let proto = fd.get_func_proto();
    let mut source_index = 0usize;
    for index in 0..proto.num_params() {
        let parameter = proto.get_param(index)?;
        if parameter.is_hidden_return() {
            continue;
        }
        if source_index == arg_index {
            return Some((parameter.get_address(), parameter.get_size() as i64));
        }
        source_index += 1;
    }
    None
}

fn variable_storage_varrefs(fd: &Funcdata, variable: &VarInfo) -> BTreeSet<u64> {
    let mut matches = Vec::new();
    if variable.is_param {
        let Some(arg_index) = variable.arg_index else { return BTreeSet::new() };
        let Some((address, size)) = parameter_storage(fd, arg_index) else {
            return BTreeSet::new();
        };
        for varnode_id in fd.vbank().iter_loc() {
            let Some(varnode) = fd.vbank().get(varnode_id) else { continue };
            if varnode.get_addr() == &address && varnode.get_size() as i64 == size {
                matches.push((
                    varnode.get_create_index() as u64,
                    varnode.get_high(),
                    varnode.is_input(),
                ));
            }
        }
        if matches.iter().any(|(_, _, is_input)| *is_input) {
            matches.retain(|(_, _, is_input)| *is_input);
        }
    } else if let (Some(stack_offset), Some(scope)) =
        (variable.stack_offset, fd.get_scope_local())
    {
        let stack_space = scope.get_space_id();
        for varnode_id in fd.vbank().iter_loc() {
            let Some(varnode) = fd.vbank().get(varnode_id) else { continue };
            let in_stack_space = varnode
                .get_addr()
                .get_space()
                .is_some_and(|space| space.get_index() == stack_space.get_index());
            if in_stack_space
                && signed_space_offset(stack_space, varnode.get_offset()) == stack_offset
                && varnode.get_size() as i64 == variable.size
            {
                matches.push((
                    varnode.get_create_index() as u64,
                    varnode.get_high(),
                    false,
                ));
            }
        }
    }

    let matches: Vec<(u64, Option<HighVariableId>)> = matches
        .into_iter()
        .map(|(varref, high, _)| (varref, high))
        .collect();
    select_storage_varrefs(fd, &matches, &variable.name)
}

fn high_matches_stack_variable(
    fd: &Funcdata,
    high_id: HighVariableId,
    variable: &VarInfo,
) -> bool {
    let (Some(stack_offset), Some(scope), Some(high)) = (
        variable.stack_offset,
        fd.get_scope_local(),
        fd.high_bank().get(high_id),
    ) else {
        return false;
    };
    let stack_space = scope.get_space_id();
    (0..high.num_instances()).any(|index| {
        let Some(varnode) = fd.vbank().get(high.get_instance(index)) else { return false };
        let Some(space) = varnode.get_addr().get_space() else { return false };
        let is_stack_reference = space.get_index() == stack_space.get_index()
            || space.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT;
        is_stack_reference
            && signed_space_offset(stack_space, varnode.get_offset()) == stack_offset
            && varnode.get_size() as i64 == variable.size
    })
}

fn named_high_varrefs(fd: &Funcdata, variable: &VarInfo) -> BTreeSet<u64> {
    let named: Vec<HighVariableId> = fd
        .high_bank()
        .iter()
        .filter_map(|(high_id, high)| {
            (high.kuna_name() == Some(variable.name.as_str())).then_some(high_id)
        })
        .collect();
    let selected: Vec<HighVariableId> = if named.len() <= 1 {
        named
    } else {
        named
            .into_iter()
            .filter(|high_id| high_matches_stack_variable(fd, *high_id, variable))
            .collect()
    };
    let mut varrefs = BTreeSet::new();
    for high_id in selected {
        extend_high_varrefs(fd, high_id, &mut varrefs);
    }
    varrefs
}

fn resolve_markup_provenance(
    fd: &Funcdata,
    markup: &crate::prettyprint::MarkupProvenance,
) -> CodeProvenance {
    let op_addresses: BTreeMap<u64, u64> = fd
        .obank()
        .iter_all()
        .filter_map(|(_, op_id)| {
            let op = fd.obank().get(op_id)?;
            op.get_addr().get_space()?;
            Some((op.get_time() as u64, op.get_addr().get_offset()))
        })
        .collect();
    resolve_markup_provenance_with_addresses(markup, &op_addresses)
}

fn resolve_markup_provenance_with_addresses(
    markup: &crate::prettyprint::MarkupProvenance,
    op_addresses: &BTreeMap<u64, u64>,
) -> CodeProvenance {
    let mut line_addresses: BTreeMap<usize, BTreeSet<u64>> = BTreeMap::new();
    let mut variable_lines: BTreeMap<u64, BTreeSet<usize>> = BTreeMap::new();

    for association in &markup.associations {
        if association.line_number == 0 {
            continue;
        }
        if let Some(address) = association.opref.and_then(|opref| op_addresses.get(&opref)) {
            line_addresses.entry(association.line_number).or_default().insert(*address);
        }
        if let Some(varref) = association.varref {
            variable_lines.entry(varref).or_default().insert(association.line_number);
        }
    }

    let line_mappings: Vec<LineMapping> = line_addresses
        .iter()
        .map(|(&line_number, addresses)| LineMapping {
            line_number,
            addresses: addresses.iter().copied().collect(),
        })
        .collect();
    let variable_uses = variable_lines
        .into_iter()
        .map(|(varref, lines)| {
            let addresses = lines
                .iter()
                .flat_map(|line| line_addresses.get(line).into_iter().flatten().copied())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            (
                varref,
                VariableUseEvidence {
                    line_numbers: lines.into_iter().collect(),
                    addresses,
                },
            )
        })
        .collect();
    CodeProvenance { line_mappings, variable_uses }
}

/// Interpret a raw spacebase `off` as a signed frame offset for `space`
/// (negative locals wrap to the high end of the unsigned range): sign-extend from
/// the space's address bit-width via the codebase's canonical
/// [`kuna_base::address::sign_extend`] (the same idiom `varmap` uses, which also
/// masks off any bits above the sign bit).  Matches ghidra
/// `getStorage().getStackOffset()`, which the decbench `type_match` metric
/// calibrates per binary against DWARF.  Stack spaces are byte-addressed
/// (`word_size == 1`), so no `byte_to_address` scaling is needed.
fn signed_space_offset(space: &Rc<kuna_base::space::AddrSpace>, off: u64) -> i64 {
    kuna_base::address::sign_extend(off as i64, space.get_addr_size() as i32 * 8 - 1)
}

/// (kuna) Extract the recovered parameters + stack locals of a decompiled
/// [`Funcdata`] as a flat `Vec<VarInfo>` for the `kuna decompile-all --json`
/// surface (→ decbench `type_match`).
///
/// Mirrors how the decbench Ghidra backend reads the `HighFunction` local symbol
/// map: parameters come from the `FuncProto` in ABI order (`arg_index` = dense
/// source position, a register-passed param has `stack_offset == None`); stack
/// locals come from the function's `ScopeLocal` stack space, keeping only
/// `NO_CATEGORY` symbols (a `FUNCTION_PARAMETER` symbol is already emitted as a
/// parameter, the `emitScopeVarDecls(no_category)` split).  Reads the
/// already-computed `Funcdata` — no decompile re-run.
pub fn extract_variables(arch: &Architecture, fd: &Funcdata) -> Vec<VarInfo> {
    let stack_index = arch.manage().get_stack_space().map(|s| s.get_index());
    let mut out: Vec<VarInfo> = Vec::new();

    // 1) Parameters, in ABI order, off the FuncProto.
    let proto = fd.get_func_proto();
    let nparams = proto.num_params();
    let mut arg_pos = 0usize;
    for i in 0..nparams {
        let Some(p) = proto.get_param(i) else { continue };
        // A hidden return-storage pointer is a synthetic ABI slot, not a
        // source-level parameter (no DWARF formal-parameter to match), so skip it.
        if p.is_hidden_return() {
            continue;
        }
        let raw_name = p.get_name();
        let name = if raw_name.is_empty() {
            format!("param_{}", arg_pos + 1)
        } else {
            raw_name.to_string()
        };
        let type_name = p
            .get_type()
            .map(|t| crate::printc::type_to_c_string(arch, t))
            .unwrap_or_default();
        let size = p.get_size() as i64;
        let addr = p.get_address();
        let stack_offset = match (stack_index, addr.get_space()) {
            (Some(si), Some(sp)) if sp.get_index() == si => {
                Some(signed_space_offset(sp, addr.get_offset()))
            }
            _ => None,
        };
        out.push(VarInfo {
            name,
            type_name,
            stack_offset,
            size,
            is_param: true,
            arg_index: Some(arg_pos),
            line_numbers: Vec::new(),
            addresses: Vec::new(),
        });
        arg_pos += 1;
    }

    // 2) Stack locals, off the ScopeLocal (NO_CATEGORY symbols only).
    if let Some(sl) = fd.get_scope_local() {
        let space = sl.get_space_id();
        let space_index = space.get_index() as usize;
        let specs = sl.database().scope_space_local_var_specs(sl.scope_id(), space_index);
        for (name, ct, addr, category) in specs {
            if category != crate::database::symbol_category::NO_CATEGORY {
                continue; // already emitted as a parameter above
            }
            let type_name = crate::printc::type_to_c_string(arch, &ct);
            out.push(VarInfo {
                name,
                type_name,
                stack_offset: Some(signed_space_offset(space, addr.get_offset())),
                size: ct.get_size() as i64,
                is_param: false,
                arg_index: None,
                line_numbers: Vec::new(),
                addresses: Vec::new(),
            });
        }
    }

/// (kuna `framelayout`) How an exported stack slot's data type is spelled.
///
/// A slot the type system never committed to is carried internally as
/// `xunknown1[N]` (Ghidra's `undefined1[N]`) or a bare `TYPE_UNKNOWN`, and
/// `type_to_c_string` renders the array form as `char[N]` -- which asserts an
/// element type the recovery never established.  Report those as the width-only
/// `undefined<N>`, the same spelling Ghidra uses for the same fact, and leave every
/// committed type exactly as the printer spells it.
fn frame_slot_type_name(arch: &Architecture, dt: &std::rc::Rc<crate::dtype::Datatype>) -> String {
    use crate::dtype::type_metatype;
    let n = dt.get_size();
    let uncommitted = match dt.get_metatype() {
        type_metatype::TYPE_UNKNOWN => true,
        type_metatype::TYPE_ARRAY => dt
            .get_array_base()
            .map(|e| e.get_metatype() == type_metatype::TYPE_UNKNOWN)
            .unwrap_or(false),
        _ => false,
    };
    if uncommitted && n > 0 {
        return format!("undefined{n}");
    }
    crate::printc::type_to_c_string(arch, dt)
}

    // 3) (kuna `framelayout`) The frame slots an EARLIER restructure pass recovered
    // and the final one no longer has, because the dataflow folded the spill away.
    // The emitted C is right to drop them; the recovered frame still contains them,
    // and `variables` reports the frame. Only offsets no parameter or surviving
    // local already covers are added, so this can never contradict section 1 or 2.
    if arch.framelayout {
        let covered: std::collections::BTreeSet<i64> =
            out.iter().filter_map(|v| v.stack_offset).collect();
        for (off, slot) in fd.frame_slots() {
            if covered.contains(&off) {
                continue;
            }
            // `$$undefNNNNNNNN` is Ghidra's internal placeholder for an unnamed
            // symbol and must not surface on a public interface; name the slot the
            // way Ghidra's stack view does.
            let name = if crate::kuna_undefname::is_undefined_name(&slot.name) {
                if off < 0 {
                    format!("local_{:x}", -off)
                } else {
                    format!("stack_{off:x}")
                }
            } else {
                slot.name
            };
            out.push(VarInfo {
                name,
                type_name: frame_slot_type_name(arch, &slot.dtype),
                stack_offset: Some(off),
                size: slot.size as i64,
                is_param: false,
                arg_index: None,
                line_numbers: Vec::new(),
                addresses: Vec::new(),
            });
        }
    }
    out
}

/// Emit an injection payload's p-code (C++ `InjectPayloadSleigh::inject` vs
/// `InjectPayloadGhidra::inject`): the locally compiled SLEIGH template when
/// the library has one, otherwise a back-end fetch through the translator
/// seam, which is where a ghidra-mode session — no local `.sla`, so no
/// compiled template ever — gets its answer.
fn emit_inject(
    arch: &Architecture,
    injectid: int4,
    context: &mut crate::pcodeinject::InjectContext,
    emit: &mut dyn kuna_sleigh::translate::PcodeEmit,
) -> KunaResult<()> {
    let payload = arch.pcodeinjectlib.get_payload(injectid);
    match arch.pcodeinjectlib.get_tpl(injectid) {
        Some(tpl) => {
            // SleighInjectEngine over the arch's const/unique/default-code spaces.
            let engine = crate::inject_sleigh::SleighInjectEngine::new(
                Rc::clone(arch.manage().get_constant_space().expect("no constant space")),
                Rc::clone(arch.manage().get_unique_space().expect("no unique space")),
                Rc::clone(arch.manage().get_default_code_space().expect("no default code space")),
            );
            engine.emit_payload(payload, tpl, context, emit)
        }
        None => arch.translate().fetch_inject_pcode(
            payload.core().get_name(),
            payload.core().get_type(),
            context,
            emit,
        ),
    }
}

#[cfg(test)]
mod tests;
