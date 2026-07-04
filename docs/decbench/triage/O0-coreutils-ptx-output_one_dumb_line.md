---
case_id: O0-coreutils-ptx-output_one_dumb_line
group_id: coreutils::output_one_dumb_line
status: needs-proposal
tier: M
margin: 98
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: null
option_closing: null
root_cause_family: ternary-ite-materialization
feature_slug: ite-ternary-recovery
scope: proposal
angr_ref: angr.analyses.decompiler.optimization_passes.ite_region_converter.ITERegionConverter (+ region_simplifiers/expr_folding single-use inlining)
kuna_stage: S8 readability-rewrites (+ S9 PrintC ternary token; S6 s6_variables/merge.rs for the v1 collision)
---

## Side-by-side

Metrics: angr(stored) 80 loc / 10 ifs / 7 ternaries; kuna(stored) 98 loc / 16 ifs / 0 ternaries; kuna(fresh) byte-identical to stored (98 loc / 16 ifs / 0 ternaries). Source `ptx.c output_one_dumb_line` has 10 ifs + 6 inline `(flag ? truncation_string_length : 0)` ternaries in `print_spaces` arguments.

angr (stored) — trimmed to the two representative regions (full body is complete, 80 loc, ends `return putchar_unlocked(10);` — NOT truncated):

```c
int output_one_dumb_line(void)
{
    unsigned long long v1;  // rcx

    if (g_414142 != 1) { ... }                 /* reference field, if/else — same as kuna */
    if (g_414840 < *((long long *)&g_414848))
    {
        sub_4041da(g_414840, *((long long *)&g_414848));
        if (g_414850)
            fputs_unlocked(g_414028, stdout);
        sub_4041a6(g_414820 - g_414020 - g_414868 + g_414860
                   - (!g_414870 ? 0 : g_414838)          /* <-- inline ternary */
                   - *((long long *)&g_414848) + g_414840
                   - (!g_414850 ? 0 : g_414838));        /* <-- inline ternary */
    }
    else
    {
        sub_4041a6(g_414820 - g_414020 - (g_414868 - g_414860) - (!g_414870 ? 0 : g_414838));
    }
    ...
    if (g_4148a0 < *((long long *)&g_4148a8))
    {
        sub_4041a6(g_414820 - g_414888 + g_414880 - (!g_414890 ? 0 : g_414838)
                   - *((long long *)&g_4148a8) + g_4148a0 - (!g_4148b0 ? 0 : g_414838));
        ...
    }
    else if ((g_414140 || g_414141) && g_414142)
    {
        v1 = (!g_414890 ? 0 : g_414838);       /* residual ternary assignment from ITERegionConverter */
        sub_4041a6(g_414820 - (g_414888 - g_414880) - (!g_414890 ? 0 : g_414838));
    }
    ...
    return putchar_unlocked(10);
}
```

kuna (stored == fresh) — trimmed to the same region (branchflip WARNING comment lines elided):

```c
void output_one_dumb_line(void)
{
  int8 v1;
  ...
  if (dat_14848 <= dat_14840) {
    if (dat_14870 != '\0') { v1 = dat_14838; } else { v1 = 0; }
    sub_41a6(((dat_14820 - dat_14020) - (dat_14868 - dat_14860)) - v1);
  }
  else {
    sub_41da(dat_14840,dat_14848);
    if (dat_14850 != '\0') { fputs_unlocked(dat_14028,dat_140e8); }
    if (dat_14870 != '\0') { v1 = dat_14838; } else { v1 = 0; }   /* diamond 1 (rax web) */
    if (dat_14850 != '\0') { v1 = dat_14838; } else { v1 = 0; }   /* diamond 2 (rcx web) OVERWRITES v1 */
    sub_41a6(((((dat_14820 - dat_14020) - (dat_14868 - dat_14860)) - v1)
              - (dat_14848 - dat_14840)) - v1);   /* BOTH reads get diamond-2's value: WRONG */
  }
  ...
  if (dat_148a8 <= dat_148a0) { ... }
  else {
    if (dat_14890 != '\0') { v1 = dat_14838; } else { v1 = 0; }
    if (dat_148b0 != '\0') { v1 = dat_14838; } else { v1 = 0; }
    sub_41a6((((dat_14820 - (dat_14888 - dat_14880)) - v1) - (dat_148a8 - dat_148a0)) - v1);
    ...
  }
  ...
  putchar_unlocked(10);
  return;
}
```

