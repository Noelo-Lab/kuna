---
case_id: O0-coreutils-factor-factor
group_id: coreutils::factor
status: feature-candidate
tier: S
margin: 12
bucket: inherited
fresh_reproduces: true
fresh_ged: 12
option_closing: null
root_cause_family: early-return-collapse
feature_slug: returndup
scope: small
angr_ref: optimization_passes/return_duplicator_high.py::ReturnDuplicatorHigh (SAILR ISC-revert, no-goto variant)
kuna_stage: S8 structure - blockaction.rs CollapseStructure::rule_block_or + ActionReturnSplit
---
## Side-by-side

Source (`src/factor.c:2219`, from `results/full_run/O0/coreutils/compiled/factor.i` line 28022 — the ground truth Joern scores against):

```c
static void
factor (uintmax_t t1, uintmax_t t0, struct factors *factors)
{
  factors->nfactors = 0;
  factors->plarge[1] = 0;

  if (t1 == 0 && t0 < 2)
    return;

  t0 = factor_using_division (&t1, t1, t0, factors);

  if (t1 == 0 && t0 < 2)
    return;

  if (prime2_p (t1, t0))
    factor_insert_large (factors, t1, t0);
  else
    {
      if (t1 == 0)
        factor_using_pollard_rho (t0, 1, factors);
      else
        factor_using_pollard_rho2 (t1, t0, 1, factors);
    }
}
```

angr (stored, GED 0 — struct typedef header trimmed):

```c
void factor(unsigned long long *a0, unsigned long long a1, struct_0 *ptr)
{
    ...
    v1 = a0;
    v0 = a1;
    ptr->field_fa = 0;
    ptr->field_8 = 0;
    if (!v1 && v0 <= 1)
        return;
    v0 = sub_4037c3(&v1, v1, v0, ptr);
    if (!v1 && v0 <= 1)
        return;
    if ((char)sub_404d2e(v1, v0))
    {
        sub_4033a7(ptr, v1, v0);
        return;
    }
    else if (!v1)
    {
        sub_405a75(v0, 1, ptr, v3, v4, v5);
        return;
    }
    else
    {
        sub_406110(v1, v0, 1, ptr, v6);
        return;
    }
}
```

kuna (fresh, current main d12ef72 — byte-identical to stored; GED 12):

```c
void factor(int8 a0,uint8 a1,int8 a2)
{
  char v1; // al
  uint8 v2; // rax
  int8 v3; // stack - 0x10

  *(void *)(a2 + 0xfa) = 0;
  *(void *)(a2 + 8) = 0;
  if (((a0 != 0) || (2 <= a1)) && ((v3 = a0, v2 = sub_37c3(&v3,a0,a1,a2), v3 != 0 || (2 <= v2)))) {
    v1 = sub_4d2e(v3,v2);
    if (v1 != '\0') {
      sub_33a7(a2,v3,v2);
    }
    else {
      if (v3 != 0) {
        sub_6110(v3,v2,1,a2);
      }
      else {
        sub_5a75(v2,1,a2);
      }
    }
  }
  return;
}
```

(branchflip WARNING comment lines trimmed from the kuna block.)

## Analysis

**Symptom.** The source has two separate early-return guards (`if (t1 == 0 && t0 < 2) return;`) with the `factor_using_division` call between them. kuna collapses both guards *and the intervening call* into one giant short-circuit condition with comma-expression side effects — `if ((A||B) && (v3=a0, v2=sub_37c3(...), C||D)) { rest }` — wrapping the entire body under a single guard with one final return. angr reproduces the source shape statement-for-statement. Metrics: angr 4 ifs / early returns, kuna 3 ifs / 0 early returns; GED 12 is exactly the node+edge delta of that restructure.

**Not an artifact.** artifact_suspect was set (only angr/phoenix at 0), but the rescore validates: `source_nodes=13` (not degenerate), fresh GED before/after = 12.0, and angr's stored block is complete (all five callees, both guards, no truncation). ghidra also scores 12 (bucket=inherited): this is a genuine, inherited Ghidra structuring behavior that angr's SAILR pipeline beats.

**Root cause (kuna, inherited from Ghidra).** At O0 both guards conditionally jump to the single shared epilogue/RETURN block. During `ActionBlockStructure`:

