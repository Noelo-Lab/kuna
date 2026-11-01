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
