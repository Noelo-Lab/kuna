//! (kuna) `kuna_overlapbranch` — a conditional branch whose target lies *inside*
//! its own fall-through instruction (S2 flow classification).  The x86
//! anti-disassembly idiom: a junk lead byte placed at the fall-through so a
//! linear/recursive follower decodes one long bogus instruction that swallows
//! the branch target and desynchronises the whole stream.
//!
//! ## The structural gap
//!
//! kuna's flow follower ([`FlowInfo`](crate::flow)) pushes both successors of a
//! conditional branch and pops the fall-through first, so the fall-through
//! instruction is decoded before the branch target is ever looked at.  When the
//! two encodings overlap, the fall-through wins by arrival order and the target
//! is never decoded on its own boundary: `set_fallthru_bound` notices the clash
//! afterwards and only records it (`reinterpreted`), because by then the losing
//! stream and everything downstream of it are already built.
//!
//! On the round-2 crackme this produced, from
//!
//! ```text
//!   08048835  75 01              JNZ  0x8048838
//!   08048837  e8 8b 45 0c 03     CALL 0x0b10cdc7     ; junk `e8` lead byte
//! ```
//!
//! a call to an address outside the image (`sub_b10cdc7()`), stores through
//! uninitialised pointers and a bogus `dat_d98ddd00` global — instead of the
//! real stream that starts one byte later at 0x8048838 (`MOV EAX,[EBP+0xc];
//! ADD EAX,[EBP+0x8]; MOVSX EDX,[EAX]`) and carries the checker's dataflow.
//!
//! ## The one legitimate overlap, and why it is excluded first
//!
//! An overlap alone does NOT mean one decode is junk.  glibc's conditional-`LOCK`
//! idiom is *compiler-generated* and does exactly this — from `malloc_consolidate`
//! in a statically linked binary:
//!
//! ```text
//!   0040fbc8  74 01              JE  0x40fbcb        ; skip the LOCK prefix
//!   0040fbca  f0 83 4f 04 01     LOCK OR [RDI+4],1   ; ends at 0x40fbcf
//!   0040fbcb     83 4f 04 01     OR   [RDI+4],1      ; ends at 0x40fbcf
//! ```
//!
//! Both streams are real; they are the *same* instruction with the prefix taken or
//! skipped, and they **reconverge immediately** — the two decodes end at the same
//! address.  Truncating the fall-through there deletes the atomic store on a live
//! path.  The junk-lead-byte case does not reconverge: `0x8048837 + 5 = 0x804883c`
//! against `0x8048838 + 3 = 0x804883b`.  So the reconvergence test is the first
//! thing this module asks, it is architecture-neutral (no prefix-byte table), and
//! it costs one extra instruction decode on a trigger that never fires in ordinary
//! code — see [`kuna_streams_reconverge`].
//!
//! ## The ownership policy
//!
//! Once reconvergence is ruled out, two decodes claim the same bytes and at most
//! one of them can be real.  This module's answer is **the branch target wins**:
//!
//! * A branch target is *explicit* — the program encodes, in the branch itself,
//!   that execution resumes at that address on that edge.  A fall-through is
//!   *implicit*: it is only ever inferred from the previous instruction's
//!   length.  When the two disagree the encoded claim is the better evidence.
//! * Excluding the reconvergent case, two instruction starts that BOTH belong to
//!   one intended stream cannot sit at `next` and strictly inside `next`, so
//!   whenever the rule fires at least one of the two decodes is already wrong;
//!   keeping the explicitly encoded one cannot make the answer worse.
//!
//! and **the loser is truncated in place, never re-anchored**:
//!
//! * The losing instruction's freshly emitted ops are dropped and an artificial
//!   `RETURN` is planted at its own address, exactly as the bad-data decode
//!   policy already does.  The conditional stays a conditional and its
//!   fall-through edge stays in the CFG — the edge simply ends there.  The halt
//!   is marked `badinstruction`, not `noreturn`: a `noreturn` halt is folded away
//!   by `kuna_ifnoexit` into an empty `if (cond) { }`, which reads as "the
//!   fall-through does nothing and carries on" — the one rendering that is
//!   actually wrong about where control goes.
//! * The check runs on the instruction *currently being decoded*, against a
//!   target that is still pending, so nothing that other flow already depends on
//!   is deleted and no instruction boundary already committed to is moved.  That
//!   is what keeps this out of the "repair the graph afterwards" class of change.
//!
//! ## What this module owns
//!
//! [`kuna_overlaps_pending_branch`] is the *decision* only.  The truncation
//! (`delete_remaining_ops` + `artificial_halt` + `step = 1`) is driven by
//! `flow.rs` inside `process_instruction`, the same hook class as
//! [`kuna_funcboundflow`](crate::kuna_funcboundflow).  The gate
//! `glb->overlap_branch` (`option overlapbranch`) is resolved by the caller.

