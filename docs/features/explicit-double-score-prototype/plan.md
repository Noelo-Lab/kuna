# Implementation plan

1. Trace each XMM0 heritage cell at the exact dataset cookie call and classify
   it against both the checker ABI output and the caller's declared output.
2. Add a separate exact-cookie predicate for `ContainsUnjustified` cells. Gate
   its use in heritage on an output-locked caller whose declared return contains
   the same cell; leave generic preservation and justified storage unchanged.
3. Extend predicate tests with the refined high half, upper scratch storage,
   known body-write, and explicit-effect controls.
4. Extend the existing double-score stage with a four-byte-refined output and
   pin locked caller, unlocked caller, and option-off behavior.
5. Vendor a deterministic PE64 refined twin, promote the tightened acceptance,
   and verify that the division assignment is the value returned rather than
   merely deleting the undefined `CONCAT44` rendering.
6. Re-run the exact dataset witness, option ablations, parity suites, workspace
   tests, specification checks, catalog freshness, and the speed budget.