ghidra (stored, decbench's real Ghidra, same region) — for the kuna-specific delta:

```c
  if (DAT_00114840 < DAT_00114848) {
    FUN_001041da(DAT_00114840,DAT_00114848);
    if (DAT_00114850 != '\0') { fputs_unlocked(PTR_DAT_00114028,stdout); }
    lVar1 = DAT_00114838;
    if (DAT_00114870 == '\0') { lVar1 = 0; }     /* hoisted copy + guarded overwrite, TWO vars */
    lVar2 = DAT_00114838;
    if (DAT_00114850 == '\0') { lVar2 = 0; }
    FUN_001041a6(((((DAT_00114820 - DAT_00114020) - (DAT_00114868 - DAT_00114860)) - lVar1)
                 - (DAT_00114848 - DAT_00114840)) - lVar2);   /* semantically correct */
  }
```

## Analysis

**Symptom.** The source function embeds six `(flag ? truncation_string_length : 0)` ternaries inside `print_spaces(...)` argument expressions. At O0, gcc compiles each as a full CFG diamond (`test; je; mov rax,[tsl]; jmp; mov eax,0`, verified by objdump at 0x5898-0x58b1 and 0x58c8-0x58e1). angr converts every diamond back into a ternary *expression* and inlines it into the consuming argument, so its decompiled CFG matches the source CFG exactly (GED 0, 10 ifs + 7 ternaries). kuna materializes every diamond as a full `if/else` statement pair assigning a temp (16 ifs, 0 ternaries), inflating the CFG by ~6 two-arm diamonds — the 98-point GED. Fresh kuna (current main, post-Jun-29 SAILR PRs) is byte-identical to the stored run: not fixed.

**Why angr wins.** `ITERegionConverter` (angr/analyses/decompiler/optimization_passes/ite_region_converter.py, in the exact venv that ran: `~/.virtualenvs/decbench/lib/python3.10/site-packages/`) transforms `if (c) {x = a} else {x = b}` regions into `x = c ? a : b` (AIL `ITE` expression), and the region-simplifier expression folding (`region_simplifiers/expr_folding.py`) inlines the single-use assignment into the call argument — jointly reproducing the source's inline ternary. The one residual `v1 = (!g_414890 ? 0 : g_414838);` assignment in angr's output is the visible signature of the converter.

**Why this is Ghidra-inherited in nature but kuna-specific in degree.** No Ghidra-lineage engine can emit `?:` — kuna's S9 printer (port of PrintC) has no ternary token (the only "ternary" hits in kuna-decomp are 3-input p-code op flags in `s5_types/typeop.rs` / `substrate/op.rs`). Hence ghidra=80, ida=65, binja=114 all lose here too. But kuna (98) is 18 worse than real Ghidra (80) for a kuna-specific reason: real Ghidra renders each diamond as a hoisted copy + guarded overwrite (`lVar1 = X; if (c=='\0') lVar1 = 0;` — one arm, fewer nodes/edges) with **two distinct variables**, while kuna emits full two-arm if/else diamonds and **over-merges the two temporaries into one HighVariable `v1`**.

**Bonus correctness bug (kuna-specific, real).** In the machine code the two diamond results live in different registers (rax at 0x58b1, rcx at 0x58e1) and the first is consumed by `sub %rax,%rdx` *before* the second is defined. kuna's speculative merge (S6, `decompiler/crates/kuna-decomp/src/s6_variables/merge.rs`) fuses both webs into one `v1` (their p-code covers don't intersect, so the cover test passes), but expression aggregation then prints the first read *textually after* the second definition: `sub_41a6((... - v1) - ... - v1)` computes the tail_truncation value for both slots — semantically incorrect C whenever `before_truncation != tail_truncation`. Real Ghidra keeps `lVar1`/`lVar2` apart. Reproduces with `--option namestyle ghidra` (so not a vN naming collision) and with `--option regionstructure off --option branchflip off` (so not a DIV-default artifact) — it is in the ported merge/expression-aggregation interaction itself (upstream guards this with COPY trims around ordering-unsafe merges; worth its own correctness issue independent of GED).

