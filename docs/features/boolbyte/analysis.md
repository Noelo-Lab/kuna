# boolbyte — census and analysis

What a byte that is only ever a truth value is declared as today, how often that
byte has `_Bool` ground truth, and which of kuna's three bool mechanisms is
responsible.

Everything below is measured on the campaign slice set: the decbench
`full_run_address_2026-09-11` tree, projects `coreutils grep gzip diffutils bzip2
findutils tar shadow` at `O0`, `O2` and `O2-noinline` — **444 slices, 10,748
scored functions**, `DECBENCH_NO_CACHE=1`, `KUNA_BIN` a fresh `main` build
(83830e86, which already carries `bytehonest`, so an *uncommitted* byte is
exported `undefined1` and already matches `_Bool`).

## 1. The metric census

`typesweep --baseline-only --dump-decisions` writes one row per ground-truth
variable, carrying the decbench match pass (`arg`, `offset`, `name`, or `miss`),
the verdict, and both type spellings. Over the 444 slices there are 65,377
ground-truth decisions, of which **3,228 have `_Bool` ground truth**.

| GT `_Bool` rows | count |
|---|---|
| matched by argument index | 788 |
| matched by calibrated stack offset | 1,269 |
| unmatchable (register-resident / no location) | 1,171 |
| **matchable (arg + offset)** | **2,057** |
| … of those, already a hit | 928 |
| … of those, a miss | 1,129 |

Of the 1,129 misses, the ones this item can address are those where kuna has
**committed** to a non-bool spelling. A width-only spelling (`undefined1`,
`uint1`, …) already matches `_Bool` through decbench's `_SIZE_SCALARS`, so it is
not a loss:

| kuna's spelling on a missed GT `_Bool` | count |
|---|---|
| `char` | 700 |
| `unsigned char` | 166 |
| `unsigned int` | 89 |
| `unsigned long` / pointers / arrays | 50 |
| width-only (`undefined*`, `u?int[1-8]`) — already matching | 124 |

**The addressable slice is 1,005 matchable committed-non-bool rows, 866 of them
one byte wide** (`char` 700, `unsigned char` 166). By storage class: 668 arguments,
337 calibrated stack slots. By optimisation level: O0 429, O2 247, O2-noinline
329. By project: coreutils 811, tar 64, grep 47, diffutils 28, findutils 28,
shadow 23, gzip 4.

That is 43x the 20-row bar this item was gated on.

## 2. How many of them are actually truth-valued

For each of the 866 one-byte rows, kuna's emitted C for that function was read
and every occurrence of the variable classified (a text heuristic: `if (v)`,
`!v`, `v == 0|1`, `v && …`, `v ^ 1`, `v ? a : b`, `return v` count as truth
uses; parentheses and the `'\x01'`/`'\0'` rendering of the flag constants are
normalised first).

| class | count | share |
|---|---|---|
| **A — every use is a truth test** | **538** | 62% |
| B — every use is a truth test or passing the byte to a call | 116 | 13% |
| C — as A, plus a call result def or a widening return | 21 | 2% |
| D — a genuine non-truth use (arithmetic, indexing, a wider compare) | 191 | 22% |

488 of the 538 class-A rows are **arguments**, which is the match pass that needs
no name or offset calibration.

Class A alone clears the "a mechanism that recovers at least half" bar (538 of
866, 62%). The shipped predicate is exactly class A: B is refused on purpose
(`putchar(c)` must not make `c` a bool because something else tests it) and C is
refused because a call result cannot be proven to be 0/1 from inside the caller.

## 3. Which mechanism is responsible

The three candidate mechanisms from the E1 report:

* **B-i** — `propagate_type_edge` refuses a `TYPE_BOOL` candidate when the
  target's non-zero mask exceeds 1 (`coreaction_infertypes.rs:481-483`).
* **B-ii** — `HighVariable::get_type_representative` orders with
  `type_order_formal`, which de-prioritises `TYPE_BOOL`, so a merged high with
  any non-bool member never declares bool.
* **B-iii** — there is no truth-value seed at all beyond the `booloutput` ops.

15 variables were sampled from classes A/B/C (9 arguments, 6 stack slots, across
all three optimisation levels) and re-decompiled with a throwaway instrumented
build that logs every one-byte Varnode's local type, non-zero mask and
written-ness at `build_localtypes`, plus every firing of the B-i refusal.

