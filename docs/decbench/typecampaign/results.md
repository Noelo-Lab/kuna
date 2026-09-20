# Type-recovery campaign — results

The campaign ran 2026-09-16 → 2026-09-19 (plan: `~/.claude/plans/today-we-are-taking-quirky-rabin.md`).
Its three stated goals were (1) get primitives right (int/char/bool), (2) emit fewer variables, (3) recover
structs the way angr does; its yardstick was decbench `type_match` against IDA. This page is the Stage-3
re-measure: the pinned campaign baseline against a fresh release build of `origin/main`, on every
instrument the campaign built, plus what the numbers cannot see. **Round C** (below) repeats every
measurement on `origin/main` `d8b9c0b1` once the eight items round B left open had landed.

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
