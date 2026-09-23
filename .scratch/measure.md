# Cast census — MEASURE lane

Read-only lane. No engine change, no PR. Everything below is measured on **current
main** (`796939d54`, built in `/home/mahaloz/kwt/castcensus`, `BUILD_RC=0`) against
the frozen rival outputs in
`/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/`.

Artifacts (all under `/home/mahaloz/kwt/castcensus/.scratch/`):

| file | what |
|---|---|
| `castcount.py` | the cast counter (grammar rules in its header) |
| `idioms.py` | composite-idiom classifier |
| `divis.py` | offset-divisibility check for the laundering idiom |
| `shapes2.py` | sign-merged shape histogram, kuna vs ida vs base |
| `run_census.py` / `run_pfoff.py` | regenerate the corpus with a given kuna build/option |
| `tables.py`, `mkshared.py` | whole-binary and address-intersection tables |
| `ablate.py` → `ablate.json`/`ablate.log` | per-option cast deltas |
| `tables.json` | every number below, machine-readable |
| `hand/*.audit` | the hand-check windows |
| `out/{new,base,pfoff}/`, `shared/{kuna,kbase,ida,ghidra,angr}/` | regenerated C |

---

## THE HEADLINE

On **4,815 functions present at the same address in both kuna's and IDA's output**
(45 binaries × {O0, O2, O2-noinline}: coreutils fmt/ls/sort/du/cp/tail/wc, grep,
gzip, diffutils cmp/diff/diff3/sdiff, tar, findutils find):

```
              casts    lines    stmts   /kloc  /100stmt   vs ida: abs  per-stmt  per-kloc
ida          37,821  243,379  140,143   155.4    27.0        1.00      1.00      1.00
ghidra       44,661  225,094  133,900   198.4    33.4        1.18      1.24      1.28
angr         25,760  253,688  154,860   101.5    16.6        0.68      0.62      0.65
kuna (base)  40,529  191,792  121,386   211.3    33.4        1.07      1.24      1.36
kuna (main)  44,821  190,590  120,204   235.2    37.3        1.19      1.38      1.51
```

kuna's excess over IDA on identical functions is **+7,000 casts (+18.5%)**.

**One idiom is the entire gap.** 36.2% of kuna's casts (16,213 of 44,821) are the
pointer→integer→pointer laundering pattern

```c
*(unsigned int *)((long)v38 + 0xc4) = 0xffffffff;      /* kuna: TWO casts */
```

IDA writes the identical access at the identical address (ls O0 `gobble_file`
@0x9a40) as

```c
*((int *)s + 49) = -1;                                  /* IDA: ONE cast */
```

IDA's share of that idiom is **1.1%** (432 casts). The excess is ~15,800 casts,
against a *total* kuna-over-IDA excess of 7,000. Deleting only the inner `(long)`
casts — 8,014 of them — would put kuna at **36,807 casts, below IDA's 37,821.**

This is not cast hiding. `(T *)P + K/sizeof(T)` is the same C expression as
`*(T *)((long)P + K)` whenever `sizeof(T)` divides `K`, and **3,422 of 3,429
constant-offset sites divide evenly (99.8%)** — measured, `divis.py`. The cast
disappears because the pointer type is *used* instead of being thrown away and
rebuilt.

---

## 1. The counter, and why it is defensible

`castcount.py` tokenizes and applies the C cast grammar. A `(...)` span is a cast
only when all three hold: **(1)** the content parses as a type-name with an
*abstract* declarator (no identifier — this is what rejects `(long long a1)`);
**(2)** the preceding token admits a cast (not an identifier — that is a call —
not a constant, `]`, `++`, `sizeof`/`typeof`/`va_arg`, and not a type keyword,
which would make it a declarator); **(3)** the following token starts a
unary-expression (a `{` means a compound literal, not a cast). Chained casts each
count: `(long)(int)x` is 2, because a `)` that closed a cast is itself a legal
cast position.

