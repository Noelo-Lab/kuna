.set noreorder
.text
.globl bounce
.type bounce,@function
bounce:
 or $8,$31,$0
 lui $31,%hi(.Lstep0)
 ori $31,$31,%lo(.Lstep0)
.Ljump:
 jr $31
 nop
.Lstep0:
 sw $4,0($5)
 addiu $4,$4,1
 lui $31,%hi(.Lstep1)
 ori $31,$31,%lo(.Lstep1)
 b .Ljump
 nop
.Lstep1:
 sw $4,4($5)
 lui $3,0x1234
 ori $3,$3,0x5678
 xor $3,$4,$3
 dsll32 $3,$3,0
 dsrl32 $3,$3,0
 dsll32 $2,$4,0
 or $2,$2,$3
 or $31,$8,$0
 b .Ljump
 nop
.size bounce,.-bounce
.globl _start
.type _start,@function
_start:
 lui $5,%hi(slots)
 ori $5,$5,%lo(slots)
 addiu $5,$5,4
 li $4,73
 jal bounce
 nop
 li $4,1
 dsrl32 $9,$2,0
 li $10,74
 bne $9,$10,.Lexit
 nop
 dsll32 $9,$2,0
 dsrl32 $9,$9,0
 lui $10,0x1234
 ori $10,$10,0x5632
 bne $9,$10,.Lexit
 nop
 lw $9,-4($5)
 li $10,12
 bne $9,$10,.Lexit
 nop
 lw $9,0($5)
 li $10,73
 bne $9,$10,.Lexit
 nop
 lw $9,4($5)
 li $10,74
 bne $9,$10,.Lexit
 nop
 lw $9,8($5)
 li $10,34
 bne $9,$10,.Lexit
 nop
 li $4,0
.Lexit:
 li $2,5058
 syscall
.size _start,.-_start
.data
.align 2
slots: .word 12,0,0,34
