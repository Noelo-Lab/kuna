---
case_id: O0-iproute2-ip-print_link_flags
group_id: iproute2::print_link_flags
status: feature-candidate
tier: A
margin: 140
bucket: inherited
fresh_reproduces: true
fresh_ged: 140
option_closing: null
root_cause_family: ite-ternary-recovery
feature_slug: iteregion
scope: small
angr_ref: angr.analyses.decompiler.optimization_passes.ite_region_converter.ITERegionConverter
kuna_stage: s8_structure (post-structuring rewrite family, alongside branchflip/dedupitetail/foldcallret)
---
## Side-by-side

Structural metrics (triage dump): angr(stored) 107 loc / 21 ifs / 20 ternaries / 0 gotos; kuna(stored) 232 loc / 41 ifs / 0 ternaries; kuna(fresh, main @ d12ef72) byte-identical to stored — the gap fully reproduces.

Source (ground truth, `results/full_run/O0/iproute2/compiled/ipaddress.c:84`) — the ternary is IN the source, once per flag via the `_PF` macro:

```c
static void print_link_flags(FILE *fp, unsigned int flags, unsigned int mdown)
{
	open_json_array(PRINT_ANY, is_json_context() ? "flags" : "<");
	if (flags & IFF_UP && !(flags & IFF_RUNNING))
		print_string(PRINT_ANY, NULL,
			     flags ? "%s," : "%s", "NO-CARRIER");
	flags &= ~IFF_RUNNING;
#define _PF(f) if (flags&IFF_##f) {					\
		flags &= ~IFF_##f ;					\
		print_string(PRINT_ANY, NULL, flags ? "%s," : "%s", #f); }
	_PF(LOOPBACK);
	_PF(BROADCAST);
	/* ... 17 more _PF sites ... */
}
```

angr (stored) — first ~30 lines; the remaining 17 flag sites are the identical pattern (cut for brevity):

```c
int print_link_flags(unsigned long long a0, unsigned int a1, unsigned int a2)
{
    unsigned int v0;  // [bp-0x14]
    unsigned long long v1;  // [bp-0x10]

    v1 = a0;
    v0 = a1;
    open_json_array(4, (!(char)is_json_context() ? "<" : "flags"));
    if (((char)v0 & 1) && !((char)v0 & 64))
        sub_410099(4, 0, (!v0 ? "%s" : "%s,"), "NO-CARRIER");
    v0 &= 4294967231;
    if (((char)v0 & 8))
    {
        v0 &= 4294967287;
        sub_410099(4, 0, (!v0 ? "%s" : "%s,"), "LOOPBACK");
    }
    if (((char)v0 & 2))
    {
        v0 &= 4294967293;
        sub_410099(4, 0, (!v0 ? "%s" : "%s,"), "BROADCAST");
    }
    /* ... identical pattern for POINTOPOINT, MULTICAST, NOARP, ALLMULTI,
       PROMISC, MASTER, SLAVE, DEBUG, DYNAMIC, AUTOMEDIA, PORTSEL,
       NOTRAILERS, UP, LOWER_UP, DORMANT, ECHO ... */
    if (v0)
        sub_41013c(4, 0, "%x", v0);
    if (a2)
        sub_410099(4, 0, ",%s", "M-DOWN");
    return close_json_array(4, "> ");
}
```

kuna (fresh == stored) — first ~45 lines; the SAME 7-line diamond repeats at every one of the 20 flag sites (232 lines total, cut for brevity):

```c
void print_link_flags(unsigned long a0,uint4 a1,int4 a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  uint4 v3; // stack - 0x14

                    /* WARNING: branchflip: flipped negated guard for linearity ... */
  if (is_json_context() != '\0') {
    v1 = "flags";
  }
  else {
    v1 = "<";
  }
  open_json_array(4,v1);
  if (((a1 & 1) != 0) && ((a1 & 0x40) == 0)) {
    if (a1 != 0) {
      v2 = 0xa7b20;
    }
    else {
      v2 = 0xa7b24;
    }
    sub_10099(4,0,v2,"NO-CARRIER");
  }
  v3 = a1 & 0xffffffbf;
  if ((a1 & 8) != 0) {
    v3 = a1 & 0xffffffb7;
    if (v3 != 0) {
      v2 = 0xa7b20;
    }
    else {
      v2 = 0xa7b24;
    }
    sub_10099(4,0,v2,"LOOPBACK");
  }
  /* ... the identical 7-line if/else diamond at each of the remaining
     18 flag sites (BROADCAST ... ECHO) ... */
  if (v3 != 0) {
    sub_1013c(4,0,0xa7bc3,v3);
  }
  if (a2 != 0) {
    sub_10099(4,0,0xa7bcd,"M-DOWN");
  }
  close_json_array(4,0xa7bd1);
  return;
}
```

## Analysis

