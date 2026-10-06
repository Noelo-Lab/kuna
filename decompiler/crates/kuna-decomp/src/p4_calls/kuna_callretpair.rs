//! (kuna) Connect a two-register return at the CALL, whatever language produced
//! the callee.
//!
//! # The symptom
//!
//! A QuickJS interpreter built by plain `gcc -O2`. `JSValue` is
//! `{ JSValueUnion u; int64_t tag; }`, sixteen bytes, so the System V x86-64 ABI
//! returns it in `RAX:RDX` and the caller tests the tag register immediately:
//!
//! ```text
//! 0x9d9d8  CALL 0x875e0
//! 0x9d9dd  CMP  EDX,0x6          ; tag == JS_TAG_EXCEPTION
//! 0x9d9e6  MOV  R13,RAX          ; keep the payload
//! 0x9d9e9  MOV  R12,RDX
//! ```
//!
//! Ask kuna about the callee and it agrees there is a sixteen-byte return --
//! `undefined16 sub_875e0(int8,char *,int8,uint4)`. Ask it about the caller and
//! the same call has no output at all:
//!
//! ```text
//! int4 *v1;          // rax   <- declared, read five times, NEVER ASSIGNED
//! unsigned long v5;  // rdx   <- declared, read four times, NEVER ASSIGNED
//! sub_875e0(a0,a1,a2,1);
//! v4 = (int4)v5;
//! ```
//!
//! Every use of the returned value reads a variable with no definition anywhere
//! in the function, and the call that produced it looks like it was made for its
//! side effects.
//!
//! # What is missing
//!
//! Not the ABI decision, and not the recovery. The x86-64 cspec's
//! `<join_dual_class/>` output rule already describes this storage, and kuna
//! already puts two active output trials on the call. What drops them is the
//! consumer seam: `FuncCallSpecs::buildOutputFromTrials` (`fspec.cc:5777`)
//! handles one used output trial and upstream handles two, but kuna shipped the
//! multi-trial arm as a stub. The INDIRECT creations that stood for "the callee
//! wrote something here" survive untouched, which is what renders as a local the
//! function never assigns.
//!
//! [`crate::kuna_rustabi`] completed that arm, but gated it on the image being
//! rustc-produced. The completion is not Rust-specific and neither is the
//! evidence behind it: a sixteen-byte aggregate return is ordinary C on this
//! ABI, and `option rustabi auto` cannot see a GCC image at all. This option is
//! the same arm on the same classification with the language test dropped.
//!
//! # What the seam can prove
//!
//! [`crate::kuna_rustabi::classify_call_output_pair`] is the whole list, and it
//! is unchanged here: the model's output rule matched a justified,
//! non-overlapping register pair; the caller reads both halves out of the call;
//! and a bounded decode of the resolved callee did not refute it by proving the
//! payload register is never written. That last check is one-sided -- it can
//! refute a pair, never confirm one -- so forming the pair means *no
//! counter-example*, which is also the evidence upstream ships this arm on.
//!
//! # The gate
//!
//! `option callretpair on|off`, default **on**. `option rustabi auto|always`
//! still reaches the same arm, so a Rust image behaves as before whichever of
//! the two is set.
//!
//! # A caller that reads only part of the pair
//!
//! `unsigned hib(unsigned a) { return (unsigned char)(full(a) >> 32); }` is, on
//! 32-bit ARM, `bl full; uxtb r0,r1`: the caller reads `r1` and overwrites `r0`
//! unread, so the `r0` trial has no Varnode and only `r1` is active. No output
//! rule returns the second register of a pair without the first, so the model
//! uses nothing, and `r1` printed as a local nothing assigns beside a bare
//! `full(a0);`. i386 (`movzbl %dl,%eax` after the call, a one-byte `DL` trial
//! inside the `EDX:EAX` join entry) and a Cortex-M `uxtb r1,r1` next to a read
//! of `r0` are the same shape.
//!
//! [`build_partial_pair`] builds the call's output from the model instead of
//! from the bytes the caller names. A read trial the model left unused that
//! lies in a general-purpose register the model never returns a value in on
//! its own (`r1`, `EDX`, `$v1`, `RDX`, big-endian `r4`) names the pair that
//! register belongs to: the two-piece join entry holding it, or the
//! first-in-class register entry of its class beside it. The model is asked
//! whether it returns exactly those two full registers together, and in which
//! order, by deriving the output map over a probe holding just them. When it
//! does, the CALL gains that pair as its output and every read trial becomes a
//! `SUBPIECE` of it at its own byte offset. The printed shift names the half a
//! read takes, so the pair is formed only in the ABI's join order: a
//! big-endian pair waits for `option bejoin on`, since the first-register-low
//! join would print a read of the low word `r4` as the high word.
//!
//! The evidence is stricter than the two-trial arm's, because a read of the
//! second register alone is also what a value the callee leaves alone looks
//! like: gcc's `-fipa-ra` keeps a caller's pointer in `$v1` or `r1` across a
//! static callee it knows does not touch the register, also through the
//! `jalr $t9` a MIPS PIC call to a static function is. So the pair is formed
//! only when the bounded decode of the callee recorded a write to the register
//! the caller reads; an indirect call has no decoded callee and keeps the old
//! rendering. The decode is not the absence proof the two-trial arm vetoes
//! with, because on ARM and MIPS it never completes (a return's `setISAMode`,
//! a `jr ra`), so a positive write is the only callee fact it gives there. A
//! write is not a return value, though: a `void` helper's scratch register, an
//! `idiv`'s remainder in `EDX` beside an `int` result. Where the run has the
//! callee's own recovery (`decompile-all`'s callee-first pass files it; a
//! single-function, `--jobs`, narrowed or streamed run does not), that
//! recovery must return a value whose storage holds the register.
//! And the register must be read by the function itself
//! -- by an op other than a CALL, CALLIND or RETURN, looking through phis, the
//! PIECE return recovery joins a returned pair with and the no-op copies of a
//! return's mode switch -- because a later call's argument or the function's
//! own return trial is a read kuna has not yet decided is real (an x86-64
//! `RDX` a next call only appears to take as an argument is one).

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_class;
use crate::fspec::{FuncCallSpecs, ParamActive, ParamListStandard};
use crate::funcdata::Funcdata;

