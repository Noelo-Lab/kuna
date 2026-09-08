# getlasterror-consumes-arguments-prepared — APPROVED, RESUMING A NEARLY-GREEN BRANCH

**Status: APPROVED by the captain. This is a SALVAGE, not a fresh design.**

The previous builder on this branch was killed after it had already implemented the whole
feature and pushed it. The branch is **3 failing tests from green**. Do NOT re-derive the
diagnosis, do NOT redo the feature, do NOT open a new branch. Start at Phase 3 by running the
workspace suite and fixing exactly the three failures below.

## Already done — MUST NOT be redone or reverted

3 commits on this branch (`086bb4bb` impl, `cb15a505` DIV-142, `e2e6a2f8` docs), 21 files,
+773/-11:

- option `calleeprotostack` (P4, default on, tier core, live_field `callee_proto_stack`),
  full `phases.toml` row, registration in `p0_knowledge/options.rs` + the arch seam
- new module `p4_calls/kuna_calleeprotostack.rs`
- DIV-142 registry row, spec prose, `docs/options.md` regenerated
- promoted acceptance probe; fixture
  `decompiler/crates/kuna-analysis/tests/fixtures/calleeprotostack_pe_i386.exe` + its generator

**`parity gates` is GREEN in CI on this branch** (run 34267248765): `make test`,
`make test-stages`, `make check-spec`, `make test-cli`, `kuna catalog --check` all pass. All 6
CodeQL jobs pass. **Only `make rust-test` is red.**

## The 3 deterministic failures — fix these and nothing else

Identical across two independent local runs. The noisy `verify_w10_*` / `verify_w3_*` tail is a
KNOWN worktree artifact of `KUNA_DECOMP_TEST`, not a regression: relaunch the gate with
`env -u KUNA_DECOMP_TEST make rust-test` and it disappears. Do not chase it.

**(a) `p0_knowledge::kuna_phases::tests::option_values_live_value_present_for_58_suppressed_for_101`**
`left: 59, right: 58` at `kuna_phases/tests.rs:732`. A FOURTH hidden count assert, in a file
where this branch already bumped three (settable_count 171->172, tier_counts 48->49 core, the
`json.matches("},\n")` 170->171). Because `calleeprotostack` declares
`live_field = callee_proto_stack`, `with_live` becomes 59. Mechanical repair: bump the assert to
59, append the ledger comment
`// 58 -> 59: +1 for calleeprotostack (live_field = callee_proto_stack, DIV-142).`
and **rename the fn** to `option_values_live_value_present_for_59_suppressed_for_101` — the
count is in the name.

**(b) `p4_calls::kuna_calleeprotostack::tests::declared_caller_frame_floor_declines_without_evidence`**
**(c) `p4_calls::kuna_calleeprotostack::tests::resolve_declared_extra_pop_leaves_a_model_that_states_its_own`**
Both panic `FuncProto::store: null` at `p4_calls/fspec.rs:5242:31`. These are the branch's OWN
new unit tests against its OWN new module — **a real bug, not bookkeeping.** Decide which side is
wrong: is the test fixture under-constructed (build the `FuncProto` with a store), or must the
new code tolerate a null store? "Make the test pass" has a WRONG answer available here — if a
real caller can reach the new path with a null store, the code must tolerate it and the test is
right to catch it. Justify whichever you pick in the commit message.

## Pre-cleared — do NOT relitigate

- **DIV-142 is uncontested.** No competing claim on the number.
- **The ABSENT `tests/stages/` XML is CORRECT.** A three-arm experiment with a control proved
  the datatest harness cannot `load file` a synthesized PE32, and a stages ERROR does not set a
  non-zero exit. Do not add one back.
- Before any `git pull` involving main, `mv docs/re-needs/getlasterror-consumes-arguments-prepared.md /tmp/`
  — that file is untracked on main and tracked here, and the collision aborts the pull.

## Branch hygiene for this resume

Your worktree's branch is **one commit AHEAD of `origin/feat/re-getlasterror-consumes-arguments-prepared`**
— this proposal file is that commit, added by the captain to brief you. **Never
`git reset --hard origin/<branch>`**; just commit on top and push. PR **#521** is already OPEN
against this branch — update it, do not open a second one.

## Acceptance

The probe that must flip is `a-40f8d23cc7c1` (need
`getlasterror-consumes-arguments-prepared`). As of main `b23e01ec` its `exit_code` clause and
its first `stdout_matches` clause (`/GetLastError\(\)/`) already PASS; it fails only on the
FormatMessageA argument-list clause — the defect clause. That is the one your merge must flip.
