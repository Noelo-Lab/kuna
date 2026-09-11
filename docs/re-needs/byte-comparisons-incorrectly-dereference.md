---
need_id: byte-comparisons-incorrectly-dereference
title: Byte comparisons incorrectly dereference an eight-byte pointer
track: quality
status: open
severity: major
probe_id: p-a6d39efa3b1d
acceptance_id: a-f6c5440be709
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [68a915938fac2855fe6fb805]
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

Preserve byte-load semantics in password comparisons.

> **Byte comparisons incorrectly dereference an eight-byte pointer** (major, `68a915938fac2855fe6fb805`)
> With tailcallframe off, C declares an unsigned long long pointer and emits (uint4)*pointer for byte comparisons. Disassembly at 0x140054b87 shows MOVSX EAX,byte ptr [RAX]. Defining the accessor and asserting a char-pointer return did not repair the dereferences.

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
    "0x1400542a1",
    "--option",
    "tailcallframe",
    "off"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "unsigned long long \\*(v[0-9]+);[\\s\\S]*?\\(uint4\\)\\*\\1\\s*;"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.obf.exe",
    "binary_sha256": "3f19f467fcd48699f6f7494f180819d519826dba1a7552f3d9712b145aeae4cb",
    "binary_size": 696320,
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
    "0x1400542a1",
    "--option",
    "tailcallframe",
    "off"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "unsigned long long \\*(v[0-9]+);[\\s\\S]*?\\(uint4\\)\\*\\1\\s*;"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.obf.exe",
    "binary_sha256": "3f19f467fcd48699f6f7494f180819d519826dba1a7552f3d9712b145aeae4cb",
    "binary_size": 696320,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Merged register variables retain one pointer type across differently sized accesses without per-use casts.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/crackme.obf.exe --backend ida` — Reference server failed before registering; comparison unavailable.

## Instances

- `68a915938fac2855fe6fb805` (round 12, tester t-r12-68a91593)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_absent` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- sibling of `vm-temporary-declared-as` — near-sibling: one printed identifier carrying an inconsistent type across differently sized accesses. Kept separate (Jaccard 0.048): this one is a pointer whose element width is wrong, the other is a scalar-declared variable that is dereferenced.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD on the hypothesis, and the IR IS PROVEN INNOCENT: kuna lifts the byte load correctly and only the PRINTED DECLARATION is wrong. That narrows the fix from a load-width or type-propagation problem to the same p9 defect already recorded on vm-temporary-declared-as.

MEASURED on freshly built main b9029a8f, binary .kuna-repipe/arena/12/68a915938fac2855fe6fb805/target/crackme.obf.exe, sub_1400542a1, --option tailcallframe off (as the probe pins).

SYMPTOM REPRODUCES VERBATIM, including the probe regex. The emitted body declares "unsigned long long *v9; // rax" and contains nine "v8 = (uint4)*v9;" sites; the filed regex matches. The true instruction, from the disassembly recorded on the sibling need frame-based-tail-call, is:

  0x140054b82   e83d2d0200    CALL 0x1400778c4
  0x140054b87   0fbe00        MOVSX EAX,byte ptr [RAX]
  0x140054b8a   83f852        CMP EAX,0x52

A ONE-BYTE SIGNED load through the pointer the call returned, widened to four. kuna prints an EIGHT-byte load truncated to four. As C that reads seven bytes past the intended object and drops the sign extension.

THE FILED HYPOTHESIS IS LITERALLY TRUE -- one merged identifier carries one pointer type across differently sized accesses -- and the emitted text shows the merge is worse than "one pointer type". These four consecutive lines are all one variable v9:

  v9 = (char *)sub_1400778c4(&v14[0x58],6);
  v8 = (uint4)*v9;
  v7 = v8;
  v9 = (uint8)(uint8)v8;

v9 is DECLARED unsigned long long *, DEFINED as a char *, dereferenced as if eight bytes wide, and then assigned an INTEGER. Later uses read "*(char **)&v12[1] = v9;". No two of those agree, and as written this does not type-check: assigning char * to unsigned long long * is -Wincompatible-pointer-types and assigning uint8 to it is -Wint-conversion, both errors by default on GCC 14+ and clang 16+. That is the same headline recorded last tick on vm-temporary-declared-as: kuna is emitting C a current toolchain rejects.

WHAT THE REFUTATION ADDS, AND IT IS THE PART THAT SHOULD DRIVE THE FIX: THE LIFTED IR IS CORRECT AND MUST NOT BE TOUCHED. Two independent proofs, both already in the emitted text.

(1) The comparison downstream of the load is FOUR BYTES WIDE, not eight. The body emits "v4 = v7 - 0x52" guarded by "SBORROW4(v7,0x52)" and "(POPCOUNT(v4 & 0xff) & 1U) == 0". SBORROW4, not SBORROW8. If kuna had actually modelled an eight-byte load the subtraction and its overflow flag would be eight bytes wide. The value flow is the MOVSX path: one-byte load, sign-extend to four, CMP EAX,0x52. So the load in the SSA is one byte, exactly as the instruction says.

(2) kuna's own cast at the definition site is (char *). The type machinery knows the call returns a char pointer; it is the DECLARATION PRINTER that chose unsigned long long * for the merged HighVariable, and the dereference is then rendered against the declared type rather than against the LOAD's own size.

CONSEQUENCE FOR THE BUILDER: if the declaration said char *, then "(uint4)*v9" would be a sign-extending one-byte load in C -- i.e. EXACTLY MOVSX EAX,byte ptr [RAX], with no other edit anywhere. The text is one token away from correct. Do not go looking in the lifter, in load-width inference, or in the p6 merge; making the printed declaration agree with the casts kuna already emits fixes the semantics.

FAMILY, AND THIS IS THE THIRD INSTANCE. cluster.py kept this apart from vm-temporary-declared-as at Jaccard 0.048 and the split was right for filing, but they are ONE mechanism seen from two sides: there a merged variable is declared scalar and used as a pointer, here it is declared as the wrong pointer and used as a char pointer and as a scalar. Both are the p9 declaration-versus-cast disagreement on a merged HighVariable, both produce C that fails -Wint-conversion, and both are fixed by making the declaration agree with the casts already present. Different binaries, different challenges, different testers. A builder taking either should be given both; a fix validated on only one of them is not validated.

CROSS-LINK WORTH RECORDING: the escape hatch is closed here. v9 is a REGISTER temporary ("// rax"), and the round-12 need local-type-assertion-target establishes by seven-run control that --assert type on a register temp is REJECTED with "No symbol named" while stack slots and parameters are accepted. So an agent that spots this wrong declaration cannot correct it by hand either. The two needs compound.

ACCEPTANCE, WITH ONE CAUTION. It is stdout_absent of the same regex, anchored on the backreference to the declared name, so a genuine fix (declaration becomes char *) passes cleanly. But note the clause keys on the LITERAL text "unsigned long long *": any unrelated change that merely renames the variable or picks a different wrong pointer type would also pass it. Whoever closes this should confirm the emitted dereference is a one-byte signed load, not merely that the regex went quiet.

REFERENCE STATUS, unchanged and worth not re-attempting blindly: the tester's IDA comparison never happened (declib backend exited 1 before registering), so there is no reference pseudocode on either this need or its sibling.
