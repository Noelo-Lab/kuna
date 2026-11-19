# (kuna) Option `calltable`: a switch whose case body opens like a function
# (push rbp; mov rbp,rsp) and a jump through a table of functions.
# Build: as calltable_x86_64.s -o t.o && ld -o calltable_x86_64 t.o && strip -s calltable_x86_64
    .text
    .globl _start
_start:
    mov  $1,%edi
    call sw
    mov  $2,%edi
    call disp
    mov  $60,%eax
    syscall
    hlt
    .p2align 4
sw:
    cmp  $2,%edi
    ja   sw_def
    mov  %edi,%edi
    jmp  *cases(,%rdi,8)
    .p2align 4
case0:
    push %rbp
    mov  %rsp,%rbp
    mov  $10,%eax
    pop  %rbp
    ret
case1:
    mov  $11,%eax
    ret
case2:
    mov  $12,%eax
    ret
sw_def:
    xor  %eax,%eax
    ret
    .p2align 4
disp:
    mov  %edi,%eax
    mov  funcs(,%rax,8),%rax
    jmp  *%rax
    .p2align 4
fa:
    mov  $1,%eax
    ret
    .p2align 4
fb:
    mov  $2,%eax
    ret
    .section .rodata
    .p2align 3
cases:
    .quad case0, case1, case2
funcs:
    .quad fa, fb
