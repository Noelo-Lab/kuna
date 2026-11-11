# Go types on i386 and ARM

The loader already recognizes these Go binaries and their functions. The
runtime-type pass returned immediately for every 32-bit image, leaving no
named type or Go header declarations. The angr Go tests include a 32-bit
Go 1.18 `syscall.Iovec` recovery check; the same public ABI is now asserted in
Kuna's real-binary `verify_gotypes` test alongside application-owned records.

`GoImage` carries the target word size. The base runtime type is `4*p + 16`
bytes; its kind and name-offset fields remain fixed-width values at offsets
computed from `p`. Moduledata words, slice headers, pointer elements and the
three-word struct-field records use `p` throughout. Go strings and interfaces
are two words, and slices are three. The packed Go 1.18 field offsets and the
child-first completion of nested value types continue to apply.

The extension requires a positively identified Go 1.18–1.25 build version on
32-bit images. Big-endian metadata, unknown 32-bit versions and Go 1.26+
32-bit descriptor regions are refused. The existing 64-bit readers retain
their scope. The test verifies refusal when run with a compiler outside the
32-bit supported range, while continuing to check its 64-bit images.

The whole-binary and normalized type-declaration diffs are committed beside
this analysis. The original measurement predates integration with current main;
its amd64 elapsed-time regression is retained in `initial-measurement.json`.
It is not the final speed result. The final `record.json` uses a fresh main
baseline, alternating 31 before/after samples pinned to the same CPU, and
records CPU time as well as elapsed time.

The final six-fixture comparison covers 10,165 functions with zero C changes
and zero errors on either side. Go 1.18 i386/ARM each gain 125 struct
declarations; Go 1.22 i386/ARM each gain 151. Both amd64 controls retain
byte-identical normalized type declarations. Every type-declaration hunk on
32-bit images is newly available metadata; no existing declaration is replaced.
The public Iovec and application layouts are checked by the real-image test.

The final elapsed-time medians are 512.67 → 493.94 ms on i386 (−3.65%),
115.24 → 116.64 ms on ARM (+1.22%), and 459.44 → 454.92 ms on amd64 (−0.98%).
All three are within the 5% budget. These numbers describe CLI load plus one
`main.main` decompile, not execution of the Go program.
