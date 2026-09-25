# castternary: plan

## The seam

`iteregion` prints an assignment diamond as `dest = c ? a : b;` in
`printc.rs (PrintC::emit_block_if_ite)`: the condition, then each arm's op pushed
with no reader (`op_push_ir(arm, None)`). A cast on an arm is printed by the same
three sites `castimplied` already answers (the `CPUI_CAST` arm of
`op_push_ir_inner`, and the cast arms of `op_int_sext_ir` / `op_int_zext_ir`),
all of which ask `PrintC::implied_cast_drops`.

So the decision is made once per conditional, before its first arm is pushed:
`PrintC::conditional_arm_drops` asks `kuna_castternary::arm_drops` which of the
arms' conversion ops to leave out, stores them in `ImpliedCasts` (`set_arm_drops`),
and clears them after the second arm. `implied_cast_drops` answers yes for a
stored op before it consults `castimplied`. The IR is not changed.

## Why a new option and not a value of `castimplied`

`castimplied` is an on/off decision about one conversion and the place its value
goes. This is a decision about a pair of operands and the conversion the operator
between them performs, with its own evidence (the conditional's type computed
under two readings) and its own failure mode (changing the conditional's type).
It gets its own row and off switch, so a regression can be bisected to it and an
ablation reads `--option castternary off` with `castimplied` unchanged. It reuses
`castimplied`'s machinery for what an operand's C type is (`operand_type`,
`def_type`, `explicit_type`) and for value preservation (`preserves`), and works
whether or not `castimplied` is on.

## The decision (kuna_castternary.rs)

1. The conditional must be assigned to a variable the printer declared with an
   integer type (`explicit_type` of `dest` is `Known` and an integer).
2. Each arm is analysed into `kept`, the promoted types its printed expression may
   have today, and optionally a candidate conversion `(op, target, operand set)`:
   - a `COPY` of a constant: the type C gives the literal token the printer writes
     (`literal_type`, over `PrintC::integer_constant_token`);
   - a `COPY` of an implied conversion, or (under `iteexpr`) the conversion op
     itself: `kept` is the target; it is a candidate when it prints as a cast, is
     not already dropped, targets a 4- or 8-byte integer, and keeps the value of
     an integer (not `bool`, not enum) operand whose type set is defined;
   - anything else: the set `value_set` gives for its `CType` (one type when
     known, every promotion of its width when not).
   An arm of no integer type (pointer, float, enum, an opaque declaration) stops
   the decision.
3. `R` = the usual arithmetic conversions of the two `kept` sets, which must be
   one type. Try both candidates, then the first, then the second: accept when
   every candidate's target is `R` and the conversions of the resulting sets are
   still exactly `R`.

## Tests

- `kuna_castternary/tests.rs`: the conversions (`usual`, `common`), the pair
  choice (`choose`), and literal typing (`literal_type`), including ILP32/LLP64.
- `tests/stages/kuna-castternary.xml`: two passes over a base64 quantum and five
  small functions (`option iteexpr on` in both passes).
- `kuna-cli/tests/decompile_all_cli.rs`
  `a_conditional_arm_cast_round_trips_through_the_printed_c`: the printed
  functions of `castternary_{gcc,clang}_O0_x86_64` compiled with gcc and clang at
  -O0 and -O2, option off and on, must print the binary's output.
