//! (kuna `formatcallees`) Format-string typing for callees no table knows.
//!
//! [`super::kuna_fmtstatic`] types a call whose callee the built-in libc tables
//! declare variadic with a format parameter. A program's own format functions
//! are declared nowhere: a logger `log_msg(int level, const char *fmt, ...)`
//! that hands its arguments to `vfprintf`, an `xasprintf` or a `die`. Without
//! a declaration a call to one keeps the arguments trial recovery finds, and
//! those miss what the callee's register-save area does not prove (a `double`
//! in `xmm0`) and keep the widths the caller computed (`(unsigned long)argc`
//! for a `%d`).
//!
//! A callee is a format function, with a direction (printf-like output or
//! scanf-like input), when
//!
//! - its name contains `printf` or `scanf` and no table declares it (Ghidra's
//!   `FormatStringAnalyzer` substring rule), the `v`-prefixed `va_list` forms
//!   and the wide-character forms excluded; or
//! - it is a function of the image whose body calls or tail-jumps to a `va_list`
//!   library function of the printf or scanf family (`vfprintf`,
//!   `__vsnprintf_chk`, `vsscanf`, `vsyslog`, ...), directly or through one more
//!   function of the image that does. Only a function holding a `va_list` can
//!   make that call, and the direction is the library function's.
//!
//! On x86-64 System V a function of the image must also read `AL` at its entry
//! before writing it: a variadic function's prologue tests the vector-register
//! count the caller passes there (System V AMD64 ABI, 3.5.7), and no other
//! function reads that register before writing it (`xor eax,eax` writes it,
//! as a forwarding call sets the count). This is what rules out a
//! `va_list` helper (`vlog(int, const char *, va_list)`) and the functions that
//! merely call a format function.
//!
//! At each call site the first of the first four integer argument slots that
//! holds a constant pointer to a NUL-terminated string in a read-only section
//! is the format (a site whose first such string has no `%`, or whose
//! conversions do not parse, is not typed). Every typed site of one callee must
//! agree on the slot, or the callee is dropped. On x86-64 System V a site whose
//! `AL` folds to a constant must pass exactly as many vector registers as the
//! format has floating conversions. The site's override is the argument slots
//! before the format as untyped words, the format as `char *`, then the
//! format's conversions; its return is left to recovery. An untyped slot is
//! as wide as the call's own block last wrote it (a 32-bit `int` level passed
//! in `edi` is four bytes, so it prints without a widening cast), and a full
//! register where the block does not write it.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::fspec::{PrototypePieces, ProtoModel};
use kuna_decomp::kuna_formatstring::VarargAbi;
use kuna_num::opcodes::OpCode;

use crate::listing::{Listing, RefKind};
use crate::pass::{AnalysisCtx, FormatSiteFact};

use super::kuna_fmtstatic::{self as fmtstatic, Key, LiftCache};
use super::{apply, Spec};

/// How many leading argument slots may hold the format.
const FORMAT_SLOTS: usize = 4;

/// How many instructions of a callee's entry the `AL` test looks at.
const PROLOGUE_INSNS: usize = 24;

/// The direction of a `va_list` library function of the printf or scanf
/// family: `Some(true)` output, `Some(false)` input.
pub fn va_list_family(name: &str) -> Option<bool> {
    let mut n = name.trim_start_matches('_');
    for prefix in ["isoc99_", "isoc23_", "stdio_common_"] {
        n = n.strip_prefix(prefix).unwrap_or(n);
    }
    let n = n.trim_start_matches('_').strip_suffix("_chk").unwrap_or(n);
    if matches!(n, "vsyslog" | "verr" | "verrx" | "vwarn" | "vwarnx") {
        return Some(true);
    }
    if !n.starts_with('v') || wide(n) {
        return None;
    }
    if n.contains("scanf") {
        Some(false)
    } else if n.contains("printf") {
        Some(true)
    } else {
        None
    }
}

fn wide(name: &str) -> bool {
    name.contains("wprintf") || name.contains("wscanf")
}

/// The direction a callee's name gives it, by the substring rule, or `None`.
pub fn named_family(name: &str) -> Option<bool> {
    if wide(name) || va_list_family(name).is_some() {
        return None;
    }
    let at = name.find("printf").or_else(|| name.find("scanf"))?;
    if name[..at].ends_with(['v', 'V']) {
        return None;
    }
    apply::classify_variadic_call(name)
}

