# `signedness` — proposal

**Status: draft, awaiting go/no-go.** A working prototype is on the branch, with
the measurements below. Nothing here changes default output.

## What is being proposed

A value option `signedness upstream|auto|prefer-signed|prefer-unsigned` (default
`upstream`, byte-identical to not having it) that decides a declared integer
local's signedness at the P9 declaration seam from the operations the body
applies to it, instead of from whichever type-inference vote happened to be most
specific. This is TRex's *type rounding* (USENIX Security 2025, §3.3.5 / §5.1)
reduced to the one decision kuna can make without touching its type lattice.

## The number the decision turns on

Measured against DWARF on 238 unstripped twins (the `compiled/` binaries beside
the stripped images decbench scores, `-O0` and `-O2`, coreutils/grep/gzip/
diffutils/bzip2/findutils/tar): 58,653 plain-integer local declarations, 6,260
of which carry a name DWARF also knows in the same function at the same width.
Signedness is binary, so a declaration that differs between two values is right
in exactly one of them.

| value | declarations moved | agrees with DWARF | `-O0` | `-O2` | moved *and* DWARF-named | right | wrong |
|---|---|---|---|---|---|---|---|
| `upstream` | 0 | **93.4%** | 97.3% | 72.0% | 0 | — | — |
| `auto` | 579 | **93.5%** | 97.3% | 72.3% | 4 | **4** | **0** |
| `prefer-signed` | 7183 | **98.4%** | 99.0% | 94.9% | 386 | 348 | 38 |
| `prefer-unsigned` | 13623 | **92.9%** | 97.1% | 69.6% | 41 | 4 | 37 |

Today's declarations agree with the source 72.0% of the time at `-O2`.
`prefer-signed` takes that to 94.9%; its 38 wrong flips are concentrated in the
types whose operations carry no signedness at all (`mode_t` 18, `size_t` 12),
and its 348 right ones are `int` 342, `idx_t` 5, `Idx` 1 — TRex §5.1's claim
reproduced. `auto` is exact (4/4) but only reaches 4 checkable declarations.
`prefer-unsigned` is refuted by its own column. Full method and the cases the
method cannot see: `analysis.md`.

## The three questions for the reviewer

**1. Is the correctness rule airtight?** The rule is: flip only when *every*
signedness-sensitive reader of the value agrees, where "signedness-sensitive" is
the whitelist enumerated in the module header and in `docs/spec/09-emission.md`,
the walk follows implied (inlined) results so a change of C type cannot escape
into a surrounding expression, and any unclassified reader vetoes the variable
outright. The whitelist coincides with the ops kuna's own cast strategy coerces
with `care_uint_int = true`, which is upstream's own statement of where
signedness carries meaning.

The rule also has to survive C's *integer promotions*, which the first draft did
not: below the promotion width nothing on the neutral list is neutral, and
re-declaring a `short` unsigned turned `if ((v2 == v8) || (v8 == -1)) break;`
into a dead branch (findutils `find -O2` `sub_bac0`). A declaration narrower
than `int` is now never touched, and the reduced bytechunk is the second
function in `tests/stages/kuna-signedness.xml`. `INT_LEFT` is an unsigned demand
for a related reason: `<<` on a negative value is UB, and `gcc -O2
-fsanitize=undefined` caught the first draft creating one in zlib `sub_ce20`.

Measured, not argued, over `fmt`/`ls`/`sort`/`du` at `-O0` and `-O2` (2,918
functions):

| value | functions changed | declaration flips | cast-drop hunks | any other hunk |
|---|---|---|---|---|
| `auto` | 34 | 42 | 53 | **0** |
| `prefer-signed` | 273 | 353 | 53 | **0** |
| `prefer-unsigned` | 732 | 889 | 53 | **0** |

Every changed function was then compiled twice, before and after, *separately*:
the gcc diagnostic set is identical in both arms for **all 1,039** of them
(34 + 273 + 732). The ones that compile clean build into a two-version program
with stubbed callees and a shared memory arena; 2,000 pseudo-random input
vectors per function produce **0** divergences across 10 (`auto`, all 34), 43
(`prefer-signed`, 161 of 273 run) and 12 (`prefer-unsigned`, an 86-function
sample) executable programs. (Two `prefer-signed` functions
report a mismatch and report the *same* mismatch when the upstream text is
compared against itself — they return a real `malloc()` pointer, and the arena
hash is identical in both arms.)

