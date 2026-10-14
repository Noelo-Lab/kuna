.text
.globl flip_first_and_add
.type flip_first_and_add,@function
flip_first_and_add:
    push %rbp
    mov %rsp, %rbp
    sub $32, %rsp
    movss %xmm0, -12(%rbp)
    mov %esi, -16(%rbp)
    mov %rdi, %rsi
    lea -8(%rbp), %rdi
    call copy_point
    lea -8(%rbp), %rdi
    call observe_word
    movss .Lsign_mask(%rip), %xmm0
    movss -8(%rbp), %xmm1
    cmpl $0, -16(%rbp)
    je .Lfirst_no_flip
    xorps %xmm0, %xmm1
    movss %xmm1, -8(%rbp)
.Lfirst_no_flip:
    movss .Loverwrite(%rip), %xmm0
    movss %xmm0, -8(%rbp)
    addss -12(%rbp), %xmm1
    movss %xmm1, %xmm0
    leave
    ret
.size flip_first_and_add, .-flip_first_and_add

.globl flip_second_and_add
.type flip_second_and_add,@function
flip_second_and_add:
    push %rbp
    mov %rsp, %rbp
    sub $32, %rsp
    movss %xmm0, -12(%rbp)
    mov %esi, -16(%rbp)
    mov %rdi, %rsi
    lea -8(%rbp), %rdi
    call copy_point
    lea -4(%rbp), %rdi
    call observe_word
    movss .Lsign_mask(%rip), %xmm0
    movss -4(%rbp), %xmm1
    cmpl $0, -16(%rbp)
    je .Lsecond_no_flip
    xorps %xmm0, %xmm1
    movss %xmm1, -4(%rbp)
.Lsecond_no_flip:
    movss .Loverwrite(%rip), %xmm0
    movss %xmm0, -4(%rbp)
    addss -12(%rbp), %xmm1
    movss %xmm1, %xmm0
    leave
    ret
.size flip_second_and_add, .-flip_second_and_add

.globl copy_point
.type copy_point,@function
copy_point:
    movss 0(%rsi), %xmm1
    movss %xmm1, 0(%rdi)
    movss 4(%rsi), %xmm1
    movss %xmm1, 4(%rdi)
    ret
.size copy_point, .-copy_point

.globl observe_word
.type observe_word,@function
observe_word:
    mov 0(%rdi), %eax
    ret
.size observe_word, .-observe_word

.globl integer_to_float
.type integer_to_float,@function
integer_to_float:
    cvtsi2ss %edi, %xmm0
    ret
.size integer_to_float, .-integer_to_float

.section .rodata
.align 4
.Lsign_mask:
    .long 0x80000000
    .long 0
    .long 0
    .long 0
.Loverwrite:
    .float 7.0
.section .note.GNU-stack,"",@progbits
