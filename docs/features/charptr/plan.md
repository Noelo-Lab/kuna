# `charptr` — plan

## Question

Ground-truth `char *` is the largest class in the decbench corpus (25.9% of all
ground-truth variables) and kuna matched 28.7% of it on `main` 2e28ece4a,
against Binary Ninja's 36.8%. Where is that gap, and what evidence would close
it?

## Step 1 — measure before building

Census the class from the round-C decision dump (`analysis.md`): split every
ground-truth `char *` by the storage it lives in, by the spelling kuna gave the
variable the metric matched, and against what the three rivals spell for the
same variables. Then census, inside kuna, what evidence actually exists for the
values the rule could reach.

Result: half the class is register-only and unreachable for every decompiler;
kuna's *argument* recovery is within 6.6 points of binja's; nine tenths of the
remaining gap is stack locals, and 932 of those 1,648 stack misses are
`framelayout` filler slots with no recovered type attached, which is a
chapter-06 problem. The evidence census says the dominant category for what is
left is "none of the above".

## Step 2 — ship the evidence that is there

`charptr off|libc|uses`, a P5 seed-fold candidate built on `ptrfromuse`'s walk:

- `libc`: the value reaches a call argument the callee **declares** `char *`
  (`declared_input_type_local`, so `protoorder`'s recovered vote is excluded —
  measured, including it scores lower).
- `uses`: adds byte-width-only dereference and character-array constants.
- Refines `void *` / `undefined1 *` but never a pointer at a named or sized
  thing; never a type-locked Varnode; refuses on a wider dereference, a wider
  element step, a contradicting callee declaration, and everything `ptrfromuse`
  refuses.
- Only function inputs and stack-space Varnodes: the storage a reader sees as a
  declaration.

## Verification

- `tests/stages/kuna-charptr.xml`, three passes (`off` = the bug, `libc` = the
  call-site half, `uses` = both) with three controls that must not move.
- Unit tests on the option surface and the two fold properties.
- 444-slice typesweep, both directions, `improved` and `worse` per project.
- Whole-corpus `decompile-all` over 8 binaries: default output byte-identical to
  `main`'s (the option ships off), plus every hunk the option's own value
  produces classified.
- `timeit` interleaved min-of-15 on fmt/ls/sort -O2 and bash -O2.

## Default

Ships **off**. The measured gain is real and one-directional but small, and
turning it on is a claim: a committed pointer forfeits the width-only free pass
an eight-byte scalar gets, and `char *` rewrites the body's arithmetic into
indexing. `docs/features/charptr/record.json` carries the numbers a later flip
would be argued from.
