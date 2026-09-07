	.text
	.globl	csjmp
	.type	csjmp, @function
csjmp:
	test	%rdi, %rdi
	movabs	$Bt, %r8
	movabs	$At, %r9
	cmovz	%r8, %r9
	jmp	*%r9
	nop
	nop
At:
	mov	%rsi, %rax
	add	$0x1, %rax
	ret
	.balign	16
Bt:
	mov	%rsi, %rax
	sub	$0x1, %rax
	ret
	.size	csjmp, .-csjmp
