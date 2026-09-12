---
need_id: explicit-double-score-prototype
title: Explicit double score prototype returns unassigned XMM0 halves
track: quality
status: closed
severity: major
probe_id: p-f5ab11021f25
acceptance_id: a-106f570103b3
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [69a54bd70f5b9757a6a5f72f]
rounds: [9, 12]
first_seen_round: 9
attempts: 1
covered_by_option: calleeretpreserves
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 608
closed_in_round: 12
closing_pr: "608"
reject_reason: null
---

## Symptom

Recover the mean log-probability returned by the password checker. Completing the challenge also requires model.nli, which is absent from the supplied input.

> **Explicit double score prototype returns unassigned XMM0 halves** (major, `69a54bd70f5b9757a6a5f72f`)
> Default output makes the score function void. Explicit prototypes for log, the cookie helper, and the score instead produce return (double)CONCAT44 of unassigned locals. Assembly computes DIVSD at 0x1400034fe, copies the result to XMM0 at 0x140003515, calls the cookie helper, and returns. The cookie helper's returning path never writes XMM0. Enabling calleepreserves and calleeretpreserves did not repair default output.

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
    "prototype 0x14000904f double log(double x)",
    "--assert",
    "prototype 0x140008140 void cookie(unsigned long long value)",
    "--assert",
    "prototype 0x140003150 double score(void *model,void *password)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "return\\s+\\(double\\)CONCAT44\\(v[0-9]+,SUB84\\([^;]+/[^;]+,0\\)\\)"
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_explicit_double_score_return_x86_64.exe",
    "binary_sha256": "9454da07fedd100366e7a5955a29d218eb7f05bbb5bf5c3aa96f7d50f1c7deb9",
    "binary_size": 3584,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_explicit_double_score_return_x86_64.exe"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "score",
    "--assert",
    "prototype 0x14000904f double log(double x)",
    "--assert",
    "prototype 0x140008140 void cookie(unsigned long long value)",
    "--assert",
    "prototype 0x140003150 double score(void *model,void *password)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?ms)^\\s*(v[0-9]+)\\s*=\\s*\\1\\s*/[^;]+;.*^\\s*return\\s+\\1;"
    ],
    "stdout_absent": [
      "return\\s+\\(double\\)CONCAT44",
      "return\\s+[^;]*SUB84"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Floating-point return preservation across cleanup and security helpers is not recovered.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/Release.zip.__x/PolyMLP.exe` — Reference unavailable: server exited with status 1 before registering. Kuna disassembly supplies the supporting evidence.

## Instances

- `69a54bd70f5b9757a6a5f72f` (round 9, tester t-r9-69a54bd7)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD in substance -- an XMM0 double return really is not preserved across the /GS helper -- but the filing is wrong about what it takes to reproduce, and it misses that the SAME write fails two different ways depending on the cookie prototype. All runs: release kuna, .kuna-repipe/arena/9/69a54bd70f5b9757a6a5f72f/target/Release.zip.__x/PolyMLP.exe, sub_140003150. THE ASSEMBLY, so nobody re-derives it: DIVSD XMM6,XMM0 at 0x1400034fe computes the mean; two cleanup calls to 0x140004740; MOVAPS XMM0,XMM6 at 0x140003515 places the return value; MOV RCX,[RBP-9]; XOR RCX,RSP; CALL 0x140008140 at 0x14000351f. 0x140008140 IS __security_check_cookie (CMP RCX,[0x14000e040]; ROL; TEST CX; RET / JMP to the failure handler) and it reads only RCX. CORRECTION 1 -- THE REPRO NEEDS ONE ASSERT, NOT THREE. 'prototype 0x140003150 double score(void *model,void *password)' ALONE yields 'return (double)CONCAT44(v31,v29);'. The log and cookie asserts change other lines and the variable numbering; neither is load-bearing for the return. Whoever builds this should cut the probe to the single assert -- three asserts hid the mechanism from the tester. CORRECTION 2 -- TWO FAILURE MODES ON ONE WRITE, AND THE FILED ONE IS THE SECOND. (i) WITHOUT a cookie prototype the tail reads: 'sub_140008140(SUB84(v4 / (double)v22,0)); return (double)CONCAT44(v31,v29);'. The mean IS recovered -- and then handed to __security_check_cookie as an ARGUMENT. XMM0 is in the Windows x64 float-argument set, so the return-value placement at 0x140003515 is captured as a parameter of the very next call, while the real argument (RCX = the frame-cookie XOR) is dropped. Doubly wrong in one line. (ii) WITH the tester's 'void cookie(unsigned long long value)' the argument line becomes correct ('sub_140008140(v38 ^ (unsigned long long)v14);') and the mean DISAPPEARS FROM THE OUTPUT ENTIRELY -- no 'v4 / (double)vN' anywhere; with XMM0 no longer an argument the MOVAPS write has no consumer, is dead-code-eliminated, and the RETURN still reads an undefined XMM0. That is CODE LOSS and it is the worse of the two. Both tails still end in the same CONCAT44 of two undefined 4-byte lanes, which is what the RETURN's read of a killed 8-byte XMM0 renders as. THE ABLATIONS ARE VACUOUS AS FILED, AND I RE-RAN THEM PROPERLY. calleepreserves and calleeretpreserves are BOTH DEFAULT ON (kuna catalog), so 'enabling calleepreserves and calleeretpreserves did not repair default output' is a non-statement -- the same trap as hint-printing-call-loses. I ran each OFF in BOTH configurations (4 runs) and every one is byte-identical to its default. Also inert: msvcstackguard on (the call is still emitted, 201 lines both ways), cookiescramble off, stackguard off, returnpair single. THE LEAD, AND WHY IT IS EMBARRASSING. kuna_calleeretpreserves.rs was written for EXACTLY this pattern -- its own module header shows an MSVC /GS main returning 'sub_140007d90(v12)' across __security_check_cookie -- and it is inert here. Its gate in p3_dataflow/heritage.rs:1559 is 'effecttype == KILLEDBYCALL && (calleepreserves::callee_preserves_range(..) || calleeretpreserves::callee_preserves_return_storage(..))', keyed on an (addr,size) range with nothing integer-specific in the signature, so the question for the builder is WHICH range the XMM0 read arrives as and whether the bounded body walk declines this callee (its returning path is the RET at 0x140008154, reached past a JNZ, with a JMP 0x1400086b8 tail on the failure arm). One instrumented run at that gate -- print trans_addr/size/effecttype and both predicates for every range at the 0x14000351f call -- settles it before any code is written. Failure mode (i) is a second, argument-side question: a callee whose decoded body reads only RCX should not take an XMM0 argument, which is calleearitybody territory. ACCEPTANCE IS PASSABLE BY THE BUG, TIGHTEN IT BEFORE DISPATCH. It asks for 'return\\s+[^;]+;' present and the CONCAT44 absent. Configuration (ii) already deletes the mean while keeping a return statement, so a 'fix' that merely stops materialising the undefined halves -- or that returns any fresh scalar -- passes while the answer the RE task actually needs stays gone. Add a positive witness that the division survives AND is what is returned (something like 'return[^;]*/[^;]*;' anchored on the (double) cast), and keep the CONCAT44 absence as the second clause.

### 2026-09-08T06:05Z — captain, round 9 B_DRAIN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate rather
than failing, and therefore unclosable by B_DONE however good a fix was. Bound to the
challenge's own image (challenge `69a54bd7`, one image, no ambiguity):

    bin/Release.zip.__x/PolyMLP.exe (54272 B, sha256 01652e8b…)

Re-run through `verify --acceptance-suite --need explicit-double-score-prototype` on the current build it now RUNS
and FAILS, on the clause that carries the filed symptom:

    stdout_absent[0] → actual "return (double)CONCAT44(v28,v27)"

Nothing about the probe's command or expectations was changed — only the target it
resolves against.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.

### 2026-09-12 — builder trace and candidate decision

On exact dependency `b5c0b4f8b2159e871f18a99baae9a333954d9d12`, an
instrumented heritage run at the cookie call `0x14000351f` showed why the prior
fix did not cover this explicit prototype: XMM0 was refined into four-byte
cells. The low cell at register offset `0x1200` was `ContainsJustified` in the
checker ABI output and already survived; the adjacent cell at `0x1204` was
`ContainsUnjustified` and was killed. The upper XMM scratch cells remained
`NoContainment`. That produced
`return (double)CONCAT44(undefined,SUB84(division,0))`.

The candidate extends only the exact-cookie proof to the adjacent
`ContainsUnjustified` cell and additionally requires that the caller prototype
be output-locked and contain that exact cell. The generic complete-body proof,
the upper scratch cells, explicit effect overrides, positive body writes, and
output-space STORE vetoes are unchanged. An inferred/unlocked caller stays on
the old path, and `calleeretpreserves off` restores the defect. Omitting the
cookie prototype exposes the separate call-argument inference defect and is
deliberately out of scope.

The dataset-only acceptance was reduced to a deterministic in-repo PE variant
that forces the same four-byte refinement without changing the computed
double. Its acceptance requires the division assignment and the same variable
to be returned, and rejects both `CONCAT44` and return-side `SUB84`; it fails on
the dependency base and passes this candidate. The probe was promoted as
`tests/cli/explicit-double-score-prototype.json`. This log records candidate
evidence only: frontmatter remains `status: open`, with no PR or closure claim.

- round 12 CLOSURE (PR #608): acceptance `a-106f570103b3` passes three repetitions on the deterministic 3,584-byte PE fixture (`sha256:9454da07fedd100366e7a5955a29d218eb7f05bbb5bf5c3aa96f7d50f1c7deb9`). The exact-cookie arm now preserves the adjacent `ContainsUnjustified` XMM0 cell only when the caller has a locked declared output containing that cell; the generic body proof, upper scratch storage, write and output-space STORE vetoes, explicit effects, and indirect calls remain conservative. The returned variable is the division result, while `calleeretpreserves off` restores `CONCAT44` and an unlocked caller restores `SUB84`. Rebased on `7a6689c8db00c03e0b1021f7339ec04c03cbaeaa` with #607's exact-site/SCC/provenance behavior preserved byte-for-byte; focused gates cover 14/14 return-preservation tests, 26/26 stackguard/SCC tests, 3/3 exact literal-rewrite tests, 1/1 provenance tests, 11/11 stage assertions, both acceptances, and all three promoted CLI arms. Shared counters have no drift and mergecheck reports zero rejects.
