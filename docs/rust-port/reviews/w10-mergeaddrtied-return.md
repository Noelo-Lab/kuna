# w10-mergeaddrtied-return
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, structurally separate from porter)
date: 2026-06-14
branch: rport/w10-mergeaddrtied-return (32d4f6a), base rust-port (3395637, == merge-base, clean FF descendant)

gate:
  - cargo test --workspace (branch, with 3 verifier tests added) -> 3253 passed, 0 failed, 36 ignored
  - cargo clippy -p kuna-decomp --lib (changed files, forced rebuild) -> clean (0 warnings in coreaction_cleanup.rs / coreaction_protos.rs)
  - cargo clippy -p kuna-decomp --test verify_w10_implied_vars_adversarial / --test verify_w10_mergeaddrtied_return_adversarial -> clean
  - C++ oracle (main tree, untouched by this rust-only branch): python -m kuna.run_tests --all --baseline docs/baseline.json -> 207/207 unit + 675/675 datatest, PARITY OK
  - branch touches ONLY 3 rust/ files (no C++ / spec / docs) -> oracle structurally untouched

## Scope

The diff makes two coupled changes (79 ins / 77 del, mostly comments):

  1. `mark_output_storage_addr_tied` (coreaction_cleanup.rs:233-264) SIMPLIFIED: un-tie
     the recovered return register iff `output_locked && written.len()==1 && def is
     CPUI_COPY`. The previous `is_pure_transient` source inspection (COPY source must
     not be addr-tied / spacebase / stack) is DROPPED.
  2. `reset_local_window()` (coreaction_protos.rs ActionPrototypeTypes) made
     UNCONDITIONAL — previously env-gated behind `KUNA_RESET_LOCAL_WINDOW` (LOSS-156).
  3. The A1 assertion in verify_w10_implied_vars_adversarial.rs updated to the new shape.

## Faithfulness (verified against the C++ in scope)

- `reset_local_window` (varmap.rs:935) is a line-by-line faithful port of C++
  `ScopeLocal::resetLocalWindow` (varmap.cc:432): same stackGrowsNegative /
  minParamOffset / maxParamOffset init, same `rangeLocked` early-return, same
  local+param range merge into `newrange`, same `setRange`. Flipping it to default-ON
  matches C++ `funcp.setScope` -> `resetLocalWindow` at funcdata.cc:70 (unconditional).
  RangeList is `BTreeSet<Range>` (ordered, == C++ `set<Range>` iteration). FAITHFUL.
- The un-tie simplification is MORE faithful, not less: C++ register `addrtied` is
  decided by the SymbolEntry's `uselimit.empty()` (database.cc:1155) and painted onto
  in-scope varnodes by `syncVarnodesWithSymbols` (funcdata_varnode.cc:959), which
  iterates ONLY the stack/local space (`lm->getSpaceId()`) — never a register. So a
  register's tied-ness depends on whether the register itself is a whole-function
  local, NOT on what a COPY into it sources. Dropping the `!src_stack`/`!is_spacebase`
  source check is therefore CLOSER to C++. FAITHFUL.
- The win matches the committed C++ oracle: tests/golden/snapshots/cpp/condconst/
  002-B5.txt recovers condconst_conn's return as `int4 v1; // stack - 0xc` (a STACK
  local). The branch surfaces the same `// stack` local — movement TOWARD the oracle.

## The collapse is REAL (not a no-op, not over-collapse)

- Driving condconst_conn end-to-end: BASE (default, env OFF) renders NO stack local
  (`uint8 v1; // rax; v1 = ZEXT(a0); return v1;` with a0 inlined). BRANCH renders
  `uint4 v2; // stack - 0xc; ...; v1 = ZEXT(v2); return v1;`. Verifier test V1
  (verify_w10_mergeaddrtied_return_stack_local_matches_oracle_not_spurious_array)
  FAILS on base, PASSES on branch — a genuine base->branch behavioral difference.
- The stack local is a SCALAR (`uint4`), NOT the degraded-pipeline `xunknown1 [N]`
  array LOSS-156 warned about (V1 asserts no `xunknown`/`$$undef`).
- NON-collapsible cases stay separate: condconst_copy (multi-write, MULTIEQUAL) stays
  explicit `v1 = ZEXT(a0)` x2 / `return v1;`, NEVER `return ZEXT(a0);` (V2 + porter A2);
  boolless ACC stays explicit and byte-identical to the B5 oracle (porter A3).

## No fabricated positive / no regression

- Full Rust datatest stringmatch tally (verify_w10_corpus_stringmatch_tally, 675
  assertions) is BYTE-IDENTICAL base==branch: 5 PASS-positive / 32 PASS-negative /
  628 FAIL-positive / 10 FAIL-negative. No assertion outcome moved.
- printc_parity: 8/9 corpus functions decompile+print on BOTH base and branch; the
  single execution failure (`gp`/printf, "Bytes at 0x00410024 are not mapped", a
  pre-existing loader gap) is identical -> EXECUTION-FAILURE COUNT does not regress.
