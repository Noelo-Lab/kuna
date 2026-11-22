.set noreorder
.text
.globl bounce
.type bounce,@function
bounce:
 or $8,$31,$0
 li $2,0
 lui $31,%hi(.Lstep0)
 ori $31,$31,%lo(.Lstep0)
.Ldispatch:
 jr $31
 or $2,$4,$0
.Lstep0:
 addiu $4,$4,7
 or $31,$8,$0
 b .Ldispatch
 nop
.size bounce,.-bounce
.globl _start
_start:
 li $4,73
 jal bounce
 nop
 li $9,80
 bne $2,$9,.Lfailed
 nop
 li $4,0
 b .Lexit
 nop
.Lfailed:
 li $4,1
.Lexit:
 li $2,6058
 syscall
