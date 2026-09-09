# Reduction of the crackmes.one 652e6f896e7e520ff1c3db3c `kernel_call` witness:
# a three-argument x86-64 Linux syscall wrapper whose number is a RUNTIME
# parameter, so no name/arity lookup can reach it.  Byte-for-byte the shape gcc
# -O0 emits, which is what makes the defect visible: the setup registers have no
# reader (`syscall()` prints with empty parens) and nothing redefines RAX across
# the instruction (`return number;` is copy propagation of the incoming number).
#
# `plain_add` is the control: no SYSCALL anywhere, so it must be identical in
# both passes.
#
# Built: gcc -nostdlib -no-pie -Wl,--build-id=none -Wl,-Ttext=0x100000 \
#            -e kernel_call -o x64syscall_x86_64 x64syscall_x86_64.s
	.text
	.globl	kernel_call
	.type	kernel_call, @function
kernel_call:
	push	%rbp
	mov	%rsp, %rbp
	mov	%rdi, -0x18(%rbp)
	mov	%rsi, -0x20(%rbp)
	mov	%rdx, -0x28(%rbp)
	mov	-0x18(%rbp), %rax
	mov	-0x20(%rbp), %rdx
	mov	-0x28(%rbp), %rsi
	mov	%rdx, %rdi
	syscall
	mov	%rax, -0x8(%rbp)
	mov	-0x8(%rbp), %rax
	pop	%rbp
	ret
	.size	kernel_call, .-kernel_call

	.globl	plain_add
	.type	plain_add, @function
plain_add:
	push	%rbp
	mov	%rsp, %rbp
	mov	%rdi, -0x18(%rbp)
	mov	%rsi, -0x20(%rbp)
	mov	-0x18(%rbp), %rax
	mov	-0x20(%rbp), %rdx
	add	%rdx, %rax
	mov	%rax, -0x8(%rbp)
	mov	-0x8(%rbp), %rax
	pop	%rbp
	ret
	.size	plain_add, .-plain_add
