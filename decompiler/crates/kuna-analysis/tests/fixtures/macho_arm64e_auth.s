	.section	__TEXT,__text,regular,pure_instructions
	.globl	_helper
	.p2align	2
_helper:
	add	w0, w0, #1
	ret

	.globl	_helper2
	.p2align	2
_helper2:
	lsl	w0, w0, #1
	ret

	.globl	_tail_auth
	.p2align	2
_tail_auth:
	pacibsp
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_helper
	ldp	x29, x30, [sp], #16
	autibsp
	eor	x16, x30, x30, lsl #1
	tbz	x16, #62, Lok1
	brk	#0xc471
Lok1:
	b	_helper2

	.globl	_call_auth_ptr
	.p2align	2
_call_auth_ptr:
	pacibsp
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	adrp	x16, _slot_ext@PAGE
	ldr	x16, [x16, _slot_ext@PAGEOFF]
	blraaz	x16
	ldp	x29, x30, [sp], #16
	retab

	.globl	_call_table
	.p2align	2
_call_table:
	pacibsp
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	adrp	x9, _fn_table@PAGE
	add	x9, x9, _fn_table@PAGEOFF
	ldr	x8, [x9]
	blraa	x8, x9
	ldp	x29, x30, [sp], #16
	retab

	.globl	_strip_check
	.p2align	2
_strip_check:
	ldr	x8, [x0]
	autda	x8, x0
	mov	x16, x8
	xpacd	x16
	cmp	x16, x8
	b.eq	Lok2
	brk	#0xc472
Lok2:
	ldr	w0, [x8]
	ret

	.globl	_main
	.p2align	2
_main:
	pacibsp
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_tail_auth
	bl	_call_auth_ptr
	bl	_call_table
	ldp	x29, x30, [sp], #16
	retab

	.section	__DATA_CONST,__auth_ptr
	.p2align	3
_slot_ext:
	.quad	_ext_func

	.section	__DATA_CONST,__const
	.p2align	3
_fn_table:
	.quad	_helper
	.quad	_ext_func
	.quad	_ext_data+16

	.section	__DATA,__data
	.p2align	3
_data_ptr:
	.quad	_helper2
.subsections_via_symbols
