//! Unit tests for the `overlapbranch` decision predicate.

use super::kuna_overlaps_pending_branch;

/// The round-2 crackme: `JNZ 0x8048838` at 0x8048835, fall-through 0x8048837
/// decoding as a 5-byte `CALL` that runs to 0x804883c and swallows the target.
#[test]
fn fires_on_the_anti_disassembly_overlap() {
    assert!(kuna_overlaps_pending_branch(true, 0x8048837, 5, Some((0x8048837, 0x8048838))));
}

#[test]
fn gate_off_never_fires() {
    assert!(!kuna_overlaps_pending_branch(false, 0x8048837, 5, Some((0x8048837, 0x8048838))));
}

#[test]
fn no_pending_branch_never_fires() {
    assert!(!kuna_overlaps_pending_branch(true, 0x8048837, 5, None));
}

/// The ordinary shapes that must stay untouched: a branch to its own
/// fall-through, and a branch that skips exactly one whole instruction.
#[test]
fn ordinary_forward_branches_do_not_fire() {
    // target == fall-through
    assert!(!kuna_overlaps_pending_branch(true, 0x1000, 5, Some((0x1000, 0x1000))));
    // target == end of the fall-through instruction (skip one instruction)
    assert!(!kuna_overlaps_pending_branch(true, 0x1000, 5, Some((0x1000, 0x1005))));
    // target well past the fall-through instruction
    assert!(!kuna_overlaps_pending_branch(true, 0x1000, 5, Some((0x1000, 0x1020))));
    // a backward branch
    assert!(!kuna_overlaps_pending_branch(true, 0x1000, 5, Some((0x1000, 0x0f00))));
}

/// The watch is only valid for the branch's own fall-through instruction.
#[test]
fn a_different_instruction_does_not_consume_the_watch() {
    assert!(!kuna_overlaps_pending_branch(true, 0x2000, 5, Some((0x8048837, 0x8048838))));
}

/// A one-byte instruction cannot contain another instruction's start.
#[test]
fn single_byte_instruction_never_fires() {
    assert!(!kuna_overlaps_pending_branch(true, 0x1000, 1, Some((0x1000, 0x1001))));
    assert!(!kuna_overlaps_pending_branch(true, 0x1000, 0, Some((0x1000, 0x1001))));
}

/// A wide instruction with the target one byte in and one byte short of the end.
#[test]
fn fires_anywhere_strictly_inside() {
    for t in 1..7u64 {
        assert!(
            kuna_overlaps_pending_branch(true, 0x1000, 7, Some((0x1000, 0x1000 + t))),
            "target +{t} inside a 7-byte instruction must fire"
        );
    }
    assert!(!kuna_overlaps_pending_branch(true, 0x1000, 7, Some((0x1000, 0x1007))));
}

// --- the reconvergence exclusion ------------------------------------------

use super::kuna_streams_reconverge;

/// glibc's conditional-LOCK idiom (`malloc_consolidate`, statically linked):
/// `JE 0x40fbcb` over `LOCK OR [RDI+4],1` at 0x40fbca (5 bytes) whose unprefixed
/// twin at 0x40fbcb (4 bytes) ends at the same address. Both streams are real.
#[test]
fn glibc_conditional_lock_prefix_reconverges() {
    assert!(kuna_streams_reconverge(0x40fbca, 5, 0x40fbcb, Some(4)));
}

/// The round-2 crackme does not reconverge: 0x8048837+5 = 0x804883c against
/// 0x8048838+3 = 0x804883b.
#[test]
fn the_anti_disassembly_overlap_does_not_reconverge() {
    assert!(!kuna_streams_reconverge(0x8048837, 5, 0x8048838, Some(3)));
}

/// A target that cannot be decoded answers "reconvergent" so the caller declines:
/// following it would gain nothing over the decode that did work.
#[test]
fn an_undecodable_target_declines() {
    assert!(kuna_streams_reconverge(0x8048837, 5, 0x8048838, None));
    assert!(kuna_streams_reconverge(0x8048837, 5, 0x8048838, Some(0)));
    assert!(kuna_streams_reconverge(0x8048837, 5, 0x8048838, Some(-1)));
}

/// A target instruction that runs PAST the fall-through's end is not the same
/// instruction under a prefix, so it does not decline.
#[test]
fn a_target_that_outruns_the_fallthru_does_not_reconverge() {
    assert!(!kuna_streams_reconverge(0x1000, 5, 0x1001, Some(9)));
}
