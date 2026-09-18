# structsynth — the default flip (`off` → `param`)

The user decided on 2026-09-18 that `structsynth` ships on. This file is the
evidence the flip was measured against, on the tree rebased onto `d3617d64`
(which carries #674, the fix for a `goto` whose label was never emitted). The
earlier opt-in evaluation is `default_decision` in `record.json`; everything here
supersedes it.

## Corpora

| gate | result |
|---|---|
| `make test` with the flip | 675/675, PARITY OK — no datatest assertion moves |
| `make test-stages` with the flip, before touching a test | 1137/1141: `ELFMAIN #1/#2`, `PEBNAMES-X86 #6/#8` |
| `make test-cli` | 185/185 before the new probe, 186/186 after |

The four stage assertions are exactly the ones the opt-in evaluation predicted,
and each was read in both arms:

- **`ELFMAIN #1/#2`** — the `elfmain off` pass. With the entry left anonymous
  its argument vector is untyped, and the pass reads `argv[0]`/`argv[1]` as a
  two-field `struct_0 *` (`a1->field_0x0`, `a1->field_0x8`). An array of
  `char *` is not a record; this is the pointer-width pair class the array rule
  keeps (see *decbench `type_match`* below), and the default `elfmain on` declares
  `char **argv` locked, so the pass never sees it. The test's first pass pins
  the upstream form, so it gains `option structsynth off`.
- **`PEBNAMES-X86 #6/#8`** — the SEH-linking function `pebnames` deliberately
  leaves untyped. Its FS base is read at 0 (the registration link) and 0x30
  (the PEB pointer), and becomes a two-member `struct_0 *`: `*v4 = v3` becomes
  `v4->field_0x0 = v3`, `v4[0xc]` becomes `v4->field_0x30`. Same bytes, and the
  TEB really is a structure. The two regexes follow the new spelling; `#8`'s name
  said "untyped base", so it is renamed.

The three `structsynth-*.xml` tests already turned the option off in their first
pass; their `#1` names said "default leaves ...", and now say "off leaves ...".
`docs/baseline-stages.json` was re-recorded: 1141/1141, and the only key changes
are those four renames.

## Whole-corpus `decompile-all` sweep

14 stripped binaries, 5,610 functions: coreutils `fmt`/`ls`/`sort`/`du` (O2, plus
`sort` and `du` at O0), findutils `find`, grep, gzip, bzip2, diffutils `diff`,
tar (530 KiB, so `auto` runs it under `reliable`) at O2, and the ARM32
firmwares `chibios` and `freertos` at O2. The off arm is a release build of
`d3617d64`; the on arm is this branch at its defaults. As a control, this branch
with `--option structsynth off` is byte-identical to the `d3617d64` build on all
14.

| binary | functions | changed |
|---|---:|---:|
| fmt O2 | 151 | 3 |
| ls O2 | 404 | 70 |
| sort O0 | 479 | 32 |
| sort O2 | 343 | 27 |
| du O2 | 320 | 32 |
| du O0 | 441 | 32 |
| find O2 | 658 | 57 |
| grep O2 | 449 | 43 |
| gzip O2 | 204 | 3 |
| tar O2 | 1,125 | 106 |
| bzip2 O2 | 114 | 4 |
| diff O2 | 398 | 36 |
| chibios O2 | 398 | 30 |
| freertos O2 | 126 | 8 |
| **total** | **5,610** | **483** |

**`goto` without a label: 0 in the off arm, 0 in the on arm** (`gotolabel.py`).
`find` O2 `sub_f620`, the function that kept the flip off last time, now emits
`label_f752:` in both arms: the on arm still duplicates the `case 0x75` return
block, and the one `goto label_f752;` left has its label.

**345 of the 483 are verified mechanically** (`canon.py`): every access through
the retyped parameter is rewritten to its byte offset in both arms (`a0[k]`,
`*a0`, `*(T *)&a0[k]`, `*((long)a0 + K)` against `a0->field_0xK`), casts are
dropped and variable numbering collapsed, and the two function bodies are then
equal line for line.

**The other 138 were read by hand**, all of them; `flip-hunks.txt` tags each one.
Every one reads and writes the same bytes as the off arm. The classes, with a
function counted once per class it shows:

| class | functions | example |
|---|---:|---|
| a load folded into its use, or a temporary holding a sum or a comparison | 50 | tar `0x35aa0`: `v1 += a0->field_0x0; if (v1 + a1 <= ...)` for `if (v1 + *a0 + a1 <= ...)` |
| an index rescaled to the structure's size, or spelled past its end | 41 | ls `0x175c0`: `&a0[2]` (8-byte elements) is `&a0[1]` (a 16-byte `struct_33`); chibios `&a0[8]` is `&a0[1].field_0x8` |
| an `undefined1` filler array decaying to its address | 40 | du `0x5e80`: `getdelim(..., &a0[3], ...)` is `getdelim(..., a0->field_0x18, ...)`, `field_0x18` being `char[16]` |
| a literal respelled for the field's type | 29 | `= 0` / `= NULL`, `0xfffffffa` / `-6`, `0x400000U` / `0x400000` |
| constant byte stores merged into one store of the field | 10 | mbuiter: eight `a0[0x18..0x1f]` byte stores are `a0->field_0x18 = 1` |
| a byte-mask test respelled on the wider field | 8 | fts: `*(uchar *)((long)a0 + 0x49) & 0x20` is `a0->field_0x48 & 0x2000` |
| a local takes the structure type or the field's pointer type | 11 | sort O0 `0x8002`: a copy of the parameter reads `v->field_0x36` |
| a pointer field indexed | 6 | grep `0xab10`: `*(ulong *)(a0[2] + 0x20)` is `a0->field_0x10[4]` (`long *field_0x10`) |
| a sub-field address spelled off a field | 3 | freertos `0x18c8`: `a1[4] = (uint)a0` is `a1->field_0x10 = &a0->field_0x0` |
| variables renumbered and merged around the accesses | 1 | tar `0x14c50` |

Every array-decay and past-the-end spelling was checked against the printed
definition (`--option structdefs on`): each bare `a0->field_0xK` used as an
address is an array member, and each `&a0[n].field_0xK` lands on the byte the
off arm addressed.

Two functions move a control-flow token (`skeleton.py`), both read in full:

- **`find` O2 `sub_f620`** — `case 0x75`'s `goto label_f752;` is replaced by the
  two statements it jumped to (the structurer's return duplication); the other
  predecessor keeps its `goto`, and `label_f752:` is emitted.
- **`chibios` O2 `sub_800397c`** — a `for` becomes a `while` with the same
  initialiser, test and step; the type lock splits the one high the `for` header
  needs.

No hunk falls outside these classes, so the sweep holds no BUG hunk.

## Project export: offsets and compilation

Eight serial `decompile-project` exports (the x86-64 seven plus `chibios`
compiled `-m32`): **311 synthesized structures, 0 with a field the compiler
moves** (`offsetcheck.py`: every `offsetof(struct_N, field_0xK) == K`). Compiling
each `.c` against its `.h` with `gcc -fsyntax-only` gives no new kind of error
and the same count of struct-related ones (no `has no member`, no incomplete
type) in both arms. Three existing kinds move by a few: `'NULL' undeclared`
(du +1, tar +7, sort O0 +1, chibios +2), because a pointer-typed field is
assigned `NULL` where the off arm assigned `0` and the export prelude does not
define `NULL`, a gap the off arm already hits 38 times in `du` alone; `sort` O0's
one call of a callee declared `void` whose result is stored, which gcc reports as
`invalid use of void expression` once the store gains a pointer cast instead of
`void value not ignored`; and `find`'s `sub_ece0` arity mismatch, once more
because `sub_f620`'s duplicated return block contains one more such call.

**A defect the flip would have made reachable, fixed here.** `decompile-project
--jobs N` merges the `.h` type block from its worker processes, and each worker
numbers its own `struct_N`. With the option on, two workers define two
different `struct_0`s, and the merge — a union deduplicated by line — tore the
definitions apart: `ls` O2 at `--jobs 4` wrote 23 `struct ... {` openings and
one `};`. Serial exports were never affected, and the defect was already there
for anyone who passed `--option structsynth param` with `--jobs`; the flip would
have put it on the default path. A sharded export now runs its workers with
`structsynth off` (`jobs.rs (structsynth_shard_note)`) and says so on stderr; its
`.c`, `.h`, `.asm` and `README.md` are byte-identical to a serial
`--option structsynth off` export (the `jobs_project_runs_workers_with_structsynth_off`
integration test on `i386_pie_nl`, and `tests/cli/sharded-project-export-structsynth-off.json`).
Sharded `decompile-all`/`decompile-graph` still synthesize; there each worker's
`struct_N` is local to the functions it decompiled, which `docs/cli.md` now says.

## decbench `type_match` (444 slices, 10,748 functions)

`scripts.decbench.typesweep --option structsynth param` against the `d3617d64`
build, O0 + O2 + O2-noinline over coreutils, grep, gzip, diffutils, bzip2,
findutils, tar and shadow:

| | off | on |
|---|---:|---:|
| perfect | 959 | 958 |
| aggregate | 3037.19 | 3035.69 (−0.049%) |

0 improved, 6 worse, 1 moved off perfect; controls clean (9,296 functions with
byte-identical variables in both arms, 0 scored differently).

`typeclasses.py` over the `--dump-decisions` rows classes all 1,641 decisions the
flip changes (1,449 functions):

| kuna now | ground truth | metric | decisions |
|---|---|---|---:|
| `struct_N *` | pointer to a named aggregate (`Hash_table` 704, `Hash_tuning` 66, `hash_entry` 44, `stat` 33, `fileinfo` 33, …) | miss → miss | 1,295 |
| `struct_N *` | primitive pointer (`void *` 97, the `void const *` typedef `V` 204, `sharefile_handle` 6, `void` 3) | miss → miss | 310 |
| `struct_N *` | pointer to a primitive pointer (`char **`, diff's line arrays) | miss → miss | 12 |
| `struct_N *` | **primitive pointer, previously matched** | **match → miss** | **19** |
| other | — | miss → miss | 5 |

**The 19 are the genuine false positives**, in the 7 moved rows: `factor`'s
`powm2` (`bp`, `ep`, `np`, `one`) and `millerrabin2` (`np`, `one`) at O0, O2 and
O2-noinline — GMP limb arrays, DWARF `__uintmax_t *`, read at offsets 0 and 8,
two pointer-sized elements the uniform-run rule keeps as a record — and
`shred` O2-noinline `passname`'s `char *name`, a seven-byte buffer written with
stores of 4, 2 and 1 bytes. The array-recognition gap is the `structarrays`
lane's to close; this change does not touch it.

The 1,295 are the accepted cost: an anonymous `struct_N *` where the source
names the aggregate, which `type_match` compares by name and so never credits.
Noelo-Lab/decbench#93 asks the metric to credit any structure pointer against a
ground-truth pointer-to-structure; under that rule these become matches, while
the 19 primitive-pointer losses stay losses.

## Census and layout precision (unchanged by the rebase)

- **False-positive census** (`fprate.py`, fmt/ls/sort/du × O0/O2): 292 retyped
  parameters — 182 DWARF struct pointers, 71 other pointers (70 `void *`, the
  one `char *` being `ls` `strmode`), 39 with no resolvable DWARF type, 0
  non-pointers. ARM32 (`chibios`, `freertos`): 37 — 29 / 1 / 3 no-type / 4 not in
  DWARF / 0.
- **Layout precision** (`layoutscore.py`): 704/787 = **0.8945** over the fields
  the pass claims (798/1021 = 0.7816 counting `undefined1` filler).

## Speed

Interleaved `off`/`param` pairs, min of 15, whole-binary `decompile-all`
(`speed.py`, `SPEED_SET=flip`; both arms name the value, one pinned release
build). The box was shared with other lanes and a workspace test run, which
the interleaving spreads over both arms.

| case | off ms | on ms | delta | median off → on |
|---|---:|---:|---:|---|
| fmt O2 | 4,494 | 4,340 | -3.42% | 4,955 → 4,777 |
| ls O2 | 14,022 | 14,432 | +2.92% | 16,611 → 16,504 |
| sort O2 | 14,790 | 15,145 | +2.40% | 17,646 → 18,059 |
| bash O2 (1.3 MB, auto = reliable) | 86,931 | 86,420 | -0.59% | 103,104 → 100,156 |
| grep O0 (inert control) | 6,868 | 6,917 | +0.72% | 8,114 → 8,065 |

**Worst delta +2.92% (`ls` O2), inside the +5% budget.** `bash` O2 is 1.3 MB,
so `auto` runs it under `reliable`, which carries the default.

## Interactions

- **`structdefs`** stays off, and flipping it is not recommended: with
  `structsynth` on, `decompile-all` of `ls` O2 under `structdefs on` is 872 lines
  longer (+6.4%), a `struct_N` definition above 70 of its 404 functions, where the
  project header declares each layout once.
- **`structdedup` (#675) and `structarrays`** are open and rebase onto this; their
  changes are default-output changes from here on.
- **`protoorder` (#669, `types` mode)** is open; once it lands, a callee's
  `struct_N` parameter propagates into its callers' argument types.
- **The Ghidra front-end** applies the defaults, so it prints `struct_N` too.
  Stock Ghidra decodes a decompiler-invented structure as an `undefined` of its
  size (`PcodeDataTypeManager.decodeDataType`), so nothing breaks, but Ghidra's own
  data-type model does not learn the layout.

## Modes

- **`reliable`** — stays empty: it is the shipped defaults, so it synthesizes.
  Pinning `off` there would make `auto`'s 500 KiB–2 MiB band disagree with the
  default the user chose, on no evidence of wrong output.
- **`aggressive`** — inherits the default and does not name the option. It is
  multi-valued, and pinning `param` would hold the preset below a future `all`.
- **`fast`** — no entry: it reshapes discovery only, and the pass's cost is in
  the noise.
