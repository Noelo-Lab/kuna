# `charptr` — where the `char *` class gap actually is

Measured on `main` 2e28ece4a (round C), over the 444-slice decbench set
(projects coreutils grep gzip diffutils bzip2 findutils tar shadow, `--opt O0
--opt O2 --opt O2-noinline`), metric pinned to decbench 625e892 the way
`docs/decbench/typecampaign/final-c/pindb.py` pins it.

## The class

Ground-truth `char *` is the largest single class in the corpus: **14,645** of
the 65,715 scored ground-truth variables on these slices (25.9% of the whole
134,209-variable ground truth). kuna matches **4,204 = 28.7%**.

## Split by where the variable lives — this is the whole story

| GT `char *` storage | n | kuna | binja | ida | ghidra |
|---|---|---|---|---|---|
| argument | 4,186 | 2,717 (64.9%) | 2,907 (71.5%) | 1,948 (47.8%) | 1,428 (34.3%) |
| stack local | 3,316 | 1,487 (44.8%) | 2,365 (71.4%) | 1,590 (47.9%) | 1,244 (37.5%) |
| register only | 7,143 | 0 (0.0%) | 51 (0.7%) | 5 (0.1%) | 3 (0.0%) |

(The rivals' own function sets differ by a few dozen slices; their `n` is
4,064/3,311/7,094 for binja.)

Three facts follow, and they decided this feature:

1. **Half the class is unreachable for everyone.** 7,143 of the 14,645 GT
   variables are register-only locals. kuna exports no register locals at all,
   and the best rival scores 0.7% on them.
2. **kuna's argument recovery is nearly saturated.** 64.9% against binja's
   71.5% is 190 variables of headroom on the whole corpus.
3. **Nine tenths of the gap is stack locals** — 878 variables — and it is not
   mainly a typing failure.

## What kuna spells instead, for the misses that have a kuna variable

| kuna spelling | argument misses | stack misses |
|---|---|---|
| `undefined8` | 0 | 932 |
| `unsigned long` | 806 | 216 |
| `void *` | 247 | 177 |
| `long` | 180 | 134 |
| `undefined16` | 0 | 79 |
| `int8` | 28 | 17 |
| `unsigned long *` | 16 | 8 |
| `stat *` | 9 | 9 |
| everything else | 35 | 76 |
| no kuna variable matched | 148 | 181 |

**The 932 `undefined8` stack misses are not a type-inference failure.** They are
`framelayout` filler slots: the frame slot exists in the union `Funcdata`
records per restructure pass, and the value that lived there has been
copy-propagated into a register `HighVariable` that kuna does not export.
Witness, `coreutils -O0 basename::perform_basename`: ground truth `char *name`
at `DW_OP_fbreg -24` (= `rbp-40`), kuna reports `local_28  undefined8  -40`,
and kuna's own body says `char *v2; // rax`. Making `record_frame_slots` prefer
a later pass's *committed* type over the first pass's uncommitted one was tried
and changes nothing — measured, no slot moves — because by the time types are
inferred the symbol is already gone. Closing that bucket is a chapter-06
(variables/merge) item, not a chapter-05 one.

## The evidence census over the population this rule can reach

Collected with `KUNA_CHARPTR_CENSUS=1`, which prints one line per candidate the
rule considers (a function input the prototype model could place a parameter in,
or a Varnode in the stack space, of pointer width, whose current vote is not
already a pointer at something named). Deduplicated per (function, storage);
`coreutils -O0 cat`, 126 functions, 298 candidates:

| what the walk found | candidates |
|---|---|
| (e) no character evidence at all | 240 |
| refused: callee declares a scalar there | 22 |
| refused: an element step or a store wider than a byte | 18 |
| refused: integer arithmetic / shift | 4 |
| refused: compared against a non-zero literal | 2 |
| refused: a load wider than a byte | 1 |
| (c) byte-width dereference only | 8 |
| (a) a declared `char *` callee parameter (all three also had byte evidence) | 3 |
| (b) a `%s` conversion argument | 0 |
| (d) a character-array constant | 0 |

The shape holds across the corpus: **the dominant category is (e) — none of the
above.** A parameter such as `cat`'s `simple_cat(char *buf, size_t bufsize)`
only ever forwards `buf` to `safe_read` and `full_write`, whose own libc
descent is `read(int, void *, size_t)`; nothing in the program says those bytes
are characters, and binja calls it `int64_t` too.

The cases where a declared `char *` *is* reachable are largely cases the
ordinary propagation already handles, because the declaration lands directly on
the argument Varnode. What is left for this rule is the set where a hop stands
in between — an `-O0` spill, a `MULTIEQUAL`, a `p + k` — plus the `void *`
placeholders `ptrfromuse` leaves.

## What that is worth

444-slice sweep, 10,748 functions, `charptr on` against the same build with the
option off:

- perfect `type_match` 1,349 -> 1,353, aggregate 3657.76 -> 3662.69
- 4 onto perfect, 14 improved, **6 worse, 0 off perfect**
- across 2,290 functions of the 8-binary corpus diff: 14 signatures change,
  **0 arities change**, 11 functions gain or lose one local declaration

Three variants were measured and rejected.

- **The declared-callee half alone** (a call argument the callee declares
  `char *`, and nothing else): +0.14 aggregate, 1 function improved, 0 onto
  perfect, over the whole 444 slices. That is why it does not ship as its own
  strength: nearly every reachable declaration already lands on the argument
  Varnode, where ordinary propagation takes it.
- **Counting `protoorder`'s recovered callee vote** as evidence rather than only
  declared prototypes: +0.95 aggregate against +1.78 on the same 812-function
  subset. A recovered `char *` is itself a guess, and the walk would launder it
  into a commitment.
- **Without the fixed-offset guard** (any one-byte dereference counts, wherever
  it sits): +8 onto perfect and 26 improved, but 13 worse instead of 6. Four of
  the extra losses convert a *correct* `void *` into `char *` off a one-byte
  struct field — coreutils `ginstall`'s
  `announce_mkdir(char const *dir, void *options)` reads the `bool` at
  `options + 0x3c`. More metric, less truth; rejected.

The six remaining losses are read individually in `record.json`, and they are
three functions seen in more than one build.

- `od::print_long_double` (O2 and O2-noinline): a *correct* `void *block`
  parameter refined to `char *`.
- `head::elide_tail_bytes_pipe` (O2-noinline): a GT `__off_t`, an integer the
  body also uses as an address.
- `tar::check_compressed_archive` (O0, O2, O2-noinline): the byte at
  `fbreg -41` is GT `_Bool temp`; kuna reports it `undefined1`, which the metric
  credits on width alone, and typing the pointer it is read through knocks the
  byte on to `char`, which the metric then scores exactly and rejects. That is
  the same cost in the other direction: committing a type forfeits the
  width-only free pass a placeholder gets.

One further shape is visible in the corpus diff without being a metric loss:
coreutils `cp` -O0 `sub_f8ce` holds a `NAME_MAX` limit in a slot the body also
steps through by one byte, and the option turns `long v6; v6 = 0xff;` into
`char *v6; v6 = (char *)0xff;`. The emitted C stays equivalent — every widening
and comparison picks up an explicit cast — but it reads worse, and it is the
same shape as `head::elide_tail_bytes_pipe`.
