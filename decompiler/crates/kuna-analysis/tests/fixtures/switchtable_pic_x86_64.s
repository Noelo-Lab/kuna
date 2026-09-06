# Companion to switchtable_x86_64.s: the SAME switch lowered the way gcc -O2 and
# MSVC actually lower it -- a table of 32-bit displacements added to a base
# register, instead of a table of absolute pointers named by the jump operand.
#
#   lea   jt(%rip), %rdx        <- the table base, on its OWN instruction
#   movslq (%rdx,%rax,4), %rax  <- a signed 32-bit DELTA, not a pointer
#   add   %rdx, %rax
#   jmp   *%rax
#
# Build (README recipe, same as the sibling fixtures):
#   gcc -nostdlib -no-pie -Wl,-Ttext=0x100000 -e dispatch \
#       -o switchtable_pic_x86_64 switchtable_pic_x86_64.s
	.text
	.globl	dispatch
	.type	dispatch, @function
dispatch:
	cmp	$0x3, %edi
	ja	.Ldefault
	mov	%edi, %eax
	lea	jt(%rip), %rdx
	movslq	(%rdx,%rax,4), %rax
	add	%rdx, %rax
	jmp	*%rax
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
	.align	4
jt:
	.long	.Lcase0-jt
	.long	.Lcase1-jt
	.long	.Lcase2-jt
	.long	.Lcase3-jt
msg_alpha:
	.asciz	"switch case alpha reached"
msg_beta:
	.asciz	"switch case beta reached"
msg_gamma:
	.asciz	"switch case gamma reached"
msg_delta:
	.asciz	"switch case delta reached"
msg_other:
	.asciz	"switch default arm reached"
