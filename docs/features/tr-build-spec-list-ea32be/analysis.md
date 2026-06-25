# Analysis — `tr.o::build_spec_list` (angr vs kuna)

angr testcase: `test_decompiling_tr_build_spec_list` · selector `build_spec_list`
(arch `x86_64`) · binary `binaries/tests/x86_64/decompiler/tr.o` (coreutils `tr`).

## Reproduction

`tr.o` is an **ELF ET_REL** relocatable object. kuna's loader is PT_LOAD-only, so
`kuna decompile tr.o build_spec_list` fails with *"Unable to load 512 bytes"* — kuna
produces **no output at all** (the known ET_REL loader gap; existing proposal PR #37).
The `scripts.pipeline.compare` side-by-side (`angr-vs-kuna.txt`) shows angr's output and
an empty kuna column for that reason.

To get a *fair* decompiler comparison (not just the loader gap) I:

1. Extracted `build_spec_list` (.text off `0xbc0`, 755 bytes) and verified that the
   intra-`.text` `call`/`jmp` displacements are already resolved (only 12 external/`.LC`
   relocations needed patching) — see `probe.xml` (a relocated bytechunk).
2. Linked `tr.o` into a standalone **non-PIE ELF** with stub externs
   (`stubs.c` → `tr_synth.elf`, `build_spec_list` at `0x402370`) so kuna's **full
   pipeline** (analysis tier + decompiler) runs. Output: `kuna-fullpipeline.c`.

## What angr does better (full-pipeline comparison)

With the full pipeline, kuna's output is structurally **close** to angr's (DWARF types
`E_string*`/`Spec_list*` recovered; the `do/while` + trailing `for` loops match). Three
differences remain:

| # | Difference | Owning stage | Status |
|---|---|---|---|
| 1 | kuna shows the `-fstack-protector` canary epilogue (`if (canary != *(fs:0x28)) __stack_chk_fail()`); angr strips it. | S7 | **Covered** by `option stackguard` (default-off). Do not duplicate. |
| 2 | kuna emits **11 gotos** with crossing edges (`goto label_40242e` jumps from an `else` branch back into a `then` branch); angr emits **4 gotos**, collapsing short-circuit conditions into compound `&&`/`||` (e.g. `if (!es_match() && !es_match() || !find_closing_delim(...))`). | **S7 structuring** | **Uncovered** — angr's SAILR condition-folding / goto-reduction. |
| 3 | kuna declares `uint8 v12;` three times and **uses `v5`/`v14`/`v15` without declaring them** (would not compile); angr declares each variable once. | S9 emit / decl | **Uncovered**, but a *generic* kuna decl/emit bug — not an angr algorithm. |

`noreturn_known` (default-on) **already** marks `__stack_chk_fail` no-return: the
full-pipeline output has no trailing dead code (the bytechunk-only probe did, because the
analysis tier does not run on the XML path).

## The one concrete gap to close

Difference **#2 — SAILR-style condition folding / crossing-edge goto reduction** — is the
only difference that is a genuine *angr algorithmic strength* (the others are covered or a
generic bug). It is owned by **S7 region structuring**.

## Hypothesis & scope

A faithful port would recover short-circuit `&&`/`||` compound conditions and virtualize
crossing-edge gotos in the structured region graph (angr's `Phoenix`/`RegionSimplifier`
condition synthesis + the SAILR `CrossJumpReverter`/`DuplicationReverter` deoptimizers).
This is **not** modelable as one option-gated `Action`/`Rule` like `kuna_loweredswitch.rs`:
it is new structuring infrastructure touching the S7 region/structuring core well beyond a
single gated early-return. Per **Hard Rule 7** this is a **LARGE** feature → it goes through
a draft `[PROPOSAL]` PR for human go/no-go, not direct implementation. See `proposal.md`.

A decider subagent independently reached the same call (verbatim in `record.json`
`decisions`): target = SAILR condition folding, stage S7, scope **large**, action
**proposal**.