/// Is the language-agnostic call-output pair arm live for this function?
pub fn live(data: &Funcdata) -> bool {
    data.get_arch().call_ret_pair
}

/// Is the call-output pair arm live on the engine-side `Architecture`?
///
/// The seeding hook in [`crate::kuna_rustabi::seed_callee_return_writes`] runs
/// from the driver, where only the engine handle exists.
pub fn live_arch(arch: &crate::architecture::Architecture) -> bool {
    arch.call_ret_pair
}

/// How many phis, PIECEs and injected copies [`read_by_the_function`] follows.
const READ_DEPTH: u32 = 8;

/// Give the call the model's two-register output when the caller reads part
/// of it the model would not return alone (see the module header).
///
/// Runs in `buildOutputFromTrials` after the output map is derived, while the
/// unused trials are still present. `trialvn` holds each trial's INDIRECT
/// creation, by original slot. Returns `false`, changing nothing, unless the
/// pair is built.
pub fn build_partial_pair(fc: &mut FuncCallSpecs, data: &mut Funcdata, trialvn: &[Option<VarnodeId>]) -> bool {
    if (!live(data) && !crate::kuna_rustabi::live(data)) || !fc.proto().has_model() {
        return false;
    }
    let mut reads: Vec<(Address, int4, VarnodeId, bool)> = Vec::new();
    {
        let active = fc.get_active_output();
        for i in 0..active.get_num_trials() {
            let t = active.get_trial(i);
            if let Some(Some(vn)) = trialvn.get((t.get_slot() - 1) as usize) {
                reads.push((t.get_address().clone(), t.get_size(), *vn, t.is_used()));
            }
        }
    }
    if reads.iter().filter(|r| r.3).count() > 1 {
        return false;
    }
    let op = fc.get_op();
    let killed: Vec<(Address, int4, VarnodeId, bool)> = killed_reads(data, op)
        .into_iter()
        .filter(|(_, _, vn)| !trialvn.contains(&Some(*vn)))
        .map(|(addr, size, vn)| (addr, size, vn, false))
        .collect();
    let Some(list) = fc.proto().model().output_list() else { return false };
    let Some((first, second)) =
        reads.iter().chain(&killed).filter(|r| !r.3).find_map(|r| pair_of(list, &r.0, r.1))
    else {
        return false;
    };
    let manager = data.get_arch().manage.clone();
    let mut probe = ParamActive::new(false);
    for (addr, size) in [&first, &second] {
        probe.register_trial(addr, *size);
    }
    for i in 0..2 {
        probe.get_trial_mut(i).mark_active();
    }
    if fc.proto().derive_output_map(&mut probe, &manager).is_err()
        || !(0..2).all(|i| probe.get_trial(i).is_used())
    {
        return false;
    }
    let order = crate::kuna_bejoin::call_join_order(data, &probe);
    if order != probe.join_pair_order() {
        return false;
    }
    let piece = |i: int4| (probe.get_trial(i).get_address().clone(), probe.get_trial(i).get_size());
    let ((lo_addr, lo_size), (hi_addr, hi_size)) = (piece(order.0), piece(order.1));
    let offset = |addr: &Address, size: int4| {
        let lo = lo_addr.justified_contain(lo_size, addr, size, false);
        let hi = hi_addr.justified_contain(hi_size, addr, size, false);
        if lo >= 0 {
            Some(lo)
        } else {
            (hi >= 0).then_some(lo_size + hi)
        }
    };
    let Some(mut pieces) = reads
        .iter()
        .map(|(addr, size, vn, _)| offset(addr, *size).map(|off| (*vn, off)))
        .collect::<Option<Vec<(VarnodeId, int4)>>>()
    else {
        return false;
    };
    let killed: Vec<(Address, int4, VarnodeId, bool)> =
        killed.into_iter().filter(|(addr, size, _, _)| offset(addr, *size).is_some()).collect();
    pieces.extend(killed.iter().filter_map(|(addr, size, vn, _)| offset(addr, *size).map(|off| (*vn, off))));
    let orphans: Vec<&(Address, int4, VarnodeId, bool)> = reads.iter().chain(&killed).filter(|r| !r.3).collect();
    let read_regs: Vec<&(Address, int4)> = orphans
        .iter()
        .map(|(addr, size, _, _)| {
            if first.0.justified_contain(first.1, addr, *size, false) >= 0 { &first } else { &second }
        })
        .collect();
    let entry = fc.get_entry_address();
    let Some(callee) = data.kuna_callee_ret_writes(entry) else { return false };
    if !read_regs.iter().all(|reg| writes(callee, &reg.0, reg.1))
        || !callee_returns_cover(data, entry, &read_regs)
        || !orphans.iter().any(|r| read_by_the_function(data, r.2, READ_DEPTH))
    {
        return false;
    }
    let Some(call_addr) = data.obank().get(op).map(|o| o.get_addr().clone()) else { return false };
    let Some(joinaddr) =
        crate::kuna_rustabi::pair_join_address(data, &hi_addr, hi_size, &lo_addr, lo_size, &call_addr)
    else {
        return false;
    };
    let Ok(whole) = data.new_varnode_out(lo_size + hi_size, &joinaddr, op) else { return false };
    if let Some(v) = data.vbank_mut().get_mut(whole) {
        v.set_write_mask();
    }
    for (vn, off) in pieces {
        let indop = data.vbank().get(vn).and_then(|v| v.get_def());
        let sub = data.new_op(2, call_addr.clone());
        data.op_set_opcode_code(sub, OpCode::CPUI_SUBPIECE);
        let c = data.new_constant(4, off as u64);
        let _ = data.op_set_input(sub, whole, 0);
        let _ = data.op_set_input(sub, c, 1);
        let _ = data.op_set_output(sub, vn);
        data.op_insert_after(sub, op);
        if let Some(indop) = indop {
            data.op_destroy(indop);
        }
    }
    true
}

