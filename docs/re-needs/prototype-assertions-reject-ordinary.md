---
need_id: prototype-assertions-reject-ordinary
title: Prototype assertions reject ordinary unsigned int declarations
track: tooling
status: open
severity: major
probe_id: p-b95fa6549eeb
acceptance_id: a-e9643c3e9aaa
hypothesis_status: upheld
credibility: 1.0
instances: 5
challenges: [5ab77f5733c5d40ad448c380, 5ab77f5833c5d40ad448c399, 640a526833c5d447bc761899, 68d9ee36224c0ec5dcedc3fc, 6a0b84982b3df128c1df5c0d]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-console/src/grammar, decompiler/crates/kuna-console/src/ifacedecomp.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Use standard C parameter types in prototype assertions.

> **Prototype assertion rejects standard C int on ARM** (minor, `5ab77f5733c5d40ad448c380`)
> Exited 0 with 'rejected: Bad C syntax'. With and without a semicolon failed; replacing int with int4 succeeded.

> **Prototype assertion rejects C int but accepts int4** (minor, `5ab77f5833c5d40ad448c399`)
> Rejected int with Bad C syntax, with or without a semicolon. Replacing int with int4 succeeded.

> **Standard C scalar types are rejected in prototype assertions** (major, `640a526833c5d447bc761899`)
> Rejected the assertion with Bad C syntax while exiting 0. A trailing semicolon did not help. int declarations also failed; float8 and int4 were accepted.

> **Prototype assertions reject ordinary unsigned int declarations** (minor, `68d9ee36224c0ec5dcedc3fc`)
> Rejected unsigned int with Syntax error. Internal uint4 and uint8 spellings worked.

> **Prototype assertions reject the standard C type int** (major, `6a0b84982b3df128c1df5c0d`)
> An int-returning prototype is rejected with Bad C syntax, with or without a semicolon. Replacing int with int4 succeeds and emits int.

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
    "sub_140004dcc",
    "--assert",
    "prototype VirtualAlloc void *VirtualAlloc(void *p,unsigned int n,unsigned int a,unsigned int b)",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "rejected"
      }
    ],
    "stderr_matches": [
      "Syntax error"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.exe",
    "binary_sha256": "30849bed966c92e64009a23df62210e615a2b3e3342a79372866af53cdffa540",
    "binary_size": 74752,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-e9643c3e9aaa",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "authenticate",
    "--json",
    "--assert",
    "prototype authenticate unsigned int authenticate(char *user,char *pass)",
    "--assert",
    "prototype read long long read(int fd,void *buf,unsigned long n)",
    "--assert",
    "type v2 unsigned char[8]"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "binary_sha256": "c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b",
    "binary_size": 8776,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "selector": "authenticate",
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
        "value": 3
      },
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "assertions[1].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "assertions[2].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "unsigned int authenticate(char *user,char *pass)"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "unsigned char v2 [8];"
      }
    ],
    "stderr_absent": [
      "Syntax error",
      "Bad C syntax"
    ]
  },
  "notes": "Desired: the standard C scalar keywords the printer EMITS are the ones --assert accepts -- in return position (`unsigned int`, `long long`), in parameter position (`int`, `unsigned long`), and as an --assert type base (`unsigned char[8]`). Retargeted onto the in-repo fauxware fixture so it runs with no dataset; all three measured rejected on cf5234ac."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The assertion parser may lack the C aliases used by the emitter.
- The assertion parser may lack target-native scalar type aliases.
- The declaration parser may not recognize unsigned type specifiers.
- The parser may lack the standard int alias in this language configuration.

## Refutation

**UPHELD by direct measurement** (captain, round 3, cf5234ac, freshly built binary).

The hypothesis is that the `--assert prototype` C-declaration grammar accepts only sized
Ghidra type names plus `void`/`char`, not the standard scalar keywords. One quoted argv
element each, on `collide` (i386), function `sub_8049f20`:

