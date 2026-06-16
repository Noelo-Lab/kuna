# w10-arrays-addtree
verdict: REJECT
verifier: opus-4.8 (independent verifier subagent)
date: 2026-06-16 (round 2 re-confirmation; round 1 verdict unchanged)
gate: cd rust && cargo test -p kuna-decomp --lib -> ok (1736 passed; 0 failed) — but PASSES UNCHANGED FROM BASE

## Round 2 re-confirmation

Re-ran the full check independently. State is byte-identical to round 1 and to
`rust-port`: branch SHA == `rust-port` SHA (`7514d11`), `git diff
rust-port...rport/w10-arrays-addtree` (ALL paths) = 0 bytes, worktree clean, no stashes,
reflog has only `@{0}: branch: Created from rust-port` (never advanced for round 2). No
porter work was added between rounds. Gate re-run: 1736 lib passed / 0 failed (== base).
Reserved files (funcdata_merge/merge/variable/dynamic/database/varmap/fspec/printc/
architecture): all 0 diff bytes. The multi-dim/array arms named for this item
(`has_matching_sub_type`+`array_hint`, `nearest_arrayed_component_backward/forward`,
`calc_subtype`, `span_add_tree`, `multsum`/`nonmultsum`+`wrapping_add`) exist on the
BASE `addtreestate.rs` (from already-verified pointer-flow/spacebase-typing/rule-pack
items), not on this branch; no special-casing literals there. Verdict stands: REJECT
(item not delivered on this branch).

## Summary: empty branch, no work delivered

The branch `rport/w10-arrays-addtree` is **byte-for-byte identical to `rust-port`**.

```
$ git rev-parse rport/w10-arrays-addtree rust-port
7514d11f123e1f86a4b8688a87efaf9b02dc21f4
7514d11f123e1f86a4b8688a87efaf9b02dc21f4         # same SHA
$ git merge-base rust-port rport/w10-arrays-addtree
7514d11...                                        # == both heads
$ git log --oneline rust-port..rport/w10-arrays-addtree
(empty)
$ git reflog rport/w10-arrays-addtree
7514d11 rport/w10-arrays-addtree@{0}: branch: Created from rust-port
```

`git diff rust-port...rport/w10-arrays-addtree -- rust/` is **empty**. The worktree
`/tmp/kuna-arraysaddtree` is clean (no uncommitted/untracked Rust work, no stashes).
The branch was created from `rust-port` and **zero commits / zero changes were added on
top**. There is no diff to review.

The declared item for this branch — faithful Rust transcription of the AddTreeState
multi-dim / intermediate-pointer arms (`AddTreeState::apply`/`calcSubtype`/`spanAddTree`)
and the array-stride helpers, plus new 2D/3D array passes vs oracle B5 — **was not
implemented on this branch**. Nothing was delivered:

- No porter passes (the suite count is identical to base; no new green tests).
- No `arrays-addtree`-tagged tests anywhere in `rust/` (grep clean).
- No verifier-required >=3 adversarial tests (none can be written against nonexistent
  delivered code; the protocol's REJECT requirement of a failing test / divergence trace
  is moot — there is no candidate code under review).

## Why this is a REJECT, not an ACCEPT(-WITH-LOSSES)

A verifier ACCEPT or ACCEPT_WITH_LOSSES certifies that *delivered* code is a faithful,
non-special-cased port. An empty branch delivers nothing to certify. A faithful PARTIAL
worthy of ACCEPT_WITH_LOSSES would still require a committed IR substrate + a losses
entry; here the commit set is empty, so even the partial path does not apply.

## Note: the named functionality ALREADY EXISTS on the base (landed under other items)

The AddTreeState multi-dimensional / array-stride machinery this branch was named for is
**already present on `rust-port`**, having been ported by earlier, already-verified items
— not by this branch:

- `rust/crates/kuna-decomp/src/addtreestate.rs` (788 lines) was introduced by
  `70f6235 rport/w10-pointer-flow` and last extended by
  `7b145f8 rport/w10-spacebase-typing` (`git log --diff-filter=A` / `git log -- <file>`).
