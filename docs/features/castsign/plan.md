# castsign: plan

## The seam

`p9_emit/kuna_typeround.rs (plan)` is the one place a local's declared signedness
is rounded. `castsign` widens what it looks at; the decision, the declaration
override and the cast drop (`SignPlan::drop_cast`) are unchanged.

- `p9_emit/kuna_castsign.rs (whole_slot_local)`: a high with symbol offset 0, a
  Symbol type that is a plain integer of the declaration's width (or none), and no
  other high in the function printing the same name (`name_counts`).
- `p9_emit/kuna_castsign.rs (neutral_reader)`: a `CPUI_CAST` to a pointer of the
  value's width, or the index of a `PTRADD` whose base is the value's width.
  `evidence_walk` consults it only where it would otherwise veto.
- The `is_param` skip is lifted under `castsign`. `retain_sole_named` already keeps
  only highs the printer declares in the body.
- Any decision one of these made (`frame_local || is_param || widened`) is kept
  only if it declares the value signed (`TYPE_INT`), no operator that overflows
  over a signed operand reads the value (`kuna_castsign.rs (can_overflow)`,
  recorded by `evidence_walk` as `CastsignWalk::wraps`), no `==` `!=` `&` `|` `^`
  meets it with a constant whose top bit is set (`kuna_castsign.rs (wide_literal)`,
  recorded as `CastsignWalk::wide`), and one of the casts the
  walk saw on a declared member prints today and becomes a no-op
  (`kuna_castsign.rs (drops_printed_cast)` over `CastsignWalk::casts`).

`castimplied` (`kuna_castimplied.rs (ImpliedCasts::assigns_to)`) accepts
`lhs = (T)e` → `lhs = e` when `castsign` is on, the plan re-signed `lhs`'s
declaration (the printed spelling is the planned type's), and that type is as
wide as `T`.

## The option

`castsign on|off`, P9 `cast-policy`, `live_field = cast_sign`. It is inert under
`signedness upstream`, because `plan` returns an empty plan there, and for Rust
output: `emit_function_document` passes it only when the language has C's
implicit integer conversions (`integer_promotion`). Off is
byte-identical to `signedness` alone: `plan`, the walk and `castimplied` check the
flag before anything new.

## Tests

- `tests/stages/kuna-castsign.xml`: pass 1 (off) pins a `uint8` stack value with
  `(int8)v1` at each signed comparison; pass 2 (on) pins `int8` and the dropped
  casts. An index the body decrements, a logically shifted value and an
  address-taken stack value stay unsigned in both passes; pass 3 locks the first
  slot with `map addr` and it keeps `uint8` and its casts.
- `kuna-cli/tests/decompile_all_cli.rs`
  `a_signed_only_variable_round_trips_through_the_printed_c`: the wrap fixtures
  (`castsign_wrap_x86_64.c`, gcc and clang -O0, gcc -O1) with inputs at 2^63 - 1,
  2^63, 2^63 + 1 and the 32-bit edges, and the original four fixtures. The printed
  C, option off and on, is built with gcc and clang at -O0 and -O2 and must print
  what the binary prints; the arithmetic shapes must print unchanged with the
  option on. It fails on the version before the arithmetic rule. The equality
  fixtures (`castsign_eq_x86_64.c`, gcc and clang -O0) compare the value with
  `3000000000u` and `10000000000000000000UL`, directly, through `!=` and after a
  `|`, and must print unchanged with the option on; they fail on the version
  before the wide-literal rule.
- `castsign_leaves_a_locked_declaration_alone`: `--assert type` on a stack local
  and a register local, and a DWARF local, keep their type with the option on.
