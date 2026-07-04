---
case_id: O0-iproute2-ip-lookup_flag_data_by_name
group_id: iproute2::lookup_flag_data_by_name
status: feature-candidate
tier: S
margin: 19
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: 19
option_closing: null
root_cause_family: whiledo-overflow-iscomplex
feature_slug: whiledo-overflow-iscomplex
scope: small
angr_ref: angr.analyses.decompiler.structuring.phoenix.PhoenixStructurer (cyclic natural-loop schema keeps the bound guard as the loop condition)
kuna_stage: S8 structuring - s8_structure/blockaction.rs CollapseStructure::is_complex (rule_block_while_do overflow decision; emission fallout in s9_emit/printc.rs emit_block_while_do)
---
## Side-by-side

angr (stored), GED 0 — full function:

```c
// Function: lookup_flag_data_by_name @ 0x14c77
extern unsigned long long g_4d5320[4];

unsigned long long lookup_flag_data_by_name(char *a0)
{
    unsigned int i;  // [bp-0xc]

    for (i = 0; i <= 12; i += 1)
    {
        if (!strcmp(a0, g_4d5320[3 * i]))
            return &g_4d5320[3 * i];
    }
    return 0;
}
```

kuna (stored, run-time), GED 19 — full function; note the INVALID C (an `if ... {return 0}` statement embedded inside the `while (` condition, and the `if` printed without parens because it is emitted in condition context):

```c
// Function: lookup_flag_data_by_name @ 0x14c77
int8 lookup_flag_data_by_name(char *a0)

{
  char *v1;
  uint4 v2; // stack - 0xc
  
  v2 = 0;
  while (
  if 0xd <= v2 {return 0
  }v1 = *(char **)((uint8)v2 * 0x18 + 0xd5320), strcmp(a0,v1) != 0) {
    v2 = v2 + 1;
  }
  return (uint8)v2 * 0x18 + 0xd5320;
}
```

kuna (fresh, current main d12ef72) — byte-identical to stored; gap fully reproduces. Rescore ran: source_nodes=6 (healthy source CFG, not degenerate), fresh GED = 19.0 == recorded (before 19.0 / after 19.0, not approximated).

## Analysis

Symptom: kuna emits syntactically invalid C — the loop's bound-check `if (0xd <= v2) return 0;` is printed *inside* the `while (...)` condition parentheses, followed by the comma-expression condition. Joern cannot parse this, so the recovered CFG is garbage → GED 19, while ghidra=5, ida=0, phoenix=5 on the same function (bucket kuna-specific: kuna is 14 worse than its own ancestor Ghidra).

Root cause (pinned to one ported function): a porting divergence in `is_complex` — kuna-wt file refs are to /home/mahaloz/github/kuna:

- Collapse sequence (faithful to upstream, verified in `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`): the loop header H (`if (0xd <= v2)`) + its return-0 clause collapse to a BlockIf (`rule_block_if_no_exit`); ruleBlockCat then concatenates [BlockIf, strcmp-block] into a **BlockList** that carries the loop's 2-way exit; `rule_block_while_do` (blockaction.rs:2287) then forms the WhileDo with that BlockList as the *condition block*.
- Upstream guards exactly this with `bool overflow = bl->isComplex();` — and in upstream C++ (verified against ~/github/ghidra `decompiler/cpp/block.hh:254,503,549,649`) the virtual dispatch is: base `FlowBlock::isComplex()` returns **true**; only `BlockBasic` (statement-count test, block.cc:2403), `BlockCopy` (`copy->isComplex()`), and `BlockCondition` (`getBlock(0)->isComplex()`) override. A **BlockList/BlockIf condition is unconditionally complex** upstream → `set_overflow_syntax()` → printed as valid `while( true ) { <cond stmts>; if (<branch>) break; <body> }`.
- kuna's port (`blockaction.rs:2001`, and a duplicated copy at `s8_structure/region_structurer.rs:821`) instead resolves *every* block via `get_front_leaf()` down to the front BlockCopy leaf and tests only that BlockBasic against `complex_blocks` — i.e. it applies the BlockCopy/BlockCondition delegation to ALL graph types including BlockList and BlockIf (its own doc comment even claims "BlockList/BlockCopy delegate down to the front BlockBasic", which is wrong for BlockList). Here the front leaf is the trivial bound-check block (1 statement) → `is_complex` = false → no overflow syntax → `emit_block_while_do` (printc.rs:2855) takes the plain `while (cond)` path and inlines the whole BlockList — including the embedded BlockIf-return — inside the parens under COMMA_SEPARATE. Invalid C.

