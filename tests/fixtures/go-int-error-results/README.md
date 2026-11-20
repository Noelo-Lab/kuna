# Static Go int/error contracts

`python3 build.py` assembles and links these authored ELF images. It never
executes them; both outputs have mode 0444. `contracts` uses zlib DWARF and
`contracts-plain` has identical code with uncompressed DWARF.

The three branches in `main.check` return `(17, table, 0)`, `(23, table, data)`,
or `(n+1, 0, 0)` in RAX/RBX/RCX. `main.forward` passes that result through a
call. `main.consume` tests the table for nil before comparing the expected
table and independently testing data, distinguishing a typed nil from a nil
interface. The fixture models those storage operations and does not implement
an executable Go runtime or interface method table.

Its cross-unit DWARF reproduces the observed Go 1.22.10 and Go 1.27.2 result
declarations: signed eight-byte `int` with Go kind 2, then `error` with kind
20 through a sixteen-byte `runtime.iface` with `tab` and `data` fields. The
table pointer targets `runtime.itab` or `internal/abi.ITab`; the data pointer
is `unsafe.Pointer`. Result locations are explicitly empty. The pointee
tables remain opaque in this fixture, as in the imported storage carrier.

Negative functions cover `int64` instead of `int`, a struct kind instead of
an interface kind, overlapping interface fields, one register instead of the
complete composite location, an absent location, a third source result,
an interface input, a producer without regabi, repeated entries and ABI0.

The stage and scoped Rust tests inspect these files through the ordinary
loader/importer. They do not execute the images. High-level Go `(int,error)`
syntax and recovery without DWARF are outside this fixture's contract.
