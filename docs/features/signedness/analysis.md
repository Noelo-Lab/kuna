# `signedness` — analysis

## The opportunity

kuna declares an optimized loop counter `unsigned int` and then casts it back to
`int` at every comparison:

```
$ kuna decompile-all fmt --addr 0x3060
  unsigned int v1;
  ...
  if (0 <= (int)v1)
  while (v4 < (int)v1)
```

Every operation applied to `v1` that cares about signedness is a **signed**
comparison. Nothing in the function reads it as unsigned.

## Why inference says unsigned

`Datatype::type_order` ranks the sub-metatypes
`... SUB_UINT_PLAIN (16) < SUB_INT_PLAIN (17) ...`
(`decompiler/crates/kuna-decomp/src/substrate/dtype.rs`), and lower is *more
specific*. `get_local_type`'s fold keeps the most specific seed
(`p5_types/coreaction_infertypes.rs`), and every propagation edge adopts the
lower sub-metatype, so **one `uint` vote outranks every `int` vote on the same
Varnode** and no later phase re-decides: `HighVariable::get_type_representative`
and `kuna_declhightype` both order with `type_order_formal`, which only demotes
`PARTIALUNION` and `BOOL`.

On `-O2` x86-64 the `uint` votes are structural rather than semantic:

* a 32-bit instruction zero-extends into its 64-bit register, and `INT_ZEXT`'s
  `metaout`/`metain` are both `TYPE_UINT` (`p5_types/typeop.rs`);
* a strength-reduced loop bound is masked and shifted, and `INT_AND`, `INT_OR`,
  `INT_XOR`, `INT_RIGHT`, `INT_DIV` and `INT_REM` all vote `TYPE_UINT` on both
  operands.

Measured over 8 stripped `-O2` binaries (≈6.6 k declarations): `TYPE_INT` 32.9%,
`TYPE_UINT` 23.3%.

## What is worth changing, and what is not

The lattice itself is an upstream transcription that every propagation edge and
every cast decision in `p5_types`/`p9_emit` is calibrated against. Changing it
would be a deliberate divergence with a very wide blast radius. The *declaration*
is not: it is written once, at the end, in the printer, and nothing reads it
back.

TRex (Bosamiya, Woo, Parno, USENIX Security 2025) makes exactly that split. Its
§3.3.5 *type rounding* decides signedness **after** propagation, from the
accumulated set of size-tagged operations a value participates in, and §5.1 adds
a flag that prefers signed for the ambiguous case, "since C programmers write
`int` when signedness does not matter". `signedness` is that decision, applied at
kuna's declaration seam.

## The correctness question

Changing a declaration without changing Varnode types leaves every
`ActionSetCasts` decision in place. Casts that become redundant are harmless. The
wrong-output case is the opposite one: an operation whose C semantics depend on
the declared signedness and for which **no cast was emitted**, because the
Varnode type already agreed.

The signedness-sensitive C constructs are enumerable, and kuna's own cast
strategy already enumerates them: `care_uint_int = true` is passed by exactly
`INT_SLESS`/`INT_SLESSEQUAL`, `INT_LESS`/`INT_LESSEQUAL`, `INT_SDIV`/`INT_SREM`,
`INT_DIV`/`INT_REM`, `INT_SRIGHT`/`INT_RIGHT` and `INT_ZEXT`/`INT_SEXT`
(`p9_emit/coreaction_casts.rs (get_input_cast)`); every other op is coerced with
`care_uint_int = false`, i.e. upstream itself treats `int` and `uint` of one
width as interchangeable there (the one addition is `INT_LEFT`, below). That is
the soundness basis: flip only when every one of those demands agrees.

Three refinements were needed on top:

* **The promotion width — of the reader, not the target.** "Signedness-
  independent at a fixed width" is a precondition, not a turn of phrase. C's
  integer promotions convert every operand narrower than `int` to `int` *before*
  the operator runs, and which extension that is comes off the declaration, so
  below the promotion width nothing on the neutral list is neutral:
  `(short)-1 == -1` is true while `(unsigned short)0xffff == -1` is a constant
  false, and `+ - * & | ^ <<` all diverge the same way. Reduced from findutils
  `find -O2` `sub_bac0`, where re-declaring `short v8` unsigned left
  `if ((v2 == v8) || (v8 == -1)) break;` in place with a dead `break`; the
  reduced bytechunk is the second function in `tests/stages/kuna-signedness.xml`.

  The width that decides it is the `int` of whoever compiles the emitted C, which
  is 4 bytes, **not** `TypeFactory::get_size_of_int()`. Reading it off the target
  was the first fix's own bug: the cspec `<data_organization><integer_size>` is 2
  on `avr8gcc`, `avr8egcc`, `TI_MSP430`, `TI_MSP430X`, `CR16`, `PIC24` and
  `x86-16`, so on those targets a 2-byte local was re-signed and the identical
  `find` defect came straight back — on `x86:LE:16:Protected Mode`, `upstream`
  printed `int2 v1; ... if (v1 != -1)` and `prefer-unsigned` printed
  `uint2 v1; ... if (v1 != -1)`, a constant-true test to any reader whose `int` is
  4 bytes. The floor is now `max(4, get_size_of_int())`, and
  `tests/stages/kuna-signedness-int16.xml` pins it on that architecture (2 of its
  4 assertions fail with the target-derived guard, 4/4 with this one), with a
  4-byte local in the same image that *does* flip so the decline is visibly the
  width rule and not an inactive option.
