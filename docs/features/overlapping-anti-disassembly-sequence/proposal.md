# RESUME BRIEF — this is not a design proposal, it is a salvaged implementation

Written by the captain, round 2 wave 8. Read this before anything else; the
`IMPLEMENTATION MODE` line in your prompt points here.

## What already exists on this branch

Wave 7's builder implemented the whole feature and was **SIGKILLed (rc=137) mid
`make rust-test`** at 15:30:23Z on 2026-09-04, before it could open a PR. The captain
committed its worktree as `742c9a42` (also tagged `salvage/r2w7-overlapping-anti`) and
deleted the worktree. Nothing else was lost, and nothing was pushed.

The commit is a complete `option overlapbranch` (P2 / flow-classification): a conditional
branch whose target lies strictly inside its own fall-through instruction ends the
fall-through in a halt, instead of letting the overlapping decode swallow the target.
21 files — the new `p2_lift/kuna_overlapbranch.rs` + tests, `flow.rs`/`architecture.rs`/
`decompile_drive.rs` wiring, the `phases.toml` row, `options.rs` registration, all four
hard-coded catalog/corpus counts (`kuna_phases/tests.rs`, `catalog_bytecompat.rs`,
`phase_catalog.json`, `kuna-base/src/xml.rs`), `docs/baseline-stages.json` re-recorded,
`tests/stages/overlapbranch.xml`, `tests/cli/overlapping-anti-disassembly-sequence.json`,
`docs/options.md`, a `docs/history.md` DIV row, `docs/spec/02-lift-and-flow.md`, an
`overlapbranch_i386` fixture, and `docs/features/<slug>/pr_body.md`.

**Assume none of it is verified.** No gate is known to have passed on it.

## What is left for you

1. Rebase onto `main` (now `3411f35c`; the branch was cut at `74118fd7`). `git merge-tree`
   reported **no conflicts** as of this brief. Re-check the four hard-coded counts and the
   DIV number after the rebase — a clean textual merge of a counter is exactly how this
   repo merges to the *wrong* number, and DIV numbers are claimed on merge, not on branch.
2. Build, then re-derive the evidence yourself: the acceptance probe
   (`verify --need overlapping-anti-disassembly-sequence --json`, currently FAIL on
   `sub_b10cdc7()` and `dat_d98ddd00 +=`), the four gates, and a measured speed delta.
3. Read the existing diff critically before defending it. The filed hypothesis is `upheld`
   (credibility 0.85), but wave 7's design decisions were never reviewed by anyone.
4. Open the PR (`pr_body.md` is already drafted) and merge it under the `merge` lease.

## Two hazards that are specific to this branch

- **The gate that killed wave 7.** Two builders died together in `make rust-test` in
  separate worktrees; OOM can be neither confirmed nor excluded on this box. Cap cargo
  parallelism (`CARGO_BUILD_JOBS`), keep `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
  CARGO_PROFILE_TEST_DEBUG=0`, and **commit your worktree before starting any long gate** —
  a commit is the only thing that survived last time.
- **A foreground bash call caps at 600s**, which `make test` and a CI-waiting
  `open_pr.sh --merge` both exceed. Run those detached with an rc file and poll.

This branch has **no remote copy**, so the first push is an ordinary fast-forward.
