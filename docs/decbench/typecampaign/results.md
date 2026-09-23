# Type-recovery campaign — results

The campaign ran 2026-09-16 → 2026-09-19 (plan: `~/.claude/plans/today-we-are-taking-quirky-rabin.md`).
Its three stated goals were (1) get primitives right (int/char/bool), (2) emit fewer variables, (3) recover
structs the way angr does; its yardstick was decbench `type_match` against IDA. This page is the Stage-3
re-measure: the pinned campaign baseline against a fresh release build of `origin/main`, on every
instrument the campaign built, plus what the numbers cannot see. **Round C** (below) repeats every
measurement on `origin/main` `d8b9c0b1` once the eight items round B left open had landed. **Round D**
and **Round E** repeat them again on `4c7704e0` and `2da619852`.

| | binary | commit |
|---|---|---|
| **baseline** | `/home/mahaloz/kwt/_baseline/kuna` (pinned copy) | main `809712e9` (2026-09-16) |
| **final** | `/home/mahaloz/kwt/_final/kuna` (pinned copy) | main `d96e3408` (2026-09-19, charbyte #683) |

decbench `main` @ `625e892` (the checkout the baseline was taken with), results tree
`full_run_address_2026-09-11`, `DECBENCH_NO_CACHE=1`, read-only on the tree.

## Headline

| goal | instrument | baseline | final | reading |
|---|---|---|---|---|
| type_match (A) | typesweep, 444 slices / 10,748 functions | **848** perfect (7.89%), mean **.2645** | **986** perfect (9.17%), mean **.2895** | +138 perfect, +9.5% mean; 1,449 functions improved, 8 worse |
| vs IDA (same 10,273 functions) | canonical replay of the tree's IDA rows | kuna 8.12% / .2718 vs ida 8.33% / .2682 | kuna **9.43% / .2978** vs ida 8.33% / .2682 | kuna passes IDA on perfect and widens its mean lead; binja still leads (12.28% / .3422) |
| goal 1: primitives | per-GT-class match rate | bool 21.6%, char 53.7%, int 50–59% | **bool 42.8%**, char 53.7%, int unchanged | bool doubled (1st of five by 27 points); int/char were already 1st and cannot move on this metric |
| goal 2: variables | varcensus, fmt/ls/sort/du O0+O2 | 7,085 non-thunk declarations, 151 `[16]` blobs | 6,945 (−2.0%), **66** blobs (−56%) | fewer, cleaner declarations; the fmt::main phantom arguments are fixed only with `argclobber on` (opt-in) |
| goal 3: structs | structscore layout-F1 (835 GT struct-pointer params) | 0.0 (kuna synthesized no structs) | **0.138** (P 0.8945, R 0.0746); 182 params typed as a struct | first nonzero struct recovery; TRex Fig. 6 mean up on all 8 binaries |
| speed | whole-binary `decompile-all`, min-of-11 interleaved | — | fmt +0.6%, ls +1.0%, sort +1.3%, bash +2.5% | the whole campaign's default-on work costs at most 2.5%, inside the +5% budget |

## 1. type_match — the 444-slice typesweep

`scripts.decbench.typesweep --baseline-only`, projects coreutils grep gzip diffutils bzip2 findutils tar shadow ×
O0/O2/O2-noinline, `--workers 24`, run with **both** pinned binaries on the same day against the same
checkpoints. Control: the re-run of the pinned baseline binary reproduces the 2026-09-16 capture exactly —
10,748/10,748 functions, 848 perfect, aggregate 2843.28, **0 values differ** — so every delta below is the
binary, not the harness or the tree. (Agreement with the tree's published kuna verdicts: 97.08% for the
baseline binary, 84.88% for the final one; the tree was scored with a kuna older than both.)

| slice | functions | perfect base → final | perfect % | mean base → final | improved | worse |
|---|---:|---|---|---|---:|---:|
| **ALL** | 10,748 | 848 → **986** (+138) | 7.89% → 9.17% | .2645 → **.2895** (+.0250) | 1,449 | 8 |
| O0 | 4,286 | 612 → 704 (+92) | 14.28% → 16.43% | .4167 → .4467 | 597 | 2 |
| O2 | 2,394 | 42 → 53 (+11) | 1.75% → 2.21% | .1330 → .1514 | 343 | 2 |
| O2-noinline | 4,068 | 194 → 229 (+35) | 4.77% → 5.63% | .1817 → .2050 | 509 | 4 |
| bzip2 | 267 | 15 → 19 | 5.62% → 7.12% | .3441 → .3664 | 38 | 0 |
| coreutils | 6,422 | 525 → 610 | 8.18% → 9.50% | .2560 → .2813 | 932 | 7 |
| diffutils | 420 | 43 → 52 | 10.24% → 12.38% | .3592 → .4004 | 68 | 0 |
| findutils | 790 | 31 → 35 | 3.92% → 4.43% | .1444 → .1622 | 63 | 0 |
| grep | 247 | 37 → 39 | 14.98% → 15.79% | .3614 → .3870 | 31 | 0 |
| gzip | 368 | 83 → 88 | 22.55% → 23.91% | .4654 → .4858 | 30 | 0 |
| shadow | 686 | 20 → 29 | 2.92% → 4.23% | .2619 → .2897 | 139 | 0 |
| tar | 1,548 | 94 → 114 | 6.07% → 7.36% | .2600 → .2824 | 148 | 1 |

139 functions moved onto perfect and 1 moved off. Every project and every opt level moved up; no slice
lost mean. Per project × opt is in `final/report-slices.md`; every moved function is in `final/moved.csv`.

**The 8 worse functions, all read:** 7 are `structsynth` putting a `struct_N *` on a pointer DWARF calls
primitive — the GMP limb arrays of `factor` (`powm2` and `millerrabin2` at all three levels: `mp_limb_t *bp`
→ `struct_0 *`) and `shred`'s `passname(char *name)`. That is the false-positive class the structsynth PR
measured (−0.151% perfect on its 300 slices) and accepted when the user flipped it on; its fix (#679) was
closed because, under the struct-crediting rule the user asked decbench for, it removes more correct struct
pointers than it fixes. The 8th is tar `write_directory_file_entry(void *data)` now typed `FILE *`.

### Where kuna stands against the other decompilers

The rivals' columns are decbench's `main` metric replayed over the **variables the tree stored for them**
(`finalsweep.py rivals`, the same `_match_structured` `reeval_typematch.py` runs, no `--emit`), on the same
444 slices. Each rival's function set is a subset of kuna's, so each row pairs the rival with kuna on exactly
the rival's functions.

| decompiler (functions it scored) | all: perfect % / mean | O0 | O2 | O2-noinline |
|---|---|---|---|---|
| **binja** (10,366) | 12.28% / .3422 | 20.36% / .4956 | 3.43% / .1868 | 8.52% / .2632 |
| kuna final, same functions | 9.35% / .2969 | 16.42% / .4465 | 2.13% / .1542 | 5.74% / .2144 |
| kuna baseline, same functions | 8.02% / .2711 | 14.27% / .4165 | 1.65% / .1351 | 4.81% / .1893 |
| **ida** (10,273) | 8.33% / .2682 | 13.70% / .3835 | 2.27% / .1485 | 5.78% / .2072 |
| kuna final, same functions | **9.43% / .2978** | **16.43% / .4467** | 2.27% / **.1558** | 5.67% / **.2117** |
| kuna baseline, same functions | 8.12% / .2718 | 14.28% / .4167 | 1.77% / .1357 | 4.83% / .1869 |
| **angr** (10,502) | 8.27% / .2652 | 14.89% / .4109 | 1.64% / .1346 | 4.95% / .1827 |
| kuna final, same functions | 9.31% / .2926 | 16.43% / .4467 | 2.24% / .1539 | 5.70% / .2057 |
| **ghidra** (10,673) | 6.84% / .2367 | 11.29% / .3394 | 1.48% / .1320 | 5.24% / .1886 |
| kuna final, same functions | 9.14% / .2904 | 16.43% / .4467 | 2.16% / .1521 | 5.49% / .2051 |

kuna went from **4th of five** on perfect% (behind binja, IDA and angr, each on its own functions) to **2nd**
(behind binja), and stayed 2nd on mean with the gap to binja cut from .071 to .045. The campaign opened on
"kuna is last-of-four at O2": on this slice set it now ties IDA's O2 perfect rate, leads it on O2 mean (.1558
vs .1485), and is within 0.11 points of it on O2-noinline perfect while leading on mean. binja's lead is
`ptr_char` and `ptr_void` (below).

## 2. Per ground-truth class

Every GT variable of every scored function, classified from its DWARF DIE (typedefs/cv/restrict stripped:
`ptr_struct` = pointer to a struct, `ptr_prim` = pointer to a non-char base type, `int_s4` = signed 4-byte
integer, …) by `final/gtclass.py`, whose DIE walk mirrors decbench's `extract_ground_truth_types` exactly and
only adds the class — the scores are unchanged (checked function-by-function against plain typesweep). Rate =
TP / GT variables of that class (a miss and a wrong type both count against it). Rival rates are over the GT
variables of the functions that rival scored.

| GT class | GT vars | share | kuna baseline | **kuna final** | ida | binja | ghidra | angr |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `ptr_struct` | 14,252 | 21.7% | 0.0% (0) | **2.5% (355)** | 2.4% (328) | 2.8% (389) | 2.3% (325) | 2.3% (315) |
| `ptr_char` | 14,645 | 22.3% | 20.6% (3,015) | 20.7% (3,028) | 24.4% (3,543) | 36.8% (5,323) | 18.3% (2,675) | 16.4% (2,361) |
| `ptr_ptr` | 1,834 | 2.8% | 18.3% (335) | **28.5% (523)** | 38.3% (670) | 30.8% (542) | 1.5% (27) | 5.6% (101) |
| `ptr_void` | 3,481 | 5.3% | 1.5% (52) | 1.4% (50) | 1.6% (53) | 12.8% (402) | 1.3% (46) | 2.4% (80) |
| `ptr_prim` | 1,242 | 1.9% | 40.7% (506) | 39.3% (488) | 30.7% (366) | 51.7% (607) | 32.0% (397) | 37.1% (452) |
| `int_s4` | 8,788 | 13.4% | 53.4% (4,690) | 53.4% (4,690) | 48.0% (4,199) | 57.5% (5,012) | 43.1% (3,781) | 53.4% (4,598) |
| `int_u4` | 1,596 | 2.4% | 54.4% (869) | 54.6% (871) | 47.1% (749) | 58.4% (902) | 41.8% (667) | 53.5% (840) |
| `int_s8` | 2,478 | 3.8% | 58.5% (1,450) | 58.5% (1,450) | 48.6% (1,201) | 58.1% (1,431) | 46.4% (1,149) | 56.2% (1,381) |
| `int_u8` | 7,657 | 11.7% | 50.7% (3,885) | 50.8% (3,888) | 42.0% (3,101) | 51.2% (3,887) | 40.9% (3,131) | 50.7% (3,791) |
| `char` | 846 | 1.3% | 53.7% (454) | 53.7% (454) | 43.7% (351) | 53.0% (441) | 34.9% (295) | 52.3% (431) |
| `bool` | 3,222 | 4.9% | 21.6% (696) | **42.8% (1,378)** | 9.1% (292) | 2.6% (83) | 15.4% (495) | 0.0% (0) |
| `enum` | 518 | 0.8% | 57.7% (299) | 57.7% (299) | 49.6% (257) | 60.1% (310) | 45.2% (234) | 55.9% (286) |
| `array` | 750 | 1.1% | 18.9% (142) | 18.9% (142) | 8.3% (62) | 18.7% (140) | 19.1% (143) | 11.9% (88) |
| `array_char` | 1,448 | 2.2% | 13.1% (190) | 13.1% (190) | 7.8% (113) | 7.1% (103) | 9.5% (137) | 42.8% (605) |
| `struct_val` | 1,769 | 2.7% | 0.0% (0) | **28.1% (497)** | 2.4% (42) | 5.1% (90) | 30.6% (541) | 20.4% (353) |
| other (float, funcptr, unions, …) | 1,189 | 1.8% | 7.8% (93) | 7.8% (93) | 12.0% (142) | 11.7% (139) | 5.3% (63) | 4.2% (48) |
| **all GT variables** | 65,715 | | 25.4% (16,676) | **28.0% (18,396)** | 24.1% (15,469 / 64,241) | 30.7% (19,801 / 64,526) | 21.5% (14,106 / 65,562) | 24.5% (15,730 / 64,273) |

| storage | GT vars | kuna baseline | **kuna final** | ida | binja | ghidra | angr |
|---|---:|---:|---:|---:|---:|---:|---:|
| argument | 21,577 (32.8%) | 36.1% | **40.4%** | 40.6% | 48.4% | 35.4% | 33.3% |
| stack | 18,317 (27.9%) | 48.5% | **52.8%** | 38.2% | 51.8% | 35.6% | 46.8% |
| register only | 25,821 (39.3%) | 0.0% | 0.0% | 0.4% | 1.4% | 0.1% | 0.9% |

What moved, and which PR moved it:

* **`ptr_struct` 0 → 355**, all libc names: `FILE *` 252, `stat *` 46, `passwd *` 18, `tm *` 12, `group *` 11,
  `DIR *` 10 (libctypes #651/#658, stdio streams #672). kuna is now at parity with IDA/ghidra/angr and 34 TP
  behind binja on the class that was its literal zero.
* **`struct_val` 0 → 497** (2nd of five, behind ghidra's 541): by-value libc structures on the stack — `stat`
  412, `sigset_t` 50, `termios` 10, `sigaction` 9 — the same libctypes vocabulary reaching the frame slots.
* **`ptr_ptr` 18.3% → 28.5%**: all of it `char **` (335 → 523 TP); not bisected to a single PR.
* **`bool` 21.6% → 42.8%**: `bool` 99 → 549 TP from boolbyte (#664, on by #680), `undefined1` 589 → 821 TP from
  bytehonest (#650 — the honest width-only spelling decbench credits against `_Bool`, as it does IDA's `_BYTE`).
* **The integer and `char` classes did not move and could not**: decbench strips `unsigned` and maps
  `int4`/`uint4` → `int`, so signedness (#656, 377 declarations flipped) scores zero by construction. kuna was
  already 1st or 2nd on every one of them.
* **`ptr_prim` −18** is the structsynth false-positive class above; **`ptr_char` +13** is flat — binja's 36.8%
  is the largest single gap left (−2,295 TP), and the lever for it (ptrfromuse, protoorder) is not on by
  default yet (§7).
* The storage split is the campaign in one line: argument rate 36.1% → 40.4% (level with IDA), stack 48.5% →
  52.8% (best of five), register-only 0.0% → 0.0% (§8).

## 3. Goal 1 — primitives (int / char / bool)

| class | baseline | final | rank of five |
|---|---:|---:|---|
| `bool` | 21.6% | **42.8%** | 1st (next: ghidra 15.4%) |
| `char` | 53.7% | 53.7% | 1st |
| `int_s4` / `int_u4` | 53.4% / 54.4% | 53.4% / 54.6% | 2nd (binja) |
| `int_s8` / `int_u8` | 58.5% / 50.7% | 58.5% / 50.8% | 1st / 2nd |
| `enum` | 57.7% | 57.7% | 2nd |

The TRex Fig. 6 `c_primitive` step (which does see signedness, from the DWARF encoding) passes more often on
all eight structscore binaries — fmt O0 142/210 → 161/228, du O2 137/224 → 186/270 (§5). The signedness work
is visible there and in the C text (377 declarations flipped, 443 casts dropped, #656), never in `type_match`.

## 4. Goal 2 — variables

`scripts.pipeline.varcensus` on the stripped binaries decbench scores (never-written locals = declared `vN`
that no statement assigns, the shape of a phantom register argument; counted by the protoorder lane's
`callsite-metrics.py` rule, copied as `final/callsite_metrics.py`):

| opt | binary | non-thunk declarations | single-def/single-read | `[16]` blobs | never-written locals | … passed as a call argument |
|---|---|---|---|---|---|---|
| O2 | fmt | 332 → 314 | 38 → 40 | 23 → 4 | 5 → 5 | 0 → 0 |
| O2 | ls | 1,278 → 1,244 | 178 → 180 | 33 → 10 | 56 → 56 | 8 → 8 |
| O2 | sort | 1,084 → 1,055 | 132 → 133 | 31 → 12 | 31 → 30 | 6 → 6 |
| O2 | du | 968 → 951 | 103 → 105 | 27 → 7 | 40 → 38 | 16 → 15 |
| O0 | fmt | 264 → 263 | 39 → 39 | 5 → 4 | 9 → 9 | 0 → 0 |
| O0 | ls | 1,146 → 1,128 | 177 → 177 | 19 → 18 | 32 → 31 | 6 → 6 |
| O0 | sort | 1,093 → 1,074 | 143 → 144 | 7 → 6 | 47 → 44 | 10 → 10 |
| O0 | du | 920 → 916 | 159 → 156 | 6 → 5 | 41 → 39 | 12 → 11 |
| **total** | | **7,085 → 6,945 (−2.0%)** | 969 → 974 | **151 → 66 (−56%)** | 261 → 252 | 58 → 56 |

85 of the 140 fewer declarations are mulblob's (#671, the 128-bit multiply operand blobs); the rest are spread
across the type PRs and were not attributed one by one. kuna was already the leanest declarer of the four
(3.33 declarations per function against IDA's 4.07 before the campaign), and `type_match` gives a missing or an
extra variable exactly zero, so none of this moves §1.

The phantom-`rdx` shape specifically — a never-written local declared `// rdx` — is **11 → 11** (all O2: fmt 2,
ls 2, du 7). Nothing on by default targets it; with `--option argclobber on` it is 9 (fmt's two are gone, the
ls and du ones are not the clobbered-argument shape).

**`fmt::main`, the named bad case** (O2, `0x26a0`; full texts in `final/fmt-main-{base,final,final-argclobber}.c`):

```c
// baseline                                      // final (default)
unsigned long sub_26a0(int a0,char **a1)          int main(int argc,char **argv,char **envp)
  void *v5; // rax                                  FILE *v6; // rax
  getopt_long(a0,v13,"…",(void *)0xbaa0,NULL)       getopt_long(argc,v13,"…",(option *)0xbaa0,NULL)
  v12 &= sub_3700(v5);                              v12 &= sub_3700(v6);
  v3 = sub_3700(stdin,v6);                          v3 = sub_3700(stdin,v7);
  v12 = sub_3700(stdin,"-",v10);                    v12 = sub_3700(stdin,"-",v10);
  return (unsigned long)((v12 ^ 1) & 0xff);         return (v12 ^ 1) & 0xff;
```

The name, prototype, `FILE *` and `option *` came from elfmain and libctypes. The goal-2 defect in this function
— `sub_3700` called with 1, 2 and 3 arguments and the read-before-write `rdx` locals `v9`/`v10`/`v11` behind the
third — is **still there by default**. `--option argclobber on` (#652) turns it into 2/2/2 and removes the three
phantom locals (`final/fmt-main-final-argclobber.c`), but it stays opt-in (§7). protoorder's `types` redesign
deliberately changes no call arity, so it would not fix this either.

## 5. Goal 3 — structs

`scripts.decbench.structscore --all` on fmt/ls/sort/du, O0 and O2, each binary run with the baseline and with
the final kuna (the final one has structsynth `param` on by default, #655 → #682):

| opt | binary | TRex mean 0-6 | mean 0-5 | `is_c_struct` passed | `c_primitive` passed | params typed as a struct / GT struct params | layout F1 (filler counted) |
|---|---|---|---|---|---|---|---|
| O0 | fmt | 3.901 → **4.032** | 3.557 → 3.642 | 260/293 → 277/293 | 142/210 → 161/228 | 0 → 4 / 34 | 0 → 0.052 |
| O0 | ls | 3.593 → **3.684** | 3.312 → 3.369 | 1092/1259 → 1139/1260 | 522/715 → 586/773 | 0 → 32 / 168 | 0 → 0.130 |
| O0 | sort | 3.601 → **3.715** | 3.332 → 3.403 | 871/1036 → 925/1036 | 406/587 → 471/640 | 0 → 35 / 162 | 0 → 0.162 |
| O0 | du | 3.660 → **3.790** | 3.392 → 3.477 | 863/1024 → 918/1026 | 384/548 → 449/611 | 0 → 34 / 174 | 0 → 0.125 |
| O2 | fmt | 1.732 → **1.848** | 1.595 → 1.675 | 114/135 → 128/138 | 58/95 → 73/109 | 0 → 2 / 23 | 0 → 0.039 |
| O2 | ls | 1.470 → **1.504** | 1.375 → 1.383 | 388/457 → 361/457 | 152/250 → 195/289 | 0 → 24 / 79 | 0 → 0.253 |
| O2 | sort | 1.623 → **1.717** | 1.502 → 1.563 | 294/372 → 330/372 | 146/227 → 187/264 | 0 → 22 / 95 | 0 → 0.181 |
| O2 | du | 1.627 → **1.746** | 1.511 → 1.588 | 288/374 → 329/376 | 137/224 → 186/270 | 0 → 29 / 100 | 0 → 0.197 |

Pooled: TRex mean O0 3.638 → **3.750**, O2 1.579 → **1.660**; 182 of the 835 GT pointer-to-struct parameters are
now typed as a struct. Layout, fields only (`docs/features/structsynth/layoutscore.py`, filler excluded): precision
**704/787 = 0.8945**, recall 0.0746, F1 **0.0 → 0.138** — about 1 claimed field in 10 is not a DWARF field at that
offset and size. (Counting the `undefined1 field_0x…[N]` filler as claims: 798/1,021 = 0.782.) For scale only —
different ground-truth normalization — RecStruct reports coreutils-O2 layout F1 22.06, TRex 7.76.

Honest caveats: **nesting F1 is still 0** everywhere (a synthesized field is never itself a struct pointer yet);
**ls O2 `is_c_struct` went down** (388 → 361), the `struct_N *`-on-a-non-struct class; and `type_match` scores
every `struct_N *` as a miss, because no synthesized name can equal a program-defined DWARF name (§8). structdedup
(#675, one layout per shape across the binary, 516 → 472 struct names) is not landed.

## 6. Speed

`kuna decompile-all <bin> --json --max-fn-seconds 120` (decbench's own invocation) on the O2 stripped binaries,
baseline and final binaries interleaved with the order alternating each round, 11 rounds, on a quiet box (load
average about 2 on 80 cores). Driver: `final/speed.py`; raw samples: `final/speed.json`.

| binary | functions | baseline min | final min | Δ min | Δ median |
|---|---:|---:|---:|---:|---:|
| coreutils fmt | 151 | 4,043.5 ms | 4,067.4 ms | +0.59% | +0.53% |
| coreutils ls | 404 | 13,038.6 ms | 13,170.7 ms | +1.01% | +1.08% |
| coreutils sort | 343 | 13,781.6 ms | 13,959.6 ms | +1.29% | −0.23% |
| bash | 2,538 | 83,056.9 ms | 85,117.5 ms | **+2.48%** | +2.61% |

Every campaign item carried its own speed block; this is their sum on whole binaries. bash is 1.3 MB, so
`--mode auto` resolves to `reliable` there (500 KiB – 2 MiB) rather than `aggressive` — it measures the other
preset. Min and median
agree on everything but sort, where both sit within 1.3% of zero.

## 7. Every campaign PR and what it measured

Typesweep numbers are the PR's own 444-slice (or stated subset) measurement at its base; the metric-neutral
ones say why.

| PR | item | default | measured effect |
|---|---|---|---|
| #647 | fix(cli): fail a bad `--option` value | strict fix | made every `kuna decompile` ablation trustworthy; no metric channel |
| #648 | `impliedrefs` / `termdup` value options | 2 / 2 (upstream) | byte-identical at the defaults |
| #649 | `elfmain` — name and type the ELF `main` | on | 7 `main`s better, 0 worse (18 slices); readability + caller return type |
| #650 | `bytehonest` — `undefined1` for an uncommitted byte | on | +10 perfect, 121 improved / 0 worse (5,298 fns); most of the `bool` gain above |
| #651 #658 | `libctypes` — `FILE *`, `stat *`, … in the libc tables | `opaque` (on) | **889 → 959 (+70)**, 720 improved / 1 worse; the whole `ptr_struct`/`struct_val` gain |
| #652 | `argclobber` — drop a clobbered trailing register argument | off | fmt::main 1/2/3 → 2/2/2; typesweep 986 → 986 (the metric cannot see arity) |
| #653 | instruments: `mine --metric`, typescore, varcensus, structscore | tooling | — |
| #654 | `foldcallretphi` | off | metric-neutral (986 = 986 at its default-on evaluation, #681); −14 declarations on that sweep |
| #655 #682 | `structsynth param` (on by #682, user decision) | `param` (on) | layout F1 0 → 0.138 at precision 0.8945; −0.151% perfect / 3 worse (factor/shred) |
| #656 | `signedness` | `auto` | typesweep byte-identical by construction; 377 declarations flipped, 443 casts dropped |
| #659 | `ptrfromuse` (`off|byte|void`) | off | byte 959 → 964, 15 improved / 0 worse; void +100 improved / 0 worse |
| #660 | `libctypes glibc` — real `FILE`/`stat`/… field layouts | opt-in value | 959 → 959, 9 improved / 3 worse |
| #661 | `structdefs` — print referenced composites above a function | off | metric-neutral (the preamble is not scored) |
| #662 | foldcallret barrier fix (GH-657) | strict fix | 0 of 10,748 functions change score |
| #663 | `indirectonly` | off | metric-neutral (10,739 functions byte-identical) |
| #664 #680 | `boolbyte` (on by #680) | on | **959 → 987 (+28)**, 430 improved / 0 worse |
| #670 | `hideshadow` | on | metric-neutral; removes redundant shadow copies |
| #671 | `mulblob` — lower the 128-bit multiply idiom | on | metric-neutral; `[16]` blobs −56% |
| #672 | libctypes stdio streams typed `FILE *` | on (with libctypes) | typed data symbols `stdin`/`stdout`/`stderr` |
| #674 | never emit a goto to a missing label | strict fix | invalid C removed (sort O2 0x3ec0); metric unmoved |
| #677 | `expandload` — keep a narrow load's width | strict fix | metric unmoved (959 = 959) |
| #678 | `truncarg` — keep a narrowed argument's truncation | strict fix | metric unmoved |
| #683 | `charbyte` — keep `char` for a byte loaded through `char *` | on | metric-neutral by construction (986 = 986); 19 variables `unsigned char *` → `char *` |

**Not landed** (measured, open or closed):

| PR / lane | item | state | measured on its base |
|---|---|---|---|
| #669 | `protoorder types` — callees first, call-site types from the callee's recovered prototype | open, review CHANGES | 986 → **1,105 (+119)** on d96e3408, 3 worse; 0 call-arity changes on its 60-binary corpus; speed ≤ +3.4% |
| #673 | `formatstring` at load time | open, review CHANGES | 987 → **1,025 (+38)**, 176 better / 2 worse; speed −1.1..+1.0% |
| #675 | `structdedup` — one layout per shape across the binary | open, review CHANGES | struct names 516 → 472, layout precision held (.8750 vs .8735 held-out) |
| #679 | structsynth homogeneous-layout decline | closed | net −91 correct struct pointers under the struct-crediting rule |
| #681 | `foldcallretphi` on by default | open, review CHANGES | 986 = 986; review rounds found unsound folds (short-circuit right operand, wide integer returns) |
| ptrfromuse-on | `ptrfromuse void` default | no PR (waits on #669) | on the protoorder base: 1,105 → **1,309 (+204)**, 85 improved / 0 worse |
| argclobber-on | `argclobber` default | no PR | stays off: 4 constructed counterexamples drop a read argument |
| libctypes-glibc-on | `libctypes glibc` default | no PR | stays opt-in: 5 defect classes of #660 reproduce (misnamed members, lost by-value `stat`, …) |

The open items were each measured separately on their own base; they are not additive, but the two
largest (protoorder, ptrfromuse void) were measured stacked: **1,309 perfect** on the 444 slices once both land.

### What stays opt-in, and why

* **argclobber** — the one fix for fmt::main's phantom arguments. Unsound on four constructed cases (it drops an
  argument the callee does read); the 758-binary sweep's 22 drops were all correct, but a wrong-output surface
  needs a sound rule, not a clean sample.
* **ptrfromuse** — its `void` arm is +204 perfect with 0 worse, but only on top of protoorder; the flip is staged
  behind #669.
* **libctypes `glibc`** — real field names are right on most functions and wrong on five measured shapes; `opaque`
  is the default.
* **foldcallretphi, indirectonly** — readability passes with reviewed soundness holes (foldcallretphi) or a lifted
  merge refusal on illegal inputs (indirectonly); both are worth zero on the metric.
* **structdefs** — a preamble, not a type decision; off to keep per-function output unchanged.
* **signedness `prefer-signed`** — moves datatest assertions; the default `auto` does not.
* **protoorder `lock`, formatstring** — `lock` fabricates parameters; formatstring waits on #673.

## 8. What the metric can and cannot see, and the ceilings left

* **Register-resident ground truth: 25,821 of 65,715 GT variables (39.3%).** kuna exports arguments, stack symbols
  and framelayout slots — never a register local — so it scores 4 of them; no rival does better than 1.4%. This caps
  the achievable mean near 0.6 for everyone and it is most of the O2 gap: TRex's `defined` step is 77–81% of the
  final binary's O2 loss in §5.
* **Program-defined struct names: 10,559 of 14,252 `ptr_struct` GT variables (16.1% of all GT).** `hash_entry`,
  `Hash_table`, `fileinfo`, `predicate`, `tar_stat_info`, … — a stripped binary does not carry these names and no
  decompiler recovers them: every one of kuna's 355 `ptr_struct` TPs is a libc name, and so is every rival's
  (IDA 328: `FILE` 315, `DIR` 11, `__sigset_t` 2). Of the rest, 2,472 carry a libc/POSIX name (by a hand list:
  `FILE`, `stat`, `passwd`, `obstack`, …) and are the reachable pool, and 1,221 are the restrict artifact below.
  structsynth recovers the *shape*, which `type_match`
  cannot credit: replaying the final rows under the crediting rule proposed in Noelo-Lab/decbench#93 (any struct
  pointer matches a pointer-to-struct GT) would add **1,287 TP and 173 functions onto perfect** (986 → ~1,159).
* **Restrict-qualified pointers are unmatchable: 4,412 GT variables (6.7%).** decbench's `_parse_type_die` has no
  `DW_TAG_restrict_type` arm, so `char *restrict dest` (`sort xstrxfrm`) arrives as the GT type `void`: 2,951
  `ptr_char`, 1,221 `ptr_struct` and 240 `ptr_void` variables that no decompiler can match. A one-line decbench fix;
  not taken here because the campaign measures against a pinned metric.
* **Zero-weight dimensions.** Signedness, variable count, call arity and the C text of any function with a non-empty
  `variables[]` are invisible to `type_match`. Goals 1 (int/char), 2 and most of 3 were therefore measured on their
  own instruments (varcensus, structscore, the corpus diffs in each PR), and the headline number moved only through
  pointers, `bool` and by-value libc structs.

## Round C — 2026-09-20

Round B left five measured items open: #669 `protoorder`, #673 `formatstring`, #675
`structdedup`, #681 `foldcallretphi` and the `ptrfromuse void` flip. All five landed, with two
fixes and a tooling change beside them (#687, #686, #688); `argclobber` default-on is still open
as #689. This section re-measures `origin/main` `d8b9c0b1` on the same instruments, keeping the
round-B binary as the middle column, so every number below reads baseline → round B → round C.

| | binary | commit |
|---|---|---|
| **round C** | `/home/mahaloz/kwt/_final-c/kuna` (pinned copy) | main `d8b9c0b1` (2026-09-20, ptrfromuse-on #690) |

**Metric pin.** The decbench checkout moved past `625e892` (the campaign's metric, decisions §11)
and `type_match._effective_offset` is gone in the newer code, so the instruments import-fail
against the working tree. Every measurement here runs against `625e892` extracted read-only
with `git archive`, with the venv's PEP-660 editable finder repointed at that copy
(`final-c/pindb.py`) — the editable finder outranks `PYTHONPATH`, so pointing the path alone
silently keeps the new metric. Control: the round-B binary re-run under the pin reproduces the
round-B capture exactly, 10,748/10,748 functions, 986 perfect, **0 values differ**.

### Headline

| goal | instrument | baseline | round B | round C | reading |
|---|---|---|---|---|---|
| type_match | typesweep, 444 slices / 10,748 functions | 848 perfect (7.89%), mean .2645 | 986 (9.17%), .2895 | **1,349** (12.55%), **.3403** | +363 perfect over round B; 1,384 improved, 2 worse, **0 dropped off perfect** |
| vs the other decompilers | canonical replay, each rival on its own functions | 4th of five | 2nd | **1st** | on binja's 10,366 functions kuna is 12.29% / .3427 against binja's 12.28% / .3422; against IDA's set 12.60% / .3464 vs 8.33% / .2682 |
| goal 1: primitives | per-GT-class match rate | bool 21.6% | bool 42.8% | unchanged | the int/char/bool classes were already 1st or 2nd and did not move this round |
| goal 2: variables | varcensus, fmt/ls/sort/du O0+O2 | 7,085 declarations | 6,945 | 6,970 (+25) | the phantom-`rdx` locals go 11 → 9; `fmt::main` still calls `sub_3700` with 1, 2 and 3 arguments by default |
| goal 3: structs | structscore TRex mean / layout precision | 3.638 / 1.579, layout 0 | 3.750 / 1.660, P .8945 | **4.108 / 1.893**, P **.5520** | TRex up on all eight builds; per-parameter layout precision is a **regression**, and it is protoorder's (below) |
| decbench#93 crediting | replay of the same rows | 848 | 986 → 1,159 | 1,349 → **1,575** | +1,569 TP, 226 more functions perfect if a struct pointer counts against a GT pointer-to-struct |
| speed | whole-binary `decompile-all`, interleaved min-of-11 | — | +0.6…+2.5% vs baseline | +0.7…+2.5% vs baseline, ≈+1% vs round B | eight items for about one percent; bash needed the >5% re-run rule and came back at −1.6% |

### C.1 type_match

Same 444 slices, same `--baseline-only` invocation, the round-C binary and the round-B control
binary running side by side at `--workers 12`.

| slice | functions | base | round B | **round C** | mean base → B → **C** | improved | worse |
|---|---:|---:|---:|---:|---|---:|---:|
| **ALL** | 10,748 | 848 (7.89%) | 986 (9.17%) | **1,349 (12.55%)** | .2645 → .2895 → **.3403** | 1,384 | 2 |
| O0 | 4,286 | 612 | 704 | **895 (20.88%)** | .4167 → .4467 → **.5041** | 577 | 1 |
| O2 | 2,394 | 42 | 53 | **74 (3.09%)** | .1330 → .1514 → **.1820** | 279 | 0 |
| O2-noinline | 4,068 | 194 | 229 | **380 (9.34%)** | .1817 → .2050 → **.2609** | 528 | 1 |
| bzip2 | 267 | 15 | 19 | 31 | .3441 → .3664 → .4097 | 24 | 0 |
| coreutils | 6,422 | 525 | 610 | 877 | .2560 → .2813 → .3339 | 725 | 1 |
| diffutils | 420 | 43 | 52 | 58 | .3592 → .4004 → .4325 | 38 | 0 |
| findutils | 790 | 31 | 35 | 41 | .1444 → .1622 → .2200 | 165 | 0 |
| grep | 247 | 37 | 39 | 51 | .3614 → .3870 → .4351 | 24 | 1 |
| gzip | 368 | 83 | 88 | 102 | .4654 → .4858 → .5197 | 28 | 0 |
| shadow | 686 | 20 | 29 | 31 | .2619 → .2897 → .3001 | 40 | 0 |
| tar | 1,548 | 94 | 114 | 158 | .2600 → .2824 → .3516 | 340 | 0 |

Every project and every optimization level moved up again, and the improved/worse columns are
round B → round C (363 functions moved onto perfect and none off). Per project × opt is in
`final-c/report-slices.md`; the 1,386 functions whose score moved are in `final-c/moved.csv`.

The three type flips were each measured on a different base (#669 +119, ptrfromuse `void` +204
on top of it, #673 +38), which predicts about +361 on round B's 986; the re-measure lands on
1,349, +363, so they compose rather than overlap.

**Both worse functions are `ptrfromuse void` on an integer**, the ambiguity #690 documents:
grep `buf_has_encoding_errors(char *buf)` prints `unsigned long *`, and coreutils `tail_forever`
gives the `uintmax_t bytes_to_read` local a `void *`.

### C.2 Per ground-truth class

Same classifier (`final/gtclass.py`), same 65,715 GT variables. The rivals' columns are round B's
replay unchanged — they score the variables the results tree stored for each decompiler, and neither
the tree nor the pinned metric moved. Only the moved rows are repeated here; the unmoved classes are
in round B's table and in `final-c/analysis.json`.

| GT class | GT vars | round B | **round C** | ida | binja | ghidra | angr |
|---|---:|---:|---:|---:|---:|---:|---:|
| `ptr_char` | 14,645 | 20.7% (3,028) | **28.7% (4,204)** | 24.4% | 36.8% (5,323) | 18.3% | 16.4% |
| `ptr_void` | 3,481 | 1.4% (50) | **17.6% (612)** | 1.6% | 12.8% | 1.3% | 2.4% |
| `ptr_prim` | 1,242 | 39.3% (488) | **46.8% (581)** | 30.7% | 51.7% | 32.0% | 37.1% |
| `ptr_struct` | 14,252 | 2.5% (355) | **3.0% (422)** | 2.4% | 2.8% | 2.3% | 2.3% |
| **all GT variables** | 65,715 | 28.0% (18,396) | **30.9% (20,295)** | 24.1% | 30.7% (19,801) | 21.5% | 24.5% |

| storage | GT vars | round B | **round C** | ida | binja | ghidra | angr |
|---|---:|---:|---:|---:|---:|---:|---:|
| argument | 21,577 | 40.4% | **48.9%** | 40.6% | 48.4% | 35.4% | 33.3% |
| stack | 18,317 | 52.8% | **53.2%** | 38.2% | 51.8% | 35.6% | 46.8% |
| register only | 25,821 | 0.0% | 0.0% | 0.4% | 1.4% | 0.1% | 0.9% |

Round C is a pointer round: +1,176 `char *`, +562 `void *`, +93 primitive pointers, +67 struct
pointers, and nothing else moves by more than 4. That is protoorder carrying a callee's recovered
parameter type back to the call site and `ptrfromuse void` declaring a dereferenced-only
parameter, which is why the whole gain sits on `argument` (40.4% → 48.9%, now the best of five).
kuna's overall variable rate passes binja's for the first time (30.9% of 65,715 GT variables
against binja's 30.7% of the 64,526 in the functions it scored), and `ptr_char` remains the
largest single gap to it (−1,119 TP).

### C.3 Goal 2 — variables

| opt | binary | declarations (base → B → C) | `[16]` blobs | never-written locals |
|---|---|---|---|---|
| O2 | fmt | 332 → 314 → 314 | 23 → 4 → 4 | 5 → 5 → 5 |
| O2 | ls | 1,278 → 1,244 → 1,236 | 33 → 10 → 10 | 56 → 56 → 54 |
| O2 | sort | 1,084 → 1,055 → 1,061 | 31 → 12 → 12 | 31 → 30 → 30 |
| O2 | du | 968 → 951 → 950 | 27 → 7 → 7 | 40 → 38 → 38 |
| O0 | fmt | 264 → 263 → 271 | 5 → 4 → 4 | 9 → 9 → 9 |
| O0 | ls | 1,146 → 1,128 → 1,132 | 19 → 18 → 18 | 32 → 31 → 31 |
| O0 | sort | 1,093 → 1,074 → 1,083 | 7 → 6 → 6 | 47 → 44 → 44 |
| O0 | du | 920 → 916 → 923 | 6 → 5 → 5 | 41 → 39 → 39 |
| **total** | | **7,085 → 6,945 → 6,970** | **151 → 66 → 66** | **261 → 252 → 250** |

Round C adds 25 declarations net — +28 at O0, −3 at O2 — where protoorder's argument casts and
`void *` parameters split a few expressions that used to fold; single-def/single-read temporaries
go 974 → 1,009 for the same reason. The phantom-`rdx` shape is **11 → 9** (ls's two are gone;
fmt's 2 and du's 7 remain).

**`fmt::main` is still the named bad case.** By default `sub_3700` is called with one, two and
three arguments and two `// rdx` locals are declared and never written. `--option argclobber on`
still turns it into 2/2/2 with no phantom locals (`final-c/fmt-main-c.c` vs
`final-c/fmt-main-c-argclobber.c`), and #689 is the PR that would make that rule the default —
it is open with review CHANGES.

### C.4 Goal 3 — structs, and the layout regression

`structscore --all` on the same eight builds, round B vs round C:

| opt | binary | TRex mean 0-6 B → **C** | mean 0-5 | `is_c_struct` passed | `c_primitive` passed | params typed as a struct / GT | layout F1 (filler counted) |
|---|---|---|---|---|---|---|---|
| O0 | fmt | 4.032 → **4.370** | 3.642 → 3.935 | 277/293 → 305/325 | 161/228 → 180/256 | 4 → 4 / 34 | 0.052 → 0.052 |
| O0 | ls | 3.684 → **4.178** | 3.369 → 3.831 | 1139/1260 → 1336/1518 | 586/773 → 644/914 | 32 → 40 / 168 | 0.130 → 0.108 |
| O0 | sort | 3.715 → **4.005** | 3.403 → 3.657 | 925/1036 → 997/1153 | 471/640 → 526/716 | 35 → 36 / 162 | 0.162 → 0.138 |
| O0 | du | 3.790 → **4.049** | 3.477 → 3.714 | 918/1026 → 970/1145 | 449/611 → 480/659 | 34 → 38 / 174 | 0.125 → 0.088 |
| O2 | fmt | 1.848 → **2.183** | 1.675 → 1.950 | 128/138 → 155/169 | 73/109 → 98/136 | 2 → 2 / 23 | 0.039 → 0.039 |
| O2 | ls | 1.504 → **1.733** | 1.383 → 1.589 | 361/457 → 437/551 | 195/289 → 231/355 | 24 → 24 / 79 | 0.253 → 0.182 |
| O2 | sort | 1.717 → **1.983** | 1.563 → 1.782 | 330/372 → 386/449 | 187/264 → 243/321 | 22 → 21 / 95 | 0.181 → 0.138 |
| O2 | du | 1.746 → **1.916** | 1.588 → 1.736 | 329/376 → 359/433 | 186/270 → 212/299 | 29 → 28 / 100 | 0.197 → 0.097 |

Pooled TRex mean: O0 3.750 → **4.108**, O2 1.660 → **1.893**; 193 of the 835 GT pointer-to-struct
parameters are typed as a struct (182 in round B), and structdedup (#675) cuts the synthesized
names on these eight builds from 188 to **161**.

Layout, however, went the wrong way. Claimed-field precision against DWARF, fields only
(`docs/features/structsynth/layoutscore.py`, the round-B instrument unchanged): **0.8945 → 0.5520**,
F1 0.1376 → 0.1021; filler counted, 0.7816 → 0.4710.

**It is protoorder, not structdedup.** With `--option protoorder off` on the same eight builds the
pooled filler-counted precision is **0.7993** (F1 **0.1642**) — better than round B's 0.7816 /
0.1525, which is structdedup's gain showing through. The per-parameter diff
(`final-c/layoutdiff.py`, du O2) says what happens: 23 parameters lose true fields, and they lose
them by being given *another function's* record.

```
hash_rehash(arg 0):     GT hash_table 10 fields | protoorder off -> struct_20, 10 fields, 10 true
                                                | default        -> struct_9,  11 fields,  5 true
quotearg_n_options(arg 3): GT quoting_options 5  | off -> struct_27, 5 fields, 5 true
                                                | default        -> struct_0,  5 fields,  1 true
```

The callee's recovered pointer type wins the vote at the call site even where the caller measured
the record itself, so the parameter ends up named after whatever record the callee saw through
that pointer. `type_match` cannot see that: it scores the spelling `struct_N *`, not the fields, so
the change that gains the most there (+119 perfect on its own base) is the one that loses the most
here. The fix belongs in protoorder — a caller that measured fields of its own should keep its
layout, and the callee's vote should only supply the *kind*. Until it lands, goal 3 reads "shape
recovered, record identity not reliable at a call site". Per-binary numbers:
`final-c/layout-ablation.md`.

**Round D correction.** The paragraph above is wrong about the mechanism. A caller's own
`structsynth` layout already wins -- it type-locks the parameter after the vote -- and du -O2
installs the same 36 layouts in both arms, 32 of them field-identical and the other 4 differing
only in a field's type spelling. What moved is the JOIN: this instrument takes parameter types
from `decompile-all` and the layouts those names refer to from the `.h` of `decompile-project`,
and `decompile-project` kept an address-order schedule while `decompile-all` decompiles callees
first, so the two ledgers numbered `struct_N` differently (du -O2: 1 of 30 names agreed by
default, 29 of 30 with `protoorder off`). With the export on the same order, pooled precision is
0.8709 fields-only / 0.7836 filler-counted and F1 0.1678 / 0.1882, above round B on F1 and
recall; the 0.024 that fields-only sits under round B is the 176 extra parameters the default
types as struct pointers at all. `docs/features/projectorder/analysis.md`.

### C.5 The decbench#93 counterfactual

Replay of the same rows under the crediting rule the user asked decbench for (a struct pointer
matches a GT pointer-to-struct whatever it is named; primitive pointers still have to match
exactly) — `final-c/credit93.py`:

| | perfect | credited perfect | TP added | mean → credited mean |
|---|---:|---:|---:|---|
| baseline | 848 | 848 | 0 | .2645 → .2645 |
| round B | 986 | 1,159 (+173) | +1,287 | .2895 → .3275 |
| **round C** | 1,349 | **1,575 (+226)** | **+1,569** | .3403 → **.3849** |

### C.6 Speed

`kuna decompile-all <bin> --json --max-fn-seconds 120` (decbench's own invocation) on the O2
stripped binaries, the three binaries interleaved with the arm order rotating every round, 11
rounds each, load average 2.5–5.3 on 80 cores. Driver `final-c/speed3.py`, raw samples
`final-c/speed.json` and `final-c/speed-bash2.json`.

| binary | functions | baseline min | round B min | **round C min** | Δ C vs B (min / median) | Δ C vs baseline (min) |
|---|---:|---:|---:|---:|---:|---:|
| coreutils fmt | 151 | 4,098.3 ms | 4,106.1 ms | 4,128.8 ms | +0.55% / −2.15% | +0.74% |
| coreutils ls | 404 | 13,192.7 ms | 13,413.0 ms | 13,500.6 ms | +0.65% / +2.52% | +2.33% |
| coreutils sort | 343 | 13,761.0 ms | 13,966.1 ms | 14,102.0 ms | +0.97% / +0.57% | +2.48% |
| bash (22 rounds) | 2,538 | 83,477.6 ms | 85,688.2 ms | 84,349.1 ms | **−1.56%** / +1.52% | +1.04% |

bash needed the re-run rule. The first 11 rounds put round C at +4.97% over round B — over the
+5% budget's alarm line — so it was re-run at another moment: 11 more rounds give −1.56%, and the
pooled min-of-22 above is that. The +4.97% was the box, not the build; both runs are in the JSON.

So the round costs about 1% over round B on the three coreutils binaries and nothing measurable
on bash, and at most +2.5% over the campaign baseline — inside the +5% budget each PR was
measured against individually (#669 ≤ +4.1%, #673 −4.6…+0.8%, #681 ≤ +1.04%, #675 −1.0…+1.0%).
bash is 1.3 MB, so `--mode auto` resolves to `reliable` there rather than `aggressive`.

### C.7 Every round-C PR and what it measured

| PR | item | default | measured effect |
|---|---|---|---|
| #669 | `protoorder types` — decompile callees first, type call-site arguments from the callee's recovered prototype | `types` (on) | 1,026 → 1,147 perfect on its base; no call gains or loses an argument over 60 binaries; speed ≤ +4.1%. The single largest contributor this round — and the cause of C.4 |
| #690 | `ptrfromuse void` by default | `void` (was off) | +204 onto perfect / +85 improved / 0 worse on the protoorder base; 836 of 4,490 functions change, declarations and casts only |
| #673 | `formatstring static` — resolve the format string at load time | `static` (was off) | 988 → 1,026 perfect, 176 better / 0 worse; 428 calls stop disagreeing with their format; `decompile-all` −4.6…+0.8% |
| #675 | `structsynth` subsumption dedup | part of `structsynth param` | struct names 515 → 471 over twelve builds (188 → 161 here); held-out claimed-field precision 0.8902 → 0.8908; typesweep unchanged |
| #681 | `foldcallretphi` on by default | `on` (was off) | metric-neutral (986 = 986, 0 improved / 0 worse); −14 declarations; worst speed +1.04% |
| #687 | `foldcallret` never folds a call into the right operand of a short-circuit `&&` or `\|\|` | strict fix | 8 of 25,556 functions change, each one a call the binary always makes that was being skipped; `variables[]` identical |
| #686 | a load is not moved past a store that overlaps its bytes | strict fix | 1 of 48,235 functions changes (bash `init_line_structures`); typesweep rows identical in 444/444 slices |
| #688 | `--jobs N` keeps synthesized structs with serial-run names | on with `--jobs` | removes the documented sharded-run limitation from #682; `--jobs 4` speed −3.1…+2.8% |

**Not landed:**

| PR / lane | item | state | measured |
|---|---|---|---|
| #689 | `argclobber` drops a trailing clobber argument only when the callee's recovered prototype says the register is free, default on | open, review CHANGES | 19 functions over 770 stripped ELFs lose one trailing argument, every one landing on the callee's true arity; `type_match` 1,351 = 1,351; worst speed +0.38% |

### C.8 What stays opt-in, and what is open

* **`argclobber`** — #689 is the sound rule (the callee's recovered prototype, not a bounded entry
  walk); until it lands, `fmt::main`'s 1/2/3 call is the default output.
* **`libctypes glibc`, `protoorder lock`, `structdefs`, `indirectonly`, `signedness prefer-signed`,
  `formatstring full`** — unchanged from round B's reasons.
* **protoorder and struct identity (C.4)** — the one regression this round, and the first item for
  a round D.
* **`ptr_char`** — 28.7% against binja's 36.8%, −1,119 TP, still the largest class gap.
* **Register-resident ground truth (39.3% of GT) and restrict-qualified GT (6.7%)** — unchanged
  ceilings; the second is a one-line decbench fix (`_parse_type_die` has no `DW_TAG_restrict_type`
  arm).

## Round D — 2026-09-20

Round C closed with one named regression (per-parameter struct layout precision) and one
default-on flip still open. Three items landed: `argclobber` default-on (#689), the
`decompile-project` order fix (#693) and the `inferfuncentry` narrowing (#694). This section
re-measures `origin/main` `4c7704e0` on the same instruments under the same metric pin, keeping
round C as the previous column, so every number reads baseline → round B → round C → round D.

| | binary | commit |
|---|---|---|
| **round D** | `/home/mahaloz/kwt/_final-d/kuna` (pinned copy) | main `4c7704e0` (2026-09-20) |

Metric pin unchanged (decisions §12): decbench `625e892` extracted read-only, the venv's
editable finder repointed at it by `final-c/pindb.py`.

### Headline

| goal | instrument | round C | round D | reading |
|---|---|---|---|---|
| type_match | typesweep, 444 slices / 10,748 functions | 1,349 perfect (12.55%), mean .3403 | **1,349** (12.55%), mean **.3405** | 13 improved, **0 worse**, none on or off perfect; aggregate 3,657.76 → 3,659.62 |
| vs the other decompilers | canonical replay, each rival on its own functions | 1st of five | **1st**, unchanged | on binja's 10,366 functions kuna is 12.29% / .3429 against binja's 12.28% / .3422 |
| goal 1: primitives | per-GT-class match rate | — | `int_u8` 50.8% → **51.0%** | +15 `unsigned long` and +4 `long`, all #694; every other class is byte-identical |
| goal 2: variables | varcensus, fmt/ls/sort/du O0+O2 | 6,970 declarations, 9 phantom `// rdx` | 6,967, **7** | **`fmt::main` now calls `sub_3700` with 2, 2 and 2 arguments by default** and declares no phantom `rdx` local |
| goal 3: structs | structscore TRex / per-parameter layout precision | 4.108 / 1.893, P **.5520** | 4.108 / 1.893, P **.8709** | the round-C regression is **repaired**; F1 .1021 → **.1678**, above round B's .1376 |
| decbench#93 crediting | replay of the same rows | 1,349 → 1,575 | 1,349 → **1,575** | +1,569 TP, +226 functions onto perfect — unchanged |
| decbench#94 (restrict) | patched copy of the pinned metric | — | 1,349, mean **.3406** | the 4,412 unmatchable GT variables are **98.2% register-only**; fixing the metric is worth 8 functions |
| speed | whole-binary `decompile-all`, interleaved min-of-11 | +0.7…+2.5% vs baseline | −3.1…+2.6% vs baseline | −1.3…+0.9% against round C; no case near the +5% line, so none needed a re-run |

### D.1 type_match

Same 444 slices, same `--baseline-only` invocation at `--workers 12`. Control: `scripts/decbench/`
is byte-identical between round C's `d8b9c0b1` and this round's `4c7704e0`, so the only thing that
differs between the two columns is the kuna binary.

| slice | functions | base | round B | round C | **round D** | mean C → **D** |
|---|---:|---:|---:|---:|---:|---|
| **ALL** | 10,748 | 848 | 986 | 1,349 | **1,349 (12.55%)** | .3403 → **.3405** |
| O0 | 4,286 | 612 | 704 | 895 | **895 (20.88%)** | .5041 → **.5044** |
| O2 | 2,394 | 42 | 53 | 74 | **74 (3.09%)** | .1820 → **.1822** |
| O2-noinline | 4,068 | 194 | 229 | 380 | **380 (9.34%)** | .2609 → **.2610** |

Thirteen functions move and every one of them is #694: `head::copy_fd`, `tail::dump_remainder`,
`tail::file_lines`, `tail::tail_forever`, `head::elide_tail_bytes_pipe`,
`head::elide_tail_lines_seekable` and `seq::seq_fast` at the optimization levels where each is
compiled, all coreutils, all the `MIN (n, BUFSIZ)` shape where `0x2000` is a PIE's `.init`.
Nothing regresses: **0 worse, 0 off perfect**, and no other project or opt level moves at all
(`final-d/report-slices.md`, `final-d/moved.csv`). #689 and #693 are metric-neutral by
construction — one changes call arities, which `type_match` does not score, and the other
changes only `decompile-project`.

`tail` -O2 `dump_remainder`, round C against round D:

```
$ kuna decompile-all <results>/O2/coreutils/stripped/tail --addr 0x6020
# round C
int * sub_6020(char a0,char *a1,unsigned int a2,void *a3)
    v3 = _DT_INIT;
    if (v5 <= (void *)0x2000)
# round D
int * sub_6020(char a0,char *a1,unsigned int a2,unsigned long a3)
    v3 = 0x2000;
    if (v5 <= 0x2000)
```

`BUFSIZ` is `0x2000` and `.init` starts there in a PIE, so the constant was printed as the
function that sits at the address and everything it touched became a pointer.

Measured against the campaign baseline the round now stands at 2,615 functions improved and
8 worse, 502 onto perfect and 1 off.

### D.2 Per ground-truth class

Same classifier (`final/gtclass.py`), same 65,715 GT variables, rivals' columns unchanged
(the results tree and the pinned metric did not move). Only the rows that moved:

| GT class | GT vars | round B | round C | **round D** | ida | binja | ghidra | angr |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `int_u8` | 7,657 | 50.8% (3,888) | 50.8% (3,887) | **51.0% (3,902)** | 42.0% | 51.2% (3,887) | 40.9% | 50.7% |
| `int_s8` | 2,478 | 58.5% (1,450) | 58.4% (1,446) | **58.5% (1,450)** | 48.6% | 58.1% | 46.4% | 56.2% |
| **all GT variables** | 65,715 | 28.0% (18,396) | 30.9% (20,295) | **30.9% (20,314)** | 24.1% | 30.7% (19,801) | 21.5% | 24.5% |

`int_u8` passing binja's 51.2% (3,902 against 3,887 TP) is #694 giving a `uintmax_t` back its
integer type where round C had made it a `void *`; `ptr_void` holds at 17.6% (612), so the
narrowing costs nothing on the class ptrfromuse won. Storage is unchanged: argument 48.9%,
stack 53.2%, register-only 0.0%.

The rivals' replay, each decompiler paired with kuna on exactly the functions that decompiler
scored (`final/finalsweep.py rivals`, the round-B replay unchanged — neither the results tree nor the
pinned metric moved):

| decompiler (functions it scored) | rival: perfect % / mean | kuna round C | **kuna round D** |
|---|---|---|---|
| **binja** (10,366) | 12.28% / .3422 | 12.29% / .3427 | **12.29% / .3429** |
| **ida** (10,273) | 8.33% / .2682 | 12.60% / .3464 | **12.60% / .3466** |
| **angr** (10,502) | 8.27% / .2652 | 12.75% / .3439 | **12.75% / .3441** |
| **ghidra** (10,673) | 6.84% / .2367 | 12.55% / .3416 | **12.55% / .3418** |

kuna stays 1st of five on perfect % and on mean. binja still leads at O2 (3.43% against kuna's
2.95% on its own functions) and kuna leads every other cell.

### D.3 Goal 2 — variables, and `fmt::main`

| opt | binary | declarations (C → **D**) | `[16]` blobs | never-written locals |
|---|---|---|---|---|
| O2 | fmt | 314 → **311** | 4 | 5 → **3** |
| O2 | ls / sort / du | 1,236 / 1,061 / 950 | 10 / 12 / 7 | 54 / 30 / 38 |
| O0 | fmt / ls / sort / du | 271 / 1,132 / 1,083 / 923 | 4 / 18 / 6 / 5 | 9 / 31 / 44 / 39 |
| **total** | | **6,970 → 6,967** | **66** | **250 → 248** |

`--option argclobber off` reproduces round C's numbers exactly on all eight builds, so the
whole delta is #689 and all of it is in `fmt::main`. The phantom-`rdx` shape — a never-written
local carrying a `// rdx` storage comment — goes 11 → 11 → 9 → **7** (the seven left are du -O2
and are not the clobbered-argument shape).

**`fmt::main` is fixed by default.** Round C's named bad case called `sub_3700` with one, two
and three arguments and declared two `// rdx` locals nothing wrote:

```
$ F=<results>/O2/coreutils/stripped/fmt
$ M="sed -n /^int main(/,/^}/p"
$ kuna decompile-all $F --option argclobber off | $M | grep -E 'rdx|sub_3700'
  unsigned long v11; // rdx
  unsigned long v9; // rdx
          v12 &= sub_3700(v6);
        v3 = sub_3700(stdin,v7);
    v12 = sub_3700(stdin,"-",v10);

$ kuna decompile-all $F | $M | grep -E 'rdx|sub_3700'        # the default
          v9 &= sub_3700(v6,v7);
        v3 = sub_3700(stdin,v7);
    v9 = sub_3700(stdin,"-");
```

All three sites land on the callee's DWARF arity of two and both phantom locals are gone. The
rule needs the callee's own recovered prototype, which `protoorder` parks only when the callee
is decompiled first, so it is **inert under `--addr`, `--functions`, `--jobs N`,
`decompile-project`, `decompile-graph`, a single-function `kuna decompile` and
`--option protoorder off`** — measuring this function with `kuna decompile-all fmt --addr 0x26a0`
reproduces round C's output exactly and says nothing about the default.

### D.4 Goal 3 — the layout regression is repaired

`structscore --all` on the same eight builds scores the TRex ladder from `decompile-all`, which
#693 leaves byte-identical: TRex mean pooled O0 **4.108** and O2 **1.893**, every step count and
both per-binary means equal to round C's to four decimals. What moves is the layout join.

| arm | fields only | filler counted | F1 (fields / filler) | recall |
|---|---|---|---|---|
| round B (before protoorder) | 0.8945 | 0.7816 (798/1,021) | 0.1376 / 0.1525 | 0.0746 |
| round C (the join defect) | 0.5520 | 0.4710 (609/1,293) | 0.1021 / 0.1135 | 0.0562 |
| **round D** | **0.8709** (877/1,007) | **0.7836** (1,010/1,289) | **0.1678 / 0.1882** | **0.0929** |
| round D, `--option protoorder off` | 0.9001 (748/831) | 0.7993 (864/1,081) | 0.1456 / 0.1642 | 0.0792 |

`final-c/layoutdiff.py` on du -O2: parameters where the default claims fewer true fields than the
`protoorder off` arm, round C **23** → round D **0**, with 29 parameters typed as a struct in both
arms. Per-binary layout F1 rises on six of the eight builds (ls -O0 0.108 → 0.229, du -O2 0.097 →
0.209, ls -O2 0.182 → 0.301) and on none does it fall; parameters typed as a struct across the
eight go 193 → 206 of 835.

Round C attributed the drop to protoorder overruling a caller's measured layout. That was wrong,
and `docs/features/projectorder/analysis.md` says why: a caller's own layout already wins (structsynth
type-locks the parameter after the vote, and du -O2 installs the same 36 records in both arms). The
instrument reads parameter types from `decompile-all` and the layouts those names refer to from
`decompile-project`'s header, and the two surfaces numbered `struct_N` in different orders — 1 of 30
names agreed by default, 29 of 30 with `protoorder off`. The fields-only 0.024 that still sits under
round B is the 176 extra claimed fields the default contributes (1,007 against 831), from parameters
it types as a struct pointer where `protoorder off` types nothing at all; recall and F1 are both
above round B.

### D.5 The two counterfactuals

**decbench#93** (a struct pointer credits against a GT pointer-to-struct, `final-c/credit93.py`) is
unchanged from round C, as it must be — the rows are the same:

| | perfect | credited perfect | TP added | mean → credited |
|---|---:|---:|---:|---|
| baseline | 848 | 848 | 0 | .2645 → .2645 |
| round B | 986 | 1,159 (+173) | +1,287 | .2895 → .3275 |
| round C | 1,349 | 1,575 (+226) | +1,569 | .3403 → .3849 |
| **round D** | 1,349 | **1,575 (+226)** | +1,569 | .3405 → **.3850** |

**decbench#94** (`_parse_type_die` has no `DW_TAG_restrict_type` arm, so `char *restrict` arrives as
the GT type `void`) is new this round, and the answer is that the bug costs kuna almost nothing.
A copy of the pinned checkout with the tag added to the `const`/`volatile` arm — one line, the
checkout itself untouched — re-scores all 444 slices:

| metric | perfect | mean | aggregate | improved | worse |
|---|---:|---:|---:|---:|---:|
| pinned `625e892` | 1,349 | .3405 | 3,659.62 | — | — |
| **+ `DW_TAG_restrict_type`** | **1,349** | **.3406** | 3,660.46 | **8** | **0** |

4,412 GT variables carry the artifact (2,951 `ptr_char`, 1,221 `ptr_struct`, 240 `ptr_void`) and
none of them can be matched under the pinned metric. But **4,334 of the 4,412 — 98.2% — are
register-only**, which no decompiler in the corpus scores at all (best: binja 1.4%); only 31 are
arguments and 47 are stack slots. The "6.7% of ground truth is unmatchable" ceiling round C
recorded is really a 0.1% one once it is intersected with the register ceiling below it. The
fix is still worth making upstream — it is one line and it makes the class honest — but it is not
a lever on this corpus.

### D.6 Speed

`kuna decompile-all <bin> --json --max-fn-seconds 120` (decbench's own invocation) on the O2
stripped binaries, the four binaries run one arm after another with the arm order rotating every
round, 11 rounds each, load average 3.4–7.1 on 80 cores. Driver `final-d/speed4.py`, raw samples
`final-d/speed.json`.

| binary | functions | baseline min | round B min | round C min | **round D min** | Δ D vs C (min / median) | Δ D vs baseline (min) |
|---|---:|---:|---:|---:|---:|---:|---:|
| coreutils fmt | 151 | 4,171.3 ms | 4,099.4 ms | 4,158.0 ms | **4,160.1 ms** | +0.05% / +0.67% | −0.27% |
| coreutils ls | 404 | 14,075.0 ms | 13,788.6 ms | 13,525.1 ms | **13,639.9 ms** | +0.85% / +1.56% | −3.09% |
| coreutils sort | 343 | 14,025.6 ms | 14,325.1 ms | 14,589.6 ms | **14,396.3 ms** | −1.32% / +1.82% | +2.64% |
| bash | 2,538 | 85,019.6 ms | 87,257.9 ms | 86,055.5 ms | **86,141.8 ms** | +0.10% / +2.23% | +1.32% |

Nothing in the round costs more than 1.4% either way and no case reached the +5% re-run line, so
none was re-run. Against the campaign baseline the four binaries sit between −3.1% and +2.6%:
the whole campaign's default-on work, five days of it, is inside the box's own variance. Medians
put round D 0.7–2.2% above round C on all four and disagree with min on sign only for sort; min
is the statistic the campaign has reported throughout. bash is 1.3 MB, so `--mode auto` resolves
to `reliable` there rather than `aggressive`.

### D.7 Every round-D PR and what it measured

| PR | item | default | measured effect |
|---|---|---|---|
| #689 | `argclobber` drops a trailing clobber argument only when the callee's recovered prototype says the register is free | `on` (was off) | 19 functions over 770 stripped ELFs each lose one trailing argument, every one landing on the callee's true arity against the unstripped twin; `type_match` unmoved (1,351 = 1,351 on its base — the metric cannot see a call's arity); varcensus −3 declarations, −2 phantom `rdx`; worst speed +0.38% |
| #693 | `decompile-project` takes `decompile-all`'s callee-first order, so one `struct_N` means one record | part of `protoorder types` | `decompile-all` byte-identical; the export's own output changes for 824 of 2,918 functions and then agrees with `decompile-all` 2,918/2,918 (before 2,094); layout precision .5520 → .8709 |
| #694 | `inferfuncentry` lifts its guard only where the constant is used as an address | strict fix | +13 functions improved / 0 worse on the 444 slices; `int_u8` +15 TP; 18 of 6,882 functions change over 16 builds |

**Not landed:**

| PR | item | state | measured on this base |
|---|---|---|---|
| #695 | `charptr` — recover `char *` from what the program does with a pointer, plus a strict loader fix (a constant in a dynamic-loader table stopped printing as a string) | open, review CHANGES; default **off** | 1,349 → **1,353**, aggregate +4.93, 14 improved / 6 down, none off perfect. The flip to on fails `make test-cli` (six probes pin the old spellings) |
| #692 | `libcstructs` — seven more named libc aggregates (`obstack`, `spwd`, `utmpx`, `utmp`, `re_pattern_buffer`, `lconv`, `statfs`) and ~75 more slots | open, review CHANGES | 1,349 → **1,353**, mean .3405 → .3419, 91 improved / 1 worse, `ptr_struct` 422 → **496**; two known regressions of one shape (a named sub-object splits a frame slot) |

### D.8 What stays opt-in, and the ranked levers left

* **`libctypes glibc`, `protoorder lock`, `structdefs`, `indirectonly`, `signedness prefer-signed`,
  `formatstring full`** — unchanged from round B's reasons. `argclobber` is no longer on this list.
* **Ranked next levers**, by TP on the table above:
  1. **`ptr_char`** — 28.7% against binja's 36.8%, **−1,119 TP**, still the largest single gap and
     14,645 GT variables wide. #695 is the lever and it is worth +4 perfect today; the class is where
     the next hundred perfects are.
  2. **`ptr_struct`** — 3.0% (422 of 14,252). #692 takes it to 496 by naming seven more libc records.
     Above that sits the 10,559 program-defined names (`hash_entry`, `fileinfo`, …) that no stripped
     binary carries and no decompiler recovers — reachable only under decbench#93's crediting rule,
     which is worth +226 functions today.
  3. **Struct layout recall 0.0929** — precision is back at 0.87, but only 877 of the 9,443
     ground-truth fields are claimed at all, and **nesting F1 is still 0** on every build (a
     synthesized field is never itself a struct pointer).
  4. **`ptr_void` at O2** and the O2 gap generally: 3.09% perfect against 12.55% pooled.
* **Ceilings, unchanged.** Register-resident ground truth is 25,821 of 65,715 GT variables (39.3%) and
  kuna exports none of them; the restrict artifact (D.5) is 98.2% inside that same set; signedness,
  variable count and call arity are worth zero on `type_match` by construction, which is why #689's
  fix to `fmt::main` shows up in varcensus and nowhere else.

## Round E — 2026-09-22

Round D ranked four levers: `ptr_char`, `ptr_struct`, struct layout recall with nesting F1 at 0,
and the O2 gap. Round E put a lane on each. Five PRs of the campaign landed in the window: the two
round D left open (#695 `charptr`, #692 `libcstructs`) and three follow-ups (#704 `charptr` fix,
#705 `structsynth nest`, #706 `libctypes` width and fit rules). Three lanes did not land:
`charptr-on` (the flip was re-evaluated and fails), `structlocals` (built and gated, no PR opened)
and `o2census` (an investigation, no engine change). #698 and #699 belong to another author and
landed in the same window; they are measured with the rest. This section re-measures
`origin/main` `2da619852` on the same instruments under the same metric pin, so every number
reads baseline → round B → round C → round D → round E.

| | binary | commit |
|---|---|---|
| **round E** | `/home/mahaloz/kwt/_final-e/kuna` (pinned copy) | main `2da619852` (2026-09-22, #706) |

Metric pin unchanged (decisions §12): decbench `625e892` extracted read-only, with the venv's
editable finder repointed at it by `final-c/pindb.py`. Control: the round-D binary re-run on this
round's tree reproduces round D's rows exactly, 10,748/10,748 functions, **0 values differ**.
Between round D's `4c7704e0` and `2da619852`, `scripts/decbench/` changes only in
`structscore.py`, where #705 fixed how the header's struct pointers are read (E.4).

### Headline

| goal | instrument | round D | round E | reading |
|---|---|---|---|---|
| type_match | typesweep, 444 slices / 10,748 functions | 1,349 perfect (12.55%), mean .3405 | **1,353** (12.59%), mean **.3415** | 70 improved, 1 worse; 4 onto perfect, 0 off; aggregate 3,659.62 → 3,669.96. All of it is #692 |
| vs the other decompilers | canonical replay, each rival on its own functions | 1st of five | **1st**, unchanged | on binja's 10,366 functions kuna is 12.33% / .3439 against binja's 12.28% / .3422 |
| `ptr_struct` | per-GT-class match rate | 3.0% (422) | **3.3% (474)** | `obstack *` 34, `termios *` 6, `re_pattern_buffer *` 4, one `lconv *` and one `utmp *`; binja 2.8% |
| `ptr_char` | per-GT-class match rate | 28.7% (4,204) | **28.9% (4,232)** | +28, again #692, most of it grep `EGexecute`'s buffer pointers; binja 36.8% |
| goal 2: variables | varcensus, fmt/ls/sort/du O0+O2 | 6,967 declarations, 7 phantom `// rdx` | 6,967, 7 | unchanged; `fmt::main` is byte-identical to round D (2/2/2, no phantom local) |
| goal 3: structs | TRex mean pooled O0 / O2 | 4.108 / 1.893 | **4.129 / 1.915** | up on all eight builds |
| goal 3: layout | per-parameter precision / recall / F1, fields only | .8709 / .0929 / .1678 | .8702 / .0859 / .1563 | the only parameters that move are ones now typed `obstack *`, the exact GT name, whose fields the instrument cannot read (E.4) |
| goal 3: nesting | nesting F1, fixed instrument | 0 (old instrument); **.0036** re-read | **.0036** (3 of 5 claimed, of 1,660) | `nest` adds nothing on these eight builds |
| decbench#93 crediting | replay of the same rows | 1,349 → 1,575 | 1,353 → **1,579** | +226 functions either way; 1,536 TP added, 33 fewer than round D because #692 turned `struct_N *` into the right libc name |
| speed | whole-binary `decompile-all`, interleaved min-of-11 | −3.1…+2.6% vs baseline | **+1.0…+3.3%** vs baseline | +0.3…+1.7% against round D; no case near the +5% line, so none needed a re-run |

### E.1 type_match

Same 444 slices, same `--baseline-only` invocation at `--workers 12`.

| slice | functions | base | round B | round C | round D | **round E** | mean D → **E** |
|---|---:|---:|---:|---:|---:|---:|---|
| **ALL** | 10,748 | 848 | 986 | 1,349 | 1,349 | **1,353 (12.59%)** | .3405 → **.3415** |
| O0 | 4,286 | 612 | 704 | 895 | 895 | **898 (20.95%)** | .5044 → **.5054** |
| O2 | 2,394 | 42 | 53 | 74 | 74 | **74 (3.09%)** | .1822 → **.1830** |
| O2-noinline | 4,068 | 194 | 229 | 380 | 380 | **381 (9.37%)** | .2610 → **.2620** |
| coreutils | 6,422 | 525 | 610 | 877 | 877 | **880** | .3341 → .3352 (45 up, 1 down) |
| grep | 247 | 37 | 39 | 51 | 51 | 51 | .4351 → .4404 (9 up) |
| tar | 1,548 | 94 | 114 | 158 | 158 | 158 | .3516 → .3524 (12 up) |
| shadow | 686 | 20 | 29 | 31 | 31 | **32** | .3001 → .3012 (3 up) |
| findutils | 790 | 31 | 35 | 41 | 41 | 41 | .2200 → .2201 (1 up) |
| bzip2, diffutils, gzip | 1,055 | | | | | unchanged | no function moves |

**Which PR moved it.** Two intermediate commits were built and swept the same way
(`final-e/attribution.json`):

| step | PRs | perfect | aggregate | improved / worse | onto / off perfect |
|---|---|---:|---:|---:|---:|
| round D `4c7704e0` → `69a947074` | #698, #699 (another author), #695 `charptr` (default off, with its strict loader fix), #700–#703 (CLI) | 1,349 | +0.00 | 0 / 0 | 0 / 0 |
| → `eaa19ebbb` | #692 `libcstructs` | **1,353** | **+10.70** | 71 / 1 | 4 / 0 |
| → `2da619852` (round E) | #704 `charptr` fix, #705 `structsynth nest` (opt-in), #706 width and fit rules | 1,353 | −0.36 | 0 / 1 | 0 / 0 |

So the round's whole `type_match` gain is #692 naming seven more libc aggregates, and #706 gives
back 0.36 of it for the soundness rules the post-merge review asked for. Everything else in the
window is metric-neutral by construction: `charptr` and `nest` are off by default, #698 is an ARM
change and #699 is opt-in, and the CLI PRs do not touch `decompile-all --json`.

The four functions that go onto perfect are one variable each, all a libc aggregate:
`stat::do_statfs` -O0 (`char[120]` → `statfs`), `stty::set_speed` -O0 and `stty::display_speed`
-O2-noinline (`unsigned long mode` → `termios *`), `newgrp::find_matching_group` -O0
(`void *gr` → `group *`). The largest single move is grep `EGexecute` at all three levels, where
the newly declared `re_search` family types 12 of the -O0 function's `void *` buffer pointers as
`char *`; two `idx_t` lengths in the same -O0 frame take `char *` with them, so the function nets
+10 of 29.

**The worse functions, both read.** Round D → round E has one:
`pinky::print_long_entry` -O0, 0.4286 → 0.2143. With `pw` typed `passwd *` through the new
declarations, the three values `pw`, `project` and `plan` stop living in stack slots and become
register variables (`passwd *v1; // rax`), so their slots export as `undefined8` filler and decbench
scores three misses. The code is the same statements in the same order; #692's record documents
the row. The second is internal to the window: grep `kwsprep` -O0 gains `obstack *` for the
`__h`/`__o` macro locals at #692 and loses them at #706, because kwset only *starts* with an
obstack. Against round D it is unchanged.

Measured against the campaign baseline the round stands at 2,653 functions improved and 9 worse,
506 onto perfect and 1 off.

### E.2 Per ground-truth class

Same classifier (`final/gtclass.py`), same 65,715 GT variables, rivals' columns unchanged (the
results tree and the pinned metric did not move). Only the rows that moved:

| GT class | GT vars | round C | round D | **round E** | ida | binja | ghidra | angr |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `ptr_struct` | 14,252 | 3.0% (422) | 3.0% (422) | **3.3% (474)** | 2.4% (328) | 2.8% (389) | 2.3% (325) | 2.3% (315) |
| `ptr_char` | 14,645 | 28.7% (4,204) | 28.7% (4,204) | **28.9% (4,232)** | 24.4% | 36.8% (5,323) | 18.3% | 16.4% |
| `struct_val` | 1,769 | 28.1% (497) | 28.1% (497) | **28.5% (505)** | 2.4% | 5.1% | 30.6% (541) | 20.4% |
| `ptr_void` | 3,481 | 17.6% (612) | 17.6% (612) | **17.7% (615)** | 1.6% | 12.8% | 1.3% | 2.4% |
| `int_s4` / `int_s8` | 8,788 / 2,478 | | 4,691 / 1,450 | 4,693 / 1,448 | | | | |
| **all GT variables** | 65,715 | 30.9% (20,295) | 30.9% (20,314) | **31.1% (20,405)** | 24.1% | 30.7% (19,801) | 21.5% | 24.5% |

| storage | GT vars | round D | **round E** | ida | binja |
|---|---:|---:|---:|---:|---:|
| argument | 21,577 | 48.9% | **49.2%** | 40.6% | 48.4% |
| stack | 18,317 | 53.2% | **53.4%** | 38.2% | 51.8% |
| register only | 25,821 | 0.0% | 0.0% | 0.4% | 1.4% |

`ptr_struct` TPs by name, round D → E: `FILE *` 305 → 310, `stat *` 55, **`obstack *` 0 → 34**,
`passwd *` 18 → 17, `tm *` 15, `group *` 11 → 13, `DIR *` 10, **`termios *` 0 → 6**,
**`re_pattern_buffer *` 0 → 4**, `lconv *` and `utmp *` 1 each. `struct_val` gains `statfs` 5 and
`mbstate_t` 3. kuna's lead on `ptr_struct` over binja widens from 33 to 85 TP; `ptr_char` stays the
largest class gap, −1,091 TP to binja.

The rivals' replay, each decompiler paired with kuna on exactly the functions that decompiler
scored:

| decompiler (functions it scored) | rival: perfect % / mean | kuna round D | **kuna round E** |
|---|---|---|---|
| **binja** (10,366) | 12.28% / .3422 | 12.29% / .3429 | **12.33% / .3439** |
| **ida** (10,273) | 8.33% / .2682 | 12.60% / .3466 | **12.64% / .3476** |
| **angr** (10,502) | 8.27% / .2652 | 12.75% / .3441 | **12.79% / .3451** |
| **ghidra** (10,673) | 6.84% / .2367 | 12.55% / .3418 | **12.58% / .3428** |

kuna stays 1st of five on perfect % and on mean. By level, binja still leads kuna on its own
functions at O2 (3.43% / .1868 against 2.95% / .1844) and at O2-noinline (8.52% / .2632 against
8.28% / .2582) — round D's "kuna leads every other cell" was wrong about O2-noinline, where
round D stood at 8.25% / .2571 — and kuna leads at O0 (20.95% / .5053 against 20.36% / .4956) and
every cell against IDA, angr and ghidra.

### E.3 Goal 2 — variables, and `fmt::main`

`final-e/goal2e.py` (round D's driver, both binaries re-run side by side):

| opt | binary | declarations (D → **E**) | `[16]` blobs | never-written locals |
|---|---|---|---|---|
| O2 | fmt / ls / sort / du | 311 / 1,236 / 1,061 → **1,060** / 950 → **949** | 4 / 10 / 12 → **11** / 7 | 3 / 54 → **53** / 30 / 38 |
| O0 | fmt / ls / sort / du | 271 / 1,132 → **1,133** / 1,083 → **1,084** / 923 | 4 / 18 / 6 / 5 | 9 / 31 / 44 / 39 |
| **total** | | **6,967 → 6,967** | **66 → 65** | **248 → 247** |

Net zero declarations: two fewer at O2 and two more at O0. The phantom-`rdx` shape is 7 → 7, all du -O2, not the
clobbered-argument shape. **`fmt::main` is byte-identical to round D** (`final-e/fmt-main-e.c`):
`sub_3700` is called as `(v6,v7)`, `(stdin,v7)` and `(stdin,"-")` and no `// rdx` local is
declared. The one fmt -O2 change is outside `main`: the `fseeko` thunk now carries
`int fseeko(FILE *a0,long a1,int a2)` and its caller forwards all three arguments.

### E.4 Goal 3 — structs

**A measurement fix first.** #705 found that half of round D's "nesting F1 is 0" was the
instrument: `structscore.header_layouts` recorded a member's pointee only when that struct was
defined *earlier* in the header. Both binaries are scored below with the fixed instrument, so the
round-D column is re-read, not copied.

`structscore --all` on the same eight builds (`final-e/structscore-table.md`):

| opt | binary | TRex mean 0-6 D → **E** | `c_primitive` passed | params typed as a struct / GT | layout F1 (filler counted) D → E |
|---|---|---|---|---|---|
| O0 | fmt | 4.3705 → **4.3753** | 180 → 182 / 256 | 4 → 4 / 34 | 0.052 → 0.052 |
| O0 | ls | 4.1775 → **4.2017** | 644/914 → 660/924 | 46 → 42 / 168 | 0.229 → 0.206 |
| O0 | sort | 4.0053 → **4.0444** | 526/716 → 545/729 | 37 → 36 / 162 | 0.179 → 0.173 |
| O0 | du | 4.0488 → **4.0523** | 480/659 → 483/660 | 39 → 39 / 174 | 0.153 → 0.153 |
| O2 | fmt | 2.1825 → **2.1872** | 98 → 100 / 136 | 2 → 2 / 23 | 0.039 → 0.039 |
| O2 | ls | 1.7331 → **1.7575** | 231/355 → 241/365 | 27 → 24 / 79 | 0.301 → 0.250 |
| O2 | sort | 1.9827 → **2.0222** | 243/321 → 256/334 | 22 → 21 / 95 | 0.194 → 0.180 |
| O2 | du | 1.9159 → **1.9201** | 212/299 → 215/300 | 29 → 29 / 100 | 0.209 → 0.209 |

Pooled TRex mean O0 **4.108 → 4.129**, O2 **1.893 → 1.915**, up on all eight builds.

Per-parameter layout against DWARF (`docs/features/structsynth/layoutscore.py`, unchanged):

| arm | fields only | filler counted | F1 (fields / filler) | recall (fields) |
|---|---|---|---|---|
| round B | 0.8945 | 0.7816 | 0.1376 / 0.1525 | 0.0746 |
| round D (re-run on this tree) | 0.8709 (877/1,007) | 0.7836 (1,010/1,289) | 0.1678 / 0.1882 | 0.0929 |
| **round E** | **0.8702** (811/932) | **0.7851** (935/1,191) | **0.1563 / 0.1759** | **0.0859** |

The drop in recall is a join the instrument cannot make, not a lost layout. fmt and du are
identical in both arms. In ls and sort, every parameter that leaves the join is an `obstack *`
parameter that round D typed as a synthesized `struct_N *` (ls -O0's `struct_4`, 7 of its 10
fields right) and round E
types with the exact GT name (ls -O0: `_obstack_newchunk`,
`print_name_with_quoting` arg 2, `push_current_dired_pos`, `quote_name` arg 5). A libc shell has no
fields in the exported header, so `layoutscore` has nothing to score for it; `type_match` credits
all of them. Taking those parameters out of round D gives round E's row exactly (877 − 66 = 811 of
1,007 − 75 = 932). sort
loses `_obstack_newchunk` the same way at both levels: nine parameters in all (4 + 3 in ls, 1 + 1
in sort), which is the whole 206 → 197 drop in parameters typed as a struct, and no other
parameter's layout changes (`final-e/layoutdiffDE.log`).

**Nesting.** Under the fixed instrument the eight builds claim 5 nested struct pointers, 3 of them
right, against 1,660 GT nestings: P .60, R .0018, **F1 .0036** — in round D and round E alike.
`--option structsynth nest` (#705, opt-in) claims the same 5 on these builds (`nestscore.py`, both
arms of the round-E build); #705's own census found its +2 nested fields in tar and grep, and its
14-build figure is 63/70 (F1 .0184) against `param`'s 62/68. Nesting recall is bounded by the rule
#705 kept: a field is nested only when its loaded value is already typed a pointer, and most
pointer fields are dereferenced only at non-zero offsets and typed `long`; relaxing that measured
33 nested fields at 106/134 inner-field precision on #705's builds, not shipped.

### E.5 The decbench#93 counterfactual

| | perfect | credited perfect | TP added | mean → credited |
|---|---:|---:|---:|---|
| baseline | 848 | 848 | 0 | .2645 → .2645 |
| round B | 986 | 1,159 (+173) | +1,287 | .2895 → .3275 |
| round C | 1,349 | 1,575 (+226) | +1,569 | .3403 → .3849 |
| round D | 1,349 | 1,575 (+226) | +1,569 | .3405 → .3850 |
| **round E** | 1,353 | **1,579 (+226)** | +1,536 | .3415 → **.3857** |

Crediting adds 33 fewer TP than in round D, mostly because 36 variables it would have credited
as an anonymous `struct_N *` are now `obstack *` (33) or `re_pattern_buffer *` (3) and score
under the pinned metric already.

### E.6 Speed

`kuna decompile-all <bin> --json --max-fn-seconds 120` (decbench's own invocation) on the O2
stripped binaries: four arms (baseline, round C, round D, round E) run one after another with the
arm order rotating every round, 11 rounds each, load average 2.8–8.6 on 80 cores. Driver
`final-e/speed5.py` (round D's, with round B swapped for round E), raw samples `final-e/speed.json`.

| binary | functions | baseline min | round C min | round D min | **round E min** | Δ E vs D (min / median) | Δ E vs baseline (min) |
|---|---:|---:|---:|---:|---:|---:|---:|
| coreutils fmt | 151 | 4,068.3 ms | 4,089.8 ms | 4,083.8 ms | **4,108.7 ms** | +0.61% / −1.43% | +0.99% |
| coreutils ls | 404 | 13,337.7 ms | 13,700.9 ms | 13,603.1 ms | **13,643.0 ms** | +0.29% / +0.35% | +2.29% |
| coreutils sort | 343 | 14,061.8 ms | 14,268.6 ms | 14,217.5 ms | **14,461.1 ms** | +1.71% / +1.10% | +2.84% |
| bash | 2,538 | 85,398.1 ms | 87,239.2 ms | 87,454.5 ms | **88,187.8 ms** | +0.84% / +1.08% | +3.27% |

Round E costs +0.3% to +1.7% over round D, and no case came near the +5% re-run line, so none
was re-run. Against the campaign baseline the four binaries sit at +1.0% to +3.3%. bash's +3.27% is
the widest; in the same run round D is +2.41% against the baseline, where round D's own run put it
at +1.32%, so a point of that is the box between runs. sort is the one binary where min and median
agree on a cost above 1% (+1.71% / +1.10%); #692 measured its own worst case at +0.98% after a
quiet re-run. bash is 1.3 MB, so `--mode auto` resolves to `reliable` there rather than
`aggressive`.

### E.7 Every round-E PR and what it measured

| PR | item | default | measured effect |
|---|---|---|---|
| #695 | `charptr` — recover `char *` from what the program does with a pointer, plus a strict loader fix (a constant in a dynamic-loader table no longer prints as a string) | `off` | round-E default output unmoved (0/0 on the 444 slices). On its base the option arm was 1,349 → 1,353, 14 up / 6 down |
| #704 | `charptr`: a constant `PTRADD` index is a field offset; `.interp` and ELF notes are not read-only data | fix to an off-by-default option + strict loader fix | default output unmoved. The offset guard now treats a constant `PTRADD` index the way it treats a literal `INT_ADD`; nearly all of the option's earlier gain was the skip-the-first-character idiom, which has that shape, so `charptr on` is now 1,353 → 1,353, 2 up / 4 down, aggregate −0.57 |
| #692 | `libcstructs` — `obstack`, `spwd`, `utmpx`, `utmp`, `re_pattern_buffer`, `lconv`, `statfs` and ~75 more slots | on (part of `libctypes`) | **the whole round**: 1,349 → 1,353 perfect, +10.70 aggregate, 71 improved / 1 worse, `ptr_struct` 422 → 487, `ptr_char` +29, `struct_val` +8 |
| #706 | `libctypes` names a libc struct only on x86-64 ELF against glibc, and declines it for an argument reached outside the struct | strict fix | −0.36 aggregate, 0 improved / 1 worse (grep `kwsprep`), `ptr_struct` 487 → 474; 136 of 199,963 functions change over 663 binaries, no call gains or loses an argument |
| #705 | `structsynth nest` — a record pointer loaded from a field, recursive where it is the same record; plus the `structscore` header fix | `param` (nest opt-in) | default output unmoved; `nest` vs `param`: typesweep 1,349 = 1,349, +2 nested fields on 10 builds, 7 functions respelled over 15 binaries. Not flipped: `--jobs 8` on tar -O2 is 15.9 s → 26.0 s |
| #698, #699 | another author: ARM call arguments spilled through the frame pointer; opt-in stack output arguments | default / opt-in | no function moves on the 444 slices (x86-64 only) |

**Not landed:**

| lane | item | state | measured |
|---|---|---|---|
| `charptr-on` | `charptr` default on | no PR; evaluation on branch `feat/charptr-on` (`docs/features/charptr/next-levers.md` there) | the flip fails two criteria on `ce008ce2b`: `make test-cli` 213/219 (protoorder's `char` pointee exemption turns 8-byte constant stores into byte stores) and the typesweep (1,353 → 1,353, 2 up / 4 down, −0.57). It found the next `char *` lever instead (E.9) |
| `structlocals` | `structsynth locals` — also synthesize a record for a pointer a call returns | built and gated on branch `feat/structlocals`, no PR opened | typesweep 1,353 = 1,353 (0 up / 0 down; 713 scored variables differ only in `struct_N` numbering); record-typed call-defined locals 40 → 128 on 14 builds (105 on a GT struct pointer); new-record precision .839; `make test` 675/675, stages and `test-cli` green, workspace suite 7,377 passed. Metric-neutral here: most such locals are registers or `framelayout` filler slots, which the metric cannot see |
| `o2census` | why kuna is 3.09% perfect at -O2 | investigation, report in `final-e/o2census/` | E.8 |

### E.8 The O2 census — the gap is the ceiling, not the engine

The `o2census` lane re-scored the 296 O2 and O2-noinline slices on round D's build with `gtclass`
extended by each GT variable's provenance (own parameter, own local, lexical block, or a variable
of an inlined callee), its DWARF location kind and its restrict flag, and reproduced round D's
values exactly (6,462 functions, 0 differ). Full report and scripts: `final-e/o2census/`.

Each GT variable is put in a tier. **U** is structurally unreachable under the metric (register-only,
which decbench matches only by name and no decompiler exports, or the restrict-`void` artifact).
**S** is a program-defined struct pointer, which no stripped decompiler can spell and only
decbench#93 would credit. **P** is everything else. A function takes the worst tier of its
variables.

| opt | functions | U | S | P | kuna perfect | perfect / P |
|---|---:|---:|---:|---:|---:|---:|
| O0 | 4,286 | 360 (8.4%) | 1,870 (43.6%) | 2,056 (48.0%) | 894 | 43.5% |
| O2 | 2,394 | **2,055 (85.8%)** | 170 (7.1%) | 169 (7.1%) | 74 | **43.8%** |
| O2-noinline | 4,068 | 2,865 (70.4%) | 480 (11.8%) | 723 (17.8%) | 379 | 52.4% |

On the functions the pinned metric can score as perfect at all, kuna converts 43.8% at O2 and
43.5% at O0. The pinned ceiling at O2 is 169 functions (7.06%); with #93 crediting, 339 (14.2%).
At the variable level, 62.6% of O2's 21,038 GT variables are U, and of the reachable P tier kuna
has 49.7% right.

* **Inlined callees are scored as the outer function's locals.** decbench counts
  `DW_TAG_inlined_subroutine` children as locals of the function they were inlined into. 416 O2
  functions (17.4%) are blocked only by such variables, mostly glibc FORTIFY wrappers (`__fmt` 404,
  `__stream` 123); in 79 of them every own variable is already right. Leaving inlined-callee
  variables out upstream would take O2 **74 → 153** and O2-noinline **380 → 498** with no engine
  change — +197 perfect, more than any engine lever measured this round, and a metric fix like
  #93 and #94.
* **Restrict** is 1,798 O2 GT variables, 98.5% of them register-only (round D's #94 finding again).
* **binja's O2 lead** (79 against 68 perfect on its functions) is 23 wins: `dir_name`, where kuna
  drops a pass-through argument, and register-only GT named `result` or `i` that binja's own
  variable names happen to equal.

The reachable blockers, as functions that would become perfect if only that bucket were fixed
("solo"), O2 / O2-noinline:

| bucket | O2 | O2-noinline | witness | where it fails |
|---|---:|---:|---|---|
| program struct pointer → `void *` (#93 only) | 20 | 143 | chcon `hash_get_n_buckets`: `unsigned long sub_68f0(void *a0)` | structsynth needs two offsets including 0; a one-field getter declines, and nothing flows caller → callee |
| `void *` → `long` / `unsigned long *` | 24 | 104 | ls `xstrcoll_ctime`, chcon `raw_comparator` | the GT comes from a callback contract (`mpsort`, `hash_initialize`), which no rule reads |
| arity short (a parameter missing) | 24 | 51 | gzip `gzip_base_name`: `void sub_d290(void) { sub_dfd0(); }` while the callee is `char * sub_dfd0(char *a0)` | a register forwarded untouched is not a parameter (`AncestorRealistic`); protoorder never moves arity |
| `char *` → integer / `void *` | 17 | 36 | gzip `gzip_error` forwards to gnulib `rpl_fprintf` | format typing only knows named libc families |
| program struct pointer → integer / primitive pointer | 11 | 41 | du `duinfo_init`: `void sub_4a40(unsigned long *a0)` | four contiguous 8-byte fields read as an array |
| 32-bit int → `unsigned long` | 1 | 21 | ls `format_user_width (uid_t u)` | the call site takes the whole register and the callee's `int` vote cannot narrow it |

### E.9 What stays opt-in, and the ranked levers left

* **Opt-in, with reasons.** `charptr` (the flip fails `test-cli` and the typesweep after #704);
  `structsynth nest` (every criterion passes but a sharded run is +63%; the fix is a request-relative
  recipe in `shard.rs`); `libctypes glibc`, `protoorder lock`, `structdefs`, `indirectonly`,
  `signedness prefer-signed`, `formatstring full` — unchanged from round B.
* **Ranked next levers**, by measured effect:
  1. **Let protoorder park the recovered types of recursive functions.** `callee_first_plan`
     gives every self-recursive function and every member of a recursion cycle `park = false`, and
     the `char *` work in coreutils and gnulib bottoms out in recursion (`quotearg_buffer_restyled`
     calls itself), so every forwarding caller above it keeps an integer. An env-gated experiment on
     `feat/charptr-on`: `self` **+36 perfect**, +163 `ptr_char` TP, 136 up / 1 down; `all` (cycles
     too) **+46**, +188, 160 up / 1 down. About nine times the best `charptr` ever measured
     (+4 perfect), and a few lines behind an option value. Needs the corpus diff, speed and a stage test.
  2. **Type a `framelayout` filler slot from the value stored into it, pointer types only.** 805 of
     the 1,625 `char *` variables only binja gets right are -O0 slots kuna exports as `undefined8`
     while its own C body types the value `char *`; 3,018 stack GT variables across all classes land
     on a filler slot and miss. Levers 1 + 2 together would bring `ptr_char` to at most ~5,230 TP
     against binja's 5,323. `pinky::print_long_entry` (E.1) is this class too.
  3. **Upstream metric fixes, worth more than any engine change.** Inlined-callee variables:
     O2 74 → 153, O2-noinline 380 → 498 (E.8). decbench#93 crediting: +226 perfect.
  4. **The O2 reachable blockers** in E.8's table: callback-contract prototypes (comparators and
     signal handlers share one rule: a sink's function-pointer parameter types its target), one-field
     struct getters, pass-through arity, local printf wrappers.
  5. **Smaller:** variadic parameter numbering (52 binja-only `char *` arguments sit behind eight
     register-save slots), 133 libc imports with a `char *` parameter missing from the signature
     table, a request-relative `--jobs` recipe to make `nest` a default candidate, and landing
     `structlocals`.
* **Ceilings, unchanged.** Register-resident ground truth is 25,821 of 65,715 GT variables (39.3%)
  and kuna exports none of them; at O2 that ceiling alone makes 85.8% of functions unreachable.
  Signedness, variable count and call arity are worth zero on `type_match` by construction.

## Round F — 2026-09-23

Round E ranked five levers. Round F built the top two and the two O2 blockers under them: `protoorder`
for recursive components (#712), a `framelayout` filler slot typed from the pointer stored into it
(#710), records for call-returned pointers (#709) and pass-through arity (#708). All four landed;
`calleevote` (#711) is open with review CHANGES. This section re-measures `origin/main` `810b7dc86`
on the same instruments under the same metric pin, so every number reads
baseline → round B → round C → round D → round E → round F.

| | binary | commit |
|---|---|---|
| **round F** | `/home/mahaloz/kwt/_final-f/kuna` (pinned copy) | main `810b7dc86` (2026-09-23, #708) |

Metric pin unchanged (decisions §12): decbench `625e892` extracted read-only, with the venv's editable
finder repointed at it by `final-c/pindb.py`. Two controls. `scripts/decbench/`,
`docs/features/structsynth/layoutscore.py`, `docs/features/structnest/nestscore.py` and
`scripts/pipeline/varcensus.py` are **byte-identical** between round E's `2da619852` and this round's
`810b7dc86`, so no instrument moved; and the round-E binary re-run on this tree reproduces round E's
rows exactly — 10,748/10,748 functions, **0 values differ**.

### Headline

| goal | instrument | round E | round F | reading |
|---|---|---|---|---|
| type_match | typesweep, 444 slices / 10,748 functions | 1,353 perfect (12.59%), mean .3415 | **1,522** (14.16%), mean **.3553** | 581 improved, 6 worse; 170 onto perfect, 1 off; aggregate 3,669.96 → 3,818.77 |
| vs the other decompilers | canonical replay, each rival on its own functions | 1st of five | **1st**, and O0 is no longer close | on binja's 10,366 functions kuna is 13.95% / .3582 against binja's 12.28% / .3422; at O0 **24.82% against 20.36%** |
| `ptr_char` | per-GT-class match rate | 28.9% (4,232) | **34.6% (5,060)** | round E's largest gap closes from −1,091 TP to −263 against binja's 36.8% (5,323); +653 of the +828 are stack slots |
| `ptr_struct` | per-GT-class match rate | 3.3% (474) | **4.0% (568)** | +94, all stack slots; binja 2.8% (389) |
| storage | stack / argument match rate | 53.4% / 49.2% | **57.6% / 50.0%** | +761 stack TP and +175 argument TP; register-only stays 0.0% |
| goal 2: variables | varcensus, fmt/ls/sort/du O0+O2 | 6,967 declarations, 65 `[16]` blobs, 7 phantom `// rdx` | 6,975, 65, 7 | +8 declarations, all the new `struct_N *` locals; `fmt::main` is byte-identical to round E (2/2/2, no phantom local) |
| goal 3: structs | TRex mean pooled O0 / O2 | 4.129 / 1.915 | **4.420 / 1.918** | the largest TRex move of the campaign, on all four O0 builds |
| goal 3: layout | per-parameter precision / recall / F1, fields only | .8702 / .0859 / .1563 | **.8710 / .0865 / .1574** | two more `ls` parameters typed as a record; nothing else moves |
| goal 3: nesting | nesting F1 | .0036 (3 of 5 claimed, of 1,660) | .0036, unchanged | `structsynth nest` still claims the same 5 on these eight builds |
| decbench#93 crediting | replay of the same rows | 1,353 → 1,579 (+226) | 1,522 → **1,781 (+259)** | +1,779 TP; mean .3553 → **.4038** |
| speed | whole-binary `decompile-all`, interleaved min-of-11 | +1.0…+3.3% vs baseline | SPEED_HEADLINE | SPEED_NOTE |

### F.1 type_match

Same 444 slices, same `--baseline-only` invocation at `--workers 12`.

| slice | functions | base | round B | round C | round D | round E | **round F** | mean E → **F** |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| **ALL** | 10,748 | 848 | 986 | 1,349 | 1,349 | 1,353 | **1,522 (14.16%)** | .3415 → **.3553** |
| O0 | 4,286 | 612 | 704 | 895 | 895 | 898 | **1,065 (24.85%)** | .5054 → **.5391** |
| O2 | 2,394 | 42 | 53 | 74 | 74 | 74 | **74 (3.09%)** | .1830 → **.1834** |
| O2-noinline | 4,068 | 194 | 229 | 380 | 380 | 381 | **383 (9.41%)** | .2620 → **.2628** |
| shadow | 686 | 20 | 29 | 31 | 31 | 32 | **86** | .3012 → .3545 (108 up) |
| tar | 1,548 | 94 | 114 | 158 | 158 | 158 | **192** | .3524 → .3733 (98 up) |
| coreutils | 6,422 | 525 | 610 | 877 | 877 | 880 | **938** | .3352 → .3441 (282 up, 4 down) |
| findutils | 790 | 31 | 35 | 41 | 41 | 41 | **51** | .2201 → .2358 (37 up) |
| diffutils | 420 | 43 | 52 | 58 | 58 | 58 | **63** | .4325 → .4445 (32 up, 1 down) |
| gzip / bzip2 / grep | 882 | | | | | 184 | **192** | 24 up, 1 down |

The round is an **O0 round**: 167 of the 169 new perfects are at O0, and the whole of it is the two
levers round E ranked first and second.

**Which PR moved it**, measured on this tree by ablating each option in turn (`final-f/sweep-*`):

| PR | option arm | perfect | improved / worse | onto / off perfect |
|---|---|---:|---:|---:|
| #710 `slotptr` | `off` → on (default) | 1,399 → **1,522** (+123) | 425 / 5 | 124 / 1 |
| #712 `protoscc` | `protoorder types` → `cycles` (default) | 1,472 → **1,522** (+50) | 171 / 1 | 50 / 0 |
| #709 `structlocals` | `structsynth param` → `locals` (default) | 1,522 → **1,522** | 0 / 0 | 0 / 0 |
| #708 `passthrough` | `off` (default) → `on` | 1,522 → **1,546** (+24) | 128 / 6 | 24 / 0 |

Taking `slotptr` out leaves #712 and #709 worth +46 together (1,353 → 1,399, 162 up / 1 down), so the
two levers reinforce: a slot typed from its stored pointer is worth more once the recursive callee has
stated what that pointer is. `structsynth locals` is exactly metric-neutral here — its records are call-defined
locals, which live in registers or in slots the metric reads through `slotptr` anyway.

A `slotptr` win, runnable against the unpatched tree (`shadow::faillog::print`, GT `struct passwd *pwent`):

```
$ F=<results>/O0/shadow/stripped/faillog
$ kuna decompile-all $F --json --option slotptr off | jq '.functions[]|select(.address==0x2be8).variables[]|"\(.name) \(.type)"'
"local_20 undefined8"  "local_18 undefined8"  "local_10 undefined8"  "local_8 undefined8"
$ kuna decompile-all $F --json | jq ...            # the default
"local_20 undefined8"  "local_18 undefined8"  "local_10 passwd *"   "local_8 undefined8"
```

A `protoscc` win is an argument instead: `mv::emit_verbose(char *src, char *dst, char *backup_dst_name)`
was `unsigned long, unsigned long, long` and is all three `char *`, because the `char *` work in coreutils
bottoms out in recursion.

**The six worse functions, all read.** Five are `slotptr` and one shape:

| function | GT | round E | round F |
|---|---|---|---|
| `shred::dorewind` -O0 | `__off_t offset` | `undefined8` | `int *` |
| `diff::scan_char_literal` -O0 | `long int digits` | `undefined8` | `char *` |
| `tail::tail_lines` -O0 | `__off_t end_pos` | `undefined8` | `int *` |
| `grep::main` -O2 | `idx_t newkeycc` | `undefined8` | `long *` |
| `split::main` -O2 | `long int to_write` | `undefined8` | `char *` |

The pinned metric credits an exported `undefined8` against an 8-byte integer, so a filler slot kuna says
nothing about scores for free; once `slotptr` gives it the type of the value stored into it, a wrong body
type becomes a visible miss. In `shred::dorewind` the body is `v1 = (int *)lseek(fd,0,0)` — `lseek`'s
`long` return is merged with `__errno_location()`'s `int *` two statements later, which is a pre-existing
defect `slotptr` only makes legible. The sixth is #712: `ginstall::install_file_in_file` -O2, where
`to_relname` (`char *`) takes `stat *` from a caller's vote.

Measured against the campaign baseline the round stands at **3,016 functions improved and 11 worse, 675
onto perfect and 1 off**. Per project × opt is in `final-f/report-slices.md`; every moved function is in
`final-f/moved.csv`.

### F.2 Per ground-truth class

Same classifier (`final/gtclass.py`), same 65,715 GT variables, rivals' columns unchanged (neither the
results tree nor the pinned metric moved). Only the rows that moved, plus the two ceilings:

| GT class | GT vars | round D | round E | **round F** | ida | binja | ghidra | angr |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `ptr_char` | 14,645 | 28.7% (4,204) | 28.9% (4,232) | **34.6% (5,060)** | 24.4% (3,543) | **36.8% (5,323)** | 18.3% | 16.4% |
| `ptr_struct` | 14,252 | 3.0% (422) | 3.3% (474) | **4.0% (568)** | 2.4% (328) | 2.8% (389) | 2.3% | 2.3% |
| `ptr_void` | 3,481 | 17.6% (612) | 17.7% (615) | **17.8% (621)** | 1.6% | 12.8% | 1.3% | 2.4% |
| `ptr_prim` | 1,242 | 46.8% (581) | 46.8% (581) | **47.8% (594)** | 30.7% | 51.7% (607) | 32.0% | 37.1% |
| `ptr_ptr` | 1,834 | 28.5% (523) | 28.5% (523) | **28.6% (525)** | 38.3% (670) | 30.8% | 1.5% | 5.6% |
| `int_u8` / `int_s8` | 7,657 / 2,478 | 3,902 / 1,450 | 3,902 / 1,448 | 3,899 / 1,444 | | | | |
| `funcptr` / `ptr_union` | 525 / 143 | 0 | 0 | **0** | 0 | 0 | 0 | 0 |
| **all GT variables** | 65,715 | 30.9% (20,314) | 31.1% (20,405) | **32.5% (21,341)** | 24.1% | 30.7% (19,801) | 21.5% | 24.5% |

| storage | GT vars | round E | **round F** | ida | binja |
|---|---:|---:|---:|---:|---:|
| argument | 21,577 | 49.2% | **50.0%** | 40.6% | 48.4% |
| stack | 18,317 | 53.4% | **57.6%** | 38.2% | 51.8% |
| register only | 25,821 | 0.0% | 0.0% | 0.4% | 1.4% |

`ptr_char` gains 828 TP, 653 of them stack slots — round E's lever 2 predicted "805 of the 1,625 `char *`
variables only binja gets right are -O0 slots kuna exports as `undefined8`", and that is what `slotptr`
collected. `ptr_struct` TPs by name, round E → F: `FILE *` 310 → 337, `obstack *` 34 → 58, `passwd *`
17 → 34, `group *` 13 → 23, `tm *` 15 → 19, `DIR *` 10 → 14, `dirent *` 2 → 5, `lconv *` 1 → 3,
`spwd *` 0 → 2; `stat *` 55 and `termios *` 6 are unchanged. The nine-point `ptr_char` gap to binja is
now 2.2 points, down from 7.9.

The rivals' replay, each decompiler paired with kuna on exactly the functions that decompiler scored:

| decompiler (functions it scored) | rival: perfect % / mean | kuna round E | **kuna round F** |
|---|---|---|---|
| **binja** (10,366) | 12.28% / .3422 | 12.33% / .3439 | **13.95% / .3582** |
| **ida** (10,273) | 8.33% / .2682 | 12.64% / .3476 | **14.28% / .3621** |
| **angr** (10,502) | 8.27% / .2652 | 12.79% / .3451 | **14.40% / .3592** |
| **ghidra** (10,673) | 6.84% / .2367 | 12.58% / .3428 | **14.17% / .3567** |

kuna stays 1st of five and its O0 lead over binja goes from 0.6 points to **4.5** (24.82% / .5389 against
20.36% / .4956 on binja's own functions). binja still leads at O2 (3.43% / .1868 against 2.95% / .1847)
and at O2-noinline (8.52% / .2632 against 8.33% / .2591) — both cells `passthrough` would take (F.7).

### F.3 Goal 2 — variables, and `fmt::main`

`final-f/goal2f.py` (round E's driver, both binaries re-run side by side):

| opt | binary | declarations (E → **F**) | `[16]` blobs | never-written locals |
|---|---|---|---|---|
| O2 | fmt / ls / sort / du | 311 / 1,236 → **1,239** / 1,060 / 949 → **950** | 4 / 10 / 11 / 7 | 3 / 53 / 30 / 38 |
| O0 | fmt / ls / sort / du | 271 / 1,133 → **1,135** / 1,084 → **1,085** / 923 → **924** | 4 / 18 / 6 / 5 | 9 / 31 / 44 / 39 |
| **total** | | **6,967 → 6,975** | **65 = 65** | **247 = 247** |

Eight more declarations over 2,918 functions (+0.11%), all the extra `struct_N *` locals #709 declares
for a pointer a call returns; `ls` -O0 `sub_1c68b(0x10)` now gives `struct_6 *v1` with `v1->field_0x0`
and `v1->field_0x8` where round E had `unsigned long *v1` and `v1[1]`. The phantom-`rdx` shape is
**7 → 7**, all du -O2 and none the clobbered-argument shape, and **`fmt::main` is byte-identical to
round E** (`final-f/fmt-main-f.c`): `sub_3700` is called as `(v6,v7)`, `(stdin,v7)` and `(stdin,"-")`
and no `// rdx` local is declared.

### F.4 Goal 3 — structs

`structscore --all` on the same eight builds, same instrument (`final-f/structscore-table.md`):

| opt | binary | TRex mean 0-6 E → **F** | `c_primitive` passed | params typed as a struct / GT | layout F1 (filler) |
|---|---|---|---|---|---|
| O0 | fmt | 4.3753 → **4.8571** | 182/256 → 218/295 | 4 / 34 | 0.052 |
| O0 | ls | 4.2017 → **4.4691** | 660/924 → 745/1,016 | 42 → 44 / 168 | 0.206 → 0.210 |
| O0 | sort | 4.0444 → **4.3172** | 545/729 → 613/799 | 36 / 162 | 0.173 |
| O0 | du | 4.0523 → **4.3368** | 483/660 → 545/730 | 39 / 174 | 0.153 |
| O2 | fmt / sort | 2.1872 / 2.0222, unchanged | unchanged | 2 / 23, 21 / 95 | 0.039 / 0.180 |
| O2 | ls | 1.7575 → **1.7606** | 241/365 → 242/366 | 24 → 25 / 79 | 0.250 → 0.254 |
| O2 | du | 1.9201 → **1.9286** | 215/300 → 217/302 | 29 / 100 | 0.209 |

Pooled TRex mean O0 **4.129 → 4.420**, O2 **1.915 → 1.918**. The O0 move is the largest of the campaign
and both levers pay: with `--option slotptr off` the same build scores 4.2404 (E 4.1288), so #712 and
#709 are worth +0.112 and `slotptr` the remaining +0.179. The `c_primitive` step is where it lands —
1,870 of 2,569 reached at round E, 2,121 of 2,840 now: a slot that used to arrive as `undefined8` is
reached by the ladder at all.

Per-parameter layout against DWARF (`docs/features/structsynth/layoutscore.py`, unchanged):

| arm | fields only | filler counted | F1 (fields / filler) | recall (fields) |
|---|---|---|---|---|
| round D | 0.8709 (877/1,007) | 0.7836 | 0.1678 / 0.1882 | 0.0929 |
| round E (re-run on this tree) | 0.8702 (811/932) | 0.7851 (935/1,191) | 0.1563 / 0.1759 | 0.0859 |
| **round F** | **0.8710** (817/938) | **0.7861** (941/1,197) | **0.1574 / 0.1769** | **0.0865** |

The round-E binary reproduces round E's published row to four decimals. Only `ls` moves, by three
parameters (`final-f/layoutdiffEF.log`): `print_color_indicator` arg 0 and `quote_name` arg 3 at -O0, and
`quote_name` arg 3 at -O2, each an untyped parameter that #712's cycle vote now types `struct_2 *` with
2 of its 2 claimed fields right. Nothing regresses. **Nesting** is 3 of 5 claimed against 1,660 GT
nestings — P .60, R .0018, F1 **.0036** — identical to round E, and `--option structsynth nest` claims
the same 5 on these builds.

### F.5 The decbench#93 counterfactual

| | perfect | credited perfect | TP added | mean → credited |
|---|---:|---:|---:|---|
| baseline | 848 | 848 | 0 | .2645 → .2645 |
| round B | 986 | 1,159 (+173) | +1,287 | .2895 → .3275 |
| round C / D | 1,349 | 1,575 (+226) | +1,569 | .3403 → .3849 |
| round E | 1,353 | 1,579 (+226) | +1,536 | .3415 → .3857 |
| **round F** | 1,522 | **1,781 (+259)** | **+1,779** | .3553 → **.4038** |

Crediting is worth 33 more functions than in round E and 243 more TP: the `struct_N *` records #709
puts on call-returned pointers and #710 propagates into their slots are, by construction, exactly the
anonymous struct pointers the rule would credit.

### F.6 Speed

SPEED_SECTION

### F.7 Every round-F PR and what it measured

| PR | item | default | measured effect |
|---|---|---|---|
| #710 | `slotptr` — a `framelayout` filler stack slot takes the declared type of the pointer stored into it, when every store resolves and agrees, the type is a committed pointer of the slot's width, and nothing touches the slot at another width | `on` | **the round's biggest lever**: 1,399 → 1,522 perfect on this tree, 425 up / 5 down, `ptr_char` +653 stack TP, `ptr_struct` +94, stack storage 53.4% → 57.6%. P-code and C output are byte-identical — only `variables[]` and the `; stack:` comments move |
| #712 | `protoorder cycles` — the members of a recursive component state the parameter types they recovered, decompiled once each in a depth-first order over the cycle's own calls | `cycles` (was `types`) | 1,472 → 1,522 perfect, 171 up / 1 down (`ginstall::install_file_in_file`); argument TP 49.2% → 50.0%; three `ls` parameters gain a scored layout |
| #709 | `structsynth locals` — a pointer a call returns and reads at two or more constant offsets is declared `struct_N *`, under five rules about the value | `locals` (was `param`) | `type_match` exactly neutral (1,522 = 1,522, 0 up / 0 down); +8 declarations over 2,918 functions; TRex and decbench#93 crediting both rise because the records are real |
| #708 | `passthrough` — a register a function hands untouched to a callee becomes a parameter of the caller and an argument at the call, when the callee's recovered prototype has a parameter there and its body reads it | **`off`** | option on: 1,522 → 1,546 perfect, 128 up / 6 down, and it is **the only round-F lever that moves O2** (74 → 81) **and O2-noinline** (383 → 400). Off by default because it adds arguments the metric cannot check: 454 of 506 gained parameters are confirmed by DWARF over 26 binaries and 0 contradicted, but nothing proves the remaining 52 |

**Not landed:**

| PR | item | state | measured on its base |
|---|---|---|---|
| #711 | `calleevote` — a parameter only read at one fixed offset, or only forwarded, takes the type every caller passes; `fields` turns its lone field into a record | open, review CHANGES (round 4); default would be `fields` | on `3a42d864b`: 1,522 → **1,575** perfect, 264 up / 3 down (gnulib `xmemdup`), credited 1,771 → 2,128. Speed inside the budget on fmt/ls/sort/bash (+0.4…+1.0%) but the second decompile costs +11.5% on kmod -O2-noinline, +9.0% on crontab and +8.1% on dpkg-divert |

### F.8 What stays opt-in, and the ranked levers left

* **Opt-in, with reasons.** `passthrough` (F.7 — arity the metric cannot check); `charptr` (the flip
  fails `test-cli` and the typesweep after #704); `structsynth nest` (every criterion passes but a
  sharded run is +63%); `libctypes glibc`, `protoorder lock`, `structdefs`, `indirectonly`,
  `signedness prefer-signed`, `formatstring full` — unchanged from round B.
* **Ranked next levers**, by measured effect:
  1. **Land `calleevote` (#711)** — +53 perfect and +357 credited on its base, the largest measured
     unlanded item, and the only one that also attacks the O2 one-field getter E.8 named. Its blocker is
     the redo cost on binaries outside the speed set, not accuracy.
  2. **Flip `passthrough` on, or find the proof it needs** — +24 perfect, and unlike everything else this
     round it moves O2 (74 → 81) and O2-noinline (383 → 400). What is missing is a check on the 52 of 506
     gained parameters DWARF does not confirm.
  3. **Upstream metric fixes, still worth more than any engine change.** Inlined-callee variables:
     O2 74 → 153, O2-noinline 380 → 498 (E.8). decbench#93 crediting: **+259** perfect now.
  4. **`ptr_char`'s last 263 TP to binja** — the class gap is 2.2 points, down from 7.9. What is left is
     the variadic register-save slots (52 arguments), the 133 libc imports with a `char *` parameter
     missing from the signature table, and register-only `char *`, which nothing can reach.
  5. **Struct layout recall .0865 and nesting F1 .0036** — precision is .87 and stable across three
     rounds, but 8,626 of 9,443 ground-truth fields are still never claimed, and a synthesized field is
     almost never itself a struct pointer.
* **Ceilings, unchanged.** Register-resident ground truth is 25,821 of 65,715 GT variables (39.3%) and
  kuna exports none of them; at O2 that ceiling alone makes 85.8% of functions unreachable (E.8).
  Signedness, variable count and call arity are worth zero on `type_match` by construction — which is why
  #708, the round's only arity change, has to be judged on DWARF rather than on this page.

## Reproduce

Tools are in `final/` (paths are this machine's; each is a thin driver over the repo's own instruments).

```bash
export KUNA_DECBENCH_RESULTS=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11 DECBENCH_NO_CACHE=1
P="--project coreutils --project grep --project gzip --project diffutils --project bzip2 --project findutils --project tar --project shadow --opt O0 --opt O2 --opt O2-noinline"
# headline + per-class decisions (one run per binary), then the canonical report
KUNA_BIN=<kuna> PYTHONPATH=final ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 24 --out <dir>
~/.virtualenvs/decbench/bin/python -m scripts.decbench.typesweep --report-only --out <dir>
# rivals (read-only replay of the tree's stored variables)
PYTHONPATH=final ~/.virtualenvs/decbench/bin/python -m finalsweep rivals $P --rival ida --rival binja --rival ghidra --rival angr --out <dir>
python3 final/analyze.py            # tables above
python3 final/goal2.py              # varcensus + never-written locals
KUNA_BIN=<kuna> ~/.virtualenvs/decbench/bin/python -m scripts.decbench.structscore <R>/{O0,O2}/coreutils/stripped/{fmt,ls,sort,du} --all
python3 final/speed.py 11           # interleaved whole-binary timing
```

Round C reruns the same drivers from `final-c/` with the metric pin in front of them
(`DECBENCH_PIN` points at a `git archive` of decbench `625e892`):

```bash
export DECBENCH_PIN=<extracted 625e892 tree>     # final-c/pindb.py repoints the editable finder
KUNA_BIN=<kuna> PYTHONPATH=final-c python -m finalsweep kuna $P --workers 12 --out <dir>
python3 final-c/analyze3.py                      # baseline / round B / round C, classes, rivals
python3 final-c/credit93.py base=<rows> roundB=<rows> roundC=<rows>
python3 final-c/goal2c.py                        # varcensus + argclobber arm
DECBENCH_PIN=... python final-c/ssrun.py <bin> --all --out <json>      # structscore
DECBENCH_PIN=... python final-c/layoutrun.py                           # layout precision
DECBENCH_PIN=... python final-c/layoutdiff.py <bin>                    # per-parameter, default vs protoorder off
python3 final-c/speed3.py 11 <speed.json>
```

Round D uses `final-d/` the same way, with two additions — the four-arm timing driver and the
decbench#94 counterfactual, which runs the whole sweep a second time against a patched COPY of the
pinned checkout (`final-d/decbench94-restrict.patch`; the checkout itself is never edited):

```bash
cp -a <625e892 tree> <tree>-r94 && patch -p1 -d <tree>-r94 < final-d/decbench94-restrict.patch
for PIN in <625e892 tree> <tree>-r94; do
  DECBENCH_PIN=$PIN KUNA_BIN=<kuna> PYTHONPATH=final-c python -m finalsweep kuna $P --workers 12 --out <dir>
done
python3 final-d/analyze4.py       # base / B / C / D, classes, rivals, the #94 replay
python3 final-d/moved4.py         # moved.csv + report-slices.md
python3 final-d/goal2d.py         # varcensus, default vs `argclobber off`
python3 final-d/sstable.py        # structscore C vs D
python3 final-d/speed4.py 11 <speed.json> fmt,ls,sort,bash
```

Round E uses `final-e/` the same way. The two attribution builds (`bisect.sh`, `sweep-bis.sh`) are
swept exactly like the round binary, and the round-D binary is re-run on the new tree as the control:

```bash
DECBENCH_PIN=<625e892 tree> KUNA_BIN=<kuna> PYTHONPATH=final-c python -m finalsweep kuna $P --workers 12 --out <dir>
python3 final-e/analyze5.py       # base / B / C / D / E (+ the D control), classes, rivals
python3 final-e/moved5.py         # moved.csv + report-slices.md
python3 final-c/credit93.py base=<rows> roundB=<rows> roundC=<rows> roundD=<rows> roundE=<rows>
python3 final-e/goal2e.py         # varcensus, round D vs round E
bash final-e/ss.sh && python3 final-e/sstable5.py   # structscore D vs E, fixed header instrument
bash final-e/layout.sh            # layout P/R (E, D control) + nestscore param vs nest
python3 final-e/layoutdiffDE.py <bin>               # per-parameter layout, round D vs round E
python3 final-e/speed5.py 11 <speed.json> fmt,ls,sort,bash
```

The O2 census re-scores the O2 and O2-noinline slices with provenance-tagged ground truth
(`final-e/o2census/run.sh`, then `census.py`, `ceiling.py`, `levers.py`, `rivals.py`).

Round F uses `final-f/` the same way. It adds per-option arms on the final binary in place of round E's
bisect builds (`finalsweepopt.py` reads `SWEEP_OPTS="<name> <value>"` and runs the whole sweep with that
flip), and a `structscore` arm with the same flip:

```bash
DECBENCH_PIN=<625e892 tree> KUNA_BIN=<kuna> PYTHONPATH=final-f python -m finalsweep kuna $P --workers 12 --out <dir>
bash final-f/sweep-arms.sh && bash final-f/sweep-arm2.sh   # slotptr off, passthrough on, structsynth param, protoorder types
python3 final-f/analyze6.py       # base / B / C / D / E / F (+ the E control), classes, rivals
python3 final-f/moved6.py         # moved.csv + report-slices.md
python3 final-c/credit93.py base=<rows> ... roundF=<rows>
python3 final-f/goal2f.py         # varcensus, round E vs round F
bash final-f/ss.sh && python3 final-f/sstable6.py   # structscore E vs F
bash final-f/ss-ablate.sh         # the same eight builds with `--option slotptr off`
bash final-f/layout.sh            # layout P/R (F, E control) + nestscore param vs nest
python3 final-f/layoutdiffEF.py <bin>               # per-parameter layout, round E vs round F
python3 final-f/speed6.py 11 <speed.json> fmt,ls,sort,bash
```
