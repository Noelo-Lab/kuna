---
case_id: O0-findutils-find-parse_ilname
group_id: findutils::parse_ilname
status: feature-candidate
tier: S
margin: 6
bucket: inherited
fresh_reproduces: true
fresh_ged: null
option_closing: null
root_cause_family: shared-return-merge
feature_slug: eagerreturnsplit
scope: small
angr_ref: optimization_passes/return_duplicator_low.ReturnDuplicatorLow (+ stack_canary_simplifier.StackCanarySimplifier)
kuna_stage: S8 structure / ActionReturnSplit (blockaction.rs) + S7 kuna_stackguard residue
---
## Side-by-side

angr (stored) — struct typedefs trimmed (16 lines of struct_0/struct_1 defs):

```c
unsigned int parse_ilname(struct_0 *a0, unsigned long long a1, unsigned int *a2)
{
    int v3;  // xmm0
    unsigned long long v0;  // [bp-0x20]
    struct_0 *ptr;  // [bp-0x18]

    if (!(char)sub_4145ab(a1, a2, &v0))
        return 0;
    ptr = sub_40eecc(a0, v0);
    *((unsigned long long *)&ptr->padding_29[15]) = v0;
    sub_415576(v0, 0);
    ptr->field_24 = *((unsigned int *)&MulV(v3, 1036831949));
    return 1;
}
```

kuna (stored, run-time) — byte-identical to fresh:

```c
bool parse_ilname(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  int8 v2; // fs_offset
  bool v3; // zf
  int8 v4; // stack - 0x18
  unsigned long v5; // stack - 0x10

  v5 = *(void *)(v2 + 0x28);
  v3 = sub_145ab(a1,a2,&v1) != '\0';
  if (v3) {
    v4 = sub_eecc(a0,v1);
    *(void *)(v4 + 0x38) = v1;
    *(float4 *)(v4 + 0x24) = (float4)sub_15576(v1,0) * dat_3c028;
  }
  return v3;
}
```

kuna (fresh, current build): identical to stored (17 loc, 1 if, 0 gotos) — the gap fully reproduces on current main.

Structural metrics: angr 33 loc / 1 if / 2 returns; kuna 17 loc / 1 if / 1 return + 1 dead canary store. Neither side has gotos/labels/switches/loops.

## Analysis

Symptom: the source (`findutils/find/parser.c parse_ilname`) has the classic guard shape — `if (collect_arg(...)) { body; return true; } return false;` — i.e. TWO return statements. GCC -O0 compiles that to a single shared return block (`al` set on both paths, one `ret`). angr restores the source shape: `ReturnDuplicatorLow` (the venv angr that ran; `return_duplicator_low.py` + `return_duplicator_base.py`, max_calls_in_regions=2) duplicates the shared return block into each predecessor, and `StackCanarySimplifier` removes the whole -fstack-protector pattern (prologue store AND epilogue check). Result: `if (!cond) return 0; body; return 1;` — GED 0 vs source (IDA also 0, so this is independently confirmed genuine, not an angr-engine artifact; angr's stored body is complete, no truncation).

kuna (like upstream Ghidra — GED 8, bucket=inherited) keeps the compiled merged form and diverges from the source CFG in three places worth ~6 edits total:
1. `return v3;` — one merged return instead of the source's two (the shared RETURN block is never split);
2. `v3 = sub_145ab(...) != '\0';` — the condition is spilled to a named bool because v3 is used twice (if-condition + return value), so `foldcallret` (single-use only) cannot fold the call into the `if`;
3. `v5 = *(void *)(v2 + 0x28);` — the dead -fstack-protector prologue store. `stackguard` (default-on, `s7_regions/kuna_stackguard.rs`) strips only the epilogue check + `__stack_chk_fail` block; the prologue store to the addr-tied stack slot survives dead-code elimination.

Root cause in kuna: the ported `ActionReturnSplit` ("returnsplit", `s8_structure/blockaction.rs:3706` -> `substrate/funcdata_block.rs:2827 return_split_apply`) is faithfully gated the upstream way: it only splits a multi-in-edge RETURN block when `gather_return_gotos` finds goto-in edges (`if gotoblocks.is_empty() { continue; }`). Here the shared return structures cleanly as if-fall-through — zero gotos — so the split never happens and the source's dual-return shape is unrecoverable downstream. Fixing the return split also fixes diff (2) for free: after the split each path's return value is a distinct constant (the MULTIEQUAL dissolves), v3 becomes single-use, and `foldcallret` folds the call into the `if`.

Why no existing option covers it: the S8 goto-quality family (gotoreduce / taildup / crossjumprevert, all default-on) only rewrites `if (cond) goto T` shapes — there is no goto here. dedupitetail / ifelseflatten do not apply (no else). No default-off option in the catalog plausibly matches (no sweep run needed); this is not a boundary overrun, so the noreturn family is irrelevant. `stackguard` is already on and by design does not touch the prologue store.

Siblings:
- O2-noinline-findutils-find-parse_ilname (margin 6, GED kuna 6 / ida 6 / ghidra 8 / angr 0): same symptom, and angr's stored output literally shows the duplicator's signature — `return v2;` cloned into both arms.
- O2-findutils-find-parse_ilname (margin 16, GED kuna 16 / ghidra 16 / ida 14 / angr 0): same family compounded by -O2 inlining of collect_arg (kuna faithfully shows the inlined 3-if body, 25 loc; angr's 10-loc output matches the source's 1-if shape) — the return-merge fix would recover only part of this sibling's margin.

## Proposed fix

Mechanism (one option-gated feature per pipeline rule 1, slug `eagerreturnsplit`): relax the `ActionReturnSplit` gate behind a new settable. When the option is on, `return_split_apply` also splits a multi-in-edge RETURN block with NO goto-in edges, when bounded and profitable: the RETURN's value input is a MULTIEQUAL (esp. of distinct constants — the compiled bool-merge), the epilog block is tiny (no calls/stores beyond the RETURN chain), and in-degree <= a small cap (angr's analog caps duplicated-region calls at 2). This reuses the already-ported `Funcdata::node_split` machinery (`substrate/funcdata_block.rs:2888`) — no new infrastructure, just a second, gated admission path next to the existing goto-driven one. That single change yields `return 0;`/`return 1;` and unlocks `foldcallret` on the condition, closing 2 of the 3 CFG diffs.

Secondary residue (candidate separate small follow-up, not this PR): extend `kuna_stackguard` to also delete the prologue canary store (`slot = *(fs:0x28)`) whenever it strips the matching epilogue check — angr's StackCanarySimplifier removes both ends; kuna currently leaves a dead assignment node in every -fstack-protector function, a ~2-3 GED tax on small functions across the whole benchmark.

Owning files: `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs` (ActionReturnSplit), `decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs` (return_split_apply / node_split), `decompiler/crates/kuna-decomp/stages.toml` (new S8 settable + the option count-test sweep per the adding-option recipe).

Risks / default-on viability: duplicating epilogs changes rendering for any function with a merged constant return — this will NOT be 0/675 byte-identical (upstream merges returns everywhere), so ship default-OFF with `--option eagerreturnsplit on`, measure on decbench, and consider a later DIV default-on sweep (the PR #110 pattern with per-test opt-outs). Speed impact ~nil (node_split on tiny epilogs, bounded fan-in). Main quality risk is over-duplication on high fan-in returns — the in-degree cap and constants-only MULTIEQUAL condition keep it to the bool-merge idiom.
