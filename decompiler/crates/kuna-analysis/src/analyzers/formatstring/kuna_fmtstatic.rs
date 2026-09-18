//! (kuna `formatstring static`) The **load-time** half of format-string varargs
//! typing: resolve each printf/scanf-family call site's format constant out of
//! the image, and park the per-call-site prototype override before anything is
//! decompiled.
//!
//! Ghidra's `FormatStringAnalyzer` is `DecompilerDependent` because it reads the
//! format pointer off the lifted `CALL` op, which means decompiling the caller,
//! installing the override and decompiling it again. That second decompile is
//! the whole cost of the feature (+43% to +77% on `decompile-all`), and it is why
//! the option is a per-run opt-in.
//!
//! The constant is in the image, though, and so is the call site. This pass finds
//! both without the decompiler:
//!
//! 1. every resolver import whose built-in signature is variadic with a `char *`
//!    last fixed parameter ([`crate::protos::variadic_format_prototype`]) is a
//!    format function, and the signature says which parameter the format is;
//! 2. the Listing's edges into that callee ([`Listing::refs_to`]) give its call
//!    sites — CALL edges, plus the tail jumps (`jmp printf@plt`), which gcc -O2
//!    emits for a forwarder and which the lifter turns into a CALL at the jump's
//!    own address — and `function_containing` gives each site's caller;
//! 3. the format parameter's ABI storage comes from the prototype model
//!    (`assign_parameter_storage`), so the register is derived from the cspec
//!    rather than hard-coded per architecture;
//! 4. a bounded backward window of the call site's own basic block is re-lifted
//!    to p-code and constant-folded forward ([`fold`]); a format argument that is
//!    not a constant in that window is simply not resolved.
//!
//! The resolved VMA is read as a NUL-terminated string, parsed by the shared
//! [`super`] parser, and turned into the same [`PrototypePieces`] override the
//! decompile-time loop builds — so the decompiler side is unchanged, and
//! `formatstring full` still runs the loop for whatever this pass could not
//! answer.
//!
//! # What it declines
//!
//! Anything it cannot prove: no Listing, no edge into the callee, a format
//! argument written outside the call's own block, a value that is not constant
//! there, a call between the write and the use (which clobbers the register), an
//! address with no readable NUL-terminated string behind it, a string in a
//! writable section (the program can rewrite it before the call), a format with
//! no conversions, a format carrying `%Lf` (whose x86-64 ABI class the override
//! pieces cannot spell), and a conversion the target does not pass the way it
//! passes a named argument ([`VarargAbi`]: every site on Apple AArch64, a
//! floating or double-width one on ARM32, RISC-V, MIPS, PowerPC and Windows on
//! AArch64). An indirect call through a function pointer has no Listing edge at
//! all, so a `printf` reached that way is invisible here. Declining costs only
//! the typing, and `full` remains available.
//!
//! # The `gettext` hop
//!
//! GNU programs never pass the literal: they pass `_(...)`, i.e.
//! `dcgettext(NULL, "…%s…", 5)`. The translation a `gettext` returns has the same
//! conversions as its msgid — that is what `xgettext`'s `c-format` check
//! enforces — so a format argument produced by a `gettext`/`dgettext`/`dcgettext`
//! call is resolved through that call's msgid parameter. Without this hop the
//! pass finds almost nothing in coreutils, grep, tar or findutils.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::{int4, uintb};
use kuna_decomp::architecture::Architecture;
use kuna_decomp::fspec::{ParameterPieces, PrototypePieces, ProtoModel};
use kuna_decomp::kuna_formatstring::{self, ImageFamily, VarargAbi};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::listing::{Listing, RefKind};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, FormatSiteFact, Phase};

use super::apply;

/// How many instructions before the call site the resolver will look at. A
/// format pointer is materialized within a handful of instructions of the call
/// in every calling convention; the bound is what keeps the pass proportional to
/// the number of format call sites rather than to the program.
const WINDOW_INSNS: usize = 48;

/// The `gettext` family, by the parameter index holding the msgid — the format
/// string the returned translation shares.
const GETTEXT_FAMILY: &[(&str, usize)] =
    &[("gettext", 0), ("dgettext", 1), ("dcgettext", 1), ("__gettext", 0)];

/// Which parameter of the `gettext`-family function `name` is its msgid.
pub fn gettext_msgid_slot(name: &str) -> Option<usize> {
    GETTEXT_FAMILY.iter().find(|(n, _)| *n == name).map(|&(_, slot)| slot)
}

