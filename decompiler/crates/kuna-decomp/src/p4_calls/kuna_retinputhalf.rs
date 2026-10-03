//! (kuna) Keep a returned register half that carries a function **input
//! parameter**, instead of discarding it as leftover.
//!
//! # The symptom
//!
//! Two functions that differ only in what the second returned half holds:
//!
//! ```c
//! typedef struct { unsigned long a, b; } P;
//! P wide(unsigned long s, unsigned long x){ P p; p.a=s+1; p.b=x;     return p; }
//! P w2  (unsigned long s, unsigned long x){ P p; p.a=s+1; p.b=x*3+7; return p; }
//! ```
//!
//! `gcc -O1`, x86-64, no DWARF, recovered:
//!
//! ```text
//! wide  ->  unsigned long wide(long a0)          // RDX half dropped, and `x` VANISHED
//! w2    ->  undefined16 w2(long a0,long a1)      // correct
//! ```
//!
//! `wide` compiles to `mov %rsi,%rdx; lea 0x1(%rdi),%rax; ret` — the high half of
//! the returned pair is a plain copy of the second argument. Dropping it does not
//! merely lose the return half: the parameter it came from then has no remaining
//! reader, so it disappears from the recovered signature too. A whole argument is
//! gone from the prototype of a two-argument function.
//!
//! # Why it happens
//!
//! [`crate::kuna_returnuncomputed`] exists to kill a genuine phantom: a returned
//! register that merely holds a value the function never computed — the epilogue's
//! callee-saved *restore*, where the register is a copy of a frame slot the
//! function only ever reads, or a callee's clobber at a no-return call. Its walk
//! chases move-only operations (copies, phis, indirects, piece/subpiece) back to a
//! terminal and calls an **unwritten** Varnode uncomputed.
//!
//! A formal input parameter is unwritten by definition. So the rule cannot tell
//! `RDX = COPY(<the frame slot the caller left something in>)` — leftover — from
//! `RDX = COPY(<RSI, the second argument>)` — a real value the function was handed
//! and is handing back.
//!
//! # The discriminator
//!
//! Storage, asked twice.
//!
//! **Is it parameter storage?** The prototype model already knows. A passthrough
//! terminal sits at a location the model characterizes as input-parameter storage
//! (`FuncProto::possibleInputParam`) -- for x86-64 SysV `RDI`/`RSI`/`RDX`/`RCX`/
//! `R8`/`R9`/`XMM0-7` and the stack region *above* the return address. The
//! callee-saved restore terminal is a **local frame slot**, stack storage *below*
//! the return address, which no input `ParamEntry` covers; and a clobber is an
//! INDIRECT creation, which the walk already rejects before reaching a terminal.
//!
//! **Did the function put it there?** Parameter storage alone is not enough,
//! because on most ABIs some argument register is also a return register. Compare
//! the terminal's address with the storage the half occupies in the returned
//! value -- its register of a pair, or its bytes of the one return register:
//!
//! * `RDX = COPY(RSI)` -- different addresses. The function executed an
//!   instruction to move the argument into the return register. Real.
//! * `RDX` reaching the RETURN as the unwritten `RDX` -- same address. The
//!   function never touched the register; the caller's value is passing straight
//!   through. Leftover, exactly what the sibling rule exists to drop. (Witness:
//!   libselinux `sub_1a330`, whose cached early return sets `RAX` from a global
//!   and leaves `RDX` alone. Without the placement test that phantom `RDX` beats
//!   the real `RAX` half and the function grows three invented parameters.)
//!
//! # Shapes this newly accepts
//!
//! A returned register half whose move-only chain ends at an unwritten
//! function-input Varnode that is in input-parameter storage **and at a different
//! address than the half it reaches**, or at the same address after the function
//! moved it out and back (below). Either way the function executed an
//! instruction to move an argument into the return register.
//!
//! Everything the sibling rule was built to kill is untouched: the restore
//! terminal fails `possible_input_param` (a local frame slot is not parameter
//! storage), the clobber terminal is an INDIRECT creation rejected earlier in the
//! walk, and an untouched return register is still leftover however much it looks
//! like an argument.
//!
//! # The rule that was tried and rejected
//!
//! An earlier version also rescued the pair when **every** half was an untouched
//! incoming argument, on the theory that `double f(double x){ return x; }` on ARM
//! returns its argument in the registers it arrived in. It recovered three
//! betaflight soft-float helpers whose halves are exactly that -- and it also
//! resurrected the GH-6990 SPARC symptom, because a *void* `main` that touches
//! nothing at all leaves `o0:o1` passing through and SPARC passes arguments in
//! those same registers (`tests/stages/gh6990-returnpair.xml`). Nothing local to
//! the pair separates "returns its argument unchanged" from "never touched the
//! return registers", so the rescue is not taken and those three functions keep
//! today's answer.
//!
//! # An argument carried out of its register and back
//!
//! The same-address test has one blind spot. A function that keeps an argument in
//! a callee-saved register across a call and then moves it back to return it
//! (`mov r4,r1; bl ext; mov r1,r4`) reaches the RETURN with the caller's own `r1`
//! once copy propagation has collapsed the two moves, and reads exactly like an
//! untouched register. The function executed both moves, so the half is placed.
//!
//! The moves exist only while return recovery runs, so that is where
//! [`note_moved_back_returns`] looks, recording each return register whose value
//! at some RETURN reaches its own input through another register. The pair
//! repair then skips the placement test for that register at every RETURN.
//!
//! A move made by an instruction that also writes the stack pointer does not
//! count. SPARC's `save` and `restore` copy every `%o` register to its `%i`
//! register and back, so a SPARC function that never touches `%o1` has this
//! shape on every return; both instructions move the stack pointer, as does the
//! `pop` of a pushed register. Nor does an incidental copy: Xtensa's `call8`
//! swaps the argument registers out and back around the call. Nor does a move
//! followed, on its way to the RETURN, by a call or a CALLOTHER: an inline system
//! call (`svc`, `ecall`, `sc`) reads its arguments from registers the p-code does
//! not show, so `r1 = b; svc 0` in an `int` function is the call's argument, not
//! a returned half.
//!
//! Moving a register back proves only that the function returns it. When the
//! moved-back register is the high register of the pair and the low one reaches
//! that RETURN as the function's own untouched argument, the pair is the
//! function's argument handed back whole (an early `return a;` in a 64-bit
//! shift that copied the high word aside), and the repair keeps the pair. The
//! high register is never returned alone on the strength of the move.
//!
//! Gated by `option retinputhalf on|off`.

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::fspec::ParamActive;
use crate::funcdata::Funcdata;

