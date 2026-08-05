//! Shared **per-function decompile step** -- the one place a function is turned
//! into a `Funcdata`, used by both console `decompile` (`IfcDecompile`) and the
//! whole-binary loop (`project::decompile_targets`, behind `kuna decompile-all`
//! / `decompile-project` / the WASM front-end).
//!
//! Before this module the two surfaces each called
//! `decompile_func_full_with_override_dyn` themselves, and only the console one
//! wrapped it in Ghidra's `FormatStringAnalyzer` half-B loop (decompile -> read
//! the printf/scanf format constants -> install per-call-site prototype
//! overrides -> re-decompile).  The whole-binary surface therefore silently
//! ignored `--option formatstring on` even though `--mode aggressive` (and
//! `auto` under 500 KiB) names it -- the divergence DIV-66 records.  Everything
//! shared now lives in [`decompile_one`]; a caller supplies only the facts it
//! actually has (the console its `map addr` / `parse line` / `override` state,
//! the whole-binary loop nothing but the DWARF locals and the no-return flow
//! prunes).
//!
//! # Format-string varargs typing (FormatStringAnalyzer half B)
//!
//! The kuna analog of Ghidra's `FormatStringAnalyzer` (DecompilerDependent):
//! after the first decompile of a caller, walk its `CALL` ops, find calls to
//! printf/scanf-family variadic functions, read the format-string constant at
//! the call's format argument, parse its specifiers, and build a per-call-site
//! prototype override (fixed types ++ format-derived varargs, varargs closed).
//! Ghidra origins are cited inline; the parser + override-pieces builder live in
//! `kuna_analysis::formatstring` (halves A + B-logic).  Gated OFF by default
//! (`Architecture::analysis_formatstring`), so the loop is inert -- and both
//! surfaces byte-identical to their pre-DIV-66 output -- unless the run enables
//! `formatstring`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4, uintb};
use kuna_decomp::architecture::Architecture;
use kuna_decomp::database::DynamicSymbolSpec;
use kuna_decomp::dtype::Datatype;
use kuna_decomp::fspec::{ParameterPieces, PrototypePieces};
use kuna_decomp::funcdata::Funcdata;

/// Every console-only fact a decompile may be seeded with, plus the two facts
/// both surfaces carry (`mapped_symbols`, `flow_overrides`).
///
/// The whole-binary loop leaves the console-only slices empty; the field order
/// mirrors `decompile_func_full_with_override_dyn`'s argument order so the
/// forwarding stays checkable by eye.
pub struct DecompileSeed<'a> {
    /// `map addr` symbols ++ the function's DWARF stack locals.
    pub mapped_symbols: &'a [(String, Rc<Datatype>, Address, uint4)],
    /// `type varnode %REG(pc)` usepoint-scoped symbols (console only).
    pub usepoint_symbols: &'a [(String, Rc<Datatype>, Address, uint4, Address, bool)],
    /// `map hash` dynamic symbols (console only).
    pub dynamic_symbols: &'a [DynamicSymbolSpec],
    /// `parse line extern <decl>` prototype for this function (console only).
    pub pending_proto: Option<&'a PrototypePieces>,
    /// `override flow` facts ++ the analysis's `call error(nonzero,..)`
    /// no-return prunes.
    pub flow_overrides: &'a [(Address, uint4)],
    /// `map param <i> <addr> <decl>` storage locks (console only).
    pub mapped_params: &'a [(int4, String, ParameterPieces)],
}

impl DecompileSeed<'_> {
    /// A seed carrying only the two facts the whole-binary loop has.
    pub fn plain<'a>(
        mapped_symbols: &'a [(String, Rc<Datatype>, Address, uint4)],
        flow_overrides: &'a [(Address, uint4)],
    ) -> DecompileSeed<'a> {
        DecompileSeed {
            mapped_symbols,
            usepoint_symbols: &[],
            dynamic_symbols: &[],
            pending_proto: None,
            flow_overrides,
            mapped_params: &[],
        }
    }
}

