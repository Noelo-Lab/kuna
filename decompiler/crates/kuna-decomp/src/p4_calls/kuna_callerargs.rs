//! (kuna `narrowargs callers`) A 32-bit ARM register parameter every caller
//! passes as a narrow value is declared that narrow.
//!
//! The AAPCS has the caller extend a `uint8_t`, `int16_t` or `_Bool` argument
//! to the whole register, so the callee's body reads the register at full
//! width and holds no evidence of the narrow type: `narrowargs`' body rule
//! never fires. The evidence sits at the call sites. A whole-binary run
//! records, after each decompile, what every direct call passes in each
//! argument register ([`record`]): whether the value fits a byte or a halfword
//! unsigned (its nonzero-bit mask) or signed (a sign extension, a constant
//! that is one), and whether the caller extended it itself (`uxtb`, `sxth`, a
//! mask by exactly `0xff` or `0xffff`), which is what a compiler does for a
//! narrow parameter and not for an `int`.
//!
//! Where the call graph lists every caller of a function -- its address is
//! stored nowhere in the image and built by no `movw`/`movt` pair, and every
//! call and tail jump to it was decompiled as a call -- and every call passes
//! a parameter's register a value of the same narrow kind, one of them extended
//! by the caller, the function is decompiled again with that parameter replaced
//! by its low part, zero- or sign-extended back to the register at the entry
//! ([`narrow_stated`]), and typed `unsigned char`/`short`, `char`/`short`, or
//! `bool` where every caller passes 0 or 1 and one of them a computed truth
//! value. Every call passes a value that fits, so every value the body computes
//! is unchanged, and so is every caller's C: the argument converts to the
//! narrow type without loss.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;

/// How a narrowed parameter is extended back to its register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ext {
    Unsigned,
    Signed,
    Bool,
}

/// A parameter register every caller passes a narrow value in.
#[derive(Debug, Clone)]
pub struct NarrowParam {
    pub addr: Address,
    pub size: int4,
    pub width: int4,
    pub ext: Ext,
}

/// What one call passes in one argument register.
#[derive(Debug, Clone)]
pub struct Arg {
    pub addr: Address,
    pub size: int4,
    /// The value fits this many bytes unsigned (0: neither 1 nor 2).
    pub unsigned: int4,
    /// The value is the sign extension of this many bytes (0: neither).
    pub signed: int4,
    /// The caller zero-extended it itself (`uxtb`, `& 0xffff`).
    pub zext: bool,
    /// The caller sign-extended it itself (`sxtb`).
    pub sext: bool,
    /// The value is 0 or 1.
    pub truth: bool,
    pub constant: bool,
}

/// One direct call.
#[derive(Debug, Clone)]
pub struct Site {
    pub caller: uintb,
    pub at: uintb,
    pub args: Vec<Option<Arg>>,
}

/// The whole-binary run's record.
#[derive(Debug, Default)]
pub struct Ledger {
    pub recording: bool,
    /// Callee entry -> the direct calls recorded to it.
    pub sites: BTreeMap<uintb, Vec<Site>>,
    /// Function entry -> its first decompile's register parameters that are
    /// whole integer registers: storage, and whether it is an integer.
    pub own: BTreeMap<uintb, Vec<(Address, int4, bool)>>,
    /// Function entry -> every call and tail jump the call graph lists to it,
    /// for the functions whose callers it can all list.
    pub expected: BTreeMap<uintb, Vec<uintb>>,
    /// Function entry -> the parameters its callers' values narrow.
    pub stated: BTreeMap<uintb, Rc<Vec<NarrowParam>>>,
}

