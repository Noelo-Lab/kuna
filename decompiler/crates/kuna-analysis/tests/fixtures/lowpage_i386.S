# GH #1364 on i386: ld.lld puts .rodata in a PIE's first page, so an address
# formed from the GOT pointer (`lea eax,[ebx-...]`) is below 0x1000.
# audit_* form one; control_number moves the same value (audit_words is 0x170)
# as a plain immediate.
	.text
	.globl	audit_string
	.type	audit_string,@function
audit_string:
	pushl	%ebx
	call	__x86.get_pc_thunk.bx
	addl	$_GLOBAL_OFFSET_TABLE_, %ebx
	leal	audit_text@GOTOFF(%ebx), %eax
	popl	%ebx
	ret
	.size	audit_string, .-audit_string

	.globl	audit_read
	.type	audit_read,@function
audit_read:
	pushl	%ebx
	call	__x86.get_pc_thunk.bx
	addl	$_GLOBAL_OFFSET_TABLE_, %ebx
	movl	audit_words@GOTOFF(%ebx), %eax
	popl	%ebx
	ret
	.size	audit_read, .-audit_read

	.globl	control_number
	.type	control_number,@function
control_number:
	movl	$0x170, %eax
	ret
	.size	control_number, .-control_number

	.section	.text.__x86.get_pc_thunk.bx,"axG",@progbits,__x86.get_pc_thunk.bx,comdat
	.globl	__x86.get_pc_thunk.bx
	.hidden	__x86.get_pc_thunk.bx
	.type	__x86.get_pc_thunk.bx,@function
__x86.get_pc_thunk.bx:
	movl	(%esp), %ebx
	ret

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