/// `KUNA_FMTSTATIC_DEBUG`: trace every call site the resolver considered and what
/// it decided, to stderr — the same shape as the `KUNA_CONDFOLD_DEBUG` /
/// `KUNA_RS_DEBUG` decision traces. Read once per pass, never in the fold.
fn trace_enabled() -> bool {
    std::env::var_os("KUNA_FMTSTATIC_DEBUG").is_some()
}

/// Resolve printf/scanf-family format strings at load and emit the per-call-site
/// prototype overrides.
pub struct FormatStringStaticPass;

impl AnalysisPass for FormatStringStaticPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "formatstring"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let Some(listing) = ctx.listing else { return out };
        if !listing.has_refs() {
            return out;
        }
        let abi = ctx.arch.format_vararg_abi.unwrap_or_else(|| {
            kuna_formatstring::vararg_abi(&ctx.arch.archid, Some(image_family(ctx.bytes)))
        });
        if abi == VarargAbi::Nothing {
            return out;
        }
        let Some(model) = ctx.arch.default_fp().cloned() else { return out };
        let types = ctx.arch.types();
        let (_addr_size, word_size) = ctx.arch.data_org();
        // The same layout decision `LibcTypesPass` makes, so an override's fixed
        // parameters carry the very `FILE *` the callee's parked prototype does.
        let layout = crate::protos::effective_libctypes_layout(ctx.file);
        let imports = crate::protos::resolved_import_addrs(ctx.file, ctx.bytes);
        let dbg = trace_enabled();

        // One lift per instruction address, shared by every call site whose
        // window covers it.
        let mut lifts: LiftCache = HashMap::new();

        // The `gettext` hop table: callee VMA -> (msgid parameter storage,
        // return storage).
        let mut gettext: HashMap<u64, (Key, Key)> = HashMap::new();
        for (name, addr) in &imports {
            let Some(slot) = gettext_msgid_slot(name) else { continue };
            let Some(pieces) =
                crate::protos::declared_libc_prototype(name, types, word_size, layout)
            else {
                continue;
            };
            let Some(assigned) = assign(&model, &pieces, ctx.arch) else { continue };
            // `assigned[0]` is the return storage; the parameters follow.
            let (Some(ret), Some(msgid)) = (assigned.first(), assigned.get(slot + 1)) else {
                continue;
            };
            let (Some(ret), Some(msgid)) = (storage_key(ret), storage_key(msgid)) else {
                continue;
            };
            gettext.insert(*addr, (ret, msgid));
        }

        for (name, addr) in &imports {
            let Some((callee, slot)) =
                crate::protos::variadic_format_prototype(name, types, word_size, layout)
            else {
                continue;
            };
            if dbg {
                eprintln!("fmtstatic: callee {name} @ {addr:#x}, format at parameter {slot}");
            }
            let Some(assigned) = assign(&model, &callee, ctx.arch) else { continue };
            let Some(store) = assigned.get(slot + 1).and_then(storage_key) else { continue };
            // `isOutputType = !name.contains("scanf")` — Ghidra
            // `FormatStringAnalyzer.parseParameters` (`:273`), widened to the
            // families the substring test never named (`err`/`warn`/`syslog`),
            // which are all output.
            let is_output = apply::classify_variadic_call(name).unwrap_or(true);
            for edge in listing.refs_to(*addr) {
                // A tail call is a format call site too: `jmp printf@plt` hands
                // the callee the caller's own argument registers, and the lifter
                // turns the jump into a CALL at the jump's own address, so the
                // override keys the same way. Such an edge is a `Code` edge (the
                // Listing only calls a CALL/CALLIND a `Call`); a fall-through
                // into the stub is not one.
                let tail_jump =
                    edge.kind == RefKind::Code && !falls_into(listing, edge.from, *addr);
                if edge.kind != RefKind::Call && !tail_jump {
                    continue;
                }
                if dbg {
                    eprintln!(
                        "fmtstatic:   site {:#x}{}",
                        edge.from,
                        if tail_jump { " (tail jump)" } else { "" }
                    );
                }
                let Some(func) = listing.function_containing(edge.from) else { continue };
                let entry = func.entry;
                let Some(vma) =
                    resolve_at(ctx, listing, edge.from, &store, &gettext, &mut lifts, dbg)
                else {
                    if dbg {
                        eprintln!("fmtstatic:   unresolved");
                    }
                    continue;
                };
                if dbg {
                    eprintln!("fmtstatic:   resolved {vma:#x}");
                }
                let Some(fmt) = read_cstring(ctx.arch, vma) else { continue };
                if !fmt.contains('%') {
                    continue;
                }
                if !in_readonly_section(ctx, vma, fmt.len() as u64 + 1) {
                    if dbg {
                        eprintln!("fmtstatic:   {vma:#x} is writable");
                    }
                    continue;
                }
                let specs = if is_output {
                    super::parse_output_types(&fmt)
                } else {
                    super::parse_input_types(&fmt)
                };
                if specs.is_empty() {
                    continue;
                }
                match apply::build_override_pieces(
                    name,
                    callee.outtype.clone(),
                    &callee.intypes,
                    &specs,
                    types,
                    word_size,
                    abi,
                ) {
                    Ok(Some(pieces)) => out.format_sites.push(FormatSiteFact {
                        func: entry,
                        callpoint: edge.from,
                        pieces,
                        format_slot: slot,
                        format_vma: vma,
                    }),
                    _ => continue,
                }
            }
        }
        out.format_sites.sort_by(|a, b| (a.func, a.callpoint).cmp(&(b.func, b.callpoint)));
        out.format_sites.dedup_by(|a, b| a.func == b.func && a.callpoint == b.callpoint);
        out
    }
}

