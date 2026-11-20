	.section	__TEXT,__text,regular,pure_instructions
	.globl	_big_e
	.p2align	2
_big_e:
	pacibsp
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	mov	x9, #0x5000
	adrp	x16, Lslot_chk@PAGE
	ldr	x16, [x16, Lslot_chk@PAGEOFF]
	blraaz	x16
	sub	sp, sp, #5, lsl #12
	add	w0, w0, #3
	mov	sp, x29
	ldp	x29, x30, [sp], #16
	retab

	.globl	_vla_e
	.p2align	2
_vla_e:
	pacibsp
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	mov	x9, x1
	adrp	x17, Lslot_chk@PAGE
	add	x17, x17, Lslot_chk@PAGEOFF
	ldr	x16, [x17]
	blraa	x16, x17
	add	x0, x0, x1
	mov	sp, x29
	ldp	x29, x30, [sp], #16
	retab

	.globl	_main
	.p2align	2
_main:
	mov	w0, #0
	ret

	.section	__DATA_CONST,__auth_ptr
	.p2align	3
Lslot_chk:
	.quad	___chkstk_darwin
.subsections_via_symbols
