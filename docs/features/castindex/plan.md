# castindex: implementation plan

## 1. Seam

`castarith`'s: inside `Funcdata::action_set_casts` (`p9_emit/coreaction_casts.rs`), per
op, after the PTRADD/PTRSUB repairs and `castarith`'s own rewrite, before the op's
inputs and output are cast. No new `Action`, so `universal_sched` and its listing
oracle are untouched; the ported `CastStrategyC` is not modified. `cast_output` gains a
twin, `cast_output_token`, that takes the output token instead of computing it.

## 2. The rules (`p9_emit/kuna_castarith.rs`)

- `element` (factored out of `castarith`'s `plan`, unchanged): the `T` a sum used as a
  `T *` steps in, and how it is used (a lone access, a value of its own type, a
  `void *` stepped in bytes).
- `plan_index` / `apply_index` / `rewrite_index`: one pointer operand, one non-constant
  addend; `scaled_index` splits an implied `x * S` / `x << s` read only by the add;
  `S == sizeof(T)` takes `x`, a one-byte `T` takes the addend; `integer_index`,
  `subscripts_another`, the base rules of `castarith`; `extension_saved +
  base_saved + sum_saved >= 1`. `apply_index` builds `PTRADD(base, x, #S)` and destroys
  the scaling op.
- `pointer_difference`: two pointers to the same one-byte integer type, result a
  pointer-width integer, `operand_saved(a) + operand_saved(b) >= 1`; returns the signed
  pointer-width integer the driver casts the result against, and the driver skips the
  op's input casts.

## 3. Option

`castindex on|off`, P9 / `cast-policy`, `presentation-default`, `core`, live field
`cast_index`. `Architecture::cast_index` is copied into the per-function context as
`cast_index && out_lang == C`. Default per the flip procedure
(`default-on-evaluation.md`).

## 4. Tests

- `tests/stages/kuna-castindex.xml`: pass 1 `option castindex off` (the bug), pass 2
  on: an `int`, a `long` and a byte index, a 16-byte stride that keeps the integer form
  in both passes, a `strchr(s,c) - s` difference.
- 12 unit tests in `p9_emit/kuna_castarith/tests.rs`: every width and index kind, a
  shifted index, a byte element, scales that are not the element, non-integer indexes,
  a sum used as a subscript, a scale not yet cast, a value use with and without a
  saving, byte-pointer differences of every signedness, other differences, a difference
  that saves nothing, the pass gate both ways for both rules.
- `decompile_all_cli.rs
  a_variable_index_and_a_byte_pointer_difference_round_trip_through_the_printed_c`:
  compiles the printed functions of `castindex_x86_64.c` (gcc -O0, clang -O0, gcc -O2;
  option on and off) and checks the program prints what the binary prints.
