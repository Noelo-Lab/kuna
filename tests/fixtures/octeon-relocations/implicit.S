/* Original O32 REL inputs test the shared MIPS implicit-addend arithmetic. */
.set noreorder
.text
.globl _start
.type _start, @function
_start:
lui $8, %hi(value+0x18000)
lui $9, %hi(value+0x28000)
addiu $8, $8, %lo(value+0x18000)
lui $10, %hi(other-1)
addiu $10, $10, %lo(other-1)
jal dest
nop
beq $8, $9, dest
nop
.reloc ., R_MIPS_32, value
.word 7
.reloc ., R_MIPS_NONE, 0
.reloc .-4, R_MIPS_32, value
/* Consecutive records at one offset chain: the first result is the
   second's addend and only the second writes. */
.reloc ., R_MIPS_32, value
.reloc ., R_MIPS_HI16, other
.word 7
.size _start, .-_start
.globl value, other, dest
.set value, 0x12348000
.set other, 0x78900001
.set dest, 0x400100
