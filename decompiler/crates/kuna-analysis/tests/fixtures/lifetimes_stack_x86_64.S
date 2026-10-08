.intel_syntax noprefix
# Synthetic physical-storage controls for #810. Helpers deliberately observe
# addresses, including a pointer reloaded from a global and an indirect writer.
# Rebuild: clang -nostdlib -no-pie -Wl,-Ttext=0x401000 -Wl,-Tdata=0x402000 -Wl,-e,_start -o lifetimes_stack_x86_64 lifetimes_stack_x86_64.S
.text
.global stack_reuse, read_pair, read_handle, escaped_alias, escaped_two_reads
.global escaped_partial, escaped_branch, escaped_loop, read_escaped
.global callback_rewrite, set_seven, _start
.type stack_reuse,@function
stack_reuse:
    push rbx
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov ebx,eax
    lea rax,[rip+datum]
    mov qword ptr [rsp],rax
    mov rdi,rsp
    call read_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size stack_reuse,.-stack_reuse
.type read_pair,@function
read_pair:
    mov eax,[rdi]
    add eax,[rdi+4]
    ret
.size read_pair,.-read_pair
.type read_handle,@function
read_handle:
    mov rax,[rdi]
    mov eax,[rax]
    ret
.size read_handle,.-read_handle
.type read_escaped,@function
read_escaped:
    mov rax,[rip+escaped_pointer]
    mov eax,[rax]
    ret
.size read_escaped,.-read_escaped
.type escaped_alias,@function
escaped_alias:
    sub rsp,16
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],5
    mov rdx,[rip+escaped_pointer]
    mov dword ptr [rsp],7
    mov eax,[rdx]
    add rsp,16
    ret
.size escaped_alias,.-escaped_alias
.type escaped_two_reads,@function
escaped_two_reads:
    push rbx
    sub rsp,16
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],5
    call read_escaped
    mov ebx,eax
    mov dword ptr [rsp],7
    call read_escaped
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size escaped_two_reads,.-escaped_two_reads
.type escaped_partial,@function
escaped_partial:
    push rbx
    sub rsp,16
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],0x11223344
    call read_escaped
    mov ebx,eax
    mov byte ptr [rsp+1],0xaa
    call read_escaped
    xor eax,ebx
    add rsp,16
    pop rbx
    ret
.size escaped_partial,.-escaped_partial
.type escaped_branch,@function
escaped_branch:
    sub rsp,24
    mov [rip+escaped_pointer],rsp
    test edi,edi
    je .Lelse
    mov dword ptr [rsp],5
    jmp .Ljoin
.Lelse:
    mov dword ptr [rsp],7
.Ljoin:
    call read_escaped
    add rsp,24
    ret
.size escaped_branch,.-escaped_branch
.type escaped_loop,@function
escaped_loop:
    push rbx
    push rbp
    sub rsp,24
    mov [rip+escaped_pointer],rsp
    xor ebx,ebx
    xor ebp,ebp
.Lnext:
    mov dword ptr [rsp],ebx
    call read_escaped
    add ebp,eax
    inc ebx
    cmp ebx,3
    jne .Lnext
    mov eax,ebp
    add rsp,24
    pop rbp
    pop rbx
    ret
.size escaped_loop,.-escaped_loop
.type callback_rewrite,@function
callback_rewrite:
    sub rsp,24
    mov rax,rdi
    mov dword ptr [rsp],3
    mov rdi,rsp
    call rax
    mov eax,[rsp]
    add rsp,24
    ret
.size callback_rewrite,.-callback_rewrite
.type set_seven,@function
set_seven:
    mov dword ptr [rdi],7
    ret
.size set_seven,.-set_seven

# Two formal views must name the same address, including when both arguments
# are the same p-code Varnode.
.global simultaneous_views, same_address, escaped_object_reuse, read_saved_handle
.type simultaneous_views,@function
simultaneous_views:
    sub rsp,24
    lea rax,[rip+datum]
    mov [rsp],rax
    mov rdi,rsp
    mov rsi,rsp
    call same_address
    add rsp,24
    ret
