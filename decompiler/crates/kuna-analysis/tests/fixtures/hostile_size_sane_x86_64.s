// Fixture for the GH-339 symbol-extent clamp: a data symbol whose declared
// `st_size` is hostile to the 32-bit type-factory domain the extent is cast into.
// The ELF spec makes `st_size` a 64-bit field and no header check validates it,
// so these are ordinary attacker-controlled bytes, not corrupt files.
//
// This one declares `st_size = 8` -- an ordinary, representable extent: the control.
//
// The size is written by the assembler from the source below; nothing is
// byte-patched after the link, so the fixture rebuilds reproducibly:
//
//   gcc -no-pie -nostdlib -e main -o hostile_size_sane_x86_64 hostile_size_sane_x86_64.s
	.text
	.globl	main
	.type	main, @function
main:
	movl	g_a(%rip), %eax
	ret
	.size	main, .-main

	.data
	.globl	g_a
	.type	g_a, @object
g_a:
	.quad	0
	.size	g_a, 8
