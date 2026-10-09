.intel_syntax noprefix
.text
.global wide_names, wide_source, observe_wide, _start
.type wide_names,@function
wide_names:
    push rbx
    push r12
    sub rsp,8
    call wide_source
    mov rbx,rax
    mov r12,rdx
    mov rdi,rbx
    mov rsi,r12
    call observe_wide
    mov rdi,rbx
    mov rsi,r12
    call observe_wide
    add rsp,8
    pop r12
    pop rbx
    ret
.size wide_names,.-wide_names
.type wide_source,@function
wide_source:
    mov eax,13
    mov edx,55
    ret
.size wide_source,.-wide_source
.type observe_wide,@function
observe_wide:
    add rdi,rsi
    add [rip+wide_total],rdi
    ret
.size observe_wide,.-observe_wide
.type _start,@function
_start:
    call wide_names
    xor edi,edi
    cmp qword ptr [rip+wide_total],136
    sete dil
    xor edi,1
    mov eax,60
    syscall
.size _start,.-_start
.data
.global wide_total
.type wide_total,@object
.size wide_total,8
wide_total: .quad 0
.section .note.GNU-stack,"",@progbits
