# paramcopyhoist — the copy-shadow of an unmodified parameter lands outside the entry block

Case: `O0-e2fsprogs-e2fsck-save_output` (decbench route B).
Triage record: `docs/decbench/triage/O0-e2fsprogs-e2fsck-save_output-rederive.md`
(branch `campaign/decbench-round2-triage`).

## The gap

`kuna decompile --addr 0x4eeb6` on `O0/e2fsprogs/stripped/e2fsck`:

```c
  v11 = *(unsigned long *)(v4 + 0x28);
  v7 = a0;
  if ((a0) && (!*a0))
    v7 = NULL;
  v6 = a1;                       // <-- NOT in the entry block  (+1 CFG node)
  if ((a1) && (!*a1))
    v6 = NULL;
  v5 = a2;                       // <-- NOT in the entry block  (+1 CFG node)
  if ((a2) && (!*a2))
    v5 = NULL;
```

The source (`logfile.i`) writes through the parameter, so its entry block holds
only the frame setup, and in the binary all three spills are likewise in the
entry basic block (`4eec2/4eec6/4eeca`; the entry ends at the `je` at `4eee2`).
kuna is internally inconsistent about the identical construct: parameter 1's
copy-shadow lands in the entry block, parameters 2 and 3 sink below their
guards, splitting the source's single entry block into three.

Measured with the real GED (`scripts.decbench.rescore`):

| | nodes | edges | GED |
|---|---|---|---|
| source | 46 | 72 | — |
| kuna base | 53 | 77 | **29.0** |
| hoisted | 51 | 77 | **17.0** |

-12 GED. The round-2 refuter also measured the alternative of deleting the
copies entirely and writing through the parameter (literally what the source
does) at **17.0 as well**, so the hoist is GED-optimal for this construct.

## Root cause — P6, `Merge::trimOpInput`

`gcc -O0` gives every parameter a stack home, so the guard's join carries a
MULTIEQUAL whose incoming slots are the raw input register (`print raw`,
BB0/BB3/BB6):

```
0x0004eee2:8ce:  u0x10000269 = RDI(i)
0x0004eef7:77c:  s0xff..ffc0 = u0x10000269 ? u0x10000269 ? s0xff..ffc0(0x0004eeef:46)
0x0004eefc:8cc:  u0x10000259 = RSI(i)
0x0004ef11:758:  s0xff..ffb8 = u0x10000259 ? u0x10000259 ? s0xff..ffb8(0x0004ef09:67)
0x0004ef16:8ca:  u0x10000249 = RDX(i)
```

`Merge::mergeOp` cannot merge the input-register HighVariable with the
address-tied stack-slot HighVariable, so `Merge::trimOpInput` snips each
offending slot into a COPY and places it with
`opInsertEnd(copyop, op->getParent()->getIn(slot))` — the **tail of that slot's
predecessor block**. That COPY is what prints as `vN = aM;`.

Two sub-cases produce the symptom, and the shipped fix covers both:

* **Two or more trimmed slots** (the `save_output` witness). Each slot gets its
  own COPY; `Merge::processCopyTrims` -> `Merge::buildDominantCopy` then
  collapses them at `FlowBlock::findCommonBlock` of the trim sites — a pure
  `immed_dom`-chain intersection. For `a0` that is `{BB0,BB1}` -> BB0, the entry
  block, which is exactly why parameter 1 already renders correctly; for `a1` it
  is `{BB3,BB4}` -> BB3 and for `a2` `{BB6,BB7}` -> BB6.
* **Exactly one trimmed slot** — no dominant-copy pass runs at all and the
  single `opInsertEnd` is the final placement. Witness `ea_refcount_free` @
  `0x4a9e9`: `0x0004aa0d:91: u0x10000042 = RDI(i)` at the tail of BB1, rendering
  `v1 = a0;` below `if (!a0) return;`.

This is a **divergence**, not a port bug: upstream Ghidra sinks
`local_48 = param_2;` to exactly the same place, so it ships behind an option.

## Breadth

`kuna decompile-all --json` over four O0 binaries, counting top-level (brace
depth 1) `vN = aM;` statements emitted after the first top-level control-flow
statement:

| binary | fns | fns with a sunk copy | sunk | in entry |
|---|---|---|---|---|
| O0 e2fsprogs/e2fsck | 1991 | 100 (5.0%) | 172 | 235 |
| O0 coreutils/ls | 617 | 12 (1.9%) | 19 | 52 |
| O0 gzip/gzip | 263 | 6 (2.3%) | 6 | 26 |
| O0 bzip2/bzip2 | 160 | 3 (1.9%) | 3 | 5 |
| **total** | **3031** | **121 (4.0%)** | **200** | **318** |

So ~4 in 10 top-level parameter copy-shadows kuna emits are outside the entry
block.

## Three things that had to be measured, not reasoned

Everything below was found by building the change and diffing decompiled C over
all 3,031 functions, not by inspection. Each one produced wrong C.

### 1. `Merge::trimOpInput` is the wrong hook site

The triage proposed hooking `trimOpInput`'s MULTIEQUAL arm directly (via
`op_insert_end_pred`). **That was implemented, measured, and rejected.**

`trimOpInput` exists precisely to give the snipped slot a *tiny* Cover so the
following `Merge::mergeOp` cover test passes. Widening it there makes that test
fail, and `mergeOp` falls through to `trimOpOutput` ("one last trim"),
restructuring the MULTIEQUAL into `u = MULTIEQUAL(...); dest = u;` whose two
COPYs `markInternalCopies` then hides. Measured on `e2fsck ext2fs_read_inode2`
(`0x7f6bb`):

```c
-      if ((int4)a3 <= (int4)v10)
-        v10 = a3;
+      if ((int4)a3 <= (int4)v10) {
+      }
       memcpy(a2, ..., (int8)(int4)v10);      // v10 never receives a3
```

`print raw` confirms the mechanism: the phi output moved from `EAX` to a fresh
unique plus a trailing `EAX = u0x1000031e` COPY, and both COPYs were hidden.

The legality question is also *unanswerable* at trim time. `mergeAddrTied` has
run but `ActionMergeCopy` has not, so the stack slot's other definitions
(`v10 = 0x80`, in the entry block) are still in a **different** HighVariable —
the entry-block clobber is invisible to any Cover test run there.

### 2. Running before `ActionFinalStructure` perturbs structuring

With the pass placed right after `ActionCopyMarker`, moving a COPY out of a
basic block changes what the P8 duplication/dedup passes see. Measured on
`e2fsck sub_544cc`, where an `if (!a0) return;` early return degenerated into an
`if (a0) { ... return; }` wrapper. Running the pass **last** removes the
interaction: nothing downstream reads block contents again, so the only thing
the move can change is which basic block's statement list holds the COPY.

### 3. The Cover test cannot see two definitions converging

The Cover test judges each move against the placements the *other* definitions
have today. Two definitions of one variable can therefore both be accepted: each
is dead where the other is live so they never intersect, and yet after both moves
the second write kills the first on every path. Measured on `e2fsck sub_7431a`:

```c
-  if (a1 != *a0)
-    v1 = a1;
-  else { ... v1 = a2; }
+  v1 = a1;
+  v1 = a2;                      // returns a2 unconditionally -- wrong
+  if (a1 != *a0) { }
+  else { ... }
```

The shipped pass therefore drops every candidate whose HighVariable has more
than one. A parameter copy-shadow is one COPY into one variable, so the
restriction costs nothing on the construct.