**2. Is a readability-only option worth the option budget?** It buys **zero**
metric. `extract_variables` exports prototype parameter types, `ScopeLocal`
stack-symbol types and `framelayout` slots — never a HighVariable declaration —
and decbench normalizes `unsigned` away anyway. `structscore --trex`, whose last
scoring step *is* signedness, is byte-identical with the option on (mean 1.7322,
`c_primitive` 58/95 on `fmt -O2`). What it buys is the DWARF agreement in the
table above, plus 53 cast hunks that stop being written.

**3. Should `auto` become the default?** It already clears the repo's bar: built
with `auto` as the default on this tree, `make test` is 675/675 **PARITY OK**
and `make test-stages` is 1044/1044 **PARITY OK** — zero assertions move on
either corpus — and the speed delta is **+0.26%** on the minimum of 21
interleaved whole-binary `decompile-all` runs on `fmt -O2` (median −2.34%), well
inside the +5% budget. This PR still ships `upstream`; a flip would be its own
PR. Note that `auto`'s DWARF agreement gain is +0.1pp: it is the *cast removal*
and the removal of declaration-vs-body contradictions that a default flip would
buy, not accuracy.

## Witnesses (coreutils `fmt` `-O2`, CLI vocabulary)

```
$ kuna decompile-all fmt --addr 0x3060                      # today
  unsigned int v1;
  unsigned int v3;
    if (0 <= (int)v1)
    } while (v5 < (int)v3 >> 3);
  while (v4 < (int)v1) {

$ kuna decompile-all fmt --addr 0x3060 --option signedness auto
  int v1;
  int v3;
    if (0 <= v1)
    } while (v5 < v3 >> 3);
  while (v4 < v1) {
```

The 8-byte form is `sub_7130`:

```
  unsigned long v3;                          ->  long v3;
    if (0x80 <= (long)v3) goto label_71ee;   ->    if (0x80 <= v3) goto label_71ee;
```

Three functions that do **not** move are the rule working, and all three were
proposed as witnesses in the design note:

* `sub_74d0` also prints `if (0 <= (long)v28)`, but the same `v28` is read as
  `v28 >> 0x36` — an unsigned shift — so the demands disagree and `auto`
  declines. (The design note expected this one to flip; it should not.)
* `sub_7900`'s `unsigned long v3; if ((int)v3)` is a **width** change: `(int)v3`
  is a `SUBPIECE` rendered as a cast, which is classified `Opaque` here, so the
  option leaves that line alone. Narrowing a declaration is a different decision.
* `fmt::main`'s `unsigned int v12` has no signedness-sensitive reader at all —
  only `^`, `&`, assignments and the `return` — so `auto` leaves it and
  `prefer-signed` declares it `int`.
* At `-O0`, `fmt::main`'s `unsigned long max` no longer moves either. It used to
  be declared `long` purely by an `INT_SEXT` *definition* vote (`movslq %eax,%rdx`
  feeding `xdectoumax`'s third argument) with no reader evidence at all, and
  DWARF spells it `uintmax_t`. Extensions no longer vote as definitions.

## Open design points the reviewer may want to overrule

* **Ship `prefer-unsigned` at all?** It is sound, and it is the only value whose
  own measurement says it makes output *less* faithful than doing nothing (4 of
  41 checkable flips right; `-O2` agreement 72.0% → 69.6%). Shipping
  `upstream|auto|prefer-signed` is a small change if the answer is no.
* **`INT_ZEXT` as a definition is silent** (TRex's `ZeroExtendTgt => None`) while
  as a *reader* it is a hard unsigned demand. Without that asymmetry every value
  a 32-bit x86-64 instruction widens into its 64-bit register is vetoed and the
  option does nothing. A zero-extension describes the *source* operand's type,
  and `int v = *p;` with an `unsigned char *p` is ordinary C.
* **`INT_SEXT`/`INT_2COMP` are silent as definitions too**, which is *not* TRex's
  choice (`SignExtendTgt => Some(true)`). That vote is what declared `fmt::main`'s
  `uintmax_t max` as `long` with no reader evidence at all, so only the
  divide-and-shift family votes now.
* **1- and 2-byte locals are never re-signed.** The promotion rule is a width
  rule, not an operator rule, so it also gives up the narrow cases where the
  demands really are unanimous. If that is too blunt, the alternative is to make
  every neutral operator promotion-aware.
* **1-byte values would re-sign to `int1`, never `char`** (`get_base_no_char`),
  if the width rule ever allowed it; widening a byte into a character type is a
  different decision with its own option.

## The go/no-go question

Ship the option at default `upstream`, with which value set — and is
`prefer-signed`'s 72.0% → 94.9% `-O2` agreement with DWARF (348 right / 38 wrong
on the checkable flips) worth an option slot for a feature that scores exactly
zero on `type_match`?
