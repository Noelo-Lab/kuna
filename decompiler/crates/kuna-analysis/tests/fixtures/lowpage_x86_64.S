# GH #1364 on x86-64: ld.lld puts .rodata in a shared object's first page, so
# `lea rax,[rip+...]` forms an address below 0x1000. audit_* form one;
# control_number moves the same value (audit_words is 0x2a4) as a plain
# immediate.
	.text
	.globl	audit_string
	.type	audit_string,@function
audit_string:
	leaq	audit_text(%rip), %rax
	ret
	.size	audit_string, .-audit_string

	.globl	audit_table
	.type	audit_table,@function
audit_table:
	leaq	audit_words(%rip), %rcx
	movl	(%rcx,%rdi,4), %eax
	ret
	.size	audit_table, .-audit_table

	.globl	control_number
	.type	control_number,@function
control_number:
	movl	$0x2a4, %eax
	addq	$0x2a4, %rdi
	ret
	.size	control_number, .-control_number

	.section	.rodata
	.p2align	2
	.type	audit_words,@object
audit_words:
	.long	1, 2, 3, 4
	.size	audit_words, 16
	.type	audit_text,@object
audit_text:
	.asciz	"first page"
	.size	audit_text, 11