/// The storage the prototype model assigns to `pieces` — index 0 is the return,
/// the parameters follow. `None` when the model refuses, or when a hidden
/// return-buffer parameter shifts the map (no format function has one, so the
/// shift is a reason to decline rather than to compensate).
fn assign(
    model: &Rc<ProtoModel>,
    pieces: &PrototypePieces,
    arch: &Architecture,
) -> Option<Vec<ParameterPieces>> {
    let mut res: Vec<ParameterPieces> = Vec::new();
    model
        .assign_parameter_storage(pieces, &mut res, true, arch.types(), arch.manage())
        .ok()?;
    let hidden = res
        .iter()
        .any(|p| p.flags & kuna_decomp::fspec::parameter_pieces_flags::HIDDENRETPARM != 0);
    (!hidden).then_some(res)
}

/// Read the NUL-terminated byte string at `vma` out of the load image. Bails on
/// the first unreadable byte, on an empty string, and on non-UTF-8 — the same
/// contract as the decompile-time reader.
fn read_cstring(arch: &Architecture, vma: uintb) -> Option<String> {
    const MAX: u64 = 4096;
    let space = Rc::clone(arch.manage().get_default_code_space()?);
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..MAX {
        let addr = Address::new(Rc::clone(&space), vma.wrapping_add(i));
        let b = arch.read_loadimage_value(&addr, 1).ok()? as u8;
        if b == 0 {
            return if bytes.is_empty() { None } else { String::from_utf8(bytes).ok() };
        }
        bytes.push(b);
    }
    None
}

/// The constant in `store` at `call_site`, or `None` if it is not constant in
/// the call's own basic block.
fn resolve_at(
    ctx: &AnalysisCtx,
    listing: &Listing,
    call_site: u64,
    store: &Key,
    gettext: &HashMap<u64, (Key, Key)>,
    lifts: &mut LiftCache,
    dbg: bool,
) -> Option<u64> {
    let chain = backward_block(listing, call_site);
    if dbg {
        eprintln!("fmtstatic:   window {chain:x?}");
    }
    if chain.is_empty() {
        return None;
    }
    fold(ctx, listing, &chain, store, gettext, lifts)
}

/// The instructions of `call_site`'s basic block that precede it, oldest first.
///
/// Walks back one instruction at a time through the Listing's partition and
/// stops at the first block boundary: an address some other instruction branches
/// to, or a gap in the decode.
fn backward_block(listing: &Listing, call_site: u64) -> Vec<u64> {
    let mut chain: Vec<u64> = Vec::new();
    let mut cur = call_site;
    for _ in 0..WINDOW_INSNS {
        // The caller's own entry ends the walk: whatever precedes it belongs to
        // another function.
        if !chain.is_empty() && listing.function_at(cur).is_some() {
            break;
        }
        // A branch target ends the walk: control can arrive at `cur` without
        // having executed whatever precedes it.
        if listing
            .refs_to(cur)
            .iter()
            .any(|r| r.kind != RefKind::Call && !falls_into(listing, r.from, cur))
        {
            break;
        }
        let Some(prev) = previous_instruction(listing, cur) else { break };
        let Some(insn) = listing.instruction_at(prev) else { break };
        // Only a plain fall-through links the two: a conditional branch to the
        // next instruction, or a call, does not carry a register value forward
        // we may rely on (the call case is handled by the fold itself).
        if insn.fall_through != Some(cur) {
            break;
        }
        chain.push(prev);
        cur = prev;
    }
    chain.reverse();
    chain
}

