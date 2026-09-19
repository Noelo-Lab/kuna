# ptrfromuse — the default flip (`off` → `void`)

`ptrfromuse` shipped opt-in in #659 (`record.json` `default_flip`: none). This
file is the evidence the flip to `void` was measured against, on the tree that
carries `protoorder` (#669, default `types`), because a callee's recovered
parameter types now reach its call sites and the two options interact. Every
number below is from one build of this branch, both arms the same binary, the
arms selected with `--option ptrfromuse off|byte|void` so that nothing depends
on which value is compiled in as the default.

Base: `origin/main` `d96e3408` with `protoorder` #669 (head `57437388`) applied, plus
this branch. The typesweep, the corpus sweep and the speed run name each arm
explicitly; the release build with the default flipped was then checked against them:
its default output is byte-identical to the `void` arm, and its `--option ptrfromuse off`
output to the `off` arm, on all 4,490 functions of the corpus sweep.

## Which value

The 444-slice `typesweep` (coreutils grep gzip diffutils bzip2 findutils tar
shadow x, O0/O2/O2-noinline, 10,748 functions, `DECBENCH_NO_CACHE=1`):

| arm | PERFECT | aggregate | onto perfect | off perfect | improved | worse |
|---|---|---|---|---|---|---|
| `byte` | 1,105 → 1,112 (+7) | 3,363.95 → 3,374.42 (+10.46) | 7 | 0 | 21 | **0** |
| `void` | 1,105 → 1,309 (+204) | 3,363.95 → 3,596.67 (+232.71) | 204 | 0 | 85 | **0** |

Controls: functions whose two arms hand the metric identical variables score
identically (8,158 `byte`, 8,161 `void`, 0 discrepancies).

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
the same functions `char *`, which matches none of those declarations; its 7
are `format_inode`, `sysinttostr`, `path_prefix` and `decimal_absval_add_one`.

## (a) `make test`

675/675, PARITY OK, with the default flipped — no datatest assertion moves, so no
per-test opt-out was needed.

## (b) `make test-stages`

1177/1177, PARITY OK, after the edits below; 1166/1177 before them. The 11 assertions the flip moves, each read in both arms. None is
a stage test of `ptrfromuse` (`kuna-ptrfromuse.xml` already names every value in
every pass); each pins a parameter spelling incidental to its own subject, and
each follows the new spelling:

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

Every one is the declaration and the base cast, nothing else. No assertion name
changed, so `docs/baseline-stages.json` is unchanged.

## (c) `make test-cli`

204/204. Three `protoorder` probes on the MIPS o32 fixture pinned `g3(int a0,int a1)`
and `g20(int a0,int a1,unsigned short *a2)`: `a1` is only ever dereferenced there, so the
default now declares it `void *`, and the probes (ids recomputed) and the round-trip test
that compiles those callers follow it. Two probes are new:
`ptrfromuse-default-declares-a-dereferenced-parameter-void` and
`protoorder-types-keeps-a-wide-store-whole`.

## (d) Accuracy

The `typesweep` above: `void` 0 worse. The arguments and variables the metric
cannot see were counted over the whole-corpus sweep below (11 binaries, 4,490
functions), `off` against `void`: call-site arguments 36,942 → 36,942, phantom
arguments 133 → 133, `variables[]` argument rows 8,255 → 8,255, `variables[]`
rows 28,305 → 28,305. The flip adds and removes no argument and no variable.

The same sweep over the four firmware slices `protoorder` was measured on (nuttx,
mirai, betaflight, freertos at O2, 3,451 functions): perfect 217 → 217,
aggregate 539.88 → 547.59 (+7.71), 30 improved, 7 worse. All seven worse are
betaflight bus callbacks declared `busStatus_e mpuIntCallback(uint32_t arg)`
whose first statement is `gyroDev_t *gyro = (gyroDev_t *)arg;` (likewise
`bmi160Intcallback`, `bmi270Intcallback` and the `m25p16_`/`w25n01g_` flash
callbacks): the body only ever dereferences the parameter, so it is declared
`void *` where the program declares an integer it casts to a pointer. That is the
ambiguity the option's `use_when` names.

## (e) Speed

`speed.py` (this directory): whole-binary `decompile-all`, `off` and `void` alternating inside each
round with the order swapped every round, minimum and median of 15, on a shared 80-core box.

| binary | off min (ms) | void min (ms) | Δ min | median off → void |
|---|---:|---:|---:|---:|
| fmt O2 | 4,090.8 | 4,084.7 | -0.15% | 4,109.6 → 4,310.8 |
| ls O2 | 13,140.9 | 13,144.8 | +0.03% | 13,307.4 → 13,442.5 |
| sort O2 | 13,910.2 | 13,897.3 | -0.09% | 14,233.6 → 14,188.6 |
| bash O2 (1.3 MB, auto = reliable) | 78,484.6 | 78,421.6 | -0.08% | 79,151.0 → 79,133.7 |

Worst delta on the minimum: +0.03%, budget +5%. `bash` is 1.3 MB, so `auto` runs it under `reliable`;
the default applies there too. Raw numbers: `flip-speed.json`.

The whole branch against main (the `protoorder` tree without this branch), each build at its
defaults, the same way (`speed-main-vs-branch.py`): this adds the `protoorder` refusal's walk of
the accesses through a primitive-pointee vote.

| binary | main min (ms) | branch min (ms) | Δ min | median main → branch |
|---|---:|---:|---:|---:|
| fmt O2 | 4,081.9 | 4,091.2 | +0.23% | 4,338.7 → 4,346.9 |
| ls O2 | 13,100.4 | 13,154.1 | +0.41% | 13,173.3 → 13,256.0 |
| sort O2 | 13,859.1 | 13,974.3 | +0.83% | 13,939.0 → 14,191.0 |
| bash O2 | 77,810.2 | 79,149.0 | +1.72% | 78,417.0 → 79,824.7 |

Worst +1.72% (`bash`), budget +5%. Raw numbers: `flip-speed-main-vs-branch.json`.

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
| member reach (the same bytes through a retyped pointer) | 43 |
| literal (a constant argument gains its pointer cast) | 2 |
| pointer arithmetic | 2 |
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
reordered across a store or a call anywhere in the sweep, and no value changes.
`flip-ndiff.py` prints one function's normalised diff and `flip-stmtdelta.py`
the statement, call and arity counts per changed function.

The shape that kept the opt-in PR from recommending a flip, a parameter
subscripting a named global (`dat_24580[a2]`), occurs at four sites in O0 and
O2 `dash` and O0 `tar`; with the default flipped all four print exactly as they
do with the option off.

### The store split, and the `protoorder` refusal that closes it

The first sweep on the `protoorder` tree found one class outside the option's
effect, in both non-off values: bzip2 `sub_5a10` and `sub_60e0` and tar
`sub_133d0` printed wide constant stores as byte stores —

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
store per byte. `protoorder`'s own record lists the same thing on main
(`analysis.md` §5 `store-width`, §9): gzip's `".tar"` as five byte stores,
betaflight's `1.0f` as four.

`protoorder` already refuses a vote whose pointee disagrees with how the caller
uses the pointer, but only for float and composite pointees. This branch adds
the primitive case (`kuna_protoorder.rs` `pointee_refuses`): a vote whose
pointee is a non-character integer, `bool` or pointer is refused when the caller
stores a constant wider than it through the pointer at a fixed place or at a
record stride. Two narrower first attempts were measured and dropped: refusing
on any wider access cost gzip-style `builtin_strncpy` and a betaflight
parameter's struct pointer, and refusing on any wider constant store also
refused betaflight `read_data_sector`'s correct `uint8_t *`, which fills the
sector with `0xdeadbeef` a word at a time — a stride of the store's own width,
now allowed. A character pointee is not checked, because its byte stores are
what the string-copy idiom prints as `builtin_strncpy`.

With the refusal the split is gone from the sweep in both values, and the
`void` arm's classification above is the tree with it. Its own effect, `ptrfromuse
off` with and without it: on the 11 binaries 1 of 4,490 functions changes (gzip
`sub_6e90` gets `builtin_strncpy(v15,".tar",5)` back); on O2-noinline gzip
`sub_69d0` does the same; on betaflight (5,797 functions) 4 change — `sub_8041e24`
prints `a2[v1 * 5 + 9] = 0x3f800000;` for four byte stores, `sub_80251ec` one
word store for four, and two callers' constant arguments carry the new cast.
Neither type sweep moves: 0 of the 10,748 functions of the 444 slices and 0 of
the 3,451 firmware functions score differently with the refusal, in either arm.
`protoorder_narrowvote_x86_64` is the fixture, and
`a_byte_pointee_vote_keeps_the_callers_wide_stores` fails without the refusal.

## (g) Presets

`ptrfromuse` is multi-valued, so `aggressive_carries_every_default_off_option`
does not consider it, and it is in neither `AGGRESSIVE_OVERRIDES` nor any other
preset. The default applies under every mode.