**All 15 are B-iii.** Every sampled argument's byte carries local type `char`,
mask `0xff`, `written=false` — there is no bool candidate for B-i to refuse or
B-ii to demote. The sampled stack slots carry `char`/`uint1`/`xunknown1`, four of
the six with mask `0x1` (the value is *provably* 0 or 1 and kuna still says
`char`). B-i does fire in these functions — 3 to 546 times each — but always on
other Varnodes; it is not what blocks these.

The root cause is one line of upstream faithfulness:
`TypeOpEqual::get_input_local` votes `get_base(1, TYPE_INT)` for the value being
compared, and `cache_core_types` fills the one-byte `TYPE_INT` cell with the
ASCII `char` ("Char is preferred over other int types"). So the fold's entire
candidate set for a tested byte is `{char}`.

This corrects the D2 design, which proposed relaxing B-i as the metric-relevant
half: relaxing it is neither necessary (the seed wins the fold on its own —
`SUB_BOOL` 10 beats `SUB_INT_CHAR` 19) nor safe (the refusal is what stops a
seeded bool from travelling along a copy chain into a value that can exceed 1).
The shipped change leaves `:481-483` exactly as upstream wrote it.

## 4. The C-text census (quality)

Independently of the metric, the emitted C of `fmt`, `ls`, `sort` and `du` at O0
and O2 was scanned for declarations spelled `char`/`unsigned char` whose every
use is a truth test.

| slice | char-family local declarations | all uses boolean | … and a real boolean read | `bool` declarations today |
|---|---|---|---|---|
| fmt O0 | 14 | 1 | 1 | 15 |
| fmt O2 | 10 | 1 | 1 | 15 |
| ls O0 | 69 | 8 | 3 | 43 |
| ls O2 | 67 | 4 | 1 | 36 |
| sort O0 | 65 | 8 | 3 | 37 |
| sort O2 | 62 | 3 | 1 | 28 |
| du O0 | 42 | 6 | 2 | 43 |
| du O2 | 42 | 3 | 1 | 36 |
| **total** | **371** | **34** | **13** | 253 |

The quality tail is thin — one to three declarations per binary, in functions
that already declare 15 to 43 bools — because this census looks only at *locals*
in the printed body, and the population the metric census found is dominated by
*parameters*, which do not appear in a local declaration list at all. The two
censuses agree on the shape of the win: it is a parameter-declaration win, not a
body win.

## 5. Result

Measured with the shipped predicate over the same 444 slices
(`typesweep --option boolbyte on`):

* `type_match` PERFECT 959 -> 987 (**+28**), aggregate 3037.05 -> 3115.29 (+78.24)
* moved ONTO perfect 28, moved OFF perfect **0**
* improved but not perfect 438, worsened **0**
* harness control: 10,243 functions with byte-identical `variables` in both arms,
  0 scored differently

See `record.json` for the benchmark block, the speed block and the whole-corpus
before/after classification, and `census-classified.json` for the 866 rows of
section 2 one per line (slice, function, kuna spelling, ground-truth name, class).

## 6. Soundness checks

* **No wider value inherits bool.** Over `ls` -O2, `grep` -O2 and `find` -O0, 54
  exported variables change type to `bool` between the two arms, and all 54 are
  one byte. No pointer and no 2/4/8-byte scalar acquires bool: the `TYPE_BOOL`
  propagation gate, left untouched, is what stops a seeded bool from crossing a
  copy edge into a Varnode whose mask exceeds 1.
* **The lattice still settles.** A throwaway instrumented build counting every
  `set_type_recovery_exceeded` (the 7-pass ceiling, `coreaction_render.rs:2934`)
  over a whole-binary `decompile-all` reports the same count in both arms on all
  four binaries measured: `ls` -O2 6/6, `grep` -O2 10/10, `find` -O0 2/2, `tar`
  -O2 23/23. The extra candidate does not make type propagation oscillate.

## 7. The INDIRECT hole, and the two declines that close it

The first cut of the rule read `CPUI_INDIRECT` as value-preserving identity on
both sides of the walk. It is not. An INDIRECT output is the value *after* the
op it annotates — a call whose callee may have written this storage, or a store
that may alias it — so a truth test on the output is no evidence about the
input, and a proof about the input is no evidence about the output. Two
counterexamples, both reduced to a `gcc -O2` binary anyone can rebuild:

```c
__attribute__((noinline)) void fill(unsigned char *p){ *p = 200; }
__attribute__((noinline)) int  d2(void){ unsigned char c = 0; fill(&c); return c ? 11 : 12; }

__attribute__((noinline)) void show(unsigned char *p){ printf("show=%d\n",(int)*p); }
__attribute__((noinline)) int  f1(unsigned char n){ unsigned char c = n; show(&c); return c ? 11 : 12; }
```

