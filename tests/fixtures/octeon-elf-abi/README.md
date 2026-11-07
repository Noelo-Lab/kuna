# Octeon ELF and hard-float ABI fixtures

`abi.c` is original freestanding source. The four sealed ELF files contain only
that source, compiled independently by GCC 7.3.0 and GCC 4.7.0 for BE N32/N64.
Four small shared libraries built from `imports.c` exercise dynamic import names.
Tests use these files directly; CI does not need an Octeon toolchain or emulator.

Compiler source archives from MarvellEmbeddedProcessors/Octeon-Toolchain:

| Compiler | Archive | SHA-256 |
| --- | --- | --- |
| GCC 7.3.0 | toolchain-src-265.0.tar.bz2 | 1616c4c43a5cbb5ae67f88fc5fbd5b577be697ecc59ed441669af4b01c5967a1 |
| GCC 4.7.0 | toolchain-build-54.tar.bz2 | 9e7d261c8488d7c0b2c95bd6b2dea742b2af776b6025a9f6bad86e34a6d77d4f |

To rebuild, run in this directory with `OCTEON_CC` pointing to the selected
compiler, `OCTEON_LD` to GNU multiarch ld 2.47.20260726, and `compiler` set to
`gcc73` or `gcc47`:

```sh
for abi in n32 64; do
  "$OCTEON_CC" -march=octeon3 -mabi="$abi" -mhard-float -mfp64 -EB -O2 -g \
    -fno-builtin -ffreestanding -fPIC -mabicalls -fdebug-prefix-map="$PWD"=. \
    -c abi.c -o "/tmp/octeon-abi-$compiler-$abi.o"
  case "$abi" in n32) emulation=elf32btsmipn32;; 64) emulation=elf64btsmip;; esac
  "$OCTEON_LD" -m "$emulation" -z max-page-size=0x1000 -static -Ttext=0x100000 \
    -e indirect "/tmp/octeon-abi-$compiler-$abi.o" -o "abi-$compiler-$abi.elf"
done
```

Build the import images with the same compile flags, replacing `abi.c` with
`imports.c`, and link with `-shared -soname liboriginal-imports.so` instead of
`-static -Ttext=0x100000 -e indirect`. Name the outputs
`imports-$compiler-$abi.so`. They test loader recovery; they are not executed
through a target dynamic linker.

GCC 7.3 declares MIPS64r5/Octeon3 and 64-bit GPR/FPRs in `.MIPS.abiflags`.
GCC 4.7 instead carries GNU FP attributes, with its legacy hard-float header
bit 0x200; modern binutils calls that bit `fp64`. Preserve these differences.
Neither compiler's metadata is patched to resemble the other.

The PIC globals exercise t9-derived gp recovery. Separate scalar, aggregate,
variadic and indirect-call functions expose storage allocation without inlining
a caller and callee together. Scalar quad returns use f0/f2; the single-quad
struct uses f0/f1. Two-float and two-double structs return f0/f2, while mixed
double/integer structs return v0/v1.

The explicit 16-byte alignment on `AlignedDoubles` is not emitted in either
compiler's DWARF here. Alignment storage tests state it from the source rather
than inferring it from the aggregate's size; an ordinary two-double struct has
eight-byte alignment.

`driver.c` independently calls all 23 probes over 256 vectors, including integer
boundaries, negative zero and distinct halves of quad and aggregate values.
For optional execution, select one `abi` and matching `OCTEON_QEMU`, retain the
callee object from the rebuild above, and run:

```sh
case "$abi" in
  n32) emulation=elf32btsmipn32; exit_nr=6058;;
  64) emulation=elf64btsmip; exit_nr=5058;;
esac
"$OCTEON_CC" -march=octeon3 -mabi="$abi" -mhard-float -mfp64 -EB -O2 \
  -std=gnu99 -ffreestanding -fno-builtin -fPIC -mabicalls \
  -c driver.c -o /tmp/octeon-abi-driver.o
"$OCTEON_CC" -march=octeon3 -mabi="$abi" -mhard-float -mfp64 -EB \
  -mno-abicalls -fno-pic -fno-pie -DEXIT_NR="$exit_nr" \
  -c start.S -o /tmp/octeon-abi-start.o
"$OCTEON_LD" -m "$emulation" -z max-page-size=0x1000 -static -Ttext=0x100000 \
  -e _start /tmp/octeon-abi-start.o /tmp/octeon-abi-driver.o \
  "/tmp/octeon-abi-$compiler-$abi.o" -o /tmp/octeon-abi-run.elf
"$OCTEON_QEMU" -cpu Octeon68XX /tmp/octeon-abi-run.elf
```

