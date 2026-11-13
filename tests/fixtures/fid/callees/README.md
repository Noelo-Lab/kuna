# Callee fixture

A stripped program for the FLIRT matcher and its interplay with FID
(`kuna-console/tests/verify_flirt_sites.rs`, signature `tests/fixtures/flirt/callees.sig`).

| File | What |
|---|---|
| `lib.c` | `scale_a`, `scale_b`; `wrap_a`/`wrap_b`, the same instructions calling one each; `bump`, an 11-byte stub with no callee. |
| `main.c` | A libc-free `_start` that calls the wrappers and `bump`. |
| `lib.o` | **Vendored.** `lib.c` compiled (below). |
| `lib.fid` | **Vendored.** `kuna fid build lib.o`: FID names `scale_b`, as FLIRT does. |
| `lib_alt.fid` | **Vendored.** The same with `scale_b` renamed `alt_scale`, so FID and FLIRT disagree. |
| `prog` | **Vendored.** `lib.o` + `main.o` linked without libc and stripped. `scale_a` 0x401000, `scale_b` 0x401020, `wrap_a` 0x401040, `wrap_b` 0x401060, `bump` 0x401080. |

## Build

```bash
CF="-O2 -ffreestanding -fno-stack-protector -fno-pic -fcf-protection=none"
gcc $CF -c lib.c -o lib.o
gcc $CF -c main.c -o main.o
gcc -nostdlib -static -no-pie -Wl,--build-id=none lib.o main.o -o prog.full
strip --strip-all -o prog prog.full
python3 ../../flirt/make_callees_sig.py
kuna fid build lib.o -o lib.fid --lang x86:LE:64:default --cspec gcc
objcopy --redefine-sym scale_b=alt_scale lib.o lib_alt.o
kuna fid build lib_alt.o -o lib_alt.fid --lang x86:LE:64:default --cspec gcc
```