/// What [`decompile_one`] produced: the drive result, plus the format-string
/// call-site overrides it discovered (empty unless `formatstring` is on and the
/// function calls a printf/scanf-family function with a readable constant format
/// string).
///
/// The console persists `discovered` into its `pending_proto_overrides` so a
/// later re-decompile of the same function keeps the typing; the whole-binary
/// loop discards it (each function is decompiled once).
pub struct DecompileStep {
    pub result: KunaResult<Funcdata>,
    pub discovered: Vec<(Address, PrototypePieces)>,
}

/// Decompile one function -- the single source of truth for both surfaces.
///
/// Runs `decompile_func_full_with_override_dyn` and, when the run enables
/// `formatstring`, Ghidra's `FormatStringAnalyzer` half-B loop on top of it:
/// inspect the first decompile for printf/scanf-family calls, build
/// per-call-site varargs overrides from their format-string constants and -- if
/// any are new -- decompile a second time with them installed, so the variadic
/// args render typed.
///
/// `proto_overrides` are the overrides already installed for this function (the
/// console's `override prototype` facts; empty on the whole-binary path); a
/// hand-typed override at a callpoint always wins over a discovered one.
///
/// The `readonlypropagate` flip is scoped to this call and restored before
/// returning: to *type* a variadic call we must first *read* its format-string
/// constant, and on ARM the format address is loaded PC-relatively from a
/// literal pool, so the format-arg varnode is a memory LOAD that only
/// constant-folds when `Funcdata::fillin_read_only` is enabled.  Ghidra's
/// `FormatStringAnalyzer` decompile likewise reads constant strings out of
/// read-only memory.  Gate-safe: with `formatstring` off the value is unchanged,
/// so every parity gate is byte-identical.
pub fn decompile_one(
    arch: &mut Architecture,
    name: &str,
    entry: Address,
    size: int4,
    seed: &DecompileSeed<'_>,
    proto_overrides: &[(Address, PrototypePieces)],
) -> DecompileStep {
    let formatstring_enabled = arch.analysis_formatstring;
    let saved_readonlypropagate = arch.readonlypropagate;
    if formatstring_enabled {
        arch.readonlypropagate = true;
    }
    let mut result = kuna_decomp::decompile_drive::decompile_func_full_with_override_dyn(
        arch,
        name,
        // Cloned so `entry` survives for the conditional re-decompile below
        // (`formatstring` off => the clone is the only change and the second
        // call never runs).
        entry.clone(),
        size,
        seed.mapped_symbols,
        seed.usepoint_symbols,
        seed.dynamic_symbols,
        seed.pending_proto,
        seed.flow_overrides,
        proto_overrides,
        seed.mapped_params,
    );
    let mut discovered: Vec<(Address, PrototypePieces)> = Vec::new();
    if formatstring_enabled {
        if let Ok(fd) = &result {
            let mut merged = proto_overrides.to_vec();
            for (callpoint, pieces) in extract_format_string_overrides(arch, fd) {
                // Only the call-site overrides not already installed (a
                // hand-typed `override prototype` at the same callpoint wins).
                if !merged.iter().any(|(a, _)| a == &callpoint) {
                    merged.push((callpoint.clone(), pieces.clone()));
                    discovered.push((callpoint, pieces));
                }
            }
            if !discovered.is_empty() {
                result = kuna_decomp::decompile_drive::decompile_func_full_with_override_dyn(
                    arch,
                    name,
                    entry,
                    size,
                    seed.mapped_symbols,
                    seed.usepoint_symbols,
                    seed.dynamic_symbols,
                    seed.pending_proto,
                    seed.flow_overrides,
                    &merged,
                    seed.mapped_params,
                );
            }
        }
    }
    arch.readonlypropagate = saved_readonlypropagate;
    DecompileStep { result, discovered }
}

