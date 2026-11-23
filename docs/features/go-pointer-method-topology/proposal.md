# Verify Go pointer receiver declarations

Go pointer methods can print as free functions even when Kuna already
recovers their receiver type and field accesses. Their direct callers
then use a mangled namespace where Go expects a method expression.

## Reproduce

Use Go 1.22.10 or 1.27.2 on Linux AMD64 with Kuna's processor specs
available.

```sh
d=$(mktemp -d)
cat >"$d/main.go" <<'GO'
package main
import "reflect"
type Meter struct { Base, Total uint64 }
//go:noinline
func (m *Meter) Apply(delta uint64) uint64 {
    m.Total += m.Base + delta
    return m.Total
}
//go:noinline
func Forward(m *Meter, delta uint64) uint64 { return m.Apply(delta) }
var result uint64
var method reflect.Method
func main() {
    m := Meter{Base: 3, Total: 5}
    result = Forward(&m, 7)
    method, _ = reflect.TypeOf((*Meter)(nil)).MethodByName("Apply")
}
GO
CGO_ENABLED=0 go build -trimpath -o "$d/meter" "$d/main.go"
kuna decompile-all "$d/meter" --language go --option godwarfsig on \
  --functions 'main.(*Meter).Apply,main.Forward'
```

Stock emits `func Apply(m *Meter, delta uint64) uint64`, keeps the
`Base` load and `Total` store, and calls
`_kuna_ident_282a4d6574657229.Apply(m,delta)`. The proposed output has
`func (m *Meter) Apply(delta uint64) uint64` and
`return (*Meter).Apply(m,delta)`. Both retain the receiver in RAX and
the scalar argument in RBX.

## First implementation

Start with named little-endian AMD64 Go ELF, a simple pointer method in
its own package, and an existing complete `godwarfsig` prototype with one
word-sized scalar argument and one word-sized scalar result.

- **P1 program preparation.** Reuse the anchored module-data, type and
  pclntab readers. Require a unique method entry with matching Ifn and Tfn,
  a normal `_func` record, a complete nonvariadic Mtyp, and a unique named
  pointee. Match the package and receiver, argument, and result runtime
  type offsets to the admitted DWARF formals. Verify the pointee's fields
  against the known layout. Use the underlying value descriptor and
  `_func` flags to reject a wrapper for a value method.
- **P0 prototype authority.** Attach receiver provenance to the exact
  parked FunctionSymbol prototype. Let replacement clear it. A fresh
  function drive carries the provenance when it selects that prototype
  without a local declaration or mapped receiver override.
- **P9 Go emission.** Use the existing first parameter's name, type and
  physical slot in the receiver position. Reuse the direct CALL printer
  with a method expression when the exact callee and its current arguments
  still match. Explicit function names and call-site prototypes take
  precedence. Preserve body operations, parameter order, and result storage.

## Evidence and acceptance

The positive method and a receiver-looking ordinary function can have
identical pointer type, parameter name, register location and field
accesses. The ordinary function lacks a matching method-table entry.
A generated pointer wrapper can have matching Ifn and Tfn plus the same
pointer formal. Its wrapper identity and the separate value entry
distinguish it.

Cover both compiler versions, the genuine method and direct caller, free
function with the same basename, ordinary receiver-looking symbol,
value method and generated wrapper, stripped and pruned metadata, and
prototype, name, and call-site overrides. Compare the field and call
operations in high P-code, compile the emitted Go with an authored
observer, and check C and Rust output plus optimized timing. Stripped
receiver binding, generic wrappers, interface dispatch, and other
architectures have separate ABI and provenance requirements.
