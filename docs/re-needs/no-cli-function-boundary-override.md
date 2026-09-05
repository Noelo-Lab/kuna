---
need_id: no-cli-function-boundary-override
title: an agent cannot tell kuna where a function starts or ends
track: tooling
status: closed
severity: blocker
probe_id: p-83ce32ba278c
acceptance_id: a-88c4db106ade
hypothesis_status: upheld
credibility: 1.0
instances: 1
rounds: [2]
first_seen_round: 2
attempts: 0
touches: [decompiler/crates/kuna-cli/src, decompiler/crates/kuna-console/src/ifacedecomp.rs]
scope: large
closed_in_round: 2
closing_pr: "374"
---

## Symptom

The console has `map function <addr> [name] [nocode]` and `load addr <addr> [name]`, both functional. The `kuna` binary can emit neither: `decompile.rs::build_script` produces a fixed vocabulary (load file / option / read symbols / load function|addr / kassert / decompile / print) and there is no `kuna console`, no `--script`, and no passthrough. Worse, `function F spans [start,end)` does not exist ANYWHERE -- extent is derived in `kuna-console/src/funcextent.rs` as [entry, min(next_entry, section_end)) with no override, and `phases.toml` has no subphase for function-entry identification at all. On an obfuscated or packed image, where discovery is exactly what fails, the agent has no lever. Round 1 hit this three times: `kuna functions` returning count 0, a phantom entry at 0xfe6dca9f, and `disassemble main` on an image whose main was never discovered.

## Reproduction

Every named CLI flag is rejected with `error: unknown option`, verified on the round-1 merge build; the stubs are verified by driving `decomp_dbg` directly and reading back `engine integration not yet ported`.

## Acceptance

Asserts the DESIRED behaviour on the same command: `--define-function
<start[-end][=name] | @file>` declares an entry discovery missed AND bounds its
flow follow, so the extent kuna reports is the declared one and the body stops at
the declared end instead of swallowing its neighbours.

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-88c4db106ade",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--json",
    "--functions",
    "stage1",
    "--define-function",
    "0x13c9-0x1420=stage1"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "binary_sha256": "1a592a85f424cc2db8953d5a38c86676bcee5e37b242b6fb6244a2c9fccfeeef",
    "binary_size": 14408,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "selector": "0x13c9",
    "selector_kind": "addr"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "functions[0].name",
        "op": "eq",
        "value": "stage1"
      },
      {
        "path": "functions[0].address",
        "op": "eq",
        "value": 5065
      },
      {
        "path": "functions[0].size",
        "op": "eq",
        "value": 87
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "stage1"
      },
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "sub_1393"
      }
    ]
  },
  "notes": "Desired: --define-function <start[-end][=name]|@file> declares both halves of a boundary. start names an entry discovery missed; end is exclusive and bounds the flow follow, so the extent is the declared 87 bytes and the body stops before the callees past 0x1420 (sub_1393 is the last of the 25 calls the unbounded 682-byte follow swallows). In-repo fixture, so it promotes verbatim."
}
```

## Hypothesis

ADVISORY. The cheap half is exposure, not implementation: most of these commands already work and only lack a path from the `kuna` binary. The expensive half is the stubs. A builder should measure which is which before choosing a design, and should NOT assume a `kuna console` passthrough is the right shape -- a scriptable console is a different product from a set of flags an agent can compose.

## Decision log

- seeded for round 2 from a source survey of the override surface, after round 1 showed testers hitting obfuscated images with no lever to correct kuna with. Not tester-filed: round 2 should confirm the demand.
- round 2 T_TRIAGE (captain): track tooling / touches [kuna-cli/src, kuna-console/src/ifacedecomp.rs] / scope large CONFIRMED as filed; it was already dispatched this round and its builder died on the account 429 with no commits, so nothing here is re-litigated. What T_TRIAGE adds is the demand evidence the B_PLAN note said was missing: this need was captain-SEEDED with instances 1, and round 2's testers have now produced an independent witness -- overlapping-anti-disassembly-sequence records `kuna decompile ... 0x80489e6 --addr` exiting 1 at internal target 0x8048c49, a tester hitting exactly this wall while working a different challenge. The 'defer rather than approve' caution standing over its [PROPOSAL] is therefore weaker than when it was written.
- round 2, builder: the hypothesis is upheld on the START half and OVERTURNED on the END half. Declaring a start really was pure exposure -- `map function` works, and `kuna decompile --addr` already reached an undiscovered entry. But "the expensive half is the stubs" is wrong: no stub was involved. The END half was inert PLUMBING. `Funcdata::size` is threaded from `map function`/`load addr`/`decompile_one` all the way to `Architecture::new_funcdata`, `FlowInfo` has a fully ported `set_range`/`new_address`/`handle_out_of_bounds` range machinery, and the two ends were simply never joined: every call site passed `UNBOUNDED_SIZE` and `follow_flow_on_fd` never called `set_range`. Closing it took ~10 lines in `decompile_drive.rs`, not an engine port.
- shipped `--define-function <start[-end][=name] | @file>` on `decompile`, `decompile-all`, `functions`, `decompile-project` and `disassemble`, over one new kuna-only console command (`function bounds <start> [<end>] [as <name>]`) and one `ConsoleProgram::declared_extents` store consulted by every later load of that entry. NOT closed: `xrefs` and `strings` load through the same `Args` but do not accept the flag (they pass an empty declaration list), and durability is caller-carried (the `@file` is the artifact; kuna does not write boundaries back into the image).
- a wrong declared end used to produce a silently EMPTY body: `FlowInfo::handle_out_of_bounds` computed the C++ "Function flow out of bounds" message and dropped it on the floor (a W4 stub), so the one failure mode the new flag introduces was invisible. Un-stubbed to the two `Funcdata::warning`/`warning_header` calls the C++ makes; the flow range is the whole entry-point space unless an extent is declared, so this fires only under a declared boundary and is inert everywhere else (measured: 69/69 real-ELF fixtures byte-identical to main).
- deliberately out of scope, and left to `no-cli-rename-or-prototype-override`: renaming an entry is only supported here as the `=NAME` half of a boundary declaration.
- closed: acceptance a-88c4db106ade now PASSES at 751a960de2fd

## Probe

Asserts the CURRENT bad behaviour: every named boundary flag is rejected, so the
`kuna` binary has no lever at all. PASSes on the round-1 merge build (e5ac9c77).

```json
{
  "schema": "re-probe/1",
  "probe_id": "p-83ce32ba278c",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--json",
    "--functions",
    "stage1",
    "--define-function",
    "0x13c9-0x1420=stage1"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "binary_sha256": "1a592a85f424cc2db8953d5a38c86676bcee5e37b242b6fb6244a2c9fccfeeef",
    "binary_size": 14408,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "selector": "0x13c9",
    "selector_kind": "addr"
  },
  "expect": {
    "exit_code": {
      "ne": 0
    },
    "stderr_matches": [
      "unknown option --define-function"
    ]
  },
  "notes": "Current: no kuna surface can declare where a function starts or ends -- every named flag is rejected. The console has map function / load addr; the kuna binary can emit neither, and 'function F spans [start,end)' is not expressible anywhere in the engine."
}
```
