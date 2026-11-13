	.text
	.globl	___chkstk_darwin
	.type	___chkstk_darwin, %function
	.p2align	2
___chkstk_darwin:
	mov	x0, #7
	ret
	.size	___chkstk_darwin, .-___chkstk_darwin

	.globl	caller
	.type	caller, %function
	.p2align	2
caller:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	mov	x9, #0x2000
	bl	___chkstk_darwin
	add	x0, x0, #1
	ldp	x29, x30, [sp], #16
	ret
	.size	caller, .-caller
