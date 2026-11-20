/* Go 1.22/1.27 guarded receive wrapper shapes and minimal Go DWARF. Static only. */
.text
.globl _start
_start:
    ret

.macro wrapper name, scratch, helper
.globl \name
.type \name,@function
\name:
    cmp 0x10(%r14),%rsp
    jbe .Lgrow\@
    push %rbp
    mov %rsp,%rbp
    sub $24,%rsp
    lea \scratch(%rsp),%rbx
    call \helper
    add $24,%rsp
    pop %rbp
    ret
.Lgrow\@:
    mov %rax,8(%rsp)
    call runtime.morestack_noctxt.abi0
    mov 8(%rsp),%rax
    jmp \name
.Lend\name:
.size \name,.-\name
.endm

wrapper main.recvStatus, 16, runtime.chanrecv2
wrapper main.discardStatus, 16, runtime.chanrecv2
wrapper main.otherSlot, 8, runtime.chanrecv2
wrapper main.widerElement, 16, runtime.chanrecv2
wrapper main.otherCallee, 16, localReceive

.globl runtime.morestack_noctxt.abi0
.type runtime.morestack_noctxt.abi0,@function
runtime.morestack_noctxt.abi0:
    ud2
.size runtime.morestack_noctxt.abi0,.-runtime.morestack_noctxt.abi0

.globl runtime.chanrecv2
.type runtime.chanrecv2,@function
runtime.chanrecv2:
    movq $-7,(%rbx)
    test %rax,%rax
    setne %al
    ret
.LrecvEnd:
.size runtime.chanrecv2,.-runtime.chanrecv2

.globl localReceive
.type localReceive,@function
localReceive:
    movq $-7,(%rbx)
    test %rax,%rax
    setne %al
    ret
.size localReceive,.-localReceive

.section .gopclntab,"a",@progbits
.Lpcln:
    .long 0xfffffff0
    .byte 0,0,1,8
    .quad 1,0,_start
    .quad .Lnames-.Lpcln,0,0,0,.Lftab-.Lpcln
.Lnames:
    .asciz "runtime.chanrecv2"
.Lftab:
    .long runtime.chanrecv2-_start,.LrecvFunc-.Lftab
    .long .LrecvEnd-_start,0
.LrecvFunc:
    .long runtime.chanrecv2-_start,0,0,0

.section .note.go.buildid,"a",@note
    .long 4,8,4
    .asciz "Go"
    .byte 0
    .ascii "authored"

.section .debug_abbrev
    .uleb128 1,0x11
    .byte 1
    .uleb128 0x25,0x08,0x13,0x05,0,0
    .uleb128 2,0x24
    .byte 0
    .uleb128 0x03,0x08,0x3e,0x0b,0x0b,0x0b,0x2900,0x0b,0,0
    .uleb128 3,0x0f
    .byte 0
    .uleb128 0x03,0x08,0x0b,0x0b,0x49,0x13,0,0
    .uleb128 4,0x16
    .byte 0
    .uleb128 0x03,0x08,0x49,0x13,0x2900,0x0b,0x2902,0x13,0,0
    .uleb128 5,0x2e
    .byte 1
    .uleb128 0x03,0x08,0x11,0x01,0x12,0x06,0,0
    .uleb128 6,0x05
    .byte 0
    .uleb128 0x03,0x08,0x4b,0x0c,0x49,0x13,0x02,0x18,0,0
    .uleb128 7,0x13
    .byte 0
    .uleb128 0x03,0x08,0x0b,0x0b,0,0
    .uleb128 0

.section .debug_info
.Lunit:
    .long .LunitEnd-.LunitBody
.LunitBody:
    .short 4
    .long 0
    .byte 8
    .uleb128 1
    .asciz "Go cmd/compile go1.22.10; regabi"
    .short 0x16
.Lint:
    .uleb128 2
    .asciz "int"
    .byte 5,8,2
.Lwide:
    .uleb128 2
    .asciz "int"
    .byte 5,16,2
.Lbool:
    .uleb128 2
    .asciz "bool"
    .byte 2,1,1
.Lrecord:
    .uleb128 7
    .asciz "hchan<int>"
    .byte 96
.Lbacking:
    .uleb128 3
    .asciz "*hchan<int>"
    .byte 8
    .long .Lrecord-.Lunit

.macro chan_type label, element
\label:
    .uleb128 4
    .asciz "<-chan int"
    .long .Lbacking-.Lunit
    .byte 18
    .long \element-.Lunit
.endm

chan_type .Lchan, .Lint
chan_type .LwideChan, .Lwide

.macro signature name, channel, status
    .uleb128 5
    .asciz "\name"
    .quad \name
    .long .Lend\name-\name
    .uleb128 6
    .asciz "c"
    .byte 0
    .long \channel-.Lunit
    .byte 1,0x50
.if \status
    .uleb128 6
    .asciz "~r0"
    .byte 1
    .long .Lbool-.Lunit
    .byte 0
.endif
    .byte 0
.endm

signature main.recvStatus, .Lchan, 1
signature main.discardStatus, .Lchan, 0
signature main.otherSlot, .Lchan, 1
signature main.widerElement, .LwideChan, 1
signature main.otherCallee, .Lchan, 1
    .byte 0
.LunitEnd:

.section .note.GNU-stack,"",@progbits
