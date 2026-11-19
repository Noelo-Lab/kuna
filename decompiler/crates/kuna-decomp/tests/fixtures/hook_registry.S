# x86-64 fixture for tests/hook_registry.rs.
#   unsigned mba_sum(unsigned x, unsigned y) { unsigned t; fill(&t); return ((x | y) + (x & y)) ^ t; }
# Build: gcc -c hook_registry.S, then the .text bytes of each function (the
# call displacement resolved to fill) are the bytechunks of hook_registry.xml.
	.intel_syntax noprefix
	.text
	.globl	mba_sum
mba_sum:
	push	rbx
	sub	rsp, 16
	mov	ebx, edi
	or	ebx, esi
	and	edi, esi
	add	ebx, edi
	lea	rdi, [rsp+12]
	call	fill
	mov	eax, ebx
	xor	eax, DWORD PTR [rsp+12]
	add	rsp, 16
	pop	rbx
	ret
	.p2align 4
	.globl	fill
fill:
	mov	DWORD PTR [rdi], 7
	ret
