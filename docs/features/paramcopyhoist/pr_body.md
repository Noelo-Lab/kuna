## What

`gcc -O0` gives every parameter a stack home, and the source idiom
`if (p && !*p) p = NULL;` leaves a MULTIEQUAL at the guard's join whose incoming
slots are the raw input register. `Merge::mergeOp` cannot merge the
input-register HighVariable with the address-tied stack-slot HighVariable, so
`Merge::trimOpInput` snips each offending slot into a COPY and places it at the
tail of that slot's **predecessor** block — and that COPY is what prints as
`vN = aM;`.

For the **first** parameter the guard's join predecessors intersect at the entry
block, so `buildDominantCopy`'s `findCommonBlock` lands the copy there and it
renders correctly. For every later parameter the intersection is the previous
guard's join, so the copy sinks below that guard and splits the source's single
entry block in two. kuna is internally inconsistent about the identical
construct; upstream Ghidra sinks it the same way, so this ships as an **option**
(`paramcopyhoist`, default-OFF), not as a bug fix.

Both producing sub-cases are covered:

* **two-or-more trimmed slots** — later collapsed by `buildDominantCopy`
  (witness `e2fsck save_output` `0x4eeb6`);
* **exactly one trimmed slot** — no dominant-copy pass runs at all, so the single
  `opInsertEnd` is the final placement (witness `e2fsck ea_refcount_free`
  `0x4a9e9`).

## Before / after

`kuna decompile ~/…/O0/e2fsprogs/stripped/e2fsck --addr 0x4eeb6`

```diff
   v11 = *(unsigned long *)(v4 + 0x28);
   v7 = a0;
-  if ((a0) && (!*a0))
-    v7 = NULL;
-  v6 = a1;                     // outside the entry block: +1 CFG node
-  if ((a1) && (!*a1))
-    v6 = NULL;
-  v5 = a2;                     // outside the entry block: +1 CFG node
-  if ((a2) && (!*a2))
-    v5 = NULL;
+  v5 = a2;
+  v6 = a1;
+  if ((a0) && (!*a0))
+    v7 = NULL;
+  if ((a1) && (!*a1))
+    v6 = NULL;
+  if ((a2) && (!*a2))
+    v5 = NULL;
   if (((!v7) && (!v6)) && (!v5))
     return 0;
```

Second sub-case, `--addr 0x4a9e9`:

```diff
-  if (!a0)
-    return;
   v1 = a0;
+  if (!a0)
+    return;
   if (*(int8 *)(a0 + 0x18))
     ext2fs_free_mem(a0 + 0x18);
```

## GED

`scripts.decbench.rescore --case O0-e2fsprogs-e2fsck-save_output --option paramcopyhoist on`

| | nodes | edges | GED |
|---|---|---|---|
| source | 46 | 72 | — |
| before | 53 | 77 | **29.0** |
| after | 51 | 77 | **17.0** |

**−12 GED**, and the round-2 refuter separately measured the alternative of
deleting the copies entirely and writing through the parameter (literally what
the source does) at **17.0 as well** — so the hoist is GED-optimal for this
construct and there is nothing left on the table here.

## Breadth (four O0 binaries, 3,031 functions)

| | off | on |
|---|---|---|
| top-level param copy-shadows **outside** the entry block | 200 | **108** |
| top-level param copy-shadows **in** the entry block | 318 | **479** |
| functions with at least one sunk copy | 121 | 63 |
| new decompile errors | — | **0** |

116 functions change. **All 116 were diffed**: 99 are pure re-orderings of
`vN = aM;`; 17 have a secondary rendering effect — mostly a comma side-effect
inside a short-circuit condition becoming a plain statement (an improvement);
2 leave an empty `if (c) { }` then-branch. **0 semantic regressions.**

## Three things that had to be measured, not reasoned

Each of these produced **wrong C** and was found by building the change and
diffing decompiled output over all 3,031 functions. They are why the shipped
pass looks the way it does.

