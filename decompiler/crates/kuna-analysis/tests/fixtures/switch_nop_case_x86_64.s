	.file	"switch_nop_case_x86_64.c"
	.text
	.globl	g
	.type	g, @function
g:
.LFB0:
	.cfi_startproc
	endbr64
	movl	%edi, sink(%rip)
	leal	(%rdi,%rdi,2), %eax
	ret
	.cfi_endproc
.LFE0:
	.size	g, .-g
	.globl	pick
	.type	pick, @function
pick:
.LFB1:
	.cfi_startproc
	endbr64
	movl	$270, %eax
	cmpl	$4, %edi
	ja	.L2
	movl	%edi, %edi
	cmpl	$4, map(,%rdi,4)
	ja	.L4
	movl	map(,%rdi,4), %eax
	notrack jmp	*.L6(,%rax,8)
	.section	.rodata
	.align 8
	.align 4
.L6:
	.quad	.L10
	.quad	.L9
	.quad	.L8
	.quad	.L7
	.quad	.L5
	.text
.L10:
	movl	$2070, %edi
	call	g
	xorl	$1, %eax
	ret
.L9:
	movl	$2071, %edi
	call	g
	xorl	$2, %eax
	ret
.L8:
	nop
	movl	$2072, %edi
	call	g
	xorl	$3, %eax
	ret
.L7:
	nop
	movl	$2073, %edi
	call	g
	xorl	$4, %eax
	ret
.L5:
	movl	$2074, %edi
	call	g
	xorl	$5, %eax
	ret
.L4:
	movl	$7, sink(%rip)
	movl	$77, %eax
.L2:
	ret
	.cfi_endproc
.LFE1:
	.size	pick, .-pick
	.section	.rodata
	.align 16
	.type	map, @object
	.size	map, 20
map:
	.long	2
	.long	3
	.long	6
	.long	3
	.long	10
	.globl	sink
	.bss
	.align 4
	.type	sink, @object
	.size	sink, 4
sink:
	.zero	4
	.ident	"GCC: (Ubuntu 11.4.0-1ubuntu1~22.04.3) 11.4.0"
	.section	.note.GNU-stack,"",@progbits
	.section	.note.gnu.property,"a"
	.align 8
	.long	1f - 0f
	.long	4f - 1f
	.long	5
0:
	.string	"GNU"
1:
	.align 8
	.long	0xc0000002
	.long	3f - 2f
2:
	.long	0x3
3:
	.align 8
4:
