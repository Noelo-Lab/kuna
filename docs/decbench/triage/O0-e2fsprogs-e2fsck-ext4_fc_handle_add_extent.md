---
case_id: O0-e2fsprogs-e2fsck-ext4_fc_handle_add_extent
group_id: e2fsprogs::ext4_fc_handle_add_extent
status: feature-candidate
tier: S
margin: 7
bucket: ahead
fresh_reproduces: true
fresh_ged: 7
option_closing: null
root_cause_family: return-duplication
feature_slug: returndup
scope: small
angr_ref: optimization_passes/return_duplicator_high.py (ReturnDuplicatorHigh + ReturnDuplicatorBase._analyze_core / _is_simple_return_graph)
kuna_stage: S8 structure - blockaction ActionReturnSplit ("returnsplit" group, universalaction.rs:665)
---
## Side-by-side

Source (`journal.c:799`, guard-clause style — 3 returns):

```c
static int ext4_fc_handle_add_extent(e2fsck_t ctx, __u8 *val)
{
	...
	memcpy(&add_range, val, sizeof(add_range));
	ino = le32_to_cpu(add_range.fc_ino);
	ext4_fc_flush_extents(ctx, ino);
	ret = ext4_fc_read_extents(ctx, ino);
	if (ret)
		return ret;
	memset(&extent, 0, sizeof(extent));
	ret = errcode_to_errno(ext2fs_decode_extent(&extent, (void *)add_range.fc_ex, sizeof(add_range.fc_ex)));
	if (ret)
		return ret;
	return ext4_add_extent_to_list(ctx, &ctx->fc_replay_state.fc_extent_list, &extent);
}
```

angr (stored, GED 0 — struct typedef header trimmed):

```c
int ext4_fc_handle_add_extent(struct_0 *a0, void* a1)
{
    unsigned int v0;  // [bp-0x54]
    unsigned long v1;  // [bp-0x50]
    unsigned long long v2[4];  // [bp-0x48]
    unsigned int v3;  // [bp-0x28]
    char v4;  // [bp-0x24]

    v0 = 0;
    memcpy(&v3, a1, 16);
    v1 = v3;
    sub_43e379(a0, v1);
    v0 = sub_43e305(a0, v1);
    if (v0)
        return v0;
    memset(&v2, 0, 24);
    v0 = sub_43d5a8(ext2fs_decode_extent(&v2, &v4, 12, &v4), "ext4_fc_handle_add_extent", 814);
    if (!v0)
        return sub_43e29f(a0, &a0->padding_0[944], &v2);
    return v0;
}
```

kuna (stored == fresh, byte-identical; GED 7):

```c
uint8 ext4_fc_handle_add_extent(int8 a0,void *a1)
{
  uint4 v1; // eax
  uint8 v2; // rax
  unsigned int v3; // stack - 0x28
  char v4 [32];
  char v5 [20];
  int8 v6; // fs_offset
  unsigned long v7; // stack - 0x10

  v7 = *(void *)(v6 + 0x28);
  memcpy(&v3,a1,0x10);
  sub_3e379(a0,v3);
  v1 = sub_3e305(a0,v3);
                /* WARNING: branchflip: flipped negated guard ... */
  if (v1 != 0) {
    v2 = (uint8)v1;
  }
  else {
    memset(v4,0,0x18);
    v2 = ext2fs_decode_extent(v4,v5,0xc);
    v1 = sub_3d5a8(v2,"ext4_fc_handle_add_extent",0x32e);
                /* WARNING: branchflip: flipped negated guard ... */
    if (v1 != 0) {
      v2 = (uint8)v1;
    }
    else {
      v2 = sub_3e29f(a0,a0 + 0x3b0,v4);
    }
  }
  return v2;
}
```

## Analysis

- **Fresh reproduces**: kuna(fresh) is byte-identical to kuna(stored); the Joern rescore
  (kuna commit d12ef72) confirms GED 7.0 before and after, `source_nodes: 5` (not a
  degenerate source CFG), `ged_approximated: false`. `bucket=ahead` — kuna (7) beats
  ghidra (15), but angr AND ida both score 0 independently, so this is a genuine
  structuring gap, not an angr-engine artifact.
- **Dominant structural difference (the one sentence)**: kuna merges the three source
  `return`s into one shared exit (a MULTIEQUAL-fed `return v2;` reached through nested
  if/else with `v2 = (uint8)v1;` merge assignments), where angr — and the source — emit
  three early guard-clause returns.