**Symptom.** kuna emits 41 ifs / 0 ternaries (232 loc) where angr emits 21 ifs / 20 ternaries (107 loc). Every `flags ? "%s," : "%s"` in the source (once per `_PF` macro expansion, ~21 sites counting `is_json_context() ? "flags" : "<"`) is compiled at O0 into a two-arm register-assignment diamond (`lea rax, "%s,"` / `lea rax, "%s"`); kuna structures each diamond as a literal `if/else { v2 = const }` statement pair followed by the call, doubling the branch count of the whole function.

**Why angr wins.** angr's `ITERegionConverter` (`~/.virtualenvs/decbench/lib/python3.10/site-packages/angr/analyses/decompiler/optimization_passes/ite_region_converter.py`, stage AFTER_GLOBAL_SIMPLIFICATION) matches exactly this schema — a ConditionalJump block with two single-assignment children converging on one tail — and rewrites it to `x = c ? a : b`; expression folding (`region_simplifiers/expr_folding.py`) then inlines the single-use temp into the call argument, reproducing the source form token-for-token. That gets angr (and phoenix, same engine) to GED 0.

**Not an artifact.** Mandatory Tier-A rescore ran: `source_nodes=45` (real CFG, not degenerate), fresh GED = 140 = recorded (approximated |dNodes|+|dEdges|; ~20 diamonds x ~7 extra nodes+edges each). angr's stored output is the complete function — no truncation. ghidra=140 and ida=140 cluster with kuna because neither converts assignment diamonds to ternaries here; this is a genuine inherited structuring-form gap, not a scoring bug.

**Why no existing option covers it.** `kuna catalog --json` has no ternary/ITE conversion knob. `branchflip` (default-on, S8) fires on these very diamonds — see the WARNING comments — but only normalizes negated-guard arm order; it cannot collapse the diamond. `dedupitetail` dedups cloned if/else tails (different schema); `foldcallret` folds call returns into use sites (would be the model for folding the ternary temp, but there is no ternary to fold); `ifelseflatten`/`gotoreduce`/`taildup`/`crossjumprevert` are goto-family and this function has zero gotos. All remaining default-OFF options are switch-recovery/loader/analysis-tier — none plausibly changes expression form, so no option sweep could close the gap (none run beyond the fresh default dump).

**Secondary cosmetic issue (not GED-relevant).** In the diamond arms kuna renders the string-pointer constants raw (`v2 = 0xa7b20` instead of `"%s,"`, also `0xa7baa`/`0xa7bd1` at direct call sites) — the strings pass plants literals for direct call args in most sites but the register-diamond assignments escape string markup. Fixing iteregion + fold would route these through the same const-render path as the direct arguments.

**Siblings.** None (group `iproute2::print_link_flags` has an empty sibling list).

## Proposed fix

**Feature: `iteregion` (option-gated S8 rewrite, kuna port of angr's `ITERegionConverter`).**

Mechanism sketch:
1. New `decompiler/crates/kuna-decomp/src/s8_structure/kuna_iteregion.rs`, modeled directly on `kuna_dedupitetail.rs` (same ActionBase/option/seam plumbing, ElementId in the 4000+ range, `stages.toml` entry + catalog count bumps per the adding-option recipe).
2. After structuring, match a 3-component `BlockIf` with else: condition block + two arms that are each a single COPY/assignment to the same varnode (the post-SSA merge of the diamond's MULTIEQUAL), converging on one tail. This is the exact schema angr's `_find_ite_assignment_regions` checks (two successors, each child exactly one non-jump statement).
3. Rewrite to a single assignment whose RHS is a kuna ternary form — either a kuna pcode-op (addlflags 0x1000+ range) that `s9_emit` prints as `cond ? a : b`, or a BlockIf mark consumed by the printer. Ghidra's printc has no ternary operator, so the S9 emit hook is the genuinely new piece.
4. Optional follow-up (separate increment): fold the now-single-def/single-use temp into its one use site, reusing the order-safety machinery of `foldcallret` — this is what makes the output literally match the source argument position; the CFG win (which drives GED) comes from step 3 alone.

Owning files: `s8_structure/kuna_iteregion.rs` (new), `s8_structure/mod.rs`, `infra/universalaction.rs` (schedule after structuring, before/alongside branchflip), `stages.toml`, one `s9_emit` printc hook, plus the standard count-test/golden bumps.

Risks / default-on viability: pattern is narrow (single-assignment two-arm diamond, same destination, single convergence) so 0/675 datatest churn is plausible but NOT guaranteed — any upstream test with such a diamond changes rendering, so ship default-OFF first per pipeline rule 3, flip in a later default-on sweep with per-test opt-outs (the PR #110 recipe). Speed impact negligible (one linear post-structuring walk, well under the 5% budget). Scope: small — one option-gated pass per pipeline rule 1; the S9 ternary print form is the only new infrastructure and it is contained to one emit hook.
