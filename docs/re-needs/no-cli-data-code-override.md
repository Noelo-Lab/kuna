---
need_id: no-cli-data-code-override
title: an agent cannot mark a range as code or data, or set a type at an address
track: tooling
status: closed
severity: major
acceptance_id: a-567d3207ce62
hypothesis_status: overturned
credibility: 1.0
instances: 1
rounds: [2]
first_seen_round: 2
attempts: 1
touches: [decompiler/crates/kuna-cli/src, decompiler/crates/kuna-console/src/ifacedecomp.rs]
scope: large
pr: "391"
closed_in_round: 2
closing_pr: "391"
---

## Symptom

`map address <addr> <typedeclaration>`, `parse line`, `type varnode`, `readonly <addr+size>` and `volatile <addr+size>` all work in the console and none is reachable from `kuna`. `--option readonly on|off` is a GLOBAL toggle, not a range. `global add`/`global remove` -- the commands `phases.toml` names as the exposure for the `code-data-partition` subphase -- are `engine_unavailable` stubs, so marking a gap as code is unavailable even in the console. This is the lever the round-1 need `runtime-decrypted-code-opaque` wants: a buffer decrypted at runtime is data at load time and code afterwards, and nothing can say so.

## Reproduction

Every named CLI flag is rejected with `error: unknown option`, verified on the round-1 merge build; the stubs are verified by driving `decomp_dbg` directly and reading back `engine integration not yet ported`.

## Acceptance

Asserts the DESIRED behaviour: an agent states two RANGE facts kuna cannot derive
-- eight bytes of `.data` that nothing ever writes, and a device word outside every
loaded section -- on one `kuna decompile --json`, and both land in the emitted C
while the run reports each one's fate machine-readably at its own phase
coordinates. FAILS at `ed353fb2`: exit 2, `unknown directive "readonly"` (Stage A
shipped `--assert` with nine directives, none of them a range).

