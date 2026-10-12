.syntax unified
.arch armv5t
.arm

.section .text,"ax",%progbits
.global CopyFiveWords
.type CopyFiveWords,%function
CopyFiveWords:
    ldr r0, =mutable_tile_fallback
    ldr r1, =source_words
    ldmia r1!, {r2-r6}
    stmia r0!, {r2-r6}
    bx lr
.size CopyFiveWords, .-CopyFiveWords
.ltorg

.section .data,"aw",%progbits
.balign 4
.global source_words
source_words:
    .word 0x11223344, 0x55667788, 0x99aabbcc, 0xddeeff00, 0x01234567
.global mutable_tile_fallback
mutable_tile_fallback:
    .space 20, 0
