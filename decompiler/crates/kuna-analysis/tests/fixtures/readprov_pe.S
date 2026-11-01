// GH #1401: .bss merges into .data, whose VirtualSize then runs past its raw
// data; .sealed is linked with /section:.sealed,P, which leaves it no R, W or X.
	.text
	.globl	main
main:
	movl	initialized(%rip), %eax
	addb	scratch+1(%rip), %al
	ret

	.section .sealed,"dr"
	.globl	sealed
sealed:
	.quad	0x0123456789abcdef

	.data
	.globl	initialized
initialized:
	.quad	0x1122334455667788

	.bss
	.globl	scratch
scratch:
	.zero	0x2000
