// kuna `thunknames`: one-instruction `b <target>` thunks with no symbol of their
// own. `.Lthunk` jumps to the named `release_box`, `.Lthunk2` jumps to
// `.Lthunk`, and `drop_box` / `drop_two` call through them.
	.text
	.p2align 2
	.globl	release_box
	.type	release_box,%function
release_box:
	ldr	x8, [x0]
	sub	x8, x8, #1
	str	x8, [x0]
	ret
	.size	release_box, .-release_box

.Lthunk:
	b	release_box

.Lthunk2:
	b	.Lthunk

	.globl	drop_box
	.type	drop_box,%function
drop_box:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	.Lthunk
	ldp	x29, x30, [sp], #16
	ret
	.size	drop_box, .-drop_box

	.globl	drop_two
	.type	drop_two,%function
drop_two:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	.Lthunk2
	ldp	x29, x30, [sp], #16
	ret
	.size	drop_two, .-drop_two