Exit zero means all 5,888 checks passed. A nonzero exit identifies the failed
check in `driver.c`. These original executions establish compiler behavior;
they do not alone prove the recovered C is correct.

Recovered-C execution currently covers 19 functions. `read_global` retains a
GOT indirection, and `unnamed_varargs` needs variadic-access recovery. The two aligned aggregate
getters require explicit source alignment as described above. These four cases
are not semantic passes for automatic recovered C. Dynamic import arity is
checked with an asserted prototype: a zero-argument import call can have the
same instructions as a call using the preserved incoming argument.

| File | SHA-256 |
| --- | --- |
| abi-gcc73-n32.elf | d4ea9ad823b5240808f1475c0d8c9ef51acd5a3552ac63dabfe60fccf3ae63d4 |
| abi-gcc73-64.elf | a8a35786136d33588d200258cb678b27c2e4326a9d245cf4eade6bcded294882 |
| abi-gcc47-n32.elf | f4149573dc7ec4ebb61f737cd52fd0044dcd9d2cbb949a6c0dea02c4f8fe8782 |
| abi-gcc47-64.elf | 5652a31fe59cc09750b52e94c79ec8eecbdc3874efb81efa86d3b8ff8a53720d |
| imports-gcc47-64.so | 55a83cee9089bf8e005da32d29592f615ed9c7377d8b97a1830b7a1517f02cfa |
| imports-gcc47-n32.so | 050a5d3aafde8427ec9a277f749057be5f120750c45c01e18f7571e2463cae9f |
| imports-gcc73-64.so | ca245458343cfba7445a07ffa025ced4828ba0bf1a2c4036794af62f94bea917 |
| imports-gcc73-n32.so | 191b7170a2db903e493d2da23c934b52fb9ca6084b9f35853dde1ddb7bc4a01b |

`fields.c` adds eleven original probes for direct versus nested floating fields,
arrays, heterogeneous float/double returns with padding, a double following a
quad field, register/stack splits, and a large struct returned through a hidden
pointer. Rebuild the four `fields-$compiler-$abi.elf` images with the `abi.c`
recipe above, substituting `fields.c` and `-e nested_first`. The corresponding
`fields-driver.c` uses the same driver/start recipe and checks all eleven probes
over 256 bit-pattern vectors (2,816 checks per compiler/ABI). It compares value
fields only; struct padding is unspecified by the calling convention.

Recovered-C execution covers ten of these eleven field probes with both
compilers, both ABIs and O0/O2 recompilation. `two_quads` has correct hidden-pointer
return storage but still renders artificial byte-slice members. Its original
execution checks are counted separately from recovered-C checks.

`bitfields.c` supplies the instruction bytes embedded in the console test
`floating_structs_with_bitfields_return_through_gprs`. Both pinned compilers,
using `-march=octeon3 -mabi=n32` or `-mabi=64` with
`-mhard-float -mfp64 -EB -O2 -g -c`, produce the recorded bodies. GNU objdump
confirms that the direct double enters in f12 or f13, while both aggregate
results use v0/v1. The test asserts the source prototypes explicitly and needs
neither a compiler nor an emulator in CI.

| File | SHA-256 |
| --- | --- |
| fields-gcc47-64.elf | e666226b1043a5f4d5cda6e1e6d696040da859b1a244e30f08ceaf92ca6cf7dc |
| fields-gcc47-n32.elf | f008cce017f016b757dae0588fdcec76244933672d69d42233f2d85dd921af17 |
| fields-gcc73-64.elf | f489ea4d673b0dd66be4fdb5e68e3de2961834bfd3cccb98e13f13efc178728a |
| fields-gcc73-n32.elf | dbfd7387947f34e99d911413aca6333f4f08b0e3068844701f3ceacc250bdec0 |
