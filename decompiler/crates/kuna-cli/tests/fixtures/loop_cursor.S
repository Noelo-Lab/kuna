.intel_syntax noprefix
.text
.globl cursor_walk
.type cursor_walk,@function
cursor_walk:
 push rbx
 push r12
 push r13
 push r14
 push r15
 mov r15,rdi
 mov r12,rdx
 mov r13d,esi
 test rdi,rdi
 je .done
 test esi,esi
 je .other
 mov eax,[r15]
 test eax,eax
 je .done
 mov rbx,[r15+16]
 lea r14,[rax-1]
 lea r14,[rbx+r14*8]
 cmp rbx,r14
 ja .done
.center:
 mov rdi,[rbx]
 test rdi,rdi
 je .done
 add DWORD PTR [rdi],2
 call r12
 add rbx,8
 cmp rbx,r14
 jbe .center
 jmp .done
.other:
 mov eax,[r15]
 test eax,eax
 je .done
 mov rbx,[r15+16]
 lea r14,[rax-1]
 lea r14,[rbx+r14*8]
 cmp rbx,r14
 ja .done
.walk:
 mov rdi,[rbx]
 test rdi,rdi
 je .done
 add DWORD PTR [rdi],1
 call r12
 add rbx,8
 cmp rbx,r14
 jbe .walk
.done:
 pop r15
 pop r14
 pop r13
 pop r12
 pop rbx
 ret
.size cursor_walk,.-cursor_walk
.section .note.GNU-stack,"",@progbits
