# `signedness` — proposal

**Status: answered. Shipping at default `auto`, with `prefer-signed` opt-in.**
The measurements below are the ones the decision was taken on; the section "The
decision" at the end records it.

## What is being shipped

A value option `signedness upstream|auto|prefer-signed|prefer-unsigned` that
decides a declared integer local's signedness at the P9 declaration seam from
the operations the body applies to it, instead of from whichever type-inference
vote happened to be most specific. This is TRex's *type rounding* (USENIX
Security 2025, §3.3.5 / §5.1) reduced to the one decision kuna can make without
touching its type lattice. `auto` — flip only on unanimous evidence — is the
default; `upstream` restores what type inference produced, byte for byte.

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
| `auto` *(default)* | 575 | **93.5%** | 97.3% | 72.2% | 4 | **4** | **0** |
| `prefer-signed` | 7,081 | **98.4%** | 99.0% | 94.8% | 386 | 348 | 38 |
| `prefer-unsigned` | 13,636 | **92.9%** | 97.1% | 69.7% | 40 | 4 | 36 |
| *control:* all-signed, no walk | 15,136 | **92.8%** | 98.9% | **59.2%** | 790 | 376 | 414 |

**The last row is the control, and it is the answer to "does the walk earn its
option slot".** `prefer-signed` is `auto` plus one line — "declare an unobserved
value signed" — so the fair question is whether the 700-line walk in front of
that line does anything, or whether declaring *every* eligible local signed would
score the same. It would not. The control keeps the same candidate set, the same
width guard and the same ownership guard and deletes only the operation walk; it
takes `-O2` agreement to **59.2%**, below leaving the declarations alone (71.9%),
and of its 790 judged flips 376 are right and **414 are wrong**. The wrong ones
are **379 `size_t`** locals declared `int`, against `prefer-signed`'s 12. The
unsigned demands the walk collects — an unsigned compare, a `>>`, a `/`, a
`ZEXT` — are exactly what holds `size_t` unsigned, and they are the whole
difference between 98.4% and 92.8%. (Built from the same source through a
measurement-only escape hatch that exists in `.scratch/measbin/kuna` and in no
committed file.)

**The last three columns are the measurement; the first three are context.** The
oracle can only score a declaration kuna names the way DWARF does in the same
function, and those are overwhelmingly stack locals: of the 6,259 comparable
rows, **5,308 are `-O0` and 951 are `-O2`**. The defect this option exists for is
an `-O2` register local, which the oracle usually cannot see at all.

| value | checkable rows `-O0` / `-O2` | flips it makes there | all flips image-wide | coverage |
|---|---|---|---|---|
| `auto` | 5,308 / 951 | 1 / 3 | 575 | **0.7%** |
| `prefer-signed` | 5,308 / 951 | 134 / 252 | 7,081 | 5.5% |
| `prefer-unsigned` | 5,308 / 951 | 13 / 27 | 13,636 | 0.3% |
| *control:* all-signed | 5,308 / 951 | 143 / 647 | 15,136 | 5.2% |

