.section .text.unlikely,"ax",@progbits
.type _ZN6owners5splitEl.cold,@function
_ZN6owners5splitEl.cold:
    mov $44, %eax
    ret
.size _ZN6owners5splitEl.cold, .-_ZN6owners5splitEl.cold

.text
.globl _ZN6owners5splitEl
.type _ZN6owners5splitEl,@function
_ZN6owners5splitEl:
    test %rdi, %rdi
    jz .Lsplit_miss
    jmp *%rdi
.Lsplit_first:
    mov $11, %eax
    ret
.Lsplit_miss:
    mov $77, %eax
    ret
.size _ZN6owners5splitEl, .-_ZN6owners5splitEl

.p2align 4
.globl exported
.type exported,@function
exported:
    mov $55, %eax
    ret

.p2align 4
.Lhidden:
    test %rdi, %rdi
    jz .Lhidden_miss
    jmp *%rdi
.Lhidden_local:
    mov $66, %eax
    ret
.Lhidden_miss:
    mov $77, %eax
    ret

.p2align 4
.Ldeclared:
    dec %rsi
    jz .Ldeclared_miss
.globl inner
.type inner,@function
inner:
    jmp *%rdi
.Ldeclared_local:
    mov $88, %eax
    ret
.Ldeclared_miss:
    mov $77, %eax
    ret

.p2align 4
.globl _start
.type _start,@function
_start:
    lea _ZN6owners5splitEl.cold(%rip), %rdi
    call _ZN6owners5splitEl
    cmp $44, %eax
    jne .Lfailed
    lea .Lsplit_first(%rip), %rdi
    call _ZN6owners5splitEl
    cmp $11, %eax
    jne .Lfailed
    xor %edi, %edi
    call _ZN6owners5splitEl
    cmp $77, %eax
    jne .Lfailed
    lea exported(%rip), %rdi
    call .Lhidden
    cmp $55, %eax
    jne .Lfailed
    lea .Lhidden_local(%rip), %rdi
    call .Lhidden
    cmp $66, %eax
    jne .Lfailed
    xor %edi, %edi
    call .Lhidden
    cmp $77, %eax
    jne .Lfailed
    lea .Ldeclared(%rip), %rdi
    mov $2, %esi
    call .Ldeclared
    cmp $77, %eax
    jne .Lfailed
    lea .Ldeclared_local(%rip), %rdi
    mov $2, %esi
    call .Ldeclared
    cmp $88, %eax
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
