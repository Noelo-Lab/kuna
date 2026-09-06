	.text
	.globl	dispatch
	.type	dispatch, @function
dispatch:
	cmp	$0x3, %edi
	ja	.Ldefault
	mov	%edi, %eax
	jmp	*jt(,%rax,8)
.Lcase0:
	lea	msg_alpha(%rip), %rdi
	jmp	.Lemit
.Lcase1:
	lea	msg_beta(%rip), %rdi
	jmp	.Lemit
.Lcase2:
	lea	msg_gamma(%rip), %rdi
	jmp	.Lemit
.Lcase3:
	lea	msg_delta(%rip), %rdi
	jmp	.Lemit
.Ldefault:
	lea	msg_other(%rip), %rdi
.Lemit:
	call	emit
	xor	%eax, %eax
	ret
	.size	dispatch, .-dispatch

	.globl	emit
	.type	emit, @function
emit:
	movzbl	(%rdi), %eax
	ret
	.size	emit, .-emit

	.section	.rodata
	.align	8
jt:
	.quad	.Lcase0
	.quad	.Lcase1
	.quad	.Lcase2
	.quad	.Lcase3
msg_alpha:
	.asciz	"switch case alpha reached"
msg_beta:
	.asciz	"switch case beta reached"
msg_gamma:
	.asciz	"switch case gamma reached"
msg_delta:
	.asciz	"switch case delta reached"
msg_other:
	.asciz	"switch default reached"
