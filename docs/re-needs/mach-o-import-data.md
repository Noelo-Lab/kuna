---
need_id: mach-o-import-data
title: Mach-O import data slots make function names ambiguous
track: loader
status: open
severity: minor
probe_id: p-0574eaac8b66
acceptance_id: a-17a1cf6a6f7c
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [64c8b272b25df8732eebc2a6]
rounds: [12]
first_seen_round: 12
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

Decompile strcmp by its inventory name.

> **Mach-O import data slots make function names ambiguous** (minor, `64c8b272b25df8732eebc2a6`)
> Rejected strcmp as ambiguous between code at 0x100003ede and a lazy pointer at 0x100008030. Both candidates were labelled synthetic.

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
    "strcmp",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "selector .*strcmp.* is ambiguous"
    ]
  },
  "target": {
    "binary_rel": "bin/main",
    "binary_sha256": "979555f6b20fc5f024358ef26603f4c25e8db326941f86af30726eca9fea453e",
    "binary_size": 50080,
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
    "strcmp",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "strcmp\\("
    ]
  },
  "target": {
    "binary_rel": "bin/main",
    "binary_sha256": "979555f6b20fc5f024358ef26603f4c25e8db326941f86af30726eca9fea453e",
    "binary_size": 50080,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Import pointer slots compete with executable stubs during function selection.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `64c8b272b25df8732eebc2a6` (round 12, tester t-r12-64c8b272)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `bad-ux|decompile|exit_code,stdout_matches` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, measured, but the OBVIOUS FIX ROUTE IS WRONG-OUTPUT AND WOULD TAKE OUT A SIBLING NEED. Hypothesis as filed ("import pointer slots compete with executable stubs during function selection") is exactly right. Mapped both candidates to Mach-O sections by hand: 0x100003ede is __TEXT,__stubs initprot=5 (r-x, EXECUTABLE), section type 0x08 = S_SYMBOL_STUBS, size 6 (one jmp *(%rip)); 0x100008030 is __DATA,__la_symbol_ptr initprot=3 (rw-, NOT executable), type 0x07 = S_LAZY_SYMBOL_POINTERS, size 0. Not a one-off: ALL 9 duplicated names in this 26-row inventory are that exact shape (__stack_chk_fail/__strcpy_chk/printf/rand/scanf/srand/strcmp/strlen/time), each one executable stub + one non-executable slot, so the executable bit resolves 9 of 9 uniquely. CAUTION ON MY OWN NUMBERS: my first pass printed execs=2 for every pair because I tested initprot&1, which is the READ bit; execute is &4. Corrected it is 1 of 2 per pair. THE ACCEPTANCE IS REACHABLE AND HONEST: kuna decompile <bin> 0x100003ede --addr --option calleearity on --option varargstackargs on already exits 0 and emits "int strcmp(char *a0,char *a1)", so resolving the selector to the executable candidate satisfies stdout_matches "strcmp\(" with correct output, not a technicality. ELF CONTROL SAYS THIS IS MACH-O-LOCAL AND THE CORRECT BEHAVIOUR IS ALREADY SHIPPED ELSEWHERE: kuna functions /bin/ls --json gives 278 rows and ZERO duplicated names, i.e. the ELF loader already declines to emit a function row for .got.plt while naming the PLT stub. So the target state is "make Mach-O match ELF", not new policy. NOW THE PART A BUILDER MUST NOT GET WRONG. The tempting route -- drop non-executable import-pointer rows from the function inventory -- SHIPS WRONG OUTPUT AND REGRESSES THE OPEN SIBLING mach-o-objc-msgsend. Measured on that needs own binary (arena 11/5ab77f56, fat Mach-O, x86_64 slice): 84 rows, 9 non-executable. SEVEN of them have a stub twin (twins=2) and are safe to drop, but TWO ARE LONE ROWS WITH NO EXECUTABLE TWIN -- objc_msgSend 0x100004038 in __DATA,__got (twins=1) and dyld_stub_binder 0x100004028 in __DATA,__nl_symbol_ptr (twins=1). objc_msgSend 0x100004038 is LITERALLY the row mach-o-objc-msgsend is built on ("functions identifies objc_msgSend at 0x100004038"); deleting it destroys the only place that name appears and kills the sibling. __got/__nl_symbol_ptr slots reached by direct indirect call have no stub, so a blanket drop loses names permanently. THE SAFE RULE: disambiguate at SELECTION time -- when a selector matches several candidates, prefer the one in an executable section -- which only fires when a twin exists, resolves all 9 pairs here and all 7 there, and leaves every lone slot row intact. NOT AN OPPOSING ACCEPTANCE with the other same-binary sibling: typed-mach-o-tail asserts "data 0x100008030 int (*strcmp_ptr)(char *,char *)" and so already treats 0x100008030 as DATA, keyed by address in the data plane rather than by the function inventory; it agrees with this fix rather than fighting it.
- round 12 BUILDER b-r12-mach-o-import-da: symptom reproduced, hypothesis UPHELD and the refuter's safe rule implemented verbatim -- `ConsoleProgram::lone_executable_candidate` narrows a multi-candidate selector to the one entry in executable memory (`entry_is_executable`: a loader section carrying CODE, minus the loader's own import slots), and only when EXACTLY one candidate qualifies. Wired into BOTH name lookups: `resolve_entry` (decompile / decompile-all --functions / xrefs) and `find_entry_by_name` (disassemble / read), because narrowing only the first would have made `kuna decompile X memcpy` and `kuna disassemble X memcpy` answer at two different addresses -- measured on the vendored PE fixtures, 78 names resolved to the pointer word rather than the thunk. Inventory untouched, so the sibling `mach-o-objc-msgsend`'s lone rows survive (verified: that binary still reports 84 rows with objc_msgSend at 0x100004038 and dyld_stub_binder at 0x100004028, and both still resolve). Sweep over 163 vendored images: 134 of 155 duplicated names go error -> resolved, 0 go the other way, and the 21 that stay ambiguous are all same-named CODE definitions. CI twin is `tests/cli/mach-o-import-data.json`, retargeted onto the vendored `macho_imports` (printf at 0x1000005cc / 0x100003000, the same veneer+slot shape); the acceptance here stays on the dataset witness.
