# gcc 11 -O2 -fcf-protection=none and clang 14 -O2 builds of functions that
# return a value they also compare on one path and tail-call a value-returning
# function on the other (keep_zc with gcc's -fzero-call-used-regs=all scrub,
# cut to six registers), with six that stay void: one tail-calls a void
# function, one loads through the pointer it tests, three return only where
# they found eax zero or 7 (`if (!g) return;`, which `if (!g) return 0;` also
# compiles to; through `sete %dl; test %dl,%dl` and `mov %eax,%ecx`), and one
# leaves eax unwritten on its other path.
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

.globl keeps_gcc
.type keeps_gcc,@function
keeps_gcc:
    movslq (%rdi), %rax
    cmpq $5, %rax
    jg .Lkeeps_gcc_ret
    jmp getl
.Lkeeps_gcc_ret:
    ret
.size keeps_gcc, .-keeps_gcc

.globl keep_zc
.type keep_zc,@function
keep_zc:
    movl (%rdi), %eax
    cmpl $5, %eax
    jg .Lkeep_zc_ret
    jmp getv
.Lkeep_zc_ret:
    xorl %edx, %edx
    xorl %ecx, %ecx
    xorl %esi, %esi
    xorl %edi, %edi
    pxor %xmm0, %xmm0
    pxor %xmm1, %xmm1
    ret
.size keep_zc, .-keep_zc

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

.globl vptr
.type vptr,@function
vptr:
    movq (%rdi), %rax
    testq %rax, %rax
    je .Lvptr_ret
    movsbl 4(%rax), %edi
    jmp getk
.Lvptr_ret:
    ret
.size vptr, .-vptr

.globl zflag
.type zflag,@function
zflag:
    movl g(%rip), %eax
    testl %eax, %eax
    jne .Lzflag_tail
    ret
.Lzflag_tail:
    jmp getv
.size zflag, .-zflag

.globl zsete
.type zsete,@function
zsete:
    movl (%rdi), %eax
    cmpl $7, %eax
    sete %dl
    testb %dl, %dl
    jne .Lzsete_ret
    jmp getv
.Lzsete_ret:
    ret
.size zsete, .-zsete

.globl zcopy
.type zcopy,@function
zcopy:
    movl (%rdi), %eax
    movl %eax, %ecx
    testl %ecx, %ecx
    je .Lzcopy_ret
    jmp getv
.Lzcopy_ret:
    ret
.size zcopy, .-zcopy

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
