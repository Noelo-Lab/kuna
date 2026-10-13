The assembly in this directory is independently authored and contains no application
assets. `copy_arm.s` copies five words to a mapped 20-byte object. The two `.kuna`
files vary only its array versus struct declaration. `access_x86_64.S` exercises
unaligned 1/2/4/8-byte transfers, whole-array covers, signed reads and overlapping
writes. The C drivers check every resulting byte without using the renderer's
helpers as an oracle. `bits.h` supplies the usual unsigned CONCAT/SUB bit-piece
intrinsics needed by the overlap probe; it does not alter emitted bodies.

Run from the repository root with matching built processor specs:

```sh
cargo test --manifest-path decompiler/Cargo.toml -p kuna-cli --test mapped_array_c
```

The tests compile unchanged emitted bodies with each available GCC and Clang at
`-O0` and `-O2`, strict aliasing and UBSan. ARM binutils builds both target endian
variants; their emitted C runs on the host against explicit endian byte oracles.
This does not execute the ARM ELF. The x86 probe also runs its original assembly.
Absent optional toolchains skip their checks; failures from installed tools fail
the test. `KUNA_TEST_RUNNER` can select a CLI launcher; `KUNA_BINARY` selects its
binary, allowing externally guarded runs without changing the fixtures.
