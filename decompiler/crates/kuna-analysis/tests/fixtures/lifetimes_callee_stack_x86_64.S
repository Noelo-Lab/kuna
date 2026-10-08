.intel_syntax noprefix
.text
.macro caller name, helper
.global \name
.type \name,@function
\name:
    push rbx
    sub rsp,32
    mov dword ptr [rsp+16],5
    mov dword ptr [rsp+20],7
    lea rax,[rsp+16]
    mov [rsp],rax
    xor edi,edi
    xor esi,esi
    xor edx,edx
    xor ecx,ecx
    xor r8d,r8d
    xor r9d,r9d
    call \helper
    mov ebx,eax
    mov dword ptr [rsp+16],9
    lea rdi,[rsp+16]
    call stack_read_pair
    add eax,ebx
    add rsp,32
    pop rbx
    ret
.size \name,.-\name
.endm
caller overwrite_after_stack_read, stack_read_second
caller overwrite_after_stack_hidden, stack_hidden_read

.global read_after_stack_write
.type read_after_stack_write,@function
read_after_stack_write:
    sub rsp,40
    mov dword ptr [rsp+16],5
    mov dword ptr [rsp+20],7
    lea rax,[rsp+16]
    mov [rsp],rax
    xor edi,edi
    xor esi,esi
    xor edx,edx
    xor ecx,ecx
    xor r8d,r8d
    xor r9d,r9d
    call stack_const_write
    mov eax,[rsp+16]
    add eax,[rsp+20]
    add rsp,40
    ret
.size read_after_stack_write,.-read_after_stack_write

.macro returned name, before=0
.global \name
.type \name,@function
\name:
    push rbx
    sub rsp,32
    lea rax,[rsp+16]
    mov [rip+stack_saved_pointer],rax
    mov dword ptr [rsp+16],5
    mov dword ptr [rsp+20],7
    mov [rsp],rax
    xor edi,edi
    xor esi,esi
    xor edx,edx
    xor ecx,ecx
    xor r8d,r8d
    xor r9d,r9d
    call stack_return_pointer
    .if \before
    mov dword ptr [rsp+16],9
    .endif
    mov ebx,[rax]
    .if !\before
    mov dword ptr [rsp+16],9
    .endif
    lea rdi,[rsp+16]
    call stack_read_pair
    add eax,ebx
    add rsp,32
    pop rbx
    ret
.size \name,.-\name
.endm
returned returned_pointer_read_before_overwrite
returned returned_pointer_read_after_overwrite,1

.global stack_return_pointer
.type stack_return_pointer,@function
stack_return_pointer:
    mov rax,[rip+stack_saved_pointer]
    ret
.size stack_return_pointer,.-stack_return_pointer

.global stack_read_pair, stack_read_second, stack_hidden_read, stack_const_write
.type stack_read_pair,@function
stack_read_pair:
    mov eax,[rdi]
    add eax,[rdi+4]
    ret
.size stack_read_pair,.-stack_read_pair
.type stack_read_second,@function
stack_read_second:
    mov rax,[rsp+8]
    mov eax,[rax+4]
    ret
.size stack_read_second,.-stack_read_second
.type stack_hidden_read,@function
stack_hidden_read:
    mov eax,[rsp+24]
    ret
.size stack_hidden_read,.-stack_hidden_read
.type stack_const_write,@function
stack_const_write:
    mov rax,[rsp+8]
    mov dword ptr [rax],9
    xor eax,eax
    ret
.size stack_const_write,.-stack_const_write

.global _start
.type _start,@function
_start:
    call overwrite_after_stack_read
    cmp eax,23
    jne .Lfail
    call overwrite_after_stack_hidden
    cmp eax,21
    jne .Lfail
    call read_after_stack_write
    cmp eax,16
    jne .Lfail
    call returned_pointer_read_before_overwrite
    cmp eax,21
    jne .Lfail
    call returned_pointer_read_after_overwrite
    cmp eax,25
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
.global stack_saved_pointer
.type stack_saved_pointer,@object
stack_saved_pointer: .quad 0
.size stack_saved_pointer,8
.section .note.GNU-stack,"",@progbits
