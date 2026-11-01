# Go DWARF signature fixture

`python3 build.py` uses GNU as/ld/objcopy to build the two committed amd64 ELF
images. Their authored instruction sequences follow Go's internal register ABI;
the DWARF and inline Go build-info marker are synthetic. `compressed` has the
same metadata as `uncompressed`, encoded as ELF zlib debug sections.

The function and type compilation units use absolute type references, Go kind
attributes, input/result flags and entry register locations. Twelve contracts
cover integer widths, bool, independent float/integer register banks, scalar
pointers, a pointer to a flat two-field record, caller propagation, void returns,
result-only functions and a concrete abstract-origin instance. Twelve controls
cover a wrong input register, a missing result-role flag, multiple results,
compiler trampolines, a closure suffix, broken type references, oversized scalars
a producer without register-ABI evidence, wrong result locations, stack inputs
and integer/float register overflow.

`verify_godwarfsig.rs` compares compressed and uncompressed recoveries, checks
rejected contracts against the disabled option, and compiles/executes eleven
supported Go bodies without prototype assertions. The record-pointer body is
checked separately because the current Go printer retains `main.point` in a
same-package type reference. The same tests build the native Go fixture from
`../go-output/output.go` and execute fourteen recovered functions.
