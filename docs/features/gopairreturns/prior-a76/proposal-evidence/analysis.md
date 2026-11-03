# Go pair results

Kuna represents a Go function's two integer results as one 16-byte value. That
storage is useful inside the engine, but the Go output needs two result types,
two return expressions, and consistent calls. Correcting only the declaration
would make callers disagree with callees.

This reproduction uses Go's default optimized build with DWARF retained:

```sh
cat >/tmp/pair.go <<'GO'
package main
import "fmt"
//go:noinline
func pair(a, b int64) (int64, int64) { return a + b, a - b }
func main() { fmt.Println(pair(7, 3)) }
GO
go build -trimpath -o /tmp/pair /tmp/pair.go
/tmp/pair
decompiler/target/release/kuna decompile /tmp/pair main.pair \
  --language go --option godwarfsig on
```

The source prints `10 4`. Kuna emits:

```go
// main.pair
func pair(a0 int64, a1 int64) [16]byte {
  var v1 [16]byte // rbx:rax | rcx

  *(*uint64)(unsafe.Add(unsafe.Pointer(&v1),8)) = uint64(a0 - a1)
  *(*uint64)(unsafe.Add(unsafe.Pointer(&v1),0)) = uint64(a0 + a1)
  return v1
}
```

The larger authored fixture reproduces this with Go 1.22.5 and 1.27.3. Both
source executables agree on 105 pairs of boundary inputs, including signed
extremes, zero and negative operands. Nine complete recovered function bodies
from each image fail to compile together without body edits. Besides the
result-count mismatch, selecting the first result attempts to convert a byte
array to `int64`, selecting the second calls undefined `SUB168`, and forwarding
loses the function's inputs and outputs. The source fixture also includes a
call counter, branches, unsigned results, and unsupported-result controls for
the later implementation proof. Source-oracle agreement is not proof that
recovered code works; that proof remains to be implemented.

The metadata identifies two ordered output formal parameters, each with an
8-byte integer type. Their location expressions are empty in these optimized
images. The existing `godwarfsig` importer accepts this unavailable-location
form for a scalar result, but rejects the second output. Its other checks
validate the Go producer, register ABI, origins, input entry locations and
assigned parameter storage. The proposed extension keeps those checks and
verifies the full two-register output assignment.

The pinned angr tests require paired results for `main.divmod` and an
integer/interface result for `main.parse`. The latter needs more machinery and
is outside the first profile. See
[`check_prototypes_and_declarations` and `check_multiple_results_and_spills`](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/tests/analyses/decompiler/test_go_decompiler.py).
The first feature will preserve the scalar speller's existing `int64`/`uint64`
names; recovering Go's source-level `int` spelling is a separate gap. This
proposal does not claim that those entire tests, or overall language parity,
already pass.

The owning decision is whether complete Go DWARF evidence proves two logical
results in the ABI's joined output. P1 imports that contract. P9 consumes its
language facet for result signatures, calls and returns. The shared engine
continues to have one sized output, as specified by
[`docs/multilang.md`, W3/W4](../../multilang.md).

The physical-register interpretation follows the
[Go internal ABI specification](https://go.dev/src/cmd/compile/abi-internal),
with result allocation starting independently from input allocation. The
vendored amd64 Go specification assigns the low word to RAX and the high word
to RBX. The implementation must validate the actual assigned join, rather
than accepting any 16-byte output. The
[Go return rules](https://go.dev/ref/spec#Return_statements) also require a
multi-result call to be handled consistently in its surrounding expression.

See [the proposal](proposal.md) for the exact supported profile and proof
requirements. No production implementation is included in this draft.

## Direct reference capture

Pinned angr `6afa0c79a83d9f677a46b0275cdb63bf16c1979e` was run on both authored
images and the upstream Go 1.27.1 `divmod`. It recovered paired signatures for
all nine paired-result functions among the 17 selected function captures.
Both authored versions give the following structural results:

```go
func main.pair(a int64, b int64) (int64, int64) {
    return a + b, a - b
}

func main.consume(a int64, b int64) int64 {
    v3, v31 := main.pair(a, b)
    return v3 ^ v31
}

func main.forward(a int64, b int64) (int64, int64) {
    v3, v31 := main.pair(a, b)
    return v3, v31
}

func main.consumeEffect(a int64) int64 {
    v3, v31 := main.effect(a)
    return v3 ^ v31
}
```

These are literal reference outputs, including angr's qualified function names.
They establish the structural gap; no claim is made that the reference output
was compiled or executed. The captured `divmod` also keeps the minimum-integer
division control and returns both computed values.

## Existing join machinery

A source-true manual `GoPair` struct-return assertion makes the existing engine
preserve the two fields, typed calls, forwarding and first/second-result access
without a new pcode output model. All eight complete emitted Go bodies compile
with only the surrounding struct/global declarations and a harness. Across both
toolchains, 210 input comparisons agree with the source, including division by
zero, minimum integer divided by minus one and a counter proving the side-effect
call occurs once. The assertion records and full bodies are in `record.json`.

This is evidence for the storage design, not automatic pair recovery: the manual
contract still exposes one ordinary struct result. Automatic DWARF acceptance
and multi-result syntax remain the implementation work described in the proposal.
