# GH #1360's controls on PowerPC: `bcl 20,31,1f` (the PIC base idiom) and
# `bl 1f` to the next instruction lift as an LR write and a jump, `bl` and
# `bctrl` as calls. None of them references the return address it leaves in LR.
	.text
	.globl	pic_base
	.type	pic_base,@function
pic_base:
	mflr	0
	bcl	20, 31, 1f
1:	mflr	3
	mtlr	0
	blr
	.globl	bl_next
	.type	bl_next,@function
bl_next:
	mflr	0
	bl	1f
1:	mflr	3
	mtlr	0
	blr
	.globl	calls
	.type	calls,@function
calls:
	mflr	0
	stw	0, 4(1)
	stwu	1, -16(1)
	bl	pic_base
	lis	12, bl_next@ha
	addi	12, 12, bl_next@l
	mtctr	12
	bctrl
	addi	1, 1, 16
	lwz	0, 4(1)
	mtlr	0
	blr
	.globl	main
	.type	main,@function
main:
	b	calls
