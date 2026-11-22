@ Assemble with GNU arm-none-eabi-as 2.45.1.20251203:
@ arm-none-eabi-as -o tests/fixtures/bsim/arm-relocations.o tests/fixtures/bsim/arm-relocations.s
.syntax unified
.arch armv7-a
.thumb

.section .text,"ax",%progbits
.p2align 2
.global fixture_arm_relocations
.type fixture_arm_relocations,%function
.type fixture_external_call,%function
.thumb_func
fixture_arm_relocations:
    push {r4, r5, r6, lr}
    bl fixture_external_call
    movw r4, #:lower16:fixture_bss
    movt r4, #:upper16:fixture_bss
    movw r5, #:lower16:fixture_common
    movt r5, #:upper16:fixture_common
    @ LDR r2,[pc,#0], with a two-byte fixup to the separate literal section.
    .inst.n 0x4a00
    .reloc .-2, R_ARM_THM_PC8, fixture_rodata
    str r0, [r4]
    ldr r1, [r5]
    adds r0, r1
    adds r0, r2
    pop {r4, r5, r6, pc}
.size fixture_arm_relocations, .-fixture_arm_relocations

.global fixture_arm_wide_jump
.type fixture_arm_wide_jump,%function
.thumb_func
fixture_arm_wide_jump:
    b.w fixture_arm_short_target
.size fixture_arm_wide_jump, .-fixture_arm_wide_jump

.global fixture_arm_narrow_jump
.type fixture_arm_narrow_jump,%function
.thumb_func
fixture_arm_narrow_jump:
    @ B.n with a -4 addend; force the narrow encoding across sections.
    .inst.n 0xe7fe
    .reloc .-2, R_ARM_THM_JUMP11, fixture_arm_short_target
.size fixture_arm_narrow_jump, .-fixture_arm_narrow_jump

.global fixture_arm_addresses
.type fixture_arm_addresses,%function
.type fixture_external_data,%object
.thumb_func
fixture_arm_addresses:
    movw r0, #:lower16:fixture_rodata
    movt r0, #:upper16:fixture_rodata
    movw r1, #:lower16:fixture_external_data
    movt r1, #:upper16:fixture_external_data
    eors r0, r1
    bx lr
.size fixture_arm_addresses, .-fixture_arm_addresses

.p2align 2
.word fixture_arm_short_target
.word fixture_rodata + 4
.word fixture_bss + 12
.word fixture_common
.word fixture_external_data

.section .text.short,"ax",%progbits
.p2align 2
.global fixture_arm_short_target
.type fixture_arm_short_target,%function
.thumb_func
fixture_arm_short_target:
    adds r0, #7
    bx lr
.size fixture_arm_short_target, .-fixture_arm_short_target

.section .rodata,"a",%progbits
.p2align 2
.global fixture_rodata
.type fixture_rodata,%object
fixture_rodata:
    .word 0x11223344, 0x55667788
.size fixture_rodata, .-fixture_rodata

.section .bss,"aw",%nobits
.p2align 4
    .space 16
.global fixture_bss
.type fixture_bss,%object
fixture_bss:
    .space 16
.size fixture_bss, .-fixture_bss

.comm fixture_common, 32, 16
.section .note.GNU-stack,"",%progbits
