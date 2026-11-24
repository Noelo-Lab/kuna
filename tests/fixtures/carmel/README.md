# Carmel compiler and assembler fixtures

The C89 programs and assembler source here are original. `compiler-o0.hex` and
`compiler-o2.hex` are canonical three-byte CODE slots from their linked images,
not firmware or toolchain examples. `assembler.json` records the separate CODE
and immutable CLIW bank from `issues.asm`. Rust tests do not require Wine.

Use the checked-in CODE images directly; no compiler, locator or Wine is
required to run the tests. `-O0` is the unoptimized image, and `-O2` uses
size optimization. CODE hex contains bytes in file order, with three bytes
per addressable word. `assembler.json` supplies the CLIW extent separately
as template 0.

## Fixed entries and defined input witnesses

Addresses below are CODE words. Hex array values are 16-bit DATA words.

| Case | `-O0` | `-O2` | Defined input and expected observation |
|---|---:|---:|---|
| `addone` | `0x5e` | `0x2e` | `-2, 0, 42` → `-1, 1, 43` respectively |
| `call_addone` | `0` | `0` | Same, through a separately compiled caller |
| `call_stack` | `0xc` | `0x4` | `9` → `45` |
| `call_indirect` | `0x22` | `0x14` | `(0, 42)` → `42`; `(1, 42)` → `43` |
| `call_pointer` | `0x48` | `0x24` | `(&addone, 42)` → `43` |
| `scalar` | `0x6a` | `0x30` | `(0xabcd, -2)` → `0xabce` |
| `nested` | `0x8a` | `0x36` | `(4, 3)` → `10`; `(0, 3)` → `0` |
| `bump_field` | `0xce` | `0x5a` | Two structs in the same array, fields `[1, 9, 2, 3]`; `out == in` → return `10`, fields `[1, 9, 2, 10]` |
| `volatile_pair` | `0xee` | `0x66` | `[0x1357, 1]`, no other writers → return `0x1357`, store `0` in second word; preserve two volatile reads |
| `stack_args` | `0x11c` | `0x7e` | `(1, 2, 3, 4, 5, 6, 7, 8, 9)` → `45` |
| `dot_q7` | `0x158` | `0x9e` | `[128, -128, 64, 0]` and `[32, 16, -32, 8]` → `0`; no stores |
| `fir_q7` | `0x194` | `0xae` | `[128, -128, 64, 0, 128]`, taps `[64, 32, -32]` → `[16, -48, 0]` |

The `addone` and `call_addone` p-code tests exercise `A0L` as the first `int`
argument and result and the caller's two-DATA-word return link.
The listings separately show the six half-register integer arguments before
stack spill, two-word function pointers, and independently allocated 16-bit
DATA pointers. These observations are not a full TASKING compiler model.

The [conditional-delay emission test](../../../decompiler/crates/kuna-cli/tests/carmel_emission.rs)
embeds its CODE bytes and expected returns. On 64-bit Linux with `cc`, it
compiles and executes Kuna’s generated C at `-O0` and `-O2`.
