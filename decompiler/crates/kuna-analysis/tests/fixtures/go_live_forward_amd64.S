/* Go ABIInternal AMD64 forwarding and stack-growth controls. Static analysis only. */
.text

.macro begin name
.globl \name
.type \name,@function
\name:
.endm

.macro endfn name
.size \name,.-\name
.endm

.macro guard name
    cmp 0x10(%r14),%rsp
    jbe .Lgrow_\name
.endm

.macro grow name
.Lgrow_\name:
    mov %rax,8(%rsp)
    mov %rbx,16(%rsp)
    call runtime.morestack_noctxt.abi0
    mov 8(%rsp),%rax
    mov 16(%rsp),%rbx
    jmp \name
.endm

begin _start
    ret
endfn _start

begin read_word
    mov (%rax),%rax
    mov %rax,sink(%rip)
    ret
endfn read_word

begin read_text
    mov %rax,8(%rsp)
    test %rbx,%rbx
    jz .Lempty
    movzbl -1(%rax,%rbx),%eax
    mov $1,%ebx
    ret
.Lempty:
    xor %eax,%eax
    xor %ebx,%ebx
    ret
endfn read_text

begin read_nothing
    movq $7,sink(%rip)
    ret
endfn read_nothing

begin forward_word
    guard forward_word
    sub $8,%rsp
    call read_word
    add $8,%rsp
    ret
    grow forward_word
endfn forward_word

begin forward_text
    guard forward_text
    sub $24,%rsp
    mov %rax,32(%rsp)
    call read_text
    add $24,%rsp
    ret
    grow forward_text
endfn forward_text

begin no_guard
    sub $8,%rsp
    call read_word
    add $8,%rsp
    ret
endfn no_guard

begin clobber_word
    guard clobber_word
    sub $8,%rsp
    lea sink(%rip),%rax
    call read_word
    add $8,%rsp
    ret
    grow clobber_word
endfn clobber_word

begin forward_void
    guard forward_void
    sub $8,%rsp
    call read_nothing
    add $8,%rsp
    ret
    grow forward_void
endfn forward_void

begin multiple_calls
    guard multiple_calls
    sub $8,%rsp
    call read_nothing
    call read_word
    add $8,%rsp
    ret
    grow multiple_calls
endfn multiple_calls

begin runtime.morestack_noctxt.abi0
    ud2
endfn runtime.morestack_noctxt.abi0

.data
.p2align 3
.globl sink
.type sink,@object
sink:
    .quad 0
.size sink,8

.section .note.go.buildid,"a",@note
    .long 4,8,4
    .asciz "Go"
    .byte 0
    .ascii "authored"

.section .note.GNU-stack,"",@progbits
