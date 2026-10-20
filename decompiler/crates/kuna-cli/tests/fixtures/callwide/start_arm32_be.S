.syntax unified
.arch armv5te
.arm
.section .text.harness_start,"ax",%progbits
.global _start
.hidden _start
.type _start,%function
_start:
    bl callwide_run
    mov r7,#1
    svc #0
.size _start,.-_start
