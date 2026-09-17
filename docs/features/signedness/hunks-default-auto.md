# `signedness` — every hunk the default flip writes, classified

What changed when `auto` became the default, over twelve whole binaries from
nine projects — eight x86-64 and four 32-bit ARM (Cortex-M) firmware images:
`kuna decompile-all <bin>` (the new default) diffed against
`kuna decompile-all <bin> --option signedness upstream` (what the same build
printed before the flip), function by function, with every changed line-pair
classified mechanically.

Classifier: `classify_hunks.py`, next to this file — a pair is `decl-signedness`
when it is the same declaration with the same name and the same base type at a
different signedness, `cast-dropped` when the two lines are identical after
deleting every
integer cast token and the new one has fewer of them, `CAST-ADDED` when it has
more, and anything else falls into an `OTHER` bucket that is reported with a
sample. It reads all three printer vocabularies: the `aggressive` spelling
(`unsigned int`), the `reliable` spelling a binary over 500 KiB gets (`uint4`,
which is what `tar` prints), and the 8-byte spelling of a 32-bit target
(`unsigned long long`, where an LP64 target says `unsigned long`). The third one
matters: on ARM32 a flipped 8-byte local reads

```
-  long long v2;
+  unsigned long long v2;
-      v7 = ... + (int)((unsigned long long)v2 >> 0x20);
+      v7 = ... + (int)(v2 >> 0x20);
```

and both halves — the declaration and the cast it makes a no-op — are classified,
not dropped into `OTHER`.

## coreutils — `fmt`, `ls`, `sort`, `du`

| bin | opt | funcs | changed | decl flips | cast-drop hunks | other hunks |
|---|---|---|---|---|---|---|
| fmt | O0 | 191 | 0 | 0 | 0 | **0** |
| ls | O0 | 589 | 3 | 3 | 1 | **0** |
| sort | O0 | 479 | 4 | 4 | 5 | **0** |
| du | O0 | 441 | 2 | 2 | 2 | **0** |
| fmt | O2 | 151 | 3 | 4 | 6 | **0** |
| ls | O2 | 404 | 4 | 7 | 6 | **0** |
| sort | O2 | 343 | 8 | 9 | 18 | **0** |
| du | O2 | 320 | 10 | 13 | 15 | **0** |
| **total** | | **2,918** | **34** | **42** | **53** | **0** |

Cast tokens: 5,092 → 5,030 (62 removed, **0 added**).
Flip directions: `unsigned int -> int` 20, `unsigned long -> long` 9,
`long -> unsigned long` 7, `int -> unsigned int` 6. Sub-`int` flips: **0**.

## grep, gzip, tar, findutils — four projects the earlier sweep did not cover

| bin | opt | funcs | changed | decl flips | cast-drop hunks | other hunks |
|---|---|---|---|---|---|---|
| grep | O0 | 642 | 6 | 6 | 8 | **0** |
| gzip | O0 | 248 | 9 | 11 | 34 | **0** |
| tar | O0 | 1,570 | 24 | 28 | 22 | **0** |
| find | O0 | 862 | 7 | 7 | 7 | **0** |
| grep | O2 | 449 | 14 | 14 | 20 | **0** |
| gzip | O2 | 204 | 9 | 13 | 29 | **0** |
| tar | O2 | 1,125 | 36 | 39 | 30 | **0** |
| find | O2 | 658 | 19 | 22 | 21 | **0** |
| **total** | | **5,758** | **124** | **140** | **171** | **0** |

Cast tokens: 15,226 → 15,042 (184 removed, **0 added**).
Flip directions: `unsigned int -> int` 39, `uint8 -> int8` 21, `int4 -> uint4`
19, `int8 -> uint8` 16, `int -> unsigned int` 13, `unsigned long -> long` 12,
`uint4 -> int4` 11, `long -> unsigned long` 9. Sub-`int` flips: **0**.

## libopencm3, FreeRTOS, ChibiOS, crazyflie — 32-bit ARM

Stripped Cortex-M firmware, where an 8-byte local is spelled `long long` and a
`size_t` is 4 bytes, so the arithmetic the pass reads is a different shape from
the x86-64 sweeps above.

| bin | opt | funcs | changed | decl flips | cast-drop hunks | other hunks |
|---|---|---|---|---|---|---|
| adc-dac-printf.elf | O0 | 181 | 14 | 22 | 33 | **0** |
| RTOSDemo.out | O0 | 150 | 0 | 0 | 0 | **0** |
| ch.elf | O0 | 772 | 5 | 5 | 2 | **0** |
| cf2.elf | O0 | 2,679 | 34 | 45 | 49 | **0** |
| adc-dac-printf.elf | O2 | 171 | 18 | 26 | 36 | **0** |
| RTOSDemo.out | O2 | 126 | 3 | 3 | 6 | **0** |
| ch.elf | O2 | 398 | 17 | 19 | 6 | **0** |
| cf2.elf | O2 | 1,971 | 61 | 74 | 65 | **0** |
| **total** | | **6,448** | **152** | **194** | **197** | **0** |

Cast tokens: 16,910 → 16,713 (197 removed, **0 added**).
Flip directions: `unsigned int -> int` 95, `int -> unsigned int` 70,
`long long -> unsigned long long` 29. Sub-`int` flips: **0**.

The 32-bit side is the *more* active one per function: `adc-dac-printf.elf` at
`-O2` changes 18 of 171 functions where `grep -O2` changes 14 of 449.

## All three sweeps together

15,124 functions decompiled over twelve binaries and nine projects, **310** of
them changed, **376** declaration flips, **421** cast-drop hunks, **443** cast
tokens removed and **0** added, and **0** hunks of any other kind — no statement
moved, deleted, re-anchored or re-worded, and no declaration appeared or
disappeared. Also **0** flips on a declaration narrower than `int`, which is the
promotion-width guard holding on every one of these targets (all 4-byte-`int`;
`tests/stages/kuna-signedness-int16.xml` is what pins the guard where the
target's own `int` is 2 bytes).

The stderr of each pair is identical too, so no analysis-tier decision moved.

Reproduce any row with the two `decompile-all` runs the classifier's header
shows; the raw dumps this table was built from are untracked (`.scratch/dumps4`,
`.scratch/dumps6`, `.scratch/dumps7`).
