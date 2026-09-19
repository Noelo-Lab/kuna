# Type-recovery campaign — results

The campaign ran 2026-09-16 → 2026-09-19 (plan: `~/.claude/plans/today-we-are-taking-quirky-rabin.md`).
Its three stated goals were (1) get primitives right (int/char/bool), (2) emit fewer variables, (3) recover
structs the way angr does; its yardstick was decbench `type_match` against IDA. This page is the Stage-3
re-measure: the pinned campaign baseline against a fresh release build of `origin/main`, on every
instrument the campaign built, plus what the numbers cannot see.

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
| speed | whole-binary `decompile-all`, min-of-11 interleaved | — | SPEED_HEADLINE | SPEED_READING |

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
(behind binja), and stayed 2nd on mean with the gap to binja cut from .071 to .045. The campaign opened on "kuna is last-of-four at O2":
on this slice set it now ties IDA's O2 perfect rate, leads it on O2 mean (.1558 vs .1485), and is within 0.11
points of it on O2-noinline perfect while leading on mean. binja's lead is `ptr_char` and `ptr_void` (below).

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
across the type PRs and were not attributed one by one. kuna was already the leanest declarer of the four (3.33 declarations per function against IDA's 4.07
before the campaign), and `type_match` gives a missing or an extra variable exactly zero, so none of this moves
§1.

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

| opt | binary | TRex mean 0-6 | mean 0-5 | `is_c_struct` passed | `c_primitive` passed | params typed as a struct / GT struct params | layout F1 |
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

SPEED_SECTION

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
  (IDA 328: `FILE` 315, `DIR` 11, `__sigset_t` 2). The
  remaining 2,472 libc-named ones are the reachable pool. structsynth recovers the *shape*, which `type_match`
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
