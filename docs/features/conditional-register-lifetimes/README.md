# Conditional register transfers

`fixture.S` is authored ARMv5 assembly. `candidate` calls a declared four-byte
scalar service, which returns in r0 and kills r1 under the selected convention.
It loads r1 and the pending byte only when the service returns nonzero. The
following guarded word load never consumes the previous r1 value. The base
instead emits an unconditional `v3 = v2` from an uninitialized inferred local.
`scan` repeats this flow across loop joins. `remember` retains the last
eligible pointer across later invalid candidates.

The CLI regression compiles the unchanged generated bodies with GCC and
arm-none-eabi-gcc at O0/O2, treating uninitialized diagnostics as errors. On
Linux x86 it executes freestanding 32-bit C against an independent model;
no 32-bit libc is needed. It covers 781 loop sequences, null/dead candidates,
all 256 pending-byte values, nested guards, unconditional loads, distinct
conditional definitions and retained incoming values. `unknown_used` and
`unknown_loop` deliberately return an unspecified call-clobbered value on one
path, including a loop-carried phi, and must retain their O0 diagnostics.
Optional external compilers can be absent; an installed compiler that fails is
an error.

```sh
cargo test --manifest-path decompiler/Cargo.toml -p kuna-cli \
  --test conditional_register_lifetimes -- --nocapture

arm-none-eabi-gcc -nostdlib -marm -march=armv5te \
  -Wl,-Ttext=0x10000 -Wl,-e,candidate \
  docs/features/conditional-register-lifetimes/fixture.S -o /tmp/kuna-conditional.elf
python3 docs/features/conditional-register-lifetimes/verify_arm.py /tmp/kuna-conditional.elf
```

The independent ARM check requires binutils and Python Unicorn. It executes
2,184 defined guard/model cases, with poisoned unmapped pointers on paths that
must not read them. The intentionally unspecified return is a compiler control,
not a numerical oracle. No application bytes or application fixtures are needed.
