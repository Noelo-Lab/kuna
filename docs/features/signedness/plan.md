# `signedness` — plan

## Shape

| | |
|---|---|
| option | `signedness upstream\|auto\|prefer-signed\|prefer-unsigned`, default `auto` |
| phase / subphase | P9 / `naming-policy` (the declaration seam, next to `declhightype`) |
| tier / change_kind | `transform` / `presentation-default` |
| module | `decompiler/crates/kuna-decomp/src/p9_emit/kuna_typeround.rs` |
| consumed at | `p9_emit/printc.rs` — `emit_function_document` (plan), `local_decl_type_and_comment` (the declaration), `emit_local_var_decls` (records what was written), `op_type_cast_ir` (drops the cast the declaration makes a no-op) |
| spec | `docs/spec/09-emission.md` §9.3, cross-referenced from `docs/spec/05-types.md` §5.1 |
| stage test | `tests/stages/kuna-signedness.xml` (two functions, 6 assertions: the two-pass witness plus the sub-`int` width guard) and `tests/stages/kuna-signedness-int16.xml` (4 assertions: the same width guard on a target whose own `int` is 2 bytes, with a 4-byte control) |

## Mechanism

Once per emitted function, for every HighVariable the printer would declare as a
plain integer (`SUB_INT_PLAIN`/`SUB_UINT_PLAIN`, **at least 4 bytes wide — the
`int` of the compiler that reads the output, not the target's**, not a typedef,
not type-locked, not a prototype parameter, not mapped onto a composite symbol):

1. Seed a work list with the high's **explicit** members — the ones printed by
   name, which is where the declaration's type applies.
2. Fold the members' *defining* ops into the verdict as votes (a definition
   converts at a fixed width, so it can never constrain). Only the
   divide-and-shift family votes; the extension ops and `INT_2COMP` describe the
   operand, not the value, and say nothing.
3. Walk every reader. `Demands(signed|unsigned)` for the signedness-sensitive C
   constructs; `DemandsCarrying` for the three of them whose printed result keeps
   the operand's type (`>>`, `/`, `%`) and for `INT_LEFT` operand 0, which is a
   stated unsigned *preference* rather than a soundness requirement; `Carries`
   for `+ - * & | ^ ~` and the value-moving ops (extend the walk into an implied
   result); `Opaque` for `== !=`, truncation, concatenation, a condition, a call
   argument, a `return`, a stored value; a same-width `CPUI_CAST` to a plain
   integer is a demand for what it casts to; everything else, a `CPUI_CAST` to
   anything else included, is `Veto`.
4. `auto` writes `TYPE_INT` when the fold is unanimously signed, `TYPE_UINT` when
   unanimously unsigned, and leaves the declaration alone otherwise.
   `prefer-signed`/`prefer-unsigned` additionally settle the no-demand case.
5. `get_base_no_char` for the signed side, so a re-signed value never declares
   a character type (widening a value into `char` is a different decision with
   its own option). With the width rule in place this is unreachable today.

Step 0 is the width rule, and it is the whole reason the rest can call `+ - * &
| ^ == !=` neutral: C's integer promotions convert anything narrower than `int`
to `int` before any operator runs, and the declaration decides which extension
that is. The floor is `max(4, TypeFactory::get_size_of_int())`, because the
promotion that matters happens in the compiler reading the output, where `int` is
4 bytes, and not on the target (the cspec says 2 on avr8gcc, avr8egcc, TI_MSP430,
TI_MSP430X, CR16, PIC24 and x86-16).

Step 6 is declaration ownership: the plan is filtered to highs whose rendered
name is unique on the printer's candidate list, before any of the declaration
collapses runs (`SignPlan::retain_sole_named`, called from
`emit_local_var_decls`). Every collapse — the composite-Symbol retain,
`collapse_symbol_decls`, `DeclDedup`, the `local_name_aliases` group suppression
— pairs candidates that render the same name, so a sole-named entry can neither
be suppressed by one nor absorb a sibling, and a flip can move neither a
collapse key nor the single line a non-flipped sibling's uses read through.

The declaration override is then recorded **only when the emitter actually wrote
it** (a mapped-symbol, array or dedup-collapse override can still take it back),
and only a recorded override authorizes dropping a cast.

## The default

`auto` ships as the default. It re-signs a declaration only on unanimous
evidence, so the only text it can move is that declaration and the casts the new
declaration makes into no-ops — which is what every instrument below measures,
and why it clears the repo's default-ON bar: 0 of 675 datatest assertions, stages
`PARITY OK` (1064/1064), speed within the +5% budget, and every hunk over eight
whole binaries classified as a declaration flip or a dropped cast.

`prefer-signed` is the arm with the better DWARF agreement (98.4% overall,
94.8% at `-O2`, against `upstream`'s 93.4%/71.9%) and it stays **opt-in**: it
moves 7,081 declarations image-wide against `auto`'s 575, which is more than
unanimity covers and more than the datatest corpus can absorb as a default.
`prefer-unsigned` stays as its control and is documented as not recommended.
`upstream` restores the declaration type inference produced, byte for byte.

## Evidence required before this can be considered for a default flip

* whole-corpus before/after over `fmt`/`ls`/`sort`/`du` at `-O0` and `-O2`, every
  hunk classified, for every value — done, see `record.json`;
* a per-arm differential warning oracle on every changed function
  (`gcc -fsyntax-only -Wsign-compare -Wsign-conversion`, warning sets compared,
  not error sets) and an executable round trip on the ones that link — done,
  0 ordered-comparison sign-compare deltas and 0 round-trip mismatches;
* agreement with DWARF on unstripped twins — done, see `analysis.md`; `auto`
  buys +0.1pp on four checkable observations, so the case for shipping it as the
  default is cast removal and the end of declaration-vs-body contradictions, not
  the agreement rate;
* the control that says the walk is not decoration — "declare every eligible
  plain-integer local signed, no walk" scores 92.8% overall and **59.2%** at
  `-O2` against `prefer-signed`'s 98.4% / 94.8%, because it turns 379 `size_t`
  locals signed where `prefer-signed` turns 12 — done, see `analysis.md`;
* 0/675 datatests and stages `PARITY OK` with the default flipped — done;
* speed within +5% — done (+0.17% on the minimum of 21 interleaved runs).

## Interactions

* `declhightype` (default on) chooses *which member's* type is declared; this
  chooses its signedness. It runs after, on whatever that returned.
* `realtypes` (default on) relabels residual `TYPE_UNKNOWN`; an `xunknownN` is
  not a plain integer, so it is never re-signed.
* `ctypes` decides the *spelling* (`uint4` vs `unsigned int`); orthogonal.
* `dedupvardecls` may replace a declaration line; such a high is filtered out of
  the plan before the collapse runs, so it keeps both its declaration and its
  cast.
* The `variables` JSON surface (`framelayout`, `bytehonest`, the prototype
  recovery lanes) is not reached at all.

## Not in scope

* the `SUB_UINT_PLAIN < SUB_INT_PLAIN` order itself (upstream, wide blast radius);
* width changes — `unsigned long v3; if ((int)v3)` is a `SUBPIECE` rendered as a
  cast, a different decision, and `SUBPIECE` is `Opaque` here;
* locals narrower than `int`, at all: making the neutral operators
  promotion-aware instead of declining the variable is a follow-up;
* re-seeding inference from the verdict (that is a TRex substrate port, XL).
