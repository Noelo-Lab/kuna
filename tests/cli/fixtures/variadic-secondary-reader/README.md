# Stored variadic integer

This authored x86-64 ELF uses the Microsoft ABI to isolate EDX argument recovery.
`render_value` returns the physical RDX bits; `render_two` packs the low 32 bits
of RDX and R8 into one result. The reference checks integer boundaries as well
as small values. An EDX write zeroes the upper half, so the reference returns the
value as uint32_t widened to uint64_t, including for negative inputs.

```sh
gcc -O2 -fno-optimize-sibling-calls -c tests/cli/fixtures/variadic-secondary-reader/selected.c -o /tmp/variadic-selected.o
gcc -O0 -no-pie tests/cli/fixtures/variadic-secondary-reader/{probe.S,native.c} /tmp/variadic-selected.o -o /tmp/variadic-reader
/tmp/variadic-reader
kuna decompile /tmp/variadic-reader caller_stored_integer --json --mode reliable --assert-strict \
  --assert 'prototype caller_stored_integer unsigned long long MSABI caller_stored_integer(int *value)' \
  --assert 'prototype render_value unsigned long long MSABI render_value(const char *format,...)'
```

Unpatched output stores `*value + 1` but calls `render_value("%i")` without it.
The direct-forwarding `caller_integer` control retains the argument.

The CLI regression compiles the ten recovered positive bodies with GCC and
Clang at `-O0` and `-O2`, checking argument values and pointer stores. It also
compares option-on/off output for unused registers, unsupported and mutable
formats, clobbered or undeclared values, partially defined integer storage,
undefined earlier argument slots, unknown comparison operands, unknown phi arms
and shifts of undefined upper bytes.
The optimized C cases select an increment/decrement from a declared tag (including
a tag returned by another call), clamp the result and store it before the variadic
call. Thirteen values around the clamp bounds and integer limits are crossed with
six tags; native execution and emitted C must agree on the result and stored value.
Those negative controls are decompiled only; their assembly deliberately includes
calls without a valid format operand or complete promoted argument.

`imported.py` generates a small PE32+ with the same load/increment/store sequence,
an authored import table and a named read-only format. It needs no Windows SDK
or DLL. The native reference executes the equivalent indirect call from ELF;
the emitted PE body must preserve the same value and store. The CLI test also
checks writable-section, unsupported-format and undeclared-register negatives,
plus a healthy call without the store:

```sh
python3 tests/cli/fixtures/variadic-secondary-reader/imported.py /tmp/variadic-imported.exe
kuna decompile-all /tmp/variadic-imported.exe --addr 0x140001000 --mode reliable --assert-strict \
  --assert 'function 0x140001000-0x14000101d=caller_imported_integer' \
  --assert 'prototype caller_imported_integer unsigned long long caller_imported_integer(int *value)' \
  --assert 'prototype 0x140002050 unsigned long long render_value(const char *format,...)' \
  --assert 'data 0x1400020a0 char integer_format[3]'
```

The self-contained `mixed-format.xml` and `named-format.xml` check formats whose
conversion byte or NUL terminator is writable, alongside an entirely immutable
positive control. The second file repeats the checks with explicit char-array
symbols, whose flags otherwise conceal the writable bytes:

```sh
kuna test --datatests --datatests-dir tests/cli/fixtures/variadic-secondary-reader
```
