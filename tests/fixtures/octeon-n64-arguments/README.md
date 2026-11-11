# Octeon N64 soft-float arguments

Original C, compiled independently with Cavium GCC 7.3.0 from
`MarvellEmbeddedProcessors/Octeon-Toolchain`'s `toolchain-src-265.0.tar.bz2`
(archive SHA-256 `1616c4c43a5cbb5ae67f88fc5fbd5b577be697ecc59ed441669af4b01c5967a1`).
No vendor binary or source is incorporated. The sealed ELF includes DWARF so
the console tests exercise actual compiler-produced prototypes, without
requiring a cross-compiler at test time.

SHA-256: `arguments.c`
`7c38342a365fe41b4cfe5f44241cf13e486b40ced2ee7dd13316144001385156`;
`arguments.elf`
`6ed12b1d6ca53bfeec14b96cb162baee291a2253b838dbeb015bac728b962b33`.

Rebuild in this directory with `OCTEON_CC` pointing to that GCC and
`OCTEON_LD` to GNU multiarch ld 2.47.20260726:

```sh
"$OCTEON_CC" -march=octeon -mabi=64 -msoft-float -mno-abicalls \
  -fno-pic -fno-builtin -fdebug-prefix-map="$PWD"=. -O2 -g \
  -c arguments.c -o /tmp/octeon-n64-arguments.o
"$OCTEON_LD" -m elf64btsmip -z max-page-size=0x1000 -Ttext=0x100000 \
  -e get_one /tmp/octeon-n64-arguments.o -o arguments.elf
```

`get_one`, `get_two`, `get_four` and `four_last` extract high register bytes;
`float_identity`, `float_second` and `float_last` copy the low word.
`four_stack` and `float_stack` load `0(sp)`, `uint_stack` loads `4(sp)`, and
`float_stack_second` loads `8(sp)`. `get_eight`, `get_union`,
`double_identity`, `make_four` and `variadic_four` are ABI controls.
The variadic probe independently shows high-word placement in the saved GPR;
recovering a typed `va_arg` expression is outside this regression's contract.

The source of these distinctions is GCC's `config/mips/mips.c`:
`mips_pad_reg_upward` keeps scalar floats in the low register bytes;
`mips_pad_arg_upward` puts non-integral values at the stack-slot start on
big-endian N64. `mips_function_arg` uses mixed FPR/GPR struct allocation only
for hard float. The soft-float spec therefore needs an unconditional aggregate
rule and separate register/stack rules for scalar floats.

The Rust regression checks storage, return placement, following argument slots
and DWARF-driven emitted C for all four Octeon languages. The stage regression
also exercises explicit C prototypes against four functions' sealed bytes.