The typedef/identifier ambiguity is resolved by **harvesting the vocabulary per
file** from that file's own `typedef`s, struct/union/enum tags, and declaration
positions, plus a fixed seed of the four tools' spellings. So `(FILE *)p` is a
cast and `(ptr)`, `(v5)`, `(local_3d0)` are not. Types are then normalized
(`_QWORD` ≡ `unsigned long` ≡ `undefined8` ≡ `uint8` → `u64`) so shapes compare
across tools.

**Self-test**: a hand-written file with 16 planted casts and 5 planted traps
(a parenthesized variable, a `sizeof` operand, a `void (*fp)(int)` declarator, a
chained cast, a function-pointer cast) — 16/16 found, 0 false positives.

**Hand check, 100 lines from each tool** (the densest 100-line window of
`{ida,kuna,ghidra,angr}_ls.c` O0, audited with whole-file vocabulary via
`--auditrange`; `hand/*.audit`):

| tool | `(...)` spans | judged CAST | false positives | false negatives |
|---|---|---|---|---|
| ida | 134 | 80 | 0 | 0 |
| kuna | 231 | 78 | 0 | 0 |
| ghidra | 179 | 52 | 0 | 0 |
| angr | 152 | 49 | 0 | 0 |
| **total** | **696** | **259** | **0 (0.0%)** | **0 (0.0%)** |

This is a full classification of all 696 spans, not a sample of them. Every one of
the 437 rejections carries `not-a-type-name`, i.e. **no span was rejected by the
predecessor/successor rules while being a valid type-name** — the two heuristic
rules cost nothing here. The only type-*looking* rejections are parenthesized
variables (`(a1)`, `(v16)`, `(dat_2b3d5)`, `(local_3d0)`, `(ptr)`), all correct.

**Known limits** (stated, not hidden): a variable whose name is also used as a
type name in the same file would be miscounted (none observed); `code` and
`undefined` are treated as types (correct for Ghidra/kuna); an array type in a
cast is normalized to a pointer.

---

## 2. Current main vs the rivals, wide sample

### 2a. Which normalizer to trust

The per-1000-lines figure the campaign has been quoting **overstates kuna's gap**,
because kuna's C is more compact for the same code — on identical functions IDA
emits 243,379 lines and 140,143 statements where kuna emits 190,590 and 120,204
(IDA spills each call into a temporary and breaks long argument lists over lines).

```
kuna(main) vs ida on identical functions:  absolute 1.19x | per-statement 1.38x | per-kloc 1.51x
```

Report the absolute and per-statement numbers. **+18.5% casts on identical
functions** is the honest headline; "x1.51 per kloc" is partly a line-density
artifact.

One counting detail, because it changes kuna's number and nobody else's: the
typedef vocabulary must be harvested from the **whole file**, not per function.
kuna declares its synthesized `struct_N` typedefs once at file top and casts to
them throughout, so per-function harvesting undercounts kuna by 1.4% (44,821 →
44,210) and everyone else by ≤0.7%. All numbers here use whole-file harvesting;
`tables.json` holds the per-function variant, which is why its totals are slightly
lower.

### 2b. By optimization level (shared set, absolute casts)

```
opt              ida  ghidra    angr  k_base  k_main   k_main/ida (abs / per-stmt / per-kloc)
O0             12193   14132    7908   13208   14727    1.21 / 1.30 / 1.46
O2             13443   16533    9544   15161   15947    1.19 / 1.45 / 1.58
O2-noinline    12149   13903    8134   12145   13536    1.11 / 1.32 / 1.44
```

kuna is worst at O0/O2 and closest at O2-noinline. kuna is consistently **better
than Ghidra** (1.18x ida) and consistently **worse than angr** (0.68x ida — angr
casts least of the four, largely because it commits to fewer types at all).

### 2c. Whole-binary, per binary

