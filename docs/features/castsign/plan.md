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
  only if it declares the value signed (`TYPE_INT`).

`castimplied` (`kuna_castimplied.rs (ImpliedCasts::assigns_to)`) accepts
`lhs = (T)e` → `lhs = e` when `castsign` is on, the plan re-signed `lhs`'s
declaration (the printed spelling is the planned type's), and that type is as
wide as `T`.

## The option

`castsign on|off`, P9 `cast-policy`, `live_field = cast_sign`. It is inert under
`signedness upstream`, because `plan` returns an empty plan there. Off is
byte-identical to `signedness` alone: `plan`, the walk and `castimplied` check the
flag before anything new.

## Tests

- `tests/stages/kuna-castsign.xml`: two passes over gcc -O0 bytes. Pass 1 (off)
  pins the `uint8` stack index with `(int8)v1`; pass 2 (on) pins `int8` and the
  dropped cast. A logically shifted value and an address-taken stack value stay
  unsigned in both passes.
- `kuna-cli/tests/decompile_all_cli.rs`
  `a_signed_only_variable_round_trips_through_the_printed_c`: four fixtures (gcc
  and clang, -O0 and -O1). The printed C, option off and on, is compiled with gcc
  and clang and must print what the binary prints, and the two must-stay-unsigned
  functions keep `unsigned long v1;` and their casts.
- `p9_emit/kuna_castsign/tests.rs`: the modular-conversion argument, the
  pointer-index and pointer-conversion bit identities.
