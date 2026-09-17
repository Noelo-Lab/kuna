    .intel_syntax noprefix
    .text
    .globl walk
    .type walk, @function
walk:
    push    rbp
    push    rbx
    push    r14
    sub     rsp, 0x20
    lea     rbp, [rsp]              # &a : a.p at [rsp+0], a.n at [rsp+8]
    mov     rbx, rdi                # items
    mov     rdi, rbp
    mov     esi, 3
    call    a_init@PLT
    mov     rdi, rbp
    mov     rsi, QWORD PTR [rbx]
    call    a_push@PLT
    mov     rdi, rbp
    mov     rsi, QWORD PTR [rbx+8]
    call    a_push@PLT
    mov     rdi, rbp
    mov     rsi, QWORD PTR [rbx+16]
    call    a_push@PLT
    mov     r14d, DWORD PTR [rsp+8]  # i = a.n
    sub     r14d, 1
    js      .Ldone
.Lloop:
    mov     rax, QWORD PTR [rsp]
    movsxd  rbx, r14d
    mov     rdi, QWORD PTR [rax+rbx*8]
    call    sink@PLT
    sub     r14d, 1
    cmp     r14d, -1
    jne     .Lloop
.Ldone:
    mov     rdi, rbp
    call    a_done@PLT
    add     rsp, 0x20
    pop     r14
    pop     rbx
    pop     rbp
    ret
    .size walk, .-walk
    .section .note.GNU-stack,"",@progbits
