.text
.globl sample
.type sample,@function
sample:
 testl %edi,%edi
 jne .Lsecond
 movss .Lvalues(%rip),%xmm0
 ret
.Lsecond:
 movss .Lvalues+4(%rip),%xmm0
 ret
.size sample,.-sample
.section .rodata
.p2align 3
.Lvalues:
 .long 0x42640000,0x40414243,0x42700000
.section .note.GNU-stack,"",@progbits
