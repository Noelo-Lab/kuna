@ GH #1364 on ARM: a firmware image linked at 0 keeps its code and constants in
@ the first page, so `adr` forms an address below 0x1000, in A32 and in Thumb.
@ audit_* form one; control_numbers builds the same values (audit_text is
@ 0x148, audit_words + 4 is 0x13c) as plain numbers.
	.syntax unified
	.text
	.arm
	.globl	audit_a32
	.type	audit_a32,%function
audit_a32:
	adr	r0, audit_text
	bx	lr
	.size	audit_a32, .-audit_a32

	.thumb
	.globl	audit_thumb
	.type	audit_thumb,%function
	.p2align	2
audit_thumb:
	adr	r0, audit_words
	ldr	r0, [r0, #4]
	bx	lr
	.size	audit_thumb, .-audit_thumb

	.globl	audit_thumb_wide
	.type	audit_thumb_wide,%function
	.p2align	2
audit_thumb_wide:
	adr.w	r0, audit_text
	bx	lr
	.size	audit_thumb_wide, .-audit_thumb_wide

	.globl	control_numbers
	.type	control_numbers,%function
control_numbers:
	movw	r0, #0x148
	addw	r1, r1, #0x13c
	bx	lr
	.size	control_numbers, .-control_numbers

	.arm
	.p2align	2
	.globl	main
	.type	main,%function
main:
	push	{r4, lr}
	bl	audit_a32
	blx	audit_thumb
	blx	audit_thumb_wide
	blx	control_numbers
	pop	{r4, pc}
	.size	main, .-main

	.p2align	2
	.type	audit_words,%object
audit_words:
	.word	1, 2, 3, 4
	.size	audit_words, 16
	.type	audit_text,%object
audit_text:
	.asciz	"first page"
	.size	audit_text, 11