/// The read register values `callop` creates that are not output trials: the
/// INDIRECT creations just before it of a register the call kills, which
/// heritage plants without a trial when the range read is too narrow for any
/// output entry on its own (i386 `DL`, below the `EDX:EAX` entry's minimum).
fn killed_reads(data: &Funcdata, callop: OpId) -> Vec<(Address, int4, VarnodeId)> {
    let mut found = Vec::new();
    let mut cur = data.op_previous_op(callop);
    while let Some(io) = cur {
        let Some(o) = data.obank().get(io).filter(|o| o.code() == OpCode::CPUI_INDIRECT) else { break };
        let created = o.get_out().filter(|_| o.is_indirect_creation());
        if let Some((vn, v)) = created.and_then(|vn| data.vbank().get(vn).map(|v| (vn, v))) {
            let register = v.get_addr().get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR);
            if register && v.num_descend() > 0 {
                found.push((v.get_addr().clone(), v.get_size(), vn));
            }
        }
        cur = data.op_previous_op(io);
    }
    found
}

/// The two registers of the model's general-purpose pair that `[addr, size)`
/// lies in, when it lies in a register the model never returns a value in on
/// its own: the pieces of a two-piece join entry, or a register entry that is
/// not first in its class together with the first-in-class entry of that class.
fn pair_of(list: &ParamListStandard, addr: &Address, size: int4) -> Option<((Address, int4), (Address, int4))> {
    let entries = list.get_entry();
    let general = |e: &crate::fspec::ParamEntry| e.get_type() == type_class::TYPECLASS_GENERAL;
    let register = |e: &crate::fspec::ParamEntry| (Address::new(e.get_space().clone(), e.get_base()), e.get_size());
    if entries
        .iter()
        .any(|e| e.get_join_record().is_none() && e.is_first_in_class() && e.justified_contain(addr, size) >= 0)
    {
        return None;
    }
    for e in entries.iter().filter(|e| general(e)) {
        if let Some(jr) = e.get_join_record() {
            if jr.num_pieces() == 2 && e.justified_contain(addr, size) >= 0 {
                let piece = |i| {
                    let p = jr.get_piece(i);
                    (p.get_addr(), p.size as int4)
                };
                return Some((piece(1), piece(0)));
            }
        } else if e.justified_contain(addr, size) >= 0 {
            let first = entries.iter().find(|f| {
                general(f) && f.get_join_record().is_none() && f.is_first_in_class() && f.get_group() + 1 == e.get_group()
            })?;
            return Some((register(first), register(e)));
        }
    }
    None
}

