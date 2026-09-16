# (kuna) `elfmain` fixture -- a stripped x86-64 PIE whose `main` reads argument
# registers PAST the third, so the locked three-parameter prototype has
# something to take away.
#
# `_start` is the glibc crt1 shape oracle 4 decodes (`lea main(%rip),%rdi`
# followed by `call *__libc_start_main@GOTPCREL`), and `main` is the shape the
# real-world cases have: on the path the decompiler sees, `r8` and `r9` are
# never written, and both are forwarded anyway -- `r8` as the fourth register
# argument of a call, `r9` into a global.  Reading the body alone therefore
# produces a six-parameter signature whose last two slots are fabricated: the
# C runtime passes three arguments, so `a4`/`a5` are undefined register reads
# that recovery has nowhere else to put.
#
# With `elfmain` on, the three parameters the runtime really passes are
# declared and the two fabrications lose their source: the forwarded `r8`
# argument is dropped from the call, and the stored `r9` becomes a local that
# nothing assigns.  Both shapes are pinned (tests/cli/elf-main-extra-args.json,
# tests/stages/kuna-elfmain.xml) so the cost stays visible.
#
# Built with:
#     gcc -nostartfiles -o elfmainextra_x86_64 elfmainextra_x86_64.s
#     strip elfmainextra_x86_64
	.text
	.globl _start
	.type _start, @function
_start:
	endbr64
	xor	%ebp, %ebp
	mov	%rdx, %r9
	pop	%rsi
	mov	%rsp, %rdx
	and	$-16, %rsp
	push	%rax
	push	%rsp
	xor	%r8d, %r8d
	xor	%ecx, %ecx
	lea	main(%rip), %rdi
	call	*__libc_start_main@GOTPCREL(%rip)
	hlt
	.size _start, .-_start

	.globl main
	.type main, @function
main:
	endbr64
	sub	$8, %rsp
	mov	%r8, %rcx
	mov	%r9, g4(%rip)
	mov	%rsi, %rdx
	mov	%edi, %esi
	lea	tag(%rip), %rdi
	call	record
	add	$8, %rsp
	xor	%eax, %eax
	ret
	.size main, .-main

	.globl record
	.type record, @function
record:
	endbr64
	mov	%rdi, g0(%rip)
	mov	%rsi, g1(%rip)
	mov	%rdx, g2(%rip)
	mov	%rcx, g3(%rip)
	ret
	.size record, .-record

	.section .rodata
tag:	.asciz "kuna elfmain extra-register fixture"

	.bss
	.align 8
g0:	.quad 0
g1:	.quad 0
g2:	.quad 0
g3:	.quad 0
g4:	.quad 0
