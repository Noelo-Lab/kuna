# clang -nostdlib -no-pie -Wl,--build-id=none,-Ttext=0x401000,-Tdata=0x402000,-e,_start -o lifetimes_frame_events_x86_64 lifetimes_frame_events_x86_64.S
.intel_syntax noprefix
.text
.global frame_alias, read_key, read_item, _start
.type frame_alias,@function
frame_alias:
    push r12
    push rbp
    push rbx
    sub rsp,16
    mov r12,rdi
    mov rbp,rsi
    movabs rax,0x1700000001
    mov [rsp],rax
    mov rdi,rsp
    call expose_input
    test r12,r12
    je .Ldone
    mov eax,[r12]
    test eax,eax
    je .Ldone
    mov rbx,[r12+8]
    lea r12,[rbx+rax*8-8]
    cmp rbx,r12
    ja .Ldone
.Lnext:
    mov rax,[rbx]
    test rax,rax
    je .Ldone
    mov ecx,[rax]
    cmp ecx,1
    jne .Lpointer
    mov eax,[rax+4]
    mov [rsp],eax
    mov rdi,rsp
    call read_key
    jmp .Ladvance
.Lpointer:
    cmp ecx,2
    jne .Lobserve
    mov [rsp],rax
    mov rcx,[rbp+8]
    cmp rcx,[rbp+16]
    je .Lgrow
    mov [rcx],rax
    add qword ptr [rbp+8],8
    jmp .Ladvance
.Lgrow:
    mov rdi,rsp
    call read_item
    jmp .Ladvance
.Lobserve:
.Ladvance:
    add rbx,8
    cmp rbx,r12
    jbe .Lnext
.Ldone:
    xor eax,eax
    add rsp,16
    pop rbx
    pop rbp
    pop r12
    ret
.size frame_alias,.-frame_alias
.type frame_seed,@function
frame_seed:
    push r12
    push rbp
    push rbx
    sub rsp,16
    mov r12,rdi
    mov rbp,rsi
    mov dword ptr [rsp],0
    mov rdi,rsp
    call read_key
    test r12,r12
    je .Lseed_done
    mov eax,[r12]
    test eax,eax
    je .Lseed_done
    mov rbx,[r12+8]
    lea r12,[rbx+rax*8-8]
    cmp rbx,r12
    ja .Lseed_done
.Lseed_next:
    mov rax,[rbx]
    test rax,rax
    je .Lseed_done
    mov ecx,[rax]
    cmp ecx,1
    jne .Lseed_pointer
    mov eax,[rax+4]
    mov [rsp],eax
    mov rdi,rsp
    call read_key
    jmp .Lseed_advance
.Lseed_pointer:
    cmp ecx,2
    jne .Lseed_observe
    mov [rsp],rax
    mov rcx,[rbp+8]
    cmp rcx,[rbp+16]
    je .Lseed_grow
    mov [rcx],rax
    add qword ptr [rbp+8],8
    jmp .Lseed_advance
.Lseed_grow:
    mov rdi,rsp
    call read_item
    jmp .Lseed_advance
.Lseed_observe:
.Lseed_advance:
    add rbx,8
    cmp rbx,r12
    jbe .Lseed_next
.Lseed_done:
    xor eax,eax
    add rsp,16
    pop rbx
    pop rbp
    pop r12
    ret
.size frame_seed,.-frame_seed

.global frame_seed, frame_cached, advance_word
.type frame_cached,@function
frame_cached:
    push r12
    sub rsp,16
    mov [rsp],rdi
    mov rdi,rsp
    call advance_word
    mov r12,[rsp]
    mov dword ptr [rsp],5
    mov rdi,rsp
    call read_key
    lea rax,[rip+item_two]
    mov [rsp],rax
    mov rdi,rsp
    call read_item
    mov rax,r12
    add rsp,16
    pop r12
    ret
.size frame_cached,.-frame_cached
.type advance_word,@function
advance_word:
    add qword ptr [rdi],2
    ret
.size advance_word,.-advance_word
.global expose_input
.type expose_input,@function
expose_input:
    jmp read_key
.size expose_input,.-expose_input
.type read_key,@function
read_key:
    mov [rip+held],rdi
    mov eax,[rip+key_calls]
    test eax,eax
    jne .Lrecord
    cmp dword ptr [rip+alias_enabled],0
    je .Lrecord
    mov [rip+items],rdi
.Lrecord:
    lea rcx,[rip+observed_keys]
    mov edx,[rdi]
    mov [rcx+rax*4],edx
    inc eax
    mov [rip+key_calls],eax
    ret
.size read_key,.-read_key
.type read_item,@function
read_item:
    mov rax,[rdi]
    mov [rip+observed_item],rax
    ret
.size read_item,.-read_item
.type _start,@function
_start:
    mov dword ptr [rip+alias_enabled],1
    lea rdi,[rip+array]
    lea rsi,[rip+queue]
    call frame_alias
    cmp dword ptr [rip+key_calls],2
    jne .Lbad
    cmp dword ptr [rip+observed_keys],1
    jne .Lbad
    cmp dword ptr [rip+observed_keys+4],23
    jne .Lbad
    lea rax,[rip+item_two]
    cmp [rip+slots],rax
    jne .Lbad
    mov dword ptr [rip+alias_enabled],0
    mov dword ptr [rip+key_calls],0
    lea rax,[rip+item_one]
    mov [rip+items],rax
    lea rax,[rip+slots+8]
    mov [rip+queue+16],rax
    lea rdi,[rip+array]
    lea rsi,[rip+queue]
    call frame_seed
    cmp dword ptr [rip+key_calls],2
    jne .Lbad
    cmp dword ptr [rip+observed_keys],0
    jne .Lbad
    cmp dword ptr [rip+observed_keys+4],23
    jne .Lbad
    lea rax,[rip+item_two]
    cmp [rip+observed_item],rax
    jne .Lbad
    movabs rdi,0x12345678fffffffe
    call frame_cached
    movabs rcx,0x1234567900000000
    cmp rax,rcx
    jne .Lbad
    xor edi,edi
    jmp .Lexit
.Lbad:
    mov edi,1
.Lexit:
    mov eax,60
    syscall
.size _start,.-_start
.data
.balign 8
held: .quad 0
key_calls: .long 0
alias_enabled: .long 0
observed_keys: .zero 16
observed_item: .quad 0
.balign 8
slots: .zero 16
queue: .quad slots,slots,slots+16
item_one: .long 1,23
item_ignored: .long 0,100
item_two: .long 2,14
items: .quad item_one,item_ignored,item_two,0
array: .long 4,0
    .quad items
.section .note.GNU-stack,"",@progbits
