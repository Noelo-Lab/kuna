# impliedrefs / termdup — analysis

## 1. What was unreachable

`ActionMarkExplicit` decides, per SSA value, whether the value gets a token of its
own in the output (a declaration plus a statement) or is inlined into every place
it is read. It asks two numeric questions, and both answers were compiled in:

| constant | read at | ships | upstream's own comment |
|---|---|---:|---|
| `max_implied_ref` | `coreaction_cleanup.rs` `base_explicit` (`ActionMarkExplicit::apply`) | 2 | *"2 is best, in specific cases a higher number might be good"* |
| `max_term_duplication` | `coreaction_cleanup.rs` `process_multiplier` | 2 | *"2 and 3 (4) are reasonable"* |

Both live on `Architecture` (`infra/architecture.rs`, set by
`reset_defaults_internal`) and are copied into the per-function `ArchSeam`
context, so the plumbing to reach them already existed — there was simply no
option, no catalog row, and therefore no way to change either without editing the
source and rebuilding.

They are the two independent halves of one decision:

* `max_implied_ref` bounds **how many places** an expression may be copied to. A
  value with more readers than the bound is forced explicit.
* `max_term_duplication` bounds **how large** the copied expression may be. A
  value that survived the first test with more than one reader has its expression
  walked, and the terminal terms (explicit variables and constants, spacebase
  excluded) that would be re-printed at every use are counted.

## 2. What moving them does

Measured with `kuna decompile-all` on the decbench coreutils `fmt` -O2 stripped
binary (151 functions), counting declared locals out of the emitted C:

| value | `impliedrefs` decls | functions changed | `termdup` decls | functions changed |
|---:|---:|---:|---:|---:|
| 0 | 630 | 76 | 385 | 19 |
| 1 | 384 | 19 | 380 | 18 |
| **2 (shipped)** | **369** | **0** | **369** | **0** |
| 3 | 366 | 6 | 366 | 6 |
| 4 | 363 | 8 | 363 | 9 |
| 6 | 364 | 9 | 362 | 11 |

`impliedrefs 6` declaring *more* than `impliedrefs 4` is not noise: letting a
wider value stay implied inflates its Cover, which changes what the speculative
merges of §6.1 can combine, so the declaration count is not monotone in the bound.

Raising the bound is a small, local effect at 3 and 4 and a large one downward:
`impliedrefs 0` (nothing stays implied) rewrites half the binary and adds 261
declarations and 1,385 statements.

## 3. The witness

`fmt` -O2 `sub_26a0` (`main`), at the shipped bound:

```c
if (((2 <= a0) && (v6 = a1[1], *v6 == '-')) && ((unsigned int)((int)v6[1] - 0x30U) <= 9)) {
  v15 = &v6[1];
```

`v6` has exactly three readers, one over the bound, so it is declared. With
`--option impliedrefs 4`:

```c
if (((2 <= a0) && (*a1[1] == '-')) && ((unsigned int)((int)a1[1][1] - 0x30U) <= 9)) {
  v15 = &a1[1][1];
```

## 4. The one non-obvious coupling

`max_implied_ref` is a program-wide tunable, not a private constant of the
explicit-marking pass. Two other readers take it off the same field:

* `substrate/funcdata_block.rs` `Funcdata::bb_is_complex` — upstream
  `BlockBasic::isComplex`, the statement-count test that decides whether
  `CollapseStructure::rule_block_or` may absorb an OR-clause block;
* `p8_structure/kuna_condfold.rs` — the same printed-width test for the
  short-circuit fold.

So `impliedrefs` loosens those two structure tests along with the marking, exactly
as editing the field by hand always did. This is stated in the option's catalog
row and in `docs/spec/06-variables-and-merge.md` §6.1 rather than being split off
into a second field, because splitting it would be a behaviour change dressed as
plumbing.

## 5. Why this is `opt-in-tool` and not a default flip

Nothing here is more correct than the shipped 2. Both bounds are readability
judgements whose best value depends on what the reader is doing — chasing data
flow wants fewer inlined expressions, skimming control flow wants fewer
temporaries. The defaults stay at the values upstream chose, and the options exist
so the decision is a per-run flag instead of a rebuild.
