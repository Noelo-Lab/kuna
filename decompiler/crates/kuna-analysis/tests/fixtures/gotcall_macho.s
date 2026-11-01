// GH #1376's own assembly: an arm64 dylib calls _malloc through its GOT slot,
// which a chained-fixup bind fills.
	.text
	.space	0x1000
	.globl	_audit_import_call
	.p2align	2
_audit_import_call:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	adrp	x16, _malloc@GOTPAGE
	ldr	x16, [x16, _malloc@GOTPAGEOFF]
	blr	x16
	ldp	x29, x30, [sp], #16
	ret
