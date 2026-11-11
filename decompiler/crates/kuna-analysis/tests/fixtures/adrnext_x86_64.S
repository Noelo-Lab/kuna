# GH #1360 on x86-64: `lea rax,[rip]` and `mov eax,offset next` form the
# address of the next instruction. Every link is a control: `call`, `call *`,
# `call $+5` (which SLEIGH lifts as a push and a jump) and `syscall` (which
# leaves the return address in rcx).
	.text
	.globl	lea_next
	.type	lea_next,@function
lea_next:
	leaq	1f(%rip), %rax
1:	ret
	.globl	lea_self
	.type	lea_self,@function
lea_self:
	leaq	lea_self(%rip), %rax
	ret
	.globl	mov_next
	.type	mov_next,@function
mov_next:
	movl	$1f, %eax
1:	ret
	.globl	calls
	.type	calls,@function
calls:
	pushq	%rbx
	call	lea_next
	leaq	lea_self(%rip), %rbx
	call	*%rbx
	call	1f
1:	popq	%rax
	syscall
	popq	%rbx
	ret
	.globl	main
	.type	main,@function
main:
	call	calls
	call	mov_next
	jmp	lea_self
