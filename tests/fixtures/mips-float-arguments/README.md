# MIPS floating-point argument slots

GCC 7.3 PIC shared objects whose callers pass constants to imports. The
callees' prototypes mix floating-point and integer arguments. The `use_*`
functions are callees with the same signatures.
`tests/stages/kuna-mips-float-arguments.xml` declares each import with
`map prototype` and checks the registers and stack slots the declared
arguments take against the GCC instructions at each call:

- `n32.so` and `n32-le.so` (N32 BE and LE) and `n64.so` (N64 BE, a control)
  are built from `n32.c`.
- `r6.so` (MIPS32r6 O32, FP64) is built from `r6.c`.
- `o64.so` (O64) is built from `o64.c`.

Rebuild with `./build.sh`. Set `MIPS_CC` to GCC 7.3.0 from the Octeon toolchain
(`mips64-octeon-linux-gnu-gcc`), `CLANG` to Clang 21.1.8 and `MIPS_LD` to GNU
ld 2.47. Clang only assembles GCC's output, and GCC's own assembler writes the
O64 image's ELF flags. The images rebuild byte for byte.

| File | SHA-256 |
| --- | --- |
| `n32.c` | `9f63033674dfeed31bba2abcd5766f5a260c5863f5c2a681ed6bcd2bdc9c1176` |
| `r6.c` | `6f02800052cfd7acc95a67eab30c88c3b95aaba07cdc0ee31c129fc661388642` |
| `o64.c` | `87c28b0337812a81511a8bae6062eef45360d450383dc0ad5ee239f61df7f2f1` |
| `build.sh` | `73a4e16440bf3a94b261fe04f4df2f8737e906c9dea4ff37ae995cc741044bce` |
| `n32.so` | `ee90aad896a5af786ff3e39aed6179b3ba5c3739160161de47f3e2f1e68d2719` |
| `n32-le.so` | `6b2b18ce8eefcc18e4a87f547537eeae64cac1cdd3a1858e7b5f60696ff4f67e` |
| `n64.so` | `e81ea4dee2094de1c78b9b72000f03aa73ce7f6c24942bb2839b7f856dfff8b8` |
| `r6.so` | `d6e8978ef6d58a68cf00773726c4d736242ec72d79d9adc86a5ed2f51cf27e56` |
| `o64.so` | `f7b075526edd07384a4a8f8f84c7c25501aa38ff2003256723971f23ba5db53d` |
| GCC `mips64-octeon-linux-gnu-gcc` 7.3.0 | `5d26f5ed013593e77d0237dac6f3179f68cdbb681f8c20d7948444b7ff3e0b49` |
| Clang 21.1.8 | `412bbe8c60571a1eb06f48fde89635033621caeb01a9b4ee76d46711bae8e932` |
| GNU ld 2.47.20260726 | `2865846ece2642439cfae404ef930bfc5494c4babd2347757878b8f0f8f761cf` |
