.intel_syntax noprefix
# Byte-definition controls for logical object identities, including same-layout
# reuse, unchanged contents, branch/loop families and a partial overwrite.
# Rebuild: clang -nostdlib -no-pie -Wl,--build-id=none,-Ttext=0x401000,-Tdata=0x402000,-e,_start -o lifetimes_objects_x86_64 lifetimes_objects_x86_64.S
.text
.global read_pair, obj_repeat, obj_reuse, obj_branch, obj_loop, obj_partial, _start
.type read_pair,@function
read_pair:
    mov eax,[rdi]
    add eax,[rdi+4]
    ret
.size read_pair,.-read_pair
.type obj_repeat,@function
obj_repeat:
    push rbx
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov ebx,eax
    mov rdi,rsp
    call read_pair
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size obj_repeat,.-obj_repeat
.type obj_reuse,@function
obj_reuse:
    push rbx
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov ebx,eax
    mov dword ptr [rsp],9
    mov dword ptr [rsp+4],11
    mov rdi,rsp
    call read_pair
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size obj_reuse,.-obj_reuse
.type obj_branch,@function
obj_branch:
    sub rsp,24
    test edi,edi
    je .Lelse
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    jmp .Ljoin
.Lelse:
    mov dword ptr [rsp],9
    mov dword ptr [rsp+4],11
.Ljoin:
    mov rdi,rsp
    call read_pair
    add rsp,24
    ret
.size obj_branch,.-obj_branch
.type obj_loop,@function
obj_loop:
    push rbx
    push rbp
    push r12
    sub rsp,16
    xor ebp,ebp
    mov r12d,2
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
.Lnext:
    mov rdi,rsp
    call read_pair
    add ebp,eax
    mov dword ptr [rsp],9
    dec r12d
    jne .Lnext
    mov eax,ebp
    add rsp,16
    pop r12
    pop rbp
    pop rbx
    ret
.size obj_loop,.-obj_loop
.type obj_partial,@function
obj_partial:
    push rbx
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov ebx,eax
    mov byte ptr [rsp+1],0xaa
    mov rdi,rsp
    call read_pair
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size obj_partial,.-obj_partial
.type _start,@function
_start:
    call obj_repeat
    cmp eax,24
    jne .Lfail
    call obj_reuse
    cmp eax,32
    jne .Lfail
    xor edi,edi
    call obj_branch
    cmp eax,20
    jne .Lfail
    mov edi,1
    call obj_branch
    cmp eax,12
    jne .Lfail
    call obj_loop
    cmp eax,28
    jne .Lfail
    call obj_partial
    cmp eax,43544
    jne .Lfail
    call obj_global_reuse
    cmp eax,67
    jne .Lfail
    call obj_volatile_reuse
    cmp eax,67
    jne .Lfail
    xor edi,edi
    call obj_volatile_branch
    cmp eax,14
    jne .Lfail
    mov edi,1
    call obj_volatile_branch
    cmp eax,16
    jne .Lfail
    call obj_volatile_partial
    cmp eax,0x9900
    jne .Lfail
    call obj_volatile_loop
    cmp eax,8
    jne .Lfail
    call obj_global_shifted
    cmp eax,67
    jne .Lfail
    call obj_volatile_shifted
    cmp eax,67
    jne .Lfail
    lea rdi,[rip+indirect_sink]
    call obj_indirect_reuse
    cmp eax,67
    jne .Lfail
    xor edi,edi
    jmp .Lexit
.Lfail:
    mov edi,1
.Lexit:
    mov eax,60
    syscall
.size _start,.-_start

.global obj_global_reuse, obj_volatile_reuse
.macro global_reuse name, pair, handle
.type \name,@function
\name:
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov rax,rsp
    mov [rip+\pair],rax
    mov rcx,[rip+\pair]
    mov r8d,[rcx]
    add r8d,[rcx+4]
    lea rdx,[rip+object_datum]
    mov [rsp],rdx
    mov rax,rsp
    mov [rip+\handle],rax
    mov rcx,[rip+\handle]
    mov rax,[rcx]
    add r8d,[rax]
    mov eax,r8d
    add rsp,16
    ret
.size \name,.-\name
.endm
global_reuse obj_global_reuse, object_pair, object_handle
global_reuse obj_volatile_reuse, volatile_pair, volatile_handle

.global obj_volatile_branch, obj_volatile_partial, obj_volatile_loop
.type obj_volatile_branch,@function
obj_volatile_branch:
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov [rip+volatile_pair],rsp
    mov rcx,[rip+volatile_pair]
    mov r8d,[rcx]
    test edi,edi
    je .Lglobal_else
    mov dword ptr [rsp],11
    jmp .Lglobal_join
.Lglobal_else:
    mov dword ptr [rsp],9
.Lglobal_join:
    mov rcx,[rip+volatile_pair]
    mov eax,[rcx]
    add eax,r8d
    add rsp,16
    ret
.size obj_volatile_branch,.-obj_volatile_branch
.type obj_volatile_partial,@function
obj_volatile_partial:
    sub rsp,16
    mov dword ptr [rsp],0x11223344
    mov dword ptr [rsp+4],7
    mov [rip+volatile_pair],rsp
    mov rcx,[rip+volatile_pair]
    mov r8d,[rcx]
    mov byte ptr [rsp+1],0xaa
    mov rcx,[rip+volatile_pair]
    mov eax,[rcx]
    xor eax,r8d
    add rsp,16
    ret
.size obj_volatile_partial,.-obj_volatile_partial
.type obj_volatile_loop,@function
obj_volatile_loop:
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov [rip+volatile_pair],rsp
    mov rcx,[rip+volatile_pair]
    mov r8d,[rcx]
    xor edx,edx
.Lglobal_next:
    inc edx
    mov [rsp],edx
    cmp edx,3
    jne .Lglobal_next
    mov rcx,[rip+volatile_pair]
    mov eax,[rcx]
    add eax,r8d
    add rsp,16
    ret
.size obj_volatile_loop,.-obj_volatile_loop

.global obj_global_shifted, obj_volatile_shifted, obj_indirect_reuse
.macro shifted_reuse name,pair,handle
.type \name,@function
\name:
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov [rip+\pair],rsp
    mov rcx,[rip+\pair]
    mov r8d,[rcx]
    add r8d,[rcx+4]
    lea rdx,[rip+object_datum]
    mov [rsp+4],rdx
    lea rax,[rsp+4]
    mov [rip+\handle],rax
    mov rcx,[rip+\handle]
    mov rax,[rcx]
    add r8d,[rax]
    mov eax,r8d
    add rsp,16
    ret
.size \name,.-\name
.endm
shifted_reuse obj_global_shifted,object_pair,object_handle
shifted_reuse obj_volatile_shifted,volatile_pair,volatile_handle
.type obj_indirect_reuse,@function
obj_indirect_reuse:
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov [rdi],rsp
    mov rcx,[rdi]
    mov r8d,[rcx]
    add r8d,[rcx+4]
    lea rdx,[rip+object_datum]
    mov [rsp],rdx
    mov [rdi+8],rsp
    mov rcx,[rdi+8]
    mov rax,[rcx]
    add r8d,[rax]
    mov eax,r8d
    add rsp,16
    ret
.size obj_indirect_reuse,.-obj_indirect_reuse

.data
.global object_datum, object_pair, object_handle, volatile_pair, volatile_handle
object_datum: .long 55
.balign 8
object_pair: .quad 0
object_handle: .quad 0
volatile_pair: .quad 0
volatile_handle: .quad 0
.global indirect_sink
indirect_sink: .zero 16
.section .note.GNU-stack,"",@progbits
