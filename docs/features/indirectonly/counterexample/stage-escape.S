.section .text
.globl walk
walk:
    push   %rbp
    push   %rbx
    push   %r14
    sub    $0x20, %rsp
    mov    %rsp, %rbp
    mov    %rdi, %rbx
    mov    %rbp, %rdi
    mov    $3, %esi
    call   a_init
    mov    %rbp, %rdi
    mov    (%rbx), %rsi
    call   a_push
    mov    %rbp, %rdi
    mov    8(%rbx), %rsi
    call   a_push
    mov    %rbp, %rdi
    mov    16(%rbx), %rsi
    call   a_push
    mov    8(%rsp), %r14d
    sub    $1, %r14d
    js     2f
1:
    mov    (%rsp), %rax
    movslq %r14d, %rbx
    mov    (%rax,%rbx,8), %rdi
    call   sink
    sub    $1, %r14d
    cmp    $-1, %r14d
    jne    1b
2:
    mov    %rbp, %rdi
    call   a_done
    add    $0x20, %rsp
    pop    %r14
    pop    %rbx
    pop    %rbp
    ret
.globl a_init
a_init:
    movq   $0, (%rdi)
    mov    %esi, 8(%rdi)
    ret
.globl a_push
a_push:
    mov    %rsi, (%rdi)
    addl   $1, 8(%rdi)
    ret
.globl a_done
a_done:
    mov    8(%rdi), %eax
    ret
.globl sink
sink:
    mov    %rdi, %rax
    ret
.globl _start
_start:
    call   walk
    hlt
