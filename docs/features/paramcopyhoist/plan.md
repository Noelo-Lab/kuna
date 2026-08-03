# paramcopyhoist — design

## Shape

One new module + one new Action; the ported merge core is untouched apart from a
single visibility widening:

* `decompiler/crates/kuna-decomp/src/p6_variables/kuna_paramcopyhoist.rs`
  (`ELEM_PARAMCOPYHOIST` = 4120, `OptionParamCopyHoist`, `ActionParamCopyHoist`).
* Registered in `infra/universalaction.rs` as the **last** action before
  `ActionPrototypeWarnings`.
* `Architecture::param_copy_hoist` + `ArchContext::param_copy_hoist` + the
  `"paramcopyhoist"` arm of `Architecture::set_option`, the name in
  `p0_knowledge/options.rs`, the `settableTable` row in `phases.toml`.
* One `// (kuna)` widening in the ported core: `FuncdataCoverCtx` in
  `substrate/funcdata.rs` becomes `pub(crate)` so the pass can build the
  hypothetical Cover off the same read view `buildDominantCopy` uses.

## Why the pass runs last

Both reasons were measured, not designed — see `analysis.md` §"Three things that
had to be measured":

1. Inside `Merge::trimOpInput` the move is **wrong code**: it defeats the trim's
   own purpose, sends `mergeOp` down the `trimOpOutput` path and makes the
   assignment vanish from the emitted C.
2. Anywhere before `ActionFinalStructure` the move perturbs the P8
   duplication/dedup passes.

Running last, the only observable effect is which basic block's statement list
holds the COPY.

## The predicate

`is_hoistable(fd, op, entry)` accepts `op` iff all of:

* `op` is a live, printing `CPUI_COPY` whose parent is not the entry block and
  which the entry block dominates;
* its input is a function input Varnode occupying a **formal parameter's**
  storage (`FuncProto::get_param(i)` address+size match). `Varnode::isInput`
  alone is too broad: a global or a read-before-written stack slot is also an
  SSA input. Observed moving `v5 = dat_de720;` in `ext2fs_max_extent_depth`
  before the restriction was added;
* at least one read of its output is a `CPUI_MULTIEQUAL` and every read is a
  `MULTIEQUAL` or an `INDIRECT`. The MULTIEQUAL requirement is what makes this
  the `trimOpInput` guard-join family; a copy read only by an INDIRECT is
  `Merge::mergeIndirect`'s call-adjacent snip (75 of the triage's 143 classified
  occurrences), a different Cover shape left for a separate, separately-measured
  change. `ea_refcount_free`'s copy is read by both, which is why the rule is
  "at least one MULTIEQUAL", not "all MULTIEQUAL";
* the Cover test below accepts;
* and, across the whole function, its HighVariable has exactly one candidate
  (`one_per_high`) — the Cover test cannot see two definitions of one variable
  converging on the entry block.

## The Cover test

The same test `Merge::buildDominantCopy` applies to its own redirects, run
against the hypothetical hoisted placement and against the **final**
HighVariable:

* `b_cover` = union of `Cover::rebuild` over the target high's instances,
  skipping the moving Varnode itself and any COPY instance that copy-shadows the
  same root (upstream's own filter — those carry the same value);
* `a_cover` = `add_def_point(entry_block, CoverPoint::Begin)` followed by
  `add_ref_point_for` at every read of the COPY's output;
* reject when `b_cover.intersect(&a_cover) > 1` (an overlap beyond a def/use
  boundary).

Taking the def point at the *start* of the entry block rather than at the real
insertion point (just above the terminating branch) over-approximates the
widened range, so the test errs toward rejecting. This is what correctly
declines `ext2fs_read_inode2`, whose `v10` is defined in the entry block.

## Not done

* The op keeps its original `Address` (the predecessor block's stop address)
  while sitting in the entry block. `PcodeOpBank` keys its `optree` on the
  `SeqNum`, so re-addressing an op would mean destroying and rebuilding it. The
  emitted C is identical either way (verified against the discarded trim-time
  prototype, which did carry the entry block's address).
* The `Merge::mergeIndirect` call-adjacent family is out of scope, as above.