- It already contains the multi-dim arms: `has_matching_sub_type(off, array_hint)`
  (addtreestate.rs:279), `base_nearest_backward`/`base_nearest_forward`
  (:255/:266 -> `nearest_arrayed_component_backward/forward`), `calc_subtype` (:471),
  `span_add_tree` (:439), `build_multiples` (:576). These mirror the C++
  `AddTreeState::calcSubtype`/`spanAddTree`/`hasMatchingSubType` arm-for-arm.
- The `kuna_arraystride.cc/.hh` array-stride helpers are checklist item
  `w5-kuna-rule-pack` (status `verified`, blob shas
  `db1b7c26…`/`273b572b…`), NOT this branch.

There is no `w10-arrays-addtree` entry in `docs/rust-port/checklist.json` at all. This
branch appears to be a stray/redundant branch whose scope already landed.

## Verifier focus checklist (recorded; all moot under empty diff)

- (1) FAITHFULNESS of multi-dim/intermediate-ptr arms: cannot certify — no diff. The
  pre-existing base implementation (from pointer-flow/spacebase-typing) is outside this
  item's scope and already verified under `w10-pointer-flow.md` /
  `w10-spacebase-typing.md`.
- (2) NO SPECIAL-CASING: grep of the existing `addtreestate.rs` for function-name /
  address / type-name / stride-constant hardcoding is clean — the only literals are
  structural `self.size == 0` div-by-zero guards and `% self.size` multiple tests
  (addtreestate.rs:336,392,473,578,586), faithful to C++. But this is BASE code, not this
  branch's work.
- (3) NEW PASSES REAL: none claimed; suite count unchanged from base (1736 lib tests,
  same set). No 2D/3D-array-vs-B5 pass to inspect.
- (4) NO REGRESSION: trivially none — the tree is identical to `rust-port`. No reserved
  file touched (no file touched at all). Switch datatests / boolless / readstruct /
  condconst_conn / whole-suite >=327 / C++ oracle 675/675 / B0: all unchanged because
  nothing changed.

## Hunt list (verification.md §"mandatory hunt list")

All entries N/A — no diff to hunt. Recorded for protocol completeness:
signedness / integer-widths / wrapping / comparator-totality / iteration-order /
off-by-one-do-while / erase-while-iterating / exception->Result-partial-state:
**no ported code on this branch to assess.**

## Mechanical pass

- `git diff rust-port...rport/w10-arrays-addtree -- rust/` -> empty.
- `cargo test -p kuna-decomp --lib` -> ok, 1736 passed (== base; not attributable to this
  branch).
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/bare-`as`/`arrays_addtree` on the diff
  -> N/A (no diff).
- clippy: nothing new to lint.

## Adversarial tests

None committed: there is no delivered code under review to target, and no candidate
implementation to drive a divergence trace against oracle B5. The verifier cannot
manufacture a port. (Protocol requires the REJECT to carry a failing test or a divergence
trace; the concrete "divergence" here is structural — the branch contains zero of the
item's required changes, evidenced by the identical SHA above.)

## Work order (back to todo)

1. Either implement the item: commit the AddTreeState multi-dim/intermediate-ptr arms +
   array-stride helper wiring + new 2D/3D-array passes as a real diff on this branch, with
   the porter's green passes and the verifier's >=3 adversarial tests; OR
2. Close the item as already-satisfied: the named substrate already landed under
   `w10-pointer-flow` and `w10-spacebase-typing` (verified) and `w5-kuna-rule-pack`
   (verified). If anything remains (e.g. a printc array-index render arm or a dtype
   array-resolve), record it as `seams_remaining` and route it to a NEW, scoped item —
   do NOT edit the reserved files (printc.rs / dtype.rs etc.).

No prior parity regressed (nothing changed); this REJECT is solely "item not delivered on
this branch."
