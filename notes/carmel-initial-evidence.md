# Carmel: initial lift evidence

## Compiler suite

The [checked-in C and assembler fixtures](../tests/fixtures/carmel/README.md)
include source, linked CODE images, entry addresses, and defined inputs.

| Source fixture | `-O0` Kuna | `-O2` Kuna | First-slice boundary |
|---|---|---|---|
| `addone` | truncated | complete | Unoptimized `LINK` frame unlifted |
| `call_addone` | truncated | complete p-code | Unoptimized `LINK` frame unlifted; C inference still incomplete |
| `call_stack` | truncated | truncated | Stack argument PUSH / wide CALL |
| `call_indirect` | truncated | truncated | Saved-register stack and conditional/indirect flow |
| `call_pointer` | truncated | truncated | Saved-register stack and indirect call |
| `scalar` | truncated | truncated | General logic and shifts |
| `nested` | truncated | truncated | Comparisons, conditional branches and hardware repeat |
| `bump_field` | truncated | truncated | Offset DATA accesses and frame forms |
| `volatile_pair` | truncated | truncated | Volatile DATA loads and frame forms |
| `stack_args` | truncated | truncated | `LINK`, stack DATA reads and general add |
| `dot_q7` | truncated | truncated | Wider MAC, DATA loads and repeat |
| `fir_q7` | truncated | truncated | Wider MAC, DATA loads and repeated/conditional flow |

Only `addone` at `-O2` has both a complete default C result and a recovered
scalar argument. No complete default pointer, nested-loop, or DSP C result
is claimed. For the stated defined inputs the returned value is `x + 1`.
The emitted expression retains the machine's optional saturation at
`0x7fff`; `ECFG` appears as a register-derived local instead of an ABI input.
Signed overflow at `0x7fff` is not a C witness.

```c
short sub_2e(short a0)
{
  unsigned short v1; // ecfg
  return (a0 + 1) - (unsigned short)(a0 == 0x7fff && v1 & 4);
}
```

The raw odd-slot CLI fixture reaches `INC A1L` and then `RETURN`.
The 16-bit TASKING scalar result is in `A0L`, so SSA removes the `A1L` write
from C as dead. The raw p-code test pins the actual `A1L` write and absence of
an `A0L` write; the stage test separately puts a live-return increment in the
odd slot so the emitted C exercises that path.

## Checked-in regression tests

- [Execution fixtures](../decompiler/crates/kuna-sleigh/tests/carmel_pcode.rs)
  exercise parallel old-state reads, INC flags, CODE-word return addresses,
  stack restoration, and the signed CLIW product. The adjacent DATA test
  checks that the load sees the old word, that the store publishes the new
  word, and that a direct entry at the load also works.
- [Raw CODE fixtures](../decompiler/crates/kuna-console/tests/verify_carmel_raw.rs)
  exercise odd-slot targets, issue-counted delays, reserved encodings, and
  read-only CLIW specialization. A readable listing does not imply that an
  instruction has executable p-code.

Run the commands in the [processor README](../specs/Ghidra/Processors/Carmel/README.md).
The tests cover an executable subset; they do not establish complete
instruction or pipeline semantics.
