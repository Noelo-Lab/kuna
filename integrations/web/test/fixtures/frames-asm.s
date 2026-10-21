# Prologue shapes the frame walk has to read (assembled, no libc):
# `shrink_wrapped` tests its argument before it saves a register and reserves
# its locals; `probed` reserves a large frame a page at a time through a probe
# loop, as -fstack-clash-protection does.
#
# Rebuild:  gcc -nostdlib -static -o frames-asm.elf frames-asm.s
	.intel_syntax noprefix
	.text

	.globl shrink_wrapped
	.type shrink_wrapped, @function
shrink_wrapped:
	test rdi, rdi
	je 1f
	push rbx
	sub rsp, 0x20
	mov rbx, rdi
	mov rdi, rsp
	call fill
	mov rax, qword ptr [rsp + 0x8]
	add rax, rbx
	add rsp, 0x20
	pop rbx
	ret
1:	xor eax, eax
	ret
	.size shrink_wrapped, .-shrink_wrapped

	.globl probed
	.type probed, @function
probed:
	lea r11, [rsp - 0x3000]
2:	sub rsp, 0x1000
	or qword ptr [rsp], 0
	cmp rsp, r11
	jne 2b
	sub rsp, 0x20
	mov rdi, rsp
	call fill
	mov eax, dword ptr [rsp + 0x10]
	add rsp, 0x3020
	ret
	.size probed, .-probed

	.globl fill
	.type fill, @function
fill:
	mov qword ptr [rdi], 0
	mov qword ptr [rdi + 0x8], 0
	ret
	.size fill, .-fill

	.globl _start
	.type _start, @function
_start:
	xor edi, edi
	call shrink_wrapped
	call probed
	mov eax, 60
	xor edi, edi
	syscall
	.size _start, .-_start
