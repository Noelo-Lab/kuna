# Re-entered indirect dispatch fixtures

These are original handwritten MIPS functions and freestanding Linux test callers.
No vendor code is included. `bounce` saves the caller's return address, dispatches
through `$31` or ordinary `$9`, writes one, two or four adjacent words, and returns.
The linked `_start` checks every written word, surrounding canaries, and (for the
six wide-return cases) both halves of the 64-bit return value.

The `.bin` files contain only `bounce`, extracted after GNU link-time relocations.
`provenance.json` records their base, extent, ABI, source/output hashes and tools.
`-addiu` crosses a signed low-immediate boundary at base `0x407ff0`.
`-join` merges odd/even value updates before re-entering the dispatcher.
`-separate-return` uses a distinct genuine return instruction. `-early` reaches
the dispatcher with the caller's return before the second step is known,
`-delay` updates the value in the dispatcher's delay slot, and `-condreturn`
carries the caller's return through a conditional branch. Its defined selector
and default return are now replayed like every other fixture. `-indirect-reentry`
re-enters old code through a second recovered indirect jump, creating an edge
without creating any operations at the destination. `-wide-return`
returns the last value in the high half and its XOR with `0x12345678` in the
low half of a 64-bit result, computed entirely in registers. Shift-31 and
shift-63 variants retain the sign-extended input bits at neighboring and maximum
legal shift counts. Native replay also runs UBSan and ignores only the C
parentheses style warning; uninitialized-use diagnostics remain errors.

Rebuild with an Octeon-capable GNU assembler and matching GNU tools, retaining
execution images outside the checkout:

```
python3 build.py --assembler /path/to/mips64-octeon-linux-gnu-as \
  --linker /path/to/ld --objcopy /path/to/objcopy --nm /path/to/nm \
  --artifacts /path/to/external/artifacts
```

The original linked N32/N64 callers were executed on pinned QEMU Octeon68XX.
The CLI regression separately compiles emitted C at O0/O2 with the declared
prototype, checks 10,000 unsigned inputs per fixture, all stores/canaries and
wide return values. It also checks option-off and ordinary-register controls.