.size simultaneous_views,.-simultaneous_views
.type same_address,@function
same_address:
    cmp rdi,rsi
    jne .Ldifferent_address
    mov rax,[rsi]
    mov eax,[rax]
    inc eax
    ret
.Ldifferent_address:
    xor eax,eax
    ret
.size same_address,.-same_address
.type escaped_object_reuse,@function
escaped_object_reuse:
    push rbx
    sub rsp,16
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov ebx,eax
    lea rax,[rip+datum]
    mov [rsp],rax
    call read_saved_handle
    add ebx,eax
    mov rdi,rsp
    call read_handle
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size escaped_object_reuse,.-escaped_object_reuse
.type read_saved_handle,@function
read_saved_handle:
    mov rax,[rip+escaped_pointer]
    mov rax,[rax]
    mov eax,[rax]
    ret
.size read_saved_handle,.-read_saved_handle


.global escaped_indirect_write, escaped_indirect_partial, callee_updates_saved, write_saved
.type escaped_indirect_write,@function
escaped_indirect_write:
    sub rsp,24
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],3
    mov rdx,[rip+escaped_pointer]
    mov dword ptr [rdx],7
    mov eax,[rsp]
    add rsp,24
    ret
.size escaped_indirect_write,.-escaped_indirect_write
.type escaped_indirect_partial,@function
escaped_indirect_partial:
    sub rsp,24
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],0x11223344
    mov rdx,[rip+escaped_pointer]
    mov byte ptr [rdx+1],0xaa
    mov eax,[rsp]
    add rsp,24
    ret
.size escaped_indirect_partial,.-escaped_indirect_partial
.type callee_updates_saved,@function
callee_updates_saved:
    sub rsp,24
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],3
    call write_saved
    mov eax,[rsp]
    add rsp,24
    ret
.size callee_updates_saved,.-callee_updates_saved
.type write_saved,@function
write_saved:
    mov rax,[rip+escaped_pointer]
    mov dword ptr [rax],7
    ret
.size write_saved,.-write_saved

.global indexed_bytes, indexed_words, indexed_load
.type indexed_bytes,@function
indexed_bytes:
    sub rsp,16
    mov dword ptr [rsp],0x11223344
    xor ecx,ecx
.Lbytes:
    movzx eax,byte ptr [rdi+rcx]
    mov byte ptr [rsp+rcx],al
    inc ecx
    cmp ecx,3
    jne .Lbytes
    mov eax,dword ptr [rsp]
    add rsp,16
    ret
.size indexed_bytes,.-indexed_bytes
.type indexed_words,@function
indexed_words:
    sub rsp,16
    mov dword ptr [rsp],0x11223344
    xor ecx,ecx
.Lwords:
    movzx eax,word ptr [rdi+rcx*2]
    mov word ptr [rsp+rcx*2],ax
    inc ecx
    cmp ecx,2
    jne .Lwords
    mov eax,dword ptr [rsp]
    add rsp,16
    ret
.size indexed_words,.-indexed_words
.type indexed_load,@function
indexed_load:
    sub rsp,16
    mov dword ptr [rsp],0x11223344
    and edi,1
    movzx eax,word ptr [rsp+rdi*2]
    add rsp,16
    ret
.size indexed_load,.-indexed_load
# A dummy PUSH is invisible once its only aliased suffix is overwritten.
# The second control needs the suffix from a whole-word write without overwrite.
.global suffix_reinit, suffix_initial
.type suffix_reinit,@function
suffix_reinit:
    push rax
    mov dword ptr [rsp+4],edi
    lea rax,[rsp+4]
    mov [rip+escaped_pointer],rax
    call read_escaped
    add rsp,8
    ret
.size suffix_reinit,.-suffix_reinit
.type suffix_initial,@function
suffix_initial:
    sub rsp,8
    movabs rax,0x11223344aabbccdd
    mov qword ptr [rsp],rax
    lea rax,[rsp+4]
    mov [rip+escaped_pointer],rax
    call read_escaped
    add rsp,8
    ret
