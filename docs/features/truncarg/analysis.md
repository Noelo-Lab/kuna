# truncarg: analysis

## 1. The symptom

`f(v) { sink((unsigned short)(v * 3)); }`, built with gcc -O2, is
`lea (%rdi,%rdi,2),%edi; movzwl %di,%edi; jmp sink`. kuna on main prints
`sink(a0 * 3)` with `short a0`. C promotes `a0 * 3` to `int`, so the printed
call passes the untruncated (or negative) product where the binary passes
`zext16(v * 3)`, and `sink` reads the whole register. Upstream Ghidra 12.1
prints the same `sink(param_1 * 3)`.

The same thing happens to a byte loaded with `movzbl` and handed straight to a
call: `c1(unsigned char *p) { sink(p[3]); }` prints
`sink(*(char *)(a0 + 3))`. Nothing typed the byte, so it is TYPE_UNKNOWN,
which `realtypes` spells `char`, and C sign-extends a signed `char`. Round trip
on main: 128 of 256 byte values arrive wrong (`0x80` becomes `0xffffff80`).

## 2. The root cause

`ActionDeadCode` counts only `minimalmask(nzmask)` of an unlocked call input
as consumed. `RDI = ZEXT(x & 0xffff)` therefore consumes `0xffff`, and
`RuleSubvarAnd`'s subvariable flow trims the input through `tryCallPull`:
`CALL sink, RDI:8` becomes `CALL sink, DI:2`. The p-code is still right, since
the dropped bits are known zero. The loss is in emission: `ActionSetCasts`'
default input arm compares the unlocked parameter's local type with the
argument and asks for no cast, and C's integer promotion then widens the
printed expression again. Found with an instrumented throwaway build that
printed the p-code after every changing rule and action.

The byte case has a second cause. Upstream's promotion tables
(`CastStrategyC::localExtensionType`) treat TYPE_UNKNOWN as zero-extending,
which is right for Ghidra's own `undefined1` spelling. kuna's `realtypes`
prints the same byte as `char`, so the check and the printed C disagree about
the byte's signedness.

## 3. What is still open

A 32-bit value trimmed out of a 64-bit register keeps its `int` type:
`w(unsigned v) { sink64((unsigned)(v * 3)); }` prints `sink64(a0 * 3)` with
`int a0`, and a callee defined as `void sink64(unsigned long)` then receives
the sign-extension. Casting every trimmed 32-bit signed argument would touch
almost every `int` argument on x86-64 to fix the few whose callee reads the
full register, so it is left alone and named in 09-emission.md.

## 4. Measurements

- Round trip (`roundtrip/roundtrip.sh`, eight callers, 650 values): main
  mismatches 299, the fix matches all 650.
- A 25-caller x86-64 battery with a compile-and-run driver: main mismatches
  601 values over 14 callers, a version without the unknown-byte view still
  mismatched 384 (the byte loads), and the fix mismatches none. mipsel and
  i386 builds of the same callers get the same casts.
- Whole-corpus `decompile-all` diff, two disjoint 14-binary corpora
  (`corpus-diff.txt`): every changed line is an added unsigned narrow cast at a
  call argument, or a signed truncation retyped to unsigned; nothing else
  changes, in the C or in any other JSON field.
- typesweep: `variables[]` is untouched, so type_match cannot move
  (`typesweep.txt`).
