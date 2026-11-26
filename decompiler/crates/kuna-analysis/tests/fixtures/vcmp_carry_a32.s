/* ARM carry after vcmp and vmrs for unordered operands (tests/stages/kuna-vcmp-carry.xml).
   clang --target=armv7a-linux-gnueabihf -mfloat-abi=hard -mfpu=vfpv3-d16 -c vcmp_carry_a32.s -o vcmp_carry_a32.o */
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
BR a32_hi, hi
BR a32_ls, ls
BR a32_cs, cs
BR a32_cc, cc
.global a32_cs_zero
.type a32_cs_zero, %function
a32_cs_zero:
    vcmpe.f32 s0, #0
    vmrs APSR_nzcv, fpscr
    bcs 1f
    mov r0, #3
    bx lr
1:  mov r0, #5
    bx lr
.size a32_cs_zero, .-a32_cs_zero
