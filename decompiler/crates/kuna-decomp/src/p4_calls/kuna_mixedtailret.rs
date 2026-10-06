//! (kuna) `mixedtailret` — a function that hands back a tail call's result on
//! one path returns the value it leaves in that register on the others too.
//!
//! # The symptom
//!
//! ```text
//!   int keep(int *p) { int x = *p; if (x > 5) return x; return getv(); }
//!
//!   mov (%rdi),%eax; cmp $5,%eax; jle L; ret; L: jmp getv     (clang -O2)
//! ```
//!
//! printed `void keep(int *a0) { if (6 <= *a0) return; getv(); }`. The `ret`
//! path's `eax` is also compared, so upstream's `onlyOpUse` refuses it as a
//! return value, and `passthrough` takes a tail call's result only when EVERY
//! live RETURN hands one back.
//!
//! # The rule
//!
//! A function's return value lives in one storage on every RETURN. When some
//! RETURN hands back the result of a call whose callee's recovered prototype
//! returns a value there (what `passthrough` already claims), each other RETURN
//! is taken as returning that storage too, provided the function itself wrote
//! it last on every path into that RETURN ([`writes_own_value`]): no call in
//! between, no path from the entry that leaves the incoming register there,
//! and a write covering the whole storage. A RETURN reached from a call whose
//! callee states nothing, or another storage, refuses the claim as before.
//!
//! The return register is an argument register on ARM, AArch64, RISC-V and
//! PowerPC, where a `void` function null-checks the pointer it loaded into
//! `x0` and tail-calls `release(x0)`; there the claim also needs every claimed
//! tail callee to take no parameter in it ([`feeds_tail_call`]).
//!
//! The claim is a judgment: a `void` function may tail-call a value-returning
//! one and leave a scratch value in the register on its other paths. It is
//! inert wherever `passthrough` is (no callee decompiled first, `--option
//! passthrough off`). Default-**on**; `off` keeps a RETURN that is not a tail
//! call out of the claim.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// `option mixedtailret on|off`.
pub struct OptionMixedTailRet;

impl OptionMixedTailRet {
    /// The option name.
    pub const NAME: &'static str = "mixedtailret";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::mixed_tail_ret`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Mixed tail-call return recovery turned {prop}")))
    }
}

/// How many blocks [`last_write_is_own`] visits for one RETURN.
const OWN_WALK_BLOCKS: usize = 32;

/// Does the function itself write each of `pieces` last on every path into
/// `ret`?
///
/// Asked of the raw p-code before the first heritage. Walking back from `ret`
/// over every predecessor, the first op whose output shares a byte with a
/// piece must cover all of it and be an ordinary op of the function; a CALL or
/// CALLIND met first (its result or clobber is what the RETURN would read),
/// a CALLOTHER output, a partial write, the function's entry (the incoming
/// register) and a walk past [`OWN_WALK_BLOCKS`] all refuse. The `r0 = r0`
/// mode switch injected at an ARM return moves nothing and is walked past.
pub fn writes_own_value(data: &Funcdata, ret: OpId, pieces: &[(Address, int4)]) -> bool {
    data.get_arch().mixed_tail_ret && pieces.iter().all(|(a, s)| last_write_is_own(data, ret, a, *s))
}

/// Does any claimed tail callee take a parameter in a byte of `pieces`?
///
/// `producers` are the calls whose stated result the RETURNs hand back.
pub fn feeds_tail_call(data: &Funcdata, producers: &[OpId], pieces: &[(Address, int4)]) -> bool {
    producers.iter().any(|&call| {
        let Some(idx) = (0..data.num_calls()).find(|&i| data.get_call_specs(i).get_op() == call) else {
            return true;
        };
        let Some(stated) = data.kuna_protoorder_types(data.get_call_specs(idx).get_entry_address()) else {
            return true;
        };
        stated.inputs.iter().any(|(a, s, _)| pieces.iter().any(|(pa, ps)| overlaps(a, *s, pa, *ps)))
    })
}

fn overlaps(a: &Address, asize: int4, b: &Address, bsize: int4) -> bool {
    let same = match (a.get_space(), b.get_space()) {
        (Some(x), Some(y)) => x.get_index() == y.get_index(),
        _ => false,
    };
    let (aoff, boff) = (a.get_offset(), b.get_offset());
    same && aoff < boff.wrapping_add(bsize.max(0) as u64) && boff < aoff.wrapping_add(asize.max(0) as u64)
}

/// What the op at a step of the walk does to the piece.
enum Write {
    None,
    Own,
    Refuse,
}

fn write_of(data: &Funcdata, op: OpId, addr: &Address, size: int4) -> Write {
    let Some(o) = data.obank().get(op) else { return Write::Refuse };
    if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
        return Write::Refuse;
    }
    let Some(out) = o.get_out().and_then(|v| data.vbank().get(v)) else { return Write::None };
    if !overlaps(out.get_addr(), out.get_size(), addr, size) || crate::p4_calls::kuna_passthrough::is_injected_noop(data, op)
    {
        return Write::None;
    }
    let (off, end) = (out.get_offset(), out.get_offset().wrapping_add(out.get_size().max(0) as u64));
    let covers = off <= addr.get_offset() && addr.get_offset().wrapping_add(size.max(0) as u64) <= end;
    if covers && o.code() != OpCode::CPUI_CALLOTHER {
        Write::Own
    } else {
        Write::Refuse
    }
}

fn last_write_is_own(data: &Funcdata, ret: OpId, addr: &Address, size: int4) -> bool {
    let Some(start) = data.obank().get(ret).and_then(|o| o.get_parent()) else { return false };
    let entry = data.bblocks_get_block(0);
    let mut seen = vec![start];
    let mut work = vec![(start, data.op_previous_op(ret))];
    while let Some((bl, mut cur)) = work.pop() {
        let mut written = false;
        while let Some(op) = cur {
            match write_of(data, op, addr, size) {
                Write::None => cur = data.op_previous_op(op),
                Write::Own => {
                    written = true;
                    break;
                }
                Write::Refuse => return false,
            }
        }
        if written {
            continue;
        }
        let b = data.bblocks_ref().block(bl);
        if bl == entry || b.size_in() == 0 {
            return false;
        }
        for k in 0..b.size_in() {
            let pred = b.get_in(k);
            if seen.contains(&pred) {
                continue;
            }
            if seen.len() >= OWN_WALK_BLOCKS {
                return false;
            }
            seen.push(pred);
            work.push((pred, data.bb_op_tail(pred)));
        }
    }
    true
}

#[cfg(test)]
mod tests;
