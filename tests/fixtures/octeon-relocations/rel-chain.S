/* Original N32 REL fields, including repeated offsets and 32-bit wrapping. */
.set noreorder
.text
.globl read_combined, read_wrapped
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

.section .rodata
.balign 4
.globl combined_value, wrapped_value
combined_value:
.reloc ., R_MIPS_32, first
.reloc ., R_MIPS_32, second
.word 7
wrapped_value:
.reloc ., R_MIPS_32, carry
.reloc ., R_MIPS_32, negative
.word 0xfffffff0
.globl first, second, carry, negative
.set first, 0x12340000
.set second, 0x56780000
.set carry, 0x30
.set negative, 0xfffffff0
