# `signedness` — plan

## Shape

| | |
|---|---|
| option | `signedness upstream\|auto\|prefer-signed\|prefer-unsigned`, default `upstream` |
| phase / subphase | P9 / `naming-policy` (the declaration seam, next to `declhightype`) |
| tier / change_kind | `transform` / `presentation-default` |
| module | `decompiler/crates/kuna-decomp/src/p9_emit/kuna_typeround.rs` |
| consumed at | `p9_emit/printc.rs` — `emit_function_document` (plan), `local_decl_type_and_comment` (the declaration), `emit_local_var_decls` (records what was written), `op_type_cast_ir` (drops the cast the declaration makes a no-op) |
| spec | `docs/spec/09-emission.md` §9.3, cross-referenced from `docs/spec/05-types.md` §5.1 |
| stage test | `tests/stages/kuna-signedness.xml` (two functions, 6 assertions: the two-pass witness plus the sub-`int` width guard) |

## Mechanism

Once per emitted function, for every HighVariable the printer would declare as a
plain integer (`SUB_INT_PLAIN`/`SUB_UINT_PLAIN`, **at least `int` wide**, not a
typedef, not type-locked, not a prototype parameter, not mapped onto a composite
symbol):

1. Seed a work list with the high's **explicit** members — the ones printed by
   name, which is where the declaration's type applies.
2. Fold the members' *defining* ops into the verdict as votes (a definition
   converts at a fixed width, so it can never constrain). Only the
   divide-and-shift family votes; the extension ops and `INT_2COMP` describe the
   operand, not the value, and say nothing.
3. Walk every reader. `Demands(signed|unsigned)` for the signedness-sensitive C
   constructs, plus `INT_LEFT` operand 0 as an unsigned demand (`<<` on a
   negative value is UB); `Carries` for `+ - * & | ^ ~` and the value-moving ops
   (extend the walk into an implied result); `Opaque` for `== !=`, truncation,
   concatenation, a condition, a call argument, a `return`, a stored value; a
   same-width `CPUI_CAST` to a plain integer is a demand for what it casts to;
   everything else, a `CPUI_CAST` to anything else included, is `Veto`.
4. `auto` writes `TYPE_INT` when the fold is unanimously signed, `TYPE_UINT` when
   unanimously unsigned, and leaves the declaration alone otherwise.
   `prefer-signed`/`prefer-unsigned` additionally settle the no-demand case.
5. `get_base_no_char` for the signed side, so a re-signed value never declares
   a character type (widening a value into `char` is a different decision with
   its own option). With the width rule in place this is unreachable today.

Step 0 is the width rule, and it is the whole reason the rest can call `+ - * &
| ^ == !=` neutral: C's integer promotions convert anything narrower than `int`
to `int` before any operator runs, and the declaration decides which extension
that is.

The declaration override is recorded **only when the emitter actually wrote it**
(a mapped-symbol, array or dedup-collapse override can take it back), and only a
recorded override authorizes dropping a cast.

## Evidence required before this can be considered for a default flip

* whole-corpus before/after over `fmt`/`ls`/`sort`/`du` at `-O0` and `-O2`, every
  hunk classified, for every value — done, see `record.json`;
* a per-arm `gcc -fsyntax-only` pair on every changed function and an executable
  round trip on the ones that link — done, 0 mismatches;
* agreement with DWARF on unstripped twins — done, see `analysis.md`; `auto`
  buys +0.1pp, which is the argument *against* flipping the default on accuracy
  grounds and for flipping it on cast-removal grounds;
* 0/675 datatests and stages `PARITY OK` with the default flipped — done;
* speed within +5% — done (+0.26%).

## Interactions

* `declhightype` (default on) chooses *which member's* type is declared; this
  chooses its signedness. It runs after, on whatever that returned.
* `realtypes` (default on) relabels residual `TYPE_UNKNOWN`; an `xunknownN` is
  not a plain integer, so it is never re-signed.
* `ctypes` decides the *spelling* (`uint4` vs `unsigned int`); orthogonal.
* `dedupvardecls` may replace a declaration line; such a high keeps its cast.
* The `variables` JSON surface (`framelayout`, `bytehonest`, the prototype
  recovery lanes) is not reached at all.

## Not in scope

* the `SUB_UINT_PLAIN < SUB_INT_PLAIN` order itself (upstream, wide blast radius);
* width changes — `unsigned long v3; if ((int)v3)` is a `SUBPIECE` rendered as a
  cast, a different decision, and `SUBPIECE` is `Opaque` here;
* locals narrower than `int`, at all: making the neutral operators
  promotion-aware instead of declining the variable is a follow-up;
* re-seeding inference from the verdict (that is a TRex substrate port, XL).
