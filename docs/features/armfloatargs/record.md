# Scalar ARM VFP input recovery

`armfloatargs` is off by default and excluded from presets. It requires a
container that positively states the VFP argument convention. Without a declared
prototype, recovered scalar VFP inputs precede core-register inputs; this is a
consistent ABI order, not a reconstruction of an unavailable source declaration.
Declared parameter order remains authoritative.

Validation uses authored instruction fixtures and C compiled for ARM and Thumb
at O1, O2, O3 and Os. Cases cover both mixed source orders, forwarding callers,
multiple doubles, a double input with a float result, earlier call clobbers,
conversion-only and conditional leaves, and variadic base-convention calls.
Narrowing controls also cover tail and ordinary wrappers, a second double or
float parameter, and compilation/execution of the recovered caller/callee pair.
Three authored-instruction regressions fail before forwarding claims are made
ahead of synthetic reads and reconstructed incoming pieces are recognized.
Recovered mixed callers and callees were compiled as host C and executed to
check the integer and floating values. Soft-float and absent/ambiguous ABI
metadata controls retain baseline output. The option takes effect only with
`armfloatreturn` on; alone it leaves output unchanged.

The stage uses the existing redistributable `fmtlf_armhf` fixture and its adjacent
source. The original corpus remains 675/675. The stage baseline adds four passing
assertions (three here and the catalog default); no existing expectation changes.

For the initial implementation (`50cc5130`), twenty timed
`decompile-all --mode aggressive` repetitions after warmup on an
idle host compared source-built release binaries against main `ea7f6597`:

| Input | Baseline median | Option off | Option on |
|---|---:|---:|---:|
| Authored linked hard-float C | 47.69 ms | 48.16 ms (+0.98%) | 47.99 ms (+0.63%) |
| Existing format fixture | 50.10 ms | 50.98 ms (+1.76%) | 51.42 ms (+2.63%) |

Every option-off output was byte-identical to the baseline. These small inputs
establish a bounded speed measurement; they do not justify preset promotion or
a default change on a broader corpus.

The forwarding correction was measured against `50cc5130` with twenty interleaved
repetitions after warmup. On the expanded narrowing fixture, median time changed
from 51.23 to 51.38 ms with the option on (+0.30%); on the existing format fixture,
52.45 to 52.07 ms (-0.74%). Option-off medians changed by -0.32% and -0.60%, with
byte-identical output. Across ten ARM/Thumb compiler variants, only the narrowing
wrapper's return and the two affected caller argument lists changed. All ten
recovered variants compiled and executed with the expected values.

## Review corrections

Three option-on defects were corrected, each inside the option:

- A callee stating s0 and s1 as separate words now gets both words at a call
  whose caller heritages the whole d0, and a hole below a later double follows
  the stated words. Previously the call read a stale d0 (an earlier double
  result) and lost its own float result.
- A d-register input read only as its two words becomes the two s-register
  inputs when floating operations consume both words (see the second round
  below for the integer-word case).
- Passthrough no longer hands one forwarded word to a stated double.

Option-off output of all 359 in-repo binaries stays byte-identical to main in
default and `--mode aggressive`, and `armfloatreturn`-only output is unchanged.
With the option on, 10 functions change against the first implementation, all in
`armfloatreturn_armhf.o` and betaflight: uninitialized d-register locals become
parameters. Two betaflight tail-call wrappers of a word-split callee now pass no
arguments; the spec lists that shape as a known limitation.

On an idle host, betaflight (5,087 functions, `--mode aggressive`) took a median
46.20 s user time with the option off and 46.52 s on (+0.7%, three runs each);
twenty `fmtlf_armhf` runs took 1.91 s and 1.95 s.

## Second review corrections

Three option-on shapes printed calls that disagreed with their callees:

- `g1(float, float, float, double)`: the callee listed the back-fill slot s3
  as `float a3`, so its callers read s3 (a stale half of d1, or at -O0 a phantom
  input of the caller) and, at -O2, also the 3.0f the caller left in s6. The
  back-fill slot is no longer filled in, and a call takes exactly the VFP inputs
  an arity-sound callee contract states. Six clang ARM/Thumb builds (-O0 to -Os)
  of that source now compile and return the source's values on the host.
- A double read only as two integer words (Cortex-M4F code handing it to
  `__aeabi_dmul`; crazyflie `enqueueTDOA`, newlib `__d2b`) became two
  integer-typed s-register parameters that no caller passes. Words are now split
  only when floating operations consume both; integer words keep the
  `armfloatreturn` output.
- With `armfloatreturn` off, an unused leading float parameter moved the next
  argument into its place. The option now requires `armfloatreturn`.

## Third review corrections

Three option-on shapes still printed calls that disagreed with their callees;
`armfloatargs_calls.c` (fixtures in `kuna-analysis/tests/fixtures`, clang 14,
ARM and Thumb at -O0 and -O2, and Thumb -Os) holds one of each:

