.set noreorder
.section .text.harness_start,"ax",@progbits
.globl _start
.hidden _start
.type _start,@function
_start:
    jal callwide_run
    nop
    addu $4,$2,$0
    addiu $2,$0,4001
    syscall
.size _start,.-_start
