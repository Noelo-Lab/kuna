	.text
	.globl	dispatch
	.type	dispatch, @function
dispatch:
	movl	4(%esp), %eax
	cmpl	$0x3, %eax
	ja	.Ldefault
	jmp	*jt(,%eax,4)
.Lcase0:
	pushl	$msg_alpha
	jmp	.Lemit
.Lcase1:
	pushl	$msg_beta
	jmp	.Lemit
.Lcase2:
	pushl	$msg_gamma
	jmp	.Lemit
.Lcase3:
	pushl	$msg_delta
	jmp	.Lemit
.Ldefault:
	pushl	$msg_other
.Lemit:
	call	emit
	addl	$4, %esp
	ret
	.size	dispatch, .-dispatch

	.globl	emit
	.type	emit, @function
emit:
	movl	4(%esp), %eax
	movzbl	(%eax), %eax
	ret
	.size	emit, .-emit

	.section	.rodata
	.align	4
jt:
	.long	.Lcase0
	.long	.Lcase1
	.long	.Lcase2
	.long	.Lcase3
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
