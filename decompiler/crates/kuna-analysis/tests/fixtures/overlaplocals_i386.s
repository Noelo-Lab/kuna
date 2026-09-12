# Project-authored reduction for overlapping subregister local declarations.
#
# Rebuild:
#   as --32 -o overlaplocals_i386.o overlaplocals_i386.s
#   ld -m elf_i386 --build-id=none -o overlaplocals_i386 overlaplocals_i386.o
#
# Three 10-byte string prefixes are copied through EAX/AX/AL/AH inside a loop.
# Keeping the buffers live until after all three copies makes the emitted locals
# expose both byte pieces of their two-byte whole owners.

.section .text
.globl overlap_bytes
.type overlap_bytes,@function
overlap_bytes:
    subl $256, %esp
.Lsequence:
    movzwl source+8, %eax
    movq source, %xmm0
    pushl $50
    movw %ax, 0x50(%esp)
    leal 0x52(%esp), %eax
    pushl $0
    pushl %eax
    movq %xmm0, 0x50(%esp)
    call memset_fixture
    addl $12, %esp

    movzwl source_two+8, %eax
    movq source_two, %xmm0
    pushl $50
    movw %ax, 0x0c(%esp)
    leal 0x0e(%esp), %eax
    pushl $0
    pushl %eax
    movq %xmm0, 0x0c(%esp)
    call memset_fixture
    addl $12, %esp

    movzwl source_three+8, %eax
    movq source_three, %xmm0
    pushl $50
    movw %ax, 0x90(%esp)
    leal 0x92(%esp), %eax
    pushl $0
    pushl %eax
    movq %xmm0, 0x90(%esp)
    call memset_fixture
    addl $12, %esp

    leal 0x44(%esp), %eax
    pushl %eax
    call consume_bytes
    addl $4, %esp
    pushl %esp
    call consume_bytes
    addl $4, %esp
    leal 0x84(%esp), %eax
    pushl %eax
    call consume_bytes
    addl $4, %esp
    cmpl $0, loop_flag
    jne .Lsequence
    addl $256, %esp
    ret
.size overlap_bytes, .-overlap_bytes

.globl memset_fixture
.type memset_fixture,@function
memset_fixture:
    ret
.size memset_fixture, .-memset_fixture

.globl consume_bytes
.type consume_bytes,@function
consume_bytes:
    ret
.size consume_bytes, .-consume_bytes

.globl _start
.type _start,@function
_start:
    call overlap_bytes
    ret
.size _start, .-_start

.section .rodata
.globl source
source:
    .asciz "0123456789"
.globl source_two
source_two:
    .asciz "abcdefghij"
.globl source_three
source_three:
    .asciz "klmnopqrst"

.section .data
.globl loop_flag
loop_flag:
    .long 0
