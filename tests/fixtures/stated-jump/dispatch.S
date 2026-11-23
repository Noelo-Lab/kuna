.text
.globl dispatch
.type dispatch,@function
dispatch:
    cmp $2, %esi
    ja .Lmiss
    mov (%rdi,%rsi,8), %rax
    jmp *%rax
.Lmiss:
    mov $77, %eax
    ret
.org 0x20
.Lfirst:
    mov $11, %eax
    ret
.org 0x30
.Lsecond:
    mov $22, %eax
    ret
.org 0x50
.Lthird:
    mov $33, %eax
    ret
.size dispatch, .-dispatch

.globl other
.type other,@function
other:
    jmp *%rdi
.size other, .-other

.globl _start
.type _start,@function
_start:
    lea .Lpermutation(%rip), %rdi
    xor %esi, %esi
.Lcheck:
    call dispatch
    cmp .Lresults(,%rsi,4), %eax
    jne .Lfailed
    inc %esi
    cmp $4, %esi
    jne .Lcheck
    xor %edi, %edi
    jmp .Lexit
.Lfailed:
    mov $1, %edi
.Lexit:
    mov $60, %eax
    syscall
.size _start, .-_start

.section .rodata
.p2align 3
.Lpermutation:
    .quad .Lthird,.Lfirst,.Lsecond
.Lresults:
    .long 33,11,22,77
.section .note.GNU-stack,"",@progbits
