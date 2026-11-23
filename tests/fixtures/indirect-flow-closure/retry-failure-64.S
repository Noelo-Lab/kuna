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
 li $2,73
 sw $2,0($5)
 or $31,$4,$0
 b .Ldispatch
 nop
.Lstep1:
 li $2,74
 sw $2,4($5)
 or $31,$8,$0
 jr $31
 nop
.size bounce,.-bounce
.globl _start
_start:
 lui $4,%hi(.Lstep1)
 ori $4,$4,%lo(.Lstep1)
 lui $5,%hi(slots)
 ori $5,$5,%lo(slots)
 jal bounce
 nop
 lw $9,0($5)
 li $10,73
 bne $9,$10,.Lfailed
 nop
 lw $9,4($5)
 li $10,74
 bne $9,$10,.Lfailed
 nop
 li $4,0
 b .Lexit
 nop
.Lfailed:
 li $4,1
.Lexit:
 li $2,5058
 syscall
.data
.align 2
slots:.word 0,0
