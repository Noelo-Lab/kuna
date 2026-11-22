# Recover one named Go map allocation result

Kuna can print a direct `runtime.makemap_small` call with a raw pointer
result or omit the returned value. A complete source map declaration can
identify the fresh result independently of the runtime helper.

## Reproduce

Use Linux AMD64, Go 1.22.10, and a stock Kuna checkout with built specs.

```sh
d=$(mktemp -d)
cat >"$d/main.go" <<'GO'
package main
type Key uint32
type Value uint64
type Registry map[Key]Value
//go:noinline
func NewRegistry() Registry { return make(Registry) }
//go:noinline
func WithHint(n int) Registry { return make(Registry, n) }
var x, y Registry
func main() { x, y = NewRegistry(), WithHint(32) }
GO
CGO_ENABLED=0 go build -trimpath -o "$d/maps" "$d/main.go"
chmod a-x "$d/maps"
kuna decompile "$d/maps" main.NewRegistry --language go \
  --sleighpath "${KUNA_SPECS:-specs}"
```

Stock prints this direct runtime call with a physical result.

```go
func NewRegistry() uint64 {
  return runtime.makemap_small()
}
```

The proposed Go spelling is `func NewRegistry() Registry` with
`return make(Registry)`.

## First implementation

- **P1 type and target proof.** Use the existing `godwarfsig` certificate
  for one complete, named map result on AMD64 Go ELF. Its exact Go map
  typedef, key and element must agree with a mapped runtime descriptor
  anchored to the same module. Check its one-word physical representation
  and independently verify RAX result storage under ABIInternal. Use the
  validated executable `pclntab` entry for `runtime.makemap_small` and
  reject ambiguous or missing targets. The helper's name, return register
  and private map layout cannot establish the source map type.
- **P4 result and effect proof.** Use the selected prototype and existing
  call-site locking machinery. Admit one direct zero-argument call whose
  full-width result reaches the returned RAX unchanged, with no other
  hot-path calls or observable effects. The ordinary verified Go stack
  growth path can keep its existing treatment. Retain the allocator as a
  side-effecting operation and preserve explicit function and call-site
  prototypes. Keep the new source contract local to the proved wrapper.
- **P9 Go result.** Bind the source result to the named map type and render
  one `make` expression at the existing producer. Keep its evaluation count
  and return path. Use the ordinary Go name and type-declaration surfaces
  for the nominal map and its key and value. C and Rust retain their physical
  representation.

## Acceptance

Check single-function and selected batched Go output on Go 1.22 and 1.27.
The named result must be fresh, empty, nonnil and writable. Two calls
must return independent maps. Verify native Go type identity and key
and value widths. Include a same-shape different map with absent runtime
identity, raw pointer result, nil path, dynamic hint, additional effect,
two allocations and explicit declarations. Preserve physical results
with the option disabled, and compare C and Rust with stock.

The first contract covers one named scalar-key and scalar-value map from
one proved allocator. It leaves global and field destinations, zero-size
values, dynamic hints, interface carriers, indirect calls and map lookup
for separate work.