/// The register-wide integer parameters of a function's first decompile, or
/// `None` when nothing may be stated about them (a declared, variadic or
/// locked prototype, or a parameter that is not a plain value).
fn own_params(arch: &Architecture, entry: &Address, fd: &Funcdata) -> Option<Vec<(Address, int4, bool)>> {
    if arch.symboltab.function_proto_pieces_across_scopes(entry).is_some() {
        return None;
    }
    let proto = fd.get_func_proto();
    if !proto.has_store() || proto.is_dotdotdot() || proto.is_input_locked() {
        return None;
    }
    let mut out = Vec::new();
    for i in 0..proto.num_params() {
        let p = proto.get_param(i)?;
        if p.is_hidden_return() || p.is_indirect_storage() || p.is_this_pointer() || p.is_type_locked() {
            return None;
        }
        let integer = p.get_type().is_some_and(|t| {
            matches!(
                t.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
            )
        });
        out.push((p.get_address(), p.get_size(), integer));
    }
    Some(out)
}

/// The width of which `vn` is the sign extension: a constant that is one, a
/// sign extension, a zero extension of a byte (it fits a signed halfword), or
/// a copy or merge of such values.
fn signed_width(data: &Funcdata, vn: VarnodeId, depth: u32, seen: &mut BTreeSet<VarnodeId>) -> Option<int4> {
    if !seen.insert(vn) {
        return Some(0);
    }
    if depth > 8 {
        return None;
    }
    let v = data.vbank().get(vn)?;
    let size = v.get_size();
    if v.is_constant() {
        let c = v.get_offset() & calc_mask(size);
        return [1, 2]
            .into_iter()
            .find(|&w| w < size && kuna_base::address::sign_extend_sized(c & calc_mask(w), w, size) == c);
    }
    let o = data.obank().get(v.get_def()?)?;
    match o.code() {
        OpCode::CPUI_INT_SEXT => data.vbank().get(o.get_in(0)?).map(|x| x.get_size()).filter(|&w| w <= 2),
        OpCode::CPUI_INT_ZEXT => (data.vbank().get(o.get_in(0)?)?.get_size() == 1).then_some(2),
        OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => {
            let mut w = 0;
            for k in 0..o.num_input() {
                w = w.max(signed_width(data, o.get_in(k)?, depth + 1, seen)?);
            }
            (w > 0).then_some(w)
        }
        _ => None,
    }
}

/// Is `vn` the output of an extension or mask the caller applied to a value it
/// computed, rather than a load, a call's result or its own input that
/// arrives narrow? Returns (zero extension, sign extension).
fn caller_extends(data: &Funcdata, vn: VarnodeId) -> (bool, bool) {
    let Some(o) = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
        return (false, false);
    };
    let source = |k: int4| o.get_in(k).and_then(|x| data.vbank().get(x));
    match o.code() {
        OpCode::CPUI_INT_AND => {
            let mask = source(1).filter(|c| c.is_constant()).map(|c| c.get_offset());
            (matches!(mask, Some(0xff | 0xffff)), false)
        }
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            let Some(x) = source(0) else { return (false, false) };
            if x.get_size() > 2 || x.is_constant() || x.is_input() {
                return (false, false);
            }
            let fetched = x
                .get_def()
                .and_then(|d| data.obank().get(d))
                .is_some_and(|d| matches!(d.code(), OpCode::CPUI_LOAD | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND));
            if fetched {
                return (false, false);
            }
            (o.code() == OpCode::CPUI_INT_ZEXT, o.code() == OpCode::CPUI_INT_SEXT)
        }
        _ => (false, false),
    }
}

/// What the value `vn`, passed in `addr:size`, says about the parameter.
fn argument(data: &Funcdata, vn: VarnodeId, addr: Address, size: int4, zext_trimmed: bool) -> Option<Arg> {
    let v = data.vbank().get(vn)?;
    let constant = v.is_constant();
    if v.get_size() != size {
        let w = v.get_size();
        return (w < size && w <= 2 && zext_trimmed).then(|| Arg {
            addr,
            size,
            unsigned: w,
            signed: 0,
            zext: false,
            sext: false,
            truth: v.get_nz_mask() <= 1,
            constant,
        });
    }
    let nz = v.get_nz_mask();
    let unsigned = if nz & !0xff == 0 { 1 } else if nz & !0xffff == 0 { 2 } else { 0 };
    let signed = signed_width(data, vn, 0, &mut BTreeSet::new()).unwrap_or(0);
    let (zext, sext) = caller_extends(data, vn);
    Some(Arg { addr, size, unsigned, signed, zext, sext, truth: nz <= 1, constant })
}

