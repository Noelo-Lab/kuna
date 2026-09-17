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
diffutils/bzip2/findutils/tar): 58,051 plain-integer local declarations, 6,259
of which carry a name DWARF also knows in the same function at the same width.
Signedness is binary, so a declaration that differs between two values is right
in exactly one of them.

| value | declarations moved | agrees with DWARF | `-O0` | `-O2` | moved *and* DWARF-named | right | wrong |
|---|---|---|---|---|---|---|---|
| `upstream` | 0 | **93.4%** | 97.3% | 71.9% | 0 | — | — |
| `auto` | 575 | **93.5%** | 97.3% | 72.2% | 4 | **4** | **0** |
| `prefer-signed` | 7089 | **98.4%** | 99.0% | 94.8% | 386 | 348 | 38 |
| `prefer-unsigned` | 13650 | **92.9%** | 97.1% | 69.6% | 41 | 4 | 37 |

**The last three columns are the measurement; the first three are context.** The
oracle can only score a declaration kuna names the way DWARF does in the same
function, and those are overwhelmingly stack locals: of the 6,259 comparable
rows, **5,308 are `-O0` and 951 are `-O2`**. The defect this option exists for is
an `-O2` register local, which the oracle usually cannot see at all.

| value | checkable rows `-O0` / `-O2` | flips it makes there | all flips image-wide | coverage |
|---|---|---|---|---|
| `auto` | 5,308 / 951 | 1 / 3 | 575 | **0.7%** |
| `prefer-signed` | 5,308 / 951 | 134 / 252 | 7,089 | 5.4% |
| `prefer-unsigned` | 5,308 / 951 | 13 / 28 | 13,650 | 0.3% |

