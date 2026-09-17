# mulblob — plan

## Change

One new module, `decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_mulblob.rs`,
exporting `declines_zext(&Funcdata, OpId) -> bool` plus the `OptionMulBlob`
value parser. `RulePieceStructure` consults it at its three
`convert_zext_to_piece` sites (the direct `INT_ZEXT` entry arm, the extension
found above a CONCAT root, and each `INT_ZEXT` leaf inside a gathered tree) and
declines there. Nothing else in the engine moves.

Gate: `Architecture::mul_blob` / `ArchContext::mul_blob`, option
`mulblob on|off`, phase P3, subphase simplification-quiescence, tier transform,
change_kind presentation-default.

## Test

`tests/stages/mulblob-wide-multiply-operands.xml` — a two-pass bytechunk over
the gcc -O2 body of

```c
uint64_t widemul(uint64_t a, uint64_t b, uint64_t *hi) {
    unsigned __int128 p = (unsigned __int128)a * (unsigned __int128)b;
    *hi = (uint64_t)(p >> 64);
    return (uint64_t)p;
}
```

(`endbr64; mov rax,rdi; mov r8,rdx; mul rsi; mov [r8],rdx; ret`), both halves of
the product used. Pass 1 with `option mulblob off` asserts the bug (two
`char vN [16]` declarations, the half-fills, the blob product read); pass 2 with
the default asserts the fix (`*hi = SUB168(ZEXT816(a) * ZEXT816(b),8);` and the
matching low half, with no blob left).

## Default decision

Ship default **on** if, and only if: 675/675 datatest assertions unmoved, stages
PARITY OK, the typesweep is neutral-or-positive on the 444-slice campaign
corpus, the whole-corpus `decompile-all` sweep shows no hunk outside
{blob removed, expression re-rendered over the operands}, and the interleaved
min-of-15 speed delta is inside +5%.

## Wrong-output lens

The high and the low half of the product must be preserved exactly. Two
instruments:

1. a compile-and-run oracle (`.scratch/oracle/`): the fixture above plus an
   overflow-checked multiply-accumulate, compiled with gcc -O2, decompiled in
   both arms, both renderings transcribed back into C behind `SUB168`/`ZEXT816`/
   `CARRY8` shims and run against the original source over 81 edge pairs and
   10,000 random triples;
2. a whole-corpus before/after `decompile-all` diff over 12 binaries
   (O0/O2/O2-noinline), with every changed function's hunks classified.
