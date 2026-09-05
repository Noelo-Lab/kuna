# RESUME BRIEF — this is not a design proposal, it is a salvaged implementation

Written by the captain, round 2 wave 8. Read this before anything else; the
`IMPLEMENTATION MODE` line in your prompt points here.

## What already exists on this branch

Wave 7's builder implemented the fix and was **SIGKILLed (rc=137) mid `make rust-test`** at
15:30:23Z on 2026-09-04, before it could open a PR. The captain committed its worktree as
`679f4a70` (also tagged `salvage/r2w7-decompiling-3396`) and deleted the worktree. Nothing
was pushed.

The commit makes `kuna decompile <bin> <fn>` stop following the same function's flow twice —
`load function` and `decompile` each rebuilt the whole lift, block build and per-jump-table
sub-decompilation from scratch. 8 files: `kuna-console` `decompile_step.rs` /
`ifacedecomp.rs` / `interface.rs`, `kuna-decomp/src/infra/decompile_drive.rs`, a new
`kuna-console/tests/verify_flowreuse.rs` (211 lines), `docs/spec/00-overview.md`, and a
drafted `pr_body.md`. This is **attempt 2**: attempt 1 (#380, merged) removed the O(N²)
dead-list scan and took the witness 71.5s -> 19.4s; the acceptance bar is a median under
10s, so the need stayed open.

**Assume none of it is verified.** No gate is known to have passed on it.

## What is left for you

1. Rebase onto `main` (now `3411f35c`; the branch was cut at `74118fd7`). `git merge-tree`
   reported **no conflicts** as of this brief.
2. Build, then re-derive the evidence yourself: acceptance
   `verify --need decompiling-3396-byte-main --json` (currently FAIL on the `wall_ms`
   clause alone — median 19299 ms against a `< 10000` bar; `exit_code` already passes),
   the four gates, and an interleaved min-of-N timing, never a single run on a loaded box.
3. Read the existing diff critically. Reusing a followed flow across `load function` and
   `decompile` is a correctness question, not only a speed one: the facts a decompile is
   seeded with (`map address` symbols, DWARF stack locals, `type varnode` usepoints,
   `map hash` dynamic symbols) are consumed **at flow time**, which is why kuna rebuilt.
   Convince yourself the reuse path cannot silently drop one of them — a wrong decompile
   that is 3x faster fails this need, and `verify_flowreuse.rs` is wave 7's own test, so it
   is not independent evidence.

## Two hazards that are specific to this branch

- **Your branch is `feat/re-decompiling-3396-w8`, deliberately NOT
  `feat/re-decompiling-3396-byte-main`.** That older name still exists on `origin` pointing
  at `0e8ce0eb`, the already-merged head of PR #380, which is not an ancestor of anything
  you have; `open_pr.sh` pushes without `--force`, so pushing the old name would be
  rejected non-fast-forward. Do not rename the branch back.
- **The gate that killed wave 7.** Two builders died together in `make rust-test` in
  separate worktrees; OOM can be neither confirmed nor excluded on this box. Cap cargo
  parallelism (`CARGO_BUILD_JOBS`), keep `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
  CARGO_PROFILE_TEST_DEBUG=0`, and **commit your worktree before starting any long gate** —
  a commit is the only thing that survived last time. A foreground bash call also caps at
  600s, which `make test` and a CI-waiting `open_pr.sh --merge` both exceed; run those
  detached with an rc file and poll.
