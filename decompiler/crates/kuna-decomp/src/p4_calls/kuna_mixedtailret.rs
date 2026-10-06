//! (kuna) `mixedtailret` — a function that hands back a tail call's result on
//! one path, and on another returns the value it branched on to get there,
//! returns that value.
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
//! When some RETURN hands back the result of a call whose callee's recovered
//! prototype returns a value in one register (what `passthrough` claims), a
//! RETURN reached from no call is taken as returning that register too when
//! the value there is the one the function branched on to choose between the
//! two ([`returns_decided_value`]):
//!
//! * the function itself wrote the register last on every path into the
//!   RETURN, as wide as the callee states it: no call in between, no path from
//!   the entry that leaves the incoming register there;
//! * walking up the single-predecessor chain from the RETURN, the nearest
//!   conditional branch whose other side reaches a claimed tail call tests that
//!   value, in the register or in the one it was copied from on the way (gcc's
//!   `cmp $5,%edi; jg L; jmp getk; L: mov %edi,%eax; ret`).
//!
//! A value the function only computed and handed to the RETURN is one upstream
//! already returns. What the branch test adds is that the value decides which
//! of the two returns is taken, which a scratch value does not: the
//! stack-protector check `sub %fs:0x28,%rax; jne fail` leaves `0` in `rax`
//! before every `ret` of a `void` function, and its branch leads to
//! `__stack_chk_fail`, not to the tail call.
//!
//! The return register is an argument register on ARM, AArch64, RISC-V and
//! PowerPC, where a `void` function null-checks the pointer it loaded into
//! `x0` and tail-calls `release(x0)`; there the claim also needs every claimed
//! tail callee to take no parameter in it ([`feeds_tail_call`]).
//!
//! The claim is a judgment: a `void` function may also branch on a value it
//! leaves in the return register before tail-calling a value-returning one.
//! It is inert wherever `passthrough` is (no callee decompiled first,
//! `--option passthrough off`). Default-**on**; `off` keeps a RETURN that is
//! not a tail call out of the claim.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
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

/// How many single-predecessor blocks [`decided_by_value`] climbs.
const CHAIN_BLOCKS: usize = 8;

/// How many blocks the other side of a branch is searched for a tail call.
const REACH_BLOCKS: usize = 16;

/// Does `ret`, a RETURN reached from no call, return the value in `pieces` the
/// function branched on, beside the claimed tail calls ending at `tail_rets`?
///
/// Asked of the raw p-code before the first heritage, for a claim of one
/// register ([`last_write_is_own`], [`decided_by_value`]).
pub fn returns_decided_value(data: &Funcdata, ret: OpId, pieces: &[(Address, int4)], tail_rets: &[OpId]) -> bool {
    let [(addr, size)] = pieces else { return false };
    data.get_arch().mixed_tail_ret
        && last_write_is_own(data, ret, addr, *size)
        && decided_by_value(data, ret, addr, *size, tail_rets)
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

/// Is `vn`, read by a RETURN, what a claimed tail call leaves in a register it
/// does not return in: the INDIRECT creation the call's clobber planted, read
/// directly or through a low SUBPIECE or an injected no-op?
///
/// `returns_own_value` asks it of the trials outside the claim: the `xmm0`
/// `jmp xstrdup` leaves beside the `pxor %xmm0,%xmm0` that
/// `-fzero-call-used-regs` puts before the function's other `ret` is neither
/// path's value.
pub fn is_tail_leftover(data: &Funcdata, vn: VarnodeId) -> bool {
    if !data.get_arch().mixed_tail_ret {
        return false;
    }
    let mut def = data.vbank().get(vn).and_then(|v| v.get_def());
    for _ in 0..4 {
        let Some(o) = def.and_then(|d| data.obank().get(d)) else { return false };
        let low = o.code() == OpCode::CPUI_SUBPIECE
            && o.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant() && c.get_offset() == 0);
        if !low && !def.is_some_and(|d| crate::p4_calls::kuna_passthrough::is_injected_noop(data, d)) {
            break;
        }
        def = o.get_in(0).and_then(|i| data.vbank().get(i)).and_then(|i| i.get_def());
    }
    let Some(o) = def.and_then(|d| data.obank().get(d)) else { return false };
    if !o.is_indirect_creation() {
        return false;
    }
    let Some(iop) = o.get_in(1).and_then(|i| data.vbank().get(i)) else { return false };
    let call = OpId::from(slotmap::KeyData::from_ffi(iop.get_offset()));
    data.kuna_passthrough_claims().iter().any(|c| c.ret_owners.contains(&call))
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
    if covers && o.code() != OpCode::CPUI_CALLOTHER && value_width(data, op) == Some(size) {
        Write::Own
    } else {
        Write::Refuse
    }
}

/// How wide the value `op` writes is: its output, or the input an extension
/// widened (`RAX = zext(EAX)` after every x86-64 32-bit write).
fn value_width(data: &Funcdata, op: OpId) -> Option<int4> {
    let o = data.obank().get(op)?;
    let vn = match o.code() {
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => o.get_in(0)?,
        _ => o.get_out()?,
    };
    data.vbank().get(vn).map(|v| v.get_size())
}