- **Root cause**: at O0 gcc compiles each `return ret;` as `mov eax,...; jmp .Lepilogue`
  into one shared epilogue block, so the binary CFG has a single multi-predecessor RETURN
  block. angr's **ReturnDuplicatorHigh** (SAILR return duplication,
  `optimization_passes/return_duplicator_high.py`, angr 9.2.213 in the decbench venv)
  runs pre-structuring (AFTER_GLOBAL_SIMPLIFICATION), classifies the shared exit as a
  "simple return graph" (`_is_simple_return_graph` allows jumps + up to one assignment +
  RETURN — a variable return, not just a constant), and duplicates it into every
  predecessor, so Phoenix/SAILR structures three separate returns that match the source
  CFG exactly. kuna's port of upstream Ghidra's **ActionReturnSplit**
  (`substrate/funcdata_block.rs::return_split_apply`, scheduled in the `returnsplit`
  group at `infra/universalaction.rs:665`) is only the analog of the *goto-driven*
  ReturnDuplicatorLow: it requires `gather_return_gotos` to find a goto edge into the
  RETURN block. Here structuring succeeds goto-free (clean nested if/else), so
  `gotoblocks.is_empty()` and the split never fires — kuna has no analog of the
  **gotoless** ReturnDuplicatorHigh.
- **Why no existing option covers it**: fresh == stored, and the only plausible
  default-off structuring option (`regionedgeorder`) produces identical output (verified).
  The whole S8 goto-quality family (`gotoreduce`/`taildup`/`crossjumprevert`/
  `loopbreak_recovery`) triggers on a goto in the structured output — there are zero
  gotos here. `ifelseflatten` (default-on) would flatten the else-chains into
  guard-clause form, but only once the true-arm is statement-terminating, which it never
  becomes without the return being duplicated into it. `branchflip` fires (both guards
  positive) but cannot change the merge topology.
- **Secondary contributors to the 7**: (a) the residual canary *load*
  `v7 = *(void *)(v6 + 0x28);` + the `v6 // fs_offset` decl — `stackguard` strips the
  check epilogue but not the entry load, while angr's StackCanarySimplifier removes both;
  (b) the `v2 = ext2fs_decode_extent(...)` spill (angr inlines it into the `sub_43d5a8`
  call) — `foldcallret` is blocked because `v2` doubles as the multi-def return-merge
  variable, itself a downstream effect of the missing return duplication.
- **Sibling** `O2-noinline-e2fsprogs-e2fsck-ext4_fc_handle_add_extent` (kuna 9, angr 0,
  ghidra 7, ida 3, bucket=inherited): same symptom — kuna nests both guards
  (`if (sub_30330(...) == 0) { ... if (sub_302d0(...) == 0) { ... } }`) with one trailing
  `return;` (and additionally loses the return value, rendering `void`), where angr emits
  the three early returns.

## Proposed fix

- **Mechanism** (`returndup`, option-gated per pipeline rule 1): add a gotoless mode to
  the existing `ActionReturnSplit` machinery — when a RETURN block has >= 2 in-edges and
  passes the already-ported `return_split_is_splittable` predicate (only
  MULTIEQUAL/COPY/RETURN ops with constant/annotation/non-free inputs — exactly angr's
  "simple return graph"), split every in-edge but one via the existing
  `node_split`/`CloneBlockOps` path even when `gather_return_gotos` is empty, then
  `structure_reset()`. Each predecessor then owns its return; the structurer emits early
  returns and the default-on `ifelseflatten` flattens the residual else-chains into
  guard-clause form. Duplication cost is bounded by construction: the region is
  return-only (no calls), growth is ~1 return statement per predecessor; add a pred-count
  cap (angr uses dup-all when >= N-2 preds qualify) for safety.
- **Owning files**: `decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs`
  (`return_split_apply` — add the gotoless branch), `s8_structure/blockaction.rs`
  (ActionReturnSplit), `infra/universalaction.rs` (`returnsplit` group ordering after
  ActionStripStackGuard is already correct), `stages.toml` + the settableTable/catalog
  count tests per the kuna-adding-option-count-tests recipe.
- **Risks**: output churn on any function with a shared simple-return epilogue (very
  common at O0), so default-ON would disturb many of the 675 datatests — ship
  **default-off** as an angr-parity flag first, then consider a DIV-14-style default-on
  sweep with per-test `option returndup off` opt-outs. Speed: negligible (one scan of
  RETURN ops per structuring iteration, same as the existing goto path); the node_split
  clone path is already exercised by the goto-driven case. Watch interaction with
  `dedupitetail` (must not re-merge the duplicated returns) and functions where the
  RETURN block is the entry block (skip, `size_in <= 1` guard already present).
- **Follow-up (separate, smaller)**: teach `stackguard` to also strip the entry canary
  load (`vN = *(fs_offset + 0x28)` dead residue) — worth ~1-2 GED on nearly every O0
  stack-protected function.