/// Does the instruction at `from` fall through into `to` (as opposed to
/// branching there)?
fn falls_into(listing: &Listing, from: u64, to: u64) -> bool {
    listing.instruction_at(from).is_some_and(|i| i.fall_through == Some(to))
        && from + u64::from(listing.instruction_at(from).map_or(0, |i| i.len)) == to
}

/// The instruction immediately before `vma` in the partition, when it abuts it.
fn previous_instruction(listing: &Listing, vma: u64) -> Option<u64> {
    let lo = vma.saturating_sub(64);
    listing
        .instructions_in_range(lo, Some(vma))
        .filter(|(&addr, insn)| addr + u64::from(insn.len) == vma)
        .map(|(&addr, _)| addr)
        .last()
}

/// A constant-folding key: one storage location.
type Key = (usize, u64, u32);

/// One lifted instruction per address, for the whole pass.
type LiftCache = HashMap<u64, Rc<Lifted>>;

fn key_of(vn: &VarnodeData) -> Option<Key> {
    let space = vn.space.as_ref()?;
    Some((space.get_index() as usize, vn.offset, vn.size))
}

/// The p-code of one instruction, captured whole (opcode, output, inputs).
struct Lifted {
    ops: Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>,
    /// Did the decode succeed? A failed one folds to nothing rather than to the
    /// partial op list it emitted.
    ok: bool,
}

impl PcodeEmit for Lifted {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops.push((opc, outvar.cloned(), vars.to_vec()));
    }
}

/// Constant-fold `chain` forward and read `store` at the end of it.
fn fold(
    ctx: &AnalysisCtx,
    listing: &Listing,
    chain: &[u64],
    store: &Key,
    gettext: &HashMap<u64, (Key, Key)>,
    lifts: &mut LiftCache,
) -> Option<u64> {
    let translate = ctx.arch.translate();
    let space = Rc::clone(ctx.arch.manage().get_default_code_space()?);
    let mut known: HashMap<Key, u64> = HashMap::new();
    for &vma in chain {
        // Neighbouring call sites share most of their window, so one lift per
        // address per binary rather than one per (site, address).
        let lifted = match lifts.entry(vma) {
            std::collections::hash_map::Entry::Occupied(e) => Rc::clone(e.get()),
            std::collections::hash_map::Entry::Vacant(e) => {
                let mut l = Lifted { ops: Vec::new(), ok: true };
                let at = Address::new(Rc::clone(&space), vma);
                l.ok = translate.one_instruction(&mut l, &at).is_ok();
                Rc::clone(e.insert(Rc::new(l)))
            }
        };
        if !lifted.ok {
            known.clear();
            continue;
        }
        // A call clobbers every register the ABI allows it to; the only value it
        // is allowed to leave behind is the `gettext` hop's msgid.
        if let Some(target) = call_target(listing, vma) {
            let hop = gettext
                .get(&target)
                .and_then(|(ret, msgid)| known.get(msgid).map(|value| (*ret, *value)));
            known.clear();
            if let Some((ret, value)) = hop {
                known.insert(ret, value);
            }
            continue;
        }
        apply_ops(&lifted.ops, &mut known, &|at, size| read_const(ctx, at, size));
    }
    known.get(store).copied()
}

/// The fold key for one assigned parameter/return slot: its storage address plus
/// the WIDTH of the type that occupies it (a register space's own address size
/// is not the datum's width).
fn storage_key(p: &ParameterPieces) -> Option<Key> {
    let space = p.addr.get_space()?;
    let size = p.type_.as_ref()?.get_size();
    if size == 0 {
        return None;
    }
    Some((space.get_index() as usize, p.addr.get_offset(), size as u32))
}

