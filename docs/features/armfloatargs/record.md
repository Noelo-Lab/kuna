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
metadata controls retain baseline output. Input recovery also works with
`armfloatreturn` off; that setting retains legacy return recovery.

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
  inputs. Previously, with `armfloatreturn` off, it became an uninitialized
  local (Cortex-M4F `double` helpers; betaflight `sub_80094e0`).
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
