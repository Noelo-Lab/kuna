/* Fixture source for `a32tablefp_arm_le32`: predicated computed jumps that
   are not A32 jump tables, mapped at 0xff000 so a GOT-offset pool word
   decodes as `andeq pc,pc,...`.

   `guarded` falls through `bl __stack_chk_fail` into a pool whose first word
   reads as a predicated computed jump and whose next words are code
   addresses. `slots8` is `addls pc,pc,r0,lsl #3` (8-byte slots), `unscaled`
   an `add pc,pc,r0` behind a `bhi` guard. `oddword` is an `ldrls` table whose
   third word has bit 0 set (an interworking target).

       arm-linux-musleabihf-gcc -c -o a32tablefp_arm_le32.o a32tablefp_arm_le32.S
       arm-linux-musleabihf-gcc -nostdlib -static -no-pie -Wl,-Ttext=0xff000 \
           -Wl,--build-id=none -o a32tablefp_arm_le32 a32tablefp_arm_le32.o

   (GCC 14.2, homebrew `musl-cross` 0.9.11; the two steps keep the object's
   name out of the symbol table.)

   SHA-256 1443ee449bd6b23570d491d707ebe1692ab8b8c68c3b20e0b7f6f0fa903a1ac1. */

        .syntax unified
        .arch armv7-a
        .text
        .arm
        .global _start
_start: bl      guarded
        bl      slots8
        bl      unscaled
        bl      oddword
1:      b       1b

        .global guarded
        .type guarded, %function
guarded:
        push    {r4, lr}
        ldr     r4, .Lgot
        bl      __stack_chk_fail
.Lgot:  .word   0x000ff128
        .word   0x000ff1a0
        .size guarded, . - guarded

        .global slots8
        .type slots8, %function
slots8:
        cmp     r0, #3
        addls   pc, pc, r0, lsl #3
        b       9f
        mov     r0, #10
        bx      lr
        mov     r0, #11
        bx      lr
        mov     r0, #12
        bx      lr
        mov     r0, #13
        bx      lr
9:      mov     r0, #0
        bx      lr
        .size slots8, . - slots8

        .global unscaled
        .type unscaled, %function
unscaled:
        cmp     r0, #8
        bhi     9f
        add     pc, pc, r0
        nop
        mov     r1, #1
        mov     r1, #2
9:      mov     r0, r1
        bx      lr
        .size unscaled, . - unscaled

        .global oddword
        .type oddword, %function
oddword:
        cmp     r0, #2
        ldrls   pc, [pc, r0, lsl #2]
        b       9f
        .word   1f, 2f, 3f + 1
1:      mov     r0, #20
        bx      lr
2:      mov     r0, #21
        bx      lr
3:      mov     r0, #22
        bx      lr
9:      mov     r0, #0
        bx      lr
        .size oddword, . - oddword

        .global __stack_chk_fail
        .type __stack_chk_fail, %function
__stack_chk_fail:
        b       __stack_chk_fail

        .p2align 8
        .global other
        .type other, %function
other:
        .rept 0x40
        mov     r0, r0
        .endr
        bx      lr