/// Resolve a `CALL` argument varnode to the constant pointer it carries, if any
/// (the kuna analog of reading `callOp.getInput(paramCount).getAddress()` in
/// Ghidra's `PcodeFunctionParser.searchForVariadicCallData`, `:99`).
///
/// In kuna's IR the format-string pointer is rarely a bare constant varnode at
/// the call site; it is typically a value produced by a `PTRSUB`/`PTRADD`/
/// `COPY`/`CAST`/`INT_ADD` op whose constant input is the read-only data address
/// (e.g. `u0x… = ->(#0x0,#0x402004)`).  So if the arg varnode is itself a
/// constant we take its offset; otherwise we follow its single defining op and
/// look for a constant input that is *not* the spacebase/zero base.  Returns the
/// resolved code-space offset (the format-string VMA) or `None`.
pub(crate) fn resolve_const_pointer(
    fd: &Funcdata,
    vn_id: kuna_decomp::context::VarnodeId,
) -> Option<uintb> {
    let vn = fd.vbank().get(vn_id)?;
    if vn.is_constant() {
        return Some(vn.get_offset());
    }
    // Follow the defining op and harvest its constant input (skip a #0 base, the
    // common `->(#0x0,#0xVMA)` PTRSUB shape).
    let def = vn.get_def()?;
    let op = fd.obank().get(def)?;
    use kuna_num::opcodes::OpCode;
    match op.code() {
        OpCode::CPUI_PTRSUB
        | OpCode::CPUI_PTRADD
        | OpCode::CPUI_INT_ADD
        | OpCode::CPUI_COPY
        | OpCode::CPUI_CAST => {
            let mut acc: Option<uintb> = None;
            for slot in 0..op.num_input() {
                if let Some(in_id) = op.get_in(slot) {
                    if let Some(in_vn) = fd.vbank().get(in_id) {
                        if in_vn.is_constant() {
                            let off = in_vn.get_offset();
                            // Skip a zero base; sum the rest (PTRADD/INT_ADD).
                            if off != 0 {
                                acc = Some(acc.unwrap_or(0).wrapping_add(off));
                            }
                        }
                    }
                }
            }
            acc
        }
        _ => None,
    }
}

/// Read a NUL-terminated byte string out of the load image at `vma` (the kuna
/// analog of Ghidra resolving the constant address to its defined string `Data`
/// in `PcodeFunctionParser`).  Reads byte-by-byte via the architecture's
/// `read_loadimage_value`; stops at the first NUL or after `MAX` bytes, and bails
/// (returns `None`) on the first unreadable byte (out of any mapped section).
pub(crate) fn read_cstring(arch: &Architecture, vma: uintb) -> Option<String> {
    const MAX: u64 = 4096;
    // The default code/data space (the loadimage's addressable space).
    let space = Rc::clone(arch.manage().get_default_code_space()?);
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..MAX {
        let addr = Address::new(Rc::clone(&space), vma.wrapping_add(i));
        let b = arch.read_loadimage_value(&addr, 1).ok()? as u8;
        if b == 0 {
            // A non-empty, terminated string only.
            return if bytes.is_empty() {
                None
            } else {
                String::from_utf8(bytes).ok()
            };
        }
        bytes.push(b);
    }
    None
}

