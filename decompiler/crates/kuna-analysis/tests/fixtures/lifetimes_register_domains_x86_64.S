# Independent byte hashes and pointer choices reuse RAX (#809).
# Rebuild: clang -nostdlib -no-pie -Wl,-Ttext=0x401000 -Wl,-Tdata=0x402000 -Wl,-e,_start -o lifetimes_register_domains_x86_64 lifetimes_register_domains_x86_64.S
.intel_syntax noprefix
.text
.global value_domains,_start
.type value_domains,@function
value_domains:
 mov r9,rdi
 mov r10,rsi
 mov r8,0x100000001b3
 movzx eax,byte ptr [rdi+17]
 imul r8,rax
 movzx eax,byte ptr [rdi+18]
 xor r8,rax
 test ecx,ecx
 jz .Lfirst
 mov rax,rsi
 jmp .Lselected
.Lfirst:
 mov rax,rdi
.Lselected:
 mov [rdx],rax
 movzx eax,byte ptr [r9+19]
 xor r8,rax
 mov [rip+observed_hash],r8
 mov rax,r10
 ret
.size value_domains,.-value_domains
_start:
 lea rdi,[rip+first]
 lea rsi,[rip+last]
 lea rdx,[rip+selected]
 xor ecx,ecx
 call value_domains
 lea rsi,[rip+last]
 cmp rax,rsi
 jne .Lfail
 lea rdi,[rip+first]
 cmp [rip+selected],rdi
 jne .Lfail
 mov r8,0x100000001b3
 imul r8,r8,86
 xor r8,52
 xor r8,18
 cmp [rip+observed_hash],r8
 jne .Lfail
 lea rdx,[rip+selected]
 mov ecx,1
 call value_domains
 lea rsi,[rip+last]
 cmp rax,rsi
 jne .Lfail
 cmp [rip+selected],rsi
 jne .Lfail
 xor edi,edi
 jmp .Lexit
.Lfail:
 mov edi,1
.Lexit:
 mov eax,60
 syscall
.data
.global first,last,selected,observed_hash
first: .quad 0,0
 .long 0x12345678,0
last: .quad 0,0
 .long 0,0
selected: .quad 0
observed_hash: .quad 0
.section .note.GNU-stack,"",@progbits