**Option sweep.** Full `kuna catalog --json` review: no option addresses ternary/ITE expression recovery. Tested: `namestyle ghidra` (no structural change), `regionstructure off + branchflip off` (still 16 ifs, still single `v1`, loc 98→87 only from dropped WARNING comments). `foldcallret`/`dedupitetail`/`condexeplace` are already on and orthogonal. Nothing closes the gap: not covered-by-option.

**Artifact check** (quick; artifact_suspect=false): angr's stored block is a complete function — all 10 source branches present, ends with the source's final `putchar('\n')` as `return putchar_unlocked(10);`. Source CFG is non-degenerate (10 ifs). phoenix=26 differs from angr=0, and ghidra/ida/binja cluster at 65-114, consistent with a genuine ternary-recovery win, not a truncation artifact. Rescore skipped (tier M; fresh output byte-identical to stored, so fresh GED = stored 98).

**Siblings:** none in the group.

## Proposed fix

**Mechanism (multi-part, hence [PROPOSAL]):** an ITE/SELECT recovery in kuna:

1. A new kuna pseudo-op `KUNA_SELECT(cond, a, b)` (kuna opcode/ElementId range 4000+, addlflags 0x1000+), with a `TypeOpInfo` entry (3-input, output type = meet of a/b).
2. An option-gated recognizer (working name `ite-ternary-recovery`) that pattern-matches a two-arm diamond whose arms each contain a single side-effect-free definition feeding one MULTIEQUAL at the join (exactly angr's `ITERegionConverter` precondition), replaces the MULTIEQUAL with `KUNA_SELECT` and collapses the diamond blocks. Running before S6 merge means the consuming SUB then reads the SELECT directly, so the existing expression-aggregation inlines it into the call argument with **zero** new variables — matching angr's folded form without needing a separate expr-folding pass, and incidentally eliminating the over-merged `v1` correctness bug on this function.
3. S9 PrintC support: a `?:` OpToken at C conditional-operator precedence (the printer currently has no ternary token at all).

**Owning files:** recognizer near `decompiler/crates/kuna-decomp/src/s8_structure/` or as a Band-B/S3 rule beside `s3_dataflow/`; op tables `substrate/op.rs` + `s5_types/typeop.rs`; emission `s9_emit/` (PrintC op tokens); catalog entry in `decompiler/crates/kuna-decomp/stages.toml` (+ the full hard-coded count-test sweep per the adding-option recipe).

**Risks / default-on viability:** a new opcode threads through S3-S9 (type propagation, cast rules, merge covers, printer precedence/parenthesization) — this is new infrastructure, not a one-pass fix, so it goes through a [PROPOSAL] draft PR first. Output-changing on any O0 diamond, so the 675-datatest gate will need per-test `option ite-ternary-recovery off` opt-outs or a DIV re-pin if later flipped default-on; ship default-OFF initially. Speed impact negligible (one linear diamond scan). Independently of the feature, file the S6 over-merge double-read as a **correctness bug** (kuna emits semantically wrong C that real Ghidra gets right; `s6_variables/merge.rs` speculative merge vs expression-aggregation ordering — upstream inserts COPY trims for ordering-unsafe merges).