/// Walk the decompiled `fd`'s `CALL` ops and produce per-call-site prototype
/// overrides for printf/scanf-family variadic calls whose format string is a
/// readable constant (the kuna analog of `FormatStringAnalyzer.decompile` +
/// `PcodeFunctionParser.parseFunctionForCallData` + `overrideCallList`).
///
/// For each `CALL`:
/// 1. find its `FuncCallSpecs` (callee name + recovered proto);
/// 2. classify the callee name (`printf`/`scanf` family — Ghidra
///    `VARIADIC_SUBSTRINGS`); a non-variadic callee is skipped;
/// 3. require the recovered proto be varargs with a char-pointer last fixed param
///    (Ghidra `usesVariadicFormatString`, `:229`) — using the recovered
///    `num_params` as the format slot (`getInput(paramCount)`, `:99`);
/// 4. resolve the format-arg varnode to its constant VMA and read the string;
/// 5. parse it ([`parse_output_types`]/[`parse_input_types`]) and, on a non-empty
///    spec list, build the override [`PrototypePieces`]
///    ([`apply::build_override_pieces`]).
///
/// Returns the `(callpoint Address, PrototypePieces)` pairs to install.
pub(crate) fn extract_format_string_overrides(
    arch: &Architecture,
    fd: &Funcdata,
) -> Vec<(Address, PrototypePieces)> {
    use kuna_analysis::formatstring::{self, apply};
    use kuna_num::opcodes::OpCode;

    let mut overrides = Vec::new();
    let (_addr_size, word_size) = arch.data_org();
    let types = arch.types();

    // Iterate the call sites (the analog of Ghidra walking the HighFunction's
    // CALL ops; FuncCallSpecs already pairs each CALL op with the recovered
    // callee proto).
    for i in 0..fd.num_calls() {
        let spec = fd.get_call_specs(i);
        let op_id = spec.get_op();
        let Some(op) = fd.obank().get(op_id) else {
            continue;
        };
        // Direct CALL only (CALLIND/CALLOTHER have no fixed callee here).
        if op.code() != OpCode::CPUI_CALL {
            continue;
        }
        let callpoint = op.get_addr().clone();
        // (2) Classify the callee name (printf/scanf family).
        let Some(is_output) = apply::classify_variadic_call(spec.get_name()) else {
            continue;
        };
        // (3) usesVariadicFormatString: recovered proto must be varargs with a
        // char-pointer last fixed param.  We require `is_dotdotdot()` and at
        // least one fixed param; the format string is the *last* fixed param,
        // at CALL input slot `num_params` (slot 0 is the call target — Ghidra
        // `getInput(getParameterCount())`, `:99`).
        let num_params = spec.proto().num_params();
        if !spec.is_dotdotdot() || num_params <= 0 {
            continue;
        }
        // The call must actually pass arguments beyond the fixed ones (Ghidra
        // `callOp.getNumInputs() <= function.getParameterCount()` → skip, `:81`).
        if op.num_input() <= num_params {
            continue;
        }
        // (4) Read the format constant at the format-arg slot.
        let Some(fmt_vn) = op.get_in(num_params) else {
            continue;
        };
        let Some(vma) = resolve_const_pointer(fd, fmt_vn) else {
            continue;
        };
        let Some(fmt) = read_cstring(arch, vma) else {
            continue;
        };
        // A format string without specifiers is a no-op (Ghidra parses but the
        // empty type list yields no override).
        if !fmt.contains('%') {
            continue;
        }
        // (5) Parse + build the override pieces.
        let specs = if is_output {
            formatstring::parse_output_types(&fmt)
        } else {
            formatstring::parse_input_types(&fmt)
        };
        if specs.is_empty() {
            continue;
        }
        // Collect the callee's recovered *fixed* param types (the analog of
        // Ghidra `namesToParameters.get(callFunctionName)` = the declared params
        // minus the trailing "...").
        let mut fixed: Vec<Rc<Datatype>> =
            Vec::with_capacity(num_params as usize);
        let mut ok = true;
        for p in 0..num_params {
            match spec.proto().get_param(p).and_then(|pp| pp.get_type().cloned()) {
                Some(ty) => fixed.push(ty),
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        // The callee return type (Ghidra `namesToReturn.get(...)`).
        let outtype = spec.proto().get_output().get_type().cloned();
        match apply::build_override_pieces(
            spec.get_name(),
            outtype,
            &fixed,
            &specs,
            types,
            word_size,
        ) {
            Ok(Some(pieces)) => overrides.push((callpoint, pieces)),
            Ok(None) => {}
            Err(_) => {} // a spec→datatype build failure: skip this call site.
        }
    }
    overrides
}
