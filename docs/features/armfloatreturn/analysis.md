# ARM scalar VFP return evidence

An ABI-declared double result in d0 lost its high word and floating type. Recovery
now requires ARM ELF VFP calling-convention evidence, preserves overlapping double
storage, and types the scalar return before constant folding. Explicit declarations
win; absent, conflicting, custom and soft-float metadata stay conservative.

## Corpus

`corpus.py` builds every `fixtures/*.c` with clang as hard-float ARM, hard-float
Thumb, `softfp` and soft-float objects, adds every ARM ELF fixture in
`decompiler/crates/kuna-analysis/tests/fixtures`, and diffs `decompile-all --mode
aggressive` with the option off and on (run after `make binaries`):

```sh
python3 docs/features/armfloatreturn/corpus.py --output ../tmp/armfloatreturn-corpus
```

46 files, 489 functions, 95 changed; `corpus.diff` is the whole diff. No
`softfp` or soft-float object changes, and no ARM fixture changes other than the
three hard-float ones with floating-point code (`armfloatreturn_armhf.o`,
`fmtabi_armhf`, `fmtlf_armhf`). The Thumb objects change exactly as the ARM ones.
Every changed function:

- Return type and parameters recovered, meaning unchanged: `scale`, `fixed`,
  `fixedf`, `twoargs`, `cmp`, `loopsum`, `pick`, `wrapper`, `fwd` and `fwdf`
  (forwarders, through `passthrough`), `twocalls` (both calls keep their
  arguments), `narrow`, `widen`, `keep_double`, `mixed_load`, the `double_*`
  paths, `constant_paths`, `loaded_paths`, `copied_paths`, `double_low_bits`,
  `rd`, `read_double`, `f_sum` (which also gains the stack-passed `a` it hands
  to `__printf_chk`).
- An integer return stays in `r0` while `d0` is also written (`keep`, `to_int`)
  or still holds a callee's double (`bump`: `half(a0); return a1 + 1;`, where
  the wider `d0` used to win and printed `double bump(double a0) { return
  half(a0); }`); `half` itself returns its double.
- A double returned from a call on one path and computed on another: `a3`,
  `a7`, `a6` return both (`if (1.0 < a0) return (double)half(a0); return a0 *
  3.0;`). The added `d0` entry used to carry the per-piece checks overlap
  resolution gives a containing entry, so the call path's value dropped the
  whole return and these printed `void`, losing `x * 3.0` and `2.5`.
- Holes and float pairs: `second` fills the unused `d0` below the `d1` it
  returns with one `double` parameter (it printed `(unsigned int,unsigned
  int,double)`); `w2` reads two floats as the halves of `d0` and gets no
  8-byte `unsigned long long` parameter (it prints the option-off `(void)`
  with the returned `double`).
- Callers: `fmtabi_armhf:main` passes `(double)argc, 0.5` to `f_sum` and `argc`
  to `f_conv`; `fmtlf_armhf:main` returns `(int)rd(argv[0])`.
- Partial: `many` recovers its eight `d` parameters but not the ninth double on
  the stack; `mixargs` lists the VFP parameters before the `r0` integer, the
  model's order, as it does with the option off; `fmtlf_armhf:show2` gains its
  `d0` parameter (typed `unsigned long long`, it is only stored), but `main`
  still passes only `argc`: that recovered list is not canonical storage for
  its types, so it is no caller contract.
- Better return, arguments as with the option off: `callext` (extern `g`/`h`
  state no prototype, so a live `d0` is not their argument), `fwdw` (the float
  argument to `scalef` is lost either way), `fwdn` (its phantom `r0`/`r1`
  arguments are identical with the option off).
- Union bit manipulation: `lowbits`, `setlow` and `integer_bits` return the
  right type; the punning prints as an integer expression cast to the float
  type, and `integer_bits` types its `r0` parameter `float` because its bits
  are negated into the float result.

A `void` function whose last call leaves a double in `d0` (`half(x); gi = k *
3;`) prints that double as its return in `decompile-all`: the code is the same
as returning `half`'s result, as it is for a float callee in `s0`.

Homogeneous float aggregates are not reconstructed. `mkf2`, `mkf4` and `mkd2`
here print `void` either way, but a `struct { double a, b; }` or a `_Complex
double` whose members are computed prints as a `double` returning only its
first member (`double pair2(double a0) { return a0 + a0; }`); it needs a
declared type. Two floats read as the halves of `d0` below a used VFP
parameter (`double mixed(float a, double b, float c)`, `c` back-filled into
`s1`) keep that slot as one `unsigned long long` parameter so that `b` stays
second, and `float d2f(double)`, whose `d0` is read in two 4-byte pieces,
prints `(unsigned int,unsigned int)` either way.

