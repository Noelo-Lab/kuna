# Reduction of the crackmes.one `no-standards` witness (6736b3a09b533b4c22bd2b9f):
# a prompt whose first characters are multi-byte UTF-8, loaded by the routine that
# prints it.  Read at 1-byte width the run starts after the LAST sequence, so the
# inventory reports an address nothing in the image refers to.
#
#   gcc -nostdlib -no-pie -Wl,-Ttext=0x100000 -e prompt_user \
#       -o utf8prompt_x86_64 utf8prompt_x86_64.s
#
# `prompt` is `_φ( °-°)/ so what was the magical keycombination? ` — U+FF3F
# FULLWIDTH LOW LINE, U+03C6 GREEK SMALL LETTER PHI and two U+00B0 DEGREE SIGNs,
# then plain ASCII.  `plain` is the control: pure ASCII, so both readings of the
# 1-byte width must report it identically.
	.text
	.globl	prompt_user
	.type	prompt_user, @function
prompt_user:
	lea	prompt(%rip), %rsi
	mov	$1, %edi
	mov	$55, %edx
	mov	$1, %eax
	syscall
	lea	plain(%rip), %rsi
	mov	$1, %edi
	mov	$27, %edx
	mov	$1, %eax
	syscall
	mov	$60, %eax
	xor	%edi, %edi
	syscall
	.size	prompt_user, .-prompt_user

	.section .rodata
prompt:
	.string	"\357\274\277\317\206( \302\260-\302\260)/ so what was the magical keycombination? "
plain:
	.string	"plain ascii control literal"
