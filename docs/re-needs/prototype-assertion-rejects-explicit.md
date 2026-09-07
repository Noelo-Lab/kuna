---
need_id: prototype-assertion-rejects-explicit
title: Prototype assertion rejects the explicit decompilation target
track: tooling
status: open
severity: major
probe_id: p-8450e40778a3
acceptance_id: a-df3bacaa5f17
hypothesis_status: overturned
credibility: 0.7
instances: 2
challenges: [64f1f7fad931496abf909535, 5ab77f5d33c5d40ad448c6f6]
rounds: [5, 6]
first_seen_round: 5
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/484
closed_in_round: null
closing_pr: https://github.com/Noelo-Lab/kuna/pull/484
reject_reason: null
---

## Symptom

Apply an address-based prototype to the function selected in the same invocation.

> **Prototype assertion rejects the explicit decompilation target** (major, `64f1f7fad931496abf909535`)
> Emits the function at 0x401571 but rejects its prototype with 'no function starts at 0x401571'. Adding --define-function for the same address makes the assertion succeed.

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
    "0x401571",
    "--assert-strict",
    "--assert",
    "prototype 0x401571 void decrypt(unsigned int key, unsigned int start, unsigned int end)"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "rejected: no function starts at 0x401571"
    ]
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
    "0x401571",
    "--assert-strict",
    "--assert",
    "prototype 0x401571 void decrypt(unsigned int key, unsigned int start, unsigned int end)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stderr_absent": [
      "rejected: no function starts"
    ],
    "stdout_matches": [
      "&key"
    ]
  },
  "target": {
    "binary_rel": "bin/illusion.exe",
    "binary_sha256": "8093e4a899faa8283b1573f571834fffbee872c7e40da384bf36e6edb9b88a8e",
    "binary_size": 1292288,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Assertions may resolve before the explicitly selected function is registered.

## Refutation

Round 6 BUILDER: the round-5 refutation is CONFIRMED and is what shipped. Registration,
not ordering, is the missing step -- measured directly in `decomp_dbg`: with a declaration
in place both `function bounds` / `load addr` / `map prototype` and `function bounds` /
`map prototype` / `load addr` produce the identical `void sub_401571(uint4 key,uint4
start,uint4 end)`, and with NO declaration the directive is rejected in both orders,
including after `load addr` has already built and followed the function. One correction to
the refuter's account of WHY the by-name run works: it is not that resolving a synthesized
name installs the symbol, it is that `load function sub_401571` MISSES on the first attempt
and `kuna decompile`'s own retry with the wider `funcstart_patterns`+`aif` discovery bundle
then finds 0x401571 as a real entry. A by-address run never misses, so it never retries.

## Reference

_none recorded_

## Instances

- `64f1f7fad931496abf909535` (round 5, tester t-r5-64f1f7fa)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 5 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED -- the symptom stands, the ordering framing does not, and a 3-way A/B localizes the defect to something the filing never names: WHICH FORM THE TARGET WAS SELECTED IN. Symptom reproduced exactly on illusion.exe: kuna decompile <bin> 0x401571 --assert-strict --assert 'prototype 0x401571 void decrypt(...)' -> exit 1, 'rejected: no function starts at 0x401571', and the function is nonetheless emitted in full. THE A/B THAT SETTLES IT -- three runs, same binary, same assertion text, only the SELECTOR and the ASSERTION OPERAND vary. target 0x401571 + assert 'prototype 0x401571 ...' -> REJECTED, exit 1. target 0x401571 + assert 'prototype sub_401571 ...' -> BINDS, exit 0, void sub_401571(uint4 key,uint4 start,uint4 end). target sub_401571 + assert 'prototype 0x401571 ...' -> BINDS, exit 0, same signature. So the ADDRESS form of the assertion is not broken -- it works fine the moment the target was selected by name. The defect is that selecting the target BY ADDRESS leaves no FunctionSymbol at that address for the resolver to find. WHY 'RESOLVE LATER' IS THE WRONG LEVER. The filed hypothesis says assertions resolve BEFORE the function is registered, which presumes the failing run registers it eventually. Evidence says otherwise: in the by-name-selected run the symbol is there at resolution time, and in the by-address-selected run nothing installs one at all. This is a missing registration on one selection path, not a race between two ordered steps. A builder who goes hunting for a phase-ordering fix will find no ordering to move. THE SEAM, by name. kuna-console/src/assertions.rs:463 resolve_proto_target(). It tries symboltab.query_function_by_name() first, then for an 0x-prefixed operand code_addr() + symboltab.function_display_name_across_scopes(&addr), and returns Err('no function starts at {func}') when that display-name lookup comes back None. The lookup is against the DECOMPILER's symboltab, which is not the analysis tier's discovered-function list -- kuna functions illusion.exe --json reports sub_401571 at 0x401571 with size 475 among 1826 functions, so 'the function was never found' is false at the analysis tier and true at the symboltab the resolver reads. sub_401571 is a SYNTHESIZED name (no real symbol in the image), so the by-name selection path is resolving a synthesized name through the analysis tier AND installing the FunctionSymbol as a side effect; the by-address path skips that install. That asymmetry is the bug. --define-function 0x401571 works for the same reason: it is the explicit form of the install the by-address path is missing. GUARD THE FIX MUST NOT BREAK. Making an unresolvable 0x operand park silently is NOT acceptable -- kuna-cli/tests/decompile_cli.rs:765 and kuna-console/tests/verify_assertplane.rs:598 both assert 'no function starts at 0x999999' for a genuinely bogus address, and the doc comment above resolve_proto_target states the rule deliberately ('0x... is not a C identifier, so such a directive is provably inert and the caller deserves to hear it'). The fix has to make the CLI's by-address selection install the entry, not make the resolver lenient. RELATED, ALREADY-SHIPPED PRECEDENT worth reading before writing code: commit 38c461d1 (accepted-sqrt-prototype-still, 'a prototype assertion at an entry address binds') and 3e02a0dd (qualified-parameter-assertions-modify) are the same plane and the same file; this is the third member of that family and the first one keyed to the SELECTOR rather than to the operand.
round 5 TRIAGE (captain): CONFIRMED AS FILED -- tooling / kuna-cli / small / major. DISPATCH CONSTRAINT: b-r5-rejected-flow-ov is live in the same assertion plane; do not dispatch this until that branch is merged or read.
round 6 BUILDER: CLOSED. `kuna decompile`'s generated script now declares the entry a
by-address run selected (`function bounds <vma>`), between the caller's own
`--define-function` lines and the program-scoped directives, and skipped when a
`--define-function` already names that start. `resolve_proto_target` is untouched, so a
`0x` operand naming any other unregistered address is still rejected. Two latent
`declare_function` defects surfaced by the fixture sweep were fixed with it: an import's
`UndefinedExternal` provenance survives a declaration, and the ARM/Thumb mode bit is folded
out of the declared address. Acceptance PASS; promoted to
`tests/cli/prototype-assertion-rejects-explicit.json` retargeted onto the vendored fauxware
(0x400678, an instruction boundary discovery does not call an entry) because CI has no
dataset. Sweep: 464 by-address decompiles over 41 fixtures, 0 differences.
