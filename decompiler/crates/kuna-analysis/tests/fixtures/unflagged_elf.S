/* GH #1406: code in .rodata, a section with no SHF_EXECINSTR that the link
   script places in the R E PT_LOAD (as GNU ld does on AArch64). */
    .text
    .globl _start
_start:
    call middle
    ret
    .globl goal
    .type goal, @function
goal:
    mov $42, %eax
    ret
    .globl direct_goal
    .type direct_goal, @function
direct_goal:
    call goal
    ret

    .section .rodata, "a", @progbits
    .globl middle
    .type middle, @function
middle:
    call goal
    ret
    .globl lonely
    .type lonely, @function
lonely:
    call goal
    ret
    .globl table
    .type table, @object
table:
    .byte 0xe8
    .long goal - . - 4
    .size table, 5
.Lunnamed:
    .byte 0xe8
    .long goal - . - 4

    .data
    .globl counter
    .type counter, @object
counter:
    .long 7
