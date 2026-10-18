//! (kuna `regnarrow`) A register local whose every value fits a narrower low
//! part is declared that narrow.
//!
//! A `uint8_t` counter on ARM lives in a whole register (`adds r5, #1; uxtb r5,
//! r5`), a byte read with `ldrb` stays zero-extended in it, and an x86-64
//! `int` written through `%eax` is zero-extended into `%rax`; the SSA web that
//! becomes the variable keeps the register's width and prints as `unsigned
//! int` or `unsigned long`. The nonzero-bit analysis already proves that every
//! value of such a web fits its low 1, 2 or 4 bytes. When one of them can set
//! that part's top bit, and every value entering the web is an explicit
//! narrowing (a zero extension, a mask, a constant or a copy), the web is
//! rebuilt at the narrow width by [`crate::subflow::run_zext_uses`]: the
//! arithmetic that only feeds the web's low bytes is narrowed with it, and
//! every other read takes the zero extension of the narrow value, which is
//! exactly the value it read before. The narrow values are recorded so type
//! inference can declare one nothing else types unsigned ([`unsigned_vote`]).

use std::collections::BTreeSet;

use kuna_base::address::calc_mask;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// How many times one function's webs are narrowed; each later run only sees
/// what the previous one exposed.
const MAX_RUNS: u8 = 2;

/// The narrowest of 1, 2 and 4 bytes, below `size`, whose low part holds every
/// bit of `nz` and whose top bit `nz` can set.
fn narrow_width(nz: uintb, size: int4) -> Option<int4> {
    let w = [1, 2, 4].into_iter().find(|&w| w < size && nz & !calc_mask(w) == 0)?;
    ((nz >> (8 * w - 1)) & 1 == 1).then_some(w)
}

/// Can `vn` be a member of a narrowed web: a written register of an integer
/// register width, typed as an integer so far, that is neither tied to memory
/// nor type-locked? A pointer whose values happen to be small (string
/// addresses in a small image) is no narrow value.
fn plain_container(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if !matches!(v.get_size(), 2 | 4 | 8)
        || !v.is_written()
        || v.is_addr_tied()
        || v.is_persist()
        || v.is_addr_force()
        || v.is_type_lock()
    {
        return false;
    }
    v.get_addr().get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR)
        && matches!(
            v.get_type().get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        )
}

/// Do `vn`'s possibly nonzero bits fit the low half of it, the widest part a
/// web holding it can narrow to?
fn fits_low_half(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).is_some_and(|v| v.get_nz_mask() & !calc_mask(v.get_size() / 2) == 0)
}

fn def_code(data: &Funcdata, vn: VarnodeId) -> Option<OpCode> {
    let v = data.vbank().get(vn)?;
    data.obank().get(v.get_def()?).map(|o| o.code())
}

/// The SSA web of `seed`: the varnodes MULTIEQUALs join with it.
fn web_of(data: &Funcdata, seed: VarnodeId) -> Vec<VarnodeId> {
    let mut web = vec![seed];
    let mut i = 0;
    while i < web.len() && web.len() <= 256 {
        let cur = web[i];
        i += 1;
        let Some(v) = data.vbank().get(cur) else { continue };
        let mut next = Vec::new();
        if let Some(o) = v.get_def().and_then(|d| data.obank().get(d)) {
            if o.code() == OpCode::CPUI_MULTIEQUAL {
                next.extend((0..o.num_input()).filter_map(|k| o.get_in(k)));
            }
        }
        for op in v.descend_iter() {
            if let Some(o) = data.obank().get(op).filter(|o| o.code() == OpCode::CPUI_MULTIEQUAL) {
                next.extend(o.get_out());
            }
        }
        for n in next {
            if !web.contains(&n) {
                web.push(n);
            }
        }
    }
    web
}

/// Is every value entering `web` from outside it an explicit narrowing to `w`
/// bytes: a zero extension from at most `w` bytes, a mask that keeps only those
/// bytes, a constant, or a copy?
fn narrowing_defs(data: &Funcdata, web: &[VarnodeId], w: int4) -> bool {
    let low = calc_mask(w);
    web.iter().all(|&vn| {
        let Some(o) = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            return false;
        };
        let input = |k: int4| o.get_in(k).and_then(|x| data.vbank().get(x));
        match o.code() {
            OpCode::CPUI_MULTIEQUAL => true,
            OpCode::CPUI_COPY => input(0).is_some_and(|c| {
                !c.is_constant() || (c.get_offset() & !low == 0 && !image_address(data, c.get_offset()))
            }),
            OpCode::CPUI_INT_ZEXT => input(0).is_some_and(|x| x.get_size() <= w && !x.is_addr_tied()),
            OpCode::CPUI_INT_AND => {
                (0..2).any(|k| input(k).is_some_and(|c| c.is_constant() && c.get_offset() & !low == 0))
            }
            _ => false,
        }
    })
}

