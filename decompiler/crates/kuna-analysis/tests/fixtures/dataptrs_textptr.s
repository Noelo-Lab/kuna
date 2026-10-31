# i386 PE: relocated words in .text that are instruction operands, and a
# pointer table in .text right after an instruction naming the same function.
	.text
	.globl	_start
_start:
	pushl	$_helper
	call	_dispatch
	call	_helper
	addl	$4, %esp
	ret
	.globl	_dispatch
_dispatch:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ldefault
	jmp	*.Lcases(,%eax,4)
.Lcases:
	.long	.Lcase0
	.long	.Lcase1
	.long	.Lcase2
	.long	.Lcase3
.Lcase0:	movl	$10, %eax
	ret
.Lcase1:	movl	$11, %eax
	ret
.Lcase2:	movl	$12, %eax
	ret
.Lcase3:	movl	$13, %eax
	ret
.Ldefault:	xorl	%eax, %eax
	ret
	.globl	_helper
_helper:
	movl	$_dispatch, %eax
	ret
	.p2align 2
	.globl	_text_table
_text_table:
	.long	_helper
	.long	_dispatch
	.data
	.globl	_data_table
_data_table:
	.long	_helper
	.long	_dispatch
