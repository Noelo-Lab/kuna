// GH #1376's ELF control: the same GOT load and call in an AArch64 shared
// object (an R_AARCH64_GLOB_DAT slot).
	.text
	.globl	audit_import_call
	.type	audit_import_call, %function
	.p2align	2
audit_import_call:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	adrp	x16, :got:malloc
	ldr	x16, [x16, #:got_lo12:malloc]
	blr	x16
	ldp	x29, x30, [sp], #16
	ret
	.size	audit_import_call, .-audit_import_call
