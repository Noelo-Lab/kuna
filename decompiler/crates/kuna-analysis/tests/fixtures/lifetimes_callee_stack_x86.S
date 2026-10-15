.intel_syntax noprefix
.text
.macro caller name, helper
.global \name
.type \name,@function
\name:
    push ebx
    sub esp,16
    mov dword ptr [esp+4],5
    mov dword ptr [esp+8],7
    lea eax,[esp+4]
    mov [esp],eax
    call \helper
    mov ebx,eax
    mov dword ptr [esp+4],9
    lea eax,[esp+4]
    mov [esp],eax
    call stack_read_pair
    add eax,ebx
    add esp,16
    pop ebx
    ret
.size \name,.-\name
.endm
caller overwrite_after_stack_read, stack_read_second
caller overwrite_after_stack_hidden, stack_hidden_read

.global read_after_stack_write
.type read_after_stack_write,@function
read_after_stack_write:
    sub esp,16
    mov dword ptr [esp+4],5
    mov dword ptr [esp+8],7
    lea eax,[esp+4]
    mov [esp],eax
    call stack_const_write
    mov eax,[esp+4]
    add eax,[esp+8]
    add esp,16
    ret
.size read_after_stack_write,.-read_after_stack_write

.macro returned name, before=0
.global \name
.type \name,@function
\name:
    push ebx
    sub esp,16
    lea eax,[esp+4]
    mov [stack_saved_pointer],eax
    mov dword ptr [esp+4],5
    mov dword ptr [esp+8],7
    mov [esp],eax
    call stack_return_pointer
    .if \before
    mov dword ptr [esp+4],9
    .endif
    mov ebx,[eax]
    .if !\before
    mov dword ptr [esp+4],9
    .endif
    lea eax,[esp+4]
    mov [esp],eax
    call stack_read_pair
    add eax,ebx
    add esp,16
    pop ebx
    ret
.size \name,.-\name
.endm
returned returned_pointer_read_before_overwrite
returned returned_pointer_read_after_overwrite,1

.global stack_return_pointer
.type stack_return_pointer,@function
stack_return_pointer:
    mov eax,[stack_saved_pointer]
    ret
.size stack_return_pointer,.-stack_return_pointer

.global stack_read_pair, stack_read_second, stack_hidden_read, stack_const_write
.type stack_read_pair,@function
stack_read_pair:
    mov eax,[esp+4]
    mov edx,[eax]
    add edx,[eax+4]
    mov eax,edx
    ret
.size stack_read_pair,.-stack_read_pair
.type stack_read_second,@function
stack_read_second:
    mov eax,[esp+4]
    mov eax,[eax+4]
    ret
.size stack_read_second,.-stack_read_second
.type stack_hidden_read,@function
stack_hidden_read:
    mov eax,[esp+8]
    ret
.size stack_hidden_read,.-stack_hidden_read
.type stack_const_write,@function
stack_const_write:
    mov eax,[esp+4]
    mov dword ptr [eax],9
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
    xor ebx,ebx
    jmp .Lexit
.Lfail:
    mov ebx,1
.Lexit:
    mov eax,1
    int 0x80
.size _start,.-_start
.data
.global stack_saved_pointer
.type stack_saved_pointer,@object
stack_saved_pointer: .long 0
.size stack_saved_pointer,4
.section .note.GNU-stack,"",@progbits
