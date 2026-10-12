.text
.globl bounded_tail
.type bounded_tail,@function
bounded_tail:
    testl %edi, %edi
    jne .Lfailure
    movl $5, %eax
    ret
.Lfailure:
    subq $8, %rsp
    call fail_wrapper
    nop
.size bounded_tail, .-bounded_tail
.p2align 4
.globl fail_wrapper
.type fail_wrapper,@function
fail_wrapper:
    subq $8, %rsp
    call fail_leaf
    int3
.size fail_wrapper, .-fail_wrapper
.globl fail_leaf
.type fail_leaf,@function
fail_leaf:
    movl $60, %eax
    movl $73, %edi
    syscall
    ud2
.size fail_leaf, .-fail_leaf
.globl outer_wrapper
.type outer_wrapper,@function
outer_wrapper:
    subq $8, %rsp
    call fail_wrapper
    int3
.size outer_wrapper, .-outer_wrapper
.globl returning_wrapper
.type returning_wrapper,@function
returning_wrapper:
    testl %edi, %edi
    je .Lreturns
    subq $8, %rsp
    call fail_leaf
    int3
.Lreturns:
    ret
.size returning_wrapper, .-returning_wrapper
.globl returning_caller
.type returning_caller,@function
returning_caller:
    subq $8, %rsp
    call returning_wrapper
    addq $8, %rsp
    movl $9, %eax
    ret
.size returning_caller, .-returning_caller
.globl unknown_wrapper
.type unknown_wrapper,@function
unknown_wrapper:
    testl %edi, %edi
    jne .Lunknown
    subq $8, %rsp
    call fail_leaf
    int3
.Lunknown:
    jmp *%rsi
.size unknown_wrapper, .-unknown_wrapper
.globl missing_wrapper
.type missing_wrapper,@function
missing_wrapper:
    testl %edi, %edi
    jne .Lmissing
    subq $8, %rsp
    call fail_leaf
    int3
.Lmissing:
    nop
.size missing_wrapper, .-missing_wrapper
.p2align 4
.globl ordinary_leaf
.type ordinary_leaf,@function
ordinary_leaf:
    ret
.size ordinary_leaf, .-ordinary_leaf
.globl bounded_missing
.type bounded_missing,@function
bounded_missing:
    testl %edi, %edi
    jne .Lmissing_call
    movl $7, %eax
    ret
.Lmissing_call:
    subq $8, %rsp
    call ordinary_leaf
    nop
.size bounded_missing, .-bounded_missing
.p2align 4
.globl trap_only
.type trap_only,@function
trap_only:
    int3
.size trap_only, .-trap_only
.section .note.GNU-stack,"",@progbits
