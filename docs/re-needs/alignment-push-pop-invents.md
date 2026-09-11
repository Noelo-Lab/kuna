---
need_id: alignment-push-pop-invents
title: Alignment push/pop invents a fifth decryptor argument and 128-bit return
track: quality
status: open
severity: major
probe_id: p-657d96ac25e7
acceptance_id: a-52c7f7e4dcd5
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [67f9bdc38f555589f3530a85]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover the four-argument XOR decryptor returning its allocated buffer.

> **Alignment push/pop invents a fifth decryptor argument and 128-bit return** (major, `67f9bdc38f555589f3530a85`)
> Emits undefined16 and appends a fifth argument as the high return half. Disassembly shows PUSH R8 and POP RDX around the body; RAX holds the buffer. Argument recovery options do not fix this. An explicit prototype does.

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
    "0x10e27",
    "--addr",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "^undefined16 "
      },
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "\\._8_8_ = a[0-9]+;"
      }
    ]
  },
  "target": {
    "binary_rel": "bin/WeeperVM--Level_1",
    "binary_sha256": "8ecacb009c64989b3d269193617a742ee039ffad7e2690ed148f11312751ef03",
    "binary_size": 14112,
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
    "0x10e27",
    "--addr",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "undefined16",
      "\\._8_8_ = a[0-9]+;"
    ]
  },
  "target": {
    "binary_rel": "bin/WeeperVM--Level_1",
    "binary_sha256": "8ecacb009c64989b3d269193617a742ee039ffad7e2690ed148f11312751ef03",
    "binary_size": 14112,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The incidental RDX value restored from an alignment slot is treated as a second return register.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `67f9bdc38f555589f3530a85` (round 12, tester t-r12-67f9bdc3)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_absent` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **overturned** (was inconclusive). REFUTER RAN (captain, in-tick, 2026-09-09 06:2xZ). SYMPTOM STANDS, FILED CAUSE IS WRONG IN BOTH HALVES, AND THE REAL GATE IS A SHIPPED DEFAULT-ON OPTION.

Filed cause: 'the incidental RDX value restored from an alignment slot is treated as a second return register'. Measured false in its specifics.

MINIMAL REPRO (3 functions, 8 instructions, gcc -shared -fPIC -nostdlib idiom.s):
  same_reg_restore:  push %rdx; mov %rdi,%rax; add $7,%rax; pop %rdx; ret
  cross_reg_restore: push %r8;  mov %rdi,%rax; add $7,%rax; pop %rdx; ret
  dead_rdx_write:    mov %rdi,%rax; add $7,%rax; mov %rsi,%rdx; ret
kuna decompile idiom.so <f>:
  same_reg_restore  -> long same_reg_restore(long a0)                      CORRECT
  cross_reg_restore -> undefined16 (long,ulong,ulong,ulong,ulong), v1._8_8_ = a4   THE DEFECT, 5 instructions
  dead_rdx_write    -> undefined16 (long,ulong), v1._8_8_ = a1                     SAME DEFECT, NO STACK AT ALL
So (a) the alignment SLOT is irrelevant -- dead_rdx_write touches no stack and shows the identical shape; (b) same-register restore is ALREADY recognised, so the gap is specifically cross-register.

THE ACTUAL GATE IS retinputhalf (P4, core, default ON), NOT a missing capability:
  kuna decompile <bin> 0x10e27 --addr --option retinputhalf off
  -> void * sub_10e27(long a0,int a1,long a2,int a3)   i.e. EXACTLY the acceptance shape and exactly the task the tester was set
  kuna decompile <bin> 0x10e27 --addr --option returnpair single   -> same clean output
The uncomputed-return repair ALREADY drops a return half that traces back only to inputs. retinputhalf re-adds it because 'the function PLACED it there' -- and here the function did place R8's value into RDX, by the alignment pop. The predicate is satisfied by an accident. The tester's 'argument recovery options do not fix this' is true and irrelevant: this is a RETURN option, not an argument one.

THE CIRCULARITY, which is the defect proper: RDX at the RET traces to R8 at entry, so R8 is invented as parameter a4; a4 being a parameter is then what makes the half 'an input parameter the function PLACED there' and satisfies retinputhalf. R8's ONLY read in cross_reg_restore is the PUSH itself.

DO NOT FIX IT BY FLIPPING EITHER OPTION -- BOTH ARE MEASURED TO DESTROY GENUINE 128-BIT RETURNS. Control /tmp/ctl.c, gcc -O2 -shared -fPIC:
  __int128 wide_ret(long a,long b){return ((__int128)a<<64)|(ulong)b;}
  struct two {long x,y;}; struct two two_ret(long a,long b){...a*3, b^0x5a...}
  default:                wide_ret undefined16 v1._8_8_=a0 / v1._0_8_=a1  CORRECT;  two_ret undefined16 both halves CORRECT
  retinputhalf off:       wide_ret -> 'unsigned long wide_ret(...){return a1;}'  HIGH HALF SILENTLY LOST;  two_ret survives (halves computed)
  returnpair single:      wide_ret -> return a1;  two_ret -> 'long two_ret(long a0){return a0*3;}'  A RETURN HALF AND A WHOLE PARAMETER GONE
undefined16 is kuna's NORMAL, CORRECT rendering of a genuine 128-bit return -- the acceptance's stdout_absent on the literal string 'undefined16' is safe only because it is pinned to this one address. Do not read the need as 'undefined16 is wrong'.

REDIRECT FOR THE BUILDER, the discriminator that separates every case measured here: a pop from a stack slot whose matching push sourced a DIFFERENT register is stack maintenance, not a value placement, so it must not satisfy retinputhalf's 'the function placed it there' -- equivalently, a candidate input register whose only use is a push later popped into another register is not a parameter. That rule fixes the target and cross_reg_restore, leaves same_reg_restore untouched, and cannot reach wide_ret/two_ret (no push/pop involved). dead_rdx_write is genuinely ambiguous (indistinguishable from a struct{long,long} return) and should be left alone. Caller-side RDX liveness is NOT available as a discriminator on this binary: kuna xrefs --to 0x10e27 returns 0 callers (VM/indirect dispatch).

CONTROL PROVING THIS IS NOT THE SOURCE SHAPE: the same four-arg XOR decryptor compiled by plain gcc -O2 (ctl.c xordec) decompiles CORRECTLY today as void *xordec(long,int,long,int). Only the push-R8/pop-RDX prologue idiom in the target triggers it.
