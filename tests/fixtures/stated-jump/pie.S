.text
.globl dispatch
.type dispatch,@function
dispatch:
    test %rdi, %rdi
    jz .Lmiss
    jmp *%rdi
.Lfirst:
    mov $11, %eax
    ret
.Lsecond:
    mov $22, %eax
    ret
.Lmiss:
    mov $77, %eax
    ret
.size dispatch, .-dispatch

.globl _start
.type _start,@function
_start:
    lea .Lfirst(%rip), %rdi
    call dispatch
    cmp $11, %eax
    jne .Lfailed
    lea .Lsecond(%rip), %rdi
    call dispatch
    cmp $22, %eax
    jne .Lfailed
    xor %edi, %edi
    call dispatch
    cmp $77, %eax
    jne .Lfailed
    xor %edi, %edi
    jmp .Lexit
.Lfailed:
    mov $1, %edi
.Lexit:
    mov $60, %eax
    syscall
.size _start, .-_start
.section .note.GNU-stack,"",@progbits
