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
//!   RETURN, no wider than the callee states it: no call in between, no path from
//!   the entry that leaves the incoming register there;
//! * walking up the single-predecessor chain from the RETURN, the nearest
//!   conditional branch whose other side reaches a claimed tail call tests that
//!   value, in the register or in the one it was copied from on the way (gcc's
//!   `cmp $5,%edi; jg L; jmp getk; L: mov %edi,%eax; ret`), and the RETURN is
//!   not on the side where the test found the value equal to a constant;
//! * from where it is computed, the value only decides branches, moves between
//!   registers and reaches the claimed tail calls: it is not loaded or stored
//!   through, stored, counted or passed to another call.
//!
//! A value the function only computed and handed to the RETURN is one upstream
//! already returns; the test that chooses between the two returns is the use
//! upstream refuses. The other two conditions are what a scratch value in a
//! `void` function fails: the stack-protector check `sub %fs:0x28,%rax; jne
//! fail` leaves `0` in `rax` before every `ret` of a `void` function, but its
//! branch leads to `__stack_chk_fail`, not to the tail call; `if (!flag)
//! return;` returns the `0` it found; `if (!tb[i]) return;` tests the pointer
//! it then loads through; `if (guard++) return;` counts the value it tests.
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
    if covers && o.code() != OpCode::CPUI_CALLOTHER && value_width(data, op).is_some_and(|w| w <= size) {
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
/// shares a byte with the register must cover all of it with a value no wider
/// than `size` ([`value_width`]) and be an ordinary op of the function; a CALL or
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
/// ([`condition_feeders`]), and not be the side where that test found the value
/// equal to a constant ([`returns_the_constant`]); the climb then goes on to
/// the op that wrote the value other than by copying it. Any other write of the value before the
/// test, a join, the entry, a call or [`CHAIN_BLOCKS`] blocks first refuse.
fn decided_origin(data: &Funcdata, ret: OpId, addr: &Address, size: int4, tail_rets: &[OpId]) -> Option<OpId> {
    let tail_blocks: Vec<BlockId> =
        tail_rets.iter().filter_map(|&r| data.obank().get(r).and_then(|o| o.get_parent())).collect();
    let mut bl = data.obank().get(ret)?.get_parent()?;
    let mut tracked = (addr.clone(), size);
    let mut cur = data.op_previous_op(ret);
    let mut feeders: Option<Vec<OpId>> = None;
    let mut decided = false;
    let mut side: Option<(BlockId, BlockId)> = None;
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
                if decided && side.is_some_and(|(p, s)| returns_the_constant(data, p, s, &tracked)) {
                    return None;
                }
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
            side = Some((pred, bl));
        }
        bl = pred;
        cur = data.bb_op_tail(pred);
    }
    None
}

/// The register a COPY or extension `op` reads, when it moves one register's
/// value into another register or a temporary.
fn register_copy_source(data: &Funcdata, op: OpId) -> Option<(Address, int4)> {
    let o = data.obank().get(op)?;
    if !matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT) {
        return None;
    }
    let v = data.vbank().get(o.get_in(0)?)?;
    let out = data.vbank().get(o.get_out()?)?;
    (is_register(v.get_addr()) && held(out.get_addr())).then(|| (v.get_addr().clone(), v.get_size()))
}

/// Is `addr` in the register space?
fn is_register(addr: &Address) -> bool {
    addr.get_space().is_some_and(|s| s.get_name() == "register")
}

/// Is `addr` a register or a p-code temporary, rather than memory: a COPY to
/// `r0x4001c0`, which is what `mov %eax,g(%rip)` lifts to, is a store?
fn held(addr: &Address) -> bool {
    is_register(addr) || addr.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_INTERNAL)
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
/// LOAD or STORE through or of either, a write of either to memory (`mov
/// %eax,g(%rip)` lifts to a COPY into `r0x4001c0`), a CALLOTHER or BRANCHIND
/// reading either,
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
            if (reads_value || reads_derived) && !held(&out.0) {
                return false;
            }
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