- `x3(float, float, float, double)` passes its double to `k7(float, float,
  double, float)` in d1. The argument was typed by comparing the caller's
  storage (d2) with the callee's (d1), so the double stayed two words and `x3`
  printed a phantom `s3` and two integer words its callers never passed. A value
  passed to a call now takes the type the callee states for the slot it is
  passed in.
- A call into a non-leaf back-fill callee kept a positional filler in `s3` on
  Thumb, and `fn5(float a, float unused, double d)` lost `unused` although its
  -O0 body spills it from `s1`. The back-fill rule now keeps a slot the
  function's own body reads (a probe of its own entry, taken only with the
  option on), and a call drops a filler its callee's contract skips.
- `yd(float a, double unused, double d)`: clang leaves a scratch constant in
  d1, the trial scoring took it for the caller's own value, and the inactive
  chain ended the argument list there, dropping `d`. A call to a callee with an
  arity-sound contract now takes the contract's VFP inputs up to the last one
  the caller wrote for the call or the callee is seen to read.

That frontier, and two more limits, came from a firmware sweep. A wrapper such
as betaflight's `cosf` (`sinf(x + pi/2)`) states sixteen floats because its
forwarding walk into `sinf` cannot finish; binding every stated input gave its
callers sixteen arguments and their own callers phantom parameters, so inputs
past the frontier are left alone. A stated input the caller's scoring rules out
has had its value replaced by zero, so it is kept only where the callee
provably ignores the register. A call that recovered no argument at all to a
callee with core-register inputs is left to the empty-call rescue, which
recovers both banks.

With the option on, each of the five builds prints `x3`, `v2`, `fn5` and `yd`
with exactly their callees' parameters, and the printed C, compiled on the
host, computes the source's `top`; before, every build failed at least one of
the four. On 20 firmware images (cf2, cleanflight and betaflight at O0, O2 and
O2-noinline, libopencm3, RIOT, ChibiOS, NuttX, FreeRTOS), option-off output is
byte-identical to the previous head and to `armfloatreturn` alone. With the
option on, 12 images are unchanged against the previous head and 8 change in
6-48 functions each; the callees whose callers disagree with them on arity fall
by 4-31 per image, and one cf2 callee
becomes newly inconsistent because its recovered prototype gained the float the
binary passes in `s0` while a caller still lacks a core argument the compiler
kept live across an earlier call (`-fipa-ra`). Spot checks against the
disassembly (`sqrtf`, `__ieee754_rem_pio2f`, `__kernel_sinf` and pt2-filter
calls) show the added arguments carry the values the binary passes.

## Whole stated doubles and leaf back-fill slots

Two more call shapes printed more arguments than their callee lists:

- A float-returning wrapper that hands its double to `wl(double, float)` in d0
  and uses it again after the call writes only s0 itself, so heritage reads
  that d0 as two words and the call held them as two word trials:
  `wl(SUB84(a0,0),SUB84(a0,4),0x40600000)`. A stated double the call holds as
  its two words is now passed as one PIECE of them in the low word's slot, so
  the call prints `wl(a0,3.5)`.
- `top` passed `hole6(double, double, double, float, double, double)` the
  back-fill slot s7, which it writes only after the call, as a seventh
  argument, and that read gave `top` eight phantom parameters. The rule that
  drops a VFP register the callee neither reads nor forwards never fired on
  ARM, because every `bx lr` switches the instruction set through a user
  operation before it returns and the forwarding walk counts a user operation
  as an opaque transfer. The entry probe now notes a user operation whose
  instruction then returns unconditionally, so a leaf that only returns
  forwards nothing.

On 60 generated float/double programs at six ARM and Thumb builds (-O0 to -Os),
programs whose printed C computes the source's value went from 89 to 97 of
360, and calls with more arguments than their callee from 36 to 7. The seven
are all -O0 builds of a callee whose unused trailing float is only spilled,
now listed among the limitations; with the option off the same calls also
pass more arguments than their callee lists. On 80 hand-written and generated sources at nine builds
including Cortex-M4F, 194 then 219 of 720 builds match, and the calls with too
many arguments are a strict subset of the previous head's. On 23 firmware
images (the 20 above plus betaflight O2-noinline, lcd-dma O0 and mandel
O2-noinline), option off is byte-identical to the previous head merged with the
same main. With the option on, 19 images are unchanged and four change: cf2
O2-noinline in 6 functions, each a VFP argument dropped from a call to a callee
that never reads it (`vmul.f32 s0, s0, s15; bx lr`, or `vneg` of s0-s2 into an
empty stub), and the three betaflight builds in 4 functions each, where a `sqrt`
wrapper that moves its d0 into r4:r5 is now `sub_8008190(float8 a0)` instead of
`(int4 a0, uint4 a1)` and its three callers pass it the converted double they
leave in d0 instead of nothing. Each change was checked against the
disassembly; no callee became newly inconsistent with its callers.

