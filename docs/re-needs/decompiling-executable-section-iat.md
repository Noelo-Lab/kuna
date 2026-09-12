---
need_id: decompiling-executable-section-iat
title: Decompiling an executable-section IAT slot invents an arithmetic function
track: quality
status: closed
severity: major
probe_id: p-b0ded31a46b9
acceptance_id: a-fffdb4ce0518
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [5ab77f6433c5d40ad448cae1]
rounds: [12]
first_seen_round: 12
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: "588"
closed_in_round: 12
closing_pr: "588"
reject_reason: null
---

## Symptom

Inspect GetDlgItemTextA as an import without decoding its four-byte pointer slot as a function.

> **Decompiling an executable-section IAT slot invents an arithmetic function** (major, `5ab77f6433c5d40ad448cae1`)
> Resolved GetDlgItemTextA to 0x401078, then emitted CARRY1, pointer arithmetic, a memory write, and an overlapbranch warning. This entry is IAT data within .text.

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
    "GetDlgItemTextA"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\*v[0-9]+\\s*=\\s*\\*v[0-9]+\\s*\\+"
    ]
  },
  "target": {
    "binary_rel": "bin/GiTS_2010_Crypto_Crackmes.zip.__x/crypto4.exe",
    "binary_sha256": "f88284c9bfe6e8581261ee224395c83d79507b26a64f81614631d78498cf63b9",
    "binary_size": 123904,
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_i386.exe",
    "binary_sha256": "fc9a367d8615741a122c5b0dbd3d3042f408cbce93451a796bcc45f7710c719b",
    "binary_size": 4608,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_i386.exe"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "VirtualAlloc"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stdout_absent": [
      "CARRY1\\(",
      "\\*v[0-9]+\\s*=\\s*\\*v[0-9]+"
    ],
    "stderr_matches": [
      "^error: selector \\\"VirtualAlloc\\\" identifies import VirtualAlloc at 0x401000; the IAT slot contains a loader-written pointer, not a function body\\n$"
    ]
  },
  "notes": "The vendored PE reproduces the dataset witness's layout: its IAT is inside a CODE|EXECUTE section. The CLI must retain VirtualAlloc as a call-binding symbol but refuse to lift its pointer slot."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Function-body selection does not exclude known IAT slots when their section is executable.

## Refutation

_not yet refuted_

## Reference

- `objdump -p target/GiTS_2010_Crypto_Crackmes.zip.__x/crypto4.exe` — Identifies GetDlgItemTextA as the fourth USER32.dll import, placing its pointer slot at 0x401078. The bytes kuna decoded are import data. IDA's server failed before registering.

## Instances

