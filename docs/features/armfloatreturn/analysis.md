# ARM scalar VFP return evidence

An ABI-declared double result in d0 lost its high word and floating type. Recovery
now requires ARM ELF VFP calling-convention evidence, preserves overlapping double
storage, and types the scalar return before constant folding. Explicit declarations
win; absent, conflicting, custom and soft-float metadata stay conservative.

Every changed function in the corpus is classified here:

- `returns-hard.o:scale`: recovers its double input and complete arithmetic result.
- `returns-hard.o:fixed`: recovers 1.5 instead of its zero low word.
- `returns-hard.o:wrapper`: forwards the double argument and uses the full call result.
- `returns-hard.o:read_double` and `fmtlf_armhf:rd`: return the complete parsed double.
  The failure call keeps zero arguments; a live VFP register is not argument evidence.
- `fmtlf_armhf:main`: uses rd's actual result instead of an unconnected d0 value.
- `fmtlf_armhf:show2`: retains a previously missing eight-byte VFP input. Its mixed
  integer/VFP parameter types, ordering and caller agreement are still unresolved.
  The original tree already loses that double argument. This task does not claim
  general mixed-bank or variadic argument recovery; use an explicit prototype.

Synthetic regressions additionally cover two double parameters, a float constant,
a memory double, a true integer return, malformed metadata, and explicit contracts.
Aggregate and vector reconstruction remain outside scalar return inference.

The seven-file corpus contains 47 functions. All synthetic input source is in
`fixtures/`; `fmtlf_armhf` and its C source are already in the repository's
`kuna-analysis` tests. No third-party binary was added. Replay after `make` using
Python 3, `arm-linux-gnueabi-as`, and the GNU ARM soft/hard-float cross-compilers:

```sh
python3 docs/features/armfloatreturn/corpus.py --output ../tmp/armfloatreturn-corpus
```

`corpus.diff` contains every changed function, with trailing whitespace trimmed.
The replay directory retains the raw outputs. The other functions are byte-identical.
Compiler versions can affect synthetic instruction selection; the Rust regressions
use fixed instruction bytes and construct their ELF metadata directly.

Both runtime settings pass the original 675 assertions without a baseline change.
The new raw stage checks conservative behavior when the frontend cannot supply
the required evidence; the synthetic ELF CLI tests exercise the positive path.
The option stays off by default and is absent from automatic presets. Broader
architecture/corpus and speed evidence is required before preset promotion.

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
arm-linux-gnueabihf-gcc -O2 -marm -mfpu=vfpv3-d16 -c \
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
double through a union; its last register write has the same width as a float
return. Use `--assert 'prototype double_low_bits double double_low_bits(double)'`
to retain the whole double. A regression verifies that this declaration preserves
the high word and its arithmetic in both option settings. Ordinary eight-byte
constant, load, copy, arithmetic and conversion results retain their width.
