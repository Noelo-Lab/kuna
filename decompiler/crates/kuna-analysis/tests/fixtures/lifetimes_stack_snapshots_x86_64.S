# Cache a callback-mutated frame word before reusing its bytes as int and Handle.
# clang -nostdlib -no-pie -Wl,--build-id=none,-Ttext=0x401000,-Tdata=0x402000,-e,_start -o lifetimes_stack_snapshots_x86_64 lifetimes_stack_snapshots_x86_64.S
.intel_syntax noprefix
.text
.global cached_stack_word, callback_full, callback_half, read_integer, read_handle, _start
.type cached_stack_word,@function
cached_stack_word:
    push r12
    push rbx
    push rbp
    sub rsp,16
    mov rbp,rsi
    mov [rsp],rdi
    mov rdi,rsp
    call rbp
    mov r12,[rsp]
    mov dword ptr [rsp],5
    mov rdi,rsp
    call read_integer
    mov ebx,eax
    lea rax,[rip+datum]
    mov [rsp],rax
    mov rdi,rsp
    call read_handle
    add eax,ebx
    add r12,rax
    mov rax,r12
    add rsp,16
    pop rbp
    pop rbx
    pop r12
    ret
.size cached_stack_word,.-cached_stack_word
.type callback_full,@function
callback_full:
    add qword ptr [rdi],2
    ret
.size callback_full,.-callback_full
.type callback_half,@function
callback_half:
    add dword ptr [rdi],2
    ret
.size callback_half,.-callback_half
.type read_integer,@function
read_integer:
    mov eax,[rdi]
    ret
.size read_integer,.-read_integer
.type read_handle,@function
read_handle:
    mov rax,[rdi]
    mov eax,[rax]
    ret
.size read_handle,.-read_handle
.type _start,@function
_start:
    mov edi,13
    lea rsi,[rip+callback_full]
    call cached_stack_word
    cmp rax,75
    jne .Lbad
    mov rdi,-7
    lea rsi,[rip+callback_full]
    call cached_stack_word
    cmp rax,55
    jne .Lbad
    movabs rdi,0x12345678fffffffe
    lea rsi,[rip+callback_full]
    call cached_stack_word
    movabs rcx,0x123456790000003c
    cmp rax,rcx
    jne .Lbad
    movabs rdi,0x12345678fffffffe
    lea rsi,[rip+callback_half]
    call cached_stack_word
    movabs rcx,0x123456780000003c
    cmp rax,rcx
    jne .Lbad
    lea rdi,[rip+queue]
    lea rsi,[rip+source]
    call publish_frame_word
    cmp eax,60
    jne .Lbad
    lea rax,[rip+datum]
    cmp [rip+slots],rax
    jne .Lbad
    lea rax,[rip+slots+8]
    cmp [rip+queue+8],rax
    jne .Lbad
    lea rdi,[rip+queue]
    lea rsi,[rip+source]
    call publish_frame_word
    cmp eax,60
    jne .Lbad
    lea rax,[rip+datum]
    cmp [rip+slots+8],rax
    jne .Lbad
    lea rdi,[rip+queue]
    lea rsi,[rip+source]
    call publish_frame_word
    cmp eax,60
    jne .Lbad
    lea rax,[rip+slots+16]
    cmp [rip+queue+8],rax
    jne .Lbad
    lea rax,[rip+slots]
    cmp [rip+queue],rax
    jne .Lbad
    xor edi,edi
    jmp .Lexit
.Lbad:
    mov edi,1
.Lexit:
    mov eax,60
    syscall
.size _start,.-_start
.global publish_frame_word
.type publish_frame_word,@function
publish_frame_word:
 push r12
 push rbp
 push rbx
 sub rsp,16
 mov rbx,rdi
 mov r12,rsi
 mov dword ptr [rsp],5
 mov rdi,rsp
 call read_integer
 mov ebp,eax
 mov rax,[r12]
 mov [rsp],rax
 mov rdi,rsp
 call read_handle
 mov r12d,eax
 mov rax,[rbx+8]
 cmp rax,[rbx+16]
 je .Lskip
 mov rcx,[rsp]
 mov [rax],rcx
 add rax,8
 mov [rbx+8],rax
.Lskip:
 lea eax,[rbp+r12]
 add rsp,16
 pop rbx
 pop rbp
 pop r12
 ret
.size publish_frame_word,.-publish_frame_word
.data
.global datum
datum: .long 55
.balign 8
slots: .zero 16
queue: .quad slots,slots,slots+16
source: .quad datum
.section .note.GNU-stack,"",@progbits
