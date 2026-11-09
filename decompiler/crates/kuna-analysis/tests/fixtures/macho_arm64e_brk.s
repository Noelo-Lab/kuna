	.section	__TEXT,__text,regular,pure_instructions
	.globl	_check
	.p2align	2
_check:
	cmp	w0, #0x2a
	b.ne	Ltrap1
	mov	w0, #1
	ret
Ltrap1:
	brk	#0xc470

	.globl	_check_x0
	.p2align	2
_check_x0:
	eor	x16, x0, x0, lsl #1
	tbz	x16, #62, Lok2
	brk	#0xc472
Lok2:
	mov	w0, #1
	ret

	.globl	_main
	.p2align	2
_main:
	mov	w0, #0
	ret
.subsections_via_symbols
