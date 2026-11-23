# Render a verified Go string input as one parameter

Go output can represent a verified string as a pointer parameter, a
length parameter and an unsafe byte load. A source declaration can group
that input and spell its guarded load as `s[index]`.

## Reproduce

Run this from a stock Kuna checkout with the CLI on `PATH` and built processor
specs.

```sh
kuna decompile tests/fixtures/go-dwarf-signatures/compressed main.stringIndex \
  --language go --option godwarfsig on --option gobounds off \
  --sleighpath "${KUNA_SPECS:-specs}"
```

Stock prints this declaration and guarded byte read.

```go
func stringIndex(s_data *byte, s_len int64, index int64) byte {
  if uint64(index) < uint64(s_len) {
    return *(*byte)(unsafe.Add(unsafe.Pointer(s_data),index * 1))
  }
  runtime.panicIndex() // no-return
}
```

This fixture's failure path calls `runtime.panicIndex`. The first
implementation covers the smaller call-free case with a returned fallback
byte and preserves the original bounds check.

## First implementation

- **P1 program preparation.** Use the existing `godwarfsig` evidence from
  one named AMD64 Go ELF string input. Record the original source name,
  the two formal indexes and their verified physical storage. The resolved
  function prototype and resolved direct call prototype carry the evidence
  only while both slots still belong to that declaration. Explicit function,
  parameter and call-site declarations take precedence, even when they use
  the same names and types.
- **Body proof.** Admit a leaf with one byte load and the original fallback
  return. The passing edge of a full-width unsigned `index < length` guard
  must dominate the load. The index must be the same value in that guard and
  in the unit-stride address derived from the input pointer. If the address
  is observed or escapes, a load is unguarded, or either input is redefined
  on the path, keep the physical contract. The typed body uses `len(s)` and
  `s[index]` and retains the fallback.
- **Shared declaration.** Use the same entry certificate for the Go
  definition, emitted prototype and direct calls. Establish it before
  emission so an isolated caller and parallel output use the same decision.
  An unresolved caller or unsupported use leaves the entry on its physical
  form. P4 continues to recover the physical words in ABI order. C and
  Rust keep their physical parameter and call representation.
- **Call operands.** Take the pointer and length from the final call
  operands. An unchanged verified pair may be reconstructed once as
  `unsafe.String(data, length)`. A shortened pair uses the live length and
  needs a dominating unsigned guard that proves it fits its original
  buffer. A mapped read-only literal can use exactly its proved bytes.
  A call with an explicit signature follows that declaration.

## Acceptance

Cover Go 1.22 and 1.27, a guarded read and its isolated declaration,
unchanged and shortened forwarding, and forwarding from different
backing. Compile emitted Go bodies against an authored observer with
zero-length inputs, negative and extreme indexes, embedded zero bytes,
and valid and invalid UTF-8. Keep the same-name explicit pointer/count
prototype, explicit per-call arity, observable empty backing, unguarded
read, and length-only physical regressions. Compare C and Rust output
byte for byte. Check both ordinary and project output, and measure the
entry proof on representative Go images before enabling it by default.

The first implementation is limited to one verified formal on named
AMD64 Go ELF. Stripped recovery, multiple strings, slices, string returns,
loop indexing and the panic case require their own proofs.
