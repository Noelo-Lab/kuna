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
use kuna_decomp::kuna_formatstring::ParkedFormatSite;

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
    decompile_one_prefollowed(arch, name, entry, size, seed, proto_overrides, None)
}

/// [`decompile_one`], but able to adopt an already-followed `Funcdata` for the
/// FIRST drive instead of following the same flow a second time (see
/// `kuna_decomp::decompile_drive::decompile_func_full_with_override_dyn_prefollowed`).
///
/// Only the first drive can adopt it: the format-string re-decompile installs
/// new per-call-site prototype overrides, which are consumed AT FLOW TIME, so
/// that second pass always re-follows.
pub fn decompile_one_prefollowed(
    arch: &mut Architecture,
    name: &str,
    entry: Address,
    size: int4,
    seed: &DecompileSeed<'_>,
    proto_overrides: &[(Address, PrototypePieces)],
    prefollowed: Option<Funcdata>,
) -> DecompileStep {
    let formatstring_mode = arch.analysis_formatstring;
    let saved_readonlypropagate = arch.readonlypropagate;
    // Only the LOOP needs the drive to fold the format constant out of read-only
    // memory; the static resolver read it out of the image at load.
    if formatstring_mode.loop_pass() {
        arch.readonlypropagate = true;
    }
    // (kuna `formatstring static`) The per-call-site overrides the load-time
    // resolver parked for THIS function. A caller-supplied override at the same
    // callpoint wins, exactly as a hand-typed `override prototype` outranks a
    // discovered one below.
    let parked: Vec<ParkedFormatSite> = if formatstring_mode.statik() {
        match arch.format_call_overrides.get(&entry.get_offset()) {
            Some(sites) if entry.get_space().is_some() => sites
                .iter()
                .filter(|s| !proto_overrides.iter().any(|(a, _)| a.get_offset() == s.callpoint))
                .cloned()
                .collect(),
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };
    let first_overrides: Vec<(Address, PrototypePieces)> = if parked.is_empty() {
        proto_overrides.to_vec()
    } else {
        let space = entry.get_space().expect("checked above");
        let mut merged = proto_overrides.to_vec();
        merged.extend(
            parked
                .iter()
                .map(|s| (Address::new(Rc::clone(space), s.callpoint), s.pieces.clone())),
        );
        merged
    };
    let proto_overrides: &[(Address, PrototypePieces)] = &first_overrides;
    // A RET-call chain beginning at the function entry is safe to recognize
    // without an assertion: every link stored its own fall-through before the
    // RETURN, which distinguishes it from the incoming return address and from
    // ordinary RET/RET-immediate instructions. Seed the derived CALLs first so
    // an explicit flow assertion at the same address remains authoritative.
    let entry_chain = if arch.entry_ret_dispatch {
        crate::kuna_retcallchain::kuna_entry_chain_sites(
            arch.translate(),
            &entry,
            crate::kuna_retcallchain::CHAIN_MAX_SITES,
            crate::kuna_retcallchain::CHAIN_MAX_INSNS,
        )
    } else {
        Vec::new()
    };
    let push_immediate_ret = if arch.push_immediate_ret {
        crate::kuna_retcallchain::kuna_push_immediate_ret(
            arch.translate(),
            &entry,
            crate::kuna_retcallchain::CHAIN_MAX_INSNS,
        )
    } else {
        None
    };
    let has_derived_flow = !entry_chain.is_empty() || push_immediate_ret.is_some();
    let mut flow_overrides = entry_chain
        .iter()
        // An explicit fact at this address owns the classification. In
        // particular, `flow ... return` vetoes the derived CALL.
        .filter(|site| !seed.flow_overrides.iter().any(|(at, _)| at == *site))
        .cloned()
        .map(|site| (site, kuna_decomp::overrides::flow_type::CALL))
        .collect::<Vec<_>>();
    if let Some((site, _target)) = push_immediate_ret.as_ref() {
        if !seed.flow_overrides.iter().any(|(at, _)| at == site) {
            flow_overrides.push((site.clone(), kuna_decomp::overrides::flow_type::BRANCH));
        }
    }
    flow_overrides.extend(seed.flow_overrides.iter().filter_map(|(site, kind)| {
        // RETURN on a raw RETURN link is the original instruction semantics,
        // not an override the engine can apply. Treat it as a clean veto.
        if *kind == kuna_decomp::overrides::flow_type::RETURN
            && (entry_chain.iter().any(|derived| derived == site)
                || push_immediate_ret
                    .as_ref()
                    .is_some_and(|(derived, _)| derived == site))
        {
            None
        } else {
            Some((site.clone(), *kind))
        }
    }));
    // `load function` followed the entry before automatic recognition ran.
    // Rebuild only this uncommon shape; every ordinary function still adopts
    // the already-followed IR. A parked format-string override is the same
    // shape: prototype overrides are consumed AT FLOW TIME, so IR followed
    // without them does not carry the typing.
    let prefollowed = if has_derived_flow || !parked.is_empty() { None } else { prefollowed };
    let mut result = kuna_decomp::decompile_drive::decompile_func_full_with_override_dyn_prefollowed(
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
        &flow_overrides,
        proto_overrides,
        seed.mapped_params,
        prefollowed,
    );
    // A parked override the drive contradicts is withdrawn, and the function is
    // driven again without it. See [`audit_parked_format_sites`].
    let dropped: Vec<u64> = match &result {
        Ok(fd) if !parked.is_empty() => audit_parked_format_sites(arch, fd, &parked),
        _ => Vec::new(),
    };
    let mut discovered: Vec<(Address, PrototypePieces)> = Vec::new();
    if formatstring_mode.loop_pass() {
        if let Ok(fd) = &result {
            for (callpoint, pieces) in extract_format_string_overrides(arch, fd) {
                // Only the call-site overrides not already installed (a
                // hand-typed `override prototype` at the same callpoint wins).
                if !proto_overrides.iter().any(|(a, _)| a == &callpoint)
                    && !discovered.iter().any(|(a, _)| a == &callpoint)
                {
                    discovered.push((callpoint, pieces));
                }
            }
            if std::env::var_os("KUNA_FMTSTATIC_DEBUG").is_some() {
                for (at, _) in &discovered {
                    eprintln!("fmtloop: site only the loop found {:#x}", at.get_offset());
                }
            }
        }
    }
    if !dropped.is_empty() || !discovered.is_empty() {
        let mut merged: Vec<(Address, PrototypePieces)> = proto_overrides
            .iter()
            .filter(|(a, _)| !dropped.contains(&a.get_offset()))
            .cloned()
            .collect();
        merged.extend(discovered.iter().cloned());
        result = kuna_decomp::decompile_drive::decompile_func_full_with_override_dyn(
            arch,
            name,
            entry,
            size,
            seed.mapped_symbols,
            seed.usepoint_symbols,
            seed.dynamic_symbols,
            seed.pending_proto,
            &flow_overrides,
            &merged,
            seed.mapped_params,
        );
    }
    arch.readonlypropagate = saved_readonlypropagate;
    DecompileStep { result, discovered }
}

/// (kuna `formatstring static`) The parked call sites whose override the drive
/// contradicts, by callpoint.
///
/// The load-time resolver reads the format out of a window of the call's own
/// block, and the Listing it walks back through does not know every edge into
/// that block: a join reached only from the case bodies of a jump table looks
/// like straight-line code there. The drive does know them, so a site is kept
/// only while the drive agrees on two things:
///
/// - the format argument is the resolved constant — every value that can reach
///   it is that string, directly, through a read-only load, or as the msgid of a
///   `gettext`-family call; and
/// - the call passes exactly the arguments the override declares. In a function
///   whose stack pointer the drive cannot track (an `alloca` frame) a closed
///   prototype picks up the slot the call pushes its return address into as one
///   more argument; the open varargs prototype does not.
///
/// A site whose call is not in the drive at all is kept: the override has
/// nothing to act on.
fn audit_parked_format_sites(
    arch: &Architecture,
    fd: &Funcdata,
    parked: &[ParkedFormatSite],
) -> Vec<u64> {
    use kuna_num::opcodes::OpCode;
    let mut dropped: Vec<u64> = Vec::new();
    for i in 0..fd.num_calls() {
        let Some(op) = fd.obank().get(fd.get_call_specs(i).get_op()) else { continue };
        if op.code() != OpCode::CPUI_CALL {
            continue;
        }
        let at = op.get_addr().get_offset();
        let Some(site) = parked.iter().find(|s| s.callpoint == at) else { continue };
        if dropped.contains(&at) {
            continue;
        }
        let declared = site.pieces.intypes.len() as int4;
        let arity = op.num_input() - 1 == declared;
        let format = op.get_in(site.format_slot as int4 + 1).is_some_and(|vn| {
            let mut seen = Vec::new();
            format_is(arch, fd, vn, site.format_vma, &mut seen)
        });
        if !(arity && format) {
            if std::env::var_os("KUNA_FMTSTATIC_DEBUG").is_some() {
                eprintln!(
                    "fmtaudit: withdrew {at:#x} (args {} vs declared {declared}, format {})",
                    op.num_input() - 1,
                    if format { "agrees" } else { "differs" }
                );
            }
            dropped.push(at);
        }
    }
    dropped
}

/// Is every value that can reach `vn_id` the string at `vma`?
fn format_is(
    arch: &Architecture,
    fd: &Funcdata,
    vn_id: kuna_decomp::context::VarnodeId,
    vma: uintb,
    seen: &mut Vec<kuna_decomp::context::VarnodeId>,
) -> bool {
    use kuna_num::opcodes::OpCode;
    if seen.contains(&vn_id) {
        return true;
    }
    if seen.len() >= 64 {
        return false;
    }
    seen.push(vn_id);
    if let Some(v) = const_value(fd, vn_id, 0) {
        return v == vma;
    }
    let Some(def) = fd.vbank().get(vn_id).and_then(|vn| vn.get_def()) else { return false };
    let Some(op) = fd.obank().get(def) else { return false };
    let input = |slot: int4| op.get_in(slot);
    match op.code() {
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input())
            .all(|i| input(i).is_some_and(|v| format_is(arch, fd, v, vma, seen))),
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => {
            input(0).is_some_and(|v| format_is(arch, fd, v, vma, seen))
        }
        OpCode::CPUI_LOAD => {
            let size = fd.vbank().get(vn_id).map_or(0, |vn| vn.get_size());
            input(1).and_then(|a| const_value(fd, a, 0)).is_some_and(|at| {
                let Some(space) = arch.manage().get_default_code_space() else { return false };
                arch.read_loadimage_value(&Address::new(Rc::clone(space), at), size)
                    .is_ok_and(|v| v == vma)
            })
        }
        OpCode::CPUI_CALL => {
            let Some(spec) = fd.get_call_specs_index(def) else { return false };
            let name = fd.get_call_specs(spec).get_name();
            kuna_analysis::formatstring::kuna_fmtstatic::gettext_msgid_slot(name).is_some_and(
                |slot| input(slot as int4 + 1).is_some_and(|v| format_is(arch, fd, v, vma, seen)),
            )
        }
        _ => false,
    }
}