- ISOLATION: base with `KUNA_RESET_LOCAL_WINDOW=1` produces output BYTE-IDENTICAL to
  the branch for both condconst_conn and condconst_copy. The entire observable change
  is the env-gate flip (a faithful, accepted-but-dormant capability, LOSS-156); the
  un-tie simplification is output-NEUTRAL on the measured corpus (can't over-collapse
  what it doesn't change) and is a latent correctness improvement.

## No special-casing

grep of the diff (coreaction_cleanup.rs, coreaction_protos.rs, test) for function
names / addresses / register names: every `condconst`/`boolless`/`eax`/`rax`/`acc`/
`global_cross` hit is in a COMMENT, a `#[cfg(test)]` generic address (`0x1000`), or a
generic action-name assertion. The live decision logic keys ONLY on `output_locked`
(proto flag), `written.len()==1` (IR shape), and `op.code()==CPUI_COPY` (IR shape).
No name/address/register hardcode. PASS.

## Hunt list

- Signedness: clean — no signed/unsigned comparison in changed logic (`written.len()`
  is usize vs literal; `size<=0` guards int4 size).
- Integer widths: clean — no int4/uintb arithmetic introduced.
- Wrapping: clean — no arithmetic on uintb-lineage values in the change.
- Comparator totality: n/a — no comparator added; RangeList Ord unchanged.
- Iteration order: clean — `iter_loc_size_addr` (fixed addr/size, mark-all idempotent,
  order-insensitive); `reset_local_window` iterates BTreeSet<Range> (ordered == C++
  set<Range>). No HashMap/HashSet.
- Off-by-one / do-while / reverse iter: n/a.
- Erase-while-iterating: clean — `targets` collected first, then iterated.
- Exception->Result partial-state: n/a — function returns (), early-returns for no-op,
  no `?` mid-mutation.
- Mechanical: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare-`as`
  in the changed decision logic.

findings:
  - F1 (LOW, documentation): the comments on `mark_output_storage_addr_tied`
    (coreaction_cleanup.rs:216-219) and `coreaction_protos.rs:175-177) state the
    condconst_conn collapse happens "because the eax register is written by a single
    return-value COPY, so it is never a whole-function local". On condconst_conn the
    eax `v1` is defined by `ZEXT(v2)`, NOT a `CPUI_COPY`, so the un-tie does NOT fire
    there — the observable collapse + `// stack` local come ENTIRELY from the
    now-default window-reset (proven: base env-ON == branch, byte-identical). The
    comment overstates the un-tie's role on this specific function. Code is correct &
    output-neutral; comment is inaccurate.
       cpp: decompiler/cpp/database.cc:1155, funcdata_varnode.cc:959 (the rule the
            comment cites is real; its application to condconst_conn is mischaracterized)
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:216, coreaction_protos.rs:175
  - F2 (LOW, ledger): LOSS-157 (already merged) claims "condconst is byte-identical
    with the env on or off" and "there is no stack int4 local at all". This branch's
    env-default-flip exposes that as a factual error — env ON clearly produces
    `uint4 v2; // stack - 0xc`. Not a defect in THIS branch; flagged so the loss ledger
    is corrected.
       rust: docs/rust-port/losses.md (LOSS-157 surface bullet)

adversarial tests (committed with the verdict, item-tagged):
  rust/crates/kuna-decomp/tests/verify_w10_mergeaddrtied_return_adversarial.rs
  - verify_w10_mergeaddrtied_return_stack_local_matches_oracle_not_spurious_array (V1)
      — the discriminating test: FAILS on base (no `// stack`), PASSES on branch
        (matches oracle 002-B5.txt `int4 v1; // stack - 0xc`, scalar not array).
  - verify_w10_mergeaddrtied_return_multiwrite_not_overcollapsed (V2)
      — multi-write condconst_copy stays explicit, never `return ZEXT(...)`.
  - verify_w10_mergeaddrtied_return_default_reset_no_cross_function_leak (V3)
      — no boolless tokens leak; named return local preserved (no over-inline).
  All 3 pass on the branch; V1 fails on base (proving real movement, not a no-op).

losses: LOSS-158 (the eax `v1 = ZEXT(v2)` register round-trip still differs from the
  oracle's clean `v1 = x`; the un-tie source-check drop is latent/unobservable;
  comment inaccuracy F1) — see docs/rust-port/losses.md.

## Why ACCEPT-WITH-LOSSES (not ACCEPT, not REJECT)

Not REJECT: the changes are faithful transcriptions of their C++, there is NO
special-casing, the collapse is real and matches the committed oracle direction, no
datatest assertion regresses or is fabricated, boolless/readstruct are byte-identical,
the execution-failure count is flat, and the full workspace + C++ oracle are green.
Not ACCEPT: the observable win is a PARTIAL-parity movement (the eax round-trip
`v1 = ZEXT(v2)` remains where the oracle has `v1 = x`), the un-tie simplification is
output-neutral/latent rather than load-bearing on the measured corpus, and the in-code
comments overstate the un-tie's role on the headline function (F1). These are losses,
not blockers.
