# `kuna flirt make` fixture

A static library of three objects and a stripped program linked against it
(`kuna-cli/tests/flirt_make_cli.rs`).

| File | What |
|---|---|
| `mix.c`, `sum.c`, `wrap.c` | The library. `wrap_mix` and `wrap_sum` are the same bytes around a call to a different function; `twin_one` and `twin_two` are the same bytes with nothing to tell them apart. |
| `main.c` | A libc-free `_start` that calls into the library. |
| `libthree.a` | **Vendored.** The three objects. |
| `prog` | **Vendored.** `main.o` + `libthree.a`, stripped. |
| `three.pat` | **Vendored.** `kuna flirt make libthree.a -o three.pat`. |

## Build

```bash
CF="-O2 -ffreestanding -fno-stack-protector -fno-pic -fcf-protection=none -fno-asynchronous-unwind-tables"
for f in mix sum wrap main; do gcc $CF -c $f.c -o $f.o; done
ar rcsD libthree.a mix.o sum.o wrap.o
gcc -nostdlib -static -no-pie -Wl,--build-id=none main.o libthree.a -o prog.full
strip --strip-all -o prog prog.full
kuna flirt make libthree.a -o three.pat
```
