# Go ABI0 inputs from linked argument metadata

A resolved `.abi0` alias identifies which convention a direct call uses. It
does not identify which bytes are inputs. The former candidate bounded its
stack model by `_func.args`, which includes both inputs and results. That
candidate printed a fourth argument for a three-input `runtime.write.abi0`
wrapper. Changing the model on `runtime.sigtrampgo.abi0` alone also made its
foreign caller appear to return an unassigned register.

The [compiler's argument-info emitter](https://go.dev/src/cmd/compile/internal/ssagen/ssa.go)
encodes offsets and widths of input values and components. The
[runtime's funcdata lookup](https://go.dev/src/runtime/symtab.go) adds the
indexed record offset to the module's function-data base. The Go 1.18 compiler
also emits argument info for bodyless ABI0 declarations, and the assembler
references it for matching package functions.

| Authored image or linked function | `_func.args` | Incoming offset/width pairs | Outcome |
|---|---:|---|---|
| Go 1.18.1 `main.store` | 16 | 0/8, 8/8 | Bind bounded stack model |
| Go 1.18.1 `main.sum` | 24 | 0/8, 8/8 | Refuse, result bytes remain |
| Go 1.18.1 and 1.27.3 `runtime.write.abi0` | 32 | 0/8, 8/8, 16/4 | Refuse, mixed widths and result bytes |
| Go 1.18.1 `runtime.sigtrampgo.abi0` | 24 | 0/4, 8/8, 16/8 | Refuse, mixed widths and padding |
| Go 1.18.1 `runtime.write1.abi0` | 28 | no ArgInfo | Refuse |
| Go 1.27.3 `runtime.write1.abi0` | 28 | 0/8, 8/8, 16/4 | Refuse, mixed widths and result bytes |
| Go 1.27.3 `runtime.gogo.abi0` | 8 | 0/8 | Refuse, special FuncID and private tail transfer |
| Go 1.27.3 `runtime.exitThread.abi0` | 8 | 0/8 | Refuse, syscall and trap loop |

Observed from file-backed metadata in the authored fixtures and their linked
runtimes. The Go 1.18 linker uses a 40-byte fixed `_func` header with the
`go.func.*` symbol, and the Go 1.27 image uses a 44-byte header and
`go:func.*`. The candidate requires a matching moduledata address, full
function record and terminal ArgInfo marker. A truncated list, aggregate,
gap or narrower argument yields no bound model. Generic names are excluded
because the compiler can omit their implicit dictionary. A positive match
proves input storage extent and width. It supplies no source type names,
signedness, or independent result storage.

The model also requires the normal FuncID and accepts only the assembly
flag. The original text symbol must bound a mapped, straight-line body
ending exactly after a decoded near return. A checked SLEIGH walk restores
decoder context and rejects earlier stack-pointer writes, branches, calls
and user operations. The positive store is 22 bytes. The marked special
`gogo` transfers control at `0x47c9ec` to a private convention. The ordinary
`exitThread` ID still needs the body check, since it leaves via a syscall.

The stage decompiles the authored two-input store and both call orders, an
ordinary function named `abi0`, and the linked runtime wrapper and signal
trampoline affected by the earlier overreach. It also pins the established
`gogo` tail transfer and `exitThread` trap continuation at their exact
entries `0x47c9e0` and `0x47f8c0`. The parser tests exercise
malformed offsets, field boundaries, missing metadata, both linker layouts
and the explicit refusal of runtime cases. These files are read as data.
