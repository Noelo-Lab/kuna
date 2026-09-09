---
need_id: changing-namestyle-invalidates-discovered
title: Changing namestyle invalidates a discovered function name
track: tooling
status: open
severity: minor
probe_id: p-e0fd907be6d0
acceptance_id: a-2f69533e66cc
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [6927c8d12d267f28f69b8131]
rounds: [11]
first_seen_round: 11
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/decompile.rs, decompiler/crates/kuna-decomp/src/p9_emit/kuna_naming.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Decompile a name returned by functions while changing the emitted naming convention.

> **Changing namestyle invalidates a discovered function name** (minor, `6927c8d12d267f28f69b8131`)
> Default decompile accepts sub_15dc. Adding namestyle ghidra exits 1 with no function. Selecting 0x15dc --addr succeeds.

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
    "sub_15dc",
    "--option",
    "namestyle",
    "ghidra"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "no function"
    ]
  },
  "target": {
    "binary_rel": "bin/0xjam3z-evolvingsbox.zip.__x/0xJam3z-Medium",
    "binary_sha256": "c1e78f2a0ba7a15d9bf6a563f0874656437112b617b94d42cd858e3e477ef665",
    "binary_size": 14440,
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
    "sub_15dc",
    "--option",
    "namestyle",
    "ghidra"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "Correct!"
    ]
  },
  "target": {
    "binary_rel": "bin/0xjam3z-evolvingsbox.zip.__x/0xJam3z-Medium",
    "binary_sha256": "c1e78f2a0ba7a15d9bf6a563f0874656437112b617b94d42cd858e3e477ef665",
    "binary_size": 14440,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Selector resolution may occur after presentation names change without retaining discovered aliases.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6927c8d12d267f28f69b8131` (round 11, tester t-r11-6927c8d1)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 REFUTER: hypothesis **upheld** (was inconclusive). Refuted in-tick by the captain (round 11). VERDICT UPHELD, mechanism pinned, and one piece of BOOKKEEPING IS WRONG.

SYMPTOM REPRODUCED exactly on the need's own arena binary
(.kuna-repipe/arena/11/6927c8d12d267f28f69b8131/target/0xjam3z-evolvingsbox.zip.__x/0xJam3z-Medium):
default `kuna decompile BIN sub_15dc` = rc 0, full body; adding `--option namestyle ghidra` = rc 1,
'no function "sub_15dc" ... pass an address with --addr'.

MECHANISM: this is NOT a broken selector, it is a name-space swap. Under namestyle ghidra the same
function resolves BY ITS OTHER NAME: `kuna decompile BIN func_0x000015dc --option namestyle ghidra`
returns rc 0 and the full body. So lookup works; the synthesized name is simply regenerated in the
upstream style (func_0x%08x, see architecture.rs and tests/verify_w4_fw_architecture.rs:79) and the
default-style name sub_15dc is not retained as an alias. The tester's filed hypothesis is upheld.
Do NOT guess Ghidra's own prefix: `FUN_000015dc` is rc 1 as well; kuna's upstream style is func_0x.

THE TWO SURFACES DISAGREE BY CONSTRUCTION: `kuna functions --json` ignores namestyle and always
reports {"name": "sub_15dc", "aliases": []}. That empty `aliases` field already exists in the schema
and is the natural carrier for a fix.

ACCEPTANCE VERIFIED SOUND (recipe check (v), by control rather than by simulation):
`kuna decompile BIN 0x15dc --addr --option namestyle ghidra` gives rc 0 with "Correct!" present
once in stdout -- exactly the shape the acceptance demands. A correct fix flips it. Contrast
live-simd-string-copies, whose acceptance cannot be flipped.

*** regression_of: analysis-generated-function-name IS FALSE AND IS INFLATING THE RANK. *** That
round-2 need was closed by PR #377 and its acceptance still PASSES on current main (measured this
tick: verify --acceptance-suite --need analysis-generated-function-name -> passed true), and the
default-namestyle case works on this binary too. This is an uncovered case, not a regression -- the
known title-similarity trap. The link is what scores this need 1055.5 against 43.9 for the runner-up,
i.e. it is currently #1 in the build queue on a false premise. Clear regression_of at T_TRIAGE and let
it re-rank (it lands near 13.9, alongside the other single-instance needs).

TOUCHES IS UNPROVEN: filed as decompiler/crates/kuna-cli. The name lookup happens inside the forked
decomp_dbg -- kuna-cli only translates the console's unknown-function output into that message at
kuna-cli/src/decompile.rs:505. A CLI-tier fix (resolve the name to an address against the analysis
function list before forking) and a decomp-tier fix (keep the default-style name as an alias) are both
plausible. Confirm at T_TRIAGE before the lease algebra aims a builder at one tree.
- round 11 TRIAGE (captain): cleared regression_of=analysis-generated-function-name -- the refuter measured that need's acceptance still PASSING on main, so the link was title-similarity, not a regression; it was scoring this need 1055.5 vs 43.9 for the runner-up. touches was filed kuna-cli alone and the refuter left it unproven; recorded BOTH candidate seams, since kuna-analysis names the function at load and namestyle is applied AFTER the load (kuna_entrymainproto.rs:84), so the rename that breaks the lookup happens in kuna-decomp while the error text is produced at kuna-cli/src/decompile.rs:505. touches carries no lease (select.PATH_RESOURCES leases only phases.toml/options.rs/tests-stages/history.md), so naming two seams costs no scheduling breadth. track stays tooling: the fix may land in kuna-decomp but the defect is a CLI round-trip gap, not emitted C.
- round 11 BUILDER (b-r11-changing-namesty): hypothesis UPHELD and the refuter's mechanism confirmed
  exactly. Not a lost alias and not a broken selector: PR #499 (RE-need `string-owner-function-name`)
  already retries a by-name MISS as the address the name spells, and decides by MINTING -- it rendered
  the candidate offset through `self.arch().name_function(addr)` and accepted it only on an exact
  match, which tied the retry to the style the run happens to have ACTIVE. `option namestyle ghidra`
  clears `name_style_angr`, so `sub_15dc` stopped being a name this build would mint and the two
  styles became disjoint name spaces. Closed in `ConsoleProgram::placeholder_name_address` (kuna-console,
  not kuna-cli: kuna-cli only translates the console's miss into its error text at decompile.rs:505,
  and a fix there would have left `disassemble`/`xrefs`/`decompile-all --functions` refusing the same
  name) by rendering the candidate in ALL THREE styles -- `minted_function_names`, one call per
  `KunaNameStyle` arm, the same three functions `name_function` dispatches between. Both pre-existing
  gates are untouched, so the change can only turn a MISS into a hit. The ACCEPTANCE IS UNCHANGED and
  still measures 0xJam3z-Medium (PASS on this branch); the promoted
  `tests/cli/changing-namestyle-invalidates-discovered.json` is retargeted onto the vendored
  `stripped_dynamic_x86_64`, where `sub_1249 --option namestyle ghidra` answered `no function` pre-fix
  -- the same defect, on a binary CI has. No option, no counters, no stages case, no DIV row.
