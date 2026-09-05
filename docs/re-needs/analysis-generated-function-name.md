---
need_id: analysis-generated-function-name
title: Analysis-generated function name cannot be used by decompile
track: tooling
status: closed
severity: major
probe_id: p-4c57e2d76dcd
acceptance_id: a-022969ccc61d
hypothesis_status: upheld
credibility: 0.85
instances: 2
challenges: [5ab77f5533c5d40ad448c1ea]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/decompile.rs]
scope: small
regression_of: null
pr: null
closed_in_round: 2
closing_pr: "377"
reject_reason: null
---

## Symptom

Decompile the checker by the exact owning-function name returned by kuna strings and functions

> **Analysis-generated function name cannot be used by decompile** (major, `5ab77f5533c5d40ad448c1ea`)
> kuna strings assigned the relevant strings to sub_4045b0, but kuna decompile rejected sub_4045b0 with exit 1. The address plus --addr succeeded.

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
    "sub_4045b0"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "no function.*sub_4045b0"
    ]
  },
  "target": {
    "binary_rel": "bin/GiveMeMoney.zip.__x/KeyVal2.exe",
    "binary_sha256": "dae523033740dd11e0a4a3cdbe9abdc209e660169e89771e2bdbd2cd5d9b8f42",
    "binary_size": 558592,
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
    "sub_4045b0"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "sub_4045b0"
    ]
  },
  "target": {
    "binary_rel": "bin/GiveMeMoney.zip.__x/KeyVal2.exe",
    "binary_sha256": "dae523033740dd11e0a4a3cdbe9abdc209e660169e89771e2bdbd2cd5d9b8f42",
    "binary_size": 558592,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The single-function name resolver uses a narrower symbol table than analysis-generated names exposed by inventory and string ownership.

## Refutation

**SYMPTOM UPHELD, mechanism corrected, track/touches wrong (captain, round 2 T_REFUTE).**

- Reproduced exactly: `functions --json` lists `sub_4045b0` (address 0x4045b0, size 2391) among 2044 entries, while `decompile <bin> sub_4045b0` exits 1 with `no function "sub_4045b0" ... for a stripped binary pass an address with --addr` (`decompiler/crates/kuna-cli/src/decompile.rs:349`, mapping the console's `Unknown function name:`).
- **There is no "narrower symbol table".** `sub_<hex>` is a *synthesized display name*: the inventory renders it from the entry address and it is never registered as a symbol anywhere, so no table on the decompile path could contain it. The fix shape is therefore selector resolution in `kuna-cli` — fold an inventory-shaped name back to its entry address — not symbol-table widening in `kuna-decomp`.
- **Consequence for T_TRIAGE:** the filed `track: quality` and `touches: [decompiler/crates/kuna-decomp]` are both wrong; this is `tooling` in `decompiler/crates/kuna-cli`. It also stops colliding with the option-adding counter leases, which changes what it can be co-scheduled with.
- **Wrong-output risk:** low. The one hazard is accepting a `sub_<hex>` that is not a discovered entry and silently decompiling from mid-function, so the resolver must require an exact entry match and report ambiguity rather than guess.

## Reference

_none recorded_

## Instances

- `5ab77f5533c5d40ad448c1ea` (round 2, tester t-r2-5ab77f55)
- `5bd1d1bb33c5d4110a29b31e` (round 2, tester t-r2-5bd1d1bb) -- twin sighting, NOT machine-gated: 'a function listed in auto mode cannot be selected by its displayed name in reliable mode'

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): instances hand-set 1 -> 2 and credibility 0.7 -> 0.85. The twin sighting (5bd1d1bb obs0, ELF x86, a different tester and a different mode) gated `unrunnable` on the probe JSON-escape defect, not on its merits, so select would have ranked this two-tester/two-format/two-mode gap as a single opinion. Two testers independently hit 'kuna prints a name it will not then accept'.
- round 2 T_REFUTE (captain): symptom **UPHELD**, mechanism corrected -- there is no narrower symbol table; `sub_<hex>` is a synthesized display name that is never a symbol at all, so the fix is selector resolution in `kuna-cli`, and the filed `track: quality` / `touches: [decompiler/crates/kuna-decomp]` are wrong (should be `tooling` / `kuna-cli`). T_TRIAGE must correct both.
- round 2 T_TRIAGE (captain): track quality -> TOOLING and touches kuna-decomp -> kuna-cli/src/decompile.rs, applying the T_REFUTE correction. `sub_<hex>` is a synthesized display name that is never registered as a symbol anywhere, so there is no symbol table in kuna-decomp to widen; the fix is selector resolution in the CLI (decompile.rs:349 is where the console's `Unknown function name` becomes exit 1). Operational consequence: as `tooling` this need no longer contends for counter:catalog / counter:div / counter:stages-corpus / phases.toml / docs/options.md, so it is co-schedulable with the one option-adding quality builder -- which matters because it is the round's strongest tester-filed gap (2 instances, 2 testers, 2 formats, 2 modes). scope stays small: fold an inventory-shaped name back to its entry address, requiring an EXACT entry match and reporting ambiguity rather than guessing. If the builder finds it needs a phases.toml option, STOP -- a tooling contract does not hold the counter leases.
- round 2 B_BUILD (builder b-r2-analysis-generat): symptom **UPHELD**, the T_REFUTE mechanism **OVERTURNED**. `sub_4045b0` IS a `FunctionEntry` name in `function_entries_canonical` and `resolve_entry(Name(..))` resolves it -- `decompile --json`, `disassemble` and `decompile-all` all select the witness by that exact name today. The text surface fails because that SESSION never discovered the entry: `kuna-cli`'s in-process drivers inject the DIV-20/DIV-68 non-x86-64 discovery bundle (`listing` + `funcstart_patterns` + `aif`) in `load_program`, and `build_script` injected only `option listing on`. Bisected on the witness: `listing` alone still answers `no function matches`; `listing` + `funcstart_patterns` selects it. So the fix is not name folding but reaching the same inventory. Plain alignment was implemented first and REJECTED by its own collateral sweep: injecting the bundle up front changes the entry set, and on i386 and PPC64 the prologue matcher seeds a start a few bytes inside a function it already knew (PPC64 ELFv2's local entry point, 8 bytes past the global one), after which `funcboundflow` truncates the outer function -- `plt_ppc64le __do_global_ctors_aux` went from a correct loop to an empty husk, 8 such truncations over 33 non-x86-64 fixtures. Shipped instead as a SECOND ATTEMPT: the by-name miss, and only the miss, is retried once with the bundle, so every invocation that already worked is byte-identical and the husk trade stays the whole-binary surfaces' to make. The husk itself is a live non-x86-64 discovery defect visible today in `kuna functions` / `kuna decompile-all`; it is NOT closed by this PR and is worth a need of its own. The mode policy is why it looked like two bugs: `auto` picks `aggressive` under 500 KiB and that preset names all three options itself, so the gap shows only above the threshold (KeyVal2.exe is 558,592 bytes -> `reliable`) or under an explicit `--mode reliable` -- which is precisely the twin sighting `5bd1d1bb`, same root cause.
- closed: acceptance a-022969ccc61d now PASSES at 6d315ffe5836
