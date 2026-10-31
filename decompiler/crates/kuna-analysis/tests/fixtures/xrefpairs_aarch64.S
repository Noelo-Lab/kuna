// kuna xrefs fixture: AArch64 adrp pairs the compiler schedules apart, and the
// shapes that must NOT compose (a join, a call clobbering a scratch register).
	.text
	.p2align 2
	.globl	edge_sched
	.type	edge_sched, %function
edge_sched:                       // adrp ... two unrelated insns ... ldr
	adrp	x8, edge_word
	mov	w1, #5
	add	w0, w0, w1
	ldr	w9, [x8, :lo12:edge_word]
	add	w0, w0, w9
	ret

	.globl	edge_branch
	.type	edge_branch, %function
edge_branch:                      // the pair straddles a conditional branch's fall-through
	adrp	x8, edge_word
	cbz	w0, 1f
	ldr	w0, [x8, :lo12:edge_word]
1:	ret

	.globl	edge_saved
	.type	edge_saved, %function
edge_saved:                       // x19 survives the call (callee-saved)
	stp	x29, x30, [sp, #-32]!
	str	x19, [sp, #16]
	adrp	x19, edge_text
	bl	edge_sched
	add	x0, x19, :lo12:edge_text
	ldr	x19, [sp, #16]
	ldp	x29, x30, [sp], #32
	ret

	.globl	edge_clobbered
	.type	edge_clobbered, %function
edge_clobbered:                   // x8 does not survive the call: no reference
	stp	x29, x30, [sp, #-16]!
	adrp	x8, edge_text
	bl	edge_sched
	add	x0, x8, :lo12:edge_text
	ldp	x29, x30, [sp], #16
	ret

	.globl	edge_join
	.type	edge_join, %function
edge_join:                        // two pages meet at 1: — the add's base is unknown
	adrp	x8, edge_text
	cbz	w0, 1f
	adrp	x8, edge_other
1:	add	x0, x8, #0x10
	ret

	.section .rodata
	.p2align 4
	.globl	edge_pad
	.type	edge_pad, %object
edge_pad:	.asciz	"PAD_SO_EDGE_TEXT_IS_NOT_PAGE_ALIGNED"
	.size	edge_pad, .-edge_pad
	.globl	edge_text
	.type	edge_text, %object
edge_text:	.asciz	"EDGE_TEXT_LABEL"
	.size	edge_text, .-edge_text
	.p2align 12
	.globl	edge_pad2
	.type	edge_pad2, %object
edge_pad2:	.asciz	"PAD_TWO"
	.size	edge_pad2, .-edge_pad2
	.globl	edge_other
	.type	edge_other, %object
edge_other:	.asciz	"EDGE_OTHER_PAGE"
	.size	edge_other, .-edge_other

	.data
	.p2align 2
	.globl	edge_word
	.type	edge_word, %object
edge_word:	.word	42
	.size	edge_word, .-edge_word
