# MIPS narrow-address stack fixtures

Original freestanding source, compiled with Cavium GCC 7.3.0 from
`toolchain-src-265.0.tar.bz2` (SHA-256
`1616c4c43a5cbb5ae67f88fc5fbd5b577be697ecc59ed441669af4b01c5967a1`).
No vendor implementation or target library is incorporated. The sealed ELF
files retain DWARF for declared-prototype tests; stage tests use the original
instruction bytes without DWARF to exercise inferred parameters.

Build in this directory with `OCTEON_CC` pointing to that compiler and
`OCTEON_LD` to GNU multiarch ld 2.47.20260726:

```sh
for abi in n32 64; do
  "$OCTEON_CC" -march=octeon -mabi="$abi" -msoft-float -mno-abicalls \
    -fno-pic -fno-builtin -fdebug-prefix-map="$PWD"=. -O2 -g \
    -c stack.c -o "/tmp/mips-stack-$abi.o"
  case "$abi" in n32) emulation=elf32btsmipn32;; 64) emulation=elf64btsmip;; esac
  "$OCTEON_LD" -m "$emulation" -z max-page-size=0x1000 -Ttext=0x100000 \
    -e sum_ten "/tmp/mips-stack-$abi.o" -o "stack-$abi.elf"
done
```

GNU objdump shows `sum_ten` reading stack offsets 0 and 8, `frame_ten`
adjusting the N32 stack with `addiu` while loading/storing 64-bit values, and
`pointer_ninth`/`signed_ninth` reading offset 4 versus `float_ninth` offset 0.
`address_and_bits` uses the same full-width input as both a narrowed address
and an integer; `upper_half` is an unaffected integer control. `write_value`
and `indirect_frame` exercise stores and indirect calls across this boundary.
The N64 copy is the control with equal register and address widths.

`indirect_frame` also exposes a separate, pre-existing N32 call-result recovery
defect. It is retained for comparison, but excluded from N32 execution passes;
the stack fix alone does not recover its return value correctly.