Why angr wins outright: angr's Phoenix cyclic natural-loop schema keeps the header guard as the loop condition and its condition-based for-loop recovery rebuilds `for (i = 0; i <= 12; i += 1)` with the strcmp-if in the body — structurally identical to the source (GED 0). But the kuna-specific 14-point delta vs Ghidra is purely the invalid emission; with correct overflow syntax kuna would render the same shape Ghidra does (GED ~5).

Why no existing option covers it: the option sweep was run — `--option regionstructure off` and `--option regionlooprefine off` both produce byte-identical output (the classic CollapseStructure path and the region-structurer path share the same collapse and the same buggy helper). No catalog option gates the whiledo overflow decision; the S8 goto-quality family (gotoreduce/taildup/crossjumprevert/dedupitetail/ifelseflatten) runs after structuring and cannot repair a statement swallowed into a condition; no boundary overrun, so the noreturn family is irrelevant. option_closing = null.

Secondary exposure, same helper: `rule_block_or` (blockaction.rs) and the region structurer's short-circuit fold both use `is_complex(orblock)` to refuse folding statement-bearing blocks into a BlockCondition; the same wrong front-leaf verdict can fold a BlockList and misprint its statements inside an `if (... && ...)` condition. Fix both call sites' helper.

Siblings: O2-noinline-iproute2-ip-lookup_flag_data_by_name — GED 5 vs angr 0, bucket inherited; kuna emits the *correct* `while( true ) { ... if (v1 == 0xd) break; }` overflow-syntax form there (the O2 loop rotates so the condition block tests complex), i.e. the sibling shows exactly the rendering the O0 case should have produced; no bug, angr just structures a cleaner for-loop (SAILR-class, already-tracked loop-condition quality).

## Proposed fix

Mechanism (small, one helper, two copies): make `is_complex` implement the real upstream virtual dispatch instead of unconditional front-leaf descent. In `CollapseStructure::is_complex` (`decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs:2001`) and `RegionStructurer::is_complex` (`s8_structure/region_structurer.rs:821`): match on block type — Copy → its BlockBasic's `complex_blocks` verdict; Condition → recurse into component 0; everything else (List, If, WhileDo, Goto, ...) → `true` (upstream `FlowBlock::isComplex` base, block.hh:254). `get_front_leaf` remains correct only for Copy/Condition chains. Expected O0 output after fix: `while( true ) { if (0xd <= v2) return 0; v1 = ...; if (strcmp(a0,v1) == 0) break; v2 = v2 + 1; }` — valid C, expected GED ≈ Ghidra's 5 (closing the remaining 5 to angr's 0 is the separate, already-in-flight SAILR for-loop-condition work).

Owning files: `s8_structure/blockaction.rs` (rule_block_while_do consumer at :2287, rule_block_or), `s8_structure/region_structurer.rs` (dup helper), rendering consequence in `s9_emit/printc.rs` (no change needed there — the overflow branch is already correct, as the O2 sibling proves).

Risks / default-on viability: this is a correctness fix that *restores* upstream Ghidra semantics, so default-on is the right default; per pipeline rule it is output-changing ⇒ gate behind `--option whiledocomplex` (default-on) and log it. Blast radius: any function whose collapsed multi-block whiledo condition previously tested non-complex flips from (often invalid) inline-condition form to `while(true)+break` form — every such site today is at best borderline and at worst invalid C, so the flip is strictly quality-positive; a few of the 675 datatest assertions may shift if upstream-divergent output was silently pinned (none expected, since the 675 were validated equivalent to upstream C++ — any shift would reveal a pre-existing silent divergence and should be re-pinned per the DIV process). Speed impact: nil (a type match replacing a leaf walk). Gates: `make test` 675/675 PARITY OK, `make test-stages` no new failures, `make rust-test` green; the two existing `is_complex_*` unit tests in `blockaction/tests.rs:775,800` encode the buggy contract and must be updated alongside.
