.text
.globl toggle_dispatch
.type toggle_dispatch,@function
toggle_dispatch:
    cmpl $3, %esi
    ja .Lreturn
    movslq %esi, %rax
    movl $0x400000, %r8d
    movl .Ltable(,%rax,4), %edx
    addq %r8, %rdx
    jmp *%rdx
.Lfirst:
    cmpb $0, (%rdi)
    sete %al
    movb %al, (%rdi)
    ret
.Lsecond:
    cmpb $0, 1(%rdi)
    sete %al
    movb %al, 1(%rdi)
    ret
.Llast:
    cmpb $0, 2(%rdi)
    sete %al
    movb %al, 2(%rdi)
.Lreturn:
    ret
.size toggle_dispatch, .-toggle_dispatch
.section .rodata
.p2align 2
.Ltable:
    .long .Lfirst - 0x400000
    .long .Lreturn - 0x400000
    .long .Lsecond - 0x400000
    .long .Llast - 0x400000

.section .note.GNU-stack,"",@progbits
