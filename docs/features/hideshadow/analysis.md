# hideshadow — wiring ActionHideShadow

## The gap

`ActionHideShadow` is scheduled in kuna's merge phalanx (index 0252, immediately
before `copymarker`) and its engine half, `Merge::hide_shadows`, is a complete
port of `merge.cc:1070-1100`. The action body was not: it returned 0 with the
upstream body transcribed in a comment and three blockers named — no written
def-set iterator, no HighVariable mark surface, no `MergeContext` bridge.

All three are stale:

| blocker named by the stub | reality |
|---|---|
| `beginDef`/`endDef(flags)` def-set iterator | `VarnodeBank::iter_def_flag` (`substrate/varnode.rs:2234`) |
| the HighVariable mark surface | `MergeContext::high_is_mark` / `bank_set_mark` / `bank_clear_mark` (`p6_variables/funcdata_merge.rs:340,472,477`), already used by `Merge::merge_by_datatype` |
| the `getMerge()`/`MergeContext` bridge | `Funcdata::with_covermerge` (`p6_variables/funcdata_merge.rs:259`), already driven at `coreaction_cleanup.rs:660,1542,1591,1639,1728,1775` |

So the body is the upstream body, unmodified.

## What the pass does, and what actually changes in the C

`Merge::hide_shadows` re-points the later of two copies of one value at the
earlier Varnode, so ancestor -> first -> second becomes a single chain. Both
Varnodes are already instances of the SAME HighVariable (`find_single_copy`
walks `high->getInstance(i)`); what is NOT in that high is the COPY's *input*.
After the re-point the input is in the high too, and `ActionCopyMarker`
(`Merge::mark_internal_copies`), the very next action, marks the COPY
non-printing. That is the whole visible effect: one assignment statement
disappears. **No declaration disappears** — the variable survives, only the
redundant write to it goes. `varcensus` therefore reports a zero delta
(measured on ls O0/O2, touch O2, usermod O2-noinline: `declarations` and
`single_def_single_use` identical in both arms), and that is the honest
framing: this is a statement-count item, not a declaration-count item.

## Why it fires rarely

Instrumented run over 40 stripped ELFs (O0 + O2): the walk reached 9,248
HighVariables whose `find_single_copy` list had more than one entry; the pair
tests accepted **110** and declined **9,138**. Of the accepted ones, most
re-point a COPY the printer was already suppressing for another reason — the
`for`-loop initialize/iterate ops that `funcdata_block.rs:567,571` marks
non-printing because the loop header prints them. Only the remainder reach the
text.

The two tests that do the declining are upstream's, and they are the
wrong-output guard:

1. `Varnode::copyShadow` — both Varnodes must trace to one common ancestor
   through COPYs only. A COPY never changes a value, so the two are the same
   value by construction; there is no "equal on some paths" case to get wrong.
2. `Cover::containVarnodeDef(...) == 1` — the surviving Varnode's live range
   must contain the other's definition point **strictly interior**, not at a
   boundary. This is what makes the re-pointed read legal: the value is live
   and unclobbered exactly where the copy is made to read it. Two copies on
   exclusive branches (the common case) fail it.

kuna reads the Cover through `MergeContext::vn_cover_ref`, a plain field read,
where C++ `Varnode::getCover()` lazily re-runs `updateCover()` on a cover that
was dirtied — and the previous iteration's `opSetInput` is exactly what dirties
it. `hide_shadows` now calls `MergeContext::bank_update_cover(high)` before each
containment test, which rebuilds every member Varnode's cover
(`Funcdata::refresh_high_cover`) and then the high's. Measured: on the binaries
that change, the output is identical with and without the refresh, so no
decision in the corpus depended on a stale cover — the refresh is there so the
guard cannot silently decide on one.

## Whole-corpus classification

444 stripped ELFs (coreutils, grep, gzip, diffutils, bzip2, findutils, tar,
shadow at O0, O2, O2-noinline), `decompile-all` with `--option hideshadow off`
vs `on`. See `record.json` for the table and `corpus-diffs.txt` for every hunk.


## Result

444 stripped ELFs: 434 byte-identical, 10 changed, 15 duplicate assignments
removed (12 whole-line deletions plus 3 lines rewritten to drop a duplicated
comma-expression term; net -12 lines). Every one of the 15 is an assignment
that an identical assignment already dominates. Zero BUG hunks,
zero declarations moved, `type_match` unmoved (`record.json`). Speed within
budget on every surface measured, against an off-vs-off control that puts this
box's noise floor at about +/-1.4%. Shipped **default-on**: it is the upstream
pass, stock Ghidra runs it unconditionally, 0/675 datatest assertions move and
the stages corpus stays PARITY OK. `option hideshadow off` restores the stub's
behaviour exactly.
