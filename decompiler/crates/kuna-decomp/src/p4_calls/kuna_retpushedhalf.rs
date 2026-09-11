//! (kuna) A register the function only ever **pushed** is not a value the
//! function placed in a return register.
//!
//! # The symptom
//!
//! A four-argument XOR decryptor returning the buffer it allocated:
//!
//! ```text
//! undefined16 sub_10e27(long a0,int a1,long a2,int a3,unsigned long a4)
//! {
//!   ...
//!   v1._8_8_ = a4;
//!   return v1._0_16_;
//! }
//! ```
//!
//! There is no fifth argument and no 128-bit return. The prologue pushes `R8`
//! to realign the stack and the epilogue pops that slot into `RDX`:
//!
//! ```text
//! 10e3d:  push %r8
//!   ...
//! 10e73:  pop  %rdx
//! 10e7a:  ret
//! ```
//!
//! # Why it happens
//!
//! `RDX` at the RETURN traces back to `R8` at entry, so [`crate::
//! kuna_retinputhalf`] sees an unwritten Varnode in input-parameter storage at a
//! different address than the half it reaches, calls it a value the function
//! *placed* there, and keeps the half. Keeping the half is what gives `R8` a
//! reader, which is what makes `R8` a parameter — and `R8` being a parameter is
//! the whole of the evidence that the half is a returned argument. The predicate
//! is satisfied by an accident of stack maintenance.
//!
//! By the time the repair runs the accident is invisible: copy propagation has
//! collapsed the store and the load, and `RDX = COPY(R8)` at the RETURN is
//! byte-for-byte the shape a deliberate `mov %r8,%rdx` leaves behind. The
//! evidence only exists while the instructions are being lifted.
//!
//! # The rule
//!
//! A register is **push-only** when some stack-adjusting instruction stores it to
//! memory and no instruction in the function ever writes it. Its value came from
//! the caller, was saved once, and the function never produced a value of its
//! own for it — so whatever later reads that slot is restoring the stack, not
//! delivering a result, and the register is not a placement source.
//!
//! Recorded during the flow build ([`observe_instruction`], one pass over the
//! ops an instruction just emitted) and consulted by `kuna_retinputhalf`'s
//! terminal test. A register that is pushed and popped back into *itself* is
//! written by the pop, so the ordinary callee-saved save/restore never qualifies;
//! neither does a callee-saved register the function loads a value into before
//! returning it.
//!
//! # What it deliberately does not reach
//!
//! Nothing without a stack-adjusting store: `mov %r8,%rdx` — a genuine returned
//! fifth argument — and the untouched `RAX:RDX` of a real 128-bit return are
//! decided exactly as before. The residual ambiguity is a function that pushes an
//! argument register purely to preserve it across a call and pops it into a
//! *different* register that it then returns; nothing local separates that from
//! the alignment idiom, and this rule reads it as maintenance.
//!
//! Gated by `option retpushedhalf on|off`.

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// A register location: space index, offset, size.
type RegLoc = (int4, u64, i32);

/// What the flow build saw about registers that are only ever saved.
///
/// Two flat vectors rather than sets: a function has a handful of pushes and a
/// few hundred register writes, and the only query is a linear overlap test made
/// at most twice per function.
#[derive(Debug, Clone, Default)]
pub struct PushedRegisters {
    /// Registers stored to memory by an instruction that also adjusts the stack
    /// pointer (the push).
    saved: Vec<RegLoc>,
    /// Every register any lifted op in the function writes.
    written: Vec<RegLoc>,
}

/// Do two register locations overlap?
fn overlaps(a: &RegLoc, b: &RegLoc) -> bool {
    a.0 == b.0 && a.1 < b.1 + b.2 as u64 && b.1 < a.1 + a.2 as u64
}

impl PushedRegisters {
    /// Is `addr`/`size` a register the function only ever pushed?
    pub fn is_push_only(&self, addr: &Address, size: i32) -> bool {
        let Some(sp) = addr.get_space() else { return false };
        let probe: RegLoc = (sp.get_index(), addr.get_offset(), size);
        self.saved.iter().any(|s| overlaps(s, &probe))
            && !self.written.iter().any(|w| overlaps(w, &probe))
    }
}

