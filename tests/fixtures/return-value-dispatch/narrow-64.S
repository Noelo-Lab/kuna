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
 addiu $2,$2,1
.Lstep0:
 lui $31,%hi(.Lstep1)
 ori $31,$31,%lo(.Lstep1)
 b .Ldispatch
 nop
.Lstep1:
 or $31,$8,$0
 b .Ldispatch
 nop
.size bounce,.-bounce
.globl _start
_start:
 li $4,73
 jal bounce
 nop
 li $9,3
 bne $2,$9,.Lfailed
 nop
 li $4,0
 b .Lexit
 nop
.Lfailed:
 li $4,1
.Lexit:
 li $2,5058
 syscall
