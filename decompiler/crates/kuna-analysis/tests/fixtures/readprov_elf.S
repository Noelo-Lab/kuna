// GH #1401: a .bss past the RW segment's file content, and a PT_LOAD the image
// declares with no permissions at all (FLAGS(0) in readprov_elf.ld).
	.text
	.globl	_start
_start:
	movl	initialized(%rip), %eax
	ret

	.section .noaccess,"a",@progbits
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
	.zero	64
