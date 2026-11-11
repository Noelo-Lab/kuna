@ GH #1360 on A32 and Thumb-2: `adr r0, .+4` and `adr.w r0, .+4` form the
@ address of the next instruction. The links are controls: `bl`, `blx`, a `bl`
@ to the next instruction, and `mov lr, pc` (SLEIGH lifts `lr = inst_next + 4`
@ for the `bx` after it).
	.syntax unified
	.arm
	.text
	.globl	a_next
	.type	a_next,%function
a_next:
	adr	r0, 1f
1:	bx	lr
	.globl	a_self
	.type	a_self,%function
a_self:
	adr	r0, a_self
	bx	lr
	.globl	a_calls
	.type	a_calls,%function
a_calls:
	push	{r4, lr}
	bl	a_next
	adr	r4, a_self
	blx	r4
	mov	lr, pc
	bx	r4
	bl	1f
1:	pop	{r4, pc}
	.thumb
	.p2align 2
	.globl	t_next
	.type	t_next,%function
t_next:
	nop
	adr.w	r0, 1f
1:	bx	lr
	.globl	t_calls
	.type	t_calls,%function
t_calls:
	push	{r4, lr}
	bl	t_next
	blx	r4
	pop	{r4, pc}
	.arm
	.p2align 2
	.globl	main
	.type	main,%function
main:
	push	{r4, lr}
	bl	a_calls
	blx	t_calls
	pop	{r4, pc}
