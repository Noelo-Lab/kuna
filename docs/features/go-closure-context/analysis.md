# Go AMD64 closure context evidence

Kuna loses the capture in the named leaf and the closure record at the
indirect call. The following Linux AMD64 program reproduces both failures.
The built file is used as data for Kuna.

```sh
cat > closure.go <<'GO'
package main

//go:noinline
func scaled(k int) func(int) int { return func(x int) int { return x * k } }

//go:noinline
func invoke(f func(int) int, x int) int { return f(x) }

func main() { println(invoke(scaled(7), 3)) }
GO
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o closure.elf closure.go
chmod a-x closure.elf
kuna decompile closure.elf main.scaled.func1 --language go
kuna decompile closure.elf main.invoke --language go
```

Stock `856a8960` emits the same bodies for Go 1.22.10 and Go 1.27.2. The
leaf declares `var v1 int64 // rdx` without assigning it and uses
`v1 + 8` as the capture address. The wrapper emits
`(func(uint64) uint64)(unsafe.Pointer(uintptr(*a0)))`, which Go rejects.
The named and stripped Go 1.27.1 reference fixtures have the same missing
capture in `main.scaler.func1` and invalid invocation in `main.applyAll`.
[Pinned closure tests](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/tests/analyses/decompiler/test_go_closures.py)

## Named leaf

The nine-byte AMD64 body loads `[RDX+8]` into RCX, multiplies RAX by RCX,
and returns. For Go 1.27.2, the compiler identifies `k` as a signed
eight-byte capture with `DW_AT_go_closure_offset` equal to eight. Its
only location range is RCX from the end of the load to the end of the
function. The input starts in RAX and both formals and the capture refer
to the same signed word type. The AMD64 Go ABIInternal places the closure
object pointer in RDX. Go 1.22.10 emits the same instructions without the
capture-offset attribute. The pinned unoptimized leaf is 44 bytes and
has a frame-relative result. [Go internal ABI](https://go.dev/src/cmd/compile/abi-internal#closures)

An explicit storage assertion binds the context while keeping the ordinary
input and result in RAX.

```sh
kuna decompile closure.elf main.scaled.func1 --language go --assert-strict \
  --assert 'typedef struct GoScaledContext { unsigned long entry; long k; };' \
  --assert 'param 0 RDX struct GoScaledContext *ctx' \
  --assert 'param 1 RAX long x' --assert 'return RAX long'
```

Kuna then emits `return x * (*ctx).k`. A Go observer built from that
body passed six cases including signed overflow and zero. An earlier
private stage against stock `a447beee` passed the wrong-displacement
refusal and failed its other three predicates. The strict assertions
establish the behavior of an explicit binding. Automatic import still
needs validation.

## Invocation

In the authored wrapper, the indirect call loads the entry PC from the
record at RAX, copies that same record address into RDX, and moves the
user argument from RBX to RAX before the call. In the reference
`main.applyAll`, the ordinary argument comes from RCX because the wrapper
also takes a slice. The live RDX definition is absent from the high IR.
A typed function-value prototype recovers the authored wrapper's input
declaration while leaving the invalid conversion in the call body.

An earlier authored observer gave two function values different captures
at the same entry PC. Their results differ for the same argument. The
observer also recovered each value from a slot holding the record
pointer by using the address of that slot. Taking the record pointer
itself as the slot address reads the entry PC as another record pointer.
Invocation recovery therefore needs the proven function-value record
and its exact call storage, including the live RDX definition.
