---
need_id: bulk-decompilation-decodes-pe
title: Bulk decompilation decodes a PE import pointer slot as a function
track: loader
status: open
severity: major
probe_id: p-4673be7e96d8
acceptance_id: a-64ea8ac57030
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f5c33c5d40ad448c681]
rounds: [9]
first_seen_round: 9
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Retain VirtualAlloc for call naming without inventing a function body for its IAT slot.

> **Bulk decompilation decodes a PE import pointer slot as a function** (major, `5ab77f5c33c5d40ad448c681`)
> The filtered bulk run emitted C for VirtualAlloc at 0x4010ce, size 4, including a write through an uninitialized pointer and a next-function-boundary warning. The full batch similarly decoded many import slots. Already-filed hit: block-processing-panics-out occurred repeatedly in the full batch; not filed again.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--json",
    "--filter",
    "^VirtualAlloc$",
    "--no-vars"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].name",
        "op": "eq",
        "value": "VirtualAlloc"
      },
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "\\{[\\s\\S]*\\*"
      }
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
      "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_i386.exe",
      "binary_sha256": "fc9a367d8615741a122c5b0dbd3d3042f408cbce93451a796bcc45f7710c719b",
      "binary_size": 4608,
      "binary_source": "in-repo",
      "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_i386.exe"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--json",
    "--filter",
    "^VirtualAlloc$",
    "--no-vars"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 0
      }
    ]
  },
  "notes": "Vendored twin of the reported crypter: a PE32 whose one section is CODE|EXECUTE|READ|WRITE (0xe0000060) and holds the whole import directory, so its VirtualAlloc/GetModuleHandleA/ExitProcess IAT slots sit in code exactly as the original's 50 do. Built by the sibling pe_iatincode_i386.py. The dataset image stays the witness in Reproduction."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Imports inside an executable section appear to pass the code-body eligibility check despite their known imported identities.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5c33c5d40ad448c681` (round 9, tester t-r9-5ab77f5c)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the discriminator is the executable-range universe, not the import identity. Measured on a POST-#502/#504 binary (copied out of the b-r9-flow-call-overri worktree, HEAD 6b27436d = #501+#502+#504), so the stale-main caveat from the previous tick is discharged. THE SYMPTOM STANDS: 'kuna decompile-all bm3.exe --json --filter ^VirtualAlloc$ --no-vars' still returns count 1, name VirtualAlloc, address 0x4010ce, size 4, body 'void VirtualAlloc(void){ char *v1; *v1 = *v1 + (char)v1; }' with a funcboundflow warning. 0x4010ce IS a real kernel32 IAT slot (parsed the import directory by hand: kernel32 IAT at 0x4010c6, 19 slots, VirtualAlloc third). IT IS NOT ONE FUNCTION, IT IS 50 OF 56: 'kuna functions bm3.exe --json' reports 56 entries and 50 of them are IAT slots at size 4 (kernel32 0x4010c6.., msvcrt 0x4012a0.., user32 0x4013e2..). The real function inventory of this binary is 6 entries; the other 50 are import pointers. THE CONTROL THAT PINS THE CAUSE. bm3.exe has TWO sections, both unnamed, and the first (0x401000, VS 0x1515, chars 0xe0000060) is EXECUTABLE+WRITE+CODE and contains the whole import directory including every IAT slot. In phantomgate.exe (same round's arena, 6a095817) the IAT is at 0x14003b548 inside .idata, NOT executable: 'kuna functions' lists all 124 of its slots too, but at SIZE 0, and '_assert' -- the one phantomgate name that exists ONLY as a size-0 slot and has no .text thunk -- decompiles to COUNT 0. Same loader path (loader/pe_iat.rs registers one FunctionSymbol per slot VA by design, the ELF GOT model), same import identity, opposite outcome; the only variable is whether the slot lands in listing/mod.rs's exec_ranges (crate::entry::executable_sections), where an uncovered byte is CodeUnit::Undefined and gets decoded, versus outside it, where everything is conservatively Data. So the filed hypothesis is right as written -- imports inside an executable section pass the code-body eligibility check -- with the refinement that the size field is the visible tell (4 vs 0) and 'count 0 for a slot-only import' is ALREADY the engine's behaviour everywhere else. The fix is to make an import-slot FunctionSymbol non-eligible for a body regardless of section flags, not to add new suppression logic. TWO DISPATCH NOTES. (1) Nothing is lost by suppressing here: 'kuna xrefs bm3.exe --to 0x4010ce --json' returns count 0 -- no call site in this image names VirtualAlloc today (the crypter resolves through GetProcAddress at runtime), so the acceptance's count==0 cannot cost call naming on this binary. (2) But the acceptance IS absence-only and a fix that simply stopped registering PE imports passes it. The guard is the control above: phantomgate's 'decompile-all --filter ^CloseHandle$' must still emit 'void CloseHandle(void){ CloseHandle(); }' at the 0x14001d0d8 thunk. Pair that clause in at T_TRIAGE.
- round 9 TRIAGE: touches retargeted, acceptance left as filed, pairing clause recorded here because it cannot be expressed as a probe. touches kuna-decomp -> kuna-analysis: the fix is import-slot body eligibility (loader/pe_iat.rs registers one FunctionSymbol per slot VA, and listing/mod.rs decodes any uncovered byte inside exec_ranges), not a decompiler pass. THE ACCEPTANCE CANNOT CARRY THE PAIRING CLAUSE: a re-probe/1 probe is one argv against one {{BIN}}, and `cli-pair` is a declared kind with no separate runner, so the phantomgate control cannot be a second clause. It is therefore a HARD BUILDER REQUIREMENT instead, and the builder must run it by hand and paste the output in the PR: on .kuna-repipe/arena/9/6a09581717539b5175d122b9/.../phantomgate.exe, `kuna decompile-all <bin> --json --filter '^CloseHandle$'` must still emit `void CloseHandle(void){ CloseHandle(); }` at the 0x14001d0d8 thunk. Without it, a fix that simply stops registering PE imports passes `count == 0` while destroying call naming. At B_DONE, promote BOTH commands into tests/cli/ as two probes.

### 2026-09-08T06:05Z — captain, round 9 B_DRAIN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate rather
than failing, and therefore unclosable by B_DONE however good a fix was. Bound to the
challenge's own image (challenge `5ab77f5c`, one image, no ambiguity):

    bin/bm3.zip.__x/bm3.exe (26624 B, sha256 8116e2cb…)

Re-run through `verify --acceptance-suite --need bulk-decompilation-decodes-pe` on the current build it now RUNS
and FAILS, on the clause that carries the filed symptom:

    json[0] → actual 1

Nothing about the probe's command or expectations was changed — only the target it
resolves against.

### 2026-09-08T07:00Z — captain, round 9 B_DRAIN: track quality → loader

The round-4/9 refuters on this need and on `whole-binary-decompilation-treats` both closed
with the same track note — *touches is kuna-analysis (loader), not kuna-decomp* — and the
mechanism they landed on ("make an import-slot `FunctionSymbol` non-eligible for a body
regardless of section flags, not new suppression logic") writes in
`loader/pe_iat.rs` + `listing/mod.rs`. Neither is a decision point in the emitted-C option
plane. The track is corrected here so the selector draws the right leases; `touches` was
already retargeted at T_TRIAGE and is unchanged.

What this buys: `select.py`'s `TRACK_RESOURCES` hands `quality` the whole option-counter set
(`counter:catalog`, `counter:stages-corpus`, `counter:div`, `file:phases.toml`,
`file:docs/options.md`) and hands `loader` nothing, so filed as quality this need contended
for five leases a round-9 quality builder already holds and was undispatchable for the rest
of the round. As `loader` it contends for its own cluster only.

Why that is safe rather than a lease bypass — measured, not assumed. Across the 66 closed
needs whose closing commit is identifiable on `main`, **39 non-quality closes (34 tooling,
2 loader, 3 perf) touched `phases.toml` / `options.rs` / `docs/options.md` exactly zero
times**, while quality is 13 of 27. The lease-free tracks have never once carried an
option-adder in this loop.

**Hard requirement on whoever builds this, and the reason the note exists.** If the mechanism
turns out to need a `settableTable` row after all, that measurement is wrong for this need:
**stop and report `blocked` rather than adding the option unleased.** Round 9 is already
carrying a silent catalog-count collision (two concurrent quality builders both moving the
count 165 → 166) that arrived by exactly this route, and an unleased loader option-adder
would reproduce it.

**Sibling — do not dispatch these two separately.** `whole-binary-decompilation-treats` is
the same defect (PE IAT slots registered as function bodies) on a second image, and its
refuter traced it to the same two lines: `loadimage_object.rs:486-489` pushes every
`ImportSym` in as a `FuncSym`, `pe_iat.rs:214` emits one per slot VA. Both are now `loader`,
so `select` sees disjoint clusters and will hand them to two builders who write the same fix
and collide. At B_FANOUT bind them onto one builder from this anchor:

    res = sorted(set(select.resources_for(need)) | {"cluster:whole-binary-decompilation-treats"})
    captain.spawn_builder(N, needs.load("bulk-decompilation-decodes-pe"), res)

then confirm the sibling vanishes from `select -k 3 --json`. Both acceptances must flip; note
that the sibling's own refuter found *its* acceptance gameable in the opposite direction —
suppressing the slots on that image leaves an empty function list, which passes its
`stdout_absent` clause on nothing at all — so the pairing clause recorded above (phantomgate
`^CloseHandle$` must still emit its thunk body) guards both.
