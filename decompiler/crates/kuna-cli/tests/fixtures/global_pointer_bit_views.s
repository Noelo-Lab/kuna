.data
.p2align 3
.globl storage
.type storage,@object
.size storage,32
storage: .zero 32
.text
.globl sample
.type sample,@function
sample:
    leaq storage(%rip),%rax
    movq (%rdi),%rcx
    movq %rcx,8(%rax)
    movsd 8(%rax),%xmm0
    movsd %xmm0,(%rsi)
    fildll 8(%rax)
    fstpl (%rdx)
    ret
.size sample,.-sample
.section .note.GNU-stack,"",@progbits
