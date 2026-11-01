// GH #1360: `adr x0, .+4` forms the address of the next instruction, a data
// reference like `adr x0, .` and `adr x0, .+8`. The calls are controls: the
// return address a `bl`/`blr` leaves in x30 (including a `bl` to the next
// instruction) is never one.
	.text
	.p2align 2
	.globl	audit_self
	.type	audit_self,%function
audit_self:
	adr	x0, audit_self
	ret
	.size	audit_self, .-audit_self

	.globl	audit_next
	.type	audit_next,%function
audit_next:
	adr	x0, 1f
1:	ret
	.size	audit_next, .-audit_next

	.globl	audit_after
	.type	audit_after,%function
audit_after:
	adr	x0, 1f
	nop
1:	ret
	.size	audit_after, .-audit_after

	.globl	audit_calls
	.type	audit_calls,%function
audit_calls:
	stp	x29, x30, [sp, #-16]!
	bl	audit_self
	adr	x1, audit_after
	blr	x1
	bl	1f
1:	ldp	x29, x30, [sp], #16
	ret
	.size	audit_calls, .-audit_calls

	.globl	main
	.type	main,%function
main:
	stp	x29, x30, [sp, #-16]!
	bl	audit_next
	bl	audit_calls
	ldp	x29, x30, [sp], #16
	b	audit_self
	.size	main, .-main
