# Provenance — `overlapbranch` (RE-need `overlapping-anti-disassembly-sequence`)

This bundle has no design proposal of its own. The need was triaged **scope: large**
in round 2 because the fix moves instruction boundaries, and the round-2 refutation
named the real risk one step past the filed hypothesis: *"which stream owns the
overlapped bytes, and what happens to flow and dataflow already built from the
discarded one. A builder must answer that explicitly (and gate it), or the fix
trades bogus out-of-image code for a silently truncated function."*

That question is answered in **`record.json`** (`ownership-policy`,
`reconvergence-exclusion-ablated`) and in **`pr_body.md`**; the normative prose lives
in `docs/spec/02-lift-and-flow.md` and the divergence rationale in the DIV-106 row of
`docs/history.md`.

## How this branch was built

Round 2 wave 7 implemented the whole option and was SIGKILLed mid `make rust-test`
before it could open a PR. The captain committed the worktree as a salvage commit
(`salvage/r2w7-overlapbranch`) with no gate known to have passed on it, and wave 8
resumed from there: rebased onto `main`, re-derived every shared counter, ran the four
gates and the acceptance probe, swept 428 binaries, ablated the reconvergence
exclusion, measured the speed delta, and added the end-to-end CLI gate. Nothing on
this branch is inherited unverified — the evidence in `record.json` is wave 8's own.