/// The shared state of one pass over the image.
pub(super) struct Inputs<'a> {
    pub ctx: &'a AnalysisCtx<'a>,
    pub listing: &'a Listing,
    pub model: &'a Rc<ProtoModel>,
    pub abi: VarargAbi,
    pub gettext: &'a HashMap<u64, (Key, Key)>,
    pub imports: &'a [(String, u64)],
    pub dbg: bool,
}

/// The format sites of every callee [`format_callees`] admits.
pub(super) fn infer(inp: &Inputs, lifts: &mut LiftCache) -> Vec<FormatSiteFact> {
    let sysv = sysv_al(inp.ctx);
    let callees = format_callees(inp, sysv.as_ref(), lifts);
    let Some(slot_keys) = slot_keys(inp) else { return Vec::new() };
    let mut out = Vec::new();
    for (callee, (name, is_output)) in &callees {
        let mut sites: Vec<FormatSiteFact> = Vec::new();
        let mut slots: HashSet<usize> = HashSet::new();
        for edge in inp.listing.refs_to(*callee) {
            let tail = edge.kind == RefKind::Code && !fmtstatic::falls_into(inp.listing, edge.from, *callee);
            if edge.kind != RefKind::Call && !tail {
                continue;
            }
            let Some(func) = inp.listing.function_containing(edge.from) else { continue };
            let Some((slot, vma, fmt)) = format_at(inp, edge.from, &slot_keys, lifts) else { continue };
            let specs = if *is_output {
                super::parse_output_types(&fmt)
            } else {
                super::parse_input_types(&fmt)
            };
            if specs.is_empty() {
                continue;
            }
            if let Some(al) = &sysv {
                let floats = specs.iter().filter(|s| matches!(s, Spec::Double)).count() as u64;
                if al_at(inp, edge.from, al, lifts).is_some_and(|n| n != floats) {
                    if inp.dbg {
                        eprintln!("fmtcallee:   {:#x}: AL disagrees with {floats} floats", edge.from);
                    }
                    continue;
                }
            }
            let widths: Vec<u32> = slot_keys[..slot]
                .iter()
                .map(|(key, _)| {
                    fmtstatic::written_width(inp.ctx, inp.listing, edge.from, key, lifts).unwrap_or(key.2)
                })
                .collect();
            let Some(pieces) = site_pieces(inp, name, &slot_keys[slot].1, &widths, &specs) else { continue };
            if inp.dbg {
                eprintln!("fmtcallee:   {name} site {:#x}: format {fmt:?} at slot {slot}", edge.from);
            }
            slots.insert(slot);
            sites.push(FormatSiteFact {
                func: func.entry,
                callpoint: edge.from,
                pieces,
                format_slot: slot,
                format_vma: vma,
            });
        }
        if slots.len() == 1 {
            out.extend(sites);
        } else if inp.dbg && slots.len() > 1 {
            eprintln!("fmtcallee: {name}: sites disagree on the format slot");
        }
    }
    out
}

