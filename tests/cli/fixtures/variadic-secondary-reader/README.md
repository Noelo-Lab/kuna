# Stored variadic integer

This authored x86-64 ELF uses the Microsoft ABI to isolate EDX argument recovery.
The callee returns the physical RDX bits. The reference checks integer boundaries
as well as small values. An EDX write zeroes the upper half, so the reference returns the value as uint32_t widened
to uint64_t, including for negative inputs.

```sh
gcc -O0 -no-pie tests/cli/fixtures/variadic-secondary-reader/{probe.S,native.c} -o /tmp/variadic-reader
/tmp/variadic-reader
kuna decompile /tmp/variadic-reader caller_stored_integer --json --mode reliable --assert-strict \
  --assert 'prototype caller_stored_integer unsigned long long MSABI caller_stored_integer(int *value)' \
  --assert 'prototype render_value unsigned long long MSABI render_value(const char *format,...)'
```

Unpatched output stores `*value + 1` but calls `render_value("%i")` without it.
The direct-forwarding `caller_integer` control retains the argument.

The CLI regression compiles the four recovered positive bodies with GCC and
Clang at `-O0` and `-O2`, checking argument values and pointer stores. It also
compares option-on/off output for unused registers, unsupported and mutable
formats, clobbered or undeclared values, and partially defined integer storage.
Those negative controls are decompiled only; their assembly deliberately includes
calls without a valid format operand or complete promoted argument.

The self-contained `mixed-format.xml` checks formats whose conversion byte or
NUL terminator is writable, alongside an entirely immutable positive control:

```sh
kuna test --datatests --datatests-dir tests/cli/fixtures/variadic-secondary-reader
```
