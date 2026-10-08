# RGB bytes followed by pointer reuse in the Windows x64 home area (#810).
# Rebuild: clang -nostdlib -no-pie -Wl,-Ttext=0x401000 -Wl,-Tdata=0x402000 -Wl,-e,_start -o lifetimes_partial_windows_x64 lifetimes_partial_windows_x64.S
.intel_syntax noprefix
.text
.global rgb_reuse,read_rgb,make_manager,read_manager,_start
.type rgb_reuse,@function
rgb_reuse:
 push rbx
 sub rsp,16
 mov byte ptr [rsp+34],r8b
 test dl,dl
 je .Lnormal
 mov byte ptr [rsp+34],r8b
 mov ax,0xc8ff
 jmp .Lcolor
.Lnormal:
 mov byte ptr [rsp+34],255
 mov ax,0x9600
.Lcolor:
 mov word ptr [rsp+32],ax
 lea rcx,[rsp+32]
 call read_rgb
 call make_manager
 mov [rsp+32],rax
 mov rcx,[rsp+32]
 call read_manager
 add eax,[rip+rgb_result]
 add rsp,16
 pop rbx
 ret
.size rgb_reuse,.-rgb_reuse
.type read_rgb,@function
read_rgb:
 movzx eax,byte ptr [rcx]
 movzx edx,byte ptr [rcx+1]
 add eax,edx
 movzx edx,byte ptr [rcx+2]
 add eax,edx
 mov [rip+rgb_result],eax
 ret
.size read_rgb,.-read_rgb
.type make_manager,@function
make_manager:
 lea rax,[rip+datum]
 ret
.size make_manager,.-make_manager
.type read_manager,@function
read_manager:
 mov eax,[rcx]
 ret
.size read_manager,.-read_manager
_start:
 sub rsp,48
 xor edx,edx
 xor r8d,r8d
 call rgb_reuse
 cmp eax,460
 jne .Lfailed
 mov edx,1
 xor r8d,r8d
 call rgb_reuse
 cmp eax,510
 jne .Lfailed
 mov edx,1
 mov r8d,1
 call rgb_reuse
 cmp eax,511
 jne .Lfailed
 mov edx,1
 mov r8d,255
 call rgb_reuse
 cmp eax,765
 jne .Lfailed
 xor edi,edi
 jmp .Lexit
.Lfailed:
 mov edi,1
.Lexit:
 mov eax,60
 syscall
.data
.global datum
datum: .long 55
.global rgb_result
rgb_result: .long 0

.section .note.GNU-stack,"",@progbits
