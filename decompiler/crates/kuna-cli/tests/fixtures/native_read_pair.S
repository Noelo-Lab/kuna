.data
.p2align 3
.globl guard_address
.type guard_address, %object
.size guard_address, 8
guard_address: .xword guard_word
guard_word: .xword 0x13579bdf2468ace0
.text
.p2align 2
.globl probe
.type probe, %function
probe:
 .inst 0xd503233f // paciasp
 sub sp, sp, #48
 mov w0, #0
 adrp x2, guard_address
 ldr x2, [x2, :lo12:guard_address]
 add x1, sp, #8
 stp x29, x30, [sp, #32]
 add x29, sp, #32
 ldr x3, [x2]
 str x3, [sp, #24]
 mov x3, #0
 bl fill
 adrp x1, guard_address
 ldr x1, [x1, :lo12:guard_address]
 ldp x2, x3, [sp, #8]
 ldr x0, [sp, #24]
 ldr x4, [x1]
 subs x0, x0, x4
 mov x4, #0
 b.ne .Lbad
 ldp x29, x30, [sp, #32]
 mov x0, x2
 mov x1, x3
 add sp, sp, #48
 .inst 0xd50323bf // autiasp
 ret
.Lbad:
 bl __stack_chk_fail
.size probe, .-probe
.globl fill
.type fill, %function
fill:
 mov x4, #0x1234
 mov x5, #0x5678
 stp x4, x5, [x1]
 ret
.size fill, .-fill
.globl __stack_chk_fail
.type __stack_chk_fail, %function
__stack_chk_fail:
 brk #0
.size __stack_chk_fail, .-__stack_chk_fail
.globl narrow
.type narrow, %function
narrow:
 ldr x0, [x0]
 ret
.size narrow, .-narrow
#ifdef VERIFY_NATIVE
.globl _start
_start:
 bl probe
 mov x2, #0x1234
 cmp x0, x2
 b.ne .Lwrong
 mov x2, #0x5678
 cmp x1, x2
 b.ne .Lwrong
 mov x0, #0
 b .Lexit
.Lwrong:
 mov x0, #1
.Lexit:
 mov x8, #93
 svc #0
#endif