.size suffix_initial,.-suffix_initial
.global suffix_callee_update, suffix_callee_reinit
.type suffix_callee_update,@function
suffix_callee_update:
    sub rsp,8
    movabs rax,0x11223344aabbccdd
    mov qword ptr [rsp],rax
    lea rdi,[rsp+4]
    call set_seven
    mov eax,[rsp+4]
    add rsp,8
    ret
.size suffix_callee_update,.-suffix_callee_update
.type suffix_callee_reinit,@function
suffix_callee_reinit:
    push rax
    mov dword ptr [rsp+4],3
    lea rdi,[rsp+4]
    call set_seven
    mov eax,[rsp+4]
    add rsp,8
    ret
.size suffix_callee_reinit,.-suffix_callee_reinit
.global shifted_views
.type shifted_views,@function
shifted_views:
    push rbx
    sub rsp,16
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov rdi,rsp
    call read_pair
    mov ebx,eax
    mov dword ptr [rsp+4],11
    mov dword ptr [rsp+8],13
    mov rdi,rsp
    call read_pair
    add ebx,eax
    lea rdi,[rsp+4]
    call read_pair
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size shifted_views,.-shifted_views
.global float_reuse, read_float, read_integer
.type float_reuse,@function
float_reuse:
    push rbx
    sub rsp,16
    movss xmm0,[rip+float_datum]
    addss xmm0,xmm0
    movss [rsp],xmm0
    mov rdi,rsp
    call read_float
    mov ebx,eax
    mov dword ptr [rsp],7
    mov rdi,rsp
    call read_integer
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size float_reuse,.-float_reuse
.type read_float,@function
read_float:
    cvttss2si eax,dword ptr [rdi]
    ret
.size read_float,.-read_float
.type read_integer,@function
read_integer:
    mov eax,[rdi]
    ret
.size read_integer,.-read_integer
.global converted_view, returned_pointer, get_pointer
.type converted_view,@function
converted_view:
    push rbx
    sub rsp,16
    cvtsi2ss xmm0,edi
    movss [rsp],xmm0
    mov rdi,rsp
    call read_float
    mov ebx,eax
    mov dword ptr [rsp],9
    mov rdi,rsp
    call read_integer
    add eax,ebx
    add rsp,16
    pop rbx
    ret
.size converted_view,.-converted_view
.type returned_pointer,@function
returned_pointer:
    sub rsp,24
    mov dword ptr [rsp],3
    mov rdi,rsp
    call read_integer
    call get_pointer
    mov [rsp],rax
    mov rdi,rsp
    call read_handle
    add rsp,24
    ret
.size returned_pointer,.-returned_pointer
.type get_pointer,@function
get_pointer:
    lea rax,[rip+datum]
    ret
.size get_pointer,.-get_pointer
.global typed_indirect
.type typed_indirect,@function
typed_indirect:
    push rbx
    push r12
    push r13
    sub rsp,16
    mov r12,rdi
    mov r13,rsi
    mov dword ptr [rsp],5
    mov dword ptr [rsp+4],7
    mov rdi,rsp
    call r12
    mov ebx,eax
    lea rax,[rip+datum]
    mov [rsp],rax
    mov rdi,rsp
    call r13
    add eax,ebx
    add rsp,16
    pop r13
    pop r12
    pop rbx
    ret
.size typed_indirect,.-typed_indirect
.global all_paths_overwrite, one_path_overwrites, loop_overwrites, loop_may_skip
.type all_paths_overwrite,@function
all_paths_overwrite:
    sub rsp,24
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],1
    test edi,edi
    je .Lall_else
    mov dword ptr [rsp],5
    jmp .Lall_join
.Lall_else:
    mov dword ptr [rsp],7
.Lall_join:
    call read_escaped
    add rsp,24
    ret
.size all_paths_overwrite,.-all_paths_overwrite
.type one_path_overwrites,@function
one_path_overwrites:
    sub rsp,24
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],1
    test edi,edi
    je .Lone_join
    mov dword ptr [rsp],5
.Lone_join:
    call read_escaped
    add rsp,24
    ret
.size one_path_overwrites,.-one_path_overwrites
.type loop_overwrites,@function
loop_overwrites:
    sub rsp,24
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],1
.Ldo_loop:
    mov dword ptr [rsp],7
    sub edi,1
    jg .Ldo_loop
    call read_escaped
    add rsp,24
    ret