| assertion | result |
|---|---|
| `int sub_8049f20(void)` | rejected, "Bad C syntax" |
| `unsigned int sub_8049f20(void)` | rejected |
| `double sub_8049f20(void)` | rejected |
| `int4 sub_8049f20(int x)` | rejected (parameter position too) |
| `int4 sub_8049f20(int4 x)` | applied |
| `void sub_8049f20(void)` | applied |

So the defect is a missing keyword set in the grammar, present in return AND parameter
position; it is not shell splitting. PR #415 asserts the opposite ("Quoted, the same
assertions are accepted: int, int4, unsigned int, char ** all parse") and is wrong on that
claim -- #415 fixed a real but different defect, the error message. A builder must not read
#415 and close this need.

SIDE FINDING, not this need's business but recorded so it is not lost: an ACCEPTED assertion
`float8 sub_8049f20(void)` on the same i386 function exits 1 on an un-ported seam
(LOSS-131, `heritage typeop_skeleton: unexpected opcode CPUI_FLOAT_FLOAT2FLOAT`), and with
`--json` the same run panics at `p3_dataflow/heritage.rs:4610` instead of reporting it.
Related to [accepted-sqrt-prototype-still].

## Reference

_none recorded_

## Instances

- `5ab77f5733c5d40ad448c380` (round 3, tester t-r3-5ab77f57)
- `5ab77f5833c5d40ad448c399` (round 3, tester t-r3-5ab77f58)
- `640a526833c5d447bc761899` (round 3, tester t-r3-640a5268)
- `68d9ee36224c0ec5dcedc3fc` (round 3, tester t-r3-68d9ee36)
- `6a0b84982b3df128c1df5c0d` (round 3, tester t-r3-6a0b8498)

## Decision log

- filed by cluster.py from 5 observation(s)
captain T_DEDUP r3: MERGED 3 deterministic clusters (prototype-assertion-rejects-standard obs0/obs4, standard-c-scalar-types obs9/obs20, this one obs15) -- one gap, split only because kind differed (bad-ux vs wrong-output) and group() will not merge across kinds.
captain T_DEDUP r3: witness overridden to the obs15 instance. The other four acceptances assert only `stderr_absent: rejected`, which a crash or a silently-dropped spec also satisfies; this one asserts assertions[0].status == "applied" in --json. Severity raised minor -> major (max across the merged group).
captain T_DEDUP r3: measured on cf5234ac (post-#415), one quoted argv element each -- REJECTED: `int`, `unsigned int`, `double`, and `int` in a parameter. ACCEPTED: `void`, `char *`, `char **`, `int4`, `float8`, `int4 a,char **b`. The grammar lacks the standard scalar keywords; it is not a quoting problem.
captain T_REFUTE r3: hypothesis upheld -- see ## Refutation (measured on cf5234ac with the release binary).
captain T_TRIAGE r3: track tooling CONFIRMED; touches CORRECTED kuna-cli -> kuna-console/src/grammar, which is where parse_C runs and where run_parse_c raises the 'Bad C syntax' the probe matches (ifacedecomp.rs:705). Upheld at T_REFUTE: int / unsigned int / double are rejected in BOTH return and parameter position while int4 / void / char * are accepted -- the grammar lacks the standard scalar keywords. Highest corroboration in the round: 5 instances across 5 challenges, credibility 1.0. NOTE FOR THE BUILDER: PR #415 asserts this is already handled and is wrong on that claim; re-measure before believing it.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
builder b-r3-prototype-assert r3: acceptance RETARGETED (a-35ab0d1e49fc -> a-e9643c3e9aaa) from the dataset PE onto the in-repo fixture `decompiler/crates/kuna-analysis/tests/fixtures/fauxware`, because `verify --promote` refuses a dataset target (CI has no dataset) and the need's regression guard is worth more than the original binary. Strengthened while moving: three directives instead of one (`unsigned int` in return position, `long long`/`int`/`unsigned long` in a callee prototype, `unsigned char[8]` as a `type` base), all three asserted `applied`, plus two positive clauses on the emitted C and `stderr_absent` on both "Syntax error" and "Bad C syntax". Measured rejected 3/3 on the unpatched cf5234ac binary; the ORIGINAL dataset acceptance was also measured PASS on the fixed tree before the retarget.