/// Is `addr` in the processor's `register` space?
fn is_register(addr: &Address) -> bool {
    addr.get_space().map(|s| s.get_name() == "register").unwrap_or(false)
}

/// The register a stored value came from, following the temporaries the same
/// instruction used to stage it.
///
/// An x86-64 `push %r8` is not `STORE(ram, RSP, R8)` but `t = COPY(R8)`,
/// `RSP = RSP - 8`, `STORE(ram, RSP, t)` — SLEIGH reads the operand into a
/// temporary before the stack pointer moves. Only ops belonging to this same
/// instruction are searched, so the walk cannot leave the instruction.
fn stored_register(data: &Funcdata, ops: &[OpId], vn: VarnodeId, depth: u32) -> Option<RegLoc> {
    if let Some(loc) = reg_loc(data, vn) {
        return Some(loc);
    }
    if depth >= 4 {
        return None;
    }
    // Before heritage every use is its own free Varnode, so the temporary is
    // matched by storage, not by identity.
    let want = vn_loc(data, vn)?;
    let src = ops.iter().find_map(|&o| {
        let op = data.obank().get(o)?;
        if op.code() != OpCode::CPUI_COPY {
            return None;
        }
        if op.get_out().and_then(|out| vn_loc(data, out)) != Some(want) {
            return None;
        }
        op.get_in(0)
    })?;
    stored_register(data, ops, src, depth + 1)
}

/// The storage of any Varnode: space index, offset, size.
fn vn_loc(data: &Funcdata, vn: VarnodeId) -> Option<RegLoc> {
    let v = data.vbank().get(vn)?;
    let addr = v.get_addr();
    Some((addr.get_space()?.get_index(), addr.get_offset(), v.get_size()))
}

/// The location of a Varnode, if it is a register.
fn reg_loc(data: &Funcdata, vn: VarnodeId) -> Option<RegLoc> {
    let v = data.vbank().get(vn)?;
    let addr = v.get_addr();
    if !is_register(addr) {
        return None;
    }
    Some((addr.get_space()?.get_index(), addr.get_offset(), v.get_size()))
}

/// Record what the ops just emitted for one machine instruction say about
/// saved-only registers (see the module docs).
///
/// Called from the flow build with `ops` the ops that instruction produced, in
/// emission order. A no-op unless `option retpushedhalf` is on.
pub fn observe_instruction(data: &mut Funcdata, ops: &[OpId]) {
    if !data.get_arch().ret_pushed_half {
        return;
    }
    let Some(stackspc) = data.get_arch().manage().get_stack_space().cloned() else { return };
    let Ok(spreg) = stackspc.get_spacebase(0) else { return };
    let Some(spspace) = spreg.space.as_ref() else { return };
    let sploc: RegLoc = (spspace.get_index(), spreg.offset, spreg.size as i32);

    let mut adjusts_sp = false;
    let mut written: Vec<RegLoc> = Vec::new();
    for &op in ops {
        let Some(o) = data.obank().get(op) else { continue };
        let Some(out) = o.get_out() else { continue };
        let Some(loc) = reg_loc(data, out) else { continue };
        if overlaps(&loc, &sploc) {
            adjusts_sp = true;
        }
        written.push(loc);
    }

    let mut saved: Vec<RegLoc> = Vec::new();
    if adjusts_sp {
        for &op in ops {
            let Some(o) = data.obank().get(op) else { continue };
            if o.code() != OpCode::CPUI_STORE || o.num_input() < 3 {
                continue;
            }
            let Some(val) = o.get_in(2) else { continue };
            if let Some(loc) = stored_register(data, ops, val, 0) {
                if !overlaps(&loc, &sploc) {
                    saved.push(loc);
                }
            }
        }
    }

    let regs = data.kuna_pushed_registers_mut();
    regs.written.append(&mut written);
    regs.saved.append(&mut saved);
}

#[cfg(test)]
#[path = "kuna_retpushedhalf/tests.rs"]
mod tests;