/// Is `side`, a successor of the CBRANCH ending `bl`, the one where the branch
/// has tested `value` equal to a constant?
///
/// Then the RETURN there hands back a value it knows, not one the test let
/// through: `if (!flag) return; ...; clear();` loads `flag` into `eax` and
/// leaves the `0` there, and `void` functions do that as readily as ones
/// returning `0`. The condition is read by storage before heritage
/// ([`equality_test`]); the true edge is the branch taken.
fn returns_the_constant(data: &Funcdata, bl: BlockId, side: BlockId, value: &(Address, int4)) -> bool {
    let Some(cb) = data.bb_op_tail(bl) else { return false };
    let Some(o) = data.obank().get(cb).filter(|o| o.code() == OpCode::CPUI_CBRANCH) else { return false };
    let b = data.bblocks_ref().block(bl);
    if b.size_out() != 2 || b.get_true_out() == b.get_false_out() {
        return false;
    }
    let Some(eq) = o.get_in(1).and_then(|c| equality_test(data, cb, c, value, TEST_DEPTH)) else { return false };
    (eq != o.is_boolean_flip()) == (b.get_true_out() == side)
}

/// How many defining ops [`equality_test`] and [`pins`] follow.
const TEST_DEPTH: u32 = 8;

/// Is `cond`, read at `at`, true exactly where `value` equals a constant
/// (`Some(true)`), exactly where it does not (`Some(false)`), or neither?
fn equality_test(data: &Funcdata, at: OpId, cond: VarnodeId, value: &(Address, int4), depth: u32) -> Option<bool> {
    let c = data.vbank().get(cond)?;
    if depth == 0 || c.is_constant() {
        return None;
    }
    let def = def_before(data, at, c.get_addr(), c.get_size())?;
    let o = data.obank().get(def)?;
    match o.code() {
        OpCode::CPUI_BOOL_NEGATE => equality_test(data, def, o.get_in(0)?, value, depth - 1).map(|e| !e),
        OpCode::CPUI_COPY => equality_test(data, def, o.get_in(0)?, value, depth - 1),
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            let konst = |v: VarnodeId| data.vbank().get(v).is_some_and(|v| v.is_constant());
            let (a, b) = (o.get_in(0)?, o.get_in(1)?);
            let e = match (konst(a), konst(b)) {
                (false, true) => a,
                (true, false) => b,
                _ => return None,
            };
            pins(data, def, e, value, depth - 1).then_some(o.code() == OpCode::CPUI_INT_EQUAL)
        }
        _ => None,
    }
}

/// Is `e`, read at `at`, a one-to-one function of `value`, so that comparing
/// it with a constant compares `value` with one: the value itself, a copy or
/// extension of it, it plus, minus or exclusive-or a constant, or `x & x` (the
/// `test %eax,%eax` of a zero test)? Any part of the register counts as the
/// value: the `eax` tested before `rax = zext(eax)` is returned.
fn pins(data: &Funcdata, at: OpId, e: VarnodeId, value: &(Address, int4), depth: u32) -> bool {
    let Some(v) = data.vbank().get(e) else { return false };
    if overlaps(v.get_addr(), v.get_size(), &value.0, value.1) {
        return true;
    }
    if depth == 0 || v.is_constant() {
        return false;
    }
    let Some(def) = def_before(data, at, v.get_addr(), v.get_size()) else { return false };
    let Some(o) = data.obank().get(def) else { return false };
    let input = |k: int4| o.get_in(k).and_then(|x| data.vbank().get(x).map(|v| (x, v)));
    let (Some((i0, v0)), second) = (input(0), input(1)) else { return false };
    let follow = |x: VarnodeId| pins(data, def, x, value, depth - 1);
    match o.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_2COMP | OpCode::CPUI_INT_NEGATE => {
            follow(i0)
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_XOR => match second {
            Some((_, v1)) if v1.is_constant() => follow(i0),
            Some((i1, _)) if v0.is_constant() => follow(i1),
            _ => false,
        },
        OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR => match second {
            Some((_, v1)) if !v0.is_constant() && v0.get_addr() == v1.get_addr() && v0.get_size() == v1.get_size() => {
                follow(i0)
            }
            _ => false,
        },
        _ => false,
    }
}

/// The op before `at` in its block that writes `[addr, addr+size)`, when the
/// nearest write of any of those bytes writes exactly them.
fn def_before(data: &Funcdata, at: OpId, addr: &Address, size: int4) -> Option<OpId> {
    let mut cur = data.op_previous_op(at);
    while let Some(op) = cur {
        let o = data.obank().get(op)?;
        if let Some(v) = o.get_out().and_then(|v| data.vbank().get(v)) {
            if overlaps(v.get_addr(), v.get_size(), addr, size) {
                return (v.get_addr() == addr && v.get_size() == size).then_some(op);
            }
        }
        cur = data.op_previous_op(op);
    }
    None
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
