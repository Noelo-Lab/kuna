# Analysis — angr `test_od_else_simplification` (`skip` in `od_gccO2.o`)

## The gap

angr's decompiler emits the early-exit guard of `skip` as the **first** statement and
de-indents the body:

```c
if (!cur)
    return 1;
v3 = 0;
v4 = 1;
while (in_stream) { ... }       // body de-indented, no else wrapping
```

kuna instead wraps the **entire** function body inside `if (a0 != 0) { ... }` and lets the
`!a0` path fall implicitly through to a shared epilogue:

```c
if (a0 != 0) {
    ...whole body...
}
v14 = 1;
label_403d25:
if (canary != ...) __stack_chk_fail();
return v14;                      // the !a0 path lands here -> return 1
```

The angr test (`cstyle_ifs=False`) asserts the first `if` is `if (!a0) { return 1; }` with
**no else** — i.e. an early-return guard, not a body-wrapping condition.

## Why angr is better here

The early-return guard is the idiomatic shape for a function whose first action is an
argument-validity check. kuna's body-wrapping `if (a0 != 0) { ... }` pushes the entire
function one indent level deeper and obscures the guard.

## angr's mechanism (read from angr source)

Closing this gap is **two cooperating angr passes**, both operating on angr's mutable
region tree (`ConditionNode` / `SequenceNode` / `LoopNode`):

1. **Return duplication** — `optimization_passes/return_duplicator_*.py`. The shared
   canary+`return 1` epilogue is duplicated into the `!a0` guard path so that path becomes a
   standalone, *terminating* `return 1`.
2. **If/else flattening** — `region_simplifiers/ifelse.py` `IfElseFlattener`. Once the if's
   true-branch always terminates (returns) and the false-branch does not, it removes the
   `else` node and re-parents the body as a sibling **after** the `if`, de-indenting it.

## Owning stage in kuna

kuna has **no angr-style mutable region tree at emit time**. The if/else shape is decided by
Ghidra's **block-graph collapse Structurer** — `CollapseStructure` / the block-if-else rules
in `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs` (~3600 lines), Ghidra's
S7/S8 region recovery. Reproducing angr's output requires re-creating both halves of angr's
mechanism (return duplication into the guard path + else removal + body de-indent) inside
that Structurer, plus a likely emit-side change.

## Verdict

**LARGE** (Hard Rule 7): a new structuring pass *type* that touches S7/S8 structuring code
well beyond a single gated early-return. Not modelable as one Action/Rule like
`kuna_loweredswitch.rs` (an S2 detection pass). A decider subagent independently confirmed
`scope: large` (see `record.json` → `decisions`). This is the same S7/S8 SAILR
region-structuring class consistently parked LARGE in the five prior ET_REL opportunities
(`condfold`, `elf-reloc-objects`, …).

Note: `od_gccO2.o` is an ET_REL object that kuna's PT_LOAD-only loader cannot map (the
loader gap, proposal PR #37). The kuna side above was obtained by link-bypass into a
standalone ELF (`/tmp/kuna_od/synth`) so the *structuring* gap could be isolated from the
loader gap. The residual structuring gap is the subject of this proposal.
