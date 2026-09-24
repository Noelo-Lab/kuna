# castarith — implementation plan

## 1. Where the rewrite lives: inside the cast pass, not in P5

The `INT_ADD`s this changes are already final when P9 starts: the census found
`PTRSUB -> INT_ADD` demotion fires 0 times on the cast-census corpus, so these ops
were never anything but integer additions, and every type and variable is merged.
A P5 rewrite to `PTRADD` would feed type propagation (a `PTRADD` pushes its element
back onto the base) and could move declared types, which is exactly what this lane
must not do. The rewrite therefore runs inside `Funcdata::action_set_casts`
(`p9_emit/coreaction_casts.rs`), per op, right after the upstream
`cast_fixup_ptradd` / `cast_fixup_ptrsub` repairs (so a sum a repair has just
demoted is seen too) and before that op's inputs and output are cast. No new
`Action` is scheduled, so `universal_sched` and its listing oracle are untouched.
The ported `CastStrategyC` decision matrix is not modified.

## 2. The rule (`p9_emit/kuna_castarith.rs`)

`plan` accepts `INT_ADD(p, #K)` when `p` is a non-constant whose merged type is a
pointer into a byte-addressed space and the sum is used as a `T *`:

- a lone LOAD reads `T` through it, or a lone STORE writes `T` (the sum's own
  pointee wins when it is the same type, or an integer of the same width);
- otherwise the sum's own pointee; a `void` pointee steps in bytes, but only for a
  value that stays inside the function (not a call argument or return value, whose
  type the whole-program prototype votes read);
- `T` is a scalar or a pointer whose size is its aligned size, and `K` (signed) is
  a whole number of `T`s.

It refuses: a variable index, an aggregate or padded element, a word-addressed
space, a sum used as an integer (integer arithmetic, a store into an integer
slot, or an assignment to a variable declared as an integer), an address shared by several accesses, a store of a value whose defining op
the pass has not reached yet unless its opcode fixes the value's kind, a constant
that names a global, and an address-like constant beside an integer cast to a
pointer (`table[i]`).

`apply` turns the op into `PTRADD(base, #k, #sizeof T)`, with `k` typed as a
signed pointer-sized integer. The base is `p` itself when `p` is a variable whose
printed declaration points at `T` (checked with the printer's own
`printc::declared_variable_type`), a retargeted implied cast when `p` is an implied
cast read only here, a new implied `CPUI_CAST` to `T *` of that cast's input when `p`
is an implied cast other ops share (so no cast stacks on it), and a new implied
`CPUI_CAST` to `T *` of `p` otherwise.

`castimplied` reads a converted load's C type from the subscript's cast base, the way
it reads `*(T *)p`, so a widening it left out of the integer form stays out.

## 3. Rendering

No printer change. The `PTRADD` prints the way every `PTRADD` does:
`((T *)p)[k]` under a dereference, `&((T *)p)[k]` as a value, or `(T *)p + k` with
`arraynotation off`.

## 4. Option

`castarith on|off`, P9 / `cast-policy`, `presentation-default`, default `on` per
the flip procedure (record.json `default_on_evaluation`). `Architecture::cast_arith`
is copied into the per-function context as `cast_arith && out_lang == C`, so the
Rust back-end never sees the rewrite.

## 5. Tests

- `tests/stages/kuna-castarith.xml`: pass 1 `option castarith off` (the bug), pass 2
  on, pass 3 `arraynotation off`; controls for a non-whole offset; a byte and a word
  widened under the subscript (castimplied).
- 23 unit tests in `p9_emit/kuna_castarith/tests.rs`: every width and sign, float,
  negative offset, a negative index of 2^31 elements or more (kept), non-whole offset, direct base, other-width base, retargeted cast,
  shared cast read through,
  stored/compared/subtracted sums, void in and out of the function, aggregate,
  integer sum, variable index, unsettled and settled stores, the pass gate both ways.
- `decompile_all_cli.rs a_pointer_plus_whole_elements_round_trips_through_the_printed_c`:
  compiles every tested function of `castarith_x86_64.c` as printed (gcc -O0,
  clang -O0, gcc -O2; option on and off) and checks the program prints what the
  binary prints, including `far_idx`, whose reads 2^31 elements back must keep the
  integer form.