/// The direct call target of the instruction at `vma`, if it has one.
fn call_target(listing: &Listing, vma: u64) -> Option<u64> {
    let insn = listing.instruction_at(vma)?;
    if !insn.flow.is_call {
        return None;
    }
    insn.flows.first().copied().or(Some(u64::MAX))
}

/// One instruction's p-code. A branch inside the instruction makes whatever it
/// writes afterwards conditional: x86 `cmovcc` is `if (!cc) goto inst_next;
/// dst = src;`, so its destination is either value, and is killed rather than
/// set.
fn apply_ops(
    ops: &[(OpCode, Option<VarnodeData>, Vec<VarnodeData>)],
    known: &mut HashMap<Key, u64>,
    load: &dyn Fn(u64, u32) -> Option<u64>,
) {
    let mut conditional = false;
    for (opc, outvar, vars) in ops {
        if matches!(opc, OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND) {
            conditional = true;
            continue;
        }
        step(known, *opc, outvar.as_ref(), vars, conditional, load);
    }
}

/// One p-code op of the fold. Anything not modelled kills its output, and so
/// does a write that may not happen (`conditional`).
fn step(
    known: &mut HashMap<Key, u64>,
    opc: OpCode,
    outvar: Option<&VarnodeData>,
    vars: &[VarnodeData],
    conditional: bool,
    load: &dyn Fn(u64, u32) -> Option<u64>,
) {
    let value = match opc {
        OpCode::CPUI_COPY => input(known, vars.first()),
        OpCode::CPUI_INT_ZEXT => input(known, vars.first()),
        OpCode::CPUI_INT_SEXT => input(known, vars.first()).map(|v| {
            let bits = vars[0].size.saturating_mul(8).min(64);
            sign_extend(v, bits)
        }),
        OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR
        | OpCode::CPUI_INT_LEFT
        | OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_SUBPIECE
        | OpCode::CPUI_PIECE => {
            let lo_bytes = vars.get(1).map_or(0, |v| v.size);
            binary(known, vars, |a, b| arith(opc, a, b, lo_bytes))
        }
        // A PC-relative literal-pool load (ARM/Thumb) is how the format pointer
        // reaches its register there; only an address in a section the image
        // initializes and does not write may be folded.
        OpCode::CPUI_LOAD => vars
            .get(1)
            .and_then(|a| input(known, Some(a)))
            .and_then(|at| load(at, outvar.map_or(0, |o| o.size))),
        // A STORE writes memory, not a tracked register, and everything else is
        // unmodelled: kill the output (below) and move on.
        _ => None,
    };
    if opc == OpCode::CPUI_CALLOTHER {
        // A user-op's declared output is not the whole of what it writes (x86
        // `cpuid` writes four registers through one), so nothing survives it.
        known.clear();
        return;
    }
    let Some(out) = outvar else { return };
    let Some(k) = key_of(out) else { return };
    kill_overlapping(known, &k);
    if let Some(v) = value.filter(|_| !conditional) {
        let bits = out.size.saturating_mul(8).min(64);
        known.insert(k, truncate(v, bits));
    }
}

/// One binary op over already-truncated inputs; `lo_bytes` is the width of the
/// second input, which a `PIECE` concatenates below the first. A shift by the
/// whole width or more is zero, not Rust's masked shift.
fn arith(opc: OpCode, a: u64, b: u64, lo_bytes: u32) -> Option<u64> {
    let shl = |v: u64, by: u64| if by >= 64 { 0 } else { v << by };
    let shr = |v: u64, by: u64| if by >= 64 { 0 } else { v >> by };
    Some(match opc {
        OpCode::CPUI_INT_ADD => a.wrapping_add(b),
        OpCode::CPUI_INT_SUB => a.wrapping_sub(b),
        OpCode::CPUI_INT_MULT => a.wrapping_mul(b),
        OpCode::CPUI_INT_AND => a & b,
        OpCode::CPUI_INT_OR => a | b,
        OpCode::CPUI_INT_XOR => a ^ b,
        OpCode::CPUI_INT_LEFT => shl(a, b),
        OpCode::CPUI_INT_RIGHT => shr(a, b),
        OpCode::CPUI_SUBPIECE => shr(a, b.saturating_mul(8)),
        OpCode::CPUI_PIECE => shl(a, u64::from(lo_bytes) * 8) | b,
        _ => return None,
    })
}

