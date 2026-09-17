	.text
	.globl	f
	.type	f, @function
f:
	movl	k(%rip), %eax
	addl	%edi, %eax
	ret
	.size	f, .-f
	.globl	main
	.type	main, @function
main:
	subq	$8, %rsp
	movl	$7, %edi
	call	f
	leal	(%rax,%rax,2), %edx
	movl	$42, k(%rip)
	leal	42(%rdx), %eax
	addq	$8, %rsp
	ret
	.size	main, .-main
	.data
	.globl	k
	.align 4
	.type	k, @object
	.size	k, 4
k:
	.long	1