/// Record a finished decompile of the function at `entry`: what each direct
/// call passes, replacing an earlier decompile's record, and, the first time,
/// its own parameters.
pub fn record(arch: &mut Architecture, entry: &Address, fd: &Funcdata) {
    if !arch.kuna_callerargs.recording {
        return;
    }
    let me = entry.get_offset();
    let own = if arch.kuna_callerargs.own.contains_key(&me) { None } else { own_params(arch, entry, fd) };
    let mut calls: Vec<(uintb, Site)> = Vec::new();
    for i in 0..fd.num_calls() {
        let fc = fd.get_call_specs(i);
        let callee = fc.get_entry_address();
        if callee.is_invalid() {
            continue;
        }
        let Some(o) = fd.obank().get(fc.get_op()) else { continue };
        if o.code() != OpCode::CPUI_CALL {
            continue;
        }
        let storage = fc.final_input_storage();
        let n = o.num_input();
        let mut args = Vec::new();
        for s in 1..n {
            let (Some(vn), Some((addr, size))) = (o.get_in(s), storage.get((s - 1) as usize).cloned()) else {
                args.push(None);
                continue;
            };
            let w = fd.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
            args.push(argument(fd, vn, addr, size, fc.is_zext_trimmed_input(s, w, n)));
        }
        calls.push((callee.get_offset(), Site { caller: me, at: o.get_addr().get_offset(), args }));
    }
    let ledger = &mut arch.kuna_callerargs;
    for sites in ledger.sites.values_mut() {
        sites.retain(|s| s.caller != me);
    }
    for (callee, site) in calls {
        ledger.sites.entry(callee).or_default().push(site);
    }
    if let Some(own) = own {
        ledger.own.insert(me, own);
    }
}

/// The narrow kind every argument in `args` agrees on, or `None`.
fn agree(args: &[&Arg]) -> Option<(int4, Ext)> {
    if args.is_empty() {
        return None;
    }
    if args.iter().all(|a| a.truth) && args.iter().any(|a| a.truth && !a.constant) {
        return Some((1, Ext::Bool));
    }
    let zext = args.iter().any(|a| a.zext);
    let sext = args.iter().any(|a| a.sext);
    if zext == sext {
        return None;
    }
    if zext && args.iter().all(|a| a.unsigned > 0) {
        return Some((args.iter().map(|a| a.unsigned).max()?, Ext::Unsigned));
    }
    if sext && args.iter().all(|a| a.signed > 0) {
        return Some((args.iter().map(|a| a.signed).max()?, Ext::Signed));
    }
    None
}