Read plainly: **`prefer-signed` is the only value whose fidelity is actually
measured** (386 judged flips, 348 right; `-O2` 71.9% → 94.8%; its 38 wrong flips
are `mode_t` 18, `size_t` 12, `gid_t` 2, `uintmax_t` 2, `unsigned int` 2, `ino_t`
1, `reg_syntax_t` 1, and its 348 right ones are `int` 342, `idx_t` 5, `Idx` 1 —
TRex §5.1's claim reproduced). **`auto`'s fidelity is unmeasured**: four
observations, three of them at `-O2`, out of 575 flips; it is 4-for-4 and its
+0.1pp on the overall rate is noise at that count. **`prefer-unsigned` is refuted
by its own column** (4 of 41 judged flips right; `-O2` 71.9% → 69.6%). Full
method and the cases it cannot see: `analysis.md`.

## The three questions for the reviewer

**1. Is the correctness rule airtight?** The rule is: flip only when *every*
signedness-sensitive reader of the value agrees, where "signedness-sensitive" is
the whitelist enumerated in the module header and in `docs/spec/09-emission.md`,
the walk follows implied (inlined) results so a change of C type cannot escape
into a surrounding expression, and any unclassified reader vetoes the variable
outright. The whitelist is exactly the ops kuna's own cast strategy coerces with
`care_uint_int = true`, which is upstream's own statement of where signedness
carries meaning — plus `INT_LEFT`, which is a stated preference and is discussed
in the open-design list below.

Two things the rule has to survive that the first drafts did not:

* **C's integer promotions.** Below the promotion width nothing on the neutral
  list is neutral, and re-declaring a `short` unsigned turned
  `if ((v2 == v8) || (v8 == -1)) break;` into a dead branch (findutils
  `find -O2` `sub_bac0`). The guard must be read off **the `int` of whoever
  compiles the emitted C, 4 bytes**, not off `TypeFactory::get_size_of_int()`:
  the target cspec says 2 on `avr8gcc`, `avr8egcc`, `TI_MSP430`, `TI_MSP430X`,
  `CR16`, `PIC24` and `x86-16`, and a target-derived guard let the identical
  defect through there. Both are pinned as stage tests
  (`kuna-signedness.xml` on x86-64, `kuna-signedness-int16.xml` on
  `x86:LE:16:Protected Mode`, each with a wider control local that does flip).
* **Transitivity.** `>>`, `/` and `%` constrain their operand *and* print at its
  type, so they record a demand and keep walking; a comparison, a cast-printing
  extension and a carry intrinsic stop the walk because their printed result
  establishes a type of its own.

Measured, not argued, over `fmt`/`ls`/`sort`/`du` at `-O0` and `-O2` (2,918
functions):

| value | functions changed | declaration flips | cast-drop hunks | any other hunk |
|---|---|---|---|---|
| `auto` | 34 | 42 | 46 (+7 lines dropping several casts at once) | **0** |
| `prefer-signed` | 270 | 345 | same 46 (+7) | **0** |
| `prefer-unsigned` | 737 | 894 | same 46 (+7) | **0** |

62 cast tokens leave the text and none is added, under every value. No sub-`int`
declaration moves anywhere in the sweep.

**The correctness oracle is a differential warning diff, not an error diff.** The
previous revision compiled each changed function's before and after text with
`gcc -fsyntax-only -w` and compared the *error* sets, which by construction
cannot observe the only diagnostics this feature can move. Both arms are now
compiled with the same synthesized declarations and
`-Wsign-compare -Wsign-conversion`, and the **warning** sets are diffed
(`.scratch/signwarn.py`):

| value | changed functions | compiled in both arms | `-Wsign-compare` deltas | of which on `< <= > >=` |
|---|---|---|---|---|
| `auto` | 34 | 20 | 0 | **0** |
| `prefer-signed` | 270 | 134 | 5 | **0** |
| `prefer-unsigned` | 737 | 231 | 19 | **0** |

Every one of the 24 deltas is on `==` or `!=` (`if (v3 == strlen(v1))`,
`if (v1 != -1)`, `while (v2 != v5)`), which is benign at a fixed width: both
operands convert to the same width and the bit patterns compared are identical.
**Zero** are on an ordered comparison, which is the number that would say a
comparison changed meaning. The `-Wsign-conversion` deltas are expected by
construction — re-declaring a local changes the conversion at its assignments,
arguments and returns, all same-width and therefore bit-identical — and are
reported separately in `record.json` rather than hidden.

The executable round trip is the third leg, and its coverage is narrow enough
that the number belongs next to it: both texts are built into one program with
stubbed callees and a shared arena, and 2,000 pseudo-random input vectors per
function are compared. `auto`: **8 of 34** functions link and run, 0 mismatches
(22 do not compile out of context, 4 crash the harness).

**2. Is a readability-only option worth the option budget?** It buys **zero**
metric. `extract_variables` exports prototype parameter types, `ScopeLocal`
stack-symbol types and `framelayout` slots — never a HighVariable declaration —
and decbench normalizes `unsigned` away anyway. `structscore --trex`, whose last
scoring step *is* signedness, is byte-identical with the option on (mean 1.7322,
`c_primitive` 58/95 on `fmt -O2`). What it buys is the DWARF agreement in the
table above, plus the cast hunks that stop being written.

**3. Should `auto` become the default?** It clears the repo's mechanical bar —
built with `auto` as the default, `make test` is 675/675 **PARITY OK**, stages is
**PARITY OK**, and the speed delta is inside the +5% budget — but the fidelity
case for it is not there: four observations. If `auto` is ever made the default,
the argument has to be **cast removal and the end of declaration-vs-body
contradictions**, which is visible in every changed function, not a +0.1pp
agreement rate. This PR ships `upstream`; a flip would be its own PR.

## Witnesses (coreutils `fmt` `-O2`, CLI vocabulary)

```
$ kuna decompile-all fmt --addr 0x3060                      # today
  unsigned int v1;
  unsigned int v3;
    if (0 <= (int)v1)
  while (v4 < (int)v1) {

$ kuna decompile-all fmt --addr 0x3060 --option signedness auto
  int v1;
  int v3;
    if (0 <= v1)
  while (v4 < v1) {
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
  41 checkable flips right; `-O2` 71.9% → 69.6%). Shipping
  `upstream|auto|prefer-signed` is a small change if the answer is no.
* **`INT_LEFT` operand 0 is an unsigned demand, and it is a preference, not a
  soundness rule.** `a << k` shifts the same bits either way, and `INT_LEFT`
  takes the *default* `getInputCast` arm (`care_uint_int = false`), so it is not
  part of what makes a flip meaning-preserving. The earlier justification — "`<<`
  on a negative value is UB" — does not separate it from `+ - *`, whose signed
  overflow is UB on the same footing and which are neutral here; that argument is
  withdrawn. What is left is two measured grounds: a value the body ORs and
  shifts left is a bit buffer, which C source spells unsigned (zlib's `deflate`
  bit writer `sub_ce20`/`sub_d3c0` is the corpus case, and without the rule
  `auto` declares its accumulators `int`), and a signed `<<` is the one
  otherwise-neutral operator `-fsanitize=undefined` reports — which it did, on
  that exact function. **Its cost was measured by building both rules and
  diffing: 0 declarations differ under `auto`, 8 under `prefer-signed`, and the
  DWARF-checkable accuracy of every value is identical either way.** So it is
  nearly inert, it points the way C source does where it acts, and dropping it is
  a one-line change. (The review that asked for this item attributed `auto`'s six
  `int -> unsigned int` flips to this rule; they do not come from it — with the
  rule removed all six remain, and they are ordinary unsigned demands:
  `du sub_6310`'s `(unsigned int)v1 >> 8` and `sort sub_6f70`'s
  `(unsigned int)(v3 - 0x30U) <= 9`.)
* **`INT_ZEXT` as a definition is silent** (TRex's `ZeroExtendTgt => None`) while
  as a *reader* it is a hard unsigned demand. Without that asymmetry every value
  a 32-bit x86-64 instruction widens into its 64-bit register is vetoed and the
  option does nothing. A zero-extension describes the *source* operand's type,
  and `int v = *p;` with an `unsigned char *p` is ordinary C.
* **`INT_SEXT`/`INT_2COMP` are silent as definitions too**, which is *not* TRex's
  choice (`SignExtendTgt => Some(true)`). That vote is what declared `fmt::main`'s
  `uintmax_t max` as `long` with no reader evidence at all, so only the
  divide-and-shift family votes now.
* **1- and 2-byte locals are never re-signed, on any target.** The promotion rule
  is a width rule, not an operator rule, so it also gives up the narrow cases
  where the demands really are unanimous. If that is too blunt, the alternative
  is to make every neutral operator promotion-aware.
* **1-byte values would re-sign to `int1`, never `char`** (`get_base_no_char`),
  if the width rule ever allowed it; widening a byte into a character type is a
  different decision with its own option.

## The go/no-go question

What the numbers support, stated as a recommendation so it can be rejected in one
line:

1. **Ship** `signedness` at default `upstream` with values
   `upstream|auto|prefer-signed`.
2. **Drop `prefer-unsigned`** — its own column refutes it.
3. **Do not make `auto` the default on fidelity grounds.** Its measured gain is
   +0.1pp on four observations. If it is flipped later, flip it for the casts.

The question for the user: is a fidelity option that scores exactly zero on
`type_match` worth an option slot, given that the one value with a real
measurement (`prefer-signed`, `-O2` agreement 71.9% → 94.8%, 348 right / 38 wrong
on the flips DWARF can judge) is the one that is *not* proposed as a default?
