.intel_syntax noprefix
.global bounded_index_store, read_pair, _start
.text
.type bounded_index_store,@function
bounded_index_store:
    push rbx
    push r12
    sub rsp,24
    mov ebx,edi
    and ebx,1
    mov DWORD PTR [rsp],5
    mov DWORD PTR [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov r12d,eax
    mov DWORD PTR [rsp+rbx*4],9
    mov rdi,rsp
    call read_pair
    add eax,r12d
    add rsp,24
    pop r12
    pop rbx
    ret
.size bounded_index_store,.-bounded_index_store
.global bounded_index_load, bounded_byte_store
.type bounded_index_load,@function
bounded_index_load:
    push rbx
    push r12
    sub rsp,24
    mov ebx,edi
    and ebx,1
    mov DWORD PTR [rsp],5
    mov DWORD PTR [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov r12d,eax
    mov eax,DWORD PTR [rsp+rbx*4]
    add eax,r12d
    add rsp,24
    pop r12
    pop rbx
    ret
.size bounded_index_load,.-bounded_index_load
.type bounded_byte_store,@function
bounded_byte_store:
    push rbx
    push r12
    sub rsp,24
    mov ebx,edi
    and ebx,1
    mov DWORD PTR [rsp],5
    mov DWORD PTR [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov r12d,eax
    mov BYTE PTR [rsp+rbx*4],9
    mov rdi,rsp
    call read_pair
    add eax,r12d
    add rsp,24
    pop r12
    pop rbx
    ret
.size bounded_byte_store,.-bounded_byte_store
.global bounded_reverse_store
.type bounded_reverse_store,@function
bounded_reverse_store:
    push rbx
    push r12
    sub rsp,24
    mov ebx,edi
    and ebx,1
    mov DWORD PTR [rsp],5
    mov DWORD PTR [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov r12d,eax
    lea rax,[rsp+4]
    lea rdx,[rbx*4]
    sub rax,rdx
    mov DWORD PTR [rax],9
    mov rdi,rsp
    call read_pair
    add eax,r12d
    add rsp,24
    pop r12
    pop rbx
    ret
.size bounded_reverse_store,.-bounded_reverse_store
.global unbounded_index_store
.type unbounded_index_store,@function
unbounded_index_store:
    push rbx
    push r12
    sub rsp,24
    mov ebx,edi
    mov DWORD PTR [rsp],5
    mov DWORD PTR [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov r12d,eax
    mov DWORD PTR [rsp+rbx*4],9
    mov rdi,rsp
    call read_pair
    add eax,r12d
    add rsp,24
    pop r12
    pop rbx
    ret
.size unbounded_index_store,.-unbounded_index_store
.type read_pair,@function
read_pair:
    mov eax,DWORD PTR [rdi]
    add eax,DWORD PTR [rdi+4]
    ret
.size read_pair,.-read_pair
.type _start,@function
_start:
    xor edi,edi
    call bounded_index_store
    cmp eax,28
    jne failed
    mov edi,1
    call bounded_index_store
    cmp eax,26
    jne failed
    xor edi,edi
    call bounded_index_load
    cmp eax,17
    jne failed
    mov edi,1
    call bounded_index_load
    cmp eax,19
    jne failed
    xor edi,edi
    call bounded_byte_store
    cmp eax,28
    jne failed
    mov edi,1
    call bounded_byte_store
    cmp eax,26
    jne failed
    xor edi,edi
    call bounded_reverse_store
    cmp eax,26
    jne failed
    mov edi,1
    call bounded_reverse_store
    cmp eax,28
    jne failed
    xor edi,edi
    call unbounded_index_store
    cmp eax,28
    jne failed
    mov edi,1
    call unbounded_index_store
    cmp eax,26
    jne failed
    xor edi,edi
    jmp finish
failed:
    mov edi,1
finish:
    mov eax,60
    syscall
.size _start,.-_start
.section .note.GNU-stack,"",@progbits