/// Decide what every function's callers state about its parameters; returns
/// the entries whose statement is new.
pub fn decide(arch: &mut Architecture) -> Vec<uintb> {
    let ledger = &arch.kuna_callerargs;
    let mut decided: Vec<(uintb, Vec<NarrowParam>)> = Vec::new();
    for (&entry, expected) in &ledger.expected {
        let (Some(own), Some(sites)) = (ledger.own.get(&entry), ledger.sites.get(&entry)) else { continue };
        let at: BTreeSet<uintb> = sites.iter().map(|s| s.at).collect();
        if expected.is_empty() || !expected.iter().all(|e| at.contains(e)) {
            continue;
        }
        let mut stated = Vec::new();
        for (i, (addr, size, integer)) in own.iter().enumerate() {
            if !integer || *size != 4 || addr.get_space().is_none_or(|s| s.get_type() != kuna_base::space::spacetype::IPTR_PROCESSOR) {
                continue;
            }
            let args: Option<Vec<&Arg>> = sites
                .iter()
                .map(|s| s.args.get(i).and_then(|a| a.as_ref()).filter(|a| &a.addr == addr && a.size == *size))
                .collect();
            if let Some((width, ext)) = args.and_then(|a| agree(&a)) {
                stated.push(NarrowParam { addr: addr.clone(), size: *size, width, ext });
            }
        }
        if stated.is_empty() || ledger.stated.contains_key(&entry) {
            continue;
        }
        decided.push((entry, stated));
    }
    let keys = decided.iter().map(|(k, _)| *k).collect();
    for (k, s) in decided {
        if trace() {
            for p in &s {
                eprintln!("[callerargs] 0x{k:x} {:x}:{} -> {} {:?}", p.addr.get_offset(), p.size, p.width, p.ext);
            }
        }
        arch.kuna_callerargs.stated.insert(k, Rc::new(s));
    }
    keys
}

/// Hand the function its callers' statement before the pipeline runs.
pub fn seed(arch: &Architecture, data: &mut Funcdata) {
    if !arch.narrow_args_callers {
        return;
    }
    if let Some(s) = arch.kuna_callerargs.stated.get(&data.get_address().get_offset()) {
        data.kuna_set_callerargs(Some(Rc::clone(s)));
    }
}

/// Replace every stated register input by its low part, extended at the
/// entry as the callers extended it. Returns how many were replaced.
pub fn narrow_stated(data: &mut Funcdata) -> int4 {
    let Some(stated) = data.kuna_callerargs().cloned() else { return 0 };
    let mut count = 0;
    for p in stated.iter() {
        let vn = data.vbank().iter_def_flag(crate::varnode::varnode_flags::input).find(|&vn| {
            data.vbank().get(vn).is_some_and(|v| v.get_size() == p.size && v.get_addr() == &p.addr)
        });
        let Some(vn) = vn else { continue };
        if data.vbank().get(vn).is_none_or(|v| v.is_type_lock() || v.is_addr_tied() || v.is_persist()) {
            continue;
        }
        if !data.get_func_proto().possible_input_param(&p.addr, p.size) {
            continue;
        }
        let ext = if p.ext == Ext::Signed { OpCode::CPUI_INT_SEXT } else { OpCode::CPUI_INT_ZEXT };
        if crate::kuna_narrowargs::narrow_input_to(data, vn, p.width, ext) {
            count += 1;
        }
    }
    count
}

/// The type the callers' statement gives the narrowed input `vn`, offered to
/// `Varnode::getLocalType`'s fold in place of an integer of its size.
pub fn input_vote(data: &Funcdata, vn: VarnodeId, ct: &Datatype) -> Option<Rc<Datatype>> {
    let stated = data.kuna_callerargs()?;
    let v = data.vbank().get(vn)?;
    if !v.is_input() || v.is_type_lock() || ct.get_size() != v.get_size() {
        return None;
    }
    if !matches!(
        ct.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_BOOL
    ) {
        return None;
    }
    let p = stated.iter().find(|p| p.width == v.get_size() && v.get_addr().get_offset() == p.addr.get_offset())?;
    let meta = match p.ext {
        Ext::Unsigned => type_metatype::TYPE_UINT,
        Ext::Signed => type_metatype::TYPE_INT,
        Ext::Bool => type_metatype::TYPE_BOOL,
    };
    data.get_arch().types()?.get_base(p.width, meta).ok()
}

/// Is the decision trace on (`KUNA_CALLERARGS_TRACE=1`)?
pub fn trace() -> bool {
    std::env::var("KUNA_CALLERARGS_TRACE").is_ok_and(|v| v != "0" && !v.is_empty())
}

#[cfg(test)]
mod tests;
