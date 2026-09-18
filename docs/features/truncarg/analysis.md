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

## 3. What a narrow input does not prove

A narrow call input is not always a trimmed zero-extension. clang assumes the
caller already extended a narrow argument, so `r1(signed char c) { acc8 = c;
sinki(c); }` compiles to `mov %dil,acc8; jmp sinki`: EDI goes on exactly as
the caller sign-extended it. The call input is `DIL:1` from the start (heritage
guards the call with the only range the function touched), no trim ever runs,
and main's `sinki(a0)` with `char a0` is right. An unsigned cast there passes
255 where the binary passes -1. The same holds for a sign-extension trimmed
because the callee reads only its low bytes, and for an input narrowed by
parameter recovery (libedit's `sub_b980((char)a0)`, where the binary passes
the whole RDI pointer).

So the cast is tied to evidence. `SubvariableFlow::try_call_pull` checks
whether the wide input's non-zero mask lies inside the logical mask; if so,
`do_replacement` records the slot, its new size and the call's input count on
the call's `FuncCallSpecs`. `narrowed_arg_cast` casts only a recorded slot
that still has that size and input count. `r1` and `r7` keep main's output;
`r6(unsigned char c)` is the same bytes as `r1`, so it keeps main's (wrong)
`char a0` reading too, which is a parameter-type question, not this one.

Rust output has no integer promotion (`u16 * 3` is a `u16`), so the rule is
off there (`LangCaps::integer_promotion`).

## 4. What is still open

A 32-bit value trimmed out of a 64-bit register keeps its `int` type:
`w(unsigned v) { sink64((unsigned)(v * 3)); }` prints `sink64(a0 * 3)` with
`int a0`, and a callee defined as `void sink64(unsigned long)` then receives
the sign-extension. Casting every trimmed 32-bit signed argument would touch
almost every `int` argument on x86-64 to fix the few whose callee reads the
full register, so it is left alone and named in 09-emission.md.

## 5. Measurements

- Round trip (`roundtrip/roundtrip.sh`, eleven callers including the clang
  forwarders r1/r6/r7, 719 values, compared value by value with main):
  gcc build: main wrong on 309, the fix on 0. clang build: main wrong on 309,
  the fix on 10 (r6's, which main also gets wrong), NEW_WRONG 0. The previous
  version of this branch had NEW_WRONG 20 on the clang build.
- A 24-caller x86-64 battery with a compile-and-run driver: gcc build, main
  601 wrong values, the fix 0; clang build, main 555, the fix 0.
- Whole-corpus `decompile-all` diff (`corpus-diff.txt`): two disjoint gcc/MSVC
  14-binary corpora and a clang -O2 corpus (nginx plus 21 objects from decbench
  sources). Every changed line is an added unsigned narrow cast at a call
  argument, or a signed truncation retyped to unsigned; nothing else changes,
  in the C or in any other JSON field. A 900 functions / 2,206 casts, B 455 /
  1,327, C 14 / 28. Spot checks of the clang casts against the disassembly
  find a `movzbl`/`movzwl` at each (`ngx_inet_ntop`'s four octets, which main
  prints sign-extended).
- typesweep: `variables[]` is untouched, so type_match cannot move
  (`typesweep.txt`).