`d2` proves `c == 0` before the call and truth-tests it after; crossing the
INDIRECT joined the two and declared a byte the callee sets to 200 a `bool`.
`f1` is the same hole on a *parameter*: compiling the two emitted arms and
running them side by side, the option-off arm prints `show=200` like the native
binary and the option-on arm prints `show=1`. Both now refuse, and both are
pinned in `tests/stages/kuna-boolbyte.xml` (`callfill`, `callslot`).

Removing the INDIRECT arms was not enough for the second defect the same
premise produced. cronie `crond` -O2 `sub_7a20` reads the first byte of a
131088-byte `fgets` line buffer as its loop terminator, and typing that byte
retyped the whole buffer — `bool v2 [131088]` on the scored `variables[]`
surface, while it was still being handed to `fgets`, `strlen` and `strcmp`
(same shape at `crond` 0x8090 and `crontab` 0x606b / 0x6d1d). The byte there is
a *stack-space function input*: storage the function never writes, whose entry
value is junk. Two declines close it, and both are the rule's own contract
written out:

* A function **input** only carries the "this is what a `_Bool` parameter looks
  like from the inside" argument if it is a parameter at all, so the storage has
  to be somewhere `FuncProto::possible_input_param` admits. This is what closes
  all four cronie sites.
* A Varnode that is **one byte of a mapped symbol bigger than one byte** is not
  a declaration of its own — the declaration that would change is the
  container's. General form of the same rule; it is what keeps the `callfill`
  control's `char v1 [9]` slot a character.

## 8. The SUBPIECE hole on the printer side

A `bool` declaration reaches the printer, and one printer decision had never
seen one. `CastStrategyC::isSubpieceCast` (cast.cc:411-432) lists the
destination metatypes a SUBPIECE may print as a cast — `TYPE_INT`, `TYPE_UINT`,
`TYPE_UNKNOWN`, `TYPE_PTR`, `TYPE_FLOAT` — and `TYPE_BOOL` is not among them,
because upstream never puts a bool there: `TYPE_BOOL` is minted only as a
comparison's own one-byte output, which is never the destination of a
truncation. Left alone the printer falls to the functional arm and emits the raw
`SUB41(x,0)` p-code intrinsic. That is an undeclared identifier and not
compilable C; it is the class the `subright` rule exists to keep out of the
output.

Measured on tar -O2 `sub_41370`, the function this option changes most:

```
off:  v17 = (char)v15;
on :  v41 = SUB41(v15,0);      # before the arm
on :  v41 = (bool)v15;         # after it
```

`kuna_boolbyte::truncation_prints_as_cast`, consulted from
`printc::subpiece_is_cast`, supplies the missing arm for a low-piece truncation
into a `bool`. The rewrite is safe for the same reason the declaration is: the
seed that put `bool` on the destination already required the value reaching it
to carry a non-zero mask of at most 1, so `(bool)x` and the low byte of `x`
agree. It is also strictly a rewrite — the truncation is still printed, just as
the cast it is, which is what the option-off arm prints too (`(char)v15`).

The arm is gated on the option, so with `boolbyte off` the printer is
byte-for-byte what it was. That matters because the class is **pre-existing**: a
comparison alone can make a truncation's destination a `bool` without this rule
at all, and it does on three lines in `tar` -O2 and one each in `ls` -O2 and
`du` -O0. Ungating the arm was measured — over `tar`/`ls`/`grep`/`sort` -O2,
`du` -O0 and `find` -O2 with the option off it changes five lines, all
`SUBnn(x,0)` → `(bool)(x)` — but that is a default-path change to a class this
option did not create, and it belongs in its own PR.

The net effect of the option on the class is therefore negative, not positive.
Whole-binary counts of the intrinsic, option off → on: `tar` -O2 4 → 1, `ls` -O2
1 → 0, `du` -O0 1 → 0, `grep` -O2 0 → 0. Counted over the 100 functions the
option changes anywhere in the sixteen-binary corpus, 7 → 0: the four that lose
one are `du` -O0 `sub_815c`, `find` -O0 `sub_347a5`, `libedit` -O2 `sub_25e80`
and `tar` -O2 `sub_41370`.

`tests/stages/kuna-boolbyte.xml` pins it with `truncjoin`, a stack byte one arm
sets with `setg` and the other with a truncated `x >> 31`: pass 1 (off) asserts
the `SUB41` the printer emits without the arm, pass 2 (on) asserts the cast.