/// Is `web` one masked value read several times, with no MULTIEQUAL? Its
/// narrow value would be the low part of the wider one, which every reader
/// that needs the register's width widens back into a mask of its own.
fn masked_copy(data: &Funcdata, web: &[VarnodeId]) -> bool {
    web.len() == 1 && def_code(data, web[0]) == Some(OpCode::CPUI_INT_AND)
}

/// Could constant `c` be the address of something in the loaded image (a
/// string in a small position-independent image)? The first page holds no
/// data a program points at.
fn image_address(data: &Funcdata, c: uintb) -> bool {
    if c < 0x1000 {
        return false;
    }
    let Some(loader) = data.get_arch().loader.as_ref() else { return false };
    let Ok(image) = loader.try_borrow() else { return true };
    match image.shared_bytes() {
        Some(bytes) => bytes.mapped_covers(c, c + 1),
        None => image.get_segments().iter().any(|&(lo, hi, _)| lo <= c && c < hi),
    }
}

/// Narrow every register-local web whose values all fit a low part. Returns
/// how many webs were rebuilt. A web whose flow did not close is not traced
/// again from the same seed in a later `fullloop` pass.
pub fn narrow_locals(data: &mut Funcdata) -> int4 {
    if !data.get_arch().narrow_locals || data.bblocks_get_size() == 0 || data.kuna_regnarrow_runs >= MAX_RUNS {
        return 0;
    }
    let signed = data.get_arch().narrow_signed;
    let seeds: Vec<VarnodeId> = data
        .vbank()
        .iter_loc()
        .filter(|&vn| {
            (fits_low_half(data, vn)
                || (signed && matches!(def_code(data, vn), Some(OpCode::CPUI_INT_SEXT | OpCode::CPUI_MULTIEQUAL))))
                && plain_container(data, vn)
                && (def_code(data, vn) == Some(OpCode::CPUI_MULTIEQUAL)
                    || data.vbank().get(vn).is_some_and(|v| v.descend_iter().nth(1).is_some()))
        })
        .collect();
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut count = 0;
    for seed in seeds {
        if seen.contains(&seed) || data.kuna_regnarrow_failed.contains(&seed) {
            continue;
        }
        let web = web_of(data, seed);
        seen.extend(web.iter().copied());
        let Some(size) = data.vbank().get(seed).map(|v| v.get_size()) else { continue };
        if web.len() > 256
            || !web.iter().all(|&vn| plain_container(data, vn) && data.vbank().get(vn).is_some_and(|v| v.get_size() == size))
        {
            continue;
        }
        let nz = web.iter().filter_map(|&vn| data.vbank().get(vn)).fold(0, |m, v| m | v.get_nz_mask());
        let Some(w) = narrow_width(nz, size) else {
            if signed {
                if let Some(w) = signed_width(data, &web, size) {
                    count += narrow_signed_web(data, seed, w, &mut seen);
                }
            }
            continue;
        };
        if !narrowing_defs(data, &web, w) || masked_copy(data, &web) {
            continue;
        }
        match crate::subflow::run_zext_uses(data, seed, calc_mask(w)) {
            Some((flow, narrow)) => {
                seen.extend(flow);
                data.kuna_regnarrow_values.extend(narrow);
                count += 1;
                if trace() {
                    eprintln!(
                        "[regnarrow] 0x{:x} web of {} {size}-byte varnodes -> {w}",
                        data.get_address().get_offset(),
                        web.len()
                    );
                }
            }
            None => {
                data.kuna_regnarrow_failed.insert(seed);
            }
        }
    }
    if count > 0 {
        data.kuna_regnarrow_runs += 1;
    }
    count
}