## Callee-ignored stated inputs

Joining a stated double's two words could make a caller read a register it
never wrote. In `float w3(float a) { return i4(8.25, a, a, 5.25) * 3 + a; }`,
`i4(double, float, float, double)` never reads its first parameter, so clang
leaves d0 holding `w3`'s own float in s0 and whatever s1 held on entry. Building
the double from those words gave `w3` a phantom high word: it printed
`float w3(double a0)`, and `top` passed it a `CONCAT44` built from an
uninitialized local, which compiled and computed the wrong value. The same
shape held as one d0 read (`u1`, `u2`, `u3` in `armfloatargs_ignored.c`)
printed `float u1(double a0)` with `SUB84(a0,0)` for the float. Two more shapes
had the same cause, a read of a register the callee never looks at:
`k3(float, float, float)` ignores its second float and `top` calls it with
`q2`'s float result still in s0, and `wk` calls `kd(double, float)`, which
ignores its double, right after another call. Passing the s1 above the earlier
float result made that result an `unsigned long long` converted by value.

The two words are now joined only where the callee's body, followed through
its own calls, reads both of them, and never where passing them whole would
read more of the caller's entry registers than the caller itself does. A stated
input the callee never reads (it writes the register first, or is a leaf that
returns without touching it) is zero where the caller's value would widen its
own entry registers that way, is an earlier call's leftover, or does not exist
at the call at all: `w3(float a0)` calls `i4(0.0,a0,a0,5.25)`, `top` calls
`k3(v6,0.0,v6 + 7.0)` and `wk` calls `kd(0.0,0.0,a0)`. A double the caller
computed, forwards untouched or also reads whole is passed as before.

On 220 sources at six ARM and Thumb builds (the gen2 p/q seeds 31-60, the
r3 a/b sets, gen3 r and the hand-written set), 483 of 1320 builds print C that
computes the source's value, against 443 for the previous head merged with
the same main; the 40 that changed all went to the source's value (24 from a
wrong value, 16 from C that did not compile), and nothing else moved. The
hand-written set at arm -O3, Thumb -Oz and ARMv8 moved the same way (19 builds,
none back), as did one Cortex-M4F build, and 60 more generated sources with
mostly unused parameters gained 4 builds. The 23 firmware images print the
same as the previous head with the option on, and the same as main with it off.

## Parameters handed back on some path

Zero is only safe where the callee never reads the register, and two kinds of
callee were taken for ones that never do. `float clampf(float a, float b, int
c) { if (c > 3) { a = ...; } return a; }` compiles to `cmp r0,#4; bxlt lr;
...`: the early return hands the caller's s0 back. The leaf rule took a body
that only returns without touching s0 as one that never reads it, so a caller
holding an earlier call's result there passed `clampf(0.0,2.5,a0)`, which
compiled and computed the wrong value (the same for the double `clampd`). And
`float pick(float a, float b, int c) { if (c > 3) a = b; return a; }` compiles
to `cmp r0,#3; vmovgt.f32 s0,s1; bx lr`: the callee-body walk credited the
conditional move as writing s0 on both paths, so the register looked dead and
the call bound zero. The walk also stopped at the user operation inside
`bxlt lr`, so the code after a conditional return was never decoded.

With the option on, a register now counts as written on every path only where
instructions that always run write it, and the leaf rule needs a walk that left
no path behind and saw no conditional write to the register; the walk records
both facts without changing what it answers anywhere else. A register counts as
never read when every path writes it that way, or when the callee is a leaf
that returns without touching it and every part of it inside the storage of the
callee's recovered result is written on every path first (statements now record
that storage even where some return does not compute the result).
`calleedeadarg` asks for the same firm proof in a run with the option on, so
`int pick(int a, int b, int c)` (`movgt r0,r1`) keeps its first argument as
well; with the option off it is unchanged.

Making the walk itself follow both paths of a conditional instruction was tried
first and rejected on the firmware sweep. A literal pool in front of a Thumb
function can decode as an IT instruction (cleanflight's `0x3fc90fdb` before
`cosf` reads as `lsrs; itett gt`), which leaves the function's first
instructions marked conditional. The per-path walk then saw reads of registers
those instructions always write, and calls to such callees on cf2, cleanflight
and betaflight gained up to sixteen phantom float arguments.

Two follow-on cases came out of the sweep. A back-fill slot inside a double
result (`L3(float, double, double)` returning in d0, with s1 never read) is
still dropped where every path writes it first. And a stated double the callee
reads only in its low word, where the caller holds an earlier double result's
high word above its own float, passes zero in that word instead of turning the
earlier result into integer bits.

An earlier call's whole result passed into a slot the callee never reads is
zero like any other leftover. A variant that passed such a result on instead
was measured on the same sweeps and changed no build's outcome, so the smaller
rule stays: the zero only lands where the callee provably never reads the
register.
