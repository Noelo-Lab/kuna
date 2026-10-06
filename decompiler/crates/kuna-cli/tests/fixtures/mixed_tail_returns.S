# gcc 11 -O2 -fcf-protection=none and clang 14 -O2 builds of functions that
# return a value they also compare on one path and tail-call a value-returning
# function on the other, with two that stay void: one tail-calls a void
# function, one leaves eax unwritten on its other path.
.text
.globl getv
.type getv,@function
getv:
    movl g(%rip), %eax
    leal (%rax,%rax,2), %eax
    ret
.size getv, .-getv

.globl getk
.type getk,@function
getk:
    movl g(%rip), %eax
    imull %edi, %eax
    ret
.size getk, .-getk

.globl getl
.type getl,@function
getl:
    movq gl(%rip), %rax
    leaq (%rax,%rax,4), %rax
    ret
.size getl, .-getl

.globl vsink
.type vsink,@function
vsink:
    movl g(%rip), %eax
    movl %eax, (%rdi)
    ret
.size vsink, .-vsink

.globl keep_gcc
.type keep_gcc,@function
keep_gcc:
    movl (%rdi), %eax
    cmpl $5, %eax
    jg .Lkeep_gcc_ret
    jmp getv
.Lkeep_gcc_ret:
    ret
.size keep_gcc, .-keep_gcc

.globl keep_clang
.type keep_clang,@function
keep_clang:
    movl (%rdi), %eax
    cmpl $5, %eax
    jle .Lkeep_clang_tail
    ret
.Lkeep_clang_tail:
    jmp getv
.size keep_clang, .-keep_clang

.globl fwd_clang
.type fwd_clang,@function
fwd_clang:
    movl (%rdi), %eax
    cmpl $5, %eax
    jle .Lfwd_clang_tail
    ret
.Lfwd_clang_tail:
    movl %eax, %edi
    jmp getk
.size fwd_clang, .-fwd_clang

.globl keepl_gcc
.type keepl_gcc,@function
keepl_gcc:
    movq (%rdi), %rax
    cmpq $5, %rax
    jg .Lkeepl_gcc_ret
    jmp getl
.Lkeepl_gcc_ret:
    ret
.size keepl_gcc, .-keepl_gcc

.globl vkeep_gcc
.type vkeep_gcc,@function
vkeep_gcc:
    movl (%rdi), %eax
    cmpl $5, %eax
    jle .Lvkeep_gcc_tail
    movl %eax, g(%rip)
    ret
.Lvkeep_gcc_tail:
    jmp vsink
.size vkeep_gcc, .-vkeep_gcc

.globl vin
.type vin,@function
vin:
    testl %edi, %edi
    jne .Lvin_tail
    ret
.Lvin_tail:
    jmp getv
.size vin, .-vin

.bss
.globl g
.align 4
.type g,@object
.size g,4
g:
    .zero 4
.globl gl
.align 8
.type gl,@object
.size gl,8
gl:
    .zero 8
.section .note.GNU-stack,"",@progbits
