#ifndef ENTRY_NAME
#define ENTRY_NAME pointer_after_checks
#endif
.text
.globl ENTRY_NAME
.type ENTRY_NAME,@function
ENTRY_NAME:
    sub $24, %rsp
    mov cookie_value(%rip), %r10
    xor %rsp, %r10
    mov %r10, 16(%rsp)
    .rept 271
    call cookie_noop
    .endr
    call make_cookie_object
    mov 16(%rsp), %rdi
    xor %rsp, %rdi
    call cookie_check
    add $24, %rsp
    ret
.size ENTRY_NAME, .-ENTRY_NAME
.globl cookie_noop
.type cookie_noop,@function
cookie_noop:
    incq cookie_noop_count(%rip)
    ret
.size cookie_noop, .-cookie_noop
.globl make_cookie_object
.type make_cookie_object,@function
make_cookie_object:
    lea returned_object(%rip), %rax
    ret
.size make_cookie_object, .-make_cookie_object
.type cookie_check,@function
cookie_check:
    cmp cookie_value(%rip), %rdi
    je .Lcookie_ok
    ud2
.Lcookie_ok:
    ret
.size cookie_check, .-cookie_check
.data
.p2align 3
.globl cookie_noop_count
.type cookie_noop_count,@object
.size cookie_noop_count, 8
cookie_noop_count:
    .quad 0
cookie_value:
    .quad 0x34a51eac2179c003
returned_object:
    .quad 0
.section .note.GNU-stack,"",@progbits
