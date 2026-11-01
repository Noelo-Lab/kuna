# Octeon aggregate and quad ABI probes

Original freestanding source, with no vendor implementation or target runtime.
The two sealed ELFs contain GCC-produced DWARF. Console tests use those declared
types; stage tests separately assert prototypes against the compiled bytes.
The source includes controls for scalar versus aggregate return registers and
register/stack alignment. Only the exercised regression contracts are asserted;
full N32 stack-pointer recovery and C rendering of every aggregate remain
separate qualification work.

Build in this directory with Cavium GCC 7.3.0 from
MarvellEmbeddedProcessors/Octeon-Toolchain, archive `toolchain-src-265.0.tar.bz2`
(SHA-256 `1616c4c43a5cbb5ae67f88fc5fbd5b577be697ecc59ed441669af4b01c5967a1`),
and GNU multiarch ld 2.47.20260726. Set `OCTEON_CC`, `OCTEON_LD` and
`OCTEON_AS` to their tool paths:

```sh
for abi in n32 64; do
  "$OCTEON_CC" -march=octeon -mabi="$abi" -msoft-float -mno-abicalls \
    -fno-pic -fno-builtin -fdebug-prefix-map="$PWD"=. -O2 -g \
    -c arguments.c -o "/tmp/octeon-abi-$abi.o"
  case "$abi" in n32) emulation=elf32btsmipn32;; 64) emulation=elf64btsmip;; esac
  "$OCTEON_LD" -m "$emulation" -z max-page-size=0x1000 -Ttext=0x100000 \
    -e pair_first "/tmp/octeon-abi-$abi.o" -o "arguments-$abi.elf"
done
"$OCTEON_AS" -64 -march=octeon2 atomics.s -o /tmp/octeon-atomics.o
```

GNU objdump shows `pair_first`/`pair_second` reading a0/a1, `small_result`
shifting its value into v0's high word, `quad_after` reading t0 after skipping
a1, and `quad_identity` copying a0/a1 into v0/a0. The struct and integer quad
controls copy those same inputs into v0/v1 instead. `layout_size` returns 32,
and `layout_offset` returns 16. Stack quad alignment puts the following scalar
at offset 16 or 32 for `quad_last_after` and `quad_stack_after`.

GCC's `config/mips/mips.c` supplies the independent ABI reference:
`mips_get_arg_info` and `mips_function_arg_boundary` align and split arguments;
`mips_return_in_msb` left-justifies aggregates; `mips_function_value_1` handles
TFmode via `mips_return_fpr_pair`. With soft float, `FP_RETURN` is GP register
2 and the pair stride is two, so the scalar quad result is registers 2/4 in
both N32 and N64. Aggregate and integer results follow the separate GP path.

`atomics.s` covers both inherited save/restore collisions, twelve load/atomic
forms and saa/saad. Their words are pinned in the SLEIGH integration regression;
both assembly decoding and p-code lifting must refuse them until atomic
semantics are implemented. Generic MIPS save/restore remain positive controls.
