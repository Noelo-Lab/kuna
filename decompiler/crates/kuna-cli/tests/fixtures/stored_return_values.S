# gcc 11 -O2 -fno-ipa-ra and clang 14 -O2 builds of functions that return a
# value they also store or test, read by `reader`.
.text
.globl triple_store
.type triple_store,@function
triple_store:
    leal (%rdi,%rdi,2), %eax
    movl %eax, gi(%rip)
    ret
.size triple_store, .-triple_store

.globl load_store
.type load_store,@function
load_store:
    movl (%rdi), %eax
    leal 1(%rax), %edx
    movl %eax, gj(%rip)
    movl %edx, gi(%rip)
    ret
.size load_store, .-load_store

.globl load_store_clang
.type load_store_clang,@function
load_store_clang:
    movl (%rdi), %eax
    leal 1(%rax), %ecx
    movl %ecx, gi(%rip)
    movl %eax, gj(%rip)
    ret
.size load_store_clang, .-load_store_clang

.globl triple_test
.type triple_test,@function
triple_test:
    leal (%rdi,%rdi,2), %eax
    cmpl $7, %eax
    jle .Ltest_done
    addl $1, gj(%rip)
.Ltest_done:
    ret
.size triple_test, .-triple_test

.globl triple_test_clang
.type triple_test_clang,@function
triple_test_clang:
    leal (%rdi,%rdi,2), %eax
    cmpl $8, %eax
    jl .Ltestc_done
    addl $1, gj(%rip)
.Ltestc_done:
    ret
.size triple_test_clang, .-triple_test_clang

.globl triple_out
.type triple_out,@function
triple_out:
    leal (%rdi,%rdi,2), %eax
    movl %eax, (%rsi)
    ret
.size triple_out, .-triple_out

.globl triple_void
.type triple_void,@function
triple_void:
    leal (%rdi,%rdi,2), %eax
    movl %eax, gi(%rip)
    ret
.size triple_void, .-triple_void

.globl check
.type check,@function
check:
    xorl %eax, %eax
    cmpl $10, (%rdi)
    setle %al
    leal -5(%rax,%rax,4), %eax
    ret
.size check, .-check

# -fzero-call-used-regs=all: every call-clobbered register but %eax is zeroed.
.globl put
.type put,@function
put:
    pushq %rbp
    movl %esi, %ebp
    pushq %rbx
    movq %rdi, %rbx
    call check
    testl %eax, %eax
    jne .Lput_done
    movl %ebp, (%rbx)
.Lput_done:
    popq %rbx
    popq %rbp
    fldz
    fldz
    fldz
    fldz
    fldz
    fldz
    fldz
    fldz
    fstp %st(0)
    fstp %st(0)
    fstp %st(0)
    fstp %st(0)
    fstp %st(0)
    fstp %st(0)
    fstp %st(0)
    fstp %st(0)
    xorl %edx, %edx
    xorl %ecx, %ecx
    xorl %esi, %esi
    xorl %edi, %edi
    pxor %xmm0, %xmm0
    pxor %xmm1, %xmm1
    pxor %xmm2, %xmm2
    pxor %xmm3, %xmm3
    pxor %xmm4, %xmm4
    pxor %xmm5, %xmm5
    pxor %xmm6, %xmm6
    pxor %xmm7, %xmm7
    xorl %r8d, %r8d
    xorl %r9d, %r9d
    xorl %r10d, %r10d
    xorl %r11d, %r11d
    pxor %xmm8, %xmm8
    pxor %xmm9, %xmm9
    pxor %xmm10, %xmm10
    pxor %xmm11, %xmm11
    pxor %xmm12, %xmm12
    pxor %xmm13, %xmm13
    pxor %xmm14, %xmm14
    pxor %xmm15, %xmm15
    ret
.size put, .-put

.globl reader
.type reader,@function
reader:
    pushq %rbx
    movl $2, %edi
    call triple_store
    movl %eax, %ebx
    leaq cell(%rip), %rdi
    call load_store
    addl %eax, %ebx
    leaq cell(%rip), %rdi
    call load_store_clang
    addl %eax, %ebx
    movl $2, %edi
    call triple_test
    addl %eax, %ebx
    movl $2, %edi
    call triple_test_clang
    addl %eax, %ebx
    movl $2, %edi
    leaq cell(%rip), %rsi
    call triple_out
    addl %eax, %ebx
    movl $2, %edi
    call triple_void
    leaq cell(%rip), %rdi
    movl $3, %esi
    call put
    addl %eax, %ebx
    movl %ebx, %eax
    popq %rbx
    ret
.size reader, .-reader

.bss
.globl gi
.type gi,@object
.size gi, 4
gi:
    .zero 4
.globl gj
.type gj,@object
.size gj, 4
gj:
    .zero 4
.globl cell
.type cell,@object
.size cell, 4
cell:
    .zero 4
