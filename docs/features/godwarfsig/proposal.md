# Recover Go scalar and pointer DWARF signatures

Add a `godwarfsig` analysis option and an isolated Go DWARF reader arm. Read only
Go compiler compilation units, resolve both unit-relative and absolute type
references, and distinguish inputs from result formals using the recorded flag.
Keep the ordinary C, C++, Fortran and Rust DWARF paths unchanged.

Start with little-endian amd64 Go register-ABI functions whose whole signature
can be represented: fixed scalar/pointer inputs and zero or one scalar/pointer
result. Verify compiler/ABI evidence and the argument storage against the Go
register sequence; refuse stack overflow, unsupported composites, ambiguous or
missing metadata, ABI0 wrappers, compiler trampolines and closure bodies with hidden context.
A result is never installed as an input.
Use bounded reference traversal and conservative refusal for cycles and broken
references. Resolve concrete out-of-line abstract-origin metadata only where
its identity and complete parameter list are grounded.

Retain the existing per-unit snapshots for the Go graph instead of parsing every
unit twice. Keep Go-only absolute references and result/location attributes in a
side record, so other languages retain their existing unit-local interpretation.
Defer type interning as well as prototype installation until the option is on;
collecting a disabled feature must not alter type identity or field layouts.

Carry the recovered facts separately to the deferred analysis commit, where
`godwarfsig` and the active Go ABI model select them. Bind by function address,
so alias normalization cannot redirect a signature. Preserve explicit user
prototype precedence. Existing inference remains available for rejected cases.
The implementation must recover values even for identity functions with only a
RET instruction, and propagate contracts to calls as well as definitions.

Implement new logic in `kuna_godwarfsig.rs` under the DWARF analyzer. Add option
metadata, dispatch and live-reader wiring; document the contract in the P0/P1
spec owner. Keep the option off until ablation, speed and corpus evidence justify
default-on. Do not change the original regression baseline.

Tests include a compact hermetic ELF with realistic cross-unit Go DWARF and a
native Go 1.22.5 fixture, scalar widths and signedness, bool, float, pointers,
identity/narrowing returns, parameter names, callers, zero results, result flags,
unsupported multi-results/aggregates, wrong producer/ABI, unresolved references,
and malformed graphs. Compile and execute supported recovered Go bodies.
Compare the pinned angr DWARF cases within this feature's scope, sweep all
changed functions, measure off/on and pre-change startup/decompilation time,
and run all repository gates before each commit.

This is a proposal because it extends type-reference and prototype-import
infrastructure. The user's standing instruction authorizes implementing and
merging the feature after validation.

Primary ABI reference:
https://go.googlesource.com/go/+/refs/tags/go1.22.5/src/cmd/compile/abi-internal.md
Go compiler DWARF producer:
https://github.com/golang/go/blob/go1.22.5/src/cmd/internal/dwarf/dwarf.go
