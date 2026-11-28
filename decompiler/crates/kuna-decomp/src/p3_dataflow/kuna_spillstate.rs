//! (kuna) What a stack slot holds when stack pointers are spilled to it in
//! more than one state.
//!
//! `Heritage::discover_indexed_stack_pointers` follows a stack pointer through
//! a slot it is spilled to before the stack is in SSA form, so it cannot tell
//! which write a read of the slot sees: it walks every read with the state of
//! one write. A slot written with a plain frame address in one place and an
//! indexed one in another (an interpreter's operand stack, a spill slot the
//! compiler reuses) was walked as whichever the walk met first, and an indexed
//! LOAD behind it got no guard until the pass had renamed the ranges it reads.
//! Here every state a slot is written with is joined, to the lowest offset and
//! the union of the traversals, and the walk is repeated while a slot was
//! walked with less than its join.

use crate::kuna_fasthash::FxHashMap;
use kuna_base::types::{int4, uint4, uintb};

/// A slot: (offset, size).
type Slot = (uintb, int4);
/// A pointer state: (offset from the stack pointer, traversal flags).
type State = (uintb, uint4);

fn join(a: State, b: State) -> State {
    (if (b.0 as i64) < (a.0 as i64) { b.0 } else { a.0 }, a.1 | b.1)
}

/// The spill states of one discovery.
#[derive(Default)]
pub(crate) struct SpillStates {
    known: FxHashMap<Slot, State>,
    met: FxHashMap<Slot, State>,
    walked: FxHashMap<Slot, State>,
    rounds: u32,
}

/// The walks one discovery makes at most.
const MAX_ROUNDS: u32 = 8;

impl SpillStates {
    /// A pointer in `state` is written to `slot`.
    pub(crate) fn meet(&mut self, slot: Slot, state: State) {
        self.met.entry(slot).and_modify(|s| *s = join(*s, state)).or_insert(state);
    }

    /// The state to walk the reads of `slot` with, when the write that
    /// queued it holds `state`.
    pub(crate) fn walk(&mut self, slot: Slot, state: State) -> State {
        let state = self.known.get(&slot).map_or(state, |&k| join(k, state));
        self.walked.insert(slot, state);
        state
    }

    /// The walk is over: was a slot walked with less than what was written
    /// to it?  Then the next walk starts from the joins.
    pub(crate) fn repeat(&mut self) -> bool {
        self.rounds += 1;
        let mut stale = false;
        for (slot, used) in self.walked.drain() {
            let Some(&met) = self.met.get(&slot) else { continue };
            let all = join(used, met);
            if all != used {
                stale = true;
                self.known.insert(slot, all);
            }
        }
        self.met.clear();
        stale && self.rounds < MAX_ROUNDS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT: Slot = (0xffff_ffff_ffff_2060, 8);

    #[test]
    fn a_slot_written_in_one_state_is_walked_once() {
        let mut states = SpillStates::default();
        states.meet(SLOT, (0x10, 0));
        assert_eq!(states.walk(SLOT, (0x10, 0)), (0x10, 0));
        assert!(!states.repeat());
    }

    #[test]
    fn a_plain_and_an_indexed_pointer_join_to_the_indexed_one() {
        let mut states = SpillStates::default();
        states.meet(SLOT, (0xffff_ffff_ffff_ff58, 1));
        states.meet(SLOT, (0xffff_ffff_ffff_ff68, 0));
        assert_eq!(states.walk(SLOT, (0xffff_ffff_ffff_ff68, 0)), (0xffff_ffff_ffff_ff68, 0));
        assert!(states.repeat());
        states.meet(SLOT, (0xffff_ffff_ffff_ff68, 0));
        assert_eq!(states.walk(SLOT, (0xffff_ffff_ffff_ff68, 0)), (0xffff_ffff_ffff_ff58, 1));
        states.meet(SLOT, (0xffff_ffff_ffff_ff58, 1));
        assert!(!states.repeat());
    }

    #[test]
    fn the_lowest_offset_is_signed() {
        assert_eq!(join((0x8, 0), (0xffff_ffff_ffff_fff0, 0)), (0xffff_ffff_ffff_fff0, 0));
    }

    #[test]
    fn the_walks_are_bounded() {
        let mut states = SpillStates::default();
        let mut rounds = 0;
        loop {
            rounds += 1;
            states.meet(SLOT, (0x1000 - rounds as u64, 0));
            states.walk(SLOT, (0x1000, 0));
            if !states.repeat() {
                break;
            }
        }
        assert_eq!(rounds, MAX_ROUNDS);
    }
}
