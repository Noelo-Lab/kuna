	.section	__TEXT,__text,regular,pure_instructions
	.globl	_A_f
	.p2align	2
_A_f:
	mov	w0, #1
	ret

	.globl	_A_g
	.p2align	2
_A_g:
	mov	w0, #2
	ret

	.globl	_B_f
	.p2align	2
_B_f:
	mov	w0, #3
	ret

	.globl	_call_f
	.p2align	2
_call_f:
	ldr	x16, [x0]
	mov	x17, x0
	movk	x17, #0x6ae1, lsl #48
	autda	x16, x17
	ldr	x8, [x16, #0]!
	mov	x17, x16
	movk	x17, #0x0f0f, lsl #48
	braa	x8, x17

	.globl	_call_g
	.p2align	2
_call_g:
	ldr	x16, [x0]
	mov	x17, x0
	movk	x17, #0x6ae1, lsl #48
	autda	x16, x17
	ldr	x8, [x16, #8]!
	mov	x17, x16
	movk	x17, #0x0a0a, lsl #48
	braa	x8, x17

	.globl	_main
	.p2align	2
_main:
	b	_call_g

	.section	__DATA_CONST,__const
	.p2align	3
_vtable_A:
	.quad	0
	.quad	0
	.quad	_A_f
	.quad	_A_g
_vtable_B:
	.quad	0
	.quad	0
	.quad	_B_f
	.quad	_A_g
.subsections_via_symbols
