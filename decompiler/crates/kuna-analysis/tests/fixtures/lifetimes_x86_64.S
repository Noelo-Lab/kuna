.intel_syntax noprefix
# Two register-lifetime cases for #809; no stack-object recovery is exercised.
# Rebuild: clang -nostdlib -no-pie -Wl,-Ttext=0x401000 -Wl,-e,_start -o lifetimes_x86_64 lifetimes_x86_64.S
.text
.global register_names, register_overlap, consume_number, identity_pointer, observe_number, observe_pointer, _start
.org 0x100
.type consume_number,@function
consume_number:
    lea rax, [rdi + rdi]
    ret
.size consume_number,.-consume_number
.org 0x120
.type register_names,@function
register_names:
    push rbx
    push rbp
    sub rsp, 8
    mov rbp, rsi
    call consume_number
    mov rbx, rax
    mov rdi, rbx
    call observe_number
    mov rdi, rbx
    call observe_number
    mov rdi, rbp
    call identity_pointer
    mov rbx, rax
    mov rdi, rbx
    call observe_pointer
    mov rdi, rbx
    call observe_pointer
    add rsp, 8
    pop rbp
    pop rbx
    ret
.size register_names,.-register_names
.org 0x170
.type identity_pointer,@function
identity_pointer:
    mov rax, rdi
    ret
.size identity_pointer,.-identity_pointer
.org 0x180
.type observe_number,@function
observe_number:
    add qword ptr [rip + observed_total], rdi
    ret
.size observe_number,.-observe_number
.org 0x190
.type observe_pointer,@function
observe_pointer:
    mov eax, dword ptr [rdi]
    add qword ptr [rip + observed_total], rax
    ret
.size observe_pointer,.-observe_pointer
.org 0x200
.type register_overlap,@function
register_overlap:
    push rbx
    push rbp
    push r12
    mov rbp, rsi
    call consume_number
    mov rbx, rax
    mov rdi, rbx
    call observe_number
    mov rdi, rbp
    call identity_pointer
    mov r12, rax
    mov rdi, r12
    call observe_pointer
    mov rdi, rbx
    call observe_number
    mov rdi, r12
    call observe_pointer
    pop r12
    pop rbp
    pop rbx
    ret
.size register_overlap,.-register_overlap
.org 0x240
.type _start,@function
_start:
    mov edi, 13
    lea rsi, [rip + datum]
    call register_names
    cmp qword ptr [rip + observed_total], 162
    jne failed
    mov qword ptr [rip + observed_total], 0
    mov edi, 13
    lea rsi, [rip + datum]
    call register_overlap
    cmp qword ptr [rip + observed_total], 162
    jne failed
    mov edi, 200
    call register_parameter_reuse
    cmp eax, 200
    jne failed
    mov edi, 142
    call register_parameter_reuse
    cmp eax, 116
    jne failed
    mov edi, 31
    call register_parameter_reuse
    cmp eax, 31
    jne failed
    mov edi, -3
    call register_parameter_reuse
    cmp eax, -3
    jne failed
    xor edi, edi
    jmp exit
failed:
    mov edi, 1
exit:
    mov eax, 60
    syscall
.size _start,.-_start
.data
.p2align 3
datum: .long 55
.p2align 3
observed_total: .quad 0
.section .note.GNU-stack,"",@progbits

.text
.global register_branch, register_loop
.org 0x400
.type register_branch,@function
register_branch:
    push rbx
    push rbp
    push r12
    mov rbp,rsi
    mov r12d,edx
    call consume_number
    mov rbx,rax
    test r12d,r12d
    je .Lnumber_once
    mov rdi,rbx
    call observe_number
.Lnumber_once:
    mov rdi,rbx
    call observe_number
    mov rdi,rbp
    call identity_pointer
    mov rbx,rax
    mov rdi,rbx
    call observe_pointer
    mov rdi,rbx
    call observe_pointer
    pop r12
    pop rbp
    pop rbx
    ret
.size register_branch,.-register_branch
.org 0x480
.type register_loop,@function
register_loop:
    push rbx
    push rbp
    push r12
    mov rbp,rsi
    call consume_number
    mov rbx,rax
    mov r12d,2
.Lnumbers:
    mov rdi,rbx
    call observe_number
    dec r12d
    jne .Lnumbers
    mov rdi,rbp
    call identity_pointer
    mov rbx,rax
    mov r12d,2
.Lpointers:
    mov rdi,rbx
    call observe_pointer
    dec r12d
    jne .Lpointers
    pop r12
    pop rbp
    pop rbx
    ret
.size register_loop,.-register_loop

.global register_parameter_reuse, consume_key
.org 0x500
.type register_parameter_reuse,@function
register_parameter_reuse:
    cmp edi,200
    je .Lhundred
    cmp edi,142
    jne .Loriginal
    mov edi,58
    jmp .Lconsume_key
.Lhundred:
    mov edi,100
.Lconsume_key:
    sub rsp,8
    call consume_key
    add rsp,8
    ret
.Loriginal:
    mov eax,edi
    ret
.size register_parameter_reuse,.-register_parameter_reuse
.type consume_key,@function
consume_key:
    lea eax,[rdi+rdi]
    ret
.size consume_key,.-consume_key
