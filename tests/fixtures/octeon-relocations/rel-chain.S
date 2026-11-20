/* Original N32 REL fields, including repeated offsets and 32-bit wrapping. */
.set noreorder
.text
.globl read_combined, read_wrapped, read_mixed, read_mixed_lo, read_chain
.type read_combined, @function
read_combined:
lui $8, %hi(combined_value)
lw $2, %lo(combined_value)($8)
jr $31
nop
.size read_combined, .-read_combined
.type read_wrapped, @function
read_wrapped:
lui $8, %hi(wrapped_value)
lw $2, %lo(wrapped_value)($8)
jr $31
nop
.size read_wrapped, .-read_wrapped
.type read_mixed, @function
read_mixed:
lui $8, %hi(mixed_value)
lw $2, %lo(mixed_value)($8)
jr $31
nop
.size read_mixed, .-read_mixed
.type read_mixed_lo, @function
read_mixed_lo:
lui $8, %hi(mixed_lo)
lw $2, %lo(mixed_lo)($8)
jr $31
nop
.size read_mixed_lo, .-read_mixed_lo
.type read_chain, @function
read_chain:
lui $8, %hi(chain_value)
lw $2, %lo(chain_value)($8)
jr $31
nop
.size read_chain, .-read_chain

.section .rodata
.balign 4
.globl combined_value, wrapped_value, mixed_value, mixed_lo, chain_value
combined_value:
.reloc ., R_MIPS_32, first
.reloc ., R_MIPS_32, second
.word 7
wrapped_value:
.reloc ., R_MIPS_32, carry
.reloc ., R_MIPS_32, negative
.word 0xfffffff0
/* Consecutive same-offset records of different types: the first result is
   the next record's addend and only the last record writes. */
mixed_value:
.reloc ., R_MIPS_32, first
.reloc ., R_MIPS_HI16, second
.word 7
mixed_lo:
.reloc ., R_MIPS_32, first
.reloc ., R_MIPS_LO16, low
.word 0x00050007
/* %hi(%neg(%gp_rel(first))) spelled as REL records. */
chain_value:
.reloc ., R_MIPS_GPREL16, first
.reloc ., R_MIPS_SUB, 0
.reloc ., R_MIPS_HI16, 0
.word 0x3c010000
.globl first, second, carry, negative, low
.set first, 0x12340000
.set second, 0x56780000
.set carry, 0x30
.set negative, 0xfffffff0
.set low, 0x1234
