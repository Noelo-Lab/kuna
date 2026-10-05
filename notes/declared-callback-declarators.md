# Declared callback C declarators

The failure is independently reproducible with 20 bytes of ARMv5TE code. No ROM,
external image, debug information, register reuse or inferred signature is needed.

```sh
python3 -c 'from pathlib import Path; Path("/tmp/callback.bin").write_bytes(bytes.fromhex("10402de90040a0e134ff2fe1ff0000e21080bde8"))'
kuna decompile /tmp/callback.bin --raw-image --base 0x10000 --addr 0x10000 \
  --target ARM:LE:32:v5t:default --isa arm --mode reliable --assert-strict \
  --assert 'prototype 0x10000 unsigned char invoke(unsigned int (*callback)(void))'
```

Before the fix (b3d0e996):

```c
uint1 sub_10000(undefined1 *callback)
{
  return (uint1)(*callback)();
}
```

The prototype assertion is applied. `BLX r4` at `0x10008` calls the pointer passed
in r0; `AND r0,r0,#255` at `0x1000c` consumes only the low byte of its word return.
This is a C declaration defect: the indirect call retains the recorded prototype
internally, but the printed byte pointer cannot be called by a C compiler.
The C renderer is identical in b3d0e996 and the branch's upstream base 281451c.

`CSpeller::declarator` used to traverse only pointers and arrays. An anonymous
`TYPE_CODE` with a stored prototype therefore became the size-one fallback base
`undefined1`. The fixed traversal adds the function parameter suffix, recurses
through parameter abstract declarators and continues through the return type.
The existing pointer/postfix grouping handles arrays of callbacks, pointers to
callbacks, pointer returns, nested callback parameters and callback returns.
`type_name` also uses this traversal for anonymous code types.

This prints existing type information. It does not modify signature inference,
indirect-call ABI handling, calling-convention printing or lifetime identities.
Named typedefs and code types without a stored prototype retain their old
spelling. The signature correction is a strict bug fix and needs no new option.

## Relationship to existing reports

- #802 and the merged #804 concern applying typed indirect-call prototypes,
  arguments and ABI. This patch prints the types that recovery already records.
- #869, consolidated under #890, demonstrates an untyped table entry printed
  through `void **`. That core code type has no stored prototype; this patch does
  not invent one and does not fully resolve #869.
- mgostIH's #894, #895 and #896 concern function extents, ARM CPSR state and ARM
  register shifts respectively. None of those changes is included here.

## Regression coverage

`tests/stages/kuna-declared-callback-c.xml` is a self-contained ARM byte fixture.
It checks parameters, a published global, a saved callback local, callback return
values, structure fields, arrays, variadic inputs, recursive callback parameters
and callback-returning callbacks. It exercises both core and `ctypes` spelling.
The parameter and saved-local declaration expectations fail before this fix.
Grammar tests round-trip actual stored prototype types, including a callback
returning a pointer to an array, and preserve named typedefs and unprototyped code.

The external synthetic validation compiles unchanged emitted C for the host and
ARMv5TE, and compares dispatch and low-byte results with ARM instruction execution.
A mutation control replaces the published callback during the first invocation;
the second invocation must retain its previously saved pointer.

Local validation: 675/675 datatests and 1,830/1,830 stage assertions pass, with all
old expectations retained. The full Rust workspace passes 7,844 tests (38 existing
ignored tests); specification, Python tooling, Clippy and catalog checks pass.
Unchanged emitted C passes 140 ARM/host comparisons and ARMv5TE cross-compilation.
The fourteen new stage assertions pass; twelve fail on the pinned pre-fix binary.

Alternating CLI timing on the 20-byte fixture (30 measured runs per binary):
b3d0e996 baseline 42.130 ms median; the 281451c candidate 44.735 ms (+6.18%,
2.605 ms). This includes loading/process overhead and unrelated changes between
the two upstream bases, so it is not an isolated renderer speed measurement.
