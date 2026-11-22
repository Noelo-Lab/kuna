# Go pointer method topology fixture

`method.S` ties a named AMD64 Go method's executable entry to its pointer
method table, Go function table, record fields, and compiler DWARF. The
lookalike and free function use the same instructions without a method-table
entry. A second caller names its receiver operand `Meter`, which shadows the
type in the call's scope. The ELF is a static decompiler fixture.

```sh
p=tests/fixtures/go-pointer-method-topology
as --64 -o /tmp/kuna-go-pointer-method.o "$p/method.S"
ld -m elf_x86_64 --build-id=none -o "$p/method-elf" /tmp/kuna-go-pointer-method.o
chmod a-x "$p/method-elf"
```
