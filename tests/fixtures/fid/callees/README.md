# Callee fixture

A stripped program for the FLIRT matcher (`kuna-console/tests/verify_flirt_sites.rs`,
signature `tests/fixtures/flirt/callees.sig`).

| File | What |
|---|---|
| `lib.c` | `scale_a`, `scale_b`; `wrap_a`/`wrap_b`, the same instructions calling one each; `bump`, an 11-byte stub with no callee. |
| `main.c` | A libc-free `_start` that calls the wrappers and `bump`. |
| `lib.o` | **Vendored.** `lib.c` compiled (below). |
| `prog` | **Vendored.** `lib.o` + `main.o` linked without libc and stripped. `scale_a` 0x401000, `scale_b` 0x401020, `wrap_a` 0x401040, `wrap_b` 0x401060, `bump` 0x401080. |

## Build

```bash
CF="-O2 -ffreestanding -fno-stack-protector -fno-pic -fcf-protection=none"
gcc $CF -c lib.c -o lib.o
gcc $CF -c main.c -o main.o
gcc -nostdlib -static -no-pie -Wl,--build-id=none lib.o main.o -o prog.full
strip --strip-all -o prog prog.full
python3 ../../flirt/make_callees_sig.py
```
