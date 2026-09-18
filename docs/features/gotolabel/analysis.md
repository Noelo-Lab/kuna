# gotolabel — analysis

## 1. The measured symptom

`kuna decompile-all <O2/coreutils/stripped/sort> --addr 0x3ec0` printed a switch
arm that jumps to a label the function does not contain:

```c
    switch(v52) {
      default:
        goto label_661d;        // there is no `label_661d:` in main
```

`gcc -fsyntax-only` on the emitted function says so:

```
error: label ‘label_661d’ used but not defined
```

Class size, over 20 stripped binaries (O0 / O2 / O2-noinline; coreutils, grep,
gzip, bzip2, diffutils, findutils, tar, shadow, plus an ARM32 betaflight image),
16,080 functions decompiled with `decompile-all` and scanned for a `goto X;`
whose `X:` is absent from the same function (`.scratch/orphan_scan.py`, shipped
in this bundle as `orphan_scan.py`):

| corpus | functions | functions with an orphan goto | orphan gotos |
|---|---:|---:|---:|
| 20 stripped binaries | 16,080 | 23 | 27 |
| 149 in-repo ELF fixtures | 2,710 | 10 | 10 |

## 2. Three causes, one invariant

The printer emits `label_X:` for a block carrying `f_unstructured_targ`
(`substrate/block.rs (BlockGraph::mark_unstructured)` sets it; `printc.rs
(PrintC::emit_label_statement_c)` reads it). Every emitted `goto` must therefore
have a marked target. Three independent paths broke that:

1. **A released label a switch arm still needed.** `gotoreduce`, `taildup` and
   `crossjumprevert` convert a `goto` into a duplicated tail and then clear the
   target's label if nothing else jumps there. Each carried its own copy of the
   "still referenced" census, and all three copies looked at `BlockGoto` and
   `BlockIf` carriers only — a `BlockSwitch` case arm whose edge stayed
   unstructured (and a `BlockMultiGoto`'s virtual edges) read as *unreferenced*.
   `sort` O2 `main` is this: `taildup` converted one goto into a tail and
   released the label the `default:` arm still jumped to. `--option taildup off`
   made the witness disappear, which is what identified the pass.

2. **A goto the printer emits but the marker skipped.** `mark_unstructured`
   marks a `BlockGoto`'s target only `if goto_prints()` — upstream's rule,
   correct upstream because `PrintC::emitBlockGoto` suppresses the goto under
   the same test. kuna's `emit_block_goto` prints the trailing goto for every
   target it has, so a goto whose target *is* the next block in flow printed
   unlabelled (`grep` O0 `main`, `goto label_aa4b;` with the target block
   immediately below it).

3. **A release that compared the wrong blocks.** The label lives on the
   target's *front leaf*, but the release test compared the blocks the carriers
   name. A surviving `goto` naming an enclosing `BlockList` and a converted one
   naming the `BlockCopy` that opens it resolve to the same leaf, so the label
   was released while the list-targeting goto still printed (`shadow/useradd` O2
   `sub_16720`).

## 3. Why this is not an option

Each cause produces output that is not C — a `goto` with no label never renders
a judgement call differently, it fails to compile. All three fixes are
one-directional: they can only *keep* or *add* a label, never remove one, so no
function without an orphan goto can change. The corpus diff confirms it: 23 of
16,080 functions change, and all 23 are exactly the functions that carried an
orphan goto.
