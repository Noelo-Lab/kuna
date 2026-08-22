//! (kuna `typedepth`) The DWARF type mapper's recursion guard — a faithful port
//! of Ghidra's `DWARFDataTypeImporter.trackRecursion`, replacing the fixed hop
//! budget that truncated ordinary declarations to `void`.
//!
//! ## What the budget was actually protecting against
//!
//! [`super::build_datatype`] walks a DWARF type DIE chain by following
//! `DW_AT_type`. Nothing in the DWARF format forbids that chain from closing on
//! itself (a `DW_TAG_pointer_type` whose `DW_AT_type` is its own offset, a
//! `typedef`/`const` pair pointing at each other, a truncated or hand-forged
//! `.debug_info`), and upstream hits the same hazard on real C++ input where a
//! struct's fields are populated. Unguarded, the mapper recurses forever.
//!
//! Upstream guards it with a **per-DIE-offset re-entry counter**:
//!
//! ```java
//! private boolean trackRecursion(long id, int delta) {
//!     Integer count = recursionTrackingOffsetToLoopCount.getOrDefault(id, 0);
//!     count = count + delta;
//!     switch (count) { case 3: Msg.error(...); return false; }
//!     recursionTrackingOffsetToLoopCount.put(id, count);
//!     return true;
//! }
//! ```
//!
//! — a DIE may be re-entered twice, and the third entry is refused. That fires
//! **only** on a cycle: an acyclic chain visits each offset once.
//!
//! ## What kuna had instead
//!
//! The port reduced it to `MAX_TYPE_DEPTH = 3` hops counted over the whole walk,
//! transparent qualifiers included, which conflates "the same DIE again" (a
//! cycle) with "a deep but finite chain" (ordinary C). `const char **` is four
//! DIEs, `const size_t *` is four, `char *const authors[]` is four — all of them
//! came out `void`. The budget could not be raised into safety either: with no
//! cycle detection under it, ANY cap is both too low for real code and unable to
//! prove termination for a self-referential chain.
//!
//! ## The guard here
//!
//! [`TypeWalk::Cycle`] is upstream's counter. Termination: every recursive step
//! is bracketed by [`TypeWalk::enter`]/[`TypeWalk::leave`], an offset already on
//! the path twice is refused, and the number of distinct offsets in a
//! compilation unit is finite — so no walk can be infinite. [`MAX_NESTING`] is a
//! second, blunter bound that keeps a *long* (not cyclic) forged chain from
//! exhausting the native stack, which a Java port does not have to worry about
//! and a Rust one does.
//!
//! [`TypeWalk::Depth`] is the pre-fix budget, kept so `--option typedepth off`
//! reproduces the old mapping exactly.

use std::collections::{BTreeMap, BTreeSet};

/// Re-entry count at which a DIE offset is refused — upstream's `case 3`.
const MAX_REENTRY: u32 = 3;

/// Absolute nesting bound for the cycle-guarded walk: a native-stack backstop for
/// a forged `.debug_info` whose type chain is long but never repeats an offset
/// (the cycle counter alone would let it recurse once per DIE in the unit). No
/// real declaration comes close — the deepest in the C corpus measured here is 4.
const MAX_NESTING: u32 = 64;

/// The pre-fix hop budget (`MAX_TYPE_DEPTH`), retained for the gate-off arm.
const MAX_TYPE_DEPTH: u32 = 3;

/// The recursion guard threaded through [`super::build_datatype`].
///
/// [`Guard`] is the guard proper; `active` is the (kuna `dwarfstructs`) set of
/// aggregate names whose field population is on the current path, so a
/// self-referential member resolves to the interned shell instead of re-entering
/// the populate — see [`super::kuna_dwarfstructs`].
pub(super) struct TypeWalk {
    guard: Guard,
    active: BTreeSet<String>,
}

/// The recursion-guard arms proper.
pub(super) enum Guard {
    /// `--option typedepth off`: the flat hop budget, every hop counted.
    Depth { depth: u32 },
    /// Default: upstream's per-offset re-entry counter plus a stack backstop.
    Cycle { counts: BTreeMap<usize, u32>, nesting: u32 },
}

impl TypeWalk {
    /// A walk configured from the process gate
    /// ([`kuna_decomp::kuna_typedepth::typedepth_enabled`]).
    pub(super) fn new() -> Self {
        Self::with_gate(kuna_decomp::kuna_typedepth::typedepth_enabled())
    }

    /// A walk with the gate supplied explicitly (the unit tests, which must not
    /// depend on process environment).
    pub(super) fn with_gate(full_depth: bool) -> Self {
        let guard = if full_depth {
            Guard::Cycle { counts: BTreeMap::new(), nesting: 0 }
        } else {
            Guard::Depth { depth: 0 }
        };
        TypeWalk { guard, active: BTreeSet::new() }
    }

