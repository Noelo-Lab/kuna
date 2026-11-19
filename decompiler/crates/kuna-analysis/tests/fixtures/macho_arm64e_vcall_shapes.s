	.section	__TEXT,__text,regular,pure_instructions
	.globl	_C_m
	.p2align	2
_C_m:
	mov	w0, #1
	ret

	.globl	_filler
	.p2align	2
_filler:
	mov	w0, #0
	ret

	.globl	_S_m1
	.p2align	2
_S_m1:
	mov	w0, #2
	ret

	.globl	_S_m2
	.p2align	2
_S_m2:
	mov	w0, #3
	ret

	.globl	_S_m3
	.p2align	2
_S_m3:
	stp	x29, x30, [sp, #-16]!
	bl	_filler
	ldp	x29, x30, [sp], #16
	ret

	.globl	_G_f
	.p2align	2
_G_f:
	mov	w0, #4
	ret

	.globl	_W_f
	.p2align	2
_W_f:
	mov	w0, #5
	ret

	.globl	_H_f
	.p2align	2
_H_f:
	mov	w0, #6
	ret

	.globl	_call_big
	.p2align	2
_call_big:
	ldr	x16, [x0]
	mov	x17, x0
	movk	x17, #0x6ae1, lsl #48
	autda	x16, x17
	mov	x17, #256
	add	x16, x16, x17
	ldr	x8, [x16]
	mov	x9, x16
	mov	x17, x9
	movk	x17, #0x3232, lsl #48
	braa	x8, x17

	.globl	_call_swift
	.p2align	2
_call_swift:
	ldr	x16, [x20]
	mov	x17, x20
	movk	x17, #0x6ae1, lsl #48
	autda	x16, x17
	ldr	x8, [x16, #0x58]!
	mov	x17, x16
	movk	x17, #0x5252, lsl #48
	braa	x8, x17

	.globl	_call_swift_m3
	.p2align	2
_call_swift_m3:
	stp	x29, x30, [sp, #-16]!
	ldr	x16, [x20]
	mov	x17, x20
	movk	x17, #0x6ae1, lsl #48
	autda	x16, x17
	ldr	x8, [x16, #0x60]!
	mov	x17, x16
	movk	x17, #0x5353, lsl #48
	blraa	x8, x17
	ldp	x29, x30, [sp], #16
	ret

	.globl	_call_static
	.p2align	2
_call_static:
	adrp	x8, _gslot@PAGE
	add	x8, x8, _gslot@PAGEOFF
	ldr	x9, [x8]
	mov	x17, x8
	movk	x17, #0x6060, lsl #48
	braa	x9, x17

	.globl	_call_writable
	.p2align	2
_call_writable:
	adrp	x8, _wslot@PAGE
	add	x8, x8, _wslot@PAGEOFF
	ldr	x9, [x8]
	mov	x17, x8
	movk	x17, #0x6161, lsl #48
	braa	x9, x17

	.globl	_call_const
	.p2align	2
_call_const:
	adrp	x8, _cslot@PAGE
	ldr	x9, [x8, _cslot@PAGEOFF]
	mov	x17, #0x7070
	braa	x9, x17

	.globl	_main
	.p2align	2
_main:
	ret

	.section	__TEXT,__const
	.p2align	2
_swift_desc:
	.long	0x50
	.long	0

	.section	__DATA_CONST,__const
	.p2align	3
_vtable_C:
	.quad	0
	.quad	0
	.rept	32
	.quad	_filler
	.endr
	.quad	_C_m
	.quad	0
_gslot:
	.quad	_G_f
	.quad	0
_cslot:
	.quad	_H_f

	.section	__DATA,__data
	.p2align	3
_wslot:
	.quad	_W_f
	.quad	0
_swift_meta_prefix:
	.quad	_filler
	.quad	0
_swift_meta:
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.long	0
	.long	0
	.long	0
	.long	0
	.long	0x78
	.long	0x10
	.quad	_swift_desc
	.quad	0
	.quad	_S_m1
	.quad	_S_m2
	.quad	_S_m3
.subsections_via_symbols