- `5ab77f6433c5d40ad448cae1` (round 12, tester t-r12-5ab77f64)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|stdout_absent` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- sibling of `pe-import-calls-become` — shared root condition (IAT data inside .text); kept separate, this half is a CLI refusal.
- round 12 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED. The symptom is exact and now reproduces on a SECOND, unrelated binary, but the hypothesis's discriminator -- "when their section is executable" -- is measurably false, and the clause that is true is misframed in a way that would send a builder at load-bearing code.

MEASURED ON b9029a8f (release binaries rebuilt 05:39).

THE CONTROL THAT OVERTURNS IT. decompiler/crates/kuna-analysis/tests/fixtures/pe_imports.exe puts its IAT in .idata at 0x14000d000, flags CONTENTS ALLOC LOAD DATA -- WRITABLE and NOT EXECUTABLE, the exact opposite of the filed condition. kuna registers 50 functions inside that .idata range (DeleteCriticalSection 0x14000d1dc, EnterCriticalSection 0x14000d1e4, GetLastError 0x14000d1ec, ...), and kuna decompile <fixture> 0x14000d1ec --addr emits the SAME class of invented arithmetic body: "v2 = *v8; *(unsigned int *)CONCAT71(v3,v2) = *(int *)CONCAT71(v3,v2) << (v4 & 0x1f) | ...; *(char *)CONCAT71(v3,v2) = *(char *)CONCAT71(v3,v2) + v2;". Pointer arithmetic and a memory write off decoded import-pointer bytes, in a non-executable data section. Section permission is NOT the discriminator. A builder who adds an is-executable guard changes nothing on either binary.

BOTH SHIPPED OPTIONS ARE PROVEN INNOCENT on the reported command. kuna decompile crypto4.exe GetDlgItemTextA emits the identical CARRY1 body under --option litpoolconst off AND under --option peimportcall off. So this is NOT the constant-fold defect its sibling turned out to be (see pe-import-calls-become, UPHELD, cause litpoolconst) -- the two needs do not share a root cause, only a binary.

WHAT IS ACTUALLY HAPPENING, and why the other clause is misframed. It is not that "function-body selection does not exclude known IAT slots". kuna's PE loader POSITIVELY ASSERTS that each IAT slot is a function: loader/pe_iat.rs registers a FunctionSymbol at every slot VA, and kuna_peimportcall.rs:28-31 says so in as many words -- "ActionDeindirect resolves it through the FunctionSymbol pe_iat already registered at the slot VA... kuna's query_function keys on the Varnode's own address rather than upstream's ExternRefSymbol::refaddr indirection, so the slot-VA registration lines up without a second symbol." THE SLOT-VA FUNCTION REGISTRATION IS LOAD-BEARING FOR DIV-57. Deleting it, or teaching body selection to skip IAT slots wholesale, breaks peimportcall's import binding on every PE. That is the trap in the filed wording and it is why this reads overturned rather than upheld-with-detail.

The real gap is narrower: an import is modelled as a function whose ENTRY is its IAT slot, and nothing between that symbol and the decoder distinguishes "this entry exists so a call can be bound by address" from "this entry has a body worth lifting". kuna's own decoder already notices the collision -- the emitted body carries "warn: overlapbranch: this instruction overlaps the branch target at 0x00401088", and 0x401088 is SetWindowPos, the NEIGHBOURING IAT slot, also registered as a function. The registered sizes admit it too: in crypto4 the slots come out size 4, or size 8 for the last entry of a DLL block where the null terminator is absorbed (SetBkMode 0x40100c, HeapDestroy 0x401064); in the control they come out size 0.

REDIRECT, not a proven line: make the refusal at the decompile entry point, on the fact that the target is a known import slot (loader/pe_iat.rs:93 resolve_pe_import_slots already returns exactly those ranges), and keep the symbol. An import has no body in this image by definition, so declining with the import's identity -- name, ordinal, DLL -- is strictly more useful to an agent than four bytes of decoded pointer.

A SECOND OBSERVATION THE CONTROL SURFACED, not this need but adjacent: on the control fixture, kuna decompile <fixture> GetLastError errors with "selector is ambiguous; candidates: GetLastError at synthetic 0x1400079f8, GetLastError at synthetic 0x14000d1ec" -- the thunk veneer and the IAT slot both carry the name -- AND EXITS 0. A name-selector error path returning success is its own hazard for a scripted agent.

ACCEPTANCE IS PERMISSIVE, READ IT BEFORE BUILDING. It has no exit_code clause and no positive clause: only stdout_absent of the pointer-arithmetic shape and of CARRY1. Any refusal, any error, or empty stdout passes it. That matches this need's stated intent (the T_DEDUP note calls this half "a CLI refusal"), so it is not wrong -- but it cannot distinguish a good refusal from a suppressed one, and a builder must not satisfy it by making the emitter quieter in general. Pair it with the sibling's acceptance, which is strict.

DISPATCH CONSEQUENCE: the T_DEDUP note's stated shared root condition, "IAT data living inside .text", is now MEASURED FALSE for this half -- it reproduces where the IAT is not in .text at all. Do not cluster the pair on that premise. They remain adjacent work (both live in loader/pe_iat.rs and both need resolve_pe_import_slots), so one builder taking both is still reasonable, but for the code they touch, not for a shared cause.
- round 12 B_IDLE (captain): **probe target bound by hand; this need is now dispatchable.** Both
  its probe blocks used `{{BIN}}` with no `target`, so every replay died with "`{{BIN}}` used but the
  context supplies no bin" -- clauses `[]`, repeat 0 -- and two consecutive `B_PLAN` ticks had to
  skip it because an acceptance that cannot run can never flip, however good the fix. The binary is
  identified from the filing tester's own `toolcalls.jsonl`, by an argv that matches the probe cmd
  exactly, not by guessing from the arena: `kuna decompile target/GiTS_2010_Crypto_Crackmes.zip.__x/crypto4.exe GetDlgItemTextA`. Bound to
  `bin/GiTS_2010_Crypto_Crackmes.zip.__x/crypto4.exe` (sha256 `f88284c9bfe6e8581261ee224395c83d79507b26a64f81614631d78498cf63b9`, 123904 bytes,
  source `dataset`, resolved as `challenges/5ab77f6433c5d40ad448cae1/<rel>`). `target` is not part of
  `probe_id`/`acceptance_id` (they hash `cmd`+`expect` only), so both ids are unchanged and no
  front-matter drifted. Replayed on unpatched main `17da9472`: **reproduction PASSES** (the defect
  is present) and **acceptance FAILS** on both `stdout_absent` clauses (`*v1 = *v1 +` and `CARRY1(` are still emitted) -- runnable, honestly failing, which is the
  dispatch precondition.
- builder attempt 1: promoted acceptance to the shipped in-repo `pe_iatincode_i386.exe`
  fixture so CI pins an executable-section IAT, a non-success verdict, stable import identity,
  and absence of lifted pointer arithmetic. Canonical acceptance changed from
  `a-adefc9329f47` to `a-fffdb4ce0518`; the original dataset reproduction and
  `p-b0ded31a46b9` remain unchanged. Implementation is pending review and merge, so status
  remains `open` with `pr: null`.
- closed: acceptance `a-fffdb4ce0518` now PASSES at exact implementation head
  `46075ebcf48df2aff29a6ee2701bf283c1b3e7a2`. Measured surfaces: the dataset
  witness `GetDlgItemTextA` at `0x401078` exits 1 with the stable import-slot
  diagnostic and no `CARRY1`/invented pointer arithmetic; the executable-IAT
  fixture's `VirtualAlloc` at `0x401000` and the non-executable-IAT fixture's
  `GetLastError` at `0x14000d1ec` are likewise refused; an ordinary real body at
  `0x401010` still decompiles with named `VirtualAlloc`/`GetModuleHandleA` calls;
  and the executable thunk for `GetLastError` still decompiles with its imported
  `unsigned int` prototype. Filtered decompile-all/project and direct WASM
  name/address paths also pass. Closure metadata is durable only when PR #588
  merges.