Full 45-row table in `tables.json` (`shared` for the intersection, `whole` for
each tool's own function set). Per-binary shared-set casts/kloc, kuna_main vs ida,
worst ratios: cmp O0 3.54x, gzip O0 2.81x, cmp O2-noinline 2.81x, wc O0 2.76x,
cp O2 2.03x. Best: du O0 0.74x, sdiff O0 0.81x, du O2-noinline 0.89x, fmt O0 0.95x.
kuna already casts *less* than IDA on 4 of 45 binaries.

Note the whole-binary table is **not** apples-to-apples — `kuna decompile-all`
emits every function (527k lines) while the frozen rival files hold only the
benchmarkable set (243k). Only the intersection numbers above are comparable.

### 2d. Distribution: head *and* tail, and kuna wins a third of functions

```
net excess                        +6,425 casts (+17.0%)
gross excess                     +14,776 over 1,821 functions
gross deficit (kuna casts LESS)   -8,351 over 1,739 functions
functions: kuna>ida 38%   equal 26%   kuna<ida 36%
median per-function casts/kloc:  kuna 143   ida 125   (+14%)
top  1% of functions hold 30.2% of the gross excess
top  5%                  61.9%
top 10%                  76.9%
top 25%                  95.0%
```

So it is **not** a few huge functions — but it is not uniform either. A quarter of
functions carry 95% of the excess, and the median function is only +14% worse than
IDA (vs +49% pooled per-kloc). Worst offenders: cp O2 @0x76f0 (kuna 1062 / ida 509),
cp O0 @0xabda (581/150), cp O2-noinline @0x9470 (520/323), grep O2 @0xcec0 (589/422).

---

## 3. Regression check: the campaign made casting WORSE, and one option did it

The campaign baseline binary `/home/mahaloz/kwt/_baseline/kuna` produces
**code-identical output to the frozen 2026-09-11 kuna files** — verified body by
body on ls O0: 216/216 differ only in the function *name* (`file_or_link_mode` vs
`sub_53e7`), with identical line, statement and cast counts. So base is a clean
pre-campaign reference.

```
same 4,815 functions:
  kuna_base                  40,529 casts   211.3/kloc   1.07x ida
  kuna_main                  44,821 casts   235.2/kloc   1.19x ida   +4,292 (+10.6%)
  kuna_main, ptrfromuse off  41,176 casts   216.1/kloc   1.09x ida   -3,645 (-8.1%) vs main
                                                                     +647 (+1.6%) vs base
```

**`ptrfromuse` explains 85% of the regression**, and with it off main is within
1.6% of the pre-campaign baseline. Making an integer into a `void *`
gives kuna a pointer it then has to launder back through `long` at every
byte-offset access — the type got better and the *emitter* paid for it twice.

Per-option ablation, same build, same 12 binaries (fmt/ls/sort/du at O0+O2,
grep+gzip at O0+O2), whole-binary, so the function set is constant (`ablate.log`):

```
flip                 casts     /kloc    delta vs default
structsynth=off      26,627    237.5    +653  (+2.5%)   <- struct synthesis REMOVES casts
calleevote=off       26,317    234.8    +343  (+1.3%)   <- calleevote REMOVES casts
charbyte=off         26,011    232.1     +37  (+0.1%)
charptr=on           25,979    231.8      +5  (+0.0%)
DEFAULT              25,974    231.8       0
slotptr=off          25,974    231.8       0            <- no effect on casts
framelayout=off      25,974    231.8       0            <- no effect on casts
typedepth=off        25,974    231.8       0
passthrough=off      25,971    231.7      -3
boolbyte=off         25,967    231.7      -7
libctypes=off        25,936    231.4     -38  (-0.1%)
structsynth=all      25,722    229.4    -252  (-1.0%)   <- MORE struct synthesis, FEWER casts
protoorder=off       25,657    229.0    -317  (-1.2%)
libctypes=glibc      25,582    228.2    -392  (-1.5%)
ptrfromuse=off       23,665    211.1  -2,309  (-8.9%)
ptrfromuse=byte      22,943    204.7  -3,031 (-11.7%)   <- the one big lever
```

(`libcwiden` is not a catalog option name; those 12 runs errored and are excluded.)

Two directions worth separating:

- **`structsynth` and `calleevote` reduce casts** — knowing the record/parameter
  type removes casts, which is exactly the user's premise. `structsynth=all` is a
  further -1.0%.
- **`ptrfromuse void` increases them by ~9-12%** — not because the type is wrong,
  but because a `void *` base plus a byte offset has no lowering except `(long)`.
  Fixing the *emitter* keeps the better type and drops the cast.

---

## 4. Top 30 cast shapes kuna emits (sign-merged), vs IDA on the same functions

Signedness never changes whether a cast exists, so `(long)`/`(unsigned long)` are
merged into `64`, etc. `REC` = a synthesized/record type. Counts are over the
4,815 shared functions; `k-ida` is kuna minus IDA for the *same* shape.

```
  # shape          kuna     %   base    ida   k-ida  example / reading
  1 64<var>       11339 25.3%   8034   2761  +8578  (long)v38   -- THE GAP: pointer laundered to int
  2 64*<paren>     4605 10.3%   5744   5891  -1286  (long*)(...)  IDA does this MORE
  3 8*<paren>      3416  7.6%   3476   2241  +1175  (char*)(...)
  4 8*<call>       2048  4.6%    972   1187   +861  (char*)gettext(...)  -- unknown libc return type
  5 32<var>        1933  4.3%   2183   3407  -1474  (unsigned int)v  IDA does this MORE
  6 64*<addr>      1690  3.8%   2003    551  +1139  (unsigned long*)&v  -- stack slot re-read at a width
  7 64<paren>      1659  3.7%   1831    353  +1306  (unsigned long)(...)
  8 32*<paren>     1648  3.7%   1931   1640     +8  (unsigned int*)(...)  -- level with IDA
  9 8<var>         1066  2.4%    865   1300   -234  (bool)v
 10 32<paren>      1042  2.3%   1083    909   +133  (int)(...)
 11 8**<paren>      742  1.7%    789    597   +145  (char**)(...)
 12 64*<index>      559  1.2%    736    414   +145  (long*)x[i]
 13 8*<var>         520  1.2%    371   2105  -1585  (char*)v   IDA does this FAR more
 14 8*<deref>       513  1.1%    464    179   +334  (char*)*p
 15 16*<paren>      507  1.1%    530    231   +276  (unsigned short*)(...)
 16 8**<addr>       501  1.1%    110    119   +382  (bool**)&v   -- 4.6x the base; new this campaign
 17 32*<addr>       497  1.1%    618    165   +332  (unsigned int*)&v
 18 64<deref>       472  1.1%    522     38   +434  (unsigned long)*p
 19 64*<const>      459  1.0%    158      0   +459  (unsigned long*)0x1234 -- IDA NEVER does this
 20 64*<call>       436  1.0%    513    214   +222  (unsigned long*)f(...)
 21 void**<paren>   402  0.9%    367    229   +173  (void**)(...)
 22 void*<paren>    397  0.9%    380    152   +245  (void*)(...)
 23 64*<deref>      376  0.8%    451    206   +170  (unsigned long*)*p
 24 32<deref>       362  0.8%    383    180   +182  (int)*p
 25 8*<addr>        340  0.8%    414    487   -147  (char*)&v
 26 8*<const>       330  0.7%    185      1   +329  (char*)0x1234  -- IDA NEVER does this
 27 void*<const>    329  0.7%    229      0   +329  (void*)0x1234  -- IDA NEVER does this
 28 64<call>        312  0.7%    400     63   +249  (long)f(...)
 29 void*<call>     308  0.7%    309    223    +85  (void*)f(...)
 30 8*<index>       286  0.6%    180    213    +73  (char*)x[i]
```

Shapes where **IDA casts more than kuna** (kuna is already ahead here):
`64*<var>` ida 2111 / kuna 122 (-1989); `8*<var>` 2105/520 (-1585);
`32<var>` 3407/1933 (-1474); `64*<paren>` 5891/4605 (-1286);
`8<call>` 1032/140 (-892); `32*<var>` 840/143 (-697); `f32<paren>` 456/21 (-435).

### The five ranked levers this table implies

1. **`(long)<ptr>` byte-offset laundering — 8,578 excess casts, 36.2% of all kuna
   casts when both halves are counted.** Lower `*(T *)((long)P + K)` to
   `*((T *)P + K/sizeof(T))` when `sizeof(T) | K` (99.8% of 3,429 constant-offset
   sites), and to `P->field` when the record is known (removes **both** casts).
   Idiom share by tool on the shared set: kuna 36.2%, ghidra 16.7%, angr 1.6%,
   **ida 1.1%**. Per opt level kuna's inner `(long)` count is O0 3,081 / O2 2,536 /
   O2-noinline 2,397 against IDA's 46 / 91 / 79.
2. **Casts on a constant address — 1,118 casts IDA emits essentially never**
   (`64*<const>` 459 vs 0, `8*<const>` 330 vs 1, `void*<const>` 329 vs 0). kuna
   prints `(unsigned long *)0x1234` where IDA prints the named global `qword_1234`.
   This is a *symbol/global-typing* gap, not a cast-printing gap, and it grew from
   572 to 1,118 during the campaign.
3. **`(char *)<call>` — 2,048 vs IDA 1,187, and 2.1x kuna's own base (972).**
   Unknown libc return types force a cast at the call site. `libctypes=glibc`
   already buys -1.5%; this is the `libcwiden`/`libctypes` line of work.
4. **`<addr>` casts on stack slots — `64*<addr>` +1,139, `32*<addr>` +332,
   `8**<addr>` +382 (4.6x base).** A stack slot is declared at one width and read
   at another, so every access re-casts `&v`. This is the framelayout/slot-typing
   lever, and note `framelayout=off` and `slotptr=off` move casts by **zero** —
   they change the recorded type without changing the emitted C.
5. **`(long)<call>` +249 and `64<deref>` +434** — a call result or load whose type
   is known to the analysis but printed at the wrong width.

---

## 4b. Two classes I checked and one of them is a dead end

**Provably redundant no-op casts are NOT the story — 0.2%.** A cast whose target
type equals the operand variable's own declared type can be deleted with no
semantic change at all. kuna emits **89 of them in 44,210 casts (0.2%)**; IDA
emits 84 (0.2%), Ghidra 57 (0.1%), kuna_base 95 (`redundant.py`). There is no
"just delete the pointless casts" win here. Every other cast kuna prints is doing
something in C, so the gap has to be closed by changing the **type** or the
**lowering**, never by suppressing output.

**Signedness-only casts — 943 (2.1%), and kuna leans the wrong way.** A cast
whose target differs from the operand's declared type *only* in signedness
(`signonly.py`):

```
             total   dominant direction
kuna           943   u64->i64 323, u32->i32 273   (declared unsigned, used signed)
ida            587   i32->u32 227, i64->u64 220   (declared signed, used unsigned)
ghidra         944   u64->i64 409, u32->i32 370
kuna_base    1,001
```

kuna's 596 unsigned→signed casts are the shape `if (0 <= (long)v15)` with
`unsigned long v15;` — the program does a **signed** comparison on a variable kuna
typed unsigned, so the printer must cast it back. IDA's bias is the opposite: it
defaults a local to signed, so it pays the cast in the rarer unsigned direction.
Typing such a local signed when its only ordering use is signed removes the cast
outright — the type becomes right and the cast disappears.

Worth flagging: **decbench `type_match` scores signedness at zero**, so this whole
class is invisible to the campaign metric. It is a pure output-quality lever, and
only a cast census can see it.

---

## 5. Three cautions for whoever acts on this

- **`slotptr`, `framelayout` and `typedepth` move the cast count by exactly 0.**
  Options that improve `type_match` need not touch the emitted C at all. Cast
  count and decbench `type_match` are different targets; do not assume a
  type_match win is a cast win, or the reverse.
- **The cast gap is two-sided.** kuna already casts *less* than IDA on 36% of
  functions and on 5 of the top-30 shapes. A blunt "emit fewer casts" change will
  regress the 8,351 casts where kuna is already ahead. Every candidate change
  should be measured with `castcount.py` on the shared set, both directions.
- **There is no cast-suppression shortcut.** Only 0.2% of kuna's casts are no-ops.
  The measured wins all require the type or the lowering to change: pointer
  indexing instead of integer laundering (lever 1), a named global instead of a
  cast constant (lever 2), a libc return type (lever 3), a slot width (lever 4),
  a signed local (4b). That matches what the user asked for — remove casts because
  the type is right, not because the cast was hidden.
