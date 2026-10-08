# Rebuild: clang -nostdlib -no-pie -Wl,--build-id=none lifetimes_parameter_x86_64.S -o lifetimes_parameter_x86_64
# A seventh Boolean input is read before its stack slot holds four-byte call keys.
.intel_syntax noprefix
.text
.global param_reuse, read_key, _start
.type param_reuse,@function
param_reuse:
    cmp byte ptr [rsp+8],0
    jne .Lprepared
    push rbx
    sub rsp,16
    mov dword ptr [rsp+32],10
    lea rdi,[rsp+32]
    call read_key
    mov ebx,eax
    mov dword ptr [rsp+32],11
    lea rdi,[rsp+32]
    call read_key
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.Lprepared:
    mov eax,99
    ret
.size param_reuse,.-param_reuse
.type read_key,@function
read_key:
    mov eax,dword ptr [rdi]
    ret
.size read_key,.-read_key
.type _start,@function
_start:
    sub rsp,8
    mov dword ptr [rsp],0
    call param_reuse
    cmp eax,21
    jne .Lfail
    mov dword ptr [rsp],1
    call param_reuse
    cmp eax,99
    jne .Lfail
    xor edi,edi
    jmp .Lexit
.Lfail:
    mov edi,1
.Lexit:
    mov eax,60
    syscall
.size _start,.-_start
.section .note.GNU-stack,"",@progbits
