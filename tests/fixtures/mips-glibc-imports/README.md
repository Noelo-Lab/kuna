# MIPS imports bound to glibc

`calls.c` holds three PIC wrappers. `release` passes its pointer to `free`,
`measure` returns `strlen(s) + 1` and `turn` forwards a `double` and two
pointers to `sincos`. None of them writes an argument register before the call,
so the instructions alone cannot tell `free(p)` from a `free` that takes nothing.

The images differ only in what their dynamic linking says about the provider.
`provider.c` stands in for the C library at link time, and the images record
which library they require:

| Image | ABI | Provider requirement |
| --- | --- | --- |
| `n32-glibc.elf` | N32 | `libc.so.6`, symbols versioned `GLIBC_2.0` |
| `n32-unversioned.elf` | N32 | `libc.so.6`, unversioned symbols |
| `n32-other.elf` | N32 | `libother.so.1`, symbols versioned `OTHER_1.0` |
| `n32-pthread.elf` | N32 | `libpthread.so.0`, symbols versioned `GLIBC_2.0` |
| `n32-private.elf` | N32 | `libc.so.6`, symbols versioned `GLIBC_PRIVATE` |
| `o32-glibc.elf` | O32 | `libc.so.6`, symbols versioned `GLIBC_2.0` |
| `n64-glibc.elf` | N64 | `libc.so.6`, symbols versioned `GLIBC_2.0` |

GNU ld creates the `.MIPS.stubs` lazy-binding stubs that name each import;
lld does not. Rebuild in this directory with clang 21.1.8, GNU ld
2.47.20260726 and llvm-strip 21.1.8:

```sh
cc() { clang -target "$1" -mabi="$2" -march="$3" -O2 -fPIC -mabicalls -fno-ident -ffreestanding -c "$4" -o "$5"; }
link() { ld -m "$1" -shared --build-id=none -z max-page-size=0x1000 "${@:2}"; }
mkdir -p glibc-n32 plain-n32 private-n32 glibc-o32 glibc-n64
cc mips64-linux-gnuabin32 n32 mips64r2 calls.c n32.o
cc mips64-linux-gnuabin32 n32 mips64r2 provider.c n32-provider.o
link elf32btsmipn32 -soname libc.so.6 --version-script glibc.map -o glibc-n32/libc.so.6 n32-provider.o
link elf32btsmipn32 -soname libc.so.6 -o plain-n32/libc.so.6 n32-provider.o
link elf32btsmipn32 -soname libother.so.1 --version-script other.map -o libother.so.1 n32-provider.o
link elf32btsmipn32 -soname libpthread.so.0 --version-script glibc.map -o libpthread.so.0 n32-provider.o
link elf32btsmipn32 -soname libc.so.6 --version-script private.map -o private-n32/libc.so.6 n32-provider.o
link elf32btsmipn32 -o n32-glibc.so n32.o glibc-n32/libc.so.6
link elf32btsmipn32 -o n32-unversioned.so n32.o plain-n32/libc.so.6
link elf32btsmipn32 -o n32-other.so n32.o libother.so.1
link elf32btsmipn32 -o n32-pthread.so n32.o libpthread.so.0
link elf32btsmipn32 -o n32-private.so n32.o private-n32/libc.so.6
cc mips-linux-gnu 32 mips32r2 calls.c o32.o
cc mips-linux-gnu 32 mips32r2 provider.c o32-provider.o
link elf32btsmip -soname libc.so.6 --version-script glibc.map -o glibc-o32/libc.so.6 o32-provider.o
link elf32btsmip -o o32-glibc.so o32.o glibc-o32/libc.so.6
cc mips64-linux-gnuabi64 64 mips64r2 calls.c n64.o
cc mips64-linux-gnuabi64 64 mips64r2 provider.c n64-provider.o
link elf64btsmip -soname libc.so.6 --version-script glibc.map -o glibc-n64/libc.so.6 n64-provider.o
link elf64btsmip -o n64-glibc.so n64.o glibc-n64/libc.so.6
for f in n32-glibc n32-unversioned n32-other n32-pthread n32-private o32-glibc n64-glibc; do llvm-strip --strip-all -o "$f.elf" "$f.so"; done
```

| File or tool | SHA-256 |
| --- | --- |
| calls.c | 00e002281f7e8f3b6c8c1ce00ea02f62d7496bf7df96a428ca66833de4e2abc4 |
| provider.c | a62e0ee66013fca9b45cc90f811d64db08aff72c02aca726af0263286a78931e |
| glibc.map | 820904b54227b00bf371a13b588695f63e4a75d53b5abe55a4246d8b7f458f0f |
| private.map | 686f0572ea6bdb08cc92f5cec878c7ffac35e35f1a3f77205cbb0c78609be484 |
| other.map | 5979243cb1e87cf50bfd5fa9fa0c31faf3d7c4d764e8ff1dd7e6542f522fa5d1 |
| n32-glibc.elf | 762b745b45b703673d5ace98f069d8bed96ee752d14a703d239bb2a9dbc879ed |
| n32-unversioned.elf | 53d609a94a69223d6804b983b1a94a8d8f4a356f401d0ad59403f5403c154849 |
| n32-other.elf | 5041a1a50e04fc20baaf480fa60dc307a222402a34729ee77324d290e3e2af3d |
| n32-pthread.elf | a916ad666e2f2b12859988a262fcfa332f4b868c7c4bd83fa5c6e4374457fb72 |
| n32-private.elf | bc5c55f02dd557f0e90d4dcd17be1bfcf4cf5c6a2ca74db54052bb24c2f64a18 |
| o32-glibc.elf | bf05844e639eb3fcb3a82d3a014f7b61e6656fabb26a5effee7a4603b492e5d8 |
| n64-glibc.elf | 805579ecac1c552afcd03b4f395a86cc6e6574b0342586d00425e7ac1230ed6d |
| clang | 412bbe8c60571a1eb06f48fde89635033621caeb01a9b4ee76d46711bae8e932 |
| ld | 2865846ece2642439cfae404ef930bfc5494c4babd2347757878b8f0f8f761cf |
| llvm-strip | 0de5f7adbb648ed88aa04f2d91bd05c08306b99a27741540cc0473cbac0a0b40 |
