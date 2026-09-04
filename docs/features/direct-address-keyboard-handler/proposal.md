# direct-address-keyboard-handler — RESUME BRIEFING (captain, round 2 wave 14)

**This is not a design proposal. It is a salvage briefing.** The implementation you were told to
"implement starting at Phase 3" is ALREADY WRITTEN and ALREADY COMMITTED on this branch as
`616959b7` — 1,198 lines across 23 files, `option tailcallframe` (default on, DIV-109) in
`p2_lift/kuna_tailcallframe.rs`. Attempt 1 wrote it and then stopped at its $30 budget cap with
everything committed, no PR opened, and origin/main untouched.

**Your job is to finish and land it. Do not redesign it, do not restart from scratch, and do not
re-derive the mechanism.** You have a fresh budget; spend it on verification and landing.

## Step 0 — read the CURRENT need record, not your branch's copy

Your branch copy of `docs/re-needs/direct-address-keyboard-handler.md` is one commit stale:

```bash
git show origin/main:docs/re-needs/direct-address-keyboard-handler.md
```

Its final `round 2 B_DONE (captain)` entry is the ground truth on what attempt 1 did, which of its
claims are verified (none) and which files it touched.

## Step 1 — rebase

```bash
git fetch origin && git rebase origin/main
```

Your parent is `8ff1b4ec`; main was `3d45b044` at dispatch, one docs-only commit ahead touching only
`docs/re-needs/*`. The captain verified `git merge-tree` returns rc=0 with zero conflict markers
against that head. Re-verify rather than trust it if main has moved further.

## Step 2 — attempt 1's results are CLAIMS, not results

Nothing below was verified by anyone. Re-run all of it yourself:

- `make test` / `make test-stages` / `make check-spec` / `kuna catalog --check` reported green on
  the rebased tree.
- the acceptance probe reported flipping the output from 1,555 lines to 427.
- a 62-binary / 377-entry sweep reported 0 firings off the witness and 0 output diffs.

## Step 3 — the rc=2 that killed attempt 1 was THE WORKTREE ENVIRONMENT, not its code

`make rust-test` ended `error: 327 targets failed` across kuna-cli / kuna-sleigh / kuna-wasm and
every `--doc` target. That is the known worktree spec-resolution trap, not a regression:

- `KUNA_SPECS` / `SLEIGHHOME` do NOT reach the cargo workspace suite; those targets resolve specs as
  `<repo>/specs` relative to their own crate. `worker.sh`'s setup symlinks the main tree's built
  `.sla` into this worktree — confirm it with `ls specs/*.sla | wc -l` (expect ~148) before blaming
  your code. **Never run `make specs` in a worktree.**
- `KUNA_DECOMP_TEST` was set in attempt 1's environment and makes `rust-test` fail spuriously.
  `unset KUNA_DECOMP_TEST` before running the suite.
- Build with `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0` and delete
  `decompiler/target/debug` when the suite finishes.

## Step 4 — the count-sensitive paths this branch already touches

`phases.toml`, `p0_knowledge/options.rs`, `kuna_phases/tests.rs`, `tests/catalog_bytecompat.rs`,
`tests/fixtures/phase_catalog.json`, `kuna-base/src/xml.rs` (stages corpus count),
`docs/baseline-stages.json`, `docs/options.md`, `docs/history.md` (DIV-109 — the DIV number is
claimed on MERGE, so renumber it if another DIV landed first). Run `counters --fix` AFTER your final
rebuild, then `mergecheck`. Regenerate `docs/options.md` from the freshly built binary.

## Step 5 — regression cover and the PR

`binary_source: dataset` means `verify --promote` will REFUSE to vendor the acceptance into
`tests/cli/`. Attempt 1 already added `tests/cli/direct-address-keyboard-handler.json` and
`tests/stages/tailcallframe.xml` — confirm they actually run and actually assert something rather
than assuming they do.

**There is no draft PR to undraft.** The implementation-mode preamble's `open_pr.sh --undraft` line
does not apply here — open a normal PR and merge it per the builder prompt's Phase 8.

## Step 6 — the hypothesis is not yours to settle

Attempt 1 reports it OVERTURNED the filed hypothesis: the annexation is caused by the jump TARGET
`0x4610` being absent from the discovered function map, not by `0x6500` being absent. Put the
evidence in the PR body, but leave `hypothesis_status` alone — a builder cannot refute itself.
