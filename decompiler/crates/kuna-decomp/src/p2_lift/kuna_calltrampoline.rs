//! (kuna) `kuna_calltrampoline` — a `call` whose callee discards the pushed
//! return address and jumps back into the instruction stream (S2 flow
//! classification).  Sibling of [`kuna_tailcalljump`](crate::kuna_tailcalljump)
//! and [`kuna_tailcallframe`](crate::kuna_tailcallframe), which decide the
//! opposite question about a `jmp`.
//!
//! ## The structural gap
//!
//! `CALL` is a fall-through op: the flow follower decodes the return address as
//! the next instruction.  An obfuscator breaks that assumption with a three-line
//! fragment the caller reaches by `call`:
//!
//! ```text
//!   0x40d1ae  CALL 0x44ac6b            ; return address is 0x40d1b3
//!   ...
//!   0x44ac6b  LEA ESP,[ESP + 0x4]      ; throw the return address away
//!   0x44ac6f  MOV EDI,dword ptr [0x40f068]
//!   0x44ac75  JMP 0x40d1b4             ; resume in the caller, one byte past
//! ```
//!
//! Control never arrives at `0x40d1b3`, so the byte there is free to be junk —
//! and on the round-9 RE-friction witness (a Beria-protected PE) it is: `a3 83
//! c7 64 83` decodes as `MOV [0x8364c783],EAX`, and the emitted C carries a
//! store to a global that does not exist.  Worse, the decode is now one byte out
//! of phase with the real instruction stream, so everything the fragment was
//! protecting (`ADD EDI,0x64` / `CMP dword ptr [EDI + 0x70]`, the resolver
//! dispatch it guards, and an `int3` anti-debug test) is mis-recovered too.
//!
//! The re-entry point is the fragment's own `jmp` target, which is *usually* one
//! byte past the return address but need not be: the same table in the witness
//! holds `0x44ac4d -> 0x40d19e`, which is exactly the return address.  So the
//! rule is written against the `jmp` target, never against `ra + 1`.
//!
//! ## The signal this module owns
//!
//! [`kuna_trampoline_branch_target`] scans the callee's raw p-code — decoded out
//! of band by the caller, before the flow follower commits to anything — and
//! fires only when all of the following hold:
//!
//!   * the stack pointer reaches **exactly** the value it had on entry plus one
//!     pointer word, i.e. the pushed return address is discarded and nothing
//!     else has been consumed off the stack at that moment;
//!   * the fragment then ends in a **direct unconditional branch**;
//!   * nothing in between is a call, a conditional branch, an indirect branch or
//!     a return, and no write to the stack pointer is unaccountable.
//!
//! The stack-pointer value is tracked symbolically as `entrySP + <constant>`
//! through `COPY` / `INT_ADD` / `INT_SUB` chains, because `lea esp,[esp+4]`
//! lowers through a temporary rather than writing `ESP` directly the way `add
//! esp,4` does.  Any other write to a location overlapping the stack pointer
//! ends the scan.
//!
//! The net delta is deliberately *not* what is measured: the witness's first
//! fragment is `lea esp,[esp+4]; sub esp,0xcfc; jmp 0x40d12c`, whose net effect
//! is a large allocation.  What identifies the shape is that the run *passes
//! through* `entrySP + ptrsize` — the moment the return address stops existing.
//!
//! ## What this rule cannot distinguish, and who guards it
//!
//! `add esp,4; jmp printf` has the same shape and is an ordinary tail-call
//! thunk; flowing through it would decode `printf` into the caller and abandon
//! the rest of the caller.  Nothing in the callee's bytes separates the two, so
//! the caller ([`ArchFlowEnv`](crate::decompile_drive)) requires the branch
//! target to be an address the symbol table does **not** know as a function
//! entry.  A fragment that re-enters mid-instruction-stream satisfies that by
//! construction; a thunk to a discovered callee does not, and stays a `CALL` for
//! [`kuna_tailcalljump`](crate::kuna_tailcalljump) to claim at its `jmp`.

