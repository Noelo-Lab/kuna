	.text
	.globl	ftol_conv
	.type	ftol_conv, @function
ftol_conv:
	pushl	%esi
	movl	8(%esp), %esi
	flds	24(%esi)
	call	__ftol
	movl	%eax, (%esi)
	flds	28(%esi)
	call	__ftol
	movl	%eax, 4(%esi)
	popl	%esi
	ret
	.size	ftol_conv, .-ftol_conv
