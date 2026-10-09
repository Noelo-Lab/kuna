# Rebuild: clang -nostdlib -no-pie -Wl,--build-id=none lifetimes_parameter_variants_x86_64.S -o lifetimes_parameter_variants_x86_64
# Incoming stack values survive equal-width, multiple-slot, interior and escaped reuse.
.intel_syntax noprefix
.text
.global param_scalar_escaped,read_saved_word,read_word
.type param_scalar_escaped,@function
param_scalar_escaped:
    cmp byte ptr [rsp+8],0
    jne .Lprepared
    push rbx
    sub rsp,16
    lea rax,[rsp+32]
    mov qword ptr [rip+escaped_pointer],rax
    mov dword ptr [rsp+32],10
    lea rdi,[rsp+32]
    call read_word
    mov ebx,eax
    mov dword ptr [rsp+32],11
    lea rdi,[rsp+32]
    call read_saved_word
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.Lprepared:
    mov eax,99
    ret
.size param_scalar_escaped,.-param_scalar_escaped
.type read_word,@function
read_word:
    mov eax,dword ptr [rdi]
    ret
.size read_word,.-read_word
.type read_saved_word,@function
read_saved_word:
    mov rax,qword ptr [rip+escaped_pointer]
    mov eax,dword ptr [rax]
    ret
.size read_saved_word,.-read_saved_word
.global param_equal_float,read_float_handle
.type param_equal_float,@function
param_equal_float:
    push rbx
    sub rsp,16
    movsd xmm0,qword ptr [rsp+32]
    cvttsd2si ebx,xmm0
    lea rax,[rip+datum]
    mov qword ptr [rsp+32],rax
    lea rdi,[rsp+32]
    call read_float_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size param_equal_float,.-param_equal_float
.type read_float_handle,@function
read_float_handle:
    mov rax,qword ptr [rdi]
    mov eax,dword ptr [rax]
    ret
.size read_float_handle,.-read_float_handle
.global param_multiple,read_parameter_pair,read_parameter_handle
.type param_multiple,@function
param_multiple:
    push rbx
    sub rsp,16
    lea rdi,[rsp+32]
    call read_parameter_pair
    mov ebx,eax
    lea rax,[rip+datum]
    mov qword ptr [rsp+32],rax
    lea rdi,[rsp+32]
    call read_parameter_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size param_multiple,.-param_multiple
.type read_parameter_pair,@function
read_parameter_pair:
    movzx eax,byte ptr [rdi]
    add eax,dword ptr [rdi+8]
    ret
.size read_parameter_pair,.-read_parameter_pair
.type read_parameter_handle,@function
read_parameter_handle:
    mov rax,qword ptr [rdi]
    mov eax,dword ptr [rax]
    ret
.size read_parameter_handle,.-read_parameter_handle

.global param_multiple_interior
.type param_multiple_interior,@function
param_multiple_interior:
    push rbx
    sub rsp,16
    lea rdi,[rsp+32]
    call read_parameter_pair
    mov ebx,eax
    lea rax,[rip+datum]
    mov qword ptr [rsp+40],rax
    lea rdi,[rsp+40]
    call read_parameter_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size param_multiple_interior,.-param_multiple_interior
.global param_word_partial
.type param_word_partial,@function
param_word_partial:
    push rbx
    sub rsp,16
    lea rdi,[rsp+36]
    call read_word
    mov ebx,eax
    lea rax,[rip+datum]
    mov [rsp+32],rax
    lea rdi,[rsp+32]
    call read_parameter_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size param_word_partial,.-param_word_partial

.global _start
.type _start,@function
_start:
    sub rsp,16
    mov qword ptr [rsp],0
    call param_scalar_escaped
    cmp eax,21
    jne .Lfail
    mov qword ptr [rsp],1
    call param_scalar_escaped
    cmp eax,99
    jne .Lfail
    movabs rax,0x402a000000000000
    mov qword ptr [rsp],rax
    call param_equal_float
    cmp eax,68
    jne .Lfail
    movabs rax,0x4031000000000000
    mov qword ptr [rsp],rax
    call param_equal_float
    cmp eax,72
    jne .Lfail
    mov qword ptr [rsp],0
    mov qword ptr [rsp+8],13
    call param_multiple
    cmp eax,68
    jne .Lfail
    mov qword ptr [rsp],1
    mov qword ptr [rsp+8],17
    call param_multiple
    cmp eax,73
    jne .Lfail
    mov qword ptr [rsp],0
    mov qword ptr [rsp+8],13
    call param_multiple_interior
    cmp eax,68
    jne .Lfail
    mov qword ptr [rsp],1
    mov qword ptr [rsp+8],17
    call param_multiple_interior
    cmp eax,73
    jne .Lfail
    movabs rax,0xd00000001
    mov [rsp],rax
    call param_word_partial
    cmp eax,68
    jne .Lfail
    movabs rax,0x1100000001
    mov [rsp],rax
    call param_word_partial
    cmp eax,72
    jne .Lfail
    xor edi,edi
    jmp .Lexit
.Lfail:
    mov edi,1
.Lexit:
    mov eax,60
    syscall
.size _start,.-_start
.data
.p2align 3
.global escaped_pointer,datum
escaped_pointer: .quad 0
datum: .long 55
.section .note.GNU-stack,"",@progbits
