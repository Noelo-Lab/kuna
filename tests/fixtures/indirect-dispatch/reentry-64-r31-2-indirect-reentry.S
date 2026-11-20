.set noreorder
.text
.globl bounce
.type bounce,@function
bounce:
 or $8,$31,$0
 lui $31,%hi(.Lstep0)
 ori $31,$31,%lo(.Lstep0)
.Ldispatch:
 jr $31
 nop
.Lstep0:
 sw $4,0($5)
 addiu $4,$4,1
 lui $31,%hi(.Lstep1)
 ori $31,$31,%lo(.Lstep1)
 lui $9,%hi(.Ldispatch)
 ori $9,$9,%lo(.Ldispatch)
 jr $9
 nop
.Lstep1:
 sw $4,4($5)
 or $31,$8,$0
 jr $31
 nop
.size bounce,.-bounce
.globl _start
_start:
 lui $5,%hi(slots)
 ori $5,$5,%lo(slots)
 addiu $5,$5,4
 lui $4,0x0000
 ori $4,$4,0x0049
 jal bounce
 nop
 li $4,1
 lw $9,-4($5)
 lui $10,0x0000
 ori $10,$10,0x000c
 bne $9,$10,.Lexit
 nop
 lw $9,0($5)
 lui $10,0x0000
 ori $10,$10,0x0049
 bne $9,$10,.Lexit
 nop
 lw $9,4($5)
 lui $10,0x0000
 ori $10,$10,0x004a
 bne $9,$10,.Lexit
 nop
 lw $9,8($5)
 lui $10,0x0000
 ori $10,$10,0x0022
 bne $9,$10,.Lexit
 nop
 li $4,0
.Lexit:
 li $2,5058
 syscall
.data
.align 2
slots: .word 12,0,0,34