fn input(known: &HashMap<Key, u64>, vn: Option<&VarnodeData>) -> Option<u64> {
    let vn = vn?;
    let space = vn.space.as_ref()?;
    if space.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT {
        return Some(vn.offset);
    }
    known.get(&key_of(vn)?).copied()
}

fn binary(
    known: &HashMap<Key, u64>,
    vars: &[VarnodeData],
    f: impl Fn(u64, u64) -> Option<u64>,
) -> Option<u64> {
    let a = input(known, vars.first())?;
    let b = input(known, vars.get(1))?;
    f(a, b)
}

fn truncate(v: u64, bits: u32) -> u64 {
    if bits >= 64 {
        v
    } else {
        v & ((1u64 << bits) - 1)
    }
}

fn sign_extend(v: u64, bits: u32) -> u64 {
    if bits == 0 || bits >= 64 {
        return v;
    }
    let mask = (1u64 << bits) - 1;
    let v = v & mask;
    if v & (1u64 << (bits - 1)) != 0 {
        v | !mask
    } else {
        v
    }
}

/// Every key overlapping `k` stops being known (a sub-register write invalidates
/// the whole register, and vice versa).
fn kill_overlapping(known: &mut HashMap<Key, u64>, k: &Key) {
    let (space, off, size) = *k;
    let hi = off.wrapping_add(u64::from(size));
    known.retain(|(s, o, sz), _| {
        *s != space || o.wrapping_add(u64::from(*sz)) <= off || *o >= hi
    });
}

/// Is `[vma, vma + len)` inside one section the image initializes and does not
/// write? A string in `.data` is whatever the program last stored there.
fn in_readonly_section(ctx: &AnalysisCtx, vma: u64, len: u64) -> bool {
    use object::{Object, ObjectSection, SectionKind};
    ctx.file.sections().any(|s| {
        let start = s.address();
        let end = start.wrapping_add(s.size());
        vma >= start
            && vma.wrapping_add(len) <= end
            && matches!(
                s.kind(),
                SectionKind::Text | SectionKind::ReadOnlyData | SectionKind::ReadOnlyString
            )
    })
}

/// Read a `size`-byte constant at `vma`, but only out of a section the image
/// initializes and does not write.
fn read_const(ctx: &AnalysisCtx, vma: u64, size: u32) -> Option<u64> {
    if size == 0 || size > 8 || !in_readonly_section(ctx, vma, u64::from(size)) {
        return None;
    }
    let space = Rc::clone(ctx.arch.manage().get_default_code_space()?);
    ctx.arch.read_loadimage_value(&Address::new(space, vma), size as int4).ok()
}