Read plainly: **`prefer-signed` is the only value whose fidelity is actually
measured** (386 judged flips, 348 right; `-O2` 71.9% → 94.8%; its 38 wrong flips
are `mode_t` 18, `size_t` 12, `gid_t` 2, `uintmax_t` 2, `unsigned int` 2, `ino_t`
1, `reg_syntax_t` 1, and its 348 right ones are `int` 342, `idx_t` 5, `Idx` 1 —
TRex §5.1's claim reproduced). **`auto`'s fidelity is unmeasured**: four
observations, three of them at `-O2`, out of 575 flips; it is 4-for-4 and its
+0.1pp on the overall rate is noise at that count. **`prefer-unsigned` is refuted
by its own column** (4 of 40 judged flips right; `-O2` 71.9% → 69.7%). Full
method and the cases it cannot see: `analysis.md`.

## The three questions for the reviewer

**1. Is the correctness rule airtight?** The rule is: flip only when *every*
signedness-sensitive reader of the value agrees, where "signedness-sensitive" is
the whitelist enumerated in the module header and in `docs/spec/09-emission.md`,
the walk follows implied (inlined) results so a change of C type cannot escape
into a surrounding expression, and any unclassified reader vetoes the variable
outright. What the rule rests on is the *neutral* half of that split: `+ - * & |
^ == !=`, unary `~` and `-`, an assignment, a call argument, a `return`, a
stored value, a truncation and a concatenation all produce the same bits under
either declaration at a fixed width.

kuna's own cast strategy draws the same line, which corroborates the split but
does not prove it, and the earlier drafts of this section overstated the
relationship. The ordered comparisons, `/ %`, `>>` and the two extensions pass
`care_uint_int = true` to `CastStrategyC::cast_standard`; the whitelist is a
**superset** of those, because `INT_SCARRY`/`INT_SBORROW`/`INT_CARRY`, a
same-width `CPUI_CAST` and `INT_LEFT` all take the default `get_input_cast` arm,
which passes `care_uint_int = false`. Demanding on the first two anyway is an
over-constraint rather than a gap — a carry intrinsic names its own signedness
and a cast token establishes the type it prints — and it can only ever decline a
flip. `INT_LEFT` is a stated preference and is discussed in the open-design list
below. One op goes the other way: `FLOAT_INT2FLOAT` passes `care_uint_int = true`
when the operand's nonzero mask has its top bit set, and the whitelist does not
demand on it — it is vetoed, which is strictly stronger.

Three things the rule has to survive that the first drafts did not:

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
* **Declaration ownership.** Evidence is gathered per HighVariable, but the
  printer does not write one declaration per high — the composite-Symbol retain,
  `collapse_symbol_decls`, `DeclDedup` and the `local_name_aliases` group
  suppression each fold several highs onto one line. A rounded type reaching
  those decisions could move a collapse key, splitting a declaration that used
  to collapse, or re-sign the one line a *non*-flipped sibling's uses read
  through. The plan is therefore filtered to highs whose name is unique on the
  printer's candidate list before any collapse runs; every collapse pairs
  candidates by name, so a sole-named entry can neither be suppressed by one nor
  absorb a sibling. Cost over the 238-binary corpus: **8 declarations across 7
  binaries** under `prefer-signed` (7,089 → 7,081 flips), 0 under `auto`, and 0
  bytes over the `fmt`/`ls`/`sort`/`du` sweep below. Seven of the eight are the
  case it exists for — a `VariableGroup` whose byte pieces all render one name
  and read through one line (`v1._0_1_ = x->copy_as_regular, …` in `cp`,
  `ginstall`, `mv`, `sort`, `find`, `tar`). The eighth is a conservative
  decline, where the printer's later uniqueness pass would have renamed the
  sibling to `v1_1` anyway.

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
(22 do not compile out of context, 4 crash the harness). `prefer-signed`: 34 of
the first 112 run, 0 mismatches (one apparent DIFFER is `ls sub_1b525`, which
returns a real `malloc()` pointer and reports the same DIFFER when the upstream
text is compared against itself). `prefer-unsigned`: 28 of the first 221 run, 0
mismatches. The two long runs were stopped; the warning oracle above covers all
1,041 changed functions.

**2. Is a readability-only option worth the option budget?** It buys **zero**
metric, and the walk that fills it is not decoration — the control row above says
a one-line "declare it signed" tie-break scores 92.8% where the walk scores
98.4%, and 59.2% where it scores 94.8%. `extract_variables` exports prototype parameter types, `ScopeLocal`
stack-symbol types and `framelayout` slots — never a HighVariable declaration —
and decbench normalizes `unsigned` away anyway. `structscore --trex`, whose last
scoring step *is* signedness, is byte-identical with the option on (mean 1.7322,
`c_primitive` 58/95 on `fmt -O2`). What it buys is the DWARF agreement in the
table above, plus the cast hunks that stop being written.

**3. Should `auto` become the default?** *Answered: yes, and it ships that way.*
It clears the repo's mechanical bar — built with `auto` as the default,
`make test` is 675/675 **PARITY OK** and stages is 1069/1069 **PARITY OK**, and
the speed delta is inside the +5% budget (interleaved min-of-15 whole-binary
`decompile-all` on `fmt`/`ls`/`sort` `-O2`: −0.05% / −2.11% / −4.49%, i.e. below
this box's noise floor; the tightest measurement on a quiet box, min-of-21 on
`fmt` alone, was **+0.17%**). What it is *not* flipped for is the agreement rate:
`auto` buys +0.1pp on four checkable observations. It is flipped for **cast
removal and the end of declaration-vs-body contradictions**, which is visible in
every changed function — over twelve binaries from nine projects at `-O0` and
`-O2`, eight of them x86-64 and four 32-bit ARM firmware images (15,124
functions), the flip writes 376 declaration flips and 421 cast-drop hunks,
removes 443 cast tokens, adds none, and produces **0 hunks of any other kind**.

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
  soundness rule.** `a << k` shifts the same bits either way, so it is not
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

## The decision

The user's answer, and what this PR ships:

1. **Ship** `signedness` with `auto` as the **default**. It re-signs a
   declaration only on unanimous evidence, so the only text it can move is that
   declaration and the casts the new declaration makes into no-ops — 0 of 675
   datatest assertions, 1069/1069 stages PARITY OK, `type_match` 959 → 959
   perfect over 444 slices with a byte-identical `variables[]` in all 10,748
   scored functions, and 0 non-declaration/non-cast hunks over twelve binaries
   on two architectures.
2. **`prefer-signed` stays opt-in.** It is the arm with the fidelity (`-O2`
   agreement 71.9% → 94.8%), and it moves 7,081 declarations image-wide against
   `auto`'s 575 — more than unanimity covers, and more than the datatest corpus
   can absorb as a default. Reach for it when reading `-O2` code.
3. **`prefer-unsigned` is kept as `prefer-signed`'s control**, not recommended,
   and its catalog row says so. It is also what the 16-bit stage test uses to
   prove the width guard is declining rather than inert.
4. **`upstream` restores the declaration type inference produced**, byte for
   byte, which is what to select when diffing against upstream Ghidra or against
   kuna before this option existed.

What was never in question is whether the rounding itself earns the slot.
Dropping the walk and keeping only the tie-break scores **92.8%** overall and
**59.2%** at `-O2` — worse than doing nothing — because it declares 379 `size_t`
locals `int`. The walk is the part that knows not to.