use kuna_base::marshal::ElementId;
use kuna_base::space::{spacetype, VarnodeStorage};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

/// Marshaling element `<calltrampoline>` (kuna).  ElementIds live in the 4000+
/// range; 4158 is the next free above `stackcommentbase`'s 4157.
pub const ELEM_CALLTRAMPOLINE: ElementId = ElementId::new("calltrampoline", 4158);

/// How many machine instructions of the callee the scan will decode before
/// giving up.  A fragment of this family is three instructions; the slack is for
/// a flag-setting `add` lowered across several p-code ops per instruction.
pub const KUNA_TRAMPOLINE_MAX_INSTRS: usize = 6;

/// How many raw p-code ops the scan will read before giving up, whatever the
/// instruction count says.  Bounds the cost of a probe at an address that
/// happens to decode into something enormous.
pub const KUNA_TRAMPOLINE_MAX_OPS: usize = 64;

/// One raw p-code op as the scan sees it — the [`PcodeEmit::dump`](
/// kuna_sleigh::translate::PcodeEmit::dump) payload minus the instruction
/// address, which the scan does not use.
#[derive(Clone, Debug)]
pub struct RawOp {
    /// The op-code.
    pub code: OpCode,
    /// The output location, if the op has one.
    pub out: Option<VarnodeData>,
    /// The input locations, in order.
    pub ins: Vec<VarnodeData>,
}

/// A varnode location reduced to the triple the scan compares on.
#[derive(Clone, Copy, PartialEq, Eq)]
struct VnKey {
    space: i32,
    offset: u64,
    size: u32,
}

fn key_of(vn: &VarnodeData) -> Option<VnKey> {
    let space = vn.space.as_ref()?;
    Some(VnKey { space: space.get_index(), offset: vn.offset, size: vn.size })
}

fn key_of_storage(sp: &VarnodeStorage) -> Option<VnKey> {
    let space = sp.space.as_ref()?;
    Some(VnKey { space: space.get_index(), offset: sp.offset, size: sp.size })
}

/// The literal behind `vn`, sign-extended from its own width, or `None` when it
/// is not a constant.
fn const_of(vn: &VarnodeData) -> Option<i64> {
    let space = vn.space.as_ref()?;
    if space.get_type() != spacetype::IPTR_CONSTANT {
        return None;
    }
    Some(sign_extend(vn.offset, vn.size))
}

/// Sign-extend the low `size` bytes of `val` into an `i64`.
fn sign_extend(val: u64, size: u32) -> i64 {
    if size == 0 || size >= 8 {
        return val as i64;
    }
    let bits = size * 8;
    let m = 1u64 << (bits - 1);
    (((val & ((1u64 << bits) - 1)) ^ m).wrapping_sub(m)) as i64
}

/// Do two locations share any byte?
fn overlaps(a: &VnKey, b: &VnKey) -> bool {
    a.space == b.space
        && a.offset < b.offset.saturating_add(b.size as u64)
        && b.offset < a.offset.saturating_add(a.size as u64)
}

/// Locations currently known to hold `entrySP + delta`.
struct SpTrack {
    /// The stack-pointer register itself.
    sp: VnKey,
    /// `(location, displacement from the entry stack pointer)`.
    known: Vec<(VnKey, i64)>,
}

impl SpTrack {
    fn new(sp: VnKey) -> Self {
        SpTrack { sp, known: vec![(sp, 0)] }
    }

    fn get(&self, k: &VnKey) -> Option<i64> {
        self.known.iter().find(|(key, _)| key == k).map(|(_, d)| *d)
    }

    fn get_vn(&self, vn: &VarnodeData) -> Option<i64> {
        self.get(&key_of(vn)?)
    }

