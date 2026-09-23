# castimplied: plan

## The seam

A print-time predicate, `p9_emit/kuna_castimplied.rs (ImpliedCasts::drops)`,
consulted by the printer where a conversion would print:

- the `CPUI_CAST` arm of `PrintC::op_push_ir_inner` (`op_type_cast_ir_with`),
- the cast arm of `op_int_zext_ir` / `op_int_sext_ir`, after upstream's
  `is_extension_cast_implied` has declined.

A dropped conversion pushes its operand bare, exactly as a `signedness`-dropped
cast does, so precedence is decided by the enclosing operator. The IR is not
changed; the ported `CastStrategyC`, `is_extension_cast_implied` and their pinned
tests (`verify_w10_cast_strategy_adversarial.rs`, `p9_emit/cast/tests.rs`) are
untouched.

## What the predicate needs from the printer

The printer's `ImpliedView` answers what the IR alone cannot: the spelling of a
type, whether an extension prints as a cast or is hidden, whether `signedness`
already dropped a cast, whether a truncation prints as a cast, a high's printed
name, and whether an op is being printed as `lhs = <op>;` (set only by
`emit_expression_ir`; a ternary arm pushes its op with no reader too, and must not
count). The declarations are recorded as they are written: the header's return
type and parameters (`emit_prototype_declaration_c`, `emit_prototype_inputs`) and
each local's final declaration line (`emit_local_var_decls`, after every collapse
and override), keyed by high.

## The decision

For a conversion `op` read by `read_op` (looked through implied `COPY`s):

1. The target must be an integer (`TYPE_INT`/`TYPE_UINT`, not an enum, not `bool`);
   a `CAST`'s IR source must be an integer or `bool`; an extension must print as a
   cast and not be hidden already.
2. The operand's C type is `Known`, `Unknown` or `Opaque` (see analysis.md).
3. Below another integer conversion that prints (`fixes_type`): drop iff the
   operand's type is `Known` and every value of it fits the target.
4. At the top: drop an identity `CAST` (operand already spelled as the target);
   otherwise require a value-preserving widening (from the `Known` type, or the IR
   type when `Unknown`; never when `Opaque`) into a type-locked parameter of a direct
   call with no per-call-site override and no `truncarg` trim, a statement
   assignment to a local or parameter declared with that spelling, or a `return`
   from a function declared with that spelling.

## Gating

Option `castimplied` (P9 `cast-policy`), C output only (`integer_promotion`), and
nothing when `nocastprinting` is on. Default per `default-on-evaluation.md`.

## Tests

- `tests/stages/kuna-castimplied.xml`: two passes over gcc -O0 code for a declared
  callee, a trial callee, a varargs callee, an assignment, a return, a stacked chain
  and a sign change under a widening.
- `kuna-cli/tests/decompile_all_cli.rs`
  `an_implied_cast_round_trips_through_the_printed_c`: the printed functions of
  `castimplied_{gcc,clang}_O0_x86_64` compiled with gcc and clang, option off and
  on, must print what the binary prints (negative chars, 0xfffffff0 unsigned ints,
  shorts, a varargs `printf`, a sign change that must stay).
- `p9_emit/kuna_castimplied/tests.rs`: `preserves` agrees with C's own integer
  conversion over every pair of 1/2/4/8-byte signed/unsigned types and `bool`.
