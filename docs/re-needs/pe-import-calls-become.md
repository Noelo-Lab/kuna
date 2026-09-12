---
need_id: pe-import-calls-become
title: PE import calls become calls to raw hint/name RVAs
track: quality
status: closed
severity: major
probe_id: p-ba994cd01240
acceptance_id: a-983e58b7c36b
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f6433c5d40ad448cae1]
rounds: [12]
first_seen_round: 12
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp, decompiler/crates/kuna-console]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/576
closed_in_round: 12
closing_pr: 576
reject_reason: null
---

## Symptom

Read the dialog input calls as GetDlgItemTextA, preserving their import identity.

> **PE import calls become calls to raw hint/name RVAs** (major, `5ab77f6433c5d40ad448cae1`)
> Even with peimportcall on, emitted (*(void *)0x173dc)(a0,0x68,...). Instructions load ESI from IAT slot 0x401078 and call ESI; kuna names that slot GetDlgItemTextA. The emitted constant is its on-disk hint/name RVA. Separately encountered conversion-result loss resembling already-filed bytecode-reader-return-discarded; explicit unsigned long long prototypes restored those results, so that symptom was not refiled.

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
    "0x40341c",
    "--addr",
    "--option",
    "peimportcall",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\(\\*\\(void \\*\\)0x173dc\\)"
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
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x401028",
    "--addr",
    "--option",
    "peimportcall",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "0x173dc",
      "0x10c4"
    ],
    "stdout_matches": [
      "GetDlgItemTextA\\(a0,0x68,0x404000,0x20\\);",
      "GetDlgItemTextA\\(a0,0x69,0x404020,0x20\\);"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_readonly_i386.exe",
    "binary_sha256": "9cd4f3d091d647eeb36df883e2f3f063d34ac7a508c71c9f28f26683f05ac6f6",
    "binary_size": 1024,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_readonly_i386.exe",
    "selector": "0x401028",
    "selector_kind": "addr"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Executable-section IAT bytes may be constant-folded before import resolution preserves their symbols.

## Refutation

_not yet refuted_

## Reference

- `objdump -p target/GiTS_2010_Crypto_Crackmes.zip.__x/crypto4.exe` — USER32.dll has FirstThunk RVA 0x106c; its fourth entry is GetDlgItemTextA, hint/name RVA 0x173dc. Thus 0x401078 is its IAT slot. IDA was attempted but failed to start.

## Instances

- `5ab77f6433c5d40ad448cae1` (round 12, tester t-r12-5ab77f64)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_absent,stdout_matches` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- sibling of `decompiling-executable-section-iat` — shared root condition (IAT data living inside .text) but different fixes: import identity through a constant-folded call vs refusing a non-code decompile target. Kept separate.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the cause is a SECOND SHIPPED OPTION, not the import machinery: option litpoolconst (DIV-136, default ON, P1 code-data-partition) folds the IAT read before peimportcall can bind it.

MEASURED ON b9029a8f (release binaries rebuilt 05:39), arena binary crypto4.exe.

The bytes settle the fold exactly. .text is the ONLY code section (VMA 0x401000, size 0x1656a, flags CONTENTS ALLOC LOAD READONLY CODE) and it holds the import descriptors (0x16cfc), the FirstThunk arrays (RVA 0x1000-0x11xx) AND the hint/name strings (0x16e00-0x172xx). At file offset 0x478 = RVA 0x1078 = VA 0x401078 the four bytes are dc 73 01 00 = 0x000173dc. kuna emits (*(void *)0x173dc)(a0,0x68,...). The emitted constant IS the slot's on-disk contents, byte for byte. That is a read-only fold of the LOAD, and once the CALLIND target is a constant, ActionDeindirect takes its constant arm, finds no function at 0x173dc, and prints the raw pointer.

THE DECISIVE ABLATION: --option litpoolconst off restores full import identity on the SAME command. Line 66 becomes v4 = GetDlgItemTextA(a0,0x68,v12,0x20), plus SendMessageA(a0,0xa1,2,0), SetBkMode(a2,1), SendMessageA(v5,0x80,1,v8) -- correct names, correct arity, correct arguments. Whole-binary census over decompile-all, 93 functions: DEFAULT = 0 named import calls and 115 folded (*(void *)0x1xxxx) calls; litpoolconst off = 19 named import calls (over a 10-name sample) and 0 folded. Not one Win32 API is named anywhere in this program at the default.

WHY litpoolconst fires here and nowhere normal -- the CONTROL settles it. decompiler/crates/kuna-analysis/tests/fixtures/pe_imports.exe puts its IAT in .idata at 0x14000d000 with flags CONTENTS ALLOC LOAD DATA, i.e. WRITABLE. On that binary, at the default: 0 folded indirect calls, 14 named import calls, and litpoolconst on vs off is BYTE-IDENTICAL (118744 chars both) -- the option is structurally inert because the slots are writable. Same kuna, same defaults, opposite outcome, and the only difference is the section permission. litpoolconst's warrant is "allocated, executable, non-writable, file-backed"; a PE that puts its IAT inside .text satisfies all four.

THE SOUNDNESS HOLE, IN litpoolconst's OWN WORDS. Its summary justifies the fold as "mapped r-x, so a store to them faults and the image's copy IS the run-time value". That premise is FALSE for a PE IAT slot: the Windows loader writes every one of them at load time (that is what an Import Address Table is for), so the image's copy is provably NOT the run-time value -- it is the hint/name RVA, which is exactly the wrong number kuna prints. The same summary already names the risk: "the direction that matters on a corpus of packers and protectors, where a data section's flags are least trustworthy". This is that case.

FOUR DEAD LINES CLOSED OFF. (1) peimportcall on vs off on the probe command is BYTE-IDENTICAL, 5945 bytes each -- the option the tester was told to flip on is default-ON and does nothing on this binary, so "peimportcall is a partial fix" is false. (2) The import parsing is NOT broken: kuna functions names all 176 entries correctly, GetDlgItemTextA at 0x401078, so the INT/IAT lockstep walk succeeds. (3) It is not a single-function blind spot: 115 folded calls across the whole binary. (4) It is not PE32-vs-PE32+ -- the working control is PE32+ and the failing image PE32, but the control's litpoolconst inertness is due to section flags, not width.

WHY IT IS WRONG OUTPUT AND NOT UGLY OUTPUT: an agent reading this function sees (*(void *)0x173dc)(a0,0x68,v12,0x20) and has no route to the name -- 0x173dc is not an address in the running program, it is a file offset into a name string. It also silently kills peimportcall's OTHER half, the Win32 no-return effect, so a function ending in ExitProcess runs on.

ACCEPTANCE IS SOUND. It requires both distinct `GetDlgItemTextA` calls with every argument and broadly excludes the original `0x173dc` plus the reduction's corresponding `0x10c4`. Deleting either call, dropping an argument, or cosmetically changing the raw call cannot pass. The deterministic 1 KB PE32 reduction puts code, IAT and imports in one executable/read section; unlike the dataset target it is vendorable into `tests/cli/`.

CROSS-LINK: this is the PE twin of the PIE-dynrelocs family -- a relocated slot whose on-disk value is not its run-time value, folded because the section looked immutable.
- round 12 B_IDLE (captain): **probe target bound by hand; this need is now dispatchable.** Both
  its probe blocks used `{{BIN}}` with no `target`, so every replay died with "`{{BIN}}` used but the
  context supplies no bin" -- clauses `[]`, repeat 0 -- and two consecutive `B_PLAN` ticks had to
  skip it because an acceptance that cannot run can never flip, however good the fix. The binary is
  identified from the filing tester's own `toolcalls.jsonl`, by an argv that matches the probe cmd
  exactly, not by guessing from the arena: `kuna decompile target/GiTS_2010_Crypto_Crackmes.zip.__x/crypto4.exe 0x40341c --addr --option peimportcall on`. Bound to
  `bin/GiTS_2010_Crypto_Crackmes.zip.__x/crypto4.exe` (sha256 `f88284c9bfe6e8581261ee224395c83d79507b26a64f81614631d78498cf63b9`, 123904 bytes,
  source `dataset`, resolved as `challenges/5ab77f6433c5d40ad448cae1/<rel>`). Replayed on unpatched main `17da9472`: **reproduction PASSES** and the original acceptance fails.
- round 12 BUILDER: the refuter's proposed PE-range subtraction was narrowed to the general P3 invariant already encoded by `externref`. `ActionVarnodeProps` snapshots `is_external_ref()` and skips its entire `fillin_read_only` branch for that Varnode, whether entered through `readonly`, `dynrelocs`, or `litpoolconst`. Loader ranges and option gates are untouched, so `peimportcall off` remains raw and ARM literal-pool constants still fold.
