# Recursive Go pointer fields

A valid Go definition can refer to itself through a pointer:

```go
type Pointer *Pointer
type Holder struct { Link Pointer; Tail int }
```

Passing `Holder{Tail: 7}` to `fmt.Printf` keeps this type in the runtime metadata
of a stripped binary. On main `38b4be20b6ca9d9e86a1cf31afc1557d8d008324`, loading
that binary with default `gotypes` overflows the engine's stack: field resolution
recursively follows Pointer's element descriptor back to Pointer forever.
`repro.go` builds this case with `go build -ldflags='-s -w' -o /tmp/recursive repro.go`.

Field resolution now walks pointer descriptor addresses iteratively, with cycle
detection and a limit of 64 distinct pointer layers. Missing or cyclic targets
end at the existing opaque void-pointee fallback. Proven pointer widths and the
remaining struct fields survive. The importer still stops at a named struct's
existing shell, so ordinary `*Node` links are preserved. This does not implement
nominal aliases or claim a complete recursive named-type representation; it
makes valid Go images load without an unbounded traversal.

Synthetic tests exercise 32-bit and 64-bit self-cycles, mutual cycles, a cycle
below an initial pointer, unmapped targets, finite scalar chains and long chains.
The real stripped Go fixture also includes self and mutual pointer definitions
and an ordinary recursive Node, tested on amd64, arm64, i386 and ARM.

## Breadth and timing

The baseline aborts on all four reproducer architectures; the patched loader
recovers `main_Holder` with a machine-word `void *Link` and the correctly placed
integer `Tail`. All 6,298 functions in these four binaries decompile without
errors and match the baseline's output with `gotypes` disabled. That disabled
baseline is explicitly separate from the crashing default.

Across four existing Go 1.18, 1.22 and 1.27 control images, all 7,129 function
bodies and their recovered type surfaces are byte-identical, with zero errors
before or after. The empty diffs and input hashes are recorded alongside this
analysis.

Thirty-one alternating samples per target, following three warmups on CPU 79,
measured -0.85% median wall time on amd64 and +4.76% on i386. CPU-time deltas were
+1.20% and -3.13% respectively. Both are within the five-percent wall-time budget;
the disagreement between wall and CPU deltas is consistent with scheduling noise during
the test workload. Raw samples are retained in `timing.json`. The crashing
reproducer is not used for a misleading before/after timing ratio.

Validation: 9,920 workspace tests passed (39 ignored), 675/675 original
assertions, 3417/3417 stage assertions, catalog, spec, lint and tooling checks.
Neither parity baseline changes. The real-image tests also pass with Go 1.18.
