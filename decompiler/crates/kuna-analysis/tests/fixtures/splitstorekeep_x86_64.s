	.text
	.globl	copy31
	.type	copy31, @function
# Reduction of the 0xJam3z-Medium sub_15dc witness: a 31-byte stack-to-stack
# copy done as four 8-byte moves at offsets 0, 8, 15 and 23, so the middle two
# overlap on byte 15.  Both buffers escape to `sink` so nothing here is dead.
copy31:
	sub	$0x58, %rsp
	mov	%rsp, %rdi
	call	sink
	mov	0x0(%rsp), %rax
	mov	0x8(%rsp), %rdx
	mov	%rax, 0x20(%rsp)
	mov	%rdx, 0x28(%rsp)
	mov	0xf(%rsp), %rax
	mov	0x17(%rsp), %rdx
	mov	%rax, 0x2f(%rsp)
	mov	%rdx, 0x37(%rsp)
	lea	0x20(%rsp), %rdi
	call	sink
	add	$0x58, %rsp
	ret
	.size	copy31, .-copy31

	.globl	copy32
	.type	copy32, @function
# The control: the same shape at a size that is a multiple of the word, so the
# four moves tile [0,32) and no two of them overlap.  Refinement never fires and
# the emitted C must be identical with the option on and off.
copy32:
	sub	$0x58, %rsp
	mov	%rsp, %rdi
	call	sink
	mov	0x0(%rsp), %rax
	mov	0x8(%rsp), %rdx
	mov	%rax, 0x20(%rsp)
	mov	%rdx, 0x28(%rsp)
	mov	0x10(%rsp), %rax
	mov	0x18(%rsp), %rdx
	mov	%rax, 0x30(%rsp)
	mov	%rdx, 0x38(%rsp)
	lea	0x20(%rsp), %rdi
	call	sink
	add	$0x58, %rsp
	ret
	.size	copy32, .-copy32

	.globl	sink
	.type	sink, @function
sink:
	ret
	.size	sink, .-sink
