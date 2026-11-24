# Carmel compiler and assembler fixtures

The C89 programs and assembler source here are original. `compiler-o0.hex` and
`compiler-o2.hex` are canonical three-byte CODE slots from their linked images,
not firmware or toolchain examples. `assembler.json` records the separate CODE
and immutable CLIW bank from `issues.asm`. Rust tests do not require Wine.

## Regenerate

Use TASKING `cccarm` v3.1r2 Build 021, `ccarm` Build 083, `ascarm` Build 141,
and `lkcarm` Build 040 in a private directory. Compiler fingerprints (SHA-256):

```
cccarm 52337d0863173a1bbf82f15e877ad8d468d44badf090b1d15de0e6eb0296e2c3
ccarm  73834d011bef6b92d900b43488f18dd855d1e2a5f9e5f83b9cdcecdbd39355d5
ascarm b7f25403241e194358a9276d375ad0ac1ba4f7a663c173089b255527aa66d259
lkcarm 90b8edb67471cd10cff17c6861461f51685b98482301d1028d1debd40d394dae
lccarm f27ae94544f8c91201efb4a2429ced285ec48e5818ae8e15c85a02004dfa3540
```

The 3.1r2 locator crashed in Wine 11.22 before producing a map. The locator
fingerprint above is v3.0r6 Build 037; it reads the 3.1r2 linker output. Use
`carm.dsc`, `carm.cpu`, `carm.mem`, `carm.def` and `carm.i` from the toolkit's
freestanding examples, and a fresh `WINEPREFIX`. In separate level directories:

```sh
for f in scalar caller dsp; do
  wine cccarm.exe -v -tmp -c -A0 -O2 -Wa-l -o "$f.obj" "$f.c"
done
wine lkcarm.exe -L -ofixture.out scalar.obj caller.obj dsp.obj
wine lccarm.exe -f3 -M -dcarm.dsc -o fixture.hex fixture.out
```

Repeat with `-O0` (no optimizations); `-O2` is the compiler's usual size level.
The `.lst` files are per translation unit, `fixture.map` fixes linked symbols.
This is freestanding (`-L` excludes the system library); an absent `__START`
is expected. Convert Intel HEX payload addresses in units of **three bytes**,
verify the checksum and complete contiguous coverage, and render raw bytes as
hex. For assembly use `wine cccarm.exe -v -c -Wa-l -o issues.obj issues.asm`,
then `lkcarm` and `lccarm` as above. The assembler image has a distinct CLIW
extent at native locator word `0xe80000`; classify it separately as template 0.

| Evidence (SHA-256 of native output) | `-O0` | `-O2` |
|---|---|---|
| linked Intel HEX | `a3b5ab1d3c140c46f9259da2ce5c41e1d62a3111b949d2c1b57ae6082bb823f2` | `74643b3ba5d3fe2ba96f31b0e8687088edd4f5028d64ce6fafeece3c1e218024` |
| linker map | `d60057492bcf6c0f8f0fc1e97f0b92d1e8c6cfdbbae612517c85c0bfafed3758` | `c69064e3d61c786e7320f2add2cf226921590ca8119b56c0c29cd7825f0df484` |
| canonical CODE | `f9d62709e404ae65a1eabedb133ca0f48904d3617dc9eacd19784cc60ffa35ff` | `0164bab742352a364e7f9c4b0e90796b84f9fe8736a20358a44cda5f9d88b983` |

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

The simulator confirms `A0L` for the first `int` argument and result, `R7`
restoration across a caller and callee, and a two-DATA-word return stack frame.
The listings separately show the six half-register integer arguments before
stack spill, two-word function pointers, and independently allocated 16-bit
DATA pointers. These observations are not a full TASKING compiler model.
