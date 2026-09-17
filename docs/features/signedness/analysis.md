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
width as interchangeable there. That is the soundness basis: flip only when every
one of those demands agrees.

Two refinements were needed on top:

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

A third, from TRex: `INT_ZEXT` as a **definition** is not unsigned evidence
(TRex's `ZeroExtendTgt => None`). A zero-extension describes the *source*
operand's type; `int v = *p;` with an `unsigned char *p` is ordinary C. On
x86-64 this is the difference between seeing any variable at all and vetoing
every value a 32-bit instruction widened into its 64-bit register. As a
**reader** `INT_ZEXT` is still a hard unsigned demand — that is what decides
which extension the printed C performs.

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
