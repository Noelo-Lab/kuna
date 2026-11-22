	.text
	.globl	kuna_pick
	.type	kuna_pick,@function
	.p2align	4
kuna_pick:
	testl	%edi, %edi
	jne	.Lshared
	xorl	%eax, %eax
	retq
	.size	kuna_pick, .-kuna_pick

	.globl	kuna_alloc
	.type	kuna_alloc,@function
	.p2align	4
kuna_alloc:
	pushq	%rbx
.Lshared:
	movl	$64, %edi
	callq	swift_task_alloc@PLT
	movq	%rax, 0x10(%r14)
	popq	%rbx
	retq
	.size	kuna_alloc, .-kuna_alloc
	.section	.note.GNU-stack,"",@progbits
