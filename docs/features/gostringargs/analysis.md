# Go string parameters remain separate machine words

On both Go 1.22.5 and 1.27.3, an amd64 function declared `Length(s string) uint64` currently decompiles as:

```go
func Length(a0 uint64, a1 uint64) uint64 {
  return a1
}
```

Reproduce it with an installed Go compiler:

```sh
cat >/tmp/string-argument.go <<'GO'
package main
//go:noinline
func Length(s string) uint64 { return uint64(len(s)) }
func main() { println(Length("abc")) }
GO
go build -o /tmp/string-argument /tmp/string-argument.go
decompiler/target/release/kuna decompile /tmp/string-argument main.Length \
  --language go --option godwarfsig on
```

The pinned angr revision recovers the single `string` parameter. Its body is pseudocode rather than a native correctness oracle: this example returns Go's `int` length directly from a `uint64` result, and its byte-loop examples also replace numeric byte iteration with UTF-8 range stepping. For `é`, the authored byte sum is 364, while the reconstructed range form sums 195. Kuna must recover the string contract while preserving the original byte behavior and integer conversions.

DWARF from both compiler versions identifies kind 24, a 16-byte structure with `str: *uint8` at offset zero and `len: int` at offset eight. Entry locations describe two complete eight-byte register pieces. The current source-signature importer accepts scalar locations and rejects these composites; the existing input model also requires explicit grouped storage for a two-word string. The source-contract change belongs to P0 knowledge, joined storage uses the existing P4 machinery, and rendering belongs to P9 emission.

The research includes 18 distinct string/call witness functions plus a separate Pointer function. The main 16-function source produces 1,024 top-level results per compiler. A second four-function bundle, sharing ByteSum and Mixed, adds static literal calls and two ordered scalar callbacks around a string argument; it produces 256 results and 64 effect traces per compiler. Complete current emitted bundles fail compilation. The Pointer reference covers ten header representations, including two empty strings with distinct non-null backing pointers. No candidate implementation or repaired-output result exists yet.

Authored emission constructs preserve exact header bits, contents and evaluation order on both compiler versions under default, unoptimized and pointer-checking builds. A separate named-header update construct covers all 100 pairs of those representations and preserves the caller's value. These establish candidate expression semantics only; complete emitted bodies still need independent native comparison. An authored 30-function DWARF fixture supplies positive and malformed contracts in compressed and plain forms. Current parent records match across both forms and its scalar control is recognized; future option acceptance and refusal remain untested.

The proposed `gostringargs` option requires `godwarfsig` and a complete validated amd64 ABIInternal input contract. Ordinary two-field structs, partial or reversed pieces, unsupported stack placement, hidden contexts and ambiguous origins must not acquire string meaning. String results, slices, interfaces, other architectures and general source-level Go parity remain separate work. See `proposal.md` for the storage, rendering and validation design.
