# (kuna) Option `calltable`: a switch whose case body opens like a function
# (push rbp; mov rbp,rsp), a call through a table of functions, an unchecked
# jump table of 16-aligned case labels, and a function table that runs into
# the next switch's table.
# Build: as calltable_x86_64.s -o t.o && ld -o calltable_x86_64 t.o && strip -s calltable_x86_64
    .text
    .globl _start
_start:
    mov  $1,%edi
    call sw
    mov  $2,%edi
    call disp
    mov  $1,%edi
    call usw
    mov  $1,%edi
    call sw2
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
    call *funcs(,%rax,8)
    ret
    .p2align 4
fa:
    mov  $1,%eax
    ret
    .p2align 4
fb:
    mov  $2,%eax
    ret
    .p2align 4
usw:
    mov  %edi,%edi
    jmp  *ucases(,%rdi,8)
    .p2align 4
uc0:
    mov  $20,%eax
    jmp  ujoin
    .p2align 4
uc1:
    mov  $21,%eax
ujoin:
    add  $1,%eax
    ret
    .p2align 4
sw2:
    cmp  $1,%edi
    ja   sw2_def
    mov  %edi,%edi
    jmp  *cases2(,%rdi,8)
c2_0:
    mov  $30,%eax
    ret
c2_1:
    mov  $31,%eax
    ret
sw2_def:
    xor  %eax,%eax
    ret
    .section .rodata
    .p2align 3
cases:
    .quad case0, case1, case2
funcs:
    .quad fa, fb
cases2:
    .quad c2_0, c2_1
ucases:
    .quad uc0, uc1
