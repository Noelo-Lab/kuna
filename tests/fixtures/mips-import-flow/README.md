# MIPS target-selection and import-flow fixtures

These original sources cover O32 BE/LE generic MIPS and Octeon machine flags,
and PIC import continuations with local/exported helpers, stripped/unstripped
symbols and stack protection on/off. N32/N64 provide 64-bit-register controls.
The CLI tests use committed images and need no compiler or emulator.

Regenerate with GCC 7.3 from the Octeon toolchain, a GNU assembler supporting
both byte orders, GNU multiarch ld and matching MIPS strip:

```sh
python3 build.py --cc "$MIPS_CC" --assembler "$MIPS_AS" \
  --ld "$MIPS_LD" --strip "$MIPS_STRIP" --nm "$MIPS_NM"
```

`provenance.json` records source/image/tool SHA-256 values, compiler and linker
flags, and pre-strip function addresses. These addresses locate functions in
test output; they are not assertions supplied to the decompiler. Compiler
assembly is passed unchanged to the assembler. No target libc is linked.

`duplicate` must preserve both copies, its terminator store and return.
`unknown_then_store` must retain the store and continuation after an import
with no known declaration. Stack protection introduces a genuinely non-returning
import. A lazy-binding resolver saves the caller return in t7 and transfers
through t9; it does not return into the following stub. Treating that transfer
as an ordinary returning call incorrectly spreads no-return facts backwards.

The existing `floatret_chain_mips_O0` regression separately pins previously
broken output shared by base and head. A differential comparison alone cannot
establish its correctness. Compiler-fixture coverage and optional execution
are separate from the general public-image regression sweep.

For optional execution validation, use the same compiler/assembler/linker and
QEMU's O32 BE/LE user emulators:

```sh
python3 replay.py --kuna "$KUNA" --specs "$KUNA_SPECS" --cc "$MIPS_CC" \
  --assembler "$MIPS_AS" --ld "$MIPS_LD" --qemu-be "$QEMU_MIPS" \
  --qemu-le "$QEMU_MIPSEL" --out "$RESULTS"
```

The replay compiles each unchanged recovered helper body at O0 and O2 and
executes it beside the original source. Independent checks cover returned
pointers, both copies, the terminator and an adjacent canary for 41 lengths.
It requires no target libc. Missing arguments, compiler failures and execution
failures are errors. This optional check supplements the mandatory sealed-image
CLI tests; it is not a dependency of normal CI.
