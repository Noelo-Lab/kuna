# Float bit transfers through declared layouts and physical stack call effects (#810).
# Rebuild: clang -nostdlib -no-pie -Wl,-Ttext=0x401000 -Wl,-Tdata=0x402000 -Wl,-e,_start -o lifetimes_stack_representations_x86_64 lifetimes_stack_representations_x86_64.S
.intel_syntax noprefix
.text
.global xor_view,constant_bits,effect_bits,write_word,double_bits,read_double,observe_quad,read_float,observe_word,_start
.type xor_view,@function
xor_view:
 push rbx
 sub rsp,16
 movss [rsp],xmm0
 mov rdi,rsp
 call read_float
 mov ebx,eax
 xor dword ptr [rsp],0x00400000
 mov rdi,rsp
 call observe_word
 mov rdi,rsp
 call read_float
 add eax,ebx
 add rsp,16
 pop rbx
 ret
.size xor_view,.-xor_view
.type read_float,@function
read_float:
 movss xmm0,[rdi]
 mulss xmm0,[rip+two]
 cvttss2si eax,xmm0
 ret
.size read_float,.-read_float
.type observe_word,@function
observe_word:
 mov eax,[rdi]
 mov [rip+observed_word],eax
 ret
.size observe_word,.-observe_word

.type constant_bits,@function
constant_bits:
 push rbx
 sub rsp,16
 mov dword ptr [rsp],0x3f800000
 mov rdi,rsp
 call read_float
 mov ebx,eax
 xor dword ptr [rsp],0x00400000
 mov rdi,rsp
 call observe_word
 mov rdi,rsp
 call read_float
 add eax,ebx
 add rsp,16
 pop rbx
 ret
.size constant_bits,.-constant_bits
.type effect_bits,@function
effect_bits:
 push rbx
 mov rbx,rdi
 sub rsp,16
 mov dword ptr [rsp],0x3f800000
 mov rdi,rsp
 call observe_word
 mov rdi,rsp
 call rbx
 mov rdi,rsp
 call observe_word
 mov rdi,rsp
 call read_float
 add rsp,16
 pop rbx
 ret
.size effect_bits,.-effect_bits
.type write_word,@function
write_word:
 mov dword ptr [rdi],0x40400000
 ret
.size write_word,.-write_word


.type double_bits,@function
double_bits:
 push rbx
 sub rsp,16
 movsd [rsp],xmm0
 mov rdi,rsp
 call read_double
 mov ebx,eax
 mov rax,0x0008000000000000
 xor qword ptr [rsp],rax
 mov rdi,rsp
 call observe_quad
 mov rdi,rsp
 call read_double
 add eax,ebx
 add rsp,16
 pop rbx
 ret
.size double_bits,.-double_bits
.type read_double,@function
read_double:
 movsd xmm0,[rdi]
 mulsd xmm0,[rip+two_double]
 cvttsd2si eax,xmm0
 ret
.size read_double,.-read_double
.type observe_quad,@function
observe_quad:
 mov rax,[rdi]
 mov [rip+observed_quad],rax
 ret
.size observe_quad,.-observe_quad

_start:
 mov eax,0x3f800000
 movd xmm0,eax
 call xor_view
 cmp eax,5
 jne .Lfail
 cmp dword ptr [rip+observed_word],0x3fc00000
 jne .Lfail
 mov eax,0xbf800000
 movd xmm0,eax
 call xor_view
 cmp eax,-5
 jne .Lfail
 cmp dword ptr [rip+observed_word],0xbfc00000
 jne .Lfail
 mov eax,0x3fc00000
 movd xmm0,eax
 call xor_view
 cmp eax,5
 jne .Lfail
 cmp dword ptr [rip+observed_word],0x3f800000
 jne .Lfail
 mov eax,0x40400000
 movd xmm0,eax
 call xor_view
 cmp eax,10
 jne .Lfail
 cmp dword ptr [rip+observed_word],0x40000000
 jne .Lfail
 mov eax,0xc0400000
 movd xmm0,eax
 call xor_view
 cmp eax,-10
 jne .Lfail
 cmp dword ptr [rip+observed_word],0xc0000000
 jne .Lfail
 mov eax,0x0
 movd xmm0,eax
 call xor_view
 cmp eax,0
 jne .Lfail
 cmp dword ptr [rip+observed_word],0x400000
 jne .Lfail
 mov eax,0x80000000
 movd xmm0,eax
 call xor_view
 cmp eax,0
 jne .Lfail
 cmp dword ptr [rip+observed_word],0x80400000
 jne .Lfail
 call constant_bits
 cmp eax,5
 jne .Lfail
 cmp dword ptr [rip+observed_word],0x3fc00000
 jne .Lfail
 lea rdi,[rip+write_word]
 call effect_bits
 cmp eax,6
 jne .Lfail
 cmp dword ptr [rip+observed_word],0x40400000
 jne .Lfail
 mov rax,0x3ff0000000000000
 movq xmm0,rax
 call double_bits
 cmp eax,5
 jne .Lfail
 mov rax,0x3ff8000000000000
 cmp [rip+observed_quad],rax
 jne .Lfail
 mov rax,0xbff0000000000000
 movq xmm0,rax
 call double_bits
 cmp eax,-5
 jne .Lfail
 mov rax,0xbff8000000000000
 cmp [rip+observed_quad],rax
 jne .Lfail
 mov rax,0x0
 movq xmm0,rax
 call double_bits
 cmp eax,0
 jne .Lfail
 mov rax,0x8000000000000
 cmp [rip+observed_quad],rax
 jne .Lfail
 mov rax,0x8000000000000000
 movq xmm0,rax
 call double_bits
 cmp eax,0
 jne .Lfail
 mov rax,0x8008000000000000
 cmp [rip+observed_quad],rax
 jne .Lfail
 xor edi,edi
 jmp .Lexit
.Lfail:
 mov edi,1
.Lexit:
 mov eax,60
 syscall
.data
.global two,observed_word
two: .float 2.0
observed_word: .long 0

one: .float 1.0

.balign 8
.global two_double,observed_quad
two_double: .double 2.0
observed_quad: .quad 0

.section .note.GNU-stack,"",@progbits