/// Every callee admitted as a format function, by entry: its name and whether
/// it is printf-like.
fn format_callees(
    inp: &Inputs,
    sysv: Option<&Key>,
    lifts: &mut LiftCache,
) -> BTreeMap<u64, (String, bool)> {
    let ctx = inp.ctx;
    let types = ctx.arch.types();
    let (_, word_size) = ctx.arch.data_org();
    let layout = crate::protos::effective_libctypes_layout(ctx.file);
    let declared =
        |name: &str| crate::protos::declared_libc_prototype(name, types, word_size, layout).is_some();
    let mut callees: BTreeMap<u64, (String, bool)> = BTreeMap::new();
    let mut va_list: HashMap<u64, bool> = HashMap::new();
    for (name, addr) in inp.imports {
        if let Some(dir) = va_list_family(name) {
            va_list.insert(*addr, dir);
        } else if !declared(name) {
            if let Some(dir) = named_family(name) {
                callees.insert(*addr, (name.clone(), dir));
            }
        }
    }
    let import_addrs: HashSet<u64> = inp.imports.iter().map(|(_, a)| *a).collect();
    let locals: Vec<(u64, String)> = inp
        .listing
        .functions()
        .filter(|(entry, f)| !import_addrs.contains(entry) && f.call_fixup.is_none())
        .map(|(entry, f)| (*entry, f.name.clone().unwrap_or_else(|| format!("sub_{entry:x}"))))
        .filter(|(_, name)| !declared(name))
        .collect();
    let mut forwarders: HashMap<u64, bool> = HashMap::new();
    for (entry, _) in &locals {
        if let Some(dir) = calls_into(inp.listing, *entry, &va_list) {
            forwarders.insert(*entry, dir);
        }
    }
    for (entry, name) in &locals {
        let dir = named_family(name)
            .or_else(|| forwarders.get(entry).copied())
            .or_else(|| calls_into(inp.listing, *entry, &forwarders));
        let Some(dir) = dir else { continue };
        if sysv.is_some_and(|al| !reads_at_entry(inp, *entry, al, lifts)) {
            continue;
        }
        if inp.dbg {
            eprintln!("fmtcallee: {name} @ {entry:#x} is a {} function", if dir { "printf" } else { "scanf" });
        }
        callees.insert(*entry, (name.clone(), dir));
    }
    callees
}

/// The direction of the targets in `targets` that the body of the function at
/// `entry` calls or tail-jumps to, when they agree.
fn calls_into(listing: &Listing, entry: u64, targets: &HashMap<u64, bool>) -> Option<bool> {
    let end = listing.next_function_after(entry).map(|f| f.entry);
    let mut dirs = listing
        .instructions_in_range(entry, end)
        .flat_map(|(&vma, _)| listing.refs_from(vma))
        .filter(|r| matches!(r.kind, RefKind::Call | RefKind::Code) && r.to != entry)
        .filter_map(|r| targets.get(&r.to).copied());
    let first = dirs.next()?;
    dirs.all(|d| d == first).then_some(first)
}

/// The `AL` storage key on an x86-64 System V target, where the caller of a
/// variadic function passes its vector-register count there.
fn sysv_al(ctx: &AnalysisCtx) -> Option<Key> {
    let archid = &ctx.arch.archid;
    if !archid.starts_with("x86:LE:64")
        || archid.ends_with(":windows")
        || fmtstatic::image_family(ctx.bytes) == kuna_decomp::kuna_formatstring::ImageFamily::Windows
    {
        return None;
    }
    fmtstatic::key_of(&ctx.arch.translate().probe_register_varnode(b"AL")?)
}

/// Does the function at `entry` read `al` before it writes any of it, within
/// its first straight-line instructions?
fn reads_at_entry(inp: &Inputs, entry: u64, al: &Key, lifts: &mut LiftCache) -> bool {
    let overlaps = |k: Key| k.0 == al.0 && k.1 < al.1 + u64::from(al.2) && al.1 < k.1 + u64::from(k.2);
    let mut at = entry;
    for _ in 0..PROLOGUE_INSNS {
        let Some(lifted) = fmtstatic::lift(inp.ctx, at, lifts) else { return false };
        if !lifted.ok {
            return false;
        }
        for (opc, out, ins) in &lifted.ops {
            let zeroing = matches!(opc, OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB)
                && ins.len() == 2
                && fmtstatic::key_of(&ins[0]) == fmtstatic::key_of(&ins[1]);
            if !zeroing && ins.iter().filter_map(fmtstatic::key_of).any(overlaps) {
                return true;
            }
            if out.as_ref().and_then(fmtstatic::key_of).is_some_and(overlaps) {
                return false;
            }
        }
        let Some(insn) = inp.listing.instruction_at(at) else { return false };
        if insn.flow.is_call || insn.flow.is_jump {
            return false;
        }
        match insn.fall_through {
            Some(next) => at = next,
            None => return false,
        }
    }
    false
}

/// The value `AL` holds at the call `site`, read through whichever of `AL`,
/// `EAX` and `RAX` the fold knows.
fn al_at(inp: &Inputs, site: u64, al: &Key, lifts: &mut LiftCache) -> Option<u64> {
    [1u32, 4, 8].into_iter().find_map(|size| {
        let key = (al.0, al.1, size);
        fmtstatic::resolve_at(inp.ctx, inp.listing, site, &key, inp.gettext, lifts, false).map(|v| v & 0xff)
    })
}

