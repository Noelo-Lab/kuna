/* ARM conditions read after vcmp and vmrs (tests/stages/kuna-fcmp-unordered.xml).
   clang --target=armv7a-linux-gnueabihf -mfloat-abi=hard -mfpu=vfpv3-d16 -c fcmp_unordered_a32.s -o fcmp_unordered_a32.o */
.syntax unified
.arm
.fpu vfpv3-d16
.eabi_attribute Tag_ABI_VFP_args, 1
.text
.p2align 2
.macro BR name, cond
.global \name
.type \name, %function
\name:
    vcmp.f64 d0, d1
    vmrs APSR_nzcv, fpscr
    b\cond 1f
    mov r0, #3
    bx lr
1:  mov r0, #5
    bx lr
.size \name, .-\name
.endm
BR a32_gt, gt
BR a32_le, le
BR a32_ge, ge
BR a32_lt, lt
BR a32_pl, pl
BR a32_mi, mi
