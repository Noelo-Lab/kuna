# Fixture for the member-Cover reallocation fix (RE-friction need
# finite-field-squaring-loop).  `gfinv` is the GF(2^8) multiplicative inverse
# x^254, written the way an optimizing compiler emits it: an outer loop over the
# bits of the exponent 0xfe with two inlined 8-round carry-less multiplies.
#
# The shape that matters is in the second multiply (`.Lbody2`).  Three byte
# registers are live at once -- R9B accumulates, R11B walks the bits of CL, and
# CL itself doubles each round -- and on exit the accumulator is copied back into
# CL, which is also R11B's initialiser.  That closes a copy cycle
# R9B -> CL -> R11B across the outer back edge, and the merge trims kuna inserts
# to serve it re-point an op output whose Varnode is already a member of a
# settled HighVariable.
#
# Build:
#   gcc -nostdlib -static -Wl,-e,_start -o covertrim_x86_64 covertrim_x86_64.s

	.text
	.globl	gfinv
	.type	gfinv, @function
gfinv:
	subq	$8, %rsp
	movzbl	%dil, %ecx
	testb	%cl, %cl
	jne	.L1
	xorb	%al, %al
	addq	$8, %rsp
	ret
.L1:
	movq	%rbx, (%rsp)
	movb	$1, %r10b
	movb	$-2, %bl
.Louter:
	testb	$1, %bl
	je	.Lloop2
	xorb	%r9b, %r9b
	movzbl	%cl, %r11d
	testb	%cl, %cl
	je	.Lafter1
	.p2align 4
.Lbody1:
	movzbl	%r10b, %eax
	xorb	%r9b, %al
	movzbl	%al, %edx
	testb	$1, %r11b
	movzbl	%r9b, %eax
	cmove	%eax, %edx
	movzbl	%r10b, %eax
	addb	%al, %al
	movzbl	%dl, %r9d
	movzbl	%al, %r8d
	movzbl	%r10b, %edx
	movzbl	%r8b, %eax
	xorb	$45, %al
	testb	%dl, %dl
	movzbl	%al, %r10d
	cmovns	%r8d, %r10d
	shrb	$1, %r11b
	jne	.Lbody1
.Lafter1:
	movzbl	%r9b, %r10d
.Lloop2:
	xorb	%r9b, %r9b
	movzbl	%cl, %r11d
	testb	%cl, %cl
	je	.Lafter2
	.p2align 4
.Lbody2:
	movzbl	%cl, %eax
	xorb	%r9b, %al
	movzbl	%al, %edx
	testb	$1, %r11b
	movzbl	%r9b, %eax
	cmove	%eax, %edx
	movzbl	%cl, %eax
	addb	%al, %al
	movzbl	%dl, %r9d
	movzbl	%al, %r8d
	movzbl	%cl, %edx
	movzbl	%r8b, %eax
	xorb	$45, %al
	testb	%dl, %dl
	movzbl	%al, %ecx
	cmovns	%r8d, %ecx
	shrb	$1, %r11b
	jne	.Lbody2
.Lafter2:
	shrb	$1, %bl
	movzbl	%r9b, %ecx
	jne	.Louter
	movq	(%rsp), %rbx
	movzbl	%r10b, %eax
	addq	$8, %rsp
	ret
	.size	gfinv, .-gfinv

	.globl	_start
	.type	_start, @function
_start:
	movl	$3, %edi
	call	gfinv
	movzbl	%al, %edi
	movl	$60, %eax
	syscall
	.size	_start, .-_start
