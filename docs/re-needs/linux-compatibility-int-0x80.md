---
need_id: linux-compatibility-int-0x80
title: Linux compatibility int 0x80 stays opaque in an x86-64 image
track: quality
status: closed
severity: major
probe_id: p-99117a0902de
acceptance_id: a-a3dee50cabfd
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [6736b3a09b533b4c22bd2b9f]
rounds: [10, 12]
first_seen_round: 10
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/579
closed_in_round: 12
closing_pr: 579
reject_reason: null
---

## Symptom

Expose EAX=1 and EBX=0 as the compatibility exit syscall using the explicit Linux option.

> **Linux compatibility int 0x80 stays opaque in an x86-64 image** (major, `6736b3a09b533b4c22bd2b9f`)
> linuxsyscall on still emits (*(void *)swi(0x80))(); with both inputs discarded. Its catalog explicitly excludes x86-64, so this is a scope extension rather than a regression. Also encountered already-filed x86-64-syscall-wrapper at 0x1475; not filed again.

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
    "0x1000",
    "--option",
    "linuxsyscall",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "swi\\(0x80\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/no-standards",
    "binary_sha256": "5cb1538c58846236e962497d1ccab88e02cc958a031d2c96c150e11c0d51ef27",
    "binary_size": 21840,
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
    "0x1000",
    "--option",
    "linuxsyscall",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?m)^\\s*sys_exit\\(0\\);\\s*$"
    ],
    "stdout_absent": [
      "swi\\(0x80\\)",
      "\\bsyscall\\s*\\(",
      "sys_write\\s*\\(",
      "sys_exit\\(0,"
    ]
  },
  "target": {
    "binary_rel": "bin/no-standards",
    "binary_sha256": "5cb1538c58846236e962497d1ccab88e02cc958a031d2c96c150e11c0d51ef27",
    "binary_size": 21840,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The documented language gate excludes compatibility int 0x80 in long mode; extend the explicit Linux ABI option.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/no-standards` — Server exited with status 1 before registering. No reference decompilation was obtained.

## Instances

- `6736b3a09b533b4c22bd2b9f` (round 10, tester t-r10-6736b3a0)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 10 REFUTER: hypothesis **upheld** (was inconclusive). REFUTER RAN (captain, in-tick, 2026-09-08). UPHELD, and sharpened into the one thing that decides whether the fix is right or wrong. (1) Repro confirmed on .kuna-repipe/arena/10/6736b3a09b533b4c22bd2b9f/target/no-standards (ELF 64-bit PIE, x86-64, not stripped): 'kuna decompile BIN 0x1000 --option linuxsyscall on' emits '(*(void *)swi(0x80))();' with both inputs gone. Disassembly at 0x1000 is ENDBR64 / MOV EAX,0x1 / MOV EBX,0x0 / INT 0x80 / RET -- a locally-resolvable constant number, exactly the shape the pass already recognizes. (2) The gate is real, documented, and is a PURE ADDRESS-SIZE TEST, not a defect: kuna_linuxsyscall.rs:508 resolve_abi returns None when manage.get_default_code_space().get_addr_size() != 4, and the module header (kuna_linuxsyscall.rs:72) names refusal 3 'the language must be x86-32'. phases.toml:1906 says the same in the catalog prose ('any language that is not x86-32'). Nothing else declines this site: the number IS a full-width EAX constant and IS in the table. So the need is a scope extension, exactly as filed. (3) *** THE TRAP, AND THE REASON THIS VERDICT IS WORTH MORE THAN 'UPHELD'. An x86-64 image invites a builder to reach for the x86-64 syscall table. THAT PRODUCES A FALSE STATEMENT ABOUT THIS VERY CALL. *** In long mode 'int 0x80' does NOT enter the native table -- it enters the 32-bit compat table (ia32_sys_call_table), which keeps i386 NUMBERING. kuna's existing SYSCALL_TABLE is already i386-numbered: kuna_linuxsyscall.rs:141 (1,"exit",1), :144 (4,"write",3), :188 (60,"umask",1). Ground truth here is EAX=1 EBX=0 => exit(0), which is what the tester read. Under x86-64 native numbering 1 is write and 60 is exit, so a native-table fix renders this as a 3-argument write -- wrong name, wrong arity, and precisely the 'printing a name with the wrong argument count is a false statement about the call, not a partial one' failure the option's own doc refuses to make. THE EXISTING TABLE IS THE CORRECT TABLE FOR int 0x80 ON x86-64; do not add a second one. (4) So the fix is narrower than 'support x86-64 syscalls': relax the address-size test for the int-0x80 lowering ONLY, keeping the i386 table and the existing ARG_REGISTERS (EBX,ECX,EDX,ESI,EDI,EBP -- correct on the compat path, which reads the low 32 bits). resolve_abi's own comment already states the mechanism ('x86-64 resolves EAX..EBP as sub-registers, so the address-size test is what excludes it'), so the register lookup needs to accept a 32-bit sub-register of a 64-bit register rather than demanding st.size == 4. Do NOT relax the gate for SYSCALL/SYSENTER on x86-64 -- that is the native ABI (RAX + RDI,RSI,RDX,R10,R8,R9) and a different table; this need is int 0x80 only. (5) Also note the extrapop=0 half of the existing pass still applies: int 0x80 pushes no return address in long mode either.
- round 10 T_TRIAGE (captain, 10:39Z tick): RETRACKED tooling -> quality, touches kuna-cli -> kuna-decomp, AND THIS ONE HAD A LEASE CONSEQUENCE, not just a label. The need is 'extend linuxsyscall to x86-64', and linuxsyscall is an ENGINE option: decompiler/crates/kuna-decomp/phases.toml:1897. Widening its arch scope edits phases.toml, p0_knowledge/options.rs and docs/options.md. Filed as tooling it held NO leases at all (select.py TRACK_RESOURCES['tooling'] == []), so a builder would have edited phases.toml and the catalog counters while another quality builder did the same -- the identical '85 -> 86' edit on two branches that merges CLEANLY to the wrong number, which is the exact failure the lease algebra exists to prevent (select.py's own comment says so). Nothing in kuna-cli is involved. Quality also correctly forces the DIV row and the tests/stages testcase this arch extension needs.
- round 10 CAPTAIN (18:35Z tick, B_DRAIN off-critical-path repair): PROBE TARGET BOUND -- this need was STRUCTURALLY UNCLOSABLE as filed. Both arms carried no `target` block, so `verify --acceptance-suite` returned `ProbeError: {{BIN}} used but the context supplies no bin` with unrunnable=true / transition=indeterminate: a builder could have done the work and still not closed it. Bound both arms to the single ELF/PE candidate under ds/challenges/<hexid>/ (sha256 + size captured from the dataset copy, binary_source=dataset). Verified post-fix: unrunnable=false, passed=false, transition=unchanged -- i.e. the acceptance now FAILS honestly and can be seen to flip. NOTE probe ids did NOT move: probe_id_of() is keyed on cmd+expect only (needs.py:269), so binding a target is id-stable and breaks no historical reference. PROMOTION CAVEAT: binary_source is `dataset` and `verify --promote` refuses that verbatim (CI has no dataset, --force does not help) -- vendor an in-repo fixture in the SAME PR or B_DONE cannot promote this probe into tests/cli/.
