	.text
	.globl	vm
	.type	vm, @function
vm:
	sub	$0x38, %rsp
	pxor	%xmm0, %xmm0
	movaps	%xmm0, (%rsp)
	movaps	%xmm0, 0x10(%rsp)
	mov	%rsp, %rdi
	call	sink
	lea	0x10(%rsp), %rdi
	call	sink
	movdqa	(%rsp), %xmm1
	movdqa	0x10(%rsp), %xmm2
	movaps	%xmm2, (%rsp)
	movaps	%xmm1, 0x10(%rsp)
	mov	%rsp, %rdi
	call	sink
	movzbl	0x3(%rsp), %eax
	add	$0x38, %rsp
	ret
	.size	vm, .-vm
	.globl	sink
	.type	sink, @function
sink:
	ret
	.size	sink, .-sink