/// Does the function itself write `[addr, addr+size)` last on every path into
/// `ret`?
///
/// Walking back from `ret` over every predecessor, the first op whose output
/// shares a byte with the register must cover all of it with a value as wide
/// as `size` ([`value_width`]) and be an ordinary op of the function; a CALL or
/// CALLIND met first (its result or clobber is what the RETURN would read), a
/// CALLOTHER output, a partial write, the function's entry (the incoming
/// register) and a walk past [`OWN_WALK_BLOCKS`] all refuse. The `r0 = r0`
/// mode switch injected at an ARM return moves nothing and is walked past.
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

/// Is the value `ret` returns in `[addr, addr+size)` the one the nearest
/// branch above it that can lead to a claimed tail call tests?
///
/// Climbs the single-predecessor chain from `ret` op by op, following the
/// returned value back through copies and extensions into the register they
/// read (`mov %edi,%eax`), and stops at any other write of it. The first block
/// on the chain ending in a CBRANCH whose other side reaches the block of a
/// RETURN in `tail_rets` ([`reaches_tail`]) must compute its condition from that
/// value ([`condition_feeders`]); a join, the entry, a call or [`CHAIN_BLOCKS`]
/// blocks first refuse.
fn decided_by_value(data: &Funcdata, ret: OpId, addr: &Address, size: int4, tail_rets: &[OpId]) -> bool {
    let tail_blocks: Vec<BlockId> =
        tail_rets.iter().filter_map(|&r| data.obank().get(r).and_then(|o| o.get_parent())).collect();
    let Some(mut bl) = data.obank().get(ret).and_then(|o| o.get_parent()) else { return false };
    let mut tracked = (addr.clone(), size);
    let mut cur = data.op_previous_op(ret);
    let mut feeders: Option<Vec<OpId>> = None;
    for _ in 0..CHAIN_BLOCKS {
        while let Some(op) = cur {
            let Some(o) = data.obank().get(op) else { return false };
            if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                return false;
            }
            let reads = |(a, s): &(Address, int4)| {
                (0..o.num_input()).filter_map(|i| o.get_in(i)).filter_map(|v| data.vbank().get(v)).any(|v| overlaps(v.get_addr(), v.get_size(), a, *s))
            };
            if feeders.as_ref().is_some_and(|f| f.contains(&op)) && reads(&tracked) {
                return true;
            }
            let out = o.get_out().and_then(|v| data.vbank().get(v));
            if out.is_some_and(|v| overlaps(v.get_addr(), v.get_size(), &tracked.0, tracked.1))
                && !crate::p4_calls::kuna_passthrough::is_injected_noop(data, op)
            {
                let source = matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT)
                    .then(|| o.get_in(0).and_then(|v| data.vbank().get(v)))
                    .flatten()
                    .filter(|v| v.get_space().get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR);
                match source {
                    Some(v) => tracked = (v.get_addr().clone(), v.get_size()),
                    None => return false,
                }
            }
            cur = data.op_previous_op(op);
        }
        if feeders.is_some() {
            return false;
        }
        let b = data.bblocks_ref().block(bl);
        if bl == data.bblocks_get_block(0) || b.size_in() != 1 {
            return false;
        }
        let pred = b.get_in(0);
        let p = data.bblocks_ref().block(pred);
        let other = (0..p.size_out()).map(|k| p.get_out(k)).find(|&s| s != bl);
        feeders = other.filter(|&o| reaches_tail(data, o, &tail_blocks)).map(|_| condition_feeders(data, pred));
        bl = pred;
        cur = data.bb_op_tail(pred);
    }
    false
}

/// Does a block within [`REACH_BLOCKS`] forward of `from` hold one of `tails`?
fn reaches_tail(data: &Funcdata, from: BlockId, tails: &[BlockId]) -> bool {
    let mut seen = vec![from];
    let mut work = vec![from];
    while let Some(bl) = work.pop() {
        if tails.contains(&bl) {
            return true;
        }
        let b = data.bblocks_ref().block(bl);
        for k in 0..b.size_out() {
            let next = b.get_out(k);
            if !seen.contains(&next) && seen.len() < REACH_BLOCKS {
                seen.push(next);
                work.push(next);
            }
        }
    }
    false
}

/// The ops of `bl` whose outputs its closing CBRANCH's condition is computed
/// from, found by storage before heritage: the flag ops of `cmp $5,%eax; jg`.
fn condition_feeders(data: &Funcdata, bl: BlockId) -> Vec<OpId> {
    let mut feeders = Vec::new();
    let Some(tail) = data.bb_op_tail(bl) else { return feeders };
    let Some(t) = data.obank().get(tail).filter(|o| o.code() == OpCode::CPUI_CBRANCH) else { return feeders };
    let storage = |v: VarnodeId| data.vbank().get(v).filter(|v| !v.is_constant()).map(|v| (v.get_addr().clone(), v.get_size()));
    let mut wanted: Vec<(Address, int4)> = t.get_in(1).and_then(storage).into_iter().collect();
    let mut cur = data.op_previous_op(tail);
    while let Some(op) = cur {
        let Some(o) = data.obank().get(op) else { break };
        if let Some(out) = o.get_out().and_then(storage) {
            if let Some(k) = wanted.iter().position(|w| *w == out) {
                wanted.swap_remove(k);
                feeders.push(op);
                wanted.extend((0..o.num_input()).filter_map(|i| o.get_in(i)).filter_map(storage));
            }
        }
        cur = data.op_previous_op(op);
    }
    feeders
}

#[cfg(test)]
mod tests;
