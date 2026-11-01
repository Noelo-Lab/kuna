# GH #1364 on RV64: ld.lld puts .rodata in a shared object's first page, so an
# auipc pair forms an address below 0x1000. audit_* form one; control_number
# builds the same value (audit_words is 0x2a0) as a plain number.
	.text
	.globl	audit_string
	.type	audit_string,@function
audit_string:
	lla	a0, audit_text
	ret
	.size	audit_string, .-audit_string

	.globl	audit_read
	.type	audit_read,@function
audit_read:
1:	auipc	a0, %pcrel_hi(audit_words)
	lw	a0, %pcrel_lo(1b)(a0)
	ret
	.size	audit_read, .-audit_read

	.globl	control_number
	.type	control_number,@function
control_number:
	li	a0, 0x2a0
	ret
	.size	control_number, .-control_number

	.section	.rodata
	.p2align	2
	.type	audit_words,@object
audit_words:
	.word	1, 2, 3, 4
	.size	audit_words, 16
	.type	audit_text,@object
audit_text:
	.asciz	"first page"
	.size	audit_text, 11