/// The width `w` every value of `web` is the sign extension of, when each
/// enters it as a sign extension from at most `w` bytes (at least one from
/// exactly `w`), a copy, a bitwise op of such values or a constant that is the
/// sign extension of its low `w` bytes. A web with no phi that only widens
/// function inputs is left alone: its narrow value would be the input itself,
/// and every reader would widen it again.
fn signed_width(data: &Funcdata, web: &[VarnodeId], size: int4) -> Option<int4> {
    let mut w = 0;
    let mut constants = Vec::new();
    let mut phi = false;
    let mut of_inputs = true;
    for &vn in web {
        let o = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d))?;
        let input = |k: int4| o.get_in(k).and_then(|x| data.vbank().get(x));
        match o.code() {
            OpCode::CPUI_MULTIEQUAL => phi = true,
            OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_NEGATE => {
                of_inputs = false;
            }
            OpCode::CPUI_COPY => {
                let c = input(0)?;
                if c.is_constant() {
                    constants.push(c.get_offset());
                }
            }
            OpCode::CPUI_INT_SEXT => {
                let x = input(0)?;
                if x.is_addr_tied() {
                    return None;
                }
                of_inputs &= x.is_input();
                w = w.max(x.get_size());
            }
            _ => return None,
        }
    }
    if w == 0 || w >= size || (of_inputs && !phi) || wide_frame_home(data, web, w) || feeds_switch(data, web) {
        return None;
    }
    let fits = |c: uintb| kuna_base::address::sign_extend_sized(c & calc_mask(w), w, size) == c;
    constants.into_iter().all(fits).then_some(w)
}

/// Does a value of `web` reach a BRANCHIND within a few ops? A switch
/// selector is left at the width its jump table was recovered for: the table's
/// model is matched again on the final p-code, through the selector's
/// definition.
fn feeds_switch(data: &Funcdata, web: &[VarnodeId]) -> bool {
    let mut seen: BTreeSet<VarnodeId> = web.iter().copied().collect();
    let mut frontier: Vec<VarnodeId> = web.to_vec();
    for _ in 0..8 {
        let mut next = Vec::new();
        for vn in frontier {
            let Some(v) = data.vbank().get(vn) else { continue };
            for op in v.descend_iter() {
                let Some(o) = data.obank().get(op) else { continue };
                if o.code() == OpCode::CPUI_BRANCHIND {
                    return true;
                }
                if let Some(out) = o.get_out() {
                    if seen.len() < 512 && seen.insert(out) {
                        next.push(out);
                    }
                }
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    false
}

/// Was a value of `web` stored to a frame slot wider than `w` bytes before
/// simplification removed the slot? The program's own home for it is wide: the
/// `-O0` spill of `int v = c;`.
fn wide_frame_home(data: &Funcdata, web: &[VarnodeId], w: int4) -> bool {
    let Some(stack) = data.get_arch().manage().get_stack_space() else { return false };
    web.iter().any(|&vn| {
        data.kuna_storage_sources(vn)
            .iter()
            .any(|s| s.size > w && s.address.get_space().is_some_and(|sp| sp.get_index() == stack.get_index()))
    })
}

/// Rebuild the sign-extended web of `seed` at `w` bytes; 1 when it was.
fn narrow_signed_web(data: &mut Funcdata, seed: VarnodeId, w: int4, seen: &mut BTreeSet<VarnodeId>) -> int4 {
    match crate::subflow::run_sext_uses(data, seed, calc_mask(w)) {
        Some((flow, narrow)) => {
            seen.extend(flow);
            data.kuna_regsigned_values.extend(narrow);
            if trace() {
                eprintln!("[regsigned] 0x{:x} -> {w}", data.get_address().get_offset());
            }
            1
        }
        None => {
            if trace() {
                eprintln!("[regsigned-fail] 0x{:x} -> {w}", data.get_address().get_offset());
            }
            data.kuna_regnarrow_failed.insert(seed);
            0
        }
    }
}

/// Is the decision trace on (`KUNA_REGNARROW_TRACE=1`)?
fn trace() -> bool {
    std::env::var("KUNA_REGNARROW_TRACE").is_ok_and(|v| v != "0" && !v.is_empty())
}

/// The type a value this pass narrowed takes when inference found nothing
/// more specific than its size: the zero-extended low part of a register is
/// unsigned. The sign-extended low part is signed, also over the `uint` a
/// bitwise op votes for its output (the program sign-extends it).
pub fn unsigned_vote(data: &Funcdata, vn: VarnodeId, ct: &Datatype) -> Option<std::rc::Rc<Datatype>> {
    let meta = match ct.get_metatype() {
        type_metatype::TYPE_UNKNOWN if data.kuna_regnarrow_values.contains(&vn) => type_metatype::TYPE_UINT,
        type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_UINT if data.kuna_regsigned_values.contains(&vn) => {
            type_metatype::TYPE_INT
        }
        _ => return None,
    };
    data.get_arch().types()?.get_base(ct.get_size(), meta).ok()
}
