# Literal print calls through relocated function pointers

The call deindirection path records `_print` or `_eprint` as the call name and
keeps `std::io::stdio` in the function's symbol scope. The literal recognizer
previously accepted only a single fully qualified name. The same runtime call
therefore recovered on a direct path but remained explicit after GOT resolution.

Match either the full standard name or the leaf at its exact symbol entry with
the complete namespace chain. Do not infer the standard library from the leaf
alone. This corrects the existing `rustfmtliteral` behavior; its version,
readonly-byte, argument-count and output-language checks are unchanged.

`repro.S` is a minimal synthetic Rust 1.93 ABI fixture. Assemble with
`as --64 repro.S -o repro.o`, link with
`ld -pie --no-dynamic-linker --no-relax -e show repro.o -o repro`, and run
`kuna decompile repro show --language rust`. Before the fix the call is
`_print(0x2000,0xd)`; after it is `println!("hello")`.

The committed deterministic fixture covers stdout, stderr, an unrelated root
namespace, a missing namespace, an extra namespace, and a global `_print`.
Executable tests compile the recovered functions and check exact stdout/stderr
bytes. C rendering remains unchanged in those comparisons. The stage case runs
with the existing option off and on, then checks all four unrelated names.

## Measurements

The baseline is `b550e8431e16520a0d5ee214421cf6b42fb396f0`, including primitive
print argument recovery. Six whole-image comparisons cover 8,278 distinct
functions and 8,290 language/function surfaces. Only the two new scoped print
wrappers change, with one call-line replacement each and every other line
unchanged. All runs complete without decompilation errors. Direct literal calls,
primitive argument calls, the real Rust 1.93 template fixture, and 7,763 functions
in the older Rust layout control are byte-identical. C output for the scoped
fixture is also byte-identical.

Each timing uses 31 interleaved samples after three warmups, pinned to CPU 79.
The new scoped stdout case changes from 123.51 ms to 122.60 ms (−0.74%); the
larger Rust 1.93 literal control changes from 403.78 ms to 401.70 ms (−0.52%).
Raw samples and off/on controls are in `timing-scoped.json` and `timing.json`.

Validation passes 9,936 workspace tests (39 ignored), all 675 original assertions
and 3,427 stage assertions, plus specification, catalog, shared-counter, tooling
and lint checks. The original baseline is unchanged; the stage baseline adds
three assertions. The inherited parent correction only renames count-bearing
tests and updates its validation record; it changes no runtime code.
