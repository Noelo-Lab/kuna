# nanoMIPS emulation fixtures

Inputs of `tests/nanomips_core.rs`. Each `.bin` is a flat image loaded at
`0x80001000` whose first words are a table of entry points.

| Image | Source | Build |
|---|---|---|
| `workload_O2EL.bin`, `workload_OsEL.bin`, `workload_O2EB.bin`, `workload_OsEB.bin` | `workload.c`, `runtime.c` | `-O2` / `-Os`, `-EL` / `-EB` |
| `extras_EL.bin`, `extras_EB.bin` | `extras.S` | `-EL` / `-EB` |

Built with MediaTek GNU Tools 2021.11-07 for nanoMIPS bare metal (GCC 6.3.0):

```bash
for o in O2 Os; do for e in EL EB; do
  nanomips-elf-gcc -$e -$o -msoft-float -ffreestanding -nostdlib -Wl,-T,workload.ld \
      workload.c runtime.c -o workload.elf
  nanomips-elf-objcopy -O binary workload.elf workload_$o$e.bin
done; done
for e in EL EB; do
  nanomips-elf-gcc -$e -msoft-float -nostdlib -Wl,-T,workload.ld extras.S -o extras.elf
  nanomips-elf-objcopy -O binary extras.elf extras_$e.bin
done
```

The expected values of the workload are what a native build of `workload.c`
prints for `table[case](seed)`; the file uses fixed-width arithmetic only, so
they do not depend on the host. `extras.S` is restated as `extras_model` in the
test.
