# Go scalar and pointer signatures from DWARF

Kuna recognizes Go register arguments, but it discards the source signature in
Go DWARF. For example, a Go 1.22.5 `func fromWord(p uintptr) *int32` consists of a
single RET instruction. Current Kuna emits `func fromWord() { return }`, losing
the input and result. `main.add` in the pinned angr basics image retains its
machine arithmetic but calls the parameters `a0` and `a1`.

The Go compiler writes type references as absolute `.debug_info` offsets, often
into a different compilation unit. Kuna's general DWARF snapshot only retains
unit-relative type references. Go also represents results as formal-parameter
DIEs with `DW_AT_variable_parameter=true`; treating every formal as an input
would install a wrong signature when cross-unit references start resolving.
Both parts must be handled together.

The reference is angr commit `6afa0c79a83d9f677a46b0275cdb63bf16c1979e`,
`angr/go/analyses/dwarf_signatures.py` and
`tests/analyses/decompiler/test_go_dwarf_signatures.py`. Its result metadata is
separate from inputs. The reference corpus is binaries commit
`76ad9e296dde8f82b8ede93ce06a578cd1933639`.

This feature covers verified scalar and pointer signatures. Go aggregates,
multiple results, receiver syntax, stripped-binary inference, source scalar
aliases and runtime idioms remain separate parity work. The existing Go output
profile continues to spell unaliased scalar types by width.

The seven pinned reference DWARF-signature tests pass against both Go 1.22.5
and 1.27.1 basics images and the langdetect image. These tests also cover
aggregate and multi-result metadata beyond this feature's initial scope.
A manual Kuna prototype assertion confirms that the existing call model and
Go emitter preserve the missing identity and narrowing values once supplied
with the source contract:

```text
func fromWord(p uint64) *int32 {
  return (*int32)(unsafe.Pointer(uintptr(p)))
}
func narrow(a uint64) uint32 {
  return uint32(a)
}
```

This confirms the loss is at signature import, before emission. Go 1.22.5 and
1.27.1 both describe the sample inputs with entry-time DW_OP_reg locations and
the results with empty locations plus the result flag. The source `uintptr`
is currently emitted as the width-correct `uint64`; alias preservation is not
part of this feature.

The implementation also handles ELF zlib debug sections. Kuna's `object` build
omits general decompression, so the Go 1.22.5 and 1.27.1 reference files supplied
no ordinary DWARF snapshots at all. A Go-only deferred path retains at most
64 MiB of combined stored/declared sections, inflates them only under the option,
and requires exact sizes, checksums and an explicit stream end. A truncated
stream regression verifies that a decoder returning without an I/O error is
insufficient proof of completeness.

The hermetic fixture separates function and type units and supplies 24 authored
functions. Twelve have supported contracts; twelve deliberately violate ABI or
metadata requirements. The native Go source fixture's fourteen recovered bodies
compile and execute without prototype assertions. A pointer to a grounded flat
record is accepted, including `main.scale` in the reference basics images;
record-by-value signatures, recursive records and record fields needing aggregate
lowering remain outside this change. Same-package Go type qualification is also
separate: the current emitter prints `*main.point`, which is checked for recovered
layout but is not included in the standalone Go compilation claim.

The whole-image comparison covers 11,814 distinct functions and 23,683 C/Rust/Go
function records across twelve images. With the option off, every record is
identical to commit `56e01194b98d64ab6e409ab27546ee5914262226`. Enabling it
changes 4,085 distinct functions (7,977 language records), including effects of
source contracts on callers, inferred parameter carriers and anonymous type
numbering. All 10,996 textual hunks are classified in `classification-final.json`;
`manual-review.json` records the remaining carrier-only changes and output
limitations. All 10,910 Go function bodies parse.

The import audit checks 1,386 actual installed contracts against independent
metadata. 1,384 agree with pinned angr's reader on input names, scalar widths,
pointer shapes and result roles. Two native `runtime.asmcgocall` wrappers have
the same names as separate no-formal ABI0 DIEs selected by angr's name-keyed map.
Independent DWARF inspection and Go's objdump establish their RAX/RBX inputs and
32-bit results. Kuna binds the source contracts by address. The two collision
JSON files and `asmcgocall*.txt` retain that evidence.

Go 1.27.3 reproduces the CI failure on one-byte identity functions: it gives
their inputs a zero-length entry range in DWARF 5. The general DWARF reader
correctly excludes empty ranges. This Go-specific path accepts only an exact
entry base followed by one empty register range, with a declared one-byte
extent and a corroborating C3 RET instruction. It still verifies the expected
ABI register and rejects empty result ranges. Seven authored negative cases
vary the register, range, body, extent, multiplicity, base or result role.
The identity and fourteen native bodies compile and execute under both Go
1.22.5 and 1.27.3. Across the original ten images, this revision changes only
`runtime.gclinkptr.ptr` and seven callers in Go 1.27.1 (eight C and eight Go
records). The revision diffs and `revision-review.json` record that boundary.

The Go emitter's existing `unsafe.Pointer("literal")` conversion appears in
runtime fatal-message callers once their pointer formals are known. It parses
but does not type-check; converting through the string's data pointer is a
separate follow-up. Neither arbitrary runtime bodies nor the same-package record
case are included in the standalone compilation claim. The feature remains an
explicit opt-in, including under aggressive mode.

The final timing experiment uses 31 rotating samples per arm, three warmups
and CPU 79, after this task's builds and sweeps completed. Disabled-option
median changes reach -0.04% wall and
+1.23% CPU across six controls. Enabling
import reaches +7.63% wall and
+7.54% CPU. These are startup-inclusive
single-function measurements; the full-image comparison is an output audit.
The option remains explicitly off in every mode. Every timing artifact reports
both ratios of medians and median paired changes. `timing-rust-first.json`
retains the original pre-repair noisy Rust sample for provenance; the six
current artifacts measure the repaired implementation.

Validation: 9,998 workspace tests passed, none failed, with 39 ignored across
71 suites; original datatests remain 675/675; stages are 3,458/3,458. Tooling tests,
lint, spec links, the catalog and shared counters pass. The full suite used Go
1.27.3; all seven focused signature tests also pass with Go 1.22.5. The original
regression baseline is unchanged.
