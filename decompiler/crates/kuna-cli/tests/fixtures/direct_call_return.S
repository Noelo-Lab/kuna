
.text
.globl checked_return
.type checked_return,@function
checked_return:
    pushq %rsi
    subq $32, %rsp
    movq %rcx, %rsi
    movq %rsi, %rax
    movq %rdx, %rcx
    call check_cookie
    addq $32, %rsp
    popq %rsi
    ret
.size checked_return, .-checked_return
.globl simple_checked_return
.type simple_checked_return,@function
simple_checked_return:
    subq $40, %rsp
    movq %rcx, %rax
    movq %rdx, %rcx
    call simple_check
    addq $40, %rsp
    ret
.size simple_checked_return, .-simple_checked_return
.globl simple_check
.type simple_check,@function
simple_check:
    addq $1, %rcx
    ret
.size simple_check, .-simple_check
.globl check_cookie
.type check_cookie,@function
check_cookie:
    addq $1, %rcx
    testq %rcx, %rcx
    jz .Lfailure
    ret
.Lfailure:
    subq $40, %rsp
    call cookie_failure
    int3
.size check_cookie, .-check_cookie
.globl cookie_failure
.type cookie_failure,@function
cookie_failure:
    movl $60, %eax
    movl $75, %edi
    syscall
    ud2
.size cookie_failure, .-cookie_failure

.section .note.GNU-stack,"",@progbits
