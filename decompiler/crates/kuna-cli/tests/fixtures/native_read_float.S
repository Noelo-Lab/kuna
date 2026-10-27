.syntax unified
.thumb
.fpu vfpv3-d16
.eabi_attribute 28, 1
.text
.p2align 2
#ifndef RECOVERED_FUNCTIONS
.globl ProbeFloat
.type ProbeFloat, %function
.thumb_func
ProbeFloat:
 sub sp, sp, #8
 str r0, [sp, #4]
 ldr r3, [sp, #4]
 ldrd r2, r3, [r3]
 vmov d7, r2, r3
 vmov.f32 s0, s14
 vmov.f32 s1, s15
 add sp, sp, #8
 bx lr
.size ProbeFloat, .-ProbeFloat
.globl ForwardFloat
.type ForwardFloat, %function
.thumb_func
ForwardFloat:
 push {r4, lr}
 bl ProbeFloat
 pop {r4, pc}
.size ForwardFloat, .-ForwardFloat
.globl ProbeBits
.type ProbeBits, %function
.thumb_func
ProbeBits:
 ldrd r0, r1, [r0]
 bx lr
.size ProbeBits, .-ProbeBits
#endif
#ifdef VERIFY_NATIVE
.globl _start
.type _start, %function
.thumb_func
_start:
 vldr d0, .Lsentinel
 ldr r0, =.Lnumber
 bl ProbeFloat
 vmov r2, r3, d0
 cmp r2, #0
 bne .Lwrong
 ldr r4, =0x401d0000
 cmp r3, r4
 bne .Lwrong
 vadd.f64 d0, d0, d0
 vldr d1, .Lone
 vadd.f64 d0, d0, d1
 vldr d1, .Lexpected
 vcmpe.f64 d0, d1
 vmrs APSR_nzcv, fpscr
 bne .Lwrong
 vldr d0, .Lsentinel
 ldr r0, =.Lnumber
 bl ForwardFloat
 vmov r2, r3, d0
 cmp r2, #0
 bne .Lwrong
 ldr r4, =0x401d0000
 cmp r3, r4
 bne .Lwrong
 ldr r0, =.Lnumber
 bl ProbeBits
 cmp r0, #0
 bne .Lwrong
 ldr r4, =0x401d0000
 cmp r1, r4
 bne .Lwrong
 movs r0, #0
 b .Lexit
.Lwrong:
 movs r0, #1
.Lexit:
 movs r7, #1
 svc #0
.size _start, .-_start
.p2align 3
.Lnumber: .quad 0x401d000000000000
.Lone: .quad 0x3ff0000000000000
.Lexpected: .quad 0x402f000000000000
.Lsentinel: .quad 0x4022345613579bdf
#endif
.section .note.GNU-stack,"",%progbits
