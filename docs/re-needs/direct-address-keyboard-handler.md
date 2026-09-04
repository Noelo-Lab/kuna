---
need_id: direct-address-keyboard-handler
title: Direct-address keyboard handler decompilation annexes the unrelated renderer
track: quality
status: open
severity: major
probe_id: p-36f0974fc119
acceptance_id: a-f13adb91d5d4
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [6989ca5da15272fa37a80c43]
rounds: [2]
first_seen_round: 2
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

A bounded decompilation of the keyboard callback at 0x6500.

> **Direct-address keyboard handler decompilation annexes the unrelated renderer** (major, `6989ca5da15272fa37a80c43`)
> The output begins with keyboard handling, but continues through the renderer logic from 0x4610, including the UI title and drawing calls. The disassembly shows a tail jump to 0x4610 at 0x66e2; it should remain a call/tail-call boundary rather than duplicating thousands of lines into the callback.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x6500",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "LUGOSI.*S II",
      "sub_5e30\\("
    ]
  },
  "target": {
    "binary_rel": "bin/lugosiii",
    "binary_sha256": "8ea3c454dfcea5d5f970d695608568ed81b688dd367faa2f3fcbeb4bc700aa6e",
    "binary_size": 29336,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x6500",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "sub_6500\\("
    ],
    "stdout_absent": [
      "LUGOSI.*S II",
      "sub_5e30\\("
    ]
  },
  "target": {
    "binary_rel": "bin/lugosiii",
    "binary_sha256": "8ea3c454dfcea5d5f970d695608568ed81b688dd367faa2f3fcbeb4bc700aa6e",
    "binary_size": 29336,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Because 0x6500 is absent from the discovered function map, direct-address recovery follows its tail jump into the already separate renderer instead of treating the jump as an interprocedural edge.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6989ca5da15272fa37a80c43` (round 2, tester t-r2-6989ca5d)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): SPLIT out of the 4-observation `wrong-output|decompile|stdout_absent` group. Its sibling is [keyboard-callback-uses-undefined]; both testers trace their symptom to the same root (0x6500 is absent from the discovered function map), but the remedies differ -- a tail-jump-as-interprocedural-edge boundary fix here, live-in register promotion there -- so they carry separate acceptance probes. This is also the first INDEPENDENT tester demand for the captain-seeded `no-cli-function-boundary-override`; note the tester asks kuna to get the boundary right by itself, not for a CLI override.
- round 2 T_TRIAGE (captain): track/touches/scope CONFIRMED (quality / kuna-decomp / small). Measured this tick so the builder does not have to: `funcboundflow` does NOT cover this. Decompiling 0x6500 of lugosiii with and without `--option funcboundflow on` gives byte-identical 1555-line output still containing the renderer's LUGOSI title twice, i.e. the tail jump at 0x66e2 still annexes 0x4610. (funcboundflow is default-ON per DIV-67, so `on` is a no-op; the point is that the shipped fix for function-merge does not reach a tail jump into a known entry.) Kept small: treating a jump to a discovered function entry as a tail call is a bounded, gateable decision.
- round 2 B_PLAN wave 13 (captain): DISPATCHED, attempt 1, and the acceptance was STRENGTHENED before dispatch. As filed, the probe asserted only two `stdout_absent` patterns and nothing else, so a "fix" that made `kuna decompile 0x6500 --addr` emit nothing, or error out, would have PASSED it and closed the need with worse output than we started with. Added `exit_code {eq: 0}` and one positive `stdout_matches` on `sub_6500\(` -- the emitted signature of the function actually asked for. Both new clauses were measured GREEN on today's build before the edit was kept, and the replayed probe still FAILS on exactly the two original absent clauses (a-a9a71e29cc50 -> a-f13adb91d5d4; verify resolves the block, the frontmatter id is a label and was updated with it). If a legitimate fix renames the entry away from `sub_6500`, say so in the PR rather than fighting the clause -- that is a probe bug, not a wrong fix.
- round 2 B_PLAN wave 13 (captain): briefing carried forward. (a) The T_TRIAGE measurement stands: `funcboundflow` (default-ON, DIV-67) does NOT cover this -- `--option funcboundflow on` gives byte-identical 1555-line output. (b) `no-cli-function-boundary-override` shipped in #374 as an `--assert` CLI override; the acceptance cmd deliberately carries NO `--assert` and NO `--option`, so telling the user to override the boundary by hand does not close this need. The default path has to get the tail jump at 0x66e2 right. (c) The sibling `keyboard-callback-uses-undefined` (large, live-in register promotion) is traced to the same root by its tester; do NOT annex it -- it has its own lease and its own probe. (d) `binary_source: dataset` means `verify --promote` will REFUSE to vendor this into `tests/cli/` (the #377/#392 refusal), so the in-repo regression cover has to be a `tests/stages/` two-pass XML on a hand-built bytechunk. (e) Branch `feat/re-direct-address-keyboard-handler` does not exist and no worktree of that worker id exists, so the silent-detached-worktree fallback is NOT armed for this dispatch.
- round 2 B_DONE (captain): **attempt 1 ENDED AT ITS BUDGET CAP WITH THE WORK COMMITTED AND
  UNMERGED. DO NOT RE-DISPATCH THIS AS UNTOUCHED.** `b-r2-direct-address-k` ended 22:16Z
  terminal_reason=completed, subtype=success, total_cost_usd 29.93 against a $30 cap, with a full
  4598-byte result.json -- a deliberate stop, not a 429 and not a SIGKILL. Its tree is committed as
  **`616959b7` on `feat/re-direct-address-keyboard-handler`, parented directly on `8ff1b4ec`**
  (i.e. already rebased onto current main); no PR was opened and origin/main is untouched.
  23 files: `option tailcallframe` (default on, DIV-109) in `p2_lift/kuna_tailcallframe.rs` + its
  tests, `p2_lift/flow.rs`, `infra/decompile_drive.rs`, `infra/architecture.rs`, phases.toml,
  options.rs, `docs/spec/02-lift-and-flow.md`, `docs/history.md`, `docs/options.md`,
  `tests/stages/tailcallframe.xml` + the stages baseline + corpus count, the catalog count tests
  and bytecompat fixture, and `tests/cli/direct-address-keyboard-handler.json`.
  Its armed merge chain did NOT fire and cannot: `/tmp/b-r2-dak.final` reads `rust3=2 / DONE` and
  the chain merges only on rc 0. **That rc=2 is the worktree environment, not its code** --
  `/tmp/b-r2-dak.rust3.log` ends `error: 327 targets failed` across kuna-cli/kuna-sleigh/kuna-wasm
  and every `--doc` target, i.e. the known worktree spec-resolution trap (symlink the main tree's
  built `.sla` into the worktree's `specs/`) compounded by `KUNA_DECOMP_TEST` being set in its env.
  attempts -> 1. **Resume on the same branch (`RESUME_BRANCH=feat/re-direct-address-keyboard-handler`)**
  so the next builder finishes the gate and opens the PR instead of rebuilding 1,198 lines.
  Unverified builder claims, recorded as claims: test/test-stages/check-spec/catalog green on the
  rebased tree, acceptance flips 1,555 -> 427 lines, a 62-binary / 377-entry sweep with 0 firings
  and 0 diffs off the witness. **And it says it OVERTURNED the filed hypothesis:** the annexation is
  caused by the jump TARGET `0x4610` being absent from the function map, not by `0x6500` being
  absent. `hypothesis_status` is left `inconclusive` on purpose -- that field is the refuter's, and
  no refuter has checked this; fold it in when the PR lands.