use kuna_base::marshal::ElementId;

/// Marshaling element `<overlapbranch>` (kuna).  ElementIds live in the 4000+
/// range.  Do not read the high-water mark by hand — `scripts.repipe.counters
/// --check` derives the next free id and fails on a duplicate.
pub const ELEM_OVERLAPBRANCH: ElementId = ElementId::new("overlapbranch", 4138);

/// (kuna) Does the instruction just decoded at the fall-through of a conditional
/// branch swallow that branch's own target?
///
/// `watch` is the `(fall-through address, branch target)` pair recorded when the
/// *previous* instruction was a conditional branch with a forward target;
/// `curaddr`/`step` describe the instruction that has just been decoded.  Fires
/// iff:
///
///   - `gate` is on (`glb->overlap_branch`);
///   - the instruction being decoded IS that fall-through (`curaddr == watch.0`);
///   - the branch target lies **strictly inside** it (`curaddr < target` and
///     `target < curaddr + step`).
///
/// The strictness at both ends is load-bearing.  `target == curaddr` is a branch
/// to its own fall-through and `target == curaddr + step` is a branch over one
/// instruction — both are ordinary compiler output and must not fire.  Only a
/// target that splits the fall-through encoding is pathological.
///
/// This is the CHEAP half of the decision and is not sufficient on its own: the
/// caller must also clear [`kuna_streams_reconverge`] before truncating anything.
pub fn kuna_overlaps_pending_branch(
    gate: bool,
    curaddr: u64,
    step: i32,
    watch: Option<(u64, u64)>,
) -> bool {
    // gate (opt-in / DIV default)
    if !gate {
        return false;
    }
    let (fallthru, target) = match watch {
        Some(w) => w,
        None => return false,
    };
    if step <= 1 {
        return false;
    }
    // the decoded instruction must be the recorded fall-through
    if curaddr != fallthru {
        return false;
    }
    // ... and the branch target must split its encoding
    target > curaddr && target < curaddr.wrapping_add(step as u64)
}

/// (kuna) Do the two competing decodes end at the same address?
///
/// `curaddr`/`step` are the fall-through instruction; `target`/`target_len` the
/// branch target's own decode (`target_len` is `None` when it could not be
/// decoded at all).  True means the branch target is the *same* instruction as
/// the fall-through with some leading bytes skipped — glibc's conditional-`LOCK`
/// idiom, where both streams are real and the fall-through must be kept.
///
/// A target that cannot be decoded also answers true: following it would gain
/// nothing, so there is no reason to discard the one decode that did work.
pub fn kuna_streams_reconverge(
    curaddr: u64,
    step: i32,
    target: u64,
    target_len: Option<i32>,
) -> bool {
    let len = match target_len {
        Some(l) if l > 0 => l,
        _ => return true,
    };
    target.wrapping_add(len as u64) == curaddr.wrapping_add(step as u64)
}

#[cfg(test)]
#[path = "kuna_overlapbranch/tests.rs"]
mod tests;