1. **`Merge::trimOpInput` is the wrong hook site** (the mechanism the triage
   record proposed). `trimOpInput` exists to give the snipped slot a *tiny* Cover
   so `mergeOp`'s test passes; widening it there makes that test fail, `mergeOp`
   falls through to `trimOpOutput`, and `markInternalCopies` hides both resulting
   COPYs. On `e2fsck ext2fs_read_inode2` (`0x7f6bb`) the assignment vanished:
   ```diff
   -      if ((int4)a3 <= (int4)v10)
   -        v10 = a3;
   +      if ((int4)a3 <= (int4)v10) {
   +      }
          memcpy(a2, ..., (int8)(int4)v10);   // v10 never receives a3
   ```
   The legality question is also unanswerable there: `mergeAddrTied` has run but
   `ActionMergeCopy` has not, so the stack slot's other definitions are still in a
   *different* HighVariable and the entry-block clobber is invisible.
2. **Running before `ActionFinalStructure` perturbs structuring.** Moving a COPY
   out of a block changes what the P8 duplication/dedup passes see —
   `e2fsck sub_544cc` lost an `if (!a0) return;` early return. The pass now runs
   **last** in `universal_sched`, so the only observable effect is which basic
   block's statement list holds the COPY.
3. **The Cover test cannot see two definitions converging.** It judges each move
   against where the *other* definitions sit today, so two definitions of one
   variable can both be admitted — each dead where the other is live, never
   intersecting — and after both moves the second kills the first.
   `e2fsck sub_7431a` started returning `a2` unconditionally. Hence
   `one_per_high`.

## Mechanism as shipped

`p6_variables/kuna_paramcopyhoist.rs` — `ActionParamCopyHoist` (ELEM 4120), the
last action in `universal_sched`. A candidate must be a printing `COPY` whose
input occupies a **formal parameter's** storage (`Varnode::isInput` alone also
admits globals and read-before-written stack slots), at least one of whose reads
is a `MULTIEQUAL` and all of whose reads are `MULTIEQUAL`/`INDIRECT` (the
INDIRECT-only case is `mergeIndirect`'s call-adjacent snip, deliberately out of
scope), whose HighVariable has exactly one candidate, and which passes
`buildDominantCopy`'s own Cover test re-run against the hypothetical hoisted
placement and the **final** high — with the `a_cover` def point taken at the
*start* of the entry block so it over-approximates and errs toward rejecting.
That test is what correctly declines `ext2fs_read_inode2`.

The only ported-core edit is one `// (kuna)` visibility widening:
`FuncdataCoverCtx` becomes `pub(crate)`.

## Default

**OFF, opt-in.** The default-ON ablation is clean on the datatests —
**675/675, PARITY OK, no re-pin** — and the speed gate passes, but:

* with the flip the **stage corpus regresses 1 assertion** (`condfold #3`, a
  comma-side-effect promotion);
* the **decbench aggregate ablation** the triage record named as the second gate
  for this feature has not been run;
* 2 of the 116 audited functions render a locally worse empty then-branch.

`CLAUDE.md` states 0/675 + speed as a *necessary* condition for default-ON, not
a sufficient one, so the flip is left to a follow-up with the aggregate measured.
No DIV row.

## Speed

`scripts.pipeline.timeit --option paramcopyhoist ...` on the witness, three
independent runs while the box was also running a 40-way IDA batch plus sibling
build agents:

| repeat | off (ms) | on (ms) | delta |
|---|---|---|---|
| 7 | 322.5 | 314.6 | −2.4% |
| 7 | 663.9 | 225.7 | −66.0% |
| 11 | 304.7 | 221.1 | −27.4% |

All three are **within the +5% budget**; the spread is load noise, not signal
(the option cannot make decompilation faster — the pass is one `iter_alive` scan
plus one Cover build per candidate, and on most functions it finds no candidate
at all). The honest reading is "no measurable cost".

## Gates

```
make test        -> datatests: 675/675 assertions passed   PARITY OK
make test-stages -> datatests: 361/361 assertions passed   PARITY OK
make rust-test   -> green
make check-spec  -> check-spec OK (strict mode)
kuna catalog --check -> catalog OK: documents exactly the registered kuna options
```

## Artifacts

`docs/features/paramcopyhoist/` — `analysis.md`, `plan.md`, `record.json`.
Test: `tests/stages/ghdec-paramcopyhoist.xml` (two-pass; pass 1 `option
paramcopyhoist off` asserts the bug, pass 2 `on` asserts the fix).
Spec: `docs/spec/06-variables-and-merge.md` §6.4.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
