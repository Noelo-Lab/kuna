# GH-657 fixture: `foldcallret` must not sink a call past a write to a global
# the callee reads.  helper() returns x + k; target writes k between the call
# and its use, target2 writes it between the use and the statement the folded
# expression would be printed in.  Hand-written so the two shapes survive any
# compiler's scheduling.
	.text
	.globl	target
	.type	target, @function
target:
	movl	$7, %edi
	call	helper
	movl	$42, k(%rip)
	addl	$42, %eax
	ret
	.size	target, .-target
	.globl	target2
	.type	target2, @function
target2:
	movl	$7, %edi
	call	helper
	leal	(%rax,%rax,2), %edx
	movl	$42, k(%rip)
	leal	42(%rdx), %eax
	ret
	.size	target2, .-target2
	.globl	helper
	.type	helper, @function
helper:
	movl	k(%rip), %eax
	addl	%edi, %eax
	ret
	.size	helper, .-helper
	.data
	.globl	k
	.align	4
	.type	k, @object
	.size	k, 4
k:
	.long	1
