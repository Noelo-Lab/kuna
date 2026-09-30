# Several `name`/`type` assertions in one run (issue #784).
#
# `two_locals` keeps two independent values in ebx and r12d and passes both to
# `observe`; with `prototype observe void observe(int index, int sum)` they print
# as `int4 index; // ebx` and `int4 sum; // r12d`.  `three_locals` adds r13d,
# and `stack_pair` keeps two ints on the stack by passing their addresses to
# `fill`, so the same batches can be stated on register and stack locals.
#
#   clang -c localnamebatch_x86_64.s -o localnamebatch_x86_64.o
#   ld --build-id=none -e two_locals localnamebatch_x86_64.o -o localnamebatch_x86_64
.text
.globl two_locals
.type two_locals,@function
two_locals:
    push %rbx
    push %r12
    sub $8, %rsp
    xor %ebx, %ebx
    mov $7, %r12d
.Lrepeat:
    add $1, %ebx
    imul $3, %r12d, %r12d
    add %ebx, %r12d
    mov %ebx, %edi
    mov %r12d, %esi
    call observe
    cmp $4, %ebx
    jl .Lrepeat
    lea (%rbx,%r12), %eax
    add $8, %rsp
    pop %r12
    pop %rbx
    ret
.size two_locals, .-two_locals

.globl three_locals
.type three_locals,@function
three_locals:
    push %rbx
    push %r12
    push %r13
    xor %ebx, %ebx
    mov $7, %r12d
    mov $1, %r13d
.Lthree:
    add $1, %ebx
    imul $3, %r12d, %r12d
    add %ebx, %r12d
    add %r12d, %r13d
    mov %ebx, %edi
    mov %r12d, %esi
    mov %r13d, %edx
    call observe3
    cmp $4, %ebx
    jl .Lthree
    lea (%rbx,%r12), %eax
    add %r13d, %eax
    pop %r13
    pop %r12
    pop %rbx
    ret
.size three_locals, .-three_locals

.globl stack_pair
.type stack_pair,@function
stack_pair:
    sub $24, %rsp
    lea 8(%rsp), %rdi
    lea 12(%rsp), %rsi
    call fill
    mov 8(%rsp), %eax
    imul 12(%rsp), %eax
    add $24, %rsp
    ret
.size stack_pair, .-stack_pair

.globl observe
.type observe,@function
observe:
    ret
.size observe, .-observe

.globl observe3
.type observe3,@function
observe3:
    ret
.size observe3, .-observe3

.globl fill
.type fill,@function
fill:
    movl $3, (%rdi)
    movl $4, (%rsi)
    ret
.size fill, .-fill
.section .note.GNU-stack,"",@progbits
