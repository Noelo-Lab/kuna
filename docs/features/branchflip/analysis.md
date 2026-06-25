# Analysis — branch-flipping for linearity (`option branchflip`)

Follow-up to PR #52 / the `tee_O2 x2nrealloc` case.

angr's SAILR structurer normalizes a branch guarded by a **negative** test
(`x == 0`, `x == NULL`, `!cond`) so that the **positive** condition becomes the
`if` and the common path reads top-to-bottom. kuna (inheriting Ghidra's
structurer) keeps the source polarity, so it emits `if (x == 0)` where angr emits
`if (x)`. `option branchflip on` adds the angr-style flip to kuna's pipeline as an
S8 structuring pass (`ActionBranchFlip`), opt-in default-off.

This document records the motivating case, what the feature does, **the exact
shape it handles and the shape it does not**, and the mechanism, so another agent
can build on it.

---

## The motivating example: `tee_O2 x2nrealloc`

Binary `binaries/tests/x86_64/decompiler/tee_O2`, function `x2nrealloc` @ `0x4057f0`
(angr testcase `test_decompiling_tee_O2_x2nrealloc`).

```
--- kuna (default) ---                         --- angr ---
  while( true ) {                              v1 = *(ptr);
    ...                                        if (a0) {
    v3 = *v7;                                      v2 = (v1 >> 1) + 1;
    if (a0 == (void *)0x0) break;                  v4 = v1 + v2;
    ...                                            if ((char)__CFADD__(v1, v2))
    if (!v8) goto label_5813;                          xalloc_die();
    xalloc_die(v4[-0x20]);                     } else {
    ...                                            v4 = v1;
  }                                                if (!v4)
  if (v3 == 0) { ... }                                v4 = (128 / a2) + (a2 > 128);
label_5813:                                    }
  xreallocarray(a0,v3,a2,v4[-0x20]);           ptr = xreallocarray();
  ...                                          *(ptr) = v4; return ptr;
```

Two negated guards stand out in kuna's output:

1. `if (a0 == (void *)0x0) break;` — inside a `while(true)` loop.
2. `if (v3 == 0) { ... }` — a one-armed `if` (no `else`).

angr renders the first guard's *positive* sense, `if (a0)`, as the head of a
clean `if/else` (and has no loop at all).

### Why x2nrealloc is NOT fully fixed by branch-flipping alone

The two negated guards above are **not** flippable in place, for structural
reasons (see "What the feature handles" below):

* `if (a0 == 0) break;` is an **if-goto** (the body is an unstructured `break`
  edge). The structurer (`rule_block_goto`, `blockaction.rs`) forces the
  goto/break onto the *true* branch, which is what makes the guard read `== 0`.
  Flipping the condition without an `else` arm to swap the body into is not a
  local rewrite — the `break` must stay on the taken edge.
* `if (v3 == 0) { ... }` is a **2-component** `if` (no `else`). Flipping it to
  `if (v3 != 0)` would require rotating the fall-through into a synthesized
  `else`, which is a region-level restructure, not a polarity flip.

Worse, kuna's `while(true)` loop here is itself spurious — it comes from
`xalloc_die` not being recognized as no-return (the separate `noreturn_propagate`
feature, `docs/features/tee-o2-x2nrealloc-6981e7/`). angr's loop-free `if/else`
shape is the product of that no-return knowledge **plus** its region structurer,
not of branch-flipping. **Reproducing x2nrealloc's exact angr shape needs the
region structurer; branch-flipping is one necessary ingredient, not the whole
fix.** `branchflip` is therefore scoped to the polarity normalization it *can*
do faithfully (3-component `if/else`), and x2nrealloc's specific guards fall
outside that shape.

---

## What the feature handles (and what it does not)

`ActionBranchFlip` fires only on a **3-component `BlockIf`** (a real `if/else`
with both arms and a shared merge), the one shape that can be flipped **in place**
by swapping the two arms. For such a block, when the guard is the
*negated / equality-to-zero* form, it:

```
   if (x == 0) { A } else { B }      =>      if (x != 0) { B } else { A }
```

i.e. it negates the condition to its positive complement and swaps the `if`/`else`
arms so semantics are preserved. A self-contained example (the stage testcase,
`tests/stages/branchflip-negated-guard.xml`):

