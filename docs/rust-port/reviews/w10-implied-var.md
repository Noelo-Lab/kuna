# Verdict — `rport/w10-implied-var` (round 1)

**Item:** w10-implied-var — "markInternalCopies + pushImpliedVar (implied-var COPY collapse)"
**Branch:** `rport/w10-implied-var` @ `a3ec1af`
**Base reviewed against:** `rust-port` @ `cc925d3` (merge-base of the 3-dot diff is `55bceb6`)
**Verdict:** ACCEPT_WITH_LOSSES

## What the diff actually is

The diff is a **single hunk** in
`rust/crates/kuna-decomp/src/coreaction_cleanup.rs::mark_output_storage_addr_tied`
(31 ins / 19 del, one file). It does **not** port `Merge::markInternalCopies` /
`Merge::processCopyTrims` / `Merge::markRedundantCopies` or
`PrintLanguage::pushVnImplied` / `PrintC` expression pushing — those are not in this
commit at all. The commit title is misleading.

What it changes: the predicate that decides whether the recovered return-value
register storage is left **un-tied** (eligible to be marked IMPLIED by `baseExplicit`
and inlined into the `return` expression). This function is a **pre-existing kuna
structural stand-in** (introduced `63393c1`, last touched `32d4f6a`) for the unported
C++ `ScopeLocal::inScope` / `syncVarnodesWithSymbols` register-local recovery
(`funcdata_varnode.cc:997`, `database.cc:1155`). It is, by construction, NOT a
line-for-line transcription of any one C++ function — it reconstructs the *effect* of
addr-tiedness off the IR shape. That divergence is inherited, not introduced here.

Predicate change: old `output_locked && written.len()==1 && def==CPUI_COPY` →
new `output_locked && !written.is_empty() && !has_marker_write` where
`has_marker_write` = any instance whose def `op.is_marker()`.

## Faithfulness — PASS (the refinement is MORE faithful, not a one-function tune)

- C++ `ActionMarkExplicit::baseExplicit` forces a varnode explicit when
  `def->isMarker()` (coreaction.cc:3112) or `isAddrTied()` (3120). The new predicate
  keys on exactly that `isMarker()` structural signal. The old `len()==1 && CPUI_COPY`
  test was the narrowly-shaped one (force-tied legitimate multi-op forward chains
  INT2FLOAT→FLOAT_MULT→COPY); the new test generalizes correctly.
- `is_marker()` (op.rs:648) is a faithful flag check `flags & marker != 0`, identical
  to C++ `op.hh:180`. The `marker` flag is set on CPUI_MULTIEQUAL (typeop.rs:895,
  cpp typeop.cc:1949) and CPUI_INDIRECT (typeop.rs:899, cpp typeop.cc:1990) — the two
  ops the comment names. The structural signal is genuine.
- Input-instance behavior change (no def → not a marker → no longer blocks un-tie) is
  the one real semantic shift vs the old predicate. It is justified in-comment and,
  critically, is shown harmless below (empty regressed set).
- The `baseExplicit`/markexplicit/markimplied machinery (same file, ~lines 500-650)
  is **untouched** by this commit (the only hunk is at line 240).

## No special-casing — PASS

Executable lines key only on `is_output_locked`, `is_written`/`is_input`, `get_def`,
`is_marker()`. Grep of the whole changed file for function names / hex addrs / magic
counts in code: the only hits are inside comments (prose: boolless / rand_calc /
condconst) and a `#[cfg(test)]` fixture (`0x1000`). **Zero** name/address/opcode-count
hardcoding in production logic. No string-literal function-name comparisons.

## Real parity (new passes) — PASS

Floating-point cast #5-#9 newly pass with the **exact oracle `<stringmatch>` regexes**
(`floatcast.xml`: `return \(float4\)\(v1 \- v2\);`, etc.) — real parity, not a loose
substring. mixfloatint stays FAIL on BOTH base and merged (no false improvement
manufactured). Datatest "Success" requires the expected render under `min/max` bounds
(incl. negative `max="0"` matches), so it is a robust oracle check.

## No regression — PASS (the decisive test)

The 3-dot diff base (`cc925d3`) does NOT contain the union-cluster the branch's
merge-base predates, so a naive base↔branch set diff shows 5 phantom "Union" losses
that are unrelated to this change. The honest test is to **apply the one-file diff onto
current `rust-port` HEAD** (`cc925d3`) and re-run the datatest harness:

| measurement | result |
|---|---|
| rust-port HEAD passing | 271 / 672 applied |
| + implied-var diff (applies CLEAN) | **289 / 672 (+18)** |
| **REGRESSED set (passed→failed)** | **EMPTY** |
| newly passing | Bitfields #6/#9/#10/#30, MIPS Bitfields #6/#7/#9/#30, Floating-point cast #5-#9, Deindirect Output #2, Modified conditional constant #1, Relative base #3, **Union #24, #26** |
| `cargo test --workspace` | all green, 0 failed |
| `verify_w10_implied_vars_adversarial` (4 tests) | all pass (incl. boolless byte-parity + multi-write over-inline guard) |
| struct/conditional/read datatest statuses base vs merged | identical except the 1 improvement |
| `cargo clippy -p kuna-decomp --lib -D warnings` | clean |
| reserved files (unionresolve / funcdata_union / dtype / printc union arm) | NOT touched |
| C++ oracle `run_tests --all --baseline` | 207/207 + 675/675, **PARITY OK** (rust-only change) |

The empty regressed set is the witness that the input-untie behavior change does not
over-inline any passthrough/multi-def return: a wrong predicate would have silently
flipped a previously-passing assertion, and none flipped.

## Findings (non-blocking)

1. **Commit title/scope mismatch.** Claims `markInternalCopies + pushImpliedVar`;
   ports neither. It refines an existing stand-in heuristic. Documentation/honesty
   nit, not a correctness issue.
2. **Zero new adversarial tests committed.** The protocol asks ≥3. The 4 adversarial
   tests that pass (`verify_w10_implied_vars_adversarial.rs`) PRE-DATE this commit
   (`89d624e`/`32d4f6a`) and their doc-header still describes the OLD `len()==1`
   predicate (now stale). They do still exercise and pass against the new predicate
   (multi-write `condconst_copy` stays explicit via its MULTIEQUAL marker; boolless
   byte-parity holds), so coverage is adequate — but no NEW witness was added for the
   new input-untie branch or the multi-op forward-chain collapse this commit enables.

## Decision

Faithful (more faithful than the predicate it replaces), no special-casing, real
oracle parity on every new pass, and — on the honest post-merge test — **monotonic
with an empty regressed set**, all workspace tests green, no reserved file touched, C++
oracle untouched. The two findings are process/documentation losses, not blockers.

**ACCEPT_WITH_LOSSES.**
