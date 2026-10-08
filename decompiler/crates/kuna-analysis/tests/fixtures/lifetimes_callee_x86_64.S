.intel_syntax noprefix
.text
.macro caller name, helper, saved=0, condition=0
.global \name
.type \name,@function
\name:
    push rbx
    sub rsp,16
    .if \condition
    mov esi,edi
    .endif
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    .if \saved
    mov [rip+callee_saved_pointer],rsp
    .endif
    mov rdi,rsp
    call \helper
    mov ebx,eax
    mov dword ptr [rsp],9
    mov rdi,rsp
    call callee_read_pair
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size \name,.-\name
.endm
caller overwrite_after_read_second, callee_read_second
caller overwrite_after_no_memory, callee_no_memory
caller overwrite_after_hidden_read, callee_hidden_read, 1
caller overwrite_after_branch_read, callee_read_choice, 0, 1
caller overwrite_after_const_write, callee_const_write

.global read_after_const_write
.type read_after_const_write,@function
read_after_const_write:
    sub rsp,24
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov rdi,rsp
    call callee_const_write
    mov eax,[rsp]
    add eax,[rsp+4]
    add rsp,24
    ret
.size read_after_const_write,.-read_after_const_write

.global callee_read_pair, callee_read_second, callee_no_memory
.global callee_hidden_read, callee_read_choice, callee_const_write
.type callee_read_pair,@function
callee_read_pair:
    mov eax,[rdi]
    add eax,[rdi+4]
    ret
.size callee_read_pair,.-callee_read_pair
.type callee_read_second,@function
callee_read_second:
    mov eax,[rdi+4]
    ret
.size callee_read_second,.-callee_read_second
.type callee_no_memory,@function
callee_no_memory:
    mov eax,3
    ret
.size callee_no_memory,.-callee_no_memory
.type callee_hidden_read,@function
callee_hidden_read:
    mov rax,[rip+callee_saved_pointer]
    mov eax,[rax]
    ret
.size callee_hidden_read,.-callee_hidden_read
.type callee_read_choice,@function
callee_read_choice:
    test esi,esi
    jne .Lsecond
    mov eax,[rdi]
    ret
.Lsecond:
    mov eax,[rdi+4]
    ret
.size callee_read_choice,.-callee_read_choice
.type callee_const_write,@function
callee_const_write:
    mov dword ptr [rdi],9
    xor eax,eax
    ret
.size callee_const_write,.-callee_const_write

.global _start
.type _start,@function
_start:
    call overwrite_after_read_second
    cmp eax,23
    jne .Lfail
    call overwrite_after_no_memory
    cmp eax,19
    jne .Lfail
    call overwrite_after_hidden_read
    cmp eax,21
    jne .Lfail
    xor edi,edi
    call overwrite_after_branch_read
    cmp eax,21
    jne .Lfail
    mov edi,1
    call overwrite_after_branch_read
    cmp eax,23
    jne .Lfail
    call overwrite_after_const_write
    cmp eax,16
    jne .Lfail
    call read_after_const_write
    cmp eax,16
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
.global callee_saved_pointer
.type callee_saved_pointer,@object
callee_saved_pointer: .quad 0
.size callee_saved_pointer,8
.section .note.GNU-stack,"",@progbits
