# [PROPOSAL] switch-case-shared-case-node — recover the b2sum getopt jump table (loop-carried guard)

**Status:** draft proposal — needs human go/no-go before an implementation worker is spent.
**Scope:** LARGE (decider verdict, recorded in `record.json`).
**Recommended option name:** `switchunrolledguard`

## The problem
angr recovers the `getopt_long` dispatch in `main` of
`b2sum-digest_shared_switch_nodes.o` as a real `switch (v7)` with 11 cases (including the
"shared case node" fallthrough `case 132 -> case 98`). kuna recovers **no switch at all**: it
prints `/* WARNING: Treating indirect jump as call */` and renders the `BRANCHIND` as an
indirect call, exploding the 11 cases into goto spaghetti (switches 1→0, cases 11→0, gotos
11→18). Full evidence in `analysis.md` / `angr-vs-kuna.txt`.

## Root cause (confirmed, not a loader bug)
The table base `%rbp` (`lea .rodata`) is set *before* the getopt `while` loop while the
`BRANCHIND` is *inside* it, so the jump's block has `size_in() > 1` (a loop-header
`MULTIEQUAL`). `JumpBasic::analyze_guards` (`s2_lift/jumptable.rs:~2019`) therefore dispatches
to `check_unrolled_guard` (`~:2201`), which is a bare `SEAM(structuring)` **stub**. The
index guard (`sub $0x62; cmp $0x22; ja default`) is never reconstructed across the phi →
`find_smallest_normal` leaves the range unbounded → `recover_model_basic` returns `Ok(false)`
→ no model → `truncate_indirect_jump` downgrades to `CALLIND`. Relocations are fine
(`relocobjects` default-on; `.rodata` reads resolve), so the table memory is readable.

**This is the same gap as `unrolledguard` / proposal PR #50** (MSVC optimized-`memcpy` jump
tables). This getopt case is a *second, GCC/coreutils witness* for the missing
`JumpBasic::checkUnrolledGuard` machinery.

## angr reference
angr's `CFGFast` indirect-jump resolution reconstructs the guard/bound across the loop phi
during jump-table resolution; the structurer then folds the shared case node (132→98
fallthrough). The Ghidra-equivalent kuna must port is `JumpBasic::checkUnrolledGuard` +
`BlockBasic::findMultiequal` + `liftVerifyUnroll` + `noInterveningStatement` +
`Funcdata::pushBranch` (`decompiler/cpp/jumptable.cc`, `block.cc`, `funcdata*.cc`).

## Implementation plan (multi-step — why it is LARGE)
1. **Port `BlockBasic::findMultiequal` / `noInterveningStatement`** — walk back from the
   `BRANCHIND` index varnode through the loop-header `MULTIEQUAL` to the dominating definition,
   verifying no intervening side-effecting statement. (`block.rs` + helpers.)
2. **Port `liftVerifyUnroll`** — confirm the guard CBRANCH (`cmp $0x22; ja`) dominates the
   loop body on the path to the `BRANCHIND` and lifts to a clean `index <= 0x22` bound.
3. **Port `Funcdata::pushBranch`** — push the reconstructed guard back so
   `find_smallest_normal` can size the table (35 entries). (`funcdata*.rs`.)
4. **Fill the `check_unrolled_guard` stub** in `s2_lift/jumptable.rs` to drive 1–3, gated by a
   new architecture flag `switchunrolledguard` (default-off while developing).
5. **Verify** the GCC relative-offset model (`load(base+idx*4) signext + base`) recovers and
   the case bodies, including the 132→98 shared node, structure as a `switch`.

Anchor files (ported-core): `s2_lift/jumptable.rs`, `block.rs`, `funcdata*.rs`, plus the
option/registration anchors — **4 ported-core files**, exceeding the small-feature budget.

## Speed / risk assessment
- **Risk:** medium-high. `checkUnrolledGuard` changes jump-table recovery, the most
  parity-sensitive S2 path. Must stay strictly default-off and gated so the 675-datatest
  baseline is byte-identical; turning it on must not over-accept tables on functions that
  legitimately have an indirect call inside a loop.
- **Speed:** the guard-lift runs only when a `BRANCHIND` sits behind a loop phi (rare), so the
  cost is bounded to those functions. Measure off-vs-on on the target before any default-on
  consideration; expect it to ship **default-off opt-in** at minimum.

## Recommendation
**Do not open a parallel pass.** Fold this testcase into the `unrolledguard` (PR #50) effort
as a second regression witness and a stage test, since the root cause and the machinery to
port are identical. If PR #50 lands first, this opportunity is closed by it and only needs a
`tests/stages/` witness. Approve here to (a) consolidate with PR #50, or (b) dispatch a
dedicated `switchunrolledguard` implementation worker on this branch.
