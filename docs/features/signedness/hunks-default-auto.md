# `signedness` — every hunk the default flip writes, classified

What changed when `auto` became the default, over eight whole binaries from five
projects: `kuna decompile-all <bin>` (the new default) diffed against
`kuna decompile-all <bin> --option signedness upstream` (what the same build
printed before the flip), function by function, with every changed line-pair
classified mechanically.

Classifier: `.scratch/analyze6b.py` — a pair is `decl-signedness` when it is the
same declaration with the same name and the same base type at a different
signedness, `cast-dropped` when the two lines are identical after deleting every
integer cast token and the new one has fewer of them, `CAST-ADDED` when it has
more, and anything else falls into an `OTHER` bucket that is reported with a
sample. It reads both printer vocabularies, because a binary over 500 KiB
(`tar`) is decompiled in `reliable` mode and spells its types `uint4`, not
`unsigned int`.

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

## Both sweeps together

8,676 functions decompiled, **158** of them changed, **182** declaration flips,
**224** cast-drop hunks, **246** cast tokens removed and **0** added, and **0**
hunks of any other kind — no statement moved, deleted, re-anchored or re-worded,
and no declaration appeared or disappeared. Also **0** flips on a declaration
narrower than `int`, which is the promotion-width guard holding on every one of
these targets (all of them 4-byte-`int`; `tests/stages/kuna-signedness-int16.xml`
is what pins the guard where the target's own `int` is 2 bytes).

The stderr of each pair is identical too, so no analysis-tier decision moved.

Raw dumps and the classifier output: `.scratch/dumps4`, `.scratch/dumps6`,
`.scratch/hunks4.txt`, `.scratch/hunks6.txt` (untracked).
