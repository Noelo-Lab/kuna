# i386 ELF fixture for `option spsolve` (tests/stages/kuna-spsolve.xml).
# Build: gcc -m32 -nostdlib -static -Wl,--build-id=none -o spsolve_elf_i386 spsolve_elf_i386.s
    .text
    .globl _start
_start:
    push $7
    push $5
    call caller_sret
    call caller_ptr
    add $4,%esp
    call caller_die
    add $4,%esp
    mov $1,%eax
    xor %ebx,%ebx
    int $0x80

# struct pair mk(int x): the callee pops the hidden result pointer.
    .type mk,@function
mk:
    mov 4(%esp),%eax
    mov 8(%esp),%edx
    mov %edx,(%eax)
    mov %edx,4(%eax)
    ret $4

    .type sum,@function
sum:
    mov 4(%esp),%eax
    mov (%eax),%edx
    add 4(%eax),%edx
    mov %edx,%eax
    ret

# Two paths with a different number of mk calls meet before the last sum.
    .type caller_sret,@function
caller_sret:
    push %ebx
    sub $0x18,%esp
    mov 0x20(%esp),%ebx
    push %ebx
    lea 0xc(%esp),%eax
    push %eax
    call mk
    add $4,%esp
    cmp $3,%ebx
    jle 1f
    push %ebx
    lea 0x14(%esp),%eax
    push %eax
    call mk
    add $4,%esp
    lea 0x10(%esp),%eax
    push %eax
    call sum
    add $4,%esp
    add %eax,%ebx
1:
    lea 0x8(%esp),%eax
    push %eax
    call sum
    add $4,%esp
    add %ebx,%eax
    add $0x18,%esp
    pop %ebx
    ret

# int scale(int a, int b, int c): pops its three arguments.
    .type scale,@function
scale:
    mov 4(%esp),%eax
    imul 8(%esp),%eax
    add 12(%esp),%eax
    ret $12

# The callee is reached through a pointer, so only the caller's own balance
# says what it pops.
    .type caller_ptr,@function
caller_ptr:
    push %ebx
    mov 8(%esp),%ebx
    push $3
    push 0x10(%esp)
    push %ebx
    call *handler
    add 0xc(%esp),%eax
    add %ebx,%eax
    pop %ebx
    ret

    .type check,@function
check:
    mov 4(%esp),%eax
    and $1,%eax
    ret

    .type report,@function
report:
    mov 4(%esp),%eax
    mov %eax,last
    ret

# Leaves through a pointer: nothing in its code says it does not return.
    .type die,@function
die:
    jmp *die_handler

    .type die_impl,@function
die_impl:
    mov $1,%eax
    mov $1,%ebx
    int $0x80
    hlt

# `call die` is followed by the block the other path enters with twelve more
# bytes reserved.
    .type caller_die,@function
caller_die:
    push %ebx
    sub $8,%esp
    mov 0x10(%esp),%ebx
    test %ebx,%ebx
    je 1f
    sub $0xc,%esp
    push %ebx
    call check
    add $0x10,%esp
    sub $0xc,%esp
    test %eax,%eax
    jne 2f
    add $0xc,%esp
    mov %ebx,%eax
    add $8,%esp
    pop %ebx
    ret
1:
    call die
2:
    push %ebx
    call report
    add $0x10,%esp
    xor %eax,%eax
    add $8,%esp
    pop %ebx
    ret

    .data
handler:
    .long scale
die_handler:
    .long die_impl
last:
    .long 0
