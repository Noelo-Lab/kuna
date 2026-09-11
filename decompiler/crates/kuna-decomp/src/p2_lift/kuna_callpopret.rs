//! (kuna) `kuna_callpopret` — a `call` whose callee pops the pushed return
//! address off the stack and `ret`s to its *caller's* caller (S2 flow
//! classification).  Sibling of
//! [`kuna_calltrampoline`](crate::kuna_calltrampoline), which decides the same
//! question for the flavour of the idiom that ends in a `jmp`.
//!
//! ## The structural gap
//!
//! `CALL` is a fall-through op, so the flow follower decodes the return address
//! as the next instruction.  The oldest position-independent idiom on x86 breaks
//! that by calling over its own inline data:
//!
//! ```text
//!   0x4f7036  CALL 0x4f708d          ; return address is 0x4f703b
//!   0x4f703b  .byte 0x90             ; control never arrives here
//!   0x4f703c  "kernel32.dll\0GetProcAddress\0..."
//!   ...
//!   0x4f708d  POP EAX                ; EAX = 0x4f703b, the pushed return address
//!   0x4f708e  INC EAX                ; step past the pad byte -> 0x4f703c
//!   0x4f708f  RET                    ; return to 0x4f7036's OWN caller
//! ```
//!
//! The callee never returns to `0x4f703b`: it throws that address away and
//! `ret`s through the word above it, so `sub_4f7036` is exactly
//! `char *sub_4f7036(void) { return "kernel32.dll"; }`.  Decoded as a
//! fall-through, the embedded DLL names lift as instructions instead — on the
//! round-12 RE-friction witness (a packed PE) `6b 65 72 6e` and the bytes after
//! it produce port-input operations and stores through registers that were never
//! set, and the string the caller exists to hand back is nowhere in the output.
//!
//! Section flags are not the discriminator: the whole packer stub lives in one
//! non-executable-flagged section, so a rule keyed on executability would refuse
//! the `call` as readily as the string.  The discriminator is the callee's stack
//! effect.
//!
//! ## The signal this module owns
//!
//! [`kuna_call_pops_return_address`] scans the callee's raw p-code — decoded out
//! of band by the caller, before the flow follower has committed to those bytes —
//! and fires only when all of the following hold:
//!
//!   * the stack pointer reaches **exactly** the value it had on entry plus one
//!     pointer word, i.e. the pushed return address is discarded and nothing else
//!     has been consumed off the stack at that moment;
//!   * the run ends in a `CPUI_RETURN` whose destination is the word the caller's
//!     own return address occupies — the value loaded from `entrySP + ptrsize`,
//!     with nothing added to it;
//!   * nothing in between is a call, a branch of any kind or a store, and no
//!     write to the stack pointer is unaccountable.
//!
//! The second clause is what separates this from an ordinary `ret`, and in
//! particular from the `__x86.get_pc_thunk` family: `mov ebx,[esp]; ret` reads the
//! return address without consuming it and returns through `entrySP`, so the
//! caller does get control back and the fall-through decode is right.  A callee
//! that pops twice returns through `entrySP + 2*ptrsize` and is declined too — it
//! eats a word the caller's frame owns, and modelling that is not this rule's job.
//!
//! On an architecture whose `call` does not push (the return address goes to a
//! link register) the stack-pointer displacement never reaches one pointer word
//! and the return's destination is a register, so the rule declines twice over.
//!
//! ## What the caller does with a match
//!
//! [`ArchFlowEnv`](crate::decompile_drive) rewrites the `CALL` to a `BRANCH` and
//! follows the callee's bytes into the caller, the same rewrite
//! `override_flow(BRANCH)` applies by hand.  The pushed return address, the pop
//! that reads it back and the `ret` that leaves are then all ordinary p-code in
//! one function, so the stack stays balanced and existing dataflow recovers the
//! popped constant — the caller prints as `return s_4f703c;` rather than losing
//! the pointer.

use kuna_base::marshal::ElementId;
use kuna_base::space::{spacetype, VarnodeStorage};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::kuna_calltrampoline::RawOp;

/// Marshaling element `<callpopret>` (kuna).  ElementIds live in the 4000+
/// range; 4163 was the next free id when this option was added.
pub const ELEM_CALLPOPRET: ElementId = ElementId::new("callpopret", 4163);

/// How many machine instructions of the callee the scan will decode before
/// giving up.  The idiom is `pop reg; <adjust>; ret`; the slack is for the flag
/// lowering of an `add` and for a second adjustment.
pub const KUNA_CALLPOPRET_MAX_INSTRS: usize = 6;

/// How many raw p-code ops the scan will read before giving up, whatever the
/// instruction count says.
pub const KUNA_CALLPOPRET_MAX_OPS: usize = 64;

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

/// What the scan knows a location to hold.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Held {
    /// `entrySP + delta`.
    StackPtr(i64),
    /// The pointer-sized word loaded from `entrySP + offset`, unmodified.
    Slot(i64),
}

/// Locations whose contents the scan can name.
struct Track {
    /// The stack-pointer register itself.
    sp: VnKey,
    known: Vec<(VnKey, Held)>,
}