1. `CollapseStructure::rule_block_or` (`decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs:2015`, C++ `blockaction.cc:1321`) merges guard2 into guard1 as a `BlockCondition` because the two conditionals share an out-target (the epilogue). The gate that should stop this — `BlockBasic::isComplex` (`substrate/funcdata_block.rs:934` `bb_is_complex`, C++ `block.cc:2403`) — allows up to 2 "statements", and the guard2 block scores exactly 2 (the `sub_37c3` call + the branch), so it is deemed simple enough to absorb; its statements are emitted as a comma expression inside the condition.
2. Because the merge succeeds, structuring completes goto-free — so `ActionReturnSplit` (`blockaction.rs:3703`, driver `funcdata_block.rs:2827` `return_split_apply`), the Ghidra pass that duplicates a shared return block, never fires: its `gather_return_gotos` only collects **goto**-to-return edges, and there are none. Ghidra's own early-return machinery is preempted by its own condition condenser.
3. None of kuna's SAILR-layer post passes (regionstructure/gotoreduce/taildup/crossjumprevert/dedupitetail) target goto-free condition over-condensation.

**Why angr wins.** angr's SAILR preset runs `ReturnDuplicatorHigh` (`~/.virtualenvs/decbench/.../angr/analyses/decompiler/optimization_passes/return_duplicator_high.py`, stage AFTER_GLOBAL_SIMPLIFICATION; `ReturnDuplicatorLow` is the goto variant) which duplicates return-only blocks per predecessor *even when no gotos exist* (`_should_duplicate_dst` → `dst_is_const_ret`). With a private return per guard, Phoenix structures each guard as its own `if (...) return;`, and angr's condition processor never folds side-effecting blocks into compound conditions (angr has no comma-expression emission at all). Result: CFG-identical to source.

**Option sweep.** No default-off option is relevant to goto-free early-return structuring. Tried anyway: `regionedgeorder on` (structuring-related) — no change; `foldcallret off` (expression folding) — no change (the comma-expr is BlockCondition emission, not call folding). No noreturn-family relevance (no boundary overrun). option_closing: null.

**Siblings.** `O2-noinline-coreutils-factor-factor` (margin 3, kuna=ghidra=3): same family, milder — the O2 call block is too complex to comma-fold, so kuna keeps the ifs but *nests* them (`if { if { ... } }`) instead of emitting the source's early returns; still the shared-epilogue-not-duplicated shape. A `returndup` pass should close both.

## Proposed fix

**Mechanism (`returndup`, option-gated, S8).** A kuna analog of angr's `ReturnDuplicatorHigh`: immediately before block structuring (registered ahead of `blockstructure` in `infra/universalaction.rs`), for every RETURN block with `size_in > 1` that passes the *existing* `return_split_is_splittable` filter (only MULTIEQUAL/COPY/RETURN ops — i.e. a bare epilogue), duplicate the block for each in-edge but one using the *existing* `Funcdata::node_split` (the exact machinery `ActionReturnSplit` already drives). With per-predecessor returns, `rule_block_or` no longer sees a shared out-target, the comma-merge cannot form, and structuring yields the source's early-return `if`s naturally.

- Owning files: new `decompiler/crates/kuna-decomp/src/s8_structure/kuna_returndup.rs` (or a mode on `ActionReturnSplit`), registration in `infra/universalaction.rs`, option plumbing per the settableTable recipe (`stages.toml`, catalog/count tests — see kuna-adding-option-count-tests).
- Alternative (smaller but riskier): tighten `bb_is_complex` to count any CALL-bearing or-block as complex. That leaves a goto to the shared return, which then triggers the *existing* `ActionReturnSplit` + restructure loop — same end state via Ghidra's own intended path, but it changes core CollapseStructure behavior for every function; the additive dup pass is better isolated and angr-aligned.
- Risks: broad output churn (any O0-style function with a shared bare epilogue gains early returns) — certain datatest diffs, so land **default-off** and let the DIV-14 KUNA_DEFAULT_ON ablation sweep decide the flip (angr-parity benefit is large: this shape is ubiquitous at O0). Guard against splitting non-trivial epilogues (the splittable filter already does) and cap duplications per function. Speed: a handful of node_splits per function — well under the 5% budget. 0/675 policy: satisfiable only via per-test opt-outs if flipped on; trivially satisfied default-off.
