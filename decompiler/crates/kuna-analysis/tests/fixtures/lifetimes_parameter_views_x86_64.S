# Rebuild: clang -nostdlib -no-pie -Wl,--build-id=none lifetimes_parameter_views_x86_64.S -o lifetimes_parameter_views_x86_64
# Reused Boolean parameter storage keeps incoming bytes, narrow writes and escaped aliases.
.intel_syntax noprefix
.text
.global param_pair, param_incoming, param_mutation, param_escaped
.global read_pair, read_incoming_pair, read_handle, read_saved_handle, _start
.type param_pair,@function
param_pair:
    cmp byte ptr [rsp+8],0
    jne .Lprepared_param_pair
    push rbx
    sub rsp,16
    movabs rax,0x700000005
    mov qword ptr [rsp+32],rax
    lea rdi,[rsp+32]
    call read_pair
    mov ebx,eax
    lea rax,[rip+datum]
    mov qword ptr [rsp+32],rax
    lea rdi,[rsp+32]
    call read_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.Lprepared_param_pair:
    mov eax,99
    ret
.size param_pair,.-param_pair
.type param_incoming,@function
param_incoming:
    push rbx
    sub rsp,16
    mov dword ptr [rsp+36],7
    lea rdi,[rsp+32]
    call read_incoming_pair
    mov ebx,eax
    lea rax,[rip+datum]
    mov qword ptr [rsp+32],rax
    lea rdi,[rsp+32]
    call read_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.Lprepared_param_incoming:
    mov eax,99
    ret
.size param_incoming,.-param_incoming
.type param_mutation,@function
param_mutation:
    push rbx
    sub rsp,16
    mov byte ptr [rsp+32],1
    mov dword ptr [rsp+36],7
    lea rdi,[rsp+32]
    call read_incoming_pair
    mov ebx,eax
    lea rax,[rip+datum]
    mov qword ptr [rsp+32],rax
    lea rdi,[rsp+32]
    call read_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.Lprepared_param_mutation:
    mov eax,99
    ret
.size param_mutation,.-param_mutation
.type param_escaped,@function
param_escaped:
    push rbx
    sub rsp,16
    lea rax,[rsp+32]
    mov qword ptr [rip+escaped_pointer],rax
    mov dword ptr [rsp+36],7
    lea rdi,[rsp+32]
    call read_incoming_pair
    mov ebx,eax
    lea rax,[rip+datum]
    mov qword ptr [rsp+32],rax
    lea rdi,[rsp+32]
    call read_saved_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.Lprepared_param_escaped:
    mov eax,99
    ret
.size param_escaped,.-param_escaped
.type read_pair,@function
read_pair:
    mov eax,dword ptr [rdi]
    add eax,dword ptr [rdi+4]
    ret
.size read_pair,.-read_pair
.type read_incoming_pair,@function
read_incoming_pair:
    movzx eax,byte ptr [rdi]
    add eax,dword ptr [rdi+4]
    ret
.size read_incoming_pair,.-read_incoming_pair
.type read_handle,@function
read_handle:
    mov rax,qword ptr [rdi]
    mov eax,dword ptr [rax]
    ret
.size read_handle,.-read_handle
.type read_saved_handle,@function
read_saved_handle:
    mov rax,qword ptr [rip+escaped_pointer]
    mov rax,qword ptr [rax]
    mov eax,dword ptr [rax]
    ret
.size read_saved_handle,.-read_saved_handle
.type _start,@function
_start:
    sub rsp,8
    mov dword ptr [rsp],0
    call param_pair
    cmp eax,67
    jne .Lfail
    mov dword ptr [rsp],1
    call param_pair
    cmp eax,99
    jne .Lfail
    mov dword ptr [rsp],0
    call param_incoming
    cmp eax,62
    jne .Lfail
    mov dword ptr [rsp],1
    call param_incoming
    cmp eax,63
    jne .Lfail
    mov dword ptr [rsp],0
    call param_mutation
    cmp eax,63
    jne .Lfail
    mov dword ptr [rsp],1
    call param_mutation
    cmp eax,63
    jne .Lfail
    mov dword ptr [rsp],0
    call param_escaped
    cmp eax,62
    jne .Lfail
    mov dword ptr [rsp],1
    call param_escaped
    cmp eax,63
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
.data
.align 4
.global datum, escaped_pointer
datum: .long 55
.align 8
escaped_pointer: .quad 0