```c
int bf(int x, int *p) {
    int r;
    if (x == 0) { p[0] = 7; r = p[1]; }   // OFF: if (x == 0) { ...7... } else { ...9... }
    else        { p[0] = 9; r = p[2]; }   // ON:  if (x != 0) { ...9... } else { ...7... }
    p[3] = r;
    return r * 5;
}
```

Shapes it deliberately does **not** touch (left to the region structurer):

* **if-goto / `if (cond) break;`** — no `else` arm to swap into.
* **2-component `if`** (no `else`) — flipping needs a synthesized `else`.
* **short-circuit `&&`/`||` (`BlockCondition`)** guards with no single split
  address are flagged but not logged (the warning keys on a CBRANCH address).

---

## Mechanism

The condition polarity is decided entirely in **S8 (structuring)** — by the time
**S9 (emit, `printc.rs`)** runs, it faithfully renders whatever the `boolean_flip`
flag and edge order say (it reads `op.is_boolean_flip()` and the goto accessors;
it never flips anything itself). So the feature lives in S8.

The cleanest seam was the existing `ActionPreferComplement` /
`Funcdata::block_if_prefer_complement` (`substrate/funcdata_block.rs`), which
**already** flips `if/else` arms to the complementary condition using a cost
oracle, `op_flip_in_place_test` (`substrate/funcdata_op.rs`):

| oracle result | meaning | example |
|---|---|---|
| `0` | flip **removes** negation (normalizing) | `!=`, `BOOL_NEGATE`, non-constant `<` |
| `1` | flip **adds** negation in Ghidra's model (denormalizing) | `==`, `== 0`, constant-operand `<`/`<=` |
| `2` | cannot flip in place | — |

The default `prefer_complement` flips only result `0`. **`branchflip` is the
mirror: it flips result `1`** — exactly the equality-to-zero / negated guard that
`prefer_complement` leaves alone, and exactly the one whose *positive* reading
angr produces. The new method `Funcdata::branch_flip_complement` /
`block_if_flip_negated_guard` reuses the proven dual-arena flip machinery:

```
split_flip_in_place_execute(split)   // flip boolean_flip/fallthru_true + swap bblocks edges
op_flip_in_place_execute(fliplist)   // rewrite the comparison op (== -> !=)
swap_blocks(sif, 1, 2)               // swap the BlockIf's true/false children
```

### Pipeline placement

`ActionBranchFlip::boxed("blockrecovery", false)` is registered in
`infra/universalaction.rs` immediately **after the second**
`ActionPreferComplement(false)` + `ActionStructureTransform(false)` (the final
structuring round), so it sees the finalized tree after `prefer_complement` has
already taken the positive-flip (`0`) cases — leaving only the residual `== 0`
guards for `branchflip`. The flip survives to emit untouched.

### Observability

Each flip is logged via `Funcdata::warning` at the guard's CBRANCH address; it
renders as
`/* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */`
right above the flipped `if`. Comments raised *inside* the action pipeline are
flushed to the comment DB by `drain_pipeline_comments` at the end of
`run_pipeline` (`infra/decompile_drive.rs`); flow-time warnings were already
drained before the pipeline, so this is a new, narrow drain point.

---

## Gate / wiring summary

* Option `branchflip` (boolean, opt-in **default-off**), source_decompiler `angr`,
  stage S8 / `readability-rewrites`, `change_kind = opt-in-tool`.
* Registered: `stages.toml` settableTable + a surface row; `KUNA_OPTION_NAMES`;
  the `Architecture::branch_flip` field (ctor + `reset_defaults` + `set_kuna_option`)
  **and the seam copy** in `build_arch_handle` + `substrate/seams.rs` (without the
  seam copy the gate silently no-ops). No ElementId (booleans route via
  `set_kuna_option`, like `stackguard`); no `live_field` (read console-side, like
  `stackguard`/`namestyle`).
* Default-off because the flip *does* change rendering (it is not a 0/675 no-op),
  so it is not eligible for default-on; `make test` stays 675/675 PARITY OK.
* Testcase: `tests/stages/branchflip-negated-guard.xml` (two-pass: OFF shows
  `if (x == 0)`, ON shows the flipped `if (x != 0)` + the log).

## Speed

The pass is one extra BFS over the structured-block tree, only flipping when the
cost oracle returns `1`; with the option **off** it is a single flag check and
returns immediately (zero added work on the default path). See the PR description
for the exact off-vs-on numbers captured on the target function.