* **Implied values.** `v + 1 < 0` is one printed expression, so the C type of
  `v + 1` — and therefore whether `< 0` is signed — follows `v`'s declaration
  even though `INT_ADD` is signedness-neutral. The walk follows a neutral op's
  result whenever that result is implied (inlined) and stops when it is a
  declared variable of its own (a same-width conversion is bit-identical).
* **Whitelist, not blacklist.** `PTRADD`'s index slot is the counterexample that
  forces this: `base[v]` really does differ between a signed and an unsigned `v`,
  and `get_input_cast_ptradd` defers that slot to the default `care_uint_int =
  false` body, so no cast is there to save it. Anything not explicitly classified
  vetoes the variable.

`INT_LEFT` is on the demand side, and it is the one row there that is a
**preference rather than a soundness requirement** — the module header says so
too. `a << k` shifts the same bits into the same places under either
declaration, and `INT_LEFT` takes the *default* `getInputCast` arm
(`care_uint_int = false`), so it is not part of the set that makes a flip
meaning-preserving. An earlier revision justified the row with "shifting a
negative value left is undefined in C"; that does not hold up, because signed
overflow of `+ - *` is undefined on exactly the same footing and those are
neutral here. The row is kept on two measured grounds instead:

* a value the body ORs and shifts left is a bit buffer, which C source spells
  unsigned — zlib's `deflate` bit writer (`sub_ce20`, `sub_d3c0`) is the case in
  the corpus, and without the row `auto` declares its accumulators `int`;
