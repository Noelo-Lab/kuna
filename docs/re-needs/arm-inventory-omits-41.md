---
need_id: arm-inventory-omits-41
title: ARM inventory omits 41 directly called validator functions
track: loader
status: open
severity: major
probe_id: p-c41c2026e1e6
acceptance_id: a-849f8de4d221
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5edb0b8533c5d449d91ae73b]
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

Enumerate and batch-decompile the validators called by main.

> **ARM inventory omits 41 directly called validator functions** (major, `5edb0b8533c5d449d91ae73b`)
> functions and decompile-all report 17 functions, omitting all 41 validators. main calls them explicitly, strings identifies their ownership, and direct decompilation succeeds. Explicit function definitions recover all checks.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/armageddon",
    "binary_sha256": "e96ae55f0db045e9812ba5daa9e3bf021218ef116f97975bc61bea298e2fa0bd",
    "binary_size": 25996,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_absent": [
      "\"address_hex\"\\s*:\\s*\"0x104f4\""
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
    "binary_rel": "bin/armageddon",
    "binary_sha256": "e96ae55f0db045e9812ba5daa9e3bf021218ef116f97975bc61bea298e2fa0bd",
    "binary_size": 25996,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "\"address_hex\"\\s*:\\s*\"0x104f4\""
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Whole-binary discovery does not incorporate ARM call targets recovered by single-function analysis and string xrefs.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/armageddon` — Reference failed: server exited with status 1 before registering. No IDA inventory was obtained. Kuna direct decompilation and original-instruction emulation confirm the omitted functions.

## Instances

- `5edb0b8533c5d449d91ae73b` (round 12, tester t-r12-5edb0b85)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `silent-failure|functions|exit_code,stdout_is_json,stdout_matches` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- sibling of `arm-inventory-invents-function` — opposite errors of the same ARM discovery pass (false negatives vs one false positive). Kept separate: fixing under-discovery does not fix over-discovery, and the tracks differ (loader vs quality).
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the count is exact: 42 direct BL targets in main are missing from the inventory, 41 of them real .text validators. The filed cause -- whole-binary discovery does not incorporate ARM call targets recovered by single-function analysis -- is right, and the measurement makes it unambiguous.

THE DECISIVE MEASUREMENT. kuna ALREADY KNOWS 0x104f4 is a called function; only 'functions' does not. 'kuna xrefs --to 0x104f4' returns count 1 with kind 'call', instruction 'bl 0x104f4' at 0x14aa8, from_function 'main' -- and main IS in the inventory (0x14a04, size 2224). So this is not a decode failure and not a reachability failure. Disassembling main and collecting its BL targets gives 45 distinct addresses; 42 are absent from the 17-function inventory (0x104f4, 0x106d8, 0x1082c ... 0x148bc), and the 42nd is 0x59684c, outside the image entirely -- a junk-word decode, see the obfuscation note below. The remaining 41 are exactly the validators the tester reported.

WHY THE INVENTORY CANNOT SEE THEM. EntryDiscoveryPass (kuna-analysis/src/analyzers/entry/mod.rs) documents this as a designed LOSS in its own header: 'General undirected call-target sweep is infeasible at the analyzer tier (no Listing) -- substituted by the _start->main idiom (oracle 4) + prologue patterns (oracle 5).' On this image the substitutes cover nothing: oracle 4 fires correctly (main IS found at 0x14a04, so kuna_armlibcmain works and is NOT the defect), oracle 3 is empty because .eh_frame is 4 bytes -- a bare terminator -- and .ARM.exidx is 8 bytes, one entry, and oracle 5 is documented x86-64-only. Everything past main's own entry depends on a call sweep that does not exist. Note the mechanism is arch-neutral; ARM is where it bites because the two table oracles are empty here.

DEAD LINE CLOSED, the same one as its ELF sibling: funcstart_patterns is default-OFF and is the obvious candidate, but 'decompile-all --option funcstart_patterns on' still returns 17 functions with no 0x104f4. Ablate through decompile-all -- 'kuna functions' takes no --option.

THREE THINGS THE BUILDER MUST HANDLE, all measured.
(1) THE BINARY IS BRANCH-OVER-JUNK OBFUSCATED and the probe target sits on it. At 0x104f4 the bytes decode as 'b 0x104fc' followed by a junk word at 0x104f8 ('stmdbls lr!,...'); the real prologue 'stmdb sp!,{r11,lr}' is at 0x104fc, then 'b 0x10508' over junk at 0x10504, and so on for the whole body. So a prologue-pattern approach lands on 0x104fc and FAILS the acceptance, which demands 0x104f4 exactly. The BL target is the only oracle that names the right byte.
(2) VALIDATE THE TARGET. 0x59684c comes from decoding a junk word as a BL; a sweep must keep the existing executable-section check or it will file garbage entries. This is also the direct link to the sibling need arm-inventory-invents-function -- same junk words, opposite error -- so a builder touching discovery here should read both.
(3) EXPECT BOUNDING TO MOVE. The inventory currently lists _INIT_0 at 0x104bc with size 17736, i.e. 0x104bc..0x14a04: one run-on that swallows all 41 validators, even though its real body ends at the 'b 0x1045c' at 0x104e8 with a literal pool at 0x104ec/0x104f0. Seeding the call targets is what gives the bounder something to stop at; check that _INIT_0 shrinks rather than assuming the change is purely additive.

Acceptance is sound: a positive stdout_matches on address_hex 0x104f4, which cannot be satisfied by losing anything.
- round 12 REFUTER: hypothesis **upheld**. REFUTER ADDENDUM (round 12, captain in-tick, while b-r12-arm-inventory-om is LIVE). VERDICT UNCHANGED AT UPHELD -- this is additional mechanism evidence for the builder, not a re-judgement.

THE SPECIFIC BLOCKER, WHICH THE FILED HYPOTHESIS DOES NOT NAME. Measured on b9029a8f: kuna functions <armageddon> --json returns 17 entries and the inventory is a CONTIGUOUS PARTITION -- every function's end is exactly the next one's start:
    _INIT_0   0x104bc + 17736 = 0x14a04   (== main)
    main      0x14a04 + 2224  = 0x152b4   (== sub_152b4)
    sub_152b4 0x152b4 + 244   = 0x153a8   (== _DT_FINI)
_INIT_0 CLAIMS 17,736 BYTES, 0x104bc..0x14a04, AND ALL 41 VALIDATORS LIVE INSIDE THAT RANGE. 0x104f4 -- the exact address this need's acceptance requires -- is inside it. So the validators are not "missed by a call-target sweep" so much as already OWNED by a single loader-asserted symbol whose size is being derived from the next known symbol rather than from flow. That is the thing to attack.

0x104f4 IS A REAL FUNCTION ENTRY, and it wears the same obfuscation as the rest of the image:
    0x104f4  b 0x104fc               <- skips the junk word at 0x104f8
    0x104f8  stmdbls lr!,{...}       <- junk, never executed
    0x104fc  stmdb sp!,{r11,lr}      <- the real ARM prologue
A sweep that requires a prologue AT the entry address will reject it; the prologue is one branch in.

FOUR DEAD ABLATIONS -- DO NOT RE-SPEND THEM. All on decompile-all --json (kuna functions takes NO --option, so ablate through decompile-all). Every one identical to baseline, n=17 with 0x104f4 absent:
    baseline                        n=17
    --option funcstart_patterns on  n=17
    --option poolentry on           n=17
    --option aif on                 n=17
funcstart_patterns is the full Ghidra FunctionStartAnalyzer byte-pattern port, poolentry the ARM literal-pool reconstruction, aif the Aggressive Instruction Finder gap-walk. None of the three shipped discovery levers reaches this. Neither does the gap-walk being off explain it, since turning it on changes nothing.

THE SIBLING IS NOT AN OPPOSING ACCEPTANCE -- CHECKED, AND IT MATTERS FOR HOW YOU FIX THIS. arm-inventory-invents-function is the SAME binary and the SAME command, and it demands that 0x152b4 be ABSENT while this need demands 0x104f4 be PRESENT. That looks like a trap and is not one: they are the same contiguous-partition defect in opposite directions. _INIT_0's boundary is far too large and swallows the validators; main's is too small and its remainder is re-filed as a spurious sub_152b4 at a branch-skipped junk word (0x152b0 is b 0x152b8, so 0x152b4 is never executed, and 0x152b8 branches back to 0x150f4 INSIDE main). Deriving boundaries from flow rather than from the next symbol moves both the right way at once. I refuted that sibling this tick and OVERTURNED its filed cause; its Decision log carries the disassembly.

CAUTION ON YOUR OWN ACCEPTANCE: it asserts on 0x104f4 specifically. Splitting _INIT_0 at exactly one address passes it without recovering the other 40. The tester's report is that all 41 are omitted and that explicit --define-function recovers all of them, so treat 41 as the target and 0x104f4 as the probe.

- BUILDER RESULT (worker b-r12-arm-inventory-om, no new option): acceptance arrived UNBOUND; bound to the dataset witness `bin/armageddon` by sha256 match against the arena copy, which leaves `acceptance_id` unchanged. The filed cause is upheld but the layer is one step further in: `listing/walk.rs` ALREADY claims a function at every direct CALL target -- what it lacked was a ROOT. `passes::run_listing_consumers` rebuilt its seed set from the object (`listing_seeds` = funcsyms + `entry::collect_entries`), so `armlibcmain`'s non-PIE ARM `main` -- a separately gated STANDALONE pass -- was committed to the inventory yet never seeded the walk, and on this image `main` is the only address that reaches the body. `kuna xrefs` seeds from the COMMITTED inventory, which is exactly why it already named all 41. Fixed by handing the caller's merged load-time entries down as extra walk roots under the existing `funcstart_patterns` gate (exec-filtered like every other seed). Witness: 17 -> 57 functions, all 41 validators, `_INIT_0` 17736 -> 56 bytes, `0x59684c` still rejected, and `sub_152b4` dropped -- so this ALSO satisfies the sibling `arm-inventory-invents-function` acceptance. 803 decbench binaries swept before/after: 0 inventories changed.