The option stays off by default and out of the presets.

Scalar return inference is restricted to s0/d0. Synthetic controls set d1, d2
or d3 to 1.5 while returning integer 7 in r0; all must still return 7. Merely
extending every VFP output pair would turn those dead temporaries into results.

## Double-to-float narrowing

A final `vcvt.f32.f64 s0,d0` overwrites only half of d0. Treating that storage as
a full double joins the float with stale s1 bytes and changes the return type.
The return trial now narrows when every normal exit proves a new four-byte
write plus an untouched high half. Return width does not depend on a floating
arithmetic opcode: constants, loads, copied inputs and integer bit operations
still write only four bytes. Copies and joins are bounded; mixed-width joins,
full doubles, reassembled unchanged halves, and explicit output contracts do
not use this inference. Predicated ARM/Thumb joins can still contain the old d0
value before control-flow simplification removes an infeasible path. The proof
allows that carried value only when its upper bytes are retained by a partial
write; an unrelated full-width result still prevents narrowing.

The synthetic compiler reproducer is included in `fixtures/narrow.c`:

```sh
mkdir -p ../tmp/arm-float-narrow
clang --target=armv7a-linux-gnueabihf -marm -mfloat-abi=hard -mfpu=vfpv3-d16 -O2 -c \
  docs/features/armfloatreturn/fixtures/narrow.c -o ../tmp/arm-float-narrow/narrow.o
decompiler/target/release/kuna decompile ../tmp/arm-float-narrow/narrow.o narrow \
  --mode aggressive --option armfloatreturn on
```

The fixed result is `float narrow(double a0)` with `return (float)(a0 + a0);`.
The earlier eight-byte trial instead produced `double narrow(double a0)` and
`return (double)CONCAT44((int)((unsigned long long)(a0 + a0) >> 0x20),(float)(a0 + a0));`.
CLI regressions generate the same instructions without requiring a cross-compiler;
they also cover memory/conditional narrowing, widening and full doubles, the
off setting, and explicit float/double/void output contracts. IR tests decline
mixed-width joins, unrelated halves, and cyclic evidence.

`fixtures/mixed.c` adds `float mixed_constant(double x) { return x > 0 ? 1.0f :
(float)(x*x); }`, plus float-load and copied-input alternatives. Requiring a
float-producing opcode independently on every exit incorrectly rejected the
constant/load paths and the copied input at a join, leaving stale s1 bytes in
`CONCAT44` double returns. These now retain float return types, while the
corresponding full-double controls keep their double results. CLI tests build
the compiled instructions directly and check both option settings and explicit
float/double/void contracts. IR tests cover whole-value and low-half joins.

`fixtures/constants.c` covers returns containing only constants, only loads,
or only copied inputs, plus integer bit manipulation and full-double controls.
`float constant_paths(double x) { return x*x > 1.5 ? 1.0f : 2.0f; }` previously
retained an eight-byte trial because neither constant supplied float arithmetic
evidence. It now returns the float constants without stale upper bytes. The
synthetic ELF regression embeds the compiler's instructions, so running it does
not require a cross-compiler.

Partial-word double manipulation is inherently ambiguous without a return
contract. For example, `double_low_bits` deliberately clears the low word of a
double through a union; built with GCC its last register write has the same
width as a float return (clang masks the whole register, and keeps `double`).
Use `--assert 'prototype double_low_bits double double_low_bits(double)'`
to retain the whole double. A regression verifies that this declaration preserves
the high word and its arithmetic in both option settings. Ordinary eight-byte
constant, load, copy, arithmetic and conversion results retain their width.

## Tests

- `tests/stages/kuna-arm-float-return.xml` runs a raw image (no evidence, the
  option is inert), the relocatable `armfloatreturn_armhf.o`
  (`Tag_ABI_VFP_args=1`) and the linked `fmtlf_armhf` (hard-float header flag)
  with the option off, which reproduces the truncation, and on.
- `decompiler/crates/kuna-cli/tests/arm_float_returns.rs` builds ELF objects from
  fixed instruction words for the metadata controls, narrowing, widening, mixed
  paths and explicit contracts, and runs the fixture's `twocalls`/`twocallsf`: a
  parameter passed to a first call must leave the second call the value computed
  in the same register.
- Unit tests cover the attribute parser and the narrowing walk.
