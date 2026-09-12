---
need_id: named-log-prototype-repair
title: Named log prototype does not repair the call through its PE thunk
track: quality
status: closed
severity: major
probe_id: p-eaee43130b01
acceptance_id: a-43296fdf37fe
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [69a54bd70f5b9757a6a5f72f]
rounds: [9, 12]
first_seen_round: 9
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-console]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/581
closed_in_round: 12
closing_pr: 581
reject_reason: null
---

## Symptom

Recover the double-precision log input and result using the documented named prototype override.

> **Named log prototype does not repair the call through its PE thunk** (major, `69a54bd70f5b9757a6a5f72f`)
> Exits 0 but emits log(SUB84(...,0)); and then uses unassigned XMM0 halves. The inventory names both the thunk and import slot log. Applying the identical prototype to thunk address 0x14000904f repairs the call. Enabling calleearity and varargstackargs does not.

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
    "sub_140003150",
    "--assert",
    "prototype log double log(double x)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "log\\(SUB84\\("
    ]
  },
  "target": {
    "binary_rel": "bin/Release.zip.__x/PolyMLP.exe",
    "binary_sha256": "01652e8b03e2bec127f5726320db4c9925d495d75583a8bb4534ac640f6ec230",
    "binary_size": 54272,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/Release.zip.__x/PolyMLP.exe",
    "binary_sha256": "01652e8b03e2bec127f5726320db4c9925d495d75583a8bb4534ac640f6ec230",
    "binary_size": 54272,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_140003150",
    "--assert",
    "prototype log double log(double x)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?m)^\\s*[A-Za-z_][A-Za-z0-9_]*(?:\\[[^\\]\\n]+\\])?\\s*=\\s*[^;\\n]*\\blog\\(\\s*[^\\s);][^;\\n]*\\)[^;\\n]*;"
    ],
    "stdout_absent": [
      "log\\(SUB84\\("
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The named override may bind only to the import slot without propagating to the identically named thunk.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/Release.zip.__x/PolyMLP.exe` — Reference unavailable: server exited with status 1 before registering. No successful IDA comparison.

## Instances

- `69a54bd70f5b9757a6a5f72f` (round 9, tester t-r9-69a54bd7)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, with the mechanism pinned by a single-variable control and one correction to the wording. Three runs on the arena binary (.kuna-repipe/arena/9/69a54bd70f5b9757a6a5f72f/target/Release.zip.__x/PolyMLP.exe), release kuna, function sub_140003150, only the assertion changing: (A) no assert -> 'log(SUB84(*(double *)((long long)v38 + (long long)v8 * 8) + v6,0));' [the bug]; (B) --assert 'prototype log double log(double x)' -> BYTE-IDENTICAL to (A), same 179 lines, same call line; (C) --assert 'prototype 0x14000904f double log(double x)' -> 'v3 = v5 + log(v3 + v6);' [correct, and the return value is now consumed]. So the by-name form is wholly inert while the by-address form on the same function repairs the call. WHICH SYMBOL IT LANDS ON, PROVEN: 'kuna functions --json' shows TWO FunctionSymbols named log -- the 33-byte thunk at 0x14000904f that every call targets, and the IAT slot at 0x14000a2d8 with size 0. Run (D) --assert 'prototype 0x14000a2d8 double log(double x)' reproduces (B) exactly: status applied, output identical to default. The name form resolves to the slot. CORRECTION TO THE FILED WORDING: 'binds only to the import slot without propagating to the thunk' is right about the destination but wrong to frame it as a propagation gap -- there is nothing to propagate. The read side (ActionDefaultParams -> callee_proto_pieces(entry_addr)) is keyed by the CALLEE ENTRY ADDRESS, and the call targets the thunk's entry, so a prototype parked on the slot is simply never consulted. SILENT-FAILURE HALF, WHICH THE FILING MISSES ENTIRELY: both (B) and (D) report status 'applied' in the --json assertions array with detail null. kuna tells the agent the override took effect when it changed nothing, so the tester's next three ablations (calleearity, varargstackargs) were spent on the wrong plane. Whatever the fix is, the assertion report must stop saying 'applied' for a prototype that reaches no call site. NOT A NEW GAP -- this is the residual of accepted-sqrt-prototype-still (closed by PR #470, round 3): #470 fixed the 0x-prefixed form (name first, entry address second, reject a 0x operand that starts no function) and left the by-NAME form still answering from the global scope, which on a PE import is the IAT slot. Same binary shape, different import. The builder should read assertions::resolve_proto_target and Architecture::set_function_prototype_pieces(name)/queryFunction(name); the address-keyed twin set_function_prototype_pieces_at already exists and is what DWARF/cppsig use. Obvious disambiguator: when a name matches more than one FunctionSymbol, prefer the entry that starts a body (thunk size 33 vs slot size 0), or apply to every match. Note this also reaches the library-prototype bootstrap pass (engine.rs step 5), which parks memcpy/malloc/log the same way. ACCEPTANCE IS WEAK, TIGHTEN BEFORE DISPATCH: it requires 'log\\(' present and 'log\\(SUB84\\(' absent, so an argumentless 'log()' -- the classic shape when a declared prototype reaches a call but argument recovery does not -- passes it. Require a non-empty argument, e.g. 'log\\([^)]' alongside the SUB84 absence.

### 2026-09-08T06:05Z — captain, round 9 B_DRAIN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate rather
than failing, and therefore unclosable by B_DONE however good a fix was. Bound to the
challenge's own image (challenge `69a54bd7`, one image, no ambiguity):

    bin/Release.zip.__x/PolyMLP.exe (54272 B, sha256 01652e8b…)

Re-run through `verify --acceptance-suite --need named-log-prototype-repair` on the current build it now RUNS
and FAILS, on the clause that carries the filed symptom:

    stdout_absent[0] → actual "log(SUB84("

Nothing about the probe's command or expectations was changed — only the target it
resolves against.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.
- round 12 BUILDER: named prototype assertions now resolve through `ConsoleProgram::resolve_entry`: a same-named IAT slot and lone executable thunk canonicalize to the thunk address, while two executable definitions reject as ambiguous and a genuinely unresolved name remains pending. Hardened the dataset acceptance from mere `log(` presence to a rename-tolerant complete assignment containing `log(nonempty arguments);`, while retaining the exact `log(SUB84(` absence guard. The command and dataset target are unchanged; the canonical acceptance id is `a-43296fdf37fe`. The in-repo promoted twin is `tests/cli/named-log-prototype-repair.json` (`a-20918fc99877`).
- closed: acceptance a-43296fdf37fe now PASSES at 151300280020; PR #581.