/// The storage of the first [`FORMAT_SLOTS`] pointer-sized argument slots.
fn slot_keys(inp: &Inputs) -> Option<Vec<(Key, Rc<Datatype>)>> {
    let types = inp.ctx.arch.types();
    let (_, word_size) = inp.ctx.arch.data_org();
    let ptr = types.get_size_of_pointer();
    let c = types.get_type_char(types.get_size_of_char()).ok()?;
    let char_ptr = types.get_type_pointer(ptr, c, word_size).ok()?;
    let probe = PrototypePieces {
        name: String::new(),
        outtype: Some(types.get_type_void().ok()?),
        intypes: vec![Rc::clone(&char_ptr); FORMAT_SLOTS],
        innames: vec![String::new(); FORMAT_SLOTS],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
        kuna_library_names: None,
    };
    let assigned = fmtstatic::assign(inp.model, &probe, inp.ctx.arch)?;
    let keys: Option<Vec<Key>> = assigned.iter().skip(1).map(fmtstatic::storage_key).collect();
    Some(keys?.into_iter().map(|k| (k, Rc::clone(&char_ptr))).collect())
}

/// The format at a call site: the slot, the string's address and the string.
fn format_at(
    inp: &Inputs,
    site: u64,
    slot_keys: &[(Key, Rc<Datatype>)],
    lifts: &mut LiftCache,
) -> Option<(usize, u64, String)> {
    for (slot, (key, _)) in slot_keys.iter().enumerate() {
        let Some(vma) = fmtstatic::resolve_at(inp.ctx, inp.listing, site, key, inp.gettext, lifts, false)
        else {
            continue;
        };
        let Some(fmt) = fmtstatic::read_cstring(inp.ctx.arch, vma) else { continue };
        if !fmtstatic::in_readonly_section(inp.ctx, vma, fmt.len() as u64 + 1) {
            continue;
        }
        return fmt.contains('%').then_some((slot, vma, fmt));
    }
    None
}

/// The override for one site: an untyped value of each width in `widths` for
/// the slots before the format, the format, then its conversions; no return
/// type.
fn site_pieces(
    inp: &Inputs,
    name: &str,
    format: &Rc<Datatype>,
    widths: &[u32],
    specs: &[Spec],
) -> Option<PrototypePieces> {
    let types = inp.ctx.arch.types();
    let (_, word_size) = inp.ctx.arch.data_org();
    let mut fixed: Vec<Rc<Datatype>> = Vec::with_capacity(widths.len() + 1);
    for width in widths {
        fixed.push(types.get_base(*width as i32, type_metatype::TYPE_UNKNOWN).ok()?);
    }
    fixed.push(Rc::clone(format));
    let mut pieces =
        apply::build_override_pieces(name, None, &fixed, specs, types, word_size, inp.abi).ok()??;
    if kuna_decomp::kuna_armfloatargs::applies(inp.ctx.arch)
        && !kuna_decomp::kuna_armfloatargs::base_inputs(inp.ctx.arch, &mut pieces)
    {
        return None;
    }
    Some(pieces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn va_list_functions_by_family() {
        for name in ["vfprintf", "vprintf", "vsnprintf", "__vfprintf_chk", "__vsnprintf_chk", "vasprintf", "vsyslog"] {
            assert_eq!(va_list_family(name), Some(true), "{name}");
        }
        for name in ["vsscanf", "__isoc99_vsscanf", "__isoc23_vfscanf", "vscanf"] {
            assert_eq!(va_list_family(name), Some(false), "{name}");
        }
        for name in ["printf", "vfwprintf", "vswprintf", "verify", "fprintf"] {
            assert_eq!(va_list_family(name), None, "{name}");
        }
    }

    #[test]
    fn named_format_functions_exclude_va_list_and_wide_forms() {
        assert_eq!(named_family("g_strdup_printf"), Some(true));
        assert_eq!(named_family("my_sscanf"), Some(false));
        assert_eq!(named_family("g_strdup_vprintf"), None);
        assert_eq!(named_family("vfprintf"), None);
        assert_eq!(named_family("swprintf"), None);
        assert_eq!(named_family("log_msg"), None);
    }
}
