/* Rebuild from this directory with GNU Arm binutils:
   arm-none-eabi-as -o arm_thumb_abi_fixture.o arm_thumb_abi_fixture.s
   arm-none-eabi-ld -n --fatal-warnings -T arm_thumb_abi_fixture.ld \
       -o arm_thumb_abi_linked.elf arm_thumb_abi_fixture.o
   arm-none-eabi-readelf -Wr arm_thumb_abi_fixture.o
   arm-none-eabi-objdump -d arm_thumb_abi_linked.elf
 */
.syntax unified
.arch armv7-a
.thumb
.text
.balign 4
.global thumb_abi_values
.type thumb_abi_values, %function
.thumb_func
thumb_abi_values:
    nop
    /* Force cross-section literal relocations GAS does not emit by syntax.
       REL embeds -4 for the architectural PC bias; PC8 encodes it as 0xff. */
    .reloc ., R_ARM_THM_PC8, thumb_abi_words
    .inst.n 0x48ff                  /* ldr r0, [pc, #imm8 * 4] */
    movw r1, #:lower16:thumb_abi_words+4
    movt r1, #:upper16:thumb_abi_words+4
    ldr r1, [r1]
    .reloc ., R_ARM_THM_ALU_PREL_11_0, thumb_abi_third
    subw r2, pc, #4
    ldr r2, [r2]
    .reloc ., R_ARM_THM_PC12, thumb_abi_fourth
    ldr.w r3, [pc, #-4]
    eors r0, r1
    eors r0, r2
    eors r0, r3
    bx lr
.size thumb_abi_values, .-thumb_abi_values

.global thumb_abi_forward
.type thumb_abi_forward, %function
.thumb_func
thumb_abi_forward:
    b.n thumb_abi_backward_literals
.size thumb_abi_forward, .-thumb_abi_forward
.balign 4

.section .rodata, "a", %progbits
.balign 4
.global thumb_abi_words
.type thumb_abi_words, %object
thumb_abi_words:
    .word 0x11223344, 0x22334455
.global thumb_abi_third
.type thumb_abi_third, %object
thumb_abi_third:
    .word 0x33445566
.size thumb_abi_third, .-thumb_abi_third
.global thumb_abi_fourth
.type thumb_abi_fourth, %object
thumb_abi_fourth:
    .word 0x44556677
.size thumb_abi_fourth, .-thumb_abi_fourth
.size thumb_abi_words, .-thumb_abi_words

.section .text.tail, "ax", %progbits
.balign 4
.global thumb_abi_backward_literals
.type thumb_abi_backward_literals, %function
.thumb_func
thumb_abi_backward_literals:
    nop
    .reloc ., R_ARM_THM_ALU_PREL_11_0, thumb_abi_words
    subw r1, pc, #4
    ldr r0, [r1, #8]
    ldr r1, [r1]
    subs r0, r0, r1
    bx lr
.size thumb_abi_backward_literals, .-thumb_abi_backward_literals

.global thumb_abi_backward
.type thumb_abi_backward, %function
.thumb_func
thumb_abi_backward:
    b.n thumb_abi_values
.size thumb_abi_backward, .-thumb_abi_backward
