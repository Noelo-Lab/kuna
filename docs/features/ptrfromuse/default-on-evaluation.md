# ptrfromuse — the default flip (`off` → `void`)

`ptrfromuse` shipped opt-in in #659 (`record.json` `default_flip`: none). This
file is the evidence the flip to `void` was measured against, on the tree that
carries `protoorder` (#669, default `types`), because a callee's recovered
parameter types now reach its call sites and the two options interact. Every
number below is from one build of this branch, both arms the same binary, the
arms selected with `--option ptrfromuse off|byte|void` so that nothing depends
on which value is compiled in as the default.

Base: `origin/main` `3433e05d` (`protoorder` #669 merged). The branch is the
default value and the spellings that follow it, nothing else. The typesweep, the
corpus sweep and the speed run name each arm explicitly; the release build with
the default flipped was then checked against them: its default output is the
`void` arm and its `--option ptrfromuse off` output the `off` arm.

## Which value

The 444-slice `typesweep` (coreutils grep gzip diffutils bzip2 findutils tar
shadow x, O0/O2/O2-noinline, 10,748 functions, `DECBENCH_NO_CACHE=1`):

| arm | PERFECT | aggregate | onto perfect | off perfect | improved | worse |
|---|---|---|---|---|---|---|
| `byte` | 1,147 → 1,154 (+7) | 3,413.35 → 3,423.82 (+10.46) | 7 | 0 | 21 | **0** |
| `void` | 1,147 → 1,351 (+204) | 3,413.35 → 3,646.07 (+232.71) | 204 | 0 | 85 | **0** |

Controls: functions whose two arms hand the metric identical variables score
identically (8,294 `void`, 0 discrepancies).

`void` wins on both criteria. What it wins is concentrated, and worth stating
plainly: all 204 functions it moves onto perfect are `ls`'s sort comparators
(`xstrcoll_df_ctime`, `rev_strcmp_df_width`, …), counted in the three programs
built from that source (`ls`, `dir`, `vdir`) at three optimisation levels. Their
declared parameters are `V a, V b` with `typedef void const *V`, and they only
forward those arguments to a comparator that dereferences them. `ptrfromuse`
types the dereferencing callee's parameters `void *`, `protoorder` carries that
type to the argument at the call, and the forwarding wrapper is declared with
`void *` parameters. The 85 improved-but-not-perfect functions are the same
shape more widely: gnulib hash-table and comparator callbacks (`proctab_hasher`,
`wd_comparator`, `name_hash`, `compare_nodes`, `hash_directory_meta`) and a few
directory walkers in `tar`, `mkdir`, `install`, `tail` and `sort`. `byte` gives
the same functions `char *`, which matches none of those declarations.

Per project, `off` → `void`:

| project | n | perfect off → void | aggregate off → void | improved | worse |
|---|---:|---|---|---:|---:|
| bzip2 | 267 | 31 → 31 | 108.87 → 108.87 | 0 | 0 |
| coreutils | 6,422 | 675 → 879 | 1,912.24 → 2,137.05 | 268 | 0 |
| diffutils | 420 | 58 → 58 | 180.06 → 180.06 | 0 | 0 |
| findutils | 790 | 41 → 41 | 173.72 → 173.72 | 0 | 0 |
| grep | 247 | 51 → 51 | 106.91 → 106.91 | 0 | 0 |
| gzip | 368 | 102 → 102 | 191.43 → 191.43 | 0 | 0 |
| shadow | 686 | 31 → 31 | 205.01 → 205.01 | 0 | 0 |
| tar | 1,548 | 158 → 158 | 535.12 → 543.02 | 21 | 0 |
| **pooled** | **10,748** | **1,147 → 1,351** | **3,413.35 → 3,646.07** | **289** | **0** |

## (a) `make test`

675/675, PARITY OK, with the default flipped — no datatest assertion moves, so no
per-test opt-out was needed.

## (b) `make test-stages`

1250/1250, PARITY OK, after the edits below. The 13 assertions the flip moves,
each read in both arms. None is a stage test of `ptrfromuse`
(`kuna-ptrfromuse.xml` already names every value in every pass); each pins a
parameter spelling incidental to its own subject, and each follows the new
spelling:

| assertion | `off` | `void` (default) |
|---|---|---|
| `cspecprotos #4` | `getfield(int4 a0)` | `getfield(void *a0)` |
| `cspecprotos #5` | `*(unsigned int *)(a0 + 0xc)` | `*(unsigned int *)((int4)a0 + 0xc)` |
| `iteboolean #3` | `(*(uint4 *)(a0 + 4) & 0x100)` | `(*(uint4 *)((int8)a0 + 4) & 0x100)` |
| `EXCLUSIVEARGUSE #5` | `**(char **)(a0 + 8) = *a1;` | `**(char **)((int8)a0 + 8) = *a1;` |
| `inputparamgap #1/#3/#5` | `on_key(int8 a0…)`, `on_two(int8 a0,…)` | `on_key(void *a0…)`, `on_two(void *a0,…)` |
| `inputparamgap #4` | `*(int4 *)(a0 + 0x70) = a4 + 8;` | `*(int4 *)((int8)a0 + 0x70) = a4 + 8;` |
| `MSVCSTRAPPEND #3` | `append((void *)(a0 + 8)," \"",2)` | `append((void *)((int8)a0 + 8)," \"",2)` |
| `truncarg #8/#9` | `*(char *)(a0 + 3)`, `*(uint1 *)(a0 + 1) & …` | `*(char *)((int8)a0 + 3)`, `*(uint1 *)((int8)a0 + 1) & …` |
| `ALIASOVERLAP #1/#3` | `*(unsigned int *)(a0 + 7)` | `*(unsigned int *)((int8)a0 + 7)` |

Every one is the declaration and the base cast, nothing else. `ALIASOVERLAP`'s
subject is whether the 4-byte read prints ahead of the store that overwrites its
bytes; both arms print it ahead, and `#5`–`#8`, whose parameter is declared, do
not move. No assertion name changed, so `docs/baseline-stages.json` is unchanged.

## (c) `make test-cli`

207/207. Three `protoorder` probes on the MIPS o32 fixture pinned `g3(int a0,int a1)`
and `g20(int a0,int a1,unsigned short *a2)`: `a1` is only ever dereferenced there, so the
default now declares it `void *`, and the probes (ids recomputed) and the round-trip test
that compiles those callers follow it. `protoorder-types-keeps-a-wide-store-whole` keeps
its subject — the eight-byte store stays whole — and follows the base cast
(`*(unsigned long *)((long)a0 + 0x10)`). One probe is new,
`ptrfromuse-default-declares-a-dereferenced-parameter-void`. The two `protoorder`
byte-pointee tests in `decompile_all_cli.rs` now run the default against
`--option ptrfromuse off` instead of against `void`.

## (d) Accuracy

The `typesweep` above: `void` 0 worse. The arguments and variables the metric
cannot see were counted over the whole-corpus sweep below (11 binaries, 4,490
functions), `off` against the default: call-site arguments 36,959 → 36,959,
phantom arguments 134 → 134, `variables[]` argument rows 8,251 → 8,251,
`variables[]` rows 28,300 → 28,300. The flip adds and removes no argument and no
variable, and no function loses or gains a call (`calls_lost` and `calls_gained`
0, `goto` and `return` counts unchanged).

The same sweep over four firmware slices (nuttx, mirai, betaflight, freertos at
O2, 3,451 functions): perfect 216 → 216, aggregate 538.88 → 546.59 (+7.71), 30
improved, 7 worse. All seven worse are betaflight bus callbacks, and all seven
have the same source shape — `busStatus_e mpuIntCallback(uint32_t arg)` whose
first statement is `gyroDev_t *gyro = (gyroDev_t *)arg;`, likewise
`bmi160Intcallback`, `bmi270Intcallback` and the `m25p16_`/`w25n01g_` flash
callbacks. The body only ever dereferences the parameter, so it is declared
`void *` where the program declares an integer it casts to a pointer. That is
the ambiguity the option's `use_when` names. The 30 improved are nuttx's procfs
entry points and betaflight's USB and receiver callbacks, whose parameters
really are pointers.

## (e) Speed

`speed.py` (this directory): whole-binary `decompile-all`, `off` and `void` alternating inside each
round with the order swapped every round, minimum and median of 15, on a shared 80-core box with
other lanes running.

| binary | off min (ms) | void min (ms) | Δ min-of-30 | Δ run 1 | Δ run 2 |
|---|---:|---:|---:|---:|---:|
| fmt O2 | 4,304.3 | 4,181.6 | -2.85% | -2.96% | -2.85% |
| ls O2 | 13,766.6 | 13,885.0 | +0.86% | +4.14% | +0.86% |
| sort O2 | 14,801.8 | 14,906.0 | +0.70% | -0.15% | +0.70% |
| bash O2 (1.3 MB, auto = reliable) | 83,347.9 | 85,092.7 | +2.09% | +3.63% | +2.09% |

Two independent runs of 15 rounds each, because the box was carrying other lanes
throughout: the table's minimum is over all 30 pairs, and each run's own worst
delta is beside it (`flip-speed.json`, `flip-speed-run2.json`). Run 1's `ls`
+4.14% does not reproduce; its medians (16,964.7 -> 17,075.1 ms) say +0.65%.

Worst delta on the minimum: +2.09%, budget +5%. `bash` is 1.3 MB, so `auto` runs it under
`reliable`; the default applies there too. Raw numbers: `flip-speed.json`. The branch differs from
main only in which value the option defaults to, so `off` is main's engine exactly and the two arms
of this run are main's default against the branch's.

## (f) Whole-corpus `decompile-all`

11 stripped binaries, 4,490 functions: coreutils `fmt`/`ls` O2, `sort` O0,
grep O2 and O0, gzip, bzip2, tar and diffutils `diff` O2, and the ARM32
firmwares `chibios` and `freertos` O2. Every changed function is classified by
the normalisers of `docs/features/protoorder/corpus-diff.py` (the first under
which the two bodies agree), then every function that no normaliser explains,
and every function whose statement count moves, is read by hand
(`flip-hunks.txt`).

| class | functions |
|---|---:|
| casts only (`(long)a0 + k`, `(void *)0x…` on a constant argument) | 716 |
| declarations only | 56 |
| member reach (the same bytes through a retyped pointer) | 41 |
| pointer arithmetic | 4 |
| literal (a constant argument gains its pointer cast) | 2 |
| read by hand | 17 |
| **changed** | **836** |

The 17: ten are only terms of a sum reordered around the new cast
(`a0 + 0x80 + (long)v1` → `(long)a0 + (long)v1 + 0x80`); three are the same
address through a retyped `long *` (`*(long *)(a0 + 0x1a0)` → `a0[0x34]`); one
gains a string (`sub_108c(0x22d5,0xa38,…)` → `sub_108c(0x22d5,"Tmr Svc",…)` in
freertos); one keeps a loaded value in its local and adds `v15` at each of its
two readers instead of keeping the sum (tar `sub_3c060`, the load still before
the store it feeds). Two fold an address temporary into its uses: diff
`sub_9e40` drops `a0 += v2;` and adds `v2` to each field address in the loop,
grep `sub_af70` drops `v1 = a0 + 0x70;` and spells `(long)a0 + … + 0x70` at each
read. Those two are the only functions in the sweep whose statement count
moves, and in both the folded value is an address, not a load. No load is
reordered across a store or a call anywhere in the sweep, no call is lost or
gained, no `goto` or `return` count moves, and no value changes.
`flip-ndiff.py` prints one function's normalised diff and `flip-stmtdelta.py`
the statement, call and arity counts per changed function.

The shape that kept the opt-in PR from recommending a flip, a parameter
subscripting a named global (`dat_24580[a2]`), occurs at four sites in O0 and
O2 `dash` and O0 `tar`; with the default flipped all four print exactly as they
do with the option off.

### The store split, closed in `protoorder` before this flip

The first sweep of this evaluation, on a tree where `protoorder` did not yet
refuse narrow primitive pointees, found one class outside the option's effect:
bzip2 `sub_5a10` and `sub_60e0` and tar `sub_133d0` printed wide constant stores
as byte stores —

```
-  *(unsigned long *)((long)a0 + 0x5c) = 0x100;
+  a0[0x5c] = 0;
+  a0[0x5d] = 1;
+  …                      (eight byte stores)
```

The byte pointer was not this option's. `sub_5a10` hands its parameter to two
callees; `sub_8d30`'s recovered parameter is `unsigned char *` (it reads bytes
through it) and `sub_5890`'s was `long`. With `ptrfromuse` off, `protoorder`
refuses both votes because they disagree in class. With it on, `sub_5890`'s
parameter is a pointer too, the two votes agree in class, the more specific
`unsigned char *` wins, and `SplitDatatype` reads a pointer to a one-byte
integer as an array of bytes, so a constant stored through it is split into one
store per byte.

That refusal shipped in #669 (`kuna_protoorder.rs` `pointee_refuses`: a vote
whose pointee is a non-character integer, `bool` or pointer is refused when the
caller stores a constant wider than it through the pointer), so it is on main
and not in this branch. The sweep above is the tree with it, and the split is
absent from both arms. `protoorder_narrowvote_x86_64` and
`protoorder_widefill_x86_64` are its fixtures; this branch only switches their
second arm from `--option ptrfromuse void` to `off`, since `void` is now the
default.

## (g) Presets

`ptrfromuse` is multi-valued, so `aggressive_carries_every_default_off_option`
does not consider it, and it is in neither `AGGRESSIVE_OVERRIDES` nor any other
preset. The default applies under every mode.
