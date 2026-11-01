// GH #1364: in a zero-based arm64 dylib the first page holds code and
// constants, so an address formed from the PC can be below 0x1000. Each
// _audit_* function forms one; _control_numbers builds the same values as
// plain numbers (_audit_target is 0x378, _audit_word 0x3d0), which are not
// references.
	.text
	.globl	_audit_target
	.p2align	2
_audit_target:
	mov	w0, #7
	ret

	.globl	_audit_direct
	.p2align	2
_audit_direct:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_audit_target
	ldp	x29, x30, [sp], #16
	ret

	.globl	_audit_address
	.p2align	2
_audit_address:
	adrp	x0, _audit_target@PAGE
	add	x0, x0, _audit_target@PAGEOFF
	ret

	.globl	_audit_adr
	.p2align	2
_audit_adr:
	adr	x0, _audit_target
	ret

	.globl	_audit_string
	.p2align	2
_audit_string:
	adrp	x0, l_.str@PAGE
	add	x0, x0, l_.str@PAGEOFF
	ret

	.globl	_audit_read
	.p2align	2
_audit_read:
	adrp	x8, _audit_word@PAGE
	ldr	w0, [x8, _audit_word@PAGEOFF]
	ret

	.globl	_control_numbers
	.p2align	2
_control_numbers:
	mov	x0, #0x378
	add	x1, x1, #0x378
	mov	x2, #0x3d0
	ret

	.section	__TEXT,__const
	.globl	_audit_word
	.p2align	2
_audit_word:
	.long	42

	.section	__TEXT,__cstring,cstring_literals
l_.str:
	.asciz	"first page"

	.data
	.globl	_audit_pointer
	.p2align	3
_audit_pointer:
	.quad	_audit_target