/// The object-file family of the image in `bytes`: whether an AArch64 target is
/// Apple's, Windows' or the standard AAPCS64 is a property of the container,
/// not of the language id.
pub fn image_family(bytes: &[u8]) -> ImageFamily {
    use object::FileKind;
    match FileKind::parse(bytes) {
        Ok(
            FileKind::MachO32
            | FileKind::MachO64
            | FileKind::MachOFat32
            | FileKind::MachOFat64
            | FileKind::DyldCache,
        ) => ImageFamily::Apple,
        Ok(
            FileKind::Pe32
            | FileKind::Pe64
            | FileKind::Coff
            | FileKind::CoffBig
            | FileKind::CoffImport,
        ) => ImageFamily::Windows,
        _ => ImageFamily::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_masks_to_the_output_width() {
        assert_eq!(truncate(0x1122334455667788, 64), 0x1122334455667788);
        assert_eq!(truncate(0x1122334455667788, 32), 0x55667788);
        assert_eq!(truncate(0x1122334455667788, 8), 0x88);
        // A width the emitter cannot produce (>64 bits) is a pass-through.
        assert_eq!(truncate(7, 128), 7);
    }

    #[test]
    fn sign_extend_widens_the_top_bit() {
        assert_eq!(sign_extend(0x80, 8), u64::MAX & !0x7f);
        assert_eq!(sign_extend(0x7f, 8), 0x7f);
        assert_eq!(sign_extend(0xffff_ffff, 32), u64::MAX);
        assert_eq!(sign_extend(1, 0), 1);
        assert_eq!(sign_extend(1, 64), 1);
    }

    #[test]
    fn piece_concatenates_at_the_low_input_width_and_wide_shifts_are_zero() {
        assert_eq!(arith(OpCode::CPUI_PIECE, 0x1234, 0x5678, 2), Some(0x1234_5678));
        assert_eq!(arith(OpCode::CPUI_PIECE, 0x12, 0x34, 1), Some(0x1234));
        assert_eq!(arith(OpCode::CPUI_PIECE, 0x1, 0x2, 4), Some(0x1_0000_0002));
        assert_eq!(arith(OpCode::CPUI_INT_LEFT, 1, 64, 1), Some(0));
        assert_eq!(arith(OpCode::CPUI_INT_RIGHT, u64::MAX, 70, 1), Some(0));
        assert_eq!(arith(OpCode::CPUI_INT_LEFT, 1, 63, 1), Some(1 << 63));
        assert_eq!(arith(OpCode::CPUI_SUBPIECE, 0x1122_3344_5566_7788, 4, 4), Some(0x1122_3344));
        assert_eq!(arith(OpCode::CPUI_SUBPIECE, 0x11, 8, 4), Some(0));
        assert_eq!(arith(OpCode::CPUI_COPY, 1, 2, 4), None);
    }

    fn spaces() -> (Rc<kuna_base::space::AddrSpace>, Rc<kuna_base::space::AddrSpace>) {
        use kuna_base::space::{spacetype, AddrSpace};
        (
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT)),
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
        )
    }

    fn vn(space: &Rc<kuna_base::space::AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size }
    }

    #[test]
    fn a_cmov_destination_is_not_known() {
        let (k, r) = spaces();
        let no_load = |_: u64, _: u32| None;
        let (rsi, rax) = (vn(&r, 0x30, 8), vn(&r, 0x0, 8));
        let mut known: HashMap<Key, u64> = HashMap::new();
        // RSI = 0x7500; RAX = 0x70bc
        apply_ops(
            &[
                (OpCode::CPUI_COPY, Some(rsi.clone()), vec![vn(&k, 0x7500, 8)]),
                (OpCode::CPUI_COPY, Some(rax.clone()), vec![vn(&k, 0x70bc, 8)]),
            ],
            &mut known,
            &no_load,
        );
        assert_eq!(known.get(&key_of(&rsi).unwrap()), Some(&0x7500));
        // cmove rsi,rax: if (!ZF) goto inst_next; RSI = RAX;
        apply_ops(
            &[
                (OpCode::CPUI_CBRANCH, None, vec![vn(&k, 0x2835, 8), vn(&r, 0x206, 1)]),
                (OpCode::CPUI_COPY, Some(rsi.clone()), vec![rax.clone()]),
            ],
            &mut known,
            &no_load,
        );
        assert_eq!(known.get(&key_of(&rsi).unwrap()), None);
        assert_eq!(known.get(&key_of(&rax).unwrap()), Some(&0x70bc));
    }

    #[test]
    fn a_partial_write_invalidates_the_whole_register() {
        let mut known: HashMap<Key, u64> = HashMap::new();
        // RDI (offset 0x38, 8 bytes) and RSI (0x30, 8 bytes) in one space.
        known.insert((1, 0x38, 8), 0x402004);
        known.insert((1, 0x30, 8), 0x11);
        // A 4-byte write to EDI kills the 8-byte RDI it overlaps...
        kill_overlapping(&mut known, &(1, 0x38, 4));
        assert!(!known.contains_key(&(1, 0x38, 8)));
        // ...and leaves the register beside it alone.
        assert_eq!(known.get(&(1, 0x30, 8)), Some(&0x11));
        // A different space never aliases.
        kill_overlapping(&mut known, &(2, 0x30, 8));
        assert_eq!(known.get(&(1, 0x30, 8)), Some(&0x11));
        // An exactly-abutting write does not.
        kill_overlapping(&mut known, &(1, 0x38, 8));
        assert_eq!(known.get(&(1, 0x30, 8)), Some(&0x11));
    }

    #[test]
    fn the_gettext_family_names_the_msgid_slot() {
        // `_()` is `dcgettext(NULL, msgid, LC_MESSAGES)`; `gettext(msgid)`.
        assert_eq!(GETTEXT_FAMILY.iter().find(|(n, _)| *n == "gettext").unwrap().1, 0);
        assert_eq!(GETTEXT_FAMILY.iter().find(|(n, _)| *n == "dgettext").unwrap().1, 1);
        assert_eq!(GETTEXT_FAMILY.iter().find(|(n, _)| *n == "dcgettext").unwrap().1, 1);
    }
}
