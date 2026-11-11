# Completed Go records through existing pointers

A Go `Node` with fields `Next *Node` and `Value int` has size 16 on amd64.
Before this fix, `Next` pointed to the original empty shell: its pointee reported
size 0 and no fields even after the named `Node` was complete. The same problem
occurs between mutually recursive named structs and with ordinary forward field
references. It differs from recursive pointer aliases, which intentionally stop
at a bounded `void` fallback.

The factory already supports a completion link for forward C records. Go's shell
creation now uses that API. Existing pointers resolve to the final interned
record when its validated fields are published. Failed extent checks leave the
shell incomplete; the importer does not fabricate a size or fields from an
invalid layout. The completion link is weak, so a self-reference does not create
a strong ownership cycle.

`repro.go` and `repro.rs` reproduce the library-visible failure using a stripped
Go image. Build the Go file with `go build -ldflags='-s -w' -o node repro.go`,
then compile the Rust probe against the built `kuna_console` rlib and its release
`deps` directory. Pass the binary and spec directory as its two arguments. The
baseline prints `Node size: 16; Next pointee size: 0`.

The deterministic metadata test holds a pointer before import and checks its
resolved size, fields and self-reference after completion. A malformed extent
must keep the pointee incomplete. The real stripped-Go test checks repeated
self-reference traversal and both directions of a mutually recursive pair on
amd64, arm64, 386 and ARM. Existing tests retain by-value dependency ordering,
recursive pointer-alias bounds, and the option-off control.

## Measurements

Against `b550e8431e16520a0d5ee214421cf6b42fb396f0`, the standalone probe now
prints `Node size: 16; Next pointee size: 16`. Whole-image comparisons cover
8,671 functions in the new reproducer and Go 1.18/1.22/1.27 controls. Every
function's emitted C and every exported type declaration are byte-identical,
with zero errors on both sides. The defect is visible through the pointer's type
API, rather than the already-correct top-level type declaration.

Thirty-one interleaved CPU-pinned pairs after three warmups measure +0.32% wall
and +0.36% CPU on amd64; +0.41% wall and CPU on 386. Baseline and patched runs both
use the existing default `gotypes` behavior. Raw timings, fixture hashes, empty
whole-function diffs and type-declaration diffs are retained in this directory.

Validation passes 9,936 workspace tests (39 ignored), all 675 original assertions
and all 3,424 stage assertions, plus specification/catalog/shared-counter checks, tooling tests
and lint. Neither corpus baseline changes.

The inherited parent correction only renames count-bearing tests and updates
its validation record; it changes no runtime code.