.size loop_overwrites,.-loop_overwrites
.type loop_may_skip,@function
loop_may_skip:
    sub rsp,24
    mov [rip+escaped_pointer],rsp
    mov dword ptr [rsp],1
    jmp .Lwhile_test
.Lwhile_loop:
    mov dword ptr [rsp],7
    sub edi,1
.Lwhile_test:
    test edi,edi
    jg .Lwhile_loop
    call read_escaped
    add rsp,24
    ret
.size loop_may_skip,.-loop_may_skip
.type _start,@function
_start:
    xor edi,edi
    call all_paths_overwrite
    cmp eax,7
    jne .Lfailed
    mov edi,1
    call all_paths_overwrite
    cmp eax,5
    jne .Lfailed
    xor edi,edi
    call one_path_overwrites
    cmp eax,1
    jne .Lfailed
    mov edi,1
    call one_path_overwrites
    cmp eax,5
    jne .Lfailed
    xor edi,edi
    call loop_overwrites
    cmp eax,7
    jne .Lfailed
    xor edi,edi
    call loop_may_skip
    cmp eax,1
    jne .Lfailed
    mov edi,2
    call loop_may_skip
    cmp eax,7
    jne .Lfailed
    lea rdi,[rip+read_pair]
    lea rsi,[rip+read_handle]
    call typed_indirect
    cmp eax,67
    jne .Lfailed
    mov edi,7
    call converted_view
    cmp eax,16
    jne .Lfailed
    mov edi,-7
    call converted_view
    cmp eax,2
    jne .Lfailed
    call returned_pointer
    cmp eax,55
    jne .Lfailed
    call float_reuse
    cmp eax,10
    jne .Lfailed
    call shifted_views
    cmp eax,52
    jne .Lfailed
    call suffix_callee_update
    cmp eax,7
    jne .Lfailed
    call suffix_callee_reinit
    cmp eax,7
    jne .Lfailed
    mov edi,123
    call suffix_reinit
    cmp eax,123
    jne .Lfailed
    call suffix_initial
    cmp eax,0x11223344
    jne .Lfailed
    call escaped_indirect_write
    cmp eax,7
    jne .Lfailed
    call escaped_indirect_partial
    cmp eax,0x1122aa44
    jne .Lfailed
    call callee_updates_saved
    cmp eax,7
    jne .Lfailed
    call simultaneous_views
    cmp eax,56
    jne .Lfailed
    call escaped_object_reuse
    cmp eax,122
    jne .Lfailed
    call stack_reuse
    cmp eax,67
    jne .Lfailed
    call escaped_alias
    cmp eax,7
    jne .Lfailed
    call escaped_two_reads
    cmp eax,12
    jne .Lfailed
    call escaped_partial
    cmp eax,0x9900
    jne .Lfailed
    xor edi,edi
    call escaped_branch
    cmp eax,7
    jne .Lfailed
    mov edi,1
    call escaped_branch
    cmp eax,5
    jne .Lfailed
    call escaped_loop
    cmp eax,3
    jne .Lfailed
    lea rdi,[rip+set_seven]
    call callback_rewrite
    cmp eax,7
    jne .Lfailed
    lea rdi,[rip+indexed_source]
    call indexed_bytes
    cmp eax,0x11ccbbaa
    jne .Lfailed
    lea rdi,[rip+indexed_source]
    call indexed_words
    cmp eax,0xddccbbaa
    jne .Lfailed
    xor edi,edi
    call indexed_load
    cmp eax,0x3344
    jne .Lfailed
    mov edi,1
    call indexed_load
    cmp eax,0x1122
    jne .Lfailed
    xor edi,edi
    jmp .Lexit
.Lfailed:
    mov edi,1
.Lexit:
    mov eax,60
    syscall
.size _start,.-_start
.data
.p2align 3
.global datum, escaped_pointer
datum: .long 55
.p2align 3
escaped_pointer: .quad 0
indexed_source: .byte 0xaa,0xbb,0xcc,0xdd
.global float_datum
float_datum: .float 1.5
.section .note.GNU-stack,"",@progbits
