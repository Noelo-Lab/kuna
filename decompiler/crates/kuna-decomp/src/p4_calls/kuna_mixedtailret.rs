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
//! the value there is one the function computed only to test it and return it
//! ([`returns_decided_value`]):
//!
//! * the function itself wrote the register last on every path into the
//!   RETURN, as wide as the callee states it: no call in between, no path from
//!   the entry that leaves the incoming register there;
//! * walking up the single-predecessor chain from the RETURN, the nearest
//!   conditional branch whose other side reaches a claimed tail call tests that
//!   value, in the register or in the one it was copied from on the way (gcc's
//!   `cmp $5,%edi; jg L; jmp getk; L: mov %edi,%eax; ret`);
//! * from where it is computed, the value only decides branches, moves between
//!   registers and reaches the claimed tail calls: it is not loaded or stored
//!   through, stored, counted or passed to another call.
//!
//! A value the function only computed and handed to the RETURN is one upstream
//! already returns; the test that chooses between the two returns is the use
//! upstream refuses. The other two conditions are what a scratch value in a
//! `void` function fails: the stack-protector check `sub %fs:0x28,%rax; jne
//! fail` leaves `0` in `rax` before every `ret` of a `void` function, but its
//! branch leads to `__stack_chk_fail`, not to the tail call; `if (!tb[i])
//! return;` tests the pointer it then loads through; `if (guard++) return;`
//! counts the value it tests.
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
use kuna_base::space::spacetype;
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

/// How many single-predecessor blocks [`decided_origin`] climbs.
const CHAIN_BLOCKS: usize = 8;

/// How many blocks the other side of a branch is searched for a tail call.
const REACH_BLOCKS: usize = 16;

/// Does `ret`, a RETURN reached from no call, return the value in `pieces` the
/// function branched on, beside the claimed tail calls `tails` (each a CALL
/// and the RETURN after it)?
///
/// Asked of the raw p-code before the first heritage, for a claim of one
/// register: the function wrote it on every path ([`last_write_is_own`]), the
/// branch between the two tests it ([`decided_origin`]), and it is put to no
/// other use ([`only_decides`]).
pub fn returns_decided_value(data: &Funcdata, ret: OpId, pieces: &[(Address, int4)], tails: &[(OpId, OpId)]) -> bool {
    let [(addr, size)] = pieces else { return false };
    if !data.get_arch().mixed_tail_ret || !last_write_is_own(data, ret, addr, *size) {
        return false;
    }
    let (calls, rets): (Vec<OpId>, Vec<OpId>) = tails.iter().copied().unzip();
    decided_origin(data, ret, addr, *size, &rets).is_some_and(|origin| only_decides(data, origin, &calls))
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

/// Was the function's tail claim made beside a value it returns itself?
///
/// `keep_tail_return_whole` then keeps the claim whole without asking
/// `returns_own_value`, whose evidence is a value of another storage class the
/// function computed itself: beside a value the function branched on in the
/// claimed register, the `xmm0` that `-fzero-call-used-regs` zeroes before one
/// `ret` and a tail callee leaves alone before the other is no such value, and
/// yielding to it returned `xmm0` where the function returns `eax`.
pub fn claimed_beside_own(data: &Funcdata) -> bool {
    data.get_arch().mixed_tail_ret && data.kuna_passthrough_claims().iter().any(|c| c.beside_own)
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

/// The op that computed the value `ret` returns in `[addr, addr+size)`, when
/// the nearest branch above `ret` that can lead to a claimed tail call tests
/// that value.
///
/// Climbs the single-predecessor chain from `ret` op by op, following the
/// returned value back through copies and extensions into the register they
/// read (`mov %edi,%eax`). The first block on the chain ending in a CBRANCH
/// whose other side reaches the block of a RETURN in `tail_rets`
/// ([`reaches_tail`]) must compute its condition from that value
/// ([`condition_feeders`]); the climb then goes on to the op that wrote the
/// value other than by copying it. Any other write of the value before the
/// test, a join, the entry, a call or [`CHAIN_BLOCKS`] blocks first refuse.
fn decided_origin(data: &Funcdata, ret: OpId, addr: &Address, size: int4, tail_rets: &[OpId]) -> Option<OpId> {
    let tail_blocks: Vec<BlockId> =
        tail_rets.iter().filter_map(|&r| data.obank().get(r).and_then(|o| o.get_parent())).collect();
    let mut bl = data.obank().get(ret)?.get_parent()?;
    let mut tracked = (addr.clone(), size);
    let mut cur = data.op_previous_op(ret);
    let mut feeders: Option<Vec<OpId>> = None;
    let mut decided = false;
    for _ in 0..CHAIN_BLOCKS {
        while let Some(op) = cur {
            let o = data.obank().get(op)?;
            if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                return None;
            }
            if !decided && feeders.as_ref().is_some_and(|f| f.contains(&op)) {
                decided = (0..o.num_input())
                    .filter_map(|i| o.get_in(i))
                    .filter_map(|v| data.vbank().get(v))
                    .any(|v| overlaps(v.get_addr(), v.get_size(), &tracked.0, tracked.1));
            }
            let out = o.get_out().and_then(|v| data.vbank().get(v));
            if out.is_some_and(|v| overlaps(v.get_addr(), v.get_size(), &tracked.0, tracked.1))
                && !crate::p4_calls::kuna_passthrough::is_injected_noop(data, op)
            {
                match register_copy_source(data, op) {
                    Some(source) => tracked = source,
                    None => return decided.then_some(op),
                }
            }
            cur = data.op_previous_op(op);
        }
        if feeders.is_some() && !decided {
            return None;
        }
        let b = data.bblocks_ref().block(bl);
        if bl == data.bblocks_get_block(0) || b.size_in() != 1 {
            return None;
        }
        let pred = b.get_in(0);
        if !decided {
            let p = data.bblocks_ref().block(pred);
            let other = (0..p.size_out()).map(|k| p.get_out(k)).find(|&s| s != bl);
            feeders = other.filter(|&o| reaches_tail(data, o, &tail_blocks)).map(|_| condition_feeders(data, pred));
        }
        bl = pred;
        cur = data.bb_op_tail(pred);
    }
    None
}

/// The register a COPY or extension `op` reads, when it moves one register's
/// value into another.
fn register_copy_source(data: &Funcdata, op: OpId) -> Option<(Address, int4)> {
    let o = data.obank().get(op)?;
    if !matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT) {
        return None;
    }
    let v = data.vbank().get(o.get_in(0)?)?;
    (v.get_space().get_type() == spacetype::IPTR_PROCESSOR).then(|| (v.get_addr().clone(), v.get_size()))
}

