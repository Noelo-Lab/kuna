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

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
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
struct ArchFlowEnv<'a> {
    arch: &'a Architecture,
}

impl FlowEnvironment for ArchFlowEnv<'_> {
    fn translate(&self) -> &dyn Translate {
        self.arch.translate()
    }
    fn resolve_typeop(&self, opc: OpCode) -> TypeOp {
        self.arch.resolve_typeop(opc)
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
pub fn build_and_follow_flow(
    arch: &Architecture,
    name: &str,
    entry: Address,
    size: int4,
) -> KunaResult<Funcdata> {
    let fd = arch.new_funcdata(name, entry, size)?;
    let env = ArchFlowEnv { arch };
    let mut flow = FlowInfo::new(fd, &env);
    // C++ followFlow: generateOps() then generateBlocks().
    flow.generate_ops()?;
    flow.generate_blocks()?;
    let mut data = flow.data;
    // C++ `Funcdata::startProcessing` (funcdata.cc:150) runs after `followFlow`:
    // it calls `structureReset()` — which builds the basic-block reverse-post
    // ordering AND the forward dominator tree (`bblocks.calcForwardDominator`).
    // `ActionHeritage::buildADT` *requires* that dominator tree, so the reset is
    // part of the heritage-application prerequisite (the merged tree's
    // `ActionStart` is a seam, so it is driven here, exactly as the C++
    // `followFlow`→`startProcessing` order runs it before the action pipeline).
    data.structure_reset();
    // startProcessing then sets the processing_started flag (so isProcStarted()
    // is true; the rest of startProcessing — sortCallSpecs / buildInfoList /
    // applyDeadCodeDelay — is W4 seam or handled lazily in op_heritage).
    data.set_flag_raw(funcdata_flags::processing_started);
    Ok(data)
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
    let mut ctx = ActionContext::new();
    let root = arch
        .allacts
        .get_current_mut()
        .ok_or_else(|| kuna_base::error::KunaError::lowlevel("no current action"))?;
    root.reset(fd);
    Ok(root.perform(fd, &mut ctx))
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
    let mut fd = build_and_follow_flow(arch, name, funcaddr, size)?;
    let _res = run_pipeline(arch, &mut fd)?;
    Ok(fd)
}

/// Render `fd` to C text (C++ `IfcPrintC::execute` -> `print->docFunction(fd)`).
///
/// Drives [`PrintC::doc_function`] with the function's display name
/// (`fd->getDisplayName()`) and a `void NAME(void)` signature.
///
/// ## The W10 (`w10-printc-body`) finding
///
/// The PrintC RPN body *engine* (`push_op`/`push_atom`/`op_binary`/`op_unary`/
/// `emit_op`/`emit_atom`/`parentheses`) is now ported and byte-faithfully
/// unit-tested in [`PrintC`](crate::printc::PrintC).  Driving it over a real
/// function body — and reading the real signature — is blocked **upstream of the
/// printer**, not in it:
///
///   * **The proto / return type / params** (C++ `fd->getFuncProto().
///     getOutputType()` / `getParam(i)`): `Funcdata::funcp` is the seam
///     [`FuncProto`](crate::seams::FuncProto) unit stub, not the real
///     `fspec::FuncProto`, because the proto-recovery passes are seam stubs and
///     `funcdata.rs` has not yet swapped the field type (a `funcdata`/W4/W6
///     boundary, not this glue).
///   * **The body**: the universalAction pipeline RUNS but its passes (heritage
///     / simplification / merge / type recovery / block structuring) are seam
///     stubs, so the IR reaching the printer is *raw lifted p-code* (e.g. 23 ops
///     for `boolless` vs the ~7-op decompiled form, no HighVariables with
///     symbols, no recovered types, **`sblocks` empty**).  Emitting that raw IR
///     would print non-C garbage, not byte-parity (the parity gate
///     `tests/printc_parity.rs` measures this honestly against the C++ oracle).
///
/// Until those upstream passes land, the printer emits the default `void`-return,
/// `void`-params, brace-matched shell — exactly what the e2e gate asserts; the
/// byte-match count then rises with no further change to the (now-ported) RPN
/// body engine.
pub fn print_c(arch: &mut Architecture, fd: &Funcdata) -> String {
    let display = fd.get_display_name().to_string();
    // The seam `Funcdata::funcp` (a unit stub) exposes no recovered output type
    // or params (see doc above); emit the default void/void shell.  The
    // signature wiring is `arch.print_mut().doc_function(name, model, ret,
    // params)` — ready for the real `fspec::FuncProto` the moment `funcdata.rs`
    // carries it and the proto-recovery passes populate it.
    let params: Vec<(String, String)> = Vec::new();
    arch.print_mut().doc_function(&display, None, "void", &params)
}