/// The value of `vn_id` when it is a constant or an address arithmetic of
/// constants (the `->(#0x0,#0xVMA)` shape a global's address takes).
fn const_value(fd: &Funcdata, vn_id: kuna_decomp::context::VarnodeId, depth: u32) -> Option<uintb> {
    use kuna_num::opcodes::OpCode;
    let vn = fd.vbank().get(vn_id)?;
    if vn.is_constant() {
        return Some(vn.get_offset());
    }
    if depth >= 4 {
        return None;
    }
    let op = fd.obank().get(vn.get_def()?)?;
    let arg = |slot: int4| op.get_in(slot).and_then(|v| const_value(fd, v, depth + 1));
    match op.code() {
        OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD => Some(arg(0)?.wrapping_add(arg(1)?)),
        OpCode::CPUI_PTRADD => Some(arg(0)?.wrapping_add(arg(1)?.wrapping_mul(arg(2)?))),
        _ => None,
    }
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

/// Is every byte of `[vma, vma + len)` painted read-only (the loader's
/// read-only sections, or an XML `readonly` bytechunk)?
fn is_readonly_span(arch: &Architecture, vma: uintb, len: u64) -> bool {
    let Some(space) = arch.manage().get_default_code_space() else { return false };
    (0..len).all(|i| {
        let addr = Address::new(Rc::clone(space), vma.wrapping_add(i));
        arch.symboltab.get_property(&addr) & kuna_decomp::varnode::varnode_flags::readonly != 0
    })
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
        if !is_readonly_span(arch, vma, fmt.len() as u64 + 1) {
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
            kuna_decomp::kuna_formatstring::target_vararg_abi(arch),
        ) {
            Ok(Some(pieces)) => overrides.push((callpoint, pieces)),
            Ok(None) => {}
            Err(_) => {} // a spec→datatype build failure: skip this call site.
        }
    }
    overrides
}