/// How many blocks [`only_decides`] follows the value through.
const USE_WALK_BLOCKS: usize = 32;

/// Does the value `origin` computes do nothing but decide branches, move into
/// other registers, and reach `tail_calls`?
///
/// Followed forward from `origin` over every path, before heritage: a register
/// copy or extension of it is the value again; any other op reading it, or
/// reading what was computed from it, computes a derived value, and a derived
/// value may only reach a CBRANCH (the flags of `cmp $5,%eax`, MIPS `slti`). A
/// LOAD or STORE through or of either, a CALLOTHER or BRANCHIND reading either,
/// and a call other than one of `tail_calls` while either sits in a register
/// the function's model passes arguments in all refuse: the value is used as a
/// pointer, stored, counted (`lea 1(%rax),%edx` of a recursion guard), or
/// handed to a call, which a returned value the function only tested is not.
/// A write of a register forgets what it held; a call forgets the registers it
/// may return or take arguments in.
fn only_decides(data: &Funcdata, origin: OpId, tail_calls: &[OpId]) -> bool {
    let proto = data.get_func_proto();
    let Some(o) = data.obank().get(origin) else { return false };
    let Some(out) = o.get_out().and_then(|v| data.vbank().get(v)) else { return false };
    let Some(start) = o.get_parent() else { return false };
    type Set = Vec<(Address, int4)>;
    let storage = |v: VarnodeId| data.vbank().get(v).filter(|v| !v.is_constant()).map(|v| (v.get_addr().clone(), v.get_size()));
    let hits = |set: &Set, a: &(Address, int4)| set.iter().any(|(b, t)| overlaps(&a.0, a.1, b, *t));
    let mut seen = vec![start];
    let mut work: Vec<(BlockId, Option<OpId>, Set, Set)> =
        vec![(start, o.basic_neighbours().1, vec![(out.get_addr().clone(), out.get_size())], Vec::new())];
    while let Some((bl, mut cur, mut value, mut derived)) = work.pop() {
        while let Some(op) = cur {
            let Some(o) = data.obank().get(op) else { return false };
            cur = o.basic_neighbours().1;
            let ins: Set = (0..o.num_input()).filter_map(|i| o.get_in(i)).filter_map(storage).collect();
            let reads_value = ins.iter().any(|i| hits(&value, i));
            let reads_derived = ins.iter().any(|i| hits(&derived, i));
            let out = o.get_out().and_then(storage);
            match o.code() {
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    let passes = |(a, s): &(Address, int4)| proto.possible_input_param(a, *s);
                    if !tail_calls.contains(&op) && (value.iter().any(passes) || derived.iter().any(passes)) {
                        return false;
                    }
                    let clobbered = |(a, s): &(Address, int4)| {
                        proto.possible_input_param(a, *s)
                            || proto.characterize_as_output(a, *s) != crate::fspec::Containment::NoContainment
                    };
                    value.retain(|r| !clobbered(r));
                    derived.retain(|r| !clobbered(r));
                    continue;
                }
                OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCH | OpCode::CPUI_RETURN => continue,
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER | OpCode::CPUI_BRANCHIND
                    if reads_value || reads_derived =>
                {
                    return false;
                }
                _ => {}
            }
            let Some(out) = out else { continue };
            value.retain(|r| !overlaps(&r.0, r.1, &out.0, out.1));
            derived.retain(|r| !overlaps(&r.0, r.1, &out.0, out.1));
            if reads_value && !reads_derived && register_copy_source(data, op).is_some() {
                value.push(out);
            } else if reads_value || reads_derived {
                derived.push(out);
            }
        }
        if value.is_empty() && derived.is_empty() {
            continue;
        }
        let b = data.bblocks_ref().block(bl);
        for k in 0..b.size_out() {
            let next = b.get_out(k);
            if seen.contains(&next) {
                continue;
            }
            if seen.len() >= USE_WALK_BLOCKS {
                return false;
            }
            seen.push(next);
            work.push((next, data.bb_op_head(next), value.clone(), derived.clone()));
        }
    }
    true
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
