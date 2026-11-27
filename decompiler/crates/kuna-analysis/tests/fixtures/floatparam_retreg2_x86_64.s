# A callee recovered returning in eax while its caller reads xmm0: `callee`
# leaves a value in both eax and xmm0, so its recovery returns eax, and
# `caller` converts the xmm0 the call hands back. The return a decompile
# states in another register says nothing of what the call hands back.
#
#   llvm-mc -triple=x86_64-linux-gnu -filetype=obj -o floatparam_retreg2_x86_64.o floatparam_retreg2_x86_64.s
	.text
	.globl	callee
	.type	callee,@function
callee:
	movl	(%rdi), %eax
	movss	(%rsi), %xmm0
	retq
	.size	callee, .-callee
	.globl	caller
	.type	caller,@function
caller:
	subq	$8, %rsp
	callq	callee
	cvtss2sd	%xmm0, %xmm0
	addq	$8, %rsp
	retq
	.size	caller, .-caller