Filed by the Stage B builder, which is what section 6 of the `--assert` proposal
asked for: no fixture tried there showed an observable `readonly` effect, so no
probe could be filed. The reason turned out to be ORDER, not absence -- see the
Refutation below.

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sample",
    "--json",
    "--assert",
    "readonly 0x404028+8",
    "--assert",
    "volatile 0x50000000+4"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/assertranges_x86_64",
    "binary_sha256": "1c9ad3b67437c7e85ac53d0755609c5b5e015225796a98c0287de5f2cb3d382e",
    "binary_size": 15800,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/assertranges_x86_64",
    "selector": "sample",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "assertions",
        "op": "len_eq",
        "value": 2
      },
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "assertions[0].subphase",
        "op": "eq",
        "value": "code-data-partition"
      },
      {
        "path": "assertions[1].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "assertions[1].subphase",
        "op": "eq",
        "value": "code-data-partition"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "* 7"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "+ 100"
      },
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "scale"
      },
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "bias"
      },
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "dat_50000000 * 2"
      }
    ]
  },
  "notes": "Desired: two range facts an agent knows and kuna cannot derive, on one `kuna decompile --json`. scale/bias are .data (writable, so never loader-readonly) and nothing writes them: asserted readonly they fold to `* 7` / `+ 100`. 0x50000000 is device memory, so kuna CSEs its two reads into one; asserted volatile keeps both. FAILS at ed353fb2: exit 2, unknown directive. In-repo fixture.",
  "probe_id": "a-567d3207ce62"
}
```

## Hypothesis

ADVISORY. The cheap half is exposure, not implementation: most of these commands already work and only lack a path from the `kuna` binary. The expensive half is the stubs. A builder should measure which is which before choosing a design, and should NOT assume a `kuna console` passthrough is the right shape -- a scriptable console is a different product from a set of flags an agent can compose.

## Refutation

OVERTURNED on both halves, measured at `ed353fb2` and again on the Stage B branch.

- **"The cheap half is exposure."** Half right. `readonly` and `volatile` do work
  in the console, but only if they are stated BEFORE `read symbols`: `Scope::addMap`
  folds the range property into each `SymbolEntry` as it maps it
  (`database.cc:1156-1158`) and never consults the range again, so painting a
  range after the loader's symbols is silently inert over every address the loader
  named -- which is every address a caller is likely to name. That, not absence of
  effect, is why the `--assert` proposal could not find a fixture where `readonly`
  did anything (its section 6). The generated console script therefore emits these
  before `read symbols`, and the in-process surface re-applies the property to the
  symbols the range covers.
- **"The expensive half is the stubs."** Wrong, again. `global add`/`global remove`
  name `ScopeGlobal` range add/remove, and `Database::add_range`/`remove_range` have
  been ported and public in `p0_knowledge/database.rs` the whole time; the two
  commands are ~15 lines of plumbing each.
- **A third thing the record did not know.** Wiring them shows `global add` is
  measurably a NO-OP on any ordinary ELF: every stock cspec's `<global>` claims the
  whole default data space (`<range space="ram"/>`), so the range is global before
  a caller says anything. `global remove` is the direction that moves the C (it
  kills a store that survives a call). So there is no `global` directive on the
  `--assert` plane -- shipping one would be exactly the accepted-and-inert failure
  mode this family exists to stop.
- **"Marking a gap as code is unavailable."** The Symptom's framing of `global add`
  is wrong: it adds a range to the global DATA scope, not to code. The lever for
  code is `--define-function` / `function bounds`, shipped by #374.

## Decision log

- seeded for round 2 from a source survey of the override surface, after round 1 showed testers hitting obfuscated images with no lever to correct kuna with. Not tester-filed: round 2 should confirm the demand.
- round 2 T_TRIAGE (captain): track tooling / touches [kuna-cli/src, kuna-console/src/ifacedecomp.rs] / scope large CONFIRMED. Its touches are IDENTICAL to no-cli-function-boundary-override's, so the two are not co-schedulable no matter what select's lease algebra says (both are bare cluster: keys); it waits on that need's proposal to define the delivery vehicle. Round 2 did produce the independent tester demand it was missing: a three-sighting 'kuna will not tell me about data' family (653d8860 x2 + 69a3822f), recorded at T_DEDUP.
- round 2 BUILD (b-r2-no-cli-data-code, Stage B of the `--assert` proposal): SHIPPED. `readonly <addr>+<size>` and `volatile <addr>+<size>` on the `--assert` plane at P1 code-data-partition, and `global add`/`global remove` wired onto `Database::add_range`/`remove_range` (both stubs' engine entries had been ported all along). Acceptance `a-567d3207ce62` FILED -- it did not exist, so this need was unclosable -- and promoted to `tests/cli/`. Every named command in the Symptom is now reachable or accounted for: `map address` and `parse line` shipped as Stage A's `data`/`typedef`, `readonly`/`volatile` ship here, `type varnode`'s local form is Stage A's `type`, and the `global` pair is wired but deliberately NOT given a directive (measurement, see the Refutation). The Symptom's "marking a gap as code" is `--define-function` (#374), not `global add`. Residue: the storage-keyed `type varnode [space,offset,size]` spelling, `global spaces`/`global registers` (genuine ports), and phases.toml's exposure prose (lease held elsewhere this wave).
- closed: acceptance a-567d3207ce62 now PASSES at c40e02fa7931
- round 2 B_DONE (captain): CLOSED, mechanically. Same suite/sha as its Stage-A sibling
  (c40e02fa, gates green): acceptance a-567d3207ce62 PASSES (11 clauses, 153 ms, one run, not
  flaky) where it FAILED at ed353fb2 with `unknown directive "readonly"`. pr/closing_pr 391
  (merged 20:10:15Z, mergeCommit c40e02fa) set by hand; attempts 0 -> 1 (one wave-11 dispatch,
  merged first try). covered_by_option null -- tooling track, no settable row. Promotion
  re-run, byte-identical, already in the 21/21 clitests. The two OVERTURNED findings recorded
  in the Refutation are the durable lesson and are indexed in captain memory: a range property
  must be painted BEFORE `read symbols` or it is silently inert over every loader-named
  address, and `global add` is a measured no-op because every stock cspec's <global> already
  claims the whole ram space -- which is why no `global` directive shipped.