impl Track {
    fn new(sp: VnKey) -> Self {
        Track { sp, known: vec![(sp, Held::StackPtr(0))] }
    }

    fn get(&self, k: &VnKey) -> Option<Held> {
        self.known.iter().find(|(key, _)| key == k).map(|(_, v)| *v)
    }

    fn get_vn(&self, vn: &VarnodeData) -> Option<Held> {
        self.get(&key_of(vn)?)
    }

    /// The displacement of `vn` from the entry stack pointer, if that is what it
    /// holds.
    fn sp_of(&self, vn: &VarnodeData) -> Option<i64> {
        match self.get_vn(vn)? {
            Held::StackPtr(d) => Some(d),
            Held::Slot(_) => None,
        }
    }

    /// Record what `out` now holds, or (with `None`) forget it.  Any location
    /// merely *overlapping* a tracked one is forgotten too: a partial write
    /// invalidates the whole value.
    fn set(&mut self, out: &VarnodeData, held: Option<Held>) {
        let Some(k) = key_of(out) else { return };
        self.known.retain(|(key, _)| !overlaps(key, &k));
        if let Some(h) = held {
            // Cap the map: the idiom needs two or three entries, and an unbounded
            // one would let a long decode grow it without bound.
            if self.known.len() < KUNA_CALLPOPRET_MAX_OPS {
                self.known.push((k, h));
            }
        }
    }

    /// The stack pointer's current displacement, or `None` once a write the scan
    /// could not account for has landed on it.
    fn sp_delta(&self) -> Option<i64> {
        match self.get(&self.sp)? {
            Held::StackPtr(d) => Some(d),
            Held::Slot(_) => None,
        }
    }
}

/// (kuna `callpopret`) Does this run of raw p-code discard the return address its
/// caller pushed and then `ret` through the word above it — returning to the
/// caller's own caller instead of to the call site?
///
/// `ops` are the callee's raw p-code ops in decode order, starting at its entry;
/// `sp` is the stack-pointer register location (`getStackSpace()
/// ->getSpacebaseFull(0)`).
///
/// A `true` verdict means control provably never reaches the return address, so
/// the `CALL` is a transfer and the bytes at the return address are not
/// instructions of the caller.
pub fn kuna_call_pops_return_address(ops: &[RawOp], sp: &VarnodeStorage) -> bool {
    if ops.is_empty() || ops.len() > KUNA_CALLPOPRET_MAX_OPS {
        return false;
    }
    let Some(spkey) = key_of_storage(sp) else { return false };
    let pushed = spkey.size as i64;
    let mut track = Track::new(spkey);
    let mut discarded = false;
    for op in ops {
        match op.code {
            OpCode::CPUI_RETURN => {
                // The destination must be the caller's own return address: the
                // untouched word one slot above the one the `call` pushed.
                let Some(dest) = op.ins.first() else { return false };
                return discarded && track.get_vn(dest) == Some(Held::Slot(pushed));
            }
            OpCode::CPUI_BRANCH
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            // A store can rewrite the very slot the `ret` is about to read, and
            // the scan does not model memory.
            | OpCode::CPUI_STORE => return false,
            OpCode::CPUI_LOAD => {
                let Some(out) = op.out.as_ref() else { continue };
                // in0 is the space id; in1 is the address being read.
                let held = op
                    .ins
                    .get(1)
                    .filter(|_| out.size as i64 == pushed)
                    .and_then(|a| track.sp_of(a))
                    .map(Held::Slot);
                track.set(out, held);
            }
            OpCode::CPUI_COPY => {
                let Some(out) = op.out.as_ref() else { continue };
                let held = op.ins.first().and_then(|v| track.get_vn(v));
                track.set(out, held);
            }
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                let Some(out) = op.out.as_ref() else { continue };
                let held = binary_displacement(&track, op).map(Held::StackPtr);
                track.set(out, held);
            }
            _ => {
                if let Some(out) = op.out.as_ref() {
                    track.set(out, None);
                }
            }
        }
        // A write the scan could not account for has landed on the stack pointer:
        // nothing after this point is measurable.
        let Some(delta) = track.sp_delta() else { return false };
        if delta == pushed {
            discarded = true;
        }
    }
    false
}

/// The displacement of an `INT_ADD` / `INT_SUB` whose operands are a tracked
/// stack-pointer alias and a literal, or `None` for any other shape.
fn binary_displacement(track: &Track, op: &RawOp) -> Option<i64> {
    let a = op.ins.first()?;
    let b = op.ins.get(1)?;
    if op.code == OpCode::CPUI_INT_SUB {
        // Only `tracked - constant` is a displacement; `constant - tracked`
        // negates the pointer and is not one.
        return Some(track.sp_of(a)?.wrapping_sub(const_of(b)?));
    }
    if let (Some(d), Some(k)) = (track.sp_of(a), const_of(b)) {
        return Some(d.wrapping_add(k));
    }
    if let (Some(k), Some(d)) = (const_of(a), track.sp_of(b)) {
        return Some(d.wrapping_add(k));
    }
    None
}

#[cfg(test)]
#[path = "kuna_callpopret/tests.rs"]
mod tests;