* a signed `<<` is the one otherwise-neutral operator `-fsanitize=undefined`
  reports, and it did report it (`runtime error: left shift of negative value`,
  zlib `sub_ce20`'s `v7 = v13 | v7 << (v4 & 0x1f);`).

Its cost was measured directly, by building both rules and diffing the
declarations: **0** declarations differ under `auto` and **8** under
`prefer-signed` over the `fmt`/`ls`/`sort`/`du` `-O0`+`-O2` sweep (all 8
`int` without the row, `unsigned int` with it), and the DWARF-checkable accuracy
of every value is identical either way (`prefer-signed` 386/348/38 both times).
So the row is nearly inert, it points the way C source does where it acts, and
dropping it is a one-line change if a reviewer disagrees. Because it is a
preference and not a demand it also *carries*: the walk continues into whatever
reads the shifted value, so `(v << 3) >> 2` is decided by the `>>`. The same is
true of `>>`, `/` and `%`, whose printed result keeps the operand's type.

Across the whole sweep (and zlib), **no** variable this pass declares signed is
ever the left operand of a `<<` in the emitted text: 29 newly-signed declarations
under `auto`, 332 under `prefer-signed`, 10 in zlib, 0 shiftees
(`.scratch/shiftcheck.py`).

One more, from TRex: `INT_ZEXT` as a **definition** is not unsigned evidence
(TRex's `ZeroExtendTgt => None`). A zero-extension describes the *source*
operand's type; `int v = *p;` with an `unsigned char *p` is ordinary C. On
x86-64 this is the difference between seeing any variable at all and vetoing
every value a 32-bit instruction widened into its 64-bit register. As a
**reader** `INT_ZEXT` is still a hard unsigned demand — that is what decides
which extension the printed C performs.

`INT_SEXT` and `INT_2COMP` are silent as definitions for the same reason, and
that symmetry is *not* TRex's (`SignExtendTgt => Some(true)`). It was measured
into the design: the `INT_SEXT` definition vote is what declared `fmt::main`'s
`uintmax_t max` as `long` with no reader evidence at all. Only the
divide-and-shift family votes as a definition, because only there does the
operator say what kind of number came out.

## Does it flip to the *right* signedness?

Soundness says the emitted C still computes what the binary computes. It does
not say the declaration is the one the programmer wrote — the rule reads the
compiler's instruction selection, and a compiler that proved a `size_t`
non-negative will happily emit a signed compare on it. That is a measurable
question, and DWARF answers it.

**Method.** 238 unstripped binaries (the `compiled/` twins of the stripped
images decbench scores) across coreutils, grep, gzip, diffutils, bzip2,
findutils and tar, at `-O0` and `-O2`. Whole-image `decompile-all` once per
value, then every plain-integer local declaration whose name DWARF also knows
**inside the same function** is compared against the DWARF base type's
encoding. 58,051 plain-integer declarations; 6,510 carry such a name, 6,259 of
them at the same width as the DWARF type. Signedness is binary, so a
declaration that differs between two values is right in exactly one of them.
Harness: `.scratch/acc/{dwarfsign,accuracy}.py` on the branch.

| value | declarations moved | agrees with DWARF | `-O0` | `-O2` | moved *and* DWARF-named | right | wrong |
|---|---|---|---|---|---|---|---|
| `upstream` | 0 | 5848/6259 = **93.4%** | 97.3% | 71.9% | 0 | — | — |
| `auto` | 575 | 5852/6259 = **93.5%** | 97.3% | 72.2% | 4 | **4** | **0** |
| `prefer-signed` | 7089 | 6158/6259 = **98.4%** | 99.0% | 94.8% | 386 | 348 | 38 |
| `prefer-unsigned` | 13650 | 5815/6259 = **92.9%** | 97.1% | 69.6% | 41 | 4 | 37 |

**Read the last three columns before the first three.** The oracle can only score
a declaration kuna names the same way DWARF does in the same function, and those
are overwhelmingly stack locals, i.e. `-O0`: of the 6,259 comparable rows,
**5,308 are `-O0` and 951 are `-O2`**. The flips are the other way round — the
defect this option exists for is an `-O2` register local, which the oracle
usually cannot see. Per value:

| value | checkable rows `-O0` / `-O2` | flips it makes there `-O0` / `-O2` | of all flips image-wide |
|---|---|---|---|
| `auto` | 5,308 / 951 | 1 / 3 | 575 |
| `prefer-signed` | 5,308 / 951 | 134 / 252 | 7,089 |
| `prefer-unsigned` | 5,308 / 951 | 13 / 28 | 13,650 |

So `prefer-signed` is the only value with a real measurement (386 of its flips
land where DWARF can judge them). **`auto`'s fidelity is essentially unmeasured**:
4 observations out of 575 flips, 0.7% coverage, and its +0.1pp on the overall
rate is noise at that count. It is 4-for-4, which is worth knowing and is not
worth calling validation. (This is not a DWARF type-lock artifact — on `fmt -O2`
`auto` flips 4 of the 132 plain-integer declarations on the *unstripped* image and
4 of the 223 on the stripped one, so it behaves the same with and without debug
info; the oracle's mass simply sits where the flips are not.)

Three more things fall out of that table.

* **The deficit is an `-O2` deficit.** At `-O0` today's declarations already
  agree with the source 97.3% of the time; at `-O2` they agree **71.9%** of the
  time. That is the `SUB_UINT_PLAIN < SUB_INT_PLAIN` fold meeting a compiler
  that masks, shifts and zero-extends.
* **`auto` is exact but narrow.** It moves 575 declarations image-wide and only
  4 of them are checkable against DWARF — all 4 agree (`gzip gen_codes::len`,
  `gzip ct_init::len`, `ln main::link_errno`, `tar dump_file0::fd`, each
  `unsigned`→`int` against a source `int`). Demanding unanimity buys precision
  and costs reach — and, as above, leaves the arm's fidelity unmeasured.
* **`prefer-signed` is where the fidelity is.** It carries `-O2` from 71.9% to
  **94.8%** — 252 checkable flips at `-O2`, 235 right — and `-O0` from 97.3% to
  99.0%. Its 38 wrong flips are concentrated in exactly the types whose
  operations carry no signedness at all: `mode_t` 18, `size_t` 12, `gid_t` 2,
  `uintmax_t` 2, `unsigned int` 2, `ino_t` 1, `reg_syntax_t` 1; the 348 right
  ones are `int` 342, `idx_t` 5, `Idx` 1. That is TRex's §5.1 claim reproduced:
  when nothing observed settles it, C source says `int`.
* **`prefer-unsigned` is refuted.** It moves 13,650 declarations and makes the
  output *less* faithful than leaving them alone, in both slices (4 of its 41
  checkable flips agree). It is sound, and it is the wrong default for anything.

**0 flips on a sub-`int` declaration** under any value. That is the width rule
holding, but note what this corpus can and cannot witness: every binary in it is
built for a target whose `int` is 4 bytes, so it cannot distinguish a guard read
off the target from one read off the reader. The 16-bit stage test is what
distinguishes them.

**What the method does not see.** Only names DWARF knows in the same function
are compared. kuna also names some register locals after the parameter of the
function they feed, and there the ground truth can be genuinely ambiguous: `ls`
and `du` `save_abbr` get an `unsigned long abbr_size` that `auto` declares
`long`, and `abbr_size` is `extend_abbrs`'s `size_t` parameter — but the value's
own source-level identity in `save_abbr` is `idx_t zone_size`, which is
*signed*, and C converts it at the call. Those are excluded rather than scored
either way.

## What this buys, and what it does not

`extract_variables` (`infra/decompile_drive.rs`) exports `FuncProto` parameter
types, `ScopeLocal` stack-symbol types and `framelayout` slots. No HighVariable
declaration is consulted, and no prototype or symbol type changes here — so the
scored surface is untouched:

* decbench `type_match` normalizes `unsigned` away in `QUALIFIERS` and collapses
  `int4`/`uint4` in `TYPE_MAP`: **zero** either way;
* `structscore --trex` (TRex Fig. 6, whose last step *is* the signedness step) on
  `fmt -O2` is byte-identical with the option on: mean 1.7322, `c_primitive`
  58/95.

The instruments that do move are readability ones: `varcensus`'s declared-type
histogram, and the cast count in the emitted C.