    /// Record `out` as `entrySP + delta`, or (with `None`) forget it.  Any
    /// location merely *overlapping* an alias of the stack pointer is forgotten
    /// too: a partial write invalidates the whole tracked value.
    fn set(&mut self, out: &VarnodeData, delta: Option<i64>) {
        let Some(k) = key_of(out) else { return };
        self.known.retain(|(key, _)| !overlaps(key, &k));
        if let Some(d) = delta {
            // Cap the map: a fragment this shape needs two or three entries, and
            // an unbounded one would let a long decode grow it without bound.
            if self.known.len() < KUNA_TRAMPOLINE_MAX_OPS {
                self.known.push((k, d));
            }
        }
    }

    /// The stack pointer's current displacement, or `None` once a write the scan
    /// could not account for has landed on it.
    fn sp_delta(&self) -> Option<i64> {
        self.get(&self.sp)
    }
}

/// (kuna `calltrampoline`) Does this run of raw p-code discard the return
/// address its caller pushed and then branch directly somewhere else?
///
/// `ops` are the callee's raw p-code ops in decode order, starting at its entry;
/// `sp` is the stack-pointer register location (`getStackSpace()
/// ->getSpacebaseFull(0)`).  Returns the branch's destination location on a
/// match, so the caller can apply its own policy to the target.
///
/// The scan requires the stack pointer to *pass through* `entrySP + sp.size` —
/// the instant the pushed return address ceases to exist — and the run to end in
/// a direct `CPUI_BRANCH`.  On an architecture whose call does not push (the
/// return address goes to a link register), the displacement never reaches one
/// pointer word and the rule declines, which is the correct answer: there is no
/// return address on the stack to discard.
pub fn kuna_trampoline_branch_target(
    ops: &[RawOp],
    sp: &VarnodeStorage,
) -> Option<VarnodeData> {
    if ops.is_empty() || ops.len() > KUNA_TRAMPOLINE_MAX_OPS {
        return None;
    }
    let spkey = key_of_storage(sp)?;
    let pushed = spkey.size as i64;
    let mut track = SpTrack::new(spkey);
    let mut discarded = false;
    for op in ops {
        match op.code {
            OpCode::CPUI_BRANCH => {
                let dest = op.ins.first()?;
                // A `constant` destination is a relative branch within the
                // instruction's own p-code, not a machine-level transfer.
                if const_of(dest).is_some() {
                    return None;
                }
                return discarded.then(|| dest.clone());
            }
            OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_RETURN => return None,
            OpCode::CPUI_COPY => {
                let Some(out) = op.out.as_ref() else { continue };
                let d = op.ins.first().and_then(|v| track.get_vn(v));
                track.set(out, d);
            }
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                let Some(out) = op.out.as_ref() else { continue };
                let d = binary_displacement(&track, op);
                track.set(out, d);
            }
            _ => {
                if let Some(out) = op.out.as_ref() {
                    track.set(out, None);
                }
            }
        }
        // A write the scan could not account for has landed on the stack
        // pointer: nothing after this point is measurable.
        let delta = track.sp_delta()?;
        if delta == pushed {
            discarded = true;
        }
    }
    None
}

/// The displacement of an `INT_ADD` / `INT_SUB` whose operands are a tracked
/// stack-pointer alias and a literal, or `None` for any other shape.
fn binary_displacement(track: &SpTrack, op: &RawOp) -> Option<i64> {
    let a = op.ins.first()?;
    let b = op.ins.get(1)?;
    if op.code == OpCode::CPUI_INT_SUB {
        // Only `tracked - constant` is a displacement; `constant - tracked`
        // negates the pointer and is not one.
        return Some(track.get_vn(a)?.wrapping_sub(const_of(b)?));
    }
    if let (Some(d), Some(k)) = (track.get_vn(a), const_of(b)) {
        return Some(d.wrapping_add(k));
    }
    if let (Some(k), Some(d)) = (const_of(a), track.get_vn(b)) {
        return Some(d.wrapping_add(k));
    }
    None
}

#[cfg(test)]
#[path = "kuna_calltrampoline/tests.rs"]
mod tests;
