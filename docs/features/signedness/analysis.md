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

* **The promotion width.** "Signedness-independent at a fixed width" is a
  precondition, not a turn of phrase. C's integer promotions convert every
  operand narrower than `int` to `int` *before* the operator runs, and which
  extension that is comes off the declaration, so below the promotion width
  nothing on the neutral list is neutral: `(short)-1 == -1` is true while
  `(unsigned short)0xffff == -1` is a constant false, and `+ - * & | ^ <<` all
  diverge the same way. The pass therefore declines any declaration narrower
  than `TypeFactory::get_size_of_int()`. Reduced from findutils `find -O2`
  `sub_bac0`, where re-declaring `short v8` unsigned left
  `if ((v2 == v8) || (v8 == -1)) break;` in place with a dead `break`; the
  reduced bytechunk is the second function in `tests/stages/kuna-signedness.xml`.
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

`INT_LEFT` is on the demand side for a different reason: `<<` produces the same
bits either way, but shifting a *negative* value left is undefined behaviour in
C, so a value the body shifts is declared unsigned or not re-declared at all.
TRex classifies `LeftShift` the same way. Without that rule `auto` turned
defined unsigned shifts into signed-overflow UB — zlib `sub_ce20`'s
`unsigned int v7` with `v7 = v13 | v7 << (v4 & 0x1f);` was the witness, and
`gcc -O2 -fsanitize=undefined` printed `runtime error: left shift of negative
value` on the re-declared text only.

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
encoding. 58,653 plain-integer declarations; 6,511 carry such a name, 6,260 of
them at the same width as the DWARF type. Signedness is binary, so a
declaration that differs between two values is right in exactly one of them.
Harness: `.scratch/acc/{dwarfsign,accuracy}.py` on the branch.

| value | declarations moved | agrees with DWARF | `-O0` | `-O2` | moved *and* DWARF-named | right | wrong |
|---|---|---|---|---|---|---|---|
| `upstream` | 0 | 5849/6260 = **93.4%** | 97.3% | 72.0% | 0 | — | — |
| `auto` | 579 | 5853/6260 = **93.5%** | 97.3% | 72.3% | 4 | **4** | **0** |
| `prefer-signed` | 7183 | 6159/6260 = **98.4%** | 99.0% | 94.9% | 386 | 348 | 38 |
| `prefer-unsigned` | 13623 | 5816/6260 = **92.9%** | 97.1% | 69.6% | 41 | 4 | 37 |

Three things fall out of that table.

* **The deficit is an `-O2` deficit.** At `-O0` today's declarations already
  agree with the source 97.3% of the time; at `-O2` they agree **72.0%** of the
  time. That is the `SUB_UINT_PLAIN < SUB_INT_PLAIN` fold meeting a compiler
  that masks, shifts and zero-extends.
* **`auto` is exact but narrow.** It moves 579 declarations image-wide and only
  4 of them are checkable against DWARF — all 4 agree (`gzip gen_codes::len`,
  `gzip ct_init::len`, `ln main::link_errno`, `tar dump_file0::fd`, each
  `unsigned`→`int` against a source `int`). Demanding unanimity buys precision
  and costs reach.
* **`prefer-signed` is where the fidelity is.** It carries `-O2` from 72.0% to
  **94.9%** — 252 checkable flips at `-O2`, 235 right — and `-O0` from 97.3% to
  99.0%. Its 38 wrong flips are concentrated in exactly the types whose
  operations carry no signedness at all: `mode_t` 18, `size_t` 12, `gid_t` 2,
  `uintmax_t` 2, `unsigned int` 2, `ino_t` 1, `reg_syntax_t` 1; the 348 right
  ones are `int` 342, `idx_t` 5, `Idx` 1. That is TRex's §5.1 claim reproduced:
  when nothing observed settles it, C source says `int`.
* **`prefer-unsigned` is refuted.** It moves 13,623 declarations and makes the
  output *less* faithful than leaving them alone, in both slices (4 of its 41
  checkable flips agree). It is sound, and it is the wrong default for anything.

**0 flips on a sub-`int` declaration** under any value, which is the width rule
holding across the whole corpus.

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
