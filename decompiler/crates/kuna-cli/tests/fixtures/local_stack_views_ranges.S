.intel_syntax noprefix
.text

.macro RANGE_BEGIN name
.global \name
.type \name,@function
\name:
    sub rsp,40
    mov qword ptr [rsp],rdi
    lea rax,[rsp+4]
    mov qword ptr [rip+escaped_pointer],rax
    mov rdx,qword ptr [rip+escaped_pointer]
.endm

.macro RANGE_END name
    mov rax,qword ptr [rsp]
    add rsp,40
    ret
.size \name,.-\name
.endm

RANGE_BEGIN local_range_valid
    mov dword ptr [rdx],esi
RANGE_END local_range_valid

RANGE_BEGIN local_range_cross_word
    mov qword ptr [rdx],rsi
RANGE_END local_range_cross_word

RANGE_BEGIN local_range_cross_partial
    mov word ptr [rdx+3],si
RANGE_END local_range_cross_partial

RANGE_BEGIN local_range_vector
    movq xmm0,rsi
    punpcklqdq xmm0,xmm0
    movdqu xmmword ptr [rdx],xmm0
RANGE_END local_range_vector

.macro RANGE_READ name,bytes
.global \name
.type \name,@function
\name:
    sub rsp,40
    mov qword ptr [rsp],rdi
    mov qword ptr [rsp+8],rsi
    mov rdi,rsp
    call read_range_pair
    mov rdi,rsp
    call read_range_handle
    lea rax,[rsp+4]
    mov qword ptr [rip+escaped_pointer],rax
    mov rax,qword ptr [rip+escaped_pointer]
.if \bytes == 4
    mov eax,dword ptr [rax]
.else
    mov rax,qword ptr [rax]
.endif
    add rsp,40
    ret
.size \name,.-\name
.endm

RANGE_READ local_range_read_valid,4
RANGE_READ local_range_read_cross,8

.section .note.GNU-stack,"",@progbits
