/* GH #1406: Thumb code in .rodata, a section with no SHF_EXECINSTR that the
   link script places in the R E PT_LOAD (as GNU ld does on ARM). An ARM
   STT_FUNC carries the Thumb bit in its value. The code at lonely + 8 has no
   symbol (a label would be kept in the symbol table for its relocation). */
    .syntax unified
    .thumb
    .text
    .globl _start
    .type _start, %function
    .thumb_func
_start:
    push {r4, lr}
    bl middle
    bl lonely + 8
    pop {r4, pc}
    .globl goal
    .type goal, %function
    .thumb_func
goal:
    movs r0, #42
    bx lr

    .section .rodata, "a", %progbits
    .align 2
    .globl middle
    .type middle, %function
    .thumb_func
middle:
    push {r4, lr}
    bl goal
    pop {r4, pc}
    .globl lonely
    .type lonely, %function
    .thumb_func
lonely:
    push {r4, lr}
    bl goal
    pop {r4, pc}
    push {r4, lr}
    bl goal
    pop {r4, pc}
