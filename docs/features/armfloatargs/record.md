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