/// Is `vn` an unwritten Varnode that is a **formal input parameter** of this
/// function, rather than leftover the function never computed?
///
/// Five conditions, all necessary: `option retinputhalf` is on, the Varnode is
/// unwritten (a written one is not this shape and is classified by its defining
/// op), heritage flagged it a function input (a free Varnode is not a parameter),
/// it is not a register the function only ever pushed (`option retpushedhalf`;
/// see [`crate::kuna_retpushedhalf`]), and it sits in storage the prototype model
/// characterizes as input-parameter storage. The last is what separates a passed
/// argument from a local frame slot the function only reads.
///
/// The caller adds the **placement** test — see `computes_from` in
/// [`crate::kuna_returnuncomputed`].
///
/// Runs inside `ActionOutputPrototype`, which is scheduled *before*
/// `ActionInputPrototype` — the proto's parameter list is not fixated yet, so the
/// question has to be put to the model (`possible_input_param` falls through to
/// it when no locked params exist), exactly as input recovery itself does.
pub fn is_input_parameter(data: &Funcdata, vn: VarnodeId) -> bool {
    if !data.get_arch().ret_input_half {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.get_def().is_some() || !v.is_input() {
        return false;
    }
    let (addr, size) = (v.get_addr().clone(), v.get_size());
    if data.kuna_pushed_registers().is_push_only(&addr, size) {
        return false;
    }
    data.get_func_proto().possible_input_param(&addr, size)
}

/// How many copies, phis and indirects [`moved_back`] follows from one RETURN
/// half before answering no.
const MOVE_BUDGET: u32 = 64;

/// Does the instruction at `pc` write the stack pointer?
pub(crate) fn writes_stack_pointer(data: &Funcdata, pc: &Address) -> bool {
    let Some(stackspc) = data.get_arch().manage().get_stack_space().cloned() else { return false };
    let Ok(sp) = stackspc.get_spacebase(0) else { return false };
    let Some(spspace) = sp.space.as_ref() else { return false };
    data.obank().iter_at(pc).any(|(_, op)| {
        data.obank()
            .get(op)
            .and_then(|o| o.get_out())
            .and_then(|out| data.vbank().get(out))
            .is_some_and(|v| {
                let a = v.get_addr();
                a.get_space().is_some_and(|s| s.get_index() == spspace.get_index())
                    && a.get_offset() < sp.offset + sp.size as u64
                    && sp.offset < a.get_offset() + v.get_size() as u64
            })
    })
}

/// The source of a register move, before the rules have reduced it to a COPY:
/// PowerPC's `mr` is `or rA,rS,rS`, and MIPS, SPARC and RISC-V spell a move as
/// an `or` or `add` with zero.
fn move_source(data: &Funcdata, op: &crate::op::PcodeOp) -> Option<VarnodeId> {
    let zero = |v: VarnodeId| data.vbank().get(v).is_some_and(|x| x.is_constant() && x.get_offset() == 0);
    match op.code() {
        OpCode::CPUI_COPY => op.get_in(0),
        OpCode::CPUI_INT_OR if op.num_input() == 2 && op.get_in(0) == op.get_in(1) => op.get_in(0),
        OpCode::CPUI_INT_OR | OpCode::CPUI_INT_ADD if op.num_input() == 2 => {
            let (a, b) = (op.get_in(0)?, op.get_in(1)?);
            if zero(b) {
                Some(a)
            } else if zero(a) {
                Some(b)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Can an op that reads registers its p-code does not show -- a CALLOTHER such
/// as `svc`, or a call -- run after `from` and before `to`?
pub(crate) fn call_between(data: &Funcdata, from: OpId, to: OpId) -> bool {
    let parent = |op: OpId| data.obank().get(op).and_then(|o| o.get_parent());
    let (Some(fb), Some(tb)) = (parent(from), parent(to)) else { return false };
    let graph = data.bblocks_ref();
    let reach = |start: BlockId, forward: bool| {
        let mut seen: std::collections::HashSet<BlockId> = std::collections::HashSet::new();
        let mut work = vec![start];
        while let Some(b) = work.pop() {
            let blk = graph.block(b);
            let n = if forward { blk.size_out() } else { blk.size_in() };
            for i in 0..n {
                let next = if forward { blk.get_out(i) } else { blk.get_in(i) };
                if seen.insert(next) {
                    work.push(next);
                }
            }
        }
        seen
    };
    let is_call = |op: &OpId| {
        data.obank()
            .get(*op)
            .is_some_and(|o| matches!(o.code(), OpCode::CPUI_CALLOTHER | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
    };
    let after = reach(fb, true);
    let before = reach(tb, false);
    let to_at = data.bb_ops(tb).iter().position(|&o| o == to).unwrap_or(usize::MAX);
    let reaches_to = |b: BlockId, at: usize| (b == tb && at < to_at) || before.contains(&b);
    let from_ops = data.bb_ops(fb);
    let from_at = from_ops.iter().position(|&o| o == from).unwrap_or(usize::MAX);
    if from_ops.iter().enumerate().any(|(i, op)| i > from_at && is_call(op) && reaches_to(fb, i)) {
        return true;
    }
    after
        .iter()
        .any(|&b| data.bb_ops(b).iter().enumerate().any(|(i, op)| is_call(op) && reaches_to(b, i)))
}

/// Is `vn`, read by the RETURN `ret` as the half stored at `addr`/`size`, the
/// function's own input at that storage moved out to another register and back?
///
/// Walks moves, phis and non-creating indirects back to the unwritten input at
/// exactly `addr`/`size`, and answers yes when the path passes through another
/// register of `addr`'s space. A move to a different storage made by an
/// instruction that also writes the stack pointer ends the path, as does the
/// move back when a call or a CALLOTHER can run between it and `ret`, and an
/// incidental copy (Xtensa's register-window swap around a `call8`) is walked
/// through without counting as a move.
pub fn moved_back(data: &Funcdata, vn: VarnodeId, addr: &Address, size: int4, ret: OpId) -> bool {
    let Some(space) = addr.get_space().map(|s| s.get_index()) else { return false };
    let mut seen: std::collections::HashSet<(VarnodeId, bool)> = std::collections::HashSet::new();
    let mut work: Vec<(VarnodeId, bool)> = vec![(vn, false)];
    let mut budget = MOVE_BUDGET;
    while let Some((cur, left)) = work.pop() {
        if !seen.insert((cur, left)) {
            continue;
        }
        if budget == 0 {
            return false;
        }
        budget -= 1;
        let Some(v) = data.vbank().get(cur) else { continue };
        let Some(def) = v.get_def() else {
            if left && v.is_input() && v.get_addr() == addr && v.get_size() == size {
                return true;
            }
            continue;
        };
        let Some(op) = data.obank().get(def) else { continue };
        match op.code() {
            OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => work.extend(op.get_in(0).map(|i| (i, left))),
            OpCode::CPUI_MULTIEQUAL => work.extend((0..op.num_input()).filter_map(|i| op.get_in(i)).map(|i| (i, left))),
            _ => {
                let Some(src) = move_source(data, op) else { continue };
                if op.is_incidental_copy() {
                    work.push((src, left));
                    continue;
                }
                let Some(s) = data.vbank().get(src) else { continue };
                let (sa, same) = (s.get_addr(), s.get_addr() == v.get_addr() && s.get_size() == v.get_size());
                if !same && writes_stack_pointer(data, op.get_addr()) {
                    continue;
                }
                let elsewhere = sa.get_space().is_some_and(|sp| sp.get_index() == space) && sa != addr;
                if elsewhere && !left && call_between(data, def, ret) {
                    continue;
                }
                work.push((src, left || elsewhere));
            }
        }
    }
    false
}

/// Record, before return recovery rewrites the RETURNs, every used output trial
/// whose value at some RETURN was [`moved_back`] into its register.
///
/// The record is per register, not per RETURN: a function returns in one
/// storage, so once one path moves the argument back to return it, the same
/// register left untouched on another path is that argument passing through, and
/// every RETURN keeps the half. `placed` is the first register of a pair
/// [`crate::kuna_retcallhalf::accept`] kept for the argument it holds in place,
/// which the record holds as well.
pub fn note_moved_back_returns(
    data: &mut Funcdata,
    active: &ParamActive,
    returns: &[OpId],
    placed: Option<(Address, int4)>,
) {
    if !data.get_arch().ret_input_half {
        return;
    }
    let mut found: Vec<(Address, int4)> = placed.into_iter().collect();
    for i in 0..active.get_num_trials() {
        let t = active.get_trial(i);
        if !t.is_used() {
            continue;
        }
        let moved = returns.iter().any(|&ret| {
            data.obank()
                .get(ret)
                .filter(|o| !o.is_dead() && o.get_halt_type() == 0)
                .and_then(|o| o.get_in(t.get_slot()))
                .is_some_and(|vn| moved_back(data, vn, t.get_address(), t.get_size(), ret))
        });
        if moved && !found.iter().any(|(a, s)| a == t.get_address() && *s == t.get_size()) {
            found.push((t.get_address().clone(), t.get_size()));
        }
    }
    data.kuna_set_moved_back_returns(found);
}

/// Was the return register at `addr`/`size` recorded by
/// [`note_moved_back_returns`]?
pub fn is_moved_back(data: &Funcdata, addr: &Address, size: int4) -> bool {
    data.kuna_moved_back_returns().iter().any(|(a, s)| a == addr && *s == size)
}

#[cfg(test)]
#[path = "kuna_retinputhalf/tests.rs"]
mod tests;
