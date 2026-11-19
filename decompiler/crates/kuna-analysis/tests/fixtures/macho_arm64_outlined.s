	.section	__TEXT,__text,regular,pure_instructions
	.globl	_probe
	.p2align	2
_probe:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	mov	x8, x0
	lsl	x9, x0, #4
	bl	___chkstk_darwin
	sub	sp, sp, x9
	add	w0, w8, #1
	mov	sp, x29
	ldp	x29, x30, [sp], #16
	ret

	.globl	_OUTLINED_FUNCTION_0
	.p2align	2
_OUTLINED_FUNCTION_0:
	add	w0, w0, w1
	lsl	w0, w0, #1
	ret

	.globl	_OUTLINED_FUNCTION_1
	.p2align	2
_OUTLINED_FUNCTION_1:
	mov	w0, #7
	b	_exit

	.globl	_sum2
	.p2align	2
_sum2:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_OUTLINED_FUNCTION_0
	add	w0, w0, #3
	ldp	x29, x30, [sp], #16
	ret

	.globl	_bail
	.p2align	2
_bail:
	mov	w1, #2
	b	_OUTLINED_FUNCTION_1

	.globl	_main
	.p2align	2
_main:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_probe
	bl	_sum2
	bl	_bail
	ldp	x29, x30, [sp], #16
	ret
.subsections_via_symbols