    /// (kuna `dwarfstructs`) Claim `name` for field population on this path.
    /// `false` => the aggregate is already being populated by an outer frame, so
    /// the caller must hand back the interned shell rather than recurse.
    pub(super) fn begin_aggregate(&mut self, name: &str) -> bool {
        self.active.insert(name.to_string())
    }

    /// (kuna `dwarfstructs`) Release a name claimed by [`Self::begin_aggregate`].
    pub(super) fn end_aggregate(&mut self, name: &str) {
        self.active.remove(name);
    }

    /// Begin resolving the DIE at `off`; `false` => refuse (cycle, or a bound
    /// hit), and the caller yields `void` for this piece of the type.
    pub(super) fn enter(&mut self, off: usize) -> bool {
        match &mut self.guard {
            Guard::Depth { depth } => {
                if *depth >= MAX_TYPE_DEPTH {
                    return false;
                }
                *depth += 1;
                true
            }
            Guard::Cycle { counts, nesting } => {
                if *nesting >= MAX_NESTING {
                    return false;
                }
                let seen = counts.entry(off).or_insert(0);
                if *seen + 1 >= MAX_REENTRY {
                    return false;
                }
                *seen += 1;
                *nesting += 1;
                true
            }
        }
    }

    /// Finish the DIE at `off` (pairs with an [`Self::enter`] that returned true).
    pub(super) fn leave(&mut self, off: usize) {
        match &mut self.guard {
            Guard::Depth { depth } => *depth = depth.saturating_sub(1),
            Guard::Cycle { counts, nesting } => {
                if let Some(seen) = counts.get_mut(&off) {
                    *seen = seen.saturating_sub(1);
                }
                *nesting = nesting.saturating_sub(1);
            }
        }
    }

    /// Whether the transparent `typedef`/`const`/`volatile`/`restrict` hops are
    /// collapsed for EVERY caller ([`super::strip_qualifiers`]), not just the C++
    /// one. Under the cycle guard they no longer need collapsing to survive a
    /// budget — but collapsing is what carries an anonymous aggregate's typedef
    /// name onto it, and full-depth resolution reaches many more of those (a
    /// local `mbstate_t` would otherwise intern as the shared `anon_struct`).
    pub(super) fn collapse_qualifiers(&self) -> bool {
        matches!(self.guard, Guard::Cycle { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The cycle guard admits an acyclic chain of any realistic length and
    /// refuses the third entry of a repeated offset — upstream's `case 3`.
    #[test]
    fn cycle_guard_admits_chains_and_refuses_cycles() {
        let mut w = TypeWalk::with_gate(true);
        // 32 distinct offsets: every one admitted (the budget would stop at 3).
        for off in 0..32usize {
            assert!(w.enter(off), "distinct offset {off} refused");
        }
        for off in (0..32usize).rev() {
            w.leave(off);
        }
        // The same offset: two entries admitted, the third refused.
        assert!(w.enter(7));
        assert!(w.enter(7));
        assert!(!w.enter(7), "third entry of the same DIE must be refused");
        w.leave(7);
        w.leave(7);
        // ...and the refusal is path-local: after unwinding it is admissible again.
        assert!(w.enter(7));
        w.leave(7);
    }

    /// The nesting backstop bounds a long acyclic chain (the forged-DWARF case
    /// the cycle counter alone cannot bound below the DIE count).
    #[test]
    fn cycle_guard_bounds_nesting() {
        let mut w = TypeWalk::with_gate(true);
        for off in 0..MAX_NESTING as usize {
            assert!(w.enter(off));
        }
        assert!(!w.enter(9999), "nesting bound must refuse beyond MAX_NESTING");
    }

    /// (kuna `dwarfstructs`) The aggregate-in-progress claim is exact and
    /// path-local: the same name cannot be claimed twice, and releasing it makes
    /// it claimable again (a struct reached a second time on a SIBLING branch
    /// must still be populatable).
    #[test]
    fn aggregate_claim_is_path_local() {
        let mut w = TypeWalk::with_gate(true);
        assert!(w.begin_aggregate("node"));
        assert!(!w.begin_aggregate("node"), "a re-entrant claim must be refused");
        assert!(w.begin_aggregate("other"), "a different name is unaffected");
        w.end_aggregate("node");
        assert!(w.begin_aggregate("node"), "released names are claimable again");
    }

    /// The gate-off arm is the pre-fix budget: three hops, whatever they are.
    #[test]
    fn depth_guard_reproduces_the_budget() {
        let mut w = TypeWalk::with_gate(false);
        assert!(w.enter(1));
        assert!(w.enter(2));
        assert!(w.enter(3));
        assert!(!w.enter(4), "the pre-fix budget is three hops");
        w.leave(3);
        assert!(w.enter(4), "unwinding one hop frees the budget again");
        assert!(!w.collapse_qualifiers(), "the off arm keeps the C++-only collapse");
    }
}
