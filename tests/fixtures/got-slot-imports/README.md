# Calls through an import's GOT slot

`calls.c` is compiled with `-fno-plt`, so each call and tail jump loads the
import from its `GLOB_DAT` GOT slot (`call *strlen@GOTPCREL(%rip)`,
`jmp *free@GOTPCREL(%rip)`):
- `release` forwards its pointer to `free`;
- `length` and `same` return what `strlen` and `memcmp` return;
- `first` adds `strlen`'s result to a byte it reads itself;
- `check` calls `exit(2)` on a negative argument;
- `bump` increments the data import `counter`, whose slot holds an `OBJECT`.

`viaplt.c` calls the same four imports through the PLT (`drop`, `len2`,
`cmp2`, `bail`).

Linked together, each import is reached both through its GOT slot and through
the PLT. GNU ld then gives it a `.plt.got` stub that jumps through that same
slot. So the import has two `FunctionSymbol`s with one name: the stub and the
slot. `provider.c` stands in for the C library at link time only, so the
imports are unversioned functions (and one `OBJECT`) of `libc.so.6`.

| Image | Sections | Contents |
| --- | --- | --- |
| `shared_x86_64.so` | `.plt`, `.plt.got`, `.text`, `.got` | `calls.o` and `viaplt.o`, stripped; `memcmp@plt`, `exit@plt`, `strlen@plt` and `free@plt` at 0x1010 to 0x1040 in `.plt.got`; slots `memcmp` 0x3fb8, `counter` 0x3fc0, `exit` 0x3fc8, `strlen` 0x3fd0, `free` 0x3fe0 |
| `noplt_x86_64.so` | `.text`, `.got` | `calls.o` alone, not stripped; no `.plt*` section, so each slot is the import's only `FunctionSymbol`; the slots are at the same addresses |

Built in this directory with Ubuntu's GCC 15.2.0 (`gcc-15` 15.2.0-16ubuntu1) and
GNU binutils 2.46 (`binutils-x86-64-linux-gnu` 2.46-3ubuntu2). `${OUT}` is any
scratch directory. The commands reproduce the committed images byte for byte.

```sh
gcc -O2 -fPIC -shared -nostdlib -fno-ident -Wl,--build-id=none -Wl,-soname,libc.so.6 -o ${OUT}/libc.so.6 provider.c
gcc -O2 -fPIC -fno-plt -fno-ident -c -o ${OUT}/calls.o calls.c
gcc -O2 -fPIC -fno-ident -c -o ${OUT}/viaplt.o viaplt.c
gcc -shared -nostdlib -Wl,--build-id=none -o noplt_x86_64.so ${OUT}/calls.o ${OUT}/libc.so.6
gcc -shared -nostdlib -Wl,--build-id=none -o ${OUT}/shared_x86_64.so.full ${OUT}/calls.o ${OUT}/viaplt.o ${OUT}/libc.so.6
strip --strip-all -o shared_x86_64.so ${OUT}/shared_x86_64.so.full
```

| File | SHA-256 |
| --- | --- |
| `calls.c` | `d678db65651d240d1d95e169f0a1f788cda5eedfee997979ea0d87b9da1ad694` |
| `viaplt.c` | `0a0b68dda1fc38713b935bdb13167b67209a64de071b3f1d6faa6012cfb63232` |
| `provider.c` | `78bc495e8a3420877762396bd2bfc3506230880fa73dffecdba5f87f12a3f9ca` |
| `shared_x86_64.so` | `ad14b40ccac2ff4e5bcff9e2d3e5f0cf3793785c3df2dead9e221f3148964898` |
| `noplt_x86_64.so` | `653a00c6dfad9ef4e3204aba8bf1f969be220f9ff889a1de6d68bdd170ed49ef` |