/// Does the callee's own last recovery, when this run has one (`decompile-all`
/// files it callee-first, [`crate::kuna_voidret::record`]), return a value
/// whose storage holds every register in `regs`? A callee recovered `void`, or
/// returning only its first register (an `int` whose `idiv` leaves the
/// remainder in `EDX`), does not hand the caller what it reads there.
fn callee_returns_cover(data: &Funcdata, entry: &Address, regs: &[&(Address, int4)]) -> bool {
    let Some(sp) = entry.get_space() else { return false };
    let k = (sp.get_index(), entry.get_offset());
    match data.kuna_callee_returns(k) {
        None => true,
        Some(crate::kuna_voidret::Returns::Void) => false,
        Some(_) => data.kuna_callee_return_storage(k).is_some_and(|(addr, size)| {
            let pieces: Vec<(Address, int4)> = if addr.is_join() {
                match data.get_arch().manage.find_join(addr.get_offset()) {
                    Ok(jr) => (0..jr.num_pieces()).map(|i| (jr.get_piece(i).get_addr(), jr.get_piece(i).size as int4)).collect(),
                    Err(_) => return false,
                }
            } else {
                vec![(addr.clone(), *size)]
            };
            regs.iter().all(|reg| pieces.iter().any(|(a, s)| a.justified_contain(*s, &reg.0, reg.1, false) >= 0))
        }),
    }
}

/// Did the decoded callee body write any byte of `[addr, addr+size)` on the
/// paths the bounded decode covered?
fn writes(w: &crate::kuna_rustabi::CalleeReturnWrites, addr: &Address, size: int4) -> bool {
    let Some(sp) = addr.get_space() else { return false };
    let (start, end) = (addr.get_offset(), addr.get_offset() + size as u64);
    w.written_ranges().iter().any(|&(idx, off, sz)| idx == sp.get_index() && off < end && start < off + sz as u64)
}

/// Does an op of the function itself read `vn`: anything but a CALL, CALLIND
/// or RETURN, looking through phis, INDIRECTs, the PIECE return recovery
/// joins a returned pair with, and the no-op copies a return's mode switch
/// injects?
fn read_by_the_function(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    v.descend_iter().any(|op| {
        let Some(o) = data.obank().get(op) else { return false };
        match o.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN => false,
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT | OpCode::CPUI_PIECE => {
                depth > 0 && o.get_out().is_some_and(|out| out != vn && read_by_the_function(data, out, depth - 1))
            }
            OpCode::CPUI_COPY if crate::kuna_passthrough::is_injected_noop(data, op) => {
                depth > 0 && o.get_out().is_some_and(|out| read_by_the_function(data, out, depth - 1))
            }
            _ => true,
        }
    })
}

#[cfg(test)]
#[path = "kuna_callretpair/tests.rs"]
mod tests;
