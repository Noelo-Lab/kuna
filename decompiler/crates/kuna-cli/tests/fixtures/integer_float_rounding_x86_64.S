.text
.globl rounded32
.type rounded32,@function
rounded32:
 cvtsi2ssl %edi,%xmm0
 cvtss2sd %xmm0,%xmm0
 ret
.size rounded32,.-rounded32

.globl narrowed64
.type narrowed64,@function
narrowed64:
 cvtsi2sdq %rdi,%xmm0
 cvtsd2ss %xmm0,%xmm0
 ret
.size narrowed64,.-narrowed64

.globl widened16
.type widened16,@function
widened16:
 movswl %di,%eax
 cvtsi2ssl %eax,%xmm0
 cvtss2sd %xmm0,%xmm0
 ret
.size widened16,.-widened16

.globl exact32
.type exact32,@function
exact32:
 cvtsi2sdl %edi,%xmm0
 cvtsd2ss %xmm0,%xmm0
 ret
.size exact32,.-exact32
.globl trunc_after_narrow
.type trunc_after_narrow,@function
trunc_after_narrow:
 cvtsi2sdl %edi,%xmm0
 cvtsd2ss %xmm0,%xmm1
 movss %xmm1,(%rsi)
 cvttsd2si %xmm0,%eax
 ret
.size trunc_after_narrow,.-trunc_after_narrow
.section .note.GNU-stack,"",@progbits
