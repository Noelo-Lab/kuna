# GH-657, the shape the call-to-use span does not cover: the write to `k` sits
# between the call's single use (the `lea`, which is implied) and the `ret` the
# folded expression would be printed in.  Hand-written because gcc schedules the
# global store before the arithmetic in every C form of it.
#
#   gcc -O2 -no-pie -o /tmp/travel travel.s && /tmp/travel; echo $?   # 66
#   kuna decompile /tmp/travel main
#
# Before: `k = 0x2a; return f(7) * 3 + 0x2a;`  (66 in the binary, 189 in that C)
# After:  `v1 = f(7); k = 0x2a; return v1 * 3 + 0x2a;`
	.text
	.globl	f
	.type	f, @function
f:
	movl	k(%rip), %eax
	addl	%edi, %eax
	ret
	.size	f, .-f
	.globl	main
	.type	main, @function
main:
	subq	$8, %rsp
	movl	$7, %edi
	call	f
	leal	(%rax,%rax,2), %edx
	movl	$42, k(%rip)
	leal	42(%rdx), %eax
	addq	$8, %rsp
	ret
	.size	main, .-main
	.data
	.globl	k
	.align 4
	.type	k, @object
	.size	k, 4
k:
	.long	1
