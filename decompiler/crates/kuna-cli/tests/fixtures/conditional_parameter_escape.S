.intel_syntax noprefix
.text
.global conditional_parameter_escape, unknown_parameter_escape, read_wide, read_escaped, _start

.macro reused_parameter name, pointer
.type \name,@function
\name:
 push rbx
 sub rsp,16
 lea rax,[rsp+32]
 \pointer
 test edi,edi
 cmovz rax,rcx
 mov [rip+escaped],rax
 movabs rax,0x1234567812345678
 mov [rsp+32],rax
 lea rdi,[rsp+32]
 call read_wide
 mov rbx,rax
 call read_escaped
 xor rax,rbx
 add rsp,16
 pop rbx
 ret
.size \name,.-\name
.endm

reused_parameter conditional_parameter_escape, "lea rcx,[rip+global_word]"
reused_parameter unknown_parameter_escape, "mov rcx,rsi"

.type read_wide,@function
read_wide: mov rax,[rdi]; ret
.size read_wide,.-read_wide
.type read_escaped,@function
read_escaped: mov rax,[rip+escaped]; mov rax,[rax]; ret
.size read_escaped,.-read_escaped

.type _start,@function
_start:
 sub rsp,16
 mov dword ptr [rsp],42
 mov edi,1
 call conditional_parameter_escape
 test rax,rax
 jne .Lbad
 xor edi,edi
 call conditional_parameter_escape
 movabs rcx,0x9a43302d56077469
 cmp rax,rcx
 jne .Lbad
 mov edi,1
 lea rsi,[rip+global_word]
 call unknown_parameter_escape
 test rax,rax
 jne .Lbad
 xor edi,edi
 lea rsi,[rip+global_word]
 call unknown_parameter_escape
 movabs rcx,0x9a43302d56077469
 cmp rax,rcx
 jne .Lbad
 xor edi,edi
 jmp .Lexit
.Lbad: mov edi,1
.Lexit: mov eax,60; syscall
.size _start,.-_start

.data
.global global_word,escaped
global_word: .quad 0x8877665544332211
escaped: .quad 0
.section .note.GNU-stack,"",@progbits
