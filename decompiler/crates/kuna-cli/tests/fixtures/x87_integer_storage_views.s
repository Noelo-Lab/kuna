.data
.globl storage
.type storage,@object
.size storage,32
storage: .zero 32
.text
.globl sample32
.type sample32,@function
sample32:
    leaq storage(%rip),%rax
    flds (%rdi)
    fistpl 4(%rax)
    movss 4(%rax),%xmm0
    ret
.size sample32,.-sample32
.globl sample64
.type sample64,@function
sample64:
    leaq storage(%rip),%rax
    fldl (%rdi)
    fistpll 16(%rax)
    movsd 16(%rax),%xmm0
    ret
.size sample64,.-sample64
.globl sample_both
.type sample_both,@function
sample_both:
    leaq storage(%rip),%rax
    flds (%rdi)
    fistpl 4(%rax)
    movss 4(%rax),%xmm0
    movss %xmm0,(%rsi)
    fildl 4(%rax)
    fstpl (%rdx)
    ret
.size sample_both,.-sample_both
.section .note.GNU-stack,"",@progbits
