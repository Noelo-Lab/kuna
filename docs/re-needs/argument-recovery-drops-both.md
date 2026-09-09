---
need_id: argument-recovery-drops-both
title: Argument recovery drops both watchdog arguments in the VM input handler
track: quality
status: open
severity: major
probe_id: p-9a7957a0cb99
acceptance_id: a-00ba32b8cbcb
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
regression_of: call-args-discarded
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

See the timer-array pointer and count passed after each input byte.

> **Argument recovery drops both watchdog arguments in the VM input handler** (major, `67f9bdc38f555589f3530a85`)
> With calleearity and varargstackargs enabled, emits watchdog(). Disassembly loads ESI=2 and EDI=0x13050 immediately before CALL 0x110ff. The callee uses both inputs. An explicit prototype restores both arguments.

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
    "0x113d9",
    "--addr",
    "--define-function",
    "0x110ff=watchdog",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--option",
    "linuxsyscall",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "watchdog\\(\\)"
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
    "0x113d9",
    "--addr",
    "--define-function",
    "0x110ff=watchdog",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--option",
    "linuxsyscall",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "watchdog\\(\\)"
    ],
    "stdout_matches": [
      "watchdog\\([^\\n,]+,[^\\n)]+\\)"
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

- Inferred callee arity is not applied at this call site; main retains two arguments to the same callee.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `67f9bdc38f555589f3530a85` (round 12, tester t-r12-67f9bdc3)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_absent,stdout_matches` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- sibling of `bytecode-append-call-loses` — same family (callee-arity under-applied at a call site; explicit prototype restores). Kept separate: signatures differ and Jaccard is 0.138, each carries its own acceptance, and merging would drop this need's regression_of onto a witness that lacks it. Consider forcing one builder via a shared `cluster:` at T_TRIAGE.
- round 12 REFUTER: hypothesis **overturned** (was inconclusive). REFUTER (captain in-tick, 05:0xZ round 12). VERDICT OVERTURNED -- the SYMPTOM STANDS EXACTLY AS FILED, the DIAGNOSIS DOES NOT, and the regression_of link is unearned. Everything below was measured on the arena binary .kuna-repipe/arena/12/67f9bdc38f555589f3530a85/target/WeeperVM--Level_1 (ELF x86-64, stripped) against release kuna at aa18e56d.

SYMPTOM REPRODUCED: the filed probe emits `if (watchdog()) {` at line 35. The disassembly the tester cited is exact -- 0x1146a MOV ESI,0x2 / 0x1146f MOV EDI,0x13050 / 0x11474 INC R12 / 0x11477 CALL 0x110ff.

WHY THE HYPOTHESIS IS WRONG. The filed hypothesis is "inferred callee arity is not applied at this call site". Arity inference does not fail here -- it SUCCEEDS. Decompiling the callee on its own (`kuna decompile <bin> 0x110ff --addr --option linuxsyscall on`) yields:
    undefined16 sub_110ff(long a0,int a1,unsigned long a2,unsigned long a3)
and the body proves the first two are real: a0 is dereferenced as the timer array (`v2 = *(long **)(a0 + v4 * 8)`) and a1 is the count (`if (a1 <= (int)v4)`). Those are precisely the two values the tester wanted. So the gap is NOT arity recovery; it is that the callee-side result never reaches the CALLER-side trial at 0x11477. A builder who takes the hypothesis literally would go tune arity inference, which already works.

FIVE ABLATIONS, ALL NEGATIVE -- do not repeat them:
1. BOTH CITED OPTIONS ARE DEFAULT-ON (`kuna catalog`: calleearity default on, varargstackargs default on), so the probe sentence "with calleearity and varargstackargs enabled" is VACUOUS. I ran the real A/B: `--option calleearity off`, `--option varargstackargs off`, and both off together -- all three emit the identical `watchdog()` at line 35. Neither option is on this path in either direction.
2. --define-function is not implicated: without it the call still renders `sub_110ff()`, argument-less.
3. Not the spill gate: `--option spillargtrial reload` AND `--option spillargtrial spill` both leave `watchdog()` unchanged.
4. `--option compareform canonical` does move the output on this exact command, so --option demonstrably reaches this decompile (control for the silent-unknown-option trap).
5. decompile-all cannot be used as the whole-binary control HERE: the caller at 0x113d9 is not discovered at all (only `sub_110ff` appears, as a definition) -- that is the sibling need elf-discovery-omits-explicit, not evidence about arguments.

THE SAME-CALLER CONTROL IS THE STRONGEST EVIDENCE. In this very function, `printf((char *)sub_10e27(v7,v6,v3,v2));` recovers FOUR register arguments from the identical MOV-immediate-then-call shape (0x1145e MOV ESI,0x2a / MOV EDI,0x11ccc / JMP 0x11495). So caller-side trial machinery is alive and working in this function; something kills the trials at 0x11477 specifically. Emission is fine too: the workaround reproduces, `--assert "prototype 0x110ff long watchdog(long *timers,int count)" --assert-strict` gives `watchdog((long *)0x13050,2)`.

WHERE TO LOOK (not proven to a line -- treat as the redirect, not as fact). The live gate is caller-side trial activation, `Funcdata::ancestor_op_use` -> `only_op_use` in decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs:2229 and :2405, whose own doc says a "branch/load/store/non-matching-call/persistent-output use means the Varnode is not exclusively a parameter". Note EDI is slot 0: if slot 0 goes inactive both arguments vanish, which matches the observed zero-argument call rather than a one-argument one. A builder should dump the trial state at 0x11477 before writing any code.

ACCEPTANCE TRAP -- READ BEFORE BUILDING. The acceptance asserts stdout_absent `watchdog\(\)` AND stdout_matches `watchdog\([^\n,]+,[^\n)]+\)`, i.e. it requires TWO OR MORE arguments. A fix that recovers only slot 0 (`watchdog(0x13050)`) FAILS its own acceptance. The callee-side recovery is four parameters, so propagating the callee prototype wholesale would emit four arguments and PASS -- but note only two are justified by the body, so passing the probe is not the same as being right here.

REGRESSION_OF IS UNEARNED -- FLAGGED FOR T_TRIAGE. This is filed regression_of call-args-discarded and the additive REGRESSION_FLOOR scores it 1110.90 against 20.79 for every blocker. Ablation 1 disproves the link: the mechanism call-args-discarded shipped (calleearity/varargstackargs, PR #367) is provably not on this path, and make test-stages is green on main. Note also that call-args-discarded carries probe_id: null and acceptance_id: null, so it CANNOT be replayed to settle this either way. This is a NEW call site the shipped arity options never reached -- same family, not a regression. Recommend stripping regression_of at T_TRIAGE so it ranks on its own merits.

SIBLING WORTH KNOWING: sub_110ff returns `undefined16` via a 128-bit `v3._0_16_`, the same pathology as alignment-push-pop-invents from the same tester/binary. Not merged; flagged.
