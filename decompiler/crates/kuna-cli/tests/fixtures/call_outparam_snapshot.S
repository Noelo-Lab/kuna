.text
.globl outparam_branch_snapshot
.type outparam_branch_snapshot,@function
outparam_branch_snapshot:
    push %rbp
    push %rbx
    push %r12
    sub $32, %rsp
    mov %edi, %r12d
    mov %rsi, %rbp
    xor %ebx, %ebx
    movq $0, (%rsp)
.Lread_next:
    test %r12d, %r12d
    je .Lread_alternate
    lea (%rsp), %rdi
    call fill_line
    mov (%rsp), %rdi
    test %rax, %rax
    jle .Lread_done
    jmp .Lconsume_line
.Lread_alternate:
    mov %rbp, %rdi
    test %rbp, %rbp
    je .Lread_done
    add $1, %rbp
.Lconsume_line:
    call read_first
    add %rax, %rbx
    test %rax, %rax
    jne .Lread_next
.Lread_done:
    mov %rbx, %rax
    add $32, %rsp
    pop %r12
    pop %rbx
    pop %rbp
    ret
.size outparam_branch_snapshot, .-outparam_branch_snapshot

.globl outparam_expression_safe
.type outparam_expression_safe,@function
outparam_expression_safe:
    sub $8, %rsp
    call identity_pointer
    movzbl (%rax), %eax
    add $1, %rax
    add $8, %rsp
    ret
.size outparam_expression_safe, .-outparam_expression_safe

.globl fill_line
.type fill_line,@function
fill_line:
    mov .Lline_index(%rip), %rax
    cmp $2, %rax
    jae .Lline_eof
    lea .Lline_bytes(%rip), %rdx
    add %rax, %rdx
    mov %rdx, (%rdi)
    incq .Lline_index(%rip)
    mov $1, %eax
    ret
.Lline_eof:
    xor %eax, %eax
    ret
.size fill_line, .-fill_line

.globl read_first
.type read_first,@function
read_first:
    movzbl (%rdi), %eax
    ret
.size read_first, .-read_first

.globl identity_pointer
.type identity_pointer,@function
identity_pointer:
    addb $1, (%rdi)
    mov %rdi, %rax
    ret
.size identity_pointer, .-identity_pointer

.globl reset_line_reader
.type reset_line_reader,@function
reset_line_reader:
    movq $0, .Lline_index(%rip)
    ret
.size reset_line_reader, .-reset_line_reader

.data
.p2align 3
.Lline_index:
    .quad 0
.Lline_bytes:
    .byte 3, 5
.section .note.GNU-stack,"",@progbits
